# E-105 — `std.slots` lets two values stated next to each other share a house

**Kind:** bug
**Status:** closed by [S2.2e.2](../../p2.2e_kernel_fixes/s2.2e.2_slots_neighbours.md) (`b2d8276f`)
**Filed:** by S2.2.0a, 2026-11-26, on `4bdbccaf`
**Owner:** none; a fix is a change to a rule of `stdlib/slots.ein`, which leaves Ein's syntax and semantics as
they are ([Q-M2.16](../../open_questions/kernel/Q-M2.16-what-of-the-kernel-may-change-inside-m2.md)); not a row of
the wrong-answer register, since the kernel answers the rules as written and what is wrong is the rules against
their page
**Since P2.2's close (2026-11-27):** [S2.2e.2](../../p2.2e_kernel_fixes/s2.2e.2_slots_neighbours.md); [Q-M2.42](../../open_questions/kernel/Q-M2.42-are-e-104-and-e-105-fixed-before-the-benchmark-is-built.md)

## What

**`std.slots` lets two values stated next to each other share a house.** [`05-slots.md`](../../../../docs/ein/user/04-reference/04-stdlib/05-slots.md)
states `slot-spatial` as $S(a, b) \iff S(\operatorname{slot}(a), \operatorname{slot}(b))$, so with `next-to`
irreflexive two values `(next-to a b)` relates never share a slot. The rules `slot-prune-fwd` and `slot-prune-bwd`
exclude every position that is not an `S`-neighbour of the partner's *except the partner's own*, by a guard
`(neq ?p_o ?p1)` (and `(neq ?p_o ?p2)`), so a state that puts both in one house is not refuted. `zebra2.ein`'s
comment on its own `disjunctive-prune-*` rules says the same guard is wrong where `S` is irreflexive, and that
`zebra2-minus-15.ein` would answer 55 states with it and answers 32 without.

## Evidence

With sentence (15) dropped ([`../zebra_formulations/probes/damaged/`](../../p2.2_formalizer/zebra_formulations/probes/damaged/),
`*.drop-15.ein`): `ein solve FILE --maximal` lists **55** states for `zebra`, `zebra3.1`, `zebra3.2` and `zebra3.3`
(`std.slots`) and **32** for `zebra2` and `zebra3.4` (`std.bijection` and the file's own rules); of `zebra`'s 55,
**23** put Chesterfields in the fox's house or Kools in the horse's (`--dump-answer`, counted over the models).
With the two guards taken out of a copy of the library
([`slots-without-the-same-house-exemption.diff`](../../p2.2_formalizer/zebra_formulations/probes/slots-without-the-same-house-exemption.diff),
`EIN_STDLIB` set to the copy, its manifest re-taken), `zebra.drop-15.ein` and `zebra3.1.drop-15.ein` list 32, and
`zebra.ein` and `zebra3.1.ein` still answer `Solution`.

## What it costs the application

On the full puzzle nothing (the answer is unique either way); on an under-determined text, every encoding a
formalizer writes over `std.slots` answers more states than the text allows, and the kernel says *several* with 23
models no reading of *next to* admits. A formalization judged by its count of states is judged against a wrong
count.

## Closed

By S2.2e.2, `b2d8276f`, 2026-11-27: `slot-prune-fwd` and `slot-prune-bwd` lose their `(neq ?p_o ?p1)` and
`(neq ?p_o ?p2)`, and the `absent` that follows decides the placed end's own position as it decides every other, by
`?S` itself: kept where `?S` relates the position to itself, excluded where it does not, which is the page's
$S(a, b) \iff S(\operatorname{slot}(a), \operatorname{slot}(b))$ with no condition. The rules are brought to their page;
what the library means does not change, and no decision is written. The guard came from `zebra2.ein`'s own rules as
they stood when `std.slots` was written from them (`26141452`); `a27c363c` took it out of `zebra2.ein` three days
later, and `std.slots` kept it.

With sentence (15) dropped, the six encodings answer alike: 32 states on the maximal line and the same ten classes
of *who drinks water, who owns the zebra*, each over the same count of states, which is also `nl2smt`'s count, class
by class. The full puzzle answers `Solution` on each. Held by `ein-infer/tests/acceptance.rs`,
`without_sentence_15_every_encoding_answers_the_same_states` (the six corpus encodings with the sentence taken out,
side by side; it fails on the library before the fix), and by `tests/stdlib/slots/14_prune_placed_position.ein` (an
`S` that relates no seat to itself: the placed end's seat excluded) and `15_prune_reflexive.ein` (one that relates
each seat to itself: kept). What it cost the full puzzle, and what moved in the corpus, is
[the stage's record](../../p2.2e_kernel_fixes/s2.2e.2_slots_neighbours.md#record): no answer, the work of a
description by 1.5 % to 2.3 % in facts; and three counts of H's structure on `zebra.ein` (candidates dead alone at
root, witnesses held by a single's node, pairs excluded at root), each now `zebra2.ein`'s: three single placements
were alive alone only by the exemption.
