use bson::{doc, oid::ObjectId, Document};
use mongodb::Client;
use smcore::database::mongo;

use crate::{interface::data::read::ReadProductById, config::Config};

pub async fn new_account(payload: Document, client: Client) -> Result<ObjectId, String> {
    let result = client
        .database("db1")
        .collection::<Document>("user_rust")
        .insert_one(payload, None)
        .await;

    let result_insert = match result {
        Ok(data) => data,
        Err(err) => {
            tracing::error!("Error from create account: {}", err);
            return Err(err.to_string());
        }
    };

    match result_insert.inserted_id.as_object_id() {
        Some(data) => Ok(data),
        None => {
            tracing::error!("Error from create account");
            Err("Error".to_string())
        }
    }
}

pub async fn check_account(username: String, client: Client) -> Option<ReadProductById> {
    let database = Config::Database.get_str();
    let result = mongo::Data::<ReadProductById>::new(client)
        .database(&database)
        .collection("user_rust")
        .aggregate(vec![doc! {
            "$match": {
                "username": username
            }
        }])
        .await
        .first;

    match result {
        Some(data) => Some(data),
        None => None,
    }
}
