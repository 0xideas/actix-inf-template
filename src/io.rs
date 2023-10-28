use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[derive(Deserialize)]
pub struct Query {
    pub data: Vec<f32>,
}
