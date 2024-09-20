use actix_web::web;

mod group;


pub fn init_services(cfg: &mut web::ServiceConfig) {
    cfg.service(group::create_group);
}