# The candidate representations, priced and printed

**Stage:** [S1d.3.2](s1d.3.2_representations.md) · **Phase:** [P1d.3](README.md)
**Taken:** 2026-08-25, on `examples/zebra2-minus-15.ein` at `-m 3` — the depth
that finds all 32 models.
**Instrument:** [`utils/model_set_census.py --form
{envelope,key,list,diagram}`](../../../utils/model_set_census.py), which
renders a model set as one of the candidates. No engine change: every form is
a rendering of the `verdict.solutions` [the census](model_set_census.md)
already reads.
**Re-take:** `utils/model_set_census.py --form key -k zebra2-minus-15.ein`
(each form ≈ 27 s, of which 26.5 s is the solve).

[S1d.3.2](s1d.3.2_representations.md) prices five ways to answer *"there are
32 models"* on four columns — **produce · size · exact · read** — of which the
fourth is a **veto** rather than a weight. This is the pricing, and the two
forms the stage said had to be *printed rather than argued about* are printed
below in full.

**The headline is a reversal.** The stage called (a), the certain core and the
varying frontier, *"the candidate to beat"* — free, readable, continuous with
the milestone's vocabulary. It loses. (b), the determining key, wins on every
column including the two the stage doubted: its exactness is now **verified
operationally** — 32 of 32 key rows reconstruct their model to the fact — and
the *"why these four"* objection has an answer the census supplied.

---

## 1. The four columns, filled

Every cell traceable to [`model_set_census.md`](model_set_census.md) or to §§3–7
below. Sizes are of the **actual rendered output** on `zebra2-minus-15`;
*produce* excludes the solve, which every form shares.

| | produce | size | exact | read |
|---|---|---|---|---|
| **(a) envelope** | free — two set operations, **4 ms** | 48 lines · 2 889 B · 85 cols | **no** — the box has 9.95 × 10¹³ cells for 32 models | **fails** (§6) |
| **(b) key** | **13 ms** — a minimum hitting set over C(23,4) | **44 lines · 2 506 B** · 72 cols | **yes, verified** — 32/32 rows reconstruct the model exactly (§4.1) | **passes** — the only form that answers all three questions |
| **(c) diagram** | 604 ms to *price*; building one is more | 355 nodes + 385 edges best order; 27 + 55 over the key | yes | **fails** — a graph, and the fourth column is a veto |
| **(d) disjunctive store** | **not a rendering** — a second inference mode | unmeasured | yes, by construction | unmeasured |
| **(e) list**, same alphabet | free — 3 ms | 39 lines · 13 208 B · **396 cols** | yes | passes with work; does not fit a page |
| **(e′) what `solve -e` prints today** | free — it is the run | **516 lines** · 86 cols | yes, as query bindings | — |
| the model set as facts | — | **13 920 fact lines** | yes | — |

Three readings.

**The two smallest forms are (b) and (a), and they are within 15 % of each
other** — 2 506 B against 2 889 B. So the choice between them is not a size
choice at all; it is entirely the *exact* and *read* columns, which is why the
stage was right to make the fourth a veto and wrong about which candidate it
would veto. (b) is also the only form that fits **72 columns**; (a) needs 85
and (e) 396.

**(e) is not big, it is wide.** 39 lines against (b)'s 44 — but 396 columns,
because 23 slots × "House-*n*" is 396 characters and nothing folds it. What
`solve -e` prints instead is 516 lines at 86 columns, i.e. the engine already
chose to summarise rather than to print the model set, and *which* summary it
prints is a decision available with no new representation at all.

**No form is expensive to produce.** The whole pricing exercise costs less than
a second on the phase's own case, and 26.5 s of the 27 s a re-take takes is the
solve. Production cost is not a discriminator here and would only become one at
a *k* this corpus does not have.

---

## 2. What was dropped before pricing

[`ideas.md`](../ideas.md)'s seventh form — *частичная модель с `open`-фактами,
если они независимы* — is not in the table because
[the census](model_set_census.md) killed it: the 23 varying decision variables
of `zebra2-minus-15` are **one** coupling component, so there are no
independent open facts to leave open. It survives on exactly two corpus
entries, `saturation/type-exclusivity/{colors,nationalities}`, whose two-object
model sets partition 3 × 3 — and dies on the same program with three objects.

---

## 3. (a) the envelope, printed

