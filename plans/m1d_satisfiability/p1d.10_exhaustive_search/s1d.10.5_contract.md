# S1d.10.5 — What `exhausted` means

**Phase:** P1d.10 (Exhaustive search over many models)
**Estimate:** 2 days
**Depends on:** [S1d.10.3](s1d.10.3_stopping_criterion.md),
[S1d.10.4](s1d.10.4_conflict_mining.md) — **except T1d.10.5.0**, which depends
on nothing and is an hour.
**Runs 6th of six.**

---

## What moved under it — 2026-08-26

**Half of this stage shipped in another phase, and it shipped as a rule rather
than as a fix.** [S1d.3.3](../p1d.3_model_sets/the_verdict.md) found that
`ein solve -e examples/saturation/type-exclusivity/colors.ein` printed
`solutions (k) 5` for a file with **nine** models, and made the qualifier
normative in
[`defined_behaviour.md` §5](../../../docs/kernel/defined_behaviour.md):

| the search | what a report of the model set may say |
|---|---|
| `exhausted = true` | *these are the models* |
| `exhausted = false` | *these are models **found*** |

So this stage's second acceptance bullet — *"k = 32, exhausted" and "k = 32, cap
reached at depth 5" … should not print the same* — **is met for `Solution` and
`Ambiguity`.** What is left is the half S1d.3.3 deliberately did not take,
because there the problem is a *word* and not a qualifier on a count:
`Contradiction` and `Open` still print `solutions (k) 0` with nothing beside it,
and the fixture is
[Q-M1d.1](../open_questions.md#q-m1d1--may-the-search-stop-before-the-lattice-is-exhausted)'s:
`examples/saturation/type-exclusivity/pets.ein` says *the constraints are
contradictory* at `-m 5` through `-m 8` and has **35 models** at `-m 10`.

**And the stage found a defect in the thing it is about.** See T1d.10.5.0.

**One target in § Acceptance no longer exists and must not be recreated.**
`docs/api/inference.md` is **history** since M1a S1a.10.5 — a Python embedding
contract kept whole under a 🏛 banner, with an explicit instruction not to
rewrite it to describe the current engine. The live places a verdict contract
lives are [`docs/kernel/defined_behaviour.md`](../../../docs/kernel/defined_behaviour.md)
§5 (normative, and where S1d.3.3's rule already is),
[`docs/kernel/inference/`](../../../docs/kernel/inference/README.md), and the
CLI help.

---

## Context

Whatever the two preceding stages land, the user-visible contract needs
restating. Today a run reports `exhausted true|false` and a verdict, and the
under-determined regime exposes a gap between them: a run that stops at the
depth cap reports `exhausted false` and `truncated`, which is honest but says
nothing about *how far* from exhaustion it got — 32 of 32 models, or 32 of
unknown-many?

## Acceptance

- The vocabulary is settled and documented: **`exhausted`** means the lattice
  was exhausted and the model set is proven complete; anything less says so in
  a different word. A heuristic stop never sets it. **And a search that is not
  a lattice needs the sentence rewritten rather than reused** — a tree that
  terminates by discharge ([S1d.10.3](s1d.10.3_stopping_criterion.md) (d))
  exhausts no lattice and may still have proved the model set complete, which
  is either a second word or a re-worded first one.
- **Every verdict that reports a count carries the qualifier, or the stage says
  why not.** Two of the four do since S1d.3.3; `Contradiction` and `Open` print
  a bare `solutions (k) 0`.
- `Ambiguity` reports what is known about completeness. "k = 32, exhausted"
  and "k = 32, cap reached at depth 5" are different answers to the user's
  question and should not print the same.
- If [S1d.10.3](s1d.10.3_stopping_criterion.md) found a sound criterion, the
  verdict says *which* argument closed the search — exhaustion, or the
  criterion — because they are different guarantees and a reader deserves to
  know which one they have.
- ~~`docs/api/inference.md`~~ **`docs/kernel/defined_behaviour.md` §5** and the
  CLI help carry it; a contract that lives only in a plan is not a contract.
  (`docs/api/inference.md` is history and is not to be edited — see § What moved
  under it.)
- Whatever the corpus entry for `zebra2-minus-15.ein` says about `solve -e` is
  updated to match reality — today its note reads "the exhaustive search is
  large rather than pathological", written before anyone measured it.

## Tasks

### Task T1d.10.5.0 — `-m 0` refutes every program that has anything to guess

**Independent of the rest of the phase, and a defect rather than a design
question.** The layer loop is `for layer in 1..=max_set_size`, so at a cap of
zero it never runs, `truncated` is never set, and `exhausted = !truncated` is
`true` over a frontier that is the whole alive set:

```
$ ein solve -m 0 -s examples/zebra.ein ; echo "exit $?"
  solutions (k)   0
  verdict         No solution — the constraints are contradictory

  unsat core (0 facts)

stats
  solutions (k)    0
  exhausted        true
  layers_explored  0
exit 0
```

Reproduced on `zebra`, `zebra2`, `zebra2-minus-15-obligations` and
`features/01_not_and_absent`, and the code path says which programs it is:
**every one that has anything to guess**, i.e. the 51 corpus cells that reach
the search. A program whose root is already complete answers correctly —
`tests/stdlib/algebra/23_total_owed` says `Open — owes 1` at `-m 0` and
`branching/01_saturate_only` says `Solution`, both `exhausted true`, both right,
because there is no lattice to exhaust. `-m 1` reports `exhausted false`
correctly and `-E 0` reports `aborted` correctly, so the budget paths are honest
and the depth cap at zero is the one door with no guard on it. It states a
refutation with an **empty unsat core** and a success exit code, which is the
strongest false claim the engine can currently make.

Two questions the fix has to answer rather than assume: whether `-m 0` is a
truncation (`exhausted = false`, `Contradiction k = 0`) or a refusal (the
`Aborted` shape `-E 0` already uses), and whether an empty `unsat_core` should
be constructible for a `Contradiction` at all. The second is the more
interesting one and `corpus_exits.txt` is where the answer shows up.

### Task T1d.10.5.1 — The vocabulary
### Task T1d.10.5.2 — The verdict surface
### Task T1d.10.5.2b — `Contradiction`, and what a cap may say

Q-M1d.1's word, with `saturation/type-exclusivity/pets.ein` as the corpus
fixture and the ten Q-M1d.6 entries as the control group that must *not* move
([the openness census §5](../p1d.2_obligations/openness_census.md)). The claim
channel already refuses to be fooled: an `:expect (or …)` on `pets.ein` comes
back `NOT CHECKED` rather than `FAILED`, because `expect.rs` will not refute a
claim on the strength of a stopped search. **The verdict channel says *the
constraints are contradictory* and the claim channel says *nobody knows*, about
the same run**, and whatever this task settles should leave them agreeing.

### Task T1d.10.5.3 — Docs and help
### Task T1d.10.5.4 — The corpus note, corrected

The note has been half-corrected once already and the surviving half is a record
of a session rather than of this engine. It reads, today:

> genuinely under-determined at 32 models, so the exhaustive search is large
> rather than pathological. no `solve -e`: every model is found by depth 3 and
> depths 4-5 exist only to prove there are no more (M1d P1d.10), so the run
> outlives a 5 min ceiling here and was killed at 30 min there

**"Killed at 30 min there" is 2026-08-20's and not this engine's** — the census
measured 416 s at `--jobs 1` and the reconnaissance 50 s at `-j16`
([`layer_census.md` §4](layer_census.md#4-zebra2-minus-15-all-five-layers)). The
sentence the note closes with — *"These are the exclusions M1d lifts"* — is what
this task is for, and it now has three possible endings rather than one: the
exclusion is lifted because the run is affordable, or it is kept with the
measured reason, or it is lifted for the entry's **twin** because
[S1d.10.6](s1d.10.6_the_traversal.md) made a per-obligation run cheap and the
hrule path is the one that stays out. The same note lives on
`zebra2-minus-15-obligations.ein`, verbatim and for the same reasons, so
whatever moves moves twice.

## Notes

- This stage is small and worth its own slot because it is the one a
  performance phase forgets. The engine's verdict is the product; a faster
  search that reports the same word for two different guarantees has made the
  product worse.
