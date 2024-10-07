// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, serde::Deserialize, diesel::sql_types::SqlType)]
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
        completed_by -> Array<Nullable<Int4>>,
        file_structure -> Nullable<Jsonb>,
        #[max_length = 5]
        runner_cpu -> Varchar,
        #[max_length = 5]
        runner_memory -> Varchar,
        #[max_length = 5]
        runner_timeout -> Varchar,
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

diesel::table! {
    solutions (id) {
        id -> Int4,
        submitter_id -> Int4,
        assignment_id -> Int4,
        file_structure -> Nullable<Jsonb>,
        #[max_length = 20]
        approval_status -> Nullable<Varchar>,
        #[max_length = 32]
        job_id -> Nullable<Varchar>,
        group_id -> Nullable<Int4>,
    }
}

diesel::joinable!(assignments -> groups (group_id));
diesel::joinable!(group_join_requests -> groups (group_id));
diesel::joinable!(solutions -> assignments (assignment_id));
diesel::joinable!(solutions -> groups (group_id));

diesel::allow_tables_to_appear_in_same_query!(
    assignments,
    group_join_requests,
    groups,
    solutions,
);
