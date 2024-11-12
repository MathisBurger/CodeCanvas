use diesel::BoolExpressionMethods;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel_full_text_search::plainto_tsquery;
use diesel_full_text_search::{to_tsquery, to_tsvector, TsVectorExtensions};
use serde::Serialize;

use crate::auth_middleware::UserData;
use crate::security::StaticSecurity;
use crate::{
    models::{assignment::Assignment, group::Group, DB},
    schema::{assignments, groups},
};

#[derive(Serialize)]
pub struct SpotlightGroup {
    pub id: i32,
    pub title: String,
}

#[derive(Serialize)]
pub struct SpotlightAssignment {
    pub id: i32,
    pub title: String,
    pub group_id: i32,
}

/// Gets all groups that match search
pub fn groups(search: &String, user_data: &UserData, conn: &mut DB) -> Vec<SpotlightGroup> {
    let admin_precicate = to_tsvector(groups::title).matches(to_tsquery(search));

    let default_predicate = admin_precicate
        .clone()
        .and(groups::id.eq_any(user_data.groups.clone()));

    let results =
        match StaticSecurity::is_granted(crate::security::StaticSecurityAction::IsAdmin, user_data)
        {
            true => groups::table
                .filter(admin_precicate)
                .limit(50)
                .get_results::<Group>(conn),
            false => groups::table
                .filter(default_predicate)
                .limit(50)
                .get_results::<Group>(conn),
        };

    results
        .expect("Cannot load spotlight groups")
        .into_iter()
        .map(|x| SpotlightGroup {
            id: x.id,
            title: x.title,
        })
        .collect()
}

/// Gets all assignments that match search
pub fn assignments(
    search: &String,
    user_data: &UserData,
    conn: &mut DB,
) -> Vec<SpotlightAssignment> {
    let admin_predicate = to_tsvector(assignments::description)
        .matches(to_tsquery(search))
        .or(to_tsvector(assignments::title).matches(to_tsquery(search)));

    let default_predicate = assignments::group_id
        .eq_any(user_data.groups.clone())
        .and(admin_predicate.clone());

    let results =
        match StaticSecurity::is_granted(crate::security::StaticSecurityAction::IsAdmin, user_data)
        {
            true => assignments::table
                .filter(admin_predicate)
                .limit(50)
                .get_results::<Assignment>(conn),
            false => assignments::table
                .filter(default_predicate)
                .limit(50)
                .get_results::<Assignment>(conn),
        };

    results
        .expect("Cannot load spotlight assignments")
        .into_iter()
        .map(|x| SpotlightAssignment {
            id: x.id,
            title: x.title,
            group_id: x.group_id,
        })
        .collect()
}
