# Q-M1f.23 — Where root decides a flagged program's minimal line, the route row names the cube

On a program the static check flags, a question about the minimal line where ∅ is in R is decided by root's node,
one saturation: ∅ is below every commitment, so the line is {∅}. Its `route` row reads `2, the cube — L7 the cube`:
the route is the cube in layer order, and its first layer, root, decides the line. The `exhausted` or `decided` row
says the line is all of it.

**Why.** The stop follows from the minimal line's definition, not from a theorem about a class of programs, so no
licence of its own describes it; the cube's own licence and the rows under it say what happened without a label
added to the vocabulary.

**Held by.** `ein-infer/tests/explore.rs`: `every_mode_is_the_oracle`, whose ∅ rule holds every minimal question
of every program where ∅ is in R, the flagged ones among them, to one saturation and to the oracle's cube. The
`route` row on such a run: [09-read-out-examples.md](../../ein/user/04-reference/09-read-out-examples.md), written
by the binary.
