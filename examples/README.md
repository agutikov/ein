# Zebra puzzle — human solution traced as `ein.py` inference

The Wikipedia-style deductive solution to the Zebra puzzle, translated to
English line-by-line and annotated with the ein facts, rules and hypotheses
each NL step corresponds to. The encoding under analysis is
[`zebra2.ein`](zebra2.ein); references to conditions `(1)`–`(15)` match the
numbering in its `(facts …)` block. Constants (`House_1`, `Kools`,
`Old_Gold`, `Lucky_Strike`, `Chesterfields`) follow the ein spelling; in
prose `H_i` abbreviates `House_i`.

## How to read the trace

- **Branch depth `d`.** `d = 0` is unconditional saturation. `d = 1` is the
  scope inside the outer-most hypothesis. A contradiction at depth `d`
  backjumps to `d−1` and asserts the negation of the entering hypothesis as
  a learnt no-good clause (S1.5a.18). The human solution stays flat — no
  branch ever reaches `d = 2`.
- Each row names the **ein rule** firing and the **premises → conclusion**
  it consumes/produces. Rule names match the `(rule …)` blocks in
  [`zebra2.ein`](zebra2.ein).
- Two mechanisms recur and are duals of each other:
  - **`domain-elimination`** ("X must be Y because *not* A, *not* B, *not* C,
    *not* D"): forces the only surviving value when all but one have been
    explicitly excluded on a bijective relation. The NL phrasing
    *"Therefore the first house is yellow"* is exactly this rule.
  - **`guess`** ("what does X drink? — try tea, try coffee, …"): an open
    `(?R ?a ?b)` query enumerates one hypothesis per typed instance;
    stored negatives prune the branch space. The NL phrasing *"What does
    the Norwegian drink? — Not tea … not coffee … not milk … not juice …
    therefore water"* is the same conclusion reached bottom-up — the
    saturator prefers `domain-elimination` because it short-circuits the
    branch.

## Condition cross-reference

| # | NL condition | ein fact |
|---|---|---|
| 1 | Five houses in a row. | `(right-of House_{i+1} House_i)` for i=1..4 |
| 2 | The Englishman lives in the red house. | `(co-located nation-loc Englishman color-loc Red)` |
| 3 | The Spaniard owns the dog. | `(co-located nation-loc Spaniard pet-loc Dog)` |
| 4 | Coffee is drunk in the green house. | `(co-located drink-loc Coffee color-loc Green)` |
| 5 | The Ukrainian drinks tea. | `(co-located nation-loc Ukrainian drink-loc Tea)` |
| 6 | The green house is immediately right of the ivory house. | `(adjacent-via right-of color-loc Ivory color-loc Green)` |
| 7 | The Old Gold smoker owns snails. | `(co-located smoke-loc Old_Gold pet-loc Snail)` |
| 8 | Kools are smoked in the yellow house. | `(co-located smoke-loc Kools color-loc Yellow)` |
| 9 | Milk is drunk in the middle house. | `(drink-loc Milk House_3)` |
| 10 | The Norwegian lives in the first house. | `(nation-loc Norwegian House_1)` |
| 11 | Chesterfields is smoked next to the house with the fox. | `(adjacent-via next-to smoke-loc Chesterfields pet-loc Fox)` |
| 12 | Kools is smoked next to the house with the horse. | `(adjacent-via next-to smoke-loc Kools pet-loc Horse)` |
| 13 | The Lucky Strike smoker drinks orange juice. | `(co-located smoke-loc Lucky_Strike drink-loc Juice)` |
| 14 | The Japanese smokes Parliaments. | `(co-located nation-loc Japanese smoke-loc Parliament)` |
| 15 | The Norwegian lives next to the blue house. | `(adjacent-via next-to nation-loc Norwegian color-loc Blue)` |

Puzzle query (from the `(query …)` block):

```
(and (drink-loc Water ?h_water)
     (pet-loc   Zebra ?h_zebra))
```

> *Below are the deductive steps that reach the solution. The method is to
> fit the known relations into a table, progressively eliminating
> impossible options; key inferences are italicised.*

---

## Step 1 — placing the first two houses (all `d = 0`)

> **By (10) the Norwegian lives in the first house.** Numbering direction
> (left-to-right vs right-to-left) is irrelevant — only the order matters.

| d | NL | ein rule | premises → conclusion |
|---|---|---|---|
| 0 | Norwegian in `H_1` | input fact (10) | `(nation-loc Norwegian House_1)` |

> **From (10) and (15) the second house is blue.** What colour is the first
> house? Not green and not ivory — they must be adjacent (from (6) and the
> fact that H_2 is blue). Not red — that is the Englishman's (2).
> ***Therefore the first house is yellow.***

| d | NL | ein rule | premises → conclusion |
|---|---|---|---|
| 0 | Blue at `H_2` | `(adjacent-via-fwd next-to nation-loc Norwegian color-loc Blue)` | Norwegian@`H_1`; H_1's only `next-to` neighbour is `H_2` ⟹ `(color-loc Blue House_2)` |
| 0 | not Blue at `H_1` | `(functional color-loc)` → negative-derivation | Blue@`H_2` ⟹ `(not (color-loc Blue House_1))` |
| 0 | not Green at `H_1` | `(disjunctive-prune-bwd right-of color-loc Ivory color-loc Green)` | Green is `right-of` Ivory (6); `H_1` has no left neighbour ⟹ `(not (color-loc Green House_1))` |
| 0 | not Ivory at `H_1` | `(disjunctive-prune-fwd right-of color-loc Ivory color-loc Green)` | Ivory@`H_1` would force Green@`H_2`, conflict with Blue@`H_2` ⟹ `(not (color-loc Ivory House_1))` |
| 0 | not Red at `H_1` | `(co-located nation-loc Englishman color-loc Red)` + `(functional nation-loc)` | Norwegian@`H_1` ⟹ Englishman ≠ `H_1` ⟹ Red ≠ `H_1` |
| 0 | **Yellow at `H_1`** | `(domain-elimination color-loc)` | Blue/Green/Ivory/Red all excluded at `H_1` ⟹ `(color-loc Yellow House_1)` |

> Consequently **Kools are smoked in `H_1`** (8) and **a horse is kept in
> `H_2`** (12).

| d | NL | ein rule | premises → conclusion |
|---|---|---|---|
| 0 | Kools at `H_1` | `(co-located smoke-loc Kools color-loc Yellow)` | Yellow@`H_1` ⟹ `(smoke-loc Kools House_1)` |
| 0 | Horse at `H_2` | `(adjacent-via-fwd next-to smoke-loc Kools pet-loc Horse)` | Kools@`H_1`; unique next-to neighbour `H_2` ⟹ `(pet-loc Horse House_2)` |

> **What does the Norwegian — first house, yellow, Kools — drink?** Not tea
> (Ukrainian — 5); not coffee (green house — 4); not milk (third house —
> 9); not orange juice (Lucky Strike smoker — 13). ***Therefore the
> Norwegian drinks water,*** answering the first half of the puzzle.

This is the user-cited example: the open query `(drink-loc House_1 ?d)`
would have `(guess drink-loc Drink House)` enumerate five hypotheses — Tea,
Coffee, Milk, Juice, Water — of which four are killed by stored negatives
derived from (4), (5), (9), (13). The compact dual is
`(domain-elimination drink-loc)`:

| d | NL | ein rule | premises → conclusion |
|---|---|---|---|
| 0 | not Tea at `H_1` | `(co-located nation-loc Ukrainian drink-loc Tea)` + `(functional nation-loc)` | Norwegian@`H_1` ⟹ Ukrainian ≠ `H_1` ⟹ Tea ≠ `H_1` |
| 0 | not Coffee at `H_1` | `(co-located drink-loc Coffee color-loc Green)` + `(functional color-loc)` | Yellow@`H_1` ⟹ Green ≠ `H_1` ⟹ Coffee ≠ `H_1` |
| 0 | not Milk at `H_1` | `(functional drink-loc)` → negative | Milk@`H_3` ⟹ `(not (drink-loc Milk House_1))` |
| 0 | not Juice at `H_1` | `(co-located smoke-loc Lucky_Strike drink-loc Juice)` + `(functional smoke-loc)` | Kools@`H_1` ⟹ Lucky_Strike ≠ `H_1` ⟹ Juice ≠ `H_1` |
| 0 | **Water at `H_1`** | `(domain-elimination drink-loc)` | Tea/Coffee/Milk/Juice excluded at `H_1` ⟹ `(drink-loc Water House_1)` |

End-of-step state:

```
house   1          2      3      4   5
colour  yellow     blue   ?      ?   ?
nation  Norwegian  ?      ?      ?   ?
drink   water      ?      milk   ?   ?
smoke   Kools      ?      ?      ?   ?
pet     ?          horse  ?      ?   ?
```

---

## Step 2 — first hypothesis branching (smoke at `H_2`)

> **What is smoked in `H_2`** (blue, horse)? Not Kools (already in `H_1` —
> 8). Not Old Gold — that smoker keeps snails (7), but `H_2` has a horse.

Unconditional negatives at `d = 0`:

| d | NL | ein rule | premises → conclusion |
|---|---|---|---|
| 0 | not Kools at `H_2` | `(functional smoke-loc)` → negative | Kools@`H_1` ⟹ `(not (smoke-loc Kools House_2))` |
| 0 | not Old_Gold at `H_2` | `(co-located smoke-loc Old_Gold pet-loc Snail)` + `(functional pet-loc)` | Horse@`H_2` ⟹ Snail ≠ `H_2` ⟹ Old_Gold ≠ `H_2` |

Surviving hypotheses on `(smoke-loc ?c House_2)`: Lucky_Strike, Parliament,
Chesterfields.

### Branch H_2.A — Lucky_Strike@`H_2`  (refuted)

> *Suppose* Lucky Strike is smoked in `H_2`. By (13) orange juice is drunk
> there. Who can live there? Not the Norwegian (10); not the Englishman
> (red — 2); not the Spaniard (dog — 3); not the Ukrainian (tea — 5); not
> the Japanese (Parliament — 14). *Impossible* — so Lucky Strike is not
> smoked in `H_2`.

| d | NL | ein rule | premises → conclusion |
|---|---|---|---|
| 1 | **Hypothesis** | `(guess smoke-loc Cigarette House)` opens branch | `(smoke-loc Lucky_Strike House_2)` |
| 1 | Juice at `H_2` | `(co-located smoke-loc Lucky_Strike drink-loc Juice)` | ⟹ `(drink-loc Juice House_2)` |
| 1 | not Norwegian@`H_2` | `(functional nation-loc)` | Norwegian@`H_1` ⟹ `(not (nation-loc Norwegian House_2))` |
| 1 | not Englishman@`H_2` | (2) + `(functional color-loc)` | Blue@`H_2` ⟹ Red ≠ `H_2` ⟹ Englishman ≠ `H_2` |
| 1 | not Spaniard@`H_2` | (3) + `(functional pet-loc)` | Horse@`H_2` ⟹ Dog ≠ `H_2` ⟹ Spaniard ≠ `H_2` |
| 1 | not Ukrainian@`H_2` | (5) + `(functional drink-loc)` | Juice@`H_2` ⟹ Tea ≠ `H_2` ⟹ Ukrainian ≠ `H_2` |
| 1 | not Japanese@`H_2` | (14) + `(functional smoke-loc)` | Lucky_Strike@`H_2` ⟹ Parliament ≠ `H_2` ⟹ Japanese ≠ `H_2` |
| 1 | **⊥** | `(total nation-loc)` | every Nationality excluded at `H_2` ⟹ `(false)` |
| 1→0 | learn no-good | path-condition learning (S1.5a.18) | `(not (smoke-loc Lucky_Strike House_2))` lifted to `d=0` |

### Branch H_2.B — Parliament@`H_2`  (refuted)

> *Suppose* Parliament is smoked in `H_2`. By (14) the Japanese lives
> there. What does he drink? Not tea (Ukrainian — 5); not coffee (green —
> 4); not milk (middle — 9); not juice (Lucky Strike — 13). *Impossible.*

| d | NL | ein rule | premises → conclusion |
|---|---|---|---|
| 1 | **Hypothesis** | `(guess smoke-loc Cigarette House)` | `(smoke-loc Parliament House_2)` |
| 1 | Japanese@`H_2` | `(co-located smoke-loc Parliament nation-loc Japanese)` (symmetric of 14) | ⟹ `(nation-loc Japanese House_2)` |
| 1 | not Tea@`H_2` | (5) + `(functional nation-loc)` | Japanese@`H_2` ⟹ Ukrainian ≠ `H_2` ⟹ Tea ≠ `H_2` |
| 1 | not Coffee@`H_2` | (4) + `(functional color-loc)` | Blue@`H_2` ⟹ Green ≠ `H_2` ⟹ Coffee ≠ `H_2` |
| 1 | not Milk@`H_2` | `(functional drink-loc)` | Milk@`H_3` ⟹ `(not (drink-loc Milk House_2))` |
| 1 | not Juice@`H_2` | (13) + `(functional smoke-loc)` | Parliament@`H_2` ⟹ Lucky_Strike ≠ `H_2` ⟹ Juice ≠ `H_2` |
| 1 | **⊥** | `(total drink-loc)` | every Drink excluded at `H_2` ⟹ `(false)` |
| 1→0 | learn no-good | path-condition learning | `(not (smoke-loc Parliament House_2))` |

### Resuming `d = 0` after both branches refuted

> ***Therefore Chesterfields are smoked in `H_2`.***

| d | NL | ein rule | premises → conclusion |
|---|---|---|---|
| 0 | **Chesterfields@`H_2`** | `(domain-elimination smoke-loc)` | Kools/Old_Gold/Lucky_Strike/Parliament excluded ⟹ `(smoke-loc Chesterfields House_2)` |

> What nationality is in `H_2` (blue, Chesterfields, horse)? Not Norwegian
> (10), not Englishman (red — 2), not Spaniard (dog — 3), not Japanese
> (Parliament — 14). ***So the Ukrainian — and by (5) drinks tea.***

| d | NL | ein rule | premises → conclusion |
|---|---|---|---|
| 0 | four negatives at `H_2` | as in branch H_2.A but with Chesterfields ≠ Parliament substituted | not Norwegian / not Englishman / not Spaniard / not Japanese at `H_2` |
| 0 | **Ukrainian@`H_2`** | `(domain-elimination nation-loc)` | only Ukrainian survives ⟹ `(nation-loc Ukrainian House_2)` |
| 0 | **Tea@`H_2`** | `(co-located nation-loc Ukrainian drink-loc Tea)` | ⟹ `(drink-loc Tea House_2)` |

---

## Step 3 — fox in `H_1` or `H_3`, refute `H_3`

> Since Chesterfields are smoked in `H_2`, by (11) the fox is in `H_1` or
> `H_3`.

| d | NL | ein rule | premises → conclusion |
|---|---|---|---|
| 0 | Fox ∈ {`H_1`, `H_3`} | `(disjunctive-prune-fwd next-to smoke-loc Chesterfields pet-loc Fox)` + `(disjunctive-prune-bwd next-to smoke-loc Chesterfields pet-loc Fox)` | next-to-neighbours of `H_2` = {`H_1`,`H_3`} ⟹ `(not (pet-loc Fox House_4))`, `(not (pet-loc Fox House_5))` |
| 0 | not Fox at `H_2` | `(functional pet-loc)` | Horse@`H_2` ⟹ `(not (pet-loc Fox House_2))` |

### Branch Fox.A — Fox@`H_3`  (refuted)

> *Suppose the fox is in `H_3`.* What does the snail-owning Old Gold smoker
> (7) drink? Water and tea are already excluded (steps 1, 2). Not juice
> (Lucky Strike — 13). Not milk — milk is at `H_3`, which we assumed holds
> the fox. Only coffee remains; by (4) he lives in the green house.
>
> Then the green house has an Old-Gold-smoking snail-keeping
> coffee-drinker. Who? Not Norwegian (`H_1`); not Ukrainian (tea — 5); not
> Englishman (red — 2); not Japanese (Parliament — 14); not Spaniard (dog
> — 3). *Impossible* — so the fox is in `H_1`, not `H_3`.

| d | NL | ein rule | premises → conclusion |
|---|---|---|---|
| 1 | **Hypothesis** | `(guess pet-loc Pet House)` | `(pet-loc Fox House_3)` |
| 1 | Snail ≠ `H_3` | `(functional pet-loc)` | ⟹ `(not (pet-loc Snail House_3))` |
| 1 | Old_Gold ≠ `H_3` | `(co-located smoke-loc Old_Gold pet-loc Snail)` (back-prop) | ⟹ `(not (smoke-loc Old_Gold House_3))` |
| 1 | Old_Gold smoker drinks Coffee | `(domain-elimination drink-loc)` on Old_Gold's house | Water/Tea/Juice/Milk excluded ⟹ Old_Gold-house = Coffee-house |
| 1 | Old_Gold lives in green | `(co-located drink-loc Coffee color-loc Green)` | ⟹ Old_Gold-house = green-house |
| 1 | five nationality exclusions at green-house | combination of (2), (3), (5), (10), (14) + the established hypothesis facts | every Nationality contradicted at green-house |
| 1 | **⊥** | `(total nation-loc)` | ⟹ `(false)` |
| 1→0 | learn no-good | path-condition learning | `(not (pet-loc Fox House_3))` |
| 0 | **Fox@`H_1`** | `(domain-elimination pet-loc)` | `H_2..H_5` all excluded ⟹ `(pet-loc Fox House_1)` |

---

## Step 4 — finishing colours and nationalities

> From the above, **coffee and orange juice are in `H_4` and `H_5`** (order
> still undetermined).

| d | NL | ein rule | premises → conclusion |
|---|---|---|---|
| 0 | Coffee ∈ {`H_4`,`H_5`} | `(functional drink-loc)` chain | Water@`H_1`, Tea@`H_2`, Milk@`H_3` ⟹ `(not (drink-loc Coffee House_{1..3}))` |
| 0 | Juice ∈ {`H_4`,`H_5`} | same | symmetric exclusion |

> Where does the Old-Gold-and-snail keeper live? Not the juice house —
> that smokes Lucky Strike (13).

| d | NL | ein rule | premises → conclusion |
|---|---|---|---|
| 0 | Old_Gold ≠ juice-house | (13) + `(functional smoke-loc)` | ⟹ Old_Gold ≠ Lucky_Strike-house = juice-house |

### Branch Coffee.A — Old_Gold smoker at the coffee house  (refuted)

> *Suppose he lives in the coffee house.* Then Old Gold + snail + coffee →
> green house (4). Same impossibility as step 3.

| d | NL | ein rule | premises → conclusion |
|---|---|---|---|
| 1 | **Hypothesis** | implicit branching over `{H_4, H_5}` for Old_Gold = coffee-house | Old_Gold at the coffee house |
| 1 | replays the step-3 sub-trace (green house, no valid nationality) | as in Branch Fox.A | `(false)` |
| 1→0 | learn no-good | path-condition learning | Old_Gold ≠ coffee-house |
| 0 | **Old_Gold@`H_3`, Snail@`H_3`** | `(domain-elimination smoke-loc)` over `H_3` | Kools@`H_1`, Chesterfields@`H_2`, Lucky_Strike@juice-house ∈ {`H_4`,`H_5`}, Parliament@Japanese ∈ {`H_4`,`H_5`} ⟹ `(smoke-loc Old_Gold House_3)`, then (7) ⟹ `(pet-loc Snail House_3)` |

> Then Parliament is smoked in the green house, where coffee is drunk and
> the Japanese lives (14). The Spaniard lives in the ivory ("white" in the
> NL) house because the red one is the Englishman's. The ivory house thus
> holds the Spaniard and his dog, so it cannot be `H_3` (snails). Since
> ivory must be immediately left of green (6) they are `H_4` and `H_5`
> respectively. Therefore red is `H_3` and **the Englishman lives there.**

| d | NL | ein rule | premises → conclusion |
|---|---|---|---|
| 0 | Parliament & Japanese & Coffee all in the green house | `(co-located nation-loc Japanese smoke-loc Parliament)` + `(co-located drink-loc Coffee color-loc Green)` chain | three facts share the green-house variable |
| 0 | Spaniard in ivory | `(range-elimination nation-loc)` over color-house projection | English in red, Norwegian in yellow, Ukrainian in blue, Japanese in green ⟹ Spaniard in ivory |
| 0 | Ivory ≠ `H_3` | (3) + `(functional pet-loc)` | Dog@Spaniard, Snail@`H_3` ⟹ Spaniard ≠ `H_3` ⟹ Ivory ≠ `H_3` |
| 0 | **Ivory@`H_4`, Green@`H_5`** | `(adjacent-via-fwd right-of color-loc Ivory color-loc Green)` + remaining domain | Ivory, Green ∈ {`H_4`,`H_5`}, Green = `right-of` Ivory ⟹ Ivory@`H_4`, Green@`H_5` |
| 0 | **Red@`H_3`, Englishman@`H_3`** | `(domain-elimination color-loc)` then (2) | last colour standing at `H_3` is Red; co-located forces Englishman |

> All cells are filled except one — **the Japanese keeps the zebra.**

| d | NL | ein rule | premises → conclusion |
|---|---|---|---|
| 0 | **Zebra@`H_5`** | `(domain-elimination pet-loc)` at `H_5` | Fox@`H_1`, Horse@`H_2`, Snail@`H_3`, Dog@`H_4` (Spaniard@`H_4`) ⟹ `(pet-loc Zebra House_5)` |
| 0 | **Query satisfied** | conjunction of `(drink-loc Water House_1)` and `(pet-loc Zebra House_5)` | both goal conjuncts present |

Final solution:

```
house   1          2           3         4              5
colour  yellow     blue        red       ivory          green
nation  Norwegian  Ukrainian   English   Spaniard       Japanese
drink   water      tea         milk      orange juice   coffee
smoke   Kools      Chest'flds  Old Gold  Lucky Strike   Parliament
pet     fox        horse       snail     dog            zebra
```

---

## Remark on numbering direction

The Wikipedia text notes that numbering direction is irrelevant — flipping
left↔right yields a mirrored table but the same answer (Norwegian still
drinks water, Japanese still keeps the zebra). In `zebra2.ein` this
freedom is broken by the explicit `right-of` chain and the asymmetric (6);
the flipped variant would re-orient those facts but never the `co-located`
block — the puzzle's underlying constraint graph is reflection-invariant.

## Branch budget summary

| branch | depth | refuted by | learnt clause |
|---|---|---|---|
| `Lucky_Strike@H_2` | 1 | `(total nation-loc)` at `H_2` | `(not (smoke-loc Lucky_Strike House_2))` |
| `Parliament@H_2`   | 1 | `(total drink-loc)` at `H_2`  | `(not (smoke-loc Parliament House_2))` |
| `Fox@H_3`          | 1 | `(total nation-loc)` at green-house | `(not (pet-loc Fox House_3))` |
| `Old_Gold@coffee-house` | 1 | replays Fox@`H_3` sub-trace | `(not (smoke-loc Old_Gold @ coffee-house))` |

No branch reaches `d = 2`. The human solution is a flat sequence of unit-
refutation hypotheses at depth 1; saturation at `d = 0` between branches
shrinks the next branch's surviving domain, which is why each successive
refutation is shorter than the previous one (the *"we already excluded
water and tea on the previous step"* pattern in step 3).
