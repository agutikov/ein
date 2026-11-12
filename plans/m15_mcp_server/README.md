# M15 — An Ein MCP server

**Status:** planned 2026-11-12, on the user's note of that day. Not started. **A placeholder:** the note is
one line, and this page adds no design to it. It carries no estimate (the user's ruling of 2026-11-08).
**Depends on:** the engine that ships, [`ein.rs`](../../ein.rs/) ([M1a](../../docs/history/m1a_rust/README.md)).
Nothing else is known to be needed.
**Blocks:** nothing.

## The instruction

The user, 2026-11-12, in
[`questions.md`](../m1f_hypothesis_and_documentation/p1f.6_calculator/questions.md) of the calculator's
phase, in full:

> TODO: new milestone M15 - Ein MCP server

That is all the user said. The number is the user's. *MCP* is read as the Model Context Protocol, by which
a language model's client calls tools that a server offers; the note does not say so, and the reading is
the first thing to confirm.

## What would have to be decided

Listed so the milestone can be planned when the user takes it up. **None of these is decided, and none has
a default.**

- **What it is for.** Who the client is (an assistant writing Ein programs; an agent solving a puzzle; the
  formalizer of [M2](../m2_nl_to_ir/README.md), whose loop feeds the kernel's output back to a model), and
  so what a tool call has to return to be useful to it.
- **Which commands are tools.** The binary has five subcommands, `ein {oracle,render,saturate,solve,test}`,
  and `ein kb`. Which of them a server exposes, and with which of their options: a bare `ein solve`
  describes R; `--single`, `--minimal`, `--maximal`, `-n N` and `--count` each ask a narrower question;
  `--trace` and `--dump-answer` are the explanations. A tool per question is one shape and a tool per
  subcommand another.
- **What a tool takes.** Program text in the call, a path on the server's disk, or both; how a program
  that imports a file-relative module is given; whether the stdlib is the embedded copy.
- **What a tool returns.** The machine surfaces that exist today are the candidates, and whether they
  suffice is the question:
  - `--json-summary`, the `ein-summary/2` object, which is also `--dump-answer`'s `answer.json`
    ([`answer_dump.md`](../../docs/kernel/inference/answer_dump.md));
  - the event protocol, `--events`
    ([`events.md`](../../docs/kernel/inference/events.md)), a stream and not a result;
  - `ein test --json-report`;
  - the text read-out ([`read_out.md`](../../docs/kernel/inference/read_out.md)) and a parse error's
    lines, which are written for a reader and may be what a model reads best.
- **How it is built on the engine.** As a process around the binary, or against the crates:
  [`docs/api/rust.md`](../../docs/api/rust.md) is how Ein is driven as a library (`ein-ir` to load,
  `ein-infer` to solve, `ein-render` to explain), and it is what [M20](../m20_gui/README.md) binds
  against. Server mode was dropped from M1a on 2026-08-18 and the Python binding deferred on 2026-08-21 for
  want of a consumer; this milestone would be such a consumer, and that history is read before either is
  revived.
- **Limits.** A run can be long and large (the region budget, the calculator's memory): what a server does
  with a call that does not end, and what it says it established.
- **Where it lives** in the tree, in which language, and how it is tested and released.
- **Where it sits** against the other milestones.

## What it must not assume

- **That an answer's meaning changes with the surface.** One tool, consistent answers (the user's ruling of
  2026-10-16): whatever a server returns is what the command line returns for the same question.
- **That a new surface is free.** A new output format is a contract with a reference page and a test, as
  the read-out and the summary have.

## Connections

- [M20](../m20_gui/README.md), the GUI: the other consumer of the crates.
- [M2](../m2_nl_to_ir/README.md): its first phase is *the kernel as instrumentation*, the kernel's feedback
  as input to a model, which a tool-calling server would also deliver.
- [`docs/api/`](../../docs/api/README.md) and [`docs/kernel/configuration.md`](../../docs/kernel/configuration.md),
  the options a tool's schema would be drawn from.
