use bson::{Document, oid::ObjectId};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateProduct {
    pub name: String,
    pub price: f64,
}

pub trait NewProduct<T> {
    fn convert(_: T, user_id: ObjectId) -> Self;
}

impl NewProduct<CreateProduct> for Document {
    fn convert(payload: CreateProduct, user_id: ObjectId) -> Self {
        let mut doc: Document = Document::new();
        doc.insert("name", payload.name);
        doc.insert("price", payload.price);
        doc.insert("user_id", user_id);
        doc
    }
}
