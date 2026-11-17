# Q-M1f.372 — Ein has no types: a declaration's slot names are labels

Ein has no types. No name, variable, relation, rule or fact has one, and the engine never asks whether a value is
of a kind. `(relation is-instance Object Type :why "…")` is almost an ordinary fact: the loader stores it as
`(relation is-instance Object Type)` beside `(relation is-instance)`, and `Object` and `Type` are its second and
third arguments, ordinary names. They are **labels**: they say to a reader what an argument is meant to be, mean
what the program's rules make them mean, and mean nothing where no rule reads them. No argument is held to a label:
`(lives-in House-1 Norwegian)` loads beside `(relation lives-in Person House :why "…")`.

A rule gives labels a meaning by matching the declaration as it matches any fact:

```lisp
(rule first-argument-fits (?R)
  :match  (and (relation ?R ?T1 ?T2) (?R ?x1 ?x2) (is-instance ?x1 ?T1))
  :assert (fits (?R ?x1 ?x2))
  :why    "{?x1} is an instance of {?T1}, the label {?R} declares for its first argument")
```

It finds a declaration of `?R`, a fact of `?R`, and an `is-instance` arrow from that fact's first argument to the
declaration's first label, which a program may call *the type of `?x1` is `?T1`*. A relation written as a variable
is a parameter of its rule, so an activator names the relation, `(first-argument-fits lives-in)`, and a second rule
that matches `(relation ?R ?T1 ?T2)` and concludes `(first-argument-fits ?R)` supplies one for every declaration of
two labels. `is-instance` is the program's relation, as `is-a` is in another program: no membership relation is
built in.

A declaration is *almost* an ordinary fact, in four things. It fixes the count of arguments a form of the relation
has ([Q-M1f.315](Q-M1f.315-a-form-of-a-declared-relation-has-as-many.md)). It carries the `:why` a fact of the
relation reads as. It is written once for a relation. And it tells the blind enumerator what to guess about: that
rung proposes facts of declared relations of one or two arguments, over the program's names less those some
declaration writes as a label
([Q-M1e.12](../hypotheses/Q-M1e.12-the-blind-enumerator-is-untyped-and-a-type-it.md),
[Q-M1f.306](../hypotheses/Q-M1f.306-the-blind-enumerators-objects-are-the-programs.md)). That is the one place the
engine itself reads which names are labels, and it reads them as names not to guess about, never as a kind a value
has. A fact of `relation` that a rule concludes is a fact and declares nothing.

Where a drawing shows a box, it is a drawing convention: `ein render kb` draws the second argument of a fact of a
relation spelt `is-a` as a box and a name that is only a first argument as an oval, and `ein render program` draws
the first two labels of a declaration as boxes where no fact names them. Where the engine's own words say *type*,
the read-out's `types`
row, the option `--no-types` and a refusal's *as many arguments as its declaration has types*, they mean a label, or
a name on the right of an `is-a` fact. No answer depends on either.

**Why.** Membership is a relation and what a thing is classified under is a name, so a rule reads, derives and
explains *what a thing is* as it does any fact, and a program chooses what its labels mean: nothing, a check, a
hierarchy, inheritance. A type the kernel knew would be a second record of what a fact already says, one no rule
could read and no trace explain, and it would fix one meaning of *is a* for every program.

**Held by.** `ein-cli/tests/declared_counts.rs`: `what_is_not_a_declared_relations_form_loads`, the case
`not-a-type-check`, an argument that is not what its label says, loaded on every command;
`ein_ir::from_ir`'s unit test `a_declaration_stores_itself_as_facts_so_rules_can_read_signatures`;
`ein_infer::hypgen`'s unit test `candidate_objects_drop_signature_atoms_and_primitives`, the one thing a label
does; `tests/stdlib/typing/03_type_hierarchy_welltyped.ein`, a hierarchy of labels checked by rules. The page:
[objects](../../ein/user/03-syntax-and-semantics/01-objects.md), § No types, whose rule is the one above, run and
drawn.
