MASTER GUIDE - FULL SYSTEM WORKING

MODULES AND WHERE TO FEED DATA:
1. encoder.py    -> feeds raw batch (Array2) -> outputs normalized embeddings
2. cluster.py    -> feeds embeddings -> assigns probabilities + EM updates
3. temperature.py-> feeds assignments -> updates per-cluster temperature
4. redundancy.py -> feeds centers -> tracks pair similarities
5. structural_queue.py -> feeds proposed events -> stores merge/split
6. budget_scheduler.py -> tracks steps -> allows restructuring every 50
7. structural_executor.py -> executes merges/splits with energy threshold
8. memory_store.py -> feeds clusters -> accumulates examples
9. consolidation.py -> feeds clusters + memory -> evaluates stability
10. concept_memory.py -> promotes stable clusters to concepts
11. concept_graph.py -> builds relationships between concepts
12. concept_reasoner.py -> searches graph from start node
13. energy_controller.py -> computes health from cluster state
14. trace_engine.py -> records explanations
15. structural_system.py -> orchestrates step()
16. python_bindings.py -> Python interface: PyStructuralSystem

WHERE TO FEED DATASETS:
- Python: s = PyStructuralSystem(dim=128); s.step(batch)
- Batch format: list of float lists (e.g., embeddings from sentence-transformers)

HOW TO KNOW SYSTEM IS LEARNING:
- cluster_layer: number of clusters grows/shrinks (new categories invented)
- temperature: rises in new/uncertain regions, falls in settled ones
- energy_controller: lower score = healthier system
- redundancy: new pairs appear (clusters interacting)
- memory_store: examples accumulate (live storage growing)
- consolidation: evaluates true when clusters stable
- trace_engine: records "step completed" (proves loop runs)
- budget_scheduler: tick increases (training progressing)

HOW TO THINK / CHAT WITH SYSTEM:
- This is an UNSUPERVISED STRUCTURAL FRAMEWORK, not a chatbot
- It doesn't answer questions; it INVENTS STRUCTURE from your data stream
- To "think" with it: feed it embeddings, observe temperature/energy/concept growth
- To explain: read trace_engine records + energy scores
- To interact: call step() repeatedly, monitor dashboard metrics
