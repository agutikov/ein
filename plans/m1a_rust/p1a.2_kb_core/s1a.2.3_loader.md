# S1a.2.3 — The loader (`from_ir`)

**Phase:** P1a.2 (KB core)
**Estimate:** 3 days
**Depends on:** [S1a.2.2](s1a.2.2_store_and_indexes.md),
[S1a.1.3](../p1a.1_ir_frontend/s1a.1.3_macros_and_imports.md)
**Implements:** `ein/kb/from_ir.py`

## Context

The loader turns a resolved, import-flattened, macro-expanded form list
into a populated KB. It is a validation surface as much as a
construction one: it **accumulates** errors and raises one
`KBLoadError` with them `; `-joined, so message text *and order* are
observable. `examples/broken/load/` (extracted in
[S1a.0.1](../p1a.0_conformance_harness/s1a.0.1_parity_contract_and_corpus.md))
is the fixture set.

## Acceptance

- Byte-identical `KBLoadError` for every load-negative fixture,
  including multi-error accumulation order.
- Registries populated in the same insertion order; `kb.query` /
  `kb.config` resolved with last-wins.
- `SolverConfig.from_kw_pairs` parity: kebab→snake mapping, coercions,
  and the unknown-flag message with its **sorted** list of valid flags.
- Provenance-cycle detection produces the same message and the same
  chosen cycle path.
- Every corpus file loads to a KB that passes the KB-shape diff.

## Tasks

### Task T1a.2.3.1 — Form classification and passes

Bucket top-level forms by head (`relation` / `rule` / `hrule` / `macro` /
`import` / `query` / `config` / `trace` / *fact*), then run the passes in
ein.py's order: macros → relations → rules → facts → query → config →
unexpanded-macro guard → `rebuild_indexes` → cycle check. An
`(import …)` surviving into this stream is an internal error with its own
message.

### Task T1a.2.3.2 — Relation ingest

`(relation R T1 T2 … :kw v)` — flat signature (possibly empty, per
S1.22.4), the `:why` template, the auto-stored declaration fact, and the
rejection of the wrapped form `(relation R (T1 T2))` that parses as a
generic fact. Open-world relations are auto-created for undeclared fact
heads with `declared=False`; `add_relation`'s declared-wins upgrade rule
ports as-is.

### Task T1a.2.3.3 — Rule ingest

`(rule N (?p…) :match … :assert … :why … :priority …)` and `hrule` into
the separate registry. `Pattern.from_ir` (the structural view: bound
vars, relation names) ports with it. Reserved-name collisions, duplicate
names across `rules`/`hrules`, missing required kw-pairs — all with their
existing messages.

### Task T1a.2.3.4 — Fact ingest

`_fact_args` semantics: kw-pairs dropped; `Atom`/`String`/`Int`/`Var`/
`Range` flattened to atomic values (`Var` → `"?name"`, `Range` →
`"lo..hi"` / `"lo..*"`); a nested `SForm` becomes a nested fact
recursively, with `"<nested>"` as the head when the inner head is not a
bare atom. Provenance from `:source` / `:rule` + `:using`.

### Task T1a.2.3.5 — Query and config

`Query` keeps the raw kw-pair tuple. `SolverConfig.from_kw_pairs` with
every coercion path and error message; the field set and **declaration
order** must match (it is printed by `--dump-config`).

### Task T1a.2.3.6 — Validators

The unexpanded-macro guard (S1.8a.f20) and `detect_provenance_cycles`
(user-authored `:using` chains can be circular; the loader rejects them
with the rendered path).

## Notes

- Error *accumulation order* follows the pass order and the form order
  within a pass. Getting this wrong shows up as a reordered `; `-joined
  message, which the fixtures catch immediately.
- The loader is the last place a `Loc` is available for a message —
  and top-level forms carry `None` (Q-M1a.6). Render `None` for now.
