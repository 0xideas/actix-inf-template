use crate::{
    io::Query,
    model::{AppData, Model},
    session::{AISession, AppSession},
};
use actix_web::web;
use actix_web::web::Data;
use actix_web::{post, web::Json, HttpResponse};
use std::sync::Mutex;

#[post("/infer")]
pub async fn infer(query: Json<Query>, data: Data<Mutex<AppSession>>) -> HttpResponse {
    let app_data = data.lock().unwrap();
    //let model: &Model = &app_data.model;
    let session: &AISession = &app_data.session;

    let prediction = session.predict(query);
    match prediction {
        Ok(prediction) => HttpResponse::Ok().json(prediction),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/mirror")]
pub async fn mirror(query: Json<Query>) -> HttpResponse {
    HttpResponse::Ok().json(query.data.clone())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").service(infer).service(mirror));
}
