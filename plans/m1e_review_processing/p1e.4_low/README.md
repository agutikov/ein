# P1e.4 — Low: 21 findings

**Estimate:** ~1.5 weeks — 8 stages, 7 days.
**Depends on:** [P1e.1](../p1e.1_open_questions/README.md) for
[Q7](../p1e.1_open_questions/s1e.1.5_cli_semantics.md) (`EH-L1`'s ruling) and
[Q10](../p1e.1_open_questions/s1e.1.6_coverage_gaps.md) (`TE-L5`'s
disposition); [P1e.2](../p1e.2_high/README.md) and
[P1e.3](../p1e.3_medium/README.md) for four findings that are the tail of a
larger one.
**Blocks:** nothing. This phase is **droppable** — see below.
**Source:** the eight `low.md` reports under [`review/`](../review/summary.md).

---

## What Low means here

Twenty-one findings, and the review's own summaries describe most of them as
one-line fixes. Nothing in this phase changes an answer, a verdict, a count or
an exit code, with one exception the phase takes deliberately
([EH-L1](s1e.4.4_error_handling.md), if the ruling is *refuse*).

But *Low* is not *uninteresting*, and three of these are worth reading:

- **[CO-L1](s1e.4.1_correctness.md)** — the interner and fact store bound
  arena offsets by **id count**, not by arena bytes, so a table with fewer
  than 2³⁰ entries and more than 4 GiB of text would silently wrap. It is
  unreachable at corpus scale, and it contradicts the module's own stated
  principle in the exact way the principle warns about: *hitting a limit is an
  error somebody can read rather than a silent wrap into another value's
  identity*.
- **[ST-L1](s1e.4.3_state_model.md)** — `EqClasses` **auto-vivifies on read**:
  merely asking `equivalent(a, c)` inserts `c` into the parent map, so output
  order depends on query history. Inert today because nothing fires equality
  propagation — and the first real consumer (the F4 e-graph seam) inherits it
  silently. A query-mutating API is precisely the shape the determinism rules
  exist to keep away from observables.
- **[TE-L5](s1e.4.5_tests.md)** — the release workflow's four-platform
  matrix, its jobs-cross-diff and its `--no-default-features` leg have
  **never executed**. The workflow is honest about it; a reader of the badge
  would not be.

Four are the tail of something larger and are cross-referenced rather than
re-argued: [SE-L1](s1e.4.2_semantics.md) and
[SE-L2](s1e.4.2_semantics.md) are two more of
[AR-M1](../p1e.3_medium/s1e.3.4_architecture.md)'s parallel-copy pairs;
[MA-L5](s1e.4.8_maintainability.md) is the comment above the list that drifted
in [CO-H2](../p1e.2_high/s1e.2.1_correctness.md); and
[TE-L3](s1e.4.5_tests.md) is the gate/CI divergence one flag down from
[TE-M8](../p1e.3_medium/s1e.3.6_tests.md).

## Why it is last, and why it may be dropped

Every one of these is real and none blocks anything. The milestone's stated
fallback is that if it has to end early, it ends after
[P1e.3](../p1e.3_medium/README.md) and this phase becomes one followup issue
carrying the 21 rows — the one place in the milestone where dropping scope
costs nothing but tidiness.

Two things make that fallback safe, and they are the reason the phase is
planned in full anyway: the findings are **already written down** in the
review, and the disposition column in the
[milestone index](../README.md#the-findings) is what carries them. A dropped
phase with an index is a backlog; a dropped phase without one is a lost
review.

## Stages

One per topic, matching the Medium phase's topic order so the two read as one
table. Architecture has no Low findings.

| ID | title | findings | est. |
|---|---|---|---:|
| [S1e.4.1](s1e.4.1_correctness.md) | Correctness | `CO-L1` | 0.5 d |
| [S1e.4.2](s1e.4.2_semantics.md) | Semantics | `SE-L1` `SE-L2` | 1 d |
| [S1e.4.3](s1e.4.3_state_model.md) | State model | `ST-L1` | 0.5 d |
| [S1e.4.4](s1e.4.4_error_handling.md) | Error handling | `EH-L1` `EH-L2` | 0.5 d |
| [S1e.4.5](s1e.4.5_tests.md) | Tests | `TE-L1` … `TE-L5` | 2 d |
| [S1e.4.6](s1e.4.6_code_doc_consistency.md) | Code ↔ doc consistency | `CD-L1` `CD-L2` `CD-L3` | 1 d |
| [S1e.4.7](s1e.4.7_documentation.md) | Documentation | `DO-L1` `DO-L2` | 1 d |
| [S1e.4.8](s1e.4.8_maintainability.md) | Maintainability | `MA-L1` … `MA-L5` | 1.5 d |

**One commit per stage** where the findings are independent one-liners
(`S1e.4.6`, `S1e.4.7`, `S1e.4.8`), so the log reads as *the Low doc pass*
rather than as fifteen unrelated touches. Where a finding needs a test
(`CO-L1`, `ST-L1`, `TE-L5`), it gets its own commit.

## Acceptance

- All 21 dispositioned in the
  [milestone index](../README.md#the-findings).
- The three findings that are more than cosmetic have a test or a written
  argument: the arena-offset guard, the `EqClasses` vivification decision, and
  the release matrix's status.
- Nothing in this phase moves a golden without saying so first — the one
  candidate is [MA-L2](s1e.4.8_maintainability.md), the ~22-space run inside
  the non-exhausted `Contradiction` headline, which reaches output on every
  such run and is pinned by **no** test (the corpus banks an md5 digest, which
  cannot reveal it).
- `./run_tests.sh` green.

## Risks

- **A one-line fix that is not.** [MA-L2](s1e.4.8_maintainability.md) looks
  like a lost line-continuation and changes user-visible output;
  [EH-L1](s1e.4.4_error_handling.md) may become a refusal that some corpus
  cell relies on. Both are checked before being changed.
- **Batching hides a real one.** The one-commit-per-stage rule is for
  readability, not for speed; a finding inside a batch that turns out to need
  a test comes out of the batch.
- **The phase is skipped and the index is not updated.** That is the only
  outcome here that loses information, and it is why the fallback is written
  as *becomes one followup issue carrying the 21 rows* rather than *is
  dropped*.

## Connections

- [`review/summary.md`](../review/summary.md) — the 21 in context.
- [AR-M1](../p1e.3_medium/s1e.3.4_architecture.md) — the pattern two of these
  belong to.
- [`docs/history/m1a_rust/design/02_determinism_and_order.md`](../../../docs/history/m1a_rust/design/02_determinism_and_order.md)
  — the rules [ST-L1](s1e.4.3_state_model.md) sits just outside.
