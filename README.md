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
  then **describes the solution region** R — every commitment of hypothesised
  facts whose state is consistent and owes nothing, computed over the alive
  down-set or the cube with ATMS-style provenance — and answers the question
  asked of it: all of R, a line, *N* states, or whether it is one state. It is **shipped, in Rust, and measured** —
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
(**804 tests over 90 targets, 0 failures** — `./run_tests.sh`, 2026-08-31) and,
where a number is given, recorded in a measurement document under
[`docs/history/m1a_rust/`](docs/history/m1a_rust/README.md) that names the machine state it
was taken under.

> **Every exact number in this tree carries its warrant** — M1e
> [Q-M1e.4](docs/history/m1e_review_processing/open_questions.md#q-m1e4--does-the-repo-want-an-exact-count-in-prose-at-all),
> after a review found that *every* count no test pinned had drifted at least
> one milestone while *every* count a test pinned was exact. A number is
> followed by the **command** that re-takes it, or by the **date** it was
> taken, or by the **test** that pins it; a number that can carry none of the
> three is not written down, and what prints it is named instead. So the line
> above is dated rather than checked: it is right for the 31st and a reader
> who needs today's runs `./run_tests.sh`. **Scheduled** means a milestone with phase and stage files.
**Open** means a research note in [`plans/followups/`](plans/followups/README.md)
or [`plans/ideas/`](plans/ideas/README.md) — the author's own, unimplemented,
authoritative on intent.

| component | what it does | built (verified) | scheduled | open |
|---|---|---|---|---|
| **Autoformalization** | Translates natural-language or otherwise informal problem statements into formal Ein representations: entities, relations, facts, constraints, rules and goals. | Nothing automatic. The *target* is written: [`zebra_walkthrough.md`](docs/kernel/inference/zebra_walkthrough.md) pairs every NL condition with its ein fact and every NL deduction with its rule firing; [128 hand-formalized fixtures](examples/README.md). | [M2 P2.2](plans/m2_nl_to_ir/p2.2_formalizer/README.md) — the formalizer, one shot: a local LLM (llama.cpp) under GBNF generated from the EBNF; four passes — ontology, **theory** (activators selected from the stdlib catalogue, rules written only when it lacks the property), instance with `:source` quotes, query — under a written contract whose every prompt is a hashed field of an experiment record; `einaf from-text` (the harness's command — the kernel stays LLM-free). [P2.3](plans/m2_nl_to_ir/p2.3_benchmark/README.md) — the benchmark it is scored on: eight reasoning families, generators with exact ground truth, instances whose *correct* answer is *ambiguous* or *unsat*, frozen splits. M2 was reshaped 2026-08-23 around the research plan [`EinAf.md`](plans/m2_nl_to_ir/EinAf.md); ten phases, **not started** | [F16](plans/followups/f16_autoformalization/ideas.md) autoformalization ≠ translation · [F4 Q38](plans/followups/f4_cross_cutting.md#llm-as-factrelationtyperule-extractor-q38) the LLM as fact / relation / type / rule extractor · [F13](plans/followups/f13_puzzles_beyond_zebra/ideas.md) a BBH harness with four oracle ablations |
| **Theory Library** | Maintains reusable formal theories, relation properties, reasoning patterns and domain-specific knowledge that can be instantiated and composed for particular problems. | [`stdlib/`](stdlib/README.md) — seven `std.*` modules: the relation-algebra signature (`std.algebra`), closed-world bijection and slot inference, elimination, typing, closure, macros. A property fact such as `(transitive R)` *is* a theory instantiated: the engine binds the generic rule to `R`. Embedded in the binary, manifest-checked. **And now tested**: [`tests/stdlib/`](tests/README.md) is one program per rule or tight family (`find tests/stdlib -name '*.ein' | wc -l`), each stating what it should and should not derive — `ein test tests/` in 0.04 s. Before them **38 of the 73 rules had never fired** in any corpus run and 20 more were reached by one puzzle alone; the re-measured figure is **0**, and since [S1c.1.5](docs/history/m1c_external_validation/README.md#s1c15--in-the-gate) `cargo test` is what says so — in 0.04 s, and scoped to the suite rather than to the corpus, because a rule that fires only inside `examples/zebra.ein` has no test. **M1d S1d.2.4 grew it to 77 rules**: `total-owed` / `surjective-owed` and the two slots duals say what a state *owes* rather than what is illegal in it, and are activated by the `owe` event rather than by a `fire`. | [M1c P1c.1](docs/history/m1c_external_validation/README.md#p1c1--stdlib-conformance) — **shipped 2026-08-23/24**, all five stages (`:expect`, `ein test`, the corpus, the gate). | [F12](plans/followups/f12_rules_and_relations/) properties as closure conditions `R ⋆ R ⪯ R`; a dictionary of relation *theories* · [F8](plans/followups/f8_FCA_RCA_odis_tptp/ideas.md) is there a finite atlas? the `C(n)` curve · [F4 Q34](plans/followups/f4_cross_cutting.md) `relation-profile` and the 2⁷ table |
| **Theory Selection** | Identifies and retrieves existing theories, rules and reasoning patterns relevant to a given problem and its current formalization. | By hand: `(import std.bijection)` and `(bijective pet-loc)`; the compile unit is *rule × activator*. Two selections for one puzzle — [`zebra.ein`](examples/zebra.ein) via `std.slots`, [`zebra2.ein`](examples/zebra2.ein) via `std.bijection` — reach the same model. | [M2 S2.2.4](plans/m2_nl_to_ir/p2.2_formalizer/s2.2.4_passes.md) — the theory pass: the formalizer asserts `(bijective pet-loc)` from the stdlib catalogue and writes a rule only when no module has the property; [Q9](plans/m2_nl_to_ir/open_questions.md#q9--ontology-provenance), reframed from ontologies to theories. | [F12 `ideas.md`](plans/followups/f12_rules_and_relations/ideas.md) *select* a theory, do not invent properties — the LLM writes rules only as the last fallback · [F7 C](plans/followups/f7_rule_induction.md#sub-track-c--rule-set-sufficiency) rule-set sufficiency · [F16](plans/followups/f16_autoformalization/ideas.md) queens → knights keeps `T_placement + T_board` |
| **Theory Synthesis** | Constructs new relations, constraints, rules and theories when existing knowledge is insufficient, including specialization and composition of existing theories. | — | [M2 S2.2.4](plans/m2_nl_to_ir/p2.2_formalizer/s2.2.4_passes.md) — synthesis as the formalizer's third action, permitted only when the pass names the property the catalogue lacks, and logged as such; two benchmark families (object tracking, temporal) have no stdlib theory on purpose, so the action is exercised ([P2.3](plans/m2_nl_to_ir/p2.3_benchmark/README.md)). [F7 B](plans/followups/f7_rule_induction.md#connection-to-m2)'s critical-path flag, taken up 2026-08-23. | [F7](plans/followups/f7_rule_induction.md) rule induction from relations and facts, companion-rule synthesis · [F5](plans/followups/f5_rules_as_data.md) rules as data — induce `(transitive R)` from `(R a b) (R b c) (R a c)` · [F4 Q37](plans/followups/f4_cross_cutting.md#induction--rules-from-facts-q37) · [F12](plans/followups/f12_rules_and_relations/) predicate invention (every match pattern is an unnamed relation), law discovery in a higher-order relation algebra · [F13](plans/followups/f13_puzzles_beyond_zebra/ideas.md) synthesis to a semantic fixed point |
| **Theory Transformation and Specialization** | Adapts general theories to a particular problem context, derives specialized subtheories, and transforms representations into forms better suited for reasoning. | The mechanical half: macro expansion and imports ([`ein-ir`](ein.rs/crates/ein-ir/)), activator binding of generic rules to relations, and [`.einb`](docs/history/m1a_rust/README.md#p1a8--binary-kb-container) — a loaded, optionally saturated, KB as a file. | — | [F12 `ideas3`/`ideas4`](plans/followups/f12_rules_and_relations/) `Specialize(T, C, O)`: chess → queen placement → independent set → permutation CSP, eleven transformations, partial deduction / supercompilation / theory morphisms as prior art · [F15](plans/followups/f15_math_formulae/ideas.md) theory projection into formulae · [F7 A](plans/followups/f7_rule_induction.md#sub-track-a--generalisation-automation) · [F1](plans/followups/f1_categorical_formulation.md) / [F1b](plans/followups/f1b_logical_formulation.md) the categorical and FOL readings |
| **Symbolic Reasoning Kernel** | Executes formal reasoning over Ein representations, including saturation, deduction, rule application, constraint propagation and fixed-point computation. | [`ein-core`](ein.rs/crates/ein-core/) + [`ein-ir`](ein.rs/crates/ein-ir/) + [`ein-infer`](ein.rs/crates/ein-infer/): a register matcher (O1), semi-naive saturation (O2), NAF at the closure/world boundary (O3), contradiction (O5), provenance (O6). Parse + load `zebra2` **0.67 ms**; `solve zebra2 -e` **29 ms**; all four P1a.6 targets met, the tightest with 88 % headroom. | [M1a P1a.9](docs/history/m1a_rust/README.md#p1a9--release) — packaging shipped 2026-08-23 (`ein --version`, a release that cannot ship a red gate, a four-platform matrix awaiting its first tag); the Rust embedding page closes M1a. | [F11](plans/followups/f11_deductive_layer_perf.md) beta-memories and WCOJ — measured, declined · [F5](plans/followups/f5_rules_as_data.md#kernel-minimisation--which-inference-features-belong-in-ein-lang-vs-kernel-code) kernel minimisation |
| **Hypothesis Search** | Explores alternative assumptions and candidate models through structured backtracking over the hypothesis lattice. | The commitment lattice (O7/O8) as a region since M1e2 S1e2.1a.5: route 1 visits the alive down-set layer by layer (Apriori prefix-join, a dead set pruning every superset), route 2 the cube where the static check flags a program. Until then the walk searched it, pruned by lookahead, no-goods and downward closure. `--jobs N` fans a layer out over threads — **same verdict, same models, same counters** on 20 712 corpus cells; **3.17–4.40×** on 8 cores. | [M1d P1d.10](docs/history/m1d_satisfiability/README.md#p1d10--exhaustive-search) — why `zebra2-minus-15` does not finish: 32 models, all found by depth 3, `-e` killed at 30 min. | [F9](plans/followups/f9_e_catalog.md#what-the-catalog-taught) the closed ledger — every branch-count optimisation measured inert · [F4 Q31](plans/followups/f4_cross_cutting.md#llm-as-policy-in-search-tree-q31) the LLM as search policy · [Q-M1d.1](docs/history/m1d_satisfiability/open_questions.md#q-m1d1--may-the-search-stop-before-the-lattice-is-exhausted) may the search stop early |
| **Constraint and Satisfiability Reasoning** | Enforces structural and semantic constraints, detects incompatible assignments, and searches for models satisfying the formalized theory. | Upper bounds with force — `functional` / `injective` as `(false)` rules, domain and range elimination, negative completion. Lower bounds (`total` / `surjective`) in refutation form, **plus, since M1d [S1d.2.4](docs/history/m1d_satisfiability/README.md#s1d24--obligations-in-the-saturator) (2026-08-25), in obligation form**: a rule may assert the reserved verdict atom `(open ?R)`, and a quiescent state reports what it still owes — `zebra2-minus-15` owes 46 at root, the hand census reproduced. The report is real; the **force** is not yet, and no verdict word moved. Models are found by search, not by a decision procedure. | [M1d P1d.2](docs/history/m1d_satisfiability/README.md#p1d2--obligations) obligations `L ≤ #{…} ≤ U` and three fixpoint outcomes — **S1d.2.1–.4 done**, S1d.2.5 (branch on an obligation's alternatives) and S1d.2.6 (the verdict word) next; [P1d.3](docs/history/m1d_satisfiability/README.md#p1d3--model-sets) model sets without enumeration · [M10](plans/m10_external_benchmarks/README.md) the same problems through Z3, CVC5, SWI-Prolog, Soufflé, Clingo and Lean | [`m1d/ideas.md`](docs/history/m1d_satisfiability/ideas.md) what saturation lacks to be satisfiability · [F12 `ideas3`](plans/followups/f12_rules_and_relations/ideas3.md) choice → saturation → conflict → backtrack · [F4 Q40](plans/followups/f4_cross_cutting.md#may-a-performance-lever-decide-what-a-complete-model-is-q40) a performance lever currently decides the verdict on two fixtures — unresolved |
| **Contradiction Detection and Analysis** | Identifies inconsistent states and traces contradictions back to the rules, facts and hypotheses responsible for them. | `(X, ¬X)` / `(false)` detection; an AND/OR provenance DAG; the **unsat core** — the smallest set of given facts from which one recorded contradiction follows (over every derivation the program records; not a subset-minimal MUS); one learned no-good per dead commitment. Fixture: [`ein-bugs/zebra2-bad.ein`](examples/ein-bugs/zebra2-bad.ein). | [Q-M1d.6](docs/history/m1d_satisfiability/open_questions.md#q-m1d6--may-contradiction-be-said-with-exhausted--false) — ten corpus entries say *Contradiction* at the depth cap with `exhausted = false`; *incomplete* is the word they want. [F22](plans/followups/f22_exact_core_past_the_cap.md) — the 32-alternative cap keeps each fact's smallest-closure derivation beside it since M1e2 S1e2.2.11 ([Q-M1e.15](docs/history/m1e_review_processing/open_questions.md#q-m1e15--the-alternatives-cap-decides-which-unsat-core-is-reported), [`ein-bugs/alt-cap-core.ein`](examples/ein-bugs/alt-cap-core.ein)), but a saving between two facts behind a full list is still refused ([`alt-cap-shared.ein`](examples/ein-bugs/wrong-answers/alt-cap-shared.ein), `W10/alt-cap-shared`, deferred); no shipped puzzle is changed by it. | [F3](plans/followups/f3_three_task_classes_first_class.md) `ein why-not` / `explain` · [F12 `ideas5`](plans/followups/f12_rules_and_relations/ideas5.md) `emit false` as a forbidden pattern, `T⁺ ∩ T⁻ = ∅` · [F9](plans/followups/f9_e_catalog.md) E7 / E19: MUS minimisation is unsound under NAF |
| **Formal Verification** | Mechanically checks neural-generated formalizations, rules, theories and candidate solutions against the symbolic semantics of Ein. | Of a *formalization*: parse / load / compile diagnostics at `file:line:col`, 36 negative fixtures, the thirteen [defined behaviours](docs/kernel/defined_behaviour.md). Of the *engine*: the gate, the id-order and `--jobs` invariance sweeps, a six-property fuzzer. Nothing yet checks a *neural* output, because nothing produces one. | [M2 P2.4](plans/m2_nl_to_ir/p2.4_loop/README.md) — the loop judges every repair against the source ([S2.4.4](plans/m2_nl_to_ir/p2.4_loop/s2.4.4_faithfulness.md): *formal validity is not faithfulness*); the old validator is feedback level F1 of nine ([S2.4.3](plans/m2_nl_to_ir/p2.4_loop/s2.4.3_feedback_ladder.md)); [P2.1](plans/m2_nl_to_ir/p2.1_kernel_as_instrumentation/README.md) gives the kernel's diagnostics a versioned structured form, `ein-feedback/1` · [M1c](docs/history/m1c_external_validation/README.md#the-thesis) — no claim rests only on self-agreement: `:expect` plus external confirmation | [F17](plans/followups/f17_formal_verification/ideas.md) three applications — `NL → T`, `T → proof / model / unsat`, `(T, P) → P ⊨ T` — and Ein as the theory layer between NL / code and a prover; K framework as the closest relative · [F2 PSM.3](plans/followups/f2_self_modifying_language.md) the semantic firewall · [F1b](plans/followups/f1b_logical_formulation.md) the fragment Ein cannot decide |
| **Solution and Model Generation** | Produces satisfying models, solutions, derived facts, proofs or counterexamples depending on the problem. | `ein solve` — R described, its states as one `(region …)`; `--single`'s `Solution` / `Ambiguity` / `Contradiction` / `Open`; `--maximal` the complete answers; an empty R with its unsat core or what it owes; `-p`, `--json-summary`, `--dump-answer` (every state a loadable program); a `SOLUTIONS` store in `.einb` (library API only). | [M1d P1d.3](docs/history/m1d_satisfiability/README.md#p1d3--model-sets) compact model sets · [M10](plans/m10_external_benchmarks/README.md) every answer confirmed by a system that is not Ein | [F13](plans/followups/f13_puzzles_beyond_zebra/ideas.md) benchmark answers read off models · [Q-M1d.1](docs/history/m1d_satisfiability/open_questions.md#q-m1d1--may-the-search-stop-before-the-lattice-is-exhausted) |
| **Reasoning Introspection** | Exposes derivation traces, rule dependencies, relation dependencies, hypothesis branches, unsatisfiable cores and other reasoning artifacts. | `--trace out.md` — a self-contained markdown derivation rendered through the puzzle's own `:why` templates (`--relevant`, `--reorder`); 17 DOT views, four on the CLI; [`--events`](docs/kernel/inference/events.md) JSONL (`ein-events/1`); `--stats`, `--hyp-stats`, `--timing`, `--dump-lattice` (every node a run visited), `ein oracle --dump` (the cube). The human-style trace is the acceptance criterion ([idea 08](plans/ideas/08-human-style-deductive-trace.md)). | [M20](plans/m20_gui/README.md) — a Tauri GUI: code, graph and branches views over one `StateId` · [M2 S2.4.3](plans/m2_nl_to_ir/p2.4_loop/s2.4.3_feedback_ladder.md) — the engine's outputs rendered *for a model*, nine levels: verdict, cardinality, model difference, unsat core, relation dependency (new), provenance chains; trace → NL explanation parked ([Q7](plans/m2_nl_to_ir/open_questions.md#q7--llm-as-surface-generator)) | [F15](plans/followups/f15_math_formulae/ideas.md) rules as formulae, a theory summariser, an algebraic signature of the current theory for the LLM · [F3](plans/followups/f3_three_task_classes_first_class.md) · [F4 Q33](plans/followups/f4_cross_cutting.md) |
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

### One run, one region

Ein loads a puzzle as a typed hypergraph of relations, facts and rules,
saturates the rules to a least fixpoint, and asks which **commitments** — sets
of hypothesised facts — leave a state that is consistent and owes nothing. That
set is **R**, the solution region
([`solution_semantics.md`](docs/kernel/inference/solution_semantics.md)),
counted in states (`k_s`) and in commitments (`k_c`), and `ein solve` describes
it. Since M1e2 S1e2.1a.5 every question is a part of R, and the flag names the
part — the work done is the least the question needs:

- **bare** — all of R: its states as one `(region …)`, its minimal and maximal
  lines, components, sub-cubes, `N` (the minimal dead sets) and the route that
  computed it; partial, with bounds and a `note:`, when it reaches
  `--region-budget`;
- **`--maximal` / `--minimal`** — one line; **`-n N`** — `N` states;
- **`--single`** — *is R one state?*, answered in the four words
  `Solution` / `Ambiguity` / `Contradiction` / `Open`. They appear under
  this flag alone;
- **`--count`** — *how many?*: R's two counts and the rows that qualify
  them, by describe's route, with nothing listed and no state read.

An empty R says which kind it is: **every node dead**, reported with root's
unsat core and the `:source` of each core fact, or **every alive node owes**,
with what the owing frontier still owes. `solve` / `gaps` / `contradictions`
are **three answers to one problem**
([idea 03](plans/ideas/03-three-task-classes.md)), not three commands: R with
one state, with several, and empty. Every answer exits 0; a claim the program
states about its own answer (`:expect`) and the run refutes exits 1. An earlier
design had three functions that each *chose* their verdict up front — and so
disagreed with each other on the same input — and the walk that replaced them
answered from the states its search happened to record, which is how
[twelve wrong answers](corpus/wrong_answers.toml) reached a default run.
Describing R is what fixed 24 of the register's 34 rows.

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

<!-- transcript: ein solve examples/zebra2.ein --single -->
```sh
$ ein solve examples/zebra2.ein --single
solve · examples/zebra2.ein --single
──────────────────────────────────────────────────────────────
  asked             is R one state? (--single)
  verdict           Solution — R is one state
  decided           yes — R holds no other state
  lattice           monotone (the static check) — R is convex: no hole
  universe          56 candidates
    outside         is-a*, next-to — no candidate: no hrule proposes them. R holds of them only what the program states and saturation derives
  route             the obligation tree, over states — L1 dead is upward-closed · L2 discharged is upward-closed · L3 a commitment's state is its closure's · L4 the lines are local · L5 an owed instance's alternatives are exhaustive
  work              45 saturations · 7 582 facts beyond root

  ;; state 1 of 1 — 18 candidates · from {(color-loc Green House-5), (nation-loc Ukrainian House-2)}
  (model (color-loc Blue House-2) (color-loc Green House-5)
         (color-loc Ivory House-4) (color-loc Red House-3)
         (color-loc Yellow House-1) (drink-loc Coffee House-5)
         (drink-loc Juice House-4) (drink-loc Milk House-3)
         (drink-loc Tea House-2) (drink-loc Water House-1)
         (nation-loc Englishman House-3) (nation-loc Japanese House-5)
         (nation-loc Norwegian House-1) (nation-loc Spaniard House-4)
         (nation-loc Ukrainian House-2) (pet-loc Dog House-4)
         (pet-loc Fox House-1) (pet-loc Horse House-2) (pet-loc Snail House-3)
         (pet-loc Zebra House-5) (smoke-loc Chesterfields House-2)
         (smoke-loc Kools House-1) (smoke-loc Lucky_Strike House-4)
         (smoke-loc Old_Gold House-3) (smoke-loc Parliament House-5))

  query             (and (drink-loc Water ?h_water) (nation-loc ?who_water ?h_water) (pet-loc Zebra ?h_zebra) (nation-loc ?who_zebra ?h_zebra))
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?h_water = House-1 · ?h_zebra = House-5 · ?who_water = Norwegian · ?who_zebra = Japanese
                        “The Norwegian drinks water in House-1; the Japanese owns zebra in House-5”
  as :expect        (model (drink-loc Water House-1)
                           (nation-loc Japanese House-5)
                           (nation-loc Norwegian House-1)
                           (pet-loc Zebra House-5))
```
<!-- /transcript -->

Every word of that answer comes from the **puzzle**, not the engine: the
`(query … :goal-text "…")` template renders the headline from the goal
variables, and `--single` decides that R holds no other state, which is what
makes the answer *the* answer. A bare `ein solve examples/zebra2.ein` describes
all of R since M1e2 S1e2.1c.2, by the state route, and it is the same one state:
374 saturations, 0.54 s at `-j 1` (release build, 2026-11-03). Until then it
stopped at the default `--region-budget` before it had proved there is no
other (3.6 s, 2026-10-18). `-n 1` prints it in 30 ms without the proof.

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

Thirteen behaviours — diagnostics, orderings, error strings — are normative in
[`defined_behaviour.md`](docs/kernel/defined_behaviour.md); the full kernel
specification is [`docs/kernel/`](docs/kernel/README.md).

### The engine in numbers

**M1** (shipped 2026-06-17) solved the Zebra puzzle end-to-end. **M1a**
(shipped 2026-08-23) built `ein.rs/` and made it fast. Every number below is
on the record, with its instrument and its machine state, under
[`docs/history/m1a_rust/`](docs/history/m1a_rust/README.md). They measure the
walk: every `-e` below is its exhaustive search, which no command runs since
M1e2 S1e2.1a.5's switch, and they are kept as the record of M1a's engine.

| measurement | value | record |
|---|---:|---|
| `solve zebra2.ein -e` end-to-end | **29.0 ms**; peak RSS **17 MB** | [baseline.md § the close](docs/history/m1a_rust/measurements/baseline.md#the-four-targets-at-the-close), [P1a.6 § Targets](docs/history/m1a_rust/README.md#p1a6--performance) |
| `solve zebra.ein -e` | **47.2 ms** | same |
| parse + load `zebra2.ein` · the three-fixture acceptance gate | **0.67 ms** · **0.127 s** | same |
| the P1a.6 optimisation programme on `zebra -e` | 585.8 → 47.5 ms across the phase; matcher candidates 25 160 149 → 238 567 | [baseline.md](docs/history/m1a_rust/measurements/baseline.md) |
| `--jobs 8` on 8 P-cores, the four workloads that fan out | **3.17–4.40×** (`branching/06 -e` 194 → 44 ms); the ≥ 6× target **not met**, the remaining 1.5× named as allocation, not contention | [scaling.md](docs/history/m1a_rust/measurements/scaling.md#8-t1a721--the-fan-out-and-the-three-things-it-costs), [P1a.7](docs/history/m1a_rust/README.md#p1a7--parallelism) |
| `--jobs N` is the same computation | 20 712 (file, op, jobs) cells, **0 moved**, 30 s; the verbose event stream byte-identical at both job counts, 2 200 561 lines included; 10 000 paired fuzz runs, zero findings | [P1a.7 § acceptance](docs/history/m1a_rust/README.md#p1a7--parallelism) |
| memory under parallelism | per-worker provenance: `features/01 -e` 684–708 MB → **85–91 MB**; peak RSS 79.8 / 90.3 MB at `--jobs 1 / 16` | [design/README § Measured](docs/history/m1a_rust/design/README.md#measured) |
| `.einb`, the binary KB container | a saturated `zebra2` is 57 688 bytes and opens cold in **0.614 ms**; `solve x.einb` byte-identical to `solve x.ein`; 20 000 fuzzed inputs and 3 348 bit-flips rejected by the digest | [P1a.8](docs/history/m1a_rust/README.md#p1a8--binary-kb-container) |
| the gate | **619 tests, 0 failures, 1 m 51 s** | [`run_tests.sh`](run_tests.sh) |
| the corpus | 128 entries in six groups, 629 cells as processes in 5.3 s; the slow tier 641 cells in **19.4 s** where it was 660 in 242.6 s; **two** entries are slow and `cost_ms` says so | [corpus_cost.md](docs/history/m1a_rust/measurements/corpus_cost.md#7-the-first-re-take--2026-08-22-and-it-moved-an-entry), [`corpus/`](corpus/README.md#slow) |
| determinism | every order-sensitive site specified; `--shuffle` is MT19937 exactly | [design/02](docs/history/m1a_rust/design/02_determinism_and_order.md) |

Two of the phase histories are worth reading as method rather than as
results. P1a.6 closed on a lever matrix with a **control row** pricing each
column. P1a.7 **declined six things by a number** — a validator, two
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

- **The walk gave wrong answers on well-formed programs, at default
  settings, and seven of the register's rows are still open.** Twelve cases and
  one contested reading were re-run on 2026-10-14
  ([`inventory.md`](plans/m1e2_correctness/inventory.md)); the smallest was
  seven lines, called *contradictory* although the program with no hypothesis
  is consistent. M1e2 S1e2.1a.5's switch made `ein solve` describe R, and
  [`corpus/wrong_answers.toml`](corpus/wrong_answers.toml) records each row's
  answer now, held per commit by `ein-cli/tests/wrong_answers.rs`: **55 of 63
  rows fixed** — W1, W2, W5, W9 and W11, the lookahead's and the forced
  positive's unsound exclusions among them, which no route reads, W12 and W13,
  W4, a rule's constant missing from the candidates, which M1e2 S1e2.2.4
  seeded on 2026-11-04, W15, a name written only in a rule's `:match` or
  inside a nested fact, which M1e2 S1e2.2.14 made a candidate the same day on
  the user's ruling on names (Q-M1e2.9 (a)), W14, a domain value only a
  commitment derives, which the obligations rung never proposed until M1e2
  S1e2.2.15 made it decline on such a guard (Q-M1e2.10 (a), the same day), W7,
  an `int` and a nested fact in one activator position, which lost a
  derivation until M1e2 S1e2.2.7 gave a rule application's identity the
  activator's every argument (Q-M1e.16, the same day), W6, a relation outside
  the universe that no read-out named, which every question's `outside` row
  names since M1e2 S1e2.2.8 (Q-M1e2.3 (a), *scoped*, the same day), and W3, one of W1's and one of W8's, whose right answer
  since the user's *forbid* of 2026-11-02 is a refusal at load, which M1e2
  S1e2.2.5 built the same day (a refutation may not rest on an `absent` over a
  relation a commitment can still extend, directly or through a fact a rule
  derived under one), and W10's first witness, a core the alternatives cap
  hid, which M1e2 S1e2.2.11 keeps beside the full list the same day, and
  W11's third witness, a dead node's core that moved with the interner, until
  M1e2 S1e2.2.12 put a fork's delta in the order its commitment was written,
  the same day, and W16, an int and a string with one text, which keyed one
  compile plan, so the second activator's conclusions were lost in either
  order, and an activator whose every argument is an int or a nested fact,
  which *forbid* read as none, until M1e2 S1e2.2.16 keyed a plan by its
  activator's values and asked a plan's rule for its parameters, the same
  day, and W17, a refutation resting on an `absent` through an activator
  chain that first appears at a node, which *forbid* missed until M1e2
  S1e2.2.17 read every rule a node can compile over D followed through the
  rules root has no plan of, the same day — one
  ruled not wrong (C1), and **7 open**: two deferred to a followup by the
  user, each saying why, W8's other row (F21) and W10's second, a saving
  between two facts behind a full list, which no bounded list keeps (F22);
  and W18's five, filed by M1e2 S1e2.2.18 on 2026-11-04 and owned by it: a
  string written in a rule, which the rule compiler does not lower as the
  loader lowers it in a fact, so in a `:match` it matches nothing and in an
  `:assert` it stops the run.
  P1e2.2 works one stage per case
  ([P1e2.2](plans/m1e2_correctness/p1e2.2_wrong_answers/README.md)).
- **An under-determined puzzle still does not finish.**
  `zebra2-minus-15.ein` has 32 complete answers. `ein solve --maximal` finds
  21 of them at the default `--region-budget` in 5.2 s, and all 32 at
  `--region-budget 4194304 -j 8` in 11 s, still without proving there are no
  more (release build, 2026-10-18). The walk found the 32 by depth 3 in 25 s
  and was killed at 30 min proving the rest
  ([M1d](docs/history/m1d_satisfiability/README.md)).
- **Lower bounds are stated as obligations, and R counts them.** `total` /
  `surjective` were refutations only until M1d
  [S1d.2.4](docs/history/m1d_satisfiability/README.md#s1d24--obligations-in-the-saturator)
  made them obligations (`(open ?R)`), and a state that owes is not in R
  since the switch: `zebra2-minus-15`'s root owes **46**. What a description
  still cannot do is branch on an obligation's alternatives — it enumerates
  the commitments under the alive down-set (route 1) or the cube (route 2) —
  which is what the tree traversal did and why the bullet above is slow.
- **No config option changes an answer**, measured: the 2026-10-18 sweep of
  every flag over 173 programs moved one, `features/06` under
  `enable-symmetric-mirror`
  ([`configuration.md` § 2.5](docs/kernel/configuration.md)). Under the walk a
  performance lever decided a verdict, and both settings under-reported:
  [`branching/15`+`16`](examples/branching/15_lookahead_two_step_on.ein) said
  `Solution k=1` and `Contradiction k=0` where the hand-derived answer is two
  states. Both say two since the switch, on the maximal line.
- **`--jobs` is 1.5× short of its target**, and the shortfall is named
  ([scaling.md](docs/history/m1a_rust/measurements/scaling.md#where-the-other-15-is)).
- **Every check is still relative** — to the engine's own past — until
  [M1c](docs/history/m1c_external_validation/README.md#the-thesis) and
  [M10](plans/m10_external_benchmarks/README.md) land an independent one. The precedent is specific: a stdlib guard was wrong for a
  year, and what found it was an independent enumeration written outside the
  engine.

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
zebra2.ein -e`** (the walk's, at M1a), along with `ein kb` and `--jobs`
([feature_cost.md](docs/history/m1a_rust/measurements/feature_cost.md)).

`ein --version` names the build in **five** lines: engine version, `--events`
protocol, `.einb` container format, the features compiled in, and the SHA-256
of the `std.*` manifest **this binary will load** — the last being the one
input that can differ between a binary and the checkout beside it. The five
are what [`docs/install.md`](docs/install.md) walks through. Release binaries for Linux (x86_64 + aarch64), macOS (universal2)
and Windows are [S1a.9.3](docs/history/m1a_rust/README.md#s1a93--packaging-and-release)'s
matrix, written and reviewed; the first tag is what runs it. **`pip install`
is not a channel** — the Python binding was deferred on 2026-08-21 for want of
a consumer ([Q-M1a.23](docs/history/m1a_rust/open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding)).

### Solve

```sh
ein solve <file>                   # describe R: its states, lines, components, N
ein solve <file> --single          # is R one state? Solution / Ambiguity / Contradiction / Open
ein solve <file> --maximal         # the maximal line — the complete answers (--minimal: the minimal one)
ein solve <file> -n N              # N states, the least work that finds them
ein solve <file> --count           # how many: k_s and k_c, nothing listed
ein solve <file> --jobs 8          # fan each layer out over 8 threads (or `auto`)
ein solve <file> --trace out.md    # + a self-contained markdown derivation trace
ein solve <file> --events e.jsonl  # + the narration stream, one JSON object per step
ein solve <file> --json-summary s.json
ein solve <file> --dump-answer DIR # + the read-out uncut, and every state as a loadable program
ein oracle <file>                  # R by brute force, the cube every mode is held to
```

The flag names the part of R asked about, and each question does the least
work it needs ([`solve_interface.md`](plans/m1e2_correctness/p1e2.1a_the_solution_region/solve_interface.md)).
The budgets are `--region-budget F` (the facts a description may derive,
default 1 048 576 = 2²⁰ since 2026-11-03 — past it the run stops with an answer, bounds and a
`note:`), `--region-limit N` (the cube a flagged program may need) and
`--max-time`. `--jobs N` changes the wall clock and nothing else, and no config
option changes R ([`configuration.md` § 2.5](docs/kernel/configuration.md)).
Inspection: `-p` / `-P` (each listed state's facts), `-f` (its candidates),
`-H` (H, the universe of candidates), `--models key|grid`, `--diff-models FILE`,
`--dump-lattice DIR` (every node the run visited), `--timing`; the trace
shapers `--relevant` / `--reorder` / `--no-diagrams` apply to the `--trace`
file. The walk's options — `-e`, `-m`, `-E`, `-o`, `-z`, `-d`, `-D`,
`--dump-region`, `--region-states` — are refused with what replaced each.

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
./run_tests.sh             # the gate: six static checks, cargo test --workspace and three more steps — 329 s on 2026-11-03
./run_tests.sh --slow      # + every test's slow half (EIN_CORPUS_SLOW=1), + 8 id-space seeds
./run_tests.sh --nightly   # instead of the gate: every step of .github/workflows/nightly.yml
cd ein.rs && cargo fmt && cargo clippy --workspace --all-targets -- -D warnings
EIN_BLESS=1 ./run_tests.sh # re-bank the goldens, deliberately
```

The gate needs **Graphviz** on `PATH`: `dot_wellformed.rs` is the only
authority the DOT views have on being well-formed, and it fails rather than
skips without it. CI runs the gate per commit and the slow tier, eight id-space
seeds, the stdlib censuses, the review bundle and a two-million-mutation
frontend fuzz nightly ([`.github/workflows/`](.github/workflows/)), and
`./run_tests.sh --nightly` runs the nightly's steps here.

## Scheduled — the roadmap

The full roadmap, with its four-level schema (milestone → phase → stage →
task) and its open-question indexes, is [`plans/`](plans/README.md). Status as
of 2026-10-14:

| milestone | what | status |
|---|---|---|
| **M1** — core graph reasoning | the IR, the KB, saturation, contradiction, the hypothesis loop, the trace, the Zebra solve | **shipped 2026-06-17**; its plan folder is in git history |
| [**M1a**](docs/history/m1a_rust/README.md) — the Rust engine | `ein.rs`: the crates first, speed second, scale third | **shipped 2026-08-23** — all eleven phases closed. The plan folder was deleted with it; the record is [`docs/history/m1a_rust/`](docs/history/m1a_rust/README.md) — the eleven phases and 53 stages as one document, plus the design contracts, the measurements and twenty-three questions |
| [**M1c**](docs/history/m1c_external_validation/README.md) — external validation | the check that is not relative to Ein's own past: `:expect` on `query` + `ein test` for every stdlib rule | **shipped 2026-08-24** — one phase, five stages, and **38 of 73 never-firing stdlib rules → 0**, held by `cargo test`. The plan folder was deleted with it; the record is [`docs/history/m1c_external_validation/`](docs/history/m1c_external_validation/README.md) — the phase and its stages as one document, plus the census and seven questions. Its benchmark phase left for M10 on 2026-08-23 |
| [**M1d**](docs/history/m1d_satisfiability/README.md) — from saturation to satisfiability | why an under-determined puzzle does not finish; existence requirements as first-class **obligations**; model sets without enumeration | **shipped 2026-08-27** — four phases, eighteen stages: a program can state a requirement, a state says what it **owes**, the verdict word `Open`, every count says whether it is certified, and a second traversal reaching 32 models in 86 enterings where the lattice needs 17 204 592. The record is [`docs/history/m1d_satisfiability/`](docs/history/m1d_satisfiability/README.md) |
| [**M1e**](docs/history/m1e_review_processing/README.md) — review processing | the full-tree review of 2026-08-27, finding by finding, under a standard of proof written into [`standard_of_proof.md`](docs/kernel/standard_of_proof.md) | **closed 2026-10-14** — all 63 findings dispositioned, all ten questions answered, the gate 738 → 821 tests. The record is [`docs/history/m1e_review_processing/`](docs/history/m1e_review_processing/README.md); its probes found wrong answers the review never listed |
| [**M1e2**](plans/m1e2_correctness/README.md) — correctness | every known wrong answer: first made visible (every artefact of a run rendered to markdown with graphs, a case register, the user's review), then fixed one stage each | **next**; created 2026-10-14 — 2 phases, 20 stages, from a survey that re-ran every case ([`inventory.md`](plans/m1e2_correctness/inventory.md)) |
| [**M1f**](plans/m1f_hypothesis_and_documentation/README.md) — the structure of the hypothesis set, and the documentation ein does not have | a branch structure derived from the program rather than paid for in deaths; `:priority` removed; the doc tree a released system would have | queued after M1e2 — 2 phases, 11 stages |
| [**M2**](plans/m2_nl_to_ir/README.md) — EinAf: iterative autoformalization, through Level D | the research plan [`EinAf.md`](plans/m2_nl_to_ir/EinAf.md) as a schedule: the kernel as instrumentation (`ein-feedback/1`), the one-shot formalizer, a heterogeneous benchmark with ambiguous and unsatisfiable instances, the loop with nine feedback levels, baselines B0–B5 at matched budget, ablations G1–G9, failure taxonomy, scaling and generalization, representations compared, the formal account, the result and the demo — Level B at ~8 weeks, C at ~19, D with M5 | after M1e2; **reshaped 2026-08-23** — 10 phases, P2.1–P2.5 at stage depth (21 files), **nothing started**; the old six-phase *NL → IR* plan is Level B's half, re-targeted to the crates |
| **M2+** — ontology and rule induction | induce the theory a puzzle's text *assumes* ([F4](plans/followups/f4_cross_cutting.md) / [F7](plans/followups/f7_rule_induction.md)) | beyond M2; [F7 B](plans/followups/f7_rule_induction.md#connection-to-m2) says it is on M2's critical path |
| [**M5**](plans/m5_presentation/README.md) — paper + presentation | *From the Zebra puzzle to autoformalization*: comparison against the field, benchmarks, the growth directions as future work | parked; placeholder with the outline (was M2b) |
| [**M10**](plans/m10_external_benchmarks/README.md) — external benchmarks | the same problems stated for Z3, CVC5, SWI-Prolog, Soufflé, Clingo and Lean, run by one harness, compared on the *answer* first and the clock second; every answer checked back in as an `:expect` | queued — M1a shipped, so nothing blocks it; 5 stages, promoted from M1c's P1c.2 on 2026-08-23 |
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
a complete cardinality-BFS, not a DPLL tree) and
[F11](plans/followups/f11_deductive_layer_perf.md) (beta-memories and WCOJ,
both measured and declined, with the workload that would reopen them named).

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
| `ein.rs/crates/ein-cli/` | the `ein` binary — `solve` · `test` · `saturate` · `render` · `kb` |
| `ein.rs/crates/{ein-corpus,ein-narration}/` | dev-only: the corpus manifest, fixture helpers and the bench set; the one definition of narration-vs-content |
| [`stdlib/`](stdlib/README.md) | the ein-lang standard library, seven `std.*` modules; `MANIFEST.sha256` keeps the embedded copy honest |
| [`corpus/`](corpus/README.md) | `corpus.toml` — one entry per `.ein` with the runs it is exercised under and a measured `cost_ms`; `fuzz_findings/` |
| [`examples/`](examples/README.md) | `zebra.ein` / `zebra2.ein` (the puzzle, two encodings), `zebra2-minus-15.ein` (32 models), `zebra2-hints.ein`; per-feature fixtures in `features/`, `branching/`, `saturation/`, `lattice/`, `domain_elim/`, `syntax/`; `ein-bugs/` regressions; `broken/` parse / load / compile negatives with expected messages |
| [`c/`](c/README.md) | three plain-C Zebra baselines — what the kernel is not |
| [`build.sh`](build.sh) · [`run_tests.sh`](run_tests.sh) | everything that builds, in one command · the gate |
| [`docs/guide/`](docs/guide/) | **start here** — *Learn Ein by solving the Zebra puzzle* |
| [`docs/kernel/`](docs/kernel/README.md) | the kernel specification: graph semantics, data model, ein-lang, the inference engine, defined behaviour, the event protocol, the Zebra walkthrough |
| [`docs/api/`](docs/api/rust.md) | how to drive Ein as a library — the crates `ein-ir`, `ein-infer`, `ein-render`, `ein-einb`; the worked example is a test |
| [`docs/history/`](docs/history/README.md) | shipped milestones kept as record — [M1a, the Rust engine](docs/history/m1a_rust/README.md): eleven phases, the design contracts the crates cite, the measurements; [M1c, external validation](docs/history/m1c_external_validation/README.md): one phase, and a stdlib census that *can* be re-taken; [M1d, from saturation to satisfiability](docs/history/m1d_satisfiability/README.md): four phases, the `Open` verdict, and four re-takable censuses with a `utils/` script apiece; [M1e, review processing](docs/history/m1e_review_processing/README.md): a 63-finding review processed finding by finding, the ledger and the reports verbatim |
| [`docs/lib/`](docs/lib/README.md) | the catalogue of external tech across 12 topic files — constrained generation, solvers, provers, category theory, graph rewriting, neuro-symbolic stacks, benchmarks — plus a knowledge graph |
| [`plans/`](plans/README.md) | the roadmap: milestones M1e2, M1f, M2, M5, M10, M20, plus `followups/`, `ideas/`, open questions — M1a, M1c and M1d shipped and M1e closed, and all four moved to `docs/history/` |
| [`plans/ideas/`](plans/ideas/README.md) | the author's ten founding notes — authoritative on intent |
| [`plans/followups/`](plans/followups/README.md) | the parked research threads (`ls plans/followups/`), closed ledgers, and the raw notes behind EinAf |
| [`utils/`](utils/README.md) | the scripts driving `ein.rs` (`ls utils/*.py utils/*.sh`): renderers, checks (the manifest, the six-property fuzzer), the measurement set (`bench_env.sh` first) |
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
shipped the engine on 2026-06-17; **M1a** began `ein.rs` on 2026-08-17,
met its speed targets on 2026-08-19 and closed its parallelism phase on
2026-08-23. M3 was dropped on 2026-08-18, M1c and M1d were created on
2026-08-21 out of M1a's last two phases and the user's saturation note, and
on 2026-08-23 the project took the name **EinAf**, as the framing above became
the plan. The same day the milestone ids went sparse — the GUI became **M20**,
the write-up **M5**, and M1c's benchmark phase was promoted to **M10** — so
that a milestone can be inserted without renaming its neighbours. **M1c** then
ran in two days, 2026-08-23 → 2026-08-24, and put the standard library under
test for the first time. MIT licensed.
