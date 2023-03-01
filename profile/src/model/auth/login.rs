use crate::interface::auth::login::ResultQuery;
use bson::doc;
use mongodb::Client;
use smcore::database::mongo;

pub async fn login_account(
    username: String,
    password: String,
    client: Client,
) -> Result<ResultQuery, String> {
    let result_find = mongo::Data::<ResultQuery>::new(client)
        .database("db1")
        .collection("user_rust")
        .aggregate(vec![doc! {
            "$match": {
                "username": username,
                "password": password
            }
        }])
        .await
        .first;

    let result_find = match result_find {
        Some(data) => data,
        None => return Err("User not found".to_string()),
    };

    Ok(result_find)
}
