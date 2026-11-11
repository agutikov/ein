# S1f.6.9 — What is left of the width: how long a number may be

**Phase:** [P1f.6](README.md)
**Status:** **measured and stated, 2026-11-12, and stopped for the user's choice:** the nesting limit re-taken on
the shipped theory, the cost by length measured, and both on the page; nothing is built past the statement. **The
choice between (a), (b) and (c) is the user's,**
[Q-M1f.185](../open_questions.md#q-m1f185--a-state-file-nested-past-the-parsers-limit), with each option's probe
below ([§ T1 record](#t1-record-2026-11-12-the-limit-re-taken) onward). E-4 is re-taken and waits for that
ruling. Planned 2026-11-08; **restated 2026-11-11**: the representation question it was to put to the user is
ruled ([the rulings](design.md#the-rulings-2026-11-11)), and most of the stage dissolved with it.
**Depends on:** [S1f.6.6](s1f.6.6_division.md). It may run beside S1f.6.7 and S1f.6.8.

## What dissolved

The user, 2026-11-08: *"with overflow detection or width extension if it is possible, I'm not sure now but maybe
relation could have variadic arguments then can write numbers in reverse order e.g. (num 8 7 0 3 2 1) ==
123078"*.

- **The representation** is ruled: nested, `(n PREFIX DIGIT)`, any length. No variadic relation is needed, so
  no grammar change is asked for.
- **Width extension** has nothing to extend: a number is as long as its digits.
- **Overflow** exists in `std.num9` only, and its detection and its test are
  [S1f.6.13](s1f.6.13_num9.md)'s.

## What is left

1. **The nesting limit.** The parser nests to 1 000, so a request can hold a number of 999 digits, while a rule
   can derive a longer one, and a `--dump-answer` state file holding such a fact does not load back (E-4 of
   [`findings.md`](findings.md); 999 nines + 1 on the prototype). The stage re-takes it on the shipped theory
   and puts the choice to the user:
   - **(a)** say it on the page and leave it: a number past 999 digits is answered and cannot be reloaded;
   - **(b)** the theory flags an answer longer than a stated bound, as `std.num9` flags overflow;
   - **(c)** an engine change, so that what a rule can derive the parser can read.
   The stage's recommendation is (a) until a program needs more.
2. **The size of a step fact.** Each step of a circuit holds its operands whole, so the text of a state grows
   with the square of the length: the 1 000-digit sum's state file was 32 MB. The stage measures facts, bytes
   and time by length for the four operations (10, 100 and 999 digits) and states where a run stops being
   practical.
3. **Words.** Four-digit words were measured in round 3 and lost their gain to the conversions on a single
   operation. They are not built. The stage states the one case that would reopen them, a multiplication or a
   division that stays in words throughout, with its measurement if S1f.6.10 asks for long numbers.

## Tasks

1. The limit re-taken, the three options shown with their probes, and the user's ruling applied.
2. The cost by length, as a table with its command.
3. The pages: the limit and the cost in the module's header and in `stdlib/README.md`.

## Acceptance

- The limit and the cost by length are stated with the commands that re-take them.
- E-4 is ruled or owned.

## T1 record, 2026-11-12: the limit re-taken

On the shipped `std.arith`, `std.int` and `std.rat`, the release binary of `f1fe6918`'s engine. The scripts are
under `build/s1f.6.9/` (gitignored) and each prints the table quoted from it.

**1. Where the parser stops** (`python3 build/s1f.6.9/depth.py`). A fact holding a number of 999 digits, 1 000
parentheses deep, loads. One of 1 000 digits is refused, exit 1, with one line: *`parse.ein:3:3001:` this `(` nests
deeper than the limit of 1000 parentheses*. The limit is `ein_ir::grammar::NESTING_LIMIT`, F10's: the LR parser
survives any depth, and the limit is there because macro expansion, the loader and the dumper recurse over the
tree it builds, checked on a debug build in a test thread's stack.

**2. E-4 is nearer than it was stated.** The prototype's witness was 999 nines plus 1, a rule deriving a number
longer than any program can write. On the shipped theory a state file fails to reload **from operands a program
can write, with no answer longer than they are**: a step of a circuit holds its operands inside its tag, so its
facts are deeper than the request. The deepest fact of a state, in parentheses, by the digits of the operands:

| operation | 10 digits | 20 | 40 | so: digits + | the deepest fact's relation |
|---|---:|---:|---:|---:|---|
| add | 12 | 22 | 42 | [2] | `add-walk` |
| sub | 12 | 22 | 42 | [2] | `done` |
| cmp | 12 | 22 | 42 | [2] | `cmp-walk` |
| mul by one digit | 13 | 23 | 43 | [3] | `done` |
| div by one digit | 13 | 23 | 43 | [3] | `done` |
| int-add | 14 | 24 | 44 | [4] | `add-walk` |
| int-mul by one digit | 15 | 25 | 45 | [5] | `done` |
| rat-add, denominators of one digit | 17 | 27 | 47 | [7] | `done` |
| rat-cmp, denominators of one digit | 17 | 27 | 47 | [7] | `done` |

The offset does not depend on the length, so the longest operand whose state reloads is 1 000 less it: 998 digits
for a sum, a difference and a comparison, 997 for a product and a quotient, 996 for a sum of integers, 995 for
their product, 993 for a rational's numerator. **The boundary was run** for five of them, `ein solve
--dump-answer` and then the state file loaded again (`python3 build/s1f.6.9/depth.py --boundary add cmp`, and
`--boundary sub "mul by one digit" int-add`; `01.ein` is the one state's file):

| operation | digits | solve | deepest fact written | the state file reloads |
|---|---:|---|---:|---|
| add | 997 | exit 0, state file 85.5 MB | 999 | yes |
| add | 998 | exit 0, state file 85.7 MB | 1000 | yes |
| add | 999 | exit 0, state file 85.8 MB | 1001 | no: `01.ein:8464:3010:` this `(` nests deeper than the limit of 1000 parentheses |
| cmp | 997 | exit 0, state file 40.5 MB | 999 | yes |
| cmp | 998 | exit 0, state file 40.6 MB | 1000 | yes |
| cmp | 999 | exit 0, state file 40.7 MB | 1001 | no: `01.ein:8864:3012:` this `(` nests deeper than the limit of 1000 parentheses |
| sub | 997 | exit 0, state file 127.6 MB | 999 | yes |
| sub | 998 | exit 0, state file 127.9 MB | 1000 | yes |
| sub | 999 | exit 0, state file 128.1 MB | 1001 | no: `01.ein:8863:3013:` this `(` nests deeper than the limit of 1000 parentheses |
| mul by one digit | 996 | exit 0, state file 77.6 MB | 999 | yes |
| mul by one digit | 997 | exit 0, state file 77.7 MB | 1000 | yes |
| mul by one digit | 998 | exit 0, state file 77.9 MB | 1001 | no: `01.ein:6862:3019:` this `(` nests deeper than the limit of 1000 parentheses |
| mul by one digit | 999 | exit 0, state file 78.1 MB | 1002 | no: `01.ein:6864:3019:` this `(` nests deeper than the limit of 1000 parentheses |
| int-add | 995 | exit 0, state file 324.9 MB | 999 | yes |
| int-add | 996 | exit 0, state file 325.5 MB | 1000 | yes |
| int-add | 997 | exit 0, state file 326.2 MB | 1001 | no: `01.ein:8912:3024:` this `(` nests deeper than the limit of 1000 parentheses |
| int-add | 998 | exit 0, state file 326.8 MB | 1002 | no: `01.ein:8916:3024:` this `(` nests deeper than the limit of 1000 parentheses |
| int-add | 999 | exit 1: `b.ein:2:3009:` this `(` nests deeper than the limit of 1000 parentheses | | |

So a sum of two 999-digit numbers is answered, its state file is written, 85.8 MB, and that file is refused. An
integer request with parts of 999 digits does not load at all: the request nests one deeper than `std.arith`'s.

**3. The engine has no limit of its own** (`python3 build/s1f.6.9/deep.py`, a program with no arithmetic: a
counter a rule deepens by one for each link of a written tank of fuel, 990 links, and one, two and five tanks):

| the deepest fact | command | exit | stdout | stderr's last line |
|---:|---|---:|---:|---|
| 991 | saturate | 0 | 0.0 MB |  |
| 991 | saturate --dump | 0 | 4.0 MB |  |
| 991 | solve | 0 | 0.0 MB |  |
| 991 | solve --dump-answer | 0 | 0.0 MB |  |
| 991 | the state file, loaded again | 0 | 0.0 MB |  |
| 1981 | saturate | 0 | 0.0 MB |  |
| 1981 | saturate --dump | 0 | 15.8 MB |  |
| 1981 | solve | 0 | 0.0 MB |  |
| 1981 | solve --dump-answer | 0 | 0.0 MB |  |
| 1981 | the state file, loaded again | 1 | 0.0 MB | deep2.answer/models/01.ein:2044:3004: this `(` nests deeper than the limit of 1000 parentheses |
| 4951 | saturate | 0 | 0.0 MB |  |
| 4951 | saturate --dump | 0 | 98.3 MB |  |
| 4951 | solve | 0 | 0.0 MB |  |
| 4951 | solve --dump-answer | 0 | 0.0 MB |  |
| 4951 | the state file, loaded again | 1 | 0.0 MB | deep5.answer/models/01.ein:5020:3004: this `(` nests deeper than the limit of 1000 parentheses |

Every command answers on a state 4 951 deep, and only the state file's reload is refused. Deeper, under `ein
saturate` alone (`python3 build/s1f.6.9/deeper.py`):

| the deepest fact | exit | firings | saturate ms | wall s | stderr's last line |
|---:|---:|---:|---:|---:|---|
| 19800 | 0 | 19821 | 17.95 ms | 0.0 |  |
| 99000 | 0 | 99101 | 95.51 ms | 0.1 |  |
| 396000 | 0 | 396401 | 445.13 ms | 0.6 |  |

A fact 396 000 levels deep is derived in half a second: values are interned and nothing in the saturator
recurses over one. `ein solve` on the 19 800-deep program did not finish in 170 s: it writes every fact of the
state as text, 19 800 facts of up to 19 800 levels, which is E-24 and not a limit of depth.

**The three options, each with its probe.** The question is
[Q-M1f.185](../open_questions.md#q-m1f185--a-state-file-nested-past-the-parsers-limit).

- **(a) Say it on the page and leave it.** Done by this stage, since it is true whatever is ruled:
  [`stdlib/README.md` § How long a number may be](../../../stdlib/README.md#how-long-a-number-may-be-m1f-s1f69)
  and the header of `stdlib/arith.ein` state the two limits, the table by operation, and that every answer is
  right and printed past them. It costs nothing at run time. What it leaves: a state file that is written and
  cannot be read, with nothing said when it is written; `answer_round_trip.rs` holds every corpus program's
  state to reload, and no corpus program is near the limit.
- **(b) The theory flags a number longer than a stated bound,** as `std.num9` flags overflow. **Probed**
  (`python3 build/s1f.6.9/bound_proto.py`; nothing of the stdlib touched, the rules are a program's own beside
  `std.arith`): the bound is a written fact, a ruler of 990 links; a walk steps down a number and the ruler
  together, a firing a digit, and where the ruler runs out first the request is flagged, `(too-long REQUEST)`.
  Five rules for addition: two operands and the sum measured.

  | request | exit | firings with the bound | without | flagged | answered |
  |---|---:|---:|---:|---|---|
  | 9 digits + 9 | 0 | 50 | 28 | no | yes |
  | 100 digits + 100 | 0 | 414 | 210 | no | yes |
  | 989 digits + 989 | 0 | 3970 | 1988 | no | yes |
  | 990 digits + 1 | 0 | 3975 | 1990 | yes | yes |
  | 990 digits + 990 | 0 | 3974 | 1990 | yes | yes |
  | 991 digits + 1 | 0 | 3978 | 1992 | yes | yes |
  | 998 digits + 998 | 0 | 3991 | 2006 | yes | yes |

  It works, and **it doubles a sum**: a firing a digit of each of three numbers, 50 firings where 28 for nine
  digits and 3 970 where 1 988 at 989, on every request, well-formed or not, whatever its length. The bound has to
  be the smallest any operation's state allows, 993 with the rationals, or one a module. It is a 990-level
  literal in a module's text. And it needs a decision the probe does not make: whether a flagged request is
  still answered (here it is, as nothing stops the walk), and whether the flag is a contradiction, as a division
  by zero is, or a fact beside the answer, as `std.num9`'s overflow is. A number of 990 digits is not wrong; only
  its state file is unreadable.
- **(c) An engine change, so that what a rule can derive the parser can read.** Not probed by building: the
  stage may not change the engine. What the code says: the limit is one constant, and what it protects is three
  recursive walks, macro expansion, the loader and the dumper, on a debug build in a test thread's stack. Two
  routes: the walks made iterative, or run on a stack sized for the input, after which the constant could go or
  grow; or the state file alone read by a path with no such walk, since it holds facts and no macro. Either removes
  the limit for every program, not only the calculator's, and makes the third column of the page's table the
  only one. The cost table below says what it would buy: a state past 1 000 digits is already tens of megabytes
  for a sum and gigabytes for a product, so the file that would reload is one nobody waits for.

**The recommendation is (a), until a program needs more,** as the stage file had it. The measurements add a
reason: the limit on a state file is reached only by states whose text is 40 MB at the least, which `ein solve`
takes seconds to write; what is practical ends well before 998 digits for everything but a sum. (b) taxes every
request to protect a file nobody loads; (c) is right in principle, belongs with E-24, whose fix would make such
a state small enough to be worth reloading, and is an engine stage of its own.

## T2 record, 2026-11-12: the size of a step fact

One request of `std.arith`, its operands drawn with a fixed seed, on one P-core through `utils/bench_env.sh`
(`utils/bench_env.sh python3 build/s1f.6.9/cost.py`; i9-14900HX, `powersave`, the bench line in
`build/s1f.6.9/cost.out`). *The state's own text* is the `--dump-answer` state file less the 173 094 bytes a
program that imports the theory and asks nothing writes for the tables, and *a derived fact* one of the facts
the request added. A solve past 300 s was not waited for, and one whose text would pass 2 GB was not started.

| request | firings | saturate ms | `ein saturate`, the process, ms | peak RSS MB | `ein solve --dump-answer`, the process | the state's own text | text a derived fact |
|---|---:|---:|---:|---:|---:|---:|---:|
| add, 10 digits | 30 | 0.3 | 26 | 19 | 48 ms | 18 KB | 0.6 KB |
| add, 100 digits | 210 | 0.5 | 26 | 19 | 97 ms | 927 KB | 4.4 KB |
| add, 998 digits | 2 006 | 2.9 | 36 | 22 | 6.7 s | 85.5 MB | 42.6 KB |
| add, 999 digits | 2 008 | 3.0 | 37 | 21 | 6.7 s | 85.6 MB | 42.6 KB |
| sub, 10 digits | 41 | 0.3 | 26 | 19 | 48 ms | 26 KB | 0.6 KB |
| sub, 100 digits | 311 | 0.7 | 26 | 19 | 118 ms | 1.4 MB | 4.6 KB |
| sub, 998 digits | 3 005 | 4.3 | 38 | 23 | 9.8 s | 127.7 MB | 42.5 KB |
| sub, 999 digits | 3 008 | 4.3 | 37 | 24 | 9.8 s | 127.9 MB | 42.5 KB |
| cmp, 10 digits | 18 | 0.3 | 26 | 19 | 48 ms | 10 KB | 0.6 KB |
| cmp, 100 digits | 108 | 0.5 | 26 | 19 | 71 ms | 457 KB | 4.2 KB |
| cmp, 998 digits | 1 006 | 2.2 | 35 | 19 | 3.5 s | 40.4 MB | 40.2 KB |
| cmp, 999 digits | 1 007 | 2.2 | 35 | 19 | 3.4 s | 40.5 MB | 40.2 KB |
| mul, 10 digits by 10 | 595 | 1.1 | 26 | 19 | 71 ms | 528 KB | 0.9 KB |
| mul, 100 digits by 100 | 50 698 | 72.7 | 111 | 75 | 18.4 s | 353.4 MB | 7.0 KB |
| mul, 300 digits by 300 | 452 676 | 662.0 | 801 | 470 | not finished in 300 s | | |
| mul, 998 digits by 1 | 2 013 | 3.2 | 37 | 21 | 5.9 s | 77.7 MB | 38.6 KB |
| mul, 998 digits by 10 | 38 139 | 56.3 | 94 | 71 | 152.2 s | 1.72 GB | 45.0 KB |
| mul, 998 digits by 100 | 408 300 | 610.2 | 753 | 430 | not run: the text is past 2 GB | | |
| div, 10 digits by 10 | 679 | 1.4 | 27 | 19 | 103 ms | 1.1 MB | 1.7 KB |
| div, 100 digits by 100 | 33 347 | 57.8 | 93 | 73 | 29.3 s | 544.8 MB | 16.3 KB |
| div, 600 digits by 300 | 766 926 | 1287.6 | 1573 | 871 | not run: the text is past 2 GB | | |
| div, 998 digits by 1 | 21 173 | 32.9 | 67 | 46 | 39.1 s | 491.0 MB | 23.2 KB |
| div, 998 digits by 10 | 74 613 | 123.0 | 177 | 126 | 165.1 s | 1.90 GB | 25.4 KB |
| div, 998 digits by 100 | 525 905 | 887.9 | 1089 | 759 | not run: the text is past 2 GB | | |

The longest product and quotient a program can ask, under `ein saturate` alone (`utils/bench_env.sh python3
build/s1f.6.9/longest.py`):

| request | firings | facts | saturate | the process | peak RSS |
|---|---:|---:|---:|---:|---:|
| mul, 998 digits by 998 | 4 988 896 | 4 990 871 | 9788.63 ms | 13.6 s | 5.9 GB |
| div, 998 digits by 499 | 2 071 748 | 2 073 723 | 3868.68 ms | 4.6 s | 2.5 GB |

**What it says.**

- **The stage file's number was the prototype's.** It gave 32 MB for the 1 000-digit sum's state; the shipped
  theory's 998-digit sum writes 85.7 MB, since its steps carry a tag that holds both operands.
- **A derived fact of a long computation is its numbers, written out:** about 40 KB of text at 998 digits for a
  sum, a difference, a comparison and a product by one digit (42.6, 42.5, 40.2 and 38.6 KB), some forty bytes a
  digit of the operands it carries, and 23 KB for a quotient by one digit. Stored, those numbers are shared and
  a fact is about a kilobyte (E-26).
- **Facts grow as the lengths multiplied, text as that times the length again.** A sum's state is 2 006 facts at
  998 digits; a product of two 100-digit numbers 50 698, of 998 by 998 five million.
- **Where a run stops being practical:**
  - a sum, a difference, a comparison: nowhere a program can reach under `ein saturate`, 3 to 4 ms at 999
    digits; under `ein solve`, seconds from some hundreds of digits, 6.7 s and 86 MB at 998;
  - a product or a quotient under `ein saturate`: by memory. 998 digits by 998 is 4 988 896 firings, 9.8 s and
    5.9 GB; 998 by 499, 2 071 748 firings and 2.5 GB. Both fit a workstation and neither a small runner;
  - a product or a quotient under `ein solve`: at about a hundred digits. 100 by 100 is 18 s and 354 MB of state
    for a product, 29 s and 545 MB for a quotient; 300 by 300 did not finish in 300 s; and one long operand is
    enough, 998 digits by 10 being 152 s and 1.7 GB.
- **The limit of T1 is past all of that.** A state that fails to reload is at least 40 MB of text.

## T3 record, 2026-11-12: words, and the pages

**Words are not built.** Four-digit words were measured in the design's round 3 and lost their gain to the
conversions on a single operation. The one case that would reopen them is a multiplication or a division that
stays in words throughout, a chain whose intermediate numbers are never converted back; the table above says what
it would be measured against, 4 988 896 firings for 998 digits by 998. S1f.6.10 asks for it if it asks for long
numbers; nothing here measures it.

**The pages:** [`stdlib/README.md` § How long a number may be](../../../stdlib/README.md#how-long-a-number-may-be-m1f-s1f69),
new, with the table by operation and the cost table; the header of `stdlib/arith.ein`, a paragraph, *How long a
number may be* (a comment: the module's rules, relations and facts are byte for byte what they were, and
`MANIFEST.sha256` moves with the file's digest); `std.rat`'s section, which now links to it; [`findings.md`](findings.md),
E-4 re-taken; `AGENTS.md`; Q-M1f.185; the phase README and this record.

**No rule moved, so no sweep was run:** `git diff f8f605a3 -- stdlib/arith.ein` is comment lines alone, and the
gate's goldens, which digest every rendering of the module, did not move.

**The gate** is green (`./run_tests.sh`, `build/gate-s1f.6.9.log`, 2026-11-12): the module's digest moved and
no golden did.

## Acceptance re-taken, 2026-11-12

- **The limit and the cost by length are stated with the commands that re-take them:** T1 and T2, and the page.
- **E-4 is ruled or owned:** neither yet. It is re-taken, stated on the page, and put to the user as Q-M1f.185
  with the three options probed; its row in [`findings.md`](findings.md) says so.
