# Structural System — Senior Engineer Analysis

## What IS Working (Keep)
- Encoder → L2 norm + noise injection
- Cluster assignment with adaptive temperature (soft Mahalanobis)
- EM update with sharpened probabilities + dead-cluster revival
- Temperature as per-cluster gate (self-regulating differential)
- Redundancy tracking (slow accumulation)
- Structural queue (priority queue with decay + duplicate prevention — needs fix for non-merge/split events)
- Budget scheduler (adaptive to samples seen)
- Structural executor (merge/split with energy-based accept/reject)
- Memory store (episodic memory per cluster)
- Consolidation (adaptive thresholds)
- Concept memory + concept graph + spreading activation reasoner
- Energy controller (though its formula is separate from executor's proxy — needs unification)
- Trace engine (explainability)

## What Is NOT Working / Dead (Strip Before Production)
- Unused repulsion force, semantic gravity, dynamic split block, dead-cluster removal block in cluster.py (commented out)
- Abandoned resizing schemes in temperature.py
- Abandoned simpler budget formula in budget_scheduler.py
- Memory store semantic_gravity() never called; cluster memory attraction commented out
- energy_controller.py compute() not used for any decision; structural_executor uses its own _compute_energy_proxy()
- concept_memory.py consolidate_cluster() has unconditional print() spam
- Small duplicate logic + accidental double-increment in structural_system.py CASE 2
- StructuralQueue.pop() uses different key rebuild for non merge/split events
- causal_consistency in energy controller always defaults to zero (never contributes)

## Design Philosophy (Keep)
1. Two timescales (fast clusters / slow concepts)
2. Temperature gates change locally
3. Propose freely, accept carefully, allow controlled exploration
4. Self-scaling thresholds (relative to current state)
