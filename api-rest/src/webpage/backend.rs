/// Example of api-rest service on Actix Framework
/// 
/// Web Module 
///
/// By Victor M. - CIRCE 

use actix_web::{get, HttpResponse, Responder}; /// Actix Framework Pkgs.
use std::fs; // Pkg for handling files
use std::string::String; // String pkg
use std::path::Path; // path pkg


///********************************** PATHS *************************************
const WD: &str = "/home/victor/Actix/api-rest/src";
const TEMPLATES: &str = "/home/victor/Actix/api-rest/src/templates";

///********************************** HTTP METHODS *************************************

// Index Path
#[get("/")]
pub async fn index() -> impl Responder {

	// Index path
	let path = Path::new(&TEMPLATES).join("index.html");

	// Get file as string
	let fp:String = fs::read_to_string(path)
        .expect("[ERROR] Web module: Index");

	// Logs
    println!("[LOG] Web module: Index ");

    // Return as http-response with a json
    HttpResponse::Ok().body(fp)

}