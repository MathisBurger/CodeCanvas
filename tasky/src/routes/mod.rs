use actix_web::web;

mod group;

pub fn init_services(cfg: &mut web::ServiceConfig) {
    cfg.service(group::create_group)
        .service(group::get_group)
        .service(group::create_join_request)
        .service(group::get_join_requests)
        .service(group::get_all_groups);
}
