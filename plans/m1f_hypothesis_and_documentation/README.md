# M1f — The structure of the hypothesis set, and the documentation ein does not have

**Estimate:** ~12 weeks — 2 phases, 11 stages, **55.5 days** of stage
estimates. One stage,
[S1f.5.20](p1f.5_documentation_and_other/s1f.5.20_docs_refactor.md), is 23 of
them.
**Status:** created 2026-08-29, out of
[M1e](../m1e_review_processing/README.md) — which had carried both phases since
2026-08-28 and had said in its own phase table that they were **† not review
processing** and *"additive and may be cut whole"*. This is that sentence taken
up: they were not cut, they were given an M-number.
**Depends on:** M1e in three places, all of them soft except one —
[§ What it inherits from M1e](#what-it-inherits-from-m1e).
**Blocks:** nothing.

---

## What this is

Two phases that share nothing except having been written on the same day, at
the same user's instruction, into a milestone that was about something else:

| ID | title | stages | est. | ends with |
|---|---|---:|---:|---|
| [P1f.5](p1f.5_documentation_and_other/README.md) | Documentation, and other | 3 (+1 proposed) | 33 d | every statement convertible to NL, measured at the **5 %** `zebra2`'s model renders today; `:priority` removed and the schedule derived from the rule graph, on a control sweep where **137 of 139 entries are identical** without it; and `docs/ein/` — the tree a released system would have |
| [P1f.10](p1f.10_hypothesis_structure/README.md) | The structure of the hypothesis set | 8 | 22.5 d | the exclusion relation measured; groups defined as a **cover**, not a partition; the join refusing a same-group pair before the fork with **not one model moved**; the bijection derived where it is declared nowhere, or a written *no*; the **domain** of that structure stated; the tree's **200 053×** attributed, with `--traversal` replacing `EIN_TRAVERSAL`; and a ruling on whether a refutation may rest on an `absent` |

The numbering is deliberate and the gaps are real: **1–4 and 6–9 are
unassigned**, and the two phases here keep the positions they were given rather
than being packed. A phase number that moves is a phase number every citation
of it has to chase.

## Why they are one milestone

Because the alternative was two milestones with one phase each, and because
what they have in common is the thing M1e could not give them: **neither
processes a finding.** M1e's spine is *a finding is a claim until something
holds it*; both of these start from a user's instruction and an unmeasured
idea, which is a different kind of work with a different acceptance.

They also meet in one place, and it is worth naming because it is the only
technical connection between them:
[S1f.5.6](p1f.5_documentation_and_other/s1f.5.6_rule_priority.md) removes
`:priority` and derives the firing order from the rule dependency graph;
[S1f.10.5](p1f.10_hypothesis_structure/s1f.10.5_ordering.md) replaces the
lattice's candidate order with one derived from the group structure. Two
orderings, both currently declared by hand, both to be derived from something
the program already says. Whichever runs second should read what the first
learned about deriving an order nobody wrote down.

## What it inherits from M1e

**Three dependencies, and only one of them blocks.**

| | on | kind |
|---|---|---|
| `S1f.5.6` | [`S1e.1.3`](../m1e_review_processing/p1e.1_open_questions/s1e.1.3_unsat_core_completeness.md) — the unsat core's retention rule | **satisfied** 2026-08-29 |
| `S1f.5.20` | M1e's `S1e.2.2`, `S1e.3.7`, `S1e.3.8` — the `docs/kernel` triage and the drift repairs | **hard, and unrun.** A refactor that moves 38 pages before they are triaged moves the wrong ones |
| `P1f.10` | [`S1e.1.1`](../m1e_review_processing/p1e.1_open_questions/s1e.1.1_search_soundness_probes/README.md), for [Q-M1e.6](../m1e_review_processing/open_questions.md#q-m1e6--what-is-a-solution-and-what-is-a-model)'s ruling | **satisfied** 2026-08-28 |

**The question ids stay where they were raised.** `Q-M1e.6`, `Q-M1e.9`,
`Q-M1e.11` and the rest are M1e's, they keep their ids, and the stages here
cite them across the milestone boundary. That is this repo's rule — *do not
reuse a closed id* — and the alternative, renumbering a question because the
stage that owns it moved, would make every earlier citation of it false.
[`open_questions.md`](open_questions.md) here is for what **M1f** raises, as
`Q-M1f.<n>`.

**Two stages did not come.** `S1e.5.1` (the configuration reference) and
`S1e.5.2` (what a solution is, and what a model is) had already shipped under
M1e's numbering, and five places in the tree cite `S1e.5.1` as the stage that
shipped [`configuration.md`](../../docs/kernel/configuration.md). They stayed,
and [M1e's P1e.5](../m1e_review_processing/p1e.5_documentation_and_other/README.md)
is now the one-stage record of what that phase delivered.

## Acceptance for the milestone

- **P1f.10 changes no answer.** Not one model, not one fact, not one verdict
  word, on any corpus entry, at any `-m` — the phase's own first acceptance
  line, and the reason it may ship an optimisation at all.
- **P1f.5 pins every page it adds by something that runs.** M1e's
  [Q-M1e.4](../m1e_review_processing/open_questions.md#q-m1e4--does-the-repo-want-an-exact-count-in-prose-at-all)
  is the rule and it is inherited unchanged: *every count a test pins is
  exactly right; every count only prose states has drifted.*
- **Every rung of P1f.10 is measured before the next is built**, and the
  entries where the win is **zero** are named.
- **Nothing here renumbers a shipped stage or a raised question.**
- `./run_tests.sh` green at every stage boundary, and every golden either
  phase moves is named in its stage file **before** it moves.

## Risks

- **S1f.5.20 is 23 days and blocked.** It is the largest single stage in
  either milestone, it closes no finding, and it cannot start until three M1e
  stages that have not run do. Its own § How to cut it carries a seven-stage
  split; taking that split is the first decision this milestone should make.
- **P1f.10 is a CSP in disguise**, and the risk is not that the idea is wrong
  but that a stage writes a solver instead of a *structure* and the engine
  acquires a second search. The phase's own risk list is where this is argued;
  it is repeated here because it is the milestone-level failure mode.
- **The two phases can drift on the ordering question.** S1f.5.6 and S1f.10.5
  both derive an order that is currently declared by hand, in different parts
  of the engine, and nothing forces them to agree on what *derived* means.

## Connections

- [M1e](../m1e_review_processing/README.md) — where both phases were written,
  and the milestone whose `S1e.2.2` / `S1e.3.7` / `S1e.3.8` gate `S1f.5.20`.
- [`docs/history/m1d_satisfiability/layer_census.md`](../../docs/history/m1d_satisfiability/layer_census.md)
  — the `Σₖ C(alive, k)` finding, P1f.10's motivating number.
- [`c/README.md`](../../c/README.md) — the **3 668 465×** between three C
  baselines, and *"the difference is not an algorithm — it is one integer per
  clue"*, which is P1f.10's thesis stated about a different search.
- [`plans/ideas/06-inference-rules-completeness.md`](../ideas/06-inference-rules-completeness.md)
  — the user's own framing of what the rule set owes.
