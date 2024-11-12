use actix_web::web;
use serde::Deserialize;

pub mod assignment;
pub mod assignment_wish;
pub mod code_comment;
pub mod group;
pub mod group_join_request;
pub mod notifications;
pub mod solution;

#[derive(Deserialize)]
pub struct PaginationParams {
    pub page: i64,
}

/// Initializes all endpoints
pub fn init_services(cfg: &mut web::ServiceConfig) {
    cfg.service(group::create_group)
        .service(group::get_group)
        .service(group::get_all_groups)
        .service(group::get_all_my_groups)
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
        .service(solution::create_solution)
        .service(solution::get_solution)
        .service(solution::get_solutions_for_assignment)
        .service(solution::get_solutions_for_user)
        .service(solution::approve_solution)
        .service(solution::reject_solution)
        .service(solution::get_solution_files)
        .service(assignment_wish::create_wish)
        .service(assignment_wish::get_wishes)
        .service(assignment_wish::get_wish)
        .service(assignment_wish::delete_wish)
        .service(code_comment::get_code_comments)
        .service(code_comment::create_code_comment)
        .service(notifications::create_notification)
        .service(notifications::get_notifiations)
        .service(notifications::remove_user_from_notification)
        .service(notifications::remove_user_from_all_notifications);
}
