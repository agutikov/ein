# E-111 — The answer in the statements' own words costs the kernel twice the facts

**Kind:** weakness
**Status:** open
**Filed:** by S2.2.0a, 2026-11-26, on `4bdbccaf`
**Owner:** none
**Since P2.2's close (2026-11-27):** none; it weighs on [Q-M2.38](../../open_questions/formalizer/Q-M2.38-is-a-description-abbreviated-to-its-word-or-written-as-a-term.md)

## What

**The answer in the statements' own words costs the kernel twice the facts.** The four `std.slots` encodings search
alike (353 saturations to describe R, 9 to decide `--single`, 112 candidates) and differ in what they write on the
way: `zebra.ein` 34 060 facts beyond root to describe R, `zebra3.2` 60 418, `zebra3.1` 74 213, `zebra3.3` 85 675.
The literal encodings carry the verbs, the descriptions or the adjectives through every state, and read the answer
back into the questions' verbs, which writes a link between two values `std.slots` keeps off its rectangle
(`(drinks Norwegian Water)` included into `same-house`). `ein render h-space` reads `zebra`'s pairs whole at the
default budget, in 705 808 facts, and stops on the three literal ones, whose layers take 1 733 960, 1 380 168 and
2 052 718 facts whole (`--region-budget 8388608`); Ein's sweeps of the structure leave the three out by name
(`ein-infer/tests/pair_layer.rs`' `PAST_THE_BUDGET`, `343edaf6`). `zebra2` and `zebra3.4`, five projections, take 5 411 and 5 600.

## Evidence

`einaf feedback FILE` (`work.facts`), or `ein solve FILE --json-summary S` (`work`), on each encoding under each
question: [`../zebra_formulations/probes/results/kernel.json`](../../p2.2_formalizer/zebra_formulations/probes/results/kernel.json);
`ein render h-space FILE`'s `whole` row,
[`../zebra_formulations/renderings/`](../../p2.2_formalizer/zebra_formulations/renderings/)`*.h-space.txt`. A rule of the file's own
that reads the answer back from the houses alone takes 50 622 facts where the library's `compose`, `top` and `meet`
over `same-house` took 86 854 (an earlier `zebra3.1`, without its converses).

## What it costs the application

The literal facts and the literal question are paid for in the kernel's facts, by a factor of two over the generic
link and of ten to fifteen over the projections; on a larger text the factor reaches the default budget sooner.
