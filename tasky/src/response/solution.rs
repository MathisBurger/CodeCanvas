use crate::api::UserRequest;
use crate::http::get_job;
use crate::models::assignment::AssignmentLanguage;
use crate::models::solution::QuestionResult;
use crate::{api::usernator_api_client::UsernatorApiClient, http::Job};
use serde::Serialize;

use crate::{
    error::ApiError,
    models::{assignment::AssignmentRepository, solution::Solution},
};

use super::assignment::MinifiedAssignmentResponse;
use super::{
    assignment::{AssignmentFileStructure, AssignmentResponse},
    shared::User,
    Enrich,
};

/// Solution response with enriched data
#[derive(Serialize)]
pub struct SolutionResponse {
    pub id: i32,
    pub submitter: User,
    pub assignment: AssignmentResponse,
    pub approval_status: Option<String>,
    pub file_structure: Option<AssignmentFileStructure>,
    pub job: Option<Job>,
    pub question_results: Option<Vec<QuestionResult>>,
}

/// Solution response for list views
#[derive(Serialize)]
pub struct ListSolutionResponse {
    pub id: i32,
    pub submitter: User,
    pub approval_status: Option<String>,
    pub assignment: MinifiedAssignmentResponse,
    pub job: Option<Job>,
}

/// Vec of solutions
#[derive(Serialize)]
pub struct SolutionsResponse {
    pub solutions: Vec<ListSolutionResponse>,
}

impl Enrich<Solution> for ListSolutionResponse {
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
        let assignment_response =
            MinifiedAssignmentResponse::enrich(&assignment, client, db_conn).await?;

        let mut job = None;

        if assignment.language != AssignmentLanguage::QuestionBased {
            job = match from.job_id.as_ref() {
                Some(id) => Some(get_job(id).await?),
                None => None,
            };
        }

        Ok(ListSolutionResponse {
            id: from.id,
            submitter: submitter.into_inner().into(),
            approval_status: from.approval_status.clone(),
            assignment: assignment_response,
            job,
        })
    }
}

impl Enrich<Vec<Solution>> for SolutionsResponse {
    async fn enrich(
        from: &Vec<Solution>,
        client: &mut UsernatorApiClient<tonic::transport::Channel>,
        db_conn: &mut super::DB,
    ) -> Result<Self, ApiError> {
        let mut responses: Vec<ListSolutionResponse> = vec![];
        for solution in from {
            responses.push(ListSolutionResponse::enrich(solution, client, db_conn).await?);
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

        let mut job = None;
        let mut question_results: Option<Vec<QuestionResult>> = None;

        if assignment.language == AssignmentLanguage::QuestionBased {
            question_results = serde_json::from_value(from.question_result.clone().unwrap()).ok();
        } else {
            job = match from.job_id.as_ref() {
                Some(id) => Some(get_job(id).await?),
                None => None,
            };
        }

        Ok(SolutionResponse {
            id: from.id,
            submitter: submitter.into_inner().into(),
            assignment: assigment_response,
            approval_status: from.approval_status.clone(),
            file_structure,
            job,
            question_results,
        })
    }
}
