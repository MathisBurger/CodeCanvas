use actix_web::middleware;
use actix_web::web::Data;
use actix_web::App;
use actix_web::HttpServer;
use futures::future::join;
use std::net::SocketAddr;
use tasky::auth_middleware::Auth;
use tasky::routes::init_services;
use tasky::tasky_grpc::tasky_api_server::TaskyApiServer;
use tonic::transport::Server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let grpc_sock_addr: SocketAddr = "0.0.0.0:3001".parse().unwrap();

    let (state, tasky_api) = tasky::get_states().await;

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
