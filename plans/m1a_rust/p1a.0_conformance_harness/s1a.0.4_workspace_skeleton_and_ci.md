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
