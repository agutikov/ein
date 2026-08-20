# S1a.6.6 — The differential fuzzer

**Phase:** P1a.6 (Performance)
**Estimate:** 3 days
**Depends on:** [P1a.0](../p1a.0_conformance_harness/README.md),
[S1a.6.1](s1a.6.1_profile_baseline.md)
**Implements:** [design/01](../design/01_parity_contract.md) §7

## Context

The corpus covers what someone wrote a fixture for. The fuzzer covers the
rest of the input space — and it is the only mechanism that finds parity
bugs in shapes no human authored. It lives in this phase, not P1a.0,
because it is cheap to build *once the event protocol exists* and it is
most valuable while optimisations are landing.

It is also the natural place to settle two open questions empirically:
Q-M1a.4 (mixed-type fact args) and Q-M1a.14 (crash parity) will both show
up here within minutes of running.

## Acceptance

- The generator produces valid `.ein` programs at a high enough rate to
  be useful (≥ 80 % of outputs parse and load), with a knob for
  "grammar-valid but semantically odd".
- Both engines run each case under a budget; results diff at T1 (T2 for
  cases that terminate quickly).
- Every find is corpus-minimised and lands in `conformance/corpus.toml`
  in the same commit as its fix or its ledger entry.
- ≥ 24 h of fuzzing with no unexplained divergence is a phase gate.
- The fuzzer runs nightly thereafter, seeded from the accumulated corpus.

## Tasks

### Task T1a.6.6.1 — Grammar-directed generator

Generate from `grammar.lark`'s shape: relations with signatures, rules
with `:match` / `:assert` bodies drawn from the real vocabulary
(`and`, `or` at top level, `absent`, `not`, `neq`, nested patterns,
parameters + activator facts), facts, a `(query …)`, an optional
`(config …)`. Bias toward the shapes that stress the ordering-sensitive
paths: multiple `(or …)` disjuncts with overlapping bindings (the
S1.22.0 collision), several `(__symmetric__ R)` markers (hazard H1),
`forall` / `open` macro uses, mixed `str`/`int` slot types (Q-M1a.4).

### Task T1a.6.6.2 — Mutation mode

Take corpus files and mutate: drop a clue, swap two forms, rename an
atom, flip a `:priority`, add a `(not …)` fact, toggle a config lever.
This finds *near-miss* divergences on programs that are known to be
meaningful, which pure generation rarely produces.

### Task T1a.6.6.3 — The runner

Both engines, small budgets (`--max-enterings`, `--max-time`,
`--max-set-size 2`), `--no-cache`, T1 diff, with a hard timeout that
records a case as "budget" rather than "diff". Parallel across cases.

### Task T1a.6.6.4 — Crash-parity handling

An input where ein.py raises (Q-M1a.14) is not a fuzzer failure — it is
a corpus entry in the `crash-parity` group, compared on exit code and the
first stderr line. Report these separately so they do not drown the real
divergences.

### Task T1a.6.6.5 — Minimisation

Shrink a failing case by deleting forms, simplifying rule bodies and
shortening names while the divergence persists. A 400-line generated
program is not a bug report; an 8-line one is.

### Task T1a.6.6.6 — Corpus feedback

Every minimised find becomes a permanent corpus entry
([design/01](../design/01_parity_contract.md) §4's growth rule). Keep a
seed corpus so the nightly run starts from what already found bugs.

## What was built — [`utils/fuzz_ein.py`](../../../utils/fuzz_ein.py)

**It diffs nothing itself, and that is the design.** `ein-parity` is the one
implementation of what the two engines are not required to agree on, and a
fuzzer with a private idea of a difference drifts from the gate the day it is
written. So a batch is written out as a **corpus** and `ein-conformance` runs
it — the same binary, the same tiers, the same normalisation:

```text
generate / mutate → conformance/out/fuzz/cases/*.ein
                  → conformance/out/fuzz/corpus.toml   (one entry per case)
                  → ein-conformance run --corpus … --tier T3
                  → minimise every reported cell → conformance/fuzz_findings/
```

That reuse is what made the stage cheap: T1a.6.6.3's runner is `run.rs` +
`tier.rs`, already parallel, already timing out, already capturing every
artefact both engines wrote.

Three details worth naming:

- **The canary.** Every batch corpus carries one real fixture in the
  `positive` group, so the harness's own liveness check applies. Two engines
  that both failed to start agree on every generated case too, and a fuzzer
  that cannot tell that apart from a clean run proves nothing.
- **The classifier.** One `ein solve --max-enterings 0` per case decides its
  group: exit 0/2 is a program (`generated`), a parse or load error is a
  `*-negative` entry, where what the engines must agree on is the *message*.
  It costs a parse, a load and a root saturation — ~4 ms — and it is also how
  the acceptance's "≥ 80 % parse and load" is counted.
- **Crash parity is reported separately.** A Python traceback in either
  side's captured stderr makes the find a
  [Q-M1a.14](../open_questions.md#q-m1a14--crash-parity) `crash-parity` case,
  not a T1 divergence, so the two classes do not drown each other.

## Notes

- The fuzzer diffs *behaviour*, so it needs both engines. It cannot run
  after ein.py is gone — which is one more argument for Q-M1a.2's
  recommendation that ein.py has no sunset.
- Expect the first findings to be in the frontend (error messages, odd
  literals) and the last to be in `explain` tie-breaks. Order the
  generator's bias accordingly as the easy classes get exhausted.
