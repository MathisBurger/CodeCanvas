use actix_web::{delete, get, web, HttpResponse};

use crate::{auth_middleware::UserData, models::notification::NotificationRepository, AppState};

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
