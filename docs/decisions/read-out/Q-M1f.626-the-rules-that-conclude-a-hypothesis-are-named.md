# Q-M1f.626 — The rules that conclude a hypothesis are named `hypothesis-rules` on every surface

Where a surface names the writers of $H$, the rules that conclude `(open F)`, with the program's own `(open F)`
facts, are named for what they are: the mode `hypothesis-rules`, which `ein show hyps`' note says, *H, from
\`hypothesis-rules\`: N candidates*, and `ein render h-space`'s `H` row, *· from \`hypothesis-rules\`*; the keys
`hypothesis_rules` and `hypothesis_plans` of the mechanisms and `a_hypothesis_rule_proposes_it` of a relation in
`h-space.json`; the `load` event's count `hypothesis_rules`; and the reason a relation is outside the universe,
`not-proposed`, which the read-out says as *no hypothesis rule proposes it*. The `rung` event keeps its name, and no
surface says `hrule`.

**Why.** There is no `hrule` form: a hypothesis rule is a rule that concludes `(open F)`
([Q-M1f.567](../hypotheses/Q-M1f.567-a-hypothesis-rule-is-a-rule-that-concludes-open.md)), so the words that named
the form name the rules, one word on every surface.

**Held by.** `ein_infer::oblgen::Mode::as_str` and `ein_infer::scope`'s `Why::key`;
`ein-infer/tests/obligation_rung.rs`: `the_order_and_the_modes_keep_their_names`; `ein-cli/tests/show_hyps.rs`:
`stdout_is_the_facts_and_stderr_one_note`; `ein-cli/tests/read_out.rs`:
`the_universes_scope_is_one_sentence_on_every_surface`; `ein-cli/tests/test_cli.rs`:
`the_report_row_carries_what_h_leaves_out_where_the_text_prints_it`; `ein-cli/tests/h_space_view.rs`. The pages:
[hypotheses](../../ein/user/03-syntax-and-semantics/09-hypotheses.md),
[hypotheses and the answer](../../ein/ein.rs/08-reasoning-in-code/03-hypotheses-and-the-answer.md).
