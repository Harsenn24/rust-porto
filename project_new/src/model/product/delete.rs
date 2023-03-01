use bson::{ doc, Document, oid::ObjectId };
use mongodb::Client;

use crate::config::DATABASE;
use super::COLLECTION;

pub async fn delete_data_model(client: Client, product_id: ObjectId) -> Result<(), String> {
    //delete one with mongo and actix
    let result = client
        .database(DATABASE)
        .collection::<Document>(COLLECTION)
        .delete_one(doc! { "_id" : product_id }, None).await;

    let result_final = match result {
        Ok(data) => data,
        Err(e) => {
            return Err(e.to_string());
        }
    };


    if result_final.deleted_count == 0 {
        return Err("Data is empty".to_string());
    };

    Ok(())

}