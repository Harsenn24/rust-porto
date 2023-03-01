use mongodb::Client;
use smcore::database::mongo;
use crate::interface::product;
use crate::config::DATABASE;
use super::COLLECTION;

pub async fn read_list(client: Client) -> Option<Vec<product::ProductRead>> {
    let result = mongo::Data::<product::ProductRead>
        ::new(client)
        .database(DATABASE)
        .collection(COLLECTION)
        .aggregate(vec![]).await.vector;

    match result {
        Some(data) => Some(data),
        None => None,
    }
}