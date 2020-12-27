use common::user::User;
use log::{error, info};
use mongodb::bson::{self, doc, Bson};
use mongodb::error::Error as MongoError;
use mongodb::sync::{Client, Database as MongoDatabase};
use std::ops::Deref;

#[derive(Debug)]
pub enum DBError {
    Unknown,
    MongoError(MongoError),
}

pub struct DBClient(Client);
pub struct Database(MongoDatabase);

impl DBClient {
    pub fn init(uri: &str) -> Self {
        info!(target: "Database", "Creating database client to {}", uri);
        DBClient {
            0: Client::with_uri_str(uri).unwrap_or_else(|_| panic!("Invalid mongodb uri: {}", uri)),
        }
    }
    pub fn get_app_database(&self, name: &str) -> Database {
        info! {target: "Database", "Creating database connection {}", name};
        Database {
            0: self.database(name),
        }
    }
}

impl Deref for DBClient {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for Database {
    type Target = MongoDatabase;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Database {
    pub fn get_user(&self, username: &str) -> Result<Option<User>, DBError> {
        let users = self.collection("users");

        match users.find_one(doc! {"username": username }, None) {
            Ok(Some(user)) => {
                let u: User =
                    bson::from_bson(Bson::Document(user)).expect("Error parsing document");
                Ok(Some(u))
            }
            Ok(None) => Ok(None),
            Err(e) => {
                error! {target: "DB", "Could not read from database: {:?}", e};
                Err(DBError::MongoError(e))
            }
        }
    }

    pub fn insert_user(&self, user: &User) -> Result<User, DBError> {
        return if let Ok(user_bson) = bson::to_bson(&user) {
            let users = self.collection("users");

            if let Some(user_bson) = user_bson.as_document() {
                let result = users.insert_one(user_bson.clone(), None);

                match result {
                    Ok(result) => {
                        let mut user: User = user.clone();
                        if let bson::Bson::ObjectId(id) = result.inserted_id {
                            user.id = Some(id);
                            Ok(user)
                        } else {
                            Err(DBError::Unknown)
                        }
                    }
                    Err(e) => {
                        error!(target: "DB", "Could not insert to database: {:?}", e);
                        Err(DBError::MongoError(e))
                    }
                }
            } else {
                error!(target: "app", "Could not convert bson to document {:?}", user_bson);
                Err(DBError::Unknown)
            }
        } else {
            error!(target: "app", "Could not create bson from user {:?}", user);
            Err(DBError::Unknown)
        };
    }
}
