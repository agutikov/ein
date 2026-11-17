# Q-M1f.391 — The negation of a negation is a fact

A stored negation is a fact, so it has a stored negation of its own: `(not (not g))` is the fact of `not` whose one
argument is the fact `(not g)`, and so at any depth. A file writes it wherever it writes a negation: as a fact, as
an argument, in a pattern, in a conclusion, in a goal and in a claim. A rule's conclusion `(not ?x)` builds one
where `?x` is bound to a stored negation
([Q-M1f.127](Q-M1f.127-a-conclusion-not-x-is-refused-where-the-program.md)). Written or built, it is one fact.

The kernel reads it as it reads any fact. A fact held with its negation is a contradiction, whatever the fact: so
`(not g)` beside `(not (not g))` is dead, and `g` beside `(not (not g))` is alive, since nothing there denies `g`.
Nothing concludes `g` from `(not (not g))`. That step is a rule a theory states, one rule for a count of arguments
with the relation a parameter:

```lisp
(rule eliminate (?R)
  :match  (not (not (?R ?a ?b)))
  :assert (?R ?a ?b)
  :why    "it is not so that {?a} {?R} {?b} is not so")
```

What `not` does not deny is unchanged: an atom
([Q-M1f.113](Q-M1f.113-an-object-is-an-atom-or-a-fact-a-state-could.md)), an equality, which is computed, and a
variable anywhere but as a conclusion of its own, so `(not (not ?x))` and an argument `(not ?x)` are refused
([Q-M1f.393](Q-M1f.393-a-rule-unpacks-a-fact-by-its-count.md)). The loader's `written` reads through every
negation: a negation nests what the fact it denies nests, and none of the facts denied on the way
([Q-M1f.275](Q-M1f.275-written-states-every-fact-a-file-nests.md)).

**Why.** A conclusion `(not ?x)` over a negation already stored the fact, so a state could hold what no file
could write, and the file of such a state did not load. The graph has no reason to refuse it: a negation is a node
with an arrow at a fact, and a negation is a fact. Reading the denial of a denial as the fact itself would be an
unpacking by a variable, which no rule has, and a step of one logic among several: a program that wants it says
so, as it says what any other context means.

**Held by.** `ein-ir/src/ein.y`'s `NegFact` and `NegPattern`; `ein_ir::from_ir`'s `Shapes::negation`, for what a
macro's expansion writes, and `denied`, for `written`; `ein-cli/tests/negation_of_a_negation.rs`: each place a
file writes one, a rule that matches and concludes one, *built by a rule or written in the file the answer is the
same* on `ein solve`, `ein oracle` and `ein test`, the two spellings of one rule under a search, what stays
refused, what the loader states, and the file of a state that holds one; the claims of
`examples/beliefs/03_liar_denial.ein` to `06_double_negation.ein`. The pages:
[the knowledge base](../../ein/reasoning/01-static-ir/01-knowledge-base.md), § A fact about a relation, a fact
about a fact, a negation, falsity, and § A fact, its negation, both, or neither;
[constraints](../../ein/user/03-syntax-and-semantics/06-constraints.md), § The negation of a negation.
