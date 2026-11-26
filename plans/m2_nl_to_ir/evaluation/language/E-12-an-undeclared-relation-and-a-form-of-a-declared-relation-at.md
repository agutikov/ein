# E-12 — An undeclared relation, and a form of a declared relation at another count, load in silence

**Kind:** weakness
**Status:** open
**Filed:** by [the analysis](../../analysis/README.md), 2026-11-25, on `458bf90b`, before any stage ran
**Owner:** the design: [Q-M1f.372](../../../../docs/decisions/language/Q-M1f.372-ein-has-no-types-a-declarations-slot-names-are.md), [Q-M1f.447](../../../../docs/decisions/language/Q-M1f.447-a-relation-is-overloaded-by-its-count-of.md)

## What

**An undeclared relation, and a form of a declared relation at another count, load in silence.** Ein has no types, and a relation is overloaded by its count

## Evidence

[analysis § 2](../../analysis/README.md#2-what-the-kernel-says-today-where-the-plan-expects-the-walk); [`bob1_library_alone_wrong_count.ein`](../../analysis/probes/p2.2/bob1_library_alone_wrong_count.ein), a goal written with three arguments where the relation has four: it loads, says `Solution`, and *the goal has no row*

Confirmed by S2.2.2, 2026-11-26, on the first programs a model wrote with no grammar: `library.off.bob1` and `library.off.more` of [the probe's record](../../p2.2_formalizer/model_runtime/README.md) conclude `has-more`, which they never declare, and load; the contract's check, not the kernel, is what would say so. And a name of two words written with a space is two arguments: the model's step 1 of the Zebra puzzle (`step1.off.zebra`) writes `(owns Old Gold snails)` and `(drinks Lucky Strike orange)` beside `(relation owns Person Thing …)`, two facts at a count no declaration has, which load in silence: no rule and no query of the program reads them.

## What it costs the application

a model's misspelt relation or dropped argument is no diagnostic: the first rung of the feedback ladder has little to say
