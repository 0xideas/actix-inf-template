use crate::io::{Query, Response};
use actix_web::web::Json;
pub struct Model {}

impl Model {
    pub fn new() -> Self {
        Model {}
    }

    pub fn predict(&self, query: Json<Query>) -> Result<Response, Box<dyn std::error::Error>> {
        let result = Response {
            message: "2".to_string(),
        };
        return Ok(result);
    }
}
