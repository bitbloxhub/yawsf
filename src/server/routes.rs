use rocket::{Build, Rocket, routes, serde::json::Json};
use utoipa::OpenApi;

use super::openapi::{AddBearer, AddWebhooks};

mod layer_shell;
mod session_lock;
mod system;

pub(crate) fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
	rocket
		.mount("/", system::routes())
		.mount("/layer-shell", layer_shell::routes())
		.mount("/session-lock", session_lock::routes())
		.mount("/", routes![openapi_json])
}

#[derive(OpenApi)]
#[openapi(
	info(
		title = "YAWSF Host API",
		description = "Authenticated control API for the YAWSF native host. Layer-shell mutations complete before their response; session-lock commands remain asynchronous and report lifecycle state separately.",
	),
	security(("bearer_auth" = [])),
	tags(
		(name = "layer-shell", description = "Create, inspect, update, and close wlr-layer-shell windows."),
		(name = "session-lock", description = "Inspect and control the ext-session-lock-v1 session lock."),
	),
	nest(
		(path = "/", api = system::SystemApi),
		(path = "/layer-shell", api = layer_shell::LayerShellApi),
		(path = "/session-lock", api = session_lock::SessionLockApi),
	),
	modifiers(&AddWebhooks, &AddBearer),
)]
pub(crate) struct ApiDoc;

#[rocket::get("/openapi.json")]
fn openapi_json() -> Json<utoipa::openapi::OpenApi> {
	Json(ApiDoc::openapi())
}
