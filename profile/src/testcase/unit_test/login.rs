use crate::interface::auth::login::LoginReqBody;

#[actix_web::test]
async fn register_username_greater_than_6(){
    let payload = LoginReqBody {
        username: "test".to_string(),
        password: "test".to_string()
    };

    let result = Result::from(payload);

    let error = result.as_ref().err().unwrap().to_owned();

    assert_eq!(error, "username must be greater than 6 characters".to_string());
}


#[actix_web::test]
async fn register_username_only_alphanumeric(){
    let payload = LoginReqBody {
        username: "test 1234".to_string(),
        password: "test".to_string()
    };

    let result = Result::from(payload);

    let error = result.as_ref().err().unwrap().to_owned();

    assert_eq!(error, "username must be alphanumeric".to_string());
}

#[actix_web::test]
async fn register_success(){
    let payload = LoginReqBody {
        username: "testtest".to_string(),
        password: "testtest".to_string()
    };

    let result = Result::from(payload);

    assert_eq!(result.is_ok(), true);

}

