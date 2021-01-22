/// Api-Rest example 
/// by Víctor Malumbres Talles
/// Ubuntu 20.04 LTS Machine

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hey sunshine!")
}#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hi", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}