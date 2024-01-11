// A *Factor* is a function $\phi$ that returns a value for each variable. For a set of variables X1, ...,
// $X_k$ is returns a real valued number for each combination of values. $\phi : V al(X_1, ..., X_k) → R$.
// It’s scope is the set of values which it is defined over (its arguments).

// how do we limit the arguments to factor? Is it a simple function?

use itertools::Itertools;
use petgraph::graph::DiGraph;
use std::fmt;

// Random Variables can be thought of in set notation, having a cardinality
#[derive(Debug, Clone)]
pub struct DiscreteRandomVariable {
    name: String,
    domain: Vec<i32>,
    cardinality: i32,
}

trait PMF {
    fn px(x: i32) -> f32;
}

impl fmt::Display for DiscreteRandomVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
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

    //    In general, each variable X in the model is associated with a conditional probability
    //    distribution (CPD) that specifies a distribution CPD over the values of X given each possible
    //    joint assignment of values to its parents in the model.
    pub fn cpd(&self, conditionals: &Vec<DiscreteRandomVariable>) -> () {
        let mut vec_of_domains: Vec<Vec<i32>> =
            conditionals.iter().map(|rv| rv.domain.clone()).collect();

        vec_of_domains.push(self.domain.clone());
        let _outcome_space_size: usize =
            conditionals.iter().fold(1, |acc, rv| rv.domain.len() * acc) * self.domain.len();

        let cartesian_product: Vec<Vec<&i32>> = vec_of_domains
            .iter()
            .multi_cartesian_product()
            .take(_outcome_space_size)
            .collect();
        println!(
            "Outcome space: {} {:?}",
            _outcome_space_size, cartesian_product
        );
    }

    // return sub graph for one combination
    // The Garden of Forking paths!
}

pub fn assignments(conditionals: &Vec<DiscreteRandomVariable>) -> () {
    let vec_of_domains: Vec<Vec<i32>> = conditionals.iter().map(|rv| rv.domain.clone()).collect();
    let _outcome_space_size: usize = conditionals.iter().fold(1, |acc, rv| rv.domain.len() * acc);

    let cartesian_product: Vec<Vec<&i32>> = vec_of_domains
        .iter()
        .multi_cartesian_product()
        .take(_outcome_space_size)
        .collect();
    println!(
        "Outcome space: {} {:?}",
        _outcome_space_size, cartesian_product
    );
}
// Need to think about a state space. What is the 'combinatorial expolsian' of the random variables
// and do we have a factor across any particular point in this space
// JointDistribution is a type for this.
// Is this a type of Factor?

// How do I get the outcome space of the random variables?
// Going to try look annnt a local outcome space first.
