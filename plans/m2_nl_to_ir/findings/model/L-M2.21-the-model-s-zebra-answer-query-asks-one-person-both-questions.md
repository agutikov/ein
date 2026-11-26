# L-M2.21 — The model's Zebra answer query asks one person both questions

**Status:** open
**Found:** by S2.2.5, 2026-11-26, drawing the model's Zebra program (`ein render program`)
**Owner:** the loop, [P2.4](../../p2.4_loop/README.md), and the contract's check, whose sixth finding reads the
answer query
**Since P2.2's close (2026-11-27):** [Q-M2.35](../../open_questions/formalizer/Q-M2.35-is-there-an-answer-query-for-each-question-of-the-text.md), [S2.2d.1](../../p2.2d_formalizer_corrected/s2.2d.1_a_query_a_question.md): its cause is the contract's one answer query

## What

**Step 1 writes the two questions of the Zebra puzzle as one goal over one variable**: `(query :goal (and (drinks
?who Water) (owns ?who Zebra)) :goal-text "{?who} drinks water. {?who} owns the zebra.")`, on the puzzle's text and on
both its edits (`zebra`, `zebra-minus-15`, `zebra-contradiction`), as S2.2.4's step 1 wrote it before. The query asks
for one person who drinks water and owns the zebra; in the puzzle's one solution they are two, so a program right in
every other form would answer with no row, *The statements do not determine the answer.* The prompt's example, three
stalls in a row, asks its two questions with two variables (`?who_bread`, `?who_red`), and so do the steps by hand.
No finding of the check sees it: every reference is bound, the template writes no name.

## Evidence

[`runs/library/zebra/program.ein`](../../p2.2_formalizer/from_text/runs/library/zebra/program.ein) lines 27 to 29, and the same lines of
`zebra-minus-15` and `zebra-contradiction`; the query drawn in [`zebra.query.svg`](../../p2.2_formalizer/from_text/zebra.query.svg).

## What it costs

On these runs nothing: each program is dead at step 2 before its query is read. On a run that gets the grid right it
is the difference between the answer and the undetermined text, and the feedback ladder would have to say that the
query asks one value for two questions, which the kernel's *no row* does not.
