# Q-M2.38 — Is a description abbreviated to its word, or written as a term of its own?

**Status:** open

**Raised 2026-11-26 by S2.2.0a**, which left *which axis of its table weighs most* to the user
([the phase's default 1](../../p2.2_formalizer/README.md#defaults-taken-at-the-re-plan)); given its evidence by
[the analysis of P2.2, proposal 10](../../p2.2_formalizer/report/ANALYSIS.md#94-what-the-library-and-the-kernel-owe),
and filed at the phase's close, 2026-11-27.

**The question.** The instruction to the model is *in the text's own words*. The target shape,
[`examples/zebra3.1.ein`](../../../../examples/zebra3.1.ein), writes *the green house* as `Green` and *the Old Gold
smoker* as `Old_Gold`: an abbreviation, not the text's words. The model's `GreenHouse` is the literal reading, and
it made six houses more ([L-M2.24](../../findings/model/L-M2.24-the-model-s-zebra-program-has-seven-defects-and-three-repairs-leave-r-empty.md),
defect 1). Which is the target shape?

**The default that stands**: `zebra3.1.ein`, the abbreviation.

**The alternatives.**

1. **Keep `zebra3.1` and state the abbreviation as the rule it is**, in the step-1 template with its
   counter-example (*the red house* is one of the five houses, described by its colour: `Red`, a `Color`, never
   `RedHouse`). [S2.2d.6](../../p2.2d_formalizer_corrected/s2.2d.6_run_again_and_nearest_miss.md) writes it under
   the default.
2. **Make the literal reading the right one**: a description a term of its own, `zebra3.2`'s `(the Green House)`,
   with its six rules moved into the library, so that step 1 is translation and what a description names is found
   by the theory. Closer to the notes (*facts as close to the original text as possible*); today six rules of the
   program's own, and [E-106](../../evaluation/answer/E-106-the-static-check-reads-a-relation-not-a-shape.md) flags
   its `same` form where it would be most literal.

**What bears on it.** S2.2.0a's table of the six encodings
([the analysis of the formulations](../../p2.2_formalizer/zebra_formulations/README.md)): what the literal shapes
cost the kernel ([E-111](../../evaluation/bounds/E-111-the-answer-in-the-statements-own-words-costs-the-kernel.md)),
what a core names in each ([E-109](../../evaluation/natural-language/E-109-a-sentence-written-as-a-rule-s-activating-fact-is-in-no.md));
[Q-M2.23](../kernel/Q-M2.23-does-the-library-grow-a-reading-of-a-literal-vocabulary.md), which the second
alternative reopens; and the benchmark, whose attribute programs follow the formalizer's target
([S2.3.1](../../p2.3_benchmark/s2.3.1_families_and_seed_set.md)).
