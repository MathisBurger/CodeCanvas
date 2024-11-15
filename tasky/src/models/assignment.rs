use super::notification::NotificationRepository;
use super::{Paginate, PaginatedModel, DB};
use crate::schema::assignments::dsl;
use chrono::NaiveDateTime;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::{
    prelude::{Insertable, Queryable},
    Selectable, SelectableHelper,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The language of an assignment
/// This language is the language the assignment should be completed with
#[derive(diesel_derive_enum::DbEnum, Debug, Clone, Deserialize, Serialize, PartialEq)]
#[ExistingTypePath = "crate::schema::sql_types::AssignmentLanguage"]
pub enum AssignmentLanguage {
    #[serde(rename = "Java")]
    Java,
    #[serde(rename = "Golang")]
    Golang,
    #[serde(rename = "QuestionBased")]
    QuestionBased,
}

/// The type of a question answer
#[derive(Serialize, Deserialize, Clone, Hash)]
pub enum AnswerType {
    String,
    Number,
    StrContains,
    Boolean,
}

/// The catalogue of all questions
#[derive(Serialize, Deserialize)]
pub struct QuestionCatalogue {
    pub catalogue: HashMap<String, QuestionCatalogueElement>,
}

/// An element of the question catalogue
#[derive(Serialize, Deserialize, Clone, Hash)]
pub struct QuestionCatalogueElement {
    pub question: String,
    pub answer: serde_json::Value,
    pub answer_type: AnswerType,
}

/// The assignment entity
#[derive(Queryable, Selectable, AsChangeset, Clone, Serialize)]
#[diesel(table_name = crate::schema::assignments, treat_none_as_null = true)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Assignment {
    pub id: i32,
    pub title: String,
    pub due_date: Option<NaiveDateTime>,
    pub group_id: i32,
    pub description: String,
    pub language: AssignmentLanguage,
    pub completed_by: Vec<Option<i32>>,
    pub file_structure: Option<serde_json::Value>,
    pub runner_cpu: String,
    pub runner_memory: String,
    pub runner_timeout: String,
    pub runner_cmd: String,
    pub question_catalogue: Option<serde_json::Value>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Used to create a new assignment
#[derive(Insertable)]
#[diesel(table_name = crate::schema::assignments, treat_none_as_null = true)]
pub struct CreateAssignment {
    pub title: String,
    pub due_date: Option<NaiveDateTime>,
    pub group_id: i32,
    pub description: String,
    pub language: AssignmentLanguage,
}

pub struct AssignmentRepository;

impl AssignmentRepository {
    /// Creates a new assignment
    pub fn create_assignment(assignment: &CreateAssignment, conn: &mut DB) -> Assignment {
        NotificationRepository::create_notification_for_group(
            "Created assignment".to_string(),
            format!("Created assignment {}", assignment.title.clone()),
            assignment.group_id,
            conn,
        );
        diesel::insert_into(dsl::assignments::table())
            .values(assignment)
            .returning(Assignment::as_returning())
            .get_result::<Assignment>(conn)
            .expect("Cannot create new assignment")
    }

    /// Gets assignments by group_id
    pub fn get_all_group_assignments(
        group_id: i32,
        page: i64,
        conn: &mut DB,
    ) -> PaginatedModel<Assignment> {
        dsl::assignments
            .filter(dsl::group_id.eq(group_id))
            .paginate(page)
            .load_and_count_pages::<Assignment>(conn)
            .expect("Error loading group")
    }

    /// Gets an assignment by ID and group_id
    pub fn get_assignment_by_id_and_group(
        id: i32,
        group_id: i32,
        conn: &mut DB,
    ) -> Option<Assignment> {
        dsl::assignments
            .filter(dsl::id.eq(id).and(dsl::group_id.eq(group_id)))
            .first::<Assignment>(conn)
            .optional()
            .expect("Error loading group")
    }

    /// Gets an assignment by ID
    pub fn get_assignment_by_id(id: i32, conn: &mut DB) -> Option<Assignment> {
        dsl::assignments
            .filter(dsl::id.eq(id))
            .first::<Assignment>(conn)
            .optional()
            .expect("Error loading group")
    }

    /// Updates an assignment
    pub fn update_assignment(assignment: Assignment, conn: &mut DB) {
        NotificationRepository::create_notification_for_group(
            "Updated assignment".to_string(),
            format!("Updated assignment {}", assignment.title.clone()),
            assignment.group_id,
            conn,
        );
        diesel::update(dsl::assignments.filter(dsl::id.eq(assignment.id)))
            .set::<Assignment>(assignment)
            .execute(conn)
            .expect("Cannot update assignment");
    }

    /// Gets all ids of assignments that are assigned to a specific group
    pub fn get_ids_for_group(group_id: i32, conn: &mut DB) -> Vec<i32> {
        dsl::assignments
            .select(dsl::id)
            .filter(dsl::group_id.eq(group_id))
            .get_results::<i32>(conn)
            .expect("Cannot load assignment IDs")
    }
}
