# Q40 — A fact may hold a fact as an argument

An argument of a fact is an atom or a fact: `(hypothesis (co-located Norwegian House-2))` is a fact of `hypothesis`
whose one argument is the fact `(co-located Norwegian House-2)`. The nested fact is an ordinary object, identified
by its relation and its arguments at every depth
([Q-M1f.380](Q-M1f.380-an-objects-identity-is-its-name-or-its-tuple.md)), and a pattern matches into it, `:match
(hypothesis (co-located ?a ?b))` binding `?a` and `?b`. No relation of this kind is the kernel's: a program declares
`hypothesis` and writes its facts as it writes any other.

**Why.** A statement about a fact, a hypothesis, a belief, a claim or a numeral built of digits, is then a fact of
the graph that rules read and derive like any other, with no second kind of node and no procedure outside the
matcher.

**Held by.** `ein-infer/tests/rule_semantics.rs`: `a_nested_pattern_binds_inside_the_nested_fact`;
`ein-infer/tests/search_semantics.rs`: `a_nested_positive_pairs_through_structural_identity`;
`examples/saturation/hypothesis-contradiction/coloc-disproved.ein`.
