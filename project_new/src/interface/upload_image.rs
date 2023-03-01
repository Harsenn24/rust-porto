use std::collections::HashMap;

use smcore::actix::multipart::{ File, file_vector, file_single };
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonMultipart {
    pub name: String,
    pub id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResultMultipart {
    pub json_payload: JsonMultipart,
    // pub image: Vec<File>, --> Multi Upload
    pub image: File, // --> Single Upload
}

impl From<HashMap<String, String>> for ResultMultipart {
    // Multi Upload
    // fn from (map: HashMap<String, String>) -> Self {
    //     let json_payload = map.get("json_payload").unwrap().to_owned();
    //     let json_payload: JsonMultipart = serde_json::from_str(&json_payload).unwrap();

    //     let mut image = Vec::new();

    //     if map.contains_key("image") {
    //         let image_buffer = map.get("image").expect("error image").to_string();
    //         image = file_vector(image_buffer);

    //     }

    //     Self {
    //         json_payload,
    //         image
    //     }
    // }

    //Single Upload
    fn from(map: HashMap<String, String>) -> Self {

        //match json_payload

        
        let json_payload = map.get("json_payload").unwrap().to_owned();
        let json_payload: JsonMultipart = serde_json::from_str(&json_payload).unwrap();

        let image_buffer = map.get("image").expect("error image").to_owned();

        let image = file_single(image_buffer);

        Self {
            json_payload,
            image,
        }
    }
}