use rocket::{
	State,
	http::Status,
	request::{FromRequest, Outcome},
};

use super::ServerState;

pub(crate) struct BearerToken;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BearerToken {
	type Error = ();

	async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
		let Outcome::Success(state) = request.guard::<&State<ServerState>>().await else {
			return Outcome::Error((Status::InternalServerError, ()));
		};

		let Some(header) = request.headers().get_one("Authorization") else {
			return Outcome::Error((Status::Unauthorized, ()));
		};

		let Some((scheme, token)) = header.split_once(' ') else {
			return Outcome::Error((Status::Unauthorized, ()));
		};

		if !scheme.eq_ignore_ascii_case("Bearer") || token != state.token {
			return Outcome::Error((Status::Unauthorized, ()));
		}

		Outcome::Success(Self)
	}
}
