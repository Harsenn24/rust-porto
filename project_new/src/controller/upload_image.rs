use actix_web::{ Result, web, HttpResponse, Error };
use actix_multipart::Multipart;
use smcore::actix::multipart::{ KeyFields, FileFields, Catch };

use crate::interface::upload_image;

pub async fn upload(payload: Multipart) -> Result<HttpResponse, Error> {
    let fields = vec![
        KeyFields {
            key: "json_payload",
            file: None,
        },
        KeyFields {
            key: "image",
            file: Some(FileFields {
                rename: "produk trial",
                target_dir: "tmp",
                content_type: vec!["image/png", "image/jpeg"],
            }),
        }
    ];

    let result = Catch::<upload_image::ResultMultipart>::new(fields).save(payload).await?;

    Ok(HttpResponse::Ok().json(result))
}