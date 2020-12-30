use crate::db::{Database, DatabaseAccess};
use common::user::User;
use log::error;
use rocket::http::{Cookies, Status};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::State;
use rocket_contrib::json;

use super::err::AuthError;

pub struct TokenAuth(User);

impl<'a, 'r> FromRequest<'a, 'r> for TokenAuth {
    type Error = AuthError;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let mut cookies = match request.guard::<Cookies>() {
            Outcome::Success(c) => c,
            _ => {
                error!("Failed to collect cookies (no Cookies Guard found)");
                return Outcome::Failure((Status::InternalServerError, AuthError::Unspecified));
            }
        };
        let token_cookie = match cookies.get_private("auth_token") {
            Some(c) => c,
            None => return Outcome::Failure((Status::Unauthorized, AuthError::MissingToken)),
        };

        authorize(token_cookie.value(), request)
    }
}

impl TokenAuth {
    pub fn into_inner(self) -> User {
        self.0
    }
}

/// Given a user token, look up the user and authenticate
/// Returns an `Outcome<T, E>` containing either the `TokenAuth` request guard
/// or an error plus HTTP status
///
/// # Arguments
///
/// * `token` - The token to look up a user with
/// * `request` - The active request to authenticate for
fn authorize(token: &str, request: &Request) -> Outcome<TokenAuth, AuthError> {
    // Get db
    let db = request
        .guard::<State<Database>>()
        .expect("No managed db connection");

    let query = json! {{
        "auth_token": token
    }};

    let user = db.find_one::<User>("users", query);

    let user = match user {
        Ok(Some(u)) => u,
        Ok(None) => return Outcome::Failure((Status::Unauthorized, AuthError::BadToken)),
        Err(e) => {
            return Outcome::Failure((Status::ServiceUnavailable, AuthError::DBError { source: e }))
        }
    };

    Outcome::Success(TokenAuth(user))
}
