# Q-M1f.132 — A `std.macro` macro the program does not import heads no fact

In a program that does not import `std.macro`, a fact headed by `unknown` or `forall`, at any depth, is refused at
load, one line at the call: *the fact (unknown (paid Bob)) is a call of std.macro's macro unknown, which the
program does not import — …*. A rule's `:match` and a query's `:goal` that write either are refused too, the line
saying which import to add. A `(relation unknown …)` declaration in such a program loads; no fact of it can be
written and no premise reads it.

**Why.** No premise of the program can read a fact headed by `unknown` or `forall`, imported or not, so storing one
would store a fact nothing reads. A name `std.macro` gives a meaning is refused where it would be read as that
meaning without the import.

**Held by.** `ein-cli/tests/macro_facts.rs`: `a_macros_call_written_as_a_fact_is_refused_on_every_command`, the
rows `a-macro-not-imported` and `a-macro-not-imported-inside-a-fact`.
