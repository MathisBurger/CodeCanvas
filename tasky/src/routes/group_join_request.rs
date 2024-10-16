use crate::auth_middleware::UserData;
use crate::error::ApiError;
use crate::models::group::GroupRepository;
use crate::models::group_join_request::{CreateGroupJoinRequest, GroupJoinRequestRepository};
use crate::response::group::GroupResponse;
use crate::response::group_join_request::{GroupJoinRequestResponse, GroupJoinRequestsResponse};
use crate::response::Enrich;
use crate::security::{IsGranted, SecurityAction, StaticSecurity, StaticSecurityAction};
use crate::AppState;
use actix_web::{get, post, web, HttpResponse};

/// Endpoint to create a join request on a group
#[post("/groups/{id}/create_join_request")]
pub async fn create_join_request(
    data: web::Data<AppState>,
    user: web::ReqData<UserData>,
    path: web::Path<(i32,)>,
) -> Result<HttpResponse, ApiError> {
    let conn = &mut data.db.db.get().unwrap();

    let group =
        GroupRepository::get_by_id(path.into_inner().0, conn).ok_or(ApiError::BadRequest {
            message: "Group does not exist".to_string(),
        })?;
    if !StaticSecurity::is_granted(StaticSecurityAction::IsStudent, &user)
        || group.members.contains(&Some(user.user_id))
    {
        return Err(ApiError::Forbidden {
            message: "The user is already member or not a student".to_string(),
        });
    }

    if GroupJoinRequestRepository::request_exists(group.id, user.user_id, conn) {
        return Err(ApiError::Forbidden {
            message: "User already sent a request".to_string(),
        });
    }

    let request = GroupJoinRequestRepository::create_request(
        CreateGroupJoinRequest {
            requestor: user.user_id,
            group_id: group.id,
        },
        conn,
    );

    let resp = GroupJoinRequestResponse::enrich(&request, &mut data.user_api.clone(), conn).await?;
    Ok(HttpResponse::Ok().json(resp))
}

/// Gets all join requests of a specific group by group_id
#[get("/groups/{id}/join_requests")]
pub async fn get_join_requests(
    data: web::Data<AppState>,
    user: web::ReqData<UserData>,
    path: web::Path<(i32,)>,
) -> Result<HttpResponse, ApiError> {
    if !StaticSecurity::is_granted(StaticSecurityAction::IsAdminOrTutor, &user) {
        return Err(ApiError::Forbidden {
            message: "User is not a tutor".to_string(),
        });
    }

    let conn = &mut data.db.db.get().unwrap();
    let requests = GroupJoinRequestRepository::get_group_requests(path.into_inner().0, conn);
    let resp =
        GroupJoinRequestsResponse::enrich(&requests, &mut data.user_api.clone(), conn).await?;
    Ok(HttpResponse::Ok().json(resp))
}

/// Approves a specific join request on a group
#[post("/groups/{group_id}/join_requests/{request_id}/approve")]
pub async fn approve_join_request(
    data: web::Data<AppState>,
    user: web::ReqData<UserData>,
    path: web::Path<(i32, i32)>,
) -> Result<HttpResponse, ApiError> {
    let conn = &mut data.db.db.get().unwrap();
    let user_data = user.into_inner();
    let path_data = path.into_inner();

    let mut group = GroupRepository::get_by_id(path_data.0, conn).ok_or(ApiError::BadRequest {
        message: "Group does not exist".to_string(),
    })?;
    if !group.is_granted(SecurityAction::Update, &user_data) {
        return Err(ApiError::Forbidden {
            message: "User is not allowed to approve request".to_string(),
        });
    }

    let mut request =
        GroupJoinRequestRepository::get_by_id(path_data.1, conn).ok_or(ApiError::BadRequest {
            message: "Group join request does not exist".to_string(),
        })?;
    if !request.is_granted(SecurityAction::Delete, &user_data) {
        return Err(ApiError::Forbidden {
            message: "User is not allowed to approve request".to_string(),
        });
    }

    group.members.push(Some(request.requestor));
    GroupRepository::update_group(group.clone(), conn);
    GroupJoinRequestRepository::delete_request(request, conn);

    let enriched = GroupResponse::enrich(&group, &mut data.user_api.clone(), conn).await?;
    Ok(HttpResponse::Ok().json(enriched))
}

/// Rejects a join request on a specific group
#[post("/groups/{group_id}/join_requests/{request_id}/reject")]
pub async fn reject_join_request(
    data: web::Data<AppState>,
    user: web::ReqData<UserData>,
    path: web::Path<(i32, i32)>,
) -> Result<HttpResponse, ApiError> {
    let conn = &mut data.db.db.get().unwrap();
    let user_data = user.into_inner();
    let path_data = path.into_inner();

    let mut request =
        GroupJoinRequestRepository::get_by_id(path_data.1, conn).ok_or(ApiError::BadRequest {
            message: "Group join request does not exist".to_string(),
        })?;
    if !request.is_granted(SecurityAction::Delete, &user_data) {
        return Err(ApiError::Forbidden {
            message: "Cannot reject request".to_string(),
        });
    }

    GroupJoinRequestRepository::delete_request(request, conn);
    Ok(HttpResponse::Ok().finish())
}
