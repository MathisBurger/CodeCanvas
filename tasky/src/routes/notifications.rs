use actix_web::{delete, get, post, web, HttpResponse};
use serde::Deserialize;

use crate::{
    auth_middleware::UserData,
    error::ApiError,
    models::{group::GroupRepository, notification::NotificationRepository},
    security::{IsGranted, SecurityAction},
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
}

/// Endpoint to create group notification
#[post("/groups/{id}/notifications")]
pub async fn create_group_notification(
    data: web::Data<AppState>,
    user: web::ReqData<UserData>,
    body: web::Json<CreateNotificationRequest>,
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
