use crate::db::Database;
use common::user::User;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::State;
use sha3::{Digest, Sha3_512};

struct LoginAuth(User);

#[derive(Debug)]
enum LoginError {
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

    let user = db.get_user(username);

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

/// Returns a base64-encoded SHA3-512 hash of the salt+password inputs
///
/// # Arguments
///
/// * `salt` - The salt portion of the hash input
/// * `password` - The cleartext password to hash
///
/// # Examples
///
/// ```
/// use auth::login_auth;
/// let pw_hash = login_auth::hash("salt", "password");
/// ```
pub fn hash(salt: &str, password: &str) -> String {
    let mut hasher = Sha3_512::new();

    hasher.update(salt.as_bytes());
    hasher.update(password.as_bytes());

    let result = hasher.finalize();
    let result = result.as_slice();

    base64::encode(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;
    #[test]
    fn test_hash() {
        let expected = hex!(
        "a4d53131134530f701f930e59af6d301fa350b06b762a3850535b13400685a3aea6fe190481a882c9540b1b8c00bf45044312fc125588dff349ce47b1cd3bccd"
        );

        let expected = base64::encode(expected);

        assert_eq!(hash("salt", "asdf1234"), expected);
    }
}
