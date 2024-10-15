use crate::config::AppConfig;
use crate::error::ApiError;
use crate::http::ProxyClient;
use crate::models::user::User;
use crate::{auth, State};
use actix_web::cookie::time::Duration;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::dev::ResourcePath;
use actix_web::{web, HttpRequest, HttpResponse};
use awc::body::to_bytes;

/// Handles all requests in order to filter out whitelisted ones and authenticates the rest
pub async fn handle_proxy(
    req: HttpRequest,
    state: web::Data<State>,
    body: web::Payload,
) -> Result<HttpResponse, ApiError> {
    let path = req.path();
    let config = &state.as_ref().config;
    let proxy = ProxyClient::new(config.clone());

    if path == config.login_uri.path() {
        let resp = proxy.proxy_request(&req, body, vec![]).await?;
        return handle_login_request(resp, config.clone()).await;
    }

    if auth::check_in_list(path, config.whitelist.clone()) {
        let resp = proxy.proxy_request(&req, body, vec![]).await?;
        return Ok(resp);
    }

    if auth::check_in_list(path, config.blacklist.clone()) {
        return ApiError::Forbidden {
            message: "Endpoint is on blacklist".to_string(),
        }
        .into();
    }

    let claims = auth::get_user_claims(&req, config.jwt_secret.clone())?;
    let service_key = proxy.get_service_key(path)?;
    if auth::check_in_list(
        service_key.as_str(),
        config.admin_restricted_services.clone(),
    ) {
        if claims.get(1).unwrap().1.contains("ROLE_ADMIN") {
            let resp = proxy.proxy_request(&req, body, claims).await?;
            return Ok(resp);
        }
        return Err(ApiError::Forbidden {
            message: "You are not an admin".to_string(),
        });
    }
    let resp = proxy.proxy_request(&req, body, claims).await?;
    Ok(resp)
}

async fn handle_login_request(
    resp: HttpResponse,
    config: AppConfig,
) -> Result<HttpResponse, ApiError> {
    let body = resp.into_body();
    let bytes = to_bytes(body).await.map_err(|_x| ApiError::Forbidden {
        message: "Error parsing body".to_string(),
    })?;

    let json_string = String::from_utf8_lossy(bytes.as_ref());
    let user: User =
        serde_json::from_str(json_string.as_ref()).map_err(|_e| ApiError::InternalServerError {
            message: "Cannot deserialize user".to_string(),
        })?;

    let jwt = auth::create_jwt(&user, config.jwt_secret.clone())?;
    let mut mod_resp = HttpResponse::Ok().body("");
    let mut cookie = Cookie::new("session", jwt);

    cookie.set_path("/");
    cookie.set_max_age(Duration::days(1));
    cookie.set_http_only(false);
    cookie.set_secure(true);
    cookie.set_same_site(SameSite::Strict);

    mod_resp
        .add_cookie(&cookie)
        .map_err(|_e| ApiError::InternalServerError {
            message: "Cannot add cookie".to_string(),
        })?;
    Ok(mod_resp)
}
