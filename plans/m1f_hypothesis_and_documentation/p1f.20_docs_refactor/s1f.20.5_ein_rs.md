# S1f.20.5 — `docs/ein/ein.rs/` — the concrete implementation

**Phase:** [P1f.20](README.md) (`docs/ein/`: the tree a released system would have)
**Estimate:** 3 days.
**Status:** **done 2026-11-17** ([§ The record](#the-record-2026-11-17)). It was `T1f.5.20.5` until 2026-11-08.
**Depends on:** [S1f.20.2](s1f.20.2_the_disposition_table.md)'s table, reviewed.

## What it writes

Every page the table sends to `docs/ein/ein.rs/`. It is mostly relocation, with one rule:
**[`docs/ein/ein.rs/api.md`](../../../docs/ein/ein.rs/api.md)'s marker-guarded region moves intact**, and the test that
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

## The record (2026-11-17)

Run on its own branch beside S1f.20.3 and S1f.20.4, with master (S1f.20.4) merged in before the crate re-aim. Three
of the stage's writer subagents were cut off by the usage limit; their finished pages were kept, the rest written
in the stage itself, and the matcher page taken from the writer that checked it line by line against the code.

**The override.** `benchmarks/` is not built (the user's ruling at S1f.20.2's review, the boundary): the seven
*evidence* rows moved to `utils/censuses/` as their tools' result pages, dates and commits kept, indexed in
`utils/README.md` § Censuses; the walk's measurement pages with no current instrument are deleted.

**What was written** (`docs/ein/ein.rs/`): `README.md` (the index), `api.md` (the marked region and prose region
intact, `embedding.rs` reading the new path), `architecture.md`, `algorithms.md`, `events.md`, `lattice-dump.md`,
`reserved-engine-strings.md`, and `design/` (`README.md` and the ten contracts: determinism-and-order, data-model,
ir-frontend, matcher, saturation, search, parallelism, binary-format, shared-assets, toolchain-and-layout). None has
a release-voice line, none names a process file, none states a measured number.

**Per row of the table:**

| source | status |
|---|---|
| `api/README.md`, `api/rust.md` | done (`ein.rs/README.md`, `ein.rs/api.md`), deleted |
| `inference/events.md`, `lattice_dump.md`, `reserved_engine_strings.md`, `implementation.md`, `ir/02-data-model/03_implementation.md` | done (`events.md`, `lattice-dump.md`, `reserved-engine-strings.md`, `architecture.md`), deleted |
| `kernel/architecture.md`, `inference/README.md` (as-built layout), `architecture_and_algorithms.md` §§ 6–7, `01_entities.md`, `02_store.md` (record shapes), `02_rules.md` § 8 | split: this stage's part written (`architecture.md`, `algorithms.md`, `design/data-model.md`); the pages left for their other writers |
| `m1a_rust/design/README.md`, `02`–`12`, `m1a_rust/README.md` (the cited sections), `measurements/shared_state.md` § 4 | done (`design/*`); the sources stay for S1f.20.9 |
| M1e2 `solve_interface.md` § 3.4, `alive_set_reach.md`, `detector_proposal.md` (as built) | done (`design/search.md`); the sources stay |
| `corpus_cost.md`, `stdlib_census.md`, `region_census.md`, `route1_reach.md`, `node_census.md`, `hole_census.md`, `taint_census.md` (*evidence*) | moved to the process side, `utils/censuses/` |
| `measurements/baseline.md`, `feature_cost.md`, `fork_delta_trace.md`, `scaling.md`; m1d's closure, layer, model-set and openness censuses and `lattice_exhaustive_run.log`; M1e2 `walk_census.md` | deleted, each link to them now its label |

**The crates' links**: 196 comment links in 79 Rust files re-aimed from `m1a_rust/` (design 84, the README's
sections, the measurement pages) and M1e2's three pages, each to the page and section stating what it leans on;
labels are page and section, and the sentences around them are S1f.20.6's. The links to `solve_interface.md` §§ 1
and 2.2 (`cmdline.rs`, `help_surface.rs`, `lever_invariance.rs`, `route_invariance.rs`, `tests/explore.rs`) are the
CLI reference's, left for S1f.20.3. Findings L-4 (the scripts count), L-9 (the snmalloc figure dropped) and L-11
(`data-model.md` § 10) are closed here.

**Checks on the last commit**: the tracked link check, the release-voice check (re-banked as counts fell, 6 798 →
6 650 lines), the grep of the new pages for process references, `RUSTDOCFLAGS="-D warnings" cargo doc --workspace
--no-deps`, and the tests whose page or data moved (`embedding`, `events_reference`, `help_surface`, `render_doors`,
`cli_semantics`'s event cover). No gate was run, by the brief.
