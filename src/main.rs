use actix_web::{App, HttpServer};

mod account;
mod bank;
mod document;
mod bank_service;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(bank_service::new_account)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

