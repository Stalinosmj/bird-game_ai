use rand::{ Rng, RngCore };
use approx::assert_relative_eq;

#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>, //Network is build upon many layers
}
pub struct LayerTopology {
    pub neurons: usize,
}

impl Network {
    pub fn propagate(&self, mut inputs: Vec<f32>) -> Vec<f32> {  //Numbers have to be inserted into each layer
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }
    pub fn random(rng: &mut dyn RngCore, layers: &[LayerTopology]) -> Self {
        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(rng, layers[0].neurons, layers[1].neurons))
            .collect();
        Self { layers }
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
    fn random(rng: &mut dyn RngCore, input_size: usize, output_size: usize) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::random(rng, input_size))
            .collect();
        Self { neurons }
    }
}

#[derive(Debug)]
struct Neuron {
    bias: f32,
    weights: Vec<f32>, //Neurons contains bias and weights
}

impl Neuron {
    fn propagate(&self , inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(),self.weights.len());

        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();

        (self.bias + output).max(0.0) //ReLU activation function
    }
    fn random(rng: &mut dyn RngCore, input_size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let bias = rng.gen_range(-1.0..1.0);
        
        let weights = (0..input_size)
            .map(|_| rng.gen_range(-1.0..1.0))
            .collect();
        
        Self { bias, weights }
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    
    #[test]
    fn random() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let neuron = Neuron::random(&mut rng, 4);

        assert_relative_eq!(neuron.bias, -0.6781485);

        assert_relative_eq!(
        neuron.weights.as_slice(),
        [-0.6255188, 0.8181262, 0.26284897, 0.5238807].as_ref()
    );
    }
    #[test]
    fn propagate() {
        let neuron = Neuron {
            bias: 0.5,
            weights: vec![-0.3, 0.8],
        };

        // Ensures `.max()` (our ReLU) works:
        assert_relative_eq!(
        neuron.propagate(&[-10.0, -10.0]),
        0.0,
    );

        // `0.5` and `1.0` chosen by a fair dice roll:
        assert_relative_eq!(
        neuron.propagate(&[0.5, 1.0]),
        (-0.3 * 0.5) + (0.8 * 1.0) + 0.5,
    );


    }
}

