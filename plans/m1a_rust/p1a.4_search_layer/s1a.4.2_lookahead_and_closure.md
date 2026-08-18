# S1a.4.2 — Lookahead, closure marking, NAF dependency map

**Phase:** P1a.4 (Search layer)
**Estimate:** 3 days
**Depends on:** [S1a.4.1](s1a.4.1_hypothesis_generation.md)
**Implements:** `ein/inference/{lookahead,closed,naf_deps}.py`

> **Two thirds of this stage landed with
> [S1a.4.1](s1a.4.1_hypothesis_generation.md), 2026-08-18.** The filter
> pipeline's own acceptance — "`HypGenStats` identical for every corpus
> entry, every `filtered.*` key and every `pre_candidate.*` key" — is not
> checkable without them: `enable_pre_branch_lookahead` defaults to
> **true**, and on the corpus the lookahead accounts for **547 of 4 479**
> raw candidates. A stage cannot meet its acceptance by leaving out the
> filter that decides an eighth of it, so `lookahead.rs` (T1a.4.2.1–4) and
> hypgen's own `_is_closed` reader (T1a.4.1.3) came forward, exactly as the
> NAF boundary came forward to
> [S1a.3.3](../p1a.3_deductive_core/s1a.3.3_saturator.md). Both are T2-green
> through the `hyp-shape` diff, and both carry the fixtures the mutation
> tests said the corpus was missing (below).
>
> **What is left here:** `emit_closed` / `producible_relations` — the
> *producer* of `(__closed__ R)`, which is hypgen's input and not hypgen's
> code — `naf_deps` whole, a second `hyp-shape` regime that runs it, and
> the lever fixtures.
>
> One thing to know before porting it: **`emit_closed` is not on the
> `solve` path.** Both call sites (`cli/solve.py`'s `--hyp-stats` preview
> and `cli/_summary.py`'s root observables) run it on a **fork**, so the
> search itself sees every relation open. That is why S1a.4.1's instrument
> does not run it and is nonetheless comparing what `solve` will compare —
> and it is also why the regime matters, because it moves a lot: with
> `emit_closed` the corpus totals go `closed_relation` 6 → 278,
> `no_hypothesis_relation` 36 → 0, `lookahead_killed` 547 → 279 and `raw`
> 4 479 → 3 022.

## Context

Three small modules that decide *what the enumerator is allowed to
propose*, and one that only warns.

- **Lookahead** (S1.5.6) simulates one rule step against the saturated KB
  and kills a candidate that dies immediately — no fork, no saturation.
  It is the costliest per-candidate filter, and on exhaustive zebra2 it
  currently measures *slightly negative* (0.9×) because fail-fast made
  the forks it avoids cheap. Its sign may flip once matching is 30×
  cheaper; that is a [P1a.6](../p1a.6_performance/README.md)
  measurement, not a decision to take here.
- **`closed`** marks a relation as fully populated (`(__closed__ R)`), so
  hypgen contributes zero candidates for it. Emitted by
  `emit_closed`, authored directly, or derived by `std.closure`'s
  `infer-closure` — the generator does not care which.
- **`naf_deps`** is advisory only: it reports rules whose `(absent …)`
  guards watch a rule-derived relation. Since S1.21.8 that is a
  *stratification* signal, not a soundness one.

## Acceptance

- `filtered.lookahead_killed` counts identical on every corpus entry,
  with and without `enable_lookahead_kill_cache`.
- The `(not h)` kill-cache writes identical in content and order
  (they are root-visible facts and they change later filtering).
- `producible_relations` / `emit_closed` produce the same
  `(__closed__ R)` set in the same order.
- `compute_naf_map` returns the same `NafDep` records with the same
  sorted `derived` / `declared_only` tuples; `warn_derived_naf=true`
  emits the same `DerivedNafWarning` messages (the suite runs under
  `filterwarnings=["error"]`, so text matters).
- `examples/branching/{06,07,10,11}` (lookahead on/off, kill cache
  on/off) reproduce their T2 event traces.
- **Already met at S1a.4.1** for everything but the kill-cache lever: the
  `hyp` stream and the stats block agree on all 66 loadable corpus files
  (4 489 candidates), `13_lookahead_naf_world.ein` and
  `14_lookahead_unjudgeable.ein` pin the two D3 halves, and the kill-cache
  writes are compared as the `negated_fact` verdicts they cause on later
  candidates in the same call.

## Tasks

### Task T1a.4.2.1 — `Lookahead` — **landed at S1a.4.1**

Build once per `generate_hypotheses` call (it compiles plans) and reuse
per candidate. `dies_immediately(kb, h)`: for each plan whose premises
can involve `h`, run the match with `h` present and check whether any
conclusion contradicts the KB.

Port the two skip guards exactly: `_unjudgeable(guards)` — a candidate
whose plan carries guards that cannot be judged pre-fork is not killed —
and `_has_nested_absent`. A lookahead that kills too eagerly is
*incomplete*, and incompleteness here is silent.

### Task T1a.4.2.2 — Guard evaluation in the simulation — **landed at S1a.4.1**

`_guards_pass_with(...)` evaluates the plan's `NafGuard`s against the
hypothetical KB. Reuse `World` ([S1a.3.4](../p1a.3_deductive_core/s1a.3.4_world_and_contradiction.md))
rather than a second implementation — the simulation asks the same
question of a different world.

### Task T1a.4.2.3 — `_is_contradiction` — **landed at S1a.4.1**

Whether a derived fact `f` clashes given the hypothetical `h`. Shares
shape with `contradiction.contradicts` but must keep its own
hypothetical-KB semantics; diff the two carefully rather than merging
them.

### Task T1a.4.2.4 — Kill cache — **landed at S1a.4.1**

`_write_negated(kb, h)` — idempotent `(not h)` with
`Provenance.from_rule("<lookahead-dies-immediately>")` and **empty
premises**, which is a reserved engine string whose contract is that
provenance walks ground out on it
([reserved_engine_strings.md](../../../docs/kernel/inference/reserved_engine_strings.md)).
Gated by `enable_lookahead_kill_cache`.

### Task T1a.4.2.5 — `closed` — **half landed at S1a.4.1**

`CLOSED = "__closed__"` and `hypgen._is_closed` — which reads
`_facts_by_relation[CLOSED]` and matches `args == (r_name,)` — are
hypgen's and came with it. What is left is the *producer*:
`producible_relations(kb)` walking the engine cache's
`asserted_relation`s, and `emit_closed(kb)` writing the markers and
returning the list. Note where it is called from — `cli/solve.py` and
`cli/_summary.py`, **not** `solve()` — so the second `hyp-shape` regime is
what compares it before P1a.5 has a CLI.

### Task T1a.4.2.6 — `naf_deps`

`NafDep` records, `_producible` / `_negated_producible` over the cache,
`compute_naf_map`, `emit_derived_naf_warnings`, `_label(rel, negated)`.
Emitted once per solve post-root-saturation (so the cache holds
rule-derived activators' plans), gated by `warn_derived_naf`. In Rust a
"warning" is a callback the CLI turns into a stderr line with ein.py's
exact text.

## Notes

- `Lookahead` compiles plans on construction, so under
  [design/06](../design/06_saturation.md) § Win A it should reuse the
  shared plan memo rather than building a private engine.
- Re-measure the lookahead levers in
  [S1a.6.7](../p1a.6_performance/s1a.6.7_relever_matrix.md) and record
  the new table before proposing any default change.
