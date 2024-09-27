use actix_web::get;
use actix_web::post;
use actix_web::web;
use actix_web::HttpResponse;
use chrono::NaiveDateTime;
use serde::Deserialize;

use crate::auth_middleware::UserData;
use crate::error::ApiError;
use crate::models::assignment::AssignmentLanguage;
use crate::models::assignment::AssignmentRepository;
use crate::models::assignment::CreateAssignment;
use crate::models::group::GroupRepository;
use crate::response::assignment::AssignmentResponse;
use crate::response::assignment::AssignmentsResponse;
use crate::response::Enrich;
use crate::security::IsGranted;
use crate::security::SecurityAction;
use crate::AppState;

/// Request to create an assignment
#[derive(Deserialize)]
struct CreateAssignmentRequest {
    pub title: String,
    pub due_date: NaiveDateTime,
    pub description: String,
    pub language: AssignmentLanguage,
}

/// Request to update an assignment
#[derive(Deserialize)]
struct UpdateAssignmentRequest {
    pub title: String,
    pub due_date: NaiveDateTime,
    pub description: String,
}

/// Gets all assignments on a group
#[get("/groups/{group_id}/assignments")]
pub async fn get_all_group_assignments(
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
    if !group.is_granted(SecurityAction::Update, &user_data) {
        return Err(ApiError::Forbidden {
            message: "No access to group".to_string(),
        });
    }
    let assignments = AssignmentRepository::get_all_group_assignments(group.id, conn);
    let enriched =
        AssignmentsResponse::enrich(&assignments, &mut data.user_api.clone(), conn).await?;
    Ok(HttpResponse::Ok().json(enriched))
}

/// Endpoint to create an assignment on a group
#[post("/groups/{group_id}/assignments")]
pub async fn create_assignment(
    data: web::Data<AppState>,
    user: web::ReqData<UserData>,
    req: web::Json<CreateAssignmentRequest>,
    path: web::Path<(i32,)>,
) -> Result<HttpResponse, ApiError> {
    let user_data = user.into_inner();
    let conn = &mut data.db.db.get().unwrap();
    let mut group =
        GroupRepository::get_by_id(path.into_inner().0, conn).ok_or(ApiError::BadRequest {
            message: "No access to group".to_string(),
        })?;
    if !group.is_granted(SecurityAction::Update, &user_data) {
        return Err(ApiError::Forbidden {
            message: "No access to group".to_string(),
        });
    }
    let mut create_assignment = CreateAssignment {
        title: req.title.clone(),
        due_date: req.due_date.clone(),
        group_id: group.id,
        description: req.description.clone(),
        language: req.language.clone(),
    };
    if !create_assignment.is_granted(SecurityAction::Create, &user_data) {
        return Err(ApiError::Forbidden {
            message: "Not allowed to create an assignment".to_string(),
        });
    }
    let assignment = AssignmentRepository::create_assignment(&create_assignment, conn);
    let enriched =
        AssignmentResponse::enrich(&assignment, &mut data.user_api.clone(), conn).await?;
    Ok(HttpResponse::Ok().json(enriched))
}

/// Endpoint to get an specific assignment on a group
#[get("/groups/{group_id}/assignments/{id}")]
pub async fn get_assignment(
    data: web::Data<AppState>,
    user: web::ReqData<UserData>,
    path: web::Path<(i32, i32)>,
) -> Result<HttpResponse, ApiError> {
    let user_data = user.into_inner();
    let path_data = path.into_inner();
    let conn = &mut data.db.db.get().unwrap();
    let mut group = GroupRepository::get_by_id(path_data.0, conn).ok_or(ApiError::BadRequest {
        message: "No access to group".to_string(),
    })?;
    if !group.is_granted(SecurityAction::Update, &user_data) {
        return Err(ApiError::Forbidden {
            message: "No access to group".to_string(),
        });
    }
    let mut assignment =
        AssignmentRepository::get_assignment_by_id_and_group(path_data.0, path_data.1, conn)
            .ok_or(ApiError::BadRequest {
                message: "No access to assignment".to_string(),
            })?;
    if !assignment.is_granted(SecurityAction::Read, &user_data) {
        return Err(ApiError::Forbidden {
            message: "No access to assignment".to_string(),
        });
    }
    let enrichted =
        AssignmentResponse::enrich(&assignment, &mut data.user_api.clone(), conn).await?;
    Ok(HttpResponse::Ok().json(enrichted))
}

/// Endpoint to update an specific assignment on a group
#[post("/groups/{group_id}/assignments/{id}/update")]
pub async fn update_assignment(
    data: web::Data<AppState>,
    req: web::Json<UpdateAssignmentRequest>,
    user: web::ReqData<UserData>,
    path: web::Path<(i32, i32)>,
) -> Result<HttpResponse, ApiError> {
    let user_data = user.into_inner();
    let path_data = path.into_inner();
    let conn = &mut data.db.db.get().unwrap();
    let mut group = GroupRepository::get_by_id(path_data.0, conn).ok_or(ApiError::BadRequest {
        message: "No access to group".to_string(),
    })?;
    if !group.is_granted(SecurityAction::Update, &user_data) {
        return Err(ApiError::Forbidden {
            message: "No access to group".to_string(),
        });
    }
    let mut assignment =
        AssignmentRepository::get_assignment_by_id_and_group(path_data.0, path_data.1, conn)
            .ok_or(ApiError::BadRequest {
                message: "No access to assignment".to_string(),
            })?;
    if !assignment.is_granted(SecurityAction::Read, &user_data) {
        return Err(ApiError::Forbidden {
            message: "No access to assignment".to_string(),
        });
    }
    assignment.title = req.title.clone();
    assignment.due_date = req.due_date.clone();
    assignment.description = req.description.clone();
    AssignmentRepository::update_assignment(assignment.clone(), conn);
    let enrichted =
        AssignmentResponse::enrich(&assignment, &mut data.user_api.clone(), conn).await?;
    Ok(HttpResponse::Ok().json(enrichted))
}

pub async fn create_assignment_test() {}
