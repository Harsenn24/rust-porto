use bson::{Document, oid::ObjectId};
use serde::{Deserialize, Serialize};
use smcore::hashlib::password;
use std::env;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct LoginReqBody {
    pub username: String,
    pub password: String,
}

// impl from loginreqbody to document
impl From<LoginReqBody> for Result<LoginReqBody, String> {
    fn from(mut req_payload: LoginReqBody) -> Result<LoginReqBody, String> {
        //validate username if length is less than 6
        if req_payload.username.len() < 6 {
            return Err("username must be greater than 6 characters".to_string());
        }

        //validate username if it is alphanumeric
        if !req_payload.username.chars().all(char::is_alphanumeric) {
            return Err("username must be alphanumeric".to_string());
        }

        //validate password if length is less than 6
        if req_payload.password.len() < 6 {
            return Err("password must be greater than 6 characters".to_string());
        }

        //validate password if it is alphanumeric
        if !req_payload.password.chars().all(char::is_alphanumeric) {
            return Err("password must be alphanumeric".to_string());
        }

        let salt = env::var("SALT").unwrap();

        let password_hash = password::new(&req_payload.password, &salt);

        req_payload.password = match password_hash {
            Some(data) => data,
            None => return Err("Password hash not found".to_string()),
        };

        Ok(req_payload)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ResultQuery {
    pub username: String,
    pub user_id: ObjectId,
}

impl From<Document> for ResultQuery {
    fn from(doc: Document) -> Self {
        let username = doc.get("username").unwrap().as_str().unwrap().to_string();
        let user_id = doc.get_object_id("_id").unwrap().clone();
        Self { username, user_id }
    }
}
