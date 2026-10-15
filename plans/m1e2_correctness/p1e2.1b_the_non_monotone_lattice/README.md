# P1e2.1b — The non-monotone lattice: what R is when adding a fact can take one away

**Estimate:** not yet. Five stages are named and none is specified. They are
specified when [S1e2.1a.1](../p1e2.1a_the_solution_region/s1e2.1a.1_the_oracle.md)'s
detector has measured where the corpus is non-monotone, because the size of
every stage here depends on that number.
**Status:** created 2026-10-15 on the user's instruction, given while
answering S1e2.1a.1's open questions:

> I propose to move non-monotonic solution question into phase p1e2.1b; in
> this phase we have to 1) complete handling of monotonic lattice 2)
> understand if any `absent` cause non-monotonicity or only some specific
> cases, clossify them OR move this investigation to p1e2.1b; 3) implement
> detection of non-monotonicity and add a stub for handling this case in
> `solve` and other tools

"This phase" in the quote is [P1e2.1a](../p1e2.1a_the_solution_region/README.md).
Points 1–3 are S1e2.1a.1's, and this phase gets what they leave.
**Depends on:** S1e2.1a.1. Its detector gives this phase its census, and its
stub is what S1e2.1b.5 replaces.
**Blocks:** [S1e2.2.5](../p1e2.2_wrong_answers/s1e2.2.5_naf_upward_closure.md)
(W3) and [S1e2.2.6](../p1e2.2_wrong_answers/s1e2.2.6_stale_naf_positive.md)
(W8), which take their reading from here. Also the review of any case whose
program the detector flags.

---

## Why a phase of its own

