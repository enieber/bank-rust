use actix_web::{web, get, post, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use crate::bank::open_account;

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
pub async fn new_account(account_req: web::Json<NewAccount>) -> impl Responder {
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

