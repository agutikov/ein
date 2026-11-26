# E-109 — A sentence written as a rule's activating fact is in no core

**Kind:** weakness
**Status:** open
**Filed:** by S2.2.0a, 2026-11-26, on `4bdbccaf`
**Owner:** none; [E-51](E-51-a-core-holds-facts-that-have-no-sentence.md) is
its other side
**Since P2.2's close (2026-11-27):** none; it weighs on [Q-M2.38](../../open_questions/formalizer/Q-M2.38-is-a-description-abbreviated-to-its-word-or-written-as-a-term.md)

## What

**A sentence written as a rule's activating fact is in no core.** Add *Coffee is drunk in the first house* to each
encoding: every one is `inconsistent` at root, and the core says why in each vocabulary's terms. In the literal
encodings it is the text: `zebra3.1`'s core holds `(is-drunk-in Coffee First)`, `(is-drunk-in Coffee Green)` and
`(is-immediately-to-the-right-of Green Ivory)`, the added sentence and sentences (4) and (6), with the houses, the
row and the kinds (fifteen facts); `zebra3.3`'s is those three sentences in its words with `(first House-1)` and a
house, and `zebra3.2`'s the same sentences in its own, six facts. `zebra.ein`'s holds sentence (4) and (6) too,
`(co-located Coffee Green)` and `(right-of Green Ivory)`. In
`zebra2.ein` and `zebra3.4.ein`, where sentence (4) is an activator of the rule `co-located` or `same-house`, the core
is the added fact, a membership and a declaration, `(drink-loc Coffee House-1)`, `(is-a House-1 House)`,
`(relation color-loc Color House)`, refuted by `co-located` and `co-located-negative`: the two sentences it
contradicts are named only as the rules they activate.

## Evidence

[`../zebra_formulations/probes/damaged/`](../../p2.2_formalizer/zebra_formulations/probes/damaged/), `*.add-coffee-first.ein`:
`einaf feedback FILE`'s `core.facts` and `core.refuted_by`; `results/damage.json`.

## What it costs the application

The feedback a model gets for a contradiction it wrote names, in a literal encoding, the sentences to look at; in
an activator encoding, a rule's name and a declaration, from which the model has to find the sentences itself. A
vocabulary that writes clues as activators makes the kernel's best explanation worse.
