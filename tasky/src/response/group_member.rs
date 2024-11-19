use serde::Serialize;

use crate::models::PaginatedModel;

use super::{shared::User, Enrich};
use crate::api::UsersRequest;
use crate::UsernatorApiClient;

#[derive(Serialize)]
pub struct GroupMembersResponse {
    members: Vec<User>,
    total: i64,
}

impl Enrich<PaginatedModel<i32>> for GroupMembersResponse {
    async fn enrich(
        from: &PaginatedModel<i32>,
        client: &mut UsernatorApiClient<tonic::transport::Channel>,
        _: &mut super::DB,
    ) -> Result<Self, crate::error::ApiError> {
        let members = client
            .get_users(UsersRequest {
                user_ids: from
                    .results
                    .clone()
                    .into_iter()
                    .map(|m| u64::try_from(m).unwrap())
                    .collect(),
            })
            .await?;

        Ok(GroupMembersResponse {
            members: members
                .into_inner()
                .users
                .into_iter()
                .map(|x| x.into())
                .collect(),
            total: from.total,
        })
    }
}
