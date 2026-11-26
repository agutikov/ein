# L-M2.22 — A fixed text scored right can rest on a contradiction, or a missing row, of the model's own making

**Status:** open
**Found:** by S2.2.5, 2026-11-26, reading the one shot's records against their gold answers
**Owner:** [S2.5.3](../../../p2.5_harness/s2.5.3_metrics.md), the answer's comparison; the stand-in reader is this
stage's

## What

**The stand-in reader scores what is printed, and a fixed text is the same string whatever made it.**
`zebra-contradiction`, the Zebra puzzle with *The Norwegian drinks milk.* added, prints *The statements contradict
each other.*, its gold, and is read right; but its root is dead at step 2 on a contradiction of the model's own
(`std.contradiction.clash-2` over `(instance Englishman Person)`, `(instance First House)` and
`(is-immediately-to-the-right-of First Second)`), and the core names no sentence about milk. The undetermined text
of `gap` and `kinds`, read right, is the answer query's missing row, which comes there from a calculator's name the
import does not list (`name-not-listed`), not from the quantity the text does not state.

## Evidence

`ein solve runs/library/zebra-contradiction/program.ein`: *unsat core* and *source* lines, *refuted by
std.contradiction.clash-2*; [`tables.md`](../../from_text/tables.md), the library condition's *read* column against its findings.

## What it costs

A score of the fixed texts counts a right string for a wrong reason. S2.5.3's reader takes the gold claim the kernel
checks beside the string; for an unsatisfiable instance that could be that the core names the sentence the
contradiction is about, and for an undetermined one that the program states what the text states.
