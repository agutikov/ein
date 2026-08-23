# smt/ — scratch area

Not wired into either engine. **M3 (SMT integration) was dropped
2026-08-18** — Ein has no solver back-end and none is planned; see
[`plans/README.md`](../plans/README.md) § Roadmap. This directory stays
as a scratch area: three hand-written `.smt2` files (`4-queens.smt`,
`einstain-problem.smt`, `einstain-problem-minus-15.smt`) kept as *encoding
examples* — what the Zebra puzzle looks like when written for a solver rather
than for the graph engine. [M10](../plans/m10_external_benchmarks/README.md)
counts them as three of its benchmark corpus's encodings, already written.

> **The `CVC4` submodule was deinitialised at M1a
> [S1a.10.5](../docs/history/m1a_rust/README.md#s1a105--the-removal).**
> It pointed at `CVC4/CVC4` at version 1.8 (2021) and was never built by
> anything here; what it cost was a `git clone --recurse-submodules` fetching
> a large repository for a dropped milestone. Nothing that *reads* these files
> needed it — M10's benchmark uses **CVC5**, which is a different program
> with a different name. Re-add it if some later stage actually wants 1.8:
>
> ```sh
> git submodule add https://github.com/CVC4/CVC4.git smt/CVC4
> ```

External-tech notes on solvers live in
[`docs/lib/02-solvers-csp-sat-smt.md`](../docs/lib/02-solvers-csp-sat-smt.md).

- CVC4 user manual: <http://cvc4.cs.stanford.edu/wiki/User_Manual>
- CVC4 source: <https://github.com/CVC4/CVC4>
