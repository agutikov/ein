# Ein — Python embedding API

> ### ⚠ This contract has no implementation right now
>
> **`import ein` does not work in this repo.** The Python package these pages
> describe was deleted at M1a
> [S1a.10.5](../../plans/m1a_rust/p1a.10_single_implementation/s1a.10.5_removal.md)
> (2026-08-21), when `ein.rs` became the only engine.
>
> The contract is not obsolete — it is the **specification** the PyO3 module
> [S1a.9.1](../../plans/m1a_rust/p1a.9_bindings_release/s1a.9.1_pyo3_surface.md)
> builds has to satisfy. What checks it is
> [S1a.9.2](../../plans/m1a_rust/p1a.9_bindings_release/s1a.9.2_api_parity_tests.md);
> what re-verifies these pages against the real module, sample by sample, is
> [S1a.9.4](../../plans/m1a_rust/p1a.9_bindings_release/s1a.9.4_documentation.md).
> Until those land, read every code block here as a contract rather than as a
> runnable snippet. The surface that *does* run today is the CLI:
> `ein solve <file>` · `ein saturate` · `ein render`.

How to drive Ein **as a library from another Python project**: load a
`.ein` puzzle, run the engine, and read the answer + its explanation.

> **Audience: embedders** (downstream Python users). This subtree is the
> *programmatic* contract — the Python functions and classes you import.
> If you instead want to **author puzzles** in the S-expression language,
> read [`docs/kernel/`](../kernel/) (the IR grammar, kernel API, stdlib).
> If you want the **engine internals**, read
> [`docs/kernel/inference/implementation.md`](../kernel/inference/implementation.md).

This is the programmatic face of the same pipeline
[`docs/kernel/architecture.md`](../kernel/architecture.md) diagrams:
`.ein source → parse → KnowledgeBase → saturate / solve → verdict → trace`.

## Pages

| page | covers |
|------|--------|
| **[`ein.md`](ein.md)** | The **embedding contract** — the five-step flow (parse → load → saturate → solve → read) and a complete, verified worked example on `zebra2.ein`. **Start here.** |
| [`ir.md`](ir.md) | `ein.ir` — `parse`, the AST nodes, `dump*` round-trip. |
| [`kb.md`](kb.md) | `ein.kb` — `KnowledgeBase` (construction + read surface), the entity dataclasses (`Fact`, `Relation`, `Rule`), `Provenance` / `DerivationDAG`. |
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

Verified against commit **`60c192b`** (2026-06-16), against the Python
engine. The IR/kernel surface ([`docs/kernel/`](../kernel/)) is locked by M1;
this *Python* surface was always the less frozen one — engine internals moved
under it.

**That conditional has resolved.** The [M1a Rust
port](../../plans/m1a_rust/README.md) shipped, `ein.rs` is the only engine, and
the embedding contract moves to a PyO3 module rather than staying a legacy
Python reference. So the reading of these pages inverts: they no longer
*describe* an implementation and are checked against it — they **specify** one,
and [S1a.9.2](../../plans/m1a_rust/p1a.9_bindings_release/s1a.9.2_api_parity_tests.md)
is what will hold the module to them. Where a page and the module end up
disagreeing, that is a defect to file against one of them, not a doc to
refresh quietly.
