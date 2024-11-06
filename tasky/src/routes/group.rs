use super::PaginationParams;
use crate::auth_middleware::UserData;
use crate::error::ApiError;
use crate::models::group::{CreateGroup, GroupRepository};
use crate::models::PaginatedModel;
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
