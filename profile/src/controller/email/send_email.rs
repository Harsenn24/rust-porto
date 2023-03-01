use actix_web::{ HttpResponse, Responder};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use crate::config::Config;

pub async fn send_email() -> impl Responder {
    let user_email = "caesarharsenn@gmail.com".to_string();
    let pass_email = "pgteqwtolfqmvvap".to_string();


    let email = Message::builder()
        .from("harsenn <caesarharsenn@gmail.com>".parse().unwrap())
        .to("sangkakala <harsenn24@gmail.com>".parse().unwrap())
        .subject("Test Email")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from("INI EMAIL DARI RUST HEHEHEHE"))
        .unwrap();


    let creds = Credentials::new(user_email.to_owned(), pass_email.to_owned());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(&user_email)
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => HttpResponse::Ok().json("success".to_string()),
        Err(e) => HttpResponse::BadRequest().json(e.to_string()),
    }
}
