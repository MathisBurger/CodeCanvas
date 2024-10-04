use crate::models::assignment::AssignmentLanguage;
use actix_multipart::form::MultipartForm;
use actix_web::web;
use actix_web::{get, post, HttpResponse};

use crate::AppState;
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
    if assignment.language == AssignmentLanguage::QuestionBased {
        return Err(ApiError::BadRequest {
            message: "Cannot create solution on question based assignment".to_string(),
        });
    }
    let solution =
        handle_create_multipart(form, &user_data, &data.mongodb, conn, &assignment).await?;
    let enrichted = SolutionResponse::enrich(&solution, &mut data.user_api.clone(), conn).await?;
    Ok(HttpResponse::Ok().json(enrichted))
}

//#[get("/solutions/{id}")]
//pub async fn get_solution() -> Result<HttpResponse, ApiError> {}

//#[get("/personal_solutions")]
//pub async fn get_solutions_for_user() -> Result<HttpResponse, ApiError> {}

//#[post("/assignments/{assignment_id}/solutions")]
//pub async fn get_solutions_for_assignment() -> Result<HttpResponse, ApiError> {}

//#[post("/solutions/{id}/approve")]
//pub async fn approve_solution() -> Result<HttpResponse, ApiError> {}

//#[post("/solutions/{id}/reject")]
//pub async fn reject_solution() -> Result<HttpResponse, ApiError> {}

fn get_assignment(id: i32, user_data: &UserData, conn: &mut DB) -> Result<Assignment, ApiError> {
    let assignment = AssignmentRepository::get_assignment_by_id(id, conn);
    if assignment.is_none() {
        return Err(ApiError::BadRequest {
            message: "Invalid assignment ID".to_string(),
        });
    }
    let mut unwrapped = assignment.unwrap();
    if !unwrapped.is_granted(SecurityAction::Update, user_data) {
        return Err(ApiError::Forbidden {
            message: "You do not have update access to assignment".to_string(),
        });
    }
    return Ok(unwrapped);
}
