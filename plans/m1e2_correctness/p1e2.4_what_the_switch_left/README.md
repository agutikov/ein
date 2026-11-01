# P1e2.4 — What the switch left: the `absent` ruling, and the read-out at scale

**Milestone:** [M1e2](../README.md) (Correctness)
**Estimate:** two stages named, none estimated.
**Status:** created 2026-11-01 on the user's ruling, when two items were put
to them after S1e2.1a.5's switch: *"file in additional phase in m1e2"*, both
in one phase. One is a ruling W3's fix waited on and nobody had taken. The
other is a failure the region gallery found the first time it ran against the
region modes.
**Depends on:** nothing. Either stage can run first. **It runs before
P1e2.1b** (the user, 2026-11-02: *"reorder P1e2.4 before P1e2.1b"*), because
S1e2.1b.4 applies the ruling S1e2.4.1 takes; it was briefly ordered after
P1e2.1c the same day.
**Blocks:** S1e2.4.1 blocks [S1e2.2.5](../p1e2.2_wrong_answers/s1e2.2.5_naf_upward_closure.md)
(W3), P1e2.1b's S1e2.1b.4 and M1f's
[S1f.10.8](../../m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.8_refutation_under_absent.md)
corollary. S1e2.4.2 blocks `utils/review_bundle.py --regions` building
without a process killed.

---

## Why one phase

Both arrived with the switch, and neither fits the phase that would otherwise
take it. The ruling is a language question that three stages had each claimed
a share of. The out-of-memory kill is not a wrong answer, so the register is
the wrong place for it, and it is not a question either. The phase holds what
the switch left that is neither the region search
([P1e2.1c](../p1e2.1c_the_region_search/README.md)) nor a wrong answer
([P1e2.2](../p1e2.2_wrong_answers/README.md)).

## Stages

| ID | title | the question |
|---|---|---|
| S1e2.4.1 | May a refutation rest on an `absent`? | [Q-M1e2.5](../open_questions.md#q-m1e25--does-s1f108s-ruling-move-here): *forbid* or *respect*, for W3 |
| S1e2.4.2 | `--models key` declines before it allocates | why `ein solve … --models key` was OOM-killed on an 11 339-state read-out, and the bound that stops it |

Each stage gets a file when it starts. Until then this page is the scope.

### S1e2.4.1 — May a refutation rest on an `absent`?

**The ruling.** [Q-M1e2.5](../open_questions.md#q-m1e25--does-s1f108s-ruling-move-here)
asked whether M1f's S1f.10.8 ruling moves into M1e2. The user ruled
2026-11-01 that it does, as this stage. Two answers were put in
[S1e2.2.5](../p1e2.2_wrong_answers/s1e2.2.5_naf_upward_closure.md)'s table:

- **forbid** (S1f.10.8's option A): a `(false)` or `(not …)` conclusion may
  not rest on an `(absent P)` whose relation is still extendable. It is a load
  error naming the replacement, so W3's witnesses are **refused**, not
  answered. [S1e.2.3](../../../docs/history/m1e_review_processing/README.md#s1e23--q-m1e9s-containment-a-diagnostic-for-a-refutation-resting-on-an-absent)'s
  census counted 9 exposed rules over 7 corpus entries, each to be rewritten
  or refused. That is a stdlib change.
- **respect**: the engine never learns from a refutation whose derivation
  reads an `absent` over an extendable relation. The witnesses answer right.

**Where the engine stands.** W3's four register rows were flipped to `fixed`
at the switch. Every one is a program the static check flags non-monotone,
answered by route 2, the cube, which computes each node from scratch and
learns from nothing. **The flip therefore assumed *respect*.** *Forbid* would
re-open those rows as refusals. Either way the right answers carry P1e2.1b's
caveat: on a non-monotone program R is the cube's *"under assumes a monotone
lattice"*, and what it is there is S1e2.1b.2–.3's.

**Acceptance.**

- S1f.10.8's T1, the exposed set, is taken here from S1e.2.3's 9 rules over 7
  entries, re-counted against today's stdlib and corpus with the command and
  the date.
- The user rules *forbid* or *respect* on that set, with W3's four rows and
  their cube answers in front of them.
- The ruling is recorded in [`open_questions.md`](../open_questions.md)
  (Q-M1e2.5 closed), in the register's W3 rows (kept `fixed` under *respect*,
  restated under *forbid*), and in S1e2.2.5, which applies it to whatever
  engine path still learns. S1e2.1b.4 applies it to the region search on a
  non-monotone program. S1f.10.8 keeps the corollary it writes into
  `absent_semantics.md`.

### S1e2.4.2 — `--models key` declines before it allocates

**The case.** Found 2026-11-01 by `utils/review_bundle.py --regions`, the
first gallery built against the region modes:

```
ein solve examples/saturation/type-exclusivity/pets.ein --models key
```

A bare `solve` on `pets.ein` describes R, 11 339 states, every one listed. The
run was killed by the kernel's OOM killer at **6.8 GB resident, 275 GB
virtual** (2026-11-01 18:07, kernel log). The gallery's `pets` page records
the kill in its cross-check 1, and commit `43a40a84` names it.

**The cause, read in the code.** `ein_render::models::separating` builds one
bit mask of k(k−1)/2 bits per varying slot: one bit per pair of states, set
where the slot tells the pair apart. It does this **before**
`min_key_size`'s node budget is consulted. For k = 11 339 that is 64 280 791
bits, 7.7 MiB per slot, so about 850 slots reach 6.8 GB. The budget bounds
the search that follows, so it never gets to answer *unaffordable*, which is
the outcome `--models key`'s help promises. The arithmetic is the stage's to
confirm by measurement. This page reproduced nothing, because reproducing it
kills a process.

**Acceptance.**

- The key form declines with `KeyOutcome::Unaffordable` and its reason
  **before** it allocates whenever the pair table would pass a stated memory
  bound. The bound and the reason are written where `--models key` is
  documented.
- A test holds it: a synthetic state set over the bound declines and
  allocates no pair table.
- `ein solve examples/saturation/type-exclusivity/pets.ein --models key`
  finishes, declining, with its peak RSS measured and written down with the
  command and the date.
- The other read-out forms that scale with the states listed, `--models grid`,
  `--diff-models` and `-p`, are measured the same way on the gallery's
  largest regions: `pets` (11 339 states), `square-bwd/houses` (10 077 reached
  under the default budget) and `syntax/arg-kinds`. Any that is superlinear
  in memory gets the same treatment or a row here saying why not.
- `utils/review_bundle.py --regions` builds the `pets` page with cross-check 1
  passing.
