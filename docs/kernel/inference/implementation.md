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
> [`../defined_behaviour.md`](../defined_behaviour.md). This file was a map of
> `ein.py/src/ein/inference/` until M1a
> [S1a.10.6](../../history/m1a_rust/README.md#s1a106--the-docs-after-the-oracle);
> the module *roles* are unchanged, because ein.rs is a behaviour-exact port
> and the two layouts differ in five places, each flagged **⤳** below.

## What the port was free to change, and where each is written down

M1a's two invariants pull in opposite directions on purpose: **I1** froze
every observable — same language, same stdout bytes, same exit codes, same
verdicts, same counters — and **I2** put everything inside on the table. Four
things took I2 up, and they are what make this a different engine rather than
a transliteration. Only one of them is documented on this page; the others say
where they live, because a map that repeats another map goes stale in two
places.

| | | where |
|---|---|---|
| **Integers, not objects** | every name is a `u32` `Symbol`; a fact argument is a 4-byte `Value` (`[tag:2][payload:30]`); a proposition is an interned row with a `FactId`, and identity *is* the id — `probe` is O(1) where a tuple compare was O(arity) recursing into string equality | [`../ir/02-data-model/03_implementation.md`](../ir/02-data-model/03_implementation.md) |
| **A layered, copy-on-write KB** | `Kb` is a stack of immutable `Arc<Layer>`s plus one writable top, so a fork is a push and not a copy, and the search's inner loop stops paying for the branch it is about to abandon | same |
| **A register matcher** | `compile.rs` lowers each (rule, activator) to a `Plan` of `Scan` / `Join` / `Guard` opcodes over a fixed 256-register file, and `match_.rs` executes a step span against it. [design/05](../../history/m1a_rust/design/05_matcher.md) §1 is the reason: 46 % of an exhaustive solve's self time was unification the old data model made impossible to do quickly. The 256 is [D1](../../history/m1a_rust/divergences.md#d1--a-rule-may-not-bind-more-than-256-variables) | the *Saturation core* table below, and design/05 |
| **A fanned-out lattice layer** | `--jobs N` evaluates a layer's enterings on a `rayon` pool and commits them in order | § The fan-out, below |

## Data flow

```text
KB ─▶ Engine::compile_all ─▶ Plan ─▶ Saturator::saturate ─▶ reasoning facts
        (compile.rs)                    (saturator.rs)
                          (+ naf_guards)   │ match_.rs · firing.rs
                                           │ at quiescence: the boundary phase
   ┌───────────────────────────────────────┘
   ▼ on quiescence, no goal:
 hypgen ─▶ apriori (layer N sets) ─▶ commitment::try_commitment_set ─▶ detect
   │  ▲                                  (fork + write + saturate)        │
   │  └─ the ladder (M1d S1d.2.5):    solve.rs drives the BFS ◀───────────┘
   │       hrule.rs   if the puzzle declares one                          │
   │       oblgen.rs  else, while something is owed and branchable        │
   │       blind      else                                                │
   │                                  └▶ verdict.rs reads k → Solution / Ambiguity / Contradiction

 at every consistent fixpoint (root, and each alive entering):
   obligations::tally ─▶ Owes ─▶ the lattice node (never the KB)
                                   └▶ `owe` events · --json-summary · the trace
```

## Saturation core — the deductive (monotone, append-only) layer

| module | role |
|--------|------|
| [`engine.rs`](../../../ein.rs/crates/ein-infer/src/engine.rs) | `Engine` driver: per-(rule, activator) compile cache; `compile_all` / `compile_for`; tracks `fired`; the NAF dependency map. Its queue-less `step()` is **two-phase**: purely positive matches fire first, and a NAF-guarded one is considered only at positive quiescence |
| [`compile.rs`](../../../ein.rs/crates/ein-infer/src/compile.rs) | lowers each (rule, activator) to a `Plan` of opcodes: `Scan` / `Join` / `Guard` / nested pattern — plus the NAF guards, which compilation **lifts out** of the plan so `plan.steps` (and each disjunct's steps) is a purely positive closure program. Each lifted guard becomes a [`NafGuard`](../../../ein.rs/crates/ein-infer/src/plan.rs) (`scope_of` = the parent registers a sub-plan register projects from, `watched` = the relations the negative query reads, `monotone` = no nested absent); guards are per-disjunct, `Disjunct` pairs them with their steps. A *nested* absent is not lifted — it is part of the negative query. `PlanMemo` interns plans; `plan_key` is the memo key |
| [`match_.rs`](../../../ein.rs/crates/ein-infer/src/match_.rs) | runtime matcher: executes a step span over a fixed register file, `Emit` yields each match. The closure plans it runs are **purely positive**; `Matcher::holds` is the existential query the boundary asks a guard with, evaluated as one unit including any nested guard. There is no fire-time NAF re-check — that evaluation point is gone, not bypassed |
| **⤳** the boundary itself | [`saturator.rs`](../../../ein.rs/crates/ein-infer/src/saturator.rs) — `admit_from_boundary`, `first_failing`, `negative_premises` | **the closure/world boundary**, the one place NAF is evaluated. ein.py had this as a `World` type wrapping the KB at quiescence; ein.rs asks the same three questions of the KB directly, because the boundary phase is the only code that can run there and a read-only wrapper bought nothing the borrow checker was not already giving. The questions are unchanged: does the guard's query hold (`W ⊨ ∃x̄·`), which guard is the first to fail, and which `(relation, args)` patterns had to fail |
| [`firing.rs`](../../../ein.rs/crates/ein-infer/src/firing.rs) | `Firing` record; `fire()` substitutes `:assert`, builds the derived fact with its `Prov` — including the boundary queries the firing was admitted under. `resolve` is leaf resolution in bindings |
| [`saturator.rs`](../../../ein.rs/crates/ein-infer/src/saturator.rs) | the **two-phase** fixpoint loop. `closure_step` runs purely positive plans to quiescence (priority-banded heap, delta-driven semi-naive re-enqueue, `__symmetric__` mirror); a guarded match is routed to `parked` instead of the queue; at quiescence `admit_from_boundary` judges parked candidates against that fixpoint and admits **one**, then the closure re-runs. A watch stamp skips re-asking a guard none of whose `watched` relations grew; a failing `monotone` guard retires its candidate. Observables: `naf_dropped` (structurally **0**), `naf_rounds`, `naf_admitted`, `naf_retired`. Also owns `Snapshot` / `resume` — the fork-entry delta ([D3](../../history/m1a_rust/divergences.md#d3--a-fork-resumes-roots-saturation-einpy-re-derives-it)) — and records a re-derivation as an alternative justification, from the `__symmetric__` mirror too |
| **⤳** [`terms.rs`](../../../ein.rs/crates/ein-core/src/terms.rs) | the structural reserved atoms (`not` / `and` / `or` / `absent` / `false`) — `STRUCTURAL`, in `ein-core` beside the interner rather than in the engine, because the lexer needs them too |
| [`predicates.rs`](../../../ein.rs/crates/ein-infer/src/predicates.rs) | computed-predicate registry (`eq` / `neq`) — the `Guard` evaluators |
| [`plan.rs`](../../../ein.rs/crates/ein-infer/src/plan.rs) | the compiled-plan representation the two above share: `Plan`, `Disjunct`, `Step`, `NafGuard`, `Probe`, and the `MAX_REGS` = 256 register file that [D1](../../history/m1a_rust/divergences.md#d1--a-rule-may-not-bind-more-than-256-variables) is about |

## Hypothesis generation & commitment-lattice search — the non-monotone layer

| module | role |
|--------|------|
| [`hypgen.rs`](../../../ein.rs/crates/ein-infer/src/hypgen.rs) | candidate enumeration (type-blind, S1.7.23); the filter pipeline (negated-facts / already-exists / lookahead / seen); `score_hypothesis`; `HypGenStats`. **⤳** also `complete` / `open_hypotheses` / `is_solution_node`, which were `hypgen.rs`: solution-node tracking is three predicates over what hypgen would propose, and separating them from it was a file boundary, not a seam |
| [`hrule.rs`](../../../ein.rs/crates/ein-infer/src/hrule.rs) | hypothesis-rule registry (`Hrules` drive generation, never the saturator) |
| [`oblgen.rs`](../../../ein.rs/crates/ein-infer/src/oblgen.rs) | **the obligations rung** (M1d S1d.2.5) — the ladder's middle: candidates are the facts that would discharge what the state owes, read by running each obligation guard's own sub-plan with the *witness* step skipped (`Matcher::scan_without`), so the branch set is the guard rather than a restatement of it. Declines — and hands the whole call to the blind enumerator — when an obligation scans a relation the rung itself proposes (the domain contract's C4) or its projection does not resolve for an activator; reports **stuck** when something is owed and every debt is scoped out. `EIN_OBLIGATION_CHOICE` is the walk-order lever (`rule-order` / `fail-first` / `off`), deliberately not a `SolverConfig` field because the config is in the KB-shape digest |
| [`lookahead.rs`](../../../ein.rs/crates/ein-infer/src/lookahead.rs) | pre-branch one-step death simulator (`enable_pre_branch_lookahead`); walks each disjunct and evaluates its guards in the world **with** the candidate `h` — no match in the KB *and* none created by `h`. A guard with a nested absent is non-monotone and cannot be decided that cheaply, so the disjunct is skipped rather than guessed — losing a kill keeps the "never reports a live hypothesis as dead" contract |
| [`apriori.rs`](../../../ein.rs/crates/ein-infer/src/apriori.rs) | commitment-lattice layer generation by set-size (prefix-join + no-good prune); `order_candidates` / `canonicalise` — the deterministic candidate ordering, and where [D2](../../history/m1a_rust/divergences.md#d2--sortedalive-raises-in-einpy-where-einrs-answers)'s cross-tag order is consulted |
| [`commitment.rs`](../../../ein.rs/crates/ein-infer/src/commitment.rs) | `try_commitment_set`: fork + write hypotheses + saturate + detect — the saturation stops at the killing firing when `enable_fail_fast_fork` (default on), and resumes root's rather than re-deriving it when given a snapshot |
| [`nogoods.rs`](../../../ein.rs/crates/ein-infer/src/nogoods.rs) | no-good learning: dead set → the root KB's no-good store; singletons → negated facts |
| **⤳** [`solve.rs`](../../../ein.rs/crates/ein-infer/src/solve.rs) | **the main loop**: BFS over the commitment lattice; the root phase, the layer phase, dedup by canonical `state_key`; `LatticeProof`, `SolutionRecord`, `DeadCommitment`, `LatticeStats`; `compute_alive` / `promote_forced_positives` / `record_node` / the dead-handling path. This is ein.py's whole `monotonic/` package — solver, lattice, `_state`, `_helpers` — in one module, minus the dumps |
| [`sanity.rs`](../../../ein.rs/crates/ein-infer/src/sanity.rs) | the commutativity sanity check (`monotonic/sanity.py`) |
| **⤳** [`ein-render/src/dump/`](../../../ein.rs/crates/ein-render/src/dump/) | `state.rs` · `lattice.rs` · `serialise.rs` · `snapshot.rs` · `json.rs` — the lattice and state dumps, which are **rendering** and live with the other renderers rather than inside the solver |

## Contradiction, verdict, provenance, config

| module | role |
|--------|------|
| [`contradiction.rs`](../../../ein.rs/crates/ein-infer/src/contradiction.rs) | detector: `(X, ¬X)` pairs (whatever either side's origin — S1.22.1b) + `(false)`; `contradicts(kb, fact)` is the O(1) incremental dual asked of each fact as it lands, which is what lets a dying fork stop saturating (S1.9.E23) |
| **⤳** [`explain.rs`](../../../ein.rs/crates/ein-infer/src/explain.rs) | minimum-cardinality explanation over the AND/OR proof graph (each fact an OR-node via its justifications, each justification an AND-node over its premises): ATMS-style least-fixpoint label propagation, cycle-safe by construction; `explain` / `minimal_contradiction_frontier`; `ExplanationBudget` caps the worst-case-exponential search and the result reports truncation. Minimal over the **recorded** derivations — i.e. relative to the rule set and the saturation strategy. It also carries `smallest_contradiction_frontier`, the verdict path's unsat core, which was `explain.rs`: the frontier was a thin caller of the search and is now the search's own entry point |
| [`verdict.rs`](../../../ein.rs/crates/ein-infer/src/verdict.rs) | `Solution` / `Ambiguity` / `Contradiction`; verdict read from the model count `k`; `goal_bindings` |
| [`canon.rs`](../../../ein.rs/crates/ein-infer/src/canon.rs) | `state_key` — order-insensitive canonical state identity (the representation is the identity; `state_digest` is display-only) |
| [`closed.rs`](../../../ein.rs/crates/ein-infer/src/closed.rs) | `__closed__` handling (`CLOSED` constant; suppress guessing) |
| [`naf_deps.rs`](../../../ein.rs/crates/ein-infer/src/naf_deps.rs) | static NAF-dependency map; the derived-NAF warning — not "this rule leans on the fire-time re-eval" (that re-eval is gone) but "NAF over a derived relation is the shape that can make a rule set non-stratified", the case where the engine reports one model of several. Advisory, off by default; a real stratification checker is future work |
| [`obligations.rs`](../../../ein.rs/crates/ein-infer/src/obligations.rs) | **the obligation pass** (M1d S1d.2.4) — `tally(kb, …) -> Owes`, one sweep over the quiescent KB *after* the fixpoint, over the rules in `Program::obligations` that neither the saturator nor `hypgen` walks. Reads `(open ?R)`'s argument off the compiled `:assert` (a `Slot::Const`, the activator having substituted the parameter), matches, judges the guards, and returns the undischarged instances with their rendered `:why`. **Never writes**: openness is a per-node verdict, not a fact, so the tally lives on `CommitmentSetResult.owes` beside `kind`. `Owes::default()` for a program that states no obligation, and skipped entirely on a dead node, where the read-out never consults it |
| **⤳** [`ein-core/src/why.rs`](../../../ein.rs/crates/ein-core/src/why.rs) | `:why` / `:goal-text` template rendering — text, so it lived in the renderer until M1d S1d.2.4, when the obligation pass needed it from *below* `ein-render`. `ein_render::render_why` still names it |
| **⤳** [`ein-core/src/config.rs`](../../../ein.rs/crates/ein-core/src/config.rs) | `SolverConfig` — the live solver flags (`enable_pre_branch_lookahead`, `enable_lookahead_kill_cache`, `record_alternative_justifications`, `hypgen_scoring`, `candidate_order_seed`, `lattice_order`, …). In `ein-core` because the KB reads some of them |
| [`events.rs`](../../../ein.rs/crates/ein-infer/src/events.rs) | the **event protocol** — `--events FILE`, one JSON object per line narrating every compile miss, enqueue, firing, mirror, park/admit/retire, quiescence, alternative justification, hypothesis verdict, entering, no-good and writeback. Off by default and free when off. Schema: [`events.md`](events.md). It was built for the port's T2 parity tier — "the two engines took the same steps" — which retired with the second engine at M1a S1a.10.3; the format did not, and `ein-parity` is its one consumer |

## The fan-out — `--jobs N`

[P1a.7](../../history/m1a_rust/README.md#p1a7--parallelism), closed
2026-08-23 at **3.17–4.40× on 8 cores**. It is the one part of the engine with
no counterpart in the ported design, so it is described here rather than
mapped.

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
  is a fork for a hypothesis branch, which takes a fresh saturator (or resumes
  root's — [D3](../../history/m1a_rust/divergences.md#d3--a-fork-resumes-roots-saturation-einpy-re-derives-it)).
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
  rule set the result no longer depends on rule priority (band discipline is
  advisory, not load-bearing), and a non-stratified one is still answered by
  operational order — now boundary-admission order. Normative definition:
  [`absent_semantics.md`](absent_semantics.md); operational narrative:
  [`README.md` § NAF semantics](README.md).
- **Alive-set soundness** (the M1 invariant) — rules assert no new objects /
  relations / nested-fact hypotheses, so `alive = f(closed KB)`; see
  [`README.md` § M1 invariant](README.md).
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
- [`../defined_behaviour.md`](../defined_behaviour.md) — the behaviours whose
  only statement used to be the Python source.
- [`reserved_engine_strings.md`](reserved_engine_strings.md) — the engine-internal
  reserved atoms these modules key on.
