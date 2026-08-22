# Ein — Python embedding API

> ### ⚠ This contract has no implementation, and none is scheduled
>
> **`import ein` does not work in this repo.** The Python package these pages
> describe was deleted at M1a
> [S1a.10.5](../../plans/m1a_rust/p1a.10_single_implementation/s1a.10.5_removal.md)
> (2026-08-21), when `ein.rs` became the only engine.
>
> A PyO3 module was to succeed it in
> [P1a.9](../../plans/m1a_rust/p1a.9_release/README.md). **That is deferred as
> of 2026-08-21** — the census found no consumer that needs it, and
> [Q-M1a.23](../../plans/m1a_rust/open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding)
> records the three conditions that would bring it back.
>
> So these pages are **history, held in reserve**: the embedding contract of
> the engine that was, kept whole rather than deleted, because on the day a
> trip-wire fires this is a specification instead of a blank page. Read every
> code block as a record, not as a runnable snippet — and do not "fix" one to
> match ein.rs's internals; they describe something that no longer exists.
>
> **The surfaces that do run** are the CLI — `ein solve <file>` ·
> `ein saturate` · `ein render` — and the crates, whose embedding page
> [S1a.9.4](../../plans/m1a_rust/p1a.9_release/s1a.9.4_documentation.md)
> writes.

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

**That conditional resolved twice.** The [M1a Rust
port](../../plans/m1a_rust/README.md) made `ein.rs` the only engine, and these
pages were briefly a *specification*: the contract a PyO3 successor would have
to satisfy. Then
[P1a.9](../../plans/m1a_rust/p1a.9_release/README.md) deferred that successor
(2026-08-21) for want of a consumer, and the pages settled into their third
and current reading — **a record**. Nothing checks them, because there is
nothing to check them against; nothing is scheduled to implement them, because
no planned workload needs it
([Q-M1a.23](../../plans/m1a_rust/open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding)).

They are kept for one reason and it is a good one: a deferral is cheap to
reverse only while the specification survives it. If a trip-wire fires, this
subtree is where the work starts, and the two decisions that were *forced*
when it was written are now free — the published name, and whether the
exception hierarchy keeps class names that
[`defined_behaviour.md` §4](../kernel/defined_behaviour.md) calls "a name
without a referent".
