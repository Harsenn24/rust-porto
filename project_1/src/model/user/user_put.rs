use bson::{ doc, Document, oid::ObjectId };
use mongodb::Client;

use crate::config::DATABASE;
use crate::model::COLLECTION;

pub async fn put_data(data: Document, client: Client, id_user: ObjectId) -> Result<(), String> {
    let result = client
        .database(DATABASE)
        .collection::<Document>(COLLECTION)
        .update_one(doc! { "_id" : id_user }, doc! { "$set" : data }, None).await;

    let res = match result {
        Ok(data)=> data,
        Err(err) => return Err(err.to_string())
    };

    if res.modified_count == 0 {
        return Err("Data is empty".to_string())
    }; 

    Ok(())
    
}