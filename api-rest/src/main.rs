/// Example of api-rest service on Actix Framework
/// 
/// Server App
///
/// By Victor M. - CIRCE - Based on the next page
///
/// https://codeburst.io/how-to-build-a-rest-api-to-execute-system-commands-using-actix-rust-a-step-by-step-guide-e257d5442b16


use actix_web::{App, HttpServer, web};

extern crate simple_logger;

/// How to add your own modules
mod Executor {
    pub mod auth;
    pub mod execute;
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    simple_logger:: init_with_level(log::Level::Info).unwrap();

    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/serv/")
                    .service(executor::validate_password::validate_password)
                    .service(executor::execute::execute_command)
            )
    })
        .workers(10)
        .keep_alive(15)
        .bind("127.0.0.1:8088")?
        .run()
        .await
}