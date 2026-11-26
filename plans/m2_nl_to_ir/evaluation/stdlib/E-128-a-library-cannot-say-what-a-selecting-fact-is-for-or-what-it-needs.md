# E-128 — A library cannot say, in the library, what a selecting fact is for or what its arguments need

**Kind:** gap
**Status:** open
**Filed:** at P2.2's close, 2026-11-27, from [the analysis of the phase, § 12](../../p2.2_formalizer/report/ANALYSIS.md#12-what-a-library-says-of-its-rules-to-whoever-selects-them),
on the user's note of the same day
**Owner:** the user, [Q-M2.37](../../open_questions/kernel/Q-M2.37-where-do-a-library-s-descriptions-of-its-rules-live.md);
EinAf's catalogue meanwhile ([S2.2d.3](../../p2.2d_formalizer_corrected/s2.2d.3_entries_described.md));
[E-27](E-27-no-one-catalogue-says-how-each-library-property-is-spelled.md) is its neighbour, the spelling where
this is the meaning

## What

**A library cannot say, in the library, what a selecting fact is for or what its arguments need.** A theory is
selected by a fact ([E-7](E-7-a-theory-is-selected-by-a-fact.md)), and three texts say what a rule is: its `:why`,
a sentence of one firing (for a check, its complaint: *{?R} not functional: {?a} has both {?b} and {?c}*; for
`slot-partition`, a list of ten activators); the reference page, in the algebra's words (`top` is R ⊇ Dom × Ran);
and EinAf's catalogue, by hand. All three say what a rule derives. None says its use: in every program of the
phase `top` is half of one idiom, a mask of a kind and the places that `meet` cuts the link by, and that is
written once, as a comment inside another entry's example. `slot-partition`'s one sentence holds five roles and
four conditions, and each misuse of the runs is a condition inside it that nothing sets apart. Neither
`std.algebra` nor `std.slots` declares its selecting facts, so no form of the library names an argument's role;
and no form of the language carries a description of a rule beside its `:why`.

## Evidence

`stdlib/algebra.ein` and `stdlib/slots.ein`, each rule's `:why`; `einaf catalogue`; the reference's pages on the
relation algebra and the slots; the model's selecting facts on the two attribute puzzles against the hand's,
rule for rule ([the analysis § 12.1](../../p2.2_formalizer/report/ANALYSIS.md#12-what-a-library-says-of-its-rules-to-whoever-selects-them)):
`converse` four written and four wrong, `compose` never, `top` and `meet` the example's shape with the names
changed.

## What it costs the application

Whoever selects, a model or a person, reproduces a shape and misses the meaning where the description is of the
mechanics; and a library given beside the standard one comes with nothing at all
([L-M2.30](../../findings/model/L-M2.30-modules-shown-with-no-description-were-imported-by-no-run.md)). EinAf
writes the missing words itself, apart from the library, where the two can part. A keyword of the language on
`rule` and `macro` would make the description a part of the form every tool reads: a change to Ein's syntax, which
waits for the iteration of Ein.
