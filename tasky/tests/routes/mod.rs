use actix_http::Request;
use actix_web::dev::{Service, ServiceResponse};
use actix_web::http::header::ContentType;
use actix_web::test::TestRequest;
use actix_web::web::{Data, ReqData};
use actix_web::{middleware, test, App, Error};
use tasky::auth_middleware::Auth;
use tasky::routes::init_services;
use tasky::{
    auth_middleware::{UserData, UserRole},
    get_states, AppState,
};

pub mod assignment;
pub mod group;
pub mod group_join_request;

async fn get_app() -> impl Service<Request, Response = ServiceResponse, Error = Error> {
    let (state, _) = get_states().await;
    let app = test::init_service(
        App::new()
            .wrap(Auth::new())
            .app_data(Data::new(state.clone()))
            .configure(init_services),
    )
    .await;
    app
}

fn student(req: TestRequest) -> TestRequest {
    req.append_header(("X-CodeCanvas-UserId", "3"))
        .append_header(("X-CodeCanvas-UserRoles", "ROLE_STUDENT"))
}

fn student2(req: TestRequest) -> TestRequest {
    req.append_header(("X-CodeCanvas-UserId", "4"))
        .append_header(("X-CodeCanvas-UserRoles", "ROLE_STUDENT"))
}

fn student3(req: TestRequest) -> TestRequest {
    req.append_header(("X-CodeCanvas-UserId", "5"))
        .append_header(("X-CodeCanvas-UserRoles", "ROLE_STUDENT"))
}

fn tutor(req: TestRequest) -> TestRequest {
    req.append_header(("X-CodeCanvas-UserId", "2"))
        .append_header(("X-CodeCanvas-UserRoles", "ROLE_TUTOR"))
}

fn admin(req: TestRequest) -> TestRequest {
    req.append_header(("X-CodeCanvas-UserId", "1"))
        .append_header(("X-CodeCanvas-UserRoles", "ROLE_ADMIN"))
}
