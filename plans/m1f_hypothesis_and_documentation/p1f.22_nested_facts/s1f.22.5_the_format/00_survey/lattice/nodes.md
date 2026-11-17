# Visited nodes — `examples/domain_elim/b_branch.ein`

`ein solve examples/domain_elim/b_branch.ein`: 8 nodes by the state route, every alive state once, in visit order, over 7 candidates. `nodes.json` holds the same, and `--events` each node's firings.

## Layer 0 — 1 visited: 0 dead, 0 owing, 1 in R

| # | commitment | kind | closure | facts | why |
|---:|---|---|---|---:|---|
| 1 | ∅ | in R | ∅ | 0 |  |

## Layer 1 — 7 visited: 6 dead, 0 owing, 1 in R

| # | commitment | kind | closure | facts | why |
|---:|---|---|---|---:|---|
| 2 | {(color-loc Blue H1)} | in R | {(color-loc Blue H1)} | 1 |  |
| 3 | {(color-loc Blue H2)} | dead | {(color-loc Blue H2)} | 2 | core (color-loc Blue H2), (color-loc Red H2) |
| 4 | {(color-loc Blue H3)} | dead | {(color-loc Blue H3)} | 2 | core (color-loc Blue H3), (color-loc Green H3) |
| 5 | {(color-loc Green H1)} | dead | {(color-loc Green H1)} | 2 | core (color-loc Green H1), (color-loc Green H3) |
| 6 | {(color-loc Green H2)} | dead | {(color-loc Green H2)} | 2 | core (color-loc Green H2), (color-loc Green H3) |
| 7 | {(color-loc Red H1)} | dead | {(color-loc Red H1)} | 2 | core (color-loc Red H1), (color-loc Red H2) |
| 8 | {(color-loc Red H3)} | dead | {(color-loc Red H3)} | 2 | core (color-loc Green H3), (color-loc Red H3) |

## Layer 2 — 0 visited: 0 dead, 0 owing, 0 in R

| # | commitment | kind | closure | facts | why |
|---:|---|---|---|---:|---|
