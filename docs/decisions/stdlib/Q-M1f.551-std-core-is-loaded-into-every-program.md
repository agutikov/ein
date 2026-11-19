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
`ein render rules` and shown by a trace where it fires. An explanation names the rules a program wrote or imported,
so a dead state's `refuted by` names the rule that concluded the `(false)` and reads through this module's rule to
what its firing matched.

**A file may import it, as any module.** `(import std.core)`, in a program or in a module, in any of an import's
three shapes, binds the module's names as an import does, so that `:symbols (inconsistent)` writes and prints the
rule `inconsistent` and `:as c` prints it `c.inconsistent`, and brings no second copy. The module stays where every
load puts it, after the program's forms and its imports', so a program lists the same rules in the same order, and
answers what it answers, with the line and without.

**A library `$EIN_STDLIB` names replaces it.** The module is read from the standard library the load resolves, so a
library `$EIN_STDLIB` names, which its `MANIFEST.sha256` identifies as one, brings its own `core.ein` into every
program in the place of the checkout's or the embedded one. A library there with no `core.ein` refuses every
program, one line at the load-error exit, *std.core was not found in $EIN_STDLIB*, and the directory. A saved file
carries the module with the program's other modules; a container stores the program without it, and the load that
opens the container brings it. No way to leave the module out of a program is built.

**Why.** A program that writes `(false)` and imports nothing must still refute, and most programs write it: left to
an import, the convention would make every such program unable to say a state is dead. Loaded into every program,
the rule keeps each program's answer what it was while the kernel reads no fact by its name. An import of it changes
nothing, as a second import of any module changes nothing, so it is no error to write one; and the library a user
points `$EIN_STDLIB` at is the library every program reads, the module every program gets among it.

**Held by.** `stdlib/core.ein`; `ein_ir::imports::CORE` and `Resolver::resolve_program`, which the loader reads every
program through, with `Resolver::implicit_names` and `ein_core::Rule::implicit`; `ein_infer::contradiction`, which
reads no `(false)`, and its `a_held_false_is_no_contradiction_by_its_name`; `ein-infer/tests/search_semantics.rs`'
`witness_is_the_positive_for_a_pair_and_the_matched_fact_for_an_end`; the programs under `tests/stdlib/core/`, and
`ein-infer/tests/stdlib_coverage.rs`, which fails on a rule of the library no program activates;
`ein-ir/tests/imports_semantics.rs`' `an_import_of_the_module_every_program_gets_is_an_import_like_any_other` and
`a_library_with_no_std_core_is_refused_and_one_with_another_is_read`; `ein-cli/tests/import_invariance.rs`'
`an_import_of_the_module_every_program_gets_changes_no_answer`; `ein-cli/tests/cli_semantics.rs`'
`ein_stdlib_replaces_std_core_and_a_library_without_one_is_refused`; `examples/features/21_import_of_core.ein`;
`ein-cli/tests/saved_files.rs` and `ein-einb/tests/roundtrip.rs`, a saved file's and a container's round trips.
