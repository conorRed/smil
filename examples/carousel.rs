use petgraph::dot::{Config, Dot};
use std::collections::HashMap;

use petgraph::Graph;
use smil::DiscreteRandomVariable;
use smil::Factor;

#[derive(Debug, Default)]
struct RandomVariable<'a> {
    name: &'a str,
    value: i32,
}

impl<'a> RandomVariable<'a> {
    fn new(name: &'a str) -> Self {
        return RandomVariable { name, value: 0 };
    }
}

fn main() {
    let mut bn = Graph::<Factor, &str>::new();
    let t_rv = DiscreteRandomVariable::new(String::from("time_elapsed"), vec![0, 1], 2);
    let time = Factor::new(vec![&t_rv], HashMap::new());
    let mut carousel_assignments = HashMap::new();
    carousel_assignments.insert(vec![1, 0], 0.5);
    carousel_assignments.insert(vec![0, 0], 0.5);
    carousel_assignments.insert(vec![1, 1], 0.5);
    carousel_assignments.insert(vec![0, 1], 0.5);
    let c_rv = DiscreteRandomVariable::new(String::from("carousel"), vec![0, 1], 2);
    let mut carousel = Factor::new(vec![&c_rv, &t_rv], carousel_assignments);
    carousel.observe(&vec![1, 1]);
    println!("{:?}", carousel);
    let f_c = bn.add_node(carousel);
    let f_t = bn.add_node(time);
    bn.extend_with_edges([(f_t, f_c)]);
    println!("{:?}", Dot::with_config(&bn, &[Config::EdgeNoLabel]));
}
