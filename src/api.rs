use crate::{
    io::{Query, Response},
    model::Model,
};

use actix_web::web;
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};

#[post("/infer")]
pub async fn infer(query: Json<Query>) -> HttpResponse {
    let model = Model::new("./model/model.onnx");
    let prediction = model.predict(query);
    match prediction {
        Ok(prediction) => HttpResponse::Ok().json(prediction),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").service(infer));
}
