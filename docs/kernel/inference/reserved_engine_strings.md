# Reserved engine strings

Engine-**internal** vocabulary — strings the inference engine keys on that
are *not* author-facing surface syntax. (For names a puzzle author writes,
see [`../ir/03-ein-lang/06_reserved_names.md`](../ir/03-ein-lang/06_reserved_names.md).)

All are **reserved engine strings for M1**: each lives behind a single
named constant / enum, documented here. De-naming them (string → richer
type, or removing the hardcode) is a **post-M1** question — the routes are
parked in S1.7.19–.22. The point of this doc is that *a name is reserved
iff it appears here or in the surface doc*, and nothing is undocumented.

**Dunder convention (2026-06-15).** A name that triggers kernel-hardcoded
*behaviour* is written `__dunder__`, lexically distinct from userspace
rule/relation names (the grammar admits a leading `__`; a bare name never
triggers kernel behaviour). `__closed__` and `__symmetric__` are the two so
far. The bookkeeping carrier heads and the surface task-class / control
keywords below predate the convention and keep their bare names.

## Bookkeeping carrier heads

Synthetic fact heads the search uses to carry structure; excluded from the
`state_hash` canonical form so they don't perturb model identity. Single
source: `canon.BOOKKEEPING_HEADS`.

| head | meaning | where |
|------|---------|-------|
| `hypothesis` | wraps a speculative fact introduced at a fork: `(hypothesis (R …))` | `canon.py`; `state_dump.py` |
| `contradiction-under` | wraps the hypothesis a contradiction was found under | `canon.py` |

## Task-class entries

The three search modes. Single source: the `Mode` enum in
[`inference/verdict.py`](../../../ein.py/src/ein/inference/verdict.py);
the solver's `entry` discriminator uses the same string values.

| string | `Mode` | entry function | verdict shape |
|--------|--------|----------------|---------------|
| `solve` | `Mode.SOLVE` | `solve` | `Solution` / `Ambiguity` / `Contradiction` (k from state-deduped nodes) |
| `gaps` | `Mode.GAPS` | `gaps_solve` | `Ambiguity` (distinct model states) |
| `contradictions` | `Mode.CONTRADICTIONS` | `contradictions_solve` | `Contradiction` (unsat-core union) |

## Protocol enums

Closed string sets the engine branches on internally (surveyed in
[S1.7.22](../../../plans/m1_core_graph_reasoning/p1.7_bootstrapping_zebra/s1.7.22_engine_internal_strings.md)).
Each is a `Literal[...]` / enum in one place; de-naming → string-to-enum is
post-M1.

| set | values | where |
|-----|--------|-------|
| provenance kind | `source` · `rule` · `hypothesis` · `rejected` | `kb/provenance.py` |
| lattice node verdict | `alive` · `dead` · `solution` | `monotonic/lattice.py` (`SetNode.verdict`) |
| dead kind | `dead-pre` · `dead-post` | `monotonic/lattice.py` (`DeadCommitment.kind`) |
| hypgen scoring | `most-constrained` · `popularity` (`branch-info` reserved) | `hypgen.score_hypothesis` |

## `__closed__` — engine effect

The kernel trigger `(__closed__ R)` (a **dunder** name per the convention above;
the bare `closed` is now a free userspace name) has two engine sides, both
isolated in
[`inference/closed.py`](../../../ein.py/src/ein/inference/closed.py)
(constant `CLOSED = "__closed__"`):

- **Auto-inference** — `emit_closed` writes `(__closed__ R)` for every declared
  relation no compiled rule positively asserts (`producible_relations`),
  run once before the initial saturation.
- **Hypgen suppression** — `hypgen._is_closed` reads `(__closed__ R)` facts and
  contributes zero candidates for R.

**Genuinely kernel** (a saturation rule can neither suppress hypgen generation
nor introspect "no rule asserts R" — see the symmetric contrast in the
`__symmetric__` design). Load-bearing for hypgen scoping / NAF soundness
([S1.7.10](../../../plans/m1_core_graph_reasoning/p1.7_bootstrapping_zebra/s1.7.10_closed.md)).
Renamed `closed → __closed__` 2026-06-15 per the dunder convention;
`std.closure`'s `infer-closure` asserts `(__closed__ ?R)`.

## `__symmetric__` — engine effect

The kernel trigger `(__symmetric__ R)` closes R's extension under arg-swap
**natively in the saturator**: each `(R a b)` produces `(R b a)` directly
(self-loops `a=b` and already-present mirrors skipped), as a `Firing` with rule
`__symmetric__` threading the source edge as its premise. Single source:
`SYMMETRIC = "__symmetric__"` + the mirror machinery
(`_next_mirror_firing` / `_enqueue_mirror_sources`) in
[`inference/saturator.py`](../../../ein.py/src/ein/inference/saturator.py).

**A performance optimization, NOT a capability.** It computes the *identical*
closure as the stdlib `symmetric` rule (`std.algebra`) — pinned by
`test_symmetric_native.py::test_parity_with_stdlib_symmetric` — but skips the
JoinPlan + `match.run` the rule pays per mirror (~1.2× on the synthetic
`ein.py/src/ein/cli/symmetric.py`; **no real symmetric-heavy puzzle exists yet** —
zebra2 uses `co-located*` rules, not the generic closure). Opt-in by marking
the relation; ordinary puzzles take the no-op path (the mirror queue is empty
when nothing is marked, so zero overhead). Re-adds, behind the dunder, the
kernel symmetric-awareness [S1.7.24](../../../plans/m1_core_graph_reasoning/p1.7_bootstrapping_zebra/s1.7.24_dehardcode_symmetric.md)
removed — now namespaced so it never masquerades as a userspace name.

## Query-scoping keys

`(query …)`-block keywords the hypothesis generator reads to scope which
relations it enumerates. Single source: the `HYPOTHESIS_RELATIONS` /
`NO_HYPOTHESIS` constants in
[`inference/hypgen.py`](../../../ein.py/src/ein/inference/hypgen.py); both
scope the *blind enumerator* only (hrule-driven generation ignores them).

| key | effect | where |
|-----|--------|-------|
| `hypothesis-relations` | **whitelist** — enumerate candidates *only* for the listed relations (`None` ⇒ all) | `hypgen._query_relations` |
| `no-hypothesis` | **blacklist** (S1.9.E3) — never guess on the listed relations; saturation rules on them still fire | `hypgen._no_hypothesis_relations` |

A relation named by both is excluded (blacklist wins). Neither touches the
saturator — hypgen-only scoping, distinct from `(__closed__ R)` above (which also
blocks rule-derivation).

## Result-level invariants (S1.7.24)

Not strings, but recorded here as part of the engine contract: the
lattice snapshot (`monotonic/snapshot.py`) is **result-level** — it keys
solutions/deads on post-saturation `state_hash`, NOT commitment paths, and
**excludes learned nogoods** (a clause and its symmetric mirror are
equivalent only under symmetry, so the final nogood set is an
order/orientation-sensitive optimisation artifact, not part of the result).
