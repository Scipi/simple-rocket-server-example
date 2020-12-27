use crate::security;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::convert::From;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub salt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    #[serde(with = "crate::datetime")]
    pub last_login: DateTime<Utc>,
    #[serde(with = "crate::datetime")]
    pub created: DateTime<Utc>,
    #[serde(with = "crate::datetime")]
    pub updated: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UserBrief {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    #[serde(with = "crate::datetime")]
    pub last_login: DateTime<Utc>,
    #[serde(with = "crate::datetime")]
    pub created: DateTime<Utc>,
    #[serde(with = "crate::datetime")]
    pub updated: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SignupUser {
    pub username: String,
    pub password: String,
    pub email: String,
}

impl User {
    pub fn new(email: &str, username: &str, password: &str) -> User {
        let salt = security::generate_salt(64);
        let hash = security::hash(&salt, password);

        let now: DateTime<Utc> = Utc::now();

        User {
            id: None,
            email: String::from(email),
            username: String::from(username),
            password_hash: hash,
            salt,
            auth_token: None,
            last_login: now,
            created: now,
            updated: now,
        }
    }
}

impl From<SignupUser> for User {
    fn from(data: SignupUser) -> Self {
        Self::new(&data.email, &data.username, &data.password)
    }
}

impl From<User> for UserBrief {
    fn from(data: User) -> Self {
        UserBrief {
            id: data.id,
            username: data.username,
            email: data.email,
            auth_token: data.auth_token,
            last_login: data.last_login,
            created: data.created,
            updated: data.updated,
        }
    }
}
