# P1a.3 — Deductive core

**Milestone:** [M1a — Rust port](../README.md)
**Status:** **shipped** 2026-08-18 — all four stages, acceptance below.
**Estimate:** 3.5 weeks (18 days of stages)
**Depends on:** [P1a.2](../p1a.2_kb_core/README.md)
**Blocks:** [P1a.4](../p1a.4_search_layer/README.md)

## Goal

The engine proper: the pattern compiler, the matcher, the two-phase
closure/boundary saturator, the `World` NAF boundary, and contradiction
detection. At the end of this phase the engine reaches **T2 event-trace
parity** — the same firings, in the same order, with the same provenance
— on every fixture that saturates.

This is the phase where the port stops being a transcription and starts
being an engine: [design/05](../design/05_matcher.md)'s register machine
and [design/06](../design/06_saturation.md)'s two exact wins land here.

## Stages

| stage | title | est. |
|---|---|---|
| [S1a.3.1](s1a.3.1_compiler.md) | Pattern compiler → plan bytecode | 4 d |
| [S1a.3.2](s1a.3.2_matcher.md) | Register matcher, candidate probes, entry points | 5 d |
| [S1a.3.3](s1a.3.3_saturator.md) | Closure loop, semi-naive delta, queues, mirror | 5 d |
| [S1a.3.4](s1a.3.4_world_and_contradiction.md) | NAF boundary, negative provenance, clash detection | 4 d |

**The boundary landed with S1a.3.3, not S1a.3.4.** `Saturator::step` is
the two-phase loop and does not terminate without an
`admit_from_boundary` — so `World`, the scope projection and
`negative_premises` (T1a.3.4.1–2) are S1a.3.3's dependency rather than
its sequel, exactly as S1a.2.4 was S1a.2.3's. What S1a.3.4 then carried
was the refinements Win B is made of, the fixture, and the measurement.

## Acceptance for the phase

All met, 2026-08-18, except one item that moved to P1a.5 and one that
P1a.4 is the first phase able to run — both below, with their reasons.
**207 tests** in `cargo test --workspace`, of which 13 are differential
against `ein.py`.

