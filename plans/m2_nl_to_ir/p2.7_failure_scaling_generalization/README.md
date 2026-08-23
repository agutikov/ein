# P2.7 — Failure analysis, scaling, generalization (Stages I, J, K)

**Estimate:** 3 weeks — 3 stages, as the paragraphs below; phase README
only, to be written to stage depth once [P2.6](../p2.6_ablations/README.md)
has produced the runs the sample is drawn from.
**Depends on:** [P2.5](../p2.5_harness/README.md) and
[P2.6](../p2.6_ablations/README.md) — the records; [P2.3](../p2.3_benchmark/README.md) —
the knobs, which are this phase's x-axes, and the generators, which produce
K1–K5's sets with different arguments.
**Blocks:** the **Level C gate** — the plan's Level C is *ablations and
failure analysis*, and the gate closes here;
[P2.10](../p2.10_result_artifact_demo/README.md) — the claim is shaped by
what this phase finds.
**Research plan:** [`EinAf.md` § Stage I](../EinAf.md#stage-i--failure-taxonomy),
[§ Stage J](../EinAf.md#stage-j--scaling-experiments),
[§ Stage K](../EinAf.md#stage-k--generalization-experiments).

---

## Three stages of the plan, one phase — and why

Stages I, J and K share a question the aggregate tables cannot answer:
**where does the system actually fail**, and is the limit the language side
or the search side. The failure taxonomy says *what kind*; the scaling
curves say *at what size*; the generalization sets say *under what change
of surface*. They are one phase because they read the same records and
because each one's finding changes what the other two look for — a
taxonomy dominated by *direction reversal* makes the paraphrase set (K1) the
next experiment, and a scaling curve where the solver degrades first makes
the failure sample at the largest size the one to read.

## The three stages

**S2.7.1 — The failure taxonomy (I).** A sample of failures of stated
size — stated *before* it is drawn, from the B5 and ablation runs on `val`,
stratified by family and by verdict class — read by hand and classified
under the plan's taxonomy: lexical interpretation, entity extraction,
relation extraction, direction reversal, quantifier error, cardinality
error, missing constraint, invented constraint, incorrect generic rule, type
construction error, query construction error, unsupported semantics, solver
limitation, repair regression, repair hallucination, answer extraction. Each
then filed under one of five *sides*: **neural**, **representation** (ein-lang
cannot say it — the finding [M1d](../../m1d_satisfiability/README.md) and
[F1b](../../followups/f1b_logical_formulation.md) want), **symbolic** (the
kernel was wrong or ran out — a finding for the kernel milestones, never
fixed here), **interface** (the feedback object or a renderer misled), and
**benchmark ambiguity** (the gold was wrong — the generator or the adapter
is corrected and the instance re-admitted, with the correction recorded).
The taxonomy itself is allowed to grow, and a category added is recorded
with the instance that forced it. Output: the table, the sample with its
labels in a record, and a one-paragraph reading per side.

**S2.7.2 — Scaling (J).** The generators at increasing entities, facts,
rules, relation arity, constraint density, model count, proof depth and
hypothesis depth — the knobs of [S2.3.2](../p2.3_benchmark/s2.3.2_generators.md)
swept one at a time — and BBH's own 3 / 5 / 7 ladder. Three curves per knob,
measured separately: **formalizer degradation** (B2's H2 metrics against
the canonical theory — the kernel is not in the loop), **solver degradation**
(the canonical program under `ein solve -e` — the formalizer is not in the
loop; enterings, wall, `exhausted`), and **combined** (B5). Accuracy and
latency against difficulty, per family. The boundary the plan asks about —
*is the limiting factor language understanding or symbolic search?* — is
where the first two curves cross, if they do, and the phase reports the
crossing or its absence per family. The solver curve has a known shape
already on one family: [M1d P1d.10](../../m1d_satisfiability/p1d.10_exhaustive_search/README.md)'s
table, where `zebra2-minus-15` finds all 32 models by depth 3 and cannot
finish proving there are no more.

**S2.7.3 — Generalization (K1–K5).** Five sets, each the benchmark's
generators with one argument changed, each run at B2 and B5 and reported
as invariance — the drop from the base set: **K1** paraphrase (three or
more surface templates per statement type, the minimum S2.3.2 was told to
write, plus a model-paraphrased set with its own faithfulness check);
**K2** structural (prompts developed on 3–5 entities, tested on 8–15 — the
frozen split's size bins, [S2.3.4](../p2.3_benchmark/s2.3.4_splits.md),
not a separate set);
**K3** cross-domain (developed on ordering and attribute matching, tested
on the six other families — the families the stdlib has no theory for are
the hard half of this set, by design); **K4** novel vocabulary (`left-of`
becomes `zorps`, with its semantics stated in the text — what separates
relational reasoning from lexical priors, and the one set on which a
published BBH number cannot be memory); **K5** adversarial irrelevant
information (irrelevant, redundant, distracting, superficially similar
statements inserted at a stated rate).

## Acceptance

- A failure sample of pre-stated size, labelled, in a record; the five-side
  split with a reading per side; every *benchmark ambiguity* label traced to
  a generator or adapter fix.
- Scaling curves for every knob in [S2.3.2](../p2.3_benchmark/s2.3.2_generators.md)'s
  metadata, three curves each, per family; the crossing reported or its
  absence.
- K1–K5, each as a drop from base at B2 and at B5, with the plan's
  development / test separation honoured (K2, K3).
- The Level C gate: with [P2.5](../p2.5_harness/README.md)'s table and
  [P2.6](../p2.6_ablations/README.md)'s ablations, the milestone **supports
  an empirical claim** — and this phase's finding is written as the
  sentence the claim will be, in the shape of § Stage O, before
  [P2.10](../p2.10_result_artifact_demo/README.md) tests it on `test`.

## Risks

- **Reading failures by hand is the slowest thing in the milestone** and the
  most valuable; the sample size is set by what a week can read, stated,
  and not enlarged to reach significance after the fact.
- **A kernel finding is a temptation to fix the kernel.** A *symbolic*-side
  failure goes to [M1d](../../m1d_satisfiability/README.md) or a followup as
  a finding with its instance; [Q-M2.1](../open_questions.md#q-m21--when-is-the-kernel-frozen)
  says what happens if it is fixed mid-milestone.
- **K4 may be where the local model collapses.** That is a result about
  lexical priors and is reported as one.

## Connections

- [`EinAf.md` § Stage I](../EinAf.md#stage-i--failure-taxonomy),
  [§ Stage J](../EinAf.md#stage-j--scaling-experiments),
  [§ Stage K](../EinAf.md#stage-k--generalization-experiments).
- [F13](../../followups/f13_puzzles_beyond_zebra/ideas.md) — *`logical_deduction_3/5/7`
  is almost a ready-made controlled experiment on generalization / scaling*;
  [M1d P1d.10](../../m1d_satisfiability/p1d.10_exhaustive_search/README.md) —
  the solver's curve on under-determined instances, already measured once.
- [F10 `findings.md`](../../followups/f10_m1_refactor_tail/README.md) — the
  repo's precedent for a review register kept whole; the failure sample is
  the same kind of artefact.
