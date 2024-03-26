use smil::assignments;
use smil::DiscreteRandomVariable;
use smil::Factor;

pub fn main() -> () {
    let d = DiscreteRandomVariable::new(String::from("D"), vec![6, 4], 2);
    let i = DiscreteRandomVariable::new(String::from("I"), vec![7, 3], 2);
    let g = DiscreteRandomVariable::new(String::from("G"), vec![1, 3], 2);
    let p_g_id = Factor::new(&vec![g, i, d]);
    p_g_id.set_values(vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0]);
}
