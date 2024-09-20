use diesel::Connection;
use diesel::prelude::PgConnection;
use crate::util::config::AppConfig;

pub fn establish_connection(config: AppConfig) -> PgConnection {
    let db_uri = format!("postgres://{}:{}@{}/{}", config.db_username, config.db_password, config.db_host, config.db_name);
    PgConnection::establish(&*db_uri)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_uri))
}