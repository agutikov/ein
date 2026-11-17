# Q-M1f.109 — A text's line break in the trace is a markdown hard break

`--trace` writes markdown. A line break in a text the program wrote, a rule's `:why` in its quote, a premise's
`:source`, a relation's statement or an owed instance, is written as a hard break: two spaces, a newline, and the line
after it led by its block's marker, `> ` in the quote and the item's indent in a list. The characters of the text are
kept; each line of it is a line of the rendered page.

**Why.** Written as it stands, a newline is a soft break, which a markdown reader joins into one line; the text's
line break is the writer's, as it is on the read-out
([Q-M1f.19](Q-M1f.19-a-texts-line-break-on-the-read-out-continues-at.md)).

**Held by.** `ein-cli/tests/natural_language_text.rs`: `the_trace_prints_the_rules_why_and_the_premises_source`;
[14-natural-language.md](../../ein/user/03-syntax-and-semantics/14-natural-language.md#3-a-fact-in-english-the-fallback-ladder) § 3.