| item | result |
|---|---|
| **T2** on the saturating corpus, plus root saturation of `zebra.ein` and `zebra2.ein` (502 / 378 facts): identical firing sequence, identical per-fact provenance, identical alternative-justification lists | **64 files, 23 848 events, 0 differences** — every `compile`, `enqueue` (tiebreaker included), `fire` (redundant ones too, at `verbose`), `mirror`, `park`/`admit`/`retire`, `quiesce` and `alt`, plus `ABSENT` lines for negative provenance and `CLASH` lines for the detector's output |
| Counters identical: `naf_rounds`, `naf_admitted`, `naf_retired`, `naf_dropped == 0`, redundant-firing count, `len(engine.cache)` | in the `SUMMARY` line of every one of those 64 diffs. `zebra`: 119 rounds / 118 admitted / 354 retired / 0 dropped / 32 plans. `zebra2`: 40 / 39 / 84 / 0 / 125 |
| Plan-shape parity for every `(rule, activator)` pair | **62 files, 231 plans, 0 differences** — step sequence, slot kinds, disjunct split, each guard's scope / `watched` / `monotone`, assert templates, `asserted_relation` / `negated_relation` / `naf_relation_refs` |
| Match parity: same matches, same order, same bindings, same premises | **63 files, 1 879 matches, 0 differences**, over a full run *and* a `run_seeded` at every fact in the KB — which is what forces the premise-order contract |
| Every `CompileError` message byte-identical, all four cases | `examples/broken/compile/`, each with its `.expected`, held to by both suites |
| No heap allocation in the matcher's inner loop (counting allocator) | **0 allocations** over a 64-edge self-join and 0 over a 256-edge one, with the callback reading bindings and premises |
| Win A — compile calls down to one per distinct `(rule, activator)` pair, cache **order** unchanged | one per pair, process-wide (`PlanMemo`). `zebra2` compiles **19** plans in **21.8 µs**; the cache order is what the plan-shape and T2 diffs compare |
| Win B — guard sub-plan evaluations down ≥ 80 % | **not met, and the target is the thing that was wrong** — [Q-M1a.17](../open_questions.md#q-m1a17--win-bs-80--assumed-monotone-guards-dominate) |
| `ein.rs saturate` byte-identical | **moved to [P1a.5](../p1a.5_presentation/README.md)** — see below |

The bench set gained three of its four remaining engine rows:

| bench | ein.py | ein.rs | |
|---|---:|---:|---|
| `saturate_root` zebra2 (load excluded) | 90 ms | **2.89 ms** | 31× |
| `match_hot` — every plan over the saturated zebra2 root | 2 110 µs | **38.6 µs** | 55× |
| `boundary` — a zebra root saturation, 80 % of which is the boundary | — | 7.10 ms | — |

`match_hot` is a ratio over the **same work**, not the same wall clock:
`crates/ein-infer/examples/engine_cost.rs` reports 125 plans, 691 matches
and **2 075 premises** consumed on the zebra2 root, and the Python
measurement consumes the same 2 075.

The per-commit conformance tier, re-run at close: **455 cells, 0
differences, T3, 321.3 s of engine time**. `./run_tests.sh`: 1 503 + 21
passed.

### The instrument this phase needed

Three, each one layer down from the last, and each following
[S1a.2.3](../p1a.2_kb_core/s1a.2.3_loader.md)'s shape — both
implementations render the same text and the texts are diffed, because
none of this has a CLI surface `ein-conformance` could see:

- **`plan-shape`** — every compiled plan, in `compile_all` order.
- **`match-shape`** — every match every plan produces, over a full run
  and a `run_seeded` at every fact. Bindings in **bind order**, premises
  as fact **positions**, so an order difference names itself.
- **`saturate-events`** — the `--events` protocol itself
  ([`docs/kernel/inference/events.md`](../../../docs/kernel/inference/events.md)) at
  `verbose`, so a redundant firing is emitted rather than only counted.
  This is not a fourth rendering that agrees with T2 by inspection: it
  *is* T2, delivered through the oracle because the CLI that will carry
  `--events` is P1a.5's.

### Where the phase's scope moved

- **The boundary landed with the loop** (above).
- **The register file is one space per plan, not per disjunct.**
  design/05 §2 allows different registers for the same variable in two
  disjuncts. Sharing one space is what lets the `:assert` templates
  compile **once** and still resolve against whichever disjunct produced
  the match, and it is safe because the trail is fully unwound between
  disjuncts. A guard sub-plan still gets its own space — it has to, since
  the boundary runs it under `project(bindings, scope)`.
- **`Slot::Opaque` is kept rather than collapsed.** ein.py's unifier
  falls through to `slot == arg` for an unrecognised shape, and no IR
  node equals a `str`/`int`/`Fact`, so such a slot never matches. Keeping
  the node is what lets the plan-shape diff print it — a port that agreed
  on "never" would have agreed for the wrong reason.
- **`Plan`s are `Arc`-shared out of the memo.** `_apply` holds a plan
  while it writes `engine.fired`, and P1a.7 will share the same plans
  across threads; both want a handle rather than a borrow.
- **`ein-core` gained two things the engine needed**: `Value::UNBOUND`
  (the two tag bits have four states and three are used, so the
  all-ones word is a sentinel `pack` can never produce), and a per-layer
  count of rule-application facts, which is the version counter
  `compile_all` skips its walk on.

### Where design/05 and design/06 were wrong, and what replaced them

Two claims did not survive contact, and both are recorded rather than
quietly worked around:

- **The `Probe` cannot be fully static** (design/05 §2 says it can).
  Two of `_candidates`' three conditions are dynamic: whether a register
  is bound at a step depends on the *entry point* — `run_seeded` removes
  a step from the sequence and binds it first — and whether a bound
  register holds a nested `Fact` depends on the data. So the compile-time
  win is **narrowing the scan**, not removing it: the compiler emits an
  ordered candidate list with the statically-unkeyable slots dropped, and
  the runtime takes the first entry that is a constant or a bound
  non-fact register. A debug assertion compares that choice against a
  live `_candidates` walk on every step.
- **Win B's ≥ 80 % assumed monotone guards dominate.** At root scale
  they are **11 %** of guard evaluations on zebra2 and **30 %** on zebra,
  and the reason is structural: a candidate that is still parked has a
  guard that *failed*, and a failing monotone guard is retired on the
  spot, so every re-judged candidate's failing guard is non-monotone —
  a `forall`, which the mechanism excludes by name.
  [Q-M1a.17](../open_questions.md#q-m1a17--win-bs-80--assumed-monotone-guards-dominate)
  carries the numbers, the two refinements that *did* land (a per-round
  `(guard, projected env) → verdict` memo, and an allocation-free watch
  stamp on an ordered parked set instead of a pop-and-re-push heap), and
  the trigger for deciding it with an exhaustive profile at
  [P1a.6](../p1a.6_performance/README.md).

### What is not yet checked, and why

- **`ein.rs saturate` byte-parity.** S1a.3.3's acceptance called this
  "small enough to close early"; `ein.py/src/ein/cli/saturate.py` is a
  700-line snapshot renderer — an entity census, per-relation tables,
  provenance-kind counters, a firing breakdown and a `--dump` of the
  whole KB — sitting behind an `argparse` surface whose own parity is
  [Q-M1a.13](../open_questions.md#q-m1a13--argparse-surface-parity),
  open and marked blocking P1a.5 at the time (resolved 2026-08-18 —
  `clap`, content-parity). It moves there whole rather than
  landing half a CLI here. What the phase *does* pin is everything that
  renderer reads: the saturated KB, every firing, and every counter.
- **The compile-call count on an **exhaustive** zebra2** (17 430 → ~170)
  and Win B's exhaustive mix. Both need a hypothesis search, which is
  [P1a.4](../p1a.4_search_layer/README.md). The root figures are in, the
  memo that makes the first one true is in, and the instrument that
  answers the second (`guard_evals` / `guard_evals_monotone`) is in.
- **`Engine::step` / `Engine::saturate`** — the simpler pre-`Saturator`
  driver. Its only consumers are ein.py's own tests and
  `naf_dependency_map`; nothing in the port's path calls it, so it lands
  with the first thing that needs it rather than as unexercised code.

## Risks, in hindsight

- **"Order drift is invisible until it is expensive"** — the T2 diff ran
  from the first working saturation, as the risk asked, and it earned
  that: the first run reported exactly one difference (`n_guards` counts
  disjuncts, not guarded ones) against 23 000 events that already
  matched.
- **"The two exact wins are only exact if argued"** — Win A shipped with
  the argument and the diff. Win B's argument did not survive its own
  measurement, which is the outcome the risk was written to make visible
  rather than one it failed to prevent.

## Cross-links

- [design/05 — Matcher](../design/05_matcher.md)
- [design/06 — Saturation](../design/06_saturation.md)
- [`absent_semantics.md`](../../../docs/kernel/inference/absent_semantics.md)
- [`architecture_and_algorithms.md` §O1–O3, §O5](../../../docs/kernel/inference/architecture_and_algorithms.md)
- [Q-M1a.17](../open_questions.md#q-m1a17--win-bs-80--assumed-monotone-guards-dominate)
  — Win B's target, re-measured
- [D1](../divergences.md#d1--a-rule-may-not-bind-more-than-256-variables)
  — the register-file ceiling
