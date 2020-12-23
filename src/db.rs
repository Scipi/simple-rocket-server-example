use mongodb::sync::Client;
use std::ops::Deref;

pub struct DBClient(Client);

impl DBClient {
    pub fn init(uri: String) -> Self {
        DBClient {
            0: Client::with_uri_str(&uri)
                .unwrap_or_else(|_| panic!("Could not connect to: {}", uri)),
        }
    }
}

impl Deref for DBClient {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
