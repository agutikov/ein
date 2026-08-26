# S1d.10.5 — What `exhausted` means

**Phase:** P1d.10 (Exhaustive search over many models)
**Estimate:** 2 days
**Depends on:** [S1d.10.3](s1d.10.3_stopping_criterion.md),
[S1d.10.4](s1d.10.4_conflict_mining.md) — for what is left, which is **one
task**. Five of the six are **done 2026-08-26**: `T1d.10.5.0` (the `-m 0`
boundary), `T1d.10.5.2` and `T1d.10.5.2b` (the verdict surface and
Q-M1d.1's word), `T1d.10.5.3` (docs and help) and `T1d.10.5.4` (the corpus
note). What waits is **`T1d.10.5.1`'s second half** — the sentence for a search
that is not a lattice — and it waits on [S1d.10.6](s1d.10.6_the_traversal.md)
because there is no tree to write it about yet.
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
reached at depth 5" … should not print the same* — **was met for `Solution` and
`Ambiguity`**, and [T1d.10.5.2b](#task-t1d1052b--contradiction-and-what-a-cap-may-say)
has since taken the half S1d.3.3 deliberately left: `Contradiction` and `Open`
carry the qualifier too, and the table in
[`defined_behaviour.md` §5](../../../docs/kernel/defined_behaviour.md) has four
rows rather than two.

**And the stage found a defect in the thing it is about — now fixed.** See
[T1d.10.5.0](#task-t1d1050--a-cap-of-zero-is-a-truncation--done-2026-08-26),
which was an hour and did not wait for the rest of the phase: `-m 0` said
`exhausted = true` over a frontier it had never looked at, on **51 of the 150**
corpus entries that load, and now says `false`.

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

### Task T1d.10.5.0 — a cap of zero is a truncation — **done 2026-08-26**

**Fixed**, by one guard at the top of
[`solve.rs`](../../../ein.rs/crates/ein-infer/src/solve.rs)'s `phase2`: the cap
at zero cuts *before* the first layer, so the rule the loop applies at
`layer == max_set_size` — a non-empty frontier at the cap means the lattice was
not explored — is now evaluated one step earlier, where the loop that would
have said it never runs. Phase 1 reaches `phase2` only with `alive` non-empty,
so at a cap of zero there is always something the run did not look at.

Measured on all 197 manifest entries, before and against after:

| `ein solve -m 0` | before | after |
|---|---:|---:|
| entries that load | 150 | 150 |
| …reporting `exhausted = true` | **150** | 99 |
| …reporting `exhausted = false` | **0** | **51** |
| violations of *`exhausted = true` ⇒ the run reported something the search could not have changed* | **51** | **0** |

The 51 are exactly the cells that reach the search, which is what the diagnosis
below predicted from the code path, and the 99 are unmoved to the field.

**The two questions, answered rather than assumed.**

- **A truncation, not a refusal**, and the deciding argument is not the
  flag's shape but what a cap of zero is *for*. A program whose root is already
  complete has no lattice to exhaust and answers exactly at `-m 0` —
  `branching/01_saturate_only` `Solution`, `tests/stdlib/algebra/23_total_owed`
  `Open — owes 1`, both `exhausted = true` and both still true after the fix.
  Ninety-nine of the hundred and fifty are that class. The `Aborted` shape would
  decline a question the engine answers, and
  [the reconnaissance](README.md#1-the-proof-costs-83-517-what-the-answer-does)
  asks it 171 times — once per node, as `ein solve -m 0 --json-summary`. A
  refusal would have broken the instrument that priced the phase.
- **An empty `unsat_core` stays constructible for a `Contradiction`**, and the
  measurement is why: **12 corpus entries already report one under their
  ordinary `solve` run**, every one at `exhausted = false` —
  `features/{01,02,05}`, `saturation/square-{fwd,bwd}/{floors,houses,meetings}`
  and `syntax/{arg-kinds,constraint-scopes,equality}`. Nine of the twelve are
  Q-M1d.6's ten; the tenth, `branching/07_lookahead_off`, cites a 220-fact core
  and is not in the set. So the empty core is the *normal* shape of "no model
  within the cap" on a barren search, not a `-m 0` artefact — a search that
  entered nothing has no dead commitment to cite, and inventing one would be
  the worse claim. What is left is the **word**, which is
  [Q-M1d.1](../open_questions.md#q-m1d1--may-the-search-stop-before-the-lattice-is-exhausted)'s
  and [T1d.10.5.2b](#task-t1d1052b--contradiction-and-what-a-cap-may-say)'s;
  those same twelve are its evidence set. **`corpus_exits.txt` did not move**,
  which is the answer the task said would show up there: no declared run uses
  `-m 0`, and a `Contradiction` exits 0 either way.

**One shape difference from the in-loop cut, deliberate.** `alive_at_end` stays
**empty**: the field is the commitments that were entered and *survived*, and a
cap of zero enters nothing. That is `stop_after`'s shape — truncated with no
frontier to hand a deeper run — and claiming ninety-six never-entered singletons
had survived would be the same overstatement in the other direction. It is why
`alive_at_end_is_the_frontier_the_depth_cap_cut` now pins **three** shapes
rather than two, and the pairing of `!exhausted` with a non-empty frontier is
explicitly not a biconditional.

**What states it**, all three in `cargo test`:

- `monotonic_semantics::a_cap_of_zero_is_a_truncation_not_a_refutation` — the
  twin of `every_layer_1_singleton_dying_is_a_contradiction`, and the reason
  that one asserts `exhausted` rather than reading `k`: the same fixture, the
  same `k = 0`, refutation at `-m 2` and cap at `-m 0`.
- `monotonic_semantics::a_cap_of_zero_still_answers_a_root_that_needs_no_search`
  — the boundary, on both verdicts that reach it by different routes.
- `lattice_semantics::alive_at_end_is_the_frontier_the_depth_cap_cut` — the
  third shape.

Plus the normative paragraph in
[`defined_behaviour.md` §5](../../../docs/kernel/defined_behaviour.md), beside
the S1d.3.3 rule it restores conformance with. **Not** T1d.10.5.3's work: that
task owns the vocabulary's doc surface, and this is one page recording that a
stated rule had a door with no guard on it.

---

**The diagnosis, as found.** Independent of the rest of the phase, and a defect
rather than a design question. The layer loop is `for layer in 1..=max_set_size`, so at a cap of
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

> **Both answered above**: truncation, and yes — and the second turned out not
> to be about this cap at all. `corpus_exits.txt` is unchanged, which is the
> form the answer took.

### Task T1d.10.5.1 — The vocabulary — **half done 2026-08-26**

The lattice half is settled and normative: `exhausted` means the lattice was
exhausted, all four verdicts now say whether they got it, and the four-row table
is in [`defined_behaviour.md` §5](../../../docs/kernel/defined_behaviour.md)
rather than in this plan.

**The other half cannot be written yet**, and that is a dependency rather than a
deferral: *"a search that is not a lattice needs the sentence rewritten rather
than reused"* — a tree that terminates by discharge exhausts no lattice and may
still have proved the model set complete. There is no tree to write it about
until [S1d.10.6](s1d.10.6_the_traversal.md), and writing it first would be
inventing a guarantee before the thing that offers it. **This is the whole of
what is left in this stage.**

### Task T1d.10.5.2 — The verdict surface — **done 2026-08-26**

Folded into T1d.10.5.2b below, because they are one edit: the qualifier the
acceptance asks for is the same qualifier the word question needs, and giving it
to `Contradiction` without giving it to `Open` would have left the surface
inconsistent in the other direction. `Open` gained it with no corpus cell to
show for it — every `Open` in the corpus is exhausted — which is recorded as
the branch being right rather than being exercised.

### Task T1d.10.5.2b — `Contradiction`, and what a cap may say — **done 2026-08-26**

**Answered: it may not say it is a refutation.** `exhausted = false` now prints
*No model found — the search did not exhaust the lattice* and
`solutions (k) 0   (none found — the search did not exhaust)`, where
`exhausted = true` keeps every word it had. The fixture is the one this task
named:

| `saturation/type-exclusivity/pets.ein` | before | after |
|---|---|---|
| `-m 5`, `-m 8` | *the constraints are contradictory* | *No model found — the search did not exhaust the lattice* |
| `-m 10` (**35 models**) | `Ambiguity`, already qualified by S1d.3.3 | unchanged |

**And the two channels agree now**, which is what the task asked for in its own
words. `:expect (false)` on that file at `-m 5` has always come back
`NOT CHECKED — Contradiction matches, but the search was not exhausted`; the
verdict beside it used to say *contradictory*. One solve, two read-outs, and
they no longer disagree.

**The unsat core is renamed in the same breath.** A core explains why a program
has *no model*, which a truncated run has not shown, so its header is
`refuted so far (n facts)` when `exhausted = false`. That is also what makes the
empty one legible — `unsat core (0 facts)`, which
[T1d.10.5.0](#task-t1d1050--a-cap-of-zero-is-a-truncation--done-2026-08-26)
found on 51 entries, read as *the empty set is contradictory* rather than
*nothing died*.

**The blast radius, measured before the edit**: over `solve` and `solve -e`
across the corpus, **26 cells in 13 files** report `Contradiction` at
`exhausted = false` and move; **48** report it exhausted and do not; **0**
report `Open` unexhausted. Nothing else moved — 37 of 8 171 corpus renderings,
every one of them `trace[answer]`, and **no exit code, no counter, no
`--json-summary` field and no `:expect` outcome**. `verdict.type` stays
`Contradiction`, which is the shape S1d.3.3 held to when it moved twelve
entries' words and no entry's exit code.

**On "the ten Q-M1d.6 entries as the control group that must not move".** They
move — nine of the thirteen files above are theirs — and the reading that makes
that right is the one the openness census left: their `Contradiction` at
`exhausted = false` *is* what Q-M1d.1 was about, so a change to that word
reaching them is the change working rather than over-reaching. What must not
move about them is what did not: their exit code, their counters, and the fact
that `Open` cannot reach them because they state no obligation. A control group
for the *obligations* mechanism is not a control group for this one.

What is pinned: `cli_semantics::a_truncated_k0_is_not_reported_as_a_refutation`
— the truncated arm on `pets.ein`, and `ein-bugs/zebra2-bad.ein` as the
non-vacuity control that must keep *contradictory*, its `unsat core (1 facts)`
and no qualifier at all.

### Task T1d.10.5.3 — Docs and help — **done 2026-08-26**

[`defined_behaviour.md` §5](../../../docs/kernel/defined_behaviour.md) carries
the four-row table, the rename, and the blast radius. `docs/api/inference.md` is
**not** touched, per § What moved under it.

The help had one inaccuracy and it was on the flag this stage is about:
`-e/--exhaustive` read *"exhaust the lattice — certify unique / ambiguous /
unsat"*, and `-e` does no such thing — it clears `stop_after` and leaves
`--max-set-size` in place, which is exactly how `pets.ein -e -m 5` reached a
false refutation. It now reads *"do not stop at the first model. Certification
also needs the lattice to end within --max-set-size, which `exhausted` in
--stats reports"*. `help_shape.txt` re-blessed for it; no other help string
names a verdict word.

### Task T1d.10.5.4 — The corpus note, corrected — **done 2026-08-26**

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

**The second ending is the one that was true: kept, with the measured reason.**
`solve -e` on this entry is 618 076 enterings — 416 s at `--jobs 1`, 50 s at
`-j16` — against a `slow` threshold of **1 s**, so the exclusion survives on
arithmetic rather than on a ceiling nobody had taken. What replaced the killed-at-30-min
sentence is the number it was standing in for: the lattice ends at **layer 22
and 17 204 592 enterings**, 24 min 56 s, and buys no model that depth 3 did not
already have. And the closing sentence is corrected rather than deleted —
*"These are the exclusions M1d lifts"* became **"M1d did not lift these; it
priced them"**, with a pointer to what would lift them: a cheaper argument for
the same claim, measured out of process at 171 nodes, which is
[S1d.10.6](s1d.10.6_the_traversal.md)'s to ship and not a run list's to assume.
Both notes moved, as the task said they would.

## Notes

- This stage is small and worth its own slot because it is the one a
  performance phase forgets. The engine's verdict is the product; a faster
  search that reports the same word for two different guarantees has made the
  product worse.
