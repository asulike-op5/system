use ndarray::Array1;
use std::collections::HashMap;

pub struct RedundancyTracker {
    pairs: HashMap<(usize, usize), f64>,
}

impl RedundancyTracker {
    pub fn new() -> Self { Self { pairs: HashMap::new() } }
    pub fn update(&mut self, centers: &[Array1<f64>]) {
        for i in 0..centers.len() {
            for j in (i+1)..centers.len() {
                let d = (&centers[i] - &centers[j]).dot(&(&centers[i] - &centers[j])).sqrt();
                self.pairs.insert((i, j), d);
            }
        }
    }
}
