use super::Paginate;
use super::PaginatedModel;
use super::DB;
use crate::schema::assignment_wishes::dsl;
use chrono::NaiveDateTime;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::Selectable;
use serde::Serialize;

/// Assignment wish entity type
#[derive(Queryable, Selectable, Clone, Serialize)]
#[diesel(table_name = crate::schema::assignment_wishes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AssignmentWish {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub group_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Assignment wish insertion type
#[derive(Insertable)]
#[diesel(table_name = crate::schema::assignment_wishes)]
pub struct CreateAssignmentWish {
    pub title: String,
    pub description: String,
    pub group_id: i32,
}

pub struct AssignmentWishRepository;

impl AssignmentWishRepository {
    /// Creates a new wish in the database
    pub fn create_wish(wish: &CreateAssignmentWish, conn: &mut DB) -> AssignmentWish {
        diesel::insert_into(dsl::assignment_wishes::table())
            .values(wish)
            .returning(AssignmentWish::as_returning())
            .get_result::<AssignmentWish>(conn)
            .expect("Cannot create new assignment wish")
    }

    /// Gets an specific assignment wish by ID
    pub fn get_assignment_wish(id: i32, conn: &mut DB) -> Option<AssignmentWish> {
        dsl::assignment_wishes
            .filter(dsl::id.eq(id))
            .first::<AssignmentWish>(conn)
            .optional()
            .expect("Cannot get assignment wish")
    }

    /// Gets all wishes for a specific group
    pub fn get_wishes_for_group(
        group_id: i32,
        page: i64,
        conn: &mut DB,
    ) -> PaginatedModel<AssignmentWish> {
        dsl::assignment_wishes
            .filter(dsl::group_id.eq(group_id))
            .paginate(page)
            .load_and_count_pages::<AssignmentWish>(conn)
            .expect("Cannot fetch wishes")
    }

    /// Deletes an existing assignment wish
    pub fn delete_wish(wish: &AssignmentWish, conn: &mut DB) {
        diesel::delete(dsl::assignment_wishes.filter(dsl::id.eq(wish.id)))
            .execute(conn)
            .expect("Cannot delete assignment wish");
    }

    /// Gets all assignment wishes for a tutor
    pub fn get_tutor_wishes(
        tutor_id: i32,
        page: i64,
        conn: &mut DB,
    ) -> PaginatedModel<AssignmentWish> {
        dsl::assignment_wishes
            .left_join(crate::schema::groups::table)
            .filter(crate::schema::groups::dsl::tutor.eq(tutor_id))
            .select(AssignmentWish::as_select())
            .paginate(page)
            .load_and_count_pages::<AssignmentWish>(conn)
            .expect("Cannot fetch tutor assignment wishes")
    }
}
