use crate::io::{Query, Response};
use actix_web::web::Json;
use tract_onnx::prelude::*;

pub fn load_model(
    path: &str,
) -> SimplePlan<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>> {
    let model_loaded = tract_onnx::onnx().model_for_path(path);

    let model_optimized = match model_loaded {
        Ok(m) => m.into_optimized(),
        _ => panic!("model at {} could not be loaded", path),
    };

    let model_runnable = match model_optimized {
        Ok(m) => m.into_runnable(),
        _ => panic!("model at {} could not be turned optimized", path),
    };

    let model = match model_runnable {
        Ok(m) => m,
        _ => panic!("model at {} could not be turned runnable", path),
    };

    model
}

pub struct Model {
    model: SimplePlan<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>>,
}

impl Model {
    pub fn new(path: &str) -> Self {
        let model = load_model(&path);
        Model { model: model }
    }

    pub fn predict(&self, query: Json<Query>) -> Result<Response, Box<dyn std::error::Error>> {
        let result = Response {
            message: "2".to_string(),
        };
        return Ok(result);
    }
}
