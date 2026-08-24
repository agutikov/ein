# P1d.2 — Obligations: the half of the vocabulary that says *must*

**Milestone:** [M1d — From saturation to satisfiability](../README.md)
**Estimate:** 3.5 weeks (18 days of stages)
**Depends on:** [P1d.10](../p1d.10_exhaustive_search/README.md)'s census — which
measures whether the corpus needs this at all — and
[S1c.1.1](../../../docs/history/m1c_external_validation/README.md#s1c11--what-the-stdlib-promises-and-what-is-exercised)'s
promise inventory, which is the same audit this phase's first stage would
otherwise have to run itself.

**Depth: at stage depth since 2026-08-24.** The decisions this README said
were the user's to make were made by the user, in-session, on
[`obligation_forms.md`](obligation_forms.md): the form is **G** (`:assert
open`, the per-KB verdict tally), the naming is **P3** with the probe rename
already executed (`7e1192c` — the third-state macro is `unknown`), bounds are
**numeral-free** (pairings to reference extents, never `L`/`U` operands), and
**obligations supersede `:hrules`** when a query carries none. The six stage
files below are written against those decisions; what each still leaves open
is stated in the file, not here.

---
Ideas

How we generate hypothesis now:
  - open world = any hypotheis
  - open world + constraints (closed) = any hypothsis + filter
  - hrule, :hrules - user defines hypothesis set
    - while it is not part of the theory (rules + ontology)

if we introduce rules emitting requirements/obligations
so having:
  - rules emitting relations = relations definitions
  - rules emitting false or (not ...) = constraints, upper bound, limit max number of arrows
  - rules emitting requirements/obligations = lower bound, min required number of arrows

What requirements/obligations give us?
  - effectively it is a set of L1 hypothesis
  - structure of the set
    - can use it with rules for hyps enumeration to filter incompatibles without saturation by domain/range elimination
  - counter of how far we are from the solution
    - one thing is a number of hypothesis, the other is number of required of open relations

---

## Goal

**The engine can record a requirement, narrow its witnesses, close it when one
remains, refute it when none do — and report the ones still outstanding.**
That last clause is the phase: a saturated state that can say *what it still
owes* is a state that knows whether it is a model.

## What exists today, and what the gap actually is

Both endpoints of the arithmetic are already implemented; the middle is empty.

| candidates left for a required arrow | today | where |
|---|---|---|
| **0** | `(false)` — the state is dead | `std.algebra`'s `total` / `surjective`, in their open-world-safe form |
| **1** | the positive is forced | `std.elim` / `std.bijection`'s `domain-elimination` / `range-elimination` |
| **≥ 2** | **nothing is recorded** | — |

`≥ 2` is exactly where a search happens, and the engine's only response to it
is to generate hypotheses over `alive` — the set of arrows that *may* hold,
with no memory of *which requirement* is waiting on them. That is what makes a
commitment an arbitrary subset instead of a choice among alternatives, and it
is the mechanism the milestone README's argument turns on.

## Stages

| stage | title | est. |
|---|---|---|
| [S1d.2.1](s1d.2.1_property_audit.md) | What each property enforces today, rule by rule | 3 d |
| [S1d.2.2](s1d.2.2_domains.md) | Domains: what a requirement quantifies over, and what closes it | 3 d |
| [S1d.2.3](s1d.2.3_the_form.md) | The obligation — form, surface, and where it lives | 3 d |
| [S1d.2.4](s1d.2.4_obligations_in_the_saturator.md) | Obligations in the saturator | 4 d |
| [S1d.2.5](s1d.2.5_hypotheses_from_obligations.md) | Hypotheses from obligations | 3 d |
| [S1d.2.6](s1d.2.6_verdicts_counters_corpus.md) | What it changes: verdicts, counters, corpus | 2 d |

**S1d.2.1** is an audit, not a design: per stdlib rule, which half of which
property it implements, in which form, and which corpus entries activate it.
It exists because the note's premise ("only half of each property is stated")
is *nearly* right, and the phase's whole shape depends on exactly how nearly.

**S1d.2.2** was the one that could sink the rest; the decided form drained
most of that. The witness domain is the obligation's **own guard**,
`?isa`-parameterised the way every stdlib scan already is — is-a-free, no
kernel type system ([S1.7.23](../../../docs/history/m1a_rust/README.md)
holds) — and discharge needs no closure at all. What the stage still owns:
the refutation division (unreachable stays with the `forall` scans), the
open-extent regime (`features/04_open`'s 14.3 GB wall), and the
closed-and-owing corner.

**S1d.2.3** was the decision stage; the decision was taken 2026-08-24 —
**G, a rule shape with a reserved verdict atom**, argument shape `forall`'s
dual (`(open ?b G B)`, form-bound variable, no numerals) plus the bare
degenerate. Its file records what was decided against (B's carrier fact, C's
head, D's numeric sugar, E now, F entirely) and implements what remains:
reserving `open`, loading and round-tripping the two forms, inert until the
next stage.

**S1d.2.4** is the engine work, and *only* the report stratum: obligation
rules evaluated per quiescence at the boundary — the discipline
[design/06](../../../docs/history/m1a_rust/design/06_saturation.md)'s
`absents_still_pass` already applies, and whose re-query cost was 72 % of an
exhaustive `zebra2` before P1a.6, which is why the stage carries a cost guard
— tallied, never stored, reported through `--events` / `--json-summary` / the
trace. Narrowing, closing and refuting stay with the scans; no verdict word
moves here.

**S1d.2.5** is the payoff and the risk, now with the decision behind it: the
**supersession ladder** (`:hrules` override → obligations → blind), branching
on one obligation's candidates — mutually exclusive, jointly exhaustive — and
a different traversal, therefore different counters, therefore the
[Q-M1a.18](../../../docs/history/m1a_rust/open_questions.md#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint)-shaped
re-baseline its file schedules, plus the fixture the rung needs
(`zebra2-obligations.ein`, the theory driving the search with no hrule in
the file).

## Acceptance for the phase

- **A puzzle can state a requirement** and the engine treats it as one:
  `total` and `surjective` with the force their names claim, and the general
  `L ≤ #{ȳ | R(x̄,ȳ) ∧ φ} ≤ U` form **only if a corpus entry needs it**.
- **A saturated state reports its outstanding obligations**, each with its
  live candidate set, through the same surfaces a fact store is inspected
  through (`--events`, `--json-summary`, the trace).
- **`complete` means discharged, not exhausted.** Today it means "the
  generator proposes nothing"; after this phase a state is complete when every
  obligation has a witness and no upper bound is violated — and where the two
  definitions disagree on a corpus entry, that entry is a test.
- **The open-world trap stays closed.** `std.algebra`'s totality check is
  written the way it is because a naive one "would fire false on every
  empty-yet state"; an unmet lower bound is a *contradiction* only when it has
  become unreachable, and that boundary is the same one
  [S1.21.8](../../../docs/history/m1a_rust/README.md) drew for negation-as-failure.
- **Every existing verdict is unchanged**, on every corpus entry. Counters may
  move; the stage that moves them says which and re-baselines with an
  argument.
- **A negative case per new mechanism**, in the form
  [P1c.1](../../../docs/history/m1c_external_validation/README.md#p1c1--stdlib-conformance)
  builds: not only "the obligation closes" but "it does not close *here*",
  which is where a guard bug lives.

## Risks

- **This is kernel surface.** The one thing M1a refused to touch. It is in
  scope here only because M1d exists to change what the engine can express —
  but the constraints M1a protected (no kernel type system, `grammar.lark` as
  the spec of record, M2's GBNF lift reading it) all still hold, and a new
  form is a cross-milestone edit.
- **Expressive creep.** `odd`, `same-count-as`, `at-least-one-of`, "as many as
  that set" — the note lists them as possibilities and each is one keyword
  away once counting exists. The rule that holds the line is the same one
  P1c.1 uses: **a keyword arrives when a corpus entry cannot be stated without
  it**, never because the form would be more general with it.
- **Obligation tracking is in the hot loop.** P1a.6 spent twelve stages
  getting `zebra -e` from 585.8 ms to 47.5 ms; a per-quiescence obligation
  scan can give that back. The counter-instrumentation discipline
  (`ein_core::counters`, compiled out unless asked) is the precedent for how
  to find out cheaply.
- **The census may say the corpus does not need it.** If negative-completion
  and `domain-elimination` already collapse candidate sets before hypotheses
  are raised, this phase buys structure nothing uses — a real possible
  outcome, and the reason P1d.10 runs first. Recording that finding and
  stopping is a successful phase.

## Cross-links

- [`ideas.md`](../ideas.md) — the note; §"Что именно отсутствует" and
  §"Что нужно saturation, чтобы стать satisfiability procedure" are this
  phase's specification in the user's own words
- [`stdlib/algebra.ein`](../../../stdlib/algebra.ein) ·
  [`bijection.ein`](../../../stdlib/bijection.ein) ·
  [`elim.ein`](../../../stdlib/elim.ein)
- [design/06 — Saturation](../../../docs/history/m1a_rust/design/06_saturation.md) ·
  [design/07 — Search layer](../../../docs/history/m1a_rust/design/07_search_layer.md)
- [P1c.1](../../../docs/history/m1c_external_validation/README.md#p1c1--stdlib-conformance) —
  the expectation form every new mechanism here gets tested through
