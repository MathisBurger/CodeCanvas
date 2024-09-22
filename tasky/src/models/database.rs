use crate::util::config::AppConfig;
use diesel::prelude::PgConnection;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Database that is shared between requests
#[derive(Clone)]
pub struct Database {
    pub db: DBPool,
}

impl Database {
    /// Initializes a database connection and returns the instance of the Database struct
    pub fn new(config: AppConfig) -> Self {
        let db_uri = format!(
            "postgres://{}:{}@{}/{}",
            config.db_username, config.db_password, config.db_host, config.db_name
        );
        let manager = ConnectionManager::<PgConnection>::new(db_uri);
        let result = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Database { db: result }
    }
}
