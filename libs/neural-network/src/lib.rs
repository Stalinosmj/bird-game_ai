#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>, //Network is build upon many layers
}

impl Network {
    pub fn propagate(&self, mut inputs: Vec<f32>) -> Vec<f32> {  //Numbers have to be inserted into each layer
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }
}

#[derive(Debug)]
struct Layer {
    neurons: Vec<Neuron>, //Layers are build from neurons
}

impl Layer {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> { //A single neuron accepts many inputs and returns a single output
        self.neurons
        .iter()
        .map(|neuron| neuron.propagate(&inputs))
        .collect()
    }
}

#[derive(Debug)]
struct Neuron {
    bias: f32,
    weights: Vec<f32>, //Neurons conatins bias and weights
}

impl Neuron {
    fn propagate(&self , inputs: &[f32]) -> f32 {
        todo!()
    }
}