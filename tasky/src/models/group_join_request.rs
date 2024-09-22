use crate::models::group::Group;
use crate::schema::group_join_requests;
use crate::schema::group_join_requests::dsl;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(belongs_to(Group))]
#[diesel(table_name = group_join_requests)]
pub struct GroupJoinRequest {
    pub id: i32,
    pub requestor: i32,
    pub group_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = group_join_requests)]
pub struct CreateGroupJoinRequest {
    pub requestor: i32,
    pub group_id: i32,
}

pub struct GroupJoinRequestRepository;
type DB = PooledConnection<ConnectionManager<PgConnection>>;

impl GroupJoinRequestRepository {
    pub fn get_by_id(id: i32, conn: &mut DB) -> Option<GroupJoinRequest> {
        dsl::group_join_requests
            .find(id)
            .first::<GroupJoinRequest>(conn)
            .optional()
            .expect("Error loading group join request")
    }

    pub fn create_request(req: CreateGroupJoinRequest, conn: &mut DB) -> GroupJoinRequest {
        diesel::insert_into(dsl::group_join_requests::table())
            .values(&req)
            .returning(GroupJoinRequest::as_returning())
            .get_result::<GroupJoinRequest>(conn)
            .expect("Cannot create new group join request")
    }

    pub fn get_group_request_count(group_id: i32, conn: &mut DB) -> i32 {
        dsl::group_join_requests
            .filter(dsl::group_id.eq(group_id))
            .count()
            .get_result::<i64>(conn)
            .expect("Cannot get count") as i32
    }

    pub fn get_group_requests(group_id: i32, conn: &mut DB) -> Vec<GroupJoinRequest> {
        dsl::group_join_requests
            .filter(dsl::group_id.eq(group_id))
            .get_results::<GroupJoinRequest>(conn)
            .expect("Cannot get requests")
    }

    pub fn delete_request(req: GroupJoinRequest, conn: &mut DB) {
        diesel::delete(dsl::group_join_requests.filter(dsl::id.eq(req.id)))
            .execute(conn)
            .expect("Cannot delete request");
    }
}
