# Arithmetic, priced — where an arithmetic program's time goes

**Instrument:** [`utils/arith_cost.py`](../arith_cost.py), through [`utils/bench_env.sh`](../bench_env.sh), and
`--biggest` for the size search; the programs past the shipped examples are written by
[`utils/gen_arith_examples.py`](../gen_arith_examples.py)'s functions.
**Moved here** from `stdlib/README.md` § `std.math.decimal-arith`, which states the calculator's cost as its shape
and pins it by count (`ein-infer/tests/arith_batch.rs`); the measurement lives beside the script that takes it.
The text below is as it was taken, dates and links kept.

## What a program pays: the tables, the parse, the calculation

The user, 2026-11-11: *"add examples with long numbers and many calculations
so we can compare load time with calculation time"* (M1f
S1f.6.15).
`ein saturate FILE` prints three times of its own: `parse:`, the program's
text; `kb load:`, macros, imports and facts; and `saturate:`, the calculation.
Three generated examples make a program long in three ways,
[`long_numbers.ein`](../../examples/arith/long_numbers.ein),
[`many_calculations.ein`](../../examples/arith/many_calculations.ein) and
[`factorial.ein`](../../examples/arith/factorial.ein), and one script measures
them and a sweep of sizes past them. Everything below the next line is its
output, taken on 2026-11-13 through `utils/bench_env.sh` on one P-core of an
i9-14900HX under the `powersave` governor (`utils/bench_env.sh python3
utils/arith_cost.py`; `--quick` is the first two sections). It was first
taken on 2026-11-11 and re-taken when S1f.6.12 added the check of each
operand, which is in every number below.
*wall* is the whole process and *rss* its peak resident memory.

`ein 0.1.0`, best of 5, seed 20261111; `python3 utils/arith_cost.py`

### What a program pays before its first request

| program | text | parse ms | load ms | saturate ms | firings | facts | rss MB | wall ms |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| no import: one relation | 46 B | 0.92 | 0.12 | 0.01 | 0 | 2 | 17 | 3.94 |
| `std.math.digits` alone, the seven tables | 54 B | 0.92 | 13.9 | 0.01 | 0 | 1 678 | 18 | 18.2 |
| `std.math.decimal-arith`, the theory and the tables | 36 B | 0.90 | 21.5 | 0.31 | 1 | 2 022 | 18 | 26.6 |
| `std.math.decimal-arith` and 17 + 5 | 64 B | 0.91 | 21.4 | 0.46 | 17 | 2 041 | 18 | 26.9 |

### The three examples

`ein saturate FILE`, the program as text (`add.ein`, one sum and one by names, for scale):

| example | text | parse ms | load ms | saturate ms | firings | facts | rss MB | wall ms |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `add.ein` | 3 KB | 1.13 | 21.6 | 0.52 | 50 | 2 079 | 18 | 27.1 |
| `long_numbers.ein` | 6 KB | 2.92 | 21.2 | 5.66 | 3 254 | 5 296 | 21 | 34.6 |
| `many_calculations.ein` | 8 KB | 4.55 | 21.2 | 14.7 | 8 120 | 10 460 | 26 | 47.6 |
| `factorial.ein` | 4 KB | 1.27 | 21.1 | 5.49 | 3 087 | 5 286 | 22 | 33.0 |

The same programs saved by `ein kb save FILE OUT.einb`, under `ein saturate OUT.einb`, which opens the container where the text is parsed and loaded:

| example | .einb | open ms | saturate ms | firings | wall ms | wall ms as text |
|---|---:|---:|---:|---:|---:|---:|
| `add.ein` | 218 KB | 22.1 | 0.50 | 50 | 26.5 | 27.1 |
| `long_numbers.ein` | 266 KB | 23.9 | 5.41 | 3 254 | 34.4 | 34.6 |
| `many_calculations.ein` | 238 KB | 25.6 | 14.4 | 8 120 | 47.3 | 47.6 |
| `factorial.ein` | 218 KB | 22.1 | 5.29 | 3 087 | 32.2 | 33.0 |

The whole process, by command:

| example | saturate | solve | test | saturate .einb | solve .einb |
|---|---:|---:|---:|---:|---:|
| `add.ein` | 27.1 | 27.4 | 27.3 | 26.5 | 27.4 |
| `long_numbers.ein` | 34.6 | 83.3 | 83.0 | 34.4 | 84.0 |
| `many_calculations.ein` | 47.6 | 96.9 | 96.3 | 47.3 | 96.3 |
| `factorial.ein` | 33.0 | 52.4 | 52.2 | 32.2 | 52.0 |

