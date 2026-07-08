use std::{sync::Arc, thread};

use arc_swap::ArcSwap;
use gtk4::{
	Application,
	gio::prelude::{ApplicationExt, ApplicationExtManual},
};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::{AppCommand, AppState, Cli, server::run_server};

mod surfaces;

pub fn activate_host(app: &Application, args: Cli) {
	let (app_command_tx, app_command_rx) = mpsc::unbounded_channel::<AppCommand>();
	let app_state = Arc::new(ArcSwap::from_pointee(AppState::default()));
	let cancel = CancellationToken::new();

	let hold = app.hold();
	app.connect_shutdown(move |_| {
		// Capture `hold` so it stays alive until the app shuts down.
		let _ = &hold;
	});

	surfaces::spawn_command_loop(app.clone(), app_command_rx, app_state.clone());

	spawn_server_thread(args, app_command_tx, cancel.clone(), app_state);

	app.connect_shutdown(move |_| {
		cancel.cancel();
	});
}

fn spawn_server_thread(
	args: Cli,
	app_command_tx: mpsc::UnboundedSender<AppCommand>,
	cancel: CancellationToken,
	app_state: Arc<ArcSwap<AppState>>,
) {
	thread::spawn(move || {
		let runtime = tokio::runtime::Builder::new_multi_thread()
			.enable_all()
			.build()
			.expect("failed to build Tokio runtime");

		let ctrl_c_task = {
			let args = args.clone();
			let app_command_tx = app_command_tx.clone();
			let cancel = cancel.clone();

			runtime.spawn(async move { handle_ctrl_c(args, app_command_tx, cancel).await })
		};

		let result = runtime.block_on(run_server(args, app_command_tx.clone(), cancel, app_state));

		ctrl_c_task.abort();

		if let Err(err) = result {
			eprintln!("Tokio side failed: {err:?}");
			let _ = app_command_tx.send(AppCommand::Quit);
		}
	});
}

async fn handle_ctrl_c(
	args: Cli,
	app_command_tx: mpsc::UnboundedSender<AppCommand>,
	cancel: CancellationToken,
) -> anyhow::Result<()> {
	tokio::signal::ctrl_c().await?;

	if args.webapp_command.is_some() {
		cancel.cancel();
		return Ok(());
	}

	let client = reqwest::Client::new();
	if let Ok(url) = args.base_url.join("_quit") {
		let _ = client.post(url).send().await;
	}

	app_command_tx.send(AppCommand::Quit)?;
	Ok(())
}
