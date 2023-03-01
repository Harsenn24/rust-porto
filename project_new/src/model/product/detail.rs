use bson::{oid::ObjectId, doc};
use mongodb::Client;
use smcore::database::mongo;

use crate::interface::product;
use crate::config::DATABASE;
use super::COLLECTION;

pub async fn detail_list(client: Client, product_id: ObjectId) -> Option<product::ProductRead> {
    let result = mongo::Data::<product::ProductRead>
        ::new(client)
        .database(DATABASE)
        .collection(COLLECTION)
        .aggregate(vec![
            doc! {
                "$match": {
                    "_id": product_id
                }
            }
        ]).await.first;

        tracing ::debug!("result {}", result.is_some());

    match result {
        Some(data) => Some(data),
        None => None,
    }
}