# S1f.6.29 — The biggest calculation: a generator with size arguments

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-12, on the user's reply of that day. **Running since 2026-11-13; stopped in T2 by the host's memory, the sweep half measured and not restarted unasked:** [the record](#record-2026-11-13).
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

### T1f.6.29.2 — the search for the size: the instrument done, the sweep stopped by the host, half measured

**The instrument.** `utils/arith_cost.py --biggest`, a mode of the script that runs nothing else, run by hand
through `utils/bench_env.sh` and by no test, gate step, nightly step or dev-host step. For each command,
`ein saturate` and `ein solve`, and each shape, it finds the largest size whose run ends within a memory cap and
a time limit:

| shape | the generator's arguments | the size | the parser's bound |
|---|---|---|---:|
| product | `--op mul --digits N` | N, both operands | 999 |
| quotient | `--op div --digits N --dividend 2N` | N, the divisor | 499 |
| requests | `--op mix --digits 9 --dividend 18 --count N` | N, the five operations in turn on nine-digit numbers | none |
| chain | `--op factorial --n N` | N | none |

Each program is written by the generator's `on_demand`, the function behind `--op`, under `--work`
(`build/s1f.6.29/biggest/`). **Each run is one process in a cgroup of its own:** `systemd-run --user --scope -p
MemoryMax=16000000000 -p MemorySwapMax=0 -p OOMPolicy=continue -- ein …`, so a run that would pass the cap is
killed by the kernel inside its scope (SIGKILL, reported *memory*); its `oom_score_adj` is 1000, so that were the
host to run out first the kernel would end it before anything of the user's; and a run past `--time-limit` is
killed by the script (reported *time*, or *memory* where it sat within 5 % of the cap when killed: a process held
at its cap without swap is not killed at once, its code pages are evicted and it crawls). Peak RSS is the
process's own (`wait4`'s `ru_maxrss`; `/usr/bin/time` is not installed on this host), and *available* is
`MemAvailable` before each run. The search doubles a size until a run fails or the parser's bound is reached, then
narrows between the largest size that ended and the smallest that did not, each probe aimed by the power law the
two largest sizes that ended draw for the measure that bound (RSS for the cap, wall for the limit), until the two
are within `--resolution`. `ein solve`'s runs are held to Python's answers: each query's bindings as `ein solve`
prints them, read back from `std.arith`'s spelling, against the answers `gen_arith_examples.operands` draws, so a
wrong answer is seen where the claim is past the parser's limit and left out (checked on six small programs,
2026-11-13, and a wrong expectation is caught). It writes three tables, the largest size by command and shape,
the two commands of each, and every run, and `biggest.json`.

The generator gains `operands()`, the draw `on_demand` now calls, so the script recomputes the operands it
wrote; every file it writes is the same bytes (`--op mix --digits 9 --count 1000` before and after, `cmp` silent;
`--check`, exit 0).

The choices it rests on are defaulted as [Q-M1f.227](../open_questions.md#q-m1f227--how-a-run-of-the-biggest-calculation-is-held-to-about-16-gb-and-how-long-it-may-take):
**the cap is 16 000 000 000 bytes**, 16.0 GB, 14.9 GiB: the host had 16.2 GiB available (`MemAvailable` 16 994 396
kB at 21:45:02, `build/s1f.6.29/free-before.txt`), so 16 GiB would have put a run's cap past what the host had
left; **a run past 600 s is killed**, four times the window's top; **the search narrows to 3 %**.

**The run, and why it stopped.** `utils/bench_env.sh python3 utils/arith_cost.py --biggest --mem 16000000000
--time-limit 600 --resolution 0.03 --work build/s1f.6.29/biggest`, started 2026-11-13 21:45 on cpu4, a P-core,
under the `powersave` governor (`build/s1f.6.29/biggest.log`). At 21:51, during the chain's eighth run, Claude
Code stopped the background command because the host was critically low on memory: a run at the cap leaves the
host about 1.4 GB of the 17.4 GB it had available, and the harness reaps a background command at that point. Its
note: *do not start it again on your own … start it again only when asked*. So **`ein saturate`'s three shapes are
measured, its chain to one size short of the end, and `ein solve` not at all**; the sweep is not restarted. No
`ein` process or scope was left behind (`pgrep`, `systemctl --user list-units --type=scope`, 21:51).

**What it measured, `ein saturate`**, one run each, the release binary at `2ef68530`, 2026-11-13; `ein saturate`
prints no answer, so no answer of these is checked by the run:

| shape | size | wall s | peak RSS GB | firings | facts | in 100 ± 50 s | what stopped the next size |
|---|---:|---:|---:|---:|---:|---|---|
| product | 999 digits by 999 | 14.9 | 6.13 | 5 000 895 | 5 002 922 | no | the parser's limit: an operand of 1 000 digits nests past it |
| quotient | 998 digits by 499 | 5.8 | 2.94 | 2 099 934 | 2 101 961 | no | the parser's limit |
| requests | 43 430, nine digits, the five in turn | 60.4 | 15.97 | 16 384 266 | 16 525 264 | yes | the cap: 44 732 killed at it, 15.97 GB at 53 s |
| chain | 1 075!, a number of 2 794 digits | 48.1 | 15.98 | 16 017 805 | 16 027 355 | no | not known: 1 107, the next size the search aimed at, was at 16.8 million facts when the harness stopped the sweep |

The runs, in order (wall s, peak RSS GB): product 100 (0.2, 0.08), 200 (0.5, 0.23), 400 (1.8, 0.87), 800 (8.0,
3.42), 999 (14.9, 6.13); quotient 100 (0.3, 0.14), 200 (0.9, 0.42), 400 (3.7, 1.75), 499 (5.8, 2.94); requests
2 000 (2.1, 0.90), 4 000 (4.4, 1.66), 8 000 (9.5, 3.22), 16 000 (20.5, 6.66), 32 000 (46.0, 11.62), 44 732 (killed
at the cap at 52.9 s), 43 430 (60.4, 15.97); chain 100 (0.2, 0.08), 200 (0.8, 0.41), 400 (4.0, 1.68), 800 (22.6,
8.25), 1 044 (44.5, 15.91), 1 075 (48.1, 15.98), 1 107 (stopped by the harness). The 43 430 requests are 3.39 s
of parse, 0.23 s of load and 47.9 s of saturation; 1 075! is 39.6 s of saturation behind 1.4 ms of parse.

**What it says so far.** Under `ein saturate` a written operand is bounded by the parser before memory or time:
the longest product a program can write, 999 digits by 999, is 15 s and 6.1 GB, and the longest quotient 6 s and
2.9 GB. A program that derives its numbers, or asks for many, reaches the cap first, in under a minute: about a
kilobyte a stored fact (E-26), 16.5 million facts at the cap for the requests and 16.0 million for the chain.
The batch of 43 430 requests is the one row inside the window.
