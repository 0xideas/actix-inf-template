use crate::io::{Query, Response};
use actix_web::web::Json;
use tract_onnx::prelude::*;

pub struct AppData {
    pub model: Model,
}

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
        // Create an array
        let mut array1: [f32; 3] = [0.0; 3];

        // Copy elements from the vector to the array
        for (index, element) in query.data.iter().enumerate() {
            array1[index] = *element;
        }
        let array2: [[f32; 3]; 1] = [array1];
        let tensor: Tensor = tensor2(&array2);
        let output = self.model.run(tvec!(tensor.into()));
        let output_unwrapped = match output {
            Ok(o) => o,
            e => panic!("{e:?}"),
        };
        let array_view_res: Result<
            tract_ndarray::ArrayBase<
                tract_ndarray::ViewRepr<&f32>,
                tract_ndarray::Dim<tract_ndarray::IxDynImpl>,
            >,
            tract_onnx::tract_core::anyhow::Error,
        > = output_unwrapped[0].to_array_view::<f32>();
        let best_option: Option<usize> = match array_view_res {
            Ok(av) => av
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.total_cmp(b))
                .map(|(index, _)| index),
            _ => panic!("could not compute result"),
        };

        let best = match best_option {
            Some(b) => b,
            _ => panic!("could not extract result"),
        };

        let response = Response {
            message: best.to_string(),
        };
        return Ok(response);
    }
}
