# Q-M1f.245 — The hypothesis mechanisms are judged at load, on every command

The check that the hypothesis mechanisms do not contradict one another runs at load, on every command that loads a
program. The one load path loads the program a second time into tables of its own, saturates that root and judges it,
wherever the program could declare two mechanisms at all; the command's own tables are untouched. The views and
`ein kb save` refuse as `ein solve` does, with the same line, and no container is written. A library caller calls
`ein_infer::mechanisms::check` itself; the library's routes do not.

**Why.** A derived closure and a derived activator exist only at root's fixpoint, and several commands never saturate
root; judged only where a command saturates, one program would load under one command and be refused under another.

**Held by.** `ein-cli/src/common.rs`, which calls `ein_infer::mechanisms::may_conflict` and then `check`;
`ein-cli/tests/mechanism_conflicts.rs`: `a_conflict_is_refused_on_every_command`;
[`07-diagnostics.md`](../../ein/user/04-reference/07-diagnostics.md#226-a-programs-hypothesis-mechanisms-do-not-contradict-one-another) § 2.26.
