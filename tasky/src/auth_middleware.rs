use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::error::ErrorUnauthorized;
use actix_web::{web, Error, HttpMessage};
use futures::future::LocalBoxFuture;
use std::fmt::Display;
use std::future::{ready, Ready};

use crate::models::group::GroupRepository;
use crate::AppState;

/// User data retrieved from Headers
#[derive(Clone)]
pub struct UserData {
    pub user_id: i32,
    pub user_roles: Vec<UserRole>,
    pub groups: Vec<i32>,
}

/// All roles a user can have
#[derive(Clone, PartialEq)]
pub enum UserRole {
    RoleAdmin,
    RoleTutor,
    RoleStudent,
    RoleUnknown,
}

/// Parses the user role
fn parse_user_role(s: &str) -> UserRole {
    match s {
        "ROLE_ADMIN" => UserRole::RoleAdmin,
        "ROLE_TUTOR" => UserRole::RoleTutor,
        "ROLE_STUDENT" => UserRole::RoleStudent,
        _ => UserRole::RoleUnknown,
    }
}

/// Implements display for a user role
impl Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            UserRole::RoleAdmin => "ROLE_ADMIN",
            UserRole::RoleTutor => "ROLE_TUTOR",
            UserRole::RoleStudent => "ROLE_STUDENT",
            UserRole::RoleUnknown => "ROLE_UNKNOWN",
        };
        write!(f, "{}", str)
    }
}
pub struct Auth;

impl Auth {
    /// Creates a new auth middleware
    pub fn new() -> Self {
        Auth {}
    }
}

/// Implements transform for a service request
impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

/// Auth middleware
pub struct AuthMiddleware<S> {
    service: S,
}

/// Implements service trait for auth middleware
impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let mut uid = 0;
        let mut uroles: Vec<UserRole> = vec![];
        let mut app_data = req.app_data::<web::Data<AppState>>();
        let state = app_data.as_ref().unwrap();
        let conn = &mut state.db.db.get().unwrap();

        let user_id = req.headers().get("X-CodeCanvas-UserId");
        if user_id.is_some() {
            let user_id_string = user_id.unwrap().to_str().unwrap();
            let id = user_id_string.parse::<i32>();
            if id.is_err() {
                return Box::pin(async { Err(ErrorUnauthorized("No user id provided")) });
            }
            uid = id.unwrap();
        } else {
            return Box::pin(async { Err(ErrorUnauthorized("No user id provided")) });
        }

        let user_roles = req.headers().get("X-CodeCanvas-UserRoles");
        if user_roles.is_some() {
            let user_roles_string = user_roles.unwrap().to_str().unwrap();
            uroles = user_roles_string.split(";").map(parse_user_role).collect();
        } else {
            return Box::pin(async { Err(ErrorUnauthorized("No user roles provided")) });
        }

        let groups: Vec<i32> = GroupRepository::get_groups_for_member(uid, conn)
            .into_iter()
            .map(|g| g.id)
            .collect();

        req.extensions_mut().insert(UserData {
            user_id: uid,
            user_roles: uroles,
            groups,
        });

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
