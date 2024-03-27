pub struct Synapse {
    pub size: f32,
    pub weight: f32,
}

pub struct Neurone {
    pub age: f32,
    pub size: f32,
    pub signal: f32,
    pub synapses: Vec<Synapse>,
}