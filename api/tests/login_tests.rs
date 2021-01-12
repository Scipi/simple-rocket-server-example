use rocket::http::{ContentType, Header, Status};

mod common;

#[test]
fn test_basic_login() {
    let client = common::setup();
    common::setup_mock_user(&client);
    let response = client
        .post("/login")
        .header(ContentType::JSON)
        .header(Header::new("Authorization", "foo:password1234"))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);

    // Check auth_token cookie is set
    assert_eq!(response.cookies().len(), 1);
    assert_eq!(response.cookies()[0].name(), "auth_token");
}

#[test]
fn test_missing_auth() {
    let client = common::setup();
    common::setup_mock_user(&client);
    let response = client
        .post("/login")
        .header(ContentType::JSON)
        // .header(Header::new("Authorization", "foo:password1234"))
        .dispatch();
    assert_eq!(response.status(), Status::Unauthorized);
}

#[test]
fn test_empty_auth() {
    let client = common::setup();
    common::setup_mock_user(&client);
    let response = client
        .post("/login")
        .header(ContentType::JSON)
        .header(Header::new("Authorization", ""))
        .dispatch();
    assert_eq!(response.status(), Status::Unauthorized);
}

#[test]
fn test_wrong_password() {
    let client = common::setup();
    common::setup_mock_user(&client);
    let response = client
        .post("/login")
        .header(ContentType::JSON)
        .header(Header::new("Authorization", "foo:wrong!"))
        .dispatch();
    assert_eq!(response.status(), Status::Unauthorized);
}

#[test]
fn test_wrong_username() {
    let client = common::setup();
    common::setup_mock_user(&client);
    let response = client
        .post("/login")
        .header(ContentType::JSON)
        .header(Header::new("Authorization", "wrong!:password1234"))
        .dispatch();
    assert_eq!(response.status(), Status::Unauthorized);
}

#[test]
fn test_token_auth() {
    let client = common::setup_untracked();
    common::setup_mock_user(&client);
    let auth_cookie = common::get_mock_user_auth_token(&client);

    let response = client
        .get("/self")
        .header(ContentType::JSON)
        .cookie(auth_cookie)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_token_auth_tracked() {
    let client = common::setup();
    common::setup_mock_user(&client);
    let _ = common::get_mock_user_auth_token(&client);

    let response = client
        .get("/self")
        .header(ContentType::JSON)
        // .cookie(auth_cookie)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_missing_cookie() {
    let client = common::setup_untracked();
    common::setup_mock_user(&client);
    let _ = common::get_mock_user_auth_token(&client);

    let response = client
        .get("/self")
        .header(ContentType::JSON)
        // .cookie(auth_cookie)
        .dispatch();
    assert_eq!(response.status(), Status::Unauthorized);
}

/// Tests when a cookie is provided before a user is given an auth token
#[test]
fn test_bad_cookie() {
    let client = common::setup_untracked();
    common::setup_mock_user(&client);
    // let auth_cookie = common::get_mock_user_auth_token(&client);

    let response = client
        .get("/self")
        .header(ContentType::JSON)
        // .cookie(auth_cookie)
        .dispatch();
    assert_eq!(response.status(), Status::Unauthorized);
}

/// Tests when a cookie is modified after receiving one from the server
#[test]
fn test_tampered_cookie() {
    let client = common::setup_untracked();
    common::setup_mock_user(&client);
    let mut auth_cookie = common::get_mock_user_auth_token(&client);

    let mut tampered_value: String = auth_cookie.value().into();
    tampered_value.push_str("foo");
    auth_cookie.set_value(tampered_value);

    let response = client
        .get("/self")
        .header(ContentType::JSON)
        .cookie(auth_cookie)
        .dispatch();
    assert_eq!(response.status(), Status::Unauthorized);
}
