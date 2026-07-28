use crate::cluster::ClusterLayer;

pub struct EnergyController;

impl EnergyController {
    pub fn new() -> Self { Self }

    // Unified proxy with threshold gate
    pub fn compute(&self, clusters: &ClusterLayer) -> f64 {
        let k = clusters.clusters.len() as f64;
        if k == 0.0 { return f64::INFINITY; }
        let avg_mass = clusters.clusters.iter().map(|c| c.mass).sum::<f64>() / k;
        let drift = clusters.clusters.iter().map(|c| (c.mass - avg_mass).abs()).sum::<f64>() / k;
        let score = drift * 0.5 + 1.0;
        score
    }

    pub fn should_execute(&self, clusters: &ClusterLayer) -> bool {
        self.compute(clusters) < 1.5 // threshold if energy is healthy
    }
}
