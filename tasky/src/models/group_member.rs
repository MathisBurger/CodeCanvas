use crate::schema::group_members;
use diesel::prelude::*;
use diesel::OptionalExtension;
use diesel::{
    deserialize::Queryable, dsl::count_star, BoolExpressionMethods, ExpressionMethods, QueryDsl,
    RunQueryDsl, Selectable,
};

use super::DB;

#[derive(Queryable, Selectable, Clone, Insertable)]
#[diesel(primary_key(group_id, member_id))]
#[diesel(belongs_to(Group))]
#[diesel(table_name = group_members)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GroupMember {
    pub group_id: i32,
    pub member_id: i32,
}

pub struct GroupMemberRepository;

impl GroupMemberRepository {
    /// Inserts a new member into a group
    pub fn insert_member(new: GroupMember, conn: &mut DB) {
        diesel::insert_into(group_members::table)
            .values(new)
            .execute(conn)
            .expect("Cannot insert new member");
    }

    /// Checks if a user is group member
    pub fn is_member(group_id: i32, member_id: i32, conn: &mut DB) -> bool {
        group_members::dsl::group_members
            .filter(group_members::dsl::group_id.eq(group_id))
            .filter(group_members::dsl::member_id.eq(member_id))
            .first::<GroupMember>(conn)
            .optional()
            .expect("Cannot load if user is group member")
            .is_some()
    }

    /// Unable to remove user membership
    pub fn remove_membership(group_id: i32, user_id: i32, conn: &mut DB) {
        diesel::delete(
            group_members::dsl::group_members.filter(
                group_members::dsl::group_id
                    .eq(group_id)
                    .and(group_members::dsl::member_id.eq(user_id)),
            ),
        )
        .execute(conn)
        .expect("Cannot remove user membership");
    }

    /// Gets all enlisted users from selection
    pub fn get_enlisted_from_selection(
        group_id: i32,
        selection: Vec<i32>,
        conn: &mut DB,
    ) -> Vec<i32> {
        group_members::dsl::group_members
            .filter(group_members::group_id.eq(group_id))
            .filter(group_members::member_id.eq_any(selection))
            .select(group_members::member_id)
            .get_results::<i32>(conn)
            .expect("Cannot load enlisted from selection")
    }

    /// Inserts a hand full of new members
    pub fn insert_new_members(new: Vec<GroupMember>, conn: &mut DB) {
        diesel::insert_into(group_members::table)
            .values(new)
            .execute(conn)
            .expect("Cannot insert new members");
    }

    /// Fetches member count of a group
    pub fn member_count(group_id: i32, conn: &mut DB) -> i64 {
        group_members::dsl::group_members
            .filter(group_members::group_id.eq(group_id))
            .select(count_star())
            .get_result::<i64>(conn)
            .expect("Cannot fetch member count")
    }
}
