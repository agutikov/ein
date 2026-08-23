# P2.6 — Ablations (Stage G)

**Estimate:** 3 weeks — 4 stages; one written to stage depth
([S2.6.4](s2.6.4_representation_ablations.md), because it carries the old
P2.5 link-grammar experiment whole), three as the paragraphs below, to be
written to depth when [S2.5.4](../p2.5_harness/s2.5.4_first_table.md)'s
table exists and says which of them matter.
**Depends on:** [P2.5](../p2.5_harness/README.md) — the records, the
metrics, the baseline conditions and the budget rule; every ablation is
more rows in that table's vocabulary.
**Blocks:** [P2.7](../p2.7_failure_scaling_generalization/README.md) — the
failure sample is drawn from these runs; the **Level C gate**.
**Research plan:** [`EinAf.md` § Stage G](../EinAf.md#stage-g--ablation-program),
G1–G9.

---

## This is where the architecture becomes an experiment

The plan's sentence, and the phase's. Up to here the milestone has built a
loop and shown a table; here it asks *which part of the loop does the work*,
and it does so by turning one knob at a time with everything else — model,
prompts, split, budget, seed — held by the record. The knobs are all
parameters the loop already has ([P2.4](../p2.4_loop/README.md): the level,
the strategy, the depth, the library the catalogue lists, the decomposition),
which is why an ablation is two configs and not two systems.

Every ablation is **pre-registered**: before it runs, the stage writes the
reading each outcome would have — *if G3 shows no gain from the core, the
F5 renderer is not the contribution* — so that the table is read by the
rule written before it, the way the old S2.5.2 did with its `> 0.03` /
`< 0.0` / in-between rule for link-grammar. That rule is kept as the
template.

## The nine, grouped into four stages

**S2.6.1 — Retry control and verdict (G1, G2).** *Does improvement come
merely from additional inference?* One-shot vs generic retry vs Ein-guided
retry at equal budget (G1) — which is B2 / B3 / B4 of the main table with
the budget column made the x-axis; then *generic failure* vs *SAT / UNSAT /
AMBIGUOUS* (G2), F0 against F2. These two are the cheapest ablations and the
ones the whole claim rests on: if F0 matches F2, the kernel is a retry
trigger and nothing in F3–F8 can rescue that. Pre-registered reading: the
claim survives G1 only if B4 > B3 at matched budget, on `val`, on at least
two families, with the two conditional probabilities moving the same way.

**S2.6.2 — The content of the feedback (G3, G4, G5, G6).** Four
comparisons, each a pair of levels or a pair of renderings of one level:
UNSAT vs UNSAT + core (G3) — measured as *repair success on `unsat`
instances*, where the core names the source sentences; diagnostic without
vs with provenance (G4) — F7 off and on over F5 / F6; *"4 models remain"*
vs *two representatives and their relevant difference* (G5) — F3 against
F4 on `ambiguous` instances, with the hallucination rate reported beside
the repair rate because F4 is the level most able to leak an answer; and
feedback size (G6) — minimal / medium / the complete derivation trace,
because *more information may make the model worse* and the plan says so.
The reading for each is written as a sign and a threshold before the run.

**S2.6.3 — Iteration depth (G7).** Repair iterations 0, 1, 2, 3, … at F8
and at F2; the curve of marginal improvement against cost, per family.
Written as one stage because the plan's instruction is one line — *plot
marginal improvement against cost* — and the transcript already has every
point: a run at depth *n* contains its depth-*k* prefix for every *k < n*,
so the curve costs one run per instance, not one per depth.

**[S2.6.4](s2.6.4_representation_ablations.md) — Representation and library
(G8, G9, and the link-grammar arm).** Direct Ein generation vs
*NL → structured semantic representation → Ein* (G8) — which is B1's
decomposition question from [S2.2.1](../p2.2_formalizer/s2.2.1_contract.md),
and the user's four oracle ablations from
[F13](../../followups/f13_puzzles_beyond_zebra/ideas.md) (*NL vs IR × known
vs unknown theory*) made conditions; the old P2.5 link-grammar A/B, which
is the same question about the *input* side — does a dependency parse in the
prompt help — run under the same rule and with the same submodule note; and
no stdlib / minimal / full (G9), which answers *how much of the performance
is reusable human-written theory* and is the one ablation that also reads
on [F8](../../followups/f8_FCA_RCA_odis_tptp/ideas.md)'s `C(n)` question.

## Acceptance

- Nine ablations, each a pre-registered reading, a pair (or a family) of
  configs, a table from records, and the reading applied — in that order,
  with the pre-registration committed before the records exist.
- G1 and G2 are reported first and in full, including the case where they
  refute the claim: the milestone's Stage O admits a negative result, and
  this phase is where one would appear.
- G5 reports the hallucination rate beside the repair rate; G6 reports
  tokens beside accuracy; G9 names the stdlib modules in each condition by
  manifest.
- [Q11](../open_questions.md#q11--link-grammar-value) is decided by the
  old rule, and the code state matches the verdict.

## Risks

- **Nine ablations on one `val` split is multiple comparison.** The stage
  that writes the readings also writes how many comparisons are being made
  and what that does to the thresholds; the `test` split is touched once,
  in [P2.10](../p2.10_result_artifact_demo/README.md).
- **Variance.** Local models at temperature 0 are deterministic per seed and
  not across seeds; every ablation cell is three seeds and reports the
  spread, which triples the cost and is the price of a claim.
- **The interesting negative.** If G6 shows the full trace *hurts*, the
  temptation is to fix the renderer and re-run. The rule: the renderer that
  ran is the one reported; a better renderer is a new experiment id and a
  new row, never a replacement.

## Connections

- [`EinAf.md` § Stage G](../EinAf.md#stage-g--ablation-program).
- [P2.4](../p2.4_loop/README.md) — the knobs; [P2.5](../p2.5_harness/README.md) —
  the table these rows join; [P2.7](../p2.7_failure_scaling_generalization/README.md) —
  where the failures these runs produce are classified.
- [F9](../../followups/f9_e_catalog.md) — the repo's precedent for a
  ledger of measured-and-declined mechanisms; this phase's tables are the
  same discipline applied to feedback.
