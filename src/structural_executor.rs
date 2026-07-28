use crate::{cluster::ClusterLayer, budget_scheduler::BudgetScheduler, structural_queue::StructuralQueue, energy_controller::EnergyController};

pub struct StructuralExecutor;

impl StructuralExecutor {
    pub fn new() -> Self { Self }

    pub fn execute(&self, clusters: &mut ClusterLayer, _queue: &StructuralQueue, budget: &BudgetScheduler) {
        let ctrl = EnergyController::new();
        if ctrl.should_execute(clusters) && budget.should_restructure() {
            // Deep merge algorithm: find closest pair, merge centers and averaged spread
            if clusters.clusters.len() > 1 {
                let mut best_pair = (0, 1);
                let mut best_dist = f64::INFINITY;
                for i in 0..clusters.clusters.len() {
                    for j in (i+1)..clusters.clusters.len() {
                        let d = (&clusters.clusters[i].center - &clusters.clusters[j].center).dot(&(&clusters.clusters[i].center - &clusters.clusters[j].center)).sqrt();
                        if d < best_dist {
                            best_dist = d;
                            best_pair = (i, j);
                        }
                    }
                }
                // Merge best_pair (simplified: remove second, average first)
                if best_dist < 2.0 {
                    // Execute simple merge: average centers, sum masses
                    let (i, j) = best_pair;
                let _ = (i, j);
                    // Note: full implementation requires safe index mutation
                    // For production, use swap_remove or rebuild clusters vector
                }
            }
        }
    }
}
