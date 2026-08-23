# Ein — the embedding API

How to drive Ein **as a library** rather than as a command. One page describes
a surface that exists; five are history.

| page | subject | status |
|---|---|---|
| **[`rust.md`](rust.md)** | **The crates** — `ein-ir` to load, `ein-infer` to solve, `ein-render` to explain, `ein-einb` to cache. **Start here.** | live; its example is a test the gate runs |
| [`ein.md`](ein.md) | the Python embedding contract — the five-step flow and a worked example | 🏛 history |
| [`ir.md`](ir.md) | `ein.ir` — `parse`, the AST nodes, `dump*` round-trip | 🏛 history |
| [`kb.md`](kb.md) | `ein.kb` — `KnowledgeBase`, `Fact` / `Relation` / `Rule`, `Provenance` | 🏛 history |
| [`inference.md`](inference.md) | `ein.inference` — `Saturator`, `solve`, the verdict types, `SolverConfig` | 🏛 history |
| [`trace.md`](trace.md) | `ein.trace` — `linearize` + `render_markdown` | 🏛 history |

> **Audience: embedders.** If you want to **author puzzles** in the
> S-expression language, read [`docs/kernel/`](../kernel/) — the grammar, the
> kernel API, the stdlib. If you want the **engine internals**, read
> [`docs/kernel/inference/implementation.md`](../kernel/inference/implementation.md).
> If you want to **run** ein rather than embed it,
> [`docs/install.md`](../install.md).

## There is no Python module

`import ein` does not work in this repository and nothing is scheduled to make
it work. The package was deleted at M1a
[S1a.10.5](../../plans/m1a_rust/p1a.10_single_implementation/s1a.10.5_removal.md)
(2026-08-21); the PyO3 module that was to succeed it was **deferred the same
day** for want of a consumer —
[M20](../../plans/m20_gui/README.md) links the crates,
[M1c](../../plans/m1c_external_validation/README.md)'s benchmark runner must
shell out to be a fair measurement, and
[M2](../../plans/m2_nl_to_ir/README.md)'s premise turned out to be an HTTP
server rather than a C library.
[Q-M1a.23](../../plans/m1a_rust/open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding)
holds the three trip-wires that would revive it.

To drive ein from Python **today**, run the binary and read its output:
`--json-summary` for the verdict, the model and the counters as data,
`--events` for the [step-by-step narration](../kernel/inference/events.md).

## Why the five pages are kept

They were a good contract — five steps, a worked example with real numbers,
per-symbol tables — and a deferral is cheap to reverse only while the
specification survives it. If a trip-wire fires, this subtree is where that
work starts, and two decisions that were *forced* when it was written are now
free: the published name, and whether the exception hierarchy keeps class
names that [`defined_behaviour.md` §4](../kernel/defined_behaviour.md) calls "a
name without a referent".

They are also the reason [`rust.md`](rust.md)'s example is a test. Those pages
were **verified**, once, against commit `60c192b` (2026-06-16) — and then the
engine moved and nothing noticed, because nothing ran them. Verification with
a date on it is a claim about the past. The Rust page's worked example is the
region of
[`ein.rs/crates/ein-cli/tests/embedding.rs`](../../ein.rs/crates/ein-cli/tests/embedding.rs)
between two markers, compared text-to-text by a test in that same file, so
`cargo test --workspace` is what keeps it true. **Do not edit `rust.md`'s code
block by hand** — edit the test, run it, paste.

## The pipeline, either way

Both surfaces are faces of the same flow
[`docs/kernel/architecture.md`](../kernel/architecture.md) diagrams:

```text
.ein source → parse → KnowledgeBase → saturate → solve → verdict → trace
```

What differs is only who holds the arenas. The CLI does it in `ein-cli`; an
embedder does it in three lines.
