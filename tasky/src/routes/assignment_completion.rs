use actix_web::{get, web, HttpResponse};

use crate::routes::assignment::get_group_and_assignment;
use crate::{
    auth_middleware::UserData,
    error::ApiError,
    models::assignment_completion::AssignmentCompletionRepository,
    response::{assignment_completion::AssignmentCompletionsResponse, Enrich},
    AppState,
};

use super::PaginationParams;

/// Endpoint for assignment completions
#[get("/groups/{group_id}/assignments/{id}/completions")]
pub async fn assignment_completions(
    data: web::Data<AppState>,
    user: web::ReqData<UserData>,
    pagination: web::Query<PaginationParams>,
    path: web::Path<(i32, i32)>,
) -> Result<HttpResponse, ApiError> {
    let user_data = user.into_inner();
    let path_data = path.into_inner();
    let conn = &mut data.db.db.get().unwrap();

    let (_, assignment) = get_group_and_assignment(&user_data, path_data, conn)?;

    let completions = AssignmentCompletionRepository::get_completion_ids_for_assignment(
        assignment.id,
        pagination.page,
        conn,
    );
    let enriched =
        AssignmentCompletionsResponse::enrich(&completions, &mut data.user_api.clone(), conn)
            .await?;
    Ok(HttpResponse::Ok().json(enriched))
}
