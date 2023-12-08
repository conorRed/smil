use variable::Bernoulli;
mod variable;
fn main(){
    let b = Bernoulli::new(0.45);
    println!("{}", b.p());
    println!("{}", b.q());
    println!("{}", b.pq());
}
