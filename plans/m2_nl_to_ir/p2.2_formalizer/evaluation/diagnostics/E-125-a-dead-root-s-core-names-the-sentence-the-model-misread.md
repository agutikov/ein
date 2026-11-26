# E-125 — A dead root's core names the sentence the model misread, and the rule that found it

**Kind:** strength
**Status:** standing (a strength)
**Filed:** by S2.2.4, 2026-11-26, on `7aaefe6f`
**Owner:** —

## What

**Where a model's program is dead at root, the judgement at root (`--no-search`) carries a core whose facts quote
their sentences, and the rule that refuted them**: an account of the misreading a repair can act on. Of S2.2.4's runs:

| run | the core's facts and their sentences | refuted by | the misreading |
|---|---|---|---|
| `steps/zebra`, step 2 and the complete program | `(instance GreenHouse House)`, *Coffee is drunk in the green house.*; `(type Color Attribute)` | `std.slots.slot-no-fill` | a coloured house made a house of its own, and four kinds left with no member: eleven houses for five places, either fault alone deadly (L-M2.23) |
| `steps/seating` | `(instance Ann Chair)`, `(instance Bob Chair)`, *Ann, Bob and Cy sit in a row of three chairs.*; `(sits Ann Bob)`, *Ann sits next to Bob.*; `(type Chair Attribute)` | `std.contradiction.clash-2` | the people written as chairs |
| `whole/bystander` | `(gave Bob 2 Apple Alice)`, *Bob gave 2 apples to Alice.*; `(has Bob (n z 5) Apple)`; `(std.math.digits.nz 2)` | `std.math.positional.malformed-is-false` | a number written as a name where the calculator reads it |
| `whole/pencils` | `(has-boxes School 12 Box)`, `(has-pencil-in-box Box 144 Pencil)`, *A school has 12 boxes of pencils, 144 pencils in each.* | `std.math.positional.undeclared-is-false` | the same, twice |

A program alive at root, with no hypothesis mechanism yet, is *not decided* and says nothing more
([E-114](../hypotheses/E-114-a-partial-program-s-h-is-set-by-its-relations-count-of-places.md)); a dead one says
where. The core is an explanation, not minimal and not canonical
([Q-M1f.6](../../../../../docs/decisions/answer/Q-M1f.6-a-core-is-an-explanation.md)); here each named the
misread sentence first. **Where root dies of several faults, the core names one**: the model's Zebra program dies
of either of two (L-M2.23), and its core names one house's sentence of eleven, *Coffee is drunk in the green house.*;
with `(config :record-alternative-justifications false)` it names *The Norwegian lives next to the blue house.*,
the other fault's. Under `--single` the same programs carry no core
([Q-M1f.29](../../../../../docs/decisions/read-out/Q-M1f.29-a-question-that-prints-no-core-writes-a-dead.md)).

## Evidence

[S2.2.4's record](../../steps/README.md): each run's `run.json`, `steps[].judged.feedback[0].core` (the step's
program at root) and `complete.feedback[0].core`.

## What it is worth to the application

The feedback ladder's rung that shows a core has the sentence to show beside the rule; the misreading is in step 1's
facts or step 2's ontology, the parts a repair changes last, and the core says which.
