use bson::Document;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ReadProductById {
    name: String,
    price: f64,
    username: String,
}

impl From<Document> for ReadProductById {
    fn from(doc: Document) -> Self {
        let name = doc.get("name").unwrap().as_str().unwrap().to_string();
        let price = doc.get("price").unwrap().as_f64().unwrap();
        let username = doc.get("username").unwrap().as_str().unwrap().to_string();
        Self {
            name,
            price,
            username,
        }
    }
}
