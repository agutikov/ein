# P1e.3 — Medium: 36 findings

**Estimate:** ~5.5 weeks — 9 stages, 26 days.
**Depends on:** [P1e.1](../p1e.1_open_questions/README.md) for
[Q4](../p1e.1_open_questions/s1e.1.1_search_soundness_probes/README.md) (which
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

> **Closed 2026-08-30 at the seam, and it was three surfaces and nine copies.**
> The third is the one a reader could see: `--stats` printed the label
> `solutions (k)` above `stats.solution_nodes`, so on the twelve `Open` entries
> a single invocation printed `solutions (k) 0` in the table and `solutions (k)
> 1` in the stats block. Not in the review, and not findable by the test that
> names that block either — `the_stats_block_reports_the_same_counters_as_the_json_summary`
> resolves a row by label prefix and takes the first match, which is the
> table's, so its first assertion had never once read the block it is named
> for.

**3 — A claim nothing runs.** [DO-M1](s1e.3.8_documentation.md) is eight
drifted counts and [MA-M4](s1e.3.9_maintainability.md) is the same rot inside
in-code comments; [CD-M1](s1e.3.7_code_doc_consistency.md) through
[CD-M8](s1e.3.7_code_doc_consistency.md) are eight pages that describe
something the code does not do; [TE-M2](s1e.3.6_tests.md) and
[TE-M3](s1e.3.6_tests.md) are gate assertions whose sensitivity decays
because nothing re-tightens them. The project's own thesis states the
mechanism — *a page nothing runs goes stale* — and this phase's job is to act
on it rather than restate it: **generate the number, or stop stating it.**

> **The eight closed 2026-08-31, and three of them left a check behind.**
> `events.md`'s emitters against its schema table
> ([`events_reference.rs`](../../../ein.rs/crates/ein-cli/tests/events_reference.rs),
> both directions, 22 kinds), `stdlib/README`'s inlining snippet as a
> round-trip test, and `docs/api/rust.md`'s test names resolved in both
> directions. Two of the three found a defect the review had not: `warn` had
> been an emitted event kind with **no row on the page** since S1e.2.3, and
> `EqClasses` is called by no engine code at all rather than merely not by
> `firing`. The one worth carrying into
> [S1e.3.8](s1e.3.8_documentation.md): `CD-M4`'s page **had** a mechanism and
> the claim slipped past its boundary, and the second marker the acceptance
> asked for does not close it — a marker makes two texts *agree*, and a rename
> leaves them agreeing about a name neither has. What closes it is resolving
> the name.

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

> **Built 2026-08-31, and "nothing violates it today" was wrong.** Two corpus
> programs assert a constant no fact names — `examples/ein-bugs/mixed-type-hypothesis.ein`
> and `tests/stdlib/algebra/07_schroder.ein` — and neither pays for it, for two
> different reasons. What a breach costs is **an answer**, shown by the eleven
> lines the stage added: `k = 0, exhausted = true`, *No solution*, where a
> model exists, against a control that is the same file plus one fact naming
> the invented object and answers `Solution k = 1` over exactly that model.
> The check is **static** rather than post-fixpoint — the rules' `:assert`
> constants, 7 µs at load — because that form is free, total, and finds every
> breach the scan finds. The defect it detects is
> [Q-M1e.21](../open_questions.md#q-m1e21--a-rule-may-name-an-object-the-search-can-never-hypothesise-about).

## Stages

| ID | title | findings | est. |
|---|---|---|---:|
| [S1e.3.1](s1e.3.1_correctness.md) | Correctness | `CO-M1` ~~`CO-M2`~~ `CO-M3` `CO-M4` `CO-M5` `CO-M6` ✅ **2026-08-30** | 4 d |
| [S1e.3.2](s1e.3.2_semantics.md) | Semantics | ~~`SE-M1`~~ `SE-M2` `SE-M3`, +`CD-M2`'s third item ✅ **2026-08-31** | 2 d |
| [S1e.3.3](s1e.3.3_state_model.md) | State model | `ST-M1` ✅ **2026-08-31** | 2 d |
| [S1e.3.4](s1e.3.4_architecture.md) | Architecture | `AR-M1` `AR-M2` ✅ **2026-08-30**, +`CO-M2` `SE-M1` `SE-L1` `TE-M8` | 3 d |
| [S1e.3.5](s1e.3.5_error_handling.md) | Error handling | `EH-M1` `EH-M2` ✅ **2026-08-31** | 1.5 d |
| [S1e.3.6](s1e.3.6_tests.md) | Tests | `TE-M1` … `TE-M8` ✅ **2026-08-31** | 5 d |
| [S1e.3.7](s1e.3.7_code_doc_consistency.md) | Code ↔ doc consistency | `CD-M1` … `CD-M8` ✅ **2026-08-31** | 4 d |
| [S1e.3.8](s1e.3.8_documentation.md) | Documentation | `DO-M1` `DO-M2` ✅ **2026-08-31** | 3 d |
| [S1e.3.9](s1e.3.9_maintainability.md) | Maintainability | `MA-M1` `MA-M2` `MA-M3` `MA-M4` ✅ **2026-08-31** | 2 d |

**Order.** [S1e.3.4](s1e.3.4_architecture.md) before
[S1e.3.1](s1e.3.1_correctness.md) and [S1e.3.2](s1e.3.2_semantics.md) if the
`AR-M2` seam fix is taken, since it subsumes `CO-M2` and `SE-M1`; the other
way round if it is not. [S1e.3.8](s1e.3.8_documentation.md) **last** among
the doc stages, because every other stage in the phase changes a count.

> **Taken 2026-08-30: the seam fix, so S1e.3.4 ran first.** The deciding
> evidence was a measurement rather than a judgment — the count the table
> prints already equals `verdict.k` on **all 230** corpus `solve` runs that
> render a table, so the refactor's stated risk (*"every golden that contains
> a rendered verdict line"*) is empty and the whole change set moves **110 of
> 8 835** renderings, all of them by an unrelated JSON key permutation. So
> `CO-M2` and `SE-M1` are **fixed**, and what remains of them in
> [S1e.3.1](s1e.3.1_correctness.md) and [S1e.3.2](s1e.3.2_semantics.md) is the
> half each stage's own notes called the more valuable one: a fixture for the
> mixed `Solution`/`Open` regime, and the cross-surface consistency test. The
> first landed with S1e.3.1 —
> [`examples/features/13_mixed_solution_and_open.ein`](../../../examples/features/13_mixed_solution_and_open.ein),
> and it broke a counter identity in `summary_properties` within a minute of
> being added, because that property's exception was written as *except
> `Open`* when the real one is *unless the program owes*.

> **And the second landed with S1e.3.2, 2026-08-31.** The cross-surface test
> rebuilds every `ein test -v` header **from its `--json-report` row** and
> compares, over the whole corpus in 0.06 s; **13 of the 68** checked queries
> are cells where the two numbers differ, and the control — printing
> `solution_nodes` under `k =` again — fails it by file name. The same stage
> closed `SE-M2` at the seam rather than at the arm (`build_aborted` is now
> `build` with an `Answer::Aborted`, which is mechanism **1** again) and found
> the `Aborted` arm's *third* asymmetry, `verdict.reason`, which the review had
> not named.

> ## ✅ Closed 2026-08-31 — nine stages, 36 findings, every acceptance line met
>
> `./run_tests.sh` green at the boundary: **804 tests over 90 targets**, and
> that is the number `README.md` quotes, dated, from this run. All 36
> dispositioned in the [milestone index](../README.md#the-findings), none
> empty.
>
> **The three mechanisms held, and each cost more than its findings.** The
> `k`-vs-`solution_nodes` seam was **nine** copies over three surfaces, one of
> which no test could see. `AR-M1`'s four parallel-copy pairs kept producing
> the same shape after they were named — `MA-M2`'s two comments each had a
> correct copy elsewhere, and `CD-M2`'s page had a whole missing event kind. And
> *a claim nothing runs* was the biggest by an order of magnitude: `DO-M2` was
> six dangling references in the review and **264** once the instrument that
> already existed was pointed past the one tree it had ever been run on.
>
> **Five findings turned out to be larger than reported**, all in the same
> direction — the review sampled and the sample understated:
> `CD-M1` three pages → five, `CD-M2` three items → four (`warn` had no row at
> all), `MA-M3` one increment site → three, `MA-M4` two disagreeing snapshots →
> three with no owner, `DO-M2` six links → 264.
>
> **What the phase left running.** Six new checks, none over fifty lines:
> `events_reference.rs` (emitters ↔ schema, both directions),
> `the_page_and_the_file_name_the_same_tests`,
> `the_inlining_route_the_stdlib_readme_documents_round_trips`,
> `the_state_key_merge_counter_is_not_a_constant_zero`,
> `the_injected_contradiction_fans_out_and_the_union_overstates_it`, and the
> whole-tree link sweep, which is now the gate's **seventh** static step.
>
> **And one rule.** [Q-M1e.4](../open_questions.md#q-m1e4--does-the-repo-want-an-exact-count-in-prose-at-all):
> an exact number carries the **command** that re-takes it, the **date** it was
> taken, or the **test** that pins it — and a number that can carry none is not
> written down. Applied to prose in S1e.3.8 and to code comments in S1e.3.9.

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
  [Q4](../p1e.1_open_questions/s1e.1.1_search_soundness_probes/README.md) shows the
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
