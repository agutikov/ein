# Q-M1f.19 — A text's line break on the read-out continues at the text's column

Where a row's value is a text the program wrote, a `:goal-text` filled for a class's row or an obligation's `:why`
under an `owed` or `owing` row, every line break it holds (`\r\n`, `\n`, `\r`) starts a new line at the column the
text's first character is printed at, and each such line is one of stdout's 100. The surfaces that quote a text,
`ein saturate --dump`, the program a state file re-writes and the `source` row, write the break escaped.

**Why.** The text is natural language the program wrote for a reader, and its line break is the writer's; the
read-out's job is to keep it inside its row, not to rewrite it.

**Held by.** `ein-cli/tests/natural_language_text.rs`: `the_read_out_prints_the_goal_text_as_written`, each line of
the sentence after a break at its column. The rule: [read-out.md](../../ein/user/reference/read-out.md#1-how-a-row-reads) § 1. The
trace's markdown takes the same text as a hard break
([Q-M1f.109](Q-M1f.109-a-texts-line-break-in-the-trace-is-a-markdown.md)).
