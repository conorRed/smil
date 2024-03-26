// A *Factor* is a function $\phi$ that returns a value for each variable. For a set of variables X1, ...,
// $X_k$ is returns a real valued number for each combination of values. $\phi : V al(X_1, ..., X_k) → R$.
// It’s scope is the set of values which it is defined over (its arguments).

// how do we limit the arguments to factor? Is it a simple function?

use itertools::Itertools;
use petgraph::dot::{Config, Dot};
use petgraph::graph::DiGraph;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::Write;

// Random Variables can be thought of in set notation, having a cardinality
#[derive(Debug, Clone, Default)]
pub struct DiscreteRandomVariable {
    name: String,
    domain: Vec<i32>,
    cardinality: i32,
}

#[derive(Debug, Clone, Default)]
pub struct Factor<'a> {
    variables: Vec<&'a DiscreteRandomVariable>,
    assignment_to_value: HashMap<Vec<i32>, f32>,
    observed: Option<f32>,
}

trait PMF {
    fn px(x: i32) -> f32;
}

impl fmt::Display for DiscreteRandomVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Factor<'_> {
    pub fn new(variables: Vec<&DiscreteRandomVariable>, assign: HashMap<Vec<i32>, f32>) -> Factor {
        return Factor {
            variables: variables.to_vec(),
            assignment_to_value: assign,
            observed: None,
        };
    }
    pub fn observe(&mut self, row_in_assignments: &Vec<i32>) {
        if let Some(value) = self.assignment_to_value.get(row_in_assignments) {
            self.observed = Some(*value);
        }
    }
}

impl DiscreteRandomVariable {
    pub fn new(name: String, domain: Vec<i32>, cardinality: i32) -> Self {
        DiscreteRandomVariable {
            name,
            domain,
            cardinality,
        }
    }
}

pub fn assignments(conditionals: &Vec<DiscreteRandomVariable>) -> Vec<Vec<i32>> {
    let vec_of_domains: Vec<Vec<i32>> = conditionals.iter().map(|rv| rv.domain.clone()).collect();
    let _dimensions: i32 = conditionals.iter().fold(1, |acc, rv| rv.cardinality * acc);

    let cartesian_product: Vec<Vec<i32>> = vec_of_domains
        .into_iter()
        .multi_cartesian_product()
        .take(_dimensions as usize)
        .collect();
    return cartesian_product;
}
// Need to think about a state space. What is the 'combinatorial expolsian' of the random variables
// and do we have a factor across any particular point in this space
// JointDistribution is a type for this.
// Is this a type of Factor?

// How do I get the outcome space of the random variables?
// Going to try look annnt a local outcome space first.
fn save_graph(bg: &DiGraph<DiscreteRandomVariable, &str>, filename: &str) -> std::io::Result<()> {
    println!("{:?}", Dot::with_config(&bg, &[Config::EdgeNoLabel]));
    let mut f = File::create(filename).unwrap();
    let output = format!("{}", Dot::with_config(&bg, &[Config::EdgeNoLabel]));
    return f.write_all(&output.as_bytes());
}
