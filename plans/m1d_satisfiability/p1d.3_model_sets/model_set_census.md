# The model-set census — what 32 models are made of, and whether they factor

**Stage:** [S1d.3.1](s1d.3.1_what_the_models_differ_in.md) · **Phase:** [P1d.3](README.md)
**Taken:** 2026-08-25, over all 168 `corpus.toml` entries that declare a `solve` run.
**Instrument:** [`utils/model_set_census.py`](../../../utils/model_set_census.py),
reading `--json-summary`'s `verdict.solutions` as *k* fact sets and its
`leftover` block (`EIN_LEFTOVER=1`) for the blind-enumerator probe. Nothing
here is re-derived from the event stream.
**Re-take:** `utils/model_set_census.py --json c.json` — ~9 min, of which the
two `zebra2-minus-15` entries are 2 min each.

[P1d.3](README.md) exists to decide whether 32 models should be **printed or
described**, and it opened with a hope: that a state with several
*independent* open choices already **is** the compact answer, because the model
count is then the product of the candidate-set sizes and no search is needed
to report it. This is the measurement of the word **independent**.

The short version: **the product form exists, it is worth nothing, and the
reason it is worth nothing is a number.** It holds on two corpus entries, both
of which have two objects; it is gone at three; and on the puzzle the phase is
named after the 23 varying decision variables form **one** coupling component
whose graph is K₂₃ minus five edges.

---

## 1. What a decision variable is, and who says so

A model set varies in *facts*. A factorisation is a claim about *variables*, so
something has to turn one into the other — and the thing that does it must not
be the zebra shape written into a script. Two rules:

1. **Every varying positive atom is a Boolean variable.** The general case: a
   fact is in a model or it is not. No declaration needed.
2. **A relation the program declares `functional`** (or `bijective`, which fans
   out into it) makes the atoms `(R a ·)` mutually exclusive, so for each `a`
   they collapse into **one** variable `(R, a)` whose domain is the set of
   values it takes. That is the only refinement, and the declaration is exactly
   the licence for it.

The declarations are read from the **models' own facts** — `(relation R …)` and
`(functional R)` hold in every model — not from the source text, so a program
that says `bijective`, one that says `functional` and one that derives the
marker by a rule are read identically. An atom rule 2 does not reach stays
Boolean and is **named** in `unrefined`, not folded into a count: it is a
finding about the program, which varies over a relation nobody declared
single-valued. `features/11_expect_ambiguity` is the instructive case — a
bijection puzzle in every respect, written with two hand-rolled activators
(`one-person-each`, `one-seat-each`) instead of `(bijective seat)`, so its four
`(seat …)` atoms stay Boolean and its product reads 16 against `k = 2` where a
declaration would have read 4. **The refinement is only as good as what the
program says about itself**, and the `unrefined` column is where that shows.

Two conventions follow from this and are worth stating because they set what
the numbers below mean:

- **A functional slot is a variable wherever it is, varying or not.** The
  refined variables come off the *union* of the models, so a slot the puzzle
  pinned — `(drink-loc Milk House-3)`, a clue — is a variable with a one-value
  domain rather than an invisible part of the core. *25 slots of which 2 are
  stated* is a fact about the puzzle; *23 slots* is a fact about the answer.
- **The unrefined atoms get the opposite treatment**, because a Boolean
  variable has no slot apart from its own presence: a *core* atom is not a
  fixed decision, it is just a fact. Counting all of them would report
  `zebra2`'s model as 435 variables of which 340 are fixed — which is the
  core/varies split, already two columns to the left.

**Varying negatives are not variables.** Where negative completion writes
`(not (R a b))` beside every excluded value the two halves mirror exactly, and
counting both would square the description. Measured rather than assumed: the
mirror is **exact on all 13** entries below — every varying negative is the
negation of a varying positive, and there are no leftovers.

---

## 2. The corpus's model sets — thirteen, not nine

