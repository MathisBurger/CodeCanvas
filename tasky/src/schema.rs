// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, serde::Deserialize, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "assignment_language"))]
    pub struct AssignmentLanguage;

    #[derive(diesel::query_builder::QueryId, serde::Deserialize, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "join_request_policy"))]
    pub struct JoinRequestPolicy;
}

diesel::table! {
    assignment_completions (assignment_id, member_id) {
        assignment_id -> Int4,
        member_id -> Int4,
    }
}

diesel::table! {
    assignment_wishes (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        description -> Text,
        group_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::AssignmentLanguage;

    assignments (id) {
        id -> Int4,
        #[max_length = 100]
        title -> Varchar,
        due_date -> Nullable<Timestamp>,
        group_id -> Int4,
        description -> Text,
        language -> AssignmentLanguage,
        file_structure -> Nullable<Jsonb>,
        #[max_length = 5]
        runner_cpu -> Varchar,
        #[max_length = 5]
        runner_memory -> Varchar,
        #[max_length = 5]
        runner_timeout -> Varchar,
        runner_cmd -> Text,
        question_catalogue -> Nullable<Jsonb>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    code_comments (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        content -> Text,
        commentor -> Int4,
        group_id -> Int4,
        solution_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    group_join_requests (id) {
        id -> Int4,
        requestor -> Int4,
        group_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    group_members (group_id, member_id) {
        group_id -> Int4,
        member_id -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::JoinRequestPolicy;

    groups (id) {
        id -> Int4,
        title -> Varchar,
        tutor -> Int4,
        join_policy -> JoinRequestPolicy,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        verified -> Bool,
    }
}

diesel::table! {
    notification_targets (notification_id, user_id) {
        notification_id -> Int4,
        user_id -> Int4,
    }
}

diesel::table! {
    notifications (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        content -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        show_until -> Nullable<Timestamp>,
        system_wide -> Bool,
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
        question_result -> Nullable<Jsonb>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(assignment_completions -> assignments (assignment_id));
diesel::joinable!(assignment_wishes -> groups (group_id));
diesel::joinable!(assignments -> groups (group_id));
diesel::joinable!(code_comments -> groups (group_id));
diesel::joinable!(code_comments -> solutions (solution_id));
diesel::joinable!(group_join_requests -> groups (group_id));
diesel::joinable!(group_members -> groups (group_id));
diesel::joinable!(notification_targets -> notifications (notification_id));
diesel::joinable!(solutions -> assignments (assignment_id));
diesel::joinable!(solutions -> groups (group_id));

diesel::allow_tables_to_appear_in_same_query!(
    assignment_completions,
    assignment_wishes,
    assignments,
    code_comments,
    group_join_requests,
    group_members,
    groups,
    notification_targets,
    notifications,
    solutions,
);
