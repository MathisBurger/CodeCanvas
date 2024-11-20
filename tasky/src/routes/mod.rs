use actix_web::web;
use chrono::{DateTime, NaiveDateTime};
use serde::{Deserialize, Deserializer};

pub mod assignment;
pub mod assignment_completion;
pub mod assignment_wish;
pub mod code_comment;
pub mod group;
pub mod group_join_request;
pub mod group_member;
pub mod notifications;
pub mod solution;

#[derive(Deserialize)]
pub struct PaginationParams {
    pub page: i64,
}

#[derive(Deserialize)]
pub struct PaginatedParamsWithSearch {
    pub page: i64,
    pub search: Option<String>,
}

/// Initializes all endpoints
pub fn init_services(cfg: &mut web::ServiceConfig) {
    cfg.service(group::create_group)
        .service(group::get_group)
        .service(group::get_all_groups)
        .service(group::get_all_my_groups)
        .service(group::update_group)
        .service(group::get_enlistable_users)
        .service(group::enlist_user)
        .service(group::remove_user)
        .service(group::leave_group)
        .service(group::delete_group)
        .service(group::verify_group)
        .service(group::unverify_group)
        .service(group_member::members_paginated)
        .service(group_join_request::create_join_request)
        .service(group_join_request::get_join_requests)
        .service(group_join_request::approve_join_request)
        .service(group_join_request::reject_join_request)
        .service(assignment::get_all_group_assignments)
        .service(assignment::create_assignment)
        .service(assignment::get_assignment)
        .service(assignment::update_assignment)
        .service(assignment::create_assignment_test)
        .service(assignment::view_assignment_test)
        .service(assignment::create_question_catalogue)
        .service(assignment::update_assignment_test)
        .service(assignment::get_student_pending_assignments)
        .service(assignment_completion::assignment_completions)
        .service(solution::create_solution)
        .service(solution::get_solution)
        .service(solution::get_solutions_for_assignment)
        .service(solution::get_solutions_for_user)
        .service(solution::approve_solution)
        .service(solution::reject_solution)
        .service(solution::get_solution_files)
        .service(solution::get_solutions_for_user_by_id)
        .service(solution::get_tutor_solutions)
        .service(assignment_wish::create_wish)
        .service(assignment_wish::get_wishes)
        .service(assignment_wish::get_wish)
        .service(assignment_wish::delete_wish)
        .service(assignment_wish::tutor_pending_wishes)
        .service(code_comment::get_code_comments)
        .service(code_comment::create_code_comment)
        .service(notifications::get_notifiations)
        .service(notifications::remove_user_from_notification)
        .service(notifications::remove_user_from_all_notifications)
        .service(notifications::create_group_notification)
        .service(notifications::create_system_wide_notifications)
        .service(notifications::delete_system_wide_notifications)
        .service(notifications::get_system_wide_notifications);
}

pub fn deserialize_naive_datetime<'de, D>(
    deserializer: D,
) -> Result<Option<NaiveDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let str_option: Option<String> = Deserialize::deserialize(deserializer).ok();
    if str_option.is_none() {
        return Ok(None);
    }
    let s = str_option.unwrap();
    if let Ok(datetime_with_tz) = DateTime::parse_from_rfc3339(s.as_str()) {
        // Convert to NaiveDateTime by discarding the time zone
        return Ok(Some(datetime_with_tz.naive_utc()));
    }
    NaiveDateTime::parse_from_str(s.as_str(), "%Y-%m-%dT%H:%M:%S")
        .map_err(serde::de::Error::custom)
        .map(Some)
}
