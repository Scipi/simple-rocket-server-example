use bson;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Option<bson::oid::ObjectId>,
    pub username: String,
    pub password_hash: String,
    pub salt: String,
    pub auth_token: Option<String>,
    #[serde(with = "crate::datetime")]
    pub last_login: DateTime<Utc>,
    #[serde(with = "crate::datetime")]
    pub created: DateTime<Utc>,
    #[serde(with = "crate::datetime")]
    pub updated: DateTime<Utc>,
}
