use super::DB;
use crate::schema::assignments::dsl;
use chrono::NaiveDateTime;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::{
    prelude::{Insertable, Queryable},
    Selectable, SelectableHelper,
};
use serde::{Deserialize, Serialize};

/// The language of an assignment
/// This language is the language the assignment should be completed with
#[derive(diesel_derive_enum::DbEnum, Debug, Clone, Deserialize, Serialize)]
#[ExistingTypePath = "crate::schema::sql_types::AssignmentLanguage"]
pub enum AssignmentLanguage {
    Java,
    Golang,
}

/// The assignment entity
#[derive(Queryable, Selectable, AsChangeset, Clone)]
#[diesel(table_name = crate::schema::assignments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Assignment {
    pub id: i32,
    pub title: String,
    pub due_date: NaiveDateTime,
    pub group_id: i32,
    pub description: String,
    pub language: AssignmentLanguage,
}

/// Used to create a new assignment
#[derive(Insertable)]
#[diesel(table_name = crate::schema::assignments)]
pub struct CreateAssignment {
    pub title: String,
    pub due_date: NaiveDateTime,
    pub group_id: i32,
    pub description: String,
    pub language: AssignmentLanguage,
}

pub struct AssignmentRepository;

impl AssignmentRepository {
    /// Creates a new assignment
    pub fn create_assignment(assignment: &CreateAssignment, conn: &mut DB) -> Assignment {
        diesel::insert_into(dsl::assignments::table())
            .values(assignment)
            .returning(Assignment::as_returning())
            .get_result::<Assignment>(conn)
            .expect("Cannot create new assignment")
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

    /// Updates an assignment
    pub fn update_assignment(assignment: Assignment, conn: &mut DB) {
        diesel::update(dsl::assignments.filter(dsl::id.eq(assignment.id)))
            .set::<Assignment>(assignment)
            .execute(conn)
            .expect("Cannot update assignment");
    }
}