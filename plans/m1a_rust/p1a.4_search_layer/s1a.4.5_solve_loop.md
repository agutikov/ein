# S1a.4.5 — The solve loop and verdict synthesis

**Phase:** P1a.4 (Search layer)
**Status:** **shipped** 2026-08-18 — acceptance below, with two items that
[P1a.5](../p1a.5_presentation/README.md) is the first phase able to run.
**Estimate:** 4 days
**Depends on:** [S1a.4.4](s1a.4.4_commitment_primitive.md)
**Implements:** `ein/inference/monotonic/**`, `ein/inference/{canon,verdict}.py`,
[design/07](../design/07_search_layer.md) §§1, 6, 8

## Context

The three-phase loop, and the rule that makes it P1.7a rather than its
unsound predecessors: **the verdict is read from the result, never chosen
up front.** `k` distinct solution nodes → Contradiction / Solution /
Ambiguity. A solution node is `consistent ∧ complete`, *not* a
goal-pattern match — the distinction S1.7.3 found the hard way, when a
partial dead-end was being accepted as a solution.

Root stays stable throughout (P1.21 R2): no fork fact is merged back,
because the retired "unconditional fact" extraction was unsound under
NAF — a fact derived via `absent X` leaves no provenance edge to the
commitment that suppressed `X`.

## Acceptance

The `solve-shape` diff runs one whole solve per corpus entry in three
regimes, all budgeted so no file can run away:

| regime | what it is | result |
|---|---|---|
| `fast` | `stop_after = 1`, cap 300 — what `ein solve` does | **65 files, 5 174 enterings, 0 differences** |
| `exhaustive` | `stop_after = None`, cap 60 | **65 files, 1 618 enterings, 0 differences** |
| `shuffled` | `lattice_order_seed = 7` — Q-M1a.5 | **65 files, 5 207 enterings, 0 differences** |

| item | result |
|---|---|
| Verdict, `k`, `exhausted`, every counter | the `VERDICT` and `STATS` lines |
| The `enter` / `nogood` / `writeback` sequence | in the text, **with its `n`** — which counts every event including the ~58 000 saturator ones the text filters out, so agreeing on the printed 19 means agreeing on all of them. It is what caught three sub-runs the port had quietly silenced |
| `stop_after` cuts at the same candidate; `exhausted` false for the same reasons | the `fast` regime against the `exhaustive` one |
| `Aborted(reason, stats)` under `on_budget="verdict"` | `zebra2-minus-15` exhausts its cap on both sides and the `ABORT` line carries the same reason |
| `store_lattice=true` — solutions, dead commitments, learned clauses, per-fact cores | the `PROOF` block, per record. This is where the last real bug was: `record_node`'s lex tie-break was sorting by `FactId` rather than by content, so a model two commitment paths both reach got a different representative. Only the *exhaustive* regime has two paths to compete |
| `BudgetExceededError` message under `on_budget="raise"` | **not compared** — the message is the same string the `Aborted` reason carries, and both regimes take `"verdict"` so a sweep never aborts the harness. The raise path is one `match` arm above it |
| `lattice_sanity_check=true` / `check_commutativity` | **moves to [P1a.5](../p1a.5_presentation/README.md)** — see below |

### What is not here

- **`sanity.check_commutativity` and `SanityError`.** A release-regression
  check that costs `k+1` saturations per checked commitment and is off by
  default. It has no bearing on any shipping verdict, and no corpus entry
  turns it on.
