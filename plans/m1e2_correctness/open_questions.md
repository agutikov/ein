# M1e2 — Open questions

Questions this milestone **raises**, as `Q-M1e2.<n>`. The M1e questions it
carries keep their `Q-M1e.<n>` ids and live in
[M1e's register](../../docs/history/m1e_review_processing/open_questions.md). An id is sticky
and is never reused.

Each of the first five is a **reading** a contested stage needs before it can
choose a fix. They are filed here so that the review
([S1e2.1.7](p1e2.1_seeing_the_answer/s1e2.1.7_the_review.md)) has one list to
rule on, not five stage files to find.

## Index

| Q | title | needed by | status |
|---|---|---|---|
| [Q-M1e2.1](#q-m1e21--is-a-discharged-superset-of-a-discharged-state-a-new-model) | Is a discharged superset of a discharged state a new model? | [S1e2.2.3](p1e2.2_wrong_answers/s1e2.2.3_obligations_join_superset.md) (W5) | open |
| [Q-M1e2.2](#q-m1e22--does-a-guards-absent-premise-bind-the-state-the-search-ends-in) | Does a guard's `absent` premise bind the state the search ends in? | [S1e2.2.6](p1e2.2_wrong_answers/s1e2.2.6_stale_naf_positive.md) (W8) | open |
| [Q-M1e2.3](#q-m1e23--what-does-exhausted-certify-when-the-rung-leaves-relations-uncovered) | What does `exhausted` certify when the rung leaves relations uncovered? | [S1e2.2.8](p1e2.2_wrong_answers/s1e2.2.8_uncovered_certified.md) (W6) | open |
| [Q-M1e2.4](#q-m1e24--is-an-empty-unsat-core-a-core) | Is an empty unsat core a core? | [S1e2.2.13](p1e2.2_wrong_answers/s1e2.2.13_empty_core.md) (W12) | open |
| [Q-M1e2.5](#q-m1e25--does-s1f108s-ruling-move-here) | Does S1f.10.8's ruling move here? | [S1e2.2.5](p1e2.2_wrong_answers/s1e2.2.5_naf_upward_closure.md) (W3) | open |
| [Q-M1e2.6](#q-m1e26--does-p1e23-become-a-phase) | Does P1e2.3 become a phase? | the milestone's scope | open |
| [Q-M1e2.7](#q-m1e27--the-answer-is-a-region-of-the-lattice) | The answer is a region of the lattice | [P1e2.1a](p1e2.1a_the_solution_region/README.md), all of it; then every case the review has not read | **reading given 2026-10-15**; its sub-questions open |

C1's reading is [Q-M1e.12](../../docs/history/m1e_review_processing/open_questions.md#q-m1e12--the-blind-rung-is-untyped-and-a-model-binds-a-type-as-an-object)'s
and is not re-filed.

---

## Q-M1e2.1 — Is a discharged superset of a discharged state a new model?

`examples/ein-bugs/wrong-answers/owed-seats.ein` (two guests, two chairs, `total-owed` +
`surjective-owed`) has two matchings. Is a state that seats Ann on **both**
chairs and Bob on one, owing nothing, a third model? *No* (**minimal**): k = 2,
the lattice's own design, where a complete node leaves the frontier. *Yes*
(**literal**, Q-M1e.6's *owes nothing*): k = 7. Today the lattice answers 3
and the tree 6, so both are wrong under both readings. The recommendation in
S1e2.2.3 is *minimal*. A requirement is discharged by a witness, and a second
witness is `functional`'s question, not `total`'s.

**Reframed by [Q-M1e2.7](#q-m1e27--the-answer-is-a-region-of-the-lattice)
(2026-10-15).** The two readings are two numbers of one region: *minimal*
is the size of R's minimal line (2, the matchings), *literal* is |R| (7),
and R's maximal line is one node, every guest on every chair. The question
becomes Q-M1e2.7's (a), which line `k` counts, and is re-asked when
[P1e2.1a](p1e2.1a_the_solution_region/README.md) has written the definitions.

## Q-M1e2.2 — Does a guard's `absent` premise bind the state the search ends in?

`features/07_unstratifiable.ein` records `{(p one), (q one)}`, where `(p one)`
was derived under `(absent (q one))`, and its own header says that set *"is not a
model of this program under any reading"*. Under an append-only reading of
Q-M1e.6 it qualifies. Three answers: **guards are premises** (a stale fact
disqualifies a state), **refuse what cannot be stratified** (a load error),
or **append-only is the semantics** (not wrong, and say so). The first two
compose.

## Q-M1e2.3 — What does `exhausted` certify when the rung leaves relations uncovered?

`completeness.md` §3c's fixture is `exhausted = true` with `knows` never
hypothesised, and `arity3.ein` likewise for an arity-3 goal. Relative to
`alive₀` it is right by Q-M1e.6's second clarification, and as a statement
about the program it is not. Three answers: **scoped, and the read-out says
the scope** (recommended), **uncertified unless covered**, or **open-world,
not wrong**.

## Q-M1e2.4 — Is an empty unsat core a core?

`alive-empty-phase1.ein`'s exhausted `Contradiction` is right and prints
`unsat core (0 facts)`, because its refutation rests on premise-less engine
writes. M1e's S1e.3.1 called it honest. S1e2.2.13 argues it is false: ∅ with
this program's rules is consistent, and the engine knows which deaths the
writes came from.

## Q-M1e2.5 — Does S1f.10.8's ruling move here?

W3 cannot be fixed until someone rules *forbid* or *respect* on a refutation
that rests on `(absent P)` over an extendable relation, and that ruling is
M1f's [S1f.10.8](../m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.8_refutation_under_absent.md).
The proposal is to run its T1 and its ruling as S1e2.2.5's first task, leaving
S1f.10.8 the corollary it writes into `absent_semantics.md`. The alternative
is that W3 waits for M1f.

## Q-M1e2.6 — Does P1e2.3 become a phase?

Nine surfaces that misstate a run whose answer is right, and one reachable
panic, are listed in the [milestone README](README.md#p1e23--surfaces-that-misreport-the-answer-proposed)
with witnesses. They are correctness issues and not wrong answers. Whether
they belong in this milestone, a later one or a followup is the user's call.

## Q-M1e2.7 — The answer is a region of the lattice

**A reading given 2026-10-15**, by the user, in the review of W1
([S1e2.1.7](p1e2.1_seeing_the_answer/s1e2.1.7_the_review.md)). It replaces
the question *which commitments are solutions* with *what shape is the set of
nodes that are*. Recorded verbatim:

> "maximal alive commitment" solution definition was formulated before we came
> to obligations mechanism; Basically between bottom bound defined by
> obligations and top bound defined by restrictions (emitted false,
> contradictions) also could be not only single possible model but also could
> be multiple models; […] Let's think about what does solution means on the
> search lattice (set-indexed DAG) - some early layers L0,L1,... with few
> hypothesis doesn't meet obligations and some hight layers L10,L100,L1000
> (that we never reach) produce conflicts; So clearly solution is a subset of
> lattice nodes on multiple layers that does meet obligations and doesn't
> produce conflicts; this is let's say problem solution topology - some of
> nodes are naturally combining like B+C and produce what I named before
> "maximal alive commitment". But in fact we have at first a whole solutions
> topology {empty, A, B, C, (B, C)} and an upper bound of this sub-lattice
> {A, (B, C)}; Deinitely 100% we can't throw out A because it's size is
> smaller than (B, C) if we threat "maximal commitment" literally - smaller
> number of facts. […] Main conclusion from W1 analysis - no solutions is very
> simple case - obligations and contradictions overlap leave no space for
> solutions; one solution is probably a sub-lattice with sinle bottom node,
> not just single valid node; while ambiguity is a sub-lattice maybe even
> partitioned, with multiple bottom nodes; […] So we probably can have not
> only "maximal" solutions, but also "minimal" solutions make sense, for
> example if we have obligations but no restrictions defined, with no
> restrictions the only maximal solution would be fully connected graph, but
> that is a trivial and we probably don't look for it and more than that it
> could be computationally difficult to enumerate all lattice nodes up to
> fully connected graph in the bottom; So our initial simple {no solutions,
> single solution, multiple solutios} transforms into solution sub-lattice
> and it's topology - bottom line minimal solutions and upper line maximal
> solutions. And also, obvious, value of config
> :enable-pre-branch-lookahead must not affect solution values, because no
> config option affects problem search lattice, config may only affect search
> speed.

The user draws the lattice with L1 at the top, so a *bottom node* in the
drawing is a large set. In the language of bounds, the **lower bound** is the
minimal solutions and the **upper bound** the maximal ones. The restatement
below uses *minimal line* and *maximal line* for the two, which reads the
same either way up.

### The reading, restated

A node of the lattice is a commitment `C` (a set of hypotheses). Saturate
root plus `C`, and the node is **dead** (it derives `(false)`), or alive and
**owing** (an obligation the program states is unwitnessed), or alive and
**discharged**. The region is

> **R = { C : C is alive and discharged }**

and it is the answer. Its **minimal line** is R's minimal elements, its
**maximal line** R's maximal ones.

- **Without holes.** Under monotone rules, *dead* is upward-closed (a
  contradiction stays) and so is *discharged* (a witness stays). So R has no
  holes: every node between two of its members is a member. R is then fully
  described by its two lines, which is why two lines are enough.
- **With holes.** `absent` breaks both closures. A superset of a dead node can
  be alive again (W3, Q-M1e.9), and a fact derived under an `absent` survives
  the commitment that falsifies its guard (W8). Those two cases become *R has
  holes*, and the two lines no longer describe it.
- **Which line carries the information** depends on the program. With
  restrictions only, the minimal line is ∅ and the maximal line is the
  answer. With obligations only, the maximal line is *everything* (the
  user's "fully connected graph") and the minimal line is the answer. With
  both, the lines meet. A well-posed puzzle has |R| = 1.
- **No solution** is R = ∅, and it has two kinds: no alive node at all (root
  is dead), `Contradiction`; or alive nodes that all owe, `Open` (the
  2026-08-25 ruling, `closure/03`).
- **Config never changes R.** No config option affects the problem's search
  lattice, so none may change the answer. That makes every lever-only register
  row genuinely wrong. It is stronger than the milestone's acceptance line,
  which only forbade a lever to change a *certified* answer. A **budget**
  (`-m`, `-E`) does not change R either. It stops the walk early, and the
  read-out must then say the answer is partial. That last sentence is the
  restatement's reading and is not the user's words.

### Measured

These numbers come from [`probes/region.py`](probes/region.py), a brute-force
instrument. It enumerates every subset of a program's candidate hypotheses,
adds it to the program as given facts, saturates with
`:enable-forced-positive` and `:enable-pre-branch-lookahead` off, and reads
the result off `ein saturate` and `--json-summary`'s `owes.root`. Taken
2026-10-15 at `caef67a`. Every row re-takes with one command.

| program | shape | \|R\| | minimal line | maximal line | engine today |
|---|---|---:|---|---|---|
| `branching/15` (W1) | restrictions only | 5 | ∅ | `{(p A)}`, `{(p B),(p C)}` | `Solution`, k = 1 |
| `lattice/02` (W1) | restrictions only | 7 | ∅ | the three pairs | k = 3 at default; `Contradiction` under `-L` |
| `forced-2-two` (W1) | restrictions, two firings | 1 | ∅ | ∅ | `Contradiction` |
| `owed-two-step` (W1) | both; nothing fits | 0 (∅ alive and owing 1) | — | — | `Contradiction` |
| `owed-seats` (W5) | obligations only | 7 | the two matchings | every guest on every chair | lattice k = 3, tree k = 6 |
| `owed-seats` + `(functional seats)` + `(injective seats)` | both | 2 | the two matchings | the same two | k = 2 ✓ |
| `ein-bugs/naf-upward-closure` (W3) | a restriction under `absent` | 3, **with a hole** | ∅ | `{(p A),(q A)}` | the model `{(q A)}` |

The last row is the first **measured hole**. `{(p A)}` is dead
(`(p A)` without `(q A)`), and it lies between ∅ and `{(p A),(q A)}`, which
are both in R. The engine answers `{(q A)}`, which is not on either line, so
under any reading of (a) the answer is wrong. That is W3.

The prototype's first run read obligations through `ein solve -m 0` instead,
and the commitment `{(seats Ann C1)}` on the last row came back owing
nothing. The search had written `(seats Bob C2)` at root as
`<forced-positive>`, plus two `<lookahead-dies-immediately>` negatives. That
fact follows from the obligation, not from any rule. So the oracle that
computes R has to classify by **saturation alone**, with no search-time root
writes. This is W2's mechanism seen from the other side: a forced positive
treats an obligation as if it were a restriction.

### What it settles

- **W1, all seven rows, confirmed** on 2026-10-15, with no split. In R's
  terms W1 is one defect, *a node on the maximal line is never recorded*,
  and layer 0 is the case where the maximal line is `{∅}`. The four lever rows are
  wrong by the config principle alone.
- [Q-M1e.6](../../docs/history/m1e_review_processing/open_questions.md#q-m1e6--what-is-a-solution-and-what-is-a-model)'s
  definition becomes a special case. Its maximality arm is the maximal line,
  right where no obligation is stated. Its *owes nothing* arm makes every
  node of R a solution, which is Q-M1e2.1's literal k = 7.
- [Q-M1e2.1](#q-m1e21--is-a-discharged-superset-of-a-discharged-state-a-new-model)
  becomes sub-question (a) below.

### Sub-questions, for [P1e2.1a](p1e2.1a_the_solution_region/README.md)'s first stage

- **(a) What does `k` count?** The minimal line, the maximal line, both
  (two counts), or the informative one by the program's shape? And which
  verdict word goes with which?
- **(b) How does a program say which line it asks for?** By its shape
  (obligations stated or not), or by a query keyword the language does not
  have yet?
- **(c) How does `:expect` state a region?** Today it states a model set,
  one line's worth.
- **(d) What is reported when R has holes?** Both lines no longer describe
  R. Is the answer then the list of R's maximal convex pieces, or a
  qualifier, or a refusal (Q-M1e2.2's *refuse what cannot be stratified*)?
- **(e) Commitments or states?** A node is a commitment, but a model is a
  state, and two commitments can saturate to one state. Q-M1e.6's
  *integrated* hypotheses are the same question. Is R deduplicated by state
  before its lines are taken?
- **(f) The trivial line.** The obligations-only maximal line can be
  exponentially far down, and the user called it trivial. Is it reported,
  named as trivial, or left out?
- **(g) What `exhausted` certifies.** The lines reported, not the walk
  ([Q-M1e.8](../../docs/history/m1e_review_processing/open_questions.md#q-m1e8--exhausted-certifies-the-lattice-not-the-model-set)).
- **(h) The universe.** R is over the generator's candidate list at root.
  W4 (a fresh constant never becomes a candidate) and W6 (relations never
  hypothesised) are cases where that list is too small. The oracle checks the
  walk against the list. Does R's definition also say what the list must
  contain? There is also a conflict with the config principle.
  [`solution_semantics.md` § 1](../../docs/kernel/inference/solution_semantics.md)
  defines `alive₀` *after the forced-positive cascade*, and the cascade is a
  config switch (`:enable-forced-positive`). So today the universe itself
  depends on config.

### The oracle's limit (ruled 2026-10-15)

The lattice has 2ⁿ nodes. The user's words: *"we can safely limit n with 16
by default and warn user simply if asked for 16<=n<=20 and show intensive
warning if user asks to go through lattice with n>=20"*. The two ranges
overlap at 16 and at 20, and the plan reads them as follows:

| n | behaviour |
|---|---|
| ≤ 16 | runs, silently |
| 17–19 | refused unless asked for; when asked, runs with a one-line warning |
| ≥ 20 | refused unless asked for; when asked, runs after a prominent warning that states 2ⁿ and the projected cost |
