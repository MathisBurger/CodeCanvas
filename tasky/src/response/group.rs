use crate::api::usernator_api_client::UsernatorApiClient;
use crate::error::ApiError;
use crate::models::group::JoinRequestPolicy;
use crate::models::group_join_request::GroupJoinRequestRepository;
use crate::models::group_member::GroupMemberRepository;
use crate::models::PaginatedModel;
use crate::{api::UserRequest, models::group::Group, response::shared::User};
use serde::Serialize;
use tonic::transport::Channel;

use super::{Enrich, DB};

/// The group response
#[derive(Serialize)]
pub struct GroupResponse {
    pub id: i32,
    pub title: String,
    pub tutor: User,
    pub request_count: i32,
    pub join_policy: JoinRequestPolicy,
    pub verified: bool,
}

/// The minified group response
#[derive(Serialize)]
pub struct MinifiedGroupResponse {
    pub id: i32,
    pub title: String,
    pub member_count: i64,
    pub tutor: User,
    pub join_policy: JoinRequestPolicy,
    pub verified: bool,
}

/// The groups response
#[derive(Serialize)]
pub struct GroupsResponse {
    total: i64,
    page: i64,
    groups: Vec<MinifiedGroupResponse>,
}

impl Enrich<Group> for MinifiedGroupResponse {
    async fn enrich(
        from: &Group,
        client: &mut UsernatorApiClient<Channel>,
        db: &mut DB,
    ) -> Result<Self, ApiError> {
        let tut = client
            .get_user(UserRequest {
                user_id: u64::try_from(from.tutor)?,
            })
            .await?;

        Ok(MinifiedGroupResponse {
            id: from.id,
            title: from.title.clone(),
            member_count: GroupMemberRepository::member_count(from.id, db),
            tutor: tut.into_inner().into(),
            join_policy: from.join_policy.clone(),
            verified: from.verified,
        })
    }
}

impl Enrich<PaginatedModel<Group>> for GroupsResponse {
    async fn enrich(
        from: &PaginatedModel<Group>,
        client: &mut UsernatorApiClient<Channel>,
        db_conn: &mut DB,
    ) -> Result<Self, ApiError> {
        let mut groups: Vec<MinifiedGroupResponse> = vec![];
        for group in from.results.clone() {
            groups.push(MinifiedGroupResponse::enrich(&group, client, db_conn).await?);
        }
        Ok(GroupsResponse {
            groups,
            total: from.total,
            page: from.page,
        })
    }
}

impl Enrich<Group> for GroupResponse {
    async fn enrich(
        from: &Group,
        client: &mut UsernatorApiClient<Channel>,
        conn: &mut DB,
    ) -> Result<Self, ApiError> {
        let tut = client
            .get_user(UserRequest {
                user_id: u64::try_from(from.tutor)?,
            })
            .await?;

        let request_count = GroupJoinRequestRepository::get_group_request_count(from.id, conn);
        Ok(GroupResponse {
            id: from.id,
            title: from.title.clone(),
            tutor: tut.into_inner().into(),
            join_policy: from.join_policy.clone(),
            verified: from.verified,
            request_count,
        })
    }
}
