/// Example of api-rest service on Actix Framework
/// 
/// User Auth Module
///
/// By Victor M. - CIRCE - Based on the next page
///
/// https://codeburst.io/how-to-build-a-rest-api-to-execute-system-commands-using-actix-rust-a-step-by-step-guide-e257d5442b16


use actix_web::{post, web, HttpResponse, Responder};
use log::{info, trace, warn};
use pam::Authenticator;
use serde::{Serialize, Deserialize};

/// User Data Request Struct
#[derive(Deserialize)]
pub struct Request {
    username: String,
    password: String,
}

/// Response Struct
#[derive(Serialize)]
pub struct Response {
    result: bool,
}

/// Auth. POST method
#[post("/auth")]
pub async fn validate_password(request: web::Json<Request>) -> impl Responder {

    info!("validating password for {}", request.username);

    if authenticate(&request.username, &request.password) {
        HttpResponse::Ok().json(Response { result: true })
    } else {
        HttpResponse::Ok().json(Response { result: false })
    }
}

/// Auth. Function
pub fn authenticate(username: &str, password: &str) -> bool {
    let mut authenticator =
        Authenticator::with_password("VM4lumbr3s").expect("Fail to init with client");

    authenticator
        .get_handler()
        .set_credentials(username, password);

    authenticator.authenticate().is_ok()
}
