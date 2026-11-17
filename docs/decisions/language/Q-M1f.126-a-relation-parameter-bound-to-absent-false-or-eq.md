# Q-M1f.126 — A relation parameter bound to `absent`, `false` or `eq` is refused where the form it heads would be

`absent`, `false` and `eq` are names, so an activator may bind a rule's or an hrule's relation parameter to one:
`(mk absent)`, `(mk false)`, `(mk eq)`. Where the loader sees that binding, a written activator or one a rule
derives, the parameter is held to what the grammar refuses written out: it is refused for each premise or conclusion
it heads that the grammar would refuse with the word in its place, `(absent ?x)` at any arity, `(false ?x)` at one
argument or more, `eq` at any arity but two, and for each form it heads where an object goes, `(q (?R ?x ?y))`. One
line each, naming the rule: `rule 'mk': its parameter ?R heads (?R ?x), a form of 1 argument, and the fact (mk
absent) binds ?R to absent — absent is negation as failure, a premise's test, and no fact is of it`. The words stay
names: `(h absent)` and `(mk eq)` are facts.

**Why.** The grammar reads a variable where the relation is written, so a binding could build through a parameter a
fact the rule of what an object is ([Q-M1f.113](Q-M1f.113-an-object-is-an-atom-or-a-fact-a-state-could.md))
refuses written: `(absent a)`, `(false a)` or `(q (eq a a))`. Reading which parameters can hold `eq` is how a
parameter heading an `eq` form of another arity is refused
([Q-M1f.46](Q-M1f.46-a-relation-parameter-that-can-hold-eq-heads-only.md)); the same reading serves all three words.

**Held by.** `ein_ir::from_ir`'s `refuse_eq_at_another_arity`, run once per word; `ein-cli/tests/value_rule.rs`:
`a_form_that_is_no_value_is_refused_on_every_command` (the `(mk absent)`, `(mk false)` and `(mk eq)` shapes) and
`what_a_state_could_hold_loads` (`the-three-words-as-names`, `eq-as-a-parameters-relation`).
