# Q-M2.40 — Is *the model reasons, and Ein checks its answer* an arm?

**Status:** open

**Raised 2026-11-26 by [the analysis of P2.2](../../p2.2_formalizer/report/ANALYSIS.md#103-what-the-model-lacked-and-what-ein-gives-today)**
(its proposals 19 and 21), on the user's question whether Ein interferes with a reasoning that was right; filed
at the phase's close, 2026-11-27.

**The question.** The user's two arrangements are the tool and the filter
([Q-M2.13](Q-M2.13-who-calls-the-kernel.md)). The one shot of the filter switches the model's reasoning off and
asks for a translation, and loses to that reasoning where the reasoning works. What the model alone lacked was a
verdict on its grid, and Ein gives one today: the clues by hand with twenty cells of a grid added as facts and
`(solve :no-search)` are judged in 0.04 s, and a wrong grid's core names the clue it breaks and the two cells. Is
there a third arrangement, an arm of its own: the model reasons, fills a table under a grammar, the clues are
written once, and the kernel answers whether the table satisfies them and whether it is the only one?

**The default that stands**: no third arm; the tool ([S2.4.6](../../p2.4_loop/s2.4.6_ein_as_a_tool.md)) and the
filter, as ruled.

**The alternatives.**

1. **No arm of its own** (the default): the tool is where a model's answer meets Ein, and what it calls Ein with
   is its own choice.
2. **An arm between B1 and B6**: the table and the two questions, for problems whose answer is an assignment.
3. **A level of the ladder, not an arm**: *your program prints X; you concluded Y*, and the clause-by-clause
   check, in the filter's loop ([S2.4.3](../../p2.4_loop/s2.4.3_feedback_ladder.md)).

**The guard under 2 and 3.** A model that bends its program until it prints what the model believes is the
milestone's most dangerous failure with a new cause. So the grid tests a program and is never part of the one that
answers; the answer is the program's, run without it; a fact with no sentence behind it is still a generated
assumption; and the arm is measured on texts whose gold is *undetermined* or *contradictory*.

**What bears on it.** The reading of each clue stays the model's under any of the three
([L-M2.24](../../findings/model/L-M2.24-the-model-s-zebra-program-has-seven-defects-and-three-repairs-leave-r-empty.md),
defect 4); [S2.2c.2](../../p2.2c_decoding_and_model_alone/s2.2c.2_the_model_alone.md), which finds the texts the
model fails alone on, where help can be measured at all; [S2.2c.3](../../p2.2c_decoding_and_model_alone/s2.2c.3_the_tool_probed.md),
the tool probed on them.
