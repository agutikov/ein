# Q-M1f.557 — The contradiction is a theory a program imports

A fact beside its stored negation is a contradiction only where a rule says so. The kernel has no check of its own:
`not` is a relation of one argument, a convention of the standard library, and the meaning of a state that holds
`(p a)` and `(not (p a))` is what the program's rules make of it. `std.contradiction` states the usual one, for
every relation a program declares, by importing the module and nothing else:

```lisp
(import std.contradiction)
```

Its `clash-0` to `clash-6` conclude `(false)` from a fact of `R` beside its negation, one rule a count of arguments
with the relation a parameter, and its `clash-of-0` to `clash-of-6` derive each activator `(clash-N R)` from a
declaration of `R` with `N` labels; the rule every program gets then ends the state that holds the `(false)`
([Q-M1f.551](Q-M1f.551-std-core-is-loaded-into-every-program.md)). A relation no declaration names is reached by an
activator the program writes, `(clash-1 loose)`, and a fact of more than six arguments by a rule the program writes,
the module's at its count ([Q-M1f.556](Q-M1f.556-the-rules-of-a-stored-negation-are-written-for.md)). A program that imports
no such rule has no contradiction of that shape, and a state that holds a fact beside its negation is alive in it; a
program that wants another logic, three- or four-valued or paraconsistent, writes rules of its own.

A module of the standard library that concludes a stored negation and means a death by it imports the module:
`std.algebra`, `std.bijection` and `std.slots`, so a program that imports one of them has the contradiction.

What reads a pair for an economy reads the rules, with no name
([`ein_infer::contradiction::Pairs`](../../ein/ein.rs/03-design/06-saturation.md#61-contradiction)): a rule whose
match is a fact and one fact holding it as its one argument, and whose firing ends a state, refutes the two
together. Where one is compiled, a candidate root holds the other side of is not in `H`, a candidate an alive state
so denies is dead above it (L8), and the agenda fires first a conclusion that is either side; where none is, each
is an ordinary fact. An explanation reads a death by such a rule as the pair: its core is the frontier of the two
sides, and `refuted by` names the rules that derived them.

**Why.** The user's ruling: a contradiction is a property of a theory and not of the knowledge base, which is a
graph whose nodes are neither true nor false. A kernel that killed a state holding a fact beside its negation gave
every program classical negation; as rules, the program chooses, and the kernel judges a state by one word, the
engine's `inconsistent` ([Q-M1f.550](../language/Q-M1f.550-the-engines-word-is-inconsistent.md)).

**Held by.** `stdlib/contradiction.ein`, and the programs under `tests/stdlib/contradiction/`, the last of which,
`10_undeclared_not_reached.ein`, holds a fact beside its negation alive; `ein_infer::contradiction`'s
`a_pair_with_no_rule_over_it_is_no_contradiction` and `a_pair_a_rule_refutes_is_read_as_the_pair`;
`ein-infer/tests/contradiction_rules.rs`; every corpus program's answer, held by the corpus sweeps, the oracle
against every mode (`ein-infer/tests/explore.rs`) and `corpus/regressions.toml`.
