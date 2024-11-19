use diesel::prelude::*;
use diesel::{
    deserialize::Queryable, prelude::Insertable, BoolExpressionMethods, ExpressionMethods,
    QueryDsl, Selectable,
};

use crate::schema::assignment_completions;

use super::DB;

#[derive(Queryable, Selectable, Clone, Insertable)]
#[diesel(primary_key(assignment_id, member_id))]
#[diesel(table_name = assignment_completions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AssignmentCompletion {
    pub assignment_id: i32,
    pub member_id: i32,
}

pub struct AssignmentCompletionRepository;

impl AssignmentCompletionRepository {
    /// Checks whether a user has completed assignment
    pub fn is_completed_by(assignment_id: i32, member_id: i32, conn: &mut DB) -> bool {
        assignment_completions::dsl::assignment_completions
            .filter(
                assignment_completions::assignment_id
                    .eq(assignment_id)
                    .and(assignment_completions::member_id.eq(member_id)),
            )
            .first::<AssignmentCompletion>(conn)
            .optional()
            .expect("Cannot fetch is completed state")
            .is_some()
    }

    /// Creates a new completion in the system
    pub fn create_completion(comp: AssignmentCompletion, conn: &mut DB) {
        diesel::insert_into(assignment_completions::table)
            .values(comp)
            .execute(conn)
            .expect("Cannot insert assignment completion");
    }
}
