# EinAf

**EinAf is iterative autoformalization over Ein**: a language model writes an Ein program for a problem stated in
natural language, and Ein's kernel judges it, in a loop, until the program says what the problem says and the answer
is the one Ein prints. It is an application of Ein and a project of its own: it reads Ein and links Ein's
implementation, ein.rs, as a library, and nothing of Ein reads, links or names it.

What it holds today is what the loop has to know of the kernel: **which of seven outcomes a program has under a
question about its answer** ([the outcomes](01-outcomes.md)), read off the values the library hands back; the list
of what the kernel guarantees of those values, each beside the test of Ein's that holds it ([the kernel's
invariants](02-kernel-invariants.md)); and **the feedback object**, one value for what the kernel found of one
program under one question, which a bounded call of the kernel returns and every later reader of the kernel reads
([the feedback object](03-feedback.md)). And what it asks of a formalization, written before what enforces most of
it: **the formalization contract** ([the contract](04-contract.md)), a text in, one program in five parts made in
three steps out, each form of it in one row of a table of categories that EinAf computes, with the contract's worked
example and five texts through the three steps by hand, each step's program a file its tests run. And **the model's
runtime** ([the model's runtime](05-model.md)): the model in EinAf's process, through a binding to llama.cpp behind a
Cargo feature, every call returning a generation with its envelope or a typed failure, and replay, which answers a
call from a record with no model; and the probe made with it, of Ein written with no grammar. And **the grammars**
([the grammars](06-grammars.md)): the whole grammar of Ein as GBNF, generated from Ein's two grammar files and held to
its parser in both directions, and a grammar a step of the formalization, written at request time from the program's
own vocabulary. And **the steps** ([the steps](07-steps.md)): the catalogue of the library the steps select from, each
entry a program the kernel runs; the prompts, versioned templates filled by named slots; a run of the three steps, or
of the whole program in one call, each step's program assembled in synthesis order and judged at root with the
writer of H; and the contract's check, its seven findings. And **the one shot** ([from a text to a short
answer](08-from-text.md)): `einaf from-text`, the steps, the least questions the answer needs, and the short answer
the program prints, its values formatted by EinAf's formatters, or one of EinAf's fixed texts where no program prints
one; the run's record, each call slim, which replays to the same program and answer. And **the text's analysis**
([the text's analysis](09-analysis.md)): a recorded analysis of a text by trained parsers, each sentence's AMR graph
and its named entities, read from a file EinAf runs no parser to write, and given to step 1 after the text, as the
parser printed it, a node a line, or Ein facts in a vocabulary of EinAf's own.

| page | what |
|---|---|
| [01-outcomes.md](01-outcomes.md) | the seven outcomes: each, what it is read from, the test that holds the reading; the questions and the readings |
| [02-kernel-invariants.md](02-kernel-invariants.md) | what the kernel guarantees of an answer, each with the test or the decision of Ein's that holds it |
| [03-feedback.md](03-feedback.md) | the bounded call, and the feedback object it returns, `einaf-feedback/1`, key by key; what the bounds do not stop |
| [04-contract.md](04-contract.md) | the formalization contract: the task, the three steps and their five design questions, the categories of a program's forms, what the model receives, what a run records; the steps by hand |
| [05-model.md](05-model.md) | the model's runtime: building with the feature, giving a run the model, a request and a generation, the envelope, the chat template, a grammar and where it begins, a kept context, the failures, replay, the commands and the probe |
| [06-grammars.md](06-grammars.md) | the grammars: the whole grammar generated from `ein.l` and `ein.y` and its hand-written rules, held to the parser; a grammar a step and what its vocabulary closes; where a grammar begins and how an output ends; how a token is held to a grammar; the commands |
| [07-steps.md](07-steps.md) | the steps: the catalogue, the prompts, a run, the contract's check, the record, the commands |
| [08-from-text.md](08-from-text.md) | the one shot: a run and its questions, the answer and its fixed texts, the formatters, the record, the command |
| [09-analysis.md](09-analysis.md) | the text's analysis: the recorded file, PENMAN read, the three forms and the spelling of AMR in Ein's names, step 1 given one, the record, the commands |

## What it depends on

**Ein's crates, by path**, as [embedding the crates](../ein/ein.rs/04-api.md#1-the-crates) says a program outside
the engine's workspace depends on them: `ein-core`, `ein-ir` to load, `ein-infer` to ask, `ein-render` to print a
read-out. Its tests read the corpus through `ein-corpus`, the engine's test helper, which reads
[`corpus/corpus.toml`](../../corpus/corpus.toml) and runs a sweep on the engine's worker pool.

**One module calls the library**, `einaf::kernel` ([`einaf/src/kernel/`](../../einaf/src/kernel/mod.rs)), through
two functions: the bounded call `einaf::kernel::call`, which loads a program, asks it each question under the
kernel's bounds and returns one feedback object a question; and `einaf::kernel::forms`, which parses and loads a
program and asks it nothing, and returns its top-level forms, each as `ein_ir::dump_canonical` prints it and by its
shape, with what its imports bring; and `einaf::kernel::parse`, which parses a text and loads nothing, every error
with its kind and its place; `einaf::kernel::shapes`, a text's forms read off its parse alone; and
`einaf::kernel::vocabulary`, what a text declares, imports and writes, read off its parse, with
`einaf::kernel::library`, the standard library's modules as texts, `einaf::kernel::rule_docs`, each rule a text writes
with its parameters and its `:why`, and `einaf::kernel::stdlib_digest`, the library's manifest digest; and
`einaf::kernel::root`, which builds root and H as `ein show hyps` does, under the call's bounds, and returns the
writer of H the kernel reports and root's firings and owes as values. Nothing else in the package names an item of
Ein's crates: `einaf::contract`, the contract's categories, reads the forms, `einaf::probe` reads the kernel through
them, `einaf::grammar` the vocabularies, and `einaf::catalogue`, `einaf::check` and `einaf::formalize` all of them.

**Ein's two grammar files, through grmtools**: `einaf::grammar` reads `ein.l` and `ein.y`, which the build embeds,
through `lrlex`, `cfgrammar` and `regex-syntax`, at the versions `ein-ir` builds against, and names no item of Ein's
crates ([the grammars § 1](06-grammars.md#1-the-whole-grammar)).
Its JSON is written by `serde_json`, and its digests are SHA-256, by `sha2`.

**The model's runtime, by a feature**: the chat template is rendered by `minijinja` (with `minijinja-contrib`'s
Python methods), always built; the binding to llama.cpp, `llama-cpp-2`, only with the `llama` or `cuda` feature
([the model's runtime § 1](05-model.md#1-building-with-the-model)), which the gate's build never turns on.

## The package

[`einaf/`](../../einaf/Cargo.toml) is one Cargo package, `einaf`, a library and a binary of the same name. It is the
root of a workspace of its own, so the engine's workspace never builds it, and it builds into its own `einaf/target/`,
which git ignores. Three things of its manifest are the engine's, copied, because a manifest outside a workspace does
not inherit them:

- **the patched parser runtime**: `ein-ir`'s generated parser calls into `lrpar`, and the engine's workspace patches
  it with the copy under [`ein.rs/vendor/lrpar/`](../../ein.rs/vendor/lrpar/), whose error recovery is deterministic;
  a `[patch]` reaches only the workspace whose root manifest writes it, so EinAf's writes the same one;
- **the lock file**: `einaf/Cargo.lock` was started from the engine's, so every crate the two share is at the
  engine's version;
- **the test profile**, `gate`: release code with the debug assertions and the overflow checks on, so the engine's
  own assertions run under EinAf's tests as they do under the engine's.

## Building and testing

```sh
cargo build --release --manifest-path einaf/Cargo.toml        # einaf/target/release/einaf
./build.sh --einaf                                            # the same, from the repository's build script
cargo test --manifest-path einaf/Cargo.toml --profile gate     # EinAf's tests, a step of the gate after Ein's
```

The tests are twelve files under [`einaf/tests/`](../../einaf/tests/), most running programs through the bounded call:
[`outcomes.rs`](../../einaf/tests/outcomes.rs), one test an outcome, on a program that has it, and the programs
whose saturation has no fixpoint, EinAf's own fixtures under [`einaf/tests/fixtures/`](../../einaf/tests/fixtures/);
[`corpus.rs`](../../einaf/tests/corpus.rs), every corpus program that is not `slow` asked every question, each outcome
held to what the corpus says of the program, every object read back as itself through JSON, and each question the
program's entry declares held to what `ein solve --json-summary` says of the same run (`--nocapture` prints the count
of each outcome under each reading, and of the runs compared); [`refused.rs`](../../einaf/tests/refused.rs), every
refused fixture under `examples/broken/`; [`cli.rs`](../../einaf/tests/cli.rs), the command; and
[`contract.rs`](../../einaf/tests/contract.rs), the contract's examples, the programs under
[`einaf/contract/`](../../einaf/contract/), each held to what [the contract](04-contract.md#8-held-by) says, and the
categories of every form of the reference programs; and two of the model's runtime, which need no model
([the model's runtime § 11](05-model.md#11-held-by)): [`model.rs`](../../einaf/tests/model.rs), replay, the envelope
and each typed failure from calls the binding recorded, and a line of the probe judged again; and
[`prompts.rs`](../../einaf/tests/prompts.rs), the Ein of the prompts' files and of the probe's plan; and
[`grammar.rs`](../../einaf/tests/grammar.rs), the grammars held to the parser with no model
([the grammars § 7](06-grammars.md#7-held-by)); [`catalogue.rs`](../../einaf/tests/catalogue.rs) and
[`steps.rs`](../../einaf/tests/steps.rs), the catalogue and the steps ([the steps § 7](07-steps.md#7-held-by)); and
[`from_text.rs`](../../einaf/tests/from_text.rs), the one shot, its answer, its formatters and its record
([from a text to a short answer § 6](08-from-text.md#6-held-by)); and [`analysis.rs`](../../einaf/tests/analysis.rs),
the recorded analyses of the six texts, their forms, the facts loaded and drawn, and step 1 given one
([the text's analysis § 7](09-analysis.md#7-held-by)).

`corpus.rs` runs the `ein` binary the gate built, `ein.rs/target/gate/ein`, which the gate's step before EinAf's
builds, or the one `$EIN_BIN` names, and `contract.rs` reads H off its `ein show hyps`; without one each fails,
naming the command that builds it.

## The command

`einaf outcome FILE [QUESTION]` makes one bounded call, one question, and prints the outcome under each reading the
question gives, with what it was read from, off the feedback object. `einaf feedback FILE [QUESTION]` prints the
object itself, as JSON. Every outcome exits 0; a usage error exits 2, and a fault of the kernel's 1.

<!-- output -->
```text
$ einaf outcome examples/zebra2.ein --single
einaf outcome · examples/zebra2.ein --single
  one           over all of R           Solution: one state
```

The questions are `ein solve`'s: none for a description of R, `--single`, `--minimal` or `--maximal` with or without
`--single`, `--count` and `--no-search`; the bounds are `--region-budget`, `--region-limit`, `--max-steps` and `-T`.
`einaf --help` lists them, and the model's subcommands, `render`, `generate`, `grammar`, `model` and `probe`, which
[the model's runtime § 10](05-model.md#10-the-commands) states, `grammar`'s in [the grammars § 6](06-grammars.md#6-the-commands),
`steps` and `catalogue`, which [the steps § 6](07-steps.md#6-the-commands) states, `from-text`, the one shot
([from a text to a short answer § 5](08-from-text.md#5-the-command)), and `analysis`, a text's recorded analysis in
one of its forms ([the text's analysis § 6](09-analysis.md#6-the-commands)).
