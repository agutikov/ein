# S1f.6.29 — The biggest calculation: a generator with size arguments

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-12, on the user's reply of that day. **Running since 2026-11-13:** [the record](#record-2026-11-13).
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

and asks: *Say if you want it checked in and run nightly instead.* The user's reply there, committed as `dc88520b`:

> Ok, then pick the one that fits into memory, say ~16GB; do not call it in tests and in CI, just for manual
> solve call

So **memory is the bound, about 16 GB,** where round 2 had warned that memory ends a calculation before the
window of 100 ± 50 seconds does: the biggest calculation is the largest that runs within about 16 GB of
peak memory, and its time is reported, whatever it is. And **no test and no CI step runs it:** not the gate, not the
nightly, not the dev-host tier. It is generated on demand and solved by hand, *"just for manual solve call"*.

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
- The three shipped examples and their `--check` are unchanged; the corpus gains no file, and no test or
  CI step generates or solves a large program.

## What it must not do

- **Not check a large program in, and not run one from any test, gate step, nightly step or dev-host
  step** (the user's reply to round 2's item 49). The generator's own `--check` holds the three shipped
  examples and never generates or runs a large one; the size search of T2 is a script run by hand.
- **Not change the engine or the stdlib** to make a size fit. What the sweep shows of E-24 (the read-out's
  cost) and E-26 (a kilobyte a fact) is added to those rows of [`findings.md`](findings.md), whose owner is
  M1g's [S1g.6.2](../../m1g_logic/p1g.6_engine_requirements/s1g.6.2_nested_facts_at_scale.md).
- **Not report an estimate.** Every figure is a measurement with its command.

## The standing rules

A wrong answer of the engine met at a large size is a row in `corpus/wrong_answers.toml`, filed `open`
before its fix. Findings go to [`findings.md`](findings.md) under the next free id; questions to
[`open_questions.md`](../open_questions.md) under the next free id. The gate is `./run_tests.sh`; the nightly
only if a file under `ein.rs/` changed, which none should. `utils/ci_container.sh` is not needed.

## Record, 2026-11-13

The engine every run below uses is the release binary built at `2ef68530` (`cargo build --release -p ein-cli`,
2026-11-13); no engine source and no stdlib module moves in this stage. Scratch, and every program the stage
writes, is under `build/s1f.6.29/` (gitignored).

### T1f.6.29.1 — the generator takes sizes, done

[`utils/gen_arith_examples.py`](../../../utils/gen_arith_examples.py) gains a second mode, `--op`, which writes one
program to the path `-o` names. With no `--op` the script is what it was: no argument writes the three shipped files,
`--check` compares them, and `--out DIR --digits … --factorial …` writes the three at other sizes elsewhere. The
shipped files' bytes are unchanged (`sha256sum -c build/s1f.6.29/shipped.sha256`, the three hashes taken before the
first edit, all OK), so `--check` holds those three and nothing else, as before.

| argument | what it states |
|---|---|
| `--op` | the shape: `add`, `sub`, `cmp`, `mul`, `div`; `mix`, the five in turn; `factorial`, the chain |
| `--digits N` | the digits of each operand, exactly; a division's divisor |
| `--dividend N` | a division's dividend in digits, by default twice the divisor's (as `long_numbers.ein`'s `e` by `d` and `arith_cost.py`'s one-request sweep) |
| `--count N` | requests in all, by default 1; `mix` takes at least five, one of each |
| `--n N` | the factorial chain's last number |
| `--seed S` | the operands' seed, by default the shipped files' 20261111 |
| `--no-expect` | leaves every `:expect` out |

**A size and a seed name one program.** The operands are drawn from `random.Random(seed)` in one order, the
header writes the arguments in one order and no path, so the same arguments write the same bytes to any path
(`--op mix --digits 9 --count 1000` written twice, and with its arguments in another order to another directory:
`cmp` silent, one sha256). The choices the task left open, each within the stage's latitude:

- **One request names its operands with `let`,** `a` and `b`, as `long_numbers.ein` does, so the request, its
  answer fact and the claim repeat names and not digits. **Many requests write their numbers,** as
  `many_calculations.ein` does, and are distinct pairs, so the count is the requests the engine sees (a count past
  the pairs that exist is refused with the number that do).
- **A claim is stated where Python computes it, and written where it nests within the parser's limit.** Python
  computes every answer, so the only claim left out without `--no-expect` is one whose answer nests past
  `ein_ir::grammar::NESTING_LIMIT`, 1 000: a sum of two 999-digit numbers, at this seed a product of 499 digits by 499 and
  longer, a factorial past 448!, which has 995 digits. The query stays, so `ein solve` prints the answer, and the header says the claim is left out
  and why. The boundary is the parser's own: a program nested 1 000 deep loads and one nested 1 001 deep is refused
  (`--op add --digits 999` and `--digits 1000` under `ein test`, 2026-11-13).
- **A program whose operands nest past the limit is refused by the generator,** with the arguments and the limit,
  so a size the parser would refuse never reaches the engine: `--digits 1000`, or a dividend of 1 000.
- **`mix` takes the five operations in turn,** the count shared out add first, and its queries read the first request
  of each operation; a single operation's batch has one query, on its first request.

The programs it writes carry a header that names the arguments, says that the file is not checked in and run by no
test, gate step, nightly step or dev-host step, with the user's words, and points to `stdlib/README.md` § The biggest
calculation.

Run (2026-11-13): `python3 utils/gen_arith_examples.py --check`, exit 0; 14 programs at small sizes, each of the five
operations one request of 30 digits and 40 of 6, `mix` 23 of 9 and 10 of 3 by 7, `factorial` to 30!, and one with
`--no-expect`, under `ein test build/s1f.6.29/t1`: 14 files, 21 expectations, 21 held; the argument errors, each
refused with its line (`--op` without `-o`, `--n` beside an operation, `--dividend` beside a sum, `mix` under five
requests, `--check` beside `--op`, `--count` without `--op`, 82 one-digit products); `cargo test --manifest-path
ein.rs/Cargo.toml -p ein-cli --test arith_search_free`, 2 passed (`the_generated_examples_are_what_the_generator_writes`
among them), 25.9 s.
