use super::group_join_request::GroupJoinRequestRepository;
use super::Paginate;
use super::{PaginatedModel, DB};
use crate::schema::groups::dsl;
use diesel::prelude::*;
use diesel::{associations::HasTable, dsl::not};
use serde::{Deserialize, Serialize};

/// Group entity in the database
#[derive(Queryable, Selectable, AsChangeset, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::groups)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Group {
    pub id: i32,
    pub title: String,
    pub members: Vec<Option<i32>>,
    pub tutor: i32,
}

/// Used to create a group in database
#[derive(Insertable)]
#[diesel(table_name = crate::schema::groups)]
pub struct CreateGroup {
    pub title: String,
    pub tutor: i32,
    pub members: Vec<i32>,
}

pub struct GroupRepository;

impl GroupRepository {
    /// Gets a group by ID
    pub fn get_by_id(id: i32, conn: &mut DB) -> Option<Group> {
        dsl::groups
            .find(id)
            .first::<Group>(conn)
            .optional()
            .expect("Error loading group")
    }

    /// Gets a group by title
    pub fn get_by_title(title: &String, conn: &mut DB) -> Option<Group> {
        let result = dsl::groups
            .filter(dsl::title.eq(title))
            .get_results::<Group>(conn)
            .expect("Error loading groups");
        if result.is_empty() {
            return None;
        }
        return Some(result.first().unwrap().clone());
    }

    /// Inserts a group into the database
    pub fn insert_group(group: CreateGroup, conn: &mut DB) -> Group {
        diesel::insert_into(dsl::groups::table())
            .values(&group)
            .returning(Group::as_returning())
            .get_result::<Group>(conn)
            .expect("Cannot create new group")
    }

    /// Updates the group in the database
    pub fn update_group(group: Group, conn: &mut DB) {
        diesel::update(dsl::groups.filter(dsl::id.eq(group.id)))
            .set::<Group>(group)
            .execute(conn)
            .expect("Cannot update group");
    }

    /// Gets all groups a user is not member or tutor of
    pub fn get_groups_for_member(member_id: i32, conn: &mut DB) -> Vec<Group> {
        dsl::groups
            .filter(
                dsl::tutor
                    .eq(member_id)
                    .or(dsl::members.contains(vec![Some(member_id)])),
            )
            .get_results::<Group>(conn)
            .expect("Cannot fetch groups for member")
    }

    /// Gets all groups a user is member or tutor of
    pub fn get_groups_for_not_member(
        member_id: i32,
        page: i64,
        conn: &mut DB,
    ) -> PaginatedModel<Group> {
        let requested: Vec<i32> = GroupJoinRequestRepository::get_user_requests(member_id, conn)
            .into_iter()
            .map(|x| x.group_id)
            .collect();
        dsl::groups
            .filter(not(dsl::tutor
                .eq(member_id)
                .or(dsl::id.eq_any(requested))
                .or(dsl::members.contains(vec![Some(member_id)]))))
            .paginate(page)
            .load_and_count_pages::<Group>(conn)
            .expect("Cannot fetch groups for member")
    }
}
