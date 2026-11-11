# S1g.1.1 — What Ein represents today: one program per subject, each with its computed answer

**Phase:** [P1g.1](README.md)
**Status:** planned 2026-11-11, not started.
**Depends on:** [S1f.6.17](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.17_binary_logic.md),
whose gate tables and circuit programs are the first row of the survey.
**Pulls in:** [F1b](../../followups/f1b_logical_formulation.md)'s opening question, *"which fragment is native
and which would be derived sugar?"*, as a list of programs to try.

## What it asks

*"what can be represented already and how?"* (the user, 2026-11-11), for each subject the instruction names.
The stage writes the smallest honest program for each, runs it, and files it in one of the phase's three
piles. It is a survey: breadth first, one program a subject, no theory polished.

## The programs to try

Each row is a program under `build/m1g/survey/`, with its run. The *expected* column is the coordinator's;
rows 2 to 7, and 8, 10 and 13 in part, have a first probe ([M1g § First probes](../README.md#first-probes-2026-11-11)); the
rest are **to be tried**.

| # | subject | the program | expected |
|---|---|---|---|
| 1 | a gate and a circuit | S1f.6.17's, as shipped | possible; the baseline |
| 2 | a formula as a value, evaluated under a given valuation | the sketch below, with `(val p 1) (val q 0)` written as facts | possible, search-free |
| 3 | the truth table of a formula | the sketch below as it stands | possible: R is the valuations (probe 1) |
| 4 | an unsatisfiable requirement | row 3 with `(And p (Not p))` required | possible; the read-out's words to be read with care (probe 1b) |
| 5 | a written Hilbert proof, checked | [S1g.3.1](../p1g.3_proof_systems/s1g.3.1_hilbert_systems.md)'s sketch | possible (probe 2) |
| 6 | an assumption refuted | [S1g.3.3](../p1g.3_proof_systems/s1g.3.3_natural_deduction.md)'s sketch | reported, not concluded (probe 4) |
| 7 | unary numbers, addition by recursion | [S1g.5.1](../p1g.5_peano_and_induction/s1g.5.1_unary_numbers_and_the_axioms.md)'s sketch | possible (probe 6); the enumerating rule does not stop (probe 3) |
| 8 | a formula rewritten: double negation removed, De Morgan applied | a request `(nnf F)` answered by `(nnf-of F G)` | possible for one formula (one run of [S1g.2.3](../p1g.2_propositional_logic/s1g.2.3_normal_forms.md)'s sketch); the rest to be tried |
| 9 | a proof *found*, not checked | modus ponens closed forward over a universe of formulas | to be tried; expected to flood |
| 10 | implication introduced: from *B under the assumption A* to `(Imp A B)` | the context as data, `(proves CTX A)`; and the same read off two states of R | the first way proved `(Imp p p)` in one run ([S1g.3.3](../p1g.3_proof_systems/s1g.3.3_natural_deduction.md)'s second sketch); the rest to be tried; expected nearly possible the first way, not possible the second |
| 11 | a quantified formula evaluated over a finite structure | `(All x F)` walked over a listed domain | to be tried; expected possible, with substitution the hard part |
| 12 | substitution of a term for a variable in a formula | a request `(subst F X T)` | to be tried; capture is the question |
| 13 | a witness for an existential | a Skolem term as a nested fact; an obligation `(open ?R)` | possible one level (probe 5); to be tried where the term feeds its own rule |
| 14 | equality of terms | `eq` and `neq` on nested facts as premises | to be tried; expected structural identity, no theory of equality |
| 15 | a rule named as a value by another rule | `(rule-is modus-ponens)` read by a rule that wants the rule's premises | expected not possible: [F5](../../followups/f5_rules_as_data.md), and [`08_self_describing.md`](../../../docs/kernel/ir/03-ein-lang/08_self_describing.md) § 5 |
| 16 | a universal statement about numbers | commutativity of addition for *all* numbers | expected not a fact a state can hold; [S1g.5.3](../p1g.5_peano_and_induction/s1g.5.3_induction.md) |

## The sketch for rows 2 to 4

A sketch, to be replaced by programs whose answers are computed and kept. It is probe 1's program without its `Or`: it loaded
and answered on the release binary at `8860d616` (2026-11-11), k_s 4 and one projection class, `?v = 1`. The
names are [S1g.1.3](s1g.1.3_conventions.md)'s to settle.

```lisp
;; sketch: a formula is a nested fact; its value is derived on request; a letter's value is a hypothesis
(relation And Formula Formula  :why "{?1} and {?2}")
(relation Imp Formula Formula  :why "if {?1} then {?2}")
(relation Not Formula          :why "not {?1}")
(relation atom Letter          :why "{?1} is a propositional letter")
(relation val Letter Bit       :why "{?1} has the value {?2}")
(relation eval Formula         :why "the value of {?1} is asked for")
(relation value-of Formula Bit :why "{?1} has the value {?2}")
(relation t-not Bit Bit        :why "not {?1} is {?2}")
(relation t-and Bit Bit Bit    :why "{?1} and {?2} is {?3}")
(relation t-imp Bit Bit Bit    :why "{?1} implies {?2} is {?3}")
(t-not 0 1) (t-not 1 0)
(t-and 0 0 0) (t-and 0 1 0) (t-and 1 0 0) (t-and 1 1 1)
(t-imp 0 0 1) (t-imp 0 1 1) (t-imp 1 0 0) (t-imp 1 1 1)

(rule ask-binary (?C)                      ; a request for a compound asks for its parts
  :match  (eval (?C ?a ?b))
  :assert (and (eval ?a) (eval ?b))
  :why    "to value {?a} {?C} {?b}, value both sides")
(ask-binary And) (ask-binary Imp)
(rule ask-not ()
  :match (eval (Not ?a)) :assert (eval ?a)
  :why   "to value not {?a}, value {?a}")
(rule value-atom ()
  :match (and (eval ?p) (val ?p ?v)) :assert (value-of ?p ?v)
  :why   "{?p} is given the value {?v}")
(rule value-binary (?C ?T)                 ; one rule for every binary connective, its table a parameter
  :match  (and (eval (?C ?a ?b)) (value-of ?a ?x) (value-of ?b ?y) (?T ?x ?y ?z))
  :assert (value-of (?C ?a ?b) ?z)
  :why    "{?a} is {?x} and {?b} is {?y}, so {?a} {?C} {?b} is {?z}")
(value-binary And t-and) (value-binary Imp t-imp)
(rule value-not ()
  :match  (and (eval (Not ?a)) (value-of ?a ?x) (t-not ?x ?z))
  :assert (value-of (Not ?a) ?z)
  :why    "{?a} is {?x}, so not {?a} is {?z}")

(atom p) (atom q)
(rule one-value ()                         ; a restriction: no letter has two values
  :match (and (val ?p 0) (val ?p 1)) :assert (false)
  :why   "{?p} cannot be both 0 and 1")
(rule value-owed (?R)                      ; an obligation: every letter has a value
  :match  (and (atom ?p) (absent (?R ?p ?v)))
  :assert (open ?R)
  :why    "{?p} has no value yet")
(value-owed val)
(hrule guess (?p ?v)                       ; the hypotheses: one per letter and value
  :match (atom ?p) :assert (val ?p ?v)
  :why   "suppose {?p} is {?v}")
(solve :hrules (guess (p 0) (p 1) (q 0) (q 1)))

(eval (Imp (Imp (Imp p q) p) p))           ; Peirce's law
(query :goal (value-of (Imp (Imp (Imp p q) p) p) ?v)
       :goal-text "Peirce's law has the value {?v}")
```

## Tasks

1. **T1g.1.1.1 — the sixteen programs,** each written, run and filed: its read-out, its `saturate --dump`,
   its trace, and where it has a claim, `ein test`. A program that does not load keeps its error text.
2. **T1g.1.1.2 — the three piles,** as a table: program, pile, and for the second and third pile the detour or
   the missing piece in one line.
3. **T1g.1.1.3 — what was surprising.** Anything the binary did that the *expected* column did not say. Probe
   1b is the first: an unsatisfiable requirement reads *empty: every alive node owes*, because the two
   valuations of `p` die alone and what is left cannot pay `p`'s value.
4. **T1g.1.1.4 — the views.** `ein render kb`, `render rule` and `render program` on rows 2, 5 and 7, kept as
   images: today a nested fact is one opaque labelled node (the coordinator's finding of 2026-11-11), and the
   page shows what a formula and a proof look like as drawn.

## Acceptance

- The table's sixteen rows each have a program, a run and a pile; none is filed on expectation.
- Every answer on the page is the binary's, with `ein solve FILE` or the command used, and the date.
- Any wrong answer is a register row before the page calls it a finding.
- The seven first probes are each confirmed or corrected by name.

## What it may find

- A row the plan calls possible that is not, or the reverse. The phase tables of P1g.2 to P1g.5 are then
  corrected at the stop.
- More rows. A subject the user's instruction implies and this table missed is added, not deferred.
