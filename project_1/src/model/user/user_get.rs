use mongodb::Client;
use smcore::database::mongo;
use crate::model::COLLECTION;
use crate::interface::user;
use crate::config::DATABASE;

pub async fn get_user(client: Client) -> Option<Vec<user::user_get::Userget>> {
    let result = mongo::Data::<user::user_get::Userget>
        ::new(client)
        .database(DATABASE)
        .collection(COLLECTION)
        .aggregate(vec![]).await.vector;

    match result {
        Some(data) => Some(data),
        None => None,
    }
}