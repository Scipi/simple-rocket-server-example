//! This module contains wrappers around MongoDB clients and databases
//! for use throughout the codebase without directly working with
//! mongodb objects

use log::{error, info};
use mongodb::bson::{self, Bson};
use mongodb::sync::{Client, Database as MongoDatabase};
use rocket_contrib::json::JsonValue;
use std::backtrace::Backtrace;

use super::err::DBError;

/// Represents a connection to a mongodb instance
pub struct DBClient(Client);

/// Represents a database
struct _Database(MongoDatabase);

/// Provides logging on database operations
pub struct Database(_Database);

impl DBClient {
    /// Returns a wrapped mongodb client connection with the given uri
    ///
    /// # Arguments
    ///
    /// * `uri` - The connection string
    ///
    /// # Examples
    ///
    /// ```
    /// use api::db::DBClient;
    /// let client = DBClient::init("mongodb://localhost:27017/");
    /// ```
    pub fn init(uri: &str) -> Self {
        info!(target: "Database", "Creating db client to {}", uri);
        DBClient {
            0: Client::with_uri_str(uri).unwrap_or_else(|_| panic!("Invalid mongodb uri: {}", uri)),
        }
    }

    /// Returns a wrapped database connection
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the database to connect to
    ///
    /// # Examples
    ///
    /// ```
    /// use api::db::DBClient;
    /// let client = DBClient::init("mongodb://localhost:27017/");
    /// let db = client.get_database("appdb");
    /// ```
    pub fn get_database(&self, name: &str) -> Database {
        info! {target: "Database", "Creating db connection {}", name};
        Database {
            0: _Database {
                0: self.0.database(name),
            },
        }
    }
}

impl Database {
    /// returns the wrapped mongodb database instance
    ///
    /// # Examples
    /// ```ignore
    /// let m_db = db.to_inner();
    /// ```
    pub fn to_inner(&self) -> &MongoDatabase {
        &self.0 .0
    }
}

pub trait DatabaseAccess {
    fn find_one<T>(&self, collection: &str, query: &JsonValue) -> Result<Option<T>, DBError>
    where
        T: serde::Serialize + serde::de::DeserializeOwned;

    fn insert_one<T>(&self, collection: &str, item: &T) -> Result<T, DBError>
    where
        T: serde::Serialize + serde::de::DeserializeOwned;

    fn update_one(
        &self,
        collection: &str,
        query: &JsonValue,
        update: &JsonValue,
    ) -> Result<(), DBError>;
}

