mod lib;
use lib::DiscreteRandomVariable;
use petgraph::dot::{Config, Dot};
use petgraph::graph::DiGraph;
use std::fs::File;
use std::io::Write;
fn main() -> std::io::Result<()> {
    let mut bg = DiGraph::<DiscreteRandomVariable, i32>::new();

    let d = bg.add_node(DiscreteRandomVariable::new(
        String::from("D"),
        vec![0.6, 0.4],
    ));
    let i = bg.add_node(DiscreteRandomVariable::new(
        String::from("I"),
        vec![0.7, 0.3],
    ));
    let g = bg.add_node(DiscreteRandomVariable::new(
        String::from("G"),
        vec![0.0, 0.0, 0.0],
    ));
    bg.add_edge(d, g, 0);
    bg.add_edge(i, g, 1);
    return save_graph(&bg, "img/bg.dot");
}

fn save_graph(bg: &DiGraph<DiscreteRandomVariable, i32>, filename: &str) -> std::io::Result<()> {
    println!("{:?}", Dot::with_config(&bg, &[Config::EdgeNoLabel]));
    let mut f = File::create(filename).unwrap();
    let output = format!("{}", Dot::with_config(&bg, &[Config::EdgeNoLabel]));
    return f.write_all(&output.as_bytes());
}
