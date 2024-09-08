use actix_web::{HttpRequest, HttpResponse, web};
use actix_web::dev::ResourcePath;
use crate::error::ApiError;
use crate::http::ProxyClient;
use crate::{auth, State};

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
        let mut resp =  proxy.proxy_request(&req, body).await?;
        // TODO: Add JWT to response
        return Ok(resp);
    }

    if auth::check_in_list(path.clone(), config.whitelist.clone()) {
        let resp =  proxy.proxy_request(&req, body).await?;
        return Ok(resp);
    }

    if auth::check_in_list(path.clone(), config.blacklist.clone()) {
        return ApiError::Forbidden.into();
    }
    // TODO: Get roles and userId and cache it

    let resp =  proxy.proxy_request(&req, body).await?;
    return Ok(resp);
}