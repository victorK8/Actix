/// Example of api-rest service on Actix Framework
/// 
/// Execute Module
///
/// By Victor M. - CIRCE - Based on the next page
///
/// https://codeburst.io/how-to-build-a-rest-api-to-execute-system-commands-using-actix-rust-a-step-by-step-guide-e257d5442b16

use actix_web::{post, web, HttpResponse, Responder};
use log::info;
use serde::{Serialize, Deserialize};
use std::process::Command;
use std::io::{self, Write};

/// Command Request Struct
#[derive(Deserialize)]
pub struct Request {
    commands: String,
}

/// Request Struct
#[derive(Serialize)]
pub struct Response {
    result: bool,
}

/// Execute POST method
#[post("/execute")]
pub async fn execute_command(request: web::Json<Request>) -> impl Responder {
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