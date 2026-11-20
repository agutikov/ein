# 80 · The read-out's examples page holds flagged programs' read-outs

[← part 1](../README.md) · [← the phase](../../../README.md) · [your answers](../ANSWERS.md) ·
[the finding](../../../findings.md#v-24--the-read-outs-examples-page-holds-flagged-programs-read-outs-to-the-binary)

A finding of the phase that is **yours to call**. It was on the findings page alone; it is here so that everything
that waits for you is in one place. Nothing waits on it.

## What it is

`docs/ein/user/04-reference/09-read-out-examples.md` is written by the binary and held to it by a gate step and a
test. Ten of its read-outs are of five flagged programs. It is the one place the warning's rows are shown, and a
row only a flagged program prints has no other example; the page calls no answer right. It is all the same a
golden of what a flagged program prints, where you ruled that no suite holds a flagged program to an answer.

## The question

- **(a)** Left: the page shows the warning's rows and calls no answer right. **Default.**
- **(b)** Those blocks are shown and not held to the binary.
- **(c)** One flagged example is kept, for the warning, and the rest are replaced by monotone programs.

## On the built engine

**How it was taken.** The final binary, `build/review-bin/ein-final-7b72bd84`, the release build of `7b72bd84`, with
the checkout's `stdlib/`, on the page and the corpus at `7b72bd84`; its `--version` is `runs/version.txt`.
`tools/run.sh` re-takes every run into `runs/` (both ignored by git); its helper `tools/examples.py` reads
[the page](../../../../../../docs/ein/user/04-reference/09-read-out-examples.md) block by block (every captioned
`ein solve` or `ein oracle` block, as
[`utils/read_out_examples.py`](../../../../../../utils/read_out_examples.py) writes it), takes the program each block
runs, and asks the binary the static check of each program, `ein solve --monotone` (`runs/monotone/<program>.txt`).
One row a block is `runs/examples.tsv`; the counts are `runs/examples-summary.txt`.

The full set of runs an item takes for its programs does not apply here: the question is about a page, so what is
run is each block's program under the static check, and the page against the binary. One program the page runs is
not a corpus file: X4's variant, which the generator writes into a scratch directory and whose query block the page
quotes. Its copy is [`07_two_pairs_two_tables+pair-at.ein`](programs/07_two_pairs_two_tables+pair-at.ein), the
generator's program after a header (`runs/variant-check.txt` compares the two), and its whole read-out is
`runs/variant.solve.txt`.

**The page is still what the binary writes.** `python3 utils/read_out_examples.py --check --bin <the final binary>`
exits 0 and prints nothing (`runs/read-out-examples-check.txt`).

### The count: the finding's figure, and today's

| | the finding's figure (V-24, on `1b9f87f0`) | **the built engine** | the run |
|---|---|---|---|
| read-out blocks on the page | — (48 on the page at `1b9f87f0`, by `runs/page-at-1b9f87f0.md`) | **48**, of 21 programs (X4's variant among them, a quoted query block) | `runs/examples-summary.txt` |
| blocks that print the `WARNING` row | **ten**, of five programs | **10**, of 5 programs; each also has the `WARNING` line on stderr (10) | `runs/examples-summary.txt` |
| blocks of a program the static check flags | — | **11**, of the same 5 programs | `runs/examples-summary.txt`, `runs/monotone/` |

The page at `1b9f87f0`, the commit that put the warning on it, has 10 blocks with a `WARNING` row, of the same five
programs and in the same numbers as today. The finding's *ten* counts the blocks that print the warning. The
eleventh block of a flagged program is X14's `ein solve --monotone examples/features/07_unstratifiable.ein`, a
question about the program, not about a commitment: it prints `monotone no — 2 sites` and its rows, and no
`WARNING` row.

| flagged program (all under `examples/`) | static check today | its blocks on the page | prints the `WARNING` row |
|---|---|---|---|
| `lattice/04_obligation_trigger_hole.ein` | `no — 4 sites` | X8 (`ein solve`), X17 (`ein oracle`) | both |
| `lattice/06_friends_tables.ein` | `no — 1 site` | X9: `--in-flight`, `--count`, `--minimal`, `--maximal` | all four |
| `features/07_unstratifiable.ein` | `no — 2 sites` | X14: `ein solve`, `--monotone`; X19: `--no-search` | two of the three: not `--monotone` |
| `features/01_not_and_absent.ein` | `no — 2 sites` | X15 | yes |
| `ein-bugs/wrong-answers/oblig-derived-activator.ein` | `no — 1 site` | X20 | yes |

Each verdict is the `monotone` row of `runs/monotone/<program>.txt`; the other 16 programs are `yes — no site`
there, and no block of theirs prints a `WARNING` row (`runs/examples.tsv`).

### What only a flagged program's block shows

For options (b) and (c), the rows the page prints only in a flagged program's blocks, by the label in the read-out's
first column (`runs/examples-summary.txt`, *row labels printed only in a flagged program's blocks: 14*):

| row | blocks |
|---|---|
| `WARNING` | the ten above |
| under `lattice`: `assumes`, `site` | the eleven |
| under `lattice`: `status`, `state` | the ten |
| under `lattice`: `stale` | X14's read-out, X15 |
| under `lattice`: `cycle`, `order` | X14 (both blocks), X19 |
| under `lattice`: `bit` | X14's `--monotone` |
| under `lattice`: `minimal hole` | X8, X17 |
| under `lattice`: `in flight` | X9's `--in-flight` |
| `strata`, and under it `negative cycle` | X14's read-out, X19 |
| `later debt` | X20 |

`in flight` and `later debt` are not the warning's rows: one is the check in flight (`--in-flight`), the other a debt
root does not owe and a commitment can bring. Each has its one example on a flagged program, so option (c), one
flagged example kept and the rest replaced by monotone programs, would need a monotone program that prints each, or
would leave the row with no example.
