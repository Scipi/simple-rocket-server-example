use api::db::Database;
use common::user::SignupUser;
use rocket::http::{ContentType, Cookie, Header, Status};
use rocket::local::Client;
use std::ops::Deref;
use std::ops::Drop;

pub struct TestClient(Client);

pub fn setup() -> TestClient {
    let rocket = api::build_rocket();
    TestClient(Client::new(rocket).expect("Invalid rocket instance"))
}

pub fn setup_untracked() -> TestClient {
    let rocket = api::build_rocket();
    TestClient(Client::untracked(rocket).expect("Invalid rocket instance"))
}

pub fn setup_mock_user(client: &TestClient) {
    let signup = SignupUser {
        email: "foo@example.com".into(),
        username: "foo".into(),
        password: "password1234".into(),
    };
    let response = client
        .post("/signup")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&signup).unwrap())
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}

pub fn get_mock_user_auth_token(client: &TestClient) -> Cookie<'static> {
    let response = client
        .post("/login")
        .header(ContentType::JSON)
        .header(Header::new("Authorization", "foo:password1234"))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);

    response.cookies()[0].clone().into_owned()
}

impl Deref for TestClient {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for TestClient {
    fn drop(&mut self) {
        let db = self
            .rocket()
            .state::<Database>()
            .expect("Failed to fetch db for cleanup");
        db.to_inner().drop(None).expect("Failed to drop db");
    }
}
