use actix_web::{ web, HttpResponse, Responder };
use bson::doc;
use bson::oid::ObjectId;
use mongodb::Client;
use smcore::http::response::json;


use crate::{interface::user::user_put::{IdParams, UserReqPut}, model::user::user_put::put_data};

pub async fn edit_data_put(
    client: web::Data<Client>,
    payload_id: web::Path<IdParams>,
    payload_data: web::Json<UserReqPut>
) -> impl Responder {
    //convert product_id to ObjectId
    let id_user = match ObjectId::parse_str(&payload_id.id.clone()) {
        Ok(data) => data,
        Err(_) => {
            return HttpResponse::NotModified().json("id product not valid");
        }
    };

    let document_to_save = Result::from(payload_data.into_inner());



    //match document_to_save

    let payload_data = match document_to_save {
        Ok(data) => {
            let data = match Result::from(data) {
              Ok(data)=>data,
              Err(e)=>  return HttpResponse::NotModified().json(())
            };
            data
          },
        Err(e) => {
            tracing::info!("your error is: {:?}", e);
            return HttpResponse::NotModified().json("data not valid");
        }
    };




    let result = put_data(payload_data, client.get_ref().clone(), id_user).await;

    let success = match result {
        Ok(_) => true,
        Err(e) => {
            tracing::info!("your error is: {:?}", e);
            return HttpResponse::NotModified().json(e);
        }
    };

    let response = json::Data {
        data: doc! {
            "success": success
        },
        meta: json::Meta::default(),
        status_code: 200,
    };

    HttpResponse::Ok().json(response)
}