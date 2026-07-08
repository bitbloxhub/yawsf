use std::{cell::RefCell, rc::Rc, sync::Arc};

use arc_swap::ArcSwap;
use gtk4::{
	glib,
	prelude::{GtkWindowExt, WidgetExt},
};
use gtk4_session_lock::Instance as SessionLockInstance;
use webkit6::{WebView, prelude::WebViewExt};

use crate::{AppState, SessionLockRequest, SessionLockState};

struct SessionLockSurface {
	window: gtk4::Window,
	webview: WebView,
}

pub(super) struct SessionLockController {
	instance: Option<SessionLockInstance>,
	url: Rc<RefCell<Option<url::Url>>>,
	surfaces: Rc<RefCell<Vec<SessionLockSurface>>>,
	app_state: Arc<ArcSwap<AppState>>,
}

impl SessionLockController {
	pub(super) fn new(app_state: Arc<ArcSwap<AppState>>) -> Self {
		let url = Rc::new(RefCell::new(None));
		let surfaces = Rc::new(RefCell::new(Vec::new()));
		let instance = create_session_lock(&app_state, url.clone(), surfaces.clone());

		Self {
			instance,
			url,
			surfaces,
			app_state,
		}
	}

	pub(super) fn lock(&self, request: SessionLockRequest) {
		let Some(instance) = self.instance.as_ref() else {
			update_state(&self.app_state, SessionLockState::Failed);
			eprintln!("session lock unavailable: compositor does not support ext-session-lock-v1");
			return;
		};

		if instance.is_locked() {
			update_state(&self.app_state, SessionLockState::Locked);
			eprintln!("session is already locked");
			return;
		}

		*self.url.borrow_mut() = Some(request.url);
		update_state(&self.app_state, SessionLockState::Pending);
		if !instance.lock() {
			*self.url.borrow_mut() = None;
			update_state(&self.app_state, SessionLockState::Failed);
			eprintln!("failed to start session lock");
		}
	}

	pub(super) fn unlock(&self) {
		let Some(instance) = self.instance.as_ref() else {
			update_state(&self.app_state, SessionLockState::Unlocked);
			return;
		};

		for surface in self.surfaces.borrow().iter() {
			surface.webview.stop_loading();
		}

		instance.unlock();
	}
}

fn create_session_lock(
	app_state: &Arc<ArcSwap<AppState>>,
	url: Rc<RefCell<Option<url::Url>>>,
	surfaces: Rc<RefCell<Vec<SessionLockSurface>>>,
) -> Option<SessionLockInstance> {
	if !gtk4_session_lock::is_supported() {
		return None;
	}

	let instance = SessionLockInstance::new();
	instance.connect_monitor({
		let url = url.clone();
		let surfaces = surfaces.clone();

		move |instance, monitor| {
			let Some(url) = url.borrow().clone() else {
				eprintln!("session lock monitor arrived without a lock URL");
				return;
			};

			let surface = build_surface(&url);
			instance.assign_window_to_monitor(&surface.window, monitor);
			surfaces.borrow_mut().push(surface);
		}
	});
	instance.connect_locked({
		let app_state = app_state.clone();
		move |_| update_state(&app_state, SessionLockState::Locked)
	});
	instance.connect_failed({
		let app_state = app_state.clone();
		let url = url.clone();
		let surfaces = surfaces.clone();
		move |_| {
			*url.borrow_mut() = None;
			update_state(&app_state, SessionLockState::Failed);
			let surfaces = surfaces.clone();
			glib::idle_add_local_once(move || surfaces.borrow_mut().clear());
			eprintln!("session lock failed");
		}
	});
	instance.connect_unlocked({
		let app_state = app_state.clone();
		move |_| {
			*url.borrow_mut() = None;
			update_state(&app_state, SessionLockState::Unlocked);
			let surfaces = surfaces.clone();
			glib::idle_add_local_once(move || surfaces.borrow_mut().clear());
		}
	});

	Some(instance)
}

fn build_surface(url: &url::Url) -> SessionLockSurface {
	let webview = WebView::new();
	webview.set_hexpand(true);
	webview.set_vexpand(true);
	webview.set_background_color(&gtk4::gdk::RGBA::new(0.0, 0.0, 0.0, 0.0));

	let window = gtk4::Window::new();
	window.set_child(Some(&webview));
	window.connect_map({
		let webview = webview.clone();
		let url = url.clone();
		move |_| {
			let webview = webview.clone();
			let url = url.clone();
			glib::idle_add_local_once(move || {
				if webview.is_mapped() {
					webview.load_uri(url.as_str());
				}
			});
		}
	});

	SessionLockSurface { window, webview }
}

fn update_state(app_state: &Arc<ArcSwap<AppState>>, session_lock: SessionLockState) {
	let mut state = (**app_state.load()).clone();
	state.session_lock = session_lock;
	app_state.store(Arc::new(state));
}
