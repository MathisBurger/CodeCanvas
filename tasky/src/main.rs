use log::info;
use crate::util::config::AppConfig;
use crate::util::database::establish_connection;

mod util;
mod schema;
mod models;

fn main() {
    let log_level = std::env::var("RUST_LOG").unwrap_or_else(|_e| "info".to_string());

    std::env::set_var("RUST_LOG", log_level);
    pretty_env_logger::init();
    info!(target: "startup", "");

    let config = match AppConfig::parse() {
        Ok(config) => config,
        Err(err) => {
            println!("Could not read config");
            println!("Error: {:?}", err);
            std::process::exit(2);
        }
    };

    let db_conn = &mut establish_connection(config.clone());
}
