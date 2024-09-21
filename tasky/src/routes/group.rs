use crate::auth_middleware::{UserData, UserRole};
use crate::error::ApiError;
use crate::models::group::{CreateGroup, GroupRepository};
use crate::response::group::GroupResponse;
use crate::response::Enrich;
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
    if !user.user_roles.contains(&UserRole::RoleTutor.to_string())
        && !user.user_roles.contains(&UserRole::RoleAdmin.to_string())
    {
        return Err(ApiError::Unauthorized);
    }
    let new_group = CreateGroup {
        title: (req.title).clone(),
        tutor: user.user_id,
        members: vec![],
    };
    let resp = GroupRepository::insert_group(new_group, conn);
    let enriched = GroupResponse::enrich(&resp, &mut (data.user_api.clone())).await?;
    Ok(HttpResponse::Ok().json(enriched))
}

#[get("/groups/{id}")]
pub async fn get_group(
    data: web::Data<AppState>,
    user: web::ReqData<UserData>,
    path: web::Path<(i32,)>,
) -> Result<HttpResponse, ApiError> {
    let conn = &mut data.db.db.get().unwrap();
    let group =
        GroupRepository::get_by_id(path.into_inner().0, conn).ok_or(ApiError::BadRequest)?;
    if user.user_roles.contains(&UserRole::RoleAdmin.to_string())
        || (user.user_roles.contains(&UserRole::RoleTutor.to_string())
            && group.tutor == user.user_id)
        || (user.user_roles.contains(&UserRole::RoleStudent.to_string())
            && group.members.contains(&Some(user.user_id)))
    {
        let enriched = GroupResponse::enrich(&group, &mut (data.user_api.clone())).await?;
        return Ok(HttpResponse::Ok().json(enriched));
    }

    Err(ApiError::Unauthorized)
}
