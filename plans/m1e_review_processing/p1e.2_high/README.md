# P1e.2 — High: six findings

**Estimate:** 2 weeks — 3 stages, 12 days. (2 and 11 until 2026-08-28, when
[D4](../p1e.1_open_questions/s1e.1.1_search_soundness_probes/d4_q_m1e9_upward_closure.md)
was ruled *B now* and the containment landed here as S1e.2.3.)
**Depends on:** [P1e.1](../p1e.1_open_questions/README.md) for three answers
— [Q3](../p1e.1_open_questions/s1e.1.4_defined_behaviour_q_m1a8.md) decides
what `CD-H3` is, [Q6](../p1e.1_open_questions/s1e.1.1_search_soundness_probes/README.md)
decides what `CO-H3`(c) is, and the class sweep in
[T1e.1.6.2](../p1e.1_open_questions/s1e.1.6_coverage_gaps.md) is where
`CO-H1` stops being one instance.
**Blocks:** nothing. The engine is green today and stays green through this
phase; every change here is a refusal added, a list unified, or a page
re-filed.
**Source:** [`review/correctness/high.md`](../review/correctness/high.md) (3),
[`review/code-doc-consistency/high.md`](../review/code-doc-consistency/high.md) (3).

---

## What makes these six High

Two different things, and the phase keeps them in separate stages because
they need different kinds of care.

