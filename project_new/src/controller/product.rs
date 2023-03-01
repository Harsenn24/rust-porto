use actix_web::{ web, HttpResponse, Responder };
use bson::{ Document, doc };
use bson::oid::ObjectId;
use mongodb::Client;
use smcore::hashlib::latte;
use smcore::http::response::json;
use crate::model::product::create;
use crate::interface::product::{ self, ProductFrom, ProductPatchRequest, ProductRequest };
use crate::model::product::delete::delete_data_model;
use crate::model::product::detail::detail_list;
use crate::model::product::list::read_list;
use crate::model::product::update::put_data;
use crate::interface::product::IdParams;
use crate::model::product::update_2::update_patch;

pub async fn create_product(
    payload: web::Json<product::ProductNew>,
    client: web::Data<Client>,
) -> impl Responder {
    let payload: Document = ProductFrom::<product::ProductNew>::convert(
        payload.into_inner(),
        ObjectId::new()
    );

    let result = create::newproduct(payload, client.get_ref().clone()).await;

    let success = match result {
        Ok(_) => true,
        Err(_) => false,
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

pub async fn read_product(client: web::Data<Client>) -> impl Responder {
    let result = read_list(client.get_ref().clone()).await;

    let success = match result {
        Some(data) => data,
        None => {
            return HttpResponse::NoContent().json(());
        }
    };

    let response = json::Data {
        data: success,
        meta: json::Meta::default(),
        status_code: 200,
    };

    HttpResponse::Ok().json(response)
}

pub async fn read_detail_product(
    client: web::Data<Client>,
    mut payload: web::Path<IdParams>
) -> impl Responder {
    let product_id = latte::Hash::new(payload.id.clone(), 12).decrypt();

    payload.id = product_id;
    //convert product_id to ObjectId
    let product_id = match ObjectId::parse_str(&payload.id) {
        Ok(data) => data,
        Err(_) => {
            return HttpResponse::BadRequest().json("id product not valid");
        }
    };

    let result = detail_list(client.get_ref().clone(), product_id).await;

    let success = match result {
        Some(data) => data,
        None => {
            return HttpResponse::NoContent().json(());
        }
    };

    let response = json::Data {
        data: success,
        meta: json::Meta::default(),
        status_code: 200,
    };

    HttpResponse::Ok().json(response)
}

pub async fn edit_data_patch(
    client: web::Data<Client>,
    payload_id: web::Path<IdParams>,
    payload_data: web::Json<ProductPatchRequest>
) -> impl Responder {
    let product_id = latte::Hash::new(payload_id.id.clone(), 12).decrypt();

    //convert product_id to ObjectId
    let product_id = match ObjectId::parse_str(&product_id) {
        Ok(data) => data,
        Err(_) => {
            return HttpResponse::NotModified().json("id product not valid");
        }
    };

    let document_to_save = Result::from(payload_data.into_inner());

    //match document_to_save

    let payload_data = match document_to_save {
        Ok(data) => data,
        Err(e) => {
            tracing::info!("your error is: {:?}", e);
            return HttpResponse::NotModified().json("data not valid");
        }
    };

    let result = put_data(payload_data, client.get_ref().clone(), product_id).await;

    let success = match result {
        Ok(_) => true,
        Err(e) => {
            tracing::info!("your error is: {:?}", e);
            return HttpResponse::NotModified().json(e);
        },
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



pub async fn edit_data_put(
    client: web::Data<Client>,
    payload_id: web::Path<IdParams>,
    payload_data: web::Json<ProductRequest>
) -> impl Responder {
    let product_id = latte::Hash::new(payload_id.id.clone(), 12).decrypt();

    //convert product_id to ObjectId
    let product_id = match ObjectId::parse_str(&product_id) {
        Ok(data) => data,
        Err(_) => {
            return HttpResponse::NotModified().json("id product not valid");
        }
    };

    let document_to_save = Result::from(payload_data.into_inner());

    //match document_to_save

    let payload_data = match document_to_save {
        Ok(data) => data,
        Err(e) => {
            tracing::info!("your error is: {:?}", e);
            return HttpResponse::NotModified().json("data not valid");
        }
    };

    let result = update_patch(payload_data, client.get_ref().clone(), product_id).await;

    let success = match result {
        Ok(_) => true,
        Err(e) => {
            tracing::info!("your error is: {:?}", e);
            return HttpResponse::NotModified().json(e);
        },
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


pub async fn delete_data(
    client: web::Data<Client>,
    payload_id: web::Path<IdParams>,
) -> impl Responder {
    let product_id = latte::Hash::new(payload_id.id.clone(), 12).decrypt();

    //convert product_id to ObjectId
    let product_id = match ObjectId::parse_str(&product_id) {
        Ok(data) => data,
        Err(_) => {
            return HttpResponse::BadRequest().json("id product not valid");
        }
    };


    let result = delete_data_model(client.get_ref().clone(), product_id).await;

    let success = match result {
        Ok(_) => true,
        Err(e) => {
            tracing::info!("your error is: {:?}", e);
            return HttpResponse::BadRequest().json(e);
        },
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