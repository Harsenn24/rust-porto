use actix_web::{ test, web, App, HttpResponse, Responder, http::header::ContentType };
use serde_json::json;
use crate::interface::auth::login::LoginReqBody;

pub async fn index(
    payload:web::Json<LoginReqBody>
) -> impl Responder {
    let payload = Result::from(payload.into_inner());
    // match payload 
    match payload {
        Ok(_) => {
            HttpResponse::Ok().body("all is good")
        },
        Err(_) => {
            HttpResponse::BadRequest().json("ahh error")
        }
    }
}

#[actix_web::test]
async fn index_has_no_content() {
    let app = test::init_service(App::new().route("/", web::get().to(index))).await;
    
    let json_payload = json!({
        "username": "testtest",
        "password": "testtest"
    });


    let payload  = serde_json::to_vec(&json_payload).unwrap(); //change payload json_payload to vec v8

    //change payload json_payload to vec v8

    
    let req = test::TestRequest::default()
        .insert_header(ContentType::json())
        .set_payload(payload)
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);


}   
