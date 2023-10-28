use crate::io::{Query, Response};
use actix_web::web::Json;
use tract_onnx::prelude::*;

pub fn load_model(
    path: &str,
) -> SimplePlan<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>> {
    let model_loaded = tract_onnx::onnx().model_for_path(path);

    let model_optimized = match model_loaded {
        Ok(m) => m.into_optimized(),
        e => panic!("{e:?}"),
    };

    let model_runnable = match model_optimized {
        Ok(m) => m.into_runnable(),
        e => panic!("{e:?}"),
    };

    let model = match model_runnable {
        Ok(m) => m,
        e => panic!("{e:?}"),
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
        let tensor: Tensor =
            tract_ndarray::Array1::from_shape_fn(query.data.len(), |i| query.data[i]).into();
        let output = self.model.run(tvec!(tensor.into()));
        let output_unwrapped = match output {
            Ok(o) => o,
            _ => panic!("error in running model"),
        };
        let array_view_res = output_unwrapped[0].to_array_view::<f32>();
        let best = match array_view_res {
            Ok(av) => {
                av.iter()
                    .cloned()
                    .zip(2..)
                    .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            }
            _ => panic!("could not compute result"),
        };

        let response = Response {
            message: "{best:?}".to_string(),
        };
        return Ok(response);
    }
}