| entry | cap | *k* | exhausted | facts | core | varies | vars | fixed | unrefined |
|---|---:|---:|---|---:|---:|---:|---:|---:|---:|
| `branching/02_one_dead_one_alive` | 8 | 4 | ✔ | 70 | 52 | 36 | 18 | 6 | 18 |
| `branching/04_two_levels` | — | 2 | ✔ | 78 | 70 | 16 | 8 | 8 | 8 |
| `branching/06_lookahead_on` | 10 | 22 | ✔ | 274 | 232 | 84 | 42 | 12 | 42 |
| `branching/08_hypothesis_relation_whitelist` | 10 | 8 | ✔ | 79 | 53 | 52 | 26 | 8 | 26 |
| `branching/12_typed_blind_solve` | — | 2 | ✔ | 53 | 49 | 8 | 2 | 1 | 0 |
| `features/11_expect_ambiguity` | — | 2 | ✔ | 14 | 10 | 8 | 4 | 0 | 4 |
| `lattice/01_subset_pruned` | — | 2 | ✔ | 18 | 16 | 4 | 2 | 0 | 2 |
| `lattice/02_genuine_3set_death` | — | 3 | ✔ | 13 | 10 | 6 | 3 | 0 | 3 |
| `saturation/type-exclusivity/colors` | 6 | 9 | ✔ | 17 | 9 | 16 | 8 | 0 | 8 |
| `saturation/type-exclusivity/nationalities` | 6 | 9 | ✔ | 17 | 9 | 16 | 8 | 0 | 8 |
| `saturation/type-exclusivity/pets` | 10 | 35 | ✘ | 29 | 14 | 30 | 15 | 0 | 15 |
| **`zebra2-minus-15`** | **3** | **32** | **✘** | **435** | **340** | **190** | **23** | **2** | **0** |
| `zebra2-minus-15-obligations` | 3 | 32 | ✘ | 435 | 340 | 190 | 23 | 2 | 0 |

The other 155 entries: 108 report one model or none, and 47 never reach a
fixpoint (load-, parse- and compile-negative fixtures, by design). Verdicts
across the 121 measured: **70 `Solution`, 13 `Ambiguity`, 26 `Contradiction`,
12 `Open`**.

**Thirteen, where the phase README said nine**, and the difference is the depth
cap rather than the corpus. The README counted `solve -e -m 2`; this counts
each entry at *the depth that finds every model it has*, escalating `-m` while
the run stays cheap. Four entries that count missed: `lattice/01_subset_pruned`,
whose second model is at depth 3, and the three `type-exclusivity` demos, which
need 6 and 10. Three of the original nine reach `exhausted` only above the
default — `branching/02` at `-m 8`, `06` and `08` at `-m 10`. `branching/02`
finds 2 models at `-m 2` and **4** at `-m 5`, and the extra depth buys the
proof rather than a model.

**Ten of the thirteen are exhausted, and three are what the cap reached.** That
distinction is load-bearing everywhere below, and it is the phase acceptance's
own warning: a set the cap truncated is a *subset*, so a core computed by
intersecting it is a **superset** of the truth. Intersecting fewer models gives
more core — which makes over-claiming the easy direction, not a remote one.
The three: `type-exclusivity/pets` and both `zebra2-minus-15` twins.

---

## 3. Does anything factor

Four questions, coarsest first. Only the last two have a positive answer
anywhere.

| granularity | test | answer |
|---|---|---|
| **by relation** | is `color-loc`'s projection independent of `pet-loc`'s? | **0 of 20 relation pairs independent** |
| **by variable pair** | is `proj(u,v)` the whole `dom(u) × dom(v)`? | on `zebra2-minus-15`, **248 of 253 pairs coupled** |
| **by partition** | components of the coupling graph, with `Π \|proj(cᵢ)\| == k` | **2 of 13 entries** — and both have two objects |
| **by basis** | is the set a free grid over a determining key? | **5 of 13 entries** — all with `k ≤ 4` |

The per-entry table:

| entry | *k* | Π dom | ratio | coupled/pairs | comps | partition | key | Π key | free grid |
|---|---:|---:|---:|---:|---:|---|---:|---:|---|
| `branching/02` | 4 | 262 144 | 6.55 × 10⁴ | 121/153 | 1 | ✘ | 2 | 4 | **✔** |
| `branching/04` | 2 | 256 | 128 | 28/28 | 1 | ✘ | 1 | 2 | **✔** |
| `branching/06` | 22 | 4.40 × 10¹² | 2.00 × 10¹¹ | 509/861 | 1 | ✘ | 8 | — | ✘ |
| `branching/08` | 8 | 6.71 × 10⁷ | 8.39 × 10⁶ | 197/325 | 1 | ✘ | 4 | 16 | ✘ |
| `branching/12` | 2 | 4 | 2 | 1/1 | 1 | ✘ | 1 | 2 | **✔** |
| `features/11` | 2 | 16 | 8 | 6/6 | 1 | ✘ | 1 | 2 | **✔** |
| `lattice/01` | 2 | 4 | 2 | 1/1 | 1 | ✘ | 1 | 2 | **✔** |
| `lattice/02` | 3 | 8 | 2.67 | 3/3 | 1 | ✘ | 2 | 4 | ✘ |
| `type-exclusivity/colors` | 9 | 256 | 28.4 | 12/28 | **2** | **✔** | 4 | 16 | ✘ |
| `type-exclusivity/nationalities` | 9 | 256 | 28.4 | 12/28 | **2** | **✔** | 4 | 16 | ✘ |
| `type-exclusivity/pets` | 35 | 32 768 | 936 | 21/105 | 1 | ✘ | 6 | 64 | ✘ |
| **`zebra2-minus-15`** | **32** | **9.95 × 10¹³** | **3.11 × 10¹²** | **248/253** | **1** | **✘** | **4** | **320** | **✘** |
| `zebra2-minus-15-obligations` | 32 | 9.95 × 10¹³ | 3.11 × 10¹² | 248/253 | 1 | ✘ | 4 | 320 | ✘ |

Four things to read out of it.

**The `Π dom / k` ratio is the price of the box.** It is what a reader would
believe if told only "these slots are undecided and here is each slot's range":
on `zebra2-minus-15`, 9.95 × 10¹³ combinations where 32 models exist. The
smallest ratio on any non-degenerate entry is 2. **No entry has a ratio of 1**,
which is the whole "free by product" claim, stated as a measurement.

**`branching/06`'s key is a bound, not a value.** The minimum-key search stops
at size 8 and `C(42, 8) = 118 030 185` is over the enumeration budget, so its
key count and product are reported as unknown rather than guessed. Its freeness
is settled anyway by the domain bound — `2⁸ = 256 > 22`.

**One component is not the same as "does not factor".** `branching/02` has one
component of 18 variables *and* is a free grid over 2 of them. The two tests
answer different questions: the partition asks whether the variable set splits,
the basis asks whether the model set is a product on a smaller alphabet. A
derived atom is coupled to everything it derives from, so it keeps the graph
connected while contributing no freedom at all.

**Every one of the five free grids has `k ≤ 4`**, and four of them are a single
binary choice. That is a fact about the corpus, and it is the honest ceiling on
what "report the factorisation instead" would buy here.

---

## 4. The one that does factor, and why it stops at three objects

`saturation/type-exclusivity/colors.ein` is a two-fact demo — one rule, one
`(relation co-located T T)`, `(is-a Red Color)`, `(is-a Blue Color)`. Its 9
models split as **3 × 3** over two components of four variables each, and it is
exhausted, so the factorisation is a fact about the whole set and not about a
prefix of it. `nationalities.ein` is the same program with two other names and
the same numbers.

`pets.ein` is the same program with **three** instances. It reports **35**
models over 15 varying variables in **one** component — and does not exhaust at
`-m 10`.

| | instances | *k* | varying | components | partition |
|---|---:|---:|---:|---:|---|
| `colors`, `nationalities` | 2 | 9 | 8 | **2** | **✔** |
| `pets` | 3 | 35 | 15 | 1 | ✘ |

**So the compact-by-independence path exists, and it exists exactly where the
puzzle is too small to need it.** Going from two objects to three loses the
partition and multiplies the answer by four. That is a sharper result than "it
never happens", and it is the one to quote at
[S1d.3.2](s1d.3.2_representations.md): a representation justified by
independence would be justified by a two-object fixture and inapplicable to
every entry anyone runs.

---

## 5. `zebra2-minus-15` — what the coupling is made of

