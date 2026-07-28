pub struct ConceptMemory;

impl ConceptMemory {
    pub fn new() -> Self { Self }
    pub fn consolidate(&mut self, _ctrl: &crate::consolidation::ConsolidationController) -> bool {
        true // simplified: always allow consolidation proposal
    }
}