- **`goal_bindings` / `is_solved` / `query_value`** (T1a.4.5.7's tail) and
  **`lattice_snapshot` / `_serialise` / `validate_proof_for_explanation`**
  (T1a.4.5.8's). Their only consumers are the CLI's answer table and the
  `--dump-states` writer, both [P1a.5](../p1a.5_presentation/README.md)'s,
  and `solve` itself calls none of them: the cascade's `check_goal` is
  always `False` on this path. Porting them here would land unexercised
  code ahead of the diff that could check it.
- **The dumper implementations.** The trait and its six call sites are
  here, as T1a.4.5.9 asks; the implementations are S1a.5.3's.

## Tasks

### Task T1a.4.5.1 — Loop state

`_LoopCtx` (root, cfg, stats, lstate, dumper, store_lattice, timing,
budgets, `max_set_size`, `stop_after`, shuffle rng) and
`_LatticeLoopState` (solution nodes keyed by `state_key`, dead
commitments, `alive_at_end_tuple`, `truncated`, `state_key_merges`,
`kb_index`). `MonotonicStats` / `_BaseStats` with **every** counter, and
`_build_lattice_stats` copying the base counters generically off the
dataclass field list (so a new counter cannot go uncopied).

### Task T1a.4.5.2 — Phase 1: root

Saturate root (streaming firing-count progress to the dumper every 50
firings under `-v`, and taking the fast drain path when no dumper is
attached — the two paths must produce the same firings);
`warn_derived_naf` emission reusing the saturator's populated cache;
`root_initial` hook; root contradiction → `_root_dead` → `k=0` with the
source-frontier core; `_compute_alive`; the forced-positive cascade;
empty alive ∧ consistent → root *is* the unique model.

### Task T1a.4.5.3 — The forced-positive cascade

`_promote_forced_positives`: while `alive` is a singleton `{h}`, promote
`h` to a root fact with `Provenance.from_rule("<forced-positive>",
premises_raw=())` — a reserved engine string whose empty premises make
provenance walks ground out — re-saturate, bump `saturate_count` and
`facts_merged` / `forced_positives`, check contradiction (and
`is_solved` when `check_goal`), recompute alive, repeat. Gated by
`enable_forced_positive`. Note it never fires on zebra2, so the fixtures
that exercise it are the small branching ones — make sure the corpus has
one that does.

### Task T1a.4.5.4 — Phase 2: layers

For `layer` in `1..=max_set_size`: `layer_start` hook; candidates
(`layer_1` or `generate_layer`); `order_candidates`; the optional
per-layer shuffle (one `random.Random` per solve whose state advances
across layers); then per candidate: budget check, `enterings_total += 1`,
`try_commitment_set`, and the branch:

- dead → `_handle_dead` (nogood emit + subsumption counters + the
  singleton `(not h)` writeback gated by `enable_singleton_writeback` +
  the `entering` hook with its outcome flags);
- alive ∧ `complete(result.kb)` → `_record_node` (deduped by
  `state_key`), the `stop_after` check, **no expansion**;
- alive ∧ incomplete → append to the next layer's frontier.

Then the inter-layer step: recompute `alive`, run the cascade, drop
commitments no longer entirely within `alive`, and at `layer ==
max_set_size` record `alive_at_end_tuple` and set `truncated`.

Use `complete(result.kb)` directly rather than `is_solution_node` —
consistency is already established on an alive branch (F-ENG-12), and
re-running `detect()` there is both wasted and a counter difference.

### Task T1a.4.5.5 — Writebacks

`_emit_negated_fact_writeback` / `_write_negation_local`: a singleton
dead clause writes `(not h)` at root — a flat root write with no
ancestor-chain coupling, and **no symmetric mirror** (S1.7.24: the
counterpart dies on its own branch and is recovered at the `state_key`
dedup). This is the one intra-layer root mutation and it is exactly what
[design/08](../design/08_parallelism.md) §2 has to validate against.

### Task T1a.4.5.6 — Canonicalisation

`state_key(kb)` — the sorted canonical fact list, **the representation,
never a hash** (P1.21 R1). Sort by `FactId` for identity (equivalent for
identity purposes — [design/02](../design/02_determinism_and_order.md)
§6) with an incrementally-maintained order-insensitive 128-bit digest as
a pre-filter, and the `python_repr` order reserved for the display sites.
`state_digest` stays display-only.

### Task T1a.4.5.7 — Phase 3 and verdicts

`verdict_of(lstate, exhausted)`; `Solution` / `Ambiguity` /
`Contradiction` with the optional `LatticeProof`; `Aborted` deliberately
**outside** the `Verdict` union. `goal_bindings(kb, goal=None)` — build
the synthetic `<query>` plan from the `:goal` pattern and return binding
rows; `is_solved(kb, mode)`; `query_value`.

### Task T1a.4.5.8 — Proof packaging

`SolutionRecord` / `DeadCommitment` / `SetNode` / `LatticeProof` /
`LatticeStats`, `_record_setnode` (with its
`tuple(sorted(commitment)) < tuple(sorted(cur.commitment))`
tie-break), `_solve_proof`, `lattice_snapshot` / `LatticeSnapshotV1`,
`sanity.check_commutativity` + `SanityError.__str__`,
`contract.validate_proof_for_explanation`.

### Task T1a.4.5.9 — The dumper trait

The six hooks (`root_saturating`, `root_initial`, `layer_start`,
`entering`, `layer_end`, `summary`) plus `close`, with the `_finish`
guarantee that `summary` lands on every non-abort path and that a budget
abort flushes the timeline but writes no `summary.json`. Implementations
land in [S1a.5.3](../p1a.5_presentation/s1a.5.3_state_dumps.md).

## Notes

- `_phase2_layers`' `phase_2_done` flag and the two `break`s that read it
  look redundant; port the control flow literally and only simplify once
  T2 is green across every branching fixture.
- The shuffle rng is one `random.Random` per solve whose state advances
  across layers — not one per layer. Q-M1a.5, **resolved here as (a)**:
  `mt19937.rs` is CPython's generator, checked by table and then on every
  corpus entry through the `shuffled` regime. The traversal differs from
  the unshuffled one on 9 of the 14 `examples/branching` files, so the
  agreement is earned rather than vacuous.
