use crate::{cluster::ClusterLayer, memory_store::MemoryStore};

pub struct ConsolidationController;

impl ConsolidationController {
    pub fn new() -> Self { Self }
    pub fn evaluate(&self, clusters: &ClusterLayer, _memory: &MemoryStore) -> bool {
        clusters.clusters.len() > 0 && clusters.clusters.iter().all(|c| c.mass > 0.01)
    }
}
