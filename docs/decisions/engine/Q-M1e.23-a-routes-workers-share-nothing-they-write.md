# Q-M1e.23 — A route's workers share nothing they write

When a route fans the nodes of a step out over `--jobs N` threads, each worker reads root by shared reference and
writes only its own fork of it: no node reads another node's facts, and no store is shared between workers. The
intern tables are lent for the step, readable from every thread and growable from none, so a node that would have
to number a new fact or name is handed back and computed afterwards on the calling thread. Each worker keeps its
own derivation records. The calling thread takes the results in the step's order, computes the handed-back nodes
in that order, and narrates each node as it takes it. The result is the sequential one at any job count.

**Why.** Determinism under threads then needs no protocol and nothing a scheduler can reorder: what a worker could
observe mid-flight is root, which does not change during the step.

**Held by.** `ein_infer::region`'s `fan_out` and `pool_of`, and `ein_core::Terms::lend`;
`ein-infer/tests/worker_view.rs`: `a_lent_table_answers_a_lookup_and_refuses_an_assignment`,
`reclaiming_under_a_live_view_is_loud`; `ein-infer/tests/region.rs`: `jobs_change_nothing`. The mechanism is described
in [`parallelism.md`](../../ein/ein.rs/design/parallelism.md) § 2. What it buys:
[Q-M1a.7](Q-M1a.7-a-job-count-moves-nothing-a-run-prints.md).
