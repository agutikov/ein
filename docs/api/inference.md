# `ein.inference` — saturate, solve, verdicts, config

The engine surface: forward-chaining saturation, the one solve entry, the
verdict types it returns, and the `SolverConfig` knobs. Source:
[`ein.py/src/ein/inference/`](../../ein.py/src/ein/inference/).

> **Audience: embedders.** This page is the *public* engine surface — what
> you call and what you read. The matcher / compiler / hypgen / lattice
> *internals* are [`docs/kernel/inference/python_impl.md`](../kernel/inference/python_impl.md)
> (file map) + [`architecture_and_algorithms.md`](../kernel/inference/architecture_and_algorithms.md)
> (algorithms).

*Verified against commit `60c192b` (2026-06-16).*

## Saturation

### `Saturator(kb, engine=None)`

Priority-banded forward-chaining driver over a [`KnowledgeBase`](kb.md).
`Saturator(kb)` auto-builds and compiles the engine; pass an `engine` only
if you already hold a compiled one. **You usually don't call this** —
[`solve`](#solving) saturates internally. Use it to get the monotonic
deductive closure *without* the hypothesis search.

| method | signature | returns |
|--------|-----------|---------|
| `saturate` | `saturate(*, max_steps=None)` | `Iterator[Firing]` — one per applied firing; drain to a list to run to fixpoint. |
| `step` | `step()` | `Firing \| None` — apply the single highest-priority candidate (`None` when quiescent). |
| `is_stalled` | `is_stalled()` | `bool` — no further firing possible. |
| `contradictions` | `contradictions()` | the same-layer `(X, (not X))` pairs found. |

```python
from ein.inference.saturator import Saturator
firings = list(Saturator(kb).saturate())
```

### `Firing`

A frozen record of one rule application (`from ein.inference.firing import Firing`):

| field | meaning |
|-------|---------|
| `rule` | `str` — the rule that fired. |
| `activator` | the activator binding that authorised it. |
| `bindings` | `dict[str, Any]` — the matcher's var bindings. |
| `derived` | `tuple[Fact, …]` — the fact(s) concluded (an `:assert (and …)` fans out to N). |
| `premises` | `tuple[Fact, …]` — the facts the matcher consumed. |
| `redundant` | `bool` — the conclusion was already present (shown in the trace, not re-inserted). |

## Solving

### `solve(root_kb, *, …) -> tuple[Verdict | Aborted, MonotonicStats]`

The **single engine entry** (`from ein.inference.monotonic import solve`).
Runs the set-indexed lattice search, recording every distinct solution
node, and derives the verdict from the count `k`. The verdict *type* is
**read from `k`**, never chosen by an argument (there is no `mode=`).

```python
solve(
    root_kb,                 # the KnowledgeBase to solve
    *,
    stop_after=None,         # int | None — stop after N distinct models; None exhausts
    max_set_size=5,          # int — largest commitment set size
    config=None,             # SolverConfig | None — engine knobs (precedence below)
    dumper=None,             # diagnostics sink (MonotonicDumper / LatticeDumper)
    max_time=None,           # float | None — wall-clock budget (seconds)
    max_enterings=None,      # int | None — search-step budget
    store_lattice=False,     # attach a LatticeProof (needed for the trace)
    on_budget="raise",       # "raise" → BudgetExceededError; "verdict" → Aborted
)
```

| `k` | verdict |
|-----|---------|
| `1` | [`Solution`](#verdicts) — a model (certified unique iff `stats.exhausted`). |
| `> 1` | [`Ambiguity`](#verdicts) — `k` distinct models (a gap). |
| `0` | [`Contradiction`](#verdicts) — unsat (when exhausted). |

**Stop policy (orthogonal to the verdict).** `stop_after=1` is the sound
fast path: it stops at the first complete, consistent node and sets
`stats.exhausted=False`, so a `k=1` reads as "*a* model", not
certified-unique. `stop_after=None` exhausts the lattice and certifies
unique / ambiguous / unsat. `MonotonicStats` carries `solution_nodes`
(`== k`) and `exhausted`.

**Budgets.** With `on_budget="raise"` (default), exceeding `max_time` /
`max_enterings` raises `BudgetExceededError`. With `on_budget="verdict"`
it returns an `Aborted` (carrying the partial `stats`) — note `Aborted` is
**outside** the `Verdict` union, so exhaustive `isinstance` handling of
`Solution`/`Ambiguity`/`Contradiction` is unaffected; match it explicitly.

```python
from ein.inference.monotonic import solve
verdict, stats = solve(kb, stop_after=1)
```

## Verdicts

`from ein.inference.verdict import Solution, Ambiguity, Contradiction, Aborted, Verdict`.
`Verdict = Solution | Ambiguity | Contradiction` (the proven verdicts);
`Aborted` is separate (budget cut, not proven). Each proven verdict carries
an optional `proof: LatticeProof | None`, populated only when `solve` was
called with `store_lattice=True`.

| verdict | fields |
|---------|--------|
| `Solution` | `kb: KnowledgeBase`, `trace: tuple[Firing, …]`, `proof`. |
| `Ambiguity` | `branches: tuple[Solution, …]`, `proof`. |
| `Contradiction` | `unsat_core: frozenset[Fact]`, `proof`. |
| `Aborted` | `reason: str`, `stats` (partial `MonotonicStats`). |

### Reading the answer — `goal_bindings(kb, goal=None) -> list[dict[str, str]]`

Run the query `:goal` pattern against a (solved) kb; return the binding
rows. `goal` defaults to the kb's own `(query :goal …)`; pass an explicit
goal pattern to project a different question over a solved model.

```python
from ein.inference.verdict import Solution, goal_bindings
if isinstance(verdict, Solution):
    print(goal_bindings(verdict.kb))
    # [{'h_water': 'House-1', 'who_water': 'Norwegian',
    #   'h_zebra': 'House-5', 'who_zebra': 'Japanese'}]
```

### `Mode`, `is_solved`, `query_value`

- `Mode` — `Enum(SOLVE, GAPS, CONTRADICTIONS)`; the three task classes from
  idea 03. Used for goal-checking, **not** as a `solve` argument.
- `is_solved(kb, mode) -> bool` — does the kb satisfy the goal under
  `mode`? (`SOLVE`: exactly one binding; `GAPS`: ≥ one; `CONTRADICTIONS`:
  never.)
- `query_value(query, kw_name)` — look up a `(query … :kw value)` value.

## `SolverConfig`

`from ein.inference.config import SolverConfig` — a frozen dataclass of
engine knobs, each mapping 1:1 to a `:kebab-flag` in the IR `(config …)`
block. **Resolution precedence:** explicit `solve(kb, config=…)` >
`kb.config` (from the IR) > `SolverConfig()` defaults.

| field | default | effect |
|-------|---------|--------|
| `enable_pre_branch_lookahead` | `True` | One-step `_dies_immediately` rule simulator that prunes doomed candidates pre-branch. |
| `enable_lookahead_kill_cache` | `True` | Cache a lookahead-killed candidate as a `(not h)` fact for O(1) skip (vs re-running the lookahead). |
| `hypgen_scoring` | `"popularity"` | Hypothesis ordering heuristic. `"popularity"` (weighted fact-count at relation+object level), `"most-constrained"` (escape hatch), `"branch-info"` / `"popularity+branch-info"` (reserved — raise today). |
| `hypgen_rel_weight` | `1.0` | popularity coefficient for the relation's fact-count. |
| `hypgen_obj_weight` | `1.0` | popularity coefficient for each object arg's fact-count. |
| `print_alive` | `False` | Diagnostic — log inherited alive-set size + per-filter prune counts per `_explore`. |
| `warn_derived_naf` | `False` | Emit a `DerivedNafWarning` per rule whose `(absent …)` guard watches a rule-derived relation. |
| `candidate_order_seed` | `-1` | `< 0` → deterministic content-sort branch order; `≥ 0` → a deterministic per-branch permutation (shuffle-invariance probing). |
| `lattice_sanity_check` | `False` | Verify saturation commutativity for size-`k≥2` commitments (release-regression only; costs `k+1` saturations each). |
| `lattice_order` | `"lex"` | Within-layer candidate order. `"lex"` (canonical-tuple sort, the baseline) or `"score-sum"` (per-set score; needs `hypgen_scoring="popularity"` to differentiate). |
| `lattice_order_seed` | `None` | Per-layer shuffle seed (traversal-order dependence probing); `None` disables. |
| `enable_path_nogoods` | `True` | CDCL path-condition no-good emission. Off → no clause emitted, so subsumed dead commitments are re-explored. |
| `enable_symmetric_mirror` | `True` | The `__symmetric__` native arg-swap mirror (kernel fast-path over the stdlib `symmetric` rule). Off → marked relations not closed under swap by the fast-path. |
| `enable_singleton_writeback` | `True` | Size-1 dead-clause `(not h)` writeback to `_negated_facts`. Off → the negation is re-derived rather than cached. |
| `enable_forced_positive` | `True` | Forced-positive promotion: a sole-surviving alive singleton is promoted to a root fact. |

The last four (added S1.20.I2) gate features that were previously always-on,
so P1.20 Theme I can measure each — all default `True`, so the shipped solve
is unchanged.

`SolverConfig.from_kw_pairs(kw_pairs)` builds one from a parsed `(config
…)` body (the loader uses this; unknown flags raise `ValueError`).

> The *measured* impact of these knobs against `zebra2` (which is
> load-bearing, which is perf-only) is in
> [`docs/kernel/inference/features.md`](../kernel/inference/features.md);
> this page is the definitional side and does not duplicate the numbers.

## See also

- [`ein.md`](ein.md) — the end-to-end flow.
- [`kb.md`](kb.md) — `Fact` / `Layer` / `Provenance` the verdicts carry.
- [`trace.md`](trace.md) — turning a verdict into a narrative.
- [`docs/kernel/inference/`](../kernel/inference/) — the engine internals.
