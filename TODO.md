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

;; Processed 2026-05-24:
;;   - P1.5a — house-* rename + pypy → plans/m1_core_graph_reasoning/p1.5a_zebra_solution/README.md
;;                                     (new stages S1.5a.5 rename, S1.5a.6 pypy compatibility/perf,
;;                                     S1.5a.7 hypgen scoring + branch-info ordering; dedicated
;;                                     sections "Relation-name refactor" / "PyPy" / "Hypothesis
;;                                     scoring and branch-info ordering" added)
;;   - global planning framing       → plans/README.md (new "Roadmap at a glance" preamble:
;;                                     M1=solve / M2=NL→IR / M2+=induction via F4 + F7;
;;                                     M1b GUI + M2b paper added to the status table)
;;   - move S1.5.9 → P1.8            → git mv plans/m1_core_graph_reasoning/p1.5_hypothesis_loop/
;;                                     s1.5.9_ein_lang_macros.md → plans/m1_core_graph_reasoning/
;;                                     p1.8_ein_lang_modules/s1.5.9_ein_lang_macros.md
;;                                     (sticky id; only directory changes)
;;                                     + P1.5 README row redirects to new path
;;                                     + P1.5a README cross-link updated
;;                                     + P1.8 README Theme A lists S1.5.9 in stages
;;                                     + relative cross-refs inside the file updated
;;   - P1.6 rename + CLI restructure → plans/m1_core_graph_reasoning/p1.11_package_restructure/README.md
;;                                     (NEW; labelled P1.11 because P1.6 was already taken by
;;                                     rendering + trace. ein-bot/ein_bot → ein, demo merge into
;;                                     package, cli.py split into folder.)
;;                                     + plans/m1_core_graph_reasoning/README.md (P1.11 row)
;;   - P1.5a ideas (hypgen scoring)  → plans/m1_core_graph_reasoning/p1.5a_zebra_solution/README.md
;;                                     (folded into the new S1.5a.7 stage + "Hypothesis scoring
;;                                     and branch-info ordering" section; the human-reasoning
;;                                     "why color of first house" guiding question parked there)
;;   - P1.8 ideas (induction)        → plans/followups/f7_rule_induction.md (new sub-sub-track
;;                                     B' — instance properties → type properties, with the
;;                                     no-facts/all-facts/partial cases and the sub-type-discovery
;;                                     framing)
;;   - P1.9 ideas (modes + state-hash) → plans/m1_core_graph_reasoning/p1.9_hypothesis_loop_followups/README.md
;;                                       (new catalog rows E21 solve-vs-prove split, E22
;;                                       alive-hyps in state hash, E23 prove speedup framing)
;;                                       + plans/m1_core_graph_reasoning/p1.5_hypothesis_loop/
;;                                         s1.5.3_canonicalisation.md ("Open questions parked here"
;;                                         + Q-S1.5.3.A on alive-hyps soundness, with cross-ref
;;                                         to E22)
;;   - P1.10 - kernel docs           → plans/m1_core_graph_reasoning/p1.10_kernel_docs/README.md
;;                                     (NEW phase placeholder covering 8 themes: IR 4-level
;;                                     split, kernel API ref, stdlib API ref, inference docs,
;;                                     user-vs-dev split, architecture overview, docs/index →
;;                                     docs/lib rename, ein-model atoms-vs-objects + 4-level KB
;;                                     refinement)
;;                                     + plans/m1_core_graph_reasoning/README.md (P1.10 row)
;;   - M1b : GUI                     → plans/m1b_gui/README.md (NEW placeholder; three views
;;                                     — ein-lang code, ein-graph, branches — with composable
;;                                     2-/3-pane layouts; stack choice / state-sync open)
;;                                     + plans/README.md status table
;;   - M2b : presentation + paper    → plans/m2b_presentation/README.md (NEW placeholder;
;;                                     four tracks — comparison to prior art, benchmarks,
;;                                     results write-up, growth directions)
;;                                     + plans/README.md status table
;;   - F1b - logical formulation     → plans/followups/f1b_logical_formulation.md (NEW;
;;                                     FOL fragment characterisation, propositional-calculus
;;                                     rules of inference, relation algebra, equivalence
;;                                     relation; sibling to F1's CT angle)
;;                                     + plans/followups/README.md (index row F1b)






---

P1.8 - performance part


compression - make atoms vector and replace all atoms everythere else with indexes
all relation facts become vectors of numbers - head is [0]
then if we have few atoms - can compress entire relation to number,
for example < 255 atoms 64bit integer can hold up to 8 atom flat relation
then every fact also can be encoded with unique sequential index - there would be much more of them than atoms - so can use only vectors for non-flat facts, but what are they? (not fact) (not (and fact fact fact))
maybe still can be encoded into vectors or numbers
finally think about this encoding like about hash itself or source for hash


consistent hashing or something similar
- hash method for unsat cores that allows to see if another kb has the unsat core included by comaring caches
- maybe not strict - like Bloom filter, would it help at all anyhow?


re-saturation and hyps alive-recompute:
- build and hold indexes of atoms participation: objects, relations, rules etc...
    - so when see a new fact (?R ?x ?a)
    - we also see a KB subset of facts that may interact with the new fact
    - and subset of rules the fact can interact through
Is it correct? Is it possible to implement?



---

P1.10 kernel docs - separate file for kernel inference feature list absolutely required to solve zebra in reasonable time
add config options for every (if not yet) write ein files with different config options and measure solution time with 3600s timeout
collect into table with time and stats showing impact of every option disabled


---

vscode ein syntax highlighting - to P1.6

---

M1a - ein.rs in Rust, before M1b GUI


---

to Followups:

kernel inference already have quite a lot of features about how it derive facts and where propagates them
we are trying to minimize the kernel to some reasonably minimal, not esoteric-minimal, not theoretically-minimal
what inference features make sense to express in ein lang instead of kernel code: saturation, branching, hypgen, hypfilter, backprop, ...?


---


expression proposal for M1+

(co-located (drink-loc Milk ?h) (smoke-loc Kools ?h))

Aready existing meaning (drink-loc Milk ?h) - House where milk is drinked.
So (drink-loc Milk ?h) is a variable with context, but it is mostly variable.
How to work with it? Does it make sense to implement?


---


version-based saturation optimization

version-based COW-powered KB structure


---


