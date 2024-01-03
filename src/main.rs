use petgraph::dot::{Config, Dot};
use petgraph::graph::DiGraph;
use std::fmt;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Default)]
struct DiscreteRandomVariable {
    name: String,
    domain: Vec<f32>,
}

impl fmt::Display for DiscreteRandomVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl DiscreteRandomVariable {
    fn new(name: String, domain: Vec<f32>) -> Self {
        DiscreteRandomVariable { name, domain }
    }
}

fn main() {
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
    bg.add_edge(d, g, 1);
    bg.add_edge(i, g, 2);

    println!("{:?}", Dot::with_config(&bg, &[Config::EdgeNoLabel]));
    let mut f = File::create("img/bg.dot").unwrap();
    let output = format!("{}", Dot::with_config(&bg, &[Config::EdgeNoLabel]));
    f.write_all(&output.as_bytes())
        .expect("could not write file");
}
