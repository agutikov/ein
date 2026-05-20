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

;; Processed 2026-05-19:
;;   - glossary                      → docs/kernel/glossary.md (+ docs/kernel/README.md index entry)
;;   - ontology deduction by NL      → docs/ideas/04-nlp-to-graph-to-solver-pipeline.md (new section)
;;                                     + docs/kernel/ir/03-ein-lang/01_grammar.md (facts-block-head vs facts-as-objects note)
;;   - type systems by relations     → plans/followups/f4_cross_cutting.md (new sub-section in Q34)
;;   - self-modification (3 rungs)   → docs/ideas/10-generic-self-modification.md (umbrella)
;;                                     + plans/followups/f5_rules_as_data.md (NEW rung 2)
;;                                     + plans/followups/f6_modify_own_harness.md (NEW rung 3)
;;                                     + plans/followups/README.md + plans/followups/f2_*.md cross-refs
;;                                     + docs/ideas/README.md index entry
;;   - # ein model                   → docs/kernel/ir/01-ein-graph/03_ein_model.md (reflexive algebra)
;;                                     + docs/kernel/ir/01-ein-graph/04_jack_drinks_coffee.md (worked example)
;;                                     + plans/m1_core_graph_reasoning/open_questions.md (Q27 + Q28)
;;                                     + plans/open_questions.md (global index)
;;                                     + plans/ideas.md (P1.2b audit live entry)
;;                                     + docs/kernel/ir/01-ein-graph/README.md (file index update)


followup ideas on inference of rules
(opposite or orthogonal to inference of facts)

1) there are rules that are pure structure - no hardcoded names, only variables
    example: symmetric, implies
2) rules that use named predicates (neq, not, and, ...)
    example: transitive
3) rules that use rel vars and rel consts 
    example: square-* family
4) and F1 rules that has no rel vars
    example: obsolete type-exclusivity form, that was transformed into (3)

Rules that have only 1 rel var - describe property of this one relation
Rules that have multiple rel vars - describe properties of those relations interaction
    example: square-fwd describes inference of co-location on ?R relation basis
    Btw ?R here is strict order
    so this square rule can be formulated as a property of co-located relation over right-of order relation

No doubt move from (4) to (2) or (1) can be automated. That's one followup.
Next one - automated inference of rules from relations.
This is part of ontology induction/deduction/inference/search mentioned here docs/ideas/04-nlp-to-graph-to-solver-pipeline.md

Topic is wide, worth it's own followup document


---

COW for branching
should work perfect because we have no deletes and no writes, only appends

