# S1f.20.5 — `docs/ein/ein.rs/` — the concrete implementation

**Phase:** [P1f.20](README.md) (`docs/ein/`: the tree a released system would have)
**Estimate:** 3 days.
**Status:** not started. It was `T1f.5.20.5` until 2026-11-08.
**Depends on:** [S1f.20.2](s1f.20.2_the_disposition_table.md)'s table, reviewed.

## What it writes

Every page the table sends to `docs/ein/ein.rs/`. It is mostly relocation, with one rule:
**[`docs/api/rust.md`](../../../docs/api/rust.md)'s marker-guarded region moves intact**, and the test that
diffs it against [`embedding.rs`](../../../ein.rs/crates/ein-cli/tests/embedding.rs) moves with it. It is the
repo's working precedent for a page that cannot rot, and breaking it in a refactor about documentation
quality would be the phase's worst possible outcome.

- **`design/`** takes the M1a design contracts, de-scaffolded. 106 of the crates' 464 doc-comment links into
  `docs/history/` pointed at them on 2026-08-28. Each is rewritten in the commit that moves its contract
  ([P1f.20 § Two rules](README.md#two-rules-across-the-stages)).
- **`benchmarks/`** collects the measurements and the censuses, the table's *evidence* rows. Each keeps its
  date, its commit and its `utils/` re-take script, and gains nothing else.

## The method

### Task T1f.20.5.1 — `api.md`, with its region and its test

The move, and `embedding.rs`'s diff test pointed at the new path in the same commit.

### Task T1f.20.5.2 — `architecture.md`, `design/`, `algorithms.md` and `read-outs.md`

Each page of the table's rows, with its inbound links.

### Task T1f.20.5.3 — `benchmarks/`

The evidence rows, each with its date, commit and script.

## Acceptance

- Every row the table sends to `ein.rs/` is written there.
- `rust.md`'s marked region is intact at its new path, and the test that diffs it passes.
- Every evidence page keeps its date, its commit and its re-take script.
- The link check is green, and the release-voice baseline has no line for a page under `docs/ein/ein.rs/`.
- `./run_tests.sh` is green.
