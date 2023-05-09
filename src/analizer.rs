
use bevy::prelude::*;
use rand::{thread_rng, Rng};

#[derive(Clone, Copy)]
pub struct Neuron{
    min: f32,
    max: f32,
    pub value: f32,
    pub sign: [char; 8]
}

impl Neuron {
    pub fn new(min: f32, max: f32) -> Self {
        Self{min: min, max: max, value: 0.0, sign: thread_rng().gen()}
    }
}

#[derive(Component)]
pub struct Analizer{
    pub nodes: Vec<Neuron>
}

impl Analizer {
    pub fn new() -> Self {
        Self {
            nodes: vec![]
        }
    }
    pub fn add_neuron(&mut self, n: Neuron) {
        self.nodes.push(n);
    }
    pub fn analize(&self) -> Vec<f32> {
        let mut output = Vec::<f32>::new();
        for n in self.nodes.iter() {
            let v = thread_rng().gen_range(n.min..n.max);
            output.push(v);
        }
        return output;
    }
}