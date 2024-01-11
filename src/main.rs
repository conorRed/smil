mod lib;
use lib::assignments;
use lib::DiscreteRandomVariable;
use petgraph::dot::{Config, Dot};
use petgraph::graph::DiGraph;
use std::fs::File;
use std::io::Write;

fn main() -> () {
    let d = DiscreteRandomVariable::new(String::from("D"), vec![6, 4], 2);
    let i = DiscreteRandomVariable::new(String::from("I"), vec![7, 3], 2);
    let g = DiscreteRandomVariable::new(String::from("G"), vec![1, 2, 3], 2);
    let _combinations_g = assignments(&vec![d, i, g]);
}

fn save_graph_f32(bg: &DiGraph<f32, &str>, filename: &str) -> std::io::Result<()> {
    println!("{:?}", Dot::with_config(&bg, &[Config::EdgeNoLabel]));
    let mut f = File::create(filename).unwrap();
    let output = format!("{}", Dot::with_config(&bg, &[Config::EdgeNoLabel]));
    return f.write_all(&output.as_bytes());
}
fn save_graph(bg: &DiGraph<DiscreteRandomVariable, &str>, filename: &str) -> std::io::Result<()> {
    println!("{:?}", Dot::with_config(&bg, &[Config::EdgeNoLabel]));
    let mut f = File::create(filename).unwrap();
    let output = format!("{}", Dot::with_config(&bg, &[Config::EdgeNoLabel]));
    return f.write_all(&output.as_bytes());
}