### One request, by the length of its operands

A dividend has twice the digits of its divisor. A written number nests a level a digit, and the parser nests to 1 000, so 900 digits is near the longest number a program can write.

| request | text | parse ms | load ms | saturate ms | firings | facts | rss MB | wall ms |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| add, 9 digits | 154 B | 0.96 | 21.1 | 0.48 | 46 | 2 070 | 20 | 26.4 |
| add, 30 digits | 406 B | 1.12 | 21.1 | 0.59 | 130 | 2 154 | 19 | 26.8 |
| add, 100 digits | 1 KB | 1.62 | 21.2 | 0.92 | 410 | 2 434 | 20 | 27.7 |
| add, 300 digits | 4 KB | 3.18 | 21.1 | 1.93 | 1 210 | 3 234 | 22 | 30.3 |
| add, 900 digits | 11 KB | 7.49 | 21.5 | 4.90 | 3 610 | 5 634 | 23 | 38.4 |
| sub, 9 digits | 154 B | 0.97 | 21.0 | 0.51 | 56 | 2 081 | 20 | 26.3 |
| sub, 30 digits | 406 B | 1.12 | 21.1 | 0.64 | 161 | 2 186 | 20 | 26.8 |
| sub, 100 digits | 1 KB | 1.64 | 21.2 | 1.08 | 511 | 2 536 | 20 | 27.7 |
| sub, 300 digits | 4 KB | 3.18 | 21.1 | 2.37 | 1 511 | 3 536 | 20 | 30.7 |
| sub, 900 digits | 11 KB | 7.49 | 21.4 | 6.61 | 4 511 | 6 536 | 23 | 40.5 |
| cmp, 9 digits | 154 B | 0.99 | 21.1 | 0.49 | 35 | 2 059 | 18 | 26.3 |
| cmp, 30 digits | 406 B | 1.14 | 21.2 | 0.56 | 98 | 2 122 | 19 | 26.8 |
| cmp, 100 digits | 1 KB | 1.63 | 21.1 | 0.85 | 308 | 2 332 | 18 | 27.5 |
| cmp, 300 digits | 4 KB | 3.16 | 21.2 | 1.71 | 908 | 2 932 | 20 | 30.2 |
| cmp, 900 digits | 11 KB | 7.46 | 22.5 | 4.58 | 2 708 | 4 732 | 18 | 39.1 |
| mul, 9 digits | 154 B | 1.11 | 21.9 | 1.27 | 508 | 2 532 | 18 | 28.5 |
| mul, 30 digits | 406 B | 1.15 | 21.1 | 7.65 | 4 833 | 6 857 | 23 | 35.1 |
| mul, 100 digits | 1 KB | 1.73 | 21.2 | 82.5 | 51 090 | 53 114 | 67 | 123 |
| mul, 300 digits | 4 KB | 3.30 | 21.2 | 802 | 453 275 | 455 299 | 434 | 972 |
| div, 18 by 9 | 208 B | 1.05 | 21.0 | 2.28 | 1 185 | 3 209 | 19 | 28.6 |
| div, 60 by 30 | 586 B | 1.26 | 21.2 | 16.4 | 9 439 | 11 463 | 29 | 46.3 |
| div, 200 by 100 | 2 KB | 2.18 | 21.2 | 171 | 91 959 | 93 983 | 117 | 229 |
| div, 600 by 300 | 5 KB | 4.57 | 21.4 | 1 563 | 769 555 | 771 579 | 876 | 1 918 |

### Many requests on nine-digit operands, by their count

