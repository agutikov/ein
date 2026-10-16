# The region language in real programs — the examples for acceptance

**Stage:** [S1e2.1a.4](s1e2.1a.4_the_language.md), task T1e2.1a.4.1, after
the ruling.
**Status:** written 2026-10-16 for the user to accept, as asked: *"Show real
examples for syntax to review and accept before continue"*. **Accepted the
same day**, with one question back: *"yet don't understand why we need special
`owing` form, what is wrong with just empy (region) ?"*. § 5 answers it,
measured. **Nothing parses yet.**
**How it was made:** every claim below states the program's real R. Each R is
the oracle's (`ein oracle FILE --dump DIR --states`), or route 1's for
`zebra2.ein`, which is past the oracle. The text is rendered from that data
by a script, so no fact in it was typed by hand. Taken at `02264c31`.

---

## 0. The ruling

The user, 2026-10-16, verbatim:

> Where the region claim lives, and its name: A; A state's generators:
> (merged (model <to model>) (or (model <from model 1>) (model <from model 2>)
> ...)) - so (merged <to model> (from models)); The second kind of empty R:
> yes; The query's answer: yes; Region shapes, Component subsets - don't need
> now both, the region is already defined by it's nodes; Show real examples
> for syntax to review and accept before continue

## 1. The forms, as ruled

```lisp
(solve
  [:hrules (<activator> …)]               ; the lattice: what may be guessed
  [:hypothesis-relations (<rel> …)]
  [:no-hypothesis (<rel> …)]
  [:expect <claim>])                      ; the expected region

<claim> ::= (region <item>*)              ; R, by its nodes
          | (false)                       ; every node dead
<item>  ::= (model <fact>*)               ; one state of R
          | (merged <to> <from>)          ; a state reached by several commitments
          | (owing (model <fact>*) …)     ; R empty, and these alive nodes owe
<to>    ::= (model <fact>*)               ; one of the states listed above
<from>  ::= (model <fact>*)               ; its one minimal commitment
          | (or (model <fact>*) …)        ; …or its several

(query :goal <expr>
  [:goal-text <STRING>]
  [:over all | minimal | maximal]         ; default all
  [:expect (model <fact>*) | (or (model <fact>*) …) | (false)])
```

How it reads:

1. **A `(model …)` is a state of R.** Root's facts are part of it.
2. **Naming a relation closes it, across the whole claim.** Every relation a
   claim mentions is listed at its complete extent in every model, root's
   facts included, so `(model)` is a state with none of them. Stored
   negatives are not closed. A listed `(not …)` is checked for presence.
3. **The models are R's states, one to one.** Their number is the claimed
   k_s, and their order is free.
4. **`(merged …)` is all or nothing.** A claim with none is about states, and
   claims no k_c. A claim with one lists every state reached by more than one
   commitment, and pins k_c. Inside it, `<to>` repeats a listed state, and the
   `<from>` models are **commitments**: the candidates committed, with nothing
   derived and nothing from root.
5. **A query's `:expect` lists projection classes**: each the goal's atoms
   instantiated under every binding that satisfies the goal in a state,
   deduplicated. `(model)` is the class where the goal has no row, and
   `(false)` means no class. Only goal facts may appear.
6. **Not built** (the ruling): region shapes and component subsets.

## 2. Eight programs

| | program | n | k_s | k_c | what it shows |
|---|---|---:|---:|---:|---|
| E1 | `examples/branching/05_mini_zebra.ein` | 9 | 5 | 8 | all three forms; `(merged …)` with `(or …)` of three; an undetermined goal |
| E2 | `examples/saturation/transitive/colocation-chain.ein` | 3 | 4 | 8 | `(merged …)` with two commitments of different sizes; no `:hrules` |
| E3 | `examples/lattice/synth/merged_states.ein` | 5 | 8 | 16 | `(merged …)` with one `(model …)` from, on every state; ∅ not in R |
| E4 | `examples/lattice/02_genuine_3set_death.ein` | 3 | 7 | 7 | `(model)` for ∅; three maximal states, two classes |
| E5 | `examples/features/11_expect_ambiguity.ein` | 4 | 7 | 7 | today's `(or (model …))` claim, migrated |
| E6 | `tests/stdlib/closure/03_closed_and_owing.ein` | 0 | 0 | 0 | `(owing …)`: R empty, root owes |
| E7 | `tests/stdlib/algebra/09_irreflexive_violated.ein` | 0 | 0 | 0 | `(false)`: root dead |
| E8 | `examples/zebra2.ein` | 56 | 1 | 261 982 | the zebra puzzle: `(solve :hrules …)`, a one-state region, the puzzle's answer as a query |

