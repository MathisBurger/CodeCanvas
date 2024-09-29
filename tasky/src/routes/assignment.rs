use actix_multipart::form::MultipartForm;
use actix_multipart::Multipart;
use actix_web::get;
use actix_web::post;
use actix_web::web;
use actix_web::HttpResponse;
use chrono::NaiveDateTime;

use crate::auth_middleware::UserData;
use crate::error::ApiError;
use crate::handler::assignment::handle_create_multipart;
use crate::handler::assignment::CreateCodeTestMultipart;
use crate::models::assignment::Assignment;
use crate::models::assignment::AssignmentLanguage;
use crate::models::assignment::AssignmentRepository;
use crate::models::assignment::CreateAssignment;
use crate::models::database::DBPool;
use crate::models::group::Group;
use crate::models::group::GroupRepository;
use crate::models::DB;
use crate::response::assignment::AssignmentResponse;
use crate::response::assignment::AssignmentsResponse;
use crate::response::Enrich;
use crate::security::IsGranted;
use crate::security::SecurityAction;
use crate::AppState;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer};

fn deserialize_naive_datetime<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    // Parse as DateTime<Utc> to handle the Z suffix
    let datetime = DateTime::parse_from_rfc3339(&s)
        .map_err(serde::de::Error::custom)?
        .with_timezone(&Utc);
    // Convert to NaiveDateTime (without timezone)
    Ok(datetime.naive_utc())
}

/// Request to create an assignment
#[derive(Deserialize)]
struct CreateAssignmentRequest {
    pub title: String,
    #[serde(deserialize_with = "deserialize_naive_datetime")]
    pub due_date: NaiveDateTime,
    pub description: String,
    pub language: AssignmentLanguage,
}

/// Request to update an assignment
#[derive(Deserialize)]
struct UpdateAssignmentRequest {
    pub title: String,
    #[serde(deserialize_with = "deserialize_naive_datetime")]
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
    let (group, assignment) = get_group_and_assignment(&user_data, path_data, conn)?;
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
    let (group, mut assignment) = get_group_and_assignment(&user_data, path_data, conn)?;
    assignment.title = req.title.clone();
    assignment.due_date = req.due_date.clone();
    assignment.description = req.description.clone();
    if !assignment.is_granted(SecurityAction::Update, &user_data) {
        return Err(ApiError::Forbidden {
            message: "You are not allowed to update assignment".to_string(),
        });
    }
    AssignmentRepository::update_assignment(assignment.clone(), conn);
    let enrichted =
        AssignmentResponse::enrich(&assignment, &mut data.user_api.clone(), conn).await?;
    Ok(HttpResponse::Ok().json(enrichted))
}

/// Endpoint to create the test of the assignment
#[post("/groups/{group_id}/assignments/{id}/code_test")]
pub async fn create_assignment_test(
    data: web::Data<AppState>,
    user: web::ReqData<UserData>,
    path: web::Path<(i32, i32)>,
    MultipartForm(form): MultipartForm<CreateCodeTestMultipart>,
) -> Result<HttpResponse, ApiError> {
    let user_data = user.into_inner();
    let path_data = path.into_inner();
    let conn = &mut data.db.db.get().unwrap();
    let (group, mut assignment) = get_group_and_assignment(&user_data, path_data, conn)?;
    if !assignment.is_granted(SecurityAction::Update, &user_data) {
        return Err(ApiError::Forbidden {
            message: "You are not allowed to create code tests".to_string(),
        });
    }
    let updated = handle_create_multipart(form, &data.mongodb, conn, assignment).await?;
    let enriched = AssignmentResponse::enrich(&updated, &mut data.user_api.clone(), conn).await?;
    Ok(HttpResponse::Ok().json(enriched))
}

pub async fn view_assignment_test() {}

/// Gets group and assignment from request params and connection.
/// Furthermore, it handles all the user security checks
fn get_group_and_assignment(
    user_data: &UserData,
    path_data: (i32, i32),
    conn: &mut DB,
) -> Result<(Group, Assignment), ApiError> {
    let mut group = GroupRepository::get_by_id(path_data.0, conn).ok_or(ApiError::BadRequest {
        message: "No access to group".to_string(),
    })?;
    if !group.is_granted(SecurityAction::Update, &user_data) {
        return Err(ApiError::Forbidden {
            message: "No access to group".to_string(),
        });
    }
    let mut assignment =
        AssignmentRepository::get_assignment_by_id_and_group(path_data.1, path_data.0, conn)
            .ok_or(ApiError::BadRequest {
                message: "No access to assignment".to_string(),
            })?;
    if !assignment.is_granted(SecurityAction::Read, &user_data) {
        return Err(ApiError::Forbidden {
            message: "No access to assignment".to_string(),
        });
    }
    Ok((group, assignment))
}
