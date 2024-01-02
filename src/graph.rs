use crate::variable::Variable;
#[derive(Debug)]
pub struct Graph {
    nodes: Vec<Variable>,
}

impl Graph {
    pub fn new(nodes: Vec<Variable>) -> Self {
        return Graph { nodes };
    }
}
