# Solution trace

> Solved in 3 steps; commitment ∅ (unconditional); 1 solution(s), 1 refuted.

## Step 1 — `from-condition`

> By condition (10), the Norwegian lives in the first house.

Premises: from condition (10)

Derives `nation-loc(Norwegian, H1)`.

## Step 2 — `adjacent-via`

> The Norwegian's only neighbour is House-2, so Blue is there.

Premises: `nation-loc(Norwegian, H1)`

Derives `color-loc(Blue, H2)`.

## Step 3 — `domain-elimination`

> Only Yellow remains for House-1.

Premises: `not(color-loc(Blue, H1))`

Derives `color-loc(Yellow, H1)`.

## Refuted hypotheses

<details>
<summary>Assumed {color-loc(Green, H1)} — contradicts condition (6) — refuted (dead-post)</summary>

Assumed **{color-loc(Green, H1)}**; the branch derives ⊥.

Lifted no-good: `color-loc(Green, H1)`.

</details>
