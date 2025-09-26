use actix_web::{middleware::Logger, App, HttpServer};
use std::env;

mod routes;
mod features;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::from_filename("local.env").ok();
    env_logger::init();

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8989".to_string())
        .parse()
        .expect("PORT must be a valid number");

    println!("Rust Server running at http://{}:{}", host, port);

    HttpServer::new(|| {
        App::new()
            .configure(routes::config)
            .wrap(Logger::default()) 
    })
    .bind((host, port))?
    .run()
    .await
}

