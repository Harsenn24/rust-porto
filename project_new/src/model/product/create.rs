use bson::{ Document, oid::ObjectId };
use mongodb::Client;
use super::COLLECTION;
use crate::config::DATABASE;

pub async fn newproduct(payload: Document, client: Client) -> Result<ObjectId, String> {
    let result = client
        .database(DATABASE)
        .collection::<Document>(COLLECTION)
        .insert_one(payload, None).await;

    let result_insert = match result {
        Ok(data) => data,
        Err(err) => {
            tracing::error!("Error from create product: {}", err);
            return Err(err.to_string());
        }
    };

    match result_insert.inserted_id.as_object_id() {
        Some(data) => Ok(data),
        None => {
            tracing::error!("Error from create product");
            Err("Error".to_string())
        }
    }
}