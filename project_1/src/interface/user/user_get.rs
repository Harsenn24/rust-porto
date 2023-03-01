use serde::{ Deserialize, Serialize };
use bson::Document;

#[derive(Serialize, Deserialize, Debug)]
pub struct Userget {
    pub id: String,
    pub username: String,
    pub birthday: i32,
    pub password: String,
    pub email: String,
    pub status: bool,
    pub epoch: f64,
    pub full_name: Vec<String>,
}

impl From<Document> for Userget {
    fn from(doc: Document) -> Self {
        let id: String = doc.get_object_id("_id").expect("error id").to_hex();
        let username = doc.get_str("username").expect("error username").to_string();
        let birthday = doc.get_i32("birthday_date").expect("error birthday");
        let password = doc.get_str("password").expect("error password").to_string();
        let email = doc.get_str("email").expect("error email").to_string();
        let status = doc.get_bool("status").expect("error status");
        let epoch = doc.get_f64("epoch").expect("error epoch");
        let full_name: Vec<String> = doc
            .get_array("full_name")
            .expect("error full_name")
            .iter()
            .map(|x| x.as_str().unwrap().to_string())
            .collect();

        Userget {
            id,
            username,
            birthday,
            password,
            email,
            status,
            epoch,
            full_name,
        }
    }
}