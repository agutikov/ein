# Q-M1f.578 — The two lists and a closure configure the blind enumerator, and one that disagrees with root’s open facts is an error

`:hypothesis-relations`, `:no-hypothesis` and `(__closed__ R)` are the blind enumerator's configuration, a white
list and a black list. Beside the `open` facts root holds, one of them that disagrees with them is a load error: an
`open` fact of a closed relation; a `:no-hypothesis` that names an `open` fact's relation; a
`:hypothesis-relations` that lists a relation no `open` fact is of, or leaves out one an `open` fact is of: beside
them the white list names exactly their relations. An `open` fact is of the relation it owes,
`(open R)`, or of its hypothesis's, `(open F)`. They are judged against the `open` facts root holds, on every
command, at load.

**Why.** The blind enumerator never runs beside hypotheses a rule writes or an obligation activated; a list that
says otherwise of them says what the program does not do.

**Held by.** `ein_infer::mechanisms::check`; `ein-cli/tests/mechanism_conflicts.rs`;
[`07-diagnostics.md`](../../ein/user/04-reference/07-diagnostics.md#226-a-programs-hypothesis-mechanisms-do-not-contradict-one-another) § 2.26.
