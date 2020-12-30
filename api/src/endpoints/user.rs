use crate::auth::token_auth::TokenAuth;
use common::user::UserBrief;
use rocket::get;
use rocket::http::Status;
use rocket_contrib::json::Json;

#[get("/self")]
pub fn self_endpoint(token_auth: TokenAuth) -> Result<Json<UserBrief>, Status> {
    Ok(Json(token_auth.into_inner().into()))
}
