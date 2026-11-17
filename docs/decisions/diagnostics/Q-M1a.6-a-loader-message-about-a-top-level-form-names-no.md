# Q-M1a.6 — A loader message about a top-level form names no position

A form written at the top level of a file is built without a source position, so a loader message about one
ends `at None`: `(rule r) missing :match or :assert at None`, `duplicate rule/hrule name 'g' at None`. A message
about a form inside another, the call of a macro in a rule's body among them, names the file, the line and the
column, `(rule x): macro m/2 invoked with 1 args at a.ein:6:20`, as a parse error does.

**Why.** The position is a usability gap, not a defect of any answer: the message names the form, and the form
is written once in its file. Giving every top-level form its position changes the last words of every such
message at once, and with them every banked message of a fixture the loader refuses; it waits for the change
that wants positions on those messages, so that the move is made once and on purpose rather than as noise.

**Held by.** `ein_ir::loc_repr`, which prints `None` for a form with no position, and its unit test
`a_loc_renders_as_file_line_col_or_none` (`ein-ir/src/ast.rs`); `ein-ir/tests/ir_semantics.rs`:
`a_rule_needs_params_and_a_query_needs_a_kw_pair`; `ein-cli/tests/expect_cli.rs`:
`the_load_negatives_are_refused_with_their_banked_message`, every `examples/broken/load/` fixture against its
`.expected`. The page:
[`07-diagnostics.md` § 1.5](../../ein/user/04-reference/07-diagnostics.md#15-loader-messages-about-a-top-level-form-end-in-at-none).
