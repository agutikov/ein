# L-M2.12 — The catalogue a step's grammar is rendered on is every module of the library and every rule of a parameter or more

**Status:** open
**Found:** by S2.2.3, 2026-11-26, rendering step 2's grammar for the Zebra puzzle
**Owner:** [S2.2.4](../../s2.2.4_passes.md), whose catalogue the steps' prompts show

## What

Steps 2, 2b and 3's grammars are parameterised on the catalogue. S2.2.4 writes the catalogue, each entry a program the
kernel runs; S2.2.3 came first and reads a stand-in off the library's texts (`einaf::grammar::steps::Catalogue::library`,
`einaf grammar catalogue`): every module of `std` but `std.core`, every name it declares as a name `:symbols` may
list, every rule of one parameter or more as an activator at its count, and every relation its rules read that no
module declares (`slot-partition`, `slot-spatial`, `bijective`). That is wider than what a program selects: the
calculator's modules declare 111 (`std.math.decimal-arith`) to 142 (`std.math.int`) names each, and
`std.math.positional`'s rules, which a base's own rules activate, are activators of the stand-in too. Step 2's grammar
for the Zebra puzzle is 55 kB, most of it the imports' lists of names; steps 2b and 3's, rendered on a program, close
over the modules the program imports and are 19 kB and 24 kB.

## Evidence

`einaf grammar catalogue`; `einaf grammar render 2 --program einaf/contract/zebra/step1.ein` on `680a94ad`
(55 492 bytes, 171 rules), and the speeds of [the stage's report](../../grammars/README.md#5-the-speeds), which
take the stand-in's size into account.

## What it costs

Nothing a speed shows (the speeds' table), and a grammar that allows a model to import and activate what the prompt
never showed it. S2.2.4's catalogue narrows step 2's grammar to the entries the prompt shows, and a grammar rendered on
it is what a run of the steps carries.
