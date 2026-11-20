# Q-M1f.572 — A state's `open` facts are part of it: listed, drawn and counted

A state's `open` facts are part of the state: every listing of a state lists them, `-p`, a dump, `ein saturate
--dump`, and every drawing draws them, each as a fact whose first arrow is at `open`; every count of a state's facts
counts them.

**Why.** Hypotheses and what a state owes are a valid part of the state.

**Held by.** `ein_core::Kb`'s reads, which walk the part of a state no fork inherits with the rest;
`ein-infer/tests/open_facts.rs`; [`08-read-out.md`](../../ein/user/04-reference/08-read-out.md).
