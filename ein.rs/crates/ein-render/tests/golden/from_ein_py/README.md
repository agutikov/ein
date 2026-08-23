# `from_ein_py/` — the other implementation's own bytes

Seventeen renderings **ein.py produced**, checked in years before the port and
carried across by `git mv` in
[S1a.10.2](../../../../../../docs/history/m1a_rust/README.md#s1a102--port-the-python-test-suite)
— never regenerated.

That distinction is the whole point of the directory. Every other golden in
this tree is a *self*-golden: it says "ein.rs still renders what ein.rs
rendered", which catches a regression and proves nothing about correctness.
These say something ein.rs cannot say about itself — **a second
implementation, written independently, produced these bytes** — and that is
the last independent provenance the repo has after
[P1a.10](../../../../../../docs/history/m1a_rust/README.md#p1a10--one-implementation).
Blessing one of them from ein.rs would turn it into a self-golden with a
misleading name, so:

> **Never re-bless a file in this directory.** If a rendering here fails,
> either ein.rs changed a rendering on purpose — in which case the file moves
> *out* of this directory, into the ordinary goldens, and the commit says the
> provenance was spent — or it is a regression.

They were `ein.py/tests/golden/**` until S1a.10.2. The move is
[ledger §4](../../../../../../docs/history/m1a_rust/oracle_ledger.md#4-what-the-removal-must-relocate)'s
recommendation, done a stage early: five ein.rs tests read these files without
running Python, so they would have stayed green until the commit that deleted
the tree and failed there, which is exactly the kind of defect a phase should
not carry into its last stage.

| file(s) | read by |
|---|---|
| `trace_3step.md` | `ein-render/tests/golden_trace.rs` |
| `kb_zebra_unified.dot` | `ein-render/tests/golden_dot.rs` |
| `dot/*.dot` (15) | `ein-render/tests/golden_dot.rs`, `ein-render/tests/derivation_dot.rs` |

`zebra.golden` and `zebra2.golden` moved the same way, to
`ein-ir/tests/golden/from_ein_py/`, and are read by
`ein-ir/tests/dump_goldens.rs`.
