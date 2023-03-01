use serde::{ Deserialize, Serialize };
use chrono::NaiveDateTime;
use bson::Document;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserReqPut {
    pub username: String,
    pub birthday_date: String,
    pub password: String,
    pub email: String,
    pub status: bool,
    pub full_name: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserPut {
    pub username: String,
    pub birthday_date: f64,
    pub password: String,
    pub email: String,
    pub status: bool,
    pub epoch: f64,
    pub full_name: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IdParams {
    pub id: String,
}

impl From<UserReqPut> for Result<UserPut, String> {
    fn from(user: UserReqPut) -> Result<UserPut, String> {
        let date_string = user.birthday_date.to_string();

        //change format from date to timestamp
        let timestamp = match NaiveDateTime::parse_from_str(&date_string, "%Y-%m-%d %H:%M:%S") {
            Ok(date) => date.timestamp() as f64,
            Err(e) => {
                return Err(format!("Error parsing date: {}", e));
            }
        };

        //get current timestamp
        let ep_now_timestamp = chrono::Utc::now().timestamp() as f64;

        Ok(UserPut {
            username: user.username,
            birthday_date: timestamp,
            password: user.password,
            email: user.email,
            status: user.status,
            epoch: ep_now_timestamp,
            full_name: user.full_name,
        })
    }
}

impl From<UserPut> for Result<Document, String> {
    fn from(payload: UserPut) -> Self {
        let mut doc: Document = Document::new();

        doc.insert("username", payload.username);
        doc.insert("birthday_date", payload.birthday_date);
        doc.insert("password", payload.password);
        doc.insert("email", payload.email);
        doc.insert("status", payload.status);
        doc.insert("epoch", payload.epoch);
        doc.insert("full_name", payload.full_name);
        

        Ok(doc)
    }
}