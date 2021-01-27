/// Example of api-rest service on Actix Framework
/// 
/// Lights Module 
///
/// By Victor M. - CIRCE 

use actix_web::{get, post, web, HttpResponse, Responder}; /// Actix Framework Pkgs.
use serde::{Serialize, Deserialize}; /// Serde Pkg.

///********************************** DATA STRUCTS ************************************

/// Lights Struct
#[derive(Serialize)]
pub struct Light {
	id: u8,
    r: u8, 
    g: u8,
    b: u8,
    brightness: u8,
    intensity: u8,
}

/// Result Struct
#[derive(Serialize)]
pub struct Response {
    result: bool,
}

///********************************** FUNTIONS *****************************************


///********************************** HTTP METHODS *************************************

/// Status of a light by id
#[get("/Status")]
pub async fn all_lights() -> impl Responder {

	// Status. Some dummy example
	let status = Light{
		id: 254,
	    r: 100, 
	    g: 100,
	    b: 100,
	    brightness: 100,
	    intensity: 100,
	};

	// Logs
    println!("[LOG] Lights module: All Lights Status Shadow ");

    // Return as http-response with a json
    HttpResponse::Ok().json(status)

}

/// Status of a light by id
#[get("/Status/{id}")]
pub async fn lights_by_id(path: web::Path<(u8,)>) -> impl Responder {

	// id
	let id_of_light:u8 = path.0;

	// Status. Some dummy example
	let status = Light{
		id: id_of_light,
	    r: 100, 
	    g: 100,
	    b: 100,
	    brightness: 100,
	    intensity: 100,
	};

	// Logs
    println!("[LOG] Lights module: {} Light Status Shadow ", id_of_light);

    // Return as http-response with a json
    HttpResponse::Ok().json(status)

}