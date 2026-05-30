# P1.7a — Refactoring design (the concrete change)

The route from [`analysis.md`](analysis.md) (today) to
[`target_design.md`](target_design.md) (ideal). Concrete types, signatures,
and the migration of the three entries. Decision points that
[S1.7a.1](#) measurement settles are flagged **[A1]**.

## Guiding constraint

**Minimise churn; reuse the substrate.** P1.5b already gives the shared loop,
`_compute_alive`, `state_hash`, per-branch snapshots, dead-commitment capture,
and budgets (see `analysis.md` "HAVE"). The refactor is mostly: (a) change the
*solution-recording predicate* from `is_solved` to `consistent ∧ complete`,
(b) **dedup** the recorded set by `state_hash`, (c) compute the verdict from
the count, (d) add `stop_after`, (e) demote `:goal`/`:mode`. No new search.

## 1. New predicates (S1.7a.2)

In `inference/` (likely a small new `inference/solution.py`, or extend
`verdict.py`):

```python
def open_hypotheses(kb) -> set[FactId]:
    "The open set: generate_hypotheses(kb), symmetric-canonicalised."
    # == the existing _compute_alive(kb), lifted to a public home.

def has_open_hypothesis(kb) -> bool:
    return bool(open_hypotheses(kb))

def complete(kb) -> bool:
    return not has_open_hypothesis(kb)

def consistent(kb) -> bool:
    return not ContradictionDetector(kb).detect()

def is_solution_node(kb) -> bool:
    return consistent(kb) and complete(kb)
```

`_compute_alive` (`solver.py:393`) becomes a thin alias of `open_hypotheses`
(or vice-versa) so there is exactly one implementation. **[A1]**: confirm the
open-vs-all-refuted dead-end question before trusting `complete` to *exclude*
(vs *contradiction-flag*) the zebra2 dead-end (analysis OQ1).

## 2. The run result (S1.7a.3)

A single result object the verdict reads from. Reuse `LatticeProof` as the
carrier — add the missing fields rather than introduce a parallel type:

```python
@dataclass(frozen=True)
class SolutionNode:
    state_hash: int
    kb: KnowledgeBase          # the per-branch snapshot (NOT the shared root)
    commitment: tuple[FactId, ...]   # one witnessing commitment (others merged)
    firings: tuple[Firing, ...]

# LatticeProof gains:
#   solution_nodes : tuple[SolutionNode, ...]   # deduped by state_hash; k = len()
#   exhausted      : bool                        # search reached all leaves ≤ max_set_size
#   stopped_by     : str                         # "exhausted" | "stop_after" | "budget"
# (the existing `solutions: SolutionRecord[]` list stays as the raw,
#  pre-dedup record for the dumper/audit; `solution_nodes` is the deduped view.)
```

Inside `_explore_layers`, at every point that currently records a solution on
`is_solved` (`solver.py:940,961,994`):

```
- solved = is_solved(result.kb, mode)
+ if is_solution_node(result.kb):
+     node = SolutionNode(state_hash(result.kb), result.kb.snapshot(),
+                         c, result.firings)
+     lstate.solution_nodes_by_hash.setdefault(node.state_hash, node)  # dedup
```

The incomplete-but-goal-matching dead-end is simply never recorded (it fails
`complete`). The 14 winning commitments collapse to one entry by `state_hash`.

## 3. Verdict from k (S1.7a.3)

```python
def verdict_of(proof) -> Verdict:
    k = len(proof.solution_nodes)
    if not proof.exhausted:
        return (Uncertified(proof.solution_nodes, proof)  # k >= 1
                if k else Unknown(proof))                  # k == 0, truncated
    if k == 1: return Unique(proof.solution_nodes[0], proof)
    if k > 1:  return Ambiguity(proof.solution_nodes, proof)
    return Contradiction(unsat_core=union_cores(proof), proof=proof)  # k == 0, exhausted
```

**Verdict types.** Keep `Solution` / `Ambiguity` / `Contradiction`
(`verdict.py`) to limit blast radius; add two states the target needs:
- `Unique` — alias/specialisation of `Solution` (k=1, exhausted). Simplest:
  keep returning `Solution` but require `exhausted ∧ k==1`; document that a
  bare `Solution` now *means* a true unique model.
- `Uncertified` / `Unknown` — for `¬exhausted`. Minimal form: reuse
  `Ambiguity` for `Uncertified` (k≥1, "could be more") and a distinct
  `Unknown`/`Inconclusive` for the truncated k=0. **[A1]** decides whether
  zebra2 ever needs these (it should certify Unique without truncation).

## 4. Stop policy (S1.7a.4)

Add one parameter to the core; budgets unchanged:

```python
def search(kb, *, stop_after: int | None = None,
           max_set_size=5, config=None, dumper=None,
           max_time=None, max_enterings=None) -> tuple[LatticeProof, Stats]:
```

In the loop, after recording a *new* solution node:

```python
if stop_after is not None and len(lstate.solution_nodes_by_hash) >= stop_after:
    proof.exhausted = False; proof.stopped_by = "stop_after"
    return _finalise(...)
```

`stop_after=1` is the sound fast-path (stops on the first complete∧consistent
node — never a partial match). Exhaust (`None`) is what Unique/Contradiction
certification requires (§3 couples them via `exhausted`).

## 5. Migrate the three entries

Two options; **recommend Option A** (lower churn, reversible):

- **Option A — presets over the core (recommended).** `search` + `verdict_of`
  are the new truth. The three existing names become thin presets:
  ```python
  def solve(kb, **kw):           # stop_after=kw.pop('stop_after', None); verdict_of(...)
  def all_solutions(kb, **kw):   # stop_after=None
  def refutations(kb, **kw):     # stop_after=None
  ```
  Keep `monotonic_solve`/`gaps_solve`/`contradictions_solve` as
  **deprecated shims** (one window) that call `search`+`verdict_of` and
  narrow the return type to the historical shape, so existing tests/imports
  don't break in the same commit. Drop them in a follow-up once callers move.
- **Option B — rename now.** Replace the three names outright. Cleaner end
  state, bigger single diff (every test + `cli.py` + both bench scripts).

Either way: **`entry` discriminator is removed** from the core (it only ever
varied `is_solved` short-circuit + verdict shape — both now derived).
`monotonic_solve`'s `_promote_forced_positives`-into-shared-root path is **not**
the answer source; if `solve(stop_after=1)` reuses any monotonic fast-path it
must read the answer from a per-branch node, not the root (analysis "INCORRECT"
#3). **[A1]** confirms whether the monotonic fast-path is salvageable as a
preset or retired.

## 6. Retire the mode machinery (S1.7a.5)

- Delete `mode = Mode.SOLVE` vestige in `_explore_layers` (`solver.py:673`)
  and the `mode` parameter on `monotonic_solve` (`solver.py:195,223`).
- `is_solved(kb, mode)` (`verdict.py:116`) loses its **termination** role. If
  a goal matcher is still wanted for §7 projection, keep the matcher body as
  `goal_bindings(kb, goal) -> list[bindings]` and delete the mode-aware
  count logic.
- `Mode` (`verdict.py:59`): demote to a *projection/preset* label or remove.
  If kept, it names "what to print", not "how to search".
- Revert the S1.7.3 completeness-gate experiment (`solve_requires_complete_model`
  flag + the monotonic accept-point edits), per S1.7.3a T4 — **[A1]** inventory
  what's actually on disk first.

## 7. `:goal` projection + CLI answer path (S1.7a.6)

- `goal_bindings(model_kb, goal)` projects the `:goal` pattern over a solution
  node's kb → bindings. For zebra2 extend the goal with the `nation-loc`
  projection so the answer reads "Norwegian/Japanese", not "House-1/House-5"
  (composes with [S1.7.5](../p1.7_bootstrapping_zebra/s1.7.5_query_semantics_who_vs_where.md)).
- CLI: `solve` gains the real **solve** behaviour — run `search`, compute
  `verdict_of`, and on `Unique` render the English answer line; `--mode`
  choices become `{solve,gaps,contradictions}` mapped to (stop preset ×
  projection), reconciling `cli.py:377` with the target.
- `trace/linearize.py` consumes the new verdict (Unique/Ambiguity/
  Contradiction) — the spine is the unique model's firings; reductios are the
  dead commitments; unchanged in shape.

## 8. Test infra + acceptance (S1.7a.7)

- **Remove root `pytest.ini`.** Move any needed config into
  `ein.py/pyproject.toml` `[tool.pytest.ini_options]` (or `ein.py/pytest.ini`)
  so test discovery still works from the package root.
- **`run_tests.sh`** at repo root: runs the whole suite (prefer PyPy when
  `.venv-pypy` exists, fall back to `.venv`/system), passes through args,
  `--help`. Single committed entry point (project convention: a committed
  script, not ad-hoc one-liners).
- **PyPy acceptance tests** (gated or fast as size allows): the three
  variants solved **completely** with expected results (§7 of target), the
  SAT↛⊥ / UNSAT↛Solution invariant, and a zebra2 PyPy-vs-CPython speedup
  assertion/report. **[A1]** sizing decides which run by default vs under
  `EIN_RUN_SLOW`.

## Risk register

| risk | mitigation |
|---|---|
| `zebra2-minus-15` exhaustive search blows up (dropped constraint) | **[A1]** measure first; cap with `max_set_size` + report `¬exhausted` honestly rather than hang |
| dead-end is all-refuted, not open (OQ1) | **[A1]** verify on real branch kb; if all-refuted, it's a contradiction sub-result, handled by `consistent`, still excluded from `S` |
| `state_hash` collision undercounts `k` | low (Phase A I2); optionally back stop-critical dedup with a tuple key, not Python `hash()` |
| big-bang diff breaks the 1000+ test suite | Option A shims keep old entries working during migration; land predicates+dedup first (S1.7a.2) behind the existing entries, then flip |
