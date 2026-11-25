# Q-M2.3 — What is the unit of faithfulness without a gold theory?

**Status:** open

**Arrived 2026-08-23 with the reshape.** On a synthetic instance the
generator wrote the canonical theory, so faithfulness is arithmetic:
constraint precision and recall against it (H2), an invented constraint is a
false positive, a dropped one a false negative. On an external instance —
BBH, CLUTRR, FOLIO — there is a gold *answer* and no gold theory, and a
theory that reaches the gold answer by an invented constraint is exactly the
E3 failure the plan warns about and exactly what answer accuracy cannot see.

**The candidates:** (a) the B2 provenance categories make it partly
mechanical — a fact whose `:source` span does not exist in the text, or a
rule whose category is *generated assumption*, is flagged regardless of the
answer; (b) a human-judged sample, sized in [P2.7](../../p2.7_failure_scaling_generalization/README.md);
(c) an LLM-as-judge with the source and the theory, validated against (b)
on the sample before it is trusted on the rest; (d) back-translation of the
theory to NL and an NLI check against the source, which the autoformalization
literature uses ([F16](../../../followups/f16_autoformalization/ideas.md) cites
Lean Workbook doing exactly this). **[S2.4.4](../../p2.4_loop/s2.4.4_faithfulness.md)
decides**, and the recommendation is (a) always, (b) as the ground truth for
the sample, and (c) or (d) only with their agreement to (b) reported.
