# 07 — The search layer: hypotheses, commitments, verdicts

**Settles:** hypothesis generation, the commitment lattice, no-good
learning, and verdict synthesis — §O6, §O7, §O8, §O9.
**Phase:** [P1a.4](../README.md#p1a4--search-layer).
**Replaces:** `ein/inference/{hypgen,hrule,lookahead,closed,apriori,
nogoods,commitment,solution,canon,verdict,frontier,explain}.py` and
`ein/inference/monotonic/**`.

---

## 1. The shape, unchanged

`solve()` is the one entry, and its verdict is *read from the result*
(`k` distinct solution nodes → Contradiction / Solution / Ambiguity),
never chosen up front. Three phases:

- **Phase 1 — root.** Saturate root; if contradictory → `k=0` with the
  source-frontier core. Compute `alive` (= open hypotheses); run the
  forced-positive cascade; if `alive` is empty and consistent, root
  itself is the unique model.
- **Phase 2 — layers.** For `layer` in `1..=max_set_size`: generate
  candidates (layer 1 = singletons of `alive`; layer *k* = Apriori
  prefix-join filtered by `alive` and the no-good store), order them
  (`lex` or `score-sum`, optional `--shuffle`), and enter each via
  `try_commitment_set`. Dead → learn a no-good (+ singleton `(not h)`
  writeback). Alive ∧ `complete` → record a solution node, deduped by
  `state_key`. Alive ∧ incomplete → expand to the next layer. Between
  layers: recompute `alive`, run the cascade, drop commitments that left
  `alive`.
- **Phase 3 — verdict.** `verdict_of(k, exhausted)`.

**Root stays stable** — no fork fact is ever merged back (P1.21 R2; the
retired extraction was unsound under NAF). The only root writes during
Phase 2 are the singleton `(not h)` writeback and forced-positive
promotions, both sound and both flagged by config.

Every one of these is a port target, not a redesign target. What changes
is the constant factor and, in [P1a.7](../README.md#p1a7--parallelism),
the parallelism.

---

## 2. Hypothesis generation

Two modes, and hrule presence *is* the switch:

- **hrule-driven** (`kb.hrules` non-empty) — `Hrules(kb).candidates(kb)`
  runs each hrule's match and yields its conclusions.
- **blind enumeration** — objects ordered by descending participation
  (ties by name), and for each object, every declared relation with a
  signature, every slot, every other candidate object as filler.

Then a filter pipeline, whose *attribution* is observable through
`HypGenStats`:

| stage | check | ein.rs |
|---|---|---|
| pre: `closed_relation` | `(__closed__ R)` present | a `Symbol` bitset built once per call |
| pre: `relation_not_whitelisted` | `(query :hypothesis-relations …)` | `Symbol` bitset |
| pre: `no_hypothesis_relation` | `(query :no-hypothesis …)` | `Symbol` bitset |
| pre: `self_edge` | filler == focal object | `u32` compare |
| `negated_fact` | `(not h)` present | **one bit test** on `negated` ([03](03_data_model.md) §6) |
| `fact_already_exists` | `h` present | **one bit test** on `present` |
| `lookahead_killed` | one-step rule simulation | the matcher ([05](05_matcher.md)) |
| `seen_in_call` | same-call dedup | `FxHashSet<FactId>` |

Order matters only for counter attribution, and the counters are a T1
observable — so the pipeline order is fixed.

Scale on zebra2: ~30 candidate objects × ~10 signatured relations × 2
slots × ~30 fillers ≈ 18 k raw candidates per full call, nearly all
dropped by the two bit tests. In Python each of those is a tuple build, a
`Fact` construction and two hash lookups; in Rust it is an intern +
two bit tests, and the `Fact` never gets built for a rejected candidate
(intern-on-demand: compute the row key, probe, and only materialise on
survival). Expect two orders of magnitude.

**Measured at [S1a.6.4](../README.md#s1a64--hypgen-and-lattice-hot-paths),
and the 18 k is the *blind* path's number.** zebra2 declares an `(hrule …)`,
so it never runs that arm: a pass offers **125** raw candidates on zebra and
zebra2, 336 on `terminus`, 120 on `features/05`. The intern-on-demand split is
also moot here rather than won — `FactStore::intern` *is* the probe, plus a
push on a miss, so splitting it buys a branch and costs the caller a second
lookup; it is 0.69 % of the heaviest blind-mode run. What the calls do spend is
**setup**: 71 % of a `complete()` on zebra2 was building a fresh `Lookahead`,
whose `compile_all` walks `rules × activators` — 219 compile-cache keys per
call against those 125 candidates. That is where the stage went, and the row
above ("a `Symbol` bitset built once per call") became true there rather than
before it.

`complete(kb)` — "does the generator propose anything?" — short-circuits
on the first candidate (S1.9.E16); keep the short-circuit, it is
load-bearing for cost, and keep it distinct from `open_hypotheses`, which
materialises.

### Two traps

- **`_candidate_objects` re-sorts `kb.names` on every call**, and it is
  called once per `(object, relation, slot)`. Hoisting the sorted list to
  once per `_generate` call is a pure win *provided* the yielded sequence
  cannot change mid-call. It can only change via `_write_negated`'s
  `(not h)` insertion, which bumps the name `not` — and `not` is in
  `primitives.non_object_names()`, so it is filtered out either way.
  Hoist, and pin the equivalence with a fixture that enables the kill
  cache.
- **Generation mutates the KB.** `enable_lookahead_kill_cache` makes
  `_apply_filters` write `(not h)` for a lookahead-killed candidate,
  which later candidates then see through `negated_fact`. That is a
  feed-forward dependency inside one call — relevant to
  [08](08_parallelism.md), which must therefore not parallelise the
  filter pipeline naively.

---

## 3. Lookahead

`Lookahead.dies_immediately(kb, h)` simulates one rule step against the
saturated KB: for each plan whose premises can involve `h`, run the match
with `h` present and check whether any conclusion contradicts. It is the
costliest per-candidate filter and it is the reason
`enable_pre_branch_lookahead` measures *slightly negative* on exhaustive
zebra2 today (0.9×) — it pays a simulation to avoid forks that fail-fast
already made cheap.

Port considerations:

- The `_unjudgeable` / `_has_nested_absent` guards (a plan whose guards
  cannot be judged pre-fork is skipped) are semantics; port them.
- The simulation writes nothing except the kill-cache `(not h)`.
- Because it runs the matcher, it inherits [05](05_matcher.md)'s speedup
  wholesale — which may flip the lever's sign back to positive. Re-run
  [`features.md`](../../../kernel/inference/features.md)'s matrix
  against ein.rs in [P1a.6](../README.md#p1a6--performance) and record the
  new table; do **not** change the default without that measurement.

---

## 4. Apriori candidate generation and no-goods

`CanonicalSetId` is a sorted tuple of `FactId`s ([03](03_data_model.md)
§4). With integer ids:

| operation | ein.py | ein.rs |
|---|---|---|
| `canonicalise` | `tuple(sorted(set(...)))` | sort + dedup a `SmallVec<[FactId; 4]>` |
| `apriori_prefix_join` | sorted list, pairwise, `break` on prefix mismatch | identical loop over `u32` slices |
| `filter_candidate` — alive check | `all(h in alive)` over a `frozenset` | bitset tests |
| `filter_candidate` — no-good check | `for clause in nogoods: clause.issubset(cand)` | see below |
| `order_candidates` `lex` | `sorted(candidates)` | **semantic** comparator ([02](02_determinism_and_order.md) §5 H2) |
| `order_candidates` `score-sum` | `sorted(key=(-score, c))`, stable | same, stable |

**No-good storage.** The clause set is kept subsumption-minimal on emit.
For a lattice whose alive set is ≤ 64 elements — zebra2's is — a clause
is a `u64` bitmask over an alive-index, and the subset test is
`clause & cand == clause`: one instruction instead of a Python set
operation. Above 64, fall back to a sorted `Box<[FactId]>` with a
merge-intersection. The mask index is rebuilt when `alive` changes, which
happens once per layer.

This matters more than zebra2 suggests: with
`enable_singleton_writeback` off, the exhaustive search explodes to
3 336+ enterings and a correspondingly large clause set
([`features.md`](../../../kernel/inference/features.md)), and that
is the regime where clause checking dominates.

> **It does not — measured in exactly that regime at
> [S1a.6.4](../README.md#s1a64--hypgen-and-lattice-hot-paths) (T1a.6.4.4),
> and the mask is not built.** zebra2 with `:enable-singleton-writeback false`
> explodes as predicted — **3 831 enterings, 2.38 s** — but into **354**
> clauses, because the store is subsumption-minimal on emit, and the whole
> apriori/no-good machinery is **0.3 %** of the run: `filter_candidate` 0.3 %,
> `nogood` and `is_subset` 0.0 %. `admit_from_boundary` is 60.2 %. The
> fall-back representation is therefore the only one, and the `u64` arm stays
> unwritten until a workload makes the subset test cost something.

**Do not** re-litigate the search-layer optimisations
[F9](../../../../plans/followups/f9_e_catalog.md) already measured and rejected
(reorderers, consistency pre-passes, cross-call conflict caches). The
ledger's conclusion — they are inert against a complete cardinality-BFS —
is about the *algorithm*, and rewriting it in Rust does not change the
arithmetic.

---

## 5. `try_commitment_set`

The primitive both the loop and the sanity checker call:

```
fork = root.fork()                     // O(1) in ein.rs — 03 §5
write each hypothesis with Provenance::hypothesis(branch=0)
pre-detect  → dead-pre  (+ smallest_contradiction_frontier)
saturate    → fail-fast at the firing that kills it (S1.9.E23)
post-detect → dead-post (+ frontier)
else        → alive
```

`enable_fail_fast_fork` is the one pure speed lever (1.9–2.4× on
exhaustive zebra2, verdict untouched) and it stays on. Note its
documented consequence for the *off* case — a dead fork's `firings` is
the full run and its `kb` the complete dead state — because a DAG builder
that merges dead commitments by `state_key` needs the fixpoint. Both
behaviours port.

`Saturator::new(fork)` no longer rebuilds and recompiles an engine
([06](06_saturation.md) § Win A): the plan memo is shared, so a fork pays
only for plans its own derived activators introduce.

---

## 6. Canonicalisation and dedup

`state_key(kb)` is the sorted canonical fact list — **the representation,
never a hash** (P1.21 R1). In ein.rs it is a sorted `Box<[FactId]>`:

- **identity** — sorted by `FactId` (a `u32` sort, and `memcmp`
  equality). Any total order is equivalent for identity
  ([02](02_determinism_and_order.md) §6), so this is free.
- **display** — `--dump-states` sorts nodes by `repr(state_key)`, which
  needs the `python_repr` renderer ([02](02_determinism_and_order.md) §7).
- **acceleration** — an order-insensitive 128-bit rolling digest (XOR of
  per-`FactId` hashes) maintained incrementally as facts are added, used
  to pre-filter the solution-node dedup map. The full sorted vector
  remains the key; the digest only avoids building it. This keeps the
  P1.21 R1 rule intact.

`state_digest` (Python's `hash()`) is display-only and *not* stable
across CPython runs — see [02](02_determinism_and_order.md) §8 for the
T3 consequence.

---

## 7. Provenance walks: cores, frontiers, explanations

Three related searches, all over the AND/OR justification graph:

| function | question | note |
|---|---|---|
| `kb.unsat_core(conflicting)` | the union of source-frontier terminals reachable from the conflicts | primary-justification only by default; `all_justifications=True` gives the (larger) soundness envelope |
| `frontier.smallest_contradiction_frontier` | the *minimum-cardinality* frontier of **one** witness across all recorded derivations | what a Contradiction verdict reports |
| `explain.explain` | a minimal explanation, budgeted | least fixpoint from the sources up, so a cyclic justification graph (the symmetric mirror makes one) never grounds a fact in itself |

Port notes:

- `walk_premises` is the shared closure walk; in ein.rs it is a BFS over
  `FactId` with a `BitSet` visited set instead of a Python `set` of
  `Fact`s — a large constant-factor win on the exhaustive path where the
  core is computed per dead commitment.
- The **tie-breaking is observable**: `explain` sorts environments with
  `key=repr` and `_recorded_fallback` keys on
  `" ".join(sorted(repr(f) for f in core))`. `python_repr` again.
- `ExplanationBudget` and the AND/OR minimisation/fold/propagate
  structure port shape-for-shape; this is the part of the engine where a
  "cleaner" rewrite would most easily change which explanation is
  returned, so it is the part to port most literally.

---

## 8. Config, budgets, verdicts

- `SolverConfig` becomes a `#[derive(Clone, Copy)]` struct with the same
  **field order** (it is printed by `--dump-config`) and the same
  defaults. `from_kw_pairs` reproduces the kebab→snake mapping, the
  coercion rules (`true`/`false` symbols; bool-rejects-for-int), and the
  error messages including the sorted list of valid flags.
- Precedence: `solve(config=…)` > `kb.config` > defaults.
- Budgets: `max_time` (wall clock) and `max_enterings`;
  `on_budget="raise"` → `BudgetExceededError`, `"verdict"` → `Aborted`.
  `Aborted` stays outside the `Verdict` union.
- `verdict_of(k, exhausted)`; `stats.exhausted` false when truncated by
  `stop_after` or by a non-empty frontier at the depth cap.
- The dumper hooks (`root_saturating`, `root_initial`, `layer_start`,
  `entering`, `layer_end`, `summary`, `close`) become a trait with the
  same five call sites and the same guarantee that `summary` lands on
  every non-abort path.

---

## 9. Acceptance for this design

- **T1 corpus-wide**: every counter in [01](01_parity_contract.md) §2
  identical on every corpus entry × run matrix cell.
- **T2** on `branching/**`, `lattice/**`, `domain_elim/**`: identical
  `enter` / `nogood` / `writeback` / `hyp` event sequences.
- The three acceptance fixtures (`test_zebra_two_ontologies`,
  `test_zebra_three_classes`, `test_mode_consistency`) pass against
  ein.rs with the same models.
- The [`features.md`](../../../kernel/inference/features.md) lever
  matrix regenerated against ein.rs: same verdicts, same entering counts,
  new wall-clock column. `enable_singleton_writeback` must still be the
  one load-bearing lever (if it is not, something is wrong).

## Cross-links

- [03 — Data model](03_data_model.md) §5 — the O(1) fork this layer
  leans on.
- [06 — Saturation](06_saturation.md) — the inner engine.
- [08 — Parallelism](08_parallelism.md) — the layer loop is the main
  parallel opportunity.
- [`algorithm_layer_n.md`](../../../kernel/inference/algorithm_layer_n.md)
  — the per-step contract.
- [F9 ledger](../../../../plans/followups/f9_e_catalog.md) — what not to try again.
