# Q-M1f.670 — A conflict with an `open` fact the program writes names where it is written

A load error of the hypothesis mechanisms
([Q-M1f.578](../hypotheses/Q-M1f.578-the-two-lists-and-a-closure-configure-the-blind.md)) that names an `open` fact
the program writes says where it is written, `root holds (open (q b)), written at FILE:L:C`, as a closure the
program writes is named, `(__closed__ q), written at FILE:L:C`. The place is that of the fact's head word, `open`: a
fact written at the top level keeps no position of its own
([Q-M1a.6](Q-M1a.6-a-loader-message-about-a-top-level-form-names-no.md)). The line is placed where the conflict is
written: a list's at the list, as every list's is; a closure's at the closure where the program writes it, and at the
written `open` fact where a rule derives the closure.

**Why.** A written `open` fact is one of the writers of H, as a rule that concludes one is
([Q-M1f.570](../hypotheses/Q-M1f.570-h-is-the-union-of-the-hypotheses-root-holds.md)), and the line names each
writer: a rule by its name, an obligation by its name, and a written fact, which has none, by its place. The reader
then drops the one of the two that is wrong, as the line's repair says.

**Held by.** `ein_infer::mechanisms::check`, which names an `open` fact's writer; `ein-cli/tests/mechanism_conflicts.rs`:
`a_conflict_is_refused_on_every_command`, its programs that write `(open (at Red H1))` or `(open at)` beside each list
and a closure, one line on every command. The page is
[`07-diagnostics.md` § 2.26](../../ein/user/04-reference/07-diagnostics.md#226-a-programs-hypothesis-mechanisms-do-not-contradict-one-another).
