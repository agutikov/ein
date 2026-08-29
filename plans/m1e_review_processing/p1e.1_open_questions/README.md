# P1e.1 — The ten questions

**Estimate:** 2 weeks — 6 stages, 9 days. (10 until 2026-08-28, when `Q6` left
S1e.1.1 for [P1e.1b](../p1e.1b_hypothesis_structure/s1e.1b.6_obligations_under_hypothesis.md).)
**Depends on:** nothing. This phase reads, probes and rules; it changes the
engine only where a probe's answer *is* a one-line fix.
**Blocks:** [P1e.2](../p1e.2_high/README.md) in three places and
[P1e.3](../p1e.3_medium/README.md) in one —
[Q3](s1e.1.4_defined_behaviour_q_m1a8.md) gates
[CD-H3](../README.md#the-findings),
[Q6](../p1e.1b_hypothesis_structure/s1e.1b.6_obligations_under_hypothesis.md)
gates [CO-H3](../README.md#the-findings)(c) and is **no longer this phase's** —
the ruling it needed was taken here, the probe moved to P1e.1b,
[Q5](s1e.1.1_search_soundness_probes/README.md) can move corpus goldens, and
[Q4](s1e.1.1_search_soundness_probes/README.md) decides whether
[CO-M1](../README.md#the-findings) is a bug.
**Source:** [`review/open-questions.md`](../review/open-questions.md) — ten
questions the review could not resolve from repository evidence, none promoted
to a finding.

---

## Why this phase is first

Because four of the ten decide what a later fix *is*, and because two of them
are about claims the project puts in its own headline.

The review's ten are not a residue. Read together they are a short list of
the places where this engine's guarantees rest on an argument nobody has
written out:

- **Q1** — `--jobs N` is the same computation. Twenty thousand invariance
  cells say it is; no structural argument says *why* a clause learned
  mid-layer on another thread cannot prune a candidate this thread would have
  entered. Evidence without a mechanism.
- **Q2** — the unsat core is the smallest frontier over recorded derivations.
  `MAX_ALT_JUSTIFICATIONS = 32` means *recorded* is a lossy word.
- **Q3** — `defined_behaviour.md`'s one self-declared latent bug did not
  reproduce. The page exists to replace a deleted Python file as the
  statement of behaviour. **Answered 2026-08-29** — the page named the wrong
  argument kind, the real shape loses a derivation in a release build, and
  five source comments said the false thing too
  ([S1e.1.4](s1e.1.4_defined_behaviour_q_m1a8.md)).
- **Q4, Q5** — two soundness premises, one per search path: the inter-layer
  alive-∅ shortcut and the lookahead lever that flips two fixtures' verdicts.
  **Q4 is answered** — the alive-∅ path records a state its own rules refute,
  at stock config
  ([D1](s1e.1.1_search_soundness_probes/d1_q4_which_route_reaches_the_site.md)).
- **Q6** — the tree's *asking once is asking enough*. The ruling was taken here
  on 2026-08-28 (the rung mode is re-read at every node); the probe and the
  semantics are
  [S1e.1b.6](../p1e.1b_hypothesis_structure/s1e.1b.6_obligations_under_hypothesis.md)'s.
- **Q7** — `-n 0`.
- **Q8, Q10** — two claims with no owner: that the two zebra encodings agree,
  and that the release matrix builds.
- **Q9** — the review's own hole, and the reason this milestone may not
  conclude the tree is clean.

Answering them first is not thoroughness for its own sake. It is the same
argument every measurement phase in this repo has made: **work scheduled
against an assumed answer is scheduled wrong.**

## What an answer is

Ratified in [S1e.1.1](s1e.1.1_search_soundness_probes/README.md) T1 as
[Q-M1e.1](../open_questions.md#q-m1e1--what-is-the-standard-of-proof-for-refuted),
then binding on every stage of the milestone. In short: a question about
**behaviour** is answered by an executed probe banked as a test; a question
about **absence** is answered by naming the thing or by adding it; a question
about **risk** is answered by a check, or by an argument written beside the
code — never by an argument written only in a plan.

Every stage below ends with the answer written into the tree, not into this
phase. The phase's own artefact is the disposition column of
[the milestone's question index](../README.md#the-questions--10) and, for the
questions that become permanent, a `Q-M1e.<n>` or a new section of
[`defined_behaviour.md`](../../../docs/kernel/defined_behaviour.md).

## Stages

| ID | title | est. | ends with |
|---|---|---:|---|
| [S1e.1.1](s1e.1.1_search_soundness_probes/README.md) | Two soundness probes — Q4, Q5 | 2 d | one constructed fixture per question; the lookahead flip's true model sets derived by hand and the golden audit done; the standard of proof ratified |
| [S1e.1.2](s1e.1.2_determinism_under_jobs.md) | Determinism under `--jobs` — Q1 | 2 d | the structural argument written where `Nogoods` lives, or an injected-clause test showing the commit-order replay masks it — and, if neither, the claim narrowed |
| [S1e.1.3](s1e.1.3_unsat_core_completeness.md) | What the core promises — Q2 | 1.5 d | either a fixture where eviction enlarges the core, or the retention argument written next to `MAX_ALT_JUSTIFICATIONS`; the README's claim matched to whichever holds |
| [S1e.1.4](s1e.1.4_defined_behaviour_q_m1a8.md) | Q-M1a.8's real trigger — Q3 | 1 d | ✅ **2026-08-29** — three probes banked in `rule_semantics.rs`; §3.2 rewritten to the shape that reproduces; `Q-M1a.8` closed as stated and the live half filed as [Q-M1e.16](../open_questions.md#q-m1e16--the-binding-key-compares-two-register-layouts-as-one) |
| [S1e.1.5](s1e.1.5_cli_semantics.md) | `-n 0` — Q7 | 0.5 d | a ruling: refuse it with the `jobs_spec` argument, or define it and pin it with a test |
| [S1e.1.6](s1e.1.6_coverage_gaps.md) | What nothing pins — Q8, Q9, Q10 | 2 d | the two-encodings assertion named or written; the four unswept surfaces of Q9 scoped, with one of them swept here; the release matrix's status stated where a reader would believe it |

## Acceptance

- Each of Q1–Q10 has an answer in the tree — a test, a fixture, a paragraph
  at a `file:line`, or a recorded ruling — and the milestone's
  [question index](../README.md#the-questions--10) points at it.
- **Q3, Q4 and Q5 are answered before their dependent finding is
  touched** — and Q6's *ruling* is, which is what its dependent finding
  applies — and each answer states what the dependent fix now has to be.
- No question is closed with "could not reproduce" alone: a non-reproduction
  is banked as the probe that did not reproduce it
  ([Q-M1e.1](../open_questions.md#q-m1e1--what-is-the-standard-of-proof-for-refuted)).
- Any question that cannot be answered here is re-filed as a `Q-M1e.<n>` with
  an owner, and the stage says which.
- `./run_tests.sh` green; any golden this phase moves is named in the stage
  file **before** it moves.

## Risks

- **Q5 moves goldens.** If the current verdicts for `branching/06` and
  `lattice/02` are the wrong side of the flip, fixing the semantics is a
  deliberate re-bless of corpus goldens — and a re-bless that was not
  predicted is a stop. S1e.1.1 does the hand-derivation *before* touching
  anything, and the two fixtures are small enough for that to be honest work.
- **Q1 is answerable only in the negative.** A determinism argument is hard
  to make positively and easy to break with one counterexample. The stage's
  fallback is explicit: if no argument holds, the *claim* narrows to what the
  evidence supports, which is a documentation change, not a defeat.
- **Q9 invites the milestone to grow into a second review.** It does not:
  the stage scopes the four unswept surfaces and sweeps exactly one, naming
  owners for the rest. Re-running the aborted pass is not this milestone's
  work.

## Connections

- [`review/open-questions.md`](../review/open-questions.md) — the ten, with
  the evidence that created each ambiguity.
- [`docs/history/m1a_rust/design/02_determinism_and_order.md`](../../../docs/history/m1a_rust/design/02_determinism_and_order.md)
  — where Q1's argument belongs if it can be made.
- [`docs/kernel/defined_behaviour.md`](../../../docs/kernel/defined_behaviour.md)
  — §3.2 is Q3's subject and §4 is Q7's table.
- [`docs/history/m1a_rust/open_questions.md`](../../../docs/history/m1a_rust/open_questions.md)
  — Q-M1a.8, which Q3 closed on 2026-08-29 in a third direction: the claim is
  refuted *and* the item stays a bug, under a different trigger.
