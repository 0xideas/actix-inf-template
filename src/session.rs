use std::{env, f32::consts::E};

use crate::io::{Query, Response};
use actix_web::web::Json;
use ndarray::ArrayD;
use ndarray::IxDyn;

use ndarray::{array, concatenate, s, Array1, Axis, CowArray};
use ort::{
    tensor::OrtOwnedTensor, Environment, ExecutionProvider, GraphOptimizationLevel, OrtError,
    Session, SessionBuilder, Value,
};

pub struct AppSession {
    pub session: AISession,
}

fn load_ai_session(path: &str) -> Session {
    let environment_res = Environment::builder()
        .with_name("inf")
        .with_execution_providers([ExecutionProvider::CPU(Default::default())])
        .build();

    let environment = match environment_res {
        Ok(e) => e.into_arc(),
        e => panic!("{e:?}"),
    };

    let session1 = match SessionBuilder::new(&environment) {
        Ok(s) => s,
        e => panic!("{e:?}"),
    };
    let session2 = match session1.with_model_from_file(&path) {
        Ok(sess) => sess,
        e => panic!("{e:?}"),
    };
    return (session2);
}

pub struct AISession {
    session: Session,
}

impl AISession {
    pub fn new(path: &str) -> Self {
        let session = load_ai_session(&path);
        AISession { session: session }
    }
    pub fn predict(&self, query: Json<Query>) -> Result<Response, Box<dyn std::error::Error>> {
        let mut array1 = ArrayD::<f32>::zeros(IxDyn(&[3]));
        for (index, element) in query.data.iter().enumerate() {
            array1[index] = *element;
        }
        let mut array2 = CowArray::from(array1);

        let input_values = vec![Value::from_array(self.session.allocator(), &array2)?];
        let output_res: Result<Vec<Value>, OrtError> = self.session.run(input_values);
        let outputs = match output_res {
            Ok(o) => o,
            e => panic!("{e:?}"),
        };
        let output: Result<OrtOwnedTensor<'_, _, ndarray::Dim<ndarray::IxDynImpl>>, OrtError> =
            outputs[0].try_extract::<f32>();

        let best_option = match output {
            Ok(out) => out
                .view()
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
