# Ein — the embedding API

How to drive Ein **as a library** rather than as a command. One page.

| page | subject | status |
|---|---|---|
| **[`rust.md`](rust.md)** | **The crates** — `ein-ir` to load, `ein-infer` to solve, `ein-render` to explain, `ein-einb` to cache. **Start here.** | live; its example is a test the gate runs |

> **Audience: embedders.** If you want to **author puzzles** in the
> S-expression language, read [`docs/kernel/`](../kernel/) — the grammar, the
> kernel API, the stdlib. If you want the **engine internals**, read
> [`docs/kernel/inference/implementation.md`](../kernel/inference/implementation.md).
> If you want to **run** ein rather than embed it,
> [`docs/install.md`](../install.md).

## There is no binding for another language

Nothing is scheduled to add one. A PyO3 module was **deferred** for want of a
consumer — [M20](../../plans/m20_gui/README.md) links the crates,
[M10](../../plans/m10_external_benchmarks/README.md)'s benchmark runner must
shell out to be a fair measurement, and
[M2](../../plans/m2_nl_to_ir/README.md)'s premise turned out to be an HTTP
server rather than a C library.
[Q-M1a.23](../history/m1a_rust/open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding)
holds the three trip-wires that would revive it.

To drive ein from another language **today**, run the binary and read its
output: `--json-summary` for the verdict, the model and the counters as data,
`--events` for the [step-by-step narration](../kernel/inference/events.md).

## Why the example is a test

Verification with a date on it is a claim about the past: a page checked once
against a commit goes stale the day the engine moves, because nothing runs it.
[`rust.md`](rust.md)'s worked example is the region of
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
