use crate::db::err::DBError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("No auth header was provided in the request")]
    MissingAuth,

    #[error("No auth_token cookie was provided in the request")]
    MissingToken,

    #[error("No user was found: {0}")]
    NoUser(String),

    #[error("An invalid token was provided in the request")]
    BadToken,

    #[error("An incorrect password was used for user: {0}")]
    WrongPassword(String),

    #[error("Multiple Authorization headers were found in the request")]
    BadHeaderCount,

    #[error("An issue occurred with the db: {source:?}")]
    DBError {
        #[from]
        source: DBError,
    },

    #[error("...")]
    Unspecified,
}
