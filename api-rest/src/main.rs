/// Example of api-rest service on Actix Framework
/// 
/// Server App
///
/// By Victor M. - CIRCE - Based on the next page
///
/// https://codeburst.io/how-to-build-a-rest-api-to-execute-system-commands-using-actix-rust-a-step-by-step-guide-e257d5442b16

use actix_web::{web, App, HttpServer}; /// Actix Framework Pkgs.


mod hub {
    pub mod hub;
}

/// Server Main
#[actix_rt::main]
async fn main() -> std::io::Result<()> {

	println!("[LOG] Main: Running Server");

    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/Hub/")
                    .service(hub::hub::execute_command)
                    .service(hub::hub::check_user)
                    .service(hub::hub::hub_status)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

