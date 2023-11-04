use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};

mod api;
mod io;
mod model;

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
    HttpServer::new(|| {
        App::new()
            .configure(api::config)
            .service(healthcheck)
            .default_service(web::route().to(not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
