# S1f.6.16 — Arithmetic with binary numbers

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-11, on the user's instruction of that day. **T1 done 2026-11-11, as a prototype and a
page shown to the user: [`binary_arithmetic.md`](binary_arithmetic.md)**, the three ways each tried and measured,
with a recommendation of (b). Nothing shipped; T2 to T4 wait for the user's ruling on that page's questions.
**Way (b) banked as examples 2026-11-12** (`9612b350`), on the user's instruction of that day:
[`examples/binary_arithmetic/`](../../../examples/binary_arithmetic/README.md); the [record](#record-2026-11-12-way-b-banked-as-examples)
is below. It is not T2: no stdlib module, no test of its own, and the questions stay open.
**The module is ruled and planned (2026-11-12):** the user's replies to the page's questions are in
[§ The build, as ruled](#the-build-as-ruled-2026-11-12), which replaces tasks 2 to 4 below. Not started; it runs
after the stdlib layout.
**Depends on:** [S1f.6.6](s1f.6.6_division.md), the four operations in decimal, which it is compared with.

## The instruction

The user, 2026-11-11: *"new more stages: arithmetic with binary numbers, I suspect simpler rules"*.

(On 2026-11-11 the user also said *"skip different bases"*, of hexadecimal and base 64 as a way to make decimal
arithmetic faster. This stage is not that: binary is asked for as the simplest arithmetic, and as the foot of
[S1f.6.17](s1f.6.17_binary_logic.md)'s logic.)

## The question

Whether the rules get simpler, and what exactly does. The coordinator's expectation, to be measured and not
assumed:

- **The tables shrink, a lot.** A full adder is 8 rows where decimal's is 200; a digit product with a carry is
  4 where decimal's is 900; *not zero* is one row; the comparison table is 12.
- **The wiring may not change at all.** The circuits of [`std.arith`](../../../stdlib/math/decimal-arith.ein) walk a
  number a column at a time and look each column up; nothing in a walk says *ten*. If so, binary arithmetic is
  the same rules over other tables, and the stage's finding is that the theory is **generic in its base**,
  which says where decimal lives: in the tables and nowhere else.
- **Division does get simpler.** A quotient digit is 0 or 1, so one comparison decides it, where decimal
  bisects ten multiples: the bisection tables and their rules are not needed.
- **Numbers get longer**, about 3.3 times, so the firings go up as the tables go down.

## Tasks

1. **T1, on a prototype under `build/s1f.6.16/`, and shown to the user before anything ships:** the same
   targets as decimal's, in binary, each with its answer computed; the rule and row counts side by side; the
   firings and the times side by side; and the three ways to have binary numbers, each tried:
   - **(a)** the same shape and the same relations, `(n PREFIX DIGIT)` over the digits `0` and `1`, with
     binary tables in a module of their own: then a program picks its base by which tables it imports, and a
     number does not say which base it is in;
   - **(b)** a shape of its own, e.g. `(b2 PREFIX BIT)`, and rules of its own, so the two can be imported
     together and never confused;
   - **(c)** one generic theory whose every request carries its base.
   The stage recommends one. The user's rule for new syntax is real examples first.
2. **The module, its tables and its programs,** as ruled: generated tables with `--check`, a program per
   rule, the search-free test, the rule of the theory held (no rule joins two facts on a number), every
   answer held to Rust's integers.
3. **Conversion between decimal and binary,** as requests, each a chain of the theory's own operations
   (repeated division by two one way, multiply and add the other). It is the first use of the calculator
   by a theory, before the word problems.
4. **The comparison page:** what got simpler, what did not, and the cost.

## Acceptance

- The prototype's page, with the three options and a recommendation, and the user's ruling.
- Binary numbers add, subtract, multiply, divide and compare, search-free and right against the integers.
- The page says, with numbers, which rules binary made simpler and which it left as they were.

## Record, 2026-11-12: way (b) banked as examples

The user, 2026-11-12: *"bank binary arithmetic and logic prototypes into examples too"*. Done with
[S1f.6.17](s1f.6.17_binary_logic.md#record-2026-11-12-the-recommended-variants-banked-as-examples)'s in one
commit.

**What is banked,** under [`examples/binary_arithmetic/`](../../../examples/binary_arithmetic/README.md):

| file | what | answer on the release binary of 2026-11-12 |
|---|---|---|
| `bits.ein`, `bin.ein` | way (b): the three tables, 28 rows, and the theory, 67 rules and its closing rule, generated from `stdlib/arith.ein` by `gen_binary.py` | modules; alone, R {∅} |
| `conv.ein` | the conversions, eleven rules, by hand | a module; alone, R {∅} |
| `first.ein` | 101 + 111; 110 − 11 by name | 1100; 11; 2 claims held |
| `five_operations.ein` | the page's targets | 127645; 118511 each way; 56088; 5105 remainder 20; `gt`; 5 claims held; 555 firings |
| `div_by_zero.ein` | 127645 ÷ 0 | root dead, R empty, `refuted by division-by-zero-is-false2`; `(solve :expect (false))` held |
| `both_bases.ein` | `std.arith` and `bin`, flat | 127645 and 1100; 2 claims held |
| `conversions.ein` | 0, 1, 2, 10, 255 to binary; those, 256 and 1000 to decimal | 12 conversions right; 2 claims held; 1 749 firings |

(`ein test examples/binary_arithmetic`, 12 claims held, `ein solve` and `ein saturate` on each, 2026-11-12.) Every program:
*no candidate*, k_s 1 (0 for the division by zero), one saturation, *monotone (the static check)*, exit 0.

**Checked against Python,** on the banked theory and the current stdlib (`python3
build/s1f.6.16-17/chk_arith.py`, 2026-11-12): the prototype's sample of 243 requests, 243 right; 30 numbers
each way through `conv.ein`, 2^40 + 1 among them, 60 of 60 right and in normal form; the firings of the
page's six targets, 43, 61, 120, 245, 24 and 10, and of the two conversions, 826 and 76 for 255 and 2 317 and
202 for 123078, as the page has them. **No answer differs from the page.** The claims in the banked programs
were written by a script from Python's integers (`build/s1f.6.16-17/mk_arith.py`).

**Variants.** Way (b) only, as the page recommends; (a) and (c) stay on the page. The theory is generated
from `std.arith`, as the page recommends, by `gen_binary.py`, which is the prototype's `build.py` reduced to
(b) with its reader inlined; `bin.ein` differs from the prototype's in its header and one word of its
closing rule's `:why`. The conversions are a module of their own, as on the prototype.

**Defaults taken,** each the user's to revise ([`open_questions.md`](../open_questions.md)): the generator
is run by hand and is not in the gate, since whether the binary rules are generated and held by a test is
the page's fourth question; `bits.ein` is copied into `examples/binary_logic/` by the same generator, since
an import cannot reach a sibling directory; the sizes are the corpus's, short operands and conversions to
1000.

**Not banked,** and why: ways (a) and (c), set aside by the page; the batches of 250 requests and the
conversion of 123078 and 2^40 + 1, which are measurements and whose states as text are large (E-24); the
witness of E-22, which exits 1.

**The corpus and the goldens,** named here before blessing (`EIN_BLESS=1`), for both directories, since
they were banked in one commit: 28 entries in `corpus/corpus.toml`, eight for this directory and twenty for
`examples/binary_logic/`, a module declaring `solve` and `saturate` (and `render rules` where it has a
rule), a search-free program `solve`, `test` and `saturate`, a search `solve`, `solve --single`, `solve -n
1` and `test`; `ein-cli/tests/golden/corpus_exits.txt`, +86 lines, the new files' cells;
`ein-render/tests/golden/corpus_shapes.md5`, +1 176 lines, 42 renderings for each of the 28 files, and no
other line of either; and `ein-infer/tests/alive_invariant.rs`'s list, which gains six programs: `bin.ein`,
`both_bases.ein`, `conv.ein`, `first.ein` and `five_operations.ein` for `z`, and `binary_logic/post.ein` for
the five class names its rules conclude (`git diff --numstat`, 2026-11-12). No program is `slow`.

**Committed and checked after** (2026-11-12): `9612b350`, on master at `7deec58a`, the gate green before it
(`build/gate-s1f.6.16-17.log`). On `9612b350`, the tree clean but for another agent's `questions.md`:
`./run_tests.sh --nightly` green in every step (`build/nightly-s1f.6.16-17.log`), the slow corpus cells,
`answer_dump`, the round trip's slow and `heavy_dump` entries, the levers', the cores', `region`'s and
`obligation_rung`'s slow halves, the scratch check, the mutation census at 361 of 411, the fuzzer at both
seeds and the bench smoke. `utils/ci_container.sh` was not run: the banking changed no DOT writer, nor the
review bundle, nor what CI installs (Q-M1f.77). Not run: the dev-host tier. **S1f.6.20's word problems were
not on master when this was committed**: its gate, run on a worktree, had not ended, `region.rs`'s
`fact_grain_never_calls_a_violated_cube_monotone` not finishing with its programs in the corpus (19 s
without them, no end in 75 s with `bob1.ein` or `boxes12.ein` alone added, on a copy of that worktree's
files, 2026-11-12). So the goldens here hold no line of its, and its commit re-blesses on top of this one.

**Not done:** no stdlib module, no `--check` in a test, no per-rule test programs, no page of comparison
beyond the plan page: those are T2 to T4, which wait for the ruling. **The page's seven questions stay
open**; banking the recommended way is the instruction for the examples and not a ruling on them.

## The build, as ruled (2026-11-12)

**Status:** planned 2026-11-12, not started. It replaces tasks 2 to 4 of [§ Tasks](#tasks).
**Depends on:** [S1f.6.31](s1f.6.31_the_stdlib_layout.md), the layout, which names the modules and moves
`std.arith` to `std.math.decimal-arith`; and [S1f.6.24](s1f.6.24_division_by_zero_at_the_call.md), so the
binary division is written once with the flag at the call.

### The rulings

The user's replies of 2026-11-12, each to the question of [`questions.md`](questions.md) named:

| question | the reply, as typed | what it makes of the build |
|---|---|---|
| [13](questions.md#q13), which of the three ways | *"(b)"* | a theory of its own, `(b2 (b2 z 1) 0)`, beside decimal in one program; a number says its base |
| [14](questions.md#q14), the binary theory's names | *"a is ambiguous, add2 looks like add with 2 operands etc. Take (b), but ints and rats - different question. binary and decimal is numbers representation, integers, rationals, ... complex - are numbers types based on naturals of some representation. Add new stage into p1f.8 to investigate how to make compound numbers without hardcoded representation so can switch between binary and decimal, and maybe later use any new natural numbers there, or even not numbers..."* | a word prefix: `bin-add` answered by `bin-sum-of`, and so on. The second half is [P1g.10's S1g.10.7](../../m1g_logic/p1g.10_school_algebra/s1g.10.7_compound_numbers_without_a_hardcoded_representation.md) |
| [15](questions.md#q15), is zero the same `z` | *"(a)"* | one `z`; a bare `z` is of no base |
| [16](questions.md#q16), how the equal rules are kept equal | *"yes, extract common rules into separate file the both import, btw rename arith to decimal-arith"* | neither option asked: the shared rules written once, in a file both theories import. [Round 2, item 16](questions_round2.md#q16) plans a trial first. The rename is the layout's. The user's reply in round 2 is a question back, quoted under the table |
| [17](questions.md#q17), where the conversions live | *"(b)"* | a module of their own, so a program that only computes in binary does not load decimal's 1 664 rows |
| [18](questions.md#q18), decimal's division by bisection or the simpler one | *"I expected you implement https://en.wikipedia.org/wiki/Long_division ; What is shipped now? I think there are invented many different fast division algorithms, implemented in CPUs, long numbers meth libraries, etc. Let's leave this aside for now - having one implementation now is enough"* | the shipped division stays; [round 2, item 18](questions_round2.md#q18) answers what is shipped (long division, the digit found by bisection) |

**Round 2, item 16, the user's reply** (`dc88520b`):

> So we potentially can import std.math.rat and std.math.binary-arith and then connect them with single fact?

Not today, and the trial below is what would make the answer *yes*. `std.rat`'s and `std.int`'s rules name
the decimal theory's calls in every rule, so a rational over binary naturals would be a second copy of both
modules. If the trial shows that rules with parameters cost nothing for the two bases, the same mechanism
applied to the compound types is exactly *one fact*: the activator that says *rationals over this
representation*. That step is not this stage's; it is the first direction of
[P1g.10's S1g.10.7](../../m1g_logic/p1g.10_school_algebra/s1g.10.7_compound_numbers_without_a_hardcoded_representation.md),
and the trial's record says, for that stage, whether a parameter could stand in every place a walk needs
it. The trial itself goes ahead as round 2 planned it, the reply raising no objection to it.

The shape `(b2 PREFIX BIT)` is the prototype's and the banked examples'. Reply 14 objects to the `2` suffix on
the *requests*; whether the shape's head should change with them (`bin`, say) is not said. **Default: `b2`
stays,** as banked, and the question is filed under the next free id.

### The modules

By [the layout](s1f.6.31_the_stdlib_layout.md#the-layout): `std.math.bits`, the binary tables;
`std.math.positional`, the rules both bases share; `std.math.binary-arith`, entry `binary-arith`;
`std.math.convert`, entry `convert`. `std.math.decimal-arith` becomes an importer of `positional` if the trial
succeeds.

What the prototype measured (`questions.md` 13, runs of 2026-11-11 under `build/s1f.6.16/`): way (b) is 67
rules and 28 table rows where decimal's are 88 and 1 664; 123 × 456 is 120 firings where decimal's is 72;
parse, load and compile 5.8 ms against 26.8. Under (b), 51 of the 67 rules are `std.arith`'s up to their names
(`questions.md` 16). The conversions are eleven rules; 255 to binary is 826 firings and back is 76
(`questions.md` 17).

### Tasks

1. **T1f.6.16.5 — the trial of parameterised shared rules, first, and it may stop the stage.** Round 2, item
   16:

   > Ein has the mechanism: a rule with parameters is compiled once per activator fact, so `(rule add-column
   > (?num ?table) …)` activated for `(n fa)` and for `(b2 fa2)` gives two specialised plans and should cost
   > nothing at run time. […] What is not known until it is tried: whether a parameter may stand as the head
   > of a nested number pattern in every place the walks need it, and whether the firings stay at today's
   > count.

   On a copy of the stdlib under `build/`, with the release binary: the 51 shared rules written once with
   parameters for what differs between the bases (the constructor, the tables, the request and result
   names), activated once for decimal and once for binary. Measured against the shipped `std.arith` and the
   banked `bin.ein`: firings and depth of the page's targets, candidates a request in a batch (by hand, with
   the counters), load and compile time, and whether each surface (a trace's rule names, a core, `refuted
   by`) reads as it does today.
   - **If the parameterised rules cost what the shipped ones do, they are the shared file.**
   - **If they cannot be written, or cost more, the stage stops and shows the user** the measurement beside
     the generated copy (question 16's old option (a)), which is what `examples/binary_arithmetic/` has
     today.
   What it has to watch: S1f.5.18's refusal of a relation parameter an activator could bind to a fact; the
   closing rules, which name every rule; the text test of the rule of the theory, which reads rule bodies;
   and that decimal's states do not change, or change only in rule names, shown before any golden moves.
2. **T1f.6.16.6 — the tables,** `std.math.bits`, generated with a `--check` in the gate through a cargo
   test, as `std.math.digits` is; every row held to Rust's integers.
3. **T1f.6.16.7 — `std.math.binary-arith`:** `bin-add`/`bin-sum-of`, `bin-sub`/`bin-diff-of`,
   `bin-cmp`/`bin-cmp-of`, `bin-mul`/`bin-product-of`, `bin-div`/`bin-quot-of`, on `(b2 PREFIX BIT)` and the
   shared `z`; division by zero the calculator's flag and `(false)`; the detection of malformed numbers as
   far as the shared rules bring it, and the page saying what stays silent.
4. **T1f.6.16.8 — `std.math.convert`:** `(to-bin X)` and `(to-dec B)` as requests, over both theories'
   tagged calls (the interface of [S1f.6.30](s1f.6.30_name_clashes_between_imported_theories.md)'s T1).
5. **T1f.6.16.9 — the tests,** as every module of the calculator has: a harness against Rust's integers with
   a row an operation; the rule of the theory on the text; search-free on the binary; a program per rule
   under `tests/stdlib/`; the mutation census; the batch by hand. A program that imports both bases flat and
   computes in each is among them.
6. **T1f.6.16.10 — the examples import the stdlib.** `examples/binary_arithmetic/`'s programs import
   `std.math.*`; its `bits.ein`, `bin.ein` and `conv.ein` and the generator `gen_binary.py` are retired if
   the trial succeeded, or the generator moves to `utils/` with its `--check` in the gate if the copy is
   generated. `examples/binary_logic/bits.ein`, the copy E-32 forced, goes either way. No claim of any
   example changes; the request names in them do.
7. **T1f.6.16.11 — the comparison page,** task 4 of the old list: [`binary_arithmetic.md`](binary_arithmetic.md)
   gets a closing section, what shipped and what each ruling made of the page's questions;
   [`stdlib/README.md`](../../../stdlib/README.md) a section a module; `AGENTS.md`.

### Acceptance

- Binary numbers add, subtract, multiply, divide and compare, search-free and right against the integers,
  from the stdlib, under `bin-` names.
- The rules the two bases share are written once, or the user chose the generated copy after seeing the
  trial's measurement.
- Decimal's answers and costs are what they were: no existing program's answer moves, and the firings of
  the page's decimal targets are unchanged.
- The conversions are a module of their own, and a program that imports only `std.math.binary-arith` holds
  no decimal table row.

### What it must not do

- **Not replace decimal's division** (reply 18).
- **Not build the generic theory of 13 (c),** whose every request carries its base.
- **Not make compound numbers generic in their representation:** that is S1g.10.7's investigation.
- **Not start before the layout has moved.**

### The standing rules

A wrong answer of the engine met on the way is a row in `corpus/wrong_answers.toml`, filed `open` before its
fix. Findings go to [`findings.md`](findings.md) under the next free id; questions to
[`open_questions.md`](../open_questions.md) under the next free id. A design change shows real computed
examples first (the trial). Config never changes an answer. The gate is `./run_tests.sh`, then `./run_tests.sh
--nightly` after each code commit; `utils/ci_container.sh` is not needed.
