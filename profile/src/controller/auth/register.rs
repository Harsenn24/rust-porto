use actix_web::{ web, HttpResponse, Responder };
use bson::doc;
use mongodb::Client;
use smcore::http::response::json;
use crate::interface::auth::register::RegisterReqBody;
use crate::model::auth::register::{new_account, check_account};

// resgiter from interface resgisterreqbody to document
pub async fn register(
    payload: web::Json<RegisterReqBody>,
    client: web::Data<Client>
) -> impl Responder {

    let payload_username = payload.check_username();

    let payload_username = match payload_username {
        Ok(data) => data,
        Err(e) => return HttpResponse::BadRequest().json(e),
    };

    let result = check_account(payload_username, client.get_ref().clone()).await;

    match result {
        Some(_) => return HttpResponse::BadRequest().json("Username already exists"),
        None => (),
    };

    let payload = Result::from(payload.into_inner());
    
    let payload = match payload {
        Ok(data) => data,
        Err(e) => return HttpResponse::BadRequest().json(e)
    };

    let result = new_account(payload, client.get_ref().clone()).await;

    let success = match result {
        Ok(_) => true,
        Err(e) => return HttpResponse::BadRequest().json(e)
    };

    let response = json::Data {
        data: doc! {
            "success": success
        },
        meta: json::Meta::default(),
    };

    HttpResponse::Ok().json(response)
}