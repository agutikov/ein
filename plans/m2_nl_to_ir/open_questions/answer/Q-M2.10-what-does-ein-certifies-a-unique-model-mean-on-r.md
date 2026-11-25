# Q-M2.10 — What does *Ein certifies a unique model* mean on R?

**Status:** open

**Raised 2026-11-25 by the analysis** ([§ 2](../../analysis/README.md#2-what-the-kernel-says-today-where-the-plan-expects-the-walk),
[§ 5.4](../../analysis/README.md#5-what-the-standing-rules-now-ask-that-the-plan-does-not-answer);
[`audit_p2.4.md`](../../analysis/audit_p2.4.md), [`audit_p2.5_p2.6.md`](../../analysis/audit_p2.5_p2.6.md) B.4). It defines the
milestone's two central numbers, `P(correct | certified)` and `P(faithful | certified)`.

**The user, 2026-11-25:** *"it depends on problem shape, lets make experiments and then decide"*. So it is
**open, and the user decides it after experiments**; the recommendation this entry carried is withdrawn.

The plan writes it `k = 1 ∧ exhausted`. Neither field exists. On R there are three things it could be:

| | the question asked | what it says of a program that states no obligation |
|---|---|---|
| **(a) one state in R** | `ein solve --single` answers `Solution`, decided (7 saturations on `zebra2`) | `Ambiguity`: ∅ and every alive partial commitment are in R (`features/11_expect_ambiguity.ein`: two full seatings, `k_s = 7`) |
| **(b) one maximal state** | `--maximal --single` answers `Solution` | `Solution` where exactly one full assignment survives, whatever the program owes |
| **(c) a description** | a bare solve, `k_s = 1 ∧ exhausted` | as (a), at the cost of describing R (120 saturations on `zebra2`; on the clue-dropped variant about 10 s, stopping at the default budget with `exhausted false` after all 32 states are found) |

Under (a) and (c) a formalizer that writes every clue and no theory of what must be filled is told *several
states*, and has to select the theory that owes (`std.slots`, a `total` relation); under (b) the kernel reads
past the omission.

**What the experiment needs, as the coordinator reads it** (its design is the re-plan's): no table fixes
*certified* before the user decides. Every record carries each reading it can take, per instance, beside the
instance's family and shape: the word `--single` says, the word `--maximal --single` says, and where a description
is taken, `k_s`, `k_c` and `exhausted`. The two conditional probabilities are computed under each reading, per
family, and that table is what the user decides on. A flagged program is certified under none
([Q-M2.11](Q-M2.11-what-is-a-flagged-program-to-the-loop.md)).

**Since (the re-plan, 2026-11-25).** Two things the re-plan found belong to this decision, and are kept open with
it. **A fourth candidate reading**: the answer query's classes agree over R, which is what a program with several
states and one answer has ([`boxes12.ein`](../../../../examples/word_problems/boxes12.ein)); the plan records it beside
the other three. **The answer rule**, which says what short text an outcome is scored as
([S2.5.3](../../p2.5_harness/s2.5.3_metrics.md)), takes three defaults that set the accuracy of the arms that run Ein on
ambiguous and unsatisfiable instances: several states with one answer are scored as one answer; one state whose
answer query has no row is scored as ambiguous; and `Open` is no answer, not *unsatisfiable*. The rule is a
parameter of the evaluation, so the same records are scored again under another.
