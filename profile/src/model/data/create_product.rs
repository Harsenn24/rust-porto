use bson::{ Document, oid::ObjectId };
use mongodb::Client;

use crate::config::Config;

pub async fn newproduct(payload: Document, client: Client) -> Result<ObjectId, String> {
    let database = Config::Database.get_str();

    let result = client
        .database(&database)
        .collection::<Document>("product")
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