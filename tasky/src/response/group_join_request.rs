use crate::api::UserRequest;
use crate::{api::usernator_api_client::UsernatorApiClient, models::PaginatedModel};
use serde::Serialize;
use tonic::transport::Channel;

use crate::{error::ApiError, models::group_join_request::GroupJoinRequest};

use super::{shared::User, Enrich, DB};

/// The group join request response
#[derive(Serialize)]
pub struct GroupJoinRequestResponse {
    pub id: i32,
    pub requestor: User,
    pub group_id: i32,
}

/// The group join requests response
#[derive(Serialize)]
pub struct GroupJoinRequestsResponse {
    requests: Vec<GroupJoinRequestResponse>,
    total: i64,
}

impl Enrich<PaginatedModel<GroupJoinRequest>> for GroupJoinRequestsResponse {
    async fn enrich(
        from: &PaginatedModel<GroupJoinRequest>,
        client: &mut UsernatorApiClient<Channel>,
        db_conn: &mut DB,
    ) -> Result<Self, ApiError> {
        let mut requests: Vec<GroupJoinRequestResponse> = vec![];
        for request in &from.results {
            requests.push(GroupJoinRequestResponse::enrich(request, client, db_conn).await?);
        }
        Ok(GroupJoinRequestsResponse {
            requests,
            total: from.total,
        })
    }
}

impl Enrich<GroupJoinRequest> for GroupJoinRequestResponse {
    async fn enrich(
        from: &GroupJoinRequest,
        client: &mut UsernatorApiClient<Channel>,
        _: &mut DB,
    ) -> Result<Self, ApiError> {
        let requestor = client
            .get_user(UserRequest {
                user_id: u64::try_from(from.requestor)?,
            })
            .await?;

        Ok(GroupJoinRequestResponse {
            id: from.id,
            requestor: requestor.into_inner().into(),
            group_id: from.group_id,
        })
    }
}
