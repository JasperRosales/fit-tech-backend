use actix_web::{get, post, HttpResponse, Responder};


//TODO replace the functionality with proper crud representation
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello User")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
