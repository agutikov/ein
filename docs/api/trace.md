# `ein.trace` — explanation rendering

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

Turn a solver [`Verdict`](inference.md) into a human-readable markdown
narrative — the project's main human-facing output (idea 08). The engine
behind it is [`ein-render/trace/`](../../ein.rs/crates/ein-render/src/trace/).

> **Audience: embedders.** Use this to surface *why* the engine reached its
> answer. The rendered output is what a downstream UI shows the user.

*Verified against commit `60c192b` (2026-06-16) — **against the Python engine, which no longer exists**. These signatures are a record of what that engine offered, not a description of anything in the tree and no longer a contract anything is scheduled to implement ([Q-M1a.23](../../plans/m1a_rust/open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding)).*

## The two-step render

A verdict's linearised trace needs the lattice proof, so **solve with
`store_lattice=True`** first, then `linearize` → `render_markdown`:

```python
from ein.inference.monotonic import solve
from ein.trace import linearize, render_markdown

verdict, _ = solve(kb, stop_after=1, store_lattice=True)
markdown = render_markdown(linearize(verdict), diagrams=False)
```

### `linearize(verdict) -> Trace`

Turn the engine's *unordered* commitment lattice into a depth-ordered
[`Trace`](#trace): the spine (the primary solution's firings, smallest
commitment first) + one reductio per refuted commitment + the closing
lattice DAG and solution grid. Reads `verdict.proof`, so the verdict must
come from a `store_lattice=True` solve.

### `render_markdown(trace, *, mode="engine", diagrams=True) -> str`

Render a `Trace` as a self-contained markdown string: a numbered step per
firing (rule name, English `:why`, premises with quoted source sentences),
refuted hypotheses folded into `<details>` reductios, and a closing
lattice-DAG + solution grid.

- `mode="engine"` (default) — numbered engine order.
- `mode="reorder"` — steps clustered by the entity they are about
  (`## About <X>`).
- `diagrams=True` (default) — embed inline fenced `dot` derivation slices;
  `False` omits them (faster, text-only).

## Answer renderers

For just the answer (not the full derivation):

- `render_solution_table(verdict, …)` — the five fields the CLI prints:
  `solutions (k)` · `verdict` · `query bindings` · `rendered query facts`
  · `NL result`. All English comes from *puzzle-authored* templates
  (`(relation … :why)` / `(query … :goal-text)`); there is no hardcoded
  relation→verb vocabulary.
- `render_answer(solution, …)` — the one-line NL headline (the result row).

## The `Trace` AST

### `Trace`

The linearised solve, ready for rendering:

| field | meaning |
|-------|---------|
| `steps` | `list[TraceStep]` — the solution spine. |
| `reductios` | `list[Reductio]` — refuted branches. |
| `summary`, `commitment` | the headline + the primary solution's assumed hypotheses. |
| `solved`, `n_solutions` | goal-satisfied flag + the solution count. |
| `lattice_dot`, `solution_dot`, `full_kb_dot` | the closing diagrams (DOT). |

### `TraceStep`

One firing as a narrative step (round-trips through the parser as a
`(trace …)` form via `trace_to_ir` / `parse_trace_steps`): `.n`, `.rule`,
`.why`, `.premise_labels()`, `.sources`, `.derived_label`, `.diagram`.

### `Reductio`

A refuted hypothesis, rendered as a foldable `<details>`: `.summary`,
`.commitment`, `.learned_clause`, `.diagram`.

## See also

- [`ein.md`](ein.md) — the end-to-end flow (step 6).
- [`inference.md`](inference.md) — the verdict types this consumes;
  `store_lattice` is what makes a verdict linearisable.
- [the Zebra walkthrough](../kernel/inference/zebra_walkthrough.md) — the human Zebra
  walkthrough this narrative aims to be "recognisably equivalent" to.
