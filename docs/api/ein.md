# `ein` — the embedding contract

The Python surface for embedding Ein in another project: **parse** a
`.ein` source → **load** it into a `KnowledgeBase` → (optionally
**saturate**) → **solve** → **read** the verdict and its explanation.

> **Audience: embedders** (downstream Python users). The IR-language
> contract for *authoring* puzzles is [`docs/kernel/`](../kernel/); the
> engine *internals* are
> [`docs/kernel/inference/python_impl.md`](../kernel/inference/python_impl.md).
> This page deliberately stops at the public surface — it never reaches
> into the matcher, compiler, hypothesis generator, or CDCL machinery.

*Verified against commit `60c192b` (2026-06-16) — the worked example below
was run end-to-end against [`examples/zebra2.ein`](../../examples/zebra2.ein).*

## The five steps

| step | entry symbol | import |
|------|--------------|--------|
| 1. parse | `parse(text, *, filename=None) -> tuple[SForm, ...]` | `from ein.ir import parse` |
| 2. load | `KnowledgeBase.from_file(path)` / `.from_ir(forms, *, base_dir=None)` / `ein.kb.load(forms, *, base_dir=None)` | `from ein.kb.store import KnowledgeBase` |
| 3. saturate *(optional)* | `Saturator(kb).saturate(*, max_steps=None) -> Iterator[Firing]` | `from ein.inference.saturator import Saturator` |
| 4. solve | `solve(root_kb, *, stop_after=None, max_set_size=5, config=None, …) -> tuple[Verdict \| Aborted, MonotonicStats]` | `from ein.inference.monotonic import solve` |
| 5. read | `goal_bindings(kb)`, the `Solution`/`Ambiguity`/`Contradiction` fields, `Provenance`, `ein.trace.linearize` | `from ein.inference.verdict import …` / `from ein.trace import …` |

Per-symbol detail lives in the module pages:
[`ir.md`](ir.md) · [`kb.md`](kb.md) · [`inference.md`](inference.md) ·
[`trace.md`](trace.md).

### 1 — Parse

[`ein.ir.parse`](ir.md) turns S-expression source text into a tuple of
typed AST forms (`SForm`s). It does **not** read files or resolve
`(import …)` — that is the loader's job (step 2). Most embedders never
touch the AST directly; `KnowledgeBase.from_file` calls `parse` for you.

```python
from ein.ir import parse
forms = parse("(relation likes Person Thing)\n(likes Alice Tea)")
# (SForm(head=Atom('relation'), …), SForm(head=Atom('likes'), …))
```

`parse` raises [`IRParseError`](ir.md) on malformed input.

### 2 — Load into a `KnowledgeBase`

The KB is the in-memory graph the engine reasons over (see
[`kb.md`](kb.md)). Three ways in, in order of convenience:

- **`KnowledgeBase.from_file(path)`** — read a `.ein` file and resolve its
  `(import std.* …)` / file-relative imports against the file's directory.
  **Use this for real puzzles** — `zebra2.ein` imports the stdlib, and
  only `from_file` (or `from_ir(..., base_dir=…)`) resolves those.
- **`KnowledgeBase.from_ir(forms, *, base_dir=None)`** — build from
  already-parsed forms. `base_dir=None` resolves only `std.*` imports
  (not file-relative ones).
- **`ein.kb.load(forms, *, base_dir=None)`** — the underlying function the
  two classmethods delegate to.

```python
from ein.kb.store import KnowledgeBase
kb = KnowledgeBase.from_file("examples/zebra2.ein")
# kb.facts → 75 facts; kb.rules → 30 rules; kb.relations → 17;
# kb.query is the (query …) goal; kb.config is the parsed (config …) block.
```

A malformed load raises [`KBLoadError`](kb.md) with the accumulated problems.

### 3 — Saturate (optional)

[`Saturator`](inference.md) runs the rule engine to a fixed point —
forward-chaining every rule until no new fact is derived. You rarely call
this directly: **`solve` saturates internally**. Reach for `Saturator`
only when you want the deductive closure *without* the hypothesis search
(e.g. to inspect what the monotonic rules alone derive).

```python
from ein.inference.saturator import Saturator
firings = list(Saturator(kb).saturate())   # Iterator[Firing] drained to a list
```

### 4 — Solve

[`solve`](inference.md) is the **single engine entry**. It runs the
set-indexed lattice search and returns `(verdict, stats)`. The verdict
*type* is read from the count `k` of distinct solution nodes found — it is
**not** selected by a `mode=` argument (there is none):

- `k == 1` → [`Solution`](inference.md) — a model.
- `k > 1` → [`Ambiguity`](inference.md) — `k` distinct models (a gap).
- `k == 0` → [`Contradiction`](inference.md) — unsat (when exhausted).

`stop_after` is the orthogonal stop policy: `stop_after=1` is the sound
fast path (stop at the first complete, consistent model; `stats.exhausted`
is then `False`, so a `k=1` reads as "*a* model", not certified-unique);
`stop_after=None` exhausts the lattice (certifies unique / ambiguous /
unsat). Engine config (`SolverConfig`) and budgets (`max_time`,
`max_enterings`) are documented in [`inference.md`](inference.md).

```python
from ein.inference.monotonic import solve
verdict, stats = solve(kb, stop_after=1)
# stats.solution_nodes == k ; stats.exhausted says whether the lattice was
# fully explored.
```

### 5 — Read the verdict

Branch on the verdict type. The headline answer is the query goal's
bindings, read with [`goal_bindings`](inference.md):

