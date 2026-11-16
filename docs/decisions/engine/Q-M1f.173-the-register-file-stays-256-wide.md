# Q-M1f.173 — The register file stays 256 wide

A plan's register file holds 256 variables, the engine's one compile-time bound, and it stays so. The rules of the
corpus are held to an eighth of it, 32, all but those of `std.math.num9`, which are wide by what they are: generated
school arithmetic on nine digits in one fact, a premise a column, binding some multiple of nine variables. They are
held apart, the widest, `div9-start`, to its own count of 162 and under the bound, so that count moves only when the
generator does, and then a test fails.

**Why.** The file's fixed size is what keeps the matcher's inner loop free of allocation. A module whose width is
fixed by its generator is measured against its own number; every other rule keeps the margin that says the bound is
far from anything written by hand.

**Held by.** `ein-infer/tests/compile_limits.rs`: `the_corpus_is_nowhere_near_the_ceiling`, with its
`WIDEST_BY_DESIGN`. How the bound is counted: [Q-M1f.73](Q-M1f.73-a-rule-binds-at-most-256-variables-counted.md).
