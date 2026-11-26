# Q-M2.39 — What is the minimal goal's instance, if the model alone solves the Zebra text?

**Status:** open

**Raised 2026-11-26 by [the analysis of P2.2](../../p2.2_formalizer/report/ANALYSIS.md#104-further-steps)**
(its proposal 24), on the user's question whether Ein interferes with a reasoning that was right; filed at the
phase's close, 2026-11-27.

**The question.** The minimal goal ([Q-M2.17](Q-M2.17-what-is-the-milestones-minimal-goal.md)) is that the model,
*which does not solve the Zebra puzzle by reasoning in text*, solves it with Ein. The model alone reached the
right grid in its thinking and then looped under greedy decoding; it named the puzzle
([L-M2.29](../../findings/model/L-M2.29-the-model-alone-solves-the-zebra-puzzle-in-its-thinking-and-names-it.md)).
If under its own sampler it prints the answer, the goal as worded has no other side on this text. On what text is
the goal then read?

**The default that stands**: the Zebra text, as ruled, the model alone recorded with its context and budget
stated.

**What decides it.** [S2.2c.2](../../p2.2c_decoding_and_model_alone/s2.2c.2_the_model_alone.md) runs the model
alone on the Zebra text, its two edits, the same puzzle with every noun renamed and the sentences reordered, and a
puzzle of the same size it cannot have seen; its report says on which the model fails alone.

**The proposal, once that is known**: the goal is read on a puzzle the model does not solve alone, chosen by that
stage's record, with the Zebra text kept beside it; this keeps the goal's sense where its premise failed.

**What bears on it.** [S2.5.2](../../p2.5_harness/s2.5.2_baselines.md), whose B1 is the model reasoning in text,
under the model's own chain by default; [Q-M2.27](../model/Q-M2.27-do-the-phase-s-thinking-runs-stay-greedy-when-greedy-thinking-loops.md);
the benchmark's attribute family, which would hold the chosen puzzle
([S2.3.1](../../p2.3_benchmark/s2.3.1_families_and_seed_set.md)).
