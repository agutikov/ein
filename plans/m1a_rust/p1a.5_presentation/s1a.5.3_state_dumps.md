# S1a.5.3 — State dumps

**Phase:** P1a.5 (Presentation and CLI)
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

- The whole `--dump-states` tree byte-identical for every corpus entry —
  file names, directory names, JSON key order, and content — modulo the
  timestamp fields and `state_hash.txt`, which are on the normalisation
  list ([design/01](../design/01_parity_contract.md) §5;
  `state_digest` is `PYTHONHASHSEED`-salted so ein.py is not stable there
  either).
- `-v` / `--progress-every N` stderr output identical modulo elapsed-time
  values.
- `summary.json` present on every non-abort path and **absent** after a
  budget abort, with the timeline flushed either way.
- `LatticeSnapshotV1` round-trips: dump, reload, render the lattice DOT,
  and get the same bytes as rendering from the live proof.

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
