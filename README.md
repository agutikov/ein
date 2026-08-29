# EinAf

[![per-commit](https://github.com/agutikov/ein/actions/workflows/per-commit.yml/badge.svg)](https://github.com/agutikov/ein/actions/workflows/per-commit.yml)
[![nightly](https://github.com/agutikov/ein/actions/workflows/nightly.yml/badge.svg)](https://github.com/agutikov/ein/actions/workflows/nightly.yml)

**EinAf — a Neuro-Symbolic Automated Reasoning Framework for Iterative
Autoformalization and Theory Synthesis.**

Two things live in this repository, and this page keeps them apart:

- **Ein** is the symbolic kernel — a graph-native reasoner. A problem is a
  **typed hypergraph** of relations, facts and rules written in
  [ein-lang](docs/kernel/ir/03-ein-lang/) (an S-expression IR); the engine
  **saturates** the rules to a least fixpoint (Datalog-style forward chaining),
  then **searches a commitment lattice** (CSP/SAT-style branch-and-prune with
  ATMS-style provenance and no-good learning), and reads one verdict off the
  number of models it finds. It is **shipped, in Rust, and measured** —
  [§ Built and verified](#built-and-verified--ein-the-symbolic-kernel).
- **EinAf** is the framework around it. A neural component proposes a
  formalization — entities, relations, facts, constraints, rules, a goal — and
  the kernel checks it, reasons over it and reports what it found: a solution,
  the residual ambiguity, a contradiction with its unsat core, a rule that
  never fires, a requirement left unmet. That report goes back to the neural
  side, which revises the formalization or selects / synthesises a better
  theory, and the loop runs to a fixed point or a budget. It is **scheduled**
  ([M2](plans/m2_nl_to_ir/README.md)) and **under investigation**
  ([F12–F17](plans/followups/README.md)) —
  [§ Scheduled](#scheduled--the-roadmap) and
  [§ Open](#open--investigation-directions).

The premise, in the author's words
([F13](plans/followups/f13_puzzles_beyond_zebra/ideas.md)): *the formal part —
which already exists — very quickly solves, or checks the solvability of, an
already-formulated problem and reports the contradictions and the dependencies
hidden in the rules; the neural part does the semantic analysis of the text and
the neural-guided choice of theories and synthesis of new rules; and all of it
runs in a loop until a fixed point or a time limit.* No training, no
fine-tuning: the LLM proposes, the kernel judges. Autoformalization is
therefore **not translation**
([F16](plans/followups/f16_autoformalization/ideas.md)): a puzzle's text states
its facts and leaves its *theory* implicit — that the houses are linearly
ordered, that every attribute is a bijection onto them — so the work is
`NL → discover / select / transform a theory → instantiate it → a
machine-checkable problem`, and the kernel's speed is what makes iterating on
it affordable: once `zebra2.ein` is formalized, it solves in **8 ms**. The
working hypothesis the framework exists to test is F13's last line — *most of
"reasoning" is correctly extracting the relations and choosing a small theory
of their properties; after that, formal inference is cheap.*

```text
   informal problem — NL text (EN / RU), code, a mathematical statement
                │
   ┌────────────▼─────────────────────────────────────────────────────┐
   │  neural side                                    M2 · scheduled   │
   │  semantic analysis  → entities, relations, facts, the goal       │
   │  theory selection   → which properties / rule families apply     │
   │  theory synthesis   → new rules, only when the library has none  │
   └────────────┬─────────────────────────────────────────────────────┘
                │  ein-lang: ontology + theory (rules) + instance (facts) + query
   ┌────────────▼─────────────────────────────────────────────────────┐
   │  Ein kernel                                   M1 + M1a · shipped │
   │  load → saturate (least fixpoint) → search (commitment lattice)  │
   └────────────┬─────────────────────────────────────────────────────┘
                │  k = 1 the solution · k > 1 the models · k = 0 the unsat core
                │  + derivation trace · event stream · counters · diagnostics
                └──────────▶ feedback: revise the formalization, reselect or
                             resynthesise the theory, repeat to a fixed point
```

## The thirteen components — built, scheduled, open

**Built** means in [`ein.rs/`](ein.rs/), asserted by `cargo test --workspace`
(**703 tests over 77 targets, 0 failures**, 2026-08-25) and, where a number is
given, recorded in a measurement document under
[`docs/history/m1a_rust/`](docs/history/m1a_rust/README.md) that names the machine state it
was taken under. **Scheduled** means a milestone with phase and stage files.
**Open** means a research note in [`plans/followups/`](plans/followups/README.md)
or [`plans/ideas/`](plans/ideas/README.md) — the author's own, unimplemented,
authoritative on intent.

| component | what it does | built (verified) | scheduled | open |
|---|---|---|---|---|
| **Autoformalization** | Translates natural-language or otherwise informal problem statements into formal Ein representations: entities, relations, facts, constraints, rules and goals. | Nothing automatic. The *target* is written: [`zebra_walkthrough.md`](docs/kernel/inference/zebra_walkthrough.md) pairs every NL condition with its ein fact and every NL deduction with its rule firing; [128 hand-formalized fixtures](examples/README.md). | [M2 P2.2](plans/m2_nl_to_ir/p2.2_formalizer/README.md) — the formalizer, one shot: a local LLM (llama.cpp) under GBNF generated from the EBNF; four passes — ontology, **theory** (activators selected from the stdlib catalogue, rules written only when it lacks the property), instance with `:source` quotes, query — under a written contract whose every prompt is a hashed field of an experiment record; `einaf from-text` (the harness's command — the kernel stays LLM-free). [P2.3](plans/m2_nl_to_ir/p2.3_benchmark/README.md) — the benchmark it is scored on: eight reasoning families, generators with exact ground truth, instances whose *correct* answer is *ambiguous* or *unsat*, frozen splits. M2 was reshaped 2026-08-23 around the research plan [`EinAf.md`](plans/m2_nl_to_ir/EinAf.md); ten phases, **not started** | [F16](plans/followups/f16_autoformalization/ideas.md) autoformalization ≠ translation · [F4 Q38](plans/followups/f4_cross_cutting.md#llm-as-factrelationtyperule-extractor-q38) the LLM as fact / relation / type / rule extractor · [F13](plans/followups/f13_puzzles_beyond_zebra/ideas.md) a BBH harness with four oracle ablations |
| **Theory Library** | Maintains reusable formal theories, relation properties, reasoning patterns and domain-specific knowledge that can be instantiated and composed for particular problems. | [`stdlib/`](stdlib/README.md) — seven `std.*` modules: the relation-algebra signature (`std.algebra`), closed-world bijection and slot inference, elimination, typing, closure, macros. A property fact such as `(transitive R)` *is* a theory instantiated: the engine binds the generic rule to `R`. Embedded in the binary, manifest-checked. **And now tested**: [`tests/stdlib/`](tests/README.md) is 56 programs, one per rule or tight family, each stating what it should and should not derive — `ein test tests/` in 0.04 s. Before them **38 of the 73 rules had never fired** in any corpus run and 20 more were reached by one puzzle alone; the re-measured figure is **0**, and since [S1c.1.5](docs/history/m1c_external_validation/README.md#s1c15--in-the-gate) `cargo test` is what says so — in 0.04 s, and scoped to the suite rather than to the corpus, because a rule that fires only inside `examples/zebra.ein` has no test. **M1d S1d.2.4 grew it to 77 rules**: `total-owed` / `surjective-owed` and the two slots duals say what a state *owes* rather than what is illegal in it, and are activated by the `owe` event rather than by a `fire`. | [M1c P1c.1](docs/history/m1c_external_validation/README.md#p1c1--stdlib-conformance) — **shipped 2026-08-23/24**, all five stages (`:expect`, `ein test`, the corpus, the gate). | [F12](plans/followups/f12_rules_and_relations/) properties as closure conditions `R ⋆ R ⪯ R`; a dictionary of relation *theories* · [F8](plans/followups/f8_FCA_RCA_odis_tptp/ideas.md) is there a finite atlas? the `C(n)` curve · [F4 Q34](plans/followups/f4_cross_cutting.md) `relation-profile` and the 2⁷ table |
| **Theory Selection** | Identifies and retrieves existing theories, rules and reasoning patterns relevant to a given problem and its current formalization. | By hand: `(import std.bijection)` and `(bijective pet-loc)`; the compile unit is *rule × activator*. Two selections for one puzzle — [`zebra.ein`](examples/zebra.ein) via `std.slots`, [`zebra2.ein`](examples/zebra2.ein) via `std.bijection` — reach the same model. | [M2 S2.2.4](plans/m2_nl_to_ir/p2.2_formalizer/s2.2.4_passes.md) — the theory pass: the formalizer asserts `(bijective pet-loc)` from the stdlib catalogue and writes a rule only when no module has the property; [Q9](plans/m2_nl_to_ir/open_questions.md#q9--ontology-provenance), reframed from ontologies to theories. | [F12 `ideas.md`](plans/followups/f12_rules_and_relations/ideas.md) *select* a theory, do not invent properties — the LLM writes rules only as the last fallback · [F7 C](plans/followups/f7_rule_induction.md#sub-track-c--rule-set-sufficiency) rule-set sufficiency · [F16](plans/followups/f16_autoformalization/ideas.md) queens → knights keeps `T_placement + T_board` |
| **Theory Synthesis** | Constructs new relations, constraints, rules and theories when existing knowledge is insufficient, including specialization and composition of existing theories. | — | [M2 S2.2.4](plans/m2_nl_to_ir/p2.2_formalizer/s2.2.4_passes.md) — synthesis as the formalizer's third action, permitted only when the pass names the property the catalogue lacks, and logged as such; two benchmark families (object tracking, temporal) have no stdlib theory on purpose, so the action is exercised ([P2.3](plans/m2_nl_to_ir/p2.3_benchmark/README.md)). [F7 B](plans/followups/f7_rule_induction.md#connection-to-m2)'s critical-path flag, taken up 2026-08-23. | [F7](plans/followups/f7_rule_induction.md) rule induction from relations and facts, companion-rule synthesis · [F5](plans/followups/f5_rules_as_data.md) rules as data — induce `(transitive R)` from `(R a b) (R b c) (R a c)` · [F4 Q37](plans/followups/f4_cross_cutting.md#induction--rules-from-facts-q37) · [F12](plans/followups/f12_rules_and_relations/) predicate invention (every match pattern is an unnamed relation), law discovery in a higher-order relation algebra · [F13](plans/followups/f13_puzzles_beyond_zebra/ideas.md) synthesis to a semantic fixed point |
| **Theory Transformation and Specialization** | Adapts general theories to a particular problem context, derives specialized subtheories, and transforms representations into forms better suited for reasoning. | The mechanical half: macro expansion and imports ([`ein-ir`](ein.rs/crates/ein-ir/)), activator binding of generic rules to relations, and [`.einb`](docs/history/m1a_rust/README.md#p1a8--binary-kb-container) — a loaded, optionally saturated, KB as a file. | — | [F12 `ideas3`/`ideas4`](plans/followups/f12_rules_and_relations/) `Specialize(T, C, O)`: chess → queen placement → independent set → permutation CSP, eleven transformations, partial deduction / supercompilation / theory morphisms as prior art · [F15](plans/followups/f15_math_formulae/ideas.md) theory projection into formulae · [F7 A](plans/followups/f7_rule_induction.md#sub-track-a--generalisation-automation) · [F1](plans/followups/f1_categorical_formulation.md) / [F1b](plans/followups/f1b_logical_formulation.md) the categorical and FOL readings |
| **Symbolic Reasoning Kernel** | Executes formal reasoning over Ein representations, including saturation, deduction, rule application, constraint propagation and fixed-point computation. | [`ein-core`](ein.rs/crates/ein-core/) + [`ein-ir`](ein.rs/crates/ein-ir/) + [`ein-infer`](ein.rs/crates/ein-infer/): a register matcher (O1), semi-naive saturation (O2), NAF at the closure/world boundary (O3), contradiction (O5), provenance (O6). Parse + load `zebra2` **0.67 ms**; `solve zebra2 -e` **29 ms**, 157× the PyPy engine; all four P1a.6 targets met, the tightest with 88 % headroom. | [M1a P1a.9](docs/history/m1a_rust/README.md#p1a9--release) — packaging shipped 2026-08-23 (`ein --version`, a release that cannot ship a red gate, a four-platform matrix awaiting its first tag); the Rust embedding page closes the port. | [F11](plans/followups/f11_deductive_layer_perf.md) beta-memories and WCOJ — measured, declined · [F5](plans/followups/f5_rules_as_data.md#kernel-minimisation--which-inference-features-belong-in-ein-lang-vs-kernel-code) kernel minimisation · [Q-M1e.16](plans/m1e_review_processing/open_questions.md#q-m1e16--the-binding-key-compares-two-register-layouts-as-one) the binding key compares two register layouts as one — a known bug |
| **Hypothesis Search** | Explores alternative assumptions and candidate models through structured backtracking over the hypothesis lattice. | The commitment lattice (O7/O8): layer *k* holds the size-*k* commitment sets, generated by Apriori prefix-join, pruned by lookahead, no-goods and downward closure. `--jobs N` fans a layer out over threads — **same verdict, same models, same counters** on 20 712 corpus cells; **3.17–4.40×** on 8 cores. | [M1d P1d.10](docs/history/m1d_satisfiability/README.md#p1d10--exhaustive-search) — why `zebra2-minus-15` does not finish: 32 models, all found by depth 3, `-e` killed at 30 min. | [F9](plans/followups/f9_e_catalog.md#what-the-catalog-taught) the closed ledger — every branch-count optimisation measured inert · [F4 Q31](plans/followups/f4_cross_cutting.md#llm-as-policy-in-search-tree-q31) the LLM as search policy · [Q-M1d.1](docs/history/m1d_satisfiability/open_questions.md#q-m1d1--may-the-search-stop-before-the-lattice-is-exhausted) may the search stop early |
| **Constraint and Satisfiability Reasoning** | Enforces structural and semantic constraints, detects incompatible assignments, and searches for models satisfying the formalized theory. | Upper bounds with force — `functional` / `injective` as `(false)` rules, domain and range elimination, negative completion. Lower bounds (`total` / `surjective`) in refutation form, **plus, since M1d [S1d.2.4](docs/history/m1d_satisfiability/README.md#s1d24--obligations-in-the-saturator) (2026-08-25), in obligation form**: a rule may assert the reserved verdict atom `(open ?R)`, and a quiescent state reports what it still owes — `zebra2-minus-15` owes 46 at root, the hand census reproduced. The report is real; the **force** is not yet, and no verdict word moved. Models are found by search, not by a decision procedure. | [M1d P1d.2](docs/history/m1d_satisfiability/README.md#p1d2--obligations) obligations `L ≤ #{…} ≤ U` and three fixpoint outcomes — **S1d.2.1–.4 done**, S1d.2.5 (branch on an obligation's alternatives) and S1d.2.6 (the verdict word) next; [P1d.3](docs/history/m1d_satisfiability/README.md#p1d3--model-sets) model sets without enumeration · [M10](plans/m10_external_benchmarks/README.md) the same problems through Z3, CVC5, SWI-Prolog, Soufflé, Clingo and Lean | [`m1d/ideas.md`](docs/history/m1d_satisfiability/ideas.md) what saturation lacks to be satisfiability · [F12 `ideas3`](plans/followups/f12_rules_and_relations/ideas3.md) choice → saturation → conflict → backtrack · [F4 Q40](plans/followups/f4_cross_cutting.md#may-a-performance-lever-decide-what-a-complete-model-is-q40) a performance lever currently decides the verdict on two fixtures — unresolved |
| **Contradiction Detection and Analysis** | Identifies inconsistent states and traces contradictions back to the rules, facts and hypotheses responsible for them. | `(X, ¬X)` / `(false)` detection; an AND/OR provenance DAG; the **unsat core** — the smallest set of given facts from which one recorded contradiction follows (over the derivations the store retained, at most 32 per fact; not a subset-minimal MUS); one learned no-good per dead commitment. Fixture: [`ein-bugs/zebra2-bad.ein`](examples/ein-bugs/zebra2-bad.ein). | [Q-M1d.6](docs/history/m1d_satisfiability/open_questions.md#q-m1d6--may-contradiction-be-said-with-exhausted--false) — ten corpus entries say *Contradiction* at the depth cap with `exhausted = false`; *incomplete* is the word they want. [Q-M1e.15](plans/m1e_review_processing/open_questions.md#q-m1e15--the-alternatives-cap-decides-which-unsat-core-is-reported) — the 32-alternative cap retains by premise count and the search minimises frontier size, so it can hide the smaller explanation ([`ein-bugs/alt-cap-core.ein`](examples/ein-bugs/alt-cap-core.ein)); no shipped puzzle is changed by it. | [F3](plans/followups/f3_three_task_classes_first_class.md) `ein why-not` / `explain` · [F12 `ideas5`](plans/followups/f12_rules_and_relations/ideas5.md) `emit false` as a forbidden pattern, `T⁺ ∩ T⁻ = ∅` · [F9](plans/followups/f9_e_catalog.md) E7 / E19: MUS minimisation is unsound under NAF |
| **Formal Verification** | Mechanically checks neural-generated formalizations, rules, theories and candidate solutions against the symbolic semantics of Ein. | Of a *formalization*: parse / load / compile diagnostics at `file:line:col`, 36 negative fixtures, the thirteen [defined behaviours](docs/kernel/defined_behaviour.md). Of the *engine*: the gate, the id-order and `--jobs` invariance sweeps, a six-property fuzzer. Nothing yet checks a *neural* output, because nothing produces one. | [M2 P2.4](plans/m2_nl_to_ir/p2.4_loop/README.md) — the loop judges every repair against the source ([S2.4.4](plans/m2_nl_to_ir/p2.4_loop/s2.4.4_faithfulness.md): *formal validity is not faithfulness*); the old validator is feedback level F1 of nine ([S2.4.3](plans/m2_nl_to_ir/p2.4_loop/s2.4.3_feedback_ladder.md)); [P2.1](plans/m2_nl_to_ir/p2.1_kernel_as_instrumentation/README.md) gives the kernel's diagnostics a versioned structured form, `ein-feedback/1` · [M1c](docs/history/m1c_external_validation/README.md#the-thesis) — no claim rests only on self-agreement: `:expect` plus external confirmation | [F17](plans/followups/f17_formal_verification/ideas.md) three applications — `NL → T`, `T → proof / model / unsat`, `(T, P) → P ⊨ T` — and Ein as the theory layer between NL / code and a prover; K framework as the closest relative · [F2 PSM.3](plans/followups/f2_self_modifying_language.md) the semantic firewall · [F1b](plans/followups/f1b_logical_formulation.md) the fragment Ein cannot decide |
| **Solution and Model Generation** | Produces satisfying models, solutions, derived facts, proofs or counterexamples depending on the problem. | `ein solve` — `k = 1` the solution (certified by `-e`), `k > 1` the models (`-n N` / `-e`), `k = 0` the unsat core; `--print-final-state`, `--json-summary`; a `SOLUTIONS` store in `.einb` (library API only). | [M1d P1d.3](docs/history/m1d_satisfiability/README.md#p1d3--model-sets) compact model sets · [M10](plans/m10_external_benchmarks/README.md) every answer confirmed by a system that is not Ein | [F13](plans/followups/f13_puzzles_beyond_zebra/ideas.md) benchmark answers read off models · [Q-M1d.1](docs/history/m1d_satisfiability/open_questions.md#q-m1d1--may-the-search-stop-before-the-lattice-is-exhausted) |
| **Reasoning Introspection** | Exposes derivation traces, rule dependencies, relation dependencies, hypothesis branches, unsatisfiable cores and other reasoning artifacts. | `--trace out.md` — a self-contained markdown derivation rendered through the puzzle's own `:why` templates (`--relevant`, `--reorder`); 17 DOT views, four on the CLI; [`--events`](docs/kernel/inference/events.md) JSONL (`ein-events/1`); `--stats`, `--hyp-stats`, `--timing`, `--dump-states`. The human-style trace is the acceptance criterion ([idea 08](plans/ideas/08-human-style-deductive-trace.md)). | [M20](plans/m20_gui/README.md) — a Tauri GUI: code, graph and branches views over one `StateId` · [M2 S2.4.3](plans/m2_nl_to_ir/p2.4_loop/s2.4.3_feedback_ladder.md) — the engine's outputs rendered *for a model*, nine levels: verdict, cardinality, model difference, unsat core, relation dependency (new), provenance chains; trace → NL explanation parked ([Q7](plans/m2_nl_to_ir/open_questions.md#q7--llm-as-surface-generator)) | [F15](plans/followups/f15_math_formulae/ideas.md) rules as formulae, a theory summariser, an algebraic signature of the current theory for the LLM · [F3](plans/followups/f3_three_task_classes_first_class.md) · [F4 Q33](plans/followups/f4_cross_cutting.md) |
| **Neuro-Symbolic Feedback Loop** | Feeds symbolic results — solutions, contradictions, incomplete derivations, failed hypotheses and structural information — back into the neural component to refine the formalization or synthesize / select better theories. | — (every signal it would carry already exists as engine output: the verdict, `k`, the unsat core, the counters, the event stream, the diagnostics). | [M2 P2.4](plans/m2_nl_to_ir/p2.4_loop/README.md) — the loop as an instrument: a state machine with every transition logged, repair as well as regenerate, the feedback ladder F0–F8, faithfulness, termination with cycle detection by syntactic hash and semantic digest; [P2.5](plans/m2_nl_to_ir/p2.5_harness/README.md) — baselines B0–B5 under a matched inference budget, four metric layers, immutable experiment records; [P2.6](plans/m2_nl_to_ir/p2.6_ablations/README.md) — the nine ablations that make the loop an experiment; ambiguity as hypotheses kept as a strategy ([S2.4.5](plans/m2_nl_to_ir/p2.4_loop/s2.4.5_alternatives_as_hypotheses.md)) | [F13](plans/followups/f13_puzzles_beyond_zebra/ideas.md) the loop `(Pᵢ, Tᵢ) → Ein → Dᵢ`, `(text, Pᵢ, Tᵢ, Dᵢ) → LLM → (Pᵢ₊₁, Tᵢ₊₁)` to a *semantic* fixed point; three neural actions — reinterpretation, theory selection, theory synthesis · [F16](plans/followups/f16_autoformalization/ideas.md) the diagnostics an engine can return that a compiler cannot · [F17](plans/followups/f17_formal_verification/ideas.md) `Spec₀ → verify → counterexample → Spec₁` · [idea 01](plans/ideas/01-self-modifying-constraint-language.md) / [F2](plans/followups/f2_self_modifying_language.md) the grammar itself in the loop · [F6](plans/followups/f6_modify_own_harness.md) |

Read the columns top to bottom and the shape of the project is visible: the
kernel rows (symbolic reasoning, hypothesis search, contradiction analysis,
solution generation, introspection) are **built**; the constraint row is
half-built and has a milestone; the theory rows (library, selection) are built
*by hand* and their automation is **scheduled** as the formalizer's theory
pass; the neural rows (autoformalization, feedback loop) are **scheduled**
with nothing started; theory synthesis is scheduled only as the pass's last
resort and measured where the stdlib has no theory; transformation is
**open** only.

## Built and verified — Ein, the symbolic kernel

### One run, one verdict

Ein loads a puzzle as a typed hypergraph of relations, facts and rules,
saturates the rules to a least fixpoint, then searches a commitment lattice.
One run, one verdict — **read from the result**, never chosen by a mode flag.
The count of distinct complete models `k` *is* the answer:

- `k = 1` → **the solution** — a unique complete model (certified unique once
  the search is exhausted).
- `k > 1` → **gaps** — the puzzle is under-determined: `k` distinct models, the
  residual ambiguity.
- `k = 0` → **a contradiction** — an over-constrained KB, reported with its
  unsat core: the smallest set of given facts from which one recorded
  contradiction follows (provenance-based, searched across every derivation
  the store **retained** — at most 32 per fact, shortest-premises-first; not a
  subset-minimal MUS).

`solve` / `gaps` / `contradictions` are **three answers to one problem**
([idea 03](plans/ideas/03-three-task-classes.md)), not three commands. You run
**`ein solve`** and read whichever answer the puzzle yields; the stop policy
(single / `--solutions N` / `--exhaustive`) only controls how far the search
runs. An earlier design had three functions that each *chose* their verdict up
front — and so disagreed with each other on the same input; collapsing them
into one engine is what fixed that.

Every derived fact carries provenance, so a solve can emit a self-contained,
human-readable markdown derivation trace. Reproducing the trace a human would
write — elimination by exclusion, case analysis, reductio — is the acceptance
criterion that separates this project from a wrapper around a solver
([idea 08](plans/ideas/08-human-style-deductive-trace.md)); the target trace is
[`zebra_walkthrough.md`](docs/kernel/inference/zebra_walkthrough.md).

### The running example

The classic Zebra / Einstein puzzle is the fixture, in two encodings that reach
the same model — [`examples/zebra2.ein`](examples/zebra2.ein) (one typed
relation per attribute, `std.bijection`) and
[`examples/zebra.ein`](examples/zebra.ein) (one generic `co-located` relation,
`std.slots`):

```sh
$ ein solve examples/zebra2.ein
solve · examples/zebra2.ein
──────────────────────────────────────────────────────────────
  solutions (k)   1   (not certified — pass --exhaustive)
  verdict         Solution

  query bindings
    ?h_water    = House-1
    ?h_zebra    = House-5
    ?who_water  = Norwegian
    ?who_zebra  = Japanese

    query facts                     rendered
    (drink-loc Water House-1)       Water is drunk in House-1
    (nation-loc Norwegian House-1)  the Norwegian lives in House-1
    (pet-loc Zebra House-5)         the Zebra is kept in House-5
    (nation-loc Japanese House-5)   the Japanese lives in House-5

  result
    The Norwegian drinks water in House-1; the Japanese owns zebra in House-5
```

Every word of that answer comes from the **puzzle**, not the engine: each
`(relation … :why "{?1} … {?2}")` template renders a fact, and the
`(query … :goal-text "…")` template renders the headline from the goal
variables. A relation with no `:why` prints as its IR s-expression — there is
no built-in relation→verb vocabulary. With `-e` the same run certifies `k = 1`
after 101 enterings and 67 learned no-goods, in about 30 ms.

### The kernel's operations, and where each sits in the literature

The engine fuses a deductive database, a CSP/SAT search and an ATMS, split at
the monotone / non-monotone seam
([architecture_and_algorithms.md](docs/kernel/inference/architecture_and_algorithms.md#4-the-core-operations)):

| op | what Ein does | nearest analogs |
|---|---|---|
| O1 multi-way join | bind a rule body against the KB — a register matcher with a backtrack trail | relational join, RETE / LEAPS, worst-case-optimal joins |
| O2 saturation | fire rules to a least fixpoint, semi-naive | Datalog bottom-up, magic sets, differential dataflow |
| O3 negation as failure | `(absent P)` judged at the closure / world boundary ([absent_semantics.md](docs/kernel/inference/absent_semantics.md)) | stratified Datalog, CWA, stable models |
| O4 equality | `EqClasses` — a stub | union-find, congruence closure, e-graphs |
| O5 contradiction | `(X, ¬X)` or `(false)` | clause violation, tableau clash |
| O6 provenance, unsat core | an AND/OR derivation DAG → the smallest source frontier | ATMS / JTMS justifications, provenance semirings, MUS |
| O7 hypothesis enumeration | size-*k* commitment sets over a subset lattice | DPLL decisions, **Apriori**, ATMS environments |
| O8 conflict-driven pruning | no-goods, downward-closure pruning, lookahead kills cached as `(not h)` | CDCL, backjumping, forward checking |
| O9 model canonicalisation | the sorted fact list is the state identity | symmetry breaking, SBDS / SBDD |

The thirteen behaviours that used to be defined by "whatever the Python engine
did" — diagnostics, orderings, error strings — are normative in
[`defined_behaviour.md`](docs/kernel/defined_behaviour.md); the full kernel
specification is [`docs/kernel/`](docs/kernel/README.md).

### The engine in numbers

**M1** (shipped 2026-06-17) built the engine in Python and solved the Zebra
puzzle end-to-end. **M1a** rewrote it in Rust behind a byte-exact parity gate
and then, with the gate watching, made it fast; since
[S1a.10.5](docs/history/m1a_rust/README.md#s1a105--the-removal)
`ein.rs/` is the only implementation. Every number below is on the record,
with its instrument and its machine state, under
[`docs/history/m1a_rust/`](docs/history/m1a_rust/README.md); the CPython / PyPy denominators
are **frozen constants**, because the engine they measured left the tree.

| measurement | value | record |
|---|---:|---|
| `solve zebra2.ein -e` end-to-end, PyPy → ein.rs | 4.53 s → **29.0 ms** (157×); peak RSS 223 MB → 17 MB | [baseline.md § the close](docs/history/m1a_rust/measurements/baseline.md#the-four-targets-at-the-close), [P1a.6 § Targets](docs/history/m1a_rust/README.md#p1a6--performance) |
| `solve zebra.ein -e` | 8.33 s → **47.2 ms** (175×) | same |
| parse + load `zebra2.ein` · the three-fixture acceptance gate | 0.43 s → **0.67 ms** · 36.0 s → **0.127 s** | same |
| the P1a.6 optimisation programme on `zebra -e` | 585.8 → 47.5 ms across the phase; matcher candidates 25 160 149 → 238 567 | [baseline.md](docs/history/m1a_rust/measurements/baseline.md) |
| `--jobs 8` on 8 P-cores, the four workloads that fan out | **3.17–4.40×** (`branching/06 -e` 194 → 44 ms); the ≥ 6× target **not met**, the remaining 1.5× named as allocation, not contention | [scaling.md](docs/history/m1a_rust/measurements/scaling.md#8-t1a721--the-fan-out-and-the-three-things-it-costs), [P1a.7](docs/history/m1a_rust/README.md#p1a7--parallelism) |
| `--jobs N` is the same computation | 20 712 (file, op, jobs) cells, **0 moved**, 30 s; the verbose event stream byte-identical at both job counts, 2 200 561 lines included; 10 000 paired fuzz runs, zero findings | [P1a.7 § acceptance](docs/history/m1a_rust/README.md#p1a7--parallelism) |
| memory under parallelism | per-worker provenance: `features/01 -e` 684–708 MB → **85–91 MB**; peak RSS 79.8 / 90.3 MB at `--jobs 1 / 16` | [design/README § Measured](docs/history/m1a_rust/design/README.md#measured) |
| `.einb`, the binary KB container | a saturated `zebra2` is 57 688 bytes and opens cold in **0.614 ms**; `solve x.einb` byte-identical to `solve x.ein`; 20 000 fuzzed inputs and 3 348 bit-flips rejected by the digest | [P1a.8](docs/history/m1a_rust/README.md#p1a8--binary-kb-container) |
| the gate | 312 tests in 9 m 13 s with a Python process in 42 of them → **619 tests, 0 failures, 1 m 51 s**, no Python | [oracle ledger](docs/history/m1a_rust/oracle_ledger.md), [`run_tests.sh`](run_tests.sh) |
| what replaced the two-engine oracle | 4 228 renderings banked as digests, 13 counter identities over every solve cell, an id-space permutation sweep: **0 answers moved**, 66 renderings (all narration); four accepted losses, named | [oracle ledger § 5–6](docs/history/m1a_rust/oracle_ledger.md#5-what-the-successor-found) |
| the corpus | 128 entries in six groups, 629 cells as processes in 5.3 s; the slow tier 641 cells in **19.4 s** where it was 660 in 242.6 s; **two** entries are slow and `cost_ms` says so | [corpus_cost.md](docs/history/m1a_rust/measurements/corpus_cost.md#7-the-first-re-take--2026-08-22-and-it-moved-an-entry), [`corpus/`](corpus/README.md#slow) |
| determinism | every order-sensitive site reproduces ein.py's; `--shuffle` is MT19937 exactly; a permuted id space moves 51 of 3 160 renderings and 0 answers | [design/02](docs/history/m1a_rust/design/02_determinism_and_order.md), [ledger](docs/history/m1a_rust/oracle_ledger.md#5-what-the-successor-found) |

Two of the phase histories are worth reading as method rather than as
results. P1a.6 closed on a lever matrix with a **control row** pricing each
column and a differential fuzzer that found four parity bugs in its first
twenty minutes. P1a.7 **declined six things by a number** — a validator, two
locks, `loom`, two of its four parallel levels and `--unordered` — and recorded
that a parallel run is an instrument that finds sequential waste
([P1a.7](docs/history/m1a_rust/README.md#p1a7--parallelism)); [F9](plans/followups/f9_e_catalog.md)
is the same discipline applied to the search layer, where nine optimisations
were measured inert against a complete cardinality-BFS and rejected.

### The theory library that exists today

[`stdlib/`](stdlib/README.md) is the single source of truth for `std.*`,
embedded in the binary and identified by `MANIFEST.sha256`. It is the part of
the *Theory Library* component that is real, and its activator convention —
state a property as a fact, the engine instantiates the rules — is the
mechanism the open notes build on ("the LLM asserts `(transitive left-of)`,
Ein expands it").

| module | provides |
|---|---|
| `std.algebra` | the relation-algebra signature: `converse` / `compose` / `identity`, `meet` / `join` / `difference` / `complement`, the cardinality properties `functional` / `injective` / `total` / `surjective` and the `bijective` fan-out, `symmetric` / `transitive` / `includes` closures, `imply*`, the Schröder lemmas |
| `std.bijection` | closed-world bijection inference, signature-driven: negative completion, domain / range elimination, typecheck — `zebra2.ein`'s formulation, generalised |
| `std.slots` | the same inference for one generic co-location relation whose classes are slots — `zebra.ein`'s formulation, generalised |
| `std.elim` | closed-world `domain-elimination`, `no-room-left`, positional typecheck |
| `std.closure` | `functional ∧ total ⇒ (__closed__ R)` — opt-in |
| `std.typing` | the `type-hierarchy` converse-typecheck driver, `reflexive` |
| `std.macro` | the `forall` / `unknown` pattern macros |

### Known gaps, measured

The milestones in [§ Scheduled](#scheduled--the-roadmap) exist because these
were measured, not assumed:

- **An under-determined puzzle does not finish exhaustively.**
  `zebra2-minus-15.ein` has 32 models; all 32 are found by depth 3 in 25 s,
  and depths 4–5 exist only to prove there are no more — `-e` was killed at
  30 min. The engine enumerates a powerset because it cannot say that
  something is *required* ([M1d](docs/history/m1d_satisfiability/README.md)).
- **Lower bounds are reported but still have no force.** `total` /
  `surjective` were implemented only as refutations ("every candidate is
  excluded, so this state is dead"), never as obligations ("one candidate is
  still owed"), and the `≥ 2` middle — where a search actually happens —
  recorded nothing ([M1d § what the note says](docs/history/m1d_satisfiability/README.md)).
  **M1d [S1d.2.4](docs/history/m1d_satisfiability/README.md#s1d24--obligations-in-the-saturator)
  closed the recording half** (2026-08-25): a rule may assert the verdict atom
  `(open ?R)`, four such rules ship in the stdlib, and a quiescent state
  reports what it still owes through `--events`, `--json-summary` and the
  trace — `zebra2-minus-15` owes **46** at root, the number
  `obligation_forms.md` §5 had counted by hand. What has *not* moved is the
  force: no verdict word changed, and the search still branches on subsets of
  `alive` rather than on an obligation's alternatives. S1d.2.5 is the branch,
  S1d.2.6 the word.
- **Ten corpus entries report *Contradiction* when they mean *incomplete*** —
  `exhausted = false` at the depth cap ([Q-M1d.6](docs/history/m1d_satisfiability/open_questions.md#q-m1d6--may-contradiction-be-said-with-exhausted--false)).
- **A performance lever decides a verdict, and *both* settings under-report.**
  With `enable_pre_branch_lookahead` off, `branching/06` and `lattice/02`
  change from *Solution* / *Ambiguity* to *Contradiction*
  ([F4 Q40](plans/followups/f4_cross_cutting.md#may-a-performance-lever-decide-what-a-complete-model-is-q40)).
  This was recorded as *one of the two configurations is wrong*; since M1e
  2026-08-28 the sharper statement has a test.
  [`branching/15`+`16`](examples/branching/15_lookahead_two_step_on.ein) are
  three hypotheses, both halves exhausting, and a solution set derivable in a
  paragraph: the hand-derived answer is **k = 2**, the lookahead-on half says
  `Solution k=1` and the off half `Contradiction k=0`. **Neither side is the
  definition** — `complete()` is a sound but incomplete approximation of
  maximality with the lookahead on and a strictly weaker one with it off, so
  the lever chooses between two wrong numbers. The fix is to record a
  surviving commitment whose every superset died
  ([Q-M1e.8](plans/m1e_review_processing/open_questions.md#q-m1e8--exhausted-certifies-the-lattice-not-the-model-set)),
  and the pair's `:expect`s state today's answers so that taking it moves a
  golden.
- **The binding key compares two register layouts as one**, so a program that
  puts an integer and a nested fact in one activator position loses a
  derivation silently — and a debug build asserts where a release build
  answers
  ([Q-M1e.16](plans/m1e_review_processing/open_questions.md#q-m1e16--the-binding-key-compares-two-register-layouts-as-one),
  which is `Q-M1a.8` after its probe: the *integer* trigger that entry named
  does not exist).
- **`--jobs` is 1.5× short of its target**, and the shortfall is named
  ([scaling.md](docs/history/m1a_rust/measurements/scaling.md#where-the-other-15-is)).
- **Every check is still relative** — to the engine's own past — until
  [M1c](docs/history/m1c_external_validation/README.md#the-thesis) and
  [M10](plans/m10_external_benchmarks/README.md) land an independent one. The precedent is specific: a stdlib guard was wrong for a
  year through five phases of byte-exact parity, and what found it was an
  independent enumeration written outside the engine.

### What the kernel is not — three C baselines

[`c/`](c/README.md) holds three plain-C programs solving the puzzle
`examples/zebra.ein` encodes, with exactly one thing varying: **how much the
search is told about the constraints**.

| | what the search knows | assignments | wall |
|---|---|---:|---:|
| `zebra_levels.c` | every clue, and the level at which each becomes testable | **6 840** | 0.003 s |
| `zebra_oracles.c` | fourteen opaque yes/no functions, in the puzzle's order | 25 092 302 520 | 158 s |
| `blackbox.c` + `zebra_module.c` | a grid size and one function pointer | 25 092 302 520 | 388 s |

**3 668 465×**, and the difference is not an algorithm — it is one integer per
clue. They are what `ein solve` is *not*: no propagation, no domains, nothing
learned from a dead subtree, every condition compiled code that answers this
one puzzle ([§ What none of them do](c/README.md#what-none-of-them-do)). They
are also the first rung of the story this repository tells —
*enumeration with hard-coded predicates → link-grammar + SMT → an IR as the
reasoning substrate → Ein → rule sets as theories → neural-guided theory
synthesis* ([M5](plans/m5_presentation/README.md)).

## Quickstart

**New to Ein?** Start with the **[tutorial](docs/guide/)** — *Learn Ein by
solving the Zebra puzzle*, four chapters from objects and relations to the
full solve — then come back here.

### Install

```sh
./build.sh                          # the Rust workspace (release) + the three C baselines
ein.rs/target/release/ein --version
```

**[`docs/install.md`](docs/install.md) is the install page** — the two
channels (a release binary, `cargo install --path`), how to verify a binary
against the stdlib manifest, and what `EIN_STDLIB` does. In brief: `build.sh`
needs a Rust toolchain (pinned by `ein.rs/rust-toolchain.toml`), a C compiler,
and — for the default build — `cmake` and a C++ compiler, because `ein` links
`snmalloc`. `./build.sh --no-snmalloc` (or `cargo build --release -p ein-cli
--no-default-features`) needs neither and costs a measured **+25 % on `solve
zebra2.ein -e`**, along with `ein kb` and `--jobs`
([feature_cost.md](docs/history/m1a_rust/measurements/feature_cost.md)).

`ein --version` names the build: engine version, `--events` protocol, the
features compiled in, and the SHA-256 of the `std.*` manifest **this binary
will load** — the one input that can differ between a binary and the checkout
beside it. Release binaries for Linux (x86_64 + aarch64), macOS (universal2)
and Windows are [S1a.9.3](docs/history/m1a_rust/README.md#s1a93--packaging-and-release)'s
matrix, written and reviewed; the first tag is what runs it. **`pip install`
is not a channel** — the Python binding was deferred on 2026-08-21 for want of
a consumer ([Q-M1a.23](docs/history/m1a_rust/open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding)).

> **There was a Python implementation** — `ein.py/` — and it was the reference
> for five phases of the port. It left the tree at M1a
> [S1a.10.5](docs/history/m1a_rust/README.md#s1a105--the-removal),
> commit `4c1a5b3`; its parent is the last revision that had both engines.

### Solve

```sh
ein solve <file>                   # print the solution (or the unsat core)
ein solve <file> --exhaustive      # certify unique / ambiguous / unsat
ein solve <file> --solutions N     # stop after N distinct solutions
ein solve <file> --jobs 8          # fan each lattice layer out over 8 threads (or `auto`)
ein solve <file> --stats           # + engine counters (k, enterings, layers, no-goods, wall)
ein solve <file> --trace out.md    # + a self-contained markdown derivation trace
ein solve <file> --events e.jsonl  # + the narration stream, one JSON object per step
ein solve <file> --json-summary s.json
```

The verdict is read from the result (`k = 0 / 1 / >1`); the only choice is the
**stop policy** — single (default) / `--solutions N` / `--exhaustive` — plus the
budgets `--max-set-size N` (commitment-set depth, default 5), `--max-time`,
`--max-enterings`. `--jobs N` changes the wall clock and nothing else: verdict,
models and every counter are identical by construction, and a puzzle file
cannot set a thread count. Inspection: `--print-final-state` (the model, or the
unsat-core facts), `--dump-states DIR`, `--hyp-stats`, `--timing`; the trace
shapers `--relevant` (goal-relevant slice) / `--reorder` (cluster by target
entity) / `--no-diagrams` apply to the `--trace` file. `-L` / `-K` switch the
lookahead and the kill cache off for A/B runs; `--shuffle` / `--seed` permute
the candidate order reproducibly.

### Saturate, render, save

```sh
ein saturate <file> [--dump]                       # the least fixpoint alone, with phase timings
ein render rules|rule|constraints|lattice <file>   # DOT views (lattice runs a solve)
ein kb save [--saturate] <file> out.einb           # the binary KB container
```

`render` emits Graphviz to stdout; rasterising is a shell concern
([`utils/render_examples.sh`](utils/render_examples.sh)). The remaining DOT
views — the IR graph in five variants, the whole KB on one page in six, the
full and the sliced lattice — exist as library calls in
[`ein-render`](ein.rs/crates/ein-render/) and are tested over the corpus, but
have no CLI. **`.einb` is a private cache
format, never an interchange one**: every command that takes a `.ein` path
takes a `.einb` too, dispatching on the magic bytes, and `ein solve x.einb` is
byte-identical to `ein solve x.ein` apart from the path it echoes. Anything
crossing a tool boundary is `.ein` text or the event protocol's JSON.

### ein-lang at a glance

A `.ein` file is a **flat** sequence of S-expression forms; each is classified
by its head:

| head | role |
|---|---|
| `relation` | declare a typed relation + signature, optionally a `:why` rendering template |
| `rule` / `hrule` | inference / hypothesis rule (`:match` → `:assert`, with `:why`) |
| `query` | what to ask — `:goal` variables and a `:goal-text` template |
| `config` | engine knobs |
| `import` / `macro` | module include (`std.*` or a path) / pattern-macro sugar |
| `trace` | engine-emitted derivation log (parsed back for rendering) |
| *anything else* | a **fact** — `(is-a …)`, `(right-of …)`, `(bijective pet-loc)` — layered ontology / fact / reasoning by its provenance (`:source` → fact, `:rule` / `:using` → reasoning, else ontology) |

Kernel meta-primitives (`=`, `instance`, `not`, `and`, `or`, `neq`) are
shape-pinned reserved words: wrong arity is a parse error. The complete grammar
is [`00_ebnf.md`](docs/kernel/ir/03-ein-lang/00_ebnf.md); the pattern
sub-language, the reserved names and the stdlib API are the rest of
[`ir/03-ein-lang/`](docs/kernel/ir/03-ein-lang/).

### Development loop

```sh
./run_tests.sh             # the gate: cargo test --workspace — 619 tests, ~1 m 51 s
./run_tests.sh --slow      # + the two slow corpus entries, + 8 id-space seeds (the nightly tier)
cd ein.rs && cargo fmt && cargo clippy --workspace --all-targets -- -D warnings
EIN_BLESS=1 ./run_tests.sh # re-bank the goldens, deliberately
```

The gate needs **Graphviz** on `PATH`: `dot_wellformed.rs` is the only
authority the DOT views have on being well-formed, and it fails rather than
skips without it. CI runs the gate per commit and the slow tier, eight id-space
seeds and a two-million-mutation frontend fuzz nightly
([`.github/workflows/`](.github/workflows/)).

## Scheduled — the roadmap

The full roadmap, with its four-level schema (milestone → phase → stage →
task) and its open-question indexes, is [`plans/`](plans/README.md). Status as
of 2026-08-23:

| milestone | what | status |
|---|---|---|
| **M1** — core graph reasoning | the IR, the KB, saturation, contradiction, the hypothesis loop, the trace, the Zebra solve | **shipped 2026-06-17**; its plan folder is in git history |
| [**M1a**](docs/history/m1a_rust/README.md) — the Rust port | a 1:1 observable surface outside, a free hand inside; parity first, speed second, scale third | **shipped 2026-08-23** — all eleven phases closed, `ein.rs` the only implementation. The plan folder was deleted with it; the record is [`docs/history/m1a_rust/`](docs/history/m1a_rust/README.md) — the eleven phases and 53 stages as one document, plus the design contracts, the measurements, the divergence ledger and twenty-three questions |
| [**M1c**](docs/history/m1c_external_validation/README.md) — external validation | the check that is not relative to Ein's own past: `:expect` on `query` + `ein test` for every stdlib rule | **shipped 2026-08-24** — one phase, five stages, and **38 of 73 never-firing stdlib rules → 0**, held by `cargo test`. The plan folder was deleted with it; the record is [`docs/history/m1c_external_validation/`](docs/history/m1c_external_validation/README.md) — the phase and its stages as one document, plus the census and seven questions. Its benchmark phase left for M10 on 2026-08-23 |
| [**M1d**](docs/history/m1d_satisfiability/README.md) — from saturation to satisfiability | why an under-determined puzzle does not finish; existence requirements as first-class **obligations**; model sets without enumeration | queued behind M1a; 3 phases, 14 stages, P1d.10 at stage depth |
| [**M2**](plans/m2_nl_to_ir/README.md) — EinAf: iterative autoformalization, through Level D | the research plan [`EinAf.md`](plans/m2_nl_to_ir/EinAf.md) as a schedule: the kernel as instrumentation (`ein-feedback/1`), the one-shot formalizer, a heterogeneous benchmark with ambiguous and unsatisfiable instances, the loop with nine feedback levels, baselines B0–B5 at matched budget, ablations G1–G9, failure taxonomy, scaling and generalization, representations compared, the formal account, the result and the demo — Level B at ~8 weeks, C at ~19, D with M5 | **next**; **reshaped 2026-08-23** — 10 phases, P2.1–P2.5 at stage depth (21 files), **nothing started**; the old six-phase *NL → IR* plan is Level B's half, re-targeted to the crates |
| **M2+** — ontology and rule induction | induce the theory a puzzle's text *assumes* ([F4](plans/followups/f4_cross_cutting.md) / [F7](plans/followups/f7_rule_induction.md)) | beyond M2; [F7 B](plans/followups/f7_rule_induction.md#connection-to-m2) says it is on M2's critical path |
| [**M5**](plans/m5_presentation/README.md) — paper + presentation | *From the Zebra puzzle to autoformalization*: comparison against the field, benchmarks, the growth directions as future work | parked; placeholder with the outline (was M2b) |
| [**M10**](plans/m10_external_benchmarks/README.md) — external benchmarks | the same problems stated for Z3, CVC5, SWI-Prolog, Soufflé, Clingo and Lean, run by one harness, compared on the *answer* first and the clock second; every answer checked back in as an `:expect` | queued behind M1a; 5 stages, promoted from M1c's P1c.2 on 2026-08-23 |
| [**M20**](plans/m20_gui/README.md) — GUI | Tauri 2 + React + Monaco + Cytoscape, linking the crates in-process; three views (code, graph, branches) joined by one `StateId`; the Rust `Session` owns the semantics | parked; stack decided 2026-08-18 (was M1b) |
| ~~M3~~ — SMT integration | a `(hard-slice …)` hand-off to Z3 / CVC5 | **dropped 2026-08-18** — the idea, not just the schedule; Ein stays graph-native with no solver back-end ([Q2](plans/open_questions.md#q2--when-does-the-graph-engine-hand-off)); comparing against solvers is M1c's, integrating one is nobody's |

What the neural side of the loop concretely waits on, in M2's order:
**P2.1** (the kernel as instrumentation — a versioned feedback object with a
diagnostic vocabulary, `unknown` distinct from *contradiction*, the boundary
language decided), **P2.2** (the formalizer, one shot: `llama-server` under a
GBNF generated from the EBNF, four passes, the theory *selected* from the
stdlib catalogue, `einaf from-text` with every run recorded), **P2.3** (the
benchmark: eight families, generators with exact ground truth and a canonical
theory, unique / ambiguous / unsat instances, frozen splits — the Level B
gate), **P2.4** (the loop: a logged state machine, repair as well as
regenerate, the feedback ladder F0–F8, faithfulness judged on every repair,
termination), **P2.5** (immutable experiment records, baselines B0–B5 at
matched budget, four metric layers, the first main table), **P2.6** (the nine
ablations — the link-grammar A/B among them), **P2.7** (failure taxonomy,
scaling, generalization — the Level C gate), **P2.8–P2.10** (representations
compared, the formal account, the result and the demo — Level D, with M5's
paper). Its open questions — Q7–Q11, Q23–Q25 and the four `Q-M2.<n>` that
arrived with the reshape — each name the stage that decides them
([`open_questions.md`](plans/m2_nl_to_ir/open_questions.md)).

## Open — investigation directions

[`plans/followups/`](plans/followups/README.md) parks the research threads —
neither MVP-blocking nor scheduled — and [`plans/ideas/`](plans/ideas/README.md)
holds the author's ten founding notes. The newest five followups (F12, F13,
F15, F16, F17) are raw notes in the author's own voice, partly in Russian, not
yet worked into one-page themes; they are also where the EinAf framing comes
from. Grouped by what they are about:

**The theory of rules and relations** — the substance behind the *theory*
components.

- [F12 — rules and relations](plans/followups/f12_rules_and_relations/) (five
  notes). A rule is not opaque: its match pattern is a **relation-valued
  operator** (a conjunctive query over the relations it mentions) and its
  consequent an **ordered constraint** on that operator's image — `⊆ T⁺`,
  `⊆ T⁻`, or `= ∅`. Relation properties are **closure conditions**
  `R ⋆ R ⪯ R`, points in a composition × order space, so the stdlib is a
  library of closure-rule templates and NL → Ein is theory *selection*. Rule
  sets are relational programs that can be analysed as data — factorised
  (`co-located = loc ∘ locᵀ`), mined for hidden relations, given variance
  signatures — and theories are objects of a transformation
  `Specialize(T, C, O)` that compresses chess into `IndependentSet` and then
  into a permutation CSP. Prior art named: partial deduction, supercompilation,
  theory morphisms and institutions, ontology modularisation, theory
  exploration.
- [F15 — math formulae](plans/followups/f15_math_formulae/ideas.md): export
  rules as formulae by *semantic lowering*, not `rule → LaTeX` — a faithful
  form and an abstract one (`R ∘ R ⊆ R`, then "R is an equivalence"), a theory
  summariser, and a compact algebraic signature of the current theory to show
  an LLM instead of hundreds of lines. Open: the minimal mathematical IR
  between ein rules and LaTeX.
- [F8 — FCA / RCA, TPTP](plans/followups/f8_FCA_RCA_odis_tptp/ideas.md): where
  Ein sits (Datalog < predicate-polymorphic rules < second-order logic); a
  finite atlas of relation types as a concept lattice with its
  Duquenne–Guigues implication basis; and **the strongest testable hypothesis
  in the set** — the curve `C(n)` of distinct generic rule schemas after *n*
  problems, over logic-grid puzzles, then Allen / RCC8, then TPTP: does it
  plateau?
- [F7 — rule taxonomy and induction](plans/followups/f7_rule_induction.md):
  four classes by what a rule parameterises; generalisation automation,
  induction of `(transitive R)` from facts, instance → type properties,
  rule-set sufficiency, companion-rule synthesis (`functional` → its mutex).
- [F5 — rules as data](plans/followups/f5_rules_as_data.md) (rung 2 of
  self-modification): rules that rewrite rules, with provenance and a
  termination story; and **kernel minimisation** — which inference features
  belong in ein-lang rather than in Rust.
- [F1](plans/followups/f1_categorical_formulation.md) /
  [F1b](plans/followups/f1b_logical_formulation.md): the categorical reading
  (triangle rule = composition; is the fixpoint a colimit?) and the logical
  one — which FOL fragment Ein decides, and where it stops.
- [F4 — cross-cutting](plans/followups/f4_cross_cutting.md): the 2⁷ table of
  algebraic property profiles and a `relation-profile` linter (Q34), relation
  inheritance (Q36), induction from facts (Q37), the LLM as extractor (Q38),
  the LLM as search policy (Q31), and the Q40 finding above.

**Autoformalization, verification and the loop.**

- [F16 — autoformalization](plans/followups/f16_autoformalization/ideas.md):
  the term, its frontier (Wu et al. 2022, process-driven autoformalization,
  Lean Workbook, miniF2F, the AAAI 2026 common framework), and why for Ein it
  is `NL → semantic model → theory selection / specialization → theory +
  instance → saturation / search → feedback`, with the reasoning engine as
  part of an **iterative semantic compiler** whose diagnostics — *theory
  inconsistent, goal under-constrained, multiple models remain, rule r cannot
  fire, required property absent, candidate model is a counterexample* — are
  richer than a type checker's.
- [F17 — formal verification](plans/followups/f17_formal_verification/ideas.md):
  three separate applications — autoformalization `NL → T`, automated
  reasoning `T → proof / model / unsat`, verification `(T, P) → P ⊨ T` — the
  `Spec ≠ Intent` problem and the `Spec₀ → verify → counterexample → Spec₁`
  loop; Certora, Dafny, CBMC / Frama-C, Verus / Kani / Creusot, TLA+ / Apalache
  surveyed; **K framework** and rewriting logic placed as Ein's closest
  relatives (*rule → relation, execution → composition, multi-step →
  closure*). The note's proposed niche — Ein as a theory discovery /
  transformation layer that hands proof obligations to a specialised prover —
  sits in tension with the standing no-back-end decision
  ([Q2](plans/open_questions.md#q2--when-does-the-graph-engine-hand-off)), and
  that tension is unresolved.
- [F13 — puzzles beyond Zebra](plans/followups/f13_puzzles_beyond_zebra/ideas.md):
  the benchmark ladder (BBH `logical_deduction` → `tracking_shuffled_objects` →
  CLUTRR, FOLIO, ProofWriter → logic-grid and Knights & Knaves → ARC), what
  each stresses that Zebra does not (state transitions, interval arithmetic,
  quantifiers, rule *induction*), an end-to-end BBH harness with four oracle
  ablations (NL vs IR × known vs unknown theory), and the loop's definition
  of a fixed point — semantic stabilisation, `Cl(Tᵢ₊₁, Pᵢ₊₁) = Cl(Tᵢ, Pᵢ)`.
  [Idea 09](plans/ideas/09-puzzles-beyond-zebra.md) is the human-puzzle
  companion; M10 is the *other* benchmark direction, formal-language
  shaped and not M2-gated.
- [Idea 01](plans/ideas/01-self-modifying-constraint-language.md) /
  [F2](plans/followups/f2_self_modifying_language.md) /
  [Idea 10](plans/ideas/10-generic-self-modification.md): the grammar itself in
  the loop — the LLM emits content plus grammar updates, a harness applies
  them and recompiles the GBNF — and the three rungs (grammar, rules, harness:
  F2, F5, F6) with the semantic firewall as the central open question; M2
  ships static grammars, which is rung 0. [F6](plans/followups/f6_modify_own_harness.md)
  is deferred indefinitely and says why.

**Closed ledgers — read before proposing.** [F9](plans/followups/f9_e_catalog.md)
(28 search-layer entries, all settled; the lesson: *every branch-count
optimisation failed, the one cost optimisation worked*, because the search is
a complete cardinality-BFS, not a DPLL tree),
[F10](plans/followups/f10_m1_refactor_tail/README.md) (the M1 refactor tail,
kept for its 40-finding review register) and
[F11](plans/followups/f11_deductive_layer_perf.md) (beta-memories and WCOJ,
both measured against the port and declined, with the workload that would
reopen them named).

**Open-question indexes**: [`plans/open_questions.md`](plans/open_questions.md)
(cross-milestone; Q26 compound node kinds and Q28 `()` semantics still open),
[`m1a_rust/open_questions.md`](docs/history/m1a_rust/open_questions.md) (Q-M1a.6, .8,
.16, .22 open; .23 deferred with trip-wires),
[`m1c/…`](docs/history/m1c_external_validation/open_questions.md),
[`m1d/…`](docs/history/m1d_satisfiability/open_questions.md) (Q-M1d.1–6),
[`m2/…`](plans/m2_nl_to_ir/open_questions.md).

## Layout

| path | what's in it |
|---|---|
| [`ein.rs/`](ein.rs/) | **the implementation** — a Cargo workspace of eight crates, `#![forbid(unsafe_code)]` in all but one |
| `ein.rs/crates/ein-core/` | interning, `Value` / `FactId` as integers, the layered copy-on-write KB and its indexes, provenance |
| `ein.rs/crates/ein-ir/` | ein-lang: lexer, parser, typed AST, canonical dump, macros, imports, the embedded stdlib |
| `ein.rs/crates/ein-infer/` | the engine: compile → match → saturate → the NAF boundary → the hypothesis loop; no-goods, contradiction, verdict |
| `ein.rs/crates/ein-einb/` | the `.einb` binary KB container — the one crate whose `cast.rs` is permitted `unsafe`, audited |
| `ein.rs/crates/ein-render/` | the DOT views, the markdown trace, the state / lattice dumps, the JSON summary |
| `ein.rs/crates/ein-cli/` | the `ein` binary — `solve` · `saturate` · `render` · `kb` |
| `ein.rs/crates/{ein-corpus,ein-parity}/` | dev-only: the corpus manifest, fixture helpers and the bench set; the one definition of narration-vs-content |
| [`stdlib/`](stdlib/README.md) | the ein-lang standard library, seven `std.*` modules; `MANIFEST.sha256` keeps the embedded copy honest |
| [`corpus/`](corpus/README.md) | `corpus.toml` — one entry per `.ein` with the runs it is exercised under and a measured `cost_ms`; `fuzz_findings/` |
| [`examples/`](examples/README.md) | `zebra.ein` / `zebra2.ein` (the puzzle, two encodings), `zebra2-minus-15.ein` (32 models), `zebra2-hints.ein`; per-feature fixtures in `features/`, `branching/`, `saturation/`, `lattice/`, `domain_elim/`, `syntax/`; `ein-bugs/` regressions; `broken/` parse / load / compile negatives with expected messages |
| [`c/`](c/README.md) | three plain-C Zebra baselines — what the kernel is not |
| [`build.sh`](build.sh) · [`run_tests.sh`](run_tests.sh) | everything that builds, in one command · the gate |
| [`docs/guide/`](docs/guide/) | **start here** — *Learn Ein by solving the Zebra puzzle* |
| [`docs/kernel/`](docs/kernel/README.md) | the kernel specification: graph semantics, data model, ein-lang, the inference engine, defined behaviour, the event protocol, the Zebra walkthrough |
| [`docs/api/`](docs/api/ein.md) | the Python embedding contract — **a record held in reserve**; nothing implements it and nothing is scheduled to |
| [`docs/history/`](docs/history/README.md) | shipped milestones kept as record — [M1a, the Rust port](docs/history/m1a_rust/README.md): eleven phases, the design contracts the crates cite, the measurements nothing can re-take; [M1c, external validation](docs/history/m1c_external_validation/README.md): one phase, and a stdlib census that *can* be re-taken |
| [`docs/lib/`](docs/lib/README.md) | the catalogue of external tech across 12 topic files — constrained generation, solvers, provers, category theory, graph rewriting, neuro-symbolic stacks, benchmarks — plus a knowledge graph |
| [`plans/`](plans/README.md) | the roadmap: milestones M1d, M2, M5, M10, M20, plus `followups/`, `ideas/`, open questions — M1a and M1c shipped and moved to `docs/history/` |
| [`plans/ideas/`](plans/ideas/README.md) | the author's ten founding notes — authoritative on intent |
| [`plans/followups/`](plans/followups/README.md) | F1–F17 — parked research threads, closed ledgers, the raw notes behind EinAf |
| [`utils/`](utils/README.md) | eighteen scripts driving `ein.rs`: renderers, checks (the manifest, the six-property fuzzer), the measurement set (`bench_env.sh` first) |
| `nlp/`, `smt/` | scratch from the 2021 prototype — dependency-parsing scripts and three hand-written `.smt` encodings; each README names the submodule that restores its tool |
| [`AGENTS.md`](AGENTS.md) | orientation for AI coding agents (`CLAUDE.md` is a symlink to it) |

## Knowledge graph

The topic files under `docs/lib/` are summarised as one graph in
[`docs/lib/knowledge-graph.dot`](docs/lib/knowledge-graph.dot). Two views:

```sh
utils/render_knowledge_graph.sh svg all     # static SVGs (dot / fdp / sfdp / osage) — needs graphviz
python3 utils/render_knowledge_graph_cy.py  # interactive Cytoscape.js page → docs/lib/knowledge-graph.cy/index.html
```

## History

The design is from 2020–2021 — a typed constraint graph with triangle / square
inference and multilevel hypothesis branching
([idea 05](plans/ideas/05-zebra-puzzle-graph-reasoner.md)), and a link-grammar
+ SMT front end that `nlp/` and `smt/` still hold. The modernisation started
in May 2026 with the founding notes and the external-tech catalogue; **M1**
shipped the engine on 2026-06-17; **M1a** began the Rust port on 2026-08-17,
reached byte parity on 2026-08-18, met its speed targets on 2026-08-19,
retired the Python engine on 2026-08-21 and closed its parallelism phase on
2026-08-23. M3 was dropped on 2026-08-18, M1c and M1d were created on
2026-08-21 out of M1a's last two phases and the user's saturation note, and
on 2026-08-23 the project took the name **EinAf**, as the framing above became
the plan. The same day the milestone ids went sparse — the GUI became **M20**,
the write-up **M5**, and M1c's benchmark phase was promoted to **M10** — so
that a milestone can be inserted without renaming its neighbours. **M1c** then
ran in two days, 2026-08-23 → 2026-08-24, and put the standard library under
test for the first time. MIT licensed.
