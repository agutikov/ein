# Q-M1f.555 — The rules of a stored negation are modules a program imports

The two rules a stored negation can mean are two modules of the standard library, each a theory a program imports:
`std.contradiction`, a fact beside its negation concludes `(false)`
([Q-M1f.557](Q-M1f.557-the-contradiction-is-a-theory-a-program-imports.md)), and `std.negation`, the elimination of
a double negation, `(not (not g))` concludes `g`
([Q-M1f.559](Q-M1f.559-the-elimination-of-a-double-negation-is-a-module.md)). Each rule has its relation as a
parameter and is applied under an activator of its own, `(clash-2 seats)`, `(eliminate-2 likes)`, so a program
chooses the relations a rule reaches. Neither module declares a relation or states a fact. `std.logic` is not one of
them.

**Why.** Apart, a program imports the one it means: the elimination is a step of one logic, and the contradiction
is wanted where the elimination is not. A declaration an import brings is not inert, its labels being names of the importing
program, so a module that states no declaration changes nothing a program does not ask of it.

**Held by.** `stdlib/negation.ein` and `stdlib/contradiction.ein`; the programs under `tests/stdlib/negation/` and
`tests/stdlib/contradiction/`; `ein-infer/tests/contradiction_rules.rs`; `ein-infer/tests/stdlib_coverage.rs`, which
fails on a rule of the library no program activates. The page is
[negation as rules](../../ein/user/04-reference/04-stdlib/20-negation-and-contradiction.md).
