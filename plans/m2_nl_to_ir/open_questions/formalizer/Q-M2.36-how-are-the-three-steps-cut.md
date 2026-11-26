# Q-M2.36 — How are the three steps cut?

**Status:** open

**Raised 2026-11-26 by the user**, written up in
[the analysis of P2.2, § 11](../../p2.2_formalizer/report/ANALYSIS.md#11-the-three-steps-against-the-one-call-and-how-the-steps-are-cut),
and filed at the phase's close, 2026-11-27. The user's words:

> analyze 3-steps proposal results, compared to one shot; Does it make sense of partial ontology and partial
> theory, or better strictly split - 1 - facts and queries only, 2 - ontology only, 3 - theory only; Was it
> investigated or not? If not - add proposal;

**The question.** The steps are the user's notes: the facts and a *partial* ontology, then the complete ontology
and a *partial* theory, then the complete theory. On S2.2.4's runs the two partial parts are where the slips
between steps are ([L-M2.25](../../findings/einaf/L-M2.25-the-steps-partial-parts-are-written-twice-never-revised-and-a-step-early.md)),
and against the one call the steps gave no better answers: their measured gain is the grammars a step boundary
allows. A strict cut was not investigated. Which cut is the formalizer's?

| the cut | step 1 | step 2 | step 3 |
|---|---|---|---|
| today's | statements, the answer query, a partial ontology | the ontology whole, imports, selection | the rest of the selection, rules, the mechanism |
| strict (the user's) | statements and the queries | the ontology: declarations, memberships, kinds | the theory: imports, selection, rules, the mechanism |
| two steps | statements and the queries | the ontology and the theory, one call | |
| the one call | everything | | |

**The default that stands**: today's cut, until the measurement and the user's word.

**What is known with no model.** Four of the steps by hand, cut strictly, load at every cut but `youngest`'s first
(its goal names a relation no statement uses), and answer the same; the kernel has nothing to say of facts alone
or of an ontology alone ([E-114](../../evaluation/hypotheses/E-114-a-partial-program-s-h-is-set-by-its-relations-count-of-places.md)),
so in a strict cut it is first worth asking when the theory is in
([the analysis § 11.4](../../p2.2_formalizer/report/ANALYSIS.md#114-the-strict-split-by-hand)).

**What decides it.** [S2.2d.2](../../p2.2d_formalizer_corrected/s2.2d.2_the_cut.md) measures the four cuts on
the sixteen texts and, in the oracle form, each step given the hand's earlier steps; its report is what the user
rules on.

**What bears on it.** [P2.2b](../../p2.2b_guided_thinking/README.md), whose questions are dealt to the steps as
they are cut today; [Q-M2.32](Q-M2.32-which-mode-does-baseline-b2-run-with.md), B2's mode;
[S2.6.4](../../p2.6_ablations/s2.6.4_representation_ablations.md)'s G8, which puts a model in front of the steps
and cuts them no other way; what a later step may change of an earlier one, decided by default in S2.2.1 (*adds,
does not rewrite*).