impl DatabaseAccess for _Database {
    fn find_one<T>(&self, collection: &str, query: &JsonValue) -> Result<Option<T>, DBError>
    where
        T: serde::Serialize + serde::de::DeserializeOwned,
    {
        let collection = self.0.collection(collection);

        let item = collection.find_one(
            bson::to_bson(&query)?
                .as_document()
                .ok_or(DBError::BsonDocumentError {
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

    fn insert_one<T>(&self, collection: &str, item: &T) -> Result<T, DBError>
    where
        T: serde::Serialize + serde::de::DeserializeOwned,
    {
        let mut user_bson = bson::to_bson(&item)?;

        let collection = self.0.collection(collection);

        let user_bson = user_bson
            .as_document_mut()
            .ok_or(DBError::BsonDocumentError {
                backtrace: Backtrace::capture(),
            })?;

        let result = collection.insert_one(user_bson.clone(), None)?;
        match result.inserted_id {
            bson::Bson::ObjectId(id) => {
                user_bson.insert("_id", id);
                let item = bson::from_bson::<T>(Bson::Document(user_bson.clone()))?;
                Ok(item)
            }
            _ => Err(DBError::UnknownError {
                backtrace: Backtrace::capture(),
            }),
        }
    }

    fn update_one(
        &self,
        collection: &str,
        query: &JsonValue,
        update: &JsonValue,
    ) -> Result<(), DBError> {
        let collection = self.0.collection(collection);

        let _ = collection.update_one(
            bson::to_bson(&query)?
                .as_document()
                .ok_or(DBError::BsonDocumentError {
                    backtrace: Backtrace::capture(),
                })?
                .clone(),
            bson::to_bson(&update)?
                .as_document()
                .ok_or(DBError::BsonDocumentError {
                    backtrace: Backtrace::capture(),
                })?
                .clone(),
            None,
        )?;

        Ok(())
    }
}

impl DatabaseAccess for Database {
    /// Fetches a single item from the database given the collection,
    /// query, and type `T: serde::Serialize + serde::de::DeserializeOwned`
    ///
    /// # Arguments
    ///
    /// * `collection` - Mongo collection to search in
    /// * `query` - Query to filter results by
    ///
    /// # Examples
    ///
    /// ```
    /// use api::db::{DBClient, DatabaseAccess};
    /// use serde::{Serialize, Deserialize};
    /// use rocket_contrib::json;
    ///
    /// #[derive(Debug, Serialize, Deserialize)]
    /// struct Person {
    ///   #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    ///   pub id: Option<mongodb::bson::oid::ObjectId>,
    ///   pub name: String,
    /// }
    ///
    /// let client = DBClient::init("mongodb://localhost:27017/");
    /// let db = client.get_database("appdb");
    ///
    /// let query = json! {{
    ///   "name": "Foo"
    /// }};
    ///
    /// let person: Option<Person> = db.find_one("people", &query).unwrap();
    /// ```
    fn find_one<T>(&self, collection: &str, query: &JsonValue) -> Result<Option<T>, DBError>
    where
        T: serde::Serialize + serde::de::DeserializeOwned,
    {
        let result = self.0.find_one(collection, query);

        if let Err(e) = &result {
            error!("Error fetching from db {:#?}", e)
        };

        result
    }

    /// Inserts a single item into the database given the collection,
    /// and type `T: serde::Serialize + serde::de::DeserializeOwned`
    ///
    /// # Arguments
    ///
    /// * `collection` - Mongo collection to insert to
    /// * `item` - Item to insert
    ///
    /// # Examples
    ///
    /// ```
    /// use api::db::{DBClient, DatabaseAccess};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Serialize, Deserialize)]
    /// struct Person {
    ///   #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    ///   pub id: Option<mongodb::bson::oid::ObjectId>,
    ///   pub name: String,
    /// }
    ///
    /// let client = DBClient::init("mongodb://localhost:27017/");
    /// let db = client.get_database("appdb");
    ///
    /// let p = Person{ id: None, name: "Foo".into() };
    ///
    /// db.insert_one("people", &p).unwrap();
    /// ```
    fn insert_one<T>(&self, collection: &str, item: &T) -> Result<T, DBError>
    where
        T: serde::Serialize + serde::de::DeserializeOwned,
    {
        let result = self.0.insert_one(collection, item);

        if let Err(e) = &result {
            error!("Error inserting to db {:#?}", e)
        };

        result
    }

    /// Updates a single item in the database given the collection,
    /// lookup query, and update
    ///
    /// # Arguments
    ///
    /// * `collection` - Mongo collection to insert to
    /// * `query` - Lookup query
    /// * `update` - Fields to update
    ///
    /// # Examples
    ///
    /// ```
    /// use api::db::{DBClient, DatabaseAccess};
    /// use serde::{Serialize, Deserialize};
    /// use rocket_contrib::json;
    ///
    /// #[derive(Debug, Serialize, Deserialize)]
    /// struct Person {
    ///   #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    ///   pub id: Option<mongodb::bson::oid::ObjectId>,
    ///   pub name: String,
    /// }
    ///
    /// let client = DBClient::init("mongodb://localhost:27017/");
    /// let db = client.get_database("appdb");
    ///
    /// let p = Person{ id: None, name: "Foo".into() };
    ///
    /// db.insert_one("people", &p).unwrap();
    ///
    /// let query = json! {{
    ///   "name": "Foo"
    /// }};
    ///
    /// let update = json! {{
    ///   "$set": {
    ///     "name": "Bar"
    ///   }
    /// }};
    ///
    /// db.update_one("people", &query, &update).unwrap();
    /// ```
    fn update_one(
        &self,
        collection: &str,
        query: &JsonValue,
        update: &JsonValue,
    ) -> Result<(), DBError> {
        let result = self.0.update_one(collection, query, update);

        if let Err(e) = &result {
            error!("Error fetching from db {:#?}", e)
        };

        result
    }
}