```
$ utils/model_set_census.py --form envelope -k zebra2-minus-15.ein

## (a) envelope — examples/zebra2-minus-15.ein   [OVER-APPROXIMATION]

  the box has 99,532,800,000,000 cells; the set has 32.  over-approximation 3.11e+12×
  what follows says which facts are settled — never which combinations occur.

  certain — 340 facts in all 32 models
    is-a* 103 · is-a 37 · relation 18 · co-located 16 · co-located-negative 16
    not drink-loc 10 · surjective 10 · total 10 · not nation-loc 9 · next-to 8
    bijective 5 · domain-elimination 5 · functional 5 · functional-negative 5
    injective 5 · injective-negative 5 · not color-loc 5 · range-elimination 5
    surjective-owed 5 · total-owed 5 · typecheck-arg-0 5 · typecheck-arg-1 5
    right-of 4 · adjacent-via 3 · adjacent-via-bwd 3
    adjacent-via-bwd-negative 3 · adjacent-via-endpoint-bwd 3
    adjacent-via-endpoint-fwd 3 · adjacent-via-fwd 3
    adjacent-via-fwd-negative 3 · disjunctive-prune-bwd 3
    disjunctive-prune-fwd 3 · includes 2 · not pet-loc 2 · not smoke-loc 2
    bijection-hierarchy 1 · drink-loc 1 · nation-loc 1 · symmetric 1
    transitive 1 · typecheck-hierarchy 1

    of which decided slots: 2
      drink-loc:Milk           = House-3
      nation-loc:Norwegian     = House-1

  varying — 23 slots
    color-loc:Blue           ∈ {House-1, House-2, House-3, House-4, House-5}
    color-loc:Green          ∈ {House-2, House-4, House-5}
    color-loc:Ivory          ∈ {House-1, House-3, House-4}
    color-loc:Red            ∈ {House-2, House-3, House-4, House-5}
    color-loc:Yellow         ∈ {House-1, House-2, House-3, House-4, House-5}
    drink-loc:Coffee         ∈ {House-2, House-4, House-5}
    drink-loc:Juice          ∈ {House-1, House-2, House-4, House-5}
    drink-loc:Tea            ∈ {House-2, House-4, House-5}
    drink-loc:Water          ∈ {House-1, House-2, House-4, House-5}
    nation-loc:Englishman    ∈ {House-2, House-3, House-4, House-5}
    nation-loc:Japanese      ∈ {House-2, House-3, House-4, House-5}
    nation-loc:Spaniard      ∈ {House-2, House-3, House-4, House-5}
    nation-loc:Ukrainian     ∈ {House-2, House-4, House-5}
    pet-loc:Dog              ∈ {House-2, House-3, House-4, House-5}
    pet-loc:Fox              ∈ {House-1, House-2, House-3, House-4, House-5}
    pet-loc:Horse            ∈ {House-1, House-2, House-3, House-4, House-5}
    pet-loc:Snail            ∈ {House-1, House-2, House-3, House-4, House-5}
    pet-loc:Zebra            ∈ {House-1, House-2, House-4, House-5}
    smoke-loc:Chesterfields  ∈ {House-1, House-2, House-3, House-4, House-5}
    smoke-loc:Kools          ∈ {House-1, House-2, House-4, House-5}
    smoke-loc:Lucky_Strike   ∈ {House-2, House-3, House-4, House-5}
    smoke-loc:Old_Gold       ∈ {House-1, House-2, House-3, House-4, House-5}
    smoke-loc:Parliament     ∈ {House-2, House-3, House-4, House-5}
```

**Printing it is what found the problem with it.** The stage's description of
(a) was *"these 312 facts hold in every model"*, and printing the 340 by
relation shows what they are: `is-a*` 103, `is-a` 37, `relation` 18, the
property activators, the derived closure. **Two of them are answer** —
`Milk@House-3` and `Norwegian@House-1`, the puzzle's own stated clues — and the
other 338 are scaffolding the reader supplied in the first place. A "certain
core" of 340 facts sounds like 340 things learned; it is two.

The frontier is the part with content, and it is 23 lines. It is also where
the form's dishonesty lives: five slots read *"one of five houses"* and a
reader who takes the ranges as independent computes 9.95 × 10¹³ where the
answer is 32.

---

## 4. (b) the key, printed

