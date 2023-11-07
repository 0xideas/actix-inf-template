mod api;
mod io;
mod ort_model;
mod tract_model;

use actix_web::web::Data;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
//use tract_model::{AppData, Model};
use ort_model::{AppData, Model};
use std::sync::Mutex;

#[get("/health")]
async fn healthcheck() -> impl Responder {
    let response = io::Response {
        message: "actix-inf is up".to_string(),
    };
    HttpResponse::Ok().json(response)
}

async fn not_found() -> Result<HttpResponse> {
    let response = io::Response {
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let model = Model::new("./model/model.onnx");

    // let app_data = AppData { model: model };

    let app_session = AppData { model: model };
    let data = Data::new(Mutex::new(app_session));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&data))
            .configure(api::config)
            .service(healthcheck)
            .default_service(web::route().to(not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
