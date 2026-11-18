# Q27 — A relation's properties are facts beside its declaration

A relation's algebraic properties are facts written beside its declaration, never inside it:

```lisp
(relation next-to Attribute Attribute :why "{?3}'s house is next to {?4}'s")
(symmetric next-to)
(includes right-of next-to)
```

Each property fact is the activator of a stdlib rule, `std.algebra`'s `symmetric`, `transitive`, `includes` and the
rest, and a fact of the graph like any other, with the relation's atom as its argument: it speaks of the relation as
one object ([Q-M1f.381](Q-M1f.381-a-relation-is-the-atom-under-a-facts-first-arrow.md)). A declaration takes a name,
its labels and `:why`, and nothing else.

**Why.** A property that is a fact is a node rules can match, derive and explain, and one more property is one more
fact, with no change to the declarator. Bundled into the declaration, it would be hidden in a form no pattern
reads.

**Held by.** The grammar's `Relation` production, `ein-ir/src/ein.y`, whose clauses are `:why` alone, and the row
`(relation R (T1 T2))` of the grammar's table, refused at the parse (`ein-ir/tests/grammar_pages.rs`:
`every_row_of_the_grammars_table_is_refused_at_the_parse`); `stdlib/algebra.ein`;
`tests/stdlib/algebra/21_transitive.ein`.
