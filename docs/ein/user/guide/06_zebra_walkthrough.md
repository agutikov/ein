# 6 — The Zebra puzzle, step by step

The Wikipedia-style deductive solution of the Zebra puzzle, line by line, beside what Ein derives on the same puzzle.
The encoding is [`examples/zebra2.ein`](../../../../examples/zebra2.ein); conditions `(1)`–`(15)` are the
`:source "condition (N)"` of its puzzle facts. Names follow Ein's spelling (`House-1`, `Kools`, `Old_Gold`,
`Lucky_Strike`, `Chesterfields`); in prose `H_i` abbreviates `House-i`. Every rule name in the tables below is the one
Ein reports for that fact.

New to Ein? Start with the [guide](README.md) and return here from chapter 4.

`ein solve examples/zebra2.ein --trace out.md` writes Ein's own derivation of the answer, every step with its premises
and its `:why`. Driving the same solve from a program is [the Rust API](../../ein.rs/api.md), whose worked example
solves this puzzle.

> **The other encoding.** [`zebra.ein`](../../../../examples/zebra.ein) solves the same puzzle over one generic
> `co-located` equivalence instead of five typed relations, with the rules of `std.slots`, and reaches the same state
> ([examples](examples.md) compares the two). Its root places the same facts as `zebra2.ein`'s, under other rule
> names: where a row below says `range-elimination`, that encoding says `slot-fill`; where it says `adjacent-via-fwd`,
> `slot-adjacent-fwd`; where it says `co-located` for a placement, `slot-locate`.

## How to read it

Each human step is one of three things in Ein.

- **Root**: what saturation derives from the program with nothing guessed. The human's unconditional steps are all
  here. `ein saturate examples/zebra2.ein --dump` lists root, each derived fact with the rule that derived it, and
  `ein solve examples/zebra2.ein --no-search` reports it.
- **A supposition refuted**: the human's *"suppose … — impossible"*. The `guess` hrule proposes a fact for each pair of
  typed values, and `ein show hyps examples/zebra2.ein` lists every candidate. A candidate is **dead alone** when
  assuming it, and nothing else, saturates to `(false)`. To see it, add the candidate to a copy of `zebra2.ein` as a
  stated fact: `ein solve COPY --no-search` then reports `∅ dead — root's own fixpoint derives (false)`, and its
  `unsat core` lines name the facts one explanation of the contradiction rests on.
- **Therefore**: what holds in the answer's one state. Dead is upward-closed, so no state holds a candidate that is
  dead alone; and where root *owes* something (House-2 a cigarette, the fox a house), the candidates that would pay it
  are exhaustive, so when all but one are dead every state holds the last. These are the lines `L1` and `L5` of the
  read-out's `route` row ([the read-out](../reference/read-out.md)).

A rule a module brings is printed by its full name, `std.bijection.range-elimination`; the tables give the short name.
Two rules recur, and they are duals:

- **`range-elimination`** fixes a house and forces its value when every other value is excluded there; *"therefore
  the first house is yellow"* is this rule. **`domain-elimination`** fixes a value and forces its house. Both come with
  `(bijective R)`, from `std.bijection`.
- **`guess`** proposes; a dead candidate is a value the human excludes by trying it.

Ein does not follow the human's order. Its answer is the set of states the puzzle allows, and on this puzzle one
well-chosen guess saturates to the whole solution: `ein solve examples/zebra2.ein --minimal` lists the state's
minimal commitments, each a single candidate. What this page checks is that every step of the human argument is a
step Ein takes too.

## Condition cross-reference

