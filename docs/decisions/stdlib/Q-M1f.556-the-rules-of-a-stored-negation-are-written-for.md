# Q-M1f.556 — The rules of a stored negation are written for counts of none to six

`std.contradiction` and `std.negation` each have one rule for each count of arguments from none to six:
`clash-0` to `clash-6`, with `clash-of-0` to `clash-of-6` beside them, and `eliminate-0` to `eliminate-6`. A rule
is written for one count, since a pattern matches facts of its own count alone
([Q-M1f.447](../language/Q-M1f.447-a-relation-is-overloaded-by-its-count-of.md)) and no premise is a variable for a
whole fact. A fact of more arguments has no rule of the library: a program that wants one writes it, one rule of the
module's shape at that count.

**Why.** A rule reaches one count, so a set of them is a bound, and a rule no activator names is not applied, so a
count a program does not use costs it nothing. The corpus negates facts of few arguments, and its relations of seven
labels or more are the calculator's, whose facts no rule negates; six leaves room above what is used.

**Held by.** `stdlib/contradiction.ein` and `stdlib/negation.ein`; `tests/stdlib/contradiction/`, a program at
several counts; `ein-infer/tests/stdlib_coverage.rs`, which fails on a rule no program activates.
