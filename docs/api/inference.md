# `ein.inference` — saturate, solve, verdicts, config

> ### 🏛 History — the embedding contract of the engine that was
>
> **This page describes a Python package that no longer exists**, and it is
> filed as a record rather than as a promise. `ein.py/` was deleted at M1a
> [S1a.10.5](../../plans/m1a_rust/p1a.10_single_implementation/s1a.10.5_removal.md)
> (2026-08-21); the PyO3 module that was to succeed it was **deferred the same
> day** for want of a consumer, with three trip-wires recorded in
> [Q-M1a.23](../../plans/m1a_rust/open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding).
>
> It is kept **whole and unedited** for one reason: a deferral is cheap to
> reverse only while the specification survives it. On the day a trip-wire
> fires, this is a contract to implement instead of a blank page. So read
> every code block as a record — and **do not "fix" one to match `ein.rs`'s
> internals.** A page rewritten to describe the current engine would be
> neither history nor a specification.
>
> **The embedding surface that exists is Rust**, and it is
> [`rust.md`](rust.md) — the crates, whose worked example is a test the gate
> runs. The other surface that runs is the CLI: `ein solve <file>` ·
> `ein saturate` · `ein render` · `ein kb` (`ein --help`,
> [`docs/install.md`](../install.md)).

The engine surface: forward-chaining saturation, the one solve entry, the
verdict types it returns, and the `SolverConfig` knobs. The engine behind it is
[`ein-infer`](../../ein.rs/crates/ein-infer/src/).

> **Audience: embedders.** This page is the *public* engine surface — what
> you call and what you read. The matcher / compiler / hypgen / lattice
> *internals* are [`docs/kernel/inference/implementation.md`](../kernel/inference/implementation.md)
> (file map) + [`architecture_and_algorithms.md`](../kernel/inference/architecture_and_algorithms.md)
> (algorithms).

*Verified against commit `60c192b` (2026-06-16) — **against the Python engine, which no longer exists**. These signatures are a record of what that engine offered, not a description of anything in the tree and no longer a contract anything is scheduled to implement ([Q-M1a.23](../../plans/m1a_rust/open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding)).*

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
| `step` | `step()` | `Firing \| None` — the highest-priority positive candidate, or, at positive quiescence, the one candidate the NAF boundary admits (`None` at the two-phase fixpoint). |
| `is_stalled` | `is_stalled()` | `bool` — no further firing possible; consults the boundary too, so it means the two-phase fixpoint, not mere closure quiescence. |
| `contradictions` | `contradictions()` | the `(X, (not X))` pairs found — whatever either side's origin (S1.22.1b). |

```python
from ein.inference.saturator import Saturator
firings = list(Saturator(kb).saturate())
```

**Two-phase saturation (S1.21.8).** `step` alternates a **closure** phase —
purely positive plans fire to quiescence, consulting no negation — with a
**boundary** phase: candidates whose disjunct carries an `(absent …)` guard
never enter the firing queue, they are *parked*, and at quiescence they are
judged against that fixpoint (an `ein.inference.world.World` over the stalled
KB). At most **one** is admitted per round; it re-enters the closure, and the
loop ends when a quiescence admits nothing. Consequence for embedders: on a
**stratified** rule set the deductive closure no longer depends on rule
priority (band discipline is advisory ordering, not semantics); on a
non-stratified one the engine still reports one model, chosen by
boundary-admission order. Normative page:
[`docs/kernel/inference/absent_semantics.md`](../kernel/inference/absent_semantics.md).

Counters on the `Saturator` — diagnostics, read after saturating:

| attribute | meaning |
|-----------|---------|
| `naf_rounds` | boundary rounds run — one per positive quiescence at which candidates were parked. |
| `naf_admitted` | parked candidates admitted (each re-entered the closure). |
| `naf_retired` | candidates dropped for good: an anti-monotone guard found a match, and the KB only grows, so it can never pass again. |
| `naf_dropped` | **structurally 0** — it counted firings dropped by the retired fire-time re-check; a guard is now decided once, at the moment its candidate is admitted, so there is no enqueue/fire race left to lose one to. |

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

