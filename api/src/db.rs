use log::info;
use mongodb::sync::{Client, Database};
use std::ops::Deref;

pub struct DBClient(Client);
pub struct AppDatabase(Database);

impl DBClient {
    pub fn init(uri: &str) -> Self {
        info!(target: "Database", "Creating database client to {}", uri);
        DBClient {
            0: Client::with_uri_str(uri).unwrap_or_else(|_| panic!("Invalid mongodb uri: {}", uri)),
        }
    }
    pub fn get_app_database(&self, name: &str) -> AppDatabase {
        info! {target: "Database", "Creating database connection {}", name};
        AppDatabase {
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

impl Deref for AppDatabase {
    type Target = Database;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
