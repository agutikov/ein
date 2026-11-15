# Inference engine — implementation map

The **module-by-module** developer reference for the engine. The *idiomatic*
(language-agnostic, algorithm-level) view — the nine core operations, their
CS analogs, complexity — is
[`architecture_and_algorithms.md`](architecture_and_algorithms.md); this page
is the concrete map onto code. Source root:
[`ein.rs/crates/ein-infer/src/`](../../../ein.rs/crates/ein-infer/src/), with
the data model in `ein-core` and the renderers in `ein-render`.

> **Audience: engine contributors.** A puzzle author never reads this — the
> authoring surface is [`../ir/03-ein-lang/`](../ir/03-ein-lang/).
>
> **This page is a map, not a specification.** It says where each behaviour is
> implemented; it never *defines* one. Where the engine's behaviour is only
> stated by the code, it is stated here instead:
> [`../defined_behaviour.md`](../defined_behaviour.md).

## Four design choices, and where each is written down

Four choices shape the engine's inside. Only one of them is documented on this
page; the others say where they live, because a map that repeats another map
goes stale in two places.

| | | where |
|---|---|---|
| **Integers, not objects** | every name is a `u32` `Symbol`; a fact argument is a 4-byte `Value` (`[tag:2][payload:30]`); a proposition is an interned row with a `FactId`, and identity *is* the id — `probe` is O(1), not an O(arity) compare recursing into string equality | [`../ir/02-data-model/03_implementation.md`](../ir/02-data-model/03_implementation.md) |
| **A layered, copy-on-write KB** | `Kb` is a stack of immutable `Arc<Layer>`s plus one writable top, so a fork is a push and not a copy, and the search's inner loop stops paying for the branch it is about to abandon | same |
| **A register matcher** | `compile.rs` lowers each (rule, activator) to a `Plan` of `Scan` / `Join` / `Guard` opcodes over a fixed 256-register file, and `match_.rs` executes a step span against it, so binding a variable is a register store and the inner loop allocates nothing ([design/05](../../history/m1a_rust/design/05_matcher.md)). The 256 is a ceiling: a `:match` or a `:goal` writing more distinct variables, a rule's parameters among them, is refused at load since M1f S1f.5.21 (`ein_ir::MAX_MATCH_VARIABLES`, W63), and was a `CompileError` until then | the *Saturation core* table below, and design/05 |
| **A fanned-out lattice layer** | `--jobs N` evaluates a batch of nodes on a `rayon` pool and commits them in order — every route's, through `region.rs`'s `fan_out`; it was a layer's enterings in the walk, deleted at M1e2 S1e2.1c.5 (2026-11-03) | § The fan-out, below |

## Data flow

```text
KB ─▶ Engine::compile_all ─▶ Plan ─▶ Saturator::saturate ─▶ reasoning facts
        (compile.rs)                    (saturator.rs)
                          (+ naf_guards)   │ match_.rs · firing.rs
                                           │ at quiescence: the boundary phase
   ┌───────────────────────────────────────┘
   ▼ at root's fixpoint:
 hypgen::generate_config_free ─▶ H, the universe (no config switch applied)
   │  the ladder (M1d S1d.2.5):  hrule.rs   if the puzzle declares one
   │                             oblgen.rs  else, while something is owed and branchable
   │                             blind      else
   ▼
 explore::run ─▶ one route: downset.rs (route 1) · states.rs · tree.rs · region.rs (the cube)
   │  every node: region.rs forks root, writes C, saturates, detects, tallies
   ▼
 read_out.rs ─▶ R, or the part of it the mode asked; --single's four words

 at every node, root's included, dead or alive:
   obligations::tally ─▶ Owes ─▶ the node's owing bit (never the KB)
                                   └▶ `owe` events · --json-summary · the trace
```

Until M1e2 S1e2.1c.5 (2026-11-03) the search half of this picture was the
walk: `hypgen` → `apriori` (layer N sets) → `commitment::try_commitment_set`,
driven by `solve.rs`'s BFS, with `verdict.rs` reading `k` into `Solution` /
`Ambiguity` / `Contradiction` / `Open`. It was deleted on the user's ruling of
that day, after having no CLI path since S1e2.1a.5's switch (2026-10-18).

## Saturation core — the deductive (monotone, append-only) layer

