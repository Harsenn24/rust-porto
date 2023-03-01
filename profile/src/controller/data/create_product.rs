use actix_web::{web, HttpResponse, Responder};
use mongodb::Client;
use smcore::hashlib::token;

use crate::{interface::data::create_product::{CreateProduct, NewProduct}, model::data::create_product::newproduct};


pub async fn create_product (
    token_data: web::ReqData<token::Data>,
    client: web::Data<Client>,
    payload: web::Json<CreateProduct>,
)-> impl Responder {
    let user_id = token_data.user_id.unwrap();
    let payload = payload.into_inner();

    let payload = NewProduct::convert(payload, user_id);

    let result = newproduct(payload, client.get_ref().clone()).await;

    match result {
        Ok(_) => HttpResponse::Ok().json("Success Create Product"),
        Err(_) => HttpResponse::BadRequest().json("Error from create product controller"),
    }
}