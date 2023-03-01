mod interface;
mod model;
mod controller;
mod config;

use controller::user::{ user_post::new_user, user_get::read_user, user_patch::edit_data_patch, user_put::edit_data_put };
use dotenv::dotenv;
use actix_web::{ web, App, HttpServer };
use std::env;
use mongodb::Client;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let mongo_env = env::var("mongo_uri").expect("MONGO_URI must be set.");
    let client = Client::with_uri_str(mongo_env).await.expect("failed to connect");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(
                web
                    ::scope("/user")
                    .service(
                        web
                            ::resource("")
                            .route(web::post().to(new_user))
                            .route(web::get().to(read_user))
                    )
                    .service(
                        web
                        ::resource("/{id}")
                            .route(web::patch().to(edit_data_patch))
                            .route(web::put().to(edit_data_put))

                    )
            )
    })
        .bind(("0.0.0.0", 2000))?
        .run().await
}