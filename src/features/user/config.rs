use actix_web::web;
use super::handler::{get_users};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users);
}