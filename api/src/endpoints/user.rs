//! This module contains endpoints relating to user account management

use crate::auth::login_auth::LoginAuth;
use crate::auth::token_auth::TokenAuth;
use crate::db::{Database, DatabaseAccess};
use common::security;
use common::user::{UpdateUser, UpdateUserPassword, UserBrief};
use rocket::http::{Cookie, Cookies, Status};
use rocket::response::Redirect;
use rocket::{get, patch, State};
use rocket_contrib::json;
use rocket_contrib::json::Json;

/// Fetch the logged in account (specified by the auth token)
///
/// Example:
/// `GET /self`
///
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
#[get("/self")]
pub fn self_endpoint(token_auth: TokenAuth) -> Result<Json<UserBrief>, Status> {
    Ok(Json(token_auth.into_inner().into()))
}

#[patch("/self", data = "<data>")]
pub fn update_user_endpoint(
    data: Json<UpdateUser>,
    db: State<Database>,
    token_auth: TokenAuth,
) -> Result<Json<UserBrief>, Status> {
    let data = data.into_inner();
    let query = json! {{
        "_id": token_auth.into_inner().id,
    }};

    let update = json! {{
        "$set": data,
    }};

    db.update_one("users", &query, &update)?;

    match db.find_one::<UserBrief>("users", &query)? {
        Some(user) => Ok(Json(user)),
        None => Err(Status::NotFound),
    }
}

#[patch("/self/password", data = "<data>")]
pub fn update_user_password_endpoint(
    data: Json<UpdateUserPassword>,
    db: State<Database>,
    auth: LoginAuth,
    mut cookies: Cookies,
) -> Result<Redirect, Status> {
    let data = data.into_inner();

    let salt = security::generate_salt(256);
    let password_hash = security::hash(&salt, &data.password);

    let query = json! {{
        "_id": auth.into_inner().id,
    }};

    let update = json! {{
        "$set": {
            "salt": salt,
            "password_hash": password_hash,
        },
        "$unset": {
            "auth_token": 1,
        }
    }};

    db.update_one("users", &query, &update)?;

    cookies.remove_private(Cookie::named("auth_token"));

    match db.find_one::<UserBrief>("users", &query)? {
        Some(_) => Ok(Redirect::to("/")),
        None => Err(Status::NotFound),
    }
}
