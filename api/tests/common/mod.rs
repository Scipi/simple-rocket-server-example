use api::db::Database;
use rocket::local::Client;
use std::ops::Deref;
use std::ops::Drop;

pub struct TestClient(Client);

pub fn setup() -> TestClient {
    let rocket = api::build_rocket();
    TestClient(Client::new(rocket).expect("Invalid rocket instance"))
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
