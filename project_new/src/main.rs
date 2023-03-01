mod controller;
mod interface;
mod model;
mod helper;
mod config;

use actix_web::{ web, App, HttpServer };
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let mongo_env = env::var("MONGO_URI").expect("MONGO_URI must be set");

    let client = mongodb::Client::with_uri_str(&mongo_env).await.expect("failed connect db");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(
                web
                    ::scope("/product")
                    .service(
                        web
                            ::resource("")
                            .route(web::post().to(controller::product::create_product))
                            .route(web::get().to(controller::product::read_product))
                    )
                    .service(
                        web
                            ::resource("/{id}")
                            .route(web::get().to(controller::product::read_detail_product))
                            .route(web::patch().to(controller::product::edit_data_patch))
                            .route(web::put().to(controller::product::edit_data_put))
                            .route(web::delete().to(controller::product::delete_data))
                    )
            )
            .service(
                web
                    ::scope("/upload")
                    .service(
                        web::resource("")
                        .route(web::post().to(controller::upload_image::upload))
                    )
            )
            .service(
                web
                    ::scope("/download")
                    .service(
                        web::resource("")
                        .route(web::get().to(controller::show_image::show))
                    )
            )
    })
        .bind(("localhost", 3000))?
        .run().await
}