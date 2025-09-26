use actix_web::web;

use crate::features::{auth, user};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/v1/users").configure(user::config::routes))
      .service(web::scope("v1/auth").configure(auth::config::routes));
}
