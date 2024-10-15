use tonic::Response;

use crate::models::group::GroupRepository;
use crate::tasky_grpc::tasky_api_server::TaskyApi;
use crate::tasky_grpc::GroupsRequest;
use crate::tasky_grpc::GroupsResponse;
use crate::AppState;

pub struct MyTaskyApi {
    pub app_state: AppState,
}

/// The tasky RPC api
#[tonic::async_trait]
impl TaskyApi for MyTaskyApi {

    async fn get_user_groups(
        &self,
        request: tonic::Request<GroupsRequest>,
    ) -> std::result::Result<tonic::Response<GroupsResponse>, tonic::Status> {

        let user_id = i32::try_from(request.into_inner().user_id).unwrap();
        let conn = &mut self.app_state.db.db.get().unwrap();
        let groups = GroupRepository::get_groups_for_member(user_id, conn);

        let reply = GroupsResponse {
            groups: groups
                .into_iter()
                .map(|x| x.into())
                .collect::<Vec<crate::tasky_grpc::Group>>()
                .into(),
        };

        Ok(Response::new(reply))
    }
}
