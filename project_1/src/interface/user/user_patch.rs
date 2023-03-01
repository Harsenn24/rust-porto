use serde::{ Deserialize, Serialize };
use chrono::NaiveDateTime;
use bson::Document;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserReqPatch {
    pub username: String,
    pub birthday: String,
    pub password: String,
    pub email: String,
    pub status: bool,
    pub full_name: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: Option<String>,
    pub birthday: Option<f64>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub status: Option<bool>,
    pub epoch: Option<f64>,
    pub full_name: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IdParams {
    pub id: String,
}

impl From<UserReqPatch> for Result<User, String> {
    fn from(user: UserReqPatch) -> Result<User, String> {
        let date_string = user.birthday.to_string();

        //change format from date to timestamp
        let timestamp = match NaiveDateTime::parse_from_str(&date_string, "%Y-%m-%d %H:%M:%S") {
            Ok(date) => date.timestamp() as f64,
            Err(e) => {
                return Err(format!("Error parsing date: {}", e));
            }
        };

        //get current timestamp
        let ep_now_timestamp = chrono::Utc::now().timestamp() as f64;

        Ok(User {
            username: Some(user.username),
            birthday: Some(timestamp),
            password: Some(user.password),
            email: Some(user.email),
            status: Some(user.status),
            epoch: Some(ep_now_timestamp),
            full_name: Some(user.full_name),
        })
    }
}

impl From<User> for Result<Document, String> {
    fn from(payload: User) -> Self {
        let mut doc: Document = Document::new();

        match payload.username {
            Some(data) => {
                doc.insert("username", data);
            }
            None => {}
        }

        match payload.birthday {
            Some(data) => {
                doc.insert("birthday_date", data);
            }
            None => {}
        }

        match payload.password {
            Some(data) => {
                doc.insert("password", data);
            }
            None => {}
        }

        match payload.email {
            Some(data) => {
                doc.insert("email", data);
            }
            None => {}
        }

        match payload.status {
            Some(data) => {
                doc.insert("status", data);
            }
            None => {}
        }

        match payload.epoch {
            Some(data) => {
                doc.insert("epoch", data);
            }
            None => {}
        }

        match payload.full_name {
            Some(data) => {
                doc.insert("full_name", data);
            }
            None => {}
        }

        Ok(doc)
    }
}