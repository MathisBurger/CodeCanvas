use crate::schema::solutions::dsl;
use chrono::NaiveDateTime;
use diesel::associations::HasTable;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::{
    assignment::AssignmentRepository,
    group::GroupRepository,
    notification::{CreateNotification, NotificationRepository},
    Paginate, PaginatedModel, DB,
};

/// Approval status of a request
pub enum ApprovalStatus {
    Pending,
    Rejected,
    Approved,
    Successful,
    Failed,
}

impl ApprovalStatus {
    /// Converts enum to string (this is simpler than to_string())
    pub fn string(&self) -> String {
        match self {
            Self::Pending => "PENDING".to_string(),
            Self::Approved => "APPROVED".to_string(),
            Self::Rejected => "REJECTED".to_string(),
            Self::Successful => "SUCCESSFUL".to_string(),
            Self::Failed => "FAILED".to_string(),
        }
    }
}

impl From<&str> for ApprovalStatus {
    fn from(value: &str) -> Self {
        match value {
            "PENDING" => Self::Pending,
            "APPROVED" => Self::Approved,
            "REJECTED" => Self::Rejected,
            "SUCCESSFUL" => Self::Successful,
            "FAILED" => Self::Failed,
            _ => Self::Pending,
        }
    }
}

/// A solution to a question
#[derive(Deserialize)]
pub struct QuestionSolution {
    pub answer: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
pub struct QuestionResult {
    pub question: String,
    pub answer: serde_json::Value,
    pub correct: bool,
}

/// The solution on an assignment
#[derive(Queryable, Selectable, AsChangeset, Clone, Serialize)]
#[diesel(table_name = crate::schema::solutions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Solution {
    pub id: i32,
    pub submitter_id: i32,
    pub assignment_id: i32,
    pub file_structure: Option<serde_json::Value>,
    pub approval_status: Option<String>,
    pub job_id: Option<String>,
    pub group_id: Option<i32>,
    pub question_result: Option<serde_json::Value>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Struct to create a new solution
#[derive(Insertable)]
#[diesel(table_name = crate::schema::solutions)]
pub struct NewSolution {
    pub submitter_id: i32,
    pub assignment_id: i32,
    pub approval_status: Option<String>,
    pub group_id: i32,
}

pub struct SolutionRepository;

impl SolutionRepository {
    /// Gets all solutions created on an assignment
    pub fn get_solutions_for_assignment(
        id: i32,
        page: i64,
        conn: &mut DB,
    ) -> PaginatedModel<Solution> {
        dsl::solutions
            .filter(dsl::assignment_id.eq(id))
            .paginate(page)
            .load_and_count_pages::<Solution>(conn)
            .expect("Cannot fetch solutions")
    }

    /// Gets an solution by ID
    pub fn get_solution_by_id(id: i32, conn: &mut DB) -> Option<Solution> {
        dsl::solutions
            .filter(dsl::id.eq(id))
            .first::<Solution>(conn)
            .optional()
            .expect("Cannot fetch solutions")
    }

    /// Gets all solutions for an user (submitter)
    pub fn get_solutions_for_user(id: i32, page: i64, conn: &mut DB) -> PaginatedModel<Solution> {
        dsl::solutions
            .filter(dsl::submitter_id.eq(id))
            .paginate(page)
            .load_and_count_pages::<Solution>(conn)
            .expect("Cannot fetch solutions")
    }

    /// Updates an solution
    pub fn update_solution(solution: Solution, conn: &mut DB) {
        NotificationRepository::create_notification(
            &CreateNotification {
                title: "Solution updated".to_string(),
                content: format!("Your solution {} has been updated", solution.id),
                targeted_users: vec![solution.submitter_id],
            },
            conn,
        );
        diesel::update(dsl::solutions.filter(dsl::id.eq(solution.id)))
            .set::<Solution>(solution)
            .execute(conn)
            .expect("Cannot update solution");
    }

    /// Gets all ids of solutions that are assigned to a specific group
    pub fn get_ids_for_group(group_id: i32, conn: &mut DB) -> Vec<i32> {
        dsl::solutions
            .select(dsl::id)
            .filter(dsl::group_id.eq(group_id))
            .get_results::<i32>(conn)
            .expect("Cannot load solution IDs")
    }

    /// Creates an new solution
    pub fn create_solution(new: NewSolution, conn: &mut DB) -> Solution {
        NotificationRepository::create_notification(
            &CreateNotification {
                title: "Solution created".to_string(),
                content: format!(
                    "New solution has been submitted for assignment {}",
                    AssignmentRepository::get_assignment_by_id(new.assignment_id, conn)
                        .unwrap()
                        .title
                ),
                targeted_users: vec![
                    GroupRepository::get_by_id(new.group_id, conn)
                        .unwrap()
                        .tutor,
                ],
            },
            conn,
        );
        diesel::insert_into(dsl::solutions::table())
            .values(new)
            .returning(Solution::as_returning())
            .get_result::<Solution>(conn)
            .expect("Cannot create new solution")
    }

    /// Gets all pending solutions for tutor
    pub fn get_pending_solutions_for_tutor(
        tutor_id: i32,
        page: i64,
        conn: &mut DB,
    ) -> PaginatedModel<Solution> {
        dsl::solutions
            .left_join(crate::schema::groups::table)
            .filter(crate::schema::groups::dsl::tutor.eq(tutor_id))
            .filter(
                dsl::approval_status
                    .ne("APPROVED")
                    .and(dsl::approval_status.ne("REJECTED")),
            )
            .select(Solution::as_select())
            .group_by(dsl::id)
            .paginate(page)
            .load_and_count_pages::<Solution>(conn)
            .expect("Cannot load pending solutions for tutor")
    }
}