```python
from ein.inference.verdict import Solution, Ambiguity, Contradiction, goal_bindings

if isinstance(verdict, Solution):
    rows = goal_bindings(verdict.kb)          # [{var: value, …}, …]
    for f in verdict.trace:                   # tuple[Firing, …] — the derivation
        ...                                   # f.rule, f.derived, f.premises, f.bindings
elif isinstance(verdict, Contradiction):
    core = verdict.unsat_core                 # frozenset[Fact] — the conflicting facts
elif isinstance(verdict, Ambiguity):
    models = verdict.branches                 # tuple[Solution, …]
```

Every derived [`Fact`](kb.md) carries [`Provenance`](kb.md)
(`fact.provenance.kind` ∈ `source` / `rule` / `hypothesis` / `rejected`,
plus the firing `rule` and `premises_raw`) — the substrate for the
human-readable explanation in step 6.

### 6 — Explain (optional)

[`ein.trace`](trace.md) turns a verdict into a markdown narrative. The
linearised trace needs the lattice proof, so solve with
`store_lattice=True`:

```python
from ein.trace import linearize, render_markdown
verdict, _ = solve(kb, stop_after=1, store_lattice=True)
markdown = render_markdown(linearize(verdict), diagrams=False)
```

## Worked example — solving `zebra2.ein`

A complete, copy-pasteable script. Run it from the repo root under the
project interpreter (`PYTHONPATH=ein.py/src .venv-pypy/bin/python script.py`,
or the CLI equivalent `./ein_pypy.sh solve examples/zebra2.ein`).

```python
import time
from ein.kb.store import KnowledgeBase
from ein.inference.monotonic import solve
from ein.inference.verdict import Solution, Ambiguity, Contradiction, goal_bindings
from ein.trace import linearize, render_markdown

PUZZLE = "examples/zebra2.ein"

# 1-2. parse + load (from_file resolves the (import std.*) forms)
kb = KnowledgeBase.from_file(PUZZLE)
print(f"loaded: {len(kb.facts)} facts, {len(kb.rules)} rules, "
      f"{len(kb.relations)} relations")

# 4. solve (fast path — stop at the first complete, consistent model)
t0 = time.time()
verdict, stats = solve(kb, stop_after=1)
print(f"{type(verdict).__name__} in {time.time() - t0:.1f}s  "
      f"(k={stats.solution_nodes}, exhausted={stats.exhausted})")

# 5. read the verdict
if isinstance(verdict, Solution):
    for row in goal_bindings(verdict.kb):
        print("answer:", row)
elif isinstance(verdict, Contradiction):
    print("unsat core:", sorted(f"({f.relation_name} {' '.join(map(str, f.args))})"
                                 for f in verdict.unsat_core))
elif isinstance(verdict, Ambiguity):
    print(f"{len(verdict.branches)} distinct models")

# 6. explain (re-solve with the proof attached, then render)
verdict, _ = solve(KnowledgeBase.from_file(PUZZLE), stop_after=1, store_lattice=True)
print(render_markdown(linearize(verdict), diagrams=False).split("\n\n")[0])
```

Actual output (commit `60c192b`, PyPy, ~2 s):

```text
loaded: 75 facts, 30 rules, 17 relations
Solution in 2.0s  (k=1, exhausted=False)
answer: {'h_water': 'House-1', 'who_water': 'Norwegian', 'h_zebra': 'House-5', 'who_zebra': 'Japanese'}
# Solution trace
```

The answer — *the Norwegian drinks water (House-1), the Japanese owns the
zebra (House-5)* — is the canonical Zebra solution. The same solve is what
[`examples/README.md`](../../examples/README.md) annotates as the M1 target
trace.

## Why no `import ein`

`ein/__init__.py` exposes only `__version__`; the embedding contract is
spread across the subpackages. This is deliberate for M1: P1.20 is a
**documentation** phase over a frozen kernel, and adding a top-level
re-export facade (so `import ein; ein.solve(...)` would work) is a *code*
change, however small. It is noted as a one-line post-M1 follow-up. Until
then, import from `ein.ir` / `ein.kb` / `ein.inference` / `ein.trace`
directly, as every example here does.

> **Heads-up:** `ein.inference.__init__` re-exports only `predicates`, so
> `from ein.inference import solve` does **not** work — `solve` lives in
> `ein.inference.monotonic`, and `Saturator` in `ein.inference.saturator`.

## What's *not* the contract

The engine internals — the matcher (`match.py`), the per-rule compiler
(`compile.py`), the hypothesis generator (`hypgen.py`), the contradiction
detector, the CDCL no-goods, and the `monotonic/` lattice driver's private
helpers — are **not** part of the embedding surface. They change between
releases. To understand them, read
[`docs/kernel/inference/python_impl.md`](../kernel/inference/python_impl.md)
(the file-by-file map) and
[`architecture_and_algorithms.md`](../kernel/inference/architecture_and_algorithms.md)
(the language-agnostic algorithm view).

## See also

- [`docs/api/README.md`](README.md) — the api-subtree index.
- [`docs/kernel/architecture.md`](../kernel/architecture.md) — the
  data-flow / package-dependency map this surface is the programmatic face of.
- [`examples/README.md`](../../examples/README.md) — the M1 target trace
  this worked example reproduces.
- [`utils/profile_solve.py`](../../utils/profile_solve.py) /
  [`utils/symmetric_bench.py`](../../utils/symmetric_bench.py) — the
  promoted engine runners; the same load → solve / load → saturate calls.
