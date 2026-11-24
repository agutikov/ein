# Q-M1f.580 — A load's and a run's warnings are printed when the command's run ends

A line on stderr that says something true of a program that loads, and changes no answer and no exit code, is
printed when the command's run of the program ends, after a flagged program's `WARNING` line, which stays stderr's
first ([Q-M1f.564](../answer/Q-M1f.564-a-non-monotone-program-loads-with-a-warning.md)): a load's line once a process,
a run's once a run, and under `ein test` once a file. The load's `note:` lines, each a hypothesis mechanism's
([Q-M1f.242](../hypotheses/Q-M1f.242-a-list-that-only-repeats-what-another.md)), are such lines. An activator that
binds a relation parameter to a fact gets none
([Q-M1f.1](../language/Q-M1f.1-a-rule-that-could-derive-an-activator-binding.md)).

**Why.** A line printed where it arises would put a load's line before the read-out's `WARNING`, and a run's between
rows; printed at the end, each is said once, and the one line that says nothing is claimed of the answer is the first
a reader meets.

**Held by.** `ein-cli/tests/non_monotone_warning.rs`: `the_warning_comes_before_a_loads_note`, on every command that
answers about a commitment. The page is [the diagnostics](../../ein/user/04-reference/07-diagnostics.md).
