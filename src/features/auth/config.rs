use actix_web::web;
use super::handler::{signin, register, signout};

pub fn routes(cfg: &mut web::ServiceConfig){
    cfg.service(signin)
      .service(signout)
      .service(register);
}