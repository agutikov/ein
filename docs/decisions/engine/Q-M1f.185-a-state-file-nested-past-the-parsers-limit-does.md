# Q-M1f.185 — A state file nested past the parser's limit does not load again

The parser refuses a program nested past 1 000 parentheses, before it parses it, with one line naming the `(` that
passes the limit. The engine holds a fact of any depth, and a rule can derive one deeper than the limit: a numeral of
the calculator nests a level a digit, and a step of a computation holds its operands. So a request whose operands
are near a thousand digits loads and is answered, and the state `ein solve --dump-answer` writes for it is refused
when it is loaded again. That is stated where the calculator's limits are, and left.

**Why.** The limit protects the walks over the parsed tree, macro expansion, the loader and the dumper, which
recurse. A state that deep is a very large file, one nobody would reload, so no request of the calculator pays to
flag a number that long.

**Held by.** `ein_ir::grammar::NESTING_LIMIT`; `ein-ir/tests/generated_parser.rs`: `the_nesting_limit`, which holds a
program at the limit loaded and one past it refused; [`stdlib/README.md`](../../../stdlib/README.md) § How long a
number may be.
