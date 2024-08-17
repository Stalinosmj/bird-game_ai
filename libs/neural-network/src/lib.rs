#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>, //Network is build upon many layers
}

impl Network {
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        todo!()
    }
}

#[derive(Debug)]
struct Layer {
    neurons: Vec<Neuron>, //Layers are build from neurons
}

impl Layer {
    fn propagate(&self , inputs: Vec<f32>) -> Vec<f32> {
        todo!()
    }
}

#[derive(Debug)]
struct Neuron {
    bias: f32,
    weights: Vec<f32>, //Neurons conatins bias and weights
}

