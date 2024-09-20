use actix_web::{HttpResponse, post, Responder, web};
use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::auth_middleware::UserData;

#[derive(Deserialize, Serialize)]
struct Request {
    title: String,
}

#[post("/create_group")]
pub async fn create_group(data: web::Data<AppState>, req: web::Json<Request>, user: web::ReqData<UserData>) -> impl Responder {
    println!("{}", user.user_id);
    return HttpResponse::Ok().json(req)
}