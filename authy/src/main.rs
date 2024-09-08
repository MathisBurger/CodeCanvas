mod config;
mod proxy;
mod error;
mod http;
mod auth;
mod models;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use actix_web::{App, HttpServer, web};
use actix_web::web::Data;
use crate::config::AppConfig;

#[derive(Clone)]
pub struct State {
    pub config: AppConfig,
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
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
    let locations: Vec<String> = config.clone().service_locations.iter().map(|x|format!("{}={}", x.0, x.1)).collect();
    println!("{}", locations.join("-"));
    let state = State {
        config: config.clone(),
    };
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .default_service(web::to(proxy::handle_proxy))
    })
        .bind("0.0.0.0:3000")
        .expect("Already in use")
        .run()
        .await
}
