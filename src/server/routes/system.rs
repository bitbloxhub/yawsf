use rocket::{State, response::status, routes, serde::json::Json};
use serde::Serialize;
use utoipa::{OpenApi, ToSchema};

use super::super::{ServerState, auth::BearerToken};

pub(super) fn routes() -> Vec<rocket::Route> {
	routes![health, quit]
}

#[derive(OpenApi)]
#[openapi(paths(health, quit))]
pub(super) struct SystemApi;

/// Liveness response from the native host.
#[derive(Serialize, ToSchema)]
struct HealthResponse {
	/// Always `ok` when the host accepts requests.
	status: String,
}

#[utoipa::path(
	get,
	path = "health",
	summary = "Check host availability",
	description = "Returns once the Rocket host API is accepting authenticated requests.",
	responses(
		(status = 200, description = "Host is alive", body = HealthResponse),
		(status = 401, description = "Missing or invalid bearer token"),
	)
)]
#[rocket::get("/health")]
fn health(_bearer: BearerToken) -> Json<HealthResponse> {
	Json(HealthResponse {
		status: "ok".into(),
	})
}

/// Accepted native-host shutdown request.
#[derive(Serialize, ToSchema)]
struct QuitResponse {
	/// Always `shutting down` for an accepted request.
	status: String,
}

#[utoipa::path(
	post,
	path = "quit",
	summary = "Request host shutdown",
	description = "With `--webapp-command`, requests graceful Rocket shutdown and sends the supervised child `SIGTERM` (then `SIGKILL` after five seconds). Otherwise, sends the shell an `/_quit` callback. Repeated requests do not repeat shutdown notification.",
	responses(
		(status = 202, description = "Graceful host shutdown requested", body = QuitResponse),
		(status = 401, description = "Missing or invalid bearer token"),
	)
)]
#[rocket::post("/quit")]
async fn quit(
	_bearer: BearerToken,
	state: &State<ServerState>,
	shutdown: rocket::Shutdown,
) -> status::Accepted<Json<QuitResponse>> {
	if state.begin_shutdown() {
		if !state.manages_webapp {
			let _ = state.callbacks.post_quit().await;
		}
		shutdown.notify();
	}

	status::Accepted(Json(QuitResponse {
		status: "shutting down".to_string(),
	}))
}
