use diesel::{r2d2};
use diesel::prelude::PgConnection;
use diesel::r2d2::ConnectionManager;
use crate::util::config::AppConfig;

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;
#[derive(Clone)]
pub struct Database {
    pool: DBPool
}

impl Database {
    pub fn new(config: AppConfig) -> Self {
        let db_uri = format!("postgres://{}:{}@{}/{}", config.db_username, config.db_password, config.db_host, config.db_name);
        let manager = ConnectionManager::<PgConnection>::new(db_uri);
        let result = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Database {pool: result}
    }
}