# S1f.6.29 — The biggest calculation: a generator with size arguments

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-12, on the user's reply of that day. Not started.
**Depends on:** [S1f.6.15](s1f.6.15_load_against_calculation.md), whose generator and cost script it
extends. Independent of the stdlib layout; no stdlib module and no engine source moves.

## The ruling

[Question 49](questions.md#q49) asked the user to confirm the small sizes of the three generated arithmetic
examples. The user, 2026-11-12:

> Let's have biggest solvable arithmetic calculation in let's say 100+-50 seconds on this host. make a
> generator in python that takes some size arguments and generate an example.

[Round 2, item 49](questions_round2.md#q49) says what is done meanwhile:

> `utils/gen_arith_examples.py` takes size arguments (operation, digits, count of requests, factorial's n)
> and writes a program to a path you give. The stage finds, for `ein saturate` and for `ein solve`
> separately, the sizes that land in your window or hit memory first, and records the commands and figures
> in `stdlib/README.md`. The big file is **generated on demand and not checked in**: every file under
> `examples/` is solved, dumped and reloaded per commit.

and asks: *Say if you want it checked in and run nightly instead.* The user's reply there, as it stood in the working copy when this plan was written (2026-11-12, not yet committed), the stage re-reading it before it starts:

> Ok, then pick the one that fits into memory, say ~16GB

So **memory is the bound, about 16 GB,** where round 2 had warned that memory ends a calculation before the
window of 100 ± 50 seconds does: the biggest calculation is the largest that runs within about 16 GB of
peak memory, and its time is reported, whatever it is. The reply does not ask for the file to be checked
in, so it stays generated on demand.

## What is known before it starts

Round 2's two cautions, with their figures (`questions_round2.md` 49, from S1f.6.9's and S1f.6.15's
records):

- **Memory ends a calculation before time does.** A stored fact costs about a kilobyte and every step is
  stored: the longest product a program can write, 998 digits by 998, is 9.8 s and 5.9 GB under
  `ein saturate`; a hundred seconds of saturation would need tens of gigabytes.
- **`ein solve` is many times dearer than `ein saturate`** on the same program, because it writes the state
  as text: a 100-digit product is 125 ms against 4.0 s.

So *the biggest solvable in the window* is two answers, one a command, and for `ein saturate` it may be *the
biggest that fits in memory*, well short of the window.

## Tasks

1. **T1f.6.29.1 — the generator takes sizes.** `utils/gen_arith_examples.py` gains arguments: the operation
   (`add`, `sub`, `cmp`, `mul`, `div`, a mix, the factorial chain), the digits of each operand, the count of
   requests, the factorial's n, a seed, and an output path. Operands come from the seed, so a size names one
   program. With no size arguments it writes the three shipped examples exactly as today, and its `--check`,
   which the gate runs through `arith_search_free.rs`, holds those three and nothing else. A generated
   program states its `:expect` where the expected answers are cheap to compute in Python, so `ein test`
   can hold it; the flag that leaves the claim out is for sizes where the claim's text would dominate the
   file.
2. **T1f.6.29.2 — the search for the size,** as a mode of `utils/arith_cost.py`, run through
   `utils/bench_env.sh`: for each of `ein saturate` and `ein solve`, and for each shape (one long product,
   one long division, many short requests, the factorial chain), **the largest size whose run fits in about
   16 GB of peak memory** (the user's reply in round 2), with wall time, firings, facts and peak RSS, and
   whether the time fell inside the first reply's window of 100 ± 50 seconds. Every run is under a memory
   limit (`ulimit -v` or a cgroup) set at that bound, so that a run which would exhaust memory is ended and reported, not left to take the host
   down. The parser's nesting limit of 999 digits bounds a single operand; the sweep says where a shape hits
   it before the window.
3. **T1f.6.29.3 — the page.** [`stdlib/README.md`](../../../stdlib/README.md) § What a program pays gains a
   table: for each command and shape, the size, the exact command that generates the program and the one
   that runs it, the date, and the figures; and one sentence on which of time and memory was the limit.
4. **T1f.6.29.4 — nothing checked in.** The big programs are written under `build/` and not tracked. The
   utilities' catalogue, [`utils/README.md`](../../../utils/README.md), gets the generator's new line.
   Q-M1f.166 is marked ruled with the user's words.

## Acceptance

- One command writes an arithmetic program of a stated size; the same arguments write the same file.
- The page names, with commands and dates, the biggest calculation that fits in about 16 GB under
  `ein saturate` and under `ein solve`, with the time each took.
- The three shipped examples and their `--check` are unchanged; the corpus gains no file.

## What it must not do

- **Not check a large program in:** the user's reply to round 2's item 49 did not ask for it.
- **Not change the engine or the stdlib** to make a size fit. What the sweep shows of E-24 (the read-out's
  cost) and E-26 (a kilobyte a fact) is added to those rows of [`findings.md`](findings.md), whose owner is
  M1g's [S1g.6.2](../../m1g_logic/p1g.6_engine_requirements/s1g.6.2_nested_facts_at_scale.md).
- **Not report an estimate.** Every figure is a measurement with its command.

## The standing rules

A wrong answer of the engine met at a large size is a row in `corpus/wrong_answers.toml`, filed `open`
before its fix. Findings go to [`findings.md`](findings.md) under the next free id; questions to
[`open_questions.md`](../open_questions.md) under the next free id. The gate is `./run_tests.sh`; the nightly
only if a file under `ein.rs/` changed, which none should. `utils/ci_container.sh` is not needed.