| module | role |
|--------|------|
| [`engine.rs`](../../../ein.rs/crates/ein-infer/src/engine.rs) | the per-(rule, activator) **compile cache**: `compile_all` / `compile_for`, `check_layout`, `fired`, and the NAF dependency map. It does not drive anything — the two-phase loop is `saturator.rs`'s `step` below |
| [`compile.rs`](../../../ein.rs/crates/ein-infer/src/compile.rs) | lowers each (rule, activator) to a `Plan` of opcodes: `Scan` / `Join` / `Guard` / nested pattern — plus the NAF guards, which compilation **lifts out** of the plan so `plan.steps` (and each disjunct's steps) is a purely positive closure program. Each lifted guard becomes a [`NafGuard`](../../../ein.rs/crates/ein-infer/src/plan.rs) (`scope_of` = the parent registers a sub-plan register projects from, `watched` = the relations the negative query reads, `monotone` = no nested absent); guards are per-disjunct, `Disjunct` pairs them with their steps. A *nested* absent is not lifted — it is part of the negative query. `PlanMemo` interns plans; `plan_key` is the memo key |
| [`match_.rs`](../../../ein.rs/crates/ein-infer/src/match_.rs) | runtime matcher: executes a step span over a fixed register file, `Emit` yields each match. The closure plans it runs are **purely positive**; `Matcher::holds` is the existential query the boundary asks a guard with, evaluated as one unit including any nested guard. There is no fire-time NAF re-check — that evaluation point is gone, not bypassed |
| the boundary itself | [`saturator.rs`](../../../ein.rs/crates/ein-infer/src/saturator.rs) — `admit_from_boundary`, `first_failing`, `negative_premises` | **the closure/world boundary**, the one place NAF is evaluated. There is no `World` type wrapping the KB at quiescence: the boundary asks its three questions of the KB directly, because the boundary phase is the only code that can run there and a read-only wrapper would buy nothing the borrow checker is not already giving. The questions: does the guard's query hold (`W ⊨ ∃x̄·`), which guard is the first to fail, and which `(relation, args)` patterns had to fail |
| [`firing.rs`](../../../ein.rs/crates/ein-infer/src/firing.rs) | `Firing` record; `fire()` substitutes `:assert`, builds the derived fact with its `Prov` — including the boundary queries the firing was admitted under. `resolve` is leaf resolution in bindings |
| [`saturator.rs`](../../../ein.rs/crates/ein-infer/src/saturator.rs) | the **two-phase** fixpoint loop. `closure_step` runs purely positive plans to quiescence (a FIFO heap, the written and arrival order since M1f S1f.5.15 took `:priority` out, delta-driven semi-naive re-enqueue, `__symmetric__` mirror); a guarded match is routed to `parked` instead of the queue; at quiescence `admit_from_boundary` judges parked candidates against that fixpoint and admits **one**, then the closure re-runs. A watch stamp skips re-asking a guard none of whose `watched` relations grew; a failing `monotone` guard retires its candidate. Observables: `naf_dropped` (structurally **0**), `naf_rounds`, `naf_admitted`, `naf_retired`. Also owns `Snapshot` / `resume` — the fork-entry delta: a fork resumes root's saturation rather than re-deriving it — and records a re-derivation as an alternative justification, from the `__symmetric__` mirror too |
| [`terms.rs`](../../../ein.rs/crates/ein-core/src/terms.rs) | the structural reserved atoms (`not` / `and` / `or` / `absent` / `false`) — `STRUCTURAL`, in `ein-core` beside the interner rather than in the engine, because the lexer needs them too |
| [`predicates.rs`](../../../ein.rs/crates/ein-infer/src/predicates.rs) | computed-predicate registry (`eq` / `neq`), and the program's names a guard's free variable ranges over (`domain`, M1f S1f.5.7) |
| [`plan.rs`](../../../ein.rs/crates/ein-infer/src/plan.rs) | the compiled-plan representation the two above share: `Plan`, `Disjunct`, `Step`, `NafGuard`, `Probe`, and the `MAX_REGS` = 256 register file (`ein_ir::MAX_MATCH_VARIABLES`) — a `:match` or a `:goal` writing more distinct variables than that is refused at load since M1f S1f.5.21 (W63), the compiler's check an invariant since; it was a `CompileError` until then |

## Hypothesis generation & commitment-lattice search — the non-monotone layer

Five modules in this table — `apriori.rs`, `commitment.rs`, `nogoods.rs`,
`solve.rs` and `sanity.rs` — were the walk's, and M1e2 S1e2.1c.5 deleted them
with it (2026-11-03). Their rows are kept, marked, so a citation of one still
says what it was. The search is the routes since — `explore.rs`, `downset.rs`,
`states.rs`, `tree.rs`, with `region.rs`'s node function under all of them —
and their rows are in the next table.

| module | role |
|--------|------|
| [`hypgen.rs`](../../../ein.rs/crates/ein-infer/src/hypgen.rs) | candidate enumeration (type-blind, S1.7.23); the filter pipeline (negated-facts / already-exists / lookahead / seen); `score_hypothesis`; `HypGenStats`. Also `complete` / `open_hypotheses` / `is_solution_node`: solution-node tracking is three predicates over what hypgen would propose, and separating them from it would be a file boundary, not a seam. `generate_blind` (M1d S1d.3.1) is the ladder with its two upper rungs skipped — **not a search entry point**, but the only way to ask *how many facts would the blind enumerator still propose here?* of a state the active rung called complete. It is config-free since M1e2 S1e2.3.1, as `generate_config_free` is, so it writes nothing but the interning of its candidates; `--json-summary`'s `leftover` block (`EIN_LEFTOVER=1`) runs it on a discarded fork of each listed state all the same. `generate_config_free` (M1e2 S1e2.1a.1) is the ladder with the lookahead filter and its kill cache off and the obligations rung at its default order, whatever the config says — the lattice's universe, since no config option may change the lattice; `region::prepare_with` keeps its stats, which are the summary's `root.hypgen` (S1e2.3.1). `generate`, the walk's, reads the two retired fields at their defaults and is run by no command, only by `shape.rs`'s `hyp_shape` for the corpus's `hyp` digest. A search's own root call records what the lookahead dropped in `HypGenStats::lookahead_killed`, so a solve reads the same list without a second walk |
| [`hrule.rs`](../../../ein.rs/crates/ein-infer/src/hrule.rs) | hypothesis-rule registry (`Hrules` drive generation, never the saturator) |
| [`oblgen.rs`](../../../ein.rs/crates/ein-infer/src/oblgen.rs) | **the obligations rung** (M1d S1d.2.5) — the ladder's middle: candidates are the facts that would discharge what the state owes, read by running each obligation guard's own sub-plan with the *witness* step skipped (`Matcher::scan_without`), so the branch set is the guard rather than a restatement of it. Declines — and hands the whole call to the blind enumerator — when an obligation scans a relation the rung itself proposes (the domain contract's C4), when since M1e2 S1e2.2.15 its trigger or a guard beside its witness reads a relation in D, which `monotone::dynamic` computes for the rung and the static check alike (C4 over derivation, Q-M1e2.10 (a), W14), or when its projection does not resolve for an activator; reports **stuck** when something is owed and every debt is scoped out. `Choice` is the walk order (`rule-order` / `fail-first` / `off`), pinned at `rule-order` by every caller since M1e2 S1e2.3.1 retired `EIN_OBLIGATION_CHOICE`, the environment lever that chose it |
| [`lookahead.rs`](../../../ein.rs/crates/ein-infer/src/lookahead.rs) | pre-branch one-step death simulator (`enable_pre_branch_lookahead`); walks each disjunct and evaluates its guards in the world **with** the candidate `h` — no match in the KB *and* none created by `h`. A guard with a nested absent is non-monotone and cannot be decided that cheaply, so the disjunct is skipped rather than guessed — losing a kill keeps the "never reports a live hypothesis as dead" contract |
| `apriori.rs` | **Deleted with the walk at M1e2 S1e2.1c.5 (2026-11-03).** Commitment-lattice layer generation by set-size (prefix-join + no-good prune); `order_candidates` / `canonicalise` — the deterministic candidate ordering, a total one: across value kinds it is `Int < Sym < Fact` by tag, so a candidate set mixing slot types still sorts |
| `commitment.rs` | **Deleted with the walk at M1e2 S1e2.1c.5 (2026-11-03).** `try_commitment_set`: fork + write hypotheses + saturate + detect — the saturation stops at the killing firing when `enable_fail_fast_fork` (default on), and resumes root's rather than re-deriving it when given a snapshot |
| `nogoods.rs` | **Deleted with the walk at M1e2 S1e2.1c.5 (2026-11-03).** No-good learning: dead set → the root KB's no-good store; singletons → negated facts |
| `solve.rs` | **Deleted with the walk at M1e2 S1e2.1c.5 (2026-11-03).** `SolveError` moved to [`error.rs`](../../../ein.rs/crates/ein-infer/src/error.rs). It was **the main loop**: BFS over the commitment lattice; the root phase, the layer phase, dedup by canonical `state_key`; `LatticeProof`, `SolutionRecord`, `DeadCommitment`, `LatticeStats`; `compute_alive` / `promote_forced_positives` / `record_node` / the dead-handling path. It also holds the **second traversal** (M1d S1d.10.6): `tree_traversal()` reads `EIN_TRAVERSAL`, `tree_node` branches on one owed instance's jointly-exhaustive alternatives instead of enumerating subsets of `alive`, and it **declines** on any other rung. Off by default, `-n` honoured and `-m` refused ([`configuration.md`](../configuration.md)). Since M1e2 S1e2.1a.1 the root phase also records the config-free universe and `alive₀` (`Solved::universe`), runs [`monotone`](../../../ein.rs/crates/ein-infer/src/monotone.rs)'s check after root's tally (`Solved::monotonicity`), and on a site calls `non_monotone` — **a stub** that narrates and changes nothing; no command reaches it since the switch, the warnings that replaced it are [`detector`](../../../ein.rs/crates/ein-infer/src/detector.rs)'s (S1e2.1b.5), and it goes with the walk at S1e2.1c.5. Under `store_lattice` the proof keeps every entering (`LatticeProof::entered`) and, since S1e2.1a.2, every forced positive against the enterings before it (`LatticeProof::promotions`), so a dump can place an entering on the cube node it stood on |
| `sanity.rs` | **Deleted with the walk at M1e2 S1e2.1c.5 (2026-11-03).** The commutativity sanity check |
| [`invariant.rs`](../../../ein.rs/crates/ein-infer/src/invariant.rs) | **the M1 alive-set invariant, evaluated** (M1e S1e.3.3, `ST-M1`) — `Universe::of` is the baseline the *loaded* program reaches, `rule_breaches` the static read of the rules' `:assert` constants that runs at root under `--events`, and `breaches` the post-fixpoint scan the corpus sweep uses to confirm the induction the static one assumes. Nothing evaluated this until M1e |
| [`ein-render/src/dump/`](../../../ein.rs/crates/ein-render/src/dump/) | `region.rs` · `region_answer.rs` · `region_lattice.rs` · `serialise.rs` · `json.rs` — the region, answer and lattice-node dumps, which are **rendering** and live with the other renderers rather than inside the solver. The walk's `state.rs` (`--dump-states`), `lattice.rs`, `answer.rs` and `snapshot.rs`, and `region.rs`'s `walk.json`, went with it at M1e2 S1e2.1c.5 (2026-11-03). `json.rs` also **reads** (`parse`, since M1e2 S1e2.1a.2), so a dump can be drawn from its files without `serde` in a shipping crate |
| [`ein-render/src/read_out.rs`](../../../ein.rs/crates/ein-render/src/read_out.rs) | **the read-out as text** (M1e2 S1e2.1a.5, [`read_out.md`](read_out.md)) — what `ein solve`'s region modes and `ein oracle` print from `ein_infer::read_out`: the rows, R as a `(region …)` or its members, each query's classes and `as :expect`, the claim; stdout's listings sharing 100 lines with `answer.txt` the uncut form; the stderr lines and the exit code. Its golden is `read_out_examples.md`, held by `ein-cli/tests/read_out_examples.rs` |
| [`ein-render/src/dump/region_answer.rs`](../../../ein.rs/crates/ein-render/src/dump/region_answer.rs) | **the read-out as data** (M1e2 S1e2.1a.5) — `--json-summary` as `ein-summary/2` and `--dump-answer DIR` as `ein-answer/2` for the region modes: `answer.txt`, `answer.json`, one loadable program per state under `models/` (read again through the node function), the owing frontier under `owing/`, `nogoods.json`, `index.json`. Every key on every run; `ein-cli/tests/region_answer.rs` holds it |
| [`ein-render/src/region_doc.rs`](../../../ein.rs/crates/ein-render/src/region_doc.rs) · [`region_dot.rs`](../../../ein.rs/crates/ein-render/src/region_dot.rs) | **the region drawn** (M1e2 S1e2.1a.2, S1e2.1a.7) — `region_doc` reads an `ein oracle --dump` directory back and joins a region run to the cube, from the run's `--dump-lattice` and `--dump-answer` (`Overlay`: what the run did with each node, missing and spurious by state against the part of R it asked, every reading it printed and every exclusion checked against the cube); `region_dot` draws it — the pure cube with saturation as a move inside it, four views and a table. `ein render region DIR [--visited DIR] [--answer DIR]` ([§ Region rendering](../ir/03-ein-lang/04_dot_rendering.md#region-rendering--the-cube-and-a-region-run)) |

## Contradiction, verdict, provenance, config

| module | role |
|--------|------|
| [`contradiction.rs`](../../../ein.rs/crates/ein-infer/src/contradiction.rs) | detector: `(X, ¬X)` pairs (whatever either side's origin — S1.22.1b) + `(false)`; `contradicts(kb, fact)` is the O(1) incremental dual asked of each fact as it lands, which is what let a dying fork stop saturating (S1.9.E23). That fork was the walk's, deleted at M1e2 S1e2.1c.5 (2026-11-03); a route's node saturates to quiescence, so no route asks it |
| [`explain.rs`](../../../ein.rs/crates/ein-infer/src/explain.rs) | minimum-cardinality explanation over the AND/OR proof graph (each fact an OR-node via its justifications, each justification an AND-node over its premises and, since M1e2 S1e2.2.19, the activator its plan was compiled from where a node derived or committed it, `walks::node_activator`, and since S1e2.2.21 the facts its universals read, `Prov::universal`, each folded at its own smallest frontier): ATMS-style least-fixpoint label propagation, cycle-safe by construction; `explain` / `contradiction_frontier` (a contradiction's facts together, a pair's two sides as one AND-node, M1e2 S1e2.2.21) / `minimal_contradiction_frontier`; `ExplanationBudget` caps the worst-case-exponential search and the result reports truncation. Minimal over the **recorded** derivations — i.e. relative to the rule set and the saturation strategy. It also carries `smallest_contradiction_frontier`, the verdict path's unsat core, which was `explain.rs`: the frontier was a thin caller of the search and is now the search's own entry point. And `support_frontier`, the same search with two things read otherwise, for `structure.rs`'s theory's share: the activator of every firing is followed, root's among them, where a core follows only one a node derived or committed, and a fact the loaded program stated is a leaf whatever its record says. Its frontier is the stated facts and the commitments a death rests on; past the budget's count of facts it reads no support and says so, having no walk to degrade to |
| [`verdict.rs`](../../../ein.rs/crates/ein-infer/src/verdict.rs) | a query's goal projected over a state — `query_value` / `solve_value`, `goal_bindings`, `goal_plan_error` — which is what the read-out answers each `(query …)` with. Until M1e2 S1e2.1c.5 (2026-11-03) it was also the walk's verdict types, deleted with the walk; `--single`'s four words are `explore::Verdict` since. As they were: `Solution` / `Ambiguity` / `Contradiction` / `Open`; verdict read from the model count `k`; `Answer::Aborted` for a run a budget cut. **`Open`** (M1d S1d.2.6) is `k = 0` with a debt rather than a conflict, and is why `k` — *models* — parts company with `stats.solution_nodes` — *recorded nodes*. Scoped by `OwesReport::in_scope`: only a program that states an obligation can reach it |
| [`error.rs`](../../../ein.rs/crates/ein-infer/src/error.rs) | `SolveError` (re-exported as `ein_infer::SolveError`) — why a question about R was not answered: a saturation or compile error, an unsupported question or unreadable claim, or `-T`'s deadline. Every route and the read-out return it. It was `solve.rs`'s until M1e2 S1e2.1c.5, and the walk's two arms went with the walk: a budget that stopped it (a route reports a partial answer instead) and `-y`'s commutativity failure |
| [`canon.rs`](../../../ein.rs/crates/ein-infer/src/canon.rs) | `state_key` — order-insensitive canonical state identity (the representation is the identity; `state_digest` is display-only) |
| [`closed.rs`](../../../ein.rs/crates/ein-infer/src/closed.rs) | `__closed__` handling (`CLOSED` constant; suppress guessing) |
| [`naf_deps.rs`](../../../ein.rs/crates/ein-infer/src/naf_deps.rs) | static NAF-dependency map; the derived-NAF warning — not "this rule leans on the fire-time re-eval" (that re-eval is gone) but "NAF over a derived relation is the shape that can make a rule set non-stratified", the case where the engine reports one model of several. Advisory, off by default; a real stratification checker is future work |
| [`monotone.rs`](../../../ein.rs/crates/ein-infer/src/monotone.rs) | **is the program monotone?** (M1e2 S1e2.1a.1) — a static check over compiled plans: `H` the candidate relations, `D` what a commitment can change, every read classified static / dynamic-monotone / dynamic-non-monotone by its `absent` depth and the rule's kind (an obligation's trigger reads at the polarity an `absent` does). No site is a theorem: no holes, two lines describe `R`. No interning, no event, no counter, so a solve asks it without moving ([`region_dump.md`](region_dump.md#monotonicity--the-static-check)) |
| [`forbid.rs`](../../../ein.rs/crates/ein-infer/src/forbid.rs) | **the refusal *forbid* asks for** (M1e2 S1e2.2.5, ruled 2026-11-02 and, its F2, 2026-11-04) — a rule concluding `(false)` or `(not …)` may not rest on an `(absent P)` over `D`, directly (an odd-depth read), through a fact a rule derived under one (*via*), or through its activator; and a rule with parameters whose activator relation is in `D` may not read its parameter's relation under an odd `absent`. Read in `region::prepare` after the static check, on a flagged program only, so a monotone one pays one test; its error is `SolveError::Refused`, one line per rule ([`defined_behaviour.md` § 4.7](../defined_behaviour.md#47-a-refutation-resting-on-an-absent-a-commitment-can-fill--m1e2-s1e225)). `forbid::studying` lifts it on the calling thread for a library-level fixture; no CLI path calls it. `ein-infer/tests/refutation_under_absent.rs` holds the refused set |
| [`strata.rs`](../../../ein.rs/crates/ein-infer/src/strata.rs) | **the rule dependency graph and its strata** (M1f S1f.5.15's T3 and T4, 2026-11-08) — over the plans a run compiled, as `naf_deps` asks, since a relation-polymorphic rule has edges only per plan: an edge from each relation a plan reads to each it writes, negative at odd `absent` depth, and one from a rule's activator relation; each relation's stratum, and the negative cycles, where no stratification exists. The detector's cycles over `D` are this graph restricted to `D`. Its negative cycles over every relation are the static non-stratifiability diagnostic (T4), on `monotone::Monotonicity::negative_cycles`, which the read-out's `strata` row and `ein saturate --dump`'s `;; Strata` line print; `rule_strata` is the stratum the dump's rule listing prints. T3 also derived a band per rule from its shape, to order the agenda where `:priority` had; the user ruled plain FIFO (Q-M1f.6), and the band went with the keyword at T5 |
| [`detector.rs`](../../../ein.rs/crates/ein-infer/src/detector.rs) | **what a flagged program is warned of** (M1e2 S1e2.1b.5, the design approved 2026-11-03) — gated on `monotone`'s verdict, so a program with no site pays one test. The static half, read off root's plans at load and carried on `Monotonicity::detector`: what each site concludes and which property of a node it puts at stake (the state or the status, [`monotonicity.md` § 7.6](monotonicity.md#76-the-program-the-third-stronger-property)), the cycles through an `absent` over `D` with their parity, and the order findings, where the boundary's (priority, FIFO) admission can decide what a node holds. The dynamic half, `Warning`, on every `explore::Answer` of a flagged program: the status (the cube's violations attributed to their sites), the state (every node the run computes read for stale facts, in `region.rs`'s node function) and the paths (the read-out's spot check, `region::Nodes::by_parent`). It decides nothing: what a node is there is F21's. `ein-infer/tests/detector.rs` pins every cycle and every order finding on the corpus by name, and the dynamic half's numbers on the proposal's programs |
| [`read_out.rs`](../../../ein.rs/crates/ein-infer/src/read_out.rs) | **the read-out's data** (M1e2 S1e2.1a.5): `explore`'s answer, every state it reached read once through `region.rs`'s node function (`Nodes`) for its `(model …)` and for each query's projection classes, every query answered from one solve, and the program's `(solve … :expect …)` checked against the R this run described (`claim::check_described`), not a second one. Under `--count` (M1e2 T1e2.1c.4.6) it reads no state: the queries are kept with no class and every claim is not checked, with why. `ein-render`'s `read_out.rs` prints it; `ein-infer/tests/read_out.rs` holds each model to the oracle's fixpoint and each claim to a check that computes R again |
| [`region.rs`](../../../ein.rs/crates/ein-infer/src/region.rs) | **the solution region by brute force** (M1e2 S1e2.1a.1) — every subset of the config-free root candidate list, forked from root and saturated to quiescence with none of the search's machinery; two bits per node (dead, owing — read on dead nodes too), and the topology over the status array: lines, holes, repairs, components, sub-cubes, `N`, `E`, the duality, the violations of upward closure. Since S1e2.1a.2 each node also says where saturation takes it (`closure`), which candidates it negates (`excluded`), and how many facts it holds that are not coordinates (`beyond`). Run on its own load by `ein solve --dump-region` ([`region_dump.md`](region_dump.md)) |
| [`downset.rs`](../../../ein.rs/crates/ein-infer/src/downset.rs) | **route 1: a monotone program's region from its alive down-set** (M1e2 S1e2.1a.9) — layer by layer from `∅` by the prefix join over candidate indices, visiting a commitment only when every one-smaller subset is alive, so it saturates the alive set and `N` where `region.rs` saturates `2ⁿ`; each node is `region.rs`'s own fork, through the same fan-out. It declines on a flagged program, at a budget of facts beyond root (`2²⁰` since 2026-11-03, M1e2 S1e2.1c.2's T1e2.1c.2.6; it was `2¹⁹`, ruled 2026-10-16; checked before each layer against the layer's size times the facts per node of the layer before, so whole layers only), and when it sees an alive discharged node with an alive owing node one fact up, which would contradict the static verdict. `ein-infer/tests/region.rs` holds it to the oracle node for node on every monotone program with `n ≤ 16` |
| [`structure.rs`](../../../ein.rs/crates/ein-infer/src/structure.rs) | **the structure of the hypothesis set, read from the lattice with nobody declaring it**: the *pair layer*, what the nodes of one candidate and of two decide at root, over H as root builds it. The candidates dead alone; which candidate's own node holds which other (*implies*) and which share one closure (*equivalent*); the pairs that *exclude*, each member alive alone and the pair dead, which are `N`'s members of two candidates; and the alive pairs that *interact*, whose fixpoint holds a fact neither member's does. Every node is `region.rs`'s own, through `explore.rs`'s node cache. On a monotone program a pair with a member dead alone (L1) or one of whose members implies the other (L3) is decided without its node; on a flagged program every pair is saturated, and nothing above root follows from what the layer says. It reads nodes and no route reads it. `ein-infer/tests/pair_layer.rs` holds it to `N` and to the read-out's `dead alone` on every corpus program whose describe exhausts, and `ein-infer/examples/pair_layer.rs` takes its census. **And the theory's share of it** (`theory`, a `Theory`): the same universe read a second time in the *small KB*, the program with the puzzle's statements taken out, *rules, ontology and the two facts*. The small KB is the loaded program over fewer of its facts, every rule, declaration and import kept, built over the same `Terms` so that a candidate is one fact in both, and its nodes are `region.rs`'s over its own saturated root and the universe **root** built (`region::prepare_over`); its pair layer's excluded pairs are the exclusions the theory makes alone, over all of H, a candidate dead alone at root among them. Which stated facts are the puzzle's nothing declares, and `small_kb` reads it off the candidates with no relation's name and no `:source`: a fact of a candidates' relation is dropped; where the program's own hrules or obligations built H, so is every other fact that names one of the names those relations hold, but for a fact of a relation that rung reads, which is what the universe is made of; and where the blind enumerator built H, only a fact that names a candidates' relation with such a name, or two such names. The module's header says what that cannot tell (a clue written as a rule, a clue that names no candidate's object, the blind enumerator's universe, a module's table over the candidates' objects). A smaller program derives no more unless a rule reads the absence of a statement it dropped, so what is dead in the small KB and alive at root (`against_root`) is empty where the rule fits a program and names the program where it does not. The second reading of a pair root excludes is the *support* of its death, `explain::support_frontier`: the core's search grounded out on the facts the program states and following root's activators, which a core leaves to the program; a pair whose support lies inside the small KB is the theory's, and otherwise the support names the statements it is the puzzle's by. A support is an explanation, sufficient and not least, and costs its node a second saturation. Each is checked: the small KB with the statements the support names put back, and every statement it drops of a relation a rule reads the absence of, kills the pair by a node inside the program's own. `ein-infer/tests/theory_share.rs` holds the share by invariances: a declared bijection's rows and columns are exactly the theory's pairs (`zebra2.ein`), two programs one clue apart have one theory, one puzzle in two encodings has one theory (`zebra.ein`), the small KB derives no more than the program on every corpus program the per-commit sweep reads, and the two readings agree on the instances. **And the cover** (`cover`, a `Cover`): H as the *groups* the theory makes of it, *pick at most one*. A group is a maximal set of two or more candidates that pairwise exclude by the theory alone, a maximal clique of the small KB's excluded pairs over all of H; the groups overlap, a candidate of a bijection being in a row and in a column, so they are a cover and no partition, and a candidate no pair of the theory holds is in no group. Two maps, group to members and member to groups; each candidate carries what root says of it alone (`Mark`: at root, dead alone, alive alone), a candidate dead alone at root being a member like any other and a group's *open* members those alive alone. The graph is the theory's and not root's, since at root a pair two declarations exclude and one two clues exclude are the same dead node and a row stops being maximal once the clues have decided part of a puzzle; the maximal cliques of root's exclusion graph are kept beside (`Cover::root_cliques`) and are not groups. No relation is read to form a group, by its name, its arguments or a declaration; `Cover::standing` reads a set a program declares, an owed instance's alternatives, against the cover. The enumeration is Bron and Kerbosch's with a pivot, bounded by `CLIQUE_STEPS`, 2^20 calls for one graph, a constant and no lever, and a graph it stops on says what was cut (`Cut`); a group is its members in rendered-text order and the groups are in the order of those, so a cover the bound did not cut is a function of the program. `Cover::alive_at_root` lists the theory's pairs whose node is alive at root, empty where the small KB derives no more than the program, which is what makes *at most one* true of the program's own lattice. It is computed once a program, from the program's prepared space (`cover_in`, over the space a route has open and `Loaded::of` the program, taken before root is saturated; root's singles and pairs are then that space's nodes), or from a freshly loaded program (`cover`), or off a `Theory` (`Theory::cover`). No route reads it. `ein-infer/tests/cover.rs` holds it by the declared control, `zebra2.ein`'s 36 groups being the rows and columns of its five bijections with every candidate in two, by the same groups under `zebra.ein`'s one relation, and by what a cover is on every corpus program its sweep reads; and `ein-infer/tests/structure_invariance.rs` holds the whole structure, the pair layer, the theory's share, the cover and *exactly one*, to a permuted id space and to every import form, on every corpus program that searches. **And *exactly one*** (`exactly_one`, an `ExactlyOne` beside the `Cover`): a group says *at most one*, and the *must* has one source in the engine, an obligation. `Owed` is L5's table, `oblgen::Alternatives` as the obligation tree reads it at root's fixpoint, with every instance's witnesses beside (`oblgen::witness_rows`, the same scan, licensed or not): each witness is a candidate, or denied at root, or neither, with the candidates whose own node holds it at root and in the small KB. Read against the cover, an instance L5 licenses whose alternatives are exactly a group's members makes it an **owed group**, of which every state of `R` holds exactly one open member (a member dead alone at root is an alternative and is in no alive state); one whose alternatives do not pairwise exclude is an **owed set that is not a group**, *at least one*; a group with no such instance is a **group nothing owes**, *at most one*; and an instance L5 does not license keeps its reason, and is *about* a group where the candidates that bring its witnesses by the theory alone are exactly that group's members, which does not make the group owed. L5's two sentences that name a witness are one reason here (`UNPROPOSED`), since which of them it says depends on whether a node has interned the fact. **A grid** is found from the groups alone (`Cover::grids`): a connected component of the theory's exclusion graph in which every candidate is in exactly two groups, the groups are two families no two groups of one of which share a member, and two groups share at most one member; one relation's candidates may be several grids, and a component that is none says why (`NotAGrid`). **A bijection** is a grid with as many groups in one family as in the other, every one of them owed. Nothing is synthesised and nothing is concluded from a count: `ExactlyOne::share` says how many groups of each family are owed and stops there. The table is read before any node and is not handed to the node function (`explore::Space::owed`); no route reads any of it. `ein-infer/tests/exactly_one.rs` holds it by its controls, `zebra2.ein`'s five grids being five bijections, `zebra.ein`'s one relation holding the same five grids with one family of each owed and the other the subject of 18 instances L5 does not license, and SEND + MORE = MONEY's eight letters owed and ten digits not; and `cover.rs`'s sweep holds what it is on every corpus program it reads, the table to `oblgen::alternatives_of` of the program loaded again among it. **The coupling** (`Coupling`, in `structure/coupling.rs`) is what ties the questions: a question is a group or a candidate in no group, and an edge is two or more questions that bear on each other, some conflict or some derivation needing a member of each of them or some debt a member of one of them (`EdgeKind`). It is read from root's pair layer, the ties, and from **the whole node**, root with every candidate alive alone committed, one fork saturated to quiescence and read with every rule application that reached it (`explore::Space::supported`, `region::Nodes::with_fixpoint_supported`, the saturator reading a redundant application's universals where asked and on no route). A candidate of a grid is in two groups, so the edges are a cover of the ways to a conclusion by the classes of questions that describe them, and a derived fact's label is one class. Each edge names the puzzle's statements it rests on. **A debt is an edge of its own kind** (`Debt`): an instance of L5's table whose payers lie in two questions or more ties those questions, since a state of R holds one of its payers, and one whose payers are all in one question is that question's own. Whether L5 licenses the instance does not decide it, and the edge says which (`Debt::licensed`): a licensed instance's payers are its alternatives, an unlicensed one's those a single candidate's node shows, so its edge may be too small. The components, of the candidates alive alone, are over every edge and are what R is the product over, and the components over the conflicts' and the derivations' edges alone stand beside them (`conflict_components`). Under the whole node's reading the edges read one application back are kept too (`one_step`), the finer ones a union of labels absorbs. `ein-infer/tests/coupling.rs` holds it to a describe wherever one exhausts, every member of N inside one edge of a conflict or a derivation and R the product of its parts over the components, and no product over the conflicts' components only where a debt's payers lie in two of them, and by its controls, a conflict of three being one edge of three candidates and SEND + MORE = MONEY's edges being its columns; the module's header says what it does not tell. No route reads it |
| [`states.rs`](../../../ein.rs/crates/ein-infer/src/states.rs) | **the state route: a monotone program's region over its states** (M1e2 S1e2.1c.2) — from root's state, the alive states by size, each saturated once with its one-up extensions and nothing else, on L1–L3: a dead extension prunes its supersets, a commitment already a state is not saturated again, one set is saturated once. Describe's route since S1e2.1c.2 (the modes' routes over states are `tree.rs`'s); it declines where what it meets contradicts the theorem it rests on. Each node is `region.rs`'s own, through the same fan-out |
| [`described.rs`](../../../ein.rs/crates/ein-infer/src/described.rs) | **R, described: what every surface says about the region, built once** (M1e2 S1e2.1a.5, [`read_out.md`](read_out.md)) — read off route 1's down-set or off the cube, and read by the read-out and by `claim.rs`'s check, so no two of them count R differently. Per state, the commitments that reach it, its minimal ones and its lines; the shape's components and sub-cubes; root; the owing frontier of an empty R. A partial run's description holds the members met, with a line flag only where it is certified, and no shape. `ein-infer/tests/explore.rs` holds both builders to the oracle |
| [`explore.rs`](../../../ein.rs/crates/ein-infer/src/explore.rs) | **every question `ein solve` asks about R** (M1e2 S1e2.1a.9, [`solve_interface.md`](../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/solve_interface.md)): which part (all of R, the minimal line, the maximal line) × what is asked (describe, `-n N` states, `--single`; and over all of R, `--count`, which it answers by describe's route, M1e2 T1e2.1c.4.6). A monotone program's describe takes the state route (`states.rs`, M1e2 S1e2.1c.2), and every other question the routes over states of `tree.rs` (S1e2.1c.4); until then they took route 1 with a cut per mode. A flagged program takes the cube, in layer order where the question can be decided early. Every node is `region.rs`'s own; every skip is a licence the static class proves; a budget stops a run with bounds and never a different answer. `ein-infer/tests/explore.rs` holds every mode to the oracle on every program it enumerates |
| [`tree.rs`](../../../ein.rs/crates/ein-infer/src/tree.rs) | **the modes' routes on a monotone program** (M1e2 S1e2.1c.4): over states, never commitments (L3), each extension pruned by a known dead set (L1). **The obligation tree**: an owing state branches on one owed instance's alternatives where licence L5 holds (`oblgen.rs` checks its premise at load, and every node it computes re-checks it), on every extension otherwise; `-n` takes its leaves and then the states of R above them, `--single` stops at its second leaf or checks its one leaf's extensions. The maximal line: a climb from a first state of R, then *dualise and advance* above each minimal state of R. The minimal line: each leaf's commitment shrunk while it stays in R, then every owing state extended once and the line as the alive minimal transversals of the maximal owing states' complements. `ein-infer/tests/explore.rs` holds every mode to the oracle, `tests/region.rs` L5 itself |
| [`obligations.rs`](../../../ein.rs/crates/ein-infer/src/obligations.rs) | **the obligation pass** (M1d S1d.2.4) — `tally(kb, …) -> Owes`, one sweep over the quiescent KB *after* the fixpoint, over the rules in `Program::obligations` that neither the saturator nor `hypgen` walks. Reads `(open ?R)`'s argument off the compiled `:assert` (a `Slot::Const`, the activator having substituted the parameter), matches, judges the guards, and returns the undischarged instances with their rendered `:why`. **Never writes**: openness is a per-node verdict, not a fact, so the tally lives on the node — `owes` on `region.rs`'s node and the routes' states, the walk's `CommitmentSetResult.owes` until M1e2 S1e2.1c.5 deleted it (2026-11-03) — beside its dead bit. `Owes::default()` for a program that states no obligation. The walk skipped it on a dead node; a route reads it on dead nodes too, which the minimal discharging sets `E` need |
| [`ein-core/src/why.rs`](../../../ein.rs/crates/ein-core/src/why.rs) | `:why` / `:goal-text` template rendering — text, so it lived in the renderer until M1d S1d.2.4, when the obligation pass needed it from *below* `ein-render`. `ein_render::render_why` still names it |
| [`ein-core/src/config.rs`](../../../ein.rs/crates/ein-core/src/config.rs) | `SolverConfig` — the live solver flags (`enable_pre_branch_lookahead`, `enable_lookahead_kill_cache`, `record_alternative_justifications`, `hypgen_scoring`, `candidate_order_seed`, `lattice_order`, …). In `ein-core` because the KB reads some of them |
| [`events.rs`](../../../ein.rs/crates/ein-infer/src/events.rs) | the **event protocol** — `--events FILE`, one JSON object per line narrating every compile miss, enqueue, firing, mirror, park/admit/retire, quiescence, alternative justification, hypothesis verdict, entering and layer; the walk's `nogood`, `writeback` and `traversal` went with it at M1e2 S1e2.1c.5 (2026-11-03). Off by default and free when off. Schema: [`events.md`](events.md). `ein-narration` is its one consumer |

## The fan-out — `--jobs N`

[P1a.7](../../history/m1a_rust/README.md#p1a7--parallelism), closed
2026-08-23 at **3.17–4.40× on 8 cores**. It is the one part of the engine
described here rather than mapped.

**What M1e2 S1e2.1c.5 (2026-11-03) left of it.** The section below describes
the walk's fan-out, deleted with the walk: `solve.rs`'s pool, the bounded
batches (`BATCH_PER_WORKER`, and the `EIN_BATCH_PER_WORKER` lever with them)
and `Run::fan_out_this_layer`. The mechanism carried over to the routes as
`region.rs`'s `fan_out`, which every route computes its nodes through: a
`rayon` pool built only when `jobs > 1` (`pool_of`), `Terms::lend`, a
per-worker view, and a node a worker cannot finish without numbering a new
fact handed back and computed on the committing thread, in order — so the
result is the sequential one at any `jobs`, which `jobs_invariance.rs` holds.
Its measurements (the 3.17–4.40×, the 248 writebacks, the 20 712 cells) are
the walk's.

**Where the threads are.** `solve.rs` builds one `rayon::ThreadPool` per solve
and **only when `jobs > 1`** — a default run creates no thread at all. A
fanned-out layer runs in **bounded batches**, so the results in flight cannot
grow with the layer: `jobs × BATCH_PER_WORKER` (512), and when a cut is
possible at all — `stop_after` or a budget — clamped down to *the enterings
already committed*, so the work thrown away by a cut is at most the work
already done. The flat `batch = jobs` this replaces was right about the cut
and wrong about the common case, since `-n 1` is the CLI's default and three
of four measured workloads never reach a solution under it. `jobs` lives in
`SolveOptions` and deliberately **not** in `SolverConfig`: a puzzle file must
not be able to set a thread count.

**Which layers.** `Run::fan_out_this_layer` is one line — `layer > 1 ||
!cfg.enable_singleton_writeback` — and it is the whole safety argument. A
worker may not write a fact to root, and the only enterings that do are the
size-1 singleton writebacks: **248 of 248 of them, across 8 158 205 enterings
and five layers, are in layer 1**, which is 0.016 % of the search. So layer 1
runs sequentially and everything above it fans out with no validator at all —
[design/08](../../history/m1a_rust/design/08_parallelism.md) §2's speculation
validator was measured, costed and then **deleted**, because case 1 is a
fanned-out layer by construction, case 2 was 0 in 1 078 704 audited enterings,
and case 3 needs a writeback there is none of.

**What a worker may touch.** Everything shared is `&`-shared or per-worker,
which is why there is no protocol and nothing for `loom` to model:

- `&FactStore` — read-only. A search assigns 41 to 417 fact ids *per solve*,
  and per *entering* four of six measured workloads assign zero, so a worker
  that would have to number a proposition hands the entering back
  (`Overflow::Shared`) and the committing thread re-runs it. `JobStats::handed_back`
  counts that, and it is a running number rather than a claim.
- `Terms` is **lent** for the layer (`Terms::lend`): a lent table is readable
  from every thread and growable from none, so sharing is what forbids growing
  — no lock, no shard. `intern` stays `&mut` and therefore stays on the
  committing thread.
- The **provenance arena is per worker**, with promotion only on the solution
  path (`Kb::promote_provenance`). That was a memory fix before it was a
  parallelism one: `features/01 -e` went from 684–708 MB peak RSS to 85–91 MB
  *at `--jobs 1`*.

**Why the answer cannot move.** The commit is **ordered**, and narration is
ordered with it: a worker builds its event lines into its own buffer with a
hole where the ordinal goes, and `Events::replay` fills them in at the commit.
So the verbose event stream is byte-identical at `--jobs 1` and `--jobs 8`,
`branching/06 -e`'s 2 200 561 lines included. Three instruments hold that —
[`jobs_invariance.rs`](../../../ein.rs/crates/ein-render/tests/jobs_invariance.rs)
(20 712 (file, op, jobs) cells, byte equality, 0 moved), the event stream under
both stop policies, and the fuzzer's `jobs` property (10 000 paired runs, zero
findings). [Q-M1a.7](../../history/m1a_rust/open_questions.md#q-m1a7--may---jobs--1-move-counters)
is decided: **no counter moves**, and validation is not what buys that.

**Inert without the feature.** `parallel` is default-on and forwarded from the
binary; without it `SolveOptions::jobs` is accepted and every layer runs on the
committing thread. `ein --version` is what says which build you have.

## Cross-cutting invariants

- **Append-only KB** — the saturator only adds facts; the one retracting flow
  is a fork for a hypothesis branch, which resumes root's saturation rather
  than re-deriving it.
- **The closure is purely positive; negation happens only at the boundary** —
  every top-level `(absent …)` is lifted out of its plan at compile time, so
  the plans the matcher runs to quiescence consult no negation whatsoever, and
  a match says nothing about it. Guards are evaluated **once**, at positive
  quiescence, against the resulting fixpoint — that is evaluation point E1, and
  there is no other: the fire-time re-check (E2) is deleted, and `naf_dropped`
  is structurally 0. Admission is one candidate per boundary round into an
  empty queue, so the admitted firing runs against exactly the world its guard
  was judged against and nothing can go stale. **One admission per round is a
  soundness requirement, not a throttle** — admitting a batch lets one
  admission invalidate another's guard after its verdict was taken, which on
  `p ← absent q; q ← absent p` derives both. Consequences: on a **stratified**
  rule set the result does not depend on the firing order (FIFO since M1f
  S1f.5.15, which removed `:priority` and its bands), and a non-stratified one
  is still answered by
  operational order — now boundary-admission order. Normative definition:
  [`absent_semantics.md`](absent_semantics.md); operational narrative:
  [`README.md` § NAF semantics](README.md).
- **Alive-set soundness** (the M1 invariant) — rules assert no new objects /
  relations / nested-fact hypotheses, so `alive = f(closed KB)`; see
  [`README.md` § M1 invariant](README.md). **Evaluated since M1e S1e.3.3** by
  [`invariant.rs`](../../../ein.rs/crates/ein-infer/src/invariant.rs), whose
  operational form is
  [`defined_behaviour.md` § 3.3](../defined_behaviour.md#33-the-m1-alive-set-invariant-operationally):
  a static read of the program's names at load, `invariant::program_names`
  (the facts' arguments at any depth, the rules' `:match` and `:assert`
  constants), which seeds the blind enumerator's objects (a rule's `:assert`
  constant since M1e2 S1e2.2.4, Q-M1e.21 (b); the rest since S1e2.2.14,
  Q-M1e2.9 (a)), and a scan that finds no run reaching a name outside the
  program's.
- **Provenance is per derivation, not per fact** — a fact's `Prov` is the
  primary justification and the KB's justification index returns every recorded
  one, so a fact is an OR-node over AND-nodes and the proof structure is an
  AND/OR graph. The alternatives table is *history*, not a projection of the
  fact list: an index rebuild deliberately leaves it alone, and a fork or a
  snapshot copies it per KB rather than sharing it by reference (a fork-local
  justification may name hypothesis premises root never assumed). Terminals
  take no alternatives — a `source` / `hypothesis` primary is the frontier, and
  a rule-kind primary with no premises is a synthetic engine writeback whose
  contract is that provenance walks ground out on it
  ([`reserved_engine_strings.md`](reserved_engine_strings.md)).
- **Negative dependence is recorded, not yet interpreted** — a firing admitted
  at the boundary carries the queries that had to fail on its provenance (one
  `(relation, args)` pattern each, a free marker where the query ranged free),
  so `Deps(Y)` = `PositiveDeps(Y)` ∪ `NegativeDeps(Y)` is finally
  representable. No walk reads it yet: the unsat core, the explanation search
  and the trace's "using" line still follow positive premises only — which is
  why deletion-based core minimisation stays unsound (corollary C3 of
  [`absent_semantics.md`](absent_semantics.md)).

## See also

- [`architecture_and_algorithms.md`](architecture_and_algorithms.md) — the
  idiomatic (O1–O9, CS-analog) view this map is the code-level companion to.
- [`README.md`](README.md) — design principles, M1 invariant, NAF, determinism.
- [`absent_semantics.md`](absent_semantics.md) — the normative reading of
  `(absent P)` these modules implement: worlds, the single evaluation point
  E1, and the corollaries (C1–C7) the closure/boundary split rests on.
- [`../defined_behaviour.md`](../defined_behaviour.md) — the diagnostics,
  orderings and error strings the engine defines.
- [`reserved_engine_strings.md`](reserved_engine_strings.md) — the engine-internal
  reserved atoms these modules key on.
