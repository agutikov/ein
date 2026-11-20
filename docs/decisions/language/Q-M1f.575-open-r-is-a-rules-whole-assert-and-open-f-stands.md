# Q-M1f.575 — `(open R)` is a rule’s whole `:assert`, and `(open F)` stands in a conjunction

What an obligation owes, `(open R)`, is its rule's whole `:assert`: beside another conclusion it is refused at load.
A hypothesis, `(open F)`, stands in a conjunction like any fact: `(and (open (p ?x)) (seen ?x))` concludes the
hypothesis and the fact.

**Why.** An obligation is read at the fixpoint and concludes no fact; a hypothesis is a fact a state holds.

**Held by.** `ein_ir::from_ir`'s `validate_open`; `examples/broken/open_mixed_assert.ein`.
