# Q-M1f.551 — `std.core` is loaded into every program

`std.core` is a module of the standard library that every program gets with no import line: the load brings it whole,
after the program's own forms and those of the modules it imports, under its full names. It holds one rule, from
`(false)` to the engine's word
([Q-M1f.399](../language/Q-M1f.399-a-rule-states-facts-or-a-word-to-the-engine.md),
[Q-M1f.550](../language/Q-M1f.550-the-engines-word-is-inconsistent.md)):

```lisp
(rule inconsistent ()
  :match (false)
  :ein   inconsistent
  :why   "a state that derives a contradiction is inconsistent")
```

So `(false)` is a convention of the standard library and no word the kernel reads: a state that holds it is dead by
that rule's firing. The rule is a rule of the program as any other is, listed by `ein saturate --dump`, drawn by
`ein render rules` and shown by a trace where it fires. No import names the module, in a program or in a module, and
one that does is refused. A saved file carries it with the program's other modules; a container stores the program
without it, and the load that opens the container brings it. An explanation names the rules a program wrote or
imported, so a dead state's `refuted by` names the rule that concluded the `(false)` and reads through this module's
rule to what its firing matched. No way to leave the module out, replace it or add to it is built.

**Why.** A program that writes `(false)` and imports nothing must still refute, and most programs write it: left to
an import, the convention would make every such program unable to say a state is dead. Loaded into every program,
the rule keeps each program's answer what it was while the kernel reads no fact by its name.

**Held by.** `stdlib/core.ein`; `ein_ir::imports::CORE` and `Resolver::resolve_program`, which the loader reads every
program through, with `Resolver::implicit_names` and `ein_core::Rule::implicit`; `ein_infer::contradiction`, which
reads no `(false)`, and its `a_held_false_is_no_contradiction_by_its_name`; `ein-infer/tests/search_semantics.rs`'
`witness_is_the_positive_for_a_pair_and_the_matched_fact_for_an_end`; the programs under `tests/stdlib/core/`, and
`ein-infer/tests/stdlib_coverage.rs`, which fails on a rule of the library no program activates;
`examples/broken/load/import_of_core.ein`, the refused import; `ein-cli/tests/saved_files.rs` and
`ein-einb/tests/roundtrip.rs`, a saved file's and a container's round trips.
