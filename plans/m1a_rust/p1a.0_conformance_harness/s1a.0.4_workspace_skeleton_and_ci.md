# S1a.0.4 — Workspace skeleton and CI

**Phase:** P1a.0 (Conformance harness + shared assets)
**Estimate:** 2 days
**Depends on:** [S1a.0.3](s1a.0.3_shared_stdlib_and_examples.md)
**Implements design:** [design/12](../design/12_toolchain_and_layout.md)

## Context

Stand up `ein.rs/` so P1a.1 opens with a compiling workspace, a wired
corpus, and CI that already runs the harness. Nothing here implements
engine behaviour; everything here is what makes implementing it
measurable.

## Acceptance

- `cd ein.rs && cargo build && cargo test` is green with the eight
  crates in place (most empty).
- `ein.rs/target/release/ein --version` runs.
- `ein-conformance` is a working binary that parses `conformance/corpus.toml`
  and can run the Python-vs-Python comparison from
  [S1a.0.1](s1a.0.1_parity_contract_and_corpus.md).
- CI has the three tiers from
  [design/12](../design/12_toolchain_and_layout.md) §4, with the
  per-commit tier under 5 minutes.
- `criterion` benches compile and run (against ein.py's numbers as
  reference constants in the report).
- `AGENTS.md` documents `ein.rs/`, `stdlib/`, `conformance/`.

## Tasks

### Task T1a.0.4.1 — Workspace and crates

Create the workspace with `ein-core`, `ein-ir`, `ein-infer`,
`ein-render`, `ein-cli`, `ein-conformance` (and stub manifests for
`ein-server` / `ein-py`, feature-gated off). Pin `rust-toolchain.toml`.
`#![forbid(unsafe_code)]` everywhere except the future `.einb` casting
module. Add `deny.toml`.

### Task T1a.0.4.2 — Dependency baseline

Add exactly the crates in [design/12](../design/12_toolchain_and_layout.md)
§2 that P1a.1–P1a.3 need (`rustc-hash`, `smallvec`, `memchr`,
`include_dir`, `md-5`, `serde`/`serde_json` for the conformance crate),
and no others. Record the rule in the workspace README: a new runtime
dependency needs a line of justification in the commit message.

### Task T1a.0.4.3 — The hash-map lint

Implement the check from
[design/02](../design/02_determinism_and_order.md) §9: no iteration over
a hash map at an observable site. Start as a CI grep over
`\.(iter|keys|values)\(\)` on identifiers typed as maps, with an
`// determinism-ok: <reason>` allow-list comment. Upgrade to a `dylint`
rule if the grep proves noisy.

### Task T1a.0.4.4 — CI tiers

Per-commit, nightly, release, per
[design/12](../design/12_toolchain_and_layout.md) §4 — including running
ein.py's own `./run_tests.sh --fast` on every commit. A parity gate is
worthless if the oracle silently regressed.

### Task T1a.0.4.5 — Bench harness

`criterion` benches (parse / load / saturate_root / match_hot / boundary
/ solve_fast / solve_exhaustive / fork) plus `utils/bench_baseline.py`
producing the same measurement set from ein.py, so
[design/README.md § Measured](../design/README.md#measured) is refreshed
by two commands.

### Task T1a.0.4.6 — AGENTS.md

Add `ein.rs/`, `stdlib/`, `conformance/` to *Where things live*; note
that `stdlib/` and `examples/` are shared by both implementations and
that ein.py is the parity oracle. Keep it terse — the file is an
orientation map, not a summary of this plan.

## Notes

- Do not add `ein.rs/target/release/ein` to `$PATH` or install it. The
  harness invokes both engines by explicit path so there is never a
  question of which binary ran.
- `ein-cli` starts as a stub that prints a version and exits 2 on every
  subcommand; it becomes real in [P1a.5](../p1a.5_presentation/README.md).

---

## Outcome — 2026-08-17

`cd ein.rs && cargo build && cargo test` is green over six crates, `cargo fmt
--check` and `cargo clippy -- -D warnings` are clean, `criterion` benches
compile and run, and `AGENTS.md` documents `stdlib/`, `conformance/` and
`ein.rs/`.

The workspace, `rust-toolchain.toml` and `deny.toml` (T1a.0.4.1/.2) landed
early, with S1a.0.1: `ein-conformance` cannot be the only member of a
workspace whose point is the other five, and the manifest has to list them to
build at all. `ein-server` and `ein-py` are **not** here — both carry heavy
dependencies (an async runtime, PyO3/maturin) that the dependency policy says
to defer until something needs them, and a feature-gated stub buys nothing a
`[[workspace]]` line at P1a.8 will not.

### The determinism lint (T1a.0.4.3)

`utils/check_hashmap_iteration.py`, a grep as design/02 §9 proposed, with the
escape hatch `// determinism-ok: <reason>`. Two decisions worth naming:

- **The rule is stronger than "don't use `RandomState`".** `FxHashMap` is
  deterministic run-to-run, but its order is still an artefact of hash values
  and insertion history, where ein.py's observables come from
  insertion-ordered `dict`s and explicit `sorted()`. So the check is on
  *iteration*, not on the hasher.
- **An annotation without a reason does not count.** The point is not to
  record that someone saw the warning but to record why the order cannot be
  observed — which is the part that goes stale and the part a later reader
  needs.

Name resolution is *nearest preceding binding wins*, which is enough for two
functions to each have a local called `m` of different types. What it cannot
see is a map reached through a method call (`self.cache().iter()`); that is
the grep's ceiling and the reason the `dylint` door stays open.

### The benches (T1a.0.4.5)

Eight `criterion` benches matching `utils/bench_baseline.py` name for name,
every one of them **pending** until the engine it measures exists — reporting
itself rather than a zero, because a zero in a report looks like a result.
They land now for two reasons better than the numbers they do not yet produce:
the harness compiles and runs in CI from the start, and the *set* is fixed
before there is any result to be tempted by. A benchmark set chosen after
seeing the numbers measures what the implementation happens to be good at.

### CI (T1a.0.4.4) — a decision, not just a task

**The repo had no CI at all.** "Add the three tiers" therefore means
*introducing* a CI system, which is a bigger step than the stage implies, so
it is called out rather than assumed. GitHub Actions, since the project's
`Repository` URL is GitHub; three workflows matching design/12 §4's tiers.

The per-commit tier's shape follows one rule: **the oracle runs first and on
its own.** A parity failure is meaningless if ein.py moved, so its red is the
one that makes every other red uninterpretable. Every step was run locally
before being written down.

Most of the release tier is inert — `--jobs` cross-diff waits for P1a.7 —
and says so in place rather than being added later. A release checklist
assembled at release time is a checklist nobody reviewed.
