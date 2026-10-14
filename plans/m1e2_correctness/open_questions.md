# M1e2 — Open questions

Questions this milestone **raises**, as `Q-M1e2.<n>`. The M1e questions it
carries keep their `Q-M1e.<n>` ids and live in
[M1e's register](../m1e_review_processing/open_questions.md). An id is sticky
and is never reused.

Each of the first five is a **reading** a contested stage needs before it can
choose a fix. They are filed here so that the review
([S1e2.1.7](p1e2.1_seeing_the_answer/s1e2.1.7_the_review.md)) has one list to
rule on, not five stage files to find.

## Index

| Q | title | needed by | status |
|---|---|---|---|
| [Q-M1e2.1](#q-m1e21--is-a-discharged-superset-of-a-discharged-state-a-new-model) | Is a discharged superset of a discharged state a new model? | [S1e2.2.3](p1e2.2_wrong_answers/s1e2.2.3_obligations_join_superset.md) (W5) | open |
| [Q-M1e2.2](#q-m1e22--does-a-guards-absent-premise-bind-the-state-the-search-ends-in) | Does a guard's `absent` premise bind the state the search ends in? | [S1e2.2.6](p1e2.2_wrong_answers/s1e2.2.6_stale_naf_positive.md) (W8) | open |
| [Q-M1e2.3](#q-m1e23--what-does-exhausted-certify-when-the-rung-leaves-relations-uncovered) | What does `exhausted` certify when the rung leaves relations uncovered? | [S1e2.2.8](p1e2.2_wrong_answers/s1e2.2.8_uncovered_certified.md) (W6) | open |
| [Q-M1e2.4](#q-m1e24--is-an-empty-unsat-core-a-core) | Is an empty unsat core a core? | [S1e2.2.13](p1e2.2_wrong_answers/s1e2.2.13_empty_core.md) (W12) | open |
| [Q-M1e2.5](#q-m1e25--does-s1f108s-ruling-move-here) | Does S1f.10.8's ruling move here? | [S1e2.2.5](p1e2.2_wrong_answers/s1e2.2.5_naf_upward_closure.md) (W3) | open |
| [Q-M1e2.6](#q-m1e26--does-p1e23-become-a-phase) | Does P1e2.3 become a phase? | the milestone's scope | open |

C1's reading is [Q-M1e.12](../m1e_review_processing/open_questions.md#q-m1e12--the-blind-rung-is-untyped-and-a-model-binds-a-type-as-an-object)'s
and is not re-filed.

---

## Q-M1e2.1 — Is a discharged superset of a discharged state a new model?

`probes/owed-seats.ein` (two guests, two chairs, `total-owed` +
`surjective-owed`) has two matchings. Is a state that seats Ann on **both**
chairs and Bob on one, owing nothing, a third model? *No* (**minimal**): k = 2,
the lattice's own design, where a complete node leaves the frontier. *Yes*
(**literal**, Q-M1e.6's *owes nothing*): k = 7. Today the lattice answers 3
and the tree 6, so both are wrong under both readings. The recommendation in
S1e2.2.3 is *minimal*. A requirement is discharged by a witness, and a second
witness is `functional`'s question, not `total`'s.

## Q-M1e2.2 — Does a guard's `absent` premise bind the state the search ends in?

`features/07_unstratifiable.ein` records `{(p one), (q one)}`, where `(p one)`
was derived under `(absent (q one))`, and its own header says that set *"is not a
model of this program under any reading"*. Under an append-only reading of
Q-M1e.6 it qualifies. Three answers: **guards are premises** (a stale fact
disqualifies a state), **refuse what cannot be stratified** (a load error),
or **append-only is the semantics** (not wrong, and say so). The first two
compose.

## Q-M1e2.3 — What does `exhausted` certify when the rung leaves relations uncovered?

`completeness.md` §3c's fixture is `exhausted = true` with `knows` never
hypothesised, and `arity3.ein` likewise for an arity-3 goal. Relative to
`alive₀` it is right by Q-M1e.6's second clarification, and as a statement
about the program it is not. Three answers: **scoped, and the read-out says
the scope** (recommended), **uncertified unless covered**, or **open-world,
not wrong**.

## Q-M1e2.4 — Is an empty unsat core a core?

`alive-empty-phase1.ein`'s exhausted `Contradiction` is right and prints
`unsat core (0 facts)`, because its refutation rests on premise-less engine
writes. M1e's S1e.3.1 called it honest. S1e2.2.13 argues it is false: ∅ with
this program's rules is consistent, and the engine knows which deaths the
writes came from.

## Q-M1e2.5 — Does S1f.10.8's ruling move here?

W3 cannot be fixed until someone rules *forbid* or *respect* on a refutation
that rests on `(absent P)` over an extendable relation, and that ruling is
M1f's [S1f.10.8](../m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.8_refutation_under_absent.md).
The proposal is to run its T1 and its ruling as S1e2.2.5's first task, leaving
S1f.10.8 the corollary it writes into `absent_semantics.md`. The alternative
is that W3 waits for M1f.

## Q-M1e2.6 — Does P1e2.3 become a phase?

Nine surfaces that misstate a run whose answer is right, and one reachable
panic, are listed in the [milestone README](README.md#p1e23--surfaces-that-misreport-the-answer-proposed)
with witnesses. They are correctness issues and not wrong answers. Whether
they belong in this milestone, a later one or a followup is the user's call.
