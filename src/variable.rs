pub struct Bernoulli {
    p: f64,
    q: f64,
    pq: f64,
}

impl Bernoulli {
    // implement some functionality for a type
    pub fn new(p: f64) -> Self {
        Bernoulli {
            p, 
            q: 1.0 - p, 
            pq: p*(1.0 - p),
        }
    }

    pub fn p(&self) -> f64 {
        return self.p
    }
    pub fn q(&self) -> f64 {
        return self.q
    }
    pub fn pq(&self) -> f64 {
        return self.pq
    }
}

