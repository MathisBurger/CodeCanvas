mod auth;
mod config;
mod endpoint;
mod error;
mod http;
mod models;
mod proxy;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use crate::config::AppConfig;
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::web::Data;
use actix_web::{middleware, web, App, HttpServer};

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
    let locations: Vec<String> = config
        .clone()
        .service_locations
        .iter()
        .map(|x| format!("{}={}", x.0, x.1))
        .collect();
    println!("{}", locations.join("-"));
    let state = State {
        config: config.clone(),
    };
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&config.allowed_origin)
            .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
            .allowed_headers(vec![
                header::SET_COOKIE,
                header::ACCEPT,
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
            ])
            .send_wildcard()
            .supports_credentials()
            .max_age(3600);
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(Data::new(state.clone()))
            .route(
                "/report_issue",
                web::post().to(endpoint::report_bug::report_bug),
            )
            .default_service(web::to(proxy::handle_proxy))
    })
    .bind("0.0.0.0:3000")
    .expect("Already in use")
    .run()
    .await
}
