# S1f.6.16 — Arithmetic with binary numbers

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-11, on the user's instruction of that day. **T1 done 2026-11-11, as a prototype and a
page shown to the user: [`binary_arithmetic.md`](binary_arithmetic.md)**, the three ways each tried and measured,
with a recommendation of (b). Nothing shipped; T2 to T4 wait for the user's ruling on that page's questions.
**Way (b) banked as examples 2026-11-12** (`9612b350`), on the user's instruction of that day:
[`examples/binary_arithmetic/`](../../../examples/binary_arithmetic/README.md); the [record](#record-2026-11-12-way-b-banked-as-examples)
is below. It is not T2: no stdlib module, no test of its own, and the questions stay open.
**The module is ruled and planned (2026-11-12):** the user's replies to the page's questions are in
[§ The build, as ruled](#the-build-as-ruled-2026-11-12), which replaces tasks 2 to 4 below. It runs after the
stdlib layout. **Its trial, T1f.6.16.5, done 2026-11-14, and the build stopped for the user:** the shared rules
can be written with parameters and do the shipped rules' work to the count, but take 3 to 5 % more saturation
and 1 to 4 ms more load, and decimal's states change in more than rule names
([§ Record, 2026-11-14](#record-2026-11-14-t1f6165-the-trial-and-the-stop)). Nothing built; the question is
[there](#the-question-for-the-user). **The user ruled (a) the same day, the design first: shown in
[§ The design](#the-design-after-the-users-ruling-of-2026-11-14)** (one call of a macro connects a base; the base's
own module is the file that combines; the build waits for the user's word).
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

**Status:** planned 2026-11-12. It replaces tasks 2 to 4 of [§ Tasks](#tasks). **T1f.6.16.5 done 2026-11-14;
stopped for the user** at the trial's own rule, the parameterised rules costing more
([§ Record](#record-2026-11-14-t1f6165-the-trial-and-the-stop)); T1f.6.16.6 to .11 not started.
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
--nightly` after each code commit; `utils/ci_container.sh` is not needed. *(That last sentence is superseded by
the user's cadence of 2026-11-13: one gate per batch of code commits, the nightly only after a large engine
change, a records-only commit the link check alone.)*

### Record, 2026-11-14: T1f.6.16.5, the trial, and the stop

**Stopped for the user, as the plan says where the parameterised rules cost more.** They can be written, and
they do the shipped rules' work to the count, by every count the engine keeps; they take more time, 3 to 5 %
more saturation and 1 to 4 ms more load a program; and decimal's states change in more than rule names. So
T1f.6.16.6 to .11 are not started: no module, no test, no example, no golden moved. The choice is
[§ The question for the user](#the-question-for-the-user).

**What was run.** A copy of the stdlib under `build/s1f.6.16/trial/stdlib/` (gitignored), written by
`python3 build/s1f.6.16/trial/mk_trial.py` from the checkout at `24a7fb34`:

- `math/positional.ein`: the 78 rules both bases share, each written once from `std.math.decimal-arith`'s
  text, every relation it names a parameter, in capitals as `std.algebra` writes `?R1`: `(rule add-column
  (?ADD-WALK ?N ?FA ?B) :match (and (?ADD-WALK ?k (?N ?p ?a) (?N ?q ?c) ?e ?acc) (?FA ?a ?c ?e ?s ?f))
  :assert (?ADD-WALK ?k ?p ?q ?f (?B ?s ?acc)) …)`. 251 parameters, two to five a rule; every `:why` as
  shipped;
- `math/decimal-arith.ein`: the shipped module less those 78 rules, importing positional, listing them, and
  activating each once with decimal's relations, `(add-column add-walk n fa b)`;
- `math/bits.ein`: the binary tables `bin-fa`, `bin-fs`, `bin-cl` and `bin-nz`, the last one row, `1`;
- `math/binary-arith.ein`: binary's 60 relations, `b2` and every other name `bin-`; the 78 rules activated
  with them; and 16 rules of its own, the banked theory's multiplication by a bit and division by one
  subtraction a bit, the divisor in normal form, the two checks that list the digits (`check-digit` and
  `operand-undeclared` over `0` and `1`), and `division-by-zero-is-false` (point 3 below).

The release binary at `24a7fb34` (sha256 `f9f9774c5c9ee47a…`) on an i9-14900HX; every time is taken under
`utils/bench_env.sh --core 2` by `build/s1f.6.16/trial/lt_time.py`, the variants interleaved round by round,
and the median of 11 to 61 rounds is quoted.

**Which rules are shared:** 78 of decimal's 115, the closing rule aside. The operands and the named constants;
the addition, subtraction and comparison walks; the normal form and both turns; the protocol of the five
operations, each its ask, its second operand, its start and its answer; multiplication's call (`mul-turn`,
`mul-begin`, `mul-next`, `mul-end`); division's `div-begin` and `div-end`; the two division-by-zero flags; and
S1f.6.12's check but for the two rules that list the ten digits. Decimal keeps 37: the digit multiplier's walk
(6), the division of base ten (28), `check-digit`, `operand-undeclared` and `division-by-zero-is-false`.

**What it took to write them.**

1. **A parameter heads a nested pattern wherever a walk needs one**, in a premise, `(?ADD-WALK ?k (?N ?p ?a)
   …)`, and in a conclusion, `(… (?B ?s ?acc))`; under the operand look-up's join on a name, beside
   `let-twice`'s guard, and inside a flag's request, `(?MALFORMED (?LET z ?v))`. No rule changed but in its
   heads. That is the point the plan left open, and the answer for
   [S1g.10.7](../../m1g_logic/p1g.10_school_algebra/s1g.10.7_compound_numbers_without_a_hardcoded_representation.md)
   (round 2's question back): a parameter can stand everywhere the walks need it. *One fact* it is not: a
   rule is activated by a fact of its own name, so a base is 78 activators.
2. **The closing rule may not name a shared rule.** `(arith-rule add-end)` writes the rule's name as an
   argument, and S1f.5.18's check then reads each parameterised conclusion of `add-end`'s arity,
   `(?OPERAND-IS ?k z)` among them, as a possible activator of `add-end` that binds a parameter to a fact (a
   tag): the program is refused at load, a line a pair. It is [E-55](findings.md)'s refusal, met by
   `std.algebra` at S1f.6.35. So `decimal-arith` names its own 37 rules and not positional's 78.
3. **`division-by-zero-is-false` stays decimal's**, for the same refusal: `std.math.num9`'s closing rule names
   it, `(num9-rule division-by-zero-is-false)`. Binary has its own.
4. **A parameter spelled as a relation's short name interns that name bare.** With `?sum-of`, the batch test's
   lookup of the result by its bare spelling (`arith_batch.rs`'s `saturate`; `ein_corpus::spellings` reads
   the same way) found the variable's empty relation, and counted 0 answers to 250 requests that were
   answered. Capitals keep every parameter apart from every name.

**Decimal's answers: unchanged on every program.** The 143 tracked programs that reach the calculator
(`grep -rlE 'std\.math\.|\(import (bin|conv) ' --include='*.ein' examples tests`), each run on the shipped
stdlib and on the trial's (`python3 build/s1f.6.16/trial/compare.py --solve`, 2026-11-14):

- **`ein solve`'s read-out is the same on all 143**, read with a rule's module taken off its name
  (`std.math.positional.` or `std.math.decimal-arith.`) and the times taken out; every exit code the same.
- **Root's state** (`ein saturate --dump`, a set of facts, the deriving rule taken off each) **differs on 124
  programs in two ways and nothing else**: the 78 `(arith-rule R)` facts that named a shared rule are gone
  (point 2), and 78 activators are new, `(std.math.positional.add-column std.math.decimal-arith.add-walk n
  std.math.digits.fa std.math.decimal-arith.b)` and the rest. The other 19 do not import decimal's module
  (the digit tables' tests, the banked binary examples, `examples/beyond_arith/`'s eight) or are refused at
  load on both (the three `arity-*` witnesses), and do not differ.
- **A trace reads as today but for the module in a step's rule**, `### Step 9 — std.math.positional.add-column`
  where `std.math.decimal-arith.add-column`, every `:why` and fact the same, and the closing step deriving 78
  facts fewer. **`refuted by`** names `std.math.positional.malformed-is-false` for a malformed request where
  `std.math.decimal-arith.malformed-is-false`, and `std.math.decimal-arith.division-by-zero-is-false` for a
  division by zero, as today; the unsat core is the request in both.

So decimal's states change in more than rule names: 78 facts out and 78 in, on every program that imports it;
under the generated copy they do not change at all.

**The work: equal, by every count.** The page's targets (`python3 build/s1f.6.16/trial/measure_trial.py`),
firings / depth, the closing rule's left out, every answer right:

| request | decimal, shipped | decimal, parameterised | binary, banked | binary, parameterised |
|---|---:|---:|---:|---:|
| 123078 + 4567 | 31 / 21 | 31 / 21 | 43 / 43 | 73 / 43 |
| 123078 − 4567 | 38 / 21 | 38 / 21 | 61 / 43 | 91 / 43 |
| 4567 − 123078 | 38 / 21 | 38 / 21 | 61 / 43 | 91 / 43 |
| 123 × 456 | 78 / 72 | 78 / 72 | 120 / 120 | 136 / 120 |
| 127645 ÷ 25 | 228 / 220 | 228 / 220 | 245 / 245 | 267 / 245 |
| 123078 against 4567 | 23 / 13 | 23 / 13 | 24 / 24 | 54 / 24 |
| 127645 ÷ 0 | 18 / 10 | 18 / 10 | 12 / 10 | 29 / 19 |

Binary's extra firings are its operand check, a firing a bit of each operand, which the banked theory never
had; its depths are the banked theory's but where the check's chain is the longest, the division by zero. 234
sampled binary requests, the five operations, edges and random operands to nine decimal digits: each answered
once and right against Python's integers.

**A batch** (`EIN_STDLIB=build/s1f.6.16/trial/stdlib cargo test --release -p ein-infer --features counters
--test arith_batch -- --nocapture a_batch_costs_in_proportion a_number_operand`, and the same without
`EIN_STDLIB`): **candidates a request identical to the last digit** at 250, 1 000 and 4 000 requests, 181.38,
362.00, 901.00, 9 603.87 and 6 033.98 at 250 for the five operations; firings a request identical, 45.00,
55.00, 34.00, 504.84 and 458.85; **guard values a request 312 fewer**, 687.92 where 999.92 (660.86 where
972.86 for division): `operand-undeclared` ranges over the program's names, and the 78 rule names the closing
rule no longer writes are 78 names fewer, two comparisons each for each of two operands.

**The time: more.** Parse, load and compile, then the saturation, in ms, medians:

| program | shipped | parameterised | |
|---|---:|---:|---|
| one addition (`lt/t.ein`), 61 rounds | 23.49 · 0.48 | 25.68 · 0.48 | load +2.2 ms, +9 % |
| 250 additions, 11 rounds | 43.34 · 15.60 | 45.64 · 15.96 | |
| 1 000 additions, 11 rounds | 104.40 · 62.92 | 106.92 · 64.91 | |
| 4 000 additions, 21 rounds | 346.97 · 267.59 | 350.67 · 275.78 | load +3.9 ms; saturation +3.1 % |
| 250 products, 21 rounds | 43.50 · 188.11 | 45.76 · 195.94 | saturation +4.2 % |
| 250 quotients, 21 rounds | 38.52 · 187.49 | 40.68 · 193.78 | saturation +3.4 % |

Where it goes, as far as measured (each a stdlib under `build/s1f.6.16/trial/lt/`, timed beside the others):

- **not the order**: the shipped module with its rules in the trial's order and no parameter (`lt/reorder`)
  loads and saturates as shipped, 268.83 ms where 267.89 on 4 000 sums, 188.30 where 188.40 on 250 products;
- **not the extra file**: an empty module imported beside the shipped one (`lt/emptymod`), 23.55 ms where
  23.49;
- **+1.15 ms of load is any parameterised rule at all**: one rule with a parameter and no activator added to
  the shipped module (`lt/oneparam`) costs as much as positional's 78 without their activators (`lt/noact`),
  24.61 and 24.64 where 23.46. S1f.5.18's check of derived activators returns at once on a program with no
  parameterised rule, and otherwise reads every written fact's arguments, the 1 664 table rows among them
  ([E-60](findings.md));
- **the other 1 ms of load, with the activators,** and **the 3 to 5 % of saturation** are not attributed. No
  count differs, so the saturation's share is time a firing or a plan; a parameterised firing's identity
  carries its activator's arguments (`defined_behaviour.md` § 3.2) and a written rule's carries none, which is
  a candidate and is not measured ([E-61](findings.md)).

**The generated copy, the same rules** (`python3 build/s1f.6.16/trial/mk_gencopy.py`: positional's 78 rules
written out with binary's relations, 95 rules, no parameter): the same firings, 18, 127 126 and 141 208; load
and compile 7.05 ms where the parameterised theory takes 8.20 on one sum; saturation 172.61 where 178.97 on
1 000 sums (+3.7 %) and 195.50 where 205.44 on 250 products (+5.1 %). Under the copy, decimal is the shipped
module and costs what it costs today.

**Both bases in one program** (`build/s1f.6.16/trial/p/both.ein`): decimal and binary imported flat, each
shared rule activated twice; 123078 + 4567 and 101 + 111, 2 claims held, no candidate, 54 firings.

**Defaults:** none taken; the names the trial used (`b2` kept, every other binary name `bin-`, the tables
`bin-fa`, `bin-fs`, `bin-cl` and `bin-nz`) are the trial's, not the build's, and the build files its question
when it runs. **Findings:** [E-60 and E-61](findings.md), neither a wrong answer; point 2 is E-55's. **The
gate:** not run, a records-only commit (the user's cadence of 2026-11-13); the link check.

### The question for the user

Reply 16 asked for the common rules in a file both bases import, and round 2 planned this trial and this stop.
Measured, the two ways to have the 78 shared rules:

| | (a) one file, parameterised | (b) a generated copy, question 16's old (a) |
|---|---|---|
| what is written | `std.math.positional`, 78 rules once, each activated by a fact a base | `std.math.decimal-arith` as shipped; `std.math.binary-arith` written from it by a script, its `--check` in the gate |
| decimal's answers | unchanged | unchanged |
| decimal's states | 78 `(arith-rule R)` facts out, 78 activators in; 78 rules named `std.math.positional.*` in a trace, a dump and `refuted by` | unchanged |
| work: firings, depth, candidates | the shipped ones | the shipped ones |
| guard values a request | 312 fewer | as shipped |
| load | +1 to 4 ms a program, +9 % on one request | as shipped |
| saturation | +3 to 5 % | as shipped |
| for S1g.10.7 | the mechanism it would build on, tried | nothing |

- **(a)** the shared file, at that cost.
- **(b)** the generated copy: nothing shared at run time, the binary module a generated file.
- **(c)** an engine stage first, that makes a parameterised rule cost what a written one does (E-60's check
  run only where it can refuse, E-61 attributed and removed), and then (a).

**Recommended: (a).** The difference is time and not work: no count the engine keeps moved, the time is a few
per cent, and a third of the load's share is a check any generic rule pays (E-60). (b) is the copy reply 16
answered against; (c) is (a) with an engine stage in front, whose gain is not known until E-61 is attributed.

**Meanwhile:** nothing of the build runs. S1f.6.17's T1f.6.17.7, the full adder against `std.math.bits`' table,
waits with it.

### The design, after the user's ruling of 2026-11-14

The user, 2026-11-14, on [§ The question for the user](#the-question-for-the-user):

> (a), show me the design first, if they can't be connected with single hand written fact, if it require ~100
> facts for that - then you need a separate file that combines theories, that import two of them, does the
> combination job and can then be included by end user

**Status:** designed on a prototype under `build/s1f.6.16/design/` (2026-11-14) and shown; nothing under
`stdlib/`, `tests/`, `examples/` or `ein.rs/` moved. T1f.6.16.6 to .11 wait for the user's word on the design.
Every number below is the release binary at `24a7fb34` (sha256 `f9f9774c5c9ee47a…`).

#### 1. One hand-written form connects a base: yes

**What the engine has.** A rule with parameters fires for a fact headed by its own name, one plan a fact
([`01_grammar.md` § Rules](../../../docs/kernel/ir/03-ein-lang/01_grammar.md)), so 78 rules take 78 activators,
and nothing lets one fact activate several rules. Not available: one activator relation for all 78 (two rules of
one name are refused, *conflicting definitions*); a macro's call written as a fact (refused at load since
S1f.7.12, [`defined_behaviour.md` § 4.24](../../../docs/kernel/defined_behaviour.md#424-a-macros-call-is-no-fact--m1f-s1f712));
the base's names held in a fact of a declared relation that a rule reads (S1f.5.18 refuses it: an argument of a
declared relation may hold a fact). **What does work is to derive the 78 activators from one form at root**:
activation reads a derived activator as a written one, and the activated rules compile when it appears, as
`std.algebra`'s lemmas' do. Two ways, each tried on the trial's stdlib (`python3
build/s1f.6.16/trial/mk_variants.py`):

- **(ii) a fan-out rule, one fact.** `std.math.positional` holds `(rule positional (?OPERAND ?OPERAND-IS ?N
  ?CHECK … 63 parameters) :match () :assert (and (operand-zero ?OPERAND ?OPERAND-IS) (operand-number ?OPERAND
  ?N ?OPERAND-IS ?CHECK) … 78 activators))`, and a base writes **one fact**, `(positional operand operand-is n
  check let named add-walk fa b strip turn nz done add … undeclared)`, its 63 relations in the parameters' order.
- **(iii) a macro, one call.** `std.math.positional` holds `(macro positional (… the same 63 …) (and … the same
  78 …))`, and a base writes one rule whose whole conclusion is **one call**: `(rule decimal-arith-positional ()
  :match () :assert (positional operand operand-is n check … undeclared))`.

Measured beside the trial's 78 written activators, (i) (parse, load and compile of one sum, 41 interleaved
rounds; saturation, 15; `utils/bench_env.sh --core 2 python3 build/s1f.6.16/trial/lt_time.py`; the machine was
busier than for the trial, the shipped 4 000 sums taking 288 ms where 268, and the order of the four was the same
in a second run of 21 rounds):

| | firings | depth | load + compile, one sum | saturation, 4 000 sums | saturation, 250 products |
|---|---:|---:|---:|---:|---:|
| shipped | as shipped | as shipped | 23.49 ms | 288.16 ms | 199.58 ms |
| (i) 78 activators a base | the same | the same | 25.66 ms | 297.80 ms, +3.3 % | 207.16 ms, +3.8 % |
| (ii) one fact, a fan-out rule | +1 | the same | 25.95 ms | 314.82 ms, +9.3 % | 215.34 ms, +7.9 % |
| (iii) one call of a macro | +1 | the same | 25.62 ms | 299.30 ms, +3.9 % | 207.48 ms, +4.0 % |

The firings and depths are the page's seven targets' (one more each under (ii) and (iii): 32 / 21 for 123078 +
4567 where 31 / 21). Why (ii) costs twice what (iii) does at the same firings is not attributed (E-61's
neighbour).

**What each reads,** under (ii) and (iii) alike (`compare.py --solve` with `TRIAL_DIR` on each, 2026-11-14):
every read-out of the 143 programs is the shipped one but for rule names; root's state on the 124 that import
decimal trades the 78 `(arith-rule R)` facts for the 78 activators, derived now, as under (i), and under (ii)
holds the one fact too. A trace has one step more, the first, `std.math.positional.positional` (ii) or
`decimal-arith-positional` (iii), deriving the 78 activators, and is the trial's after it. The unsat core and
`refuted by` are the trial's: the request, `std.math.positional.malformed-is-false`,
`std.math.decimal-arith.division-by-zero-is-false`. S1f.5.18's check loads both. The closing rule's list names the
base's own rules and the connecting rule and not positional's 78, as under (i) (E-55).

**(iii)'s call is a rule's whole conclusion, and not one of the closing rule's,** because of
[E-63](findings.md): a conclusion that is an `and` inside an `and`, which the grammar writes `Conclusion ::= '('
'and' Conclusions ')'`, is stored as one fact headed `and`, so the call there stored `(and (operand-zero …) …)`
and activated nothing, in silence. A wrong answer of the engine (below); once fixed, the call can stand in the
closing rule, and (iii) costs no firing at all.

**Proposed: (iii).** One hand-written form a base, at the cost the user took with (a); (ii) is the literal one
fact, at twice the saturation cost. [Q-M1f.266](../open_questions.md#q-m1f266--how-a-base-is-connected-to-the-shared-rules).

#### 2. The modules: the base's own module is the file that combines

Since one form connects a base, no file of some hundred facts exists, and the file the ruling asks for, *that
imports the two, does the combination and is what an end user includes*, is the base's module itself:
`std.math.decimal-arith` imports the shared rules and the tables, declares the base's relations, holds the base's
own rules and the connecting call, and is what a program imports, as today. **A separate combining file would buy
nothing and cost a list**: it would write the base's 63 relation names, which under S1f.6.35 a file other than the
declaring one lists by short name in its import, and the base's relations are of no use without the shared
rules (Q-M1f.266).

| module | file | imports | holds | entry, closing rule |
|---|---|---|---|---|
| `std.math.positional` | `stdlib/math/positional.ein` | nothing | the 78 shared rules, every relation a parameter in capitals (a lower-case one interns a bare name the harnesses read first); the macro `positional`, 63 parameters, whose body activates each rule once | the macro is its entry; no closing rule, since it declares no relation |
| `std.math.digits` | `stdlib/math/digits.ein` | nothing | as today: 7 tables, 1 664 rows, generated | none, facts |
| `std.math.bits` | `stdlib/math/bits.ein` | nothing | `bin-fa` 8, `bin-fs` 8, `bin-cl` 12, `bin-nz` 1: 29 rows, generated, with a `--check` in the gate through a cargo test | none, facts |
| `std.math.decimal-arith` | `stdlib/math/decimal-arith.ein` | `std.math.digits :symbols (fa fs md cl nz mid adj)`, `std.math.positional :symbols (positional)` | decimal's 74 relations, as today, every full name kept; its 37 own rules (the digit multiplier, the division of base ten, the two checks that list the digits, `division-by-zero-is-false`); the rule `decimal-arith-positional`, the call; `(arith-rule decimal-arith)` | `decimal-arith`: closes every relation and table as today, names its own 38 rules |
| `std.math.binary-arith` | `stdlib/math/binary-arith.ein` | `std.math.bits :symbols (bin-fa bin-fs bin-cl bin-nz)`, `std.math.positional :symbols (positional)` | binary's 61 relations, `b2` and every other name `bin-`; its 16 own rules (multiplication by a bit, division by one subtraction a bit, the divisor in normal form, its two checks, its `bin-division-by-zero-is-false`); `binary-arith-positional`, the call | `binary-arith`: closes its relations and the four tables, names its 17 own rules |
| `std.math.convert` | `stdlib/math/convert.ein` | `std.math.decimal-arith :symbols (n div-call div-done add-walk done)`, `std.math.binary-arith :symbols (bin-b bin-strip bin-done bin-top-first bin-top-first-is)` | `(to-bin X)` answered by `(bin-of X B)`, `(to-dec B)` by `(dec-of B X)`; 11 relations, 11 rules, `examples/binary_arithmetic/conv.ein`'s under the `bin-` names | `convert` |
| `std.math.int`, `.rat`, `.num9` | as today | `std.math.decimal-arith` as today | unchanged: every name they list, the tagged calls and `division-by-zero-is-false`, is still `std.math.decimal-arith`'s, under the same full name | as today |

A program that imports a base brings `std.math.positional` with it (an import brings its module's imports, one
copy a program), and one that imports both brings one copy, its rules activated once a base.

#### 3. What an end user writes

Each a real program under `build/s1f.6.16/design/p/`, run on the prototype's stdlib (`EIN_STDLIB=build/s1f.6.16/design/stdlib
ein test` and `ein solve`, 2026-11-14): each claim held, no candidate, one state, R {∅}, *monotone (the static
check)*. Under S1f.6.35, a file lists the short names it writes, and what it does not write prints by its full
name.

**Decimal only,** what a program writes today, unchanged:

```lisp
(import std.math.decimal-arith :symbols (decimal-arith n add div sum-of quot-of))
(add (n (n (n (n (n (n z 1) 2) 3) 0) 7) 8) (n (n (n (n z 4) 5) 6) 7))     ; 123078 + 4567
(div (n (n (n (n (n (n z 1) 2) 7) 6) 4) 5) (n (n z 2) 5))                 ; 127645 / 25
```

`?s = 127645`, `?q = 5105`, `?r = 20` (each as `(n …)`); 261 firings; 117 rules and 1 836 facts at load.

**Binary only,** and nothing of decimal loaded:

```lisp
(import std.math.binary-arith :symbols (binary-arith b2 bin-add bin-mul bin-sum-of bin-product-of))
(bin-add (b2 (b2 (b2 z 1) 0) 1) (b2 (b2 (b2 z 1) 1) 1))     ; 101 + 111
(bin-mul (b2 (b2 (b2 z 1) 0) 1) (b2 (b2 (b2 z 1) 1) 1))     ; 101 x 111
```

`(bin-sum-of … (b2 (b2 (b2 (b2 z 1) 1) 0) 0))`, 1100, and `(bin-product-of … (b2 (b2 (b2 (b2 (b2 (b2 z 1) 0) 0) 0) 1)
1))`, 100011; 81 firings. **It holds no decimal table row**: its `ein saturate --dump` writes no fact and no name
of `std.math.digits` or `std.math.decimal-arith` (`grep -c 'decimal-arith\|std\.math\.digits'`: 0); 96 rules and
166 facts at load, where the decimal program has 117 and 1 836.

**Both bases in one program,** each imported whole; they share `z` and the digits `0` and `1`, and no name else:

```lisp
(import std.math.decimal-arith :symbols (decimal-arith n add sum-of))
(import std.math.binary-arith :symbols (binary-arith b2 bin-add bin-sum-of))
(add (n (n (n (n (n (n z 1) 2) 3) 0) 7) 8) (n (n (n (n z 4) 5) 6) 7))
(bin-add (b2 (b2 (b2 z 1) 0) 1) (b2 (b2 (b2 z 1) 1) 1))
```

127645 and 1100; 56 firings.

**A conversion each way:**

```lisp
(import std.math.convert :symbols (convert to-bin to-dec bin-of dec-of))
(import std.math.decimal-arith :symbols (n))
(import std.math.binary-arith :symbols (b2))
(to-bin (n (n (n z 2) 5) 5))                                               ; 255
(to-dec (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 (b2 z 1) 1) 1) 1) 1) 0) 1) 0) 0) 0))   ; 1111101000
```

`(bin-of 255 11111111)` and `(dec-of 1111101000 1000)`, each number in its base's shape; 931 firings.

#### 4. The round-2 question under this design

> So we potentially can import std.math.rat and std.math.binary-arith and then connect them with single fact?

**The mechanism, yes; today's `std.math.int` and `std.math.rat`, no.** Their rules name decimal's relations:
`std.math.int` lists twelve of them, `n`, `operand`, `operand-is`, `add-walk`, `sub-walk`, `cmp-walk`,
`mul-call`, `div-call`, `done`, `order-is`, `div-done` and `division-by-zero`, and `std.math.rat` those and
`std.math.int`'s. Written as `std.math.positional` is, every such relation a parameter and a macro of the twelve,
a program, or a file `std.math.rat-binary` as `std.math.binary-arith` is binary's, would connect rationals to
binary with one call, `(rat-over b2 bin-operand bin-operand-is bin-add-walk …)`, at the few per cent measured
here. That is [S1g.10.7](../../m1g_logic/p1g.10_school_algebra/s1g.10.7_compound_numbers_without_a_hardcoded_representation.md)'s,
not built here.

#### 5. The open choices, each with a proposed default

Filed defaulted, the user's to revise:

- [Q-M1f.266](../open_questions.md#q-m1f266--how-a-base-is-connected-to-the-shared-rules): how a base is
  connected, **(iii) one call of a macro, in the base's own module**; or (ii) one fact; or (i) 78 facts; or a
  separate combining file.
- [Q-M1f.267](../open_questions.md#q-m1f267--the-binary-theorys-names): the binary theory's names, **`b2` kept,
  every other name `bin-`, the tables `bin-fa`, `bin-fs`, `bin-cl`, `bin-nz`; the conversions `to-bin`, `bin-of`,
  `to-dec`, `dec-of`**.
- [Q-M1f.268](../open_questions.md#q-m1f268--how-far-binary-flags-a-malformed-request): how far binary flags a
  malformed request, **as far as decimal**: the shared check and two rules of its own over `0` and `1`.
- [Q-M1f.269](../open_questions.md#q-m1f269--division-by-zero-is-false-stays-decimals): `division-by-zero-is-false`
  **stays decimal's**, since `std.math.num9`'s closing rule names it; binary has its own.
- [Q-M1f.270](../open_questions.md#q-m1f270--the-connecting-macros-body-written-or-generated): the macro's body
  **written by hand** in `std.math.positional` and held by a test that it activates each rule of the module once,
  with that rule's parameters.

#### 6. What the build then does, T1f.6.16.6 to .11

On the defaults, after the user's word on this design:

1. **T6, `std.math.bits`:** generated with its `--check` in the gate through a cargo test, as `std.math.digits` is,
   every row held to Rust's integers.
2. **`std.math.positional` and decimal re-cut** (the first half of T7): the 78 rules move out of
   `std.math.decimal-arith` as the trial wrote them, the macro beside them, `decimal-arith-positional` in their
   place; `arith_theory.rs`'s static tests read both files (the rule of the theory with a parameter as a head;
   the closing rule names the module's own rules, and a new test holds the macro's body), `the_tagged_calls_keep_their_shapes`
   unchanged in what it holds. Goldens named before blessing: every program that imports decimal moves in its
   rule names and its 78 facts traded; no claim and no exit moves (the trial's sweep says which 124).
3. **T7, `std.math.binary-arith`:** the 61 relations, the 16 own rules, the call; division by zero and the
   check as decimal's.
4. **T8, `std.math.convert`**, as above.
5. **T9, the tests:** binary's rows of `OPS` against Rust's integers, binary in `arith_search_free.rs`, a program
   a rule under `tests/stdlib/math/` (positional's rules activated by both bases' programs), a program that
   imports both and one that converts, the census and the mutants, the batch by hand on both bases.
6. **T10, the examples:** `examples/binary_arithmetic/` imports `std.math.*`; `bits.ein`, `bin.ein`, `conv.ein`
   and `gen_binary.py` retired; `examples/binary_logic/bits.ein` goes with S1f.6.17's build, which is moving that
   directory now.
7. **T11, the pages:** `binary_arithmetic.md`'s closing section, a `stdlib/README.md` section a module, and
   `AGENTS.md`'s calculator lines.

**E-63 first**, a wrong answer: its register row, witness and corpus entry in a commit of their own before any
fix (`build/s1f.6.16/design/and_in_and.ein` is the witness); the build needs no fix of it under (iii) as
designed, and needs one to put the call in the closing rule.