```
$ utils/model_set_census.py --form key -k zebra2-minus-15.ein

## (b) key — examples/zebra2-minus-15.ein   [EXACT]

  4 of 23 variables determine the model; 22 such 4-sets exist.
  This one's domains allow fewest combinations — 320, of which 32 occur.
  Every one of the 22 contains: pet-loc:Horse, pet-loc:Zebra

    color-loc:Red  nation-loc:Japanese  pet-loc:Horse  pet-loc:Zebra
    -------------  -------------------  -------------  -------------
    House-2        House-3              House-2        House-1
    House-2        House-4              House-2        House-1
    House-2        House-4              House-4        House-5
    House-2        House-5              House-2        House-4
    House-2        House-5              House-2        House-5
    House-2        House-5              House-4        House-1
    House-2        House-5              House-4        House-2
    House-3        House-2              House-3        House-1
    House-3        House-2              House-3        House-2
    House-3        House-2              House-3        House-4
    House-3        House-4              House-1        House-2
    House-3        House-4              House-4        House-5
    House-3        House-5              House-1        House-2
    House-3        House-5              House-1        House-4
    House-3        House-5              House-1        House-5
    House-3        House-5              House-2        House-5
    House-3        House-5              House-3        House-5
    House-3        House-5              House-5        House-4
    House-4        House-2              House-2        House-5
    House-4        House-2              House-4        House-1
    House-4        House-2              House-4        House-5
    House-4        House-3              House-4        House-5
    House-5        House-2              House-2        House-4
    House-5        House-2              House-4        House-2
    House-5        House-2              House-5        House-2
    House-5        House-2              House-5        House-4
    House-5        House-3              House-1        House-2
    House-5        House-3              House-3        House-2
    House-5        House-3              House-3        House-4
    House-5        House-3              House-5        House-4
    House-5        House-4              House-1        House-2
    House-5        House-4              House-1        House-4

  32 rows. The other 19 varying slots follow:
  re-saturate with a row and the model is fixed.
```

### 4.1 "The other 19 follow" — verified, not asserted

That last line is a claim about the **engine**, not about the mathematics. The
key determines the model *within the 32 the census found*; whether a reader
holding a row can actually recover the model depends on whether saturation
finishes the job. Measured, on all 32 rows:

```
for each row: append its four facts to zebra2-minus-15.ein, then solve -e -m 3
```

| | |
|---|---|
| verdict | **`Solution`, `k = 1`, `exhausted = true` — 32 of 32** |
| fact set recovered | **identical to the census's model, 32 of 32** |
| commitments entered | **0 on 30 of the 32 rows** — no layer is opened at all; the other two enter 22 and 35 across two layers |
| wall | **0.32 s for all 32**, median 10 ms, max 23 ms |

So the key table is a *lossless compression with a 10-millisecond decompressor*,
and on 30 of the 32 rows the decompressor is **pure saturation** — the four
facts pin the puzzle and no hypothesis is raised at all. That is the strongest
single result in this stage: **(b) is exact in the operational sense and not
only the set-theoretic one**, and it is nearly free to decompress.

### 4.2 "Why these four" — the objection, answered

The stage's doubt about (b):

> Twenty-two quadruples determine the set; nothing distinguishes them; and a
> reader shown one will ask *why these four*. A key chosen for minimality is an
> arbitrary basis.

Three things distinguish them, and the census supplied all three:

- **The basis is not arbitrary in its most important half.**
  `pet-loc:Horse` and `pet-loc:Zebra` are in **all 22** minimum keys. Two of
  the four columns are forced; only the other two are a choice.
- **Ten of the 23 variables are in no minimum key at all**, so the choice is
  over 11 variables, not 23.
- **The instrument picks the tightest, not the first.** Among the 22 it
  returns the one whose domains allow fewest combinations — 320, where the
  others allow 400. That is a stated rule a reader can check, not a coin toss.

The residual arbitrariness is two columns out of four, and it is worth saying
that it is a *feature* of the answer rather than of the form: the puzzle really
does have several equally good ways to be pinned down.

---

## 5. (e) the list, printed — and what `solve` prints instead

```
$ utils/model_set_census.py --form list -k zebra2-minus-15.ein

## (e) list — examples/zebra2-minus-15.ein   [EXACT]

  32 models × 23 varying slots, + 340 facts shared by all of them.

    color-loc:Blue color-loc:Green color-loc:Ivory color-loc:Red … (23 columns, 396 chars)
    -------------- --------------- --------------- ------------- …
    House-1        House-4         House-3         House-2       …
    …  (32 rows)
```

Rendered in the *same alphabet* as (a) and (b) on purpose, so the readability
test compares structures rather than formatting. It is 39 lines — fewer than
either — and unusable, because a row is 396 characters and there is nothing to
fold: 23 independent columns is what "no structure to exploit" looks like when
you try to print it.

