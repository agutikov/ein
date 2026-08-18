# S1a.5.3 — State dumps

**Phase:** P1a.5 (Presentation and CLI)
**Status:** **shipped** 2026-08-18 — acceptance below, with one **ein.py bug
fixed first** (`LatticeDumper` had no `root_saturating`) and one acceptance
item that turned out to describe code that does not exist.
**Estimate:** 2 days
**Depends on:** [S1a.5.2](s1a.5.2_trace_and_answer.md)
**Implements:** `ein/inference/monotonic/{state_dump,_lattice_dump,_serialise,snapshot}.py`

## Context

`--dump-states DIR` persists the whole search as a directory tree:
`00_timeline.jsonl` (a chronological event log), per-layer snapshots,
per-entering directories, and `summary.json`. It is the diagnostic
surface the engine already has, and — usefully for this port — it is
structurally the same idea as the
[oracle event protocol](../design/01_parity_contract.md) §3, just with a
different schema and a directory instead of one file.

Three dumper implementations sit behind the six lifecycle hooks:
`MonotonicDumper` (files), `ProgressDumper` (live stderr under `-v`,
subclassing it), and `LatticeDumper` (the richer per-commitment tree with
the proof summary).

## Acceptance

The instrument is a `dump-shape` op on `utils/ir_oracle.py` and its Rust
half `ein-render`'s `shape::dump_shape`. A directory has no line protocol
to diff over, so the tree is **rendered** as one text — every file, sorted
by path, with its bytes — and the two texts are compared. The rendering
invents nothing, which is the point: a missing file, an extra file, a
renamed directory and a changed byte all read the same way.

| item | result |
|---|---|
| The whole `--dump-states` tree byte-identical for every corpus entry | **yes**, in both layouts: `monotonic` (`layers/layer_NN_pre.ein`) and `lattice` (`layers/layer_NN/pre.ein` plus the per-commitment `enterings/` tree). File names, directory names, JSON key order and content |
| `-v` / `--progress-every N` stderr identical modulo elapsed time | the `progress` mode — run *with* an `out_dir`, because the live view is the file dumper plus a stream and the two compose |
| `summary.json` present on every non-abort path and **absent** after a budget abort, timeline flushed either way | the `abort` mode, under the *raising* policy. Asserted directly, not just diffed: a run that reports `ABORTED True` must have no `summary.json` and must have a `00_timeline.jsonl` |
| `LatticeSnapshotV1` | see below — the acceptance item as written describes code that does not exist |
| **The whole sweep** | **65 files, 325 modes, 66 MB, 0 differences** — and **36 of the `abort` runs actually aborted**, counted rather than assumed, so a budget that stopped tripping anywhere would fail the floor rather than quietly stop testing the path |

The normalisation is the timestamps and nothing else: `ts_ms` and
`elapsed_seconds` are blanked **by value, not by presence**, on both
sides, so a record that lost its `ts_ms` still fails. `state_hash.txt`
needed no normalisation because it is never written — see below.

### The ein.py bug this stage found

`LatticeDumper` has no `root_saturating`, and `solve` calls that hook
unconditionally once a dumper is attached, every 50 root firings. So
`solve(dumper=LatticeDumper(out_dir=…))` — the usage
[`lattice_dump.md`](../../../docs/kernel/inference/lattice_dump.md)
documents — raised `AttributeError` on any puzzle whose root saturation
ran past 50 firings, which is most of them. Every existing test used a
fixture small enough to stay under it; a corpus sweep does not.

Fixed in ein.py first, as the milestone's non-goals require — a no-op
override matching `MonotonicDumper`'s, so nothing observable changes for
a dumper that already worked — and pinned by
`tests/inference/lattice/test_lattice_dumper.py::test_dumper_survives_a_long_root_saturation`,
which asserts the fixture is *above* the threshold so the regression test
cannot pass vacuously. The CLI never selects `LatticeDumper`
(`_make_dumper` picks `ProgressDumper` / `_TimingDumper` /
`MonotonicDumper`, and all three have the hook), so the bug was
library-only.

### Two things that are always empty, and why that is the behaviour

`kb_index/` never materialises and `LatticeSnapshotV1.nodes_by_state_key`
is always empty, for one reason: `LatticeProof.kb_index` is written only
by a DAG builder via `_record_setnode`, and nothing on the shipping path
calls one. Its own docstring says so. That is the same fact that makes
`render lattice --view full` always take its fallback
([S1a.5.1](s1a.5.1_dot_renderers.md)), and it is why `state_hash.txt` —
the one artefact the normalisation list was holding a place for — is
never written at all. The port has no `kb_index` and emits, byte for
byte, what ein.py emits: an empty `kb_index` list in
`proof_summary.json`.

### The debt `canon.rs` booked, paid here

