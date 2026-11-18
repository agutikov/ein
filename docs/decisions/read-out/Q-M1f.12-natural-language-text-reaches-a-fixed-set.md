# Q-M1f.12 — Natural-language text reaches a fixed set of surfaces, byte for byte

A program's text, written after `:why`, `:source` and `:goal-text`, reaches these surfaces, each holding the
characters the program wrote:

- the read-out and `--dump-answer`'s `answer.txt`: a query's `:goal-text`, filled;
- `--trace`: a rule's `:why` filled at its firing, a premise's `:source`, and a declaration's `:why`, read on its
  own arrows, where a rule reads the declaration;
- `ein saturate --dump`: each `:source` and `:goal-text`, quoted;
- a `--dump-answer` state file: all three, in the program and the modules it carries as written;
- `ein render program`: a `:goal-text`, inside a DOT label;
- a `.einb` container: all three.

`ein render rules`, `kb`, `constraints` and `lattice`, `--events` and the JSON summary carry none.

**Why.** Each surface that carries a text is one where a reader meets the program's own words; a text added to a
schema or a golden elsewhere is a new surface, asked for on its own.

**Held by.** `ein-cli/tests/natural_language_text.rs`: `the_read_out_prints_the_goal_text_as_written`,
`the_trace_prints_the_rules_why_and_the_premises_source`, `a_declaration_reads_its_own_arrows_and_a_fact_none`,
`the_dump_quotes_the_source_and_the_goal_text`,
`a_state_file_carries_every_text_as_written`, `the_program_view_draws_the_goal_text`,
`a_container_holds_the_texts`. The texts and where each renders:
[14-natural-language.md](../../ein/user/03-syntax-and-semantics/14-natural-language.md).