[Q-M1e2.7](../open_questions.md#q-m1e27--the-answer-is-a-region-of-the-lattice)
describes the answer as a region R with two lines, and two lines describe R
only when R has no holes. A hole needs a node that is outside R and lies
between two nodes inside it. That happens only when adding a fact can take a
conclusion away. Every argument P1e2.1a makes about R — the lines are local
tests on edges, the N/E duality, a learned nogood is sound — rests on that not
happening. So P1e2.1a finishes the monotone case and **detects** the other
one. This phase decides what the other one means.

The two questions S1e2.1a.1 could not settle without a ruling are both about
the non-monotone case:

- **A node's state, fork or scratch.** The engine builds a node from root's
  fixpoint with the commitment written in. `utils/region.py` builds it from
  the program with the commitment written in as given facts. On a monotone
  program the two agree, which is the theorem below. On
  [`stale-naf-fork.ein`](../../../examples/ein-bugs/wrong-answers/stale-naf-fork.ein)
  (W8) they do not: forked from root, `{(q one)}` is dead, because root
  already holds `(p one)`, which was derived under `(absent (q one))`. From
  scratch, `{(q one)}` is in R. This is
  [Q-M1e2.2](../open_questions.md#q-m1e22--does-a-guards-absent-premise-bind-the-state-the-search-ends-in)
  asked of every node rather than of the final one.
- **What is reported when R has holes**, Q-M1e2.7's sub-question (d), moved
  here.

## What non-monotone means, and what is not

Measured 2026-10-15, and the reason the phase is not *"`absent`"*. A hole needs
no `absent` in any saturation rule. On this eleven-line program
([`examples/lattice/04_obligation_trigger_hole.ein`](../../../examples/lattice/04_obligation_trigger_hole.ein),
banked as a corpus fixture by S1e2.1a.1):

```lisp
(import std.algebra :symbols (total-owed))
(relation is-a  T T)
(relation seats Guest Chair)
(relation meal  Diner Food)
(is-a Ann Guest) (is-a C1 Chair) (is-a Soup Food)
(rule seated-dines () :match (seats ?g ?c) :assert (is-a ?g Diner) :priority 200)
(total-owed meal is-a)
(hrule seat () :match (and (is-a ?g Guest) (is-a ?c Chair)) :assert (seats ?g ?c))
(hrule dine () :match (and (is-a ?g Guest) (is-a ?f Food))  :assert (meal ?g ?f))
(query :goal (meal ?g ?f))
```

`region.py … '(seats Ann C1)' '(meal Ann Soup)'` gives ∅ in R, `{(seats Ann
C1)}` **owing 1**, and both facts together in R. The owing node is a hole.
The two hrules make the generator's root list exactly those two facts. The
engine answers `Solution`, k = 1, `{(meal Ann Soup)}`, which is on neither
line.
Seating Ann derives `(is-a Ann Diner)`, and that is a debt `total-owed`
did not have at ∅. A rule that states an obligation is a negation of its
witness, so its **trigger** is non-monotone in the same sense an `absent` is.

So the classification S1e2.1a.1 implements is about where a read sits, and
`absent` is one of two places it can sit. Let *H* be the relations of the
candidate list, and *D* the relations a commitment can change: H, plus
everything a compiled plan asserts once it reads a relation already in D.
Every relation a plan reads is then one of:

| class | what it is | example | can it make a hole |
|---|---|---|---|
| **static** | not in D. Complete at root, the same at every node | zebra2's `adjacent-via-*` `absent`s over `right-of` / `next-to`; the stdlib's `is-a` scans when `is-a` is never hypothesised | no |
| **dynamic, monotone polarity** | in D, read where growth can only add conclusions or remove debts | `total`'s `forall … (not (R …))` (even depth); `total-owed`'s witness `absent` | no |
| **dynamic, non-monotone polarity** | in D, read where growth can remove a conclusion or add a debt | an `absent` at odd depth in a saturation rule (W3's `bad`, W8's `derive-p`); an obligation's trigger (the probe above) | **yes** |

**The theorem the detector rests on.** Suppose a program has no read of the
third kind. Then every relation in D grows with the commitment, static
relations do not move, `(false)` is upward-closed, and every debt shrinks as
the commitment grows. So R has no holes and is every node between its two
lines. That is a sufficient condition, not a necessary one. A third-kind read
can be harmless in practice, and the cube says whether it was: S1e2.1a.1
reports both the sites and the violations the cube observed.

## Stages

| ID | title | the question |
|---|---|---|
| S1e2.1b.1 | The site census | where in the corpus the third-kind reads are, by rule family and by polarity, and on how many entries the cube observes a violation (a dead node with an alive superset, a discharged node with an owing superset, a hole) |
| S1e2.1b.2 | A node's state, fork or scratch | Q-M1e2.2 for every node: which route R is defined over, and what the other route is then. W8 |
| S1e2.1b.3 | Holes | Q-M1e2.7's (d): what is reported when the lines do not describe R, for both sources, `absent` and obligation triggers |
| S1e2.1b.4 | A refutation that rests on an `absent` | whether a nogood, the kill cache or the singleton writeback may learn from such a refutation. [Q-M1e.9](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e9--is-dead-really-upward-closed-under-absent), W3, and [Q-M1e2.5](../open_questions.md#q-m1e25--does-s1f108s-ruling-move-here) (S1f.10.8's ruling) |
| S1e2.1b.5 | The handling | S1e2.1a.1's stub replaced in `solve`, `ein test`, `--dump-region` and the bundle: refuse, qualify, or search differently, as ruled |

Each stage gets a file once S1e2.1b.1 has a census. Until then the table is
the scope.

## What it inherits

| from | here |
|---|---|
| [Q-M1e2.7](../open_questions.md#q-m1e27--the-answer-is-a-region-of-the-lattice) (d), holes | S1e2.1b.3 |
| [Q-M1e2.2](../open_questions.md#q-m1e22--does-a-guards-absent-premise-bind-the-state-the-search-ends-in), W8 | S1e2.1b.2 |
| [Q-M1e.9](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e9--is-dead-really-upward-closed-under-absent), [Q-M1e2.5](../open_questions.md#q-m1e25--does-s1f108s-ruling-move-here), W3 | S1e2.1b.4 |
| `naf_deps.rs`'s `RefutationUnderAbsentWarning` (M1e S1e.2.3), the containment | S1e2.1b.4, which also decides whether S1e2.1a.1's detector replaces it. It is the third-kind subset whose conclusion is a refutation and whose relation is eligible directly, not through D |
| S1e2.1a.1's `non_monotone` stub | S1e2.1b.5 |

## Where it stops

P1e2.1a defines, computes, draws and reports R for **monotone** programs, and
detects the rest. This phase defines R for the rest. Neither makes the search
find R. That is still [P1e2.2](../p1e2.2_wrong_answers/README.md)'s, and
S1e2.2.5 and S1e2.2.6 are its stages that wait for this one.
