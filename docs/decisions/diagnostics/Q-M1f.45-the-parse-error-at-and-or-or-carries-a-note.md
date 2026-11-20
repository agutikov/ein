# Q-M1f.45 — The parse error at `(and)` or `(or)` carries a note naming the rule and the clause

`and` takes one premise or more and `or` one disjunct or more, so `(and)` and `(or)` are parse errors at their `)`.
Under the error a note names where the form is written and says what the form takes:

- `note: rule 'r''s :match writes (and), the empty conjunction, whose one spelling is () — and takes one premise or
  more`;
- `note: rule 'r''s :match writes (or) — or takes one disjunct or more`.

In an `:assert` the note speaks of the empty conclusion, whose one spelling is `()` too. The note names a
query's `:goal` or `:expect`, or a macro's template the same way where the form is written there.

**Why.** `()` is the one spelling of the empty conjunction, a `:match` that is true and fires once, and a second
spelling would be a second way to say it. The empty disjunction has no meaning in Ein, and the note reads none into
it.

**Held by.** `ein-ir/tests/grammar_pages.rs`: `every_row_of_the_grammars_table_is_refused_at_the_parse`, whose rows
write `(and)` and `(or)` in each clause, every outcome with its note banked in
`ein-ir/tests/golden/grammar_table.txt`.
