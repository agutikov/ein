# S1a.4.2 — Lookahead, closure marking, NAF dependency map

**Phase:** P1a.4 (Search layer)
**Estimate:** 3 days
**Depends on:** [S1a.4.1](s1a.4.1_hypothesis_generation.md)
**Implements:** `ein/inference/{lookahead,closed,naf_deps}.py`

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

## Tasks

### Task T1a.4.2.1 — `Lookahead`

Build once per `generate_hypotheses` call (it compiles plans) and reuse
per candidate. `dies_immediately(kb, h)`: for each plan whose premises
can involve `h`, run the match with `h` present and check whether any
conclusion contradicts the KB.

Port the two skip guards exactly: `_unjudgeable(guards)` — a candidate
whose plan carries guards that cannot be judged pre-fork is not killed —
and `_has_nested_absent`. A lookahead that kills too eagerly is
*incomplete*, and incompleteness here is silent.

### Task T1a.4.2.2 — Guard evaluation in the simulation

`_guards_pass_with(...)` evaluates the plan's `NafGuard`s against the
hypothetical KB. Reuse `World` ([S1a.3.4](../p1a.3_deductive_core/s1a.3.4_world_and_contradiction.md))
rather than a second implementation — the simulation asks the same
question of a different world.

### Task T1a.4.2.3 — `_is_contradiction`

Whether a derived fact `f` clashes given the hypothetical `h`. Shares
shape with `contradiction.contradicts` but must keep its own
hypothetical-KB semantics; diff the two carefully rather than merging
them.

### Task T1a.4.2.4 — Kill cache

`_write_negated(kb, h)` — idempotent `(not h)` with
`Provenance.from_rule("<lookahead-dies-immediately>")` and **empty
premises**, which is a reserved engine string whose contract is that
provenance walks ground out on it
([reserved_engine_strings.md](../../../docs/kernel/inference/reserved_engine_strings.md)).
Gated by `enable_lookahead_kill_cache`.

### Task T1a.4.2.5 — `closed`

`CLOSED = "__closed__"`. `producible_relations(kb)` walks the engine
cache's `asserted_relation`s; `emit_closed(kb)` writes the markers and
returns the list. `hypgen._is_closed` reads
`_facts_by_relation[CLOSED]` and matches `args == (r_name,)`.

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
