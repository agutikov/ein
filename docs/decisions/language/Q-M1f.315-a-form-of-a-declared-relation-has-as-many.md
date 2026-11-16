# Q-M1f.315 — A form of a declared relation has as many arguments as its declaration has types

A `(relation R T₁ … Tₙ)` declares that a form of `R` has n arguments, and the loader refuses a program that writes
one with another count, on every command, one line a form, placed at the form: *the fact (likes Bob) gives likes 1
argument, and (relation likes Person Drink) declares 2 — a form headed by a declared relation has as many arguments
as its declaration has types*. It holds a fact, a fact inside a fact at any depth and under `(not …)`, a rule's and
an hrule's `:match` and `:assert`, a goal, and the places
[Q-M1f.204](Q-M1f.204-the-declared-count-is-held-in-a-using-a-claim.md) names. A declaration is the program's own
or an imported one. A keyword pair is no argument. A relation nothing declares has no count. It is no type check:
what is written in an argument's place is not held to the type declared for it. No flag turns it off.

**Why.** A form of another count is matched by no premise of the declared count, so a mistake in a count would be
answered by silence: the calculator's `(add (n z 5))`, one operand where `add` declares two, would get no answer
and no flag. No rule can read a form's count in general; the loader, which reads every form against its
declaration, can.

**Held by.** `ein-cli/tests/declared_counts.rs`: `a_form_of_another_count_is_refused_on_every_command`,
`each_form_has_a_line_of_its_own`, `an_imported_declaration_is_held_too`; `ein-ir/tests/fuzz_properties.rs`:
`no_accepted_program_holds_a_declared_relation_at_another_count`. The code: `ein_ir::from_ir`'s `wrong_counts`.
