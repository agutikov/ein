# Q-M1f.72 — A nested `(or …)`, a wide `:match` and a goal's relation variable are refused as loader lines

Three shapes no plan can be compiled for are refused at load, on every command that reads the program, each with
one line of the shape the loader's other refusals of a rule have: the rule named as the loader names it, the form
as Ein text, then what to write, joined with the loader's other lines by `; ` behind `kb load error: `, with no
position:

- `rule 'r': its :match writes (or (p ?x) (q ?x)) as a premise of and — an (or …) is a whole :match, …`;
- `rule 'wide': its :match writes 257 distinct variables, and a rule binds at most 256 — split the rule`;
- `query: its :goal writes (?R k) with ?R as its relation, which nothing binds — …`.

A form a macro's expansion writes is the macro's, and its line carries the position where the form is written
([Q-M1f.82](Q-M1f.82-a-malformed-primitive-a-macro-writes-is-refused.md)).

**Why.** One spelling for every line about a rule's shape: the rule's name finds the form, and the form is printed
as written.

**Held by.** `ein-ir`'s `from_ir::tests::the_compilers_refusals_a_program_reached_are_refused_at_load`;
`ein-cli/tests/cli_semantics.rs`: `a_compile_refusal_reads_the_same_on_every_command`;
`examples/broken/load/nested_or.expected`. The limit is `ein_ir::MAX_MATCH_VARIABLES`.
