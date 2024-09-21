use crate::api::usernator_api_client::UsernatorApiClient;
use crate::error::ApiError;
use crate::{api::UserRequest, api::UsersRequest, models::group::Group, response::shared::User};
use serde::Serialize;
use tonic::transport::Channel;

use super::Enrich;

#[derive(Serialize)]
pub struct GroupResponse {
    pub id: i32,
    pub title: String,
    pub members: Vec<User>,
    pub tutor: User,
}

impl Enrich<Group, GroupResponse> for GroupResponse {
    async fn enrich(
        from: &Group,
        client: &mut UsernatorApiClient<Channel>,
    ) -> Result<GroupResponse, ApiError> {
        let tut = client
            .get_user(UserRequest {
                user_id: u64::try_from(from.tutor)?,
            })
            .await?;
        let members = client
            .get_users(UsersRequest {
                user_ids: from
                    .members
                    .clone()
                    .into_iter()
                    .filter(|m| m.is_some())
                    .map(|m| u64::try_from(m.unwrap()).unwrap())
                    .collect(),
            })
            .await?;
        Ok(GroupResponse {
            id: from.id,
            title: from.title.clone(),
            members: members
                .into_inner()
                .users
                .into_iter()
                .map(|x| x.into())
                .collect(),
            tutor: tut.into_inner().into(),
        })
    }
}
