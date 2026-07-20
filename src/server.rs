use std::{
	collections::{HashMap, HashSet},
	sync::{
		Arc, Mutex as StdMutex,
		atomic::{AtomicBool, Ordering},
	},
};

use arc_swap::ArcSwap;
use rocket::config::Shutdown;
use tokio::{
	sync::{Mutex, mpsc::UnboundedSender},
	task::JoinHandle,
	time::{Duration, sleep},
};
use tokio_util::sync::CancellationToken;
use utoipa::OpenApi;

use crate::{AppCommand, AppState, Cli};

mod auth;
mod callbacks;
mod openapi;
mod routes;
mod webapp;

pub fn openapi_document() -> utoipa::openapi::OpenApi {
	routes::ApiDoc::openapi()
}

use callbacks::ShellClient;
use webapp::WebappProcess;

pub(crate) struct ServerState {
	token: String,
	callbacks: ShellClient,
	manages_webapp: bool,
	shutdown_requested: AtomicBool,
	commands: UnboundedSender<AppCommand>,
	layer_shell_locks: StdMutex<HashMap<String, Arc<Mutex<()>>>>,
	app_state: Arc<ArcSwap<AppState>>,
}

impl ServerState {
	fn new(
		token: String,
		callbacks: ShellClient,
		manages_webapp: bool,
		commands: UnboundedSender<AppCommand>,
		app_state: Arc<ArcSwap<AppState>>,
	) -> Self {
		Self {
			token,
			callbacks,
			manages_webapp,
			shutdown_requested: AtomicBool::new(false),
			commands,
			layer_shell_locks: StdMutex::new(HashMap::new()),
			app_state,
		}
	}

	fn send(&self, command: AppCommand) -> Result<(), ()> {
		self.commands.send(command).map_err(|_| ())
	}

	fn layer_shell_lock(&self, id: &str) -> Arc<Mutex<()>> {
		self.layer_shell_locks
			.lock()
			.expect("layer-shell lock poisoned")
			.entry(id.to_owned())
			.or_default()
			.clone()
	}

	fn begin_shutdown(&self) -> bool {
		!self.shutdown_requested.swap(true, Ordering::AcqRel)
	}
}

pub async fn run_server(
	args: Cli,
	app_command_tx: UnboundedSender<AppCommand>,
	cancel: CancellationToken,
	app_state: Arc<ArcSwap<AppState>>,
) -> anyhow::Result<()> {
	let webapp_command = args.parsed_webapp_command()?;
	let manages_webapp = webapp_command.is_some();
	let host_api = url::Url::parse(&format!("http://{}/", args.bind))?;
	let client = reqwest::Client::new();
	let callbacks = ShellClient::new(client, &args, host_api);

	if let Some(command) = webapp_command {
		let mut webapp = WebappProcess::spawn(&command)?;
		let cancel = cancel.clone();
		tokio::spawn(async move {
			tokio::select! {
				status = webapp.wait() => {
					eprintln!("webapp exited: {status:?}");
					webapp.terminate().await;
				}
				_ = cancel.cancelled() => webapp.terminate().await,
			}
			cancel.cancel();
		});
	}

	let rocket = build_rocket(
		&args,
		callbacks.clone(),
		manages_webapp,
		app_command_tx.clone(),
		app_state,
	);
	let rocket = rocket.ignite().await?;
	let shutdown = rocket.shutdown();
	let rocket_task = tokio::spawn(async move { rocket.launch().await });

	if let Err(err) = post_start(&callbacks, &cancel).await {
		shutdown.notify();
		let _ = rocket_task.await;
		return Err(err);
	}

	let cancel_task = spawn_shutdown_task(cancel, shutdown);
	let result = rocket_task.await;

	cancel_task.abort();
	let _ = cancel_task.await;

	result??;
	let _ = app_command_tx.send(AppCommand::Quit);

	Ok(())
}

async fn post_start(callbacks: &ShellClient, cancel: &CancellationToken) -> anyhow::Result<()> {
	const ATTEMPTS: u32 = 300;

	for attempt in 1..=ATTEMPTS {
		tokio::select! {
			biased;
			_ = cancel.cancelled() => anyhow::bail!("webapp exited before accepting the startup callback"),
			result = callbacks.post_start() => match result {
				Ok(()) => return Ok(()),
				Err(err) if attempt == ATTEMPTS => return Err(err),
				Err(_) => sleep(Duration::from_millis(100)).await,
			},
		}
	}

	unreachable!("startup attempts are nonzero")
}

fn build_rocket(
	args: &Cli,
	callbacks: ShellClient,
	manages_webapp: bool,
	app_command_tx: UnboundedSender<AppCommand>,
	app_state: Arc<ArcSwap<AppState>>,
) -> rocket::Rocket<rocket::Build> {
	let figment = rocket::Config::figment()
		.merge(("address", args.bind.ip()))
		.merge(("port", args.bind.port()))
		.merge(("log_level", "off"))
		.merge((
			"shutdown",
			Shutdown {
				ctrlc: false,
				signals: HashSet::new(),
				..Default::default()
			},
		));

	let state = ServerState::new(
		args.token.clone(),
		callbacks,
		manages_webapp,
		app_command_tx,
		app_state,
	);

	routes::mount(rocket::custom(figment).manage(state)).mount("/", openapi::scalar(&args.token))
}

fn spawn_shutdown_task(cancel: CancellationToken, shutdown: rocket::Shutdown) -> JoinHandle<()> {
	tokio::spawn(async move {
		cancel.cancelled().await;
		shutdown.notify();
	})
}
