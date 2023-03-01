mod config;
mod controller;
mod interface;
mod model;

use actix_web::{web, App, HttpServer};
use config::Config;
use controller::{
    auth::{login::login_to, register::register},
    data::{read::read_by_id_token, create_product::create_product}, email::send_email::send_email
};
use mongodb::Client;
use smcore::actix::middleware::auth as auth_middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    config::init();
    let mongo_env = Config::Mongouri.get_str();

    let test_mongo = "mongodb://root:root@mongo:27017/?authSource=admin".to_string();

    let client = Client::with_uri_str(test_mongo)
        .await
        .expect("failed to connect");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(
                web::scope("/login").service(web::resource("").route(web::post().to(login_to))),
            )
            .service(
                web::scope("/register").service(web::resource("").route(web::post().to(register))),
            )
            .service(
                web::resource("/product")
                    .wrap(auth_middleware::Auth)
                    .route(web::get().to(read_by_id_token))
                    .route(web::post().to(create_product))
            )
            .service(
                web::resource("/send-email")
                    .route(web::get().to(send_email))
            )
    })
    .bind(("0.0.0.0", 2000))?
    .run()
    .await
}

#[cfg(test)]
mod testcase {
    mod integration;
    mod unit_test;
}
