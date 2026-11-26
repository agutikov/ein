# E-105 — `std.slots` lets two values stated next to each other share a house

**Kind:** bug
**Status:** open
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
