use crate::schema::solutions::dsl;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::{
    prelude::{Insertable, Queryable},
    Selectable,
};

use super::DB;

#[derive(Queryable, Selectable, AsChangeset, Clone)]
#[diesel(table_name = crate::schema::solutions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Solution {
    pub id: i32,
    pub submitter_id: i32,
    pub assignment_id: i32,
    pub approved_by_tutor: bool,
    pub file_structure: Option<serde_json::Value>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::solutions)]
pub struct NewSolution {
    pub submitter_id: i32,
    pub assignment_id: i32,
    pub approved_by_tutor: bool,
}

pub struct SolutionRepository;

impl SolutionRepository {
    pub fn get_solutions_for_assignment(id: i32, conn: &mut DB) -> Vec<Solution> {
        dsl::solutions
            .filter(dsl::assignment_id.eq(id))
            .get_results::<Solution>(conn)
            .expect("Cannot fetch solutions")
    }

    pub fn get_solutions_for_user(id: i32, conn: &mut DB) -> Vec<Solution> {
        dsl::solutions
            .filter(dsl::submitter_id.eq(id))
            .get_results::<Solution>(conn)
            .expect("Cannot fetch solutions")
    }

    pub fn update_solution(solution: Solution, conn: &mut DB) {
        diesel::update(dsl::solutions.filter(dsl::id.eq(solution.id)))
            .set::<Solution>(solution)
            .execute(conn)
            .expect("Cannot update solution");
    }

    pub fn create_solution(new: NewSolution, conn: &mut DB) -> Solution {
        diesel::insert_into(dsl::solutions::table())
            .values(new)
            .returning(Solution::as_returning())
            .get_result::<Solution>(conn)
            .expect("Cannot create new solution")
    }
}