### E1 — `branching/05_mini_zebra.ein`: everything at once

Two people, two drinks and two pets. The two clues put Alice with the tea and
the cat, so root already holds six `co-located` facts: the clues, their
mirrors, and the one transitivity derives. Bob is open. R has five states.
Four of them are reached by one commitment each, and the fifth, the maximal
one, by four. The goal asks who drinks coffee and which pet they keep, and it
has no row on four of the five states.

**Today:**

```lisp
(query :goal (and (co-located ?n Coffee) (co-located ?n ?p) (is-a ?p Pet))
  :hrules (guess (Nationality Drink) guess (Nationality Pet) guess (Drink Pet)))
```

**Under the new forms:**

```lisp
(solve
  :hrules (guess (Nationality Drink) guess (Nationality Pet) guess (Drink Pet))
  :expect (region
            ;; state 1 — ∅, root itself, on the minimal line
            (model (co-located Alice Cat) (co-located Alice Tea)
                   (co-located Cat Alice) (co-located Cat Tea)
                   (co-located Tea Alice) (co-located Tea Cat))
            ;; state 2
            (model (co-located Alice Cat) (co-located Alice Tea)
                   (co-located Bob Coffee) (co-located Cat Alice)
                   (co-located Cat Tea) (co-located Coffee Bob)
                   (co-located Tea Alice) (co-located Tea Cat))
            ;; state 3
            (model (co-located Alice Cat) (co-located Alice Tea)
                   (co-located Bob Dog) (co-located Cat Alice)
                   (co-located Cat Tea) (co-located Dog Bob)
                   (co-located Tea Alice) (co-located Tea Cat))
            ;; state 4 — on the maximal line
            (model (co-located Alice Cat) (co-located Alice Tea)
                   (co-located Bob Coffee) (co-located Bob Dog)
                   (co-located Cat Alice) (co-located Cat Tea)
                   (co-located Coffee Bob) (co-located Coffee Dog)
                   (co-located Dog Bob) (co-located Dog Coffee)
                   (co-located Tea Alice) (co-located Tea Cat))
            ;; state 5
            (model (co-located Alice Cat) (co-located Alice Tea)
                   (co-located Cat Alice) (co-located Cat Tea)
                   (co-located Coffee Dog) (co-located Dog Coffee)
                   (co-located Tea Alice) (co-located Tea Cat))
            ;; state 4 is reached by 4 commitments; its 3 minimal ones
            (merged (model (co-located Alice Cat) (co-located Alice Tea)
                           (co-located Bob Coffee) (co-located Bob Dog)
                           (co-located Cat Alice) (co-located Cat Tea)
                           (co-located Coffee Bob) (co-located Coffee Dog)
                           (co-located Dog Bob) (co-located Dog Coffee)
                           (co-located Tea Alice) (co-located Tea Cat))
                    (or (model (co-located Bob Coffee) (co-located Bob Dog))
                        (model (co-located Bob Coffee)
                               (co-located Coffee Dog))
                        (model (co-located Bob Dog) (co-located Coffee Dog))))))

(query :goal (and (co-located ?n Coffee) (co-located ?n ?p) (is-a ?p Pet))
  :expect (or (model)
              (model (co-located Bob Coffee) (co-located Bob Dog)
                     (is-a Dog Pet))))

(query :goal (and (co-located ?n Coffee) (co-located ?n ?p) (is-a ?p Pet))
  :over maximal
  :expect (model (co-located Bob Coffee) (co-located Bob Dog) (is-a Dog Pet)))
```

The first query answers over all of R with two classes. On four states the
goal has no row: on state 2 Bob drinks the coffee but keeps no known pet. On
state 4 Bob drinks the coffee and keeps the dog. The second query asks about
the maximal line alone, and gets one answer.

### E2 — `saturation/transitive/colocation-chain.ein`: two generators of different sizes

No `hrule`, so the candidates come from the blind rung and `(solve …)` carries
nothing but the claim. The maximal state is reached by five commitments. Two
of them are minimal: `{(co-located C A)}` alone, and
`{(co-located B A), (co-located C B)}`. Transitivity derives the rest either
way.

**Today:** `(query :goal (co-located A C))`. **Under the new forms:**

```lisp
(solve
  :expect (region
            ;; state 1 — ∅, root itself, on the minimal line
            (model (co-located A B) (co-located A C) (co-located B C))
            ;; state 2
            (model (co-located A B) (co-located A C) (co-located B A)
                   (co-located B C))
            ;; state 3 — on the maximal line
            (model (co-located A B) (co-located A C) (co-located B A)
                   (co-located B C) (co-located C A) (co-located C B))
            ;; state 4
            (model (co-located A B) (co-located A C) (co-located B C)
                   (co-located C B))
            ;; state 3 is reached by 5 commitments; its 2 minimal ones
            (merged (model (co-located A B) (co-located A C) (co-located B A)
                           (co-located B C) (co-located C A) (co-located C B))
                    (or (model (co-located B A) (co-located C B))
                        (model (co-located C A))))))

(query :goal (co-located A C)
  :expect (model (co-located A C)))
```

### E3 — `lattice/synth/merged_states.ein`: one generator per state, on every state

*A brings B* and *C brings D*. An obligation needs one of A or C, and A with C
is dead, so ∅ is not in R. Every state is reached by exactly two commitments:
its minimal one, and that one plus the fact it brings. So every state gets a
`(merged …)`, each with a single `(model …)` from.

**Today:** `(query :goal (h ?x))`. **Under the new forms:**

```lisp
(solve
  :expect (region
            ;; state 1 — on the minimal line
            (model (h A) (h B))
            ;; state 2 — on the minimal line
            (model (h C) (h D))
            ;; state 3
            (model (h B) (h C) (h D))
            ;; state 4
            (model (h A) (h B) (h D))
            ;; state 5
            (model (h A) (h B) (h E))
            ;; state 6
            (model (h C) (h D) (h E))
            ;; state 7 — on the maximal line
            (model (h B) (h C) (h D) (h E))
            ;; state 8 — on the maximal line
            (model (h A) (h B) (h D) (h E))
            ;; state 1 is reached by 2 commitments; its minimal one
            (merged (model (h A) (h B))
                    (model (h A)))
            ;; state 2 is reached by 2 commitments; its minimal one
            (merged (model (h C) (h D))
                    (model (h C)))
            ;; state 3 is reached by 2 commitments; its minimal one
            (merged (model (h B) (h C) (h D))
                    (model (h B) (h C)))
            ;; state 4 is reached by 2 commitments; its minimal one
            (merged (model (h A) (h B) (h D))
                    (model (h A) (h D)))
            ;; state 5 is reached by 2 commitments; its minimal one
            (merged (model (h A) (h B) (h E))
                    (model (h A) (h E)))
            ;; state 6 is reached by 2 commitments; its minimal one
            (merged (model (h C) (h D) (h E))
                    (model (h C) (h E)))
            ;; state 7 is reached by 2 commitments; its minimal one
            (merged (model (h B) (h C) (h D) (h E))
                    (model (h B) (h C) (h E)))
            ;; state 8 is reached by 2 commitments; its minimal one
            (merged (model (h A) (h B) (h D) (h E))
                    (model (h A) (h D) (h E)))))

(query :goal (h ?x)
  :over maximal
  :expect (or (model (h A) (h B) (h D) (h E))
              (model (h B) (h C) (h D) (h E))))
```

### E4 — `lattice/02_genuine_3set_death.ein`: three maximal states, two answers

Every pair of the three properties is alive and the triple is dead. ∅ is in
R, and its model is `(model)`, because root holds no fact of the three named
relations. Asked over the maximal line, the goal `(a-prop ?x)` gets two
classes from three states: two of them hold `(a-prop X)` and one does not.

**Today:**

```lisp
(query :goal (a-prop ?x)
  :hrules (guess (a-prop Item) guess (b-prop Item) guess (c-prop Item)))
```

**Under the new forms:**

```lisp
(solve
  :hrules (guess (a-prop Item) guess (b-prop Item) guess (c-prop Item))
  :expect (region
            ;; state 1 — ∅, root itself, on the minimal line
            (model)
            ;; state 2
            (model (a-prop X))
            ;; state 3
            (model (b-prop X))
            ;; state 4 — on the maximal line
            (model (a-prop X) (b-prop X))
            ;; state 5
            (model (c-prop X))
            ;; state 6 — on the maximal line
            (model (a-prop X) (c-prop X))
            ;; state 7 — on the maximal line
            (model (b-prop X) (c-prop X))))

(query :goal (a-prop ?x)
  :expect (or (model)
              (model (a-prop X))))

(query :goal (a-prop ?x)
  :over maximal
  :expect (or (model)
              (model (a-prop X))))
```

### E5 — `features/11_expect_ambiguity.ein`: today's claim, migrated

Today's one `(or …)` claim lists R's **maximal line**: R has seven states, and
the claim lists two. Read as a region claim it would be false. It is a pure
goal projection, so it moves to a query `:over maximal`, word for word. § 7
of the diff names it; it is one of the two claims that hold on the maximal
line only.

**Today:**

```lisp
(query
  :goal   (seat ?who ?slot)
  :hrules (guess (Ann S1) (Ann S2) (Bob S1) (Bob S2))
  :expect (or (model (seat Ann S1) (seat Bob S2))
              (model (seat Ann S2) (seat Bob S1))))
```

**Under the new forms:**

```lisp
(solve
  :hrules (guess (Ann S1) (Ann S2) (Bob S1) (Bob S2))
  :expect (region
            ;; state 1 — ∅, root itself, on the minimal line
            (model)
            ;; state 2
            (model (seat Ann S1))
            ;; state 3
            (model (seat Ann S2))
            ;; state 4
            (model (seat Bob S1))
            ;; state 5 — on the maximal line
            (model (seat Ann S2) (seat Bob S1))
            ;; state 6
            (model (seat Bob S2))
            ;; state 7 — on the maximal line
            (model (seat Ann S1) (seat Bob S2))))

(query :goal (seat ?who ?slot)
  :over maximal
  :expect (or (model (seat Ann S1) (seat Bob S2))
              (model (seat Ann S2) (seat Bob S1))))
```

### E6 — `tests/stdlib/closure/03_closed_and_owing.ein`: R empty, root owes

n = 0, and root owes one `(total-owed …)` instance, so R is empty of the
second kind. Today's claim pins root's `(closed-here r)`, and the checker reads
it off the open state. Under R there is no state of R for it to be about, so
it moves into `(owing …)` with the same fact. Root's fixpoint holds exactly
that one `closed-here` fact.

**Today:**

```lisp
(query :goal (closed-here ?R)
  :no-hypothesis (is-a closed-here)
  :expect (model (closed-here r)))
```

**Under the new forms:**

```lisp
(solve
  :no-hypothesis (is-a closed-here)
  :expect (region
            (owing (model (closed-here r)))))

(query :goal (closed-here ?R))
```

The query states nothing, because R has no state to answer from. Today's
`(false)` would also be true there, and so would `(region)`.

### E7 — `tests/stdlib/algebra/09_irreflexive_violated.ein`: every node dead

Root holds `(loops A A)`, which `irreflexive` refutes. `(false)` keeps its
spelling and its meaning.

**Today:**

```lisp
(query :goal (loops ?x ?y) :no-hypothesis (loops) :expect (false))
```

**Under the new forms:**

```lisp
(solve :no-hypothesis (loops) :expect (false))

(query :goal (loops ?x ?y))
```

### E8 — `examples/zebra2.ein`: the puzzle

R is one state with 261 982 commitments, and route 1 finds the state in 0.95 s
(`ein solve examples/zebra2.ein --maximal --region-budget 2097152 -j 8`). The
claim lists the state's 25 `*-loc` facts, 7 of them root's. It has no
`(merged …)`: its minimal commitments are not counted here, and listing them
would be the claim's bulk. So it claims k_s = 1 and not k_c. The query is the
puzzle's question, answered by four facts.

**Today:** `:hrules` sits on the `(query …)` (`examples/zebra2.ein`, the last
form). **Under the new forms:**

```lisp
(solve
  :hrules (guess
            (color-loc  Color       House)
            (nation-loc Nationality House)
            (drink-loc  Drink       House)
            (smoke-loc  Cigarette   House)
            (pet-loc    Pet         House))
  :expect (region
            ;; the one state. No (merged …): the claim is about states, k_s = 1
            (model (color-loc Blue House-2) (color-loc Green House-5)
                   (color-loc Ivory House-4) (color-loc Red House-3)
                   (color-loc Yellow House-1)
                   (drink-loc Coffee House-5) (drink-loc Juice House-4)
                   (drink-loc Milk House-3) (drink-loc Tea House-2)
                   (drink-loc Water House-1)
                   (nation-loc Englishman House-3)
                   (nation-loc Japanese House-5)
                   (nation-loc Norwegian House-1)
                   (nation-loc Spaniard House-4)
                   (nation-loc Ukrainian House-2)
                   (pet-loc Dog House-4) (pet-loc Fox House-1)
                   (pet-loc Horse House-2) (pet-loc Snail House-3)
                   (pet-loc Zebra House-5)
                   (smoke-loc Chesterfields House-2)
                   (smoke-loc Kools House-1) (smoke-loc Lucky_Strike House-4)
                   (smoke-loc Old_Gold House-3)
                   (smoke-loc Parliament House-5))))

(query
  :goal (and (drink-loc  Water      ?h_water)
             (nation-loc ?who_water ?h_water)
             (pet-loc    Zebra      ?h_zebra)
             (nation-loc ?who_zebra ?h_zebra))
  :goal-text "The {?who_water} drinks water in {?h_water}; the {?who_zebra} owns zebra in {?h_zebra}"
  :expect (model (drink-loc Water House-1)
                 (nation-loc Japanese House-5) (nation-loc Norwegian House-1)
                 (pet-loc Zebra House-5)))
```

