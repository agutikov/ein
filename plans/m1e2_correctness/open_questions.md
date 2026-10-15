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
| [Q-M1e2.2](#q-m1e22--does-a-guards-absent-premise-bind-the-state-the-search-ends-in) | Does a guard's `absent` premise bind the state the search ends in? | [P1e2.1b](p1e2.1b_the_non_monotone_lattice/README.md)'s S1e2.1b.2, then [S1e2.2.6](p1e2.2_wrong_answers/s1e2.2.6_stale_naf_positive.md) (W8) | open |
| [Q-M1e2.3](#q-m1e23--what-does-exhausted-certify-when-the-rung-leaves-relations-uncovered) | What does `exhausted` certify when the rung leaves relations uncovered? | [S1e2.2.8](p1e2.2_wrong_answers/s1e2.2.8_uncovered_certified.md) (W6) | open |
| [Q-M1e2.4](#q-m1e24--is-an-empty-unsat-core-a-core) | Is an empty unsat core a core? | [S1e2.2.13](p1e2.2_wrong_answers/s1e2.2.13_empty_core.md) (W12) | open |
| [Q-M1e2.5](#q-m1e25--does-s1f108s-ruling-move-here) | Does S1f.10.8's ruling move here? | [S1e2.2.5](p1e2.2_wrong_answers/s1e2.2.5_naf_upward_closure.md) (W3) | open |
| [Q-M1e2.6](#q-m1e26--does-p1e23-become-a-phase) | Does P1e2.3 become a phase? | the milestone's scope | open |
| [Q-M1e2.7](#q-m1e27--the-answer-is-a-region-of-the-lattice) | The answer is a region of the lattice | [P1e2.1a](p1e2.1a_the_solution_region/README.md), all of it; [P1e2.1b](p1e2.1b_the_non_monotone_lattice/README.md) for (d); then every case the review has not read | **reading given 2026-10-15**; on 2026-10-16 (a)–(c) and (e)–(i) [ruled for the monotone case](#the-second-answer-2026-10-16), and [(e)'s display and (h)'s names](#the-third-answer-2026-10-16) |
| [Q-M1e2.8](#q-m1e28--where-does-the-rework-of-ein-solve-go) | Where does the rework of `ein solve` go? | P1e2.1a's S1e2.1a.5 (it must print `k` = \|R\|), and P1e2.2's scope | **ruled 2026-10-16**: [S1e2.1a.9](p1e2.1a_the_solution_region/s1e2.1a.9_solve_describes_the_region.md) now, [P1e2.1c](p1e2.1c_the_region_search/README.md) next |

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

**Answered by (a), 2026-10-16:** `k` = |R|, so every node of R counts, and a
discharged superset of a discharged state is a solution. On `owed-seats` that
is the literal **7**, the same in commitments and in states, since nothing
there saturates onto anything else. S1e2.2.3 is re-read against it.

## Q-M1e2.2 — Does a guard's `absent` premise bind the state the search ends in?

`features/07_unstratifiable.ein` records `{(p one), (q one)}`, where `(p one)`
was derived under `(absent (q one))`, and its own header says that set *"is not a
model of this program under any reading"*. Under an append-only reading of
Q-M1e.6 it qualifies. Three answers: **guards are premises** (a stale fact
disqualifies a state), **refuse what cannot be stratified** (a load error),
or **append-only is the semantics** (not wrong, and say so). The first two
compose.

**Asked of every node (2026-10-15).** The oracle has to build each node of
the cube, and the question is then not only about the state the search ends
in. Built on root's fixpoint (the engine's fork), `{(q one)}` on
`stale-naf-fork.ein` is dead. Built from scratch (`region.py`), it is in R.
[P1e2.1b](p1e2.1b_the_non_monotone_lattice/README.md)'s S1e2.1b.2 owns the
question in that form.

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
  *Corrected 2026-10-15:* a witness stays, but a new **debt** can arrive, so
  *discharged* is upward-closed only when no obligation's trigger can grow
  with the commitment. See [§ Holes without `absent`](#holes-without-absent-2026-10-15).
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

These numbers come from `probes/region.py` (since S1e2.1a.1 [`utils/region.py`](../../utils/region.py)), a brute-force
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

### The cube, and what replaces `k` (2026-10-15)

The user's follow-up, verbatim:

> well, so instead of single number k that may be k=0, k=1. k>1 we have at
> least 3 numbers: |R|, |top(R)|, |bottom(R)|; this is only if we don't have
> holes in R; So we could have both - not connected partitions of R and one
> partition could have holes; holes also have top and bollom bounds, what does
> they mean, do we need to count them, or only outer bound of R has meaning?
> One more notice, without merging lattice looks like hypercube, is it true?
> How we can use it?

**The lattice is the n-cube, exactly.** Without merging, the nodes are the
subsets of the n candidates ordered by inclusion, and the diagram is the
n-dimensional hypercube:
- a node is an n-bit word;
- an edge adds or removes one fact;
- layer k is the words with k bits set, C(n,k) of them.

Merging commitments that saturate to one state (sub-question (e)) gives a
different lattice, one node per distinct state, and that one is no longer a
cube.

**Without holes, two lines determine R, and |R| follows.** A set with no
holes is exactly every node between some minimal node and some maximal node
above it, so |R| is derived and says little on its own. What describes R is
its **sub-cubes**. An interval [a … b] is every set between a and b: the facts
in a are fixed in, those outside b are fixed out, and the rest are free. Its
**dimension** is |b| − |a|, the number of free facts. R is a union of maximal
sub-cubes, and |R| is their sizes by inclusion–exclusion (`branching/15`:
2 + 4 − 1 = 5).

**Components.** These are R's connected pieces under one-fact edges.
- They appear only when a program has both kinds of constraint. With
  restrictions only, R connects through ∅. With obligations only, it connects
  through the full set.
- Without holes, each component has its own two lines, and R's lines are their
  union.
- A component is not the count of solutions: `branching/15` has one
  component and two maximal nodes.
- **A well-posed puzzle is the case where every component is a single point**
  (dimension 0). There the component count, the minimal line and the maximal
  line all give the classical k.

**Holes.** A hole is a node outside R that lies between two members of R.
Under monotone rules there are none, because both *dead* and *discharged* are
preserved as facts are added. So every hole comes from `absent`.
*Corrected the same day: from `absent`, or from an obligation's trigger
([§ Holes without `absent`](#holes-without-absent-2026-10-15)).* A hole is a
failure that **adding a fact repairs**: in W3, `{(p A)}` dies only because
`(q A)` has not been said yet. It is a debt written as a restriction.
- **The outer lines are the answer.** Hole nodes are not solutions.
- **The hole count is a certificate.** Zero holes means the two lines describe
  R exactly, and that every learned nogood is sound.
- **A hole's minimal nodes are the useful diagnostic.** Each one is where a
  repairable failure starts, and its core names the `absent` guard responsible.
- **Its repair edges matter more than its maximal nodes.** A repair edge goes
  from a hole node up into R, labelled with the fact that revives it. In W3 it
  is `{(p A)} + (q A) → {(p A),(q A)}`.
- **Holes are where pruning is unsound.** `{(p A)}` is a minimal dead set with
  an alive superset. A learned nogood there prunes `{(p A),(q A)}`, which is a
  node of R. That is the whole of W3.

**The lines are decided by edges.** Without holes:
- a node of R is maximal exactly when every upward edge leads to a dead node;
- a node of R is minimal exactly when every downward edge leads to an owing
  node.

Both are local tests, and the search's layer n+1 already looks at those upward
edges. So **W1's fix is a local test on the cube**. Across a hole, a local test
and the true global answer can disagree.

**Two lists determine the answer when there are no holes.** Let N be the
minimal dead sets (the minimal nogoods) and E the minimal sets that discharge
every obligation. Then:
- the **minimal line** is the members of E that are alive;
- the **maximal line** is the largest sets containing no member of N, kept
  where discharged. Equivalently, it is the complements of the minimal sets
  that hit every member of N.

This is "bottom bound from obligations, top bound from restrictions", made
exact. It is the duality of Reiter's diagnosis theory and of MUS/MCS
enumeration (MARCO), and it is the route past n = 20: the lines can be
enumerated from N and E without visiting all 2ⁿ nodes. A dead state's debt is
not read today (`owes.root` says 0 on a dead root whatever it owes), so E
needs the oracle to record *dead* and *owing* as two independent bits.

**Symmetry.** Interchangeable constants, such as the two chairs, permute the
cube's coordinates, so R must map to itself under that permutation. An answer
that does not is a defect: W5's *chosen by name order* is one. An oracle can
also enumerate one node per symmetry class.

**Prior art: Poole's Theorist.** David Poole, *A logical framework for default
reasoning*, Artificial Intelligence 36(1), 1988
([doi:10.1016/0004-3702(88)90077-X](https://doi.org/10.1016/0004-3702(88)90077-X)).
The user's notes on it are `plans/followups/f19_pool_theorist/`. Theorist has
facts, a set H of possible hypotheses, and constraints.
- A **scenario** is a consistent set of hypotheses, and an **extension**
  follows from a maximal scenario. R's maximal line is the extensions that
  discharge every obligation.
- An **explanation** is a minimal scenario that entails the goal. R's
  minimal line is the explanations, with the obligations as the goal.
- Theorist's **skeptical / credulous** distinction maps onto a line:
  facts true at every node of it, or at some node. M1d's *certain core*
  ([representations.md](../../docs/history/m1d_satisfiability/representations.md))
  is the skeptical reading of a model set.

Measured with the merged `probes/region.py` (now [`utils/region.py`](../../utils/region.py)), same
programs and settings as the table above:

| program | components | maximal sub-cubes (dimension) | holes | minimal dead sets N | duality |
|---|---:|---|---|---|---|
| `branching/15` | 1 | [∅…{A}] 1, [∅…{B,C}] 2 | — | {A,B}, {A,C} | holds |
| `lattice/02` | 1 | [∅…pair] 2, three times | — | {a,b,c} | holds |
| `forced-2-two` | 1 | [∅…∅] 0 | — | {q one}, {q two} | holds |
| `owed-two-step` | 0 | — | — | {likes Ann Soup}, {likes Ann Stew} | holds (R empty) |
| `owed-seats` | 1 | [matching…all 4] 2, twice | — | — | holds |
| `owed-seats` + `functional` + `injective` | **2** | the two matchings, dimension 0 | — | the 4 clashing pairs | holds |
| `naf-upward-closure` (W3) | 1 | [∅…{q}] 1, [{q}…{p,q}] 1 | **{p}** | {p} | **fails**: the complement rule gives {q}, R's maximal line is {p,q} |

The duality holds on every program without holes and fails exactly on the
one with a hole.

### Holes without `absent` (2026-10-15)

Measured while writing the oracle. [`examples/lattice/04_obligation_trigger_hole.ein`](../../examples/lattice/04_obligation_trigger_hole.ein)
has no `absent` in any saturation rule, and `total-owed meal is-a` over a
`Diner` type that seating a guest derives:

```
$ python3 utils/region.py examples/lattice/04_obligation_trigger_hole.ein '(seats Ann C1)' '(meal Ann Soup)'
L0: ∅=ok
L1: {(seats Ann C1)}=owes 1  {(meal Ann Soup)}=ok
L2: {(seats Ann C1), (meal Ann Soup)}=ok
region: 3 nodes;  minimal: ∅;  maximal: {(seats Ann C1), (meal Ann Soup)};  holes: {(seats Ann C1)}
```

The owing node is a hole. An obligation is the negation of its witness, so
its **trigger** reads at the same polarity an `absent` does. A hole needs a
read that a commitment can change at non-monotone polarity. An `absent` in a
saturation rule is one such read, and an obligation's trigger is the other.
An `absent` over a relation no commitment reaches is neither, which covers
zebra2's spatial `absent`s and the stdlib's `is-a` scans. The classification,
its theorem and the detector are
[P1e2.1b § What non-monotone means](p1e2.1b_the_non_monotone_lattice/README.md#what-non-monotone-means-and-what-is-not)
and [S1e2.1a.1](p1e2.1a_the_solution_region/s1e2.1a.1_the_oracle.md).

**The non-monotone case moved to a phase of its own** the same day, on the
user's answer to the oracle's first question: *"move non-monotonic solution
question into phase p1e2.1b"*. Sub-question (d) moved with it.

### Sub-questions, for [P1e2.1a](p1e2.1a_the_solution_region/README.md)'s first stage

- **(a) What does `k` count?** The minimal line, the maximal line, both
  (two counts), or the informative one by the program's shape? And which
  verdict word goes with which? Since the follow-up above, `k` may be replaced
  by a descriptor:
  - the component count;
  - for each component, its two lines and its sub-cube dimensions;
  - the minimal holes.

  The question is then which of these the verdict line prints.
- **(b) How does a program say which line it asks for?** By its shape
  (obligations stated or not), or by a query keyword the language does not
  have yet?
- **(c) How does `:expect` state a region?** Today it states a model set,
  one line's worth.
- **(d) What is reported when R has holes?** *Moved to
  [P1e2.1b](p1e2.1b_the_non_monotone_lattice/README.md)'s S1e2.1b.3 on
  2026-10-15.* Both lines no longer describe
  R. Is the answer then the list of R's maximal convex pieces, or a
  qualifier, or a refusal (Q-M1e2.2's *refuse what cannot be stratified*)?
  The follow-up's recommendation: the outer lines stay the answer, and the
  read-out adds the minimal holes, each with the `absent` guard of its core
  and its repair edges. A hole is never counted as a solution.
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
- **(i) Components.** Is the answer reported per component? Is a component
  the unit an `Ambiguity` is about, given that in a well-posed puzzle the
  components are the models?

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

### The rulings, and the questions back (2026-10-16)

The user's answer to the sub-questions, verbatim. The part of the same message
that adds a stage to P1e2.1b is quoted in
[S1e2.1b.6](p1e2.1b_the_non_monotone_lattice/s1e2.1b.6_the_two_lattices_compared.md),
and the W11 sentence in
[S1e2.2.12](p1e2.2_wrong_answers/s1e2.2.12_core_id_order.md#the-verdict-moves-under-a-lever-2026-10-16).

> What `solve` could answer? for example "does alive and discharged solutions
> exist?" - it's enough to prove at least one; "is it the only single
> solution?"; "how many different solutions exist?" it is |R|; is
> minimal/maximal solution trivial? if not how many minimal/maximal solutions
> exist? sub-cube dimentions; number of components; enumerate all/min/max
> solutions; is the problem/solution/program monotonic? is it sinlge
> component? what else could `solve` answer about solution region topology?
> our default behaviour that searches for maximal solutions is not completely
> wrong but it is relevant only in special case with no obligations. So it has
> to be reworked.
>
> In general non-monotonic case R is completely defined only by full set of
> nodes. Is that correct? Let's go from it, and simplify the results if our
> case allows or if it is reqested explicitely. (a) Let's change the meaning of
> k now from |max(R)| to |R|. (b) first of all `ein solve` study the solution
> region topology, and quering some specific subset of like min line or max
> line is secondary, more than that query goal could become incompletely
> specified with some solution, also query is not the best place for hrules -
> need to split solution-related features like hrules from queries, we solve
> once and then queries answer questions about region and about specific
> solutions; Does it make sense? (c) :expect has to be the answer for query,
> new updated query, and another expect has to be a full expected region
> definition, maybe (solve :hrules ...) and (solution (or (model ...) (model
> ...) ...)) and (query :goal ... :expect <only facts from goal, query
> response, multiple models, deduplicated, maybe different models still
> produce identical goals>) ; (d) moved (e) probably we need this information
> in full solution that some commitments saturate to the same state, what does
> it mean in practice? we can easily saturate but yet can't so easily split,
> and don't know if smaller commitments are solutions if saturated is, so
> commitments give more information, but we need also saturation equvalence
> information, propose a way to expose all of it; (f) obligations only max
> line is a set of all hypothesis, that is why I called it trivial; is that
> true? (g) exhaused gives all found information about region, if limited by
> levels or iterations or time or anything - then information is not full, but
> if not limited then it is full information about region; region is a
> certificate of the lattice, right? (h) it is limitations of our current Ein
> - we don't emit new names, only combine existing, that guarantees lattice is
> finit; emitting new names/atoms will make everything even more complex, we
> can't handle it yet, and we don't know if we actually need it; If the
> hyothesis list is too small (too small for what? for producing expected
> solution?) - then ein program is wrong, or ein solve has a bug, isn't it? Or
> I didn't understand the question then explain better. I really forgot what
> "forced positives" are? (i) No, definitely components can't be models.
> Models are models. Components are non-intersecting subsets of models, e.g.
> {A} {B,C}; e.g. 2 groups of friends want to sit at the table, and all
> friends wants to sit at the same table, so if we have 2 tables we have 2
> components correspondign to group-table, and inside each component
> permutations of seat-friend; it can't be model, but query can ask e.g. only
> what table selected, and then seats will be hidden information; 'Ambiguity'
> is still relevant when we expect a one and only one solution.
> 'Contradiction' is relevant when we expect some solution. In other cases, by
> default, we are exploring, studying the solution region.

#### Where each sub-question stands

| | the user's answer | status | goes to |
|---|---|---|---|
| **general** | R is, in general, its full node set; start there and simplify where the case allows or a query asks | **ruled**. The plan's answer to *"is that correct?"* is below | S1e2.1a.3's page; [S1e2.1b.6](p1e2.1b_the_non_monotone_lattice/s1e2.1b.6_the_two_lattices_compared.md)'s ladder |
| **(a)** | `k` = \|R\|, not \|max R\| | **ruled**, and its unit too (second answer): `k` is replaced by **k_s** (states) and **k_c** (commitments), both reported | S1e2.1a.5, S1e2.1a.9 |
| **(b)** | `ein solve` studies the region first. Asking for one line is secondary. `hrules` move out of `(query …)`. Solve once, then queries ask about the region and about single solutions | **ruled**. The plan's answer to *"does it make sense?"* is below | S1e2.1a.4; [Q-M1e2.8](#q-m1e28--where-does-the-rework-of-ein-solve-go) for the search |
| **(c)** | three forms: `(solve :hrules …)`; `(solution (or (model …) …))`, the full expected region; `(query :goal … :expect …)`, the goal's projection, deduplicated | **ruled** as the direction. The grammar is S1e2.1a.4's page diff | S1e2.1a.4 |
| **(d)** | moved | P1e2.1b | S1e2.1b.3 |
| **(e)** | asks for a way to expose saturation equivalence alongside commitments | **ruled**: k_s and k_c (second answer); `(region …)` with `(merged (to from) …)` (third answer) | S1e2.1a.1's record; S1e2.1a.4, S1e2.1a.5 |
| **(f)** | asked: *is the obligations-only maximal line the whole candidate set?* | **ruled for P1e2.1a's scope** (second answer): yes, for a monotone program, and it is trivial. The non-monotone counterexample below is S1e2.1b.6's | S1e2.1a.3's page |
| **(g)** | `exhausted` means complete information about R; a limited run gives partial information; *"region is a certificate of the lattice, right?"* | **ruled for P1e2.1a's scope** (second answer): exhausted = the full region; a limit makes it partial | S1e2.1a.5, S1e2.1a.9 |
| **(h)** | no new names, so the lattice is finite; asks what *too small* means; asks what forced positives are | **ruled for P1e2.1a's scope** (second answer): the universe is the candidate list, and a list too small for a model is a program error or an engine bug. ∅ is a solution exactly when root is alive and discharged, whatever the generator proposes | S1e2.1a.3; W4, W6 |
| **(i)** | a component is a set of models, never a model. `Ambiguity` judges an expectation of exactly one solution and `Contradiction` an expectation of some. Without an expectation the read-out explores | **ruled**, and the user's grouping is named **projection classes** (second answer) | S1e2.1a.5; [`definitions.md`](p1e2.1a_the_solution_region/definitions.md) |

#### Measured for these answers

Taken 2026-10-16 at `777d224a`, by running every `.ein` under `examples/`,
`tests/` and `corpus/` (242 files) through `ein solve FILE --dump-region DIR`,
with a 60 s timeout each. 154 regions were computed, 29 were refused by the
limit, and 59 files produced none (they failed to load or timed out).
T1e2.1a.3.1's census re-takes all of it.

| | regions |
|---|---:|
| R non-empty | 118 |
| **∅ ∈ R**: no obligation that root owes | **107** |
| exactly one state in R | 68 |
| exactly one maximal node | 103 |
| … of which R holds more than one state | **35** |
| R counted in commitments ≠ R counted in states | **14** |
| flagged non-monotone by the static check | 13 |

Of the 14 where the two counts differ, the largest is
`features/02_star_in_identifiers.ein`: 32 768 commitments and 728 states. On
`branching/05_mini_zebra.ein` it is 8 and 5.

#### The plan's answers

**General: is R, in general, its full node set?** Yes.

- A non-monotone R can be any subset of the cube. S1e2.1a.8's synthesizer
  writes a program for any target (2 100 random targets, 0 mismatches), except
  a dead ∅ under a non-empty R.
- There are 2^(2ⁿ) possible regions, so no shorter description exists in
  general.
- *Full set* still has an exact compact form, the union of R's maximal
  sub-cubes, which the oracle already computes.

The simplifications follow in order, each licensed by one property:

1. **Convex** (no holes): the two lines are exact.
2. **Monotone** (the static check): the lines follow from N and E without
   enumerating, and a certificate is N + E.
3. **Shape**: restrictions only gives min = {∅}; obligations only gives
   max = {⊤}.

S1e2.1b.6 writes this ladder down.

**(a) × (e): what `k` = |R| counts.** The two measured facts above bear on it.

- **Commitments against states.** On 14 corpus regions, R counted in
  commitments is larger than R counted in states, up to 45× (32 768 against
  728). A puzzle that states its completeness as obligations (`bijective`,
  `total`) is the extreme case. Its unique model is reached by every
  commitment that saturates to it, so R in commitments counts the ways to
  *guess* the model, not models. This is argued, not measured: zebra2's cube,
  with n = 56, is out of reach. **The plan recommends k = the number of states
  in R, with the commitment count printed beside it.**
- **Programs that state no obligation.** ∅ is in R on 107 of the 118 non-empty
  regions. In a program with restrictions only, every consistent partial
  commitment is a solution, as the user's W1 topology `{∅, A, B, C, (B,C)}`
  has it. So k = |R| is large there even when the maximal line is a single
  node: 35 regions have one maximal node and several states. For example,
  `syntax/constraint-scopes.ein` has 4 096 states. This is what (a) means and
  is not a defect. It does mean that *"is it the only single solution?"* has
  two forms:
  - |R| = 1, for a program that states what it owes;
  - |max R| = 1, for one that does not.

  An expectation of exactly one solution, (i), has to say which. The
  alternative is that a puzzle states its completeness as obligations, and
  then R collapses to its models.

**(b) Does it make sense?** Yes.

- It matches what the engine already depends on. The lattice is fixed by the
  candidate list, which `hrules` select, and a query's goal is a projection of
  an answer. Today `:hrules` sits on `(query …)`: 19 corpus files put it there
  (outside comments, 2026-10-16). In principle, two queries could then define
  two lattices. The split makes *one program, one lattice, one R* structural.
- *"incompletely specified"* is real. In a program with restrictions only, the
  goal has no row at ∅. Today's read-out prints that case as *(no query goal to
  project)* (inventory S6). A query over R therefore has to say *undetermined
  in these solutions*, or be asked over a named subset, which (b) makes
  secondary.

**(c)** Two notes for S1e2.1a.4's grammar.

- `(solution …)` listing every model is exact, but it explodes: 728 states on
  `features/02`. It should also accept region shapes:
  - a sub-cube, `[a … b]`;
  - the two lines, with a `:convex` claim that the runner checks.
- A `(model …)` inside `(solution …)` should name a **state**. *Naming a
  relation closes it* carries over unchanged.

The query's `:expect` over the goal's projection, deduplicated, is the same
notion as (i)'s *what the table is* below.

**(e) A proposal.** For a monotone program, saturation is a closure operator
on the cube:

- **extensive**: C ⊆ cl(C);
- **idempotent**: cl(cl(C)) = cl(C). Measured: 0 of 32 074 moving alive nodes
  land on another state (S1e2.1a.2);
- **monotone**: C ⊆ D ⇒ cl(C) ⊆ cl(D). This follows from the theorem.
  S1e2.1b.6 adds the test.

So each state s is a class `{C : cl(C) = s}`:

- **its top is s itself**, the closed set;
- **its bottoms are its minimal generators**: the smallest commitments that
  force s;
- **C ∈ R ⇔ cl(C) ∈ R.** This is the easy direction the user named: whether a
  smaller commitment is a solution takes one saturation.
- **The hard direction, *split*, is the minimal generators.** They are what
  has to be guessed to reach s. The size of the smallest one is the number of
  hypotheses the solution needs, which is the branch depth
  [`zebra_walkthrough.md`](../../docs/kernel/inference/zebra_walkthrough.md)
  tabulates.

**What to expose:**

- per state in R: its closed set, its minimal generators, and the size of its
  class;
- `k` counted in states, with \|R\| in commitments beside it;
- the merged diagram: the closed sets ordered by inclusion, as an `ein render
  region` view. Closed sets are closed under intersection, so it is a lattice.
  This is formal concept analysis's closed sets and minimal generators (Ganter
  & Wille, 1999).

Today the oracle keys a class by its numerically smallest mask. The proposal
keys it by its closed set, which is unique. For a non-monotone program the
classes still exist, but cl need not be monotone. That cell belongs to
S1e2.1b.6.

**(f) Yes, for a monotone program.** If nothing dies and *discharged* is
upward-closed, the full candidate set ⊤ is in R whenever R is non-empty, so the
maximal line is {⊤}. `owed-seats` (⊤ = every guest on every chair) and
`examples/lattice/synth/trivial_line.ein` show it.

**It is not true when an obligation's trigger can grow.** A probe written with
`utils/region_synth.py` on 2026-10-16 (target: candidates `ABC`; R = `[A … AB]`;
everything else owing; nothing dead) passes `--check`. Its maximal line is
`{A,B}`, and ⊤ = `{A,B,C}` owes, because `C` triggers a debt that nothing
pays. The static check flags it.

The dual also holds: with restrictions only, the minimal line is {∅}. Both
trivial lines are visible statically from the program's shape. The plan
recommends naming them in one word and never enumerating or chasing them.

**(g) Yes, with one correction to today's word.**

- *Today* `exhausted = true` certifies the **walk**, not R
  ([Q-M1e.8](../../docs/history/m1e_review_processing/open_questions.md#q-m1e8--exhausted-certifies-the-lattice-not-the-model-set)).
  W1 is a run that is `exhausted` and still misses a node of R.
- Under the ruling, `exhausted = true` means *the reported region is R*.
- A limited run reports what it found as a lower bound, and every node in it
  is checked.

*"Region is a certificate of the lattice"* reads the other way round in the
plan's terms. **R is the answer.** A *certificate* is what lets a reader check
a claimed R without recomputing it:

- **monotone**: small. It is N and E, one saturation per member, plus the
  static verdict;
- **non-monotone**: in general nothing smaller than every node's
  classification.

**(h) The question, explained.**

- **The universe is the candidate list.** R is defined over the candidates the
  generator offers at root, and **too small** means *too small to contain
  every model of the program*: a fact some model needs is not a candidate, so
  that model is not a node at all. There are two ways today:
  - **W4.** A rule asserts a constant the ontology lacks.
    `examples/ein-bugs/alive-set-fresh-name.ein` is eleven lines on which that
    costs a verdict. Under the user's rule that Ein emits no new names, that
    program is **ill-formed**. The consequence the plan draws is that the
    alive-set invariant, today a `warn` line, becomes a load error.
    *Overruled by the [third answer](#the-third-answer-2026-10-16):* rules are
    part of the program, so the name is not new, and W4 is an engine bug.
  - **W6.** A relation the program could have in a model is never
    hypothesised, such as `knows` in `completeness.md` §3c. Whether that is a
    bug depends on what the list is *supposed* to hold.

  **The ruling sought:** the universe is defined declaratively, as every
  type-correct ground atom of every hypothesisable relation over the declared
  objects. The generator's list is checkable against it, and a shortfall is an
  **engine bug**. A program that needs a name outside it is a **program
  error**.
- **The second half of (h): the universe must not depend on config.**
  `solution_semantics.md` § 1 takes `alive₀` *after* the forced-positive
  cascade. The oracle already uses the config-free list. The ruling sought is
  that the page defines the universe that way too.

**Forced positives.** When exactly one candidate hypothesis is still alive at
root, because every other one has been refuted, the engine promotes it into
root as a **fact**. Its provenance is `<forced-positive>` with no premises.
Then root is re-saturated, and the step repeats while one candidate remains.
The code is `solve.rs`'s `promote_forced_positives`, behind
`:enable-forced-positive`, which is on by default. The reasoning behind it is
*"every model extending root must contain it"*. That holds only if ∅ is not
itself a solution. W2 is the case where ∅ is the solution: on `forced-2`, ∅ is
the whole of R, the cascade promotes the one candidate, root dies, and the
engine answers `Contradiction`. Under the config principle, a forced positive
can be at most a search-time shortcut. It can never define the universe or the
answer.

**(i) Agreed, with one clash to resolve.** The oracle computes a component as
a set of nodes of R connected by one-fact edges. Under that definition, the
two-tables example does **not** give one component per table.

- With seating that is `functional` and `injective`, every full seating is an
  isolated node. Swapping two guests passes through a node with a guest in two
  seats, which is dead, or a node with a guest in none, which owes. So each
  seating is a component of its own.
- That is measured: `owed-seats` with `functional` and `injective` has
  **2 components**, the two matchings, each of dimension 0, as does
  `synth/one_model_per_component`.

The grouping the user describes, *models that agree on the table*, is the
partition a **query's goal** induces: the user's *"seats will be hidden
information"*, and (c)'s deduplicated projection. **The plan recommends** that
*component* keep its topological meaning, and that the user's notion be named
**projection classes**: a query's answers, each class being the models that
give one goal row set. A query answers with projection classes. Components
describe R's shape.

### The second answer (2026-10-16)

The user's reply to the plan's answers, verbatim:

> "when exactly one candidate is still alive at root, the engine promotes it
> to a root fact and re-saturates" - how does it differ from handling other
> previous candidates? engine also take them and add to root and saturate.
> What the differense for the last one alive? Empty set is a solution if
> sat(root) is already discharged and alive, it doesn't depends on hypgen at
> all. More than that - there could be no hypothesis at all, e.g. hrules
> doesn't match anything. But root could be already a solution. Trivial but
> valid case. (i) Provide me a definition table, in a file, what is fact, kb,
> state, model, commitment, solution, region, component, ... etc. Yes, what I
> described is a projection classes. […] (h) what is "alive-set invariant" ?
> In W11 (rule fire-0 (?P) ...) ?P variable not used. We are now at p1e2.1a,
> where solve will be changed and expect will be changed, and k changes it's
> meaning, but you keep asking about W11 k=0 vs k=2; It doesn't make sense
> now. We will revisit W11 twice after the refactoring. Once at
> plans/m1e2_correctness/p1e2.1_seeing_the_answer/s1e2.1.7_the_review.md and
> again at plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.12_core_id_order.md;
> k unit - k_states and k_commitments are different numbers, we need both,
> replace k with k_s and k_c; (f) (g) (h) - Again, we are in p1e2.1a - we
> solve monotone problems here, will solve non-monotone in next phase;
> Q-M1e2.8 - yes

and, in the same turn:

> change solve behaviour; the default simple `solve` without flags we get
> exhausive certifying full region description; while with some flags can
> simplify the task - e.g. look for any solution, or expect there is a single
> solution - so k>=2 is a contradiction; or look for only maximal solutions,
> or only for minimal solutions

The elided part asks for the seating example, which is banked as
[`examples/lattice/06_friends_tables.ein`](../../examples/lattice/06_friends_tables.ein),
with [`07_two_pairs_two_tables.ein`](../../examples/lattice/07_two_pairs_two_tables.ein)
beside it.

What it rules:

| | ruling | where it lands |
|---|---|---|
| **∅** | ∅ is a solution exactly when `sat(root)` is alive and discharged. That does not depend on the generator, and it holds when the generator proposes nothing | `definitions.md`; S1e2.1a.3's page; S1e2.1a.9 |
| **(a), (e)** | `k` is replaced by **k_s** (\|R\| in states) and **k_c** (\|R\| in commitments), both reported | S1e2.1a.5, S1e2.1a.9 |
| **(f), (g), (h)** | answered for the **monotone** case, which is P1e2.1a's scope. The non-monotone halves are P1e2.1b's | S1e2.1a.3; S1e2.1b.6 |
| **(i)** | the user's grouping is **projection classes**, and a definitions table exists: [`definitions.md`](p1e2.1a_the_solution_region/definitions.md) | S1e2.1a.4, S1e2.1a.5 |
| **W11** | not a P1e2.1a question. It is revisited twice after the refactoring: in [S1e2.1.7](p1e2.1_seeing_the_answer/s1e2.1.7_the_review.md)'s review, then in [S1e2.2.12](p1e2.2_wrong_answers/s1e2.2.12_core_id_order.md) | those two stages |
| **Q-M1e2.8** | yes: S1e2.1a.9 now, P1e2.1c next | [Q-M1e2.8](#q-m1e28--where-does-the-rework-of-ein-solve-go) |
| **`solve`'s default** | `ein solve` with no flags gives an **exhaustive, certifying, full description of R**. Flags make the task simpler: **any** solution; **expect a single solution**, so that k ≥ 2 contradicts the expectation; **maximal** solutions only; **minimal** solutions only | S1e2.1a.9 |

**The forced positive, answered.** An ordinary candidate is **entered**. The
search forks root, writes the candidate into the fork, saturates, and judges
that one node. Root does not change, and the node is one of many.

A forced positive is written **into root itself**, as a fact with no premises,
and root is re-saturated. From then on every node holds it, and the node
without it, ∅, is gone from the walk.

The *last alive* condition is the warrant: *if a solution must add a
candidate and all but one are refuted, every solution holds that one.* That is
sound only when ∅ is not itself a solution, which is the user's point. Probes
taken 2026-10-16:

- **No candidate at all.** The `hrule` matches nothing. `solve` answers
  `Solution`, model ∅, so the trivial case is handled.
- **Two alive candidates** that exclude each other. The engine answers
  `Ambiguity`, k = 2. R is `{∅, {(p A)}, {(p B)}}`, and ∅ is missing.
- **One alive candidate**, the other dead alone. The cascade promotes `(p A)`
  into root, and the engine answers `Solution` `{(p A)}`. R is
  `{∅, {(p A)}}`, and ∅ is missing.

So ∅ is recorded only when the generator proposes nothing. That follows from
today's **complete**, *the active rung proposes nothing*, which is a property
of the generator. Under the ruling it is replaced by **discharged**, a
property of the state. W1's layer-0 row is the same defect.

**The alive-set invariant (h).** It is the M1 kernel's assumption that rules
assert no new objects, no new relations, and that hypotheses connect only
existing names. From that, the alive set, and with it the candidate list, is a
function of root alone. Three mechanisms rely on it: the per-KB recompute of
`alive`, the `state_key` dedup behind `k`, and the tree traversal's
exhaustiveness. Since M1e S1e.3.3 `ein_infer::invariant` checks its static
half at load: every constant a rule's `:assert` names must be in the program's
facts. A breach is a `warn` line, not an error.
`examples/ein-bugs/alive-set-fresh-name.ein` is the eleven-line breach that
costs a verdict. *Corrected by the third answer:* rules are part of the
program, so the name that rule asserts is a program name. The program is
well-formed, and the missing candidates are an engine bug (W4).

### The third answer (2026-10-16)

The user, verbatim:

> in examples/lattice/06_friends_tables.ein if group doesn't fit - group
> doesn't sit (e.g. go to another restraunt); So not total-owed, but local
> group owed; WHY we still need to write someting into the root, when those
> forced positives arrive??? Alive-set invariant question - rules are part of
> root, so rules can't assert new names. "display which commitments saturate
> to the same state" - yes, full region info shows all valid commitments and
> all states that have multiple generating commitments with minimal ones;
> something like (region (model ...) (model ...) ...) (merged (to from) (to
> from)))

| | ruling | the plan's note | where it lands |
|---|---|---|---|
| **root is not the search's to write** | the user's question: why write into root at all when a forced positive arrives? | **There is no need.** Root is `sat(program)`, and ∅ *is* root, a node like any other. In a monotone program, if every candidate but h is dead alone, every node holding another candidate is dead, so the alive nodes are at most ∅ and {h}. Two saturations then decide the whole region, and nothing has to be written. Promoting h into root decides nothing more, loses ∅, and makes the outcome depend on a config switch (`:enable-forced-positive`, and the lookahead that decides what *sole survivor* means). **Proposed: root is immutable. The search keeps what it learns in its own state.** The search's negative writes into root fall under the same principle, and they are parked with W11 | S1e2.1a.9 (route 1 writes nothing into root); [S1e2.2.2](p1e2.2_wrong_answers/s1e2.2.2_forced_positive_refutes.md) (W2); P1e2.1c's S1e2.1c.5 |
| **(h), the alive-set invariant** | rules are part of the program, so a name a rule asserts is a name of the program, never a new one | This **corrects the plan's second answer**, which called `alive-set-fresh-name.ein` ill-formed. It is well-formed. `Z` is a program name, the universe has to include it, and the missing candidates are an **engine bug**. That is Q-M1e.21's option (b): seed the names from the rules' `:assert` constants | [S1e2.2.4](p1e2.2_wrong_answers/s1e2.2.4_fresh_name_unreachable.md) (W4); `definitions.md` |
| **(e), how a state's class is shown** | the full region lists **every valid commitment**, and **every state with several generating commitments, with its minimal ones**. The sketch is `(region (model …) (model …) …)` and `(merged (to from) (to from) …)` | For a monotone program the sketch is **exact and compact**. A state's class is the union of the intervals [from … to] over its minimal generators: if g ⊆ C ⊆ s and cl(g) = s, then s = cl(g) ⊆ cl(C) ⊆ cl(s) = s. So `(region …)` listing the states and `(merged …)` listing their minimal generators names every valid commitment, and k_c follows by inclusion–exclusion. **Proposed:** the read-out prints this block, and the language reads it back as an expectation. A run's region can then be banked as its own claim | S1e2.1a.4, S1e2.1a.5 |
| **06's obligation** | a group that does not fit goes elsewhere. The obligation is the group's, not `total-owed` | **The program is then not monotone, by construction.** *Once a member sits, the others owe a seat* is an obligation whose trigger grows with the commitment, the second source of holes. The cube: R is 21 nodes (∅; the pair alone, 8; the trio alone, 4; both, 8), and **166 holes**, every partly seated group. The engine records 8. The natural way to write a conditional requirement makes trigger holes, so they are P1e2.1b's common case, not an exotic one. The everyone-sits version is kept as the monotone twin | [`06_friends_tables.ein`](../../examples/lattice/06_friends_tables.ein); S1e2.1b.6 |

### What `solve` can answer about R (2026-10-16)

The user's questions, then the plan's additions (marked +). The cost column
says what an answer needs:

- **witness**: one node and its saturation;
- **lines**: enumerating a line, via N and E when the program is monotone;
- **R**: all of R, or its sub-cube cover;
- **static**: the program alone.

| question | answered by | cost |
|---|---|---|
| does R have a node? | one node of R, checked | witness |
| is it the only solution? | \|R\| = 1, or \|max R\| = 1 (see (a)); a witness plus a proof of no other | lines |
| how many solutions? | `k` = \|R\| (in states and in commitments, (e)) | R, or the sub-cubes by inclusion–exclusion when convex |
| is the minimal / maximal line trivial? | min = {∅}, max = {⊤}, from the program's shape | static |
| how many minimal / maximal solutions, and which? | \|min R\|, \|max R\|, enumerated | lines |
| how many components; is it one? | components under one-fact edges | R |
| sub-cube dimensions | each maximal sub-cube's free facts | R |
| enumerate all / min / max | the node set, or one line | R / lines |
| is the program monotone? | the static check's sites, and the violations the cube observes | static, then R |
| + if R is empty, which kind and why? | no alive node: a core. Every alive node owes: the debts no alive extension pays | witness per kind |
| + does it have holes? | hole count, minimal holes with their guard and repair edges | R |
| + which commitments give one state? | per state: its closed set and minimal generators (e) | R |
| + how much must be guessed? | the smallest generator's size per state | R |
| + which facts hold in every solution, in some, in none? | backbone, credulous and impossible sets: Theorist's skeptical and credulous readings | lines, for a monotone program |
| + which facts decide the others? | per-fact counts; M1d's determining key (`--models key`) | R |
| + what does a query's goal see? | projection classes (i), deduplicated (c) | R |
| + which rule shapes which part of R? | per minimal nogood its rule chain; per minimal owing node its obligations | lines |
| + which rules or facts can be dropped without changing R? | redundancy: R recomputed without each | R per rule |
| + what would make R a single state? | the smallest set of facts or rules whose addition leaves one state, and per candidate fact how far it splits R | R |
| + is R symmetric? | R mapped onto itself under interchangeable constants. An answer that is not is a defect (W5) | R |
| + how far apart are the solutions? | the minimum Hamming distance between states | R |
| + is the answer complete? | `exhausted` (g), and the certificate | — |
| + what is the universe? | the candidate list, and where it differs from `alive₀` (h) | static |

The questions marked **R** are the ones the user named for improving a theory
and for searching for one. They are why the region is the object of study and
not its first node. Which of them `ein solve` answers by default, and which
behind a flag, is S1e2.1a.5's to propose and the user's to rule.

## Q-M1e2.8 — Where does the rework of `ein solve` go?

The user, 2026-10-16: *"our default behaviour that searches for maximal
solutions is not completely wrong but it is relevant only in special case with
no obligations. So it has to be reworked"*. And under (b): *"first of all `ein
solve` study the solution region topology"*.

[P1e2.1a](p1e2.1a_the_solution_region/README.md) does not make the search find
R ([§ Where this phase stops](p1e2.1a_the_solution_region/README.md#where-this-phase-stops-and-p1e22-starts)),
and [P1e2.2](p1e2.2_wrong_answers/README.md) fixes the walk one case at a
time. Neither is a rework. There is also a constraint: under (a),
S1e2.1a.5 has to report `k` = |R|, and the walk cannot count R. It records
complete nodes and does not expand them.

The options:

1. **In P1e2.1a, now: the oracle becomes `solve`'s answer wherever it
   reaches.** This would be a new stage, S1e2.1a.9.
   - At n ≤ the limit, `ein solve` computes R exactly. That is the general
     case, stable and proved by construction. The corpus's 150 cubes take
     20 s in total (S1e2.1a.1).
   - Above the limit, the walk runs, and the read-out says the region is
     partial.
   - Before this can go into the gate, its cost in `cargo test`'s corpus sweep
     has to be measured.
2. **A phase of its own, P1e2.1c, after P1e2.1b and before P1e2.2:** the region
   search built from S1e2.1b.6's strategies (dualisation of N and E for a
   monotone program, the guard split otherwise). P1e2.2 is then re-scoped
   against it, because W1, W2 and W5 are symptoms of walking for the maximal
   line and may close by construction.
3. **After M1e2.** P1e2.2 fixes the walk as planned, and the rework is a
   milestone of its own.

**The plan recommends 1 now and 2 next.** Option 1 is what lets S1e2.1a.5
print the ruled `k` truthfully on everything the oracle reaches. Option 2
decides whether P1e2.2's 27.5 days are spent on a walk that is about to be
replaced.

**Ruled 2026-10-16: yes, 1 now and 2 next.** The same turn ruled what the
default is: *"the default simple `solve` without flags we get exhausive
certifying full region description"*, with flags that simplify the task.
Option 1 is [S1e2.1a.9](p1e2.1a_the_solution_region/s1e2.1a.9_solve_describes_the_region.md),
and option 2 is [P1e2.1c](p1e2.1c_the_region_search/README.md).
