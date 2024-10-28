use tonic::Response;

use crate::models::group::GroupRepository;
use crate::models::solution::SolutionRepository;
use crate::tasky_grpc::tasky_api_server::TaskyApi;
use crate::tasky_grpc::GroupsRequest;
use crate::tasky_grpc::GroupsResponse;
use crate::tasky_grpc::SolutionUpdateStatusRequest;
use crate::tasky_grpc::SolutionUpdateStatusResponse;
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
                .collect::<Vec<crate::tasky_grpc::Group>>(),
        };

        Ok(Response::new(reply))
    }

    async fn update_solution_status(
        &self,
        request: tonic::Request<SolutionUpdateStatusRequest>,
    ) -> std::result::Result<tonic::Response<SolutionUpdateStatusResponse>, tonic::Status> {
        let req_inner = request.into_inner();
        let solution_id = i32::try_from(req_inner.solution_id).unwrap();
        let conn = &mut self.app_state.db.db.get().unwrap();
        let mut solution = SolutionRepository::get_solution_by_id(solution_id, conn).unwrap();
        solution.approval_status = Some(req_inner.status);
        SolutionRepository::update_solution(solution, conn);
        Ok(Response::new(SolutionUpdateStatusResponse {
            message: "Success".to_string(),
        }))
    }
}
