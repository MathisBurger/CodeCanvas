use actix_web::web::Data;
use actix_web::{middleware, App, HttpServer};
use futures::future::join;
use log::info;
use std::net::SocketAddr;
use tasky::api::usernator_api_client::UsernatorApiClient;
use tasky::auth_middleware::Auth;
use tasky::grpc::MyTaskyApi;
use tasky::models::database::Database;
use tasky::mongo;
use tasky::routes::init_services;
use tasky::tasky_grpc::tasky_api_server::TaskyApiServer;
use tasky::util::config::AppConfig;
use tasky::AppState;
use tonic::transport::Server;

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

    let grpc_sock_addr: SocketAddr = "0.0.0.0:3001".parse().unwrap();

    let grpc = async move {
        tokio::task::spawn(
            Server::builder()
                .add_service(TaskyApiServer::new(tasky_api))
                .serve(grpc_sock_addr),
        )
        .await
    };

    let actix = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(Auth::new())
            .app_data(Data::new(state.clone()))
            .configure(init_services)
    })
    .bind("0.0.0.0:3000")
    .expect("Already in use")
    .run();

    let _ret = join(grpc, actix).await;

    Ok(())
}
