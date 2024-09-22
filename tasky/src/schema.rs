// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType, serde::Deserialize)]
    #[diesel(postgres_type(name = "assignment_language"))]
    pub struct AssignmentLanguage;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::AssignmentLanguage;

    assignments (id) {
        id -> Int4,
        #[max_length = 100]
        title -> Varchar,
        due_date -> Timestamp,
        group_id -> Int4,
        description -> Text,
        language -> AssignmentLanguage,
    }
}

diesel::table! {
    group_join_requests (id) {
        id -> Int4,
        requestor -> Int4,
        group_id -> Int4,
    }
}

diesel::table! {
    groups (id) {
        id -> Int4,
        title -> Varchar,
        members -> Array<Nullable<Int4>>,
        tutor -> Int4,
    }
}

diesel::joinable!(assignments -> groups (group_id));
diesel::joinable!(group_join_requests -> groups (group_id));

diesel::allow_tables_to_appear_in_same_query!(assignments, group_join_requests, groups,);
