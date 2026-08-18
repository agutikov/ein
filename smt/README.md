# smt/ — scratch area

Not wired into either engine. **M3 (SMT integration) was dropped
2026-08-18** — Ein has no solver back-end and none is planned; see
[`plans/README.md`](../plans/README.md) § Roadmap. This directory stays
as a scratch area: the CVC4 submodule plus two hand-written `.smt2`
files (`4-queens.smt`, `einstain-problem.smt`) kept as *encoding
examples* — what the Zebra puzzle looks like when written for a solver
rather than for the graph engine.

External-tech notes on solvers live in
[`docs/lib/02-solvers-csp-sat-smt.md`](../docs/lib/02-solvers-csp-sat-smt.md).

- CVC4 user manual: <http://cvc4.cs.stanford.edu/wiki/User_Manual>
- CVC4 source: <https://github.com/CVC4/CVC4>
