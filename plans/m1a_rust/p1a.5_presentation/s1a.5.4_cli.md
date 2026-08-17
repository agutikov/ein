# S1a.5.4 — The CLI

**Phase:** P1a.5 (Presentation and CLI)
**Estimate:** 4 days
**Depends on:** [S1a.5.3](s1a.5.3_state_dumps.md)
**Implements:** `ein/cli/{__init__,solve,saturate,render,_common,_factdump}.py`
**Decides:** Q-M1a.13

## Context

The last surface, and the one that makes "drop-in replacement" literal:
three subcommands (`solve`, `saturate`, `render`), ~40 flags, every one
with a short key, plus the delegated-subcommand dispatch that lets
`saturate` own its own argument parsing while still appearing in
`ein --help`.

This stage also settles Q-M1a.13 — whether to reproduce `argparse`'s
help layout and error text byte-for-byte (hand-rolled) or accept a
different presentation (`clap`). Prototype both on `solve` before
committing; the recommendation is hand-rolled, because a `--help` that
differs is the first thing a user notices.

## Acceptance

- Every subcommand's stdout/stderr/exit code byte-identical across the
  whole run matrix.
- `--help` for `ein`, `ein solve`, `ein saturate`, `ein render`, and each
  `render` sub-subcommand — matched, or Q-M1a.13 resolved the other way
  *in the ledger*, deliberately.
- Argument errors identical: bad int, missing file, unknown flag,
  violated mutual exclusion (`-n` with `-e`), bad `choices` value.
- Exit codes: 0 success, 1 parse/load error, 2 budget abort.
- Every script under `utils/` works unchanged against the Rust binary:
  `feature_matrix.py`, `profile_solve.py --no-profile`, `zebra2_trace.sh`,
  `render_examples.sh`, `symmetric_bench.py`.

## Tasks

### Task T1a.5.4.1 — Dispatcher

`ein` with `prog`/`description`, the three subparsers, and the
**delegated** dispatch: `saturate` is intercepted *before* argument
parsing so its own parser owns its flags, while still being registered
as a bare subparser purely so it appears in `ein --help`. Reproduce that
two-level behaviour, including that `ein saturate --help` prints
`saturate`'s own help with `prog="ein saturate"`.

### Task T1a.5.4.2 — `solve`

All 22 flags with their short keys and defaults: the stop policy group
(`-n/--solutions` default 1, `-e/--exhaustive`, mutually exclusive);
engine knobs (`-m -T -E -L -K -o -y`); search order (`-z -d`, including
the fresh random seed when `--shuffle` is given without `--seed` and the
`shuffle seed: N` line on stderr); diagnostics (`-v -g -D -c -H -t`);
extra stdout (`-s -p -P -f`); trace (`-r -G -F -R -l`).

Then the command body in order: `_timed_load` (split parse/load timing),
seed handling, `_resolved_config` (the `dataclasses.replace` overrides),
`--dump-config`, `--hyp-stats`, the isolated compile timing under `-t`,
dumper selection, `solve`, the solution table, `_print_final`,
`_print_stats`, `_print_timing`, `_write_trace`.

### Task T1a.5.4.3 — `solve`'s printers

`_print_stats` (the exact labels and column positions),
`_print_timing` (the phase table, its `─`×40 rule, the per-hypothesis
average, and the note that compile is measured standalone),
`_print_resolved_config` (dataclass field order, `str(v).lower()` for
bools, 32-column name field), `_print_root_hyp_preview` (fork, `emit_closed`,
saturate, `generate_hypotheses_with_stats`, the per-relation breakdown
with its `{pct:>5.1f}` column — `pyfmt`), and `_TimingDumper`.

### Task T1a.5.4.4 — `saturate`

Its own parser (`--dump`, `--max-steps`, `--progress-every` default 500)
and the whole `bench` body: `snapshot(kb, eng)`, `print_snapshot` with
its dict breakdowns, `print_firings`, `dump_kb` grouped by origin,
`_fmt_int` / `_fmt_delta` / `_band_label` / `_ir_text` / `_arg_text` /
`_fact_text` / `_fact_text_with_provenance` / `_has_nested_fact_args`.
This command's output is dense and entirely mechanical — good early T3
target.

### Task T1a.5.4.5 — `render`

`rules` / `rule --name` / `constraints` / `lattice`, with
`--rule-mode {sidebyside,overlay}`, `--view {full,solution}`,
`--max-set-size` (default 3 here, not 5). Shared loaders in `_common`
(`_parse_or_exit`, `_load_kb_or_exit`, `_rule_forms`) with their exit
codes and messages.

### Task T1a.5.4.6 — `_factdump`

`fact_sexpr(arg)` — also the event protocol's fact renderer, so it is
already implemented by
[P1a.0](../p1a.0_conformance_harness/README.md); reuse rather than
duplicate. `hypothesis_target_relations(kb)` (walking the query's
`:hrules`), `print_final_state(kb, mode, targets)` for the three
`--print-final-*` modes.

### Task T1a.5.4.7 — `--events` in ein.rs

The Rust side of the oracle protocol, behind the `events` feature so a
benchmark build has no branch at all
([design/12](../design/12_toolchain_and_layout.md) §3).

## Notes

- Do the CLI **last within the phase** but wire a minimal `solve`
  entry early in [P1a.4](../p1a.4_search_layer/README.md) — the
  conformance runner needs *something* to invoke, and a stub that
  prints only the table is enough for T0/T1.
- Note the `render lattice` default `--max-set-size 3` versus `solve`'s
  5. Small, easy to normalise away by accident, and it changes the
  rendered lattice.
