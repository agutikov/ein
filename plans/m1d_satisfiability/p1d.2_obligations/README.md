# P1d.2 — Obligations: the half of the vocabulary that says *must*

**Milestone:** [M1d — From saturation to satisfiability](../README.md)
**Estimate:** 3.5 weeks (18 days of stages)
**Depends on:** [P1d.10](../p1d.10_exhaustive_search/README.md)'s census — which
measures whether the corpus needs this at all — and
[S1c.1.1](../../../docs/history/m1c_external_validation/README.md#s1c11--what-the-stdlib-promises-and-what-is-exercised)'s
promise inventory, which is the same audit this phase's first stage would
otherwise have to run itself.

**Depth: this is a phase README, not a stage plan.** See the milestone's
[§ How deep this plan is](../README.md#how-deep-this-plan-is) — the stage files
are written when the phase starts, because the note this phase comes from is a
discussion note and the decisions below are the user's to make.

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
| S1d.2.1 | What each property enforces today, rule by rule | 3 d |
| S1d.2.2 | Domains: what a requirement quantifies over, and what closes it | 3 d |
| S1d.2.3 | The obligation — form, surface, and where it lives | 3 d |
| S1d.2.4 | Obligations in the saturator | 4 d |
| S1d.2.5 | Hypotheses from obligations | 3 d |
| S1d.2.6 | What it changes: verdicts, counters, corpus | 2 d |

**S1d.2.1** is an audit, not a design: per stdlib rule, which half of which
property it implements, in which form, and which corpus entries activate it.
It exists because the note's premise ("only half of each property is stated")
is *nearly* right, and the phase's whole shape depends on exactly how nearly.

**S1d.2.2** is the one that can sink the rest. A requirement quantifies over a
domain, so the engine has to know the domain's extent, whether it is closed,
and whether new objects may appear. Ein has `is-a` extents and the `unknown`
macro; the stdlib is deliberately **is-a-free in rule bodies** — the hierarchy
relation arrives as an activator parameter (`?isa`) — and an obligation has to
arrive the same way or it drags a type system into the kernel that
[S1.7.23](../../../docs/history/m1a_rust/README.md) said would not exist.

**S1d.2.3** decides whether an obligation is a *fact* (a derived marker the
rules read), a *kernel object* (tracked by the saturator alongside the fact
store), or a *rule shape* (something `forall` already almost expresses). Each
costs somewhere different: a fact costs matcher time, a kernel object costs
the port's data model, a rule shape costs nothing new and probably cannot
carry the candidate set.

**S1d.2.4** is the engine work: the candidate set per open obligation,
narrowed as negatives arrive, closed at one, refuted at zero, and reported at
quiescence. The invalidation problem is the one
[design/06](../../../docs/history/m1a_rust/design/06_saturation.md)'s boundary already has —
`_admit_from_boundary`'s re-query cost was 72 % of an exhaustive `zebra2` run
before P1a.6 — so an obligation index that has to be rebuilt at every
quiescence is a mechanism that pays for itself in the same coin.

**S1d.2.5** is the payoff and the risk: generating hypotheses from an
obligation's candidate set instead of from `alive`. Mutually exclusive,
jointly exhaustive branches — and a different traversal, therefore different
counters, therefore the decision
[Q-M1a.18](../../../docs/history/m1a_rust/open_questions.md#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint)
had to take before a fork was allowed to narrate less.

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
