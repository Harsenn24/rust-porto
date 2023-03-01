use actix_web::{ web, Responder, HttpResponse };
use mongodb::Client;

use crate::{ interface::user::user_post::UserReq, model::user::user_post::create_user };

pub async fn new_user(payload: web::Json<UserReq>, client: web::Data<Client>) -> impl Responder {
    let user = match Result::from(payload.into_inner()) {
        Ok(user) => user,
        Err(e) => {
            return HttpResponse::BadRequest().json(e);
        }
    };

    let db_client = client.get_ref().to_owned();

    let result = create_user(user, db_client).await;
    match result {
        Ok(_) => HttpResponse::Ok().json("data masuk"),
        Err(e) => HttpResponse::InternalServerError().json(e),
    }
}