`premises` is positive-only. A firing the NAF boundary admitted also records
what had to be *absent* — on its conclusions' provenance, as
[`Provenance.absent_premises`](kb.md#provenance), not on the `Firing`.

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
| `Contradiction` | `unsat_core: frozenset[Fact]` — the *source frontier* that forces the conflict, not the clashing facts themselves; `proof`. |
| `Aborted` | `reason: str`, `stats` (partial `MonotonicStats`). |

**`Contradiction.unsat_core`.** The smallest set of *given* facts
(`source` / `hypothesis` / un-provenanced) from which one recorded
contradiction follows — a minimum-cardinality AND/OR search over every
recorded derivation (provenance-based, NAF-safe, budgeted); **not** a
subset-minimal MUS: no proper subset is checked for satisfiability, and
minimality is relative to the rule set and to the derivations the
saturator recorded. It comes from
`ein.inference.frontier.smallest_contradiction_frontier(kb)`. The search
itself is `ein.inference.explain`, and an embedder can run it on any
fact: `explain(kb, targets, *, budget=None)` and
`minimal_contradiction_frontier(kb, witnesses=None, *, budget=None)`
return an `Explanation` (`frontier`, `target`, `exhausted`, `rounds`,
`facts_considered`) under an `ExplanationBudget(max_environments=64,
max_rounds=64, max_env_size=None, max_facts=20_000)` — the problem is
worst-case exponential, and `exhausted=False` says a cap was hit, so the
frontier is still sound but possibly not the smallest. Contrast
[`kb.unsat_core`](kb.md), which *unions* the frontiers of every
conflicting fact and therefore grows, rather than shrinks, as more
derivations are recorded.

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
| `enable_pre_branch_lookahead` | `True` | One-step `_dies_immediately` rule simulator that prunes doomed candidates pre-branch. **Not only a prune**: `complete()` asks the hypothesis generator whether anything is undecided, and the generator's candidates are lookahead-filtered — so a candidate this kills is *decided*, and turning the knob off can turn `Ambiguity` into `Contradiction` (measured on two corpus fixtures, both engines; [features.md](../kernel/inference/features.md), [F4 Q40](../../plans/followups/f4_cross_cutting.md)). |
| `enable_lookahead_kill_cache` | `True` | Cache a lookahead-killed candidate as a `(not h)` fact for O(1) skip (vs re-running the lookahead). |
| `hypgen_scoring` | `"popularity"` | Hypothesis ordering heuristic. `"popularity"` (weighted fact-count at relation+object level), `"most-constrained"` (escape hatch), `"branch-info"` / `"popularity+branch-info"` (reserved — raise today). |
| `hypgen_rel_weight` | `1.0` | popularity coefficient for the relation's fact-count. |
| `hypgen_obj_weight` | `1.0` | popularity coefficient for each object arg's fact-count. |
| `print_alive` | `False` | Diagnostic — log inherited alive-set size + per-filter prune counts per `_explore`. |
| `warn_derived_naf` | `False` | Emit a `DerivedNafWarning` per rule whose `(absent …)` guard watches a rule-derived relation. Advisory: since S1.21.8 that shape is no longer a soundness question but a **stratification** one — NAF over a derived relation is what can make a rule set non-stratified, and the engine answers such a set with one model without saying others exist. |
| `candidate_order_seed` | `-1` | `< 0` → deterministic content-sort branch order; `≥ 0` → a deterministic per-branch permutation (shuffle-invariance probing). |
| `lattice_sanity_check` | `False` | Verify saturation commutativity for size-`k≥2` commitments (release-regression only; costs `k+1` saturations each). |
| `lattice_order` | `"lex"` | Within-layer candidate order. `"lex"` (canonical-tuple sort, the baseline) or `"score-sum"` (per-set score; needs `hypgen_scoring="popularity"` to differentiate). |
| `lattice_order_seed` | `None` | Per-layer shuffle seed (traversal-order dependence probing); `None` disables. |
| `enable_path_nogoods` | `True` | Learned no-good emission (CDCL-style). Off → no clause emitted, so subsumed dead commitments are re-explored. |
| `enable_symmetric_mirror` | `True` | The `__symmetric__` native arg-swap mirror (kernel fast-path over the stdlib `symmetric` rule). Off → marked relations not closed under swap by the fast-path. |
| `enable_singleton_writeback` | `True` | Size-1 dead-clause `(not h)` writeback to `_negated_facts`. Off → the negation is re-derived rather than cached. |
| `enable_forced_positive` | `True` | Forced-positive promotion: a sole-surviving alive singleton is promoted to a root fact. |
| `enable_fail_fast_fork` | `True` | Stop a fork's saturation at the firing whose conclusion makes it inconsistent, instead of saturating to quiescence and only then scanning (sound: the KB is append-only, so a contradiction is never retracted). 2.4× on exhaustive `zebra2`. Off → dead forks run to the fixpoint, so `firings` is the full run and the dead fork's `state_key` the complete state — wanted when a *dead* fork's post-saturation KB is itself the artefact. |
| `record_alternative_justifications` | `True` | Record a re-derivation of an already-known fact as an *alternative* justification ([`kb.justifications`](kb.md)) instead of dropping it, making the proof an AND/OR graph for the minimal-explanation search to traverse. Off → one justification per fact, and the reported `unsat_core` degrades to the recorded-primary walk (sound, but dependent on rule-firing order). |

The four S1.20.I2 `enable_*` flags (`path_nogoods` … `forced_positive`) gate
features that were previously always-on, so P1.20 Theme I can measure each;
`enable_fail_fast_fork` (S1.9.E23) follows the same convention. All default
`True`, so the shipped solve is the all-on configuration.

`SolverConfig.from_kw_pairs(kw_pairs)` builds one from a parsed `(config
…)` body (the loader uses this; unknown flags raise `ValueError`).

> **There is no `jobs` row here, and there should not be.** This table is the
> engine that was, and it had no fan-out. `ein.rs` grew one at
> [P1a.7](../../plans/m1a_rust/p1a.7_parallelism/README.md) (closed
> 2026-08-23, 3.17–4.40× on 8 cores) — and it is deliberately **not** a
> `SolverConfig` field but a `SolveOptions` one, because a puzzle file must
> not be able to set a thread count. The Rust page states the parallelism
> surface as it actually is: [`rust.md` § 4](rust.md#4--solve).

> The *measured* impact of these knobs against `zebra2` (which is
> load-bearing, which is perf-only) is in
> [`docs/kernel/inference/features.md`](../kernel/inference/features.md);
> this page is the definitional side and does not duplicate the numbers.

## See also

- [`ein.md`](ein.md) — the end-to-end flow.
- [`kb.md`](kb.md) — `Fact` / `Provenance` the verdicts carry.
- [`trace.md`](trace.md) — turning a verdict into a narrative.
- [`docs/kernel/inference/`](../kernel/inference/) — the engine internals.
