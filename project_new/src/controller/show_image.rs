use actix_web::{ web, HttpResponse, Responder };
use smcore::http::response::image::{ self, Param, Source, Format };

pub async fn show() -> impl Responder {
    let image = image::serve(
        Source {
            path: "tmp/produk trial-1.png".to_string(),
            convert: Format::Jpeg,
        },
        Param {
            size: Some(720),
            quality: Some(5),
        },
        None
    );
    let (format, image) = match image {
        Ok(image) => image,
        Err(_) => {
            return HttpResponse::InternalServerError().finish();
        }
    };
    HttpResponse::Ok().insert_header(("Content-Type", format)).body(image)
}