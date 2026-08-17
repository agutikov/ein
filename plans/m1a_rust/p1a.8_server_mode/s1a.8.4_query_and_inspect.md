# S1a.8.4 — Querying and inspecting a resident KB

**Phase:** P1a.8 (Server mode)
**Estimate:** 2 days
**Depends on:** [S1a.8.3](s1a.8.3_session_and_kb_lifecycle.md)
**Implements:** [design/09](../design/09_server_mode.md) §4 ("Asking")

## Context

The stage that delivers "multiple queries to the same KB" — the ask that
motivated server mode in the first place.

The engine can already do it: `goal_bindings(kb, goal)` compiles an
arbitrary pattern into a synthetic `<query>` plan and runs the matcher
over any KB, and its docstring says so explicitly ("pass an explicit
goal pattern to project a different question"). What is missing is a way
to *hold* the saturated KB between questions, which is exactly what a
handle is.

So this stage is small, and its value is entirely in what it makes
cheap: load once, saturate once, ask a hundred times.

## Acceptance

- 100 `kb.query` calls against one saturated `zebra2` KB cost < 1 % of
  100 `ein solve` invocations.
- `kb.query` results identical to `goal_bindings` via the CLI for the
  same pattern and KB.
- `kb.facts` paging is stable: the same filter and cursor return the same
  page across calls, and iterating all pages yields the KB's fact order
  exactly once.
- A query against a *model* handle (a solution node) works identically to
  one against a saturated KB handle.
- A malformed goal pattern returns the compiler's verbatim error, not a
  generic failure.

## Tasks

### Task T1a.8.4.1 — `kb.query`

`{kb, goal}` where `goal` is a pattern in ein syntax (parsed with the
normal frontend, so `(and …)` conjunctions and nested patterns work).
Compile to the synthetic `<query>` plan, run, return binding rows as
`{var: value}` maps in match order.

Optional `limit` / `offset`, and an optional `count_only` for
"how many models satisfy this?" without materialising rows.

### Task T1a.8.4.2 — `kb.facts`

Paged listing with filters: by relation, by provenance kind
(`source` / `rule` / `hypothesis` / `rejected` / none), by name
participation (`about`), by rule (`by_rule`). These map onto
`FactView`'s existing filters
([S1a.2.2](../p1a.2_kb_core/s1a.2.2_store_and_indexes.md) T1a.2.2.5), so
the server adds paging and nothing else.

Cursor is a fact index into the KB's insertion order — stable because
the KB is append-only and a handle's KB never changes.

### Task T1a.8.4.3 — `kb.contradictions`

The detector's records: kind (`pair` / `direct`), the witness, the
positive and negative facts. Useful on its own for a GUI showing why a
branch is dead.

### Task T1a.8.4.4 — Fact rendering

Every fact crossing the wire uses `fact_sexpr` — the same renderer the
event protocol and the CLI use — so a client never sees an
implementation-internal id and three surfaces cannot drift.

Provide the provenance summary alongside on request (`include_provenance:
true`), reusing `_fact_summary` from
[S1a.5.3](../p1a.5_presentation/s1a.5.3_state_dumps.md).

### Task T1a.8.4.5 — The load-once/ask-many benchmark

The acceptance number above, as a permanent benchmark: parse+load+saturate
once, then N queries, versus N CLI invocations. It is the number that
justifies the whole phase, so it should be measured continuously rather
than once.

## Notes

- Queries are read-only, so they parallelise trivially at the request
  level (level 4 — [design/08](../design/08_parallelism.md) §5) with no
  new machinery.
- Do not grow the query surface beyond `:goal`-shaped patterns. A richer
  query language belongs in the *language*
  ([design/09](../design/09_server_mode.md) §9), where both
  implementations get it, not in the server, where only one does.
