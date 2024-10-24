use actix_web::web;

pub mod assignment;
pub mod group;
pub mod group_join_request;
pub mod solution;

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
        .service(solution::create_solution)
        .service(solution::get_solution)
        .service(solution::get_solutions_for_assignment)
        .service(solution::get_solutions_for_user)
        .service(solution::approve_solution)
        .service(solution::reject_solution)
        .service(solution::get_solution_files);
}
