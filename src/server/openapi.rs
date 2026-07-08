use serde::Serialize;
use utoipa::{
	Modify, OpenApi, ToSchema,
	openapi::{
		OpenApiVersion, PathsBuilder,
		security::{self, SecurityScheme},
	},
};
use utoipa_scalar::{Scalar, Servable};

use super::routes::ApiDoc;

/// Bootstrap payload sent from the host to the shell.
#[derive(Serialize, ToSchema)]
struct ShellStartRequest {
	/// Protocol version implemented by the host.
	protocol: String,
	/// Base URL of this authenticated host API.
	host_api: String,
	/// Bearer token required by host API requests.
	token: String,
}

#[utoipa::path(
	post,
	path = "/_start",
	operation_id = "receiveStart",
	summary = "Receive host bootstrap details",
	description = "MUST be implemented on `/_start` of your YAWSF shell. The host calls this webhook once its API is ready; persist `host_api` and `token` for subsequent authenticated host API requests.",
	request_body = ShellStartRequest,
	responses((status = 204, description = "Bootstrap accepted")),
)]
#[allow(dead_code)]
fn start_webhook() {}

#[utoipa::path(
	post,
	path = "/_events",
	operation_id = "receiveEvents",
	summary = "Receive shell events",
	description = "MUST be implemented on `/_events` of your YAWSF shell for host-originated events.",
	request_body(content = String, content_type = "application/json"),
	responses((status = 204, description = "Event accepted")),
)]
#[allow(dead_code)]
fn receive_events_webhook() {}

#[utoipa::path(
	post,
	path = "/_quit",
	operation_id = "receiveQuit",
	summary = "Request shell shutdown",
	description = "MAY be implemented on `/_quit` of your YAWSF shell. The host calls it before graceful native-host shutdown; shells that exit immediately may close the connection without a response.",
	responses((status = 204, description = "Shell shutdown accepted")),
)]
#[allow(dead_code)]
fn quit_webhook() {}

#[derive(OpenApi)]
#[openapi(paths(receive_events_webhook, start_webhook, quit_webhook))]
struct WebhookDoc;

pub(crate) struct AddWebhooks;

impl Modify for AddWebhooks {
	fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
		openapi.openapi = OpenApiVersion::Version32;

		let mut webhook_doc = WebhookDoc::openapi();
		let start = webhook_doc
			.paths
			.paths
			.remove("/_start")
			.expect("webhook path should exist");
		let events = webhook_doc
			.paths
			.paths
			.remove("/_events")
			.expect("webhook path should exist");
		let quit = webhook_doc
			.paths
			.paths
			.remove("/_quit")
			.expect("webhook path should exist");

		openapi.webhooks = Some(
			PathsBuilder::new()
				.path("/_start", start)
				.path("/_events", events)
				.path("/_quit", quit)
				.build(),
		);
	}
}

pub(crate) struct AddBearer;

impl Modify for AddBearer {
	fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
		openapi
			.components
			.get_or_insert_with(Default::default)
			.add_security_scheme(
				"bearer_auth",
				SecurityScheme::Http(security::Http::new(security::HttpAuthScheme::Bearer)),
			);
	}
}

pub(crate) fn scalar(token: &str) -> impl Into<Vec<rocket::Route>> {
	Scalar::with_url("/scalar", ApiDoc::openapi()).custom_html(include_str!("scalar.html").replace(
		"$token",
		&serde_json::to_string(token).expect("token serializes"),
	))
}
