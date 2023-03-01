use bson::{ doc, Document, oid::ObjectId };
use mongodb::Client;

use crate::config::DATABASE;
use super::COLLECTION;

pub async fn update_patch(data: Document, client: Client, product_id: ObjectId) -> Result<(), String> {
    let result = client
        .database(DATABASE)
        .collection::<Document>(COLLECTION)
        .update_one(doc! { "_id" : product_id }, doc! { "$set" : data }, None).await;

    let res = match result {
        Ok(data)=> data,
        Err(err) => return Err(err.to_string())
    };

    if res.modified_count == 0 {
        return Err("Data is empty".to_string())
    }; 

    Ok(())
    
}