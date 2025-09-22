use actix_web::{App, HttpServer};
mod routes;
mod features;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(routes::config))
        .bind(("127.0.0.1", 8989))?
        .run()
        .await
}

