use common::user::User;
use log::{error, info};
use mongodb::bson::ser::Error as BsonSerdeError;
use mongodb::bson::{self, de::Error as BsonError, doc, Bson};
use mongodb::error::Error as MongoError;
use mongodb::sync::{Client, Database as MongoDatabase};
use rocket_contrib::json::JsonValue;
use std::convert::From;
use std::ops::Deref;

#[derive(Debug)]
pub enum DBError {
    Unknown,
    MongoError(MongoError),
    BsonError(BsonError),
    BsonSerdeError(BsonSerdeError),
    BsonDocumentError,
}

impl From<MongoError> for DBError {
    fn from(e: MongoError) -> Self {
        Self::MongoError(e)
    }
}

impl From<BsonError> for DBError {
    fn from(e: BsonError) -> Self {
        Self::BsonError(e)
    }
}

impl From<BsonSerdeError> for DBError {
    fn from(e: BsonSerdeError) -> Self {
        Self::BsonSerdeError(e)
    }
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

    pub fn insert_one<T>(&self, collection: &str, item: &T) -> Result<T, DBError>
    where
        T: serde::Serialize + serde::de::DeserializeOwned,
    {
        let mut user_bson = bson::to_bson(&item)?;

        let users = self.collection(collection);

        let user_bson = user_bson
            .as_document_mut()
            .ok_or(DBError::BsonDocumentError)?;

        let result = users.insert_one(user_bson.clone(), None)?;
        match result.inserted_id {
            bson::Bson::ObjectId(id) => {
                user_bson.insert("_id", id);
                let item = bson::from_bson::<T>(Bson::Document(user_bson.clone()))?;
                Ok(item)
            }
            _ => Err(DBError::Unknown),
        }
    }

    pub fn update_one(
        &self,
        collection: &str,
        query: JsonValue,
        update: JsonValue,
    ) -> Result<(), DBError> {
        let collection = self.collection(collection);

        let _ = collection.update_one(
            bson::to_bson(&query)?
                .as_document()
                .ok_or(DBError::BsonDocumentError)?
                .clone(),
            bson::to_bson(&update)?
                .as_document()
                .ok_or(DBError::BsonDocumentError)?
                .clone(),
            None,
        )?;

        Ok(())
    }
}
