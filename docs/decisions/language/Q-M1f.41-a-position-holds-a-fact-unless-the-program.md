# Q-M1f.41 — A position holds a fact unless the program passes it to a relation parameter

The load check of [Q-M1f.1](Q-M1f.1-a-rule-that-could-derive-an-activator-binding.md) reads which argument
positions can hold a fact off the program's text:

- an argument of a **declared** relation holds any object, a fact among them; so do an argument inside a fact
  pattern, `(p (g ?x))`, and an argument of a relation a parameter names, `(?S ?x)`;
- an argument of an **undeclared** relation, a marker such as `(bijective color-loc)`, and a rule's parameter hold
  what the program does with them: where a variable bound there reaches a rule's relation parameter, the position
  holds relation names, inferred to a fixpoint, and a fact or a conclusion that writes a fact there is refused.

So a fan-out that passes a relation's name through a marker loads, and a rule that could pass a declared relation's
argument to a relation parameter is refused; a program that declares its marker, `(relation mark T)`, says that the
marker's argument may be a fact, and is refused where it passes one on.

**Why.** It is the reading that refuses every program in which a rule could bind a relation parameter to a fact and
keeps every program that names relations through markers, the way `std.algebra`'s and `zebra2`'s fan-outs do. A
declaration states a label for a position and holds its argument to nothing, so a declared position is read as
holding any object; an undeclared one is read by use.

**Held by.** `ein_ir::from_ir`'s `refuse_derived_relation_activators`. `ein-ir/src/from_ir.rs`' unit test
`a_derived_activator_binding_a_relation_parameter_to_a_fact_is_refused_at_load`, each shape and what loads beside
them; `ein-cli/tests/cli_semantics.rs`' test of the same name, which loads `examples/zebra2.ein` and the stdlib's
fan-outs on every command.
