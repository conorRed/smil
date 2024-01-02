mod graph;
mod variable;

fn main() {
    let v1 = variable::Variable::new(0.5);
    let g = graph::Graph::new(vec![v1]);
    dbg!(g);
}
