use actix_web::web;
use super::handler::{hello, echo};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(hello)
       .service(echo);
}