| requests | text | parse ms | load ms | saturate ms | firings | facts | rss MB | wall ms | firings a request |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 100 x add | 12 KB | 7.95 | 21.4 | 6.81 | 4 501 | 6 822 | 23 | 41.0 | 45.0 |
| 1 000 x add | 115 KB | 71.9 | 25.4 | 71.4 | 45 001 | 50 022 | 64 | 184 | 45.0 |
| 10 000 x add | 1.1 MB | 737 | 58.8 | 829 | 450 001 | 482 022 | 451 | 1 782 | 45.0 |
| 100 x sub | 12 KB | 7.97 | 21.4 | 8.25 | 5 501 | 7 922 | 24 | 42.9 | 55.0 |
| 1 000 x sub | 115 KB | 71.4 | 25.4 | 91.6 | 55 001 | 61 022 | 74 | 208 | 55.0 |
| 10 000 x sub | 1.1 MB | 736 | 59.5 | 1 163 | 550 001 | 592 022 | 771 | 2 232 | 55.0 |
| 100 x cmp | 12 KB | 8.00 | 21.5 | 5.79 | 3 401 | 5 722 | 23 | 40.0 | 34.0 |
| 1 000 x cmp | 115 KB | 71.7 | 25.4 | 63.2 | 34 001 | 39 022 | 57 | 174 | 34.0 |
| 10 000 x cmp | 1.1 MB | 736 | 59.0 | 702 | 340 001 | 372 022 | 419 | 1 629 | 34.0 |
| 100 x mul | 12 KB | 8.10 | 21.5 | 81.0 | 50 483 | 52 804 | 64 | 129 | 504.8 |
| 1 000 x mul | 115 KB | 72.6 | 25.4 | 977 | 504 301 | 509 322 | 519 | 1 255 | 504.3 |
| 4 000 x mul | 461 KB | 294 | 36.1 | 4 397 | 2 017 067 | 2 031 088 | 1987 | 5 476 | 504.3 |
| 100 x div | 12 KB | 8.07 | 21.6 | 114 | 60 653 | 62 974 | 83 | 167 | 606.5 |
| 1 000 x div | 115 KB | 71.8 | 25.2 | 1 299 | 606 598 | 611 619 | 702 | 1 711 | 606.6 |
| 4 000 x div | 461 KB | 289 | 36.3 | 5 872 | 2 425 850 | 2 439 871 | 2949 | 7 830 | 606.5 |

### The factorial chain, by its last number

| chain | text | parse ms | load ms | saturate ms | firings | facts | rss MB | wall ms |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 25!, 26 digits | 4 KB | 1.29 | 21.1 | 5.38 | 3 087 | 5 286 | 24 | 32.9 |
| 50!, 65 digits | 4 KB | 1.43 | 21.1 | 19.6 | 12 351 | 14 725 | 34 | 49.8 |
| 100!, 158 digits | 5 KB | 1.94 | 21.3 | 93.1 | 54 723 | 57 447 | 68 | 137 |
| 200!, 375 digits | 6 KB | 2.82 | 21.4 | 667 | 352 773 | 356 197 | 395 | 845 |

### `ein solve` beside `ein saturate`

The same program, the whole process. `ein solve` saturates once, as `ein saturate` does, and then reads the state out. A run past 120 s is not waited for.

| program | firings | facts | saturate wall ms | solve wall ms | solve / saturate |
|---|---:|---:|---:|---:|---:|
| `examples/arith/add.ein` | 50 | 2 079 | 27.0 | 27.5 | 1.0 |
| `examples/arith/long_numbers.ein` | 3 254 | 5 296 | 34.5 | 82.0 | 2.4 |
| `examples/arith/many_calculations.ein` | 8 120 | 10 460 | 46.0 | 94.3 | 2.1 |
| `examples/arith/factorial.ein` | 3 087 | 5 286 | 32.7 | 52.3 | 1.6 |
| add, 100 digits | 410 | 2 434 | 27.7 | 46.4 | 1.7 |
| add, 300 digits | 1 210 | 3 234 | 30.2 | 208 | 6.9 |
| add, 900 digits | 3 610 | 5 634 | 38.8 | 1 937 | 50.0 |
| mul, 30 digits | 4 833 | 6 857 | 34.9 | 144 | 4.1 |
| mul, 100 digits | 51 090 | 53 114 | 126 | 4 072 | 32.4 |
| div, 60 by 30 | 9 439 | 11 463 | 45.0 | 643 | 14.3 |
| div, 200 by 100 | 91 959 | 93 983 | 232 | 20 866 | 90.1 |
| 1 000 x add, nine digits | 45 001 | 50 022 | 187 | 413 | 2.2 |
| 10 000 x add, nine digits | 450 001 | 482 022 | 1 785 | 4 233 | 2.4 |
| 100 x div, nine digits | 60 653 | 62 974 | 165 | 1 115 | 6.8 |
| 1 000 x div, nine digits | 606 598 | 611 619 | 1 707 | 11 441 | 6.7 |
| 50! | 12 351 | 14 725 | 49.9 | 248 | 5.0 |
| 100! | 54 723 | 57 447 | 132 | 2 479 | 18.8 |

**What it says, each with its number from the tables above.**

