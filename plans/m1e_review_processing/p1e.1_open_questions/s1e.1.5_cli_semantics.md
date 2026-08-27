# S1e.1.5 — `-n 0`: Q7

**Phase:** [P1e.1](README.md) (The ten questions)
**Estimate:** 0.5 days
**Depends on:** nothing.
**Blocks:** [EH-L1](../p1e.4_low/s1e.4.4_error_handling.md) — the same
question at Low severity; this stage takes the ruling, that stage carries out
whatever the ruling says and pins it.
**Answers:** [`review/open-questions.md`](../review/open-questions.md) Q7.

## Context

`ein solve -n 0` is accepted. `py_int` allows zero, `stop_after` becomes
`Some(0)`
([`solve.rs:570-574`](../../../ein.rs/crates/ein-cli/src/solve.rs)), and what
the engine then does is whatever `SolveOptions { stop_after: Some(0) }`
happens to do. Nothing states a meaning: no test, no doc, no comment.

The reason this is a question rather than a shrug is sitting twenty lines
away. `--jobs 0` is **refused**, with a message that argues precisely that a
flag with two readings should be refused
([`cmdline.rs:171-179`](../../../ein.rs/crates/ein-cli/src/cmdline.rs) against
`:20-47`). `-n 0` has the same two readings — *stop before recording
anything* and *no limit* — and gets neither the refusal nor the definition.
One CLI, one argument, applied in one place and not the other.

It is half a day because the ruling is small and the precedent is already
written; what makes it worth a stage is that it is the milestone's cheapest
demonstration of the disposition discipline, and it feeds a Low finding that
should not be decided independently.

## Acceptance

- A ruling, recorded in
  [`defined_behaviour.md` § 4](../../../docs/kernel/defined_behaviour.md)
  where the CLI's error table lives: **refuse** with a message in the
  `jobs_spec` form, or **define** (`-n 0` means *no limit* / *record nothing*)
  and say so in `--help`.
- The ruling is pinned by a test either way, and the test is named for the
  behaviour.
- Whether `ein.py` accepted `-n 0`, and what it did, is established from the
  goldens under `tests/golden/from_ein_py/` or recorded as unknowable — the
  likely origin of the current behaviour is parity, and a parity behaviour
  nobody can confirm is parity is just behaviour.
- `-m 0` is checked in the same breath: the lattice honours it as a truncated
  no-op ([`solve.rs:1152-1159`](../../../ein.rs/crates/ein-infer/src/solve.rs))
  and the tree ignores it entirely
  ([CO-H3](../p1e.2_high/s1e.2.1_correctness.md)(a)), so the zero-argument
  question is really two flags wide.

## Tasks

### Task T1e.1.5.1 — Find out what it does today

Run it: `ein solve -n 0` on a single-model entry, an ambiguous one, and a
contradiction, with `--json-summary`. Record `verdict.type`, `k`,
`exhausted`, `stats.solution_nodes` and the exit code for each. Then read
`stop_after`'s consumers and say whether the observed behaviour is
*intentional*, *incidental* or *inconsistent across arms* — the third would
promote this from a Low finding to something else.

Do the same for `-m 0` on both traversals, since the review found the two
traversals disagree about it.

### Task T1e.1.5.2 — Check the parity origin

Grep `tests/golden/from_ein_py/` — the last independent provenance in the
repo — for any cell run with `-n 0`. If one exists, the current behaviour is
pinned parity and the ruling is *define and document*, because changing it
would break the one thing in the tree that is not the engine's own opinion.
If none exists, the parity story is a guess, and *refuse* is available.

### Task T1e.1.5.3 — Rule, and write it where the CLI's rules live

Recommended, subject to what T1e.1.5.2 finds: **refuse**, with the message
built the way `jobs_spec`'s is — name the flag, state both readings, say
which the tool declines to guess between. It costs one line of validation and
it is the same argument the CLI already makes about `--jobs`, which is
exactly the kind of consistency this milestone is about.

Then record it in `defined_behaviour.md` § 4 next to the other CLI refusals,
add the fixture, and mark [EH-L1](../README.md#the-findings) `fixed` with
this stage named as where the ruling was taken.

## Notes

If the ruling is *refuse*, check whether any corpus entry, script or doc
passes `-n 0` before shipping it — `corpus.toml`'s `runs` columns,
`utils/*.py`, `run_tests.sh`. A refusal that breaks the corpus is a refusal
discovered in CI rather than in the stage that took it.