**Three are code defects whose failure mode is not a wrong answer but a
broken contract.** A well-formed program panics the process
([CO-H1](../README.md#the-findings)). A guard that exists to stop a
declarator binding a reserved name is bypassable through one of three import
routes ([CO-H2](../README.md#the-findings)). A traversal ignores the CLI's
stop policy, and prints *refuted so far (0 facts)* as the evidence for a
`Contradiction` it learned nothing to support
([CO-H3](../README.md#the-findings)). Two of the three were **reproduced
against the release binary** — the only findings in the whole review that
were — so their disposition is settled before the phase starts, and the work
is the fix and the fixture.

**Three are documentation defects with unusual leverage**, and the leverage is
the project's own doing. `CLAUDE.md` declares `docs/kernel` canonical and
load-bearing: *this tree is now the only statement of intent that is not also
the implementation, so a claim here is checked by `cargo test` and by nothing
else*. Under that rule, a kernel page describing an engine that does not
exist is not a stale doc — it is a **false specification**, and the review
found at least six of them, plus five pages that contradict each other about
whether the `Open` verdict exists, plus one normative page whose single
self-declared latent bug does not reproduce.

The repo already demonstrates its own thesis here, which is the useful way to
read this phase: **every number a test pins is exactly right; every page
nothing runs has rotted.** The fix for the second half is not more diligence.

## The one finding that crosses the phase

The tree traversal appears three times in this milestone —
[CO-H3](../README.md#the-findings) here,
[SE-M3](../p1e.3_medium/s1e.3.2_semantics.md) and
[CD-M2](../p1e.3_medium/s1e.3.7_code_doc_consistency.md) in Medium, and
[Q6](../p1e.1_open_questions/s1e.1.1_search_soundness_probes/README.md) in the
questions. [S1e.2.1](s1e.2.1_correctness.md) T3 owns the **decision**; the
Medium stages render it. That split is deliberate: what a tree *reports*
where a lattice reports layers is M1d's open `T1d.10.6.4`, and this milestone
does not get to answer it. What it does get to do is stop the shipped
surfaces from saying things that are false under either answer.

## Stages

| ID | title | findings | est. | ends with |
|---|---|---|---:|---|
| [S1e.2.1](s1e.2.1_correctness.md) | Correctness — the panic, the guard, the traversal | `CO-H1` `CO-H2` `CO-H3` | 5 d | a compile-time arity check with a positioned diagnostic and a `broken/` fixture; one reserved-name constant with a test that the lists are one; the tree honouring `-n`/`-m` and refusing to print evidence it does not have |
| [S1e.2.3](s1e.2.3_naf_refutation_diagnostic.md) † | Q-M1e.9's containment — a diagnostic for a refutation resting on an `absent` | — | 1 d | the existing `warn-derived-naf` watch widened to the hypothesis-eligible case, the corpus measured for what it would fire on, and a message that names the replacement rather than only the hazard |
| [S1e.2.2](s1e.2.2_code_doc_consistency.md) | Code ↔ doc — the canonical tree | ~~`CD-H1`~~ ~~`CD-H2`~~ ~~`CD-H3`~~ | 6 d | ✅ **done 2026-08-30.** 40 pages triaged — **37 current · 3 banner · 0 move** — and four of the eleven that needed work are not in `CD-H1`, all four found by the identifier sweep. `CD-H2` in one commit. `Q-M1e.3` answered **(c)**, and its candidate (a) turned out to be a directory `docs/history/README.md` had already declined. `Q-M1e.20` raised. New: `utils/doc_audit.py` and the tree's two new README sections. **`CD-H3` was done** 2026-08-29 in [S1e.1.4](../p1e.1_open_questions/s1e.1.4_defined_behaviour_q_m1a8.md), where the probes were |

## Acceptance

- **No well-formed `.ein` program panics the process.** The known shape is
  refused at compile time with a `file:line:col` diagnostic and pinned by a
  fixture under `examples/broken/`; the class sweep from
  [T1e.1.6.2](../p1e.1_open_questions/s1e.1.6_coverage_gaps.md) has a fixture
  for every cell it found wrong.
- **One reserved-name list.** `ein-ir` consumes `ein-core`'s constant or a
  single shared one; a test asserts they are one; a fixture pair pins the
  guard *per declaration route* — direct, flat `:symbols`, and qualified
  import — because the route is what differed.
- **The tree traversal's shipped surfaces are true.** `stop_after` and
  `max_set_size` are honoured or explicitly refused with a stated reason; a
  dead branch either learns a no-good or the `Contradiction` arm declines to
  print a core it does not have; the rung premise is enforced or argued at
  the site per
  [Q6](../p1e.1_open_questions/s1e.1.1_search_soundness_probes/README.md).
- **`docs/kernel` is triaged page by page**, with each page in exactly one
  declared state — *current*, *superseded with a banner*, *moved to
  `docs/history/`* — and no page left in the fourth state the review found.
- **The tree does not contradict itself about the `Open` verdict**: one
  answer to *how many verdict words are there*, one answer to *who reads the
  obligation tally*, across all five stale pages and the three that are
  already right.
- `./run_tests.sh` green. Any golden this phase moves is named in its stage
  file first — the likely candidates are the `broken/` `.expected` set (new
  files, no move) and, if
  [Q5](../p1e.1_open_questions/s1e.1.1_search_soundness_probes/README.md) landed on
  the other side, the two lookahead entries.

## Risks

- **Fixing the traversal into a shape M1d has not chosen.** The mitigation is
  in [S1e.2.1](s1e.2.1_correctness.md) T3: two of the three defects have a
  fix that is right under *every* answer to `T1d.10.6.4`, and the third —
  what the arm prints — is taken in its narrowest form, *refuse rather than
  lie*. Anything wider waits.
- **Rewriting a kernel page instead of re-filing it.**
  [CD-H1](../README.md#the-findings)'s biggest item, `algorithm_layer_n.md`,
  is a P1.5b design document whose three solve entries never shipped and
  whose central mechanism was retired as **NAF-unsound**. Rewriting it to
  describe today's engine would produce a page that is neither history nor a
  specification — the failure mode `docs/api/`'s 🏛 banner exists to prevent.
  The rule is [Q-M1e.3](../open_questions.md#q-m1e3--who-owns-a-page-that-should-be-neither-fixed-nor-deleted)
  and it is taken before the page is touched.
  *(Held. Answered **(c)** 2026-08-30 and applied first: three banners, zero
  rewrites, zero moves. The risk had a second edge the entry did not name —
  `docs/history/m1a_rust/design/07` cites that page as the contract the port
  had to reproduce, so **deleting** it was as unavailable as rewriting it.)*
- **The arity check is wider than one predicate.** `eq`/`neq` are the two
  built-ins, but the compiler's tolerance is the pattern, not the predicate.
  Scoping the fix to `(eq ?x)` closes the instance and leaves the class —
  which is why the sweep is in P1e.1 and its results land here.
- **Two of these three code fixes touch the loader and the matcher**, the two
  hottest paths in the tree. Both fixes are load-time or compile-time, so the
  measured cost should be zero; if a bench moves, that is a finding and the
  stage says so rather than absorbing it.

## Connections

- [`review/correctness/high.md`](../review/correctness/high.md),
  [`review/code-doc-consistency/high.md`](../review/code-doc-consistency/high.md)
  — the six, with locations and the reproduction notes.
- [`docs/kernel/defined_behaviour.md`](../../../docs/kernel/defined_behaviour.md)
  — §4's error table, which `CO-H1` falls outside of, and §3.2, which is
  `CD-H3`.
- [`docs/kernel/README.md`](../../../docs/kernel/README.md) — the tree's entry
  point, and itself one of the pages `CD-H1` names.
- [`docs/history/m1d_satisfiability/README.md#s1d106--the-traversal`](../../../docs/history/m1d_satisfiability/README.md)
  — what the traversal shipped, and `T1d.10.6.4`, which it did not.
