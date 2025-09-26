use actix_web::{get, HttpResponse, Responder};


//TODO implement database configuration
//TODO implement jwt authentication

#[get("/signin")]
async fn signin() -> impl Responder{
    HttpResponse::Ok().body("Login Successful")
}

#[get("/register")]
async fn register() -> impl Responder{
    HttpResponse::Ok().body("Register Successful")
}

#[get("/signout")]
async fn signout() -> impl Responder{
    HttpResponse::Ok().body("Signout Successful")
}





