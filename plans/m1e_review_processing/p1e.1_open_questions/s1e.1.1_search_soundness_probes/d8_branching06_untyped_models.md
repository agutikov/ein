# D8 — `branching/06`'s untyped models: evidence, or its own id?

**Touches:** [T1e.1.1.3](README.md#task-t1e113--q5-derive-lattice02-by-hand-against-the-ruling),
where the fixture is currently demoted from *subject* to *evidence*.
**Not in the review's 63.**

## What was found

`examples/branching/06_lookahead_on.ein` declares five colours and five
houses, anchors four colours, and asks `(query :goal (co-located Blue ?h))`.
The intended answer is `?h = H5`.

`ein solve -e` reports **22 models**. Model 1's `co-located` facts:

```
(co-located Blue Color)    (co-located Blue H5)
(co-located Color Blue)    (co-located Color H5)
(co-located Green H2)      (co-located Green House)
(co-located H2 House)      (co-located House Green)
(co-located House H2)      (co-located H5 Color)
…
```

`Color`, `House` and `T` are **objects** — the program says `(is-a Color T)`
and `(is-a House T)` — and the blind enumerator walks every object, so it
proposes `(co-located Blue Color)` alongside `(co-located Blue H1)`. Nothing
in the program says `co-located` relates a colour to a house.

Twenty of the 22 models bind `?h` to `Color` or `House`, and the read-out
prints them as query bindings:

```
  model 1/22
  query bindings
    ?h  = Color
```

## Why it matters beyond this fixture

Three places, in increasing order of consequence:

1. **`branching/06` cannot carry Q5.** Its model set is not hand-derivable and
   its two sides do not exhaust — hence [D6](d6_the_new_q5_fixture.md).
2. **`?h = Color` is a read-out nobody would defend.** It is not wrong by the
   engine's semantics — `co-located` really is unconstrained — but a query
   binding naming a type where the puzzle means a house is the sort of thing
   [`docs/guide/`](../../../../docs/guide/README.md) would have to explain
   away.
3. **It is the standing proof for [S1e.1b.6](../../p1e.1b_hypothesis_structure/s1e.1b.6_obligations_under_hypothesis.md)'s
   loss mechanism.** The blind rung keeps proposing long after any real debt
   is discharged, which is exactly why a tree node that flips from the
   obligations rung to the blind one stops recognising solutions. Without this
   fixture that argument would be hypothetical.

## Is it a defect?

Arguably not — three defensible readings:

- **The program is under-specified.** `(relation co-located T T)` says both
  arguments are `T`, and `Color` *is* a `T`. Fix the fixture, not the engine.
  `examples/branching/12_typed_blind_solve.ein` exists precisely to show the
  typed alternative.
- **The enumerator is right and the read-out is wrong.** A query goal should
  not print bindings the puzzle's author cannot mean; that is a presentation
  question and belongs with [SE-M1](../../README.md#the-findings) / AR-M2.
- **`candidate_objects` should exclude types.** That would be an engine
  change with corpus-wide reach and is the kind of thing S1.7.23 (*"the kernel
  commits to no type system"*) deliberately refused.

## Options

| | what happens | consequence |
|---|---|---|
| **A — evidence only** | it stays a row in the stage's reconnaissance table, cited by D2 and D6 | zero cost. The observation survives only as long as this stage file is read, and M1e closes with no disposition for it |
| **B — a `Q-M1e.<n>`** | filed with the three readings above and no owner | the milestone's ledger is where a question found by the milestone belongs, and it costs one section |
| **C — fold into Q5's ruling** | the lookahead ruling states it as a second finding about the same fixture | conflates two things: one is about what `complete` means, the other about what the enumerator proposes |
| **D — fix the fixture** | add the type guard `06` obviously means, re-measure | changes a corpus fixture and moves goldens **before** any ruling, which the phase's risk section calls a stop |

**Decided 2026-08-28: B.** Filed as
[Q-M1e.12](../../open_questions.md#q-m1e12--the-blind-rung-is-untyped-and-a-model-binds-a-type-as-an-object),
with the three readings and **no owner** — a question found by the milestone
belongs in the milestone's ledger, it costs one section, and it keeps
`branching/06` untouched while Q5 is open. D is the likely eventual answer and
not this stage's: changing a corpus fixture before the ruling that would
justify it is what the phase's risk section calls a stop.
