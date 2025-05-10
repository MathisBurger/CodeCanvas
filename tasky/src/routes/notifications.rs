use actix_web::{delete, get, post, web, HttpResponse};
use chrono::NaiveDateTime;
use serde::Deserialize;

use crate::{
    auth_middleware::UserData,
    error::ApiError,
    models::{group::GroupRepository, notification::NotificationRepository},
    security::{IsGranted, SecurityAction, StaticSecurity, StaticSecurityAction},
    AppState,
};

/// Gets all notifications
#[get("/notifications")]
pub async fn get_notifiations(
    data: web::Data<AppState>,
    user: web::ReqData<UserData>,
) -> HttpResponse {
    let user_data = user.into_inner();
    let db = &mut data.db.db.get().unwrap();

    HttpResponse::Ok().json(NotificationRepository::get_notifications_for_user(
        user_data.user_id,
        db,
    ))
}

/// Deletes an user from a specific notification
#[delete("/notifications/{id}")]
pub async fn remove_user_from_notification(
    data: web::Data<AppState>,
    user: web::ReqData<UserData>,
    path: web::Path<(i32,)>,
) -> HttpResponse {
    let user_data = user.into_inner();
    let path_data = path.into_inner();
    let db = &mut data.db.db.get().unwrap();
    NotificationRepository::remove_user_from_notification(path_data.0, user_data.user_id, db);
    HttpResponse::Ok().finish()
}

/// Deletes an user from all notifications
#[delete("/notifications")]
pub async fn remove_user_from_all_notifications(
    data: web::Data<AppState>,
    user: web::ReqData<UserData>,
) -> HttpResponse {
    let user_data = user.into_inner();
    let db = &mut data.db.db.get().unwrap();
    NotificationRepository::remove_user_from_all_notification(user_data.user_id, db);
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
struct CreateNotificationRequest {
    pub title: String,
    pub content: String,
    #[serde(deserialize_with = "crate::routes::deserialize_naive_datetime")]
    pub show_until: Option<NaiveDateTime>,
}

/// Endpoint to create system wide notification
#[post("/system_wide_notifications")]
pub async fn create_system_wide_notifications(
    data: web::Data<AppState>,
    user: web::ReqData<UserData>,
    body: web::Json<CreateNotificationRequest>,
) -> Result<HttpResponse, ApiError> {
    let conn = &mut data.db.db.get().unwrap();

    if !StaticSecurity::is_granted(StaticSecurityAction::IsAdmin, &user) {
        return Err(ApiError::Forbidden {
            message: "You cannot create system wide notificytions".to_string(),
        });
    }
    if body.show_until.is_none() {
        return Err(ApiError::BadRequest {
            message: "Please supply an active deadline".to_string(),
        });
    }
    NotificationRepository::create_system_wide_notification(
        body.title.clone(),
        body.content.clone(),
        body.show_until.unwrap(),
        conn,
    );
    Ok(HttpResponse::Ok().finish())
}

/// Endpoint to get system wide notifications
#[get("/system_wide_notifications")]
pub async fn get_system_wide_notifications(
    data: web::Data<AppState>,
    _: web::ReqData<UserData>,
) -> Result<HttpResponse, ApiError> {
    let conn = &mut data.db.db.get().unwrap();

    let notifications = NotificationRepository::get_system_wide(conn);
    Ok(HttpResponse::Ok().json(notifications))
}

#[derive(Deserialize)]
struct CreateNotificationForGroupRequest {
    pub title: String,
    pub content: String,
}

/// Endpoint to create group notification
#[post("/groups/{id}/notifications")]
pub async fn create_group_notification(
    data: web::Data<AppState>,
    user: web::ReqData<UserData>,
    body: web::Json<CreateNotificationForGroupRequest>,
    path: web::Path<(i32,)>,
) -> Result<HttpResponse, ApiError> {
    let conn = &mut data.db.db.get().unwrap();

    let mut group =
        GroupRepository::get_by_id(path.into_inner().0, conn).ok_or(ApiError::BadRequest {
            message: "No access to group".to_string(),
        })?;
    if !group.is_granted(SecurityAction::Update, &user) {
        return Err(ApiError::Unauthorized {
            message: "Not authorized for action".to_string(),
        });
    }
    NotificationRepository::create_notification_for_group(
        body.title.clone(),
        body.content.clone(),
        group.id,
        conn,
    );
    Ok(HttpResponse::Ok().finish())
}

/// Endpoint to delete system wide notification
#[delete("/system_wide_notifications/{id}")]
pub async fn delete_system_wide_notifications(
    data: web::Data<AppState>,
    user: web::ReqData<UserData>,
    path: web::Path<(i32,)>,
) -> Result<HttpResponse, ApiError> {
    let conn = &mut data.db.db.get().unwrap();

    if !StaticSecurity::is_granted(StaticSecurityAction::IsAdmin, &user) {
        return Err(ApiError::BadRequest {
            message: "You are not allowed to delete".to_string(),
        });
    }

    NotificationRepository::delete(path.into_inner().0, conn);
    Ok(HttpResponse::Ok().finish())
}
