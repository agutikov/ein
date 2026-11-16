# Q-M1a.3 — A parse error names its position and its token, then its notes and repairs

A file the parser refuses reports every error recovery reaches, in file order, one line each: `<file>:<line>:<col>:
unexpected X`, where X is the token as written and, where the token has a name, the name after it (`a name`, `a
variable`, `a keyword this form does not take`). Under each error come its notes, where a decision says more than
the token does, then at most three ranked repairs, ``repair: Insert `)` ``, and `and n more repairs`. No source
line is quoted. Positions are 1-based and columns counted in characters; an error stands at the start of the token
the parse could not take, and the end of input at the position past the last character. A report cut at the
recovery budget ends with a line saying where it stopped. No repaired program is loaded: every command exits 1
(`ein test`: 2).

**Why.** A line per error, placed at the token, is read the same by a person and by a tool, and every error
recovery reaches is a round of edit-and-retry saved. A repair says what the parser would have taken there, which
the token alone does not.

**Held by.** `ein-ir/tests/grammar_decisions.rs`: `the_parse_negative_fixtures_reproduce_their_message`, against
the `.expected` file beside each `examples/broken/*.ein`; `ein-ir/tests/generated_parser.rs`:
`a_report_is_the_same_twice`, `a_report_cut_at_the_budget_says_so`, `the_end_of_input_is_where_the_text_ends`. The
budget is `ein_ir::grammar::RECOVERY_STEP_BUDGET`.
