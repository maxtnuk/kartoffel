#![no_std]

use burn::{
    module::Module,
    nn,
    tensor::{activation::log_softmax, backend::Backend, Tensor},
};
// possible acts
// move, turn, attack
// scan area 4 by 4
// hidden size 64
const INPUT_SIZE: usize = 4;
const OUTPUT_SIZE: usize = 4;
const HIDDEN_SIZE: usize = 32;
const DEPTH: usize = 3;

pub struct ModelConfig {
    pub input_size: usize,
    pub output_size: usize,
    pub hidden_size: usize,
    pub depth: usize,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            input_size: INPUT_SIZE,
            output_size: OUTPUT_SIZE,
            hidden_size: HIDDEN_SIZE,
            depth: DEPTH,
        }
    }
}

#[derive(Module, Debug)]
pub struct Model<B: Backend> {
    input: nn::Linear<B>,
    output: nn::Linear<B>,
    hidden_layer: nn::Linear<B>,
    activation: nn::Gelu,
    depth: usize,
}

impl<B: Backend> Model<B> {
    pub fn new(config: &ModelConfig, device: &B::Device) -> Self {
        let input = nn::LinearConfig::new(config.input_size, config.hidden_size).init(device);
        let hidden_layer =
            nn::LinearConfig::new(config.hidden_size, config.hidden_size).init(device);
        let output = nn::LinearConfig::new(config.hidden_size, config.output_size).init(device);
        let activation = nn::Gelu {};
        Self {
            output,
            input,
            hidden_layer,
            activation,
            depth: config.depth,
        }
    }

    pub fn forward(&self, input: Tensor<B, 2>) -> Tensor<B, 1> {
        let mut hidden_out = self.input.forward(input.reshape([-1]));
        for _ in 0..self.depth {
            hidden_out = self.hidden_layer.forward(hidden_out);
            hidden_out = self.activation.forward(hidden_out);
        }
        let output = self.output.forward(hidden_out);
        let logit = log_softmax(output, 0);
        logit
    }
}
