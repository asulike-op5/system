use ndarray::Array2;

pub struct TemperatureLayer {
    pub temperatures: Vec<f64>,
    pub decay: f64,
    pub excitement: f64,
}

impl TemperatureLayer {
    pub fn new() -> Self {
        Self { temperatures: Vec::new(), decay: 0.95, excitement: 0.05 }
    }

    pub fn update_from_assignments(&mut self, assignments: &Array2<f64>) {
        let entropy: f64 = assignments.iter().map(|p| {
            let p = *p;
            if p > 0.0 { -p * p.ln() } else { 0.0 }
        }).sum();
        let n = assignments.shape()[0] as f64;
        let avg_entropy = entropy / n.max(1.0);
        for t in &mut self.temperatures {
            *t = (*t) * self.decay + avg_entropy * self.excitement;
            if *t < 0.01 { *t = 0.01; }
        }
    }

    pub fn sync_from_clusters(&mut self, clusters: &crate::cluster::ClusterLayer) {
        self.temperatures = clusters.clusters.iter().map(|c| c.temperature).collect();
    }
}
