# Reserved engine strings

Engine-**internal** vocabulary — strings the inference engine keys on that
are *not* author-facing surface syntax. (For names a puzzle author writes,
see [`../ir/03-ein-lang/06_reserved_names.md`](../ir/03-ein-lang/06_reserved_names.md).)

All are **reserved engine strings for M1**: each lives behind a single
named constant / enum, documented here. De-naming them (string → richer
type, or removing the hardcode) is a **post-M1** question — the routes are
parked in S1.7.19–.22. The point of this doc is that *a name is reserved
iff it appears here or in the surface doc*, and nothing is undocumented.

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
[`inference/verdict.py`](../../../ein.py/src/ein_bot/inference/verdict.py);
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

## `closed` — engine effect

The author-facing `(closed R)` (see the [surface doc](../ir/03-ein-lang/06_reserved_names.md))
has two engine sides, both isolated in
[`inference/closed.py`](../../../ein.py/src/ein_bot/inference/closed.py)
(constant `CLOSED`):

- **Auto-inference** — `emit_closed` writes `(closed R)` for every declared
  relation no compiled rule positively asserts (`producible_relations`),
  run once before the initial saturation.
- **Hypgen suppression** — `hypgen._is_closed` reads `(closed R)` facts and
  contributes zero candidates for R.

Kept kernel mechanism for M1 — it is load-bearing for hypgen scoping / NAF
soundness ([S1.7.10](../../../plans/m1_core_graph_reasoning/p1.7_bootstrapping_zebra/s1.7.10_closed.md));
the de-hardcode question is parked post-M1.

## Result-level invariants (S1.7.24)

Not strings, but recorded here as part of the engine contract: the
lattice snapshot (`monotonic/snapshot.py`) is **result-level** — it keys
solutions/deads on post-saturation `state_hash`, NOT commitment paths, and
**excludes learned nogoods** (a clause and its symmetric mirror are
equivalent only under symmetry, so the final nogood set is an
order/orientation-sensitive optimisation artifact, not part of the result).
