use crate::interface::auth::login::LoginReqBody;
use crate::model::auth::login::login_account;
use actix_web::{web, HttpResponse, Responder};
use bson::doc;
use mongodb::Client;
use smcore::hashlib::token;
use smcore::http::response::json;
use std::env;

// resgiter from interface resgisterreqbody to document
pub async fn login_to(
    payload: web::Json<LoginReqBody>,
    client: web::Data<Client>,
) -> impl Responder {

    //get payload into inner
    let payload_result = Result::from(payload.into_inner());

    let result = match payload_result {
        Ok(data) => data,
        Err(e) => return HttpResponse::BadRequest().json(e),
    };

    let username = result.username;
    let password = result.password;

    let result = login_account(username, password, client.get_ref().clone()).await;

    let result = match result {
        Ok(data) => data,
        Err(e) => return HttpResponse::BadRequest().json(e),
    };

    let user_id = result.user_id;

    let token_payload = token::Data {
        user_id: Some(user_id),
        username: result.username,
        email: String::new(),
        token: String::new(),
        store_id: None,
        fullname: String::new(),
        origin: token::AuthType::User,
    };

    let salt = env::var("TOKEN_SECRET").expect("TOKEN_SECRET must be set.");

    let sign_token = token::sign(token_payload, &salt);

    let sign_token = match sign_token {
        Some(data) => format!("{} {}", token::AuthType::User.to_string(), data),
        None => return HttpResponse::BadRequest().json("Token not found"),
    };
        
    let response = json::Data {
        data: doc! {
            "token": sign_token
        },
        meta: json::Meta::default(),
    };

    HttpResponse::Ok().json(response)
}
