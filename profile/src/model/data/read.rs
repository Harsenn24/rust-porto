use crate::{config::Config, interface::data::read::ReadProductById};
use bson::{doc, oid::ObjectId};
use mongodb::Client;
use smcore::database::mongo;

pub async fn detail_list(client: Client, user_id: ObjectId) -> Option<Vec<ReadProductById>> {
    let database = Config::Database.get_str();

    let result = mongo::Data::<ReadProductById>::new(client)
        .database(&database)
        .collection("product")
        .aggregate(vec![
            doc! {
                "$match": {
                    "user_id": user_id
                }
            },
            doc! {
                "$lookup": {
                    "from": "user_rust",
                    "localField": "user_id",
                    "foreignField": "_id",
                    "as": "user",
                    "pipeline": [
                        {
                            "$project" :{
                                "username" : "$username"
                            }
                        }
                    ]
                }
            },
            doc! {
                "$project": {
                    "username": {"$ifNull" : [{"$first" : "$user.username"}, "-"]},
                    "name" : "$name",
                    "price" : "$price"
                }
            },
        ])
        .await
        .vector;

    match result {
        Some(data) => Some(data),
        None => None,
    }
}
