use crate::db::{Database, DatabaseAccess};
use common::security::hash;
use common::user::User;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::State;
use rocket_contrib::json;

pub struct LoginAuth(User);

use super::err::AuthError;

impl<'a, 'r> FromRequest<'a, 'r> for LoginAuth {
    type Error = AuthError;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let auth_header: Vec<_> = request.headers().get("Authorization").collect();
        match auth_header.len() {
            0 => Outcome::Failure((Status::Unauthorized, AuthError::MissingAuth)),
            1 => authorize(auth_header[0], request),
            _ => Outcome::Failure((Status::BadRequest, AuthError::BadHeaderCount)),
        }
    }
}

impl LoginAuth {
    pub fn into_inner(self) -> User {
        self.0
    }
}

/// Given an Authorization HTTP header (form: username:password), look up the user and authenticate
/// Returns an `Outcome<T, E>` containing either the `LoginAuth` request guard
/// or an error plus HTTP status
///
/// # Arguments
///
/// * `auth_header` - The value of the HTTP Authorization header in the form of `username:password`
/// * `request` - The active request to authenticate for
fn authorize(auth_header: &str, request: &Request) -> Outcome<LoginAuth, AuthError> {
    // Parse username and password from auth header
    let creds: Vec<&str> = auth_header.split(':').collect();

    let username = match creds.get(0) {
        Some(u) => *u,
        None => return Outcome::Failure((Status::Unauthorized, AuthError::MissingAuth)),
    };
    let password = match creds.get(1) {
        Some(p) => *p,
        None => return Outcome::Failure((Status::Unauthorized, AuthError::MissingAuth)),
    };

    // Get db
    let db = request
        .guard::<State<Database>>()
        .expect("No managed db connection");
    // Get user

    let query = json! {{
        "username": username
    }};

    let user = db.find_one::<User>("users", query);

    let user = match user {
        Ok(Some(u)) => u,
        Ok(None) => {
            return Outcome::Failure((Status::Unauthorized, AuthError::NoUser(username.into())))
        }
        Err(e) => {
            return Outcome::Failure((Status::ServiceUnavailable, AuthError::DBError { source: e }))
        }
    };

    // Hash password
    let encoded_hash = hash(&user.salt, &password);

    // Compare
    if encoded_hash == user.password_hash {
        Outcome::Success(LoginAuth(user))
    } else {
        Outcome::Failure((
            Status::Unauthorized,
            AuthError::WrongPassword(username.into()),
        ))
    }
}