[S1a.4](../p1a.4_search_layer/README.md) made `state_key` sort by `FactId`
rather than by `repr` — a `u32` sort and a `memcmp` instead of building a
string per fact — on the argument that *for identity* any total order is
equivalent ([design/02](../design/02_determinism_and_order.md) §6). The
module said so, and named the phase that would owe the difference:

> The `repr` order is still needed where the key is *displayed*, and that
> is P1a.5's.

This is that phase, and the `snapshot` mode is where the key becomes
output. The re-sort lands in `lattice_snapshot`, not in `state_key`: the
engine keeps its cheap order and the one place a state key is read by a
human gets ein.py's. Two spellings of `repr` turned out to be needed, and
the difference is load-bearing — a *commitment*'s elements carry raw
`Fact.args`, where a nested fact reprs as `Fact(relation_name=…)`, and a
*state key*'s carry `canon._hashable_args`, where the same fact reprs as
a tuple. A `Cell` therefore stores the repr its source spelled rather
than recomputing one.

### The snapshot acceptance item, measured

> `LatticeSnapshotV1` round-trips: dump, reload, render the lattice DOT,
> and get the same bytes as rendering from the live proof.

Neither half of that holds of the code, and the port is not the reason:

1. **There is no dump and no reload.** `LatticeSnapshotV1` is an
   in-memory frozen dataclass. Nothing in ein.py serialises one; its only
   consumer is `tests/inference/lattice/test_shuffle_invariance.py`,
   which compares two snapshots with `==`.
2. **A snapshot render is deliberately not the proof's render.** A
   snapshot's `solutions` and `deads` are post-saturation **state keys**,
   not commitment paths — that is the whole point of S1.7.24's
   result-level identity, and it is what makes the projection
   orientation-invariant. So its solution view draws whole *states* where
   the proof's draws commitments. Getting the same bytes would mean the
   snapshot had lost the property it exists for.

What is checkable is the part the item was reaching for, and it is
checked: the projection itself and the lattice DOT rendered *from* it
both go out in the `snapshot` mode and are diffed on every corpus entry.
`render_lattice` therefore grew a `LatticeSource` — ein.py's has accepted
either input since S1.6.3, and until this stage the port's took only a
proof.

## Tasks

### Task T1a.5.3.1 — Serialisation helpers

`_serialise.py`: `_arg_to_node`, `_fact_to_sform(fact, with_kwargs)`,
`_kb_to_ein_text(kb)` (a whole KB as `.ein` text — it goes through the
[dumper](../p1a.1_ir_frontend/s1a.1.2_ast_and_dumper.md), so line
breaking must already be right), `_firing_to_dict`, `_fact_summary`.

### Task T1a.5.3.2 — `_TimelineMixin`

`_emit_timeline(event, **fields)` with its monotonic `_timeline_seq`,
the `00_timeline.jsonl` handle, `summary(verdict, stats)` writing
`summary.json`, and `close()` flushing. Key order in the emitted JSON is
insertion order — preserve it (serde with `preserve_order`, or an
explicit ordered writer).

### Task T1a.5.3.3 — `MonotonicDumper`

`__post_init__` directory creation, `root_saturating`, `root_initial`,
`layer_start`, `entering`, `layer_end`, and `_fmt_commitment`. The
directory layout (`NN_layer/`, per-entering subdirectories) is part of
the contract.

### Task T1a.5.3.4 — `ProgressDumper`

The `-v` live view: `_say`, `_el` (elapsed), the per-layer and
per-entering lines paced by `progress_every`, the root-saturation
counter line every 50 firings, and the final summary. It also captures
`t0` / `t_root` / `t_end` / `root_facts`, which is what makes `-v -t`
show both the live search and the phase table.

### Task T1a.5.3.5 — `LatticeDumper`

`_layer_dir`, `_entering_dir`, `_commitment_slug` (with its `_field` /
`_factid_slug` helpers — these become directory names, so the slug
function is load-bearing), `_factid_json`, `_commitment_json`, and
`proof_summary` including the node ordering by `repr(state_key)`
(→ `python_repr`) and the `state_hash.txt` write.

### Task T1a.5.3.6 — Snapshots

`LatticeSnapshotV1` + `lattice_snapshot(...)`: the serialisable view of a
proof, its `nodes = tuple(sorted(...))` ordering, and the reader path
`render_lattice` uses.

## Notes

- Directory and file names derived from commitments are the most fragile
  part: they go through `_commitment_slug`, which sanitises fact ids into
  path-safe strings. A single character difference renames a directory
  and the whole tree diff explodes — implement and test the slug first,
  on its own.
- This stage is also the one that proves the
  [event protocol](../p1a.0_conformance_harness/s1a.0.2_oracle_event_protocol.md)
  did not perturb anything: the timeline and the event log are emitted
  from overlapping call sites, and they must agree about what happened.
