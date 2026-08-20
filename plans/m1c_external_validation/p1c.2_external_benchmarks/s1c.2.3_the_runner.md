# S1c.2.3 — The runner

**Phase:** P1c.2 (External benchmarks)
**Estimate:** 3 days
**Depends on:** [S1c.2.2](s1c.2.2_systems_and_install.md)

## Context

One harness, `(problem × system)` cells, no linked solvers. The shape already
exists in this repo: `crates/ein-conformance` "shells out to both
implementations and links neither", which is what let it compare Python to
Python and catch its own blind spots. The benchmark runner is the same design
with a wider set of subjects and a different pair of outputs (an answer and a
time instead of a byte stream).

**Where it lives:** `ein.rs/crates/ein-bench` for the runner, `bench/` for the
data — corpus, manifest, results. That keeps
[P1a.10](../../m1a_rust/p1a.10_single_implementation/README.md)'s rule intact:
`cargo test --workspace` is the gate, and anything that is not the gate is
still a crate rather than a shell script nobody runs.

## Acceptance

- `ein-bench run` executes every runnable cell and writes a machine-readable
  results file — `bench/results/<date>-<host>.json` — that
  [S1c.2.5](s1c.2.5_the_report.md) renders. **A re-run adds a file; it does
  not overwrite history.**
- **Every cell has one of five outcomes** and none of them is silence:
  `ok`, `disagrees`, `timeout`, `missing` (no such system on this machine),
  `error` (ran, failed, here is stderr).
- **Repetitions and the statistic are stated in the harness, not chosen at
  reading time**: N cold process runs per cell, median reported, spread
  reported alongside. [`criterion_table.py`](../../../utils/criterion_table.py)'s
  precedent is that a mean without a deviation is not a measurement.
- **Timeouts are per (problem, system) and are cell values.** A cell that
  times out is a result — "did not finish in 300 s" is exactly what
  `zebra2-minus-15 -e` deserves to report today.
- **The thread-count rule is explicit and uniform.** Ein is measured at
  `--jobs 1`; Z3 and Clingo are single-threaded by default; OR-Tools CP-SAT is
  not. Either every system runs single-threaded, or the table grows a second
  column — the stage picks one, writes down which, and the report repeats it.
- Timing is **process wall clock, cold**, taken the way
  [`e2e_baseline.py`](../../../utils/e2e_baseline.py) takes Ein's, and peak
  RSS alongside it, so a system that wins on time and loses by 30× on memory
  is visible.
- The runner is reproducible from the manifest alone: no per-system special
  cases hidden in code, all invocations in `bench/systems.toml`.

## Tasks

### Task T1c.2.3.1 — The crate skeleton and the cell model
### Task T1c.2.3.2 — Process measurement

Wall clock and peak RSS per child process, on a pinned core, with the
environment printed. Reuse the discipline rather than the code —
`bench_env.sh` is a shell script and the runner is Rust; what carries over is
*what gets recorded*.

### Task T1c.2.3.3 — Per-system invocation adapters

Command line, input file, expected output stream. The parsing of the *answer*
belongs to [S1c.2.4](s1c.2.4_answers_not_only_times.md); this task stops at
"ran it, captured stdout, here is the exit code".

### Task T1c.2.3.4 — The results file
### Task T1c.2.3.5 — Failure handling and the `missing` path

The path that decides whether this harness is trustworthy. `missing` must be
loud in the summary line and in the report — a table with three empty cells
and no explanation is how
[S1a.10.1](../../m1a_rust/p1a.10_single_implementation/s1a.10.1_bank_the_oracle.md)'s
42 invisible skips happened.

## Notes

- Resist making `ein-bench` a general benchmarking framework. It runs a fixed
  corpus against a fixed system list and writes one file. Everything else —
  charts, regression tracking, historical trends — is a consumer of the JSON
  and can be written when someone wants it.
- The runner is also the natural place to run **Ein against itself** across
  versions, which is what `e2e_baseline.py` does today. Not in this stage:
  one instrument, one job, and the Ein-vs-Ein numbers already have a home in
  [baseline.md](../../m1a_rust/p1a.6_performance/baseline.md).
