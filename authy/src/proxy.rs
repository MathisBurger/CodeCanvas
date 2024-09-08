use actix_web::{HttpRequest, HttpResponse, web};
use crate::error::ApiError;
use crate::http::ProxyClient;
use crate::State;

/// Handles all requests in order to filter out whitelisted ones and authenticates the rest
pub async fn handle_proxy(
    req: HttpRequest,
    state: web::Data<State>,
    body: web::Payload
) -> Result<HttpResponse, ApiError> {
    let path = req.path();
    let config = &state.as_ref().config;
    // TODO: check auth whitelist
    // TODO: check auth blacklist
    // TODO: Handle JWT auth with /usernator/login
    // TODO: Get roles and userId
    let proxy = ProxyClient::new(config.clone());
    let resp =  proxy.proxy_request(&req, body).await?;

    // TODO: set cookie on session
    return Ok(resp);
}