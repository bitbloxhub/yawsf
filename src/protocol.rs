use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Snapshot entry for one managed layer-shell window.
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct LayerShellWindowState {
	/// Identifier used by the layer-shell API path.
	pub id: String,

	/// Configuration currently associated with this window identifier.
	#[serde(flatten)]
	pub spec: LayerShellWindowSpec,
}

/// Layer-shell window listing response.
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LayerShellWindowsResponse {
	pub layer_shell_windows: Vec<LayerShellWindowState>,
}

/// Current host state.
#[derive(Serialize, Deserialize, Clone, Debug, Default, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AppState {
	/// Layer-shell windows currently managed by the host.
	pub layer_shell_windows: Vec<LayerShellWindowState>,
	/// Most recently observed session-lock state.
	#[serde(default)]
	pub session_lock: SessionLockState,
}

/// Lifecycle state reported by the session-lock compositor protocol.
#[derive(Serialize, Deserialize, Clone, Debug, Default, ToSchema)]
#[serde(rename_all = "kebab-case")]
pub enum SessionLockState {
	/// No session lock is active.
	#[default]
	Unlocked,
	/// Lock was requested and awaits compositor confirmation.
	Pending,
	/// Compositor confirmed session lock.
	Locked,
	/// Compositor rejected or failed the lock request.
	Failed,
}

/// Stacking layer requested from wlr-layer-shell.
#[derive(Serialize, Deserialize, Clone, Debug, Default, ToSchema)]
#[serde(rename_all = "kebab-case")]
pub enum Layer {
	/// Behind regular application windows.
	Background,
	/// Below the top layer but above normal windows.
	Bottom,
	/// Above normal application windows.
	#[default]
	Top,
	/// Above every other layer.
	Overlay,
}

/// Keyboard focus behavior requested from wlr-layer-shell.
#[derive(Serialize, Deserialize, Clone, Debug, Default, ToSchema)]
#[serde(rename_all = "kebab-case")]
pub enum KeyboardMode {
	/// Never request keyboard focus.
	#[default]
	None,
	/// Request focus only when the compositor considers it appropriate.
	OnDemand,
	/// Exclusively receive keyboard focus.
	Exclusive,
}

/// Edges to which the layer-shell surface is anchored.
#[derive(Serialize, Deserialize, Clone, Debug, Default, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LayerShellAnchors {
	/// Anchor to the top edge.
	#[serde(default)]
	pub top: bool,
	/// Anchor to the bottom edge.
	#[serde(default)]
	pub bottom: bool,
	/// Anchor to the left edge.
	#[serde(default)]
	pub left: bool,
	/// Anchor to the right edge.
	#[serde(default)]
	pub right: bool,
}

/// Per-edge layer-shell margins in logical pixels.
#[derive(Serialize, Deserialize, Clone, Debug, Default, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LayerShellMargins {
	/// Top margin.
	#[serde(default)]
	pub top: i32,
	/// Bottom margin.
	#[serde(default)]
	pub bottom: i32,
	/// Left margin.
	#[serde(default)]
	pub left: i32,
	/// Right margin.
	#[serde(default)]
	pub right: i32,
}

/// Space reserved by the layer-shell surface.
#[derive(Serialize, Deserialize, Clone, Debug, Default, ToSchema)]
#[serde(tag = "mode", content = "value", rename_all = "kebab-case")]
pub enum ExclusiveZone {
	/// Reserve no screen space.
	#[default]
	None,
	/// Let the compositor derive reserved space from surface size.
	Auto,
	/// Reserve an exact logical-pixel size.
	Fixed(i32),
}

/// Configuration for a wlr-layer-shell window.
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LayerShellWindowSpec {
	/// URL loaded in the window's web view.
	#[schema(value_type = String)]
	pub url: url::Url,

	/// Optional compositor namespace used to identify this surface.
	#[serde(default)]
	pub namespace: Option<String>,
	/// Compositor layer for the surface.
	#[serde(default)]
	pub layer: Layer,
	/// Screen edges to which the surface is anchored.
	#[serde(default)]
	pub anchors: LayerShellAnchors,
	/// Space reserved from other surfaces.
	#[serde(default)]
	pub exclusive_zone: ExclusiveZone,
	/// Offset from each anchored screen edge, in logical pixels.
	#[serde(default)]
	pub margins: LayerShellMargins,
	/// Keyboard focus behavior for the surface.
	#[serde(default)]
	pub keyboard_mode: KeyboardMode,
	/// Optional logical width. Defaults to 500 when omitted.
	pub width: Option<u32>,
	/// Optional logical height. Defaults to 240 when omitted.
	pub height: Option<u32>,
	/// Output/monitor name. `None` lets the compositor choose.
	pub monitor: Option<String>,
	/// Whether the window is mapped and visible.
	#[schema(default = true)]
	pub visible: Option<bool>,
}

/// URL loaded on every session-lock surface.
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct SessionLockRequest {
	/// URL loaded in each compositor-provided lock surface.
	#[schema(value_type = String)]
	pub url: url::Url,
}

/// Current session-lock lifecycle state.
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct SessionLockStatus {
	/// Most recently observed compositor state.
	pub state: SessionLockState,
}

#[derive(Debug)]
pub enum AppCommand {
	Quit,
	UpsertLayerShellWindow(
		String,
		LayerShellWindowSpec,
		tokio::sync::oneshot::Sender<()>,
	),
	CloseLayerShellWindow(String, tokio::sync::oneshot::Sender<()>),
	LockSession(SessionLockRequest),
	UnlockSession,
}