| # | NL condition | ein fact |
|---|---|---|
| 1 | Five houses in a row. | `(right-of House-{i+1} House-i)` for i=1..4 |
| 2 | The Englishman lives in the red house. | `(co-located nation-loc Englishman color-loc Red)` |
| 3 | The Spaniard owns the dog. | `(co-located nation-loc Spaniard pet-loc Dog)` |
| 4 | Coffee is drunk in the green house. | `(co-located drink-loc Coffee color-loc Green)` |
| 5 | The Ukrainian drinks tea. | `(co-located nation-loc Ukrainian drink-loc Tea)` |
| 6 | The green house is immediately right of the ivory house. | `(adjacent-via right-of color-loc Ivory color-loc Green)` |
| 7 | The Old Gold smoker owns snails. | `(co-located smoke-loc Old_Gold pet-loc Snail)` |
| 8 | Kools are smoked in the yellow house. | `(co-located smoke-loc Kools color-loc Yellow)` |
| 9 | Milk is drunk in the middle house. | `(drink-loc Milk House-3)` |
| 10 | The Norwegian lives in the first house. | `(nation-loc Norwegian House-1)` |
| 11 | Chesterfields is smoked next to the house with the fox. | `(adjacent-via next-to smoke-loc Chesterfields pet-loc Fox)` |
| 12 | Kools is smoked next to the house with the horse. | `(adjacent-via next-to smoke-loc Kools pet-loc Horse)` |
| 13 | The Lucky Strike smoker drinks orange juice. | `(co-located smoke-loc Lucky_Strike drink-loc Juice)` |
| 14 | The Japanese smokes Parliaments. | `(co-located nation-loc Japanese smoke-loc Parliament)` |
| 15 | The Norwegian lives next to the blue house. | `(adjacent-via next-to nation-loc Norwegian color-loc Blue)` |

The puzzle's question, from its `(query …)`:

```text
(and (drink-loc Water ?h_water) (nation-loc ?who_water ?h_water)
     (pet-loc   Zebra ?h_zebra) (nation-loc ?who_zebra ?h_zebra))
```

> *Below are the deductive steps that reach the solution. The method is to fit the known relations into a table,
> progressively eliminating impossible options; key inferences are italicised.*

---

## Step 1 — the first two houses (all at root)

> **By (10) the Norwegian lives in the first house.** Numbering direction (left-to-right or right-to-left) is
> irrelevant; only the order matters.

| where | NL | ein rule | premises → conclusion |
|---|---|---|---|
| root | Norwegian in `H_1` | stated (10) | `(nation-loc Norwegian House-1)` |

> **From (10) and (15) the second house is blue.** What colour is the first house? Not green and not ivory: they must
> be adjacent (from (6)) and the second house is blue. Not red: that is the Englishman's (2). ***Therefore the first
> house is yellow.***

| where | NL | ein rule | premises → conclusion |
|---|---|---|---|
| root | Blue at `H_2` | `adjacent-via-fwd` (15) | Norwegian@`H_1`; `H_1`'s only `next-to` neighbour is `H_2` ⟹ `(color-loc Blue House-2)` |
| root | not Blue at `H_1` | `functional-negative` | Blue@`H_2` ⟹ `(not (color-loc Blue House-1))` |
| root | not Green at `H_1` | `adjacent-via-endpoint-bwd` (6) | Green is `right-of` Ivory, and `H_1` is right of no house ⟹ `(not (color-loc Green House-1))` |
| root | not Ivory at `H_1` | `adjacent-via-fwd-negative` (6) | Ivory@`H_1` would put Green at `H_2`, its only right neighbour, which is not green (Blue@`H_2`) ⟹ `(not (color-loc Ivory House-1))` |
| root | not Red at `H_1` | `co-located-negative` (2) | Norwegian@`H_1` ⟹ the Englishman is not in `H_1` (`injective-negative`) ⟹ `(not (color-loc Red House-1))` |
| root | **Yellow at `H_1`** | `range-elimination` | Blue, Green, Ivory, Red all excluded at `H_1` ⟹ `(color-loc Yellow House-1)` |

> Consequently **Kools are smoked in `H_1`** (8) and **a horse is kept in `H_2`** (12).

| where | NL | ein rule | premises → conclusion |
|---|---|---|---|
| root | Kools at `H_1` | `co-located` (8) | Yellow@`H_1` ⟹ `(smoke-loc Kools House-1)` |
| root | Horse at `H_2` | `adjacent-via-fwd` (12) | Kools@`H_1`; its only `next-to` neighbour is `H_2` ⟹ `(pet-loc Horse House-2)` |

