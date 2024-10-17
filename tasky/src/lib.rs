use crate::api::usernator_api_client::UsernatorApiClient;
use crate::grpc::MyTaskyApi;
use crate::models::database::Database;
use crate::util::config::AppConfig;
use log::info;
use tonic::transport::Channel;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub db: Database,
    pub mongodb: mongodb::Database,
    pub user_api: UsernatorApiClient<Channel>,
}

pub mod auth_middleware;
pub mod error;
pub mod grpc;
pub mod handler;
pub mod http;
pub mod models;
pub mod mongo;
pub mod response;
pub mod routes;
pub mod schema;
pub mod security;
pub mod util;

pub mod api {
    tonic::include_proto!("api");
}

pub mod tasky_grpc {
    tonic::include_proto!("tasky_grpc");
}

pub async fn get_states() -> (AppState, MyTaskyApi) {
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
    let usernator = UsernatorApiClient::connect(user_api_uri)
        .await
        .expect("Cannot create tonic client");

    let mongodb = mongo::connect(config.clone()).await;

    let state = AppState {
        config: config.clone(),
        db,
        mongodb,
        user_api: usernator,
    };

    let tasky_api = MyTaskyApi {
        app_state: state.clone(),
    };

    (state, tasky_api)
}
