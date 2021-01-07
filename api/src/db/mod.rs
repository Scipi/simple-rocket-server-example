//! This module contains everything needed for database access

mod database;
pub mod err;

pub use database::{DBClient, Database, DatabaseAccess};
