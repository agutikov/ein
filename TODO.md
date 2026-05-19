; Scratchpad for raw thoughts. When an item is embedded into plans/ or
; docs/, prune it from here. The trail lives in commit history.

;; Processed 2026-05-18:
;;   - rule demo problems            → plans/m1_core_graph_reasoning/p1.3_inference_rules/s1.3.2_ten_core_rules.md (T1.3.2.12)
;;   - puzzles & benchmarks catalog  → docs/ideas/09-puzzles-beyond-zebra.md, docs/index/12-llm-and-reasoning-benchmarks.md
;;   - types/relations raw           → plans/followups/f4_cross_cutting.md (Q34 expanded, Q37+Q38 added),
;;                                     plans/m1_core_graph_reasoning/open_questions.md (Q22..Q25),
;;                                     plans/m1_core_graph_reasoning/p1.2_typed_hypergraph/s1.2.1_data_model.md,
;;                                     plans/m1_core_graph_reasoning/README.md (rules-of-thumb)
;;   plans/raw/ was a working scratchpad — gone after the embedding above.


expand /docs/ir.md into kernel documentation

Motivation: /docs/ir.md is symbolic language description first, must be graph-first
graph represents the semantic better
lets make a M1 kernel documentation

/docs/ir.md -mostly-goes-to-> /docs/kernel/ir/03-ein-lang/

/docs/kernel/ir/01-ein-graph/ - graph-only description
- 01_kb.md
    - knowledge base
    - no ein language or Python
    - 2 representations:
        - compact: instances, types, 2-relations as named arrows
        - detailed: objects-nodes, relation-nodes
- 02_rules.md
    - graph rewriting rules 
    - 3 types of rules description
    - no ein language or Python

/docs/kernel/ir/02-data-model/ - Python, in-memory data model, types

/docs/kernel/ir/03-ein-lang/ - language

/docs/kernel/inference/ - stub before P1.3

/docs/kernel/REAME.md




















