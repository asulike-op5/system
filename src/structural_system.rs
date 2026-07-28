use ndarray::Array2;
use crate::encoder::Encoder;

pub struct StructuralSystem {
    cluster_layer: crate::cluster::ClusterLayer,
    temperature: crate::temperature::TemperatureLayer,
    memory_store: crate::memory_store::MemoryStore,
    concept_memory: crate::concept_memory::ConceptMemory,
    structural_executor: crate::structural_executor::StructuralExecutor,
    budget_scheduler: crate::budget_scheduler::BudgetScheduler,
    structural_queue: crate::structural_queue::StructuralQueue,
    redundancy: crate::redundancy::RedundancyTracker,
    consolidation: crate::consolidation::ConsolidationController,
    trace_engine: crate::trace_engine::TraceEngine,
}

impl StructuralSystem {
    pub fn new(dim: usize) -> Self {
        Self {
            cluster_layer: crate::cluster::ClusterLayer::new(dim),
            temperature: crate::temperature::TemperatureLayer::new(),
            memory_store: crate::memory_store::MemoryStore::new(),
            concept_memory: crate::concept_memory::ConceptMemory::new(),
            structural_executor: crate::structural_executor::StructuralExecutor::new(),
            budget_scheduler: crate::budget_scheduler::BudgetScheduler::new(),
            structural_queue: crate::structural_queue::StructuralQueue::new(),
            redundancy: crate::redundancy::RedundancyTracker::new(),
            consolidation: crate::consolidation::ConsolidationController::new(),
            trace_engine: crate::trace_engine::TraceEngine::new(),
        }
    }

    pub fn step(&mut self, x: &Array2<f64>) {
        // 1. Encode
        let z = Encoder::encode(x);

        // 2. Cluster Assignment (soft)
        let assignments = self.cluster_layer.assign(&z);

        // 3. EM Update
        self.cluster_layer.em_update(&z, &assignments);

        // 4. Temperature update
        self.temperature.update_from_assignments(&assignments);
        self.temperature.sync_from_clusters(&self.cluster_layer);

        // 5. Redundancy tracking
        self.redundancy.update(&self.cluster_layer.centers());

        // 6. Structural phase (every 50 steps by default)
        self.budget_scheduler.tick();
        if self.budget_scheduler.should_restructure() {
            self.run_structural_phase();
        }

        // 7. Memory & Consolidation + Trace
        self.memory_store.update(&self.cluster_layer);
        self.consolidation.evaluate(&self.cluster_layer, &self.memory_store);
        self.trace_engine.record("step completed");
    }

    fn run_structural_phase(&mut self) {
        // Propose, budget, execute merges/splits, consolidate concepts
        self.structural_executor.execute(
            &mut self.cluster_layer,
            &self.structural_queue,
            &self.budget_scheduler,
        );
        self.concept_memory.consolidate(&self.consolidation);
    }
}
