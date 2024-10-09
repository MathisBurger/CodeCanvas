use crate::models::assignment::QuestionCatalogue;
use serde::{Deserialize, Serialize};

use crate::{
    error::ApiError,
    models::{
        assignment::{Assignment, AssignmentRepository},
        DB,
    },
};

/// Handles the creation of a question catalogue
pub fn handle_catalogue_creation(
    catalogue: QuestionCatalogue,
    assignment: &mut Assignment,
    conn: &mut DB,
) {
    let question_catalogue = serde_json::to_value(catalogue).unwrap();
    assignment.question_catalogue = Some(question_catalogue);
    AssignmentRepository::update_assignment(assignment.clone(), conn);
}
