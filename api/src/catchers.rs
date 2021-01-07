//! This module specifies catchers for returning status code responses
//! as JSON instead of HTML

use rocket::catch;
use rocket_contrib::json;
use rocket_contrib::json::JsonValue;

#[catch(404)]
pub fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "status_code": 404,
        "message": "Resource was not found"
    })
}

#[catch(500)]
pub fn internal_server_error() -> JsonValue {
    json!({
        "status": "error",
        "status_code": 500,
        "message": "The server encountered an internal error while processing this request"
    })
}

#[catch(401)]
pub fn unauthorized() -> JsonValue {
    json!({
        "status": "error",
        "status_code": 401,
        "message": "The request requires user authentication"
    })
}
