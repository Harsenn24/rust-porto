use actix_web::{ web, Responder, HttpResponse };
use mongodb::Client;
use smcore::http::response::json;
use crate::model::user::user_get::get_user;

pub async fn read_user(
    client: web::Data<Client>
) -> impl Responder {

    let result = get_user( client.get_ref().clone()).await;

    let success = match result {
        Some(data) => data,
        None => return HttpResponse::NoContent().json(()),
    };

    let response = json::Data {
        data :success,
        meta: json::Meta::default(),
        status_code: 200
    };

    HttpResponse::Ok().json(response)
}