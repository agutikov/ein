# P1.7a — Target design (the ideal end-state)

What we want the engine to be once P1.7a lands. This is the *destination*;
[`analysis.md`](analysis.md) measures the distance from today and
[`refactoring_design.md`](refactoring_design.md) plots the route.

## 1. One definition: a solution node

A **solution node** is the engine's single, domain-agnostic notion of "an
answer". For a knowledge base `kb` reached by committing a hypothesis set and
saturating:

```
solution_node(kb)  ⟺  consistent(kb)  ∧  complete(kb)
```

- **`consistent(kb)`** — no contradiction: no `(false)`, no `X ∧ ¬X`
  (`ContradictionDetector(kb).detect()` is empty).
- **`complete(kb)`** — **no open hypothesis**: the hypothesis generator
  proposes nothing that isn't already decided.
  `complete(kb) ≡ not has_open_hypothesis(kb) ≡ not _compute_alive(kb)`.
  An *open* hypothesis `h = (R a b)` is one that is neither asserted
  (`present`) nor refuted (`(not h)` asserted). Built **only** on
  `generate_hypotheses` + the KB's positive/negated fact sets — no `is-a`,
  no `total`, no relation signatures (those are encoding-specific and were
  the source of the first patch's wrongness).

The set of solution nodes from a run is **deduped by `state_hash`** — two
commitments that saturate to the same propositional state are the *same*
answer, counted once.

> A solution node is **always the same thing**, independent of why we are
> searching. `solve`, `gaps`, and `contradictions` do not change what a
> solution *is* — only how many we look for and what we print.

This is the definition that dissolves the bug: the partial dead-end
(`color-loc Green House-4`, 7 cells wrong) has open hypotheses, so it is
**not complete**, so it is **not a solution node** — no matter that the
`:goal` pattern happens to match it.

## 2. One search

The engine runs **one** procedure: the P1.5b set-indexed lattice exploration
(Apriori prefix-join over commitment sets → `try_commitment_set` → saturate →
classify), accumulating the **deduped set of solution nodes** `S` and the set
of **dead commitments** `D` (with their unsat cores + learned no-goods).

The search is **exhaustive by default** (bounded only by `max_set_size` and
resource budgets). It does **not** terminate on `:goal` match; `:goal` is not
consulted during search at all (see §5). The per-branch commitment kb is the
unit of truth — never the shared monotonic root (which is unsound under full
search, Phase A I5).

```
run(kb, stop, budgets) -> RunResult
    S : set[SolutionNode]   # consistent ∧ complete, deduped by state_hash
    D : list[DeadCommitment] # with unsat_core + learned_clause
    exhausted : bool         # did we reach all leaves within max_set_size?
    stopped_by : "exhausted" | "stop_after" | "budget"
```

## 3. The result type is read from `k = |S|`

The verdict is a **pure function of the run result**, not a mode chosen up
front:

| condition | verdict | meaning |
|---|---|---|
| `exhausted ∧ k = 1` | **Unique** | the single model; the puzzle is solved |
| `exhausted ∧ k > 1` | **Ambiguity** | `k` distinct models; the *gap* = residual open set (hypotheses true in some models, false in others) |
| `exhausted ∧ k = 0` | **Contradiction** | no model; unsat core = ⋃ dead cores (minimised) |
| `¬exhausted ∧ k ≥ 1` | **Solution(s), uniqueness uncertified** | stopped early or truncated; we have `k` real models but did not prove there are no more |
| `¬exhausted ∧ k = 0` | **Unknown** | truncated by `max_set_size`/budget before any model or full refutation — **NOT** UNSAT |

The bottom two rows are the honesty the current engine lacks: `k = 0` means
UNSAT **only if the search was exhaustive** (S1.7.3a Q-B). A truncating
`max_set_size` must never masquerade as `Contradiction`.

The three idea-03 task classes are now **three readings of this table**, not
three engines:

- **SOLVE** = "is there a model, and is it unique?" → reads rows 1/4 (and
  projects the `:goal` answer).
- **GAPS** = "what's under-determined?" → reads row 2 and reports the residual
  open set.
- **CONTRADICTIONS** = "why is it unsat?" → reads row 3 and reports the cores.

## 4. The stop policy is orthogonal and user-chosen

Stopping is a **convenience knob**, independent of the result type:

```
stop_after : int | None      # None = exhaust; 1 = first; 42 = first 42 …
# plus the existing resource budgets, unchanged:
max_set_size : int           # lattice depth cap (structural)
max_time : float | None      # wall-clock
max_enterings : int | None   # try_commitment_set count
```

- `stop_after = 1` is the fast "give me an answer" path — the **sound**
  replacement for today's `monotonic_solve` early-terminate. It stops on the
  first *complete ∧ consistent* node (never on a partial goal-match).
- `stop_after = 42` is a first-class request ("show me 42 models").
- `stop_after = None` (exhaust) is what uniqueness/gap/unsat certification
  requires.

Stopping early sets `exhausted = False`, which (per §3) caps the strongest
claim at "here are `k` models, more may exist" — you cannot get **Unique** or
**Contradiction** without exhausting. This coupling is the whole point: the
stop policy and the result type meet *only* through `exhausted`.

## 5. `:goal` is a projection, not a driver

`:goal` stops being a termination signal. It becomes a **query projected over
the solution node(s)** at the end:

- On a **Unique** result, project the goal bindings from the one model →
  the answer in words ("Norwegian drinks water; Japanese keeps the zebra").
- On **Ambiguity**, project over each model → report which goal answers vary
  (this is a sharp form of "the gap").
- A puzzle that intends `:goal` as a *constraint* (not a question) pushes it
  into `(facts …)` / `(rules …)` instead — the projection vs constraint
  distinction is [S1.7.5](../p1.7_bootstrapping_zebra/s1.7.5_query_semantics_who_vs_where.md)'s
  concern (who-vs-where). P1.7a treats `:goal` as a question by default.

## 6. The public API we want

One core, thin ergonomic presets — names survive, semantics change:

```python
# inference/monotonic/solver.py (or a neutral inference/search.py)

def search(kb, *, stop_after=None, max_set_size=5,
           config=None, dumper=None,
           max_time=None, max_enterings=None) -> tuple[Result, Stats]:
    """The one sound search. Returns a Result carrying the deduped
    solution-node set S, dead commitments D, and `exhausted`."""

# Verdict is computed from the Result, not chosen:
def verdict_of(result) -> Verdict:  # Unique | Ambiguity | Contradiction | Uncertified | Unknown
    ...

# Ergonomic presets (stop + projection), NOT separate engines:
def solve(kb, **kw):           # stop_after=1 fast-path OR exhaust to certify unique
def all_solutions(kb, **kw):   # stop_after=None; the gaps reading
def refutations(kb, **kw):     # stop_after=None; the contradictions reading
```

The current `monotonic_solve` / `gaps_solve` / `contradictions_solve` names
may be retained as **deprecated thin shims** over `search` + `verdict_of` for
one migration window, or renamed — see
[`refactoring_design.md`](refactoring_design.md). The non-negotiable is that
they no longer host *distinct searches*; they fix `stop_after` and a
projection over the single core.

### What this kills

- `is_solved(kb, mode)` as a **termination** signal — gone from the loop.
  (A goal-projection helper may keep the matcher, but it never decides when
  to stop.)
- `Mode` as a **search selector** — gone. If a `Mode`-like enum survives, it
  names a *projection/preset*, not a search.
- `monotonic_solve`'s shared-root forced-positive promotion as the answer
  source — the answer is a per-branch solution node.
- Over-counting: `gaps_solve` returning 15 records for 1 model — solution
  nodes are deduped by `state_hash`, so zebra2 → `k = 1`.

## 7. Worked end-states (the acceptance)

| input | run | verdict | projection |
|---|---|---|---|
| `zebra2.ein` | exhaust; `S` dedups 14+1 raw → **k=1** (the dead-end is incomplete, excluded) | **Unique** | "Japanese/zebra; Norwegian/water" |
| `zebra2-minus-15.ein` | exhaust; **k>1** | **Ambiguity** | residual open set; depends on dropped (15) |
| `zebra2-bad.ein` | exhaust; **k=0** | **Contradiction** | 2–3 edge core: injected fact ⊕ (6) |

All three are *one* `search(kb, stop_after=None)` run; only `verdict_of` +
the projection differ. `solve(zebra2)` with `stop_after=1` returns the same
model faster but reports it as "a solution" unless it also exhausts to certify
uniqueness — which, for a uniquely-solvable puzzle, it can, cheaply, because
the residual search collapses once the model's facts are forced.
