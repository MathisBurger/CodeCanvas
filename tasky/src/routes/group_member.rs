use actix_web::get;
use actix_web::web;
use actix_web::HttpResponse;

use super::PaginationParams;
use crate::auth_middleware::UserData;
use crate::error::ApiError;
use crate::models::group::GroupRepository;
use crate::models::group_member::GroupMemberRepository;
use crate::response::group_member::GroupMembersResponse;
use crate::response::Enrich;
use crate::security::IsGranted;
use crate::security::SecurityAction;
use crate::AppState;

#[get("/groups/{id}/members")]
pub async fn members_paginated(
    data: web::Data<AppState>,
    user: web::ReqData<UserData>,
    pagination: web::Query<PaginationParams>,
    path: web::Path<(i32,)>,
) -> Result<HttpResponse, ApiError> {
    let conn = &mut data.db.db.get().unwrap();

    let mut group =
        GroupRepository::get_by_id(path.into_inner().0, conn).ok_or(ApiError::BadRequest {
            message: "No access to group".to_string(),
        })?;
    if !group.is_granted(SecurityAction::Read, &user) {
        return Err(ApiError::Forbidden {
            message: "You are not allowed to view group".to_string(),
        });
    }
    let members = GroupMemberRepository::get_members_ids_paginated(group.id, pagination.page, conn);
    let enriched = GroupMembersResponse::enrich(&members, &mut data.user_api.clone(), conn).await?;
    Ok(HttpResponse::Ok().json(enriched))
}
