use mongodb::Client;
use bson::oid::ObjectId;
use crate::interface::user;
use crate::config::DATABASE;
use crate::model::COLLECTION;

pub async fn create_user(user: user::user_post::User, client: Client) -> Result<ObjectId, String> {
    let doc = Result::from(user)?;

    let db = client.database(DATABASE).collection(COLLECTION).insert_one(doc, None).await;
    let db = match db {
        Ok(doc) => doc,
        Err(e) => {
            return Err(e.to_string());
        }
    };
    match db.inserted_id.as_object_id() {
        Some(id) => Ok(id),
        None => Err("No id found".to_string()),
    }
}

