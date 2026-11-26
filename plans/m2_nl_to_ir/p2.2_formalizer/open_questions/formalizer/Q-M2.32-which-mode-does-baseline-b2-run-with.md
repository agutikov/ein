# Q-M2.32 — Which mode does baseline B2 run with: the three steps, or the whole program in one call?

**Status:** open

**Raised 2026-11-26 by [S2.2.4](../../s2.2.4_passes.md)**, whose T2.2.4.9 decides it as a default: *"the steps, unless
this stage's runs show the one call no worse on both materials, in which case the simpler is taken and the ledger
says so"*.

**The question.** Baseline B2 is the formalizer's one shot unchanged ([P2.5](../../../p2.5_harness/README.md)). The
formalizer has two modes: the three steps, each a call under its step's grammar, each step's program judged at root;
and the whole program in one call under the whole grammar's root. Which does B2 run with?

**The default taken: the steps.** On S2.2.4's runs ([its report § 8](../../steps/README.md#8-the-one-call-against-the-steps-and-b2s-mode)),
both under a grammar, qwen3.5:4b, thinking off, greedy:

- **the Zebra puzzle**: the steps give a program (104 forms, the right theory selected and spelled right) that is
  dead at root, its core naming the sentence the model misread; the one call repeats a form from its ninth line to
  the budget and gives none. **The one call is worse.**
- **the thirteen word texts**: the steps give no right answer alone (two the right count among others, two a wrong
  one); the one call one right answer (`kinds_full`), one among others, two wrong. **The one call is no worse.**
- **what the check finds**: 34 findings in the steps' complete programs, 111 in the one call's; no `:source` that is
  no sentence under the steps, 16 under the one call.

Worse on one material, so the steps, by the stage's rule.

**The alternatives.** The one call, which costs a third of the calls and a fifth of the generation (16 calls and
376 s against 49 and 1 745 s) and did as well on the word problems; or the mode chosen by the family, the steps for an
attribute puzzle and the one call for a word problem, which the benchmark's families would each need a run to set.

**What bears on it.** The comparison on the benchmark is G8's ([P2.6](../../../p2.6_ablations/README.md)); these are
sixteen texts, one greedy run each, and the Zebra puzzle is one of them.
