use crate::auth_middleware::UserData;
use crate::models::solution::SolutionRepository;
use crate::models::DB;
use crate::security::SecurityAction;
use crate::AppState;
use crate::{
    error::ApiError,
    models::{
        code_comment::{CodeCommentRepository, CreateCodeComment},
        solution::Solution,
    },
    security::IsGranted,
};
use actix_web::{get, post, web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CodeCommentRequest {
    pub title: String,
    pub content: String,
}

/// Gets all code comments for a solution
#[get("/solutions/{solution_id}/code_comments")]
pub async fn get_code_comments(
    data: web::Data<AppState>,
    user: web::ReqData<UserData>,
    path: web::Path<(i32,)>,
) -> Result<HttpResponse, ApiError> {
    let user_data = user.into_inner();
    let path_data = path.into_inner();
    let conn = &mut data.db.db.get().unwrap();

    let solution = get_solution(path_data.0, &user_data, conn)?;
    let comments = CodeCommentRepository::get_comments_for_solution(solution.id, conn);
    Ok(HttpResponse::Ok().json(comments))
}

/// Creates a code comment on a solution
#[post("/solutions/{solution_id}/code_comments")]
pub async fn create_code_comment(
    data: web::Data<AppState>,
    user: web::ReqData<UserData>,
    req: web::Json<CodeCommentRequest>,
    path: web::Path<(i32,)>,
) -> Result<HttpResponse, ApiError> {
    let user_data = user.into_inner();
    let path_data = path.into_inner();
    let conn = &mut data.db.db.get().unwrap();

    let solution = get_solution(path_data.0, &user_data, conn)?;
    let mut create = CreateCodeComment {
        title: req.title.clone(),
        content: req.content.clone(),
        commentor: user_data.user_id,
        group_id: solution.group_id.unwrap_or(-1),
        solution_id: solution.id,
    };
    if !create.is_granted(SecurityAction::Create, &user_data) {
        return Err(ApiError::Forbidden {
            message: "You are not allowed to create a code comment".to_string(),
        });
    }
    let comment = CodeCommentRepository::create_comment(&create, conn);
    Ok(HttpResponse::Ok().json(comment))
}

/// Gets solution and checks basic read permissions
fn get_solution(
    solution_id: i32,
    user_data: &UserData,
    conn: &mut DB,
) -> Result<Solution, ApiError> {
    let mut solution =
        SolutionRepository::get_solution_by_id(solution_id, conn).ok_or(ApiError::BadRequest {
            message: "Invalid solution".to_string(),
        })?;
    if !solution.is_granted(SecurityAction::Read, user_data) {
        return Err(ApiError::Forbidden {
            message: "You have no access to solution".to_string(),
        });
    }
    Ok(solution)
}
