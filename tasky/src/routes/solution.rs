use crate::http::run_task;
use crate::models::assignment::AssignmentLanguage;
use crate::models::solution::{ApprovalStatus, Solution, SolutionRepository};
use crate::mongo::task_file::{TaskFile, TaskFileCollection};
use crate::mongo::test_file::{TestFile, TestFileCollection};
use crate::response::solution::SolutionsResponse;
use crate::security::StaticSecurity;
use crate::security::StaticSecurityAction;
use crate::util::mongo::parse_object_ids;
use crate::AppState;
use actix_multipart::form::MultipartForm;
use actix_web::web;
use actix_web::{get, post, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::{
    auth_middleware::UserData,
    error::ApiError,
    handler::solution::{handle_create_multipart, CreateSolutionMultipart},
    models::{
        assignment::{Assignment, AssignmentRepository},
        DB,
    },
    response::{solution::SolutionResponse, Enrich},
    security::{IsGranted, SecurityAction},
};

/// Query to query solution files
#[derive(Deserialize)]
struct SolutionFilesQuery {
    pub test_files: String,
    pub task_files: String,
}

/// Response of solution files query
#[derive(Serialize)]
struct SolutionsFilesResponse {
    pub test_files: Vec<TestFile>,
    pub task_files: Vec<TaskFile>,
}

/// Endpoint to create a solution
#[post("/assignments/{assignment_id}/solutions")]
pub async fn create_solution(
    data: web::Data<AppState>,
    user: web::ReqData<UserData>,
    path: web::Path<(i32,)>,
    MultipartForm(form): MultipartForm<CreateSolutionMultipart>,
) -> Result<HttpResponse, ApiError> {
    let user_data = user.into_inner();
    let path_data = path.into_inner();
    let conn = &mut data.db.db.get().unwrap();

    let assignment = get_assignment(path_data.0, &user_data, conn)?;
    let mut solution =
        handle_create_multipart(form, &user_data, &data.mongodb, conn, &assignment).await?;

    if assignment.language != AssignmentLanguage::QuestionBased {
        let job_id = run_task(assignment, solution.clone(), &data.config).await?;
        solution.job_id = Some(job_id);
        SolutionRepository::update_solution(solution.clone(), conn);
    }

    let enrichted = SolutionResponse::enrich(&solution, &mut data.user_api.clone(), conn).await?;
    Ok(HttpResponse::Ok().json(enrichted))
}

/// Endpoint to fetch a specific solution
#[get("/solutions/{id}")]
pub async fn get_solution(
    data: web::Data<AppState>,
    user: web::ReqData<UserData>,
    path: web::Path<(i32,)>,
) -> Result<HttpResponse, ApiError> {
    let user_data = user.into_inner();
    let path_data = path.into_inner();
    let conn = &mut data.db.db.get().unwrap();

    let (_, solution) = get_solution_and_assignment(path_data.0, &user_data, conn)?;
    let response = SolutionResponse::enrich(&solution, &mut data.user_api.clone(), conn).await?;
    Ok(HttpResponse::Ok().json(response))
}

/// Endpoint to get all solutions submitted by current user
#[get("/personal_solutions")]
pub async fn get_solutions_for_user(
    data: web::Data<AppState>,
    user: web::ReqData<UserData>,
) -> Result<HttpResponse, ApiError> {
    let user_data = user.into_inner();
    let conn = &mut data.db.db.get().unwrap();

    let solutions = SolutionRepository::get_solutions_for_user(user_data.user_id, conn);
    let response = SolutionsResponse::enrich(&solutions, &mut data.user_api.clone(), conn).await?;
    Ok(HttpResponse::Ok().json(response))
}

/// Endpoint to get all solutions for an assignment
#[get("/assignments/{assignment_id}/solutions")]
pub async fn get_solutions_for_assignment(
    data: web::Data<AppState>,
    user: web::ReqData<UserData>,
    path: web::Path<(i32,)>,
) -> Result<HttpResponse, ApiError> {
    let user_data = user.into_inner();
    let path_data = path.into_inner();

    let conn = &mut data.db.db.get().unwrap();
    let mut assignment = get_assignment(path_data.0, &user_data, conn)?;
    if !assignment.is_granted(SecurityAction::Update, &user_data) {
        return Err(ApiError::Forbidden {
            message: "Not allowed to read solutions for assignment".to_string(),
        });
    }

    let solutions = SolutionRepository::get_solutions_for_assignment(assignment.id, conn);
    let response = SolutionsResponse::enrich(&solutions, &mut data.user_api.clone(), conn).await?;
    Ok(HttpResponse::Ok().json(response))
}

/// Endpoint to approve an solution
#[post("/solutions/{id}/approve")]
pub async fn approve_solution(
    data: web::Data<AppState>,
    user: web::ReqData<UserData>,
    path: web::Path<(i32,)>,
) -> Result<HttpResponse, ApiError> {
    let user_data = user.into_inner();
    let path_data = path.into_inner();
    let conn = &mut data.db.db.get().unwrap();

    let (mut assignment, mut solution) =
        get_solution_and_assignment(path_data.0, &user_data, conn)?;
    if !solution.is_granted(SecurityAction::Update, &user_data) {
        return Err(ApiError::Forbidden {
            message: "You are not allowed to approve solution".to_string(),
        });
    }

    solution.approval_status = Some(ApprovalStatus::Approved.string());
    SolutionRepository::update_solution(solution.clone(), conn);

    if !assignment
        .completed_by
        .contains(&Some(solution.submitter_id))
    {
        assignment.completed_by.push(Some(solution.submitter_id));
    }

    AssignmentRepository::update_assignment(assignment.clone(), conn);
    let response = SolutionResponse::enrich(&solution, &mut data.user_api.clone(), conn).await?;
    return Ok(HttpResponse::Ok().json(response));
}

/// Endpoint to reject an solution
#[post("/solutions/{id}/reject")]
pub async fn reject_solution(
    data: web::Data<AppState>,
    user: web::ReqData<UserData>,
    path: web::Path<(i32,)>,
) -> Result<HttpResponse, ApiError> {
    let user_data = user.into_inner();
    let path_data = path.into_inner();
    let conn = &mut data.db.db.get().unwrap();

    let (_, mut solution) = get_solution_and_assignment(path_data.0, &user_data, conn)?;
    if !solution.is_granted(SecurityAction::Update, &user_data) {
        return Err(ApiError::Forbidden {
            message: "You are not allowed to reject solution".to_string(),
        });
    }

    solution.approval_status = Some(ApprovalStatus::Rejected.string());
    SolutionRepository::update_solution(solution.clone(), conn);

    let response = SolutionResponse::enrich(&solution, &mut data.user_api.clone(), conn).await?;
    return Ok(HttpResponse::Ok().json(response));
}

/// Endpoint to fetch solution files
#[get("/solutions/{id}/files")]
pub async fn get_solution_files(
    data: web::Data<AppState>,
    user: web::ReqData<UserData>,
    path: web::Path<(i32,)>,
    query: web::Query<SolutionFilesQuery>,
) -> Result<HttpResponse, ApiError> {
    let user_data = user.into_inner();
    let path_data = path.into_inner();
    let conn = &mut data.db.db.get().unwrap();

    let (_, solution) = get_solution_and_assignment(path_data.0, &user_data, conn)?;
    let task_files_ids = parse_object_ids(query.task_files.clone())?;
    let test_files_ids = parse_object_ids(query.test_files.clone())?;

    let task_files =
        TaskFileCollection::get_for_solution(solution.id, task_files_ids, &data.mongodb).await;
    if !StaticSecurity::is_granted(StaticSecurityAction::IsAdminOrTutor, &user_data) {
        return Ok(HttpResponse::Ok().json(SolutionsFilesResponse {
            task_files,
            test_files: vec![],
        }));
    }

    let test_files = TestFileCollection::get_for_assignment(
        solution.assignment_id,
        test_files_ids,
        &data.mongodb,
    )
    .await;

    return Ok(HttpResponse::Ok().json(SolutionsFilesResponse {
        task_files,
        test_files,
    }));
}

/// Gets solution and assignment and checks basic read permissions
fn get_solution_and_assignment(
    solution_id: i32,
    user_data: &UserData,
    conn: &mut DB,
) -> Result<(Assignment, Solution), ApiError> {
    let mut solution =
        SolutionRepository::get_solution_by_id(solution_id, conn).ok_or(ApiError::BadRequest {
            message: "Invalid solution".to_string(),
        })?;
    if !solution.is_granted(SecurityAction::Read, user_data) {
        return Err(ApiError::Forbidden {
            message: "You have no access to solution".to_string(),
        });
    }

    let assignment = get_assignment(solution.assignment_id, user_data, conn)?;
    return Ok((assignment, solution));
}

/// Gets assignment and checks basic read permissions
fn get_assignment(id: i32, user_data: &UserData, conn: &mut DB) -> Result<Assignment, ApiError> {
    let assignment = AssignmentRepository::get_assignment_by_id(id, conn);
    if assignment.is_none() {
        return Err(ApiError::BadRequest {
            message: "Invalid assignment ID".to_string(),
        });
    }

    let mut unwrapped = assignment.unwrap();
    if !unwrapped.is_granted(SecurityAction::Read, user_data) {
        return Err(ApiError::Forbidden {
            message: "You do not have read access to assignment".to_string(),
        });
    }

    return Ok(unwrapped);
}
