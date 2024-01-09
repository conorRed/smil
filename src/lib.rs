// A *Factor* is a function $\phi$ that returns a value for each variable. For a set of variables X1, ...,
// $X_k$ is returns a real valued number for each combination of values. $\phi : V al(X_1, ..., X_k) → R$.
// It’s scope is the set of values which it is defined over (its arguments).

// how do we limit the arguments to factor? Is it a simple function?

use petgraph::graph::DiGraph;
use std::fmt;

// Random Variables can be thought of in set notation, having a cardinality
#[derive(Debug, Clone)]
pub struct DiscreteRandomVariable {
    name: String,
    domain: Vec<f32>,
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
    pub fn new(name: String, domain: Vec<f32>, cardinality: i32) -> Self {
        DiscreteRandomVariable {
            name,
            domain,
            cardinality,
        }
    }

    //    In general, each variable X in the model is associated with a conditional probability
    //    distribution (CPD) that specifies a distribution CPD over the values of X given each possible
    //    joint assignment of values to its parents in the model.
    pub fn cpd(&self, conditionals: &Vec<DiscreteRandomVariable>) -> DiGraph<f32, &str> {
        // create a graph of the combinatorial space.
        let mut combinatorial_space = DiGraph::<f32, &str>::new();
        // a node for each combination
        // a sub tree for each domain value.
        for &d in self.domain.iter() {
            // create left
            // create right
            let root = combinatorial_space.add_node(d);
            // add child nodes of all conditionals
            for rv in conditionals {
                for &cd in rv.domain.iter() {
                    let node_cd = combinatorial_space.add_node(cd);
                    combinatorial_space.add_edge(node_d, node_cd, "");
                }
            }
        }

        return combinatorial_space;
    }

    pub fn subtree_(root_value: f32, children: &Vec<f32>) -> DiGraph<f32, &str> {
        let mut subtree: DiGraph<f32, &str> = DiGraph::<f32, &str>::new();
        let root = subtree.add_node(root_value);
        if children.len() == 0 {
            return subtree;
        }
        for child in children {
            subtree.add_node(subtree_(child, ));
        }
        return subtree;
    }

    // return sub graph for one combination
    // The Garden of Forking paths!
    pub fn inner_cpd(&self, domain_value: f32, conditionals: &Vec<f32>) -> DiGraph<f32, &str> {
        let mut combinatorial_space = DiGraph::<f32, &str>::new();
        let node_d = combinatorial_space.add_node(domain_value);
        if conditionals.len() == 0 {
            return combinatorial_space;
        }
        for &cd in conditionals {
            let node_cd = cd.cpd(cd, &conditionals[1..]);
            inner_cpd(cd, )
            combinatorial_space.add_edge(node_d, node_cd, "");
        }

        return combinatorial_space;
    }
}

// Need to think about a state space. What is the 'combinatorial expolsian' of the random variables
// and do we have a factor across any particular point in this space
// JointDistribution is a type for this.
// Is this a type of Factor?

// How do I get the outcome space of the random variables?
// Going to try look annnt a local outcome space first.
