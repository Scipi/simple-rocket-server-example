use crate::db::{Database, DatabaseAccess};
use common::user::{SignupUser, User, UserBrief};
use rocket::http::Status;
use rocket::post;
use rocket::request::State;
use rocket_contrib::json;
use rocket_contrib::json::Json;

#[post("/signup", data = "<data>")]
pub fn signup_endpoint(
    data: Json<SignupUser>,
    db: State<Database>,
) -> Result<Json<UserBrief>, Status> {
    let user = User::from(data.into_inner());

    let query = json! {{
        "username": user.username
    }};

    match db.find_one::<User>("users", &query) {
        Ok(Some(_)) => Err(Status::PreconditionFailed),
        Ok(None) => match db.insert_one("users", &user) {
            Ok(user) => Ok(Json(UserBrief::from(user))),
            Err(_) => Err(Status::InternalServerError),
        },
        Err(_) => Err(Status::InternalServerError),
    }
}
