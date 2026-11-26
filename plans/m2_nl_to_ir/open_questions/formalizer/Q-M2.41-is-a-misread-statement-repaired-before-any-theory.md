# Q-M2.41 — Is a misread statement repaired before any theory?

**Status:** open

**Raised 2026-11-26 by [the analysis of P2.2](../../p2.2_formalizer/report/ANALYSIS.md#96-the-loops-first-rung-and-the-nearest-miss)**
(its proposal 11); filed at the phase's close, 2026-11-27.

**The question.** The user's principle for what a repair changes first is *the theory before the facts*: the
facts are *"as close to original text as possible and changed less then more variable rules"*
([P2.4 § The theory before the facts](../../p2.4_loop/README.md#the-theory-before-the-facts)). On the one shot's
Zebra run the facts are what is wrong: five statements whose relation is not their sentence's words
(`(lives-in Norwegian BlueHouse)` for *lives next to the blue house*), and a theory repaired around them cannot
answer. Does the principle take a clause: a statement whose relation is not made of its sentence's words is
repaired before any theory?

**The default that stands**: the principle as stated; a step-1 fact is changed only where the feedback names it
and no change of the ontology or the theory answers it (P2.4's default 18).

**The proposal**: the clause, once the check can name such a statement mechanically
([S2.2d.4](../../p2.2d_formalizer_corrected/s2.2d.4_check_findings.md): *`lives-in` is not in the words of this
sentence*).

**What bears on it.** Whether a step is shown its own findings before the next step, which is planned as an arm
of [P2.6](../../p2.6_ablations/README.md) (P2.2's default 39) and is the loop's first rung;
[S2.4.2](../../p2.4_loop/s2.4.2_repair.md), whose prompt carries the principle;
[S2.4.4](../../p2.4_loop/s2.4.4_faithfulness.md)'s `rewritten_fact`, which counts what a repair did to step 1's
facts either way.
