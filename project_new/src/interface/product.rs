use serde::{ Deserialize, Serialize };
use bson::{ Document, oid::ObjectId };
use smcore::hashlib::latte;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductNew {
    pub name: String,
    pub price: f64,
    pub description: String,
}

pub trait ProductFrom<T> {
    fn convert(_: T, store_id: ObjectId) -> Self;
}

impl ProductFrom<ProductNew> for Document {
    fn convert(payload: ProductNew, store_id: ObjectId) -> Self {
        let mut doc: Document = Document::new();
        doc.insert("name", payload.name);
        doc.insert("price", payload.price);
        doc.insert("description", payload.description);
        doc.insert("store_id", store_id);
        doc
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductRead {
    pub id: String,
    pub name: String,
    pub price: f64,
    pub description: String,
    pub store_id: String,
}

impl From<Document> for ProductRead {
    fn from(doc: Document) -> Self {
        let id: String = doc.get_object_id("_id").expect("error id").to_hex();
        let id: String = latte::Hash::new(id, 12).encrypt();

        let name: String = doc.get_str("name").expect("error name").to_string();
        let price: f64 = doc.get_f64("price").expect("error price") as f64;
        let description: String = doc.get_str("description").expect("error des").to_string();
        let store_id: String = doc.get_object_id("store_id").expect("error store id").to_hex();

        ProductRead {
            id,
            name,
            price,
            description,
            store_id,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IdParams {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductUpdateRequest {
    pub name: String,
    pub price: f64,
    pub description: String,
    pub store_id: String,
}

impl From<ProductUpdateRequest> for Result<Document, String> {
    fn from(payload: ProductUpdateRequest) -> Self {
        let store_id: String = latte::Hash::new(payload.store_id, 12).decrypt();

        //convert store_id to ObjectId
        let store_id = match ObjectId::parse_str(&store_id) {
            Ok(id) => id,
            Err(err) => {
                return Err(err.to_string());
            }
        };

        Ok(
            bson::doc! {
            "name" : payload.name,
            "price" : payload.price,
            "description" : payload.description,
            "store_id" : store_id
            }
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductPatchRequest {
    pub name: Option<String>,
    pub price: Option<f64>,
    pub description: Option<String>,
    pub store_id: Option<String>,
}

impl From<ProductPatchRequest> for Result<Document, String> {
    fn from(payload: ProductPatchRequest) -> Self {
        let mut doc: Document = Document::new();

        match payload.name {
            Some(data) => {
                doc.insert("name", data);
            }
            None => {}
        }

        match payload.price {
            Some(data) => {
                doc.insert("price", data);
            }
            None => {}
        }

        match payload.description {
            Some(data) => {
                doc.insert("description", data);
            }
            None => {}
        }

        match payload.store_id {
            Some(data) => {
                match ObjectId::parse_str(data) {
                    Ok(id) => id,
                    Err(err) => return Err(err.to_string())
                };
            }
            None => {}
        }

        Ok(doc)
    }
}




#[derive(Serialize, Deserialize, Debug)]
pub struct ProductRequest {
    pub name: String,
    pub price: f64,
    pub description: String,
    pub store_id: String,
}

impl From<ProductRequest> for Result<Document, String> {
    fn from(payload: ProductRequest) -> Self {
        let mut doc: Document = Document::new();

        let store_id: String = latte::Hash::new(payload.store_id, 12).decrypt();


        let store_id = match ObjectId::parse_str(&store_id) {
            Ok(id) => id,
            Err(err) => {
                return Err(err.to_string());
            }
        };
        doc.insert("name", payload.name);
        doc.insert("price", payload.price);
        doc.insert("description", payload.description);
        doc.insert("store_id", store_id);


        Ok(doc)
    }
}
