use crate::api::usernator_api_client::UsernatorApiClient;
use crate::models::database::Database;
use crate::util::config::AppConfig;
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
