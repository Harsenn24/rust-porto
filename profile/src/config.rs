use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use smcore_derive::DeltaArc;
use smcore::delta::arc;

#[derive(Clone, Debug, DeltaArc, Serialize, Deserialize)]
pub enum Config {
    Mongouri,
    Salt,
    Database,
    UserEmail,
    PassEmail,
}

pub fn init() {
    // set initialize logger
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // initialize dot env
    dotenv().ok();
    let mongo_db = std::env::var("mongo_uri").expect("MONGO_URI must be set");
    let salt = std::env::var("SALT").expect("SALT must be set");
    let database = std::env::var("DATABASE").expect("DATABASE must be set");
    let user_email = std::env::var("USER_EMAIL").expect("User Email must be set");
    let pass_email = std::env::var("PASSWORD_EMAIL").expect("Password email must be set");

    arc::set::<Config>(Config::Mongouri, mongo_db);
    arc::set::<Config>(Config::Salt, salt);
    arc::set::<Config>(Config::Database, database);
    arc::set::<Config>(Config::UserEmail, user_email);
    arc::set::<Config>(Config::PassEmail, pass_email);

    // Config::Mongouri.set_str(mongo_db);
    // Config::Salt.set_str(salt);
    // Config::Database.set_str(database);
    // Config::UserEmail.set_str(user_email);
    // Config::PassEmail.set_str(pass_email);




}

// implement get
