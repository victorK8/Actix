/// Api-Rest example 
/// by Víctor Malumbres Talles
/// Ubuntu 20.04 LTS Machine

use actix_web::{get,post, web, App, HttpResponse, HttpServer, Responder};

#[derive(Serialize,Deserialize)]
struct Data {
    brightness: u8,
    time: u64,
}


/// Request Handlers

#[derive(Serialize, Deserialize)]
struct File {
    name: String,
    time: u64,
    err: String
}

#[derive(Deserialize)]
struct Download {
    name: String,
}

#[post("/Upload/{name}")]
async fn upload() -> impl Responder {
    let u = &File {
        name: "dummy data".to_string(),
        time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),err: "".to_string()
    };
    HttpResponse::Ok().json(u)
}

#[get("/Download")]
async fn download(info: web::Path<Download>) -> HttpResponse {
    let name = String::from(info.name.as_str());
    let body = once(ok::<_, Error>(Bytes::from(name)));

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}

/// Server Main
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
	   App::new()
	        .service(upload)
	        .service(download)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}