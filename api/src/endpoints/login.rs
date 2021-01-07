//! This module contains the endpoints relating to logins

use crate::auth::login_auth::LoginAuth;
use crate::db::{Database, DatabaseAccess};
use common::security;
use common::user::UserBrief;
use rocket::http::{Cookie, Cookies, Status};
use rocket::post;
use rocket::request::State;
use rocket_contrib::json;
use rocket_contrib::json::Json;

/// Log in to the server using Basic Auth. This endpoint generates an
/// auth token for the user and sets it as a private cookie `auth_token`
///
/// Example:
/// `POST /login`
///
/// Body:
/// ```json
/// {}
/// ```
/// Content-type: application/json
/// Response code: 200
/// Response body:
/// ```json
/// {
///   "_id": "ObjectId",
///   "username": "Foo",
///   "email": "foo@example.com",
///   "last_login": "2020-12-31 12:00:00",
///   "created": "2020-12-31 12:00:00",
///   "updated": "2020-12-31 12:00:00",
/// }
/// ```
///
/// *Datetimes given in UTC
#[post("/login")]
pub fn login_endpoint(
    db: State<Database>,
    login: LoginAuth,
    mut cookies: Cookies,
) -> Result<Json<UserBrief>, Status> {
    let user = login.into_inner();

    let token = security::generate_auth_token(256);

    let query = json! {{
        "_id": user.id,
    }};

    let update = json! {{
        "$set": {
            "auth_token": token
        }
    }};

    let cookie = Cookie::build("auth_token", token)
        .path("/")
        .secure(true)
        .http_only(true)
        .finish();

    cookies.add_private(cookie);

    db.update_one("users", &query, &update)?;
    Ok(Json(user.into()))
}
