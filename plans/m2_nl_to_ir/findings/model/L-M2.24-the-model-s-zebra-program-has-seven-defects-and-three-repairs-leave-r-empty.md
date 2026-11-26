# L-M2.24 — The model's Zebra program has seven independent defects, and three repairs leave R empty

**Status:** open
**Found:** by [the analysis of P2.2 against its goals](../../p2.2_formalizer/report/ANALYSIS.md#3-why-the-zebra-one-shot-missed),
2026-11-26, on `60964047`; filed at the phase's close, 2026-11-27
**Owner:** [S2.2d.6](../../p2.2d_formalizer_corrected/s2.2d.6_run_again_and_nearest_miss.md), which mends the
program one defect at a time by hand, and [S2.4.3](../../p2.4_loop/s2.4.3_feedback_ladder.md), whose ladder reads
what that finds

## What

[L-M2.23](../../p2.2_formalizer/findings/model/L-M2.23-the-model-s-zebra-program-dies-of-two-faults-not-of-ten-houses.md),
closed by the phase report, read the one shot's Zebra program as dead of two faults, with a third, the query's one
variable, before the answer prints. **Read off the program, it has seven defects, each independent of the others**:

| | the defect | step | who sees it |
|---:|---|:---:|---|
| 1 | a description made a thing: `RedHouse` to `BlueHouse` and `MiddleHouse` are houses, eleven in all | 1 | the kernel's core, one fault at a time |
| 2 | four kinds with no member under the partition (`Smoker`, `Drinker`, `Owner`, `Color`) | 2 | the core |
| 3 | drinks, pets and cigarettes are one kind, `Thing`: fifteen members for five places | 1, 2 | only *Open: every alive node owes* |
| 4 | the three *next to* sentences written as `lives-in` and `smokes`, two over names that are nothing | 1 | nothing |
| 5 | `(solve :no-search)` beside the obligations | 3 | EinAf's check |
| 6 | the answer query asks one person both questions ([L-M2.21](L-M2.21-the-model-s-zebra-answer-query-asks-one-person-both-questions.md)) | 1 | nothing |
| 7 | the read-back selected over the empty kinds, and no `compose` | 3 | EinAf's check |

**With defects 1 and 2 removed and `(solve :no-search)` dropped, R is still empty**: `ein solve --single` on the
report's *less both* program less that line answers *Open — R is empty: every alive node owes*, 190 candidates, 171
saturations, root owing 28 instances. That it is the one kind `Thing` that leaves every node owing is the analysis'
reading of the program; the run shows only that three repairs are not enough.

## Evidence

[The analysis § 3](../../p2.2_formalizer/report/ANALYSIS.md#3-why-the-zebra-one-shot-missed): the table and the
run, on `from_text/runs/library/zebra/program.ein` and `report/out/ein/model-zebra-less-both.ein`, which
`report/make_report.sh` writes again.

## What it costs

The repair the loop inherits is not two faults and a variable. Of the seven the kernel speaks of two, one at a time,
EinAf's check of two more, and three have nothing that points at them. **The nearest miss the phase's risk list
asks for is not measured**: the least edit of the model's program that answers.
