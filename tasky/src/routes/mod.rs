use actix_web::web;

mod assignment;
mod group;

pub fn init_services(cfg: &mut web::ServiceConfig) {
    cfg.service(group::create_group)
        .service(group::get_group)
        .service(group::create_join_request)
        .service(group::get_join_requests)
        .service(group::approve_join_request)
        .service(group::reject_join_request)
        .service(group::get_all_groups)
        .service(assignment::create_assignment)
        .service(assignment::get_assignment)
        .service(assignment::update_assignment);
}
