use std::ops::Deref;
use actix_web::{App, HttpServer, middleware};
use actix_web::web::Data;
use diesel::{PgConnection, r2d2};
use diesel::r2d2::ConnectionManager;
use log::info;
use crate::auth_middleware::{Auth, AuthMiddleware};
use crate::util::config::AppConfig;
use crate::models::database::Database;
use crate::routes::init_services;

mod util;
mod schema;
mod models;
mod routes;
mod auth_middleware;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub db: Database
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
    let mut db = Database::new(config.clone());

    let state = AppState {
        config: config.clone(),
        db
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
