#[derive(Debug)]
pub struct Variable {
    p: f64,
    q: f64,
}

impl Variable {
    pub fn new(p: f64) -> Self {
        Variable { p, q: 1.0 - p }
    }

    pub fn p(&self) -> f64 {
        return self.p;
    }
    pub fn q(&self) -> f64 {
        return self.q;
    }
}
