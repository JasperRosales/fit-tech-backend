use actix_web::{middleware::Logger, App, HttpServer, web};
use std::env;

mod routes;
mod features;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::from_filename("local.env").ok();
    env_logger::init();

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    println!("Server running at http://{}:{}", host, port);

    let db = utils::db::init_db().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone())) // inject DB
            .configure(routes::config)
            .wrap(Logger::default())
    })
    .bind((host, port))?
    .run()
    .await
}
