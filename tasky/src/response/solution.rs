use crate::api::UserRequest;
use crate::http::get_job;
use crate::{api::usernator_api_client::UsernatorApiClient, http::Job};
use serde::Serialize;

use crate::{
    error::ApiError,
    models::{
        assignment::{Assignment, AssignmentRepository},
        solution::Solution,
    },
};

use super::{
    assignment::{AssignmentFileStructure, AssignmentResponse},
    shared::User,
    Enrich,
};

#[derive(Serialize)]
pub struct SolutionResponse {
    pub id: i32,
    pub submitter: User,
    pub assignment: AssignmentResponse,
    pub approval_status: Option<String>,
    pub file_structure: Option<AssignmentFileStructure>,
    pub job: Option<Job>,
}

#[derive(Serialize)]
pub struct ListSolutionResponse {
    pub id: i32,
    pub submitter: User,
    pub assignment: AssignmentResponse,
    pub approval_status: Option<String>,
}

#[derive(Serialize)]
pub struct SolutionsResponse {
    pub solutions: Vec<SolutionResponse>,
}

impl Enrich<Vec<Solution>> for SolutionsResponse {
    async fn enrich(
        from: &Vec<Solution>,
        client: &mut UsernatorApiClient<tonic::transport::Channel>,
        db_conn: &mut super::DB,
    ) -> Result<Self, ApiError> {
        let mut responses: Vec<SolutionResponse> = vec![];
        for solution in from {
            responses.push(SolutionResponse::enrich(solution, client, db_conn).await?);
        }
        Ok(SolutionsResponse {
            solutions: responses,
        })
    }
}

impl Enrich<Solution> for SolutionResponse {
    async fn enrich(
        from: &Solution,
        client: &mut UsernatorApiClient<tonic::transport::Channel>,
        db_conn: &mut super::DB,
    ) -> Result<Self, ApiError> {
        let submitter = client
            .get_user(UserRequest {
                user_id: u64::try_from(from.submitter_id)?,
            })
            .await?;
        let assignment =
            AssignmentRepository::get_assignment_by_id(from.assignment_id, db_conn).unwrap();
        let assigment_response = AssignmentResponse::enrich(&assignment, client, db_conn).await?;
        let file_structure = serde_json::from_value(
            from.file_structure
                .clone()
                .unwrap_or(serde_json::Value::Null),
        )
        .ok();
        let job = match from.job_id.as_ref() {
            Some(id) => Some(get_job(id).await?),
            None => None,
        };
        return Ok(SolutionResponse {
            id: from.id,
            submitter: submitter.into_inner().into(),
            assignment: assigment_response,
            approval_status: from.approval_status.clone(),
            file_structure,
            job,
        });
    }
}