- **Up to about a hundred requests the tables are the cost.** A program that
  imports the theory pays 21.5 ms of `kb load` before its first request, 13.9
  of it the 1 664 rows of `std.math.digits` and the rest the theory's 116 rules and
  74 relations, where a program with no import loads in 0.12 ms and the whole
  process takes 3.9 ms. One sum adds 0.15 ms of saturation to that.
  `many_calculations.ein`, a hundred requests, is the first size where the
  calculation, 14.7 ms, is of the load's order.
- **A `.einb` does not buy the load back.** `ein kb save` caches a whole
  program with its imports resolved, and `add.ein` saved so opens in 22.1 ms
  where its text parses and loads in 22.7: the container holds the program's
  2 000 facts, and opening it interns each again. It did buy it back while the theory read two
  tables: on the stdlib of `277cfc44`, addition alone, a sum's `.einb` solved
  in 6.7 ms against 18.1 as text (`ein kb save`, `ein solve`, best of 20,
  2026-11-11). A library has no cache of its own (E-15 and E-25 of the
  phase's
  `findings.md`).
- **For a sum, parsing the program's numbers costs about what adding them
  does.** 10 000 nine-digit sums are 1.1 MB of text: 737 ms to parse and
  829 ms to saturate, where the saturation was 502 ms before S1f.6.12's
  check. A written number is one nested form a digit, the parser's worst
  input so far (E-16). For a product or a quotient the calculation leads from
  the start: 4 000 nine-digit products parse in 294 ms and saturate in 4.4 s.
- **A batch costs in proportion to its count,** as the rule of the theory
  promises: 45.0, 55.0, 34.0, 504 and 607 firings a request for a sum, a
  difference, a comparison, a product and a quotient of nine-digit numbers,
  the same at 100 requests and at 10 000, and about two microseconds a
  firing. **Eighteen of each are the check of the two operands** (S1f.6.12),
  a firing a digit: 27, 37, 16, 486 and 589 without it.
- **By length:** a sum, a difference and a comparison cost a few firings a
  digit, 3 610, 4 511 and 2 708 at 900 digits, under 7 ms each, where the
  number's own text takes 7.5 ms to parse; 1 800 of each are the check. A
  product costs the two lengths multiplied, 453 275 firings and 0.80 s at
  300 digits by 300, and a quotient of 600 digits by 300 is 769 555 firings
  and 1.6 s.
- **A chain needs no text.** `factorial.ein`'s program for 200! is 6 KB and
  parses in 2.8 ms; its 352 773 firings take 0.7 s and end on a number of 375
  digits that no line of the program writes.
- **Memory is a kilobyte a stored fact.** Every step of a circuit is a fact
  that holds its whole state, and nothing is forgotten: 4 000 products are
  2.03 million facts and 2.0 GB resident, and 10 000 were 4.9 million and
  6.0 GB (2026-11-11, the first run of the sweep, which now stops at 4 000
  for a product and a quotient; E-26).
- **`ein solve` writes the state, and the state as text is what explodes.**
  `ein solve` saturates once, as `ein saturate` does, and then reads the state
  out; `ein test` does the same. Stored, a circuit's facts share their
  numbers; written, each fact repeats its own. A 900-digit sum is 38.8 ms
  under `saturate` and 1.9 s under `solve`, where it was 1.0 s before the
  check, whose every fact holds what is left of an operand and the request;
  a 100-digit product 126 ms and 4.1 s; a quotient of 200 digits by 100,
  232 ms and 20.9 s (E-24). It is why the shipped examples are the sizes they
  are: every corpus program is solved, dumped and reloaded per commit.

### The biggest calculation

The user, 2026-11-12: *"Let's have biggest solvable arithmetic calculation in
let's say 100+-50 seconds on this host. make a generator in python that takes
some size arguments and generate an example"*; told that memory ends a
calculation before time does, *"Ok, then pick the one that fits into memory,
say ~16GB; do not call it in tests and in CI, just for manual solve call"*
(M1f
S1f.6.29).
`python3 utils/gen_arith_examples.py --op …` writes such a program. It goes
under `build/`, is never checked in, and no test, gate step, nightly step or
dev-host step writes or runs one. `utils/bench_env.sh python3
utils/arith_cost.py --biggest` searched the sizes on 2026-11-13, on cpu4, a
P-core, under the `powersave` governor. Each run was one process in a cgroup
of its own, capped at 16.0 GB with no swap (`systemd-run --user --scope -p
MemoryMax=16000000000 -p MemorySwapMax=0`)
and killed past 600 s. The search doubled a size, then narrowed to 3 %, and
measured each size once. The binary was the release build at `2ef68530`, with
that commit's stdlib. Five of the sweep's smaller programs, one of each shape
and two chains, give the same firings and facts on the stdlib of
`63955507`. Another stage's agent worked on the machine while
`ein solve`'s half ran. The user stopped the sweep before it ended (*"runs way
too long than I expected - just write down generated examples and close"*), so
the table holds what was measured, and no more.

