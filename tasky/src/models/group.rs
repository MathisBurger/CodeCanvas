use super::group_join_request::GroupJoinRequestRepository;
use super::Paginate;
use super::{PaginatedModel, DB};
use crate::schema::group_members;
use crate::schema::groups::dsl;
use chrono::NaiveDateTime;
use diesel::debug_query;
use diesel::dsl::count_star;
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::{associations::HasTable, dsl::not};
use log::info;
use serde::{Deserialize, Serialize};

#[derive(diesel_derive_enum::DbEnum, Debug, Clone, Deserialize, Serialize, PartialEq)]
#[ExistingTypePath = "crate::schema::sql_types::JoinRequestPolicy"]
pub enum JoinRequestPolicy {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "request")]
    Request,
    #[serde(rename = "closed")]
    Closed,
}

/// Group entity in the database
#[derive(Queryable, Selectable, AsChangeset, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::groups)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Group {
    pub id: i32,
    pub title: String,
    pub tutor: i32,
    pub join_policy: JoinRequestPolicy,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub verified: bool,
}

/// Used to create a group in database
#[derive(Insertable)]
#[diesel(table_name = crate::schema::groups)]
pub struct CreateGroup {
    pub title: String,
    pub tutor: i32,
    pub join_policy: JoinRequestPolicy,
}

pub struct GroupRepository;

impl GroupRepository {
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
        if result.is_empty() {
            return None;
        }
        Some(result.first().unwrap().clone())
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

    /// Gets all groups a user is not member or tutor of
    pub fn get_groups_for_member(member_id: i32, conn: &mut DB) -> Vec<Group> {
        dsl::groups
            .left_join(group_members::table)
            .filter(
                dsl::tutor
                    .eq(member_id)
                    .or(group_members::member_id.eq(member_id)),
            )
            .select(Group::as_select())
            .get_results::<Group>(conn)
            .expect("Cannot fetch groups for member")
    }

    /// Gets all groups a user is not member or tutor of
    pub fn get_groups_for_member_paginated(
        member_id: i32,
        page: i64,
        conn: &mut DB,
    ) -> PaginatedModel<Group> {
        let result = dsl::groups
            .left_join(group_members::table)
            .filter(
                dsl::tutor
                    .eq(member_id)
                    .or(group_members::member_id.eq(member_id)),
            )
            .select(Group::as_select())
            .group_by((dsl::id, dsl::verified))
            .order(dsl::verified.desc())
            .paginate(page)
            .load_and_count_pages::<Group>(conn);
        if let Ok(model) = result {
            model
        } else {
            PaginatedModel {
                results: vec![],
                total: 0,
                page,
            }
        }
    }

    /// Gets all groups a user is no member or tutor of
    pub fn get_groups_for_not_member(
        member_id: i32,
        page: i64,
        search: Option<String>,
        conn: &mut DB,
    ) -> PaginatedModel<Group> {
        let requested: Vec<i32> = GroupJoinRequestRepository::get_user_requests(member_id, conn)
            .into_iter()
            .map(|x| x.group_id)
            .collect();

        let base_predicate = not(dsl::tutor.eq(member_id).or(dsl::id.eq_any(requested)).or(
            group_members::dsl::member_id
                .eq(member_id)
                .and(group_members::dsl::group_id.is_not_null()),
        ));

        let total_base_query = dsl::groups
            .left_join(group_members::dsl::group_members)
            .select(count_star())
            .filter(base_predicate.clone())
            .into_boxed();

        let sql_string = debug_query::<Pg, _>(&total_base_query).to_string();
        println!("{}", sql_string);

        let total = match search.clone() {
            None => total_base_query
                .get_result::<i64>(conn)
                .expect("Result cannot be fetched"),
            Some(search_value) => total_base_query
                .filter(dsl::title.like(format!("%{}%", search_value)))
                .get_result::<i64>(conn)
                .expect("Result cannot be fetched"),
        };

        let results_base_query = dsl::groups
            .left_join(group_members::dsl::group_members)
            .group_by((dsl::id, dsl::verified))
            .select(Group::as_select())
            .filter(base_predicate)
            .order(dsl::verified.desc())
            .limit(50)
            .offset((page - 1) * 50)
            .into_boxed();

        let results = match search {
            None => results_base_query.load::<Group>(conn),
            Some(search_value) => results_base_query
                .filter(dsl::title.like(format!("%{}%", search_value)))
                .load::<Group>(conn),
        };

        if let Err(e) = results {
            info!("Error from database: {}", e);
            return PaginatedModel {
                total: 0,
                results: vec![],
                page,
            };
        }

        PaginatedModel {
            total,
            results: results.unwrap(),
            page,
        }
    }

    /// Delete group
    pub fn delete_group(group_id: i32, conn: &mut DB) {
        diesel::delete(dsl::groups.filter(dsl::id.eq(group_id)))
            .execute(conn)
            .expect("Cannot delete group");
    }
}
