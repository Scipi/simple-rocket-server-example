use crate::db::err::DBError;
use log::{error, info};
use mongodb::bson::{self, Bson};
use mongodb::sync::{Client, Database as MongoDatabase};
use rocket_contrib::json::JsonValue;
use std::backtrace::Backtrace;
use std::ops::Deref;

pub mod err;

pub struct DBClient(Client);
pub struct _Database(MongoDatabase);
pub struct Database(_Database);

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
            0: _Database {
                0: self.database(name),
            },
        }
    }
}

impl Deref for DBClient {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for _Database {
    type Target = MongoDatabase;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for Database {
    type Target = _Database;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait DatabaseAccess {
    fn find_one<T>(&self, collection: &str, query: JsonValue) -> Result<Option<T>, err::DBError>
    where
        T: serde::Serialize + serde::de::DeserializeOwned;

    fn insert_one<T>(&self, collection: &str, item: &T) -> Result<T, err::DBError>
    where
        T: serde::Serialize + serde::de::DeserializeOwned;

    fn update_one(
        &self,
        collection: &str,
        query: JsonValue,
        update: JsonValue,
    ) -> Result<(), err::DBError>;
}

impl DatabaseAccess for _Database {
    fn find_one<T>(&self, collection: &str, query: JsonValue) -> Result<Option<T>, err::DBError>
    where
        T: serde::Serialize + serde::de::DeserializeOwned,
    {
        let collection = self.collection(collection);

        let item = collection.find_one(
            bson::to_bson(&query)?
                .as_document()
                .ok_or(err::DBError::BsonDocumentError {
                    backtrace: Backtrace::capture(),
                })?
                .clone(),
            None,
        )?;

        match item {
            Some(doc) => {
                let item: T = bson::from_bson(Bson::Document(doc))?;
                Ok(Some(item))
            }
            None => Ok(None),
        }
    }

    fn insert_one<T>(&self, collection: &str, item: &T) -> Result<T, err::DBError>
    where
        T: serde::Serialize + serde::de::DeserializeOwned,
    {
        let mut user_bson = bson::to_bson(&item)?;

        let collection = self.collection(collection);

        let user_bson = user_bson
            .as_document_mut()
            .ok_or(err::DBError::BsonDocumentError {
                backtrace: Backtrace::capture(),
            })?;

        let result = collection.insert_one(user_bson.clone(), None)?;
        match result.inserted_id {
            bson::Bson::ObjectId(id) => {
                user_bson.insert("_id", id);
                let item = bson::from_bson::<T>(Bson::Document(user_bson.clone()))?;
                Ok(item)
            }
            _ => Err(err::DBError::UnknownError {
                backtrace: Backtrace::capture(),
            }),
        }
    }

    fn update_one(
        &self,
        collection: &str,
        query: JsonValue,
        update: JsonValue,
    ) -> Result<(), err::DBError> {
        let collection = self.collection(collection);

        let _ = collection.update_one(
            bson::to_bson(&query)?
                .as_document()
                .ok_or(err::DBError::BsonDocumentError {
                    backtrace: Backtrace::capture(),
                })?
                .clone(),
            bson::to_bson(&update)?
                .as_document()
                .ok_or(err::DBError::BsonDocumentError {
                    backtrace: Backtrace::capture(),
                })?
                .clone(),
            None,
        )?;

        Ok(())
    }
}

impl DatabaseAccess for Database {
    fn find_one<T>(&self, collection: &str, query: JsonValue) -> Result<Option<T>, DBError>
    where
        T: serde::Serialize + serde::de::DeserializeOwned,
    {
        let result = self.0.find_one(collection, query);

        if let Err(e) = &result {
            error!("Error fetching from database {:#?}", e)
        };

        result
    }

    fn insert_one<T>(&self, collection: &str, item: &T) -> Result<T, DBError>
    where
        T: serde::Serialize + serde::de::DeserializeOwned,
    {
        let result = self.0.insert_one(collection, item);

        if let Err(e) = &result {
            error!("Error inserting to database {:#?}", e)
        };

        result
    }

    fn update_one(
        &self,
        collection: &str,
        query: JsonValue,
        update: JsonValue,
    ) -> Result<(), DBError> {
        let result = self.0.update_one(collection, query, update);

        if let Err(e) = &result {
            error!("Error fetching from database {:#?}", e)
        };

        result
    }
}
