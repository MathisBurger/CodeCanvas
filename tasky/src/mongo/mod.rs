use futures::StreamExt;

pub mod task_file;
pub mod test_file;

use bson::Bson;
use mongodb::{options::ClientOptions, Client, Cursor, Database};
use serde::de::DeserializeOwned;

use crate::util::config::AppConfig;

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

async fn read_cursor<T: DeserializeOwned>(mut cursor: Cursor<Bson>) -> Vec<T> {
    let mut results: Vec<T> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                results.push(bson::from_bson(document).unwrap());
            }
            _ => {}
        }
    }
    results
}
