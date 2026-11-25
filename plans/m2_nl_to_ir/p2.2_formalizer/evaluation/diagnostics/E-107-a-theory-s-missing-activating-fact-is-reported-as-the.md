# E-107 — A theory's missing activating fact is reported as the statements contradicting each other

**Kind:** weakness
**Status:** open
**Filed:** by S2.2.0a, 2026-11-26, on `4bdbccaf`
**Owner:** none; [E-11](../../../evaluation/stdlib/E-11-a-theory-selected-under-a-name-the-file-does-not-list-is.md)
and [E-12](../../../evaluation/language/E-12-an-undeclared-relation-and-a-form-of-a-declared-relation-at.md) are its
silent half

## What

**A theory's missing activating fact is reported as the statements contradicting each other.** `(symmetric next-to)`
left out, or misspelt `(symetric next-to)`, gives the same outcome in each of the six Zebra encodings (a misspelt
selector is a fact of an undeclared relation, which loads in silence): `inconsistent` in `zebra`, `zebra3.1`,
`zebra3.3` and `zebra3.4`, whose core names the Norwegian's sentences, *The Norwegian lives in the first house* and
*The Norwegian lives next to the blue house*, with the houses, the row and the kinds (`zebra3.1`: `(lives-in
Norwegian First)`, `(lives-next-to Norwegian Blue)` and twelve facts of the ontology, refuted by
`std.slots.slot-elimination` and `slot-prune-bwd`; `zebra3.3`: its four facts, refuted by its own
`a-house-in-a-place` and `slot-endpoint-bwd`; `zebra3.4`: the first sentence only, refuted by `std.algebra.total`);
`incomplete` in `zebra2`; and `several`, 124 states, in `zebra3.2`. No line names `next-to` or the selector.

## Evidence

[`../zebra_formulations/probes/damaged/`](../../zebra_formulations/probes/damaged/), `*.drop-symmetric.ein` and
`*.misspell-symmetric.ein`; `einaf feedback FILE` has the outcome and `core`; `results/damage.json` holds them all.

## What it costs the application

The feedback points at two sentences that are right, and a repair loop that trusts the core changes a fact. P2.4's
principle, *the theory before the facts*, has nothing in the feedback to act on: the theory's slip and a
contradiction between two statements read alike.
