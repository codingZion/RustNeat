use rust_neat::nim::dp::*;
use rust_neat::fnn::model::ModelConfig;
use burn::backend::Wgpu;

fn main() {
    type MyBackend = Wgpu<f32, i32>;

    let device = Default::default();
    let model = ModelConfig::new(10, 512).init::<MyBackend>(&device);

    println!("{}", model);
}