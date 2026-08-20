# fuzz_findings/

Minimised inputs on which the two implementations disagree, written by the two
fuzzers (`cargo test -p ein-ir --test fuzz_parity` for the frontend,
[`utils/fuzz_ein.py`](../../utils/fuzz_ein.py) for whole programs). **Empty is
the steady state**: a find is either fixed — and promoted to a corpus fixture
in the same commit — or accepted, and then it belongs to
[`divergences.md`](../../plans/m1a_rust/divergences.md).

What is here now, both from S1a.6.6's first session (2026-08-20), both
**[D3](../../plans/m1a_rust/divergences.md#d3--a-fork-resumes-roots-saturation-einpy-re-derives-it)**
reaching further than its entry recorded, and both kept because the decision
they need is the user's rather than a fix:

| file | what differs | what is identical |
|---|---|---|
| `d3-unsat-core.ein` | the printed **unsat core**: 6 facts in ein.py, 4 in ein.rs — a strict subset | the verdict, `k`, every counter. `EIN_FORK_DELTA=0` reproduces ein.py's six exactly, so it is the resumed fork saturator |
| `d3-goal-row-order.ein` | which **binding row** the solve table shows, when one model satisfies the goal more than once | `summary.json`'s `goal_bindings`, which carries every row and sorts them |

Neither is reachable from a corpus entry: T3 is green corpus-wide apart from
D2. Both are one command to reproduce —

```sh
ein solve conformance/fuzz_findings/d3-unsat-core.ein --max-set-size 2
```

— against either implementation, and against
`ein.rs/target-fd/release/ein` with `EIN_FORK_DELTA=0` for the third reading.
