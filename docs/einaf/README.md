# EinAf

**EinAf is iterative autoformalization over Ein**: a language model writes an Ein program for a problem stated in
natural language, and Ein's kernel judges it, in a loop, until the program says what the problem says and the answer
is the one Ein prints. It is an application of Ein and a project of its own: it reads Ein and links Ein's
implementation, ein.rs, as a library, and nothing of Ein reads, links or names it.

What it holds today is the first thing the loop has to know of the kernel: **which of seven outcomes a program has
under a question about its answer** ([the outcomes](01-outcomes.md)), read off the values the library hands back,
and the list of what the kernel guarantees of those values, each beside the test of Ein's that holds it
([the kernel's invariants](02-kernel-invariants.md)).

| page | what |
|---|---|
| [01-outcomes.md](01-outcomes.md) | the seven outcomes: each, what it is read from, the test that holds the reading; the questions and the readings |
| [02-kernel-invariants.md](02-kernel-invariants.md) | what the kernel guarantees of an answer, each with the test or the decision of Ein's that holds it |

## What it depends on

**Ein's crates, by path**, as [embedding the crates](../ein/ein.rs/04-api.md#1-the-crates) says a program outside
the engine's workspace depends on them: `ein-core`, `ein-ir` to load, `ein-infer` to ask, `ein-render` to print a
read-out. Its tests read the corpus through `ein-corpus`, the engine's test helper, which reads
[`corpus/corpus.toml`](../../corpus/corpus.toml) and runs a sweep on the engine's worker pool.

**One module calls the library**, `einaf::kernel` ([`einaf/src/kernel/`](../../einaf/src/kernel/mod.rs)): it loads
a program, asks it a question and reads the outcome; nothing else in the package names an item of Ein's crates.

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

The tests are three files under [`einaf/tests/`](../../einaf/tests/):
[`outcomes.rs`](../../einaf/tests/outcomes.rs), one test an outcome, on a program that has it;
[`corpus.rs`](../../einaf/tests/corpus.rs), every corpus program that is not `slow` asked every question, each outcome
held to what the corpus says of the program (`--nocapture` prints the count of each outcome under each reading); and
[`cli.rs`](../../einaf/tests/cli.rs), the command.

## The command

`einaf outcome FILE [QUESTION]` loads a program, asks it one question as `ein solve` would, and prints the outcome
under each reading the question gives, with what it was read from. Every outcome exits 0; a usage error exits 2.

<!-- output -->
```text
$ einaf outcome examples/zebra2.ein --single
einaf outcome · examples/zebra2.ein --single
  one           over all of R           Solution: one state
```

The questions are `ein solve`'s: none for a description of R, `--single`, `--minimal` or `--maximal` with or without
`--single`, `--count` and `--no-search`; the bounds are `--region-budget`, `--region-limit` and `-T`; `--read-out`
prints `ein solve`'s read-out of the same run after the outcome. `einaf --help` lists them.
