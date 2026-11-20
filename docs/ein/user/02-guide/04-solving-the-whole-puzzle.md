# 4 — Solving the whole puzzle

You've met atoms and facts (Ch.1), how a rule derives new facts (Ch.2),
and the rule families that prune the search (Ch.3). Now we assemble the real
puzzle, solve it, and hand off to the full trace.

## What `zebra2.ein` is made of

Open [`examples/zebra2.ein`](../../../../examples/zebra2.ein) — it's just the
pieces from this guide, in layers:

```lisp
(import std.algebra   :symbols (symmetric transitive includes))   ; the library logic
(import std.bijection :symbols (bijective-setup typecheck-setup))

(relation nation-loc Nationality House :why "the {?3} lives in {?4}")   ; the relations, their slots labelled
(relation color-loc  Color       House :why "{?4} is painted {?3}")
(relation drink-loc  Drink       House :why "{?3} is drunk in {?4}")
(relation smoke-loc  Cigarette   House :why "{?3} is smoked in {?4}")
(relation pet-loc    Pet         House :why "the {?3} is kept in {?4}")
;; … right-of, next-to, is-a and is-a* likewise

(symmetric next-to) (transitive is-a*)              ; activate the properties
(bijective color-loc) (bijective nation-loc) …

(rule co-located …) (rule adjacent-via-fwd …) (rule disjunctive-prune-fwd …) …  ; the spatial rules

(hrule guess (?R ?T1 ?T2)                           ; what may be guessed
  :match  (and (is-a ?a ?T1) (is-a ?b ?T2))
  :assert (?R ?a ?b)
  :why    "guess: ({?R} {?a} {?b})?")
(solve :hrules (guess (color-loc Color House) (nation-loc Nationality House) (drink-loc Drink House)
                      (smoke-loc Cigarette House) (pet-loc Pet House)))

(is-a Norwegian Nationality) …                      ; the is-a facts
(nation-loc Norwegian House-1 :source "condition (10)")   ; the 15 clues
(co-located nation-loc Englishman color-loc Red :source "condition (2)")
…

(query                                              ; the question
  :goal (and (drink-loc Water ?h_water)  (nation-loc ?who_water ?h_water)
             (pet-loc   Zebra ?h_zebra)  (nation-loc ?who_zebra ?h_zebra))
  :goal-text "The {?who_water} drinks water in {?h_water}; the {?who_zebra} owns zebra in {?h_zebra}")
```

The pieces, and how `ein solve` turns them into the answer:

![The pieces of zebra2.ein, and ein solve turning them into the answer](diagrams/04-zebra2-pieces.svg)

Nothing here is new — it's Chapters 1–3 at full size. The `(hrule guess …)`
is Chapter 2's `guess`, written out: it proposes `(?R ?a ?b)` for every `?a`
with an `is-a` fact into one name and every `?b` with one into another, and
`(solve :hrules …)` names the five relations it runs over, each with those
two names: for `color-loc`, every colour against every house. The names are
the ones the declarations label their slots with, and it is this clause and
the hrule's `is-a` premises that make them say what a slot is filled from;
the declaration alone does not. The `(query …)` is the only piece we haven't
used: a `:goal` pattern Ein matches against the solved KB, and a `:goal-text`
template that renders the headline answer.

The hrule drawn, from `ein render rule --name guess examples/zebra2.ein`: it
matches two facts of `is-a`, each an octagon whose arrow `1` is at the box
`is-a`, and proposes the dashed one, a fact whose arrow `1` is at the
variable `?R`.

![The hrule guess: two is-a pattern octagons it matches and the dashed ?R fact each match proposes](diagrams/04-rule-guess.svg)

## Solve it

