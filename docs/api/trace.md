# `ein.trace` — explanation rendering

Turn a solver [`Verdict`](inference.md) into a human-readable markdown
narrative — the project's main human-facing output (idea 08). Source:
[`ein.py/src/ein/trace/`](../../ein.py/src/ein/trace/).

> **Audience: embedders.** Use this to surface *why* the engine reached its
> answer. The rendered output is what a downstream UI shows the user.

*Verified against commit `60c192b` (2026-06-16).*

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
