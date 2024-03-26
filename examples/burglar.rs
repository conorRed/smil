use petgraph::graph::DiGraph;
use smil::assignments;
use smil::DiscreteRandomVariable;

pub fn main() -> () {
    let alarm = DiscreteRandomVariable::new(String::from("A"), vec![1, 0], 2);
    let burglar = DiscreteRandomVariable::new(String::from("B"), vec![1, 0], 2);
    let earthquake = DiscreteRandomVariable::new(String::from("E"), vec![1, 0], 2);
    let radio = DiscreteRandomVariable::new(String::from("R"), vec![1, 0], 2);

    let g = DiGraph::<DiscreteRandomVariable, ()>::new();
    let burglar_node = g.add_node(burglar);
    let earthquake_node = g.add_node(earthquake);
    let alarm_node = g.add_node(alarm);
    let radio_node = g.add_node(radio);
    g.add_edge(burglar_node, alarm_node, ());
    g.add_edge(earthquake_node, alarm_node, ());
    g.add_edge(earthquake_node, radio_node, ());

    // specify CPD for alarm
    alarm.set_cpd(vec![0.99, 0.99, 0.99, 0.01]);

    let _combinations_g = assignments(&vec![d, i, g]);
    println!("{:?}", _combinations_g);
}
