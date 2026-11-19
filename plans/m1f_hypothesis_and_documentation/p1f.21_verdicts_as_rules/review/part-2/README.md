# Part 2 — `open`: what a state owes, and the hypotheses

[← the bundle](../README.md) · [← the phase](../../README.md) · [your answers](ANSWERS.md)

The open questions of the phase's second part ([the order](../../README.md#the-order-two-parts)). They wait with
their part: nothing of part 1 needs them. An answer goes into [`ANSWERS.md`](ANSWERS.md) here.

| # | item | the question | state |
|---|---|---|---|
| 32 | [Q-M1f.390](32-Q-M1f.390/README.md) | what an obligation emits: the verdict, with its justification, or as a fact | not ruled; default (b); `(open F J)` beside (d) |
| 34 | [Q-M1f.549](34-Q-M1f.549/README.md) | hypotheses as facts: held, in which reading, and how far the search goes into rules | not ruled; recommended (b), held as *asked* |
| 39 | [Q-M1f.560](39-Q-M1f.560/README.md) | `std.debts`, built by S1f.21.8: merged with its stage, or with the second part | (a) held on its branch for part 2, as now |

Item 39 stands between the parts: `std.debts` was built in the first and is of the second; held is its default,
so with no answer it is merged with this part.

**The binaries.** Item 39 is run with `build/review-bin/ein-s1f218-9e40adbd`, the build of S1f.21.8's branch,
which has `std.debts`; items 32 and 34 as their pages say. `tools/run.sh` in an item re-takes its runs into
`runs/`; both are ignored, so they are on this machine only.

## Found while building

- **32**: every count of the ledger re-taken and held; the user's form holds 4 `(open F J)` facts at
  {(seats Ann C1)}, 8 at seats' root, 112 at zebra2-obligations' root. Option (c) runs today as ordinary rules (the
  unpaid requirement equals the kernel's debt at 16 of 16 nodes, same R); a debt derived as a fact is stale in 15 of
  16 nodes. The two KB mocks are drawn again in the canonical form: each fact of `open` a red, bold octagon whose
  arrow `1` is at the box `open`. `design.md` § 4.4 and § 4.5 assume a state file that lists facts; a saved file today lists none, so under
  (d) or the user's form a saved file need not change. `--dump-answer` writes no file for an owing node below a
  non-empty R.
- **34**: *zebra2* and the seats program answer the same with their candidates held as ordinary facts and one generic
  hrule over them, the read-out byte for byte; root holds 125 such facts on *zebra2* and $H$ is 56 of them. A rule
  that stores *asked and still undecided* gets the program flagged and a stale fact in 15 of 16 nodes. The lattice of
  a two-candidate program written as rules gives the oracle's $R$ in one saturation, and its rule over a second
  absence answers wrong, three strata printed and not followed (V-8).
- **39**: with `std.debts` imported and asked for, the seats program's $R$ is its own, 2 states, each holding
  `(debts-discharged)` and four facts of `debt-paid`.
