# P1e.3 — Medium: 36 findings

**Estimate:** ~5.5 weeks — 9 stages, 26 days.
**Depends on:** [P1e.1](../p1e.1_open_questions/README.md) for
[Q4](../p1e.1_open_questions/s1e.1.1_search_soundness_probes.md) (which
decides `CO-M1`) and
[Q-M1e.4](../open_questions.md#q-m1e4--does-the-repo-want-an-exact-count-in-prose-at-all)
(which decides whether [S1e.3.8](s1e.3.8_documentation.md) is a counting pass
or a de-counting one). [P1e.2](../p1e.2_high/README.md) for
[CO-H3](../p1e.2_high/s1e.2.1_correctness.md)'s traversal decision, which
[S1e.3.2](s1e.3.2_semantics.md) and
[S1e.3.7](s1e.3.7_code_doc_consistency.md) then document.
**Blocks:** nothing.
**Source:** the nine `medium.md` reports under [`review/`](../review/summary.md).

---

## The three mechanisms under 36 findings

Read as a list, this phase is a grab bag. Read as mechanisms it is three, and
each has a fix that is worth more than the findings it closes.

**1 — One semantics, two hand-maintained copies.**
[AR-M1](../README.md#the-findings) names four pairs and one of them has
already produced a behavioural bug
([CO-H2](../p1e.2_high/s1e.2.1_correctness.md)); a second produced divergent
event shapes ([SE-L1](../p1e.4_low/s1e.4.2_semantics.md)); a third reproduced
an incident the repo documents in its own headers (three red CI commits,
[TE-M8](../README.md#the-findings)). In two of the four, a comment
*predicting the unification* exists and the unification never happened. The
findings that fall out of this mechanism:
[CO-M4](s1e.3.1_correctness.md) (two macro pipelines),
[TE-M8](s1e.3.6_tests.md) (gate vs CI), [SE-L1](../p1e.4_low/s1e.4.2_semantics.md)
(two timeline emitters), and `CO-H2` in the phase before.

**2 — One number, computed in two places.** `verdict.k` counts models;
`stats.solution_nodes` counts what the search recorded. S1d.2.6 split them on
purpose, and **two surfaces still print the wrong one** —
[CO-M2](s1e.3.1_correctness.md) (the `Solution` table arm) and
[SE-M1](s1e.3.2_semantics.md) (`ein test`'s verbose header). Both live at a
seam named by [AR-M2](s1e.3.4_architecture.md): `Verdict` is computed once in
`ein-infer` and *rendered* by two crates that each choose a count. Fixing the
seam fixes the class; fixing the two arms fixes the instances.

**3 — A claim nothing runs.** [DO-M1](s1e.3.8_documentation.md) is eight
drifted counts and [MA-M4](s1e.3.9_maintainability.md) is the same rot inside
in-code comments; [CD-M1](s1e.3.7_code_doc_consistency.md) through
[CD-M8](s1e.3.7_code_doc_consistency.md) are eight pages that describe
something the code does not do; [TE-M2](s1e.3.6_tests.md) and
[TE-M3](s1e.3.6_tests.md) are gate assertions whose sensitivity decays
because nothing re-tightens them. The project's own thesis states the
mechanism — *a page nothing runs goes stale* — and this phase's job is to act
on it rather than restate it: **generate the number, or stop stating it.**

Nine findings sit outside all three, and they are the ones to read
individually: [CO-M1](s1e.3.1_correctness.md), [CO-M3](s1e.3.1_correctness.md),
[CO-M5](s1e.3.1_correctness.md), [CO-M6](s1e.3.1_correctness.md),
[SE-M2](s1e.3.2_semantics.md), [ST-M1](s1e.3.3_state_model.md),
[EH-M1](s1e.3.5_error_handling.md), [EH-M2](s1e.3.5_error_handling.md),
[TE-M7](s1e.3.6_tests.md).

## The one that is bigger than its severity

[ST-M1](s1e.3.3_state_model.md) — the M1 **alive-set invariant** is enforced
nowhere. It is the warrant for per-KB alive recompute, for state-key dedup,
and since M1d for the tree's exhaustiveness-by-discharge argument; which is to
say the entire model-counting story — `k`, dedup, exhaustion — is conditional
on a property only the stdlib's conventions maintain. The docs say outright
it should be *promoted to a typed invariant check when F5 lands*. F5 has not
landed and a third-party rule module is exactly the input
[M2](../../m2_nl_to_ir/README.md) plans to generate.

It is Medium in the review because nothing violates it today. It gets its own
stage here because the check is cheap and the thing it protects is the
milestone's most load-bearing claim.

## Stages

| ID | title | findings | est. |
|---|---|---|---:|
| [S1e.3.1](s1e.3.1_correctness.md) | Correctness | `CO-M1` `CO-M2` `CO-M3` `CO-M4` `CO-M5` `CO-M6` | 4 d |
| [S1e.3.2](s1e.3.2_semantics.md) | Semantics | `SE-M1` `SE-M2` `SE-M3` | 2 d |
| [S1e.3.3](s1e.3.3_state_model.md) | State model | `ST-M1` | 2 d |
| [S1e.3.4](s1e.3.4_architecture.md) | Architecture | `AR-M1` `AR-M2` | 3 d |
| [S1e.3.5](s1e.3.5_error_handling.md) | Error handling | `EH-M1` `EH-M2` | 1.5 d |
| [S1e.3.6](s1e.3.6_tests.md) | Tests | `TE-M1` … `TE-M8` | 5 d |
| [S1e.3.7](s1e.3.7_code_doc_consistency.md) | Code ↔ doc consistency | `CD-M1` … `CD-M8` | 4 d |
| [S1e.3.8](s1e.3.8_documentation.md) | Documentation | `DO-M1` `DO-M2` | 3 d |
| [S1e.3.9](s1e.3.9_maintainability.md) | Maintainability | `MA-M1` `MA-M2` `MA-M3` `MA-M4` | 2 d |

**Order.** [S1e.3.4](s1e.3.4_architecture.md) before
[S1e.3.1](s1e.3.1_correctness.md) and [S1e.3.2](s1e.3.2_semantics.md) if the
`AR-M2` seam fix is taken, since it subsumes `CO-M2` and `SE-M1`; the other
way round if it is not. [S1e.3.8](s1e.3.8_documentation.md) **last** among
the doc stages, because every other stage in the phase changes a count.

## Acceptance

- All 36 findings dispositioned in the
  [milestone index](../README.md#the-findings), each with its test, probe,
  written reason or owner.
- **The k-vs-`solution_nodes` class is closed at the seam**, not only at the
  two arms: no downstream surface recomputes or re-chooses a count that
  `Verdict` could carry.
- **Every parallel-copy pair of `AR-M1` is unified, diffed by a test, or
  justified at both sites.** Four pairs, four outcomes, none of them silence.
- **The alive-set invariant has a check** — a post-fixpoint comparison of
  derived symbols and relations against the load-time sets, behind a debug
  assertion or a diagnostic. Not the F5 typed form; the cheap one.
- **The gate is stronger in four places**: no silent skip on a missing
  `python3`, floors derived from the manifest rather than constant, the
  `run_tests.sh`/`per-commit.yml` step lists diffed by a test, and the
  mutation sweep banked as a script.
- **No prose count survives that nothing generates, dates or owns** —
  whichever of the three shapes
  [Q-M1e.4](../open_questions.md#q-m1e4--does-the-repo-want-an-exact-count-in-prose-at-all)
  chooses, it is applied to all eight sites, not to the ones that were easy.
- `./run_tests.sh` green at the phase boundary, and the test count quoted in
  the README comes from that run.

## Risks

- **The `AR-M2` seam fix is a refactor across three crates.** It is the right
  fix and it is also the largest engine change in the milestone. If it does
  not fit, the fallback is explicit and stated in
  [S1e.3.4](s1e.3.4_architecture.md): fix the two arms, write the seam's
  hazard where `finalise` is, and file the refactor with the two instances as
  its evidence. What is *not* acceptable is fixing the arms and leaving the
  seam undocumented, because that is the state that produced them.
- **Tightening the gate breaks the gate.** `TE-M2`'s floors are loose by a
  factor of two; deriving them from the manifest will expose whatever is
  currently not swept. That is the point, and it should be discovered inside
  the stage rather than in CI — run the derived floor as a report before
  making it an assertion.
- **A documentation stage that runs before the phase's own edits** would
  re-count numbers the phase then changes. Hence the ordering rule above; it
  is the same mistake `DO-M1` is a finding about.
- **`CO-M1` may be unreachable.** If
  [Q4](../p1e.1_open_questions/s1e.1.1_search_soundness_probes.md) shows the
  alive-∅ path is unreachable from any `.ein` program, the fix is a comment,
  not a check — and adding the `has_contradiction` call anyway would be a
  check nobody can ever remove, guarding a branch nobody can ever reach.

## Connections

- [`review/summary.md`](../review/summary.md) § Findings by severity — the
  36 in context.
- [`docs/history/m1d_satisfiability/the_verdict.md`](../../../docs/history/m1d_satisfiability/the_verdict.md)
  — where `k` and `solution_nodes` were split, and the reasoning the two
  unfixed surfaces did not get.
- [`docs/kernel/inference/README.md:140-187`](../../../docs/kernel/inference/README.md)
  — the alive-set invariant, stated with its own admission that it needs a
  check.
- [`docs/history/m1a_rust/oracle_ledger.md`](../../../docs/history/m1a_rust/oracle_ledger.md)
  § 2 — *41 tests passing on a SKIP line nobody read*, which `TE-M1`
  reproduces exactly.
