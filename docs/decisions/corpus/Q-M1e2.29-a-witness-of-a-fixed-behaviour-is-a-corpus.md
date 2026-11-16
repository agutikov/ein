# Q-M1e2.29 — A witness of a fixed behaviour is a corpus program where its check looks

A program that once showed a defect, and shows its fix now, stays as a corpus program in the place the check that
holds the fix reads. `examples/ein-bugs/nested-core.ein`, whose unsat core holds a nested fact, is a `regression`
entry, and a read-out test asserts how the core is spelt. `examples/ein-bugs/d3-goal-row-order.ein`, whose goal rows
must not move with the id space, is a `regression` entry under `examples/`, where the id-order sweep reads every
program.

**Why.** An absence is settled by naming what checks it
([Q-M1e.1](../docs/Q-M1e.1-a-behaviour-is-refuted-by-a-banked-probe.md)): a witness no check reads holds nothing,
and the checks already exist, so the witness need only stand where they look.

**Held by.** `ein-cli/tests/read_out.rs`: `a_nested_fact_in_a_core_prints_as_an_s_expression`;
`ein-render/tests/id_order_invariance.rs`, which digests every rendering of every corpus program outside `tests/`
and permutes the id space of each that searches; the two entries in `corpus/corpus.toml`.
