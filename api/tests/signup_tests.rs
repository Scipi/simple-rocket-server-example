use api::common::user::{SignupUser, UserBrief};
use chrono::Utc;
use rocket::http::{ContentType, Status};

mod common;

#[test]
fn test_signup() {
    let client = common::setup();

    let signup = SignupUser {
        email: "scipii48@gmail.com".into(),
        username: "scipi".into(),
        password: "password1234".into(),
    };

    let mut expected = UserBrief {
        id: None,
        email: "scipii48@gmail.com".into(),
        username: "scipi".into(),
        created: Utc::now(),
        updated: Utc::now(),
        last_login: Utc::now(),
        auth_token: None,
    };

    let mut response = client
        .post("/signup")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&signup).unwrap())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let result: UserBrief = serde_json::from_str(
        &response
            .body_string()
            .expect("Could not convert body to string"),
    )
    .expect("Could not deserialize response body");

    // Set values we cannot predict in this test
    expected.id = result.id.clone();
    expected.created = result.created;
    expected.updated = result.updated;
    expected.last_login = result.last_login;

    assert_eq!(result, expected);
    assert_ne!(result.id, None);
}

#[test]
fn test_signup_same_username() {
    let client = common::setup();

    let signup = SignupUser {
        email: "scipii48@gmail.com".into(),
        username: "scipi".into(),
        password: "password1234".into(),
    };

    let response = client
        .post("/signup")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&signup).unwrap())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let response = client
        .post("/signup")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&signup).unwrap())
        .dispatch();

    assert_eq!(response.status(), Status::PreconditionFailed);
}

#[test]
fn test_multi_signup() {
    let client = common::setup();

    let signup_0 = SignupUser {
        email: "scipii48@gmail.com".into(),
        username: "scipi".into(),
        password: "password1234".into(),
    };
    let signup_1 = SignupUser {
        email: "scipii48@gmail.com".into(),
        username: "scipi_2".into(),
        password: "password1234".into(),
    };

    let response = client
        .post("/signup")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&signup_0).unwrap())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let response = client
        .post("/signup")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&signup_1).unwrap())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
}
