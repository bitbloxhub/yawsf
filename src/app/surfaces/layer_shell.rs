use std::{cell::Cell, fs, rc::Rc};

use gtk4::{
	Application, ApplicationWindow,
	gdk::prelude::{DisplayExt, DisplayExtManual, MonitorExt},
	gio::prelude::ListModelExt,
	glib::object::CastNone,
	prelude::{GtkWindowExt, WidgetExt},
};
use gtk4_layer_shell::{Edge, KeyboardMode as ShellKeyboardMode, Layer as ShellLayer, LayerShell};
use webkit6::{LoadEvent, NetworkSession, WebView, prelude::WebViewExt};

use crate::{ExclusiveZone, KeyboardMode, Layer, LayerShellWindowSpec};

pub(super) fn install_transparent_style(display: &gtk4::gdk::Display) {
	let css = gtk4::CssProvider::new();
	css.load_from_string(
		r#"
		window,
		window.background,
		window.background.csd,
		window.background.solid-csd,
		window > contents,
		window.background > contents {
			background: transparent;
			background-color: transparent;
			background-image: none;
			box-shadow: none;
		}
		"#,
	);

	gtk4::style_context_add_provider_for_display(display, &css, gtk4::STYLE_PROVIDER_PRIORITY_USER);
}

pub(super) fn build_layer_shell_window(
	app: &Application,
	id: String,
	spec: &LayerShellWindowSpec,
	alive: &Rc<Cell<bool>>,
) -> Result<ApplicationWindow, String> {
	let display = gtk4::gdk::Display::default().ok_or("GTK display unavailable")?;
	if !display.backend().is_wayland() || !gtk4_layer_shell::is_supported() {
		return Err(
			"layer-shell unavailable: GTK must use Wayland and compositor must support wlr-layer-shell".into(),
		);
	}

	let network_session = webkit_network_session()?;
	let webview = WebView::builder().network_session(&network_session).build();
	webview.set_hexpand(true);
	webview.set_vexpand(true);
	webview.set_background_color(&gtk4::gdk::RGBA::new(0.0, 0.0, 0.0, 0.0));

	let window = ApplicationWindow::builder()
		.application(app)
		.title(id)
		.default_width(spec.width.unwrap_or(500) as i32)
		.default_height(spec.height.unwrap_or(240) as i32)
		.decorated(false)
		.child(&webview)
		.build();

	configure_layer_shell_window(&window, &display, spec)?;
	webview.connect_load_changed({
		let window = window.clone();
		let alive = alive.clone();
		move |_, event| {
			if event == LoadEvent::Finished && alive.get() {
				window.present();
			}
		}
	});
	webview.connect_load_failed(|_, _, uri, error| {
		eprintln!("layer-shell page failed to load {uri}: {error}");
		false
	});
	webview.load_uri(spec.url.as_str());
	Ok(window)
}

fn configure_layer_shell_window(
	window: &ApplicationWindow,
	display: &gtk4::gdk::Display,
	spec: &LayerShellWindowSpec,
) -> Result<(), String> {
	window.init_layer_shell();
	if !window.is_layer_window() {
		return Err("gtk4-layer-shell failed to initialize window".into());
	}

	window.set_namespace(spec.namespace.as_deref());
	window.set_layer(match spec.layer {
		Layer::Background => ShellLayer::Background,
		Layer::Bottom => ShellLayer::Bottom,
		Layer::Top => ShellLayer::Top,
		Layer::Overlay => ShellLayer::Overlay,
	});
	window.set_keyboard_mode(match spec.keyboard_mode {
		KeyboardMode::None => ShellKeyboardMode::None,
		KeyboardMode::OnDemand => ShellKeyboardMode::OnDemand,
		KeyboardMode::Exclusive => ShellKeyboardMode::Exclusive,
	});

	window.set_anchor(Edge::Top, spec.anchors.top);
	window.set_anchor(Edge::Bottom, spec.anchors.bottom);
	window.set_anchor(Edge::Left, spec.anchors.left);
	window.set_anchor(Edge::Right, spec.anchors.right);
	window.set_margin(Edge::Top, spec.margins.top);
	window.set_margin(Edge::Bottom, spec.margins.bottom);
	window.set_margin(Edge::Left, spec.margins.left);
	window.set_margin(Edge::Right, spec.margins.right);

	match spec.exclusive_zone {
		ExclusiveZone::None => window.set_exclusive_zone(0),
		ExclusiveZone::Auto => window.auto_exclusive_zone_enable(),
		ExclusiveZone::Fixed(zone) => window.set_exclusive_zone(zone),
	}

	set_monitor(window, display, spec.monitor.as_deref());
	Ok(())
}

fn set_monitor(
	window: &ApplicationWindow,
	display: &gtk4::gdk::Display,
	monitor_name: Option<&str>,
) {
	let Some(monitor_name) = monitor_name else {
		return;
	};

	let monitors = display.monitors();
	let monitor = (0..monitors.n_items()).find_map(|index| {
		monitors
			.item(index)
			.and_downcast::<gtk4::gdk::Monitor>()
			.filter(|monitor| monitor.connector().as_deref() == Some(monitor_name))
	});

	if let Some(monitor) = monitor.as_ref() {
		window.set_monitor(Some(monitor));
	} else {
		eprintln!("layer-shell monitor not found: {monitor_name}");
	}
}

fn webkit_network_session() -> Result<NetworkSession, String> {
	let data_directory = gtk4::glib::user_data_dir().join("yawsf/webkit");
	let cache_directory = gtk4::glib::user_cache_dir().join("yawsf/webkit");
	fs::create_dir_all(&data_directory).map_err(|err| err.to_string())?;
	fs::create_dir_all(&cache_directory).map_err(|err| err.to_string())?;

	let data_directory = data_directory
		.to_str()
		.ok_or("WebKit data path is not UTF-8")?;
	let cache_directory = cache_directory
		.to_str()
		.ok_or("WebKit cache path is not UTF-8")?;

	Ok(NetworkSession::new(
		Some(data_directory),
		Some(cache_directory),
	))
}
