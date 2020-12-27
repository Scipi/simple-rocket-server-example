use crate::db::Database;
use common::user::{SignupUser, User, UserBrief};
use rocket::http::Status;
use rocket::post;
use rocket::request::State;
use rocket_contrib::json::Json;

#[post("/signup", data = "<data>")]
pub fn signup_endpoint(
    data: Json<SignupUser>,
    db: State<Database>,
) -> Result<Json<UserBrief>, Status> {
    let user = User::from(data.into_inner());
    match db.get_user(&user.username) {
        Ok(Some(_)) => Err(Status::PreconditionFailed),
        Ok(None) => match db.insert_user(&user) {
            Ok(user) => Ok(Json(UserBrief::from(user))),
            Err(_) => Err(Status::InternalServerError),
        },
        Err(_) => Err(Status::InternalServerError),
    }
}
