# P2.8 — Representations compared (Stage L)

**Estimate:** 2 weeks; phase README only. Written to stage depth after the
Level C gate, because the five questions below are only worth asking about
a representation whose loop has numbers.
**Depends on:** the **Level C gate** ([P2.7](../p2.7_failure_scaling_generalization/README.md));
[M10](../../m10_external_benchmarks/README.md) — its hand-written encodings
for Z3, CVC5, SWI-Prolog, Soufflé, Clingo and Lean are the seed, and its
runner, which already installs and drives the systems, is reused rather than
rewritten. M10 need not have *closed*; its S10.2 (the systems) is what this
phase needs.
**Blocks:** nothing on the critical path; Level D's *formal treatment* and
*comparison* are served by this phase and [P2.9](../p2.9_formal_account/README.md)
together, and [M5](../../m5_presentation/README.md) § 9.4 *the
neural / symbolic boundary* consumes it.
**Research plan:** [`EinAf.md` § Stage L](../EinAf.md#stage-l--compare-symbolic-representations).

---

## The question, and the one it is not

Stage L asks whether Ein's representation has specific advantages **as a
target for an LLM** — not whether Ein is a better solver. The plan is
explicit: *do not attempt to prove that one representation is universally
superior*; ask five narrow questions instead.

| question | how it is measured | where the number comes from |
|---|---|---|
| Which representation is easiest for an LLM to synthesise? | one-shot parse + acceptance rate, same model, same prompt skeleton, same instances | B2 in each target's language |
| Which produces the most useful repair diagnostics? | repair success under each system's *native* feedback — an SMT solver's unsat core, Prolog's failure, Datalog's empty relation, a type checker's error, Lean's goal state — against Ein's F8 | the loop with the kernel swapped for the target and the renderer swapped for the target's output |
| Which detects semantic incompleteness? | ambiguity detection on the `ambiguous` class: a system that returns *one* model from an under-determined encoding says nothing; one that enumerates, or says `unknown`, does | the three verdict classes of [S2.3.2](../p2.3_benchmark/s2.3.2_generators.md), which every target is run on |
| Which yields the highest verified-answer precision? | `P(correct ∣ the system certifies)` per target — the H-layer probability with each system's own notion of certification | [S2.5.3](../p2.5_harness/s2.5.3_metrics.md)'s definition, applied per target |
| Which requires the least generated formal text? | tokens of formal output per instance, and per correctly solved instance | the record |

This is the question [Q10](../open_questions.md#q10--direct-llm--constraint)
was trying to ask when it was *when is direct LLM → constraint emission
acceptable?* — and re-homing it here is what made it answerable: not *is the
IR ever skipped* but *what does each target cost the model, and what does
each target tell it back*.

## What is shared with M10, and what is not

[M10](../../m10_external_benchmarks/README.md) states the same problems for
six systems **by hand**, from published encodings with provenance, and
compares answers first and clocks second. This phase has the LLM write the
encodings, compares what the model can do with each target, and never times
the solvers against each other. Shared: the systems, their installation
([S10.2](../../m10_external_benchmarks/s10.2_systems_and_install.md)), the
runner's subprocess discipline (a linked rival and a subprocess rival are
not comparable — the reason [Q-M1a.23](../../../docs/history/m1a_rust/open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding)
gave), and the hand-written encodings as the *few-shot examples* for each
target. Not shared: M10's answers are certified by three systems agreeing;
this phase's answers are the benchmark's gold, and a target's agreement with
it is the measurement.

**The no-back-end decision stands.** A target that beats Ein on a row is a
finding about targets, and the sentence [M10](../../m10_external_benchmarks/README.md#risks)
wrote applies verbatim: *every time a table shows a solver winning by 100×,
the next thought is "so call it" — that is M3, it is dropped*.

## Stages — the shape, for when it is written

- **S2.8.1** — the five targets' contracts: for each, the prompt skeleton,
  the few-shot examples from M10, the grammar where the target has one
  (SMT-LIB and Datalog do; Python does not; Lean's is its type checker), and
  the *native feedback* the target's tool returns, as a renderer.
- **S2.8.2** — the run: B2 and the loop for each target on the synthetic
  families, the three verdict classes included.
- **S2.8.3** — the five answers, as five small tables, each with the reading
  it was pre-registered to have, and the caveats where the numbers appear:
  Lean is not a solver; Datalog may be unable to state a choice
  ([M10 § Risks](../../m10_external_benchmarks/README.md#risks) — and that
  finding, if it recurs here, is direct evidence for [M1d](../../m1d_satisfiability/README.md)'s
  premise from a second direction).

## Acceptance

- Five representations, each with a contract, a run on the same instances,
  and a row in each of the five tables.
- Each table read by its pre-registered rule; no aggregate "winner".
- The `ambiguous` class reported for every target, since it is the one
  class where the targets differ in *kind* and not in degree.

## Connections

- [`EinAf.md` § Stage L](../EinAf.md#stage-l--compare-symbolic-representations).
- [M10](../../m10_external_benchmarks/README.md) — the systems, the runner,
  the encodings, and the rule about comparison inviting integration.
- [F17](../../followups/f17_formal_verification/ideas.md) — the survey of
  verification tools and the placement of K framework as Ein's closest
  relative, which is the reading list for the proof-assistant column;
  [`docs/lib/02`](../../../docs/lib/02-solvers-csp-sat-smt.md),
  [`03`](../../../docs/lib/03-theorem-proving-formal-methods.md).
- [Q2](../../open_questions.md#q2--when-does-the-graph-engine-hand-off) —
  the hand-off question, answered *never*, which this phase does not reopen.
