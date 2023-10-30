use image;
use tract_onnx::prelude::*;

fn main() -> TractResult<()> {
    println!("hello");
    let model = tract_onnx::onnx()
        // load the model
        .model_for_path("/home/leon/Downloads/mobilenetv2-7.onnx")?
        // optimize the model
        .into_optimized()?
        // make the model runnable and fix its inputs and outputs
        .into_runnable()?;

    // let data = [1.0, 2.0, 3.0];
    // let tensor: Tensor = tract_ndarray::Array1::from_shape_fn(data.len(), |i| data[i]).into();
    // run the model on the input
    // let result = model.run(tvec!(tensor.into()))?;

    let image = image::open("/home/leon/Downloads/grace_hopper.jpg")
        .unwrap()
        .to_rgb8();
    let resized =
        image::imageops::resize(&image, 224, 224, ::image::imageops::FilterType::Triangle);
    let image: Tensor = tract_ndarray::Array4::from_shape_fn((1, 3, 224, 224), |(_, c, y, x)| {
        let mean = [0.485, 0.456, 0.406][c];
        let std = [0.229, 0.224, 0.225][c];
        (resized[(x as _, y as _)][c] as f32 / 255.0 - mean) / std
    })
    .into();

    let result = model.run(tvec!(image.into()))?;

    // find and display the max value with its index
    let best = result[0]
        .to_array_view::<f32>()?
        .iter()
        .cloned()
        .zip(2..)
        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    println!("result: {:?}", best);
    Ok(())
}
