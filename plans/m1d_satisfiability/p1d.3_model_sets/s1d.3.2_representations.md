# S1d.3.2 — Candidate representations, and what each costs to produce and to read

**Phase:** [P1d.3](README.md) (Model sets without enumeration)
**Estimate:** 2 days
**Depends on:** [S1d.3.1](s1d.3.1_what_the_models_differ_in.md) — the
factorisation measurement is what removes candidates from this list rather than
argument.

## Context

[`ideas.md`](../ideas.md) lists six ways to represent a set of models —
disjunctive constraints, sets of alternatives, BDD/ZDD, a SAT formula, a
decision graph with shared subtrees, projected model counting — plus a seventh
that is not a representation but a special case: *частичная модель с
`open`-фактами, если они независимы*. [S1d.3.1](s1d.3.1_what_the_models_differ_in.md)'s
reconnaissance kills the seventh on the phase's own case: the 23 varying
decision variables of `zebra2-minus-15` form **one** coupling component, so
there are no independent open facts to leave open.

**This stage prices what is left, and the pricing has four columns**, of which
the last is the one that usually decides:

| | the question | why it is not obvious |
|---|---|---|
| **produce** | does the search already have this, or must something be built? | the free ones are free *because* the lattice holds them; the rest cost a compilation pass |
| **size** | how big is it, against 32 models × 435 facts = 13 920 fact lines | a form larger than the enumeration is not a compact form |
| **exact** | can the model set be reconstructed from it? | an *envelope* — what is certain and what varies — is not a representation of which combinations occur |
| **read** | can the person who reads the trace read this? | [idea 08](../../ideas/08-human-style-deductive-trace.md) is Ein's differentiator, and the phase README says it plainly: *a BDD is the opposite of that* |

**The fourth column is a veto, not a weight.** The phase README:

> Anything shipped here has to be readable by the same person who reads the
> trace, or it belongs in a followup.

So a representation that is exact, small and unreadable does not win on
three-out-of-four; it loses, and the honest place to record it is a followup
with the measurement attached.

## The candidates

Ordered by how likely they are to survive, which is the reverse of how
sophisticated they are.

### (a) The certain core and the varying frontier

*"These 312 facts hold in every model; these 23 slots are undecided; here is
each slot's range."* Free — it is two set operations over the models the search
already recorded — and **78 % of every model on the reconnaissance**.

It is **not exact**: it is the smallest box containing the model set, and the
box has 9.95 × 10¹³ cells where the set has 32. A reader told *"Blue is in one
of five houses and Fox is in one of five"* will believe 25 combinations are
available when 32 models exist in total across 23 such slots.

That is a real hazard and it has a real fix: say which it is. The verdict
vocabulary [S1d.2.6](../p1d.2_obligations/s1d.2.6_verdicts_counters_corpus.md)
settled already distinguishes *what is established* from *what is not*, and a
frontier reported as **an over-approximation, labelled** is honest where the
same table presented as the answer is not. Whether that label is enough is
[S1d.3.3](s1d.3.3_the_verdict.md)'s to decide.

**This is the candidate to beat**, because it is free, readable, and continuous
with the milestone's own vocabulary: an `Open` state reports what it owes, and
an `Ambiguity` reporting what is undecided is the same sentence about a set.

### (b) A determining key and its table

*"Fix (Red, Japanese, Horse, Zebra) and everything else follows; here are the
32 admissible quadruples."* Exact, and the reconnaissance measured its
compression: **4 columns instead of 25, and still 32 rows** — 32 of the 320
combinations the four domains allow.

Cheap to produce *if the key is small* — the search of
[T1d.3.1.2](s1d.3.1_what_the_models_differ_in.md) already finds it — and the
cost is combinatorial in the key size, so a puzzle needing a nine-variable key
does not get one. Readable, arguably more so than 32 full models.

**The question this stage must answer about (b) is whether the key means
anything.** Twenty-two quadruples determine the set; nothing distinguishes
them; and a reader shown one will ask *why these four*. A key chosen for
minimality is an arbitrary basis, and presenting an arbitrary basis as the
structure of the answer is a different kind of dishonesty from (a)'s.

### (c) A decision diagram over the decision variables

BDD / ZDD / a decision graph with shared subtrees — `ideas.md`'s middle three,
which are one family for this purpose. Exact, and the only candidate that
compresses a *coupled* set without enumerating it.

