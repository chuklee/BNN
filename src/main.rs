mod neurone;

fn main() {
    //Create a new Neurone
    let neurone = neurone::Neurone{
        age: 0.0,
        size: 0.0,
        signal: 0.0,
        synapses: Vec::new(),};
    //Print the Neurone
    println!("Neurone: {:?}", neurone.age);
    println!("Hello, world!");
}
// Add a new function to the Neurone struct

