# Q-M1f.242 — A hypothesis declaration that filters nothing is a note

A declaration that steers the hypotheses and contradicts nothing, yet does nothing, is one `note:` line on stderr:

- `:hypothesis-relations` or `:no-hypothesis` beside a declared hrule, since no list filters an hrule's candidates;
- with no hrule, a `:no-hypothesis` name that `:hypothesis-relations` already leaves out, or that a `(__closed__ R)`
  already closes.

The note is placed at the list or the name, printed on every command that loads the program, once a process, and
moves no exit code, answer or stdout. A declaration that contradicts another is refused instead
([Q-M1f.244](Q-M1f.244-an-obligated-relation-is-not-closed-whatever.md)).

**Why.** Such a declaration says nothing false, so the program answers; but it says nothing at all, and an author who
wrote it believed it filtered something.

**Held by.** `ein-cli/tests/mechanism_conflicts.rs`: `a_dead_or_redundant_list_is_a_note_and_a_silent_pair_says_nothing`;
`ein_infer::mechanisms::check`; [`defined_behaviour.md`](../../kernel/defined_behaviour.md) § 4.27, with the wording of
each note.
