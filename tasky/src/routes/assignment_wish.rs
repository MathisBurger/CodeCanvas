use crate::models::assignment_wish::AssignmentWishRepository;
use crate::models::group::GroupRepository;
use crate::security::{IsGranted, SecurityAction};
use crate::{
    auth_middleware::UserData, error::ApiError, models::assignment_wish::CreateAssignmentWish,
    AppState,
};
use actix_web::{delete, get, post, web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
struct CreateWishRequest {
    pub title: String,
    pub description: String,
}

#[post("/groups/{group_id}/assignment_wishes")]
pub async fn create_wish(
    data: web::Data<AppState>,
    user: web::ReqData<UserData>,
    req: web::Json<CreateWishRequest>,
    path: web::Path<(i32,)>,
) -> Result<HttpResponse, ApiError> {
    let user_data = user.into_inner();
    let conn = &mut data.db.db.get().unwrap();

    let mut group =
        GroupRepository::get_by_id(path.into_inner().0, conn).ok_or(ApiError::BadRequest {
            message: "No access to group".to_string(),
        })?;
    if !group.is_granted(SecurityAction::Read, &user_data) {
        return Err(ApiError::Forbidden {
            message: "No access to group".to_string(),
        });
    }

    let mut create = CreateAssignmentWish {
        title: req.title.clone(),
        description: req.description.clone(),
        group_id: group.id,
    };

    if !create.is_granted(SecurityAction::Create, &user_data) {
        return Err(ApiError::Forbidden {
            message: "Cannot create assignment wish".to_string(),
        });
    }

    let wish = AssignmentWishRepository::create_wish(&create, conn);
    Ok(HttpResponse::Ok().json(wish))
}

#[get("/groups/{group_id}/assignment_wishes")]
pub async fn get_wishes(
    data: web::Data<AppState>,
    user: web::ReqData<UserData>,
    path: web::Path<(i32,)>,
) -> Result<HttpResponse, ApiError> {
    let user_data = user.into_inner();
    let conn = &mut data.db.db.get().unwrap();

    let mut group =
        GroupRepository::get_by_id(path.into_inner().0, conn).ok_or(ApiError::BadRequest {
            message: "No access to group".to_string(),
        })?;
    if !group.is_granted(SecurityAction::Read, &user_data) {
        return Err(ApiError::Forbidden {
            message: "No access to group".to_string(),
        });
    }

    let wishes = AssignmentWishRepository::get_wishes_for_group(group.id, conn);
    Ok(HttpResponse::Ok().json(wishes))
}

#[get("/groups/{group_id}/assignment_wishes/{wish_id}")]
pub async fn get_wish(
    data: web::Data<AppState>,
    user: web::ReqData<UserData>,
    path: web::Path<(i32, i32)>,
) -> Result<HttpResponse, ApiError> {
    let user_data = user.into_inner();
    let conn = &mut data.db.db.get().unwrap();
    let params = path.into_inner();

    let mut group = GroupRepository::get_by_id(params.0, conn).ok_or(ApiError::BadRequest {
        message: "No access to group".to_string(),
    })?;
    if !group.is_granted(SecurityAction::Read, &user_data) {
        return Err(ApiError::Forbidden {
            message: "No access to group".to_string(),
        });
    }
    let wish = AssignmentWishRepository::get_assignment_wish(params.1, conn);
    if wish.is_none() {
        return Err(ApiError::BadRequest {
            message: "Invalid wish ID given".to_string(),
        });
    }

    let mut unwrapped_wish = wish.unwrap();

    if !unwrapped_wish.is_granted(SecurityAction::Read, &user_data) {
        return Err(ApiError::Forbidden {
            message: "Not allowed to read wish".to_string(),
        });
    }

    Ok(HttpResponse::Ok().json(unwrapped_wish))
}

#[delete("/groups/{group_id}/assignment_wishes/{wish_id}")]
pub async fn delete_wish(
    data: web::Data<AppState>,
    user: web::ReqData<UserData>,
    path: web::Path<(i32, i32)>,
) -> Result<HttpResponse, ApiError> {
    let user_data = user.into_inner();
    let conn = &mut data.db.db.get().unwrap();
    let params = path.into_inner();

    let mut group = GroupRepository::get_by_id(params.0, conn).ok_or(ApiError::BadRequest {
        message: "No access to group".to_string(),
    })?;
    if !group.is_granted(SecurityAction::Read, &user_data) {
        return Err(ApiError::Forbidden {
            message: "No access to group".to_string(),
        });
    }
    let wish = AssignmentWishRepository::get_assignment_wish(params.1, conn);
    if wish.is_none() {
        return Err(ApiError::BadRequest {
            message: "Invalid wish ID given".to_string(),
        });
    }
    AssignmentWishRepository::delete_wish(&wish.unwrap(), conn);
    Ok(HttpResponse::Ok().finish())
}
