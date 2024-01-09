mod lib;
use lib::DiscreteRandomVariable;
use petgraph::dot::{Config, Dot};
use petgraph::graph::DiGraph;
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let d = DiscreteRandomVariable::new(String::from("D"), vec![0.6, 0.4], 2);
    let i = DiscreteRandomVariable::new(String::from("I"), vec![0.7, 0.3], 2);
    let g = DiscreteRandomVariable::new(String::from("G"), vec![0.1, 0.2, 0.3], 2);
    let combinations_g = g.cpd(&vec![d, i]);
    return save_graph_f32(&combinations_g, "img/combinations.dot");
}

fn example() -> std::io::Result<()> {
    let mut bg = DiGraph::<DiscreteRandomVariable, &str>::new();

    let d = bg.add_node(DiscreteRandomVariable::new(
        String::from("D"),
        vec![0.6, 0.4],
        2,
    ));
    let i = bg.add_node(DiscreteRandomVariable::new(
        String::from("I"),
        vec![0.7, 0.3],
        2,
    ));
    let g = bg.add_node(DiscreteRandomVariable::new(
        String::from("G"),
        vec![0.0, 0.0, 0.0],
        2,
    ));
    bg.add_edge(d, g, "");
    bg.add_edge(i, g, "");
    return save_graph(&bg, "img/bg.dot");
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
