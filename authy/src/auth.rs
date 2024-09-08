use std::collections::BTreeMap;
use std::fmt::format;
use actix_web::{App, HttpRequest};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use crate::config::AppConfig;
use crate::error::ApiError;
use crate::models::user::User;

pub fn check_in_list(uri: &str, whitelist: Vec<String>) -> bool {
    for entry in whitelist {
        if uri.starts_with::<&String>(&entry) {
            return true
        }
    }
    return false
}

pub fn create_jwt(user: &User, secret: String) -> Result<String, ApiError> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes())?;
    let mut claims = BTreeMap::new();
    claims.insert("userId", format!("{}", user.id));
    claims.insert("userRoles", user.roles.join(";"));
    claims.sign_with_key(&key).map_err(|x|ApiError::InternalServerError)
}

pub fn get_user_claims(req: &HttpRequest, secret: String) -> Result<Vec<(&str, String)>, ApiError> {
    let cookie_option = req.cookie("session");
    if cookie_option.is_none() {
        return Err(ApiError::Unauthorized)
    }
    let cookie = cookie_option.unwrap();
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes())?;
    let claims: BTreeMap<String, String> = cookie.value().verify_with_key(&key).map_err(|e| ApiError::BadRequest)?;
    if !claims.contains_key("userId") || !claims.contains_key("userRoles") {
        return Err(ApiError::BadRequest)
    }
    Ok(vec![
        ("X-CodeCanvas-UserId", claims.get("userId").unwrap().to_string()),
        ("X-CodeCanvas-UserRoles", claims.get("userRoles").unwrap().to_string())
    ])
}