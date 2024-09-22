use crate::auth_middleware::Auth;
use crate::models::database::Database;
use crate::routes::init_services;
use crate::util::config::AppConfig;
use actix_web::web::Data;
use actix_web::{middleware, App, HttpServer};
use api::usernator_api_client::UsernatorApiClient;
use log::info;
use tonic::transport::Channel;

pub mod api {
    tonic::include_proto!("api");
}

mod auth_middleware;
mod error;
mod models;
mod response;
mod routes;
mod schema;
mod security;
mod util;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub db: Database,
    pub user_api: UsernatorApiClient<Channel>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
    let db = Database::new(config.clone());
    let user_api_uri = config.clone().usernator_grpc;
    info!(target: "startup", "{}", format!("Connecting to usernator: {}", user_api_uri));
    let mut usernator = UsernatorApiClient::connect(user_api_uri)
        .await
        .expect("Cannot create tonic client");

    let state = AppState {
        config: config.clone(),
        db,
        user_api: usernator,
    };

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(Auth::new())
            .app_data(Data::new(state.clone()))
            .configure(init_services)
    })
    .bind("0.0.0.0:3000")
    .expect("Already in use")
    .run()
    .await
}