Its cost is [T1d.3.1.3](s1d.3.1_what_the_models_differ_in.md)'s finding: a
diagram over a graph with a small separator is small, and over
`c/README.md`'s K₅-minus-two-edges it is not. **Price it against the measured
coupling structure rather than in general** — the literature's answer is "it
depends on the variable order", and the order is exactly what the measurement
constrains.

And it fails the fourth column outright. If (c) is the only exact small form,
the recommendation is a followup with the number, not a ship.

### (d) A disjunctive constraint store

*Saturation over symbolic constraints* — [`ideas.md`](../ideas.md)'s own
formulation of the compact answer, and the one that is not a post-processing
step but a different engine:

> **symbolic solution saturation** — fixed point над ограничениями,
> представляющими множество моделей.

**This is out of scope for a two-day stage and the stage must say so rather
than hand-wave it.** It is a second inference mode beside the one the milestone
has, it interacts with everything P1d.2 built, and pricing it needs a design
rather than a measurement. What this stage owes it is a paragraph in the
ledger: what it would buy, what it would cost, and the trip-wire that would
make it worth starting — which is the milestone's deferral discipline, and the
same treatment [P1d.2 gave forms E and A](../p1d.2_obligations/README.md).

### (e) Enumeration

The control arm, and the phase README names it as a legitimate winner:

> **The honest possible outcome is "enumerate, and say so"**: 32 models is 32
> lines, and a compact form that nobody can read is worse than a list.

It must be measured like the others, because "32 lines" is the optimistic
reading. Today `solve -e` on `zebra2-minus-15` prints **516 lines** and the
underlying model set is 13 920 facts; what a reader gets is already a summary,
and *which* summary is a decision this phase can take without any new
representation at all.

## Tasks

### Task T1d.3.2.1 — the four-column table, filled

Each candidate priced on produce / size / exact / read, against
[S1d.3.1](s1d.3.1_what_the_models_differ_in.md)'s numbers and not against
intuition. A candidate whose row cannot be filled from the census is a
candidate whose measurement is missing, and the missing measurement goes back
to S1d.3.1 rather than being estimated here.

### Task T1d.3.2.2 — (a) and (b) built as throwaway probes

Both are small enough to *write* rather than argue about, and both come out of
the census's JSON without touching the engine. Build them as
`utils/model_set_census.py --form envelope|key`, render the two forms on
`zebra2-minus-15`, and put the actual output in the record. **A representation
argued about in prose and never printed is a representation nobody has read**,
which is the failure mode the fourth column exists to catch.

### Task T1d.3.2.3 — the readability test, with a reader

The fourth column needs evidence and the evidence is not an assertion by the
person who built the form. The cheap version: render (a), (b) and (e) for
`zebra2-minus-15`, and for each ask three questions a reader actually has —
*is the Zebra's house determined? · what else would determine it? · how many
models are there?* — recording which forms answer each without arithmetic.

A form that cannot answer *"how many models"* without the reader multiplying
something is not compact, it is folded.

### Task T1d.3.2.4 — (c) priced against the measured structure

Not built. Priced: given the coupling components and whatever separator
structure T1d.3.1.3 found, what size would a diagram be, and under which
variable order? A bound with the measurement behind it is the deliverable, and
"unbounded on this shape" is an acceptable one.

### Task T1d.3.2.5 — (d) deferred, with its trip-wire

One paragraph, the milestone's deferral form: the specification that survives
the deferral and the evidence that would reverse it. The candidate trip-wire —
**a corpus entry whose model set cannot be enumerated at all**, which is not
`zebra2-minus-15` (it enumerates in 25 s at the depth that finds every model).

## Acceptance

- The four-column table is filled for (a)–(e), every cell traceable to a
  measurement in [`model_set_census.md`](model_set_census.md).
- **(a) and (b) exist as printed output**, on the phase's real case, in the
  record — not as descriptions of what they would look like.
- The readability test is run and its result recorded, including a negative:
  a form that failed it is named and kept in the record rather than dropped.
- (c) carries a size bound conditioned on the measured coupling, or a written
  reason no bound is available.
- (d) is deferred in the milestone's form, with the trip-wire stated as a
  property of a corpus entry rather than of a wish.
- **No engine change ships in this stage.** Everything here reads
  `--json-summary`; what ships, if anything, is
  [S1d.3.3](s1d.3.3_the_verdict.md)'s.
