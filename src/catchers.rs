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
