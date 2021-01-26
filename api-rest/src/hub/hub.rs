/// Example of api-rest service on Actix Framework
/// 
/// Hub Module 
///
/// By Victor M. - CIRCE 

use actix_web::{get, post, web, HttpResponse, Responder}; /// Actix Framework Pkgs.
use serde::{Serialize, Deserialize}; /// Serde Pkg.
use std::process::Command; /// Cmd Pkgs.
use std::println; /// Basic print with \n


///********************************** DATA STRUCTS ***************************************

/// Server Status
#[derive(Serialize)]
pub struct Status {
    running: bool,
    timestamp: String,
}

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

///********************************** FUNTIONS *****************************************

/// Check user
fn authenticate(user:&str, pwd:&str) -> bool{

    let result:bool;

    // Good user settings
    let gguser: &str = "vmalumbres";
    let ggpwd: &str = "VM4lumbr3s";

    // Check
    if gguser.eq(user) && ggpwd.eq(pwd) {
        result = true;
    }else{
        result = false;
    }

    // Return result
    result
}

///********************************** HTTP METHODS *************************************

/// Execute POST method
#[post("/Execute")]
pub async fn execute_command(request: web::Json<RequestCMD>) -> impl Responder {

    println!("[LOG] Hub module: Command Line Executer");


    let process = Command::new(&request.commands)
        .status()
        .expect("Failed to execute command");


    if process.success() {
        HttpResponse::Ok().json(Response { result: true })
    } else {
        HttpResponse::Ok().json(Response { result: false })
    }

}

/// Auth. POST method
#[post("/Auth")]
pub async fn check_user(request: web::Json<RequestUser>) -> impl Responder {

    println!("[LOG] Hub module: User Auth.");

    if authenticate(&request.username, &request.password) {
        println!("[LOG] User's Access Allowed!");
        HttpResponse::Ok().json(Response { result: true })
    } else {
        println!("[LOG] User's Access Denied!");
        HttpResponse::Ok().json(Response { result: false })
    }
}

/// Status of hub GET method
#[get("/Status")]
pub async fn hub_status() -> impl Responder {

	// Status
	let hub_status = Status{
		running: true,
		timestamp:String::from("26/01/2021"),
	};

	// Logs
    println!("[LOG] Hub module: Hub Status Shadow ");

    // Return as http-response with a json
    HttpResponse::Ok().json(hub_status)

}