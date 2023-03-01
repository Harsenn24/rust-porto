use bson::Document;
use serde::{Deserialize, Serialize};
use smcore::hashlib::password;
use std::env;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegisterReqBody {
    pub username: String,
    pub password: String,
}

impl From<RegisterReqBody> for Result<Document, String> {
    fn from(req_payload: RegisterReqBody) -> Result<Document, String> {
        let mut doc = Document::new();

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

        doc.insert("username", req_payload.username);
        doc.insert("password", password_hash);
        Ok(doc)
    }
}

impl RegisterReqBody {
    pub fn check_username(&self) -> Result<String, String> {
        //validate username if length is less than 6
        if self.username.len() < 6 {
            return Err("username must be greater than 6 characters".to_string());
        }

        //validate username if it is alphanumeric
        if !self.username.chars().all(char::is_alphanumeric) {
            return Err("username must be alphanumeric".to_string());
        }

        Ok(self.username.clone())
    }
}