> **What does the Norwegian, first house, yellow, Kools, drink?** Not tea (the Ukrainian's, 5); not coffee (the green
> house's, 4); not milk (the third house's, 9); not orange juice (the Lucky Strike smoker's, 13). ***Therefore the
> Norwegian drinks water,*** answering the first half of the puzzle.

Asked as a question, *what does `H_1` drink?* has five candidate answers, and four of them are excluded by stored
negatives before anything is guessed. Ein reaches the fifth the same way:

| where | NL | ein rule | premises → conclusion |
|---|---|---|---|
| root | not Tea at `H_1` | `co-located-negative` (5) | the Ukrainian is not in `H_1` ⟹ `(not (drink-loc Tea House-1))` |
| root | not Coffee at `H_1` | `co-located-negative` (4) | `H_1` is not green ⟹ `(not (drink-loc Coffee House-1))` |
| root | not Milk at `H_1` | `functional-negative` | Milk@`H_3` ⟹ `(not (drink-loc Milk House-1))` |
| root | not Juice at `H_1` | `co-located-negative` (13) | Kools@`H_1` ⟹ Lucky Strike is not in `H_1` ⟹ `(not (drink-loc Juice House-1))` |
| root | **Water at `H_1`** | `range-elimination` | Tea, Coffee, Milk, Juice excluded at `H_1` ⟹ `(drink-loc Water House-1)` |

The table at the end of the step, which is root:

```text
house   1          2      3      4   5
colour  yellow     blue   ?      ?   ?
nation  Norwegian  ?      ?      ?   ?
drink   water      ?      milk   ?   ?
smoke   Kools      ?      ?      ?   ?
pet     ?          horse  ?      ?   ?
```

---

## Step 2 — what is smoked in `H_2`

> **What is smoked in `H_2`** (blue, horse)? Not Kools (smoked in `H_1`, 8). Not Old Gold: that smoker keeps snails
> (7), and `H_2` has a horse.

| where | NL | ein rule | premises → conclusion |
|---|---|---|---|
| root | not Kools at `H_2` | `co-located-negative` (8) | `H_2` is not yellow ⟹ `(not (smoke-loc Kools House-2))` |
| root | not Old_Gold at `H_2` | `co-located-negative` (7) | Horse@`H_2` ⟹ no snail in `H_2` (`injective-negative`) ⟹ `(not (smoke-loc Old_Gold House-2))` |

Root owes `H_2` a cigarette, and three candidates are left to pay it: `(smoke-loc Chesterfields House-2)`,
`(smoke-loc Lucky_Strike House-2)` and `(smoke-loc Parliament House-2)`.

### Supposition: Lucky Strike in `H_2` (dead)

> *Suppose* Lucky Strike is smoked in `H_2`. By (13) orange juice is drunk there. Who can live there? Not the
> Norwegian (10); not the Englishman (red, 2); not the Spaniard (dog, 3); not the Ukrainian (tea, 5); not the Japanese
> (Parliament, 14). *Impossible*: so Lucky Strike is not smoked in `H_2`.

| where | NL | ein rule | premises → conclusion |
|---|---|---|---|
| supposed | Lucky_Strike@`H_2` | the candidate | `(smoke-loc Lucky_Strike House-2)` |
| supposed | Juice at `H_2` | `co-located` (13) | ⟹ `(drink-loc Juice House-2)` |
| root | not Norwegian@`H_2` | `functional-negative` | Norwegian@`H_1` |
| root | not Englishman@`H_2` | `co-located-negative` (2) | `H_2` is not red |
| root | not Spaniard@`H_2` | `co-located-negative` (3) | `H_2` has no dog: it has the horse |
| supposed | not Ukrainian@`H_2` | `co-located-negative` (5) | Juice@`H_2` ⟹ no tea in `H_2` |
| supposed | not Japanese@`H_2` | `co-located-negative` (14) | Lucky_Strike@`H_2` ⟹ no Parliament in `H_2` |
| supposed | **⊥** | `surjective` | every Nationality is excluded at `H_2` ⟹ `(false)` |

The candidate is **dead alone**. Added as a fact to a copy of `zebra2.ein`, `ein solve COPY --no-search` reports
`∅ dead — root's own fixpoint derives (false)` and an unsat core that names the human's argument: the Norwegian in
`H_1`, the Englishman and red, the Spaniard and the horse, tea, Parliament, and the supposition itself. A core is one
explanation of the contradiction, and the engine may close it through another value first, `total` instead of
`surjective`: a value with no house left rather than a house with no value.

### Supposition: Parliament in `H_2` (dead)

> *Suppose* Parliament is smoked in `H_2`. By (14) the Japanese lives there. What does he drink? Not tea (the
> Ukrainian's, 5); not coffee (green, 4); not milk (the middle house's, 9); not juice (Lucky Strike's, 13).
> *Impossible.*

| where | NL | ein rule | premises → conclusion |
|---|---|---|---|
| supposed | Parliament@`H_2` | the candidate | `(smoke-loc Parliament House-2)` |
| supposed | Japanese@`H_2` | `co-located` (14) | ⟹ `(nation-loc Japanese House-2)` |
| supposed | not Tea@`H_2` | `co-located-negative` (5) | the Ukrainian is not in `H_2` |
| root | not Coffee@`H_2` | `co-located-negative` (4) | `H_2` is not green |
| root | not Milk@`H_2`, not Water@`H_2` | `functional-negative` | Milk@`H_3`, Water@`H_1` |
| supposed | not Juice@`H_2` | `co-located-negative` (13) | Lucky Strike is not in `H_2` |
| supposed | **⊥** | `surjective` | every Drink is excluded at `H_2` ⟹ `(false)` |

This candidate is dead alone too.

### Therefore: Chesterfields in `H_2`

> ***Therefore Chesterfields are smoked in `H_2`.*** What nationality is in `H_2` (blue, Chesterfields, horse)? Not
> the Norwegian (10), not the Englishman (red, 2), not the Spaniard (dog, 3), not the Japanese (Parliament, 14).
> ***So the Ukrainian, and by (5) he drinks tea.***

Two of the three candidates that pay `H_2`'s cigarette are dead, so every state of the answer holds the third. With
it, saturation finishes the house:

| where | NL | ein rule | premises → conclusion |
|---|---|---|---|
| the state | **Chesterfields@`H_2`** | the only candidate left | `(smoke-loc Chesterfields House-2)` |
| the state | **Tea@`H_2`** | `range-elimination` | Water, Milk, Coffee excluded at `H_2` at root, Juice once Lucky Strike is not in `H_2` ⟹ `(drink-loc Tea House-2)` |
| the state | **Ukrainian@`H_2`** | `co-located` (5) | Tea@`H_2` ⟹ `(nation-loc Ukrainian House-2)` |

Ein reaches tea first and the Ukrainian from it; the human reaches the Ukrainian first. The two facts are the same.

---

## Step 3 — the fox: `H_1` or `H_3`

> Since Chesterfields are smoked in `H_2`, by (11) the fox is in `H_1` or `H_3`.

| where | NL | ein rule | premises → conclusion |
|---|---|---|---|
| the state | Fox ∉ {`H_2`, `H_4`, `H_5`} | `disjunctive-prune-fwd` (11) | Chesterfields@`H_2`, whose `next-to` neighbours are `H_1` and `H_3` ⟹ `(not (pet-loc Fox House-2))`, `(not (pet-loc Fox House-4))`, `(not (pet-loc Fox House-5))` |

### Supposition: the fox in `H_3` (dead)

> *Suppose the fox is in `H_3`.* What does the snail-owning Old Gold smoker (7) drink? Water and tea are already
> placed (steps 1, 2). Not juice (Lucky Strike's, 13). Not milk: milk is in `H_3`, which we assumed holds the fox.
> Only coffee remains; by (4) he lives in the green house.
>
> Then the green house has an Old-Gold-smoking, snail-keeping coffee drinker. Who? Not the Norwegian (`H_1`); not the
> Ukrainian (tea, 5); not the Englishman (red, 2); not the Japanese (Parliament, 14); not the Spaniard (dog, 3).
> *Impossible*: so the fox is in `H_1`, not `H_3`.

The human's argument uses steps 1 and 2. Ein finds `(pet-loc Fox House-3)` dead with no help from them: it is dead
alone, and so dead beside Chesterfields in `H_2` too, since dead is upward-closed. The argument's inferences are the
same families as before: `injective-negative` (the snail is not where the fox is), `co-located-negative` (7) (nor is
Old Gold), `range-elimination` on the Old Gold smoker's house, `co-located` (4) to the green house, and a totality
check, `surjective` or `total`, for the contradiction.

| where | NL | ein rule | premises → conclusion |
|---|---|---|---|
| the state | **Fox@`H_1`** | the only candidate left | of the fox's houses, `H_2`, `H_4`, `H_5` are excluded and `H_3` is dead ⟹ `(pet-loc Fox House-1)` |

---

## Step 4 — Old Gold, then everything

> From the above, **coffee and orange juice are in `H_4` and `H_5`** (order still undetermined).

| where | NL | ein rule | premises → conclusion |
|---|---|---|---|
| the state | Coffee, Juice ∉ {`H_1`, `H_2`, `H_3`} | `injective-negative`, `co-located-negative` | Water@`H_1`, Tea@`H_2`, Milk@`H_3` hold the first three houses |
| the state | Lucky Strike ∉ {`H_1`, `H_2`, `H_3`} | `co-located-negative` (13), `injective-negative` | it is where the juice is |

> Where does the Old Gold and snail keeper live? Not the juice house: that smokes Lucky Strike (13). *Suppose he lives
> in the coffee house.* Then Old Gold, snail and coffee are in the green house (4): the same impossibility as step 3.

With Chesterfields in `H_2` and the fox in `H_1`, Old Gold is left `H_3`, `H_4` or `H_5`. `H_4` and `H_5` are the
coffee house and the juice house in one order or the other, and each is dead beside the two earlier facts: `ein solve`
on a copy of `zebra2.ein` stating `(smoke-loc Chesterfields House-2)`, `(pet-loc Fox House-1)` and
`(smoke-loc Old_Gold House-4)` (or `House-5`) reports root dead, the contradiction closed by `surjective`.

> Then Parliament is smoked in the green house, where coffee is drunk and the Japanese lives (14). The Spaniard lives
> in the ivory house, because the red one is the Englishman's. The ivory house holds the Spaniard and his dog, so it
> cannot be `H_3` (snails). Since ivory is immediately left of green (6), they are `H_4` and `H_5`. Therefore red is
> `H_3` and **the Englishman lives there.** All cells are filled but one: **the Japanese keeps the zebra.**

Ein derives the rest from Old Gold in `H_3`, in its own order:

| where | NL | ein rule | premises → conclusion |
|---|---|---|---|
| the state | **Old_Gold@`H_3`** | the only candidate left | `(smoke-loc Old_Gold House-3)` |
| the state | Snail@`H_3` | `co-located` (7) | ⟹ `(pet-loc Snail House-3)` |
| the state | **Englishman@`H_3`** | `range-elimination` | the Norwegian is in `H_1`, the Ukrainian in `H_2`, the Spaniard's dog and the Japanese's Parliament are not in `H_3` ⟹ `(nation-loc Englishman House-3)` |
| the state | **Red@`H_3`** | `co-located` (2) | ⟹ `(color-loc Red House-3)` |
| the state | **Ivory@`H_4`, Green@`H_5`** | `domain-elimination` | green is right of ivory (6), and `H_1` to `H_3` are coloured ⟹ `(color-loc Ivory House-4)`, `(color-loc Green House-5)` |
| the state | Coffee@`H_5` | `co-located` (4) | Green@`H_5` ⟹ `(drink-loc Coffee House-5)` |
| the state | Juice@`H_4`, Lucky_Strike@`H_4` | `domain-elimination`, then `co-located` (13) | ⟹ `(drink-loc Juice House-4)`, `(smoke-loc Lucky_Strike House-4)` |
| the state | Parliament@`H_5`, Japanese@`H_5` | `range-elimination`, `co-located` (14) | ⟹ `(smoke-loc Parliament House-5)`, `(nation-loc Japanese House-5)` |
| the state | Spaniard@`H_4`, Dog@`H_4` | `domain-elimination`, `co-located` (3) | ⟹ `(nation-loc Spaniard House-4)`, `(pet-loc Dog House-4)` |
| the state | **Zebra@`H_5`** | `domain-elimination` | the fox, horse, snail and dog hold `H_1` to `H_4` ⟹ `(pet-loc Zebra House-5)` |
| the state | **the query** | — | `(drink-loc Water House-1)` and `(pet-loc Zebra House-5)` hold, with the Norwegian in `H_1` and the Japanese in `H_5` |

The final table, the one state of the answer:

```text
house   1          2           3         4              5
colour  yellow     blue        red       ivory          green
nation  Norwegian  Ukrainian   English   Spaniard       Japanese
drink   water      tea         milk      orange juice   coffee
smoke   Kools      Chest'flds  Old Gold  Lucky Strike   Parliament
pet     fox        horse       snail     dog            zebra
```

`ein solve examples/zebra2.ein --single` prints it as the `(model …)` of chapter 4, and reads the query's sentence
off it: *“The Norwegian drinks water in House-1; the Japanese owns zebra in House-5”*.

---

## The suppositions, together

| supposition | refuted by | dead |
|---|---|---|
| Lucky Strike in `H_2` | no nationality left for `H_2` | alone |
| Parliament in `H_2` | no drink left for `H_2` | alone |
| the fox in `H_3` | no nationality left for the green house | alone |
| Old Gold in the coffee house (`H_4` or `H_5`) | the same as the fox in `H_3` | beside Chesterfields in `H_2` and the fox in `H_1` |

No supposition is nested in another. The human solution is a flat sequence of single suppositions, each refuted
outright; what holds between them, at root and in the state, shrinks the next one's choices, which is why each
refutation is shorter than the one before (*"we already excluded water and tea on the previous step"*, step 3).

## Remark on numbering direction

The Wikipedia text notes that numbering direction is irrelevant: flipping left and right mirrors the table and keeps
the answer (the Norwegian still drinks water, the Japanese still keeps the zebra). In `zebra2.ein` that freedom is
broken by the explicit `right-of` chain and the asymmetric condition (6); a flipped encoding would re-orient those
facts and never the `co-located` ones: the puzzle's constraint graph is invariant under the reflection.

## Two variants: too few clues, and one too many

Two thin variants of `zebra2.ein`, with the same ontology and rules, ask the two other questions a puzzle can pose:
*which answers are there?* when the clues leave it open, and *why is there none?* when they contradict each other.
Both run through the same `ein solve`, and what the answer is decides which question was asked; no flag selects it.

- [`zebra2-minus-15.ein`](../../../../examples/zebra2-minus-15.ein) drops condition (15), the only fact that pins Blue
  at `H_2`, and leaves the colours under-determined. `ein solve examples/zebra2-minus-15.ein --single` answers
  `Ambiguity — R holds two states or more`; `--count` counts R's states, and `--maximal --models key` prints them as
  their **determining key**, the few varying slots whose values fix a whole state, instead of as one block a state.
- [`ein-bugs/zebra2-bad.ein`](../../../../examples/ein-bugs/zebra2-bad.ein) adds `(color-loc Green House-1)`, which
  collides with condition (6): green is right of ivory, so it cannot be in the leftmost house. `ein solve` answers
  that R is empty, `every node is dead — root's own fixpoint derives (false)`, with the unsat core. It lives under
  `ein-bugs/` because it is well formed: it is unsatisfiable, not malformed.

Both are generated from `zebra2.ein`, so their shared ontology, rules, `:why` texts and `:goal-text` cannot drift from
it, and the test suite holds each to being `zebra2.ein` with that one change.
