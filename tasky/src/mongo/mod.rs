use futures::StreamExt;

pub mod task_file;
pub mod test_file;

use bson::Bson;
use mongodb::{options::ClientOptions, Client, Cursor, Database};
use serde::de::DeserializeOwned;

use crate::util::config::AppConfig;

/// Connects to mongo DB database
pub async fn connect(config: AppConfig) -> Database {
    let client_uri = format!(
        "mongodb://{}:{}@{}/{}?retryWrites=true&w=majority",
        config.mongo_username, config.mongo_password, config.mongo_host, config.mongo_database
    );
    let options = ClientOptions::parse_async(&client_uri)
        .await
        .expect("Cannot build mongoDB client options");
    Client::with_options(options)
        .expect("Cannot connect to mongoDB")
        .database(&config.mongo_database)
}

/// Reads a mongo cursor to a Vec
async fn read_cursor<T: DeserializeOwned>(mut cursor: Cursor<Bson>) -> Vec<T> {
    let mut results: Vec<T> = vec![];
    while let Some(result) = cursor.next().await {
        if let Ok(document) = result {
            results.push(bson::from_bson(document).unwrap());
        }
    }
    results
}
