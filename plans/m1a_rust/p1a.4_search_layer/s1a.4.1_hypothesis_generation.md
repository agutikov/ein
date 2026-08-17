# S1a.4.1 — Hypothesis generation

**Phase:** P1a.4 (Search layer)
**Estimate:** 4 days
**Depends on:** [P1a.3](../p1a.3_deductive_core/README.md)
**Implements:** `ein/inference/{hypgen,hrule,solution}.py`,
[design/07](../design/07_search_layer.md) §2

## Context

The enumerator that proposes what to guess. Two modes — rule-driven when
the puzzle declares any `(hrule …)`, blind combinatorial otherwise, and
hrule presence *is* the switch — followed by an eight-stage filter
pipeline whose **attribution** is a T1 observable through `HypGenStats`.

The kernel imposes no type system here (S1.7.23): the enumerator
proposes type-blind and the puzzle's own rules do the pruning. Any Rust
code that reaches for `is-a` is reintroducing the thing that stage
removed.

Scale: ~30 candidate objects × ~10 signatured relations × 2 slots × ~30
fillers ≈ 18 k raw candidates per full call on zebra2, almost all
dropped by two bit tests.

## Acceptance

- `HypGenStats` identical for every corpus entry: `raw`, `emitted`,
  every `filtered.*` key and every `pre_candidate.*` key — verified
  through the `hyp` events and through `--hyp-stats` output.
- The invariant `raw == emitted + sum(filtered.values())` holds on both
  sides for every run.
- Candidate **order** identical (it decides `layer_1`'s singleton order
  and therefore the whole traversal).
- `complete(kb)` short-circuits on the first candidate, and `--stats`'
  `saturate_count` / entering counts are unchanged by that.
- `open_hypotheses` returns the same set on every fork.

## Tasks

### Task T1a.4.1.1 — Candidate objects

`_candidate_objects`: iterate `sorted(kb.names)` (string order — sort by
the interner's rank table, not by `Symbol` id), keep `category ==
"object"`, minus the **type-role atoms** (every name appearing in any
declared relation signature) and minus
`primitives.non_object_names()` (the structural vocabulary `not`/`false`/
`and`/`or`/`absent` plus the `eq`/`neq` predicates — these can appear as
a fact *head*, e.g. a `(not h)` written by the kill cache during
generation, but are never puzzle objects).

Hoist the sorted list to once per `_generate` call, with the safety
argument written down: the only mid-call mutation is `_write_negated`'s
`(not h)`, which bumps the name `not`, which is filtered by `reserved`
either way. Pin it with a fixture that has the kill cache **on**.

### Task T1a.4.1.2 — Ordering

`sorted(objects, key=(-(len(as_head)+len(as_arg)), name))` — descending
participation, ties by name, **stable**. Use a stable sort; the key
already includes the name so the stability is belt-and-braces, but the
port should not depend on that.

### Task T1a.4.1.3 — Raw enumeration

`_raw_candidates`: iterate `kb.relations.values()` in **insertion
order**, skip relations with an empty signature, then the three
pre-candidate skips in order — `closed_relation` (`(__closed__ R)`),
`relation_not_whitelisted` (`(query :hypothesis-relations …)`),
`no_hypothesis_relation` (`(query :no-hypothesis …)`) — then `slot_idx`
ascending.

`_fill_slot`: arity 1 → exactly one candidate `(R obj)` with no filler
loop and no self-edge check (S1.22.4); arity 2 → every candidate object
except the focal one (bumping `self_edge`), placed by
`_build_args(focal, fixed_slot, filler, other_slot)`; arity ≥ 3 →
unenumerated. **No symmetric mirror** (S1.7.24): both orderings arise
via different focal objects.

### Task T1a.4.1.4 — The filter pipeline

`_apply_filters` in this exact order, because the order decides which
counter a drop lands in:

1. `negated_fact` — `(rn, args) ∈ kb._negated_facts` → a bitset test on
   the interned `FactId` ([design/03](../design/03_data_model.md) §6);
2. `fact_already_exists` — presence bit;
3. `lookahead_killed` — the one-step simulation
   ([S1a.4.2](s1a.4.2_lookahead_and_closure.md)), plus the `(not h)`
   kill-cache write when `enable_lookahead_kill_cache`;
4. `seen_in_call` — same-call dedup by `FactId`.

Intern-on-probe: compute the candidate's row key and run steps 1–2
against it *before* materialising anything, so a rejected candidate costs
two bit tests and no allocation.

### Task T1a.4.1.5 — Query-scoped sets

`_query_relations` (`:hypothesis-relations`, `None` ⇒ unrestricted — and
note the `or None` that turns an empty list back into "unrestricted"),
`_no_hypothesis_relations` (`:no-hypothesis`), `_coerce_relation_names`
(a bare SYMBOL or an `(r1 r2 …)` list, reading head + atom args).

### Task T1a.4.1.6 — hrule mode

`hrule._hrule_activators` + `Hrules.candidates`: run each hrule's plans
and yield the conclusions as candidates. Activator arity filtering
applies here too (S1.22.0). The blind enumerator does **not** run when
hrules exist.

### Task T1a.4.1.7 — Scoring

`score_hypothesis` dispatching on `hypgen_scoring`:
`"most-constrained"` → `0.0`; `"popularity"` → `rel_w * |extent(R)| +
obj_w * Σ |names[arg].as_arg|` over string args only;
`"branch-info"` / `"popularity+branch-info"` → the same
`NotImplementedError` text; anything else → the same `ValueError`.

### Task T1a.4.1.8 — Solution predicates

`solution.py`: `open_hypotheses` (materialise), `complete` (**first
candidate only** — S1.9.E16; 8 of 9 `complete` calls on a zebra2 fast
solve are answered by candidate #1), `consistent`, `is_solution_node`.
Keep `complete` and `open_hypotheses` distinct; collapsing them is a
measurable regression, not a simplification.

## Notes

- `generate_hypotheses` is **not pure** when the kill cache is on: it
  writes `(not h)` facts that later candidates in the same call observe.
  That feed-forward is why [design/08](../design/08_parallelism.md) §7
  refuses to parallelise this pipeline.
- The `_candidate_objects` generator is called *inside* `_fill_slot`, so
  ein.py re-sorts `kb.names` per (object, relation, slot). That is the
  single largest constant factor here and T1a.4.1.1 removes it — but
  only after the equivalence argument, not before.
