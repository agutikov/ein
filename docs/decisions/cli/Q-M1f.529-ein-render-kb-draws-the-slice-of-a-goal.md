# Q-M1f.529 — `ein render kb` draws the slice of a goal

`ein render kb FILE --goal GOAL` draws **the slice of a goal**: the facts of the knowledge base the command draws,
root at its fixpoint or the state a saved file loads to, that answer `GOAL`, every fact a pattern of the goal matches
in a match whose `(absent …)` guards hold, the matched disjunct's alone for an `(or …)`, and no check, drawn with what
they point at ([Q-M1f.527](Q-M1f.527-a-slice-is-drawn-with-what-its-facts-point-at.md)). `GOAL` is written as a
`(query …)`'s `:goal` and read as the goal of one more query of the program, held to what that goal would be held to
([Q-M1f.119](../language/Q-M1f.119-a-goal-and-a-claim-ask-only-about-relations.md)): a macro's call expanded, a
relation the program has not refused, with the line the loader would print, exit 1, nothing drawn; a text that is
not one goal is refused the same way. `--query N` draws the slice of the goal of the program's `N`-th query, from 1,
its `:over` and `:expect` not read. The two are exclusive, the digraph is named `goal`, and `--since`, `--colour-by`
and `--fold` draw it as they draw the whole knowledge base.

The goal is matched against the knowledge base drawn and never against R: a query is about R and a slice about one
knowledge base. A state's slice is had by drawing the state's saved file, `ein render kb DIR/models/01.ein --query
1`.

**Why.** A slice asks of one knowledge base: `zebra2`'s root is not in R, and its query asked over R has no state to
answer from, while root holds facts a goal can ask about. An option asks without editing the program, and a saved
state is already a file the command draws; the program's own query is the slice most often wanted.

**Held by.** `ein_ir::read_goal`; `ein_infer::slice::answering` and its unit test
`the_facts_that_answer_a_goal_are_what_its_matches_matched`; `ein-cli/tests/render_doors.rs`:
`render_kb_draws_a_goals_slice` and `render_kb_refuses_a_goal_a_query_could_not_ask`. The page is
[rendering](../../ein/user/04-reference/12-rendering.md#the-kb--ein-render-kb).
