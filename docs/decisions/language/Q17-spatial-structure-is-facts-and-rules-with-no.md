# Q17 — Spatial structure is facts and rules, with no position arithmetic

A program states spatial structure as facts between positions and derives the rest by rules: five houses in a row
are four facts `(right-of House-2 House-1)` … `(right-of House-5 House-4)`, and `(includes right-of next-to)` with
`(symmetric next-to)` derive the eight `next-to` pairs. A spatial relation between attributes is carried to and
from positions by `std.slots`' rules over the co-location relation, `slot-adjacent-fwd` and `slot-adjacent-bwd`
among them, activated by `(slot-spatial co-located right-of instance House)`. No position is a number and no rule
computes one.

**Why.** Ein has no numbers ([Q-M1e2.24](Q-M1e2.24-ein-has-no-numbers-and-no-ranges.md)), and a position needs
none: what a puzzle says of order and adjacency is a relation between named positions, and each spatial step of a
derivation is a rule's firing a trace can show.

**Held by.** `examples/zebra.ein`, its row of houses and its `slot-spatial` activators; `stdlib/slots.ein`;
`tests/stdlib/slots/08_spatial_adjacent.ein`, adjacency carried across co-location with `:no-search`.
