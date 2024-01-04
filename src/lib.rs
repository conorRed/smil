// A *Factor* is a function $\phi$ that returns a value for each variable. For a set of variables X1, ...,
// $X_k$ is returns a real valued number for each combination of values. $\phi : V al(X_1, ..., X_k) → R$.
// It’s scope is the set of values which it is defined over (its arguments).

// how do we limit the arguments to factor? Is it a simple function?

use std::fmt;

#[derive(Debug, Default)]
pub struct DiscreteRandomVariable {
    name: String,
    domain: Vec<f32>,
}

impl fmt::Display for DiscreteRandomVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl DiscreteRandomVariable {
    pub fn new(name: String, domain: Vec<f32>) -> Self {
        DiscreteRandomVariable { name, domain }
    }
}

// Need to think about a state space. What is the 'combinatorial expolsian' of the random variables
// and do we have a factor across any particular point in this space
// JointDistribution is a type for this.
// Is this a type of Factor?

struct JointDistribution {
    random_variables: Vec<DiscreteRandomVariable>,
}

impl JointDistribution {
    fn entries(&self, fix: DiscreteRandomVariable) {
        let cols = fix.domain.len();
        for rv in self.random_variables.iter() {}
    }
}
