use ndarray::{Array1, Array2};
use rand::Rng;

pub struct Cluster {
    pub center: Array1<f64>,
    pub spread: Array1<f64>, // diagonal covariance approximation
    pub mass: f64,
    pub temperature: f64,
}

pub struct ClusterLayer {
    pub clusters: Vec<Cluster>,
    pub dim: usize,
}

impl ClusterLayer {
    pub fn new(dim: usize) -> Self {
        Self { clusters: Vec::new(), dim }
    }

    pub fn assign(&self, z: &Array2<f64>) -> Array2<f64> {
        // Soft assignment using Mahalanobis-like distance (simplified)
        let n = z.shape()[0];
        let k = self.clusters.len().max(1);
        let mut probs = Array2::<f64>::zeros((n, k));
        for (i, row) in z.rows().into_iter().enumerate() {
            let mut dists = Vec::new();
            for c in &self.clusters {
                let diff = &row - &c.center;
                let spread_inv = c.spread.mapv(|v| v.max(1e-3));
                let scaled = diff.clone() / spread_inv;
                let dist = diff.dot(&scaled);
                dists.push(dist.sqrt());
            }
            let min_d = dists.iter().cloned().fold(f64::INFINITY, f64::min);
            let mut sum = 0.0;
            for (j, d) in dists.iter().enumerate() {
                let s = (-(*d - min_d)).exp();
                probs[[i, j]] = s;
                sum += s;
            }
            if sum > 0.0 {
                for j in 0..k { probs[[i, j]] /= sum; }
            }
        }
        probs
    }

    pub fn em_update(&mut self, z: &Array2<f64>, probs: &Array2<f64>) {
        if self.clusters.is_empty() {
            // Initialize with first batch mean
            let mean = z.mean_axis(ndarray::Axis(0)).unwrap();
            self.clusters.push(Cluster {
                center: mean,
                spread: Array1::ones(self.dim),
                mass: 1.0,
                temperature: 1.0,
            });
            return;
        }
        // Update centers and mass based on soft assignments
        // Dead-cluster revival if mass collapses
        let avg_mass = self.clusters.iter().map(|c| c.mass).sum::<f64>() / self.clusters.len() as f64;
        for (i, c) in self.clusters.iter_mut().enumerate() {
            let weights = probs.column(i).to_owned();
            let total_weight: f64 = weights.iter().sum();
            if total_weight < avg_mass * 0.1 {
                // Revive near random data point
                let idx = rand::thread_rng().gen_range(0..z.shape()[0]);
                c.center = z.row(idx).to_owned();
                c.spread = Array1::ones(self.dim);
                c.mass = avg_mass;
                c.temperature = 1.0;
            } else {
                let mut new_center = Array1::<f64>::zeros(self.dim);
                for (row_idx, w) in weights.iter().enumerate() {
                    new_center += &(z.row(row_idx).to_owned() * *w);
                }
                if total_weight > 0.0 { new_center /= total_weight; }
                c.center = new_center;
                c.mass = total_weight;
            }
        }
    }

    pub fn centers(&self) -> Vec<Array1<f64>> {
        self.clusters.iter().map(|c| c.center.clone()).collect()
    }
}
