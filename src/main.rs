use actix_web::{web, get, post, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

mod account;
mod bank;
mod document;
use crate::bank::open_account;



#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Deserialize)]
struct NewAccount {
    name: String,
    document: String,
}

#[derive(Serialize)]
struct Response {
    status: bool,
    message: String,
}

#[post("/open_account")]
async fn new_account(account_req: web::Json<NewAccount>) -> impl Responder {
    let name = account_req.name.to_string();
    let document = account_req.document.to_string();
    let my_account = open_account(name, document, 0.0);
    match my_account {
        Ok(account) => {
            let response =  Response {
                status: true,
                message: account.to_string(),
            };
            return web::Json(response);
        },
        Err(err) => {
            let response = Response {
                status: false,
                message: err,
            };
            return web::Json(response);
        }
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(new_account)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

