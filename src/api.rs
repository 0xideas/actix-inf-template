use crate::{
    io::Query,
    //tract_model::{AppData, Model},
    ort_model::{AppData, Model},
};
use actix_web::web;
use actix_web::web::Data;
use actix_web::{post, web::Json, HttpResponse};
use std::sync::Mutex;

#[post("/infer")]
pub async fn infer(query: Json<Query>, data: Data<Mutex<AppData>>) -> HttpResponse {
    let app_data: std::sync::MutexGuard<'_, AppData> = data.lock().unwrap();
    let model: &Model = &app_data.model;

    let prediction = model.predict(query);
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
