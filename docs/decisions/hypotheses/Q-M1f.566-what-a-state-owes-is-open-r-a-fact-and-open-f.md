# Q-M1f.566 — What a state owes is `(open R)`, a fact, and `(open F)` is a hypothesis

**An obligation stays as it is**: a rule with a trigger and an `absent` over its witness whose whole `:assert` is
`(open R)` or `(open ?R)`, `R` a relation's name or a parameter that names one. The kernel counts its instances at
each state's fixpoint, and reads the facts that would pay them off its `absent`, as before. The bare `(open)` names
no relation and is refused at the parse.

**`(open R)` is held as a fact**, once, at a state that owes an instance of `R`; a state that owes nothing holds
none. **`(open F)`, `F` a fact, is a hypothesis**: any rule may conclude one, beside other facts
([Q-M1f.575](../language/Q-M1f.575-open-r-is-a-rules-whole-assert-and-open-f-stands.md)); a program may write one,
held at root alone; the facts an obligation's `absent` gives and the blind enumerator's guesses are written as
such.

**The `open` facts are written by a pass at each state's fixpoint** and held in a part of the state no fork
inherits: a fork drops every one, a written one among them, and the pass writes the child's at the child's own
fixpoint. Every rule that concludes `(open …)` is read at every state. No rule deletes a fact. A dead state is
written as any state is: it owes what the tally counts there.

**Why.** An obligation's verdict and a hypothesis are both about what a state has not decided. Held as facts, they can be
listed, drawn and read like the rest of a state; dropped at the fork and written again at the fixpoint, an `(open R)`
a commitment pays is gone from the state that pays it, with no rule that deletes a fact.

**Held by.** `ein_infer::open`, the pass, and `ein_core::Kb::write_open`, the part of a state no fork inherits;
`ein-infer/tests/open_facts.rs`: `the_seats_hold_what_they_owe_where_they_owe_it`,
`a_fork_drops_every_open_fact_a_written_one_among_them`; the grammar's table
(`ein-ir/tests/grammar_pages.rs`), a row for `(open)`.
