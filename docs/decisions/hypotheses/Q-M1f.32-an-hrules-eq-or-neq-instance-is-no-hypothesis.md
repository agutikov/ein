# Q-M1f.32 — An hrule's `eq` or `neq` instance is no hypothesis

A commitment states facts a state stores, and a state stores no `eq` or `neq` fact: both relations are computed. So
the hrule rung proposes no instance of either. An hrule whose `:assert` is `(eq ?x ?y)`, or `neq`, or a pattern a
parameter binds to one of them, adds nothing to H, however many rows its `:match` has.

**Why.** A commitment that adds nothing, or adds only `(false)`, is no hypothesis about the program's facts: a true
instance would reach a state of R a second time, and a false one would put in N a member no fact explains.

**Held by.** `ein_infer::hrule`, whose plan proposes only an `:assert` template that `firing::computed` does not
answer; `examples/ein-bugs/wrong-answers/hrule-eq.ein`, whose H is empty and R {∅};
`ein-infer/tests/claim_round_trip.rs`: `every_printed_region_holds_as_its_own_claim`, since no claim names `eq` or
`neq`; [`syntax.md`](../../ein/user/reference/syntax.md), on a conclusion of `eq` or `neq`.
