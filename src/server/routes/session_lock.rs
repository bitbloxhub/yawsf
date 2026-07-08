use rocket::{State, http::Status, response::status, routes, serde::json::Json};
use utoipa::OpenApi;

use crate::{AppCommand, SessionLockRequest, SessionLockStatus};

use super::super::{ServerState, auth::BearerToken};

pub(super) fn routes() -> Vec<rocket::Route> {
	routes![session_lock_status, lock, unlock]
}

#[derive(OpenApi)]
#[openapi(paths(session_lock_status, lock, unlock))]
pub(super) struct SessionLockApi;

#[utoipa::path(
	get,
	path = "",
	tag = "session-lock",
	summary = "Get session-lock status",
	description = "Returns the most recently observed session-lock state. `pending` means a lock was requested but compositor confirmation has not arrived.",
	responses(
		(status = 200, description = "Current session-lock status", body = SessionLockStatus),
		(status = 401, description = "Missing or invalid bearer token"),
	)
)]
#[rocket::get("/")]
fn session_lock_status(_auth: BearerToken, state: &State<ServerState>) -> Json<SessionLockStatus> {
	Json(SessionLockStatus {
		state: state.app_state.load().session_lock.clone(),
	})
}

#[utoipa::path(
	post,
	path = "/lock",
	tag = "session-lock",
	summary = "Request session lock",
	description = "Queues a session-lock request that loads the supplied URL on each lock surface. `202 Accepted` confirms queueing; poll the status endpoint for compositor success or failure.",
	request_body = SessionLockRequest,
	responses(
		(status = 202, description = "Session lock requested"),
		(status = 400, description = "Invalid request body"),
		(status = 401, description = "Missing or invalid bearer token"),
		(status = 503, description = "GTK application command channel is unavailable"),
	)
)]
#[rocket::post("/lock", format = "json", data = "<request>")]
fn lock(
	_auth: BearerToken,
	request: Json<SessionLockRequest>,
	state: &State<ServerState>,
) -> Result<status::Accepted<()>, Status> {
	state
		.send(AppCommand::LockSession(request.into_inner()))
		.map_err(|_| Status::ServiceUnavailable)?;

	Ok(status::Accepted(()))
}

#[utoipa::path(
	post,
	path = "/unlock",
	tag = "session-lock",
	summary = "Request session unlock",
	description = "Queues session unlock. `202 Accepted` confirms queueing; poll the status endpoint until it reports `unlocked`.",
	responses(
		(status = 202, description = "Session unlock requested"),
		(status = 401, description = "Missing or invalid bearer token"),
		(status = 503, description = "GTK application command channel is unavailable"),
	)
)]
#[rocket::post("/unlock")]
fn unlock(_auth: BearerToken, state: &State<ServerState>) -> Result<status::Accepted<()>, Status> {
	state
		.send(AppCommand::UnlockSession)
		.map_err(|_| Status::ServiceUnavailable)?;

	Ok(status::Accepted(()))
}
