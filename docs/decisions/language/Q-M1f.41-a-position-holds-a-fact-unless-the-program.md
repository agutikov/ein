# Q-M1f.41 — A position holds a fact unless the program passes it to a relation parameter

The load reads which argument positions hold relation names off the program's text, for two things: the refusal of
`eq`, `absent` and `false` where a relation name passes at another arity
([Q-M1f.46](Q-M1f.46-a-relation-parameter-that-can-hold-eq-heads-only.md)), and the relations of the program, every
name it passes where one is held ([Q-M1f.522](Q-M1f.522-a-relation-of-the-program-is-any-atom-it-writes.md)):

- a rule's parameter that heads a form holds relation names, and so does every argument of `(relation …)`;
- an argument of a **declared** relation holds any object, a fact among them; so do an argument inside a fact
  pattern, `(p (g ?x))`, and an argument of a relation a parameter names, `(?S ?x)`;
- an argument of an **undeclared** relation, a marker such as `(bijective color-loc)`, and a rule's parameter hold
  what the program does with them: where a variable bound there reaches a rule's relation parameter, the position
  holds relation names, inferred to a fixpoint.

So a name a fan-out passes through a marker, `color-loc` in `(bijective color-loc)`, is read as a relation's name;
a program that declares its marker, `(relation mark T)`, says that the marker's argument may be any object, and its
argument is not read as one. Which positions hold relation names says nothing of an activator that binds a relation
parameter to a fact, which activates nothing in silence
([Q-M1f.1](Q-M1f.1-a-rule-that-could-derive-an-activator-binding.md)).

**Why.** It reads a relation's name wherever the program passes one on, through a rule's parameter or a marker, the
way `std.algebra`'s and `zebra2`'s fan-outs do, and nowhere the program has said any object may stand. A
declaration states a label for a position and holds its argument to nothing, so a declared position is read as
holding any object; an undeclared one is read by use.

**Held by.** `ein_ir::from_ir`'s `read_relation_places`. `ein-ir/src/from_ir.rs`' unit tests
`eq_at_another_arity_is_refused_at_load`, whose marker case passes `eq` through `(mark eq)`, and
`a_derived_activator_binding_a_relation_parameter_to_a_fact_loads`, where a name passed through a marker is a
relation of the program and one written at a declared relation's argument is not.
