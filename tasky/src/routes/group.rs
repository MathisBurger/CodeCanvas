use super::PaginationParams;
use crate::auth_middleware::UserData;
use crate::error::ApiError;
use crate::models::group::{CreateGroup, GroupRepository, JoinRequestPolicy};
use crate::models::group_join_request::GroupJoinRequestRepository;
use crate::response::group::{GroupResponse, GroupsResponse};
use crate::response::Enrich;
use crate::security::{IsGranted, SecurityAction};
use crate::AppState;
use actix_web::{get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};

/// Request to create a new group
#[derive(Deserialize, Serialize)]
pub struct CreateGroupRequest {
    pub title: String,
    pub join_policy: JoinRequestPolicy,
}

/// Endpoint to create a new group
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
        join_policy: req.join_policy.clone(),
    };
    if !new_group.is_granted(SecurityAction::Create, &user) {
        return Err(ApiError::Forbidden {
            message: "User is not allowed to create a group".to_string(),
        });
    }

    let resp = GroupRepository::insert_group(new_group, conn);
    let enriched = GroupResponse::enrich(&resp, &mut data.user_api.clone(), conn).await?;
    Ok(HttpResponse::Ok().json(enriched))
}

/// Endpoint to fetch all groups you are not a member of
#[get("/groups")]
pub async fn get_all_groups(
    data: web::Data<AppState>,
    user: web::ReqData<UserData>,
    pagination: web::Query<PaginationParams>,
) -> Result<HttpResponse, ApiError> {
    let conn = &mut data.db.db.get().unwrap();

    let groups = GroupRepository::get_groups_for_not_member(
        user.into_inner().user_id,
        pagination.page,
        conn,
    );
    let resp = GroupsResponse::enrich(&groups, &mut data.user_api.clone(), conn).await?;

    Ok(HttpResponse::Ok().json(resp))
}

/// Endpoint to fetch all groups you are a member of
#[get("/my_groups")]
pub async fn get_all_my_groups(
    data: web::Data<AppState>,
    user: web::ReqData<UserData>,
    pagination: web::Query<PaginationParams>,
) -> Result<HttpResponse, ApiError> {
    let conn = &mut data.db.db.get().unwrap();

    let groups = GroupRepository::get_groups_for_member_paginated(
        user.into_inner().user_id,
        pagination.page,
        conn,
    );
    let resp = GroupsResponse::enrich(&groups, &mut data.user_api.clone(), conn).await?;

    Ok(HttpResponse::Ok().json(resp))
}

/// Endpoint to fetch a specific group by ID
#[get("/groups/{id}")]
pub async fn get_group(
    data: web::Data<AppState>,
    user: web::ReqData<UserData>,
    path: web::Path<(i32,)>,
) -> Result<HttpResponse, ApiError> {
    let conn = &mut data.db.db.get().unwrap();

    let mut group =
        GroupRepository::get_by_id(path.into_inner().0, conn).ok_or(ApiError::BadRequest {
            message: "No access to group".to_string(),
        })?;
    if group.is_granted(SecurityAction::Read, &user) {
        let enriched = GroupResponse::enrich(&group, &mut data.user_api.clone(), conn).await?;
        return Ok(HttpResponse::Ok().json(enriched));
    }

    Err(ApiError::Unauthorized {
        message: "Not authorized for action".to_string(),
    })
}

#[derive(Deserialize)]
struct UpdateGroupRequest {
    pub title: String,
    pub join_policy: JoinRequestPolicy,
}

#[post("/groups/{id}")]
pub async fn update_group(
    data: web::Data<AppState>,
    user: web::ReqData<UserData>,
    path: web::Path<(i32,)>,
    req: web::Json<UpdateGroupRequest>,
) -> Result<HttpResponse, ApiError> {
    let conn = &mut data.db.db.get().unwrap();
    let mut group =
        GroupRepository::get_by_id(path.into_inner().0, conn).ok_or(ApiError::BadRequest {
            message: "No access to group".to_string(),
        })?;

    if !group.is_granted(SecurityAction::Update, &user) {
        return Err(ApiError::Forbidden {
            message: "You are not allowed to update group".to_string(),
        });
    }

    let found_group = GroupRepository::get_by_title(&req.title, conn);
    if found_group.is_some() && group.title.clone() != found_group.unwrap().title {
        return Err(ApiError::BadRequest {
            message: "Group with this name already exists".to_string(),
        });
    }

    group.title = req.title.clone();
    group.join_policy = req.join_policy.clone();

    if group.join_policy == JoinRequestPolicy::Open {
        let requests = GroupJoinRequestRepository::get_group_requests_no_pagination(group.id, conn);
        group
            .members
            .extend(requests.iter().map(|r| Some(r.requestor)));
        GroupJoinRequestRepository::delete_all_requests_for_group(group.id, conn);
    } else if group.join_policy == JoinRequestPolicy::Closed {
        let requests = GroupJoinRequestRepository::get_group_requests_no_pagination(group.id, conn);
        for join_request in requests.iter() {
            GroupJoinRequestRepository::delete_request(join_request.clone(), conn);
        }
    }

    GroupRepository::update_group(group.clone(), conn);

    let enriched = GroupResponse::enrich(&group, &mut data.user_api.clone(), conn).await?;
    return Ok(HttpResponse::Ok().json(enriched));
}
