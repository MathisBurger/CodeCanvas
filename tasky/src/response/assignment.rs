use crate::api::usernator_api_client::UsernatorApiClient;
use crate::api::UsersRequest;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::{
    error::ApiError,
    models::{
        assignment::{Assignment, AssignmentLanguage},
        group::GroupRepository,
    },
};

use super::{group::MinifiedGroupResponse, shared::User, Enrich};

/// An file on an assignment
#[derive(Serialize, Deserialize, Clone)]
pub struct AssignmentFile {
    pub filename: String,
    pub object_id: Option<String>,
    pub file_size: Option<usize>,
    pub is_test_file: bool,
}

/// File structure of an assignment / solution
#[derive(Serialize, Deserialize, Clone)]
pub struct AssignmentFileStructure {
    pub files: Option<Vec<AssignmentFile>>,
    pub folders: Option<Vec<AssignmentFileStructure>>,
    pub current_folder_name: Option<String>,
}

/// The assignment response
#[derive(Serialize)]
pub struct AssignmentResponse {
    pub id: i32,
    pub title: String,
    pub due_date: NaiveDateTime,
    pub group: MinifiedGroupResponse,
    pub description: String,
    pub language: AssignmentLanguage,
    pub completed_by: Vec<User>,
    pub file_structure: Option<AssignmentFileStructure>,
    pub runner_cpu: String,
    pub runner_memory: String,
    pub runner_timeout: String,
}

/// Minified response returned for list views
#[derive(Serialize)]
pub struct MinifiedAssignmentResponse {
    pub id: i32,
    pub title: String,
    pub due_date: NaiveDateTime,
    pub description: String,
    pub language: AssignmentLanguage,
}

/// A vec of assignments
#[derive(Serialize)]
pub struct AssignmentsResponse {
    assignments: Vec<MinifiedAssignmentResponse>,
}

impl Enrich<Assignment> for MinifiedAssignmentResponse {
    async fn enrich(
        from: &Assignment,
        _client: &mut UsernatorApiClient<tonic::transport::Channel>,
        _db_conn: &mut super::DB,
    ) -> Result<Self, ApiError> {
        Ok(MinifiedAssignmentResponse {
            id: from.id,
            title: from.title.clone(),
            due_date: from.due_date.clone(),
            description: from.description.clone(),
            language: from.language.clone(),
        })
    }
}

impl Enrich<Vec<Assignment>> for AssignmentsResponse {
    async fn enrich(
        from: &Vec<Assignment>,
        client: &mut crate::api::usernator_api_client::UsernatorApiClient<
            tonic::transport::Channel,
        >,
        db_conn: &mut super::DB,
    ) -> Result<Self, ApiError> {
        let mut resp: Vec<MinifiedAssignmentResponse> = vec![];
        for assignment in from {
            resp.push(MinifiedAssignmentResponse::enrich(assignment, client, db_conn).await?);
        }
        Ok(AssignmentsResponse { assignments: resp })
    }
}

impl Enrich<Assignment> for AssignmentResponse {
    async fn enrich(
        from: &Assignment,
        client: &mut crate::api::usernator_api_client::UsernatorApiClient<
            tonic::transport::Channel,
        >,
        db_conn: &mut super::DB,
    ) -> Result<Self, ApiError> {
        let group = GroupRepository::get_by_id(from.group_id, db_conn).unwrap();
        let group_response = MinifiedGroupResponse::enrich(&group, client, db_conn).await?;
        let completed_by: Vec<u64> = from
            .completed_by
            .iter()
            .filter(|x| x.is_some())
            .map(|m| u64::try_from(m.unwrap()).unwrap())
            .collect();
        let users = client
            .get_users(UsersRequest {
                user_ids: completed_by,
            })
            .await?;
        let file_structure = serde_json::from_value(
            from.file_structure
                .clone()
                .unwrap_or(serde_json::Value::Null),
        )
        .ok();

        Ok(AssignmentResponse {
            id: from.id,
            title: from.title.clone(),
            due_date: from.due_date.clone(),
            group: group_response,
            description: from.description.clone(),
            language: from.language.clone(),
            completed_by: users
                .into_inner()
                .users
                .into_iter()
                .map(|x| x.into())
                .collect(),
            file_structure,
            runner_cpu: from.runner_cpu.clone(),
            runner_memory: from.runner_memory.clone(),
            runner_timeout: from.runner_timeout.clone(),
        })
    }
}