## 3. What the examples show

These are properties of the forms as ruled, visible only on real regions. None
of them needs a change, and each can be read as a reason for one.

1. **Root's facts repeat in every model** when the claim names their
   relation. In E1, six of every model's facts are root's. In E2, three are. This
   is today's rule, *naming a relation closes it*, kept because 57 corpus
   claims are on programs with no candidate and pin root's facts (the diff's
   § 2). The printer can mark root's facts with a comment, but a claim cannot
   leave them out.
2. **A derived mirror is listed beside its candidate**:
   `(co-located Bob Coffee)` and `(co-located Coffee Bob)` in E1. It is the
   same rule.
3. **`<to>` repeats a listed state in full**: 12 facts twice in E1. That is
   the ruled form. It makes each `(merged …)` readable on its own.
4. **A goal's type atoms are part of its class.** `(is-a Dog Pet)` is in E1's
   answer because the goal names `(is-a ?p Pet)`.
5. **`:over all` and `:over maximal` can give the same classes from different
   states** (E4: seven states and three, two classes each).
6. **A region too large to read states only its states** (E8). With
   `(merged …)` it would pin k_c as well, at the price of listing every
   minimal commitment.

## 4. When accepted

T1e2.1a.4.2 builds the parser, the loader and the checker against `explore`,
plus the fixtures under `tests/`. Each example here becomes a test: its claim,
printed by `ein solve`, read back and held. The diff's § 7 migration stands as
ruled. The keyword move lands in this stage, and the claims move at the
switch.

## 5. Why `(owing …)` and not `(region)` — the user's question

**Nothing is wrong with `(region)` as a statement about R.** On E6 it is
exactly R, and it stays legal: R empty, of either kind. `(owing …)` is the one
item in a claim that is *not* about R. It lists the alive nodes outside R that
owe, which is what R's emptiness leaves to say. It exists for two reasons.

**1. Thirteen fixtures test what a rule derives, on a root that owes.** These
are § 2's 13. Five of them are about the obligation itself (`total-owed`,
`surjective-owed`, `closed-and-owing`, `owed-room`, `owed-fill`). The other
eight owe only because a `bijective` or slot declaration brings an obligation
with nothing to pay it. Their claims pin root's derived facts. R is empty
whether the rule under test works or not, so a claim about R alone cannot see
the rule. Measured with `utils/stdlib_mutants.py` on 2026-10-16 at `ad7d0336`,
whose mutants are defects spliced into a stdlib rule:

| `ein test` runs | mutants killed of 217 |
|---|---:|
| all of `tests/` (today) | 157 |
| `tests/` without the 13 (`--targets`, the other 44 files) | 126 |

So **31 kills come from those 13 claims alone.** Each of the 31 was then
replayed against the 13, with `EIN_STDLIB` pointing at the mutant's tree and
`ein solve -e`:

- **27** leave every fixture `Open`, and change only the facts the rule
  derives: `bijective-setup`'s four activators, `domain-` and
  `range-elimination`, and every `slot-*` premise swap. An empty `(region)`
  passes on all 27;
- **4** make `slots/07_spatial_prune` dead (`slot-adjacent-*`'s dropped
  `absent`). R is still empty, so `(region)`, read as *either kind*, passes on
  these too. A `(region)` that meant *empty, alive nodes owe* would catch them;
- **0** make R non-empty.

Writing `(region)` on the 13 would take the sweep from 157 to 126, and
`tests/mutation_survivors.txt` would grow by 31.

**2. It is what `ein solve` prints for that kind of empty R.** The read-out
has to say *why* R is empty: no node alive, or alive nodes that all owe and
what they hold (S1e2.1a.5's acceptance). With `(owing …)`, that answer is a
claim and reads back like every other one (ruling 1, one form for both
directions). Without it, the read-out says something the language cannot
state.

**The choice, for the user:**

- **(A) Keep `(owing …)`**: the 31 kills stay, and the printed form reads
  back. *Proposed.*
- **(B) `(region)` only**: the claim language speaks of R alone. The 13
  fixtures lose their fact pins, the sweep falls to 126, and the read-out
  prints the owing frontier outside the language.
