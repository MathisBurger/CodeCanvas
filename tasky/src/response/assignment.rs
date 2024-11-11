use crate::api::UsersRequest;
use crate::models::assignment::QuestionCatalogue;
use crate::models::PaginatedModel;
use crate::security::{StaticSecurity, StaticSecurityAction};
use crate::{api::usernator_api_client::UsernatorApiClient, auth_middleware::UserData};
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
    pub due_date: Option<NaiveDateTime>,
    pub group: MinifiedGroupResponse,
    pub description: String,
    pub language: AssignmentLanguage,
    pub completed_by: Vec<User>,
    pub question_catalogue: Option<QuestionCatalogue>,
    pub file_structure: Option<AssignmentFileStructure>,
    pub runner_cpu: String,
    pub runner_memory: String,
    pub runner_timeout: String,
    pub runner_cmd: String,
    pub completed: Option<bool>,
}

/// Minified response returned for list views
#[derive(Serialize)]
pub struct MinifiedAssignmentResponse {
    pub id: i32,
    pub title: String,
    pub due_date: Option<NaiveDateTime>,
    pub description: String,
    pub language: AssignmentLanguage,
    pub completed: Option<bool>,
}

/// A vec of assignments
#[derive(Serialize)]
pub struct AssignmentsResponse {
    assignments: Vec<MinifiedAssignmentResponse>,
    total: i64,
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
            due_date: from.due_date,
            description: from.description.clone(),
            language: from.language.clone(),
            completed: None,
        })
    }
}

impl MinifiedAssignmentResponse {
    /// Determines whether current user has completed assignment
    pub fn determine_completed(&mut self, user: &UserData, source: &Assignment) {
        if StaticSecurity::is_granted(StaticSecurityAction::IsStudent, user) {
            self.completed = Some(source.completed_by.contains(&Some(user.user_id)));
        }
    }
}

impl Enrich<PaginatedModel<Assignment>> for AssignmentsResponse {
    async fn enrich(
        from: &PaginatedModel<Assignment>,
        client: &mut crate::api::usernator_api_client::UsernatorApiClient<
            tonic::transport::Channel,
        >,
        db_conn: &mut super::DB,
    ) -> Result<Self, ApiError> {
        let mut resp: Vec<MinifiedAssignmentResponse> = vec![];
        for assignment in &from.results {
            resp.push(MinifiedAssignmentResponse::enrich(assignment, client, db_conn).await?);
        }

        Ok(AssignmentsResponse {
            assignments: resp,
            total: from.total,
        })
    }
}

impl AssignmentsResponse {
    /// Determines whether current user has completed the assignments
    pub fn determine_completed(&mut self, user: &UserData, source: &PaginatedModel<Assignment>) {
        let reference: &mut Vec<MinifiedAssignmentResponse> = self.assignments.as_mut();
        for i in 0..reference.len() {
            let partial_response = reference.get_mut(i).unwrap();
            partial_response.determine_completed(user, source.results.get(i).unwrap());
        }
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

        let question_catalogue = serde_json::from_value(
            from.question_catalogue
                .clone()
                .unwrap_or(serde_json::Value::Null),
        )
        .ok();

        Ok(AssignmentResponse {
            id: from.id,
            title: from.title.clone(),
            due_date: from.due_date,
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
            question_catalogue,
            runner_cpu: from.runner_cpu.clone(),
            runner_memory: from.runner_memory.clone(),
            runner_timeout: from.runner_timeout.clone(),
            runner_cmd: from.runner_cmd.clone(),
            completed: None,
        })
    }
}

impl AssignmentResponse {
    /// Authorizes the contained data
    pub fn authorize(&mut self, user: &UserData) {
        if !StaticSecurity::is_granted(StaticSecurityAction::IsAdminOrTutor, user) {
            let catalogue_option = &mut self.question_catalogue;
            if catalogue_option.is_some() {
                let catalogue = &mut catalogue_option.as_mut().unwrap();
                for value in catalogue.catalogue.values_mut() {
                    value.answer = serde_json::Value::Null;
                }
            }
        }
    }

    /// Determines whether current user has completed the assignment
    pub fn determine_completed(&mut self, user: &UserData, source: &Assignment) {
        if StaticSecurity::is_granted(StaticSecurityAction::IsStudent, user) {
            self.completed = Some(source.completed_by.contains(&Some(user.user_id)));
        }
    }
}
