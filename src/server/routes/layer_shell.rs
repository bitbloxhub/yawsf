use rocket::{State, http::Status, routes, serde::json::Json};
use utoipa::OpenApi;

use crate::{AppCommand, AppState, LayerShellWindowSpec};

use super::super::{ServerState, auth::BearerToken};

pub(super) fn routes() -> Vec<rocket::Route> {
	routes![
		list_layer_shell_windows,
		update_layer_shell_window,
		delete_layer_shell_window
	]
}

#[derive(OpenApi)]
#[openapi(paths(
	list_layer_shell_windows,
	update_layer_shell_window,
	delete_layer_shell_window,
))]
pub(super) struct LayerShellApi;

#[utoipa::path(
	get,
	path = "",
	tag = "layer-shell",
	summary = "List managed layer-shell windows",
	description = "Returns the current host snapshot. `layerShellWindows` contains the managed layer-shell windows; `sessionLock` is included as the host-wide session-lock status.",
	responses(
		(status = 200, description = "Current host state", body = AppState),
		(status = 401, description = "Missing or invalid bearer token"),
	)
)]
#[rocket::get("/")]
fn list_layer_shell_windows(_auth: BearerToken, state: &State<ServerState>) -> Json<AppState> {
	Json((**state.app_state.load()).clone())
}

#[utoipa::path(
	put,
	path = "/{id}",
	tag = "layer-shell",
	summary = "Update a layer-shell window",
	description = "Creates the window when `id` is unknown, or updates the existing window in place. Mutations for the same ID are serialized, and the response is returned after GTK processes the command.",
	params(("id" = String, Path, description = "Layer-shell window identifier")),
	request_body = LayerShellWindowSpec,
	responses(
		(status = 204, description = "Layer-shell window update processed"),
		(status = 400, description = "Invalid request body"),
		(status = 401, description = "Missing or invalid bearer token"),
		(status = 503, description = "GTK application command channel is unavailable"),
	)
)]
#[rocket::put("/<id>", format = "json", data = "<request>")]
async fn update_layer_shell_window(
	_auth: BearerToken,
	id: String,
	request: Json<LayerShellWindowSpec>,
	state: &State<ServerState>,
) -> Result<Status, Status> {
	let upsert_lock = state.layer_shell_lock(&id);
	let _guard = upsert_lock.lock().await;
	let (completed_tx, completed_rx) = tokio::sync::oneshot::channel();
	state
		.send(AppCommand::UpsertLayerShellWindow(
			id,
			request.into_inner(),
			completed_tx,
		))
		.map_err(|_| Status::ServiceUnavailable)?;
	completed_rx.await.map_err(|_| Status::ServiceUnavailable)?;

	Ok(Status::NoContent)
}

#[utoipa::path(
	delete,
	path = "/{id}",
	tag = "layer-shell",
	summary = "Close a layer-shell window",
	description = "Closes the window identified by `id`. Mutations for the same ID are serialized, and the response is returned after GTK processes the command. Unknown IDs leave the managed window set unchanged.",
	params(("id" = String, Path, description = "Layer-shell window identifier")),
	responses(
		(status = 204, description = "Layer-shell window close processed"),
		(status = 401, description = "Missing or invalid bearer token"),
		(status = 503, description = "GTK application command channel is unavailable"),
	)
)]
#[rocket::delete("/<id>")]
async fn delete_layer_shell_window(
	_auth: BearerToken,
	id: String,
	state: &State<ServerState>,
) -> Result<Status, Status> {
	let window_lock = state.layer_shell_lock(&id);
	let _guard = window_lock.lock().await;
	let (completed_tx, completed_rx) = tokio::sync::oneshot::channel();
	state
		.send(AppCommand::CloseLayerShellWindow(id, completed_tx))
		.map_err(|_| Status::ServiceUnavailable)?;
	completed_rx.await.map_err(|_| Status::ServiceUnavailable)?;

	Ok(Status::NoContent)
}
