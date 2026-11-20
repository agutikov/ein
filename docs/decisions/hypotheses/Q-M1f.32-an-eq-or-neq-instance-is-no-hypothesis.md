# Q-M1f.32 — An `eq` or `neq` instance is no hypothesis

A commitment states facts a state stores, and a state stores no `eq` or `neq` fact: both relations are computed. So
no writer of $H$ proposes an instance of either. `(open (eq ?x ?y))` written out is refused at the parse, `eq` heading
no fact; a hypothesis whose relation is a parameter an activator binds to `eq` or `neq`, `(open (?R ?x ?y))` beside
`(guess eq)`, adds nothing to H, however many rows its `:match` has.

**Why.** A commitment that adds nothing, or adds only `(false)`, is no hypothesis about the program's facts: a true
instance would reach a state of R a second time, and a false one would put in N a member no fact explains.

**Held by.** `ein-ir/src/ein.y`, where `open`'s argument is a fact; `ein_infer::hypothesis_rules`, which proposes no
hypothesis `firing::computed` answers; `examples/ein-bugs/wrong-answers/hrule-eq.ein`, whose H is empty and R {∅};
`ein-infer/tests/claim_round_trip.rs`: `every_printed_region_holds_as_its_own_claim`, since no claim names `eq` or
`neq`; [`02-syntax.md`](../../ein/user/04-reference/02-syntax.md), on a conclusion of `eq` or `neq`.
