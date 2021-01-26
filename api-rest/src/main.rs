/// Example of api-rest service on Actix Framework
/// 
/// Server App
///
/// By Victor M. - CIRCE - Based on the next page
///
/// https://codeburst.io/how-to-build-a-rest-api-to-execute-system-commands-using-actix-rust-a-step-by-step-guide-e257d5442b16



use actix_web::{post, web, HttpResponse, Responder, App, HttpServer}; /// Actix Framework Pkgs.
use pam::Authenticator; /// Auth Pkgs.
use log::info; /// Logger Pkgs.
use serde::{Serialize, Deserialize}; /// Serde Pkg.
use std::process::Command; /// Cmd Pkgs.



/// User Auth Request Struct
#[derive(Deserialize)]
pub struct RequestUser{
	username: String,
	password: String,
}

/// Command Request Struct
#[derive(Deserialize)]
pub struct RequestCMD {
    commands: String,
}

/// Request Struct
#[derive(Serialize)]
pub struct Response {
    result: bool,
}

/// Execute POST method
#[post("/execute")]
async fn execute_command(request: web::Json<RequestCMD>) -> impl Responder {
    info!("validating password for {}", request.commands);

    let process = Command::new("sh")
        .arg(&request.commands)
        .status()
        .expect("Failed to execute command");

    info!("status: {}", &process.to_string());

    if process.success() {
        HttpResponse::Ok().json(Response { result: true })
    } else {
        HttpResponse::Ok().json(Response { result: false })
    }
}

/// Auth. Function
fn authenticate(username: &str, password: &str) -> bool {
    let mut authenticator =
        Authenticator::with_password("VM4lumbr3s").expect("Fail to init with client");

    authenticator
        .get_handler()
        .set_credentials(username, password);

    authenticator.authenticate().is_ok()
}

/// Auth. POST method
#[post("/auth")]
async fn validate_password(request: web::Json<RequestUser>) -> impl Responder {

    info!("validating password for {}", request.username);

    if authenticate(&request.username, &request.password) {
        HttpResponse::Ok().json(Response { result: true })
    } else {
        HttpResponse::Ok().json(Response { result: false })
    }
}



#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/Hub/")
                    .service(validate_password)
                    .service(execute_command)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

