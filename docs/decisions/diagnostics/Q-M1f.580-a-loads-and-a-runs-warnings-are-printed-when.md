# Q-M1f.580 — A load's and a run's warnings are printed when the command's run ends

A `warning:` line on stderr says something true of a program that loads and changes no answer and no exit code:
the load's, read off the program's text, placed at the file, and the run's, of what a state did, placed at the rule's
`:match`. Every command prints them when its run of the program ends, after a flagged program's `WARNING` line,
which stays stderr's first ([Q-M1f.564](../answer/Q-M1f.564-a-non-monotone-program-loads-with-a-warning.md)): a
load's line once a process, a run's line once a run, and under `ein test` once a file.

The load's line for an activator a rule could derive, which binds a relation parameter to a fact
([Q-M1f.1](../language/Q-M1f.1-a-rule-that-could-derive-an-activator-binding.md)), names the deriving rule, its
conclusion, the activated rule and its parameter, and ends *— a relation is named by a symbol, not a fact, so such an
activator activates nothing*; the run's names the rule, the activator, the parameter and the fact.

**Why.** A line printed where it arises would put a load's warning before the read-out's `WARNING`, and a run's
between rows; printed at the end, each is said once, and the one line that says nothing is claimed of the answer is
the first a reader meets.

**Held by.** `ein-cli/tests/cli_semantics.rs`'
`a_derived_activator_binding_a_relation_parameter_to_a_fact_is_warned_of_at_load`, on every command;
`ein-cli/tests/non_monotone_warning.rs`, which holds the `WARNING` line first; `examples/features/19_derived_activator.ein`
and `20_derived_activator_never.ein`. The page is
[the diagnostics](../../ein/user/04-reference/07-diagnostics.md).
