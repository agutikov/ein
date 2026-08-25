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
(open …)`, the per-node verdict tally), the naming is **P3** with the probe
rename already executed (`7e1192c` — the third-state macro is `unknown`),
bounds are **numeral-free** (pairings to reference extents, never `L`/`U`
operands), and **obligations supersede `:hrules`** when a query carries none.
The six stage files below are written against those decisions; what each
still leaves open is stated in the file, not here.

**Revised 2026-08-25**, one decision: the atom's **argument** is the
relation — **`(open ?R)`**, with the domain scan and the witness slot
projected out of the rule's own `absent` — superseding the `forall`-dual
triple. [S1d.2.3](s1d.2.3_the_form.md) item 3 is the record and
[`obligation_forms.md` § The slot spelling, resolved](obligation_forms.md)
is the argument.

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

| stage | title | est. | status |
|---|---|---|---|
| [S1d.2.1](s1d.2.1_property_audit.md) | What each property enforces today, rule by rule | 3 d | **done 2026-08-25** |
| [S1d.2.2](s1d.2.2_domains.md) | Domains: what a requirement quantifies over, and what closes it | 3 d | **done 2026-08-25** |
| [S1d.2.3](s1d.2.3_the_form.md) | The obligation — form, surface, and where it lives | 3 d | **done 2026-08-25** |
| [S1d.2.4](s1d.2.4_obligations_in_the_saturator.md) | Obligations in the saturator | 4 d | **done 2026-08-25** |
| [S1d.2.5](s1d.2.5_hypotheses_from_obligations.md) | Hypotheses from obligations | 3 d | **done 2026-08-25** |
| [S1d.2.6](s1d.2.6_verdicts_counters_corpus.md) | What it changes: verdicts, counters, corpus | 2 d | **done 2026-08-25** |

**S1d.2.1 is done** (2026-08-25) — [`property_audit.md`](property_audit.md),
73 rules classified from the parsed rule text and joined to two census runs.
Its headline settles the phase's premise as a number: **the `≥` half has
fifteen rules and no middle** — five refute at zero candidates, five force at
one, four generate a witness unique by construction, one refutes
open-world-naively, and **none records anything at two or more**. The note's
premise was that the lower bounds are missing; the truer statement is that
every one of them is about a candidate set of size 0 or 1. Four findings, of
which the last moves a downstream acceptance: `std.elim`'s positional markers
are premises nothing checks (F1), `connex` is the one `≥` refutation that is
not extension-safe (F2), `std.elim` has no range side and the bijective
pair's endpoints live in different modules (F3), and an obligation rule
**cannot be parameter-less** — a variable relation head must come from the
activator — so the fan-outs grow by two activator facts per declaration and
13 entries gain 50 stored facts (F4).

**S1d.2.2 is done** (2026-08-25) — [`domain_contract.md`](domain_contract.md),
four clauses. Stating and discharging need no closure at all: the domain is
the obligation's own guard, `?isa`-parameterised and is-a-free (no kernel
type system, [S1.7.23](../../../docs/history/m1a_rust/README.md) holds), and
discharge is a positive check, which is monotone. Refutation keeps the
`forall` scans, `connex`'s caveat named. **The clause the plan did not
have is C4**: a *branch* over an obligation's candidates is jointly
exhaustive only where the guard's scanned relation is not itself guessable —
measured as **12 of the 49 searching entries propose `is-a` arrows, all 12
blind, none hrule-driven** — so the rung branches on the closed side and
declines on the other.

Two findings. **"An open domain" was the wrong name**: no entry has an
infinite or growing domain, and `04_open`'s 14.3 GB is the subset lattice
over an unbounded *hypothesis space*, `C(81, 5)`. And **obligations decline
that regime rather than rescuing it** — `04_open` and the three
`square-unique` demos are among the twelve. Where the rung does branch, the
win is the size: 3 candidates against 81 arrows. The closed-and-owing corner
is two checked-in fixtures where deleting one fact leaves the verdict
`Solution` either way; "leave it to the scans" turns out not to be
conservative but wrong, and the rule is to report it with an *unreachable*
flag and leave the promotion to S1d.2.6.

**S1d.2.3** was the decision stage; the decision was taken 2026-08-24 —
**G, a rule shape with a reserved verdict atom** — and its *argument* was
revised 2026-08-25 to **`(open ?R)`**: the atom names the relation whose
extent is incomplete and nothing else, and the engine projects the domain
scan and the witness slot out of the rule's own `absent`, statically, per
activator. That supersedes the `forall`-dual triple `(open ?b G B)`, which
restated the guard in the head — where it could disagree with it, and where
its bound variable needed an exception to a normative diagnostic. The bare
`(open)` stays as the degenerate, and the two now nest: `(open)` counts and
reports, `(open ?R)` also attributes and branches. Its file records what was
decided against (B's carrier fact, C's head, D's numeric sugar, E now, F
entirely, the triple) and implements what remains: reserving `open`, loading
and round-tripping the two forms, resolving the projection, inert until the
next stage.

**S1d.2.4 is done** (2026-08-25) — the report stratum, whole. Obligation
rules run as **one pass over the quiescent KB, after saturation completes**
(the user, 2026-08-25 — not a priority band inside the loop, since a band
orders selection within it and openness has to be read after it), tallied on
the lattice node, never stored, reported through `--events`' new `owe` kind,
`--json-summary`'s `owes` block and the trace's *Outstanding obligations*
section. `std.algebra` gained `total-owed` / `surjective-owed` and `std.slots`
`slot-owed-room` / `slot-owed-fill`, fanned out by the two setup rules, and
nine programs went into `tests/stdlib/` — a firing and a satisfied case per
rule, plus the one that leaves the blind enumerator on. The fixtures' owe
claims are checked by a Rust sibling of `stdlib_coverage.rs`, because
`:expect` asserts about facts and an `open` verdict is not one; no verdict
word moved.

**Every number the stage was asked for came back the number the plan
predicted**: `zebra2-minus-15` owes **46** at root (§5's hand census, plus the
per-relation split 10/8/8/10/10 the hand census did not have), the fact store
grew by exactly **50 facts over 13 entries** (the audit's F4 table term for
term), **no verdict moved on any corpus entry** — `corpus_exits.txt` gained 45
rows and modified none — and `zebra -e` / `zebra2 -e` came in at 43.3 / 27.5
ms against P1a.6's 47.5 / 29.0, the pass measuring +0.7 / +0.6 ms in an A/B
against a stdlib with the fan-out lines removed, inside the run-to-run spread.
The cost lever T1d.2.4.6 held in reserve was not needed; what made it cheap is
that a **dead node's debts are never consulted** — the read-out checks
`(false)` first — which is 67 of an exhaustive `zebra2`'s 101 enterings.

**The stage's own finding is a defect it introduced two commits earlier.**
S1d.2.3's registry split had leaked into `Program::categorise`, which read
`self.rules` alone — so an obligation rule's *name* categorised as an object,
`hypgen::candidate_objects` kept it, and the blind enumerator proposed `(seats
total-owed C1)`: the name of a rule as a puzzle value, 3 502 of 6 231
proposals on the one fixture that leaves the enumerator on. The acceptance
bullet demanding *a fixture, not an argument* is what caught it, and
`tests/stdlib/bijection/06_blind_enumeration.ein` exists because the argument
was not good enough.

**S1d.2.6** carries a scope rule decided 2026-08-25: **a program that states
no obligation keeps today's verdict.** Only 23 of the corpus's 173 `.ein`
files declare a property with a lower bound, so without it the vacuous edge
("owes 0 and consistent ⇒ *satisfy*") would move the word on 150 programs
that never asked to be judged by discharge.

**S1d.2.5 is done** (2026-08-25) — the payoff, and the re-baseline it was the
risk of came back **empty**. The **supersession ladder** ships (`:hrules`
override → obligations → blind), with the fixture the rung needed:
[`examples/zebra2-obligations.ein`](../../../examples/zebra2-obligations.ein)
is `zebra2.ein` with the `(hrule guess …)` and its `:hrules` clause deleted and
nothing else — generated by the same script that makes the other three
variants, so "nothing else changed" is checked rather than claimed — and it
**solves to the zebra2 model with no hypothesis rule in the file**. The idea
note's complaint about `:hrules` ("while it is not part of the theory") is
closed as a fixture: what replaced the hrule is `(bijective color-loc)`.

**The measurement is the phase's central claim, and it has two halves**
([`hypotheses_from_obligations.md`](hypotheses_from_obligations.md)). Against
the blind enumerator on the same file, layer 1 is **56 candidates against
3 734** — 66.7×, and the blind arm does not finish. Against the hand-written
hrule it replaces, **not one counter moves**: 101 enterings, 34 alive, 67
dead-post, 2 layers, 67 no-goods, one model, the same fact set — and on the
under-determined twin at full depth, **618 076 enterings, 19 121 clauses and
the same 32 models**, which is the layer census's baseline reproduced exactly.
So
[Q-M1d.4](../open_questions.md#q-m1d4--may-an-obligation-driven-generator-change-the-traversal)
closes at *it may, and here it does not*: the licence to move counters was
granted and not spent.

**Two deviations from the plan, both argued in the record.** The rung proposes
the **union** of the accepted obligations' candidates rather than one chosen
obligation's — branching on one is a *depth-first* move and this engine's
search is a breadth-first lattice over root's `alive`, where a model needing a
second requirement's arrow would be unreachable at every depth. And a declined
obligation declines the **whole call** rather than falling through to another,
because per-obligation fall-through loses completeness silently. The choice
heuristic (fail-first vs rule order) is built and **measured inert** — 0
difference on every counter — with the reason recorded: the traversal re-sorts
every layer canonically, so no order the generator imposes survives. It is
kept because it is the interface a depth-first traversal needs on day one.

**And the stage's own finding**: the pre-existing corpus reaches the rung's
generating mode **nowhere** — 114 of 156 loadable files declare no obligation
rule, 19 override with `:hrules`, 11 are *stuck* (they owe, and
`:no-hypothesis` names what they owe), 9 owe nothing, 1 declines. That is why
`tests/stdlib/bijection/06_blind_enumeration.ein` had to grow a second
obligation: under the ladder a program declaring `(bijective …)` no longer
reaches the blind enumerator at all, and the file's whole check had gone
vacuous.

## The phase ledger — P1d.2 closed 2026-08-25

Six stages, one day, and the thing it set out to build exists: **a puzzle can
state a requirement, a state can say what it owes, the search can branch on
it, and the verdict can report it.** What follows is the record T1d.2.6.5
asked for — decisions and where they were taken, what landed, what was
deferred and what would un-defer it.

### The decisions, and who took them

| decision | taken | where |
|---|---|---|
| a requirement is a **rule shape** asserting a reserved verdict atom (form G), not a kernel primitive and not a stdlib convention | user, 2026-08-24 | [Q-M1d.2](../open_questions.md#q-m1d2--where-does-a-requirement-live), [S1d.2.3](s1d.2.3_the_form.md) |
| the atom is **`(open ?R)`** — it names the relation and projects the rest out of the guard; a bare `(open)` counts and attributes nothing | user, 2026-08-25 (*"why does `open` take arguments when `(false)` does not?"*) | [obligation_forms.md § The slot spelling](obligation_forms.md) |
| obligation rules run **after** the fixpoint, as one pass, never in the saturation agenda | user, 2026-08-25 | [obligation_forms.md § When the obligation rules run](obligation_forms.md) |
| obligations **supersede** `:hrules` as the generator, as a ladder | user, 2026-08-24 | [Q-M1d.4](../open_questions.md#q-m1d4--may-an-obligation-driven-generator-change-the-traversal), [S1d.2.5](s1d.2.5_hypotheses_from_obligations.md) |
| the verdict word is **`Open — owes n`**, from the `open` / `false` / `satisfy` triple | user | [S1d.2.6](s1d.2.6_verdicts_counters_corpus.md) |
| the read-out is **scoped**: judged by discharge where an obligation was stated, by exhaustion where none was | 2026-08-25 | [S1d.2.6](s1d.2.6_verdicts_counters_corpus.md), [census §3](openness_census.md) |
| closed-and-owing reports **`Open`, not `(false)`** | user, 2026-08-25 | [census §6](openness_census.md), [domain_contract.md §3](domain_contract.md) |
| the rung proposes the **union** of owed instances, not one chosen obligation's | 2026-08-25, argued from the traversal | [the record §1](hypotheses_from_obligations.md) |
| a declined obligation declines the **whole call** | 2026-08-25 | [the record §1](hypotheses_from_obligations.md) |

### What landed

| stage | what it is | the number |
|---|---|---|
| [S1d.2.1](s1d.2.1_property_audit.md) | the property audit | the `≥` half has **fifteen rules and no middle** |
| [S1d.2.2](s1d.2.2_domains.md) | the domain contract, C1–C4 | **12 of 49** searching entries propose `is-a` arrows, which is where C4 declines |
| [S1d.2.3](s1d.2.3_the_form.md) | `open` reserved — loads, resolves, stays inert | **nine** refusals with no Python counterpart |
| [S1d.2.4](s1d.2.4_obligations_in_the_saturator.md) | the report stratum | `zebra2-minus-15` owes **46**, split 10/8/8/10/10 — the hand census reproduced |
| [S1d.2.5](s1d.2.5_hypotheses_from_obligations.md) | the generator rung | **56 against 3 734** at layer 1, and **not one counter moved** against the hrule |
| [S1d.2.6](s1d.2.6_verdicts_counters_corpus.md) | the verdict | **12** words moved, **0** exits, **0** counters, **92** entries out of scope |

Two censuses are the phase's evidence and both are re-takable:
[`openness_census.md`](openness_census.md) (what the corpus owes) and
[`hypotheses_from_obligations.md`](hypotheses_from_obligations.md) (what the
ladder cost), joined by [`property_audit.md`](property_audit.md) and
[`domain_contract.md`](domain_contract.md), which are arguments rather than
measurements.

Three questions closed — [Q-M1d.2](../open_questions.md#q-m1d2--where-does-a-requirement-live),
[Q-M1d.3](../open_questions.md#q-m1d3--what-closes-a-domain) (for obligations),
[Q-M1d.4](../open_questions.md#q-m1d4--may-an-obligation-driven-generator-change-the-traversal),
[Q-M1d.6](../open_questions.md#q-m1d6--may-contradiction-be-said-with-exhausted--false)
— which is four.

### What was deferred, and what un-defers it

The milestone's rule: *a deferral is cheap to reverse only while the
specification survives it.* So each of these has a written form and a
trip-wire, not just an absence.

| deferred | the specification that survives it | what un-defers it |
|---|---|---|
| **form E** — the obligation as a first-class *object* rather than a rule shape | [`obligation_forms.md`](obligation_forms.md)'s full menu, E included | a requirement that must be **named** to be referred to — a rule that reasons about another rule's debt |
| **form A** — a kernel `(require …)` primitive | same | a requirement no rule shape can express; none has appeared |
| **numeric bounds** — `L ≤ #{ȳ ∣ R(x̄,ȳ) ∧ φ} ≤ U`, and the `odd` / `same-count-as` / `at-least-one-of` family | [`ideas.md`](../ideas.md) § Положительные обязательства, and the phase's Risks § Expressive creep | **a corpus entry that cannot be stated without it.** Every entry today needs only `≥ 1`, which is what `total` / `surjective` / the slot pair express |
| **the compound witness** — two positive `?R` steps bearing free variables, *"there must exist a 2-chain"* | [`obligation_forms.md` § the projection is static](obligation_forms.md) | it is *refused* at load rather than mis-compiled, so the trip-wire is a user hitting the refusal |
| **one chosen obligation per node** (a depth-first branch) | [the record §1](hypotheses_from_obligations.md), with the choice heuristic built and **measured inert** | a depth-first traversal — [P1d.10](../p1d.10_exhaustive_search/README.md)'s, which inherits the interface ready |
| **`:expect` for an open verdict** | [S1c.1.2](../../../docs/history/m1c_external_validation/README.md#s1c12--how-a-program-states-what-it-expects)'s three fact-shaped forms, and why an `open` conclusion is not a fact | a fixture that needs to *claim* openness rather than assert its facts; twelve moved word without one, so none has |
| **promotion of closed-and-owing to `(false)`** | [`domain_contract.md` §3](domain_contract.md) and [census §6](openness_census.md) | closed-world completion, which is [P1d.3](../p1d.3_model_sets/README.md)'s — the inference that would license it |
| **the per-state leftover-open count** | [the record §6](hypotheses_from_obligations.md), including why the probe would measure a different engine | P1d.3's compact model sets, where it becomes meaningful |

### What the phase did not touch

`exhausted` still means the lattice
([Q-M1d.1](../open_questions.md#q-m1d1--may-the-search-stop-before-the-lattice-is-exhausted)),
the ten Q-M1d.6 entries still report `Contradiction` at `exhausted = false`
because they state no obligation, and **no counter, cost or traversal moved
anywhere in the corpus.** `zebra -e` is inside
[P1a.6](../../../docs/history/m1a_rust/README.md)'s 47.5 ms baseline. The
phase is additive to every program that did not opt in, which is the property
form G was chosen for and the one it kept.

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
- ~~**Every existing verdict is unchanged**, on every corpus entry.~~ Counters
  may move; the stage that moves them says which and re-baselines with an
  argument. **Held through S1d.2.5 and spent at S1d.2.6, as designed** — that
  stage exists to move a word, and it moved **twelve**, every one from
  `Solution` to `Open` and every one a program that *states* an obligation.
  What replaces the bullet is the **scope rule**, which is the same promise
  made where it can still be kept: *a program that states no obligation keeps
  today's verdict*, measured at 92 of the 121 entries that reach a fixpoint
  ([openness_census.md §3](openness_census.md)). Counters did not move at all:
  the read-out changed and the search did not.
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
