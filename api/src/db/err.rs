use mongodb::bson::de::Error as BsonDeserializationError;
use mongodb::bson::ser::Error as BsonSerializationError;
use mongodb::error::Error as MongoError;
use rocket::http::Status;
use std::backtrace::Backtrace;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DBError {
    #[error("An unknown error occurred")]
    UnknownError { backtrace: Backtrace },
    #[error("An error originated from within MongoDB: {source:?}")]
    MongoError {
        #[from]
        source: MongoError,
        backtrace: Backtrace,
    },
    #[error("Could not deserialize BSON: {source:?}")]
    BsonDeserializationError {
        #[from]
        source: BsonDeserializationError,
        backtrace: Backtrace,
    },
    #[error("Could not serialize into BSON: {source:?}")]
    BsonSerializationError {
        #[from]
        source: BsonSerializationError,
        backtrace: Backtrace,
    },
    #[error("Could not convert BSON to Document")]
    BsonDocumentError { backtrace: Backtrace },
}

impl From<DBError> for Status {
    fn from(e: DBError) -> Status {
        match e {
            DBError::MongoError { .. } => Status::ServiceUnavailable,
            _ => Status::InternalServerError,
        }
    }
}
