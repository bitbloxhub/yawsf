use std::{cell::Cell, collections::BTreeMap, rc::Rc, sync::Arc};

use arc_swap::ArcSwap;
use gtk4::{
	Application, ApplicationWindow, gio::prelude::ApplicationExt, glib, prelude::GtkWindowExt,
};
use tokio::sync::mpsc::UnboundedReceiver;

use crate::{AppCommand, AppState, LayerShellWindowSpec, LayerShellWindowState};
use webkit6::WebView;

mod layer_shell;
mod session_lock;

struct LayerShellWindowEntry {
	window: ApplicationWindow,
	webview: WebView,
	alive: Rc<Cell<bool>>,
	spec: LayerShellWindowSpec,
}

pub(crate) fn spawn_command_loop(
	app: Application,
	app_command_rx: UnboundedReceiver<AppCommand>,
	app_state: Arc<ArcSwap<AppState>>,
) {
	glib::spawn_future_local(async move {
		let mut surfaces = LayerShellWindowManager::new(app, app_state);
		surfaces.run(app_command_rx).await;
	});
}

struct LayerShellWindowManager {
	app: Application,
	layer_shell_windows: BTreeMap<String, LayerShellWindowEntry>,

	app_state: Arc<ArcSwap<AppState>>,
	session_lock: session_lock::SessionLockController,
}

impl LayerShellWindowManager {
	fn new(app: Application, app_state: Arc<ArcSwap<AppState>>) -> Self {
		if let Some(display) = gtk4::gdk::Display::default() {
			layer_shell::install_transparent_style(&display);
		}

		let session_lock = session_lock::SessionLockController::new(app_state.clone());

		Self {
			app,
			layer_shell_windows: BTreeMap::new(),

			app_state,
			session_lock,
		}
	}

	async fn run(&mut self, mut app_command_rx: UnboundedReceiver<AppCommand>) {
		while let Some(command) = app_command_rx.recv().await {
			if !self.handle_command(command) {
				break;
			}
		}
	}

	fn handle_command(&mut self, command: AppCommand) -> bool {
		match command {
			AppCommand::UpsertLayerShellWindow(id, spec, completed) => {
				self.upsert_layer_shell_window(id, spec);
				let _ = completed.send(());
			}
			AppCommand::CloseLayerShellWindow(id, completed) => {
				self.close_layer_shell_window(&id);
				let _ = completed.send(());
			}
			AppCommand::LockSession(request) => self.session_lock.lock(request),
			AppCommand::UnlockSession => self.session_lock.unlock(),
			AppCommand::Quit => {
				self.session_lock.unlock();
				self.app.quit();
				return false;
			}
		}

		true
	}

	fn upsert_layer_shell_window(&mut self, id: String, spec: LayerShellWindowSpec) {
		if let Some(entry) = self.layer_shell_windows.get_mut(&id) {
			let url_changed = entry.spec.url != spec.url;
			layer_shell::update_layer_shell_window(
				&entry.window,
				&entry.webview,
				&spec,
				url_changed,
			);
			entry.spec = spec;
			self.publish_state();
			return;
		}

		let alive = Rc::new(Cell::new(true));
		let (window, webview) =
			match layer_shell::build_layer_shell_window(&self.app, id.clone(), &spec, &alive) {
				Ok(window) => window,
				Err(err) => {
					eprintln!("failed to create layer-shell window: {err}");
					return;
				}
			};

		self.layer_shell_windows.insert(
			id,
			LayerShellWindowEntry {
				window,
				webview,
				alive,
				spec,
			},
		);

		self.publish_state();
	}

	fn close_layer_shell_window(&mut self, id: &str) {
		if let Some(window) = self.layer_shell_windows.remove(id) {
			window.alive.set(false);
			window.window.destroy();
		}

		self.publish_state();
	}

	fn publish_state(&self) {
		let session_lock = self.app_state.load().session_lock.clone();
		let state = AppState {
			layer_shell_windows: self
				.layer_shell_windows
				.iter()
				.map(|(id, entry)| LayerShellWindowState {
					id: id.clone(),
					spec: entry.spec.clone(),
				})
				.collect(),
			session_lock,
		};

		self.app_state.store(Arc::new(state));
	}
}
