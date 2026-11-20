# Q-M1f.568 — Directly under `absent` or `open` the other is refused, but `(absent (open P))`

The kernel reads `absent` at the head of a premise, and `open` at the head of a written fact, a premise or a
conclusion. Directly under one of the two the other is refused, at the parse: `(absent (absent P))`,
`(open (open P))` and `(open (absent P))`. `(absent (open P))`, a premise over an `open` fact, is legal, and so are
a written `(open P)`, `(not (open P))` and `(open (not P))`.

**Why.** `absent`'s argument is a premise and `open`'s a relation's name or a fact a state could hold; `(absent P)`
is neither, and `(open P)` under `open` would be a hypothesis that a hypothesis may be supposed, which nothing reads.

**Held by.** `ein-ir/src/ein.y`'s `AbsentPremise`, `OpenFactArg` and `OpenTerm`; the grammar's table
(`ein-ir/tests/grammar_pages.rs`), a row for each refused shape;
[`07-diagnostics.md`](../../ein/user/04-reference/07-diagnostics.md#217-where-open-is-written) § 2.17.
