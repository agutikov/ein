# Q-M1f.115 — An activator is a fact of its rule's name, and one of another count activates nothing

No list in a `(solve …)` names the rules that conclude a hypothesis
([Q-M1f.567](../hypotheses/Q-M1f.567-a-hypothesis-rule-is-a-rule-that-concludes-open.md)): such a rule is activated
as every rule is, by a fact of its name the program writes, with as many arguments as the rule has parameters. A fact
of its name with another count activates nothing, `(guess at Color)` beside `guess (?R ?T1 ?T2)`, and a fact whose
relation is a misspelt rule name, `(gues at Color House)`, is a fact of another relation; neither is refused, and
neither moves H but by what it does not activate.

**Why.** An activator is a fact as any rule's is, read by the checks every activator is read by; a rule's count of
parameters is what binds them, and a fact of another count binds none.

**Held by.** `ein_infer::compile::activators_for`, which takes the facts of the rule's name with its count of
arguments; `ein_infer::hypothesis_rules`, one plan per activator it takes.
