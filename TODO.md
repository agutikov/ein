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

;; Processed 2026-05-21:
;;   - rule classification + induction (4-class taxonomy: pure-structure /
;;     predicate-using / rel-var+const / no-rel-vars; single vs multi rel-var;
;;     auto-generalisation, induction from relations, rule-set sufficiency;
;;     M2 NL→IR pipeline ties in)
;;                                   → plans/followups/f7_rule_induction.md (NEW)
;;                                     + plans/followups/README.md (index row F7)
;;   - COW for hypothesis branching  → plans/m1_core_graph_reasoning/p1.8_ein_lang_modules/README.md
;;                                     (Theme B; append-only KB makes true COW trivially correct)
;;   - (not …) fact volume reduction → plans/m1_core_graph_reasoning/p1.8_ein_lang_modules/README.md
;;                                     (Theme C; lazy materialisation / layer-aware filter /
;;                                     goal-driven pruning / compressed representation)
;;   - P1.8 "rename" suggestion      → plans/m1_core_graph_reasoning/p1.8_ein_lang_modules/README.md
;;                                     (title broadened from "Ein-lang modules + imports" to
;;                                     "Improvements"; directory name kept to avoid cross-link
;;                                     churn across S1.3.0, P1.3 README, etc.)
;;                                     + plans/m1_core_graph_reasoning/README.md (P1.8 row updated)



P1.5 more ideas

- guided hypothesis generation by rules from specific head or with specific name `hrule`
- option in config to enable automatic hypothesis generation, default is enabled
- another simple expernal guiding - allow hypothesis only of certain relation, e.g. `co-located` in zebra



- implication:
    - implry ?R1 ?R2 - implication between relations
        - what about implication between rules?
            - Is there any thechnical blockers?
                - we can issue new fact of course
                - but how it will be handled then?
                - would it be processed correctly? compiled, etc.
            - Type-match for rules domains?
            - we now don't check relation domains either btw
    - imply1 - for relations with 1 arg
    - imply2-fwd - for relations with 2 args - our current default imply
    - imply2-reverse, or inverse - for example right-to and left-to


- generalization rule, induction, not for Zebra
    - generalize instances properties to types properties
    - simple examples:
        - no facts of relation R between instances of A and B - no relation R betewen A and B types
        - all instances of A and B have relations R - types have relation R
        - some instances of A have relation R with instances of B, some hot have -> there are 2 subtypes


- modes:
    - solve - search for at least one valid solution ASAP, use heuristics to optimize the search
    - prove - full search, the same in case of one solution, different in case of multiple
        - is there a way to speedup prove?
        - not doing full search, replace it with what???


- Q: is it possible to get into the same state (that collapse into one) with different path so have different sets of alive hypothesis? Maybe add hyps to canonical state hash?



---


P1.10 - kernel docs


existing IR 3-level docs expand to 4:
- ein-graph remain
- data-models splits into
    - ideomatic data model
        - data types, collections, indexes, ...
    - python implementation
- ein-lang remain


inference docs
- ideomatic level
    - collections, indexes
    - algorithm diagrams
- python implementation
    - files, modules, data types, functions, classes



software architecture docs with diagrams


rename docs/index to docs/lib



also, some minor ein-model update:
- want to differentiate objects and atoms
    - atoms are names
        - `rule`, `not`, `T`, `relation`, `Alice`, `co-located` - are all atoms
    - objects are, as before, Levi-bipartite graph nodes without out arrows and with in arrows from facts
- want to declare 4-levels representation
    - 4 leves
        - objects
        - facts
        - relations
        - rules
    - idea: create self-describing model of KB types in IR ein lang



---

M1b GUI
after M1 before M2

views:
- ein language code
    - source
    - generated states
- ein-graph
    - unified and separate parts, like DOT rendering
    - compact and detailed (Livi-bipartite) view
    - auto layout, different modes
    - manual layout, remember positions
    - GUI editor
- branches
    - git mode
        - bottom to top
        - all branches dead-ends, main - solution
    - folders tree mode (top to bottom)
    - collapse branches
    - collapse chains
    - every folder/revision is a state == ein lang + graph view

2-view mode: left-right branches - lang+graph tabs 
3-view mode: all 3 views

---

M2b compare to other systems from docs/index
try benchmarks
summarize results
list further growth directions


