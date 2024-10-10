use crate::schema::solutions::dsl;
use diesel::associations::HasTable;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::DB;

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
        return match self {
            Self::Pending => "PENDING".to_string(),
            Self::Approved => "APPROVED".to_string(),
            Self::Rejected => "REJECTED".to_string(),
            Self::Successful => "SUCCESSFUL".to_string(),
            Self::Failed => "FAILED".to_string(),
        };
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
    pub fn get_solutions_for_assignment(id: i32, conn: &mut DB) -> Vec<Solution> {
        dsl::solutions
            .filter(dsl::assignment_id.eq(id))
            .get_results::<Solution>(conn)
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
    pub fn get_solutions_for_user(id: i32, conn: &mut DB) -> Vec<Solution> {
        dsl::solutions
            .filter(dsl::submitter_id.eq(id))
            .get_results::<Solution>(conn)
            .expect("Cannot fetch solutions")
    }

    /// Updates an solution
    pub fn update_solution(solution: Solution, conn: &mut DB) {
        diesel::update(dsl::solutions.filter(dsl::id.eq(solution.id)))
            .set::<Solution>(solution)
            .execute(conn)
            .expect("Cannot update solution");
    }

    /// Creates an new solution
    pub fn create_solution(new: NewSolution, conn: &mut DB) -> Solution {
        diesel::insert_into(dsl::solutions::table())
            .values(new)
            .returning(Solution::as_returning())
            .get_result::<Solution>(conn)
            .expect("Cannot create new solution")
    }
}
