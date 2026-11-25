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
([the feedback object](03-feedback.md)).

| page | what |
|---|---|
| [01-outcomes.md](01-outcomes.md) | the seven outcomes: each, what it is read from, the test that holds the reading; the questions and the readings |
| [02-kernel-invariants.md](02-kernel-invariants.md) | what the kernel guarantees of an answer, each with the test or the decision of Ein's that holds it |
| [03-feedback.md](03-feedback.md) | the bounded call, and the feedback object it returns, `einaf-feedback/1`, key by key; what the bounds do not stop |

## What it depends on

**Ein's crates, by path**, as [embedding the crates](../ein/ein.rs/04-api.md#1-the-crates) says a program outside
the engine's workspace depends on them: `ein-core`, `ein-ir` to load, `ein-infer` to ask, `ein-render` to print a
read-out. Its tests read the corpus through `ein-corpus`, the engine's test helper, which reads
[`corpus/corpus.toml`](../../corpus/corpus.toml) and runs a sweep on the engine's worker pool.

**One module calls the library**, `einaf::kernel` ([`einaf/src/kernel/`](../../einaf/src/kernel/mod.rs)), through
one function, the bounded call `einaf::kernel::call`: it loads a program, asks it each question under the kernel's
bounds and returns one feedback object a question; nothing else in the package names an item of Ein's crates. Its
JSON is written by `serde_json`, and its digests are SHA-256, by `sha2`.

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

The tests are four files under [`einaf/tests/`](../../einaf/tests/), each running programs through the bounded call:
[`outcomes.rs`](../../einaf/tests/outcomes.rs), one test an outcome, on a program that has it, and the programs
whose saturation has no fixpoint, EinAf's own fixtures under [`einaf/tests/fixtures/`](../../einaf/tests/fixtures/);
[`corpus.rs`](../../einaf/tests/corpus.rs), every corpus program that is not `slow` asked every question, each outcome
held to what the corpus says of the program, every object read back as itself through JSON, and each question the
program's entry declares held to what `ein solve --json-summary` says of the same run (`--nocapture` prints the count
of each outcome under each reading, and of the runs compared); [`refused.rs`](../../einaf/tests/refused.rs), every
refused fixture under `examples/broken/`; and [`cli.rs`](../../einaf/tests/cli.rs), the command.

`corpus.rs` runs the `ein` binary the gate built, `ein.rs/target/gate/ein`, which the gate's step before EinAf's
builds, or the one `$EIN_BIN` names; without one it fails, naming the command that builds it.

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
`einaf --help` lists them.
