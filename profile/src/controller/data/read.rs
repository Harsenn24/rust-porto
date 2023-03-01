use actix_web::{web, HttpResponse, Responder};
use smcore::hashlib::token;
use mongodb::Client;

use crate::model::data::read::detail_list;
use smcore::http::response::json;


pub async fn read_by_id_token (
    payload: web::ReqData<token::Data>,
    client: web::Data<Client>,
) -> impl Responder {
    let user_id = payload.user_id.unwrap();

    let result = detail_list(client.get_ref().clone(), user_id).await;

    let result = match result {
        Some(data) => data,
        None => return HttpResponse::BadRequest().json("User not found"),
    };

    let response = json::Data {
        data: result,
        meta: json::Meta::default(),
    };


    HttpResponse::Ok().json(response)
}