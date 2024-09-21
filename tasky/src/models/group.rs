use crate::schema::groups::dsl;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::groups)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Group {
    pub id: i32,
    pub title: String,
    pub members: Vec<Option<i32>>,
    pub tutor: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::groups)]
pub struct CreateGroup {
    pub title: String,
    pub tutor: i32,
    pub members: Vec<i32>,
}

#[derive(Clone)]
pub struct GroupRepository;

type DB = PooledConnection<ConnectionManager<PgConnection>>;

impl GroupRepository {
    pub fn get_by_id(id: i32, conn: &mut DB) -> Option<Group> {
        dsl::groups
            .find(id)
            .first::<Group>(conn)
            .optional()
            .expect("Error loading group")
    }

    pub fn get_by_title(title: &String, conn: &mut DB) -> Option<Group> {
        let result = dsl::groups
            .filter(dsl::title.eq(title))
            .get_results::<Group>(conn)
            .expect("Error loading groups");
        if result.len() == 0 {
            return None;
        }
        return Some(result.get(0).unwrap().clone());
    }

    pub fn insert_group(group: CreateGroup, conn: &mut DB) -> Group {
        diesel::insert_into(dsl::groups::table())
            .values(&group)
            .returning(Group::as_returning())
            .get_result::<Group>(conn)
            .expect("Cannot create new group")
    }
}
