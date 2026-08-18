# S1a.5.4 — The CLI

**Phase:** P1a.5 (Presentation and CLI)
**Status:** **shipped** 2026-08-18 — acceptance below. Three counts in this
doc were wrong and the check that replaced the byte diff is what found them;
one deferred engine feature (`check_commutativity`) landed here as
[S1a.4.5](../p1a.4_search_layer/s1a.4.5_solve_loop.md) said it would; and one
harness normalisation was under-specified for an *unpadded* float field.
**Estimate:** 4 days
**Depends on:** [S1a.5.3](s1a.5.3_state_dumps.md)
**Implements:** `ein/cli/{__init__,solve,saturate,render,_common,_factdump}.py`
**Decides:** Q-M1a.13 — **resolved 2026-08-18 before the stage: (b), `clap`**

## Context

The last surface, and the one that makes "drop-in replacement" literal:
three subcommands (`solve`, `saturate`, `render`), 37 options across
8 parsers, plus the delegated-subcommand dispatch that lets `saturate`
own its own argument parsing while still appearing in `ein --help`.

Counted from the parsers themselves, 2026-08-18 — the earlier "~40 flags,
every one with a short key" was right about the total and wrong about the
shape, and the check in [T1a.5.4.8](#task-t1a548--the-help-content-check)
asserts these numbers:

| parser | options | long-only |
|---|---:|---|
| `ein` | 0 | — |
| `ein solve` | 29 | `--events`, `--events-level`, `--json-summary` |
| `ein saturate` (its own) | 5 | all five |
| `ein render` | 0 | — |
| `render rules` | 1 | `--rule-mode` |
| `render rule` | 2 | both |
| `render constraints` | 0 | — |
| `render lattice` | 2 | both |

`-h/--help` is excluded throughout; every parser has it.

**39, not 37**, and `saturate` has **5**, not 3: `_events.add_arguments` puts
`--events` and `--events-level` on `saturate` as well as on `solve`, and a
`grep` for `add_argument` in `saturate.py` does not see them. Found by
[T1a.5.4.8](#task-t1a548--the-help-content-check) on its first run — which is
the case *for* the check, since a byte diff of an 89-line help text would have
reported "some lines differ" and this reports which option, in which parser,
with which field wrong.

**Q-M1a.13 was settled ahead of the stage — 2026-08-18, option (b):
`clap`.** Help layout and usage-error text are on the
[normalisation list](../design/01_parity_contract.md) §5; the *content* is
not. Same subcommands, same options with the same short keys, metavars,
arities, defaults, `choices` and exclusive groups, same help strings, same
accept/reject verdict, same exit codes — only the wrapping and the wording
of a diagnosis differ. So the surface is built with `clap`, and the days
that would have gone into an `argparse` formatter go into
[T1a.5.4.8](#task-t1a548--the-help-content-check) instead: the structural
help comparison that replaces the byte diff, and is stronger than it on
the one property the byte diff was really guarding — that no option went
missing.

## Acceptance

- Every subcommand's **stdout** byte-identical across the whole run
  matrix, and its exit code identical. **stderr** likewise, everywhere it
  is not a usage error — the `--verbose` cadence lines and `** aborted:
  … **` are engine output and stay on the byte gate.
- **Help content parity, structurally.** For `ein`, `ein solve`,
  `ein saturate`, `ein render` and each `render` sub-subcommand, the
  extracted `{option → short, metavar, arity, default, choices, group,
  help}` map is equal, and so is the subcommand set. Layout is not
  compared (Q-M1a.13). Empty-diff is not enough on its own — the
  extractor must be shown to *find* the options first, or a parser that
  silently returns `{}` passes.
- **Argument errors agree on the verdict and the code, not the text**:
  bad int (`-n x`), unknown flag, violated mutual exclusion (`-n` with
  `-e`), bad `choices` value (`--view bogus`), missing required
  positional — each rejected by both, each exit 2.
- Exit codes: 0 success, 1 load error, 2 usage error, 2 budget abort.
  The last two collide in ein.py and the collision is reproduced, not
  fixed: `ein solve -T 0.001` and `ein solve --nope` both exit 2.
- A **missing input file is not in the argument-error set** — it is an
  unguarded `Path.read_text`, hence a `FileNotFoundError` traceback and
  exit 1. It goes to the `crash-parity` group under
  [Q-M1a.14](../open_questions.md#q-m1a14--crash-parity), which is where
  the "does ein.rs name the class?" half gets decided.
- Every script under `utils/` works unchanged against the Rust binary:
  `feature_matrix.py`, `profile_solve.py --no-profile`, `zebra2_trace.sh`,
  `render_examples.sh`, `symmetric_bench.py`. None parses `--help` or
  matches ein's stderr (checked 2026-08-18), which is what makes the two
  normalised rows safe; a *new* script that does either is the trigger to
  reopen Q-M1a.13.

## What the acceptance measured

**The gate: `ein-conformance` over the corpus × run matrix, all four tiers.**

| tier | cells | same | differ |
|---|---:|---:|---:|
| T0 — verdict | 473 | 150 | 1 |
| T1 — counters | 473 | 150 | 1 |
| T2 — event trace | 473 | 239 | 1 |
| **T3 — bytes** | **473** | **472** | **1** |

The one differing cell is the same one at every tier:
`examples/ein-bugs/mixed-type-hypothesis.ein :: solve -e`, which is
**[D2](../divergences.md)** — accepted, with a fixture, since S1a.4.

T0/T1/T2 skip the cells whose runs produce no comparable artefact at that
tier; T3 skips none, which is why it is the phase's gate and why its 473 is
the number that matters.

Alongside the harness:

- **Help content parity** — 39 options across 8 parsers, every short key,
  metavar, arity, default, `choices` value, exclusive group and help string
  equal to `argparse`'s (T1a.5.4.8).
- **`--json-summary`** byte-identical on every corpus entry — which is what
  lets the harness drive ein.rs at T0/T1 at all.
- **`--events`** at `verbose`: 58 341 lines on zebra, identical modulo `impl`
  and `argv`, which [EVENTS.md](../../../conformance/EVENTS.md) excludes by
  name.
- **`--trace`**: 639 840 bytes identical on zebra2.
- **`--dump-states`**: identical trees; only `ts_ms` and `elapsed_seconds`
  differ, both on the normalisation list.
- **Crash parity**: a missing file is `FileNotFoundError: [Errno 2] No such
  file or directory: '<path>'` and exit 1 on both — the *whole last line*,
  not just the class Q-M1a.14 compares.

**Five flags the corpus does not reach**, checked by hand because
`corpus.toml`'s run lists carry none of them: `-c/--dump-config`,
`-H/--hyp-stats`, `-t/--timing`, `-v -g 1` (the live view at every entering),
and a budget abort (`-E 5` → exit 2). All identical; `--timing`'s numbers are
the normalised field and its labels, columns and parentheticals are not.
`-H` on `features/01_not_and_absent.ein` is the one that had to be waited for:
ein.rs answers instantly and CPython needs somewhere between 110 s and ~20
minutes to enumerate the same preview. Identical when it lands. Adding these
to the corpus is [P1a.6](../p1a.6_performance/README.md)'s to schedule — the
matrix is shared, and widening it re-times every entry.

### Four things the checks corrected

1. **The option counts** (above): 39, not 37; `saturate` 5, not 3.
2. **`check_commutativity` was missing entirely.** `-y` parsed, ran, and
   produced the right verdict — while doing none of the `k+1` saturations per
   alive size-`k≥2` commitment that the flag exists to perform. Invisible at
   T0, T1 and T3; **only the T2 event trace could see it**, because the extra
   saturations show up as `compile` / `fire` events and nothing else. Ported
   into `ein-infer::sanity`, wired at ein.py's call site, and T2 is clean.
   S1a.4.5 had said it "moves to P1a.5" — it did.
3. **`--json-summary`'s root block must run on the live event log.** ein.py
   builds the summary between `_events.verdict` and `_events.finish`, so its
   second root saturation is *recorded*: 92 further events on a fixture whose
   whole solve is 96. A silent one here made every `--events --json-summary`
   run diverge at T2.
4. **The harness under-normalised an unpadded float field.** `_print_stats`
   prints `{elapsed_ms:.1f}` with no field width, so the spaces before it are
   label separator, not padding — and walking back over them made a *faster*
   engine read as a diff (`13.1` vs `0.9`). Fixed in
   `ein-conformance::normalise`, with the two-run test that would have caught
   it. `--timing`'s padded `{:9.2f}` keeps the width check it had.

### And two the acceptance itself got wrong

The item "every script under `utils/` works unchanged against the Rust
binary" named five scripts. Only **two** of them are CLI consumers at all:

- `render_examples.sh` — **works**, 451 files, byte-identical trees. It needs
  `PYTHONPATH` as well as `EINBOT`, because half of it (the per-form IR/KB
  views) calls the *Python API* directly: those subcommands were removed from
  the CLI in P1.11 and never came back.
- `zebra2_trace.sh` — hardcoded `${PYBIN} -m ein.cli` with no override hook,
  so no alias could reach it. Given the `EINBOT` hook `render_examples.sh`
  already had; **works**, identical trace.
- `profile_solve.py`, `symmetric_bench.py`, `feature_matrix.py` — **not CLI
  consumers**. They `from ein… import` and drive the engine as a library, so
  no binary can be substituted for them by any means. They are P1a.9's
  (bindings), not this stage's, and the acceptance item should never have
  listed them.

## Tasks

### Task T1a.5.4.1 — Dispatcher

`ein` with `prog`/`description`, the three subparsers, and the
**delegated** dispatch: `saturate` is intercepted *before* argument
parsing so its own parser owns its flags, while still being registered
as a bare subparser purely so it appears in `ein --help`. Reproduce that
two-level behaviour, including that `ein saturate --help` prints
`saturate`'s own help with `prog="ein saturate"`.

### Task T1a.5.4.2 — `solve`

All 29 options — 26 with short keys, and the three long-only ones the
old count dropped — with their defaults: the stop policy group
(`-n/--solutions` default 1, `-e/--exhaustive`, mutually exclusive);
engine knobs (`-m -T -E -L -K -o -y`); search order (`-z -d`, including
the fresh random seed when `--shuffle` is given without `--seed` and the
`shuffle seed: N` line on stderr); diagnostics (`-v -g -D -c -H -t`);
extra stdout (`-s -p -P -f`); trace (`-r -G -F -R -l`); and the long-only
`--events FILE.jsonl`, `--events-level {normal,verbose}`,
`--json-summary FILE.json`.

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

### Task T1a.5.4.8 — The help-content check

The instrument Q-M1a.13's resolution owes. A `help-shape` op alongside
[S1a.5.3](s1a.5.3_state_dumps.md)'s `dump-shape`: walk every
`(sub)command × --help`, parse the text into
`{command → {option → short, metavar, arity, default, choices, group,
help}}`, and diff the two structures. On the Python side it reads
`argparse`'s parser objects directly rather than re-parsing its own
output — the parser *is* the structure, and scraping it back out of
formatted text would only re-import the layout this stage is exempting.
On the Rust side, `clap`'s `Command` introspects the same way.

Two floors, so the check cannot pass vacuously: the extractor reports how
many options it found per parser and every count in § Context's table is
asserted (`-h` excluded), and a mutation test — rename one
short key in a copy of the Rust `Command` — must fail it. `saturate` needs
its *own* parser introspected, not the bare one registered for the help
listing, which has no options at all.

## Notes

- Do the CLI **last within the phase** but wire a minimal `solve`
  entry early in [P1a.4](../p1a.4_search_layer/README.md) — the
  conformance runner needs *something* to invoke, and a stub that
  prints only the table is enough for T0/T1.
- Note the `render lattice` default `--max-set-size 3` versus `solve`'s
  5. Small, easy to normalise away by accident, and it changes the
  rendered lattice. This is exactly the class of difference the
  help-content check exists to catch, and exactly what a byte diff of an
  89-line help text would have reported as "one line differs".
- `clap` is a new entry in the [dependency
  policy](../design/12_toolchain_and_layout.md) §2 — added with this
  resolution, `cli` only. The engine crates stay dependency-light: the
  CLI links it, `ein-core`/`ein-ir`/`ein-infer` do not.
- `clap` exits 2 on a usage error and 0 on `--help`/`--version`, which is
  argparse's behaviour already; the delegated `saturate` dispatch is the
  one place it needs help, since `clap` has no equivalent of "registered
  so it appears in help, never actually parsed" — intercept `argv[1]`
  before `clap` sees it, as ein.py does.
