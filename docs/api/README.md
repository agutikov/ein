# Ein — Python embedding API

How to drive Ein **as a library from another Python project**: load a
`.ein` puzzle, run the engine, and read the answer + its explanation.

> **Audience: embedders** (downstream Python users). This subtree is the
> *programmatic* contract — the Python functions and classes you import.
> If you instead want to **author puzzles** in the S-expression language,
> read [`docs/kernel/`](../kernel/) (the IR grammar, kernel API, stdlib).
> If you want the **engine internals**, read
> [`docs/kernel/inference/python_impl.md`](../kernel/inference/python_impl.md).

This is the programmatic face of the same pipeline
[`docs/kernel/architecture.md`](../kernel/architecture.md) diagrams:
`.ein source → parse → KnowledgeBase → saturate / solve → verdict → trace`.

## Pages

| page | covers |
|------|--------|
| **[`ein.md`](ein.md)** | The **embedding contract** — the five-step flow (parse → load → saturate → solve → read) and a complete, verified worked example on `zebra2.ein`. **Start here.** |
| [`ir.md`](ir.md) | `ein.ir` — `parse`, the AST nodes, `dump*` round-trip. |
| [`kb.md`](kb.md) | `ein.kb` — `KnowledgeBase` (construction + read surface), the entity dataclasses (`Fact`, `Relation`, `Rule`, `Layer`), `Provenance` / `DerivationDAG`. |
| [`inference.md`](inference.md) | `ein.inference` — `Saturator`, `solve`, the `Verdict` types, `SolverConfig` knobs, `Firing`. |
| [`trace.md`](trace.md) | `ein.trace` — `linearize` + `render_markdown`, the answer renderers, the `Trace` AST. |

## The 30-second version

```python
from ein.kb.store import KnowledgeBase
from ein.inference.monotonic import solve
from ein.inference.verdict import Solution, goal_bindings

kb = KnowledgeBase.from_file("examples/zebra2.ein")   # parse + resolve imports
verdict, stats = solve(kb, stop_after=1)              # the one engine entry
if isinstance(verdict, Solution):
    print(goal_bindings(verdict.kb))
    # [{'h_water': 'House-1', 'who_water': 'Norwegian',
    #   'h_zebra': 'House-5', 'who_zebra': 'Japanese'}]
```

There is **no top-level `ein` facade**: `import ein` gives only
`__version__`. Import from the subpackages (`ein.ir`, `ein.kb`,
`ein.inference`, `ein.trace`) as above. See
[`ein.md` § Why no `import ein`](ein.md#why-no-import-ein) for the rationale.

## Stability

Verified against commit **`60c192b`** (2026-06-16). The IR/kernel surface
([`docs/kernel/`](../kernel/)) is locked by M1; this *Python* surface is
less frozen — engine internals move under it. Each page carries a
"verified for" SHA; if a signature here disagrees with the source, the
source wins (file an issue). If the [M1a Rust port](../../plans/m1a_rust/README.md)
ships, the embedding contract moves to ein.rs (PyO3 + native Rust) and
this becomes the legacy Python reference.
