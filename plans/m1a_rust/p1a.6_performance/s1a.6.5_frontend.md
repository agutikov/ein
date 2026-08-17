# S1a.6.5 — Frontend and load path

**Phase:** P1a.6 (Performance)
**Estimate:** 2 days
**Depends on:** [S1a.6.1](s1a.6.1_profile_baseline.md)
**Implements:** refinements to [design/04](../design/04_ir_frontend.md)

## Context

Parse + load is 0.63 s of a 5.7 s CPython zebra2 run and 0.78 s under
PyPy — a fixed cost paid on *every* invocation, and the one a user feels
most directly because it happens before anything is printed. The
hand-written frontend should already have collapsed it by two orders of
magnitude; this stage confirms that and removes what is left.

It also matters disproportionately for the workloads around the engine:
the conformance runner makes thousands of invocations over the same
files, and `utils/feature_matrix.py` makes ~20 fresh-process runs per
matrix.

## Acceptance

- Parse + resolve of `zebra2.ein` under 2 ms (P1a.1's gate) and under
  1 ms after this stage.
- Full `load` (parse + imports + macro expansion + relation/rule/fact
  ingest + `rebuild_indexes`) under 5 ms.
- Allocation count for a load bounded and reported.
- T3 green — the frontend's output is text goldens, so this is
  immediately verifiable.

## Tasks

### Task T1a.6.5.1 — Lexer

Confirm zero allocation per token (tokens are `(kind, span)`), use
`memchr` for comment and string scanning, and check that the reserved-word
rejection is a perfect-hash / length-bucketed compare rather than a
linear walk of eleven strings per identifier.

### Task T1a.6.5.2 — Arena pre-sizing

Size the AST node/args arenas from the source length (a good linear
estimate for s-expressions) so a parse does no reallocation. Same for the
interner's text arena on a cold start.

### Task T1a.6.5.3 — Import resolution

Resolving `zebra2.ein` pulls three stdlib modules, each parsed fresh.
Cache parsed module ASTs by resolved path + content hash **within a
process** — this is the same content-addressed idea
[design/09](../design/09_server_mode.md) §6 formalises, just scoped to
one run, and it is what makes the conformance runner's repeated loads
cheap.

Keep it off by default in the CLI if it perturbs anything observable; it
should not, since resolution is a pure function of
`(source, stdlib content, base_dir)`.

### Task T1a.6.5.4 — Macro expansion

Substitution copies subtrees. Measure whether copy-on-substitute or a
lazy view pays; the stdlib's macros are tiny (`macro.ein` is 21 lines),
so this is probably already free — confirm and move on rather than
optimising it speculatively.

### Task T1a.6.5.5 — Index building

`rebuild_indexes` is one pass over facts feeding six groupings. Size the
maps from the fact count up front, and check whether the incremental
`index_fact` path (used during saturation) and the batch path can share a
single implementation without either paying for the other's shape.

### Task T1a.6.5.6 — Startup

Measure process start to first output for `ein --help` and a trivial
solve. Binary size, dynamic-linking and the embedded stdlib all show up
here; a 5 ms engine behind a 40 ms startup is not a fast tool.

## Notes

- The load path also runs on the server's cold path
  ([design/09](../design/09_server_mode.md)), where it is amortised — so
  do not over-invest here at the expense of
  [S1a.6.3](s1a.6.3_beta_memories.md). The user-visible win is
  bounded by "already imperceptible".
- If `rebuild_indexes` shows up, check first whether the loader is
  calling it more than once.
