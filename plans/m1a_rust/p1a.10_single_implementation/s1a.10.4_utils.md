# S1a.10.4 — `utils/`, re-aimed at one engine

**Phase:** P1a.10 (One implementation)
**Estimate:** 2 days
**Depends on:** [S1a.10.3](s1a.10.3_corpus_without_an_oracle.md)

## Context

**19 of the ~30 scripts in `utils/` name `ein.py`, `pypy`, a venv or
`PYTHONPATH`**, and they do it for three different reasons:

- **comparison** — the script's whole point is two engines
  (`py_oracle.py`, `ir_oracle.py`, `bench_baseline.py`'s Python column,
  `e2e_baseline.py`'s `--impl`, `mutant_ein.py`, `fuzz_ein.py`'s
  differential arm);
- **measurement of ein.py** — scripts that exist to explain the *Python*
  engine (`profile_solve.py`, `count_work.py`, `measure_match_skips.py`,
  `measure_redundant_firings.py`, `symmetric_bench.py`, `fork_split.py`);
- **incidental** — it drives an engine and ein.py is simply the one it
  learned to call (`render_examples.sh`, `zebra2_trace.sh`,
  `feature_matrix.py`, `stdlib_manifest.py`, `find_dead_defs.py`,
  `check_hashmap_iteration.py`, `relation_algebra_examples.py`).

Only the third class is a mechanical edit. The first two need a decision each,
and "delete it" is a legitimate answer for most of the second.

## Acceptance

- Every script either runs against ein.rs or is gone; none is left importing a
  module that no longer exists.
- The **M1a measurement set** still works, because
  [baseline.md](../p1a.6_performance/baseline.md) and
  [scaling.md](../p1a.7_parallelism/scaling.md) are denominated in it:
  `bench_env.sh`, `e2e_baseline.py`, `profile_ein_rs.py`, `criterion_table.py`.
  Their **Python columns become historical constants** in those documents, not
  live measurements — and the documents say which numbers can still be
  re-measured and which are frozen.
- `CLAUDE.md`'s `utils/` description matches what is there.
- A deleted script's *result* survives where the result was the point:
  `feature_matrix_results.json` is the lever matrix's record and outlives its
  runner if the runner goes.

## Tasks

### Task T1a.10.4.1 — The comparison scripts

`fork_delta_verify.py` ([D3](../divergences.md)'s fixture) and
`spec_audit.py` ([S1a.7.0](../p1a.7_parallelism/s1a.7.0_speculation_audit.md)'s
instrument) are **already single-engine** — each runs one binary twice under
different switches — so they survive untouched and are the model for the rest.
`py_oracle.py` / `ir_oracle.py` / `mutant_ein.py` are two-engine by
construction and go with the oracle.

> **What S1a.10.3 already did here.** `mutant_ein.py`'s *claim* — that the D3
> event cut still catches a dropped productive firing — has a successor,
> `ein-infer/tests/event_cut_control.rs`, so the script can be deleted with
> nothing to bank; its header says so. `fork_delta_verify.py` and
> `spec_audit.py` were re-pointed at `corpus/corpus.toml` and widened to
> include the `regression` group, which is the same file set they had before
> the regrouping. **`fuzz_ein.py` is dead right now** — its differ was
> `ein-conformance` — and its docstring and its startup message both say so and
> name T1a.10.4.2. That is the one script in the tree that does not run.

### Task T1a.10.4.2 — The fuzzer

`fuzz_ein.py` keeps its generator and its self-checkable properties and loses
its differential arm — the decision is
[S1a.10.1](s1a.10.1_bank_the_oracle.md)'s, and this task implements it. The
header must not keep advertising "four parity bugs in twenty minutes" as
something the current script can do.

Two things S1a.10.3 leaves for this task to decide, both of them the same
decision wearing different hats:

- the corpus's **`generated`** group is empty-but-kept precisely because this
  is open. If the rewritten fuzzer still writes a throwaway manifest and files
  its cases under that group, the group stays; if it does not, the group goes
  and `ein_corpus::manifest::GROUPS` loses a name.
- the sweep the fuzzer used to drive is `ein-cli/tests/corpus_cli.rs` now, and
  it takes a manifest. Pointing the fuzzer at it — generate a batch, write a
  manifest, sweep it for panics and non-terminations — is the cheapest version
  of properties 1 and 4 in the ledger's L1 list, and it needs no new
  machinery.

### Task T1a.10.4.3 — The benches

`bench_baseline.py` is the Python half of `cargo bench`; with no Python half
it collapses into `cargo bench`. `e2e_baseline.py` keeps its shape — it times
*processes*, and one implementation is still worth timing across builds and
across allocator arms — minus `--impl ein.py` and the PyPy discovery.
`count_work.py` had the same two-column job for the work counters; the ein.rs
half is `ein_core::counters` and survives.

### Task T1a.10.4.4 — The incidental ones

Mechanical: swap the invocation for `ein.rs/target/release/ein` (or an
`$EIN_BIN` defaulting to it) and drop the `PYTHONPATH` dance. `find_dead_defs.py`
and `relation_algebra_examples.py` are Python *tools reading Python source* —
they die with their subject unless someone wants the Rust analogue, and
nothing asks for one.

### Task T1a.10.4.5 — `stdlib_manifest.py`

`MANIFEST.sha256` exists because `ein.py/src/ein/stdlib/` is a build-time copy
that must not drift ([design/11](../design/11_shared_assets.md)). With no
Python package there is **one** stdlib and nothing to drift against, so the
manifest's purpose narrows to "the copy embedded in the binary matches the
checkout". Keep it for that, or retire it and say what replaced the check.

## Notes

- Resist a `--impl` flag that only takes one value. A script that still has
  the shape of a comparison invites someone to look for the other operand.
