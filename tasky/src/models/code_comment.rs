use super::group::GroupRepository;
use super::notification::CreateNotification;
use super::notification::NotificationRepository;
use super::solution::SolutionRepository;
use super::DB;
use crate::schema::code_comments::dsl;
use chrono::NaiveDateTime;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::Selectable;
use serde::Serialize;

/// code comment entity type
#[derive(Queryable, Selectable, Clone, Serialize)]
#[diesel(table_name = crate::schema::code_comments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CodeComment {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub commentor: i32,
    pub group_id: i32,
    pub solution_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Create comment struct to create a code comment
#[derive(Insertable)]
#[diesel(table_name = crate::schema::code_comments)]
pub struct CreateCodeComment {
    pub title: String,
    pub content: String,
    pub commentor: i32,
    pub group_id: i32,
    pub solution_id: i32,
}

pub struct CodeCommentRepository;

impl CodeCommentRepository {
    /// Creates a new code comment
    pub fn create_comment(create: &CreateCodeComment, conn: &mut DB) -> CodeComment {
        let solution = SolutionRepository::get_solution_by_id(create.solution_id, conn).unwrap();
        let group = GroupRepository::get_by_id(create.group_id, conn).unwrap();
        let targeted_users = match solution.submitter_id == create.commentor {
            true => vec![group.tutor],
            false => vec![solution.submitter_id],
        };
        NotificationRepository::create_notification(
            &CreateNotification {
                title: "New code comment".to_string(),
                content: format!("You got a new code comment for solution {}", solution.id),
                targeted_users,
            },
            conn,
        );
        diesel::insert_into(dsl::code_comments::table())
            .values(create)
            .returning(CodeComment::as_returning())
            .get_result::<CodeComment>(conn)
            .expect("Cannot create new code comment")
    }

    /// Gets all comments for solution
    pub fn get_comments_for_solution(solution_id: i32, conn: &mut DB) -> Vec<CodeComment> {
        dsl::code_comments
            .filter(dsl::solution_id.eq(solution_id))
            .get_results::<CodeComment>(conn)
            .expect("Cannot fetch comments")
    }
}
