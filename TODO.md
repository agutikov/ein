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
;;                                     rendering + trace. Ein/ein → ein, demo merge into
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
;;   - P1.20 - kernel docs           → plans/m1_core_graph_reasoning/p1.20_kernel_docs/README.md
;;                                     (NEW phase placeholder covering 8 themes: IR 4-level
;;                                     split, kernel API ref, stdlib API ref, inference docs,
;;                                     user-vs-dev split, architecture overview, docs/index →
;;                                     docs/lib rename, ein-model atoms-vs-objects + 4-level KB
;;                                     refinement)
;;                                     + plans/m1_core_graph_reasoning/README.md (P1.20 row)
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

;; Processed 2026-05-27:
;;   - M1a Rust port               → plans/m1a_rust/README.md (NEW placeholder;
;;                                   ein.rs slots between M1 and M1b; PyPy 6.0×
;;                                   on zebra2 d=1 motivates native port;
;;                                   Boundary A vs B left open)
;;                                   + plans/README.md (status table M1a row +
;;                                   prose preamble)
;;   - P1.6 Levi/compact defaults  → plans/m1_core_graph_reasoning/
;;                                   p1.6_rendering_and_trace/README.md
;;                                   (new "Defaults" + "render_examples.sh"
;;                                   sections; compact project-wide,
;;                                   Levi-by-flag; rule mode (a)+LR, trace
;;                                   per-step; collapse 6-variant matrix to 1
;;                                   variant per file)
;;                                   + VSCode syntax highlighting note
;;   - S1.6.1 absent-folded match  → plans/m1_core_graph_reasoning/
;;                                   p1.6_rendering_and_trace/s1.6.1_dot_rules_constraints.md
;;                                   (Task T1.6.1.1 amended with attention to
;;                                   nested (absent (and …)) rendering;
;;                                   sub-cluster recommendation)
;;   - P1.7 drop zebra.ein support → plans/m1_core_graph_reasoning/
;;                                   p1.7_bootstrapping_zebra/README.md (new
;;                                   sub-section under Updates; goals: remove
;;                                   type/instance hardcoded keywords; rewrite
;;                                   classic against unified syntax; comment-
;;                                   table audit of what's missing vs zebra2)
;;   - P1.8 compression / hashing /  → plans/m1_core_graph_reasoning/
;;     re-saturation indexes /        p1.8_ein_lang_modules/README.md (three
;;     version-based COW + sat        new Themes added: B2.v version-based COW
;;                                    + version-based saturation; B3 atom-vector
;;                                    compression with 64-bit-int encoding;
;;                                    B4 unsat-core fingerprint / consistent
;;                                    hashing; B5 fact-participation indexes
;;                                    for re-saturation)
;;   - P1.20 features × config       → plans/m1_core_graph_reasoning/
;;     matrix + Ein API + Zebra        p1.20_kernel_docs/README.md (three new
;;     guide                           Themes added: I features-table with 3600s
;;                                    bench matrix; J Python embedding API;
;;                                    K rule-by-rule Zebra walkthrough with
;;                                    compact + Levi graphs before/after)
;;                                    + Acceptance entries for I/J/K
;;   - Kernel minimisation followup → plans/followups/f5_rules_as_data.md
;;                                    (new sub-section "Kernel minimisation —
;;                                    which inference features belong in
;;                                    ein-lang vs kernel code?"; audit
;;                                    candidates: saturation, branching,
;;                                    hypfilter, back-prop, NAF re-eval)
;;   - Facts-as-variables M1+ idea  → plans/followups/f4_cross_cutting.md
;;                                    (new Q39 row in open-questions table +
;;                                    full sub-section with semantics question:
;;                                    fact-as-value vs variable-binding readings)


;; Processed 2026-06-02:
;;   - P1.7c block-head removal     → plans/m1_core_graph_reasoning/p1.7c_block_head_removal/README.md
;;                                    (NEW placeholder; flat-form grammar — facts = head ∉
;;                                    {rule, hrule, relation, query, config, macro}; layer
;;                                    attribution flagged as the crux open question)
;;                                    + plans/m1_core_graph_reasoning/README.md (P1.7c row + prose)
;;   - relation algebra → P1.8      → plans/m1_core_graph_reasoning/p1.8_ein_lang_modules/README.md
;;                                    (Theme A stdlib §; new S1.8.A12 "Relation-algebra library
;;                                    rules" pulling F1b §PFL.3 compose/converse/meet/join + an
;;                                    A12 open-question on the surface vs needed engine support)
;;   - split P1.8 → P1.8 + P1.8a    → git mv the 8 perf stage files (s1.8.b*, s1.8.c*) into
;;                                    plans/m1_core_graph_reasoning/p1.8a_performance/ (NEW phase;
;;                                    sticky ids kept). P1.8 retitled "Ein-language modules +
;;                                    standard library" (lang/lib only); the Theme B/C prose moved
;;                                    + reorganised into the P1.8a README (with a B0 renumber task
;;                                    noting the table-vs-prose B-id collision). m1 README: P1.8 row
;;                                    retitled, P1.8a row added, placeholder prose updated.



;; Processed 2026-06-14:
;;   - zebra2: import its property + elimination rules from   → plans/m1_core_graph_reasoning/p1.8a_performance/
;;     the stdlib instead of inlining                           s1.8a.f20_stdlib_property_rules.md
;;     (functional/injective/total/surjective/bijective →       (NEW stage S1.8a.f20 — the deferred S1.8.A8
;;      std.algebra checks; typecheck/domain-elim/range-elim →  reconciliation: A5-tail byte-identical-resolved-KB
;;      std.elim, reconciling ?isa-param vs is-a-literal,       migration; total/surjective gain ?isa param;
;;      adding range-elim; negative companions = open Q)        range-elim added to std.elim; neg-companion home open)
;;                                                            + plans/.../p1.8a_performance/README.md stage row
;;                                                              (homed in P1.8a as P1.8 is COMPLETE, not perf)
;;   - bench: positive-only --print-final-state               → same stage, Task T.f20.7 (drops (not …) + internal
;;     (skip internal + negative facts)                         heads; positive hypothesis relations only)