<!-- transcript: ein solve examples/zebra2.ein --single -->
```sh
$ ein solve examples/zebra2.ein --single
solve · examples/zebra2.ein --single
──────────────────────────────────────────────────────────────
  asked             is R one state? (--single)
  verdict           Solution — R is one state
  decided           yes — R holds no other state
  lattice           monotone (the static check) — R is convex: no hole
  universe          56 candidates
    outside         is-a*, next-to — no candidate: no obligation owes them. R holds of them only what the program states and saturation derives
  route             the obligation tree, over states, branching on the owed instance with the fewest alternatives left, the first by text among equals — L1 dead is upward-closed · L2 discharged is upward-closed · L3 a commitment's state is its closure's · L4 the lines are local · L5 an owed instance's alternatives are exhaustive · L8 what a state denies is dead above it
  work              7 saturations · 278 facts beyond root

  ;; state 1 of 1 — 18 candidates · from {(color-loc Green House-5), (drink-loc Juice House-4)}
  (model (color-loc Blue House-2) (color-loc Green House-5)
         (color-loc Ivory House-4) (color-loc Red House-3)
         (color-loc Yellow House-1) (drink-loc Coffee House-5)
         (drink-loc Juice House-4) (drink-loc Milk House-3)
         (drink-loc Tea House-2) (drink-loc Water House-1)
         (nation-loc Englishman House-3) (nation-loc Japanese House-5)
         (nation-loc Norwegian House-1) (nation-loc Spaniard House-4)
         (nation-loc Ukrainian House-2) (pet-loc Dog House-4)
         (pet-loc Fox House-1) (pet-loc Horse House-2) (pet-loc Snail House-3)
         (pet-loc Zebra House-5) (smoke-loc Chesterfields House-2)
         (smoke-loc Kools House-1) (smoke-loc Lucky_Strike House-4)
         (smoke-loc Old_Gold House-3) (smoke-loc Parliament House-5))

  query             (and (drink-loc Water ?h_water) (nation-loc ?who_water ?h_water) (pet-loc Zebra ?h_zebra) (nation-loc ?who_zebra ?h_zebra))
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?h_water = House-1 · ?h_zebra = House-5 · ?who_water = Norwegian · ?who_zebra = Japanese
                        “The Norwegian drinks water in House-1; the Japanese owns zebra in House-5”
  as :expect        (model (drink-loc Water House-1)
                           (nation-loc Japanese House-5)
                           (nation-loc Norwegian House-1)
                           (pet-loc Zebra House-5))
```
<!-- /transcript -->

`--single` asks *is the answer one state?* — and the verdict word, `Solution`,
says yes. The state is listed as a `(model …)` of the guessed facts, and the
query block reads the goal off it. The headline sentence came from the
puzzle's own `:goal-text` template — Ein has no built-in vocabulary. Had the
puzzle two answers, the word would be `Ambiguity`, with two of them shown;
none, `Contradiction` (with the facts that clash) or `Open` (with what is
still owed).

## Three next moves

```sh
ein solve examples/zebra2.ein                 # describe every state the puzzle allows
ein solve examples/zebra2.ein --trace out.md  # write a full markdown derivation trace
ein solve examples/zebra2.ein -s              # + the wall clock
```

A bare `ein solve` describes the whole set of states — R — with its counts,
its lines and what it cost. On this puzzle it classifies every commitment
(`exhausted yes`) and finds the one state `--single` showed; `--single` asks
only whether R is one state, and its `work` row is smaller for it. `--trace`
writes the derivation step by step, each step's premises and what it derives,
with inline graphs.

## Where to go next

- **The full human-style solution, step by step**:
  [chapter 6](06-zebra-walkthrough.md). This guide taught the pieces; the
  walkthrough shows them firing on the real puzzle, line by line against the
  Wikipedia solution. **Read it now**: it's the natural sequel to this
  chapter.
- **What the answer is, and every row it prints**:
  [solutions and models](../03-syntax-and-semantics/13-solutions-and-models.md)
  and [the read-out](../04-reference/08-read-out.md).
- **Drive Ein from your own program**: [the Rust API](../../ein.rs/04-api.md)
  (load a `.ein`, call `solve`, read the answer and trace). From a language
  that is not Rust, run the binary and read `--json-summary` / `--events`;
  there is no binding for another language
  ([Q-M1a.23](../../../decisions/engine/Q-M1a.23-ein-has-no-binding-for-another-language.md)).
- **How the engine finds the answer**: [finding R](../../reasoning/02-kb-evolution/07-finding-r.md).
  This guide deliberately never opened that box.

