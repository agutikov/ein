# E-115 — A stored negation of a verb does not reach the link the verb is read into

**Kind:** gap
**Status:** open
**Filed:** by S2.2.1, 2026-11-26, on `98125f61`
**Owner:** none; [Q-M2.23](../../open_questions/kernel/Q-M2.23-does-the-library-grow-a-reading-of-a-literal-vocabulary.md)
(does the library grow a reading of a literal vocabulary) bears on it, beside
[E-110](E-110-the-library-has-no-reading-of-a-literal-vocabulary.md)
**Since P2.2's close (2026-11-27):** none; [Q-M2.23](../../open_questions/kernel/Q-M2.23-does-the-library-grow-a-reading-of-a-literal-vocabulary.md) bears on it

## What

**A stored negation of a verb does not reach the link the verb is read into.** The library reads a verb into the
link of a theory, `(includes sits-in same-chair)`, and the link back into the verb, `(top person-and-chair instance
Person Chair)` with `(meet same-chair person-and-chair sits-in)`; each one way. `meet` is R3 ⊇ R1 ∩ R2 and has no
negative reading (¬R3(a, b) ∧ R2(a, b) ⟹ ¬R1(a, b)), and `includes` has none, rightly, since a part's negation says
nothing of the whole. So *Cy does not sit in the first chair*, `(not (sits-in Cy First))`, never becomes `(not
(same-chair Cy First))`, the negation `std.slots`' `slot-negative` and eliminations read. The seating's step 2, with
both negations stated and the library's theory of shared places selected, is *several*; its step 3 needs a rule of
the program's own, `not-sitting-is-not-sharing`, to be *one*, *Bob sits in the first chair*. The Zebra puzzle
states no negation, so `zebra3.1.ein` needs none.

## Evidence

`einaf/contract/seating/step2.ein` and `step3.ein`; `einaf outcome FILE --single`; held by `einaf/tests/contract.rs`'s
`each_steps_program_is_judged_as_the_page_says`; `stdlib/algebra.ein`'s `meet`, `includes`, `compose-negative-s`
and `-r` (the only negative readings of an operation the algebra has, for a closed `compose`).

## What it costs the application

A sentence with *not*, one of the commonest clues of an attribute puzzle, forces a rule the model has to synthesise
in step 3 where the theory was otherwise selected; and a model that writes the negation of the link directly, `(not
(same-chair Cy First))`, has left the text's words in step 1.
