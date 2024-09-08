use std::fmt::format;
use actix_web::{HttpRequest, HttpResponse, web};
use actix_web::body::MessageBody;
use actix_web::cookie::Cookie;
use actix_web::dev::ResourcePath;
use awc::body::to_bytes;
use crate::error::ApiError;
use crate::http::ProxyClient;
use crate::{auth, State};
use crate::config::AppConfig;
use crate::models::user::User;

/// Handles all requests in order to filter out whitelisted ones and authenticates the rest
pub async fn handle_proxy(
    req: HttpRequest,
    state: web::Data<State>,
    body: web::Payload
) -> Result<HttpResponse, ApiError> {
    let path = req.path();
    let config = &state.as_ref().config;
    let proxy = ProxyClient::new(config.clone());

    if path == config.login_uri.path() {
        let resp =  proxy.proxy_request(&req, body, vec![]).await?;
        return handle_login_request(resp, config.clone()).await;
    }

    if auth::check_in_list(path.clone(), config.whitelist.clone()) {
        let resp =  proxy.proxy_request(&req, body, vec![]).await?;
        return Ok(resp);
    }

    if auth::check_in_list(path.clone(), config.blacklist.clone()) {
        return ApiError::Forbidden.into();
    }
    let claims = auth::get_user_claims(&req, config.jwt_secret.clone())?;
    let resp =  proxy.proxy_request(&req, body, claims).await?;
    return Ok(resp);
}

async fn handle_login_request(resp: HttpResponse, config: AppConfig) -> Result<HttpResponse, ApiError> {
    let body = resp.into_body();
    let bytes = to_bytes(body).await.map_err(|x| ApiError::Forbidden)?;
    let json_string = String::from_utf8_lossy(bytes.as_ref());
    println!("{}", json_string);
    let user: User = serde_json::from_str(json_string.as_ref())
        .map_err(|e| ApiError::InternalServerError)?;
    let jwt = auth::create_jwt(&user, config.jwt_secret.clone())?;
    let mut mod_resp =  HttpResponse::Ok().body("");
    let cookie = Cookie::new("session", jwt);
    mod_resp.add_cookie(&cookie).map_err(|e|ApiError::InternalServerError)?;
    return Ok(mod_resp);
}