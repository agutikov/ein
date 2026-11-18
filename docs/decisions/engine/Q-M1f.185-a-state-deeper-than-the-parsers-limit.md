# Q-M1f.185 — A state deeper than the parser's limit is answered, saved and loaded again

The parser refuses a program nested past 1 000 parentheses, before it parses it, with one line naming the `(` that
passes the limit. The engine holds a fact of any depth, and a rule can derive one deeper than the limit: a numeral of
the calculator nests a level a digit, and a step of a computation holds its operands. So a request whose operands
are near a thousand digits loads and is answered, and the state `ein solve --dump-answer` saves for it loads again:
a saved state is its program and the choices made, and no fact of it
([Q-M1f.392](../answer/Q-M1f.392-a-saved-state-is-saved-and-loaded-as-itself.md)), so what the parser reads of it is
the program, which it read already. A saved file's wrappers stand one level above the texts they carry, and the
limit is the texts'.

**Why.** The limit protects the walks over the parsed tree, macro expansion, the loader and the dumper, which
recurse. A state is derived, never parsed, so its depth is no reason to refuse it.

**Held by.** `ein_ir::grammar::NESTING_LIMIT`; `ein-ir/tests/generated_parser.rs`: `the_nesting_limit`, which holds a
program at the limit loaded and one past it refused, and `a_saved_file_carries_a_program_at_the_limit`;
[`stdlib/README.md`](../../../stdlib/README.md) § How long a number may be.
