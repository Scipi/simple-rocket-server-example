use crate::db::{Database, DatabaseAccess};
use common::security::hash;
use common::user::User;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::State;
use rocket_contrib::json;

pub struct LoginAuth(User);

#[derive(Debug)]
pub enum LoginError {
    MissingAuth,
    NoUser,
    WrongPassword,
    BadHeaderCount,
    DBError,
    // Unspecified,
}

impl<'a, 'r> FromRequest<'a, 'r> for LoginAuth {
    type Error = LoginError;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let auth_header: Vec<_> = request.headers().get("Authorization").collect();
        match auth_header.len() {
            0 => Outcome::Failure((Status::Unauthorized, LoginError::MissingAuth)),
            1 => authorize(auth_header[0], request),
            _ => Outcome::Failure((Status::BadRequest, LoginError::BadHeaderCount)),
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
fn authorize(auth_header: &str, request: &Request) -> Outcome<LoginAuth, LoginError> {
    // Parse username and password from auth header
    let creds: Vec<&str> = auth_header.split(':').collect();

    let username = match creds.get(0) {
        Some(u) => *u,
        None => return Outcome::Failure((Status::Unauthorized, LoginError::MissingAuth)),
    };
    let password = match creds.get(1) {
        Some(p) => *p,
        None => return Outcome::Failure((Status::Unauthorized, LoginError::MissingAuth)),
    };

    // Get database
    let db = request
        .guard::<State<Database>>()
        .expect("No managed database connection");
    // Get user

    let query = json! {{
        "username": username
    }};

    let user = db.find_one::<User>("users", query);

    let user = match user {
        Ok(Some(u)) => u,
        Ok(None) => return Outcome::Failure((Status::Unauthorized, LoginError::NoUser)),
        Err(_) => return Outcome::Failure((Status::ServiceUnavailable, LoginError::DBError)),
    };

    // Hash password
    let encoded_hash = hash(&user.salt, &password);

    // Compare
    if encoded_hash == user.password_hash {
        Outcome::Success(LoginAuth(user))
    } else {
        Outcome::Failure((Status::Unauthorized, LoginError::WrongPassword))
    }
}
