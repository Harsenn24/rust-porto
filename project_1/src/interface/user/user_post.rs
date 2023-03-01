use serde::{ Deserialize, Serialize };
use bson::Document;
use chrono::{ DateTime, Utc };

#[derive(Serialize, Deserialize, Debug)]
pub struct UserReq {
    pub username: String,
    pub birthday: String,
    pub password: String,
    pub email: String,
    pub status: String,
    pub epoch: f64,
    pub full_name: Vec<String>,
}

impl From<UserReq> for Result<User, String> {
    fn from(user: UserReq) -> Result<User, String> {
        let birthday = match DateTime::parse_from_rfc3339(&user.birthday) {
            Ok(date) => date.with_timezone(&Utc),
            Err(e) => {
                return Err(format!("Error parsing date: {}", e));
            }
        };

        //

        //handle awal field birthday untuk di ubah ke dalam format rfc3339

        //data yang di return adalah data yang sudah di ubah ke dalam format rfc3339 DENGAN INTERFACE User bukan UserReq

        Ok(User {
            username: user.username,
            birthday,
            password: user.password,
            email: user.email,
            status: user.status,
            epoch: user.epoch,
            full_name: user.full_name,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub birthday: DateTime<Utc>,
    pub password: String,
    pub email: String,
    pub status: String,
    pub epoch: f64,
    pub full_name: Vec<String>,
}

// impl from user to document with error handling
impl From<User> for Result<Document, String> {
    fn from(user: User) -> Result<Document, String> {
        let mut doc = Document::new();
        doc.insert("username", user.username);
        doc.insert("birthday_date", user.birthday);
        doc.insert("password", user.password);
        doc.insert("email", user.email);
        doc.insert("status", user.status);
        doc.insert("epoch", user.epoch);
        doc.insert("first_name", user.full_name);
        Ok(doc)
    }
}