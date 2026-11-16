# Q-M1e.14 — A `--jobs` sweep compares what a job count reaches

`--jobs N` reaches only a region route's fan-out, and no store is shared between its workers: a route's node is a
fork of root that no other node reads. So a run at `--jobs N` is the same computation as at `--jobs 1`, and what
it prints is byte for byte the same, narration included.

The sweep that holds this compares, on the corpus programs that search, every output a job count reaches: the
five read-outs, the region trace, the states, the lattice, the five core sets and the oracle. Every other output,
which reads no job count, is held to ignore one instead. The sweep is a failure, not a pass, where it could not
fail: on a build with no fan-out, where `--jobs N` and `--jobs 1` would be one computation compared with itself.

**Why.** A comparison whose columns can never differ is coverage in name only. Comparing exactly what a job count
can move, and checking that a moved output is caught, makes the sweep's agreement mean what it says.

**Held by.** `ein-render/tests/jobs_invariance.rs`: `jobs_does_not_move_any_observable`, the sweep;
`an_op_outside_the_predicate_ignores_jobs`, the other side of `Op::takes_jobs`;
`a_job_count_that_reaches_an_output_is_caught`, a run that leaks its job count reported as an answer that moved;
and `the_sweep_is_not_vacuous`, which fails on a build with no fan-out.
