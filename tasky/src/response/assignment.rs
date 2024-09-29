use crate::api::UsersRequest;
use chrono::NaiveDateTime;
use serde::Serialize;

use crate::{
    error::ApiError,
    models::{
        assignment::{Assignment, AssignmentLanguage},
        group::GroupRepository,
    },
};

use super::{group::MinifiedGroupResponse, shared::User, Enrich};

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
    pub file_structure: String,
}

/// A vec of assignments
#[derive(Serialize)]
pub struct AssignmentsResponse {
    assignments: Vec<AssignmentResponse>,
}

impl Enrich<Vec<Assignment>> for AssignmentsResponse {
    async fn enrich(
        from: &Vec<Assignment>,
        client: &mut crate::api::usernator_api_client::UsernatorApiClient<
            tonic::transport::Channel,
        >,
        db_conn: &mut super::DB,
    ) -> Result<Self, ApiError> {
        let mut resp: Vec<AssignmentResponse> = vec![];
        for assignment in from {
            resp.push(AssignmentResponse::enrich(assignment, client, db_conn).await?);
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
            file_structure: serde_json::from_value(
                from.file_structure
                    .clone()
                    .unwrap_or(serde_json::Value::Null),
            )
            .unwrap(),
        })
    }
}
