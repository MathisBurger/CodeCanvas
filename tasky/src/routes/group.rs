use crate::auth_middleware::UserData;
use crate::error::ApiError;
use crate::models::group::{CreateGroup, GroupRepository};
use crate::models::group_join_request::{CreateGroupJoinRequest, GroupJoinRequestRepository};
use crate::response::group::{GroupResponse, GroupsResponse};
use crate::response::group_join_request::{GroupJoinRequestResponse, GroupJoinRequestsResponse};
use crate::response::Enrich;
use crate::security::{IsGranted, SecurityAction, StaticSecurity, StaticSecurityAction};
use crate::AppState;
use actix_web::{get, post, web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
struct CreateGroupRequest {
    title: String,
}

#[post("/create_group")]
pub async fn create_group(
    data: web::Data<AppState>,
    req: web::Json<CreateGroupRequest>,
    user: web::ReqData<UserData>,
) -> Result<HttpResponse, ApiError> {
    let conn = &mut data.db.db.get().unwrap();
    let group = GroupRepository::get_by_title(&req.title, conn);
    if group.is_some() {
        return Ok(HttpResponse::Found().finish());
    }
    let mut new_group = CreateGroup {
        title: (req.title).clone(),
        tutor: user.user_id,
        members: vec![],
    };
    if !new_group.is_granted(SecurityAction::Create, &user) {
        return Err(ApiError::Unauthorized);
    }
    let resp = GroupRepository::insert_group(new_group, conn);
    let enriched = GroupResponse::enrich(&resp, &mut data.user_api.clone(), conn).await?;
    Ok(HttpResponse::Ok().json(enriched))
}

#[get("/groups")]
pub async fn get_all_groups(data: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let conn = &mut data.db.db.get().unwrap();
    let groups = GroupRepository::get_all(conn);
    let resp = GroupsResponse::enrich(&groups, &mut data.user_api.clone(), conn).await?;
    Ok(HttpResponse::Ok().json(resp))
}

#[get("/groups/{id}")]
pub async fn get_group(
    data: web::Data<AppState>,
    user: web::ReqData<UserData>,
    path: web::Path<(i32,)>,
) -> Result<HttpResponse, ApiError> {
    let conn = &mut data.db.db.get().unwrap();
    let mut group =
        GroupRepository::get_by_id(path.into_inner().0, conn).ok_or(ApiError::BadRequest)?;
    if group.is_granted(SecurityAction::Read, &user) {
        let enriched = GroupResponse::enrich(&group, &mut data.user_api.clone(), conn).await?;
        return Ok(HttpResponse::Ok().json(enriched));
    }

    Err(ApiError::Unauthorized)
}

#[post("/groups/{id}/create_join_request")]
pub async fn create_join_request(
    data: web::Data<AppState>,
    user: web::ReqData<UserData>,
    path: web::Path<(i32,)>,
) -> Result<HttpResponse, ApiError> {
    let conn = &mut data.db.db.get().unwrap();
    let group =
        GroupRepository::get_by_id(path.into_inner().0, conn).ok_or(ApiError::BadRequest)?;
    if !StaticSecurity::is_granted(StaticSecurityAction::IsStudent, &user)
        || group.members.contains(&Some(user.user_id))
    {
        return Err(ApiError::Forbidden);
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

#[get("/groups/{id}/join_requests")]
pub async fn get_join_requests(
    data: web::Data<AppState>,
    user: web::ReqData<UserData>,
    path: web::Path<(i32,)>,
) -> Result<HttpResponse, ApiError> {
    if !StaticSecurity::is_granted(StaticSecurityAction::IsTutor, &user) {
        return Err(ApiError::Forbidden);
    }
    let conn = &mut data.db.db.get().unwrap();
    let requests = GroupJoinRequestRepository::get_group_requests(path.into_inner().0, conn);
    let resp =
        GroupJoinRequestsResponse::enrich(&requests, &mut data.user_api.clone(), conn).await?;
    Ok(HttpResponse::Ok().json(resp))
}

#[post("/groups/{group_id}/join_requests/{request_id}/approve")]
pub async fn approve_join_request(
    data: web::Data<AppState>,
    user: web::ReqData<UserData>,
    path: web::Path<(i32, i32)>,
) -> Result<HttpResponse, ApiError> {
    let conn = &mut data.db.db.get().unwrap();
    let user_data = user.into_inner();
    let path_data = path.into_inner();
    let mut group = GroupRepository::get_by_id(path_data.0, conn).ok_or(ApiError::BadRequest)?;
    if !group.is_granted(SecurityAction::Update, &user_data) {
        return Err(ApiError::Forbidden);
    }
    let mut request =
        GroupJoinRequestRepository::get_by_id(path_data.1, conn).ok_or(ApiError::BadRequest)?;
    if !request.is_granted(SecurityAction::Delete, &user_data) {
        return Err(ApiError::Forbidden);
    }
    group.members.push(Some(request.requestor));
    GroupRepository::update_group(group.clone(), conn);
    GroupJoinRequestRepository::delete_request(request, conn);
    let enriched = GroupResponse::enrich(&group, &mut data.user_api.clone(), conn).await?;
    Ok(HttpResponse::Ok().json(enriched))
}

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
        GroupJoinRequestRepository::get_by_id(path_data.1, conn).ok_or(ApiError::BadRequest)?;
    if !request.is_granted(SecurityAction::Delete, &user_data) {
        return Err(ApiError::Forbidden);
    }
    GroupJoinRequestRepository::delete_request(request, conn);
    Ok(HttpResponse::Ok().finish())
}
