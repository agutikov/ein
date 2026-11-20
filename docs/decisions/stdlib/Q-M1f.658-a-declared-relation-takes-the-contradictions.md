# Q-M1f.658 — A declared relation takes the contradiction's rule of no argument too

A program that imports `std.contradiction` has, for each relation it declares, the rule of the relation's count
of arguments and the rule of no argument, `clash-0`. Every declaration stores the membership fact `(relation R)`
beside `(relation R LABEL…)` ([02-syntax.md](../../ein/user/04-reference/02-syntax.md)), and `clash-of-0` matches
it, so `(R)` beside `(not (R))` is a contradiction for a relation declared with labels as for one declared bare.

The instance matches no fact of a relation that has none of no argument: a fact of another count is turned away
by its count, with no lookup. What it costs is its being a rule instance every state holds.

**Why.** The membership fact is a part of the language, and `clash-of-0` reads it as any rule may. Taking the
instance away is the membership fact not stored, an `absent` over `relation` in the library, which flags a program
whose rule concludes a declaration, or a relation declared with labels left without the contradiction of its
fact of no argument. The user's ruling, of what the instance costs: *"this cost is absolutely acceptable"*.

**Held by.** `stdlib/contradiction.ein`'s `clash-of-0`; `tests/stdlib/contradiction/01_clash_0.ein` and
`09_clash_of_declared.ein`; `ein-infer/tests/contradiction_rules.rs`:
`each_clash_rule_fires_once_and_its_premises_are_the_pair`.