The phase's own case, at `-m 3`, the depth that finds all 32 models
(26.5 s; the run does not exhaust — [layer census
§4](../p1d.10_exhaustive_search/layer_census.md#4-zebra2-minus-15-all-five-layers)).

### 5.1 The shape of a model

| | |
|---|---:|
| facts per model | 435 |
| **shared by all 32** | **340** — 312 positive, 28 negative |
| varying | 190 — **95 positive `*-loc` arrows and the 95 `(not …)` beside them** |
| share of a model that is common | **78.2 %** |

The 95 varying positives are exactly the sum of the 23 varying domains, and the
95 negatives mirror them one for one: `mirror_gap = 0`. Nothing varies that is
not an attribute arrow — `co-located`, `next-to`, `right-of` and `is-a*` are
identical in every model, which is why `unrefined = 0` here and 8–42 everywhere
else.

### 5.2 The variables

**25 decision variables, of which 2 are fixed** — and the two are the puzzle's
two stated arrows, `(drink-loc Milk House-3)` and `(nation-loc Norwegian
House-1)`. That is
[S1d.2.4](../p1d.2_obligations/s1d.2.4_obligations_in_the_saturator.md)'s
asymmetry seen from the far end: there `nation-loc` and `drink-loc` owe 8 at
root where the other three owe 10, which is the same two clues counted before
the search instead of after it.

The 23 varying domains: **5 of size 3, 10 of size 4, 8 of size 5** — 95 in
total, product **99 532 800 000 000**.

### 5.3 The coupling graph

| | |
|---|---:|
| pairs | 253 |
| coupled | **248 (98.0 %)** |
| within a relation | **42 / 42** |
| across relations | 206 / 211 |
| components | **1**, containing all 23 |
| minimum degree | **17** |
| **minimum vertex separator** | **17** |

**The graph is K₂₃ minus five edges, and all five share one vertex.** The five
pairwise-independent pairs are

```
(color-loc  Green)      × (pet-loc Zebra)
(color-loc  Ivory)      × (pet-loc Zebra)
(drink-loc  Coffee)     × (pet-loc Zebra)
(nation-loc Japanese)   × (pet-loc Zebra)
(smoke-loc  Parliament) × (pet-loc Zebra)
```

The within-relation cliques are complete and that is not a surprise —
`injective` makes the five values of one attribute mutually exclusive, so
knowing where Red is constrains where Green can be. The finding is the other
column: **206 of the 211 *cross*-attribute pairs are coupled too**, and the
whole of that comes from **eleven clues** — eight `co-located` activators
(16 facts, both directions) and three `adjacent-via` (`zebra2-minus-15` is
`zebra2` with clue 15 removed).

**This is the structural answer [S1d.3.2](s1d.3.2_representations.md) needed
about decision diagrams.** A BDD/ZDD is small when the variable order has small
separators; here the smallest separator between any two variables is 17 of 23,
so no order helps. [`c/README.md` § Circular dependencies between
levels](../../../c/README.md) describes the puzzle's *constraint* graph as
K₅ minus two edges; the graph over its *answers* is denser still, and by the
same mechanism — one clue relating two attributes couples all twenty-five
pairs of their slots, because `injective` propagates it from the two values it
names to the other eight.

### 5.4 The determining key

**Four variables determine all 32 models; no three do.** 22 of the 8 855
quadruples work — and, spelled out for the first time here, they are not 22
arbitrary quadruples:

| variable | in how many of the 22 keys | domain |
|---|---:|---:|
| **`pet-loc:Horse`** | **22 / 22** | 5 |
| **`pet-loc:Zebra`** | **22 / 22** | 4 |
| `nation-loc:Japanese` | 7 | 4 |
| `smoke-loc:Parliament` | 7 | 4 |
| `color-loc:Red` | 6 | 4 |
| `nation-loc:Englishman` | 6 | 4 |
| `nation-loc:Spaniard` | 4 | 4 |
| `pet-loc:Dog` | 4 | 4 |
| `pet-loc:Fox` · `pet-loc:Snail` · `smoke-loc:Chesterfields` · `smoke-loc:Old_Gold` · `drink-loc:Water` | 2 each | 4–5 |

Ten of the 23 variables appear in no minimum key at all.

**`pet-loc:Zebra` is in every key and is also the variable with all five
pairwise-independent partners.** Those two facts are not in tension, they are
the whole lesson: pairwise independence is not joint independence. The variable
no description can omit is the one that looks freest edge by edge.

**And the key does not compress the way independence would.** Its four domains
allow 320 combinations and 32 occur — 10 % — so the description is a 32-row
table four columns wide rather than twenty-five. **No key is free at any size**,
and that is provable rather than searched: every varying domain here has at
least 3 values, so any four-variable key has at least 3⁴ = 81 > 32 cells, and
a key can only get bigger.

---

## 6. The leftover-open count — the probe P1d.2 handed forward

[T1d.3.1.4](s1d.3.1_what_the_models_differ_in.md), and
[P1d.2 §6](../p1d.2_obligations/hypotheses_from_obligations.md) is where the
question was parked:

> how many facts would the blind enumerator still propose at a node the rung
> called complete?

### 6.1 The probe is a read, and the fork is why

P1d.2 declined the measurement, correctly, for the pass it had in mind: a blind
generation over the live node **writes** — `enable_lookahead_kill_cache` stores
`(not h)` per kill — which would move the node's `state_key` and therefore the
model dedup that produced *k*. *A probe that has to disable a config flag to
avoid changing the answer is a probe that is measuring a different engine.*

The probe that is a read runs the same pass **on a fork of the state's KB and
drops the fork**, which is the mechanism `--json-summary`'s `root` block has
used since S1a.0.1. Three pieces:

- `hypgen::generate_blind` — the ladder with its two upper rungs skipped. Not a
  search entry point: the search always walks the ladder, and the question is
  precisely what the *bottom* rung would say at a node an upper rung called
  complete.
- `--json-summary`'s **`leftover`** block, index-aligned with
  `verdict.solutions` and `verdict.open_states`.
- `EIN_LEFTOVER=1`, an env lever for `EIN_OBLIGATION_CHOICE`'s two reasons: a
  `(config …)` field is in the KB-shape digest, and every corpus `solve` in
  `cargo test` writes a summary, so a probe that ran by default would put a
  blind pass into the gate per model.

**Checked rather than argued:** with the lever on and off, every field of every
summary outside the `leftover` block is identical on all 121 entries that reach
a fixpoint. `ein-cli/tests/leftover_probe.rs` holds the same claim on a
multi-model entry. Cost is one generation call per *recorded state* — ≈ 40 ms
on the zebra family, under 1 ms elsewhere.

### 6.2 The number

**244 states probed across 95 entries.** 160 of them propose nothing at all.

| entry | states | leftover, per state |
|---|---:|---:|
| `examples/zebra.ein` | 1 | **4 123** — `type` 1 260, `instance` 1 080, `next-to` 824, `right-of` 485, `co-located` 474 |
| `examples/zebra2.ein` · `zebra2-hints` · `zebra2-obligations` | 1 each | **3 678** |
| `examples/zebra2-minus-15{,-obligations}` | 32 each | **3 678** |
| `domain_elim/{ab,b_branch,b_only}` | 1 each | 79 — `is-a` 56, `color-loc` 23 |
| `branching/{10,11}_kill_cache_*` | 1 each | 64 |
| `branching/05_mini_zebra` | 1 | 36 |
| `lattice/01_subset_pruned` | 2 | 29 |
| `branching/09_hrule` | 1 | 24 |
| `ein-bugs/mixed-type-hypothesis` | 1 | 22 |
| `features/11_expect_ambiguity` | 2 | 16 |
| `lattice/03_state_hash_collision` | 1 | 4 |
| `lattice/02_genuine_3set_death` | 3 | 3 — `a-prop` 1, `b-prop` 1, `is-a` 1 |
| **77 further entries** | 160 states | **0** |

**The zebra puzzle's unique model leaves 3 678 facts open.** Under an
open-world reading that model is 2³⁶⁷⁸ models; the puzzle means the
closed-world one, and **nothing in the file says so**. That is the question
[S1d.3.3](s1d.3.3_the_verdict.md) owns —
[`ideas.md`](../ideas.md)'s *обязательно ли назначать значение каждому
возможному факту?* — now with a number attached instead of a hypothetical.

### 6.3 What the 3 678 are

The block attributes each count, because a bare number cannot say whether the
leftover is a domain the program *meant* to close and did not, or a relation it
never meant to decide. For `zebra2`'s unique model:

```
$ EIN_LEFTOVER=1 ein solve examples/zebra2.ein --json-summary z2.json
  leftover.models              [3678]
  leftover.models_by_relation  { "is-a": 930, "is-a*": 900,
                                 "next-to": 922, "right-of": 926 }
```

**Not one attribute arrow.** All five `*-loc` relations are decided to the last
pair — 25 positives and 100 negatives, which is exactly the five relations'
5 × 5 well-typed atoms, 20 negatives apiece — and the entire 3 678 sits on the
four relations `zebra2` never closes.
Those four are precisely the ones the obligations rung reports as
**`uncovered`** ([S1d.2.5 §6](../p1d.2_obligations/hypotheses_from_obligations.md)):
`is-a`, `is-a*`, `right-of`, `next-to`, named by no obligation and left to
saturation. Saturation does determine them — which is why the hrule and
obligation paths agree on the model set — but determining is not **closing**,
and the enumerator will go on offering what nothing has denied.

**And most of what it offers is ill-typed**, which is not a defect but
[S1.7.23](../../../docs/kernel/inference/implementation.md) working as
designed: *the kernel imposes no type system; the enumerator proposes
type-blind and the puzzle's own rules do the pruning.* `(is-a Red House-1)` is
a candidate and nothing in the KB denies it — which is also why `zebra.ein`'s
older encoding leaves its two largest blocks on `type` and `instance`, the
kernel's own ontology heads. The proof that this is where the number comes
from is `domain_elim/ab.ein`, whose
model leaves **23 `color-loc`** candidates while its 3 × 3 well-typed
`color-loc` atoms are all decided — 3 positive, 6 negative — so every one of
the 23 has an argument outside the relation's declared type.

**That is the finding S1d.3.3 should take, and it is stronger than a count.**
The literal open-world reading of an Ein model is not merely large, it is
*unusable*: the 2³⁶⁷⁸ graphs it stands for are told apart almost entirely by
atoms no reader would call possible, because the kernel has no types to bound
the atom set with. Whatever `solve` means by "the model", it is not that — and
no surface says which it is.

### 6.4 Three readings the table settles

- **A closed domain leaves nothing.** `branching/12_typed_blind_solve` closes
  its one relation and its two models are 0. Where every candidate is either a
  fact or a stored negative, open-world and closed-world agree — which is what
  makes the 160 zero states the interesting rows rather than the empty ones.
- **All 32 models of `zebra2-minus-15` leave the same 3 678.** The leftover is
  a property of the *program's* closure discipline, not of which branch the
  search took, and it does not distinguish models.
- **All twelve `Open` states leave 0.** Owing and openness are orthogonal, and
  the corpus shows it in the strongest form: a state can owe a witness it will
  never get while the enumerator has nothing left to propose.
  [S1d.2.6](../p1d.2_obligations/s1d.2.6_verdicts_counters_corpus.md)'s word is
  about **discharge**; this count is about **coverage**; neither implies the
  other.

---

## 7. The size of the thing being described

The number every later decision leans on, because a compact form larger than
the enumeration is not a compact form.

| | |
|---|---:|
| the model set, as facts | 32 × 435 = **13 920 fact lines** |
| what `solve -e -m 3` prints today | **516 lines** |
| the certain core | 340 facts (**lossy** — the box has 9.95 × 10¹³ cells) |
| the key table | 4 columns × 32 rows (**exact**) |
| the model set, as `k` | 1 line |

**What a reader gets today is already a summary**, and by a factor of 27:
`solve -e` prints each model's *query bindings and query facts*, not its 435
facts. Which summary to print is a decision available with no new
representation at all, and it is [S1d.3.2](s1d.3.2_representations.md)'s
control arm.

---

## 8. What this closes and what it hands forward

| | |
|---|---|
| **the phase's central hope** | **false where it matters.** No entry has `Π dom == k`; the two that partition have two objects each, and the same program with three does not |
| **`ideas.md`'s seventh form** — *частичная модель с `open`-фактами, если они независимы* | **inapplicable on the phase's case**, with the number: one component of 23, minimum separator 17 |
| **what an open fact even is** | **measured, and it is not what the phrase suggests**: `zebra2`'s model leaves 3 678, none of them an attribute arrow and most of them ill-typed (§6.3) |
| **a decision diagram** ([S1d.3.2](s1d.3.2_representations.md) (c)) | **priced and unattractive** — no variable order has a small separator, because there is no small separator |
| **the certain core** ([S1d.3.2](s1d.3.2_representations.md) (a)) | **free and lossy**, quantified: 78.2 % of a model, and an over-approximation by 3.11 × 10¹² |
| **the determining key** ([S1d.3.2](s1d.3.2_representations.md) (b)) | **exact, 4 columns, still 32 rows** — and the *"why these four"* objection has an answer now: two of the four are in every minimum key |
| **the leftover-open count** | **measured**, 244 states, and the probe is a read |
| **closed-world completion** | **still [S1d.3.3](s1d.3.3_the_verdict.md)'s**, now with 3 678 as the size of what the decision is about |
| **the exhaustion caveat** | **three of the thirteen sets are what the cap reached**, including both `zebra2-minus-15` twins — so every claim above about *their* core is a claim about a superset |

**The burden of proof has not moved and the measurement did not move it.** The
corpus offers one real multi-model puzzle; a representation shipped on the
strength of it would be a representation tested once. What changed is that
"enumerate, and say so" now has the arithmetic behind it rather than an
intuition, and the two candidate forms have their sizes.
