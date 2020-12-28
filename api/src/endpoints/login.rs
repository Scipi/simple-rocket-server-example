use crate::auth::login_auth::LoginAuth;
use crate::db::{DBError, Database};
use common::user::UserBrief;
use rocket::http::Status;
use rocket::post;
use rocket::request::State;
use rocket_contrib::json;
use rocket_contrib::json::Json;

#[post("/login")]
pub fn login_endpoint(db: State<Database>, login: LoginAuth) -> Result<Json<UserBrief>, Status> {
    let user = login.into_inner();

    let query = json! {{
        "_id": user.id,
    }};

    let update = json! {{
        "$set": {
            "auth_token": "foo"
        }
    }};

    match db.update_one("users", query.clone(), update) {
        Ok(()) => match db.find_one::<UserBrief>("users", query) {
            Ok(Some(user)) => Ok(Json(user)),
            Ok(None) => Err(Status::NotFound),
            Err(DBError::MongoError(_)) => Err(Status::ServiceUnavailable),
            _ => Err(Status::InternalServerError),
        },
        Err(DBError::MongoError(_)) => Err(Status::ServiceUnavailable),
        _ => Err(Status::InternalServerError),
    }
}
