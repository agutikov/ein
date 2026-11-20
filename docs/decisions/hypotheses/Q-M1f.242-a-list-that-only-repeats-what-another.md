# Q-M1f.242 — A list that only repeats what another declaration says is a note

Where root holds no `open` fact, the lists are the blind enumerator's, and a `:no-hypothesis` name that
`:hypothesis-relations` already leaves out, or that a `(__closed__ R)` already closes, is one `note:` line on stderr.
Beside the `open` facts root holds a list either disagrees with them, a load error
([Q-M1f.578](Q-M1f.578-the-two-lists-and-a-closure-configure-the-blind.md)), or agrees and says nothing. The note
is placed at the name, printed on every command that loads the program, once a process, and moves no exit code,
answer or stdout.

**Why.** Such a declaration says nothing false, so the program answers; but it says nothing at all, and an author who
wrote it believed it filtered something.

**Held by.** `ein-cli/tests/mechanism_conflicts.rs`; `ein_infer::mechanisms::check`;
[`07-diagnostics.md`](../../ein/user/04-reference/07-diagnostics.md#226-a-programs-hypothesis-mechanisms-do-not-contradict-one-another) § 2.26.
