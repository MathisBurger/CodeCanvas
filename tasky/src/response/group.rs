use crate::api::usernator_api_client::UsernatorApiClient;
use crate::error::ApiError;
use crate::models::group_join_request::GroupJoinRequestRepository;
use crate::{api::UserRequest, api::UsersRequest, models::group::Group, response::shared::User};
use serde::Serialize;
use tonic::transport::Channel;

use super::{Enrich, DB};

#[derive(Serialize)]
pub struct GroupResponse {
    pub id: i32,
    pub title: String,
    pub members: Vec<User>,
    pub tutor: User,
    pub request_count: i32,
}

#[derive(Serialize)]
pub struct MinifiedGroupResponse {
    pub id: i32,
    pub title: String,
    pub member_count: i32,
    pub tutor: User,
}

#[derive(Serialize)]
pub struct GroupsResponse {
    groups: Vec<MinifiedGroupResponse>,
}

impl Enrich<Group> for MinifiedGroupResponse {
    async fn enrich(
        from: &Group,
        client: &mut UsernatorApiClient<Channel>,
        _: &mut DB,
    ) -> Result<Self, ApiError> {
        let tut = client
            .get_user(UserRequest {
                user_id: u64::try_from(from.tutor)?,
            })
            .await?;
        Ok(MinifiedGroupResponse {
            id: from.id,
            title: from.title.clone(),
            member_count: from.members.len() as i32,
            tutor: tut.into_inner().into(),
        })
    }
}

impl Enrich<Vec<Group>> for GroupsResponse {
    async fn enrich(
        from: &Vec<Group>,
        client: &mut UsernatorApiClient<Channel>,
        db_conn: &mut DB,
    ) -> Result<Self, ApiError> {
        let mut groups: Vec<MinifiedGroupResponse> = vec![];
        for group in from {
            groups.push(MinifiedGroupResponse::enrich(&group, client, db_conn).await?);
        }
        Ok(GroupsResponse { groups })
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
        let request_count = GroupJoinRequestRepository::get_group_request_count(from.id, conn);
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
            request_count,
        })
    }
}