| command | shape | the program | wall | peak RSS | firings | facts | in 100 ± 50 s | what bound it |
|---|---|---|---:|---:|---:|---:|---|---|
| `ein saturate` | product | 999 digits by 999 | 14.9 s | 6.13 GB | 5 000 895 | 5 002 922 | no | the parser: an operand of 1 000 digits nests past its limit |
| `ein saturate` | quotient | 998 digits by 499 | 5.8 s | 2.94 GB | 2 099 934 | 2 101 961 | no | the parser |
| `ein saturate` | requests | 43 430 on nine-digit numbers, the five operations in turn | 60.4 s | 15.97 GB | 16 384 266 | 16 525 264 | **yes** | the cap: 44 732 killed at it |
| `ein saturate` | chain | 1 081!, a number of 2 813 digits | 47.0 s | 15.98 GB | 16 270 571 | 16 280 163 | no | the cap: 1 113 killed at it |
| `ein solve` | product | 547 digits by 547 | 500.8 s | 2.26 GB | not taken | not taken | no | the time limit: 563 killed at 600 s |
| `ein solve` | quotient | 626 digits by 313 | 586.4 s | 1.37 GB | not taken | not taken | no | the time limit: 322 killed at 600 s |
| `ein solve` | requests | 30 379, as above | 277.2 s | 15.98 GB | not taken | not taken | no | searched to here, stopped by the user: 35 075 was killed at the cap, and 32 643 was running when the user stopped the sweep |
| `ein solve` | chain | not measured | | | | | | the user stopped the sweep before it |

*Firings* and *facts* are what `ein saturate` prints. `ein solve` prints
neither, and the saturations that would have counted its rows were not run.
`ein saturate` prints no answer. On every `ein solve` row, the script held the
answers it printed to Python's: the product's answer, 1 094 digits, is past
the parser's nesting limit for a claim, so the file states none and the
printed answer is what was checked; the other two rows' claims hold.
*Which limit bound*: under `ein saturate`, the parser bounds a written
operand before memory or time does, and the 16 GB cap bounds a program that
asks for many results or derives its numbers, each in about a minute; under
`ein solve`, which writes the state as text (E-24), the 600 s limit bounds a
long product or quotient while it holds 2.3 GB or less, and the cap bounds the
batch.

The commands, the program's first and the run's second (the date of each is
2026-11-13):

```sh
python3 utils/gen_arith_examples.py --op mul --digits 999 --count 1 --seed 20261111 -o build/s1f.6.29/biggest/product-999.ein
ein saturate build/s1f.6.29/biggest/product-999.ein

python3 utils/gen_arith_examples.py --op div --digits 499 --dividend 998 --count 1 --seed 20261111 -o build/s1f.6.29/biggest/quotient-499.ein
ein saturate build/s1f.6.29/biggest/quotient-499.ein

python3 utils/gen_arith_examples.py --op mix --digits 9 --dividend 18 --count 43430 --seed 20261111 -o build/s1f.6.29/biggest/requests-43430.ein
ein saturate build/s1f.6.29/biggest/requests-43430.ein

python3 utils/gen_arith_examples.py --op factorial --n 1081 -o build/s1f.6.29/biggest/chain-1081.ein
ein saturate build/s1f.6.29/biggest/chain-1081.ein

python3 utils/gen_arith_examples.py --op mul --digits 547 --count 1 --seed 20261111 -o build/s1f.6.29/biggest/product-547.ein
ein solve build/s1f.6.29/biggest/product-547.ein

python3 utils/gen_arith_examples.py --op div --digits 313 --dividend 626 --count 1 --seed 20261111 -o build/s1f.6.29/biggest/quotient-313.ein
ein solve build/s1f.6.29/biggest/quotient-313.ein

python3 utils/gen_arith_examples.py --op mix --digits 9 --dividend 18 --count 30379 --seed 20261111 -o build/s1f.6.29/biggest/requests-30379.ein
ein solve build/s1f.6.29/biggest/requests-30379.ein
```

