use super::DB;
use crate::schema::groups::dsl;
use diesel::associations::HasTable;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// Group entity in the database
#[derive(Queryable, Selectable, AsChangeset, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::groups)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Group {
    pub id: i32,
    pub title: String,
    pub members: Vec<Option<i32>>,
    pub tutor: i32,
}

/// Used to create a group in database
#[derive(Insertable)]
#[diesel(table_name = crate::schema::groups)]
pub struct CreateGroup {
    pub title: String,
    pub tutor: i32,
    pub members: Vec<i32>,
}

pub struct GroupRepository;

impl GroupRepository {
    /// Gets all groups
    pub fn get_all(conn: &mut DB) -> Vec<Group> {
        dsl::groups
            .get_results::<Group>(conn)
            .expect("Error loading groups")
    }

    /// Gets a group by ID
    pub fn get_by_id(id: i32, conn: &mut DB) -> Option<Group> {
        dsl::groups
            .find(id)
            .first::<Group>(conn)
            .optional()
            .expect("Error loading group")
    }

    /// Gets a group by title
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

    /// Inserts a group into the database
    pub fn insert_group(group: CreateGroup, conn: &mut DB) -> Group {
        diesel::insert_into(dsl::groups::table())
            .values(&group)
            .returning(Group::as_returning())
            .get_result::<Group>(conn)
            .expect("Cannot create new group")
    }

    /// Updates the group in the database
    pub fn update_group(group: Group, conn: &mut DB) {
        diesel::update(dsl::groups.filter(dsl::id.eq(group.id)))
            .set::<Group>(group)
            .execute(conn)
            .expect("Cannot update group");
    }
}
