# fuzz_findings/

Minimised inputs a fuzzer found something on, written by the two fuzzers
(`cargo test -p ein-ir --test fuzz_properties` for the frontend,
[`utils/fuzz_ein.py`](../../utils/fuzz_ein.py) for whole programs). "Something"
was *a disagreement between the two implementations* until
[S1a.10.2](../../docs/history/m1a_rust/README.md#s1a102--port-the-python-test-suite)
and [S1a.10.4](../../docs/history/m1a_rust/README.md#s1a104--utils-re-aimed-at-one-engine)
retired the two oracle arms; for both fuzzers it is now **a violated
property**, named in the finding. Every file here is also one of the frontend
fuzzer's seeds — `every_seed_and_finding_parses_the_way_it_was_recorded` reads
this directory — so adding one changes `ein-ir/tests/golden/fuzz_seeds.txt`,
and that is deliberate: a find that nothing re-parses is a souvenir.

**Empty is the steady state**: a find is either fixed — and promoted to a
corpus fixture in the same commit — or accepted, and then it belongs to
[`divergences.md`](../../docs/history/m1a_rust/divergences.md) or to an open
question.

### 2026-08-21 — the rewritten engine fuzzer's first sessions

[S1a.10.4](../../docs/history/m1a_rust/README.md#s1a104--utils-re-aimed-at-one-engine)
re-aimed `utils/fuzz_ein.py` at five properties one engine can check, the
strongest of them being *the same program under a permuted interner answers
the same way* — `ein-render`'s `id_order_invariance`, pointed at a generated
batch. It found three things in its first twenty minutes, and **all three are
questions rather than fixes**, so they are here.

| file | property | what it is |
|---|---|---|
| [`hrule-reads-not.ein`](hrule-reads-not.md) | `no-crash` | an `(hrule …)` whose `:match` reads the `not` relation trips a `debug_assert!` in `Hrules::candidates`. Legal ein-lang; the release binary answers, the debug build aborts. The precondition is real and documented at the assertion — hypgen's kill cache writes `not` facts mid-enumeration — but nothing has decided what the right answer is |
| [`unsat-core-id-space.ein`](unsat-core-id-space.md) | `id-order` | which facts the **unsat core** names, and how many, moves under a permuted id space. Six of 45 ops, including `trace[answer]`'s "unsat core: 1 facts" vs "2 facts" |
| [`d3-goal-row-order.ein`](d3-goal-row-order.md) | `id-order` | already here since 2026-08-20 as a cross-engine diff — **re-derived from a different seed to the identical seven forms**, and then shown to move inside one engine with D3 held fixed. See below: D3 perturbs the row, it is not why the row is perturbable |

The two `id-order` rows are the class
[design/02](../../docs/history/m1a_rust/design/02_determinism_and_order.md) forbids,
and they are the first the id-space sweep has found that the corpus does not
reach — which is the argument for running it over generated input.

### 2026-08-23 — the `jobs` property's first session

[T1a.7.2.6](../../docs/history/m1a_rust/README.md#s1a72--level-1-parallel-enterings)
added a sixth property — *the same program at `--jobs 8` answers as it does at
`--jobs 1`* — and ran the phase's acceptance stress through it: **5 000 cases
and 25 000 runs, of which 10 000 `solve` runs each paired against a `--jobs 8`
process of its own**, with 758 cases reaching a fan-out and 79 055 enterings
evaluated on workers. `jobs` was green on all of
them. What the session found was one thing, and it is about the DOT renderer:

| file | property | what it is |
|---|---|---|
| [`kwpair-below-the-filter.ein`](kwpair-below-the-filter.md) | `no-crash` | `render constraints` panics on a keyword pair *below* the top level of an undeclared form — `constraints.rs` filters `KwPair` out of a declaration's arguments, and `value_label` then recurses into an `SForm` and reaches the ones inside. The loader accepts the program; `saturate` answers it. Two defensible fixes and nothing has chosen one, so it is a defined-behaviour question rather than a crash to paper over |

### `d3-*.ein` — 2026-08-20, S1a.6.6's first session

Both were filed as
**[D3](../../docs/history/m1a_rust/divergences.md#d3--a-fork-resumes-roots-saturation-einpy-re-derives-it)**
reaching further than its entry recorded, and both kept because the decision
they need is the user's rather than a fix. **One of the two turned out not to
be D3**, which S1a.10.4's fuzzer found by re-deriving the identical seven-form
minimum from a different seed and then failing it under a permuted id space:

| file | what differs | what is identical |
|---|---|---|
| `d3-unsat-core.ein` | the printed **unsat core**: 6 facts in ein.py, 4 in ein.rs — a strict subset | the verdict, `k`, every counter. `EIN_FORK_DELTA=0` reproduces ein.py's six exactly, so it **is** the resumed fork saturator, and the id-space sweep is green on it |
| `d3-goal-row-order.ein` | which **binding row** the solve table shows, when one model satisfies the goal more than once | `summary.json`'s `goal_bindings`, which carries every row and sorts them. **Not D3's, though D3 reaches it**: the row moves inside one engine under `EIN_ID_FILES` too, so the defect is the unsorted `rows[0]` and the two engines were each printing a legitimate row — the filename is kept for the links, the note corrects the attribution |

Neither is reachable from a corpus entry: T3 was green corpus-wide apart from
D2 when they were found. Both are one command to reproduce —

```sh
ein solve corpus/fuzz_findings/d3-unsat-core.ein --max-set-size 2
```

— and against `ein.rs/target-fd/release/ein` with `EIN_FORK_DELTA=0` for the
second reading. The third, ein.py's, is gone; what the two files record is
what it said.