**What `solve -e` actually prints is a different object**: 516 lines at 86
columns, and it is already a summary — per model, the `(query :goal …)`
bindings and the rendered query facts, four of the 435. So the enumeration
arm's real question is not *"32 lines or a compact form"* but *"which
projection of each model"*, which is a decision
[S1d.3.3](s1d.3.3_the_verdict.md) can take without any of (a)–(d).

---

## 6. The readability test

### 6.1 What was actually done, and what was not

The stage asked for a reader. **There was none**: the forms were built and
judged by the same party, which is the weakness the fourth column exists to
guard against, and saying so is cheaper than pretending otherwise.

What was done instead is mechanical, and chosen so that the answers do not
depend on taste. For each form and each of the stage's three questions:

- **can the question be answered from the form at all**, without the engine and
  without the other forms;
- **what must be scanned** — one line, one column, the whole thing;
- **is arithmetic required**, and — the question that turned out to matter —
  **does the arithmetic the form invites give the right answer**.

### 6.2 The result

| | *Is the Zebra's house determined?* | *What else would determine it?* | *How many models are there?* |
|---|---|---|---|
| **(a) envelope** | **yes** — one line of 23: `∈ {House-1, House-2, House-4, House-5}`, so no | **cannot** — the form holds no joint information at all | **cannot**, and worse: multiplying the 23 ranges gives 9.95 × 10¹³ against 32 |
| **(b) key** | **yes** — one column, 4 distinct values in 32 rows | **yes**, and it is the only form that can: the rows *are* the joint projection, and the header says nothing determines the Zebra without also fixing the Horse | **yes** — count the rows; the form says "32 rows" |
| **(e) list** | yes — one column of 32, buried in a 396-character line | in principle: every joint fact is present, and the form points at none of it | **yes** — the header says "32 models" |

**(a) fails the question the stage itself named decisive.**

> A form that cannot answer *"how many models"* without the reader multiplying
> something is not compact, it is folded.

It is worse than folded. (a) does not merely fail to answer *how many*: it
**invites a wrong answer**, because ranges printed side by side read as
independent, and here the product over-states by 3.11 × 10¹². The mitigation
the stage proposed — *"say which it is"*, label it an over-approximation — is
in the rendering above (the `[OVER-APPROXIMATION]` tag and the cell count on
line 3) and it does not repair the second column: no label makes the form able
to say what would determine the Zebra, because that information is not in it.

**(b) answers all three, and answers the second one alone.** *What else would
determine it* is the question a person solving a puzzle actually has, and it is
the one an envelope structurally cannot hold: an envelope is a projection onto
single variables and the answer lives in the pairs.

**(e) answers two of three and loses on the page.** Recorded as a pass on
content and a fail on presentation — which is exactly the split
[S1d.3.3](s1d.3.3_the_verdict.md) needs, because the fix for (e) is a better
projection and not a better representation.

### 6.3 What a real reader test would add

Three things this one cannot: whether *"one column, 32 rows"* is a scan a
person will actually perform; whether the phrase *"re-saturate with a row"*
means anything to someone who has not read
[design/06](../../../docs/history/m1a_rust/design/06_saturation.md); and
whether the key table reads as an answer or as a lookup table for a machine.
All three bear on the veto and none of them is settled here.

---

## 7. (c) the decision diagram, priced

Not built. Priced by **counting the diagram the model set would produce** — a
node of a reduced MDD is a distinct *residual set*, so the count is exact for a
given variable order rather than a bound:

```
$ utils/model_set_census.py --form diagram -k zebra2-minus-15.ein

    variable order                   nodes   edges  widest
    ------------------------------ ------- ------- -------
    canonical (the census's)           399     429      30
    domain size, ascending             355     385      27
    coupling degree, descending        439     469      30
    key variables first                534     564      30
    best of 500 random                 409     439      32
    the 4-variable key alone            27      55      11

  bounds, for any order at all: 24 ≤ nodes ≤ 737 (a level has ≥ 1 node and ≤ k)
  against the enumeration: 32 rows × 23 columns = 736 cells; against the key
  table: 32 rows × 4 columns = 128 cells
```

**The stage expected "unbounded on this shape" and that is not the answer.**
The right one is smaller and more final: at *k* = 32 a decision diagram is
**bounded between 24 and 737 nodes whatever the variable order**, because a
level holds at least one node and at most *k*. The best order found — by domain
size, ascending — gives **355 nodes and 385 edges**, which is *eleven times*
the 32 models it represents. Over the four key variables it is 27 nodes and 55
edges against a 32 × 4 table.

