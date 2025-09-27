use actix_web::{get, web, HttpResponse, Responder};
use sea_orm::DatabaseConnection;
use crate::features::user::model::{Entity as UserEntity};
use crate::features::user::dto::UserResponse;
use sea_orm::EntityTrait;


#[get("/")]
pub async fn get_users(db: web::Data<DatabaseConnection>) -> impl Responder {
    let users_result = UserEntity::find()
        .all(db.get_ref())
        .await;

    match users_result {
        Ok(users) => {
            let response: Vec<UserResponse> = users.into_iter()
                .map(UserResponse::from)
                .collect();
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            eprintln!("DB error: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