So (c) does not lose on the coupling structure. It loses on arithmetic that has
nothing to do with this puzzle: **a decision diagram is a win when *k* is
exponential in *n*, and here *k* is 32.** No variable order changes that, and
[the census](model_set_census.md)'s separator number — 17 of 23, the graph
being K₂₃ minus a five-star — is the reason a diagram would *also* be no good
if *k* were large. Both halves point the same way and neither is about the
order.

One thing the numbers say that is worth keeping: **ordering by domain size
beats the best of 500 random orders** — 355 against 409 — and *key variables
first* is the worst of the five at 534. Putting the determining variables at
the top of the diagram is the intuitive move and it is the wrong one: it forces
every distinction into the first four levels, where the widest level is 30 of a
possible 32, instead of letting the shallow levels share.

---

## 8. (d) the disjunctive constraint store, deferred

[`ideas.md`](../ideas.md)'s own formulation of the compact answer, and the one
that is not a rendering:

> **symbolic solution saturation** — fixed point над ограничениями,
> представляющими множество моделей.

**Deferred, in the milestone's form.**

**What survives the deferral** — the specification, so that reversing it is
cheap: a second inference mode beside the one M1d has, whose objects are
*constraints over model sets* rather than facts, whose fixpoint is over those
constraints, and whose answer is a saturated description of a family of graphs
rather than a member of it. It subsumes (a), (b) and (c) as read-outs. It
interacts with everything [P1d.2](../p1d.2_obligations/README.md) built —
obligations are constraints of exactly the shape it would store — and pricing
it needs a **design**, not a measurement, which is why a two-day stage may not
start it.

**The trip-wire, as a property of a corpus entry** rather than of a wish. (d)
becomes worth starting when the corpus holds an entry that is all three of:

1. its model set is **finite** — otherwise the answer is a different problem;
2. its model set is **too large to enumerate or to print** — some *k* where
   32 × 435 stops being a table;
3. the model set is **the question**, not a by-product — a `(query …)` whose
   answer is the family.

**No entry trips it today, and the two near misses fail different clauses.**
`examples/zebra2-minus-15.ein` fails (2): it enumerates in 26.5 s at the depth
that finds every model, and 32 models is a 43-line table.
`examples/features/04_open.ein` and the three `saturation/square-unique/*`
demos fail (1): nothing bounds their hypothesis space, `solve` ends in the OOM
killer rather than a verdict, and the manifest declares no `solve` run for that
reason. An unbounded space is not a large model set — it is the absence of one,
and (d) would not help.

The nearest thing to a trip-wire the corpus has is
`saturation/type-exclusivity/pets.ein`: 35 models at `-m 10`, `exhausted =
false`, one coupling component. It fails (2) and (3), but it is the entry to
watch, because it is the one whose *k* grows with the fixture.

---

## 9. What S1d.3.3 inherits

| | |
|---|---|
| **the recommendation** | **(b) if anything ships; (a) does not, on the readability veto** — and the reversal is measured, not argued |
| **(a)** | free, 2 889 B, **lossy by 3.11 × 10¹²**, cannot say how many models and invites a wrong answer. Its 340-fact "certain core" is 2 facts of answer and 338 of scaffolding |
| **(b)** | 2 506 B, **exact and verified**: 32/32 rows reconstruct the model to the fact in ≤ 23 ms, and **30 of the 32 by saturation alone**. Two of its four columns are in every minimum key |
| **(c)** | **priced out on scale, not on structure**: 355 nodes for 32 models, bounded in [24, 737] under every order |
| **(d)** | **deferred with a three-clause trip-wire**, no corpus entry trips it, and the near misses fail different clauses |
| **(e)** | exact and unusable at 396 columns *in this alphabet* — but what `solve -e` prints is a projection, and choosing a better projection needs no representation at all |
| **the exhaustion caveat, undischarged** | `solve -e zebra2-minus-15` is `exhausted = false`. Every form above describes **the 32 models the search recorded**, and (b)'s "32 rows" is a claim about that set. A 33rd model would add a row — and would silently invalidate (a)'s core, which is the asymmetry [S1d.3.3](s1d.3.3_the_verdict.md) has to phrase |
| **not settled here** | whether a person who did not build these forms can read them (§6.3), and whether a label is enough to ship an over-approximation |
