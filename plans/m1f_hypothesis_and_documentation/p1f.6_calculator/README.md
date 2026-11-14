# P1f.6 — A calculator in Ein: school arithmetic as a theory over digit atoms, computed by saturation alone

**Milestone:** [M1f](../README.md)

**Names since [S1f.6.31](s1f.6.31_the_stdlib_layout.md) (2026-11-14):** the calculator's modules are under
`std.math`, each in `stdlib/math/`: `std.arith` is **`std.math.decimal-arith`** (`math/decimal-arith.ein`, its entry
`arith` now **`decimal-arith`**), `std.digits` is `std.math.digits`, `std.int` `std.math.int`, `std.rat` `std.math.rat`
and `std.num9` `std.math.num9`; their programs moved from `tests/stdlib/arith/`, `digits/`, `int/`, `rat/` and `num9/`
to `tests/stdlib/math/decimal-arith/`, `digits/`, `int/`, `rat/` and `num9/`. A full name moves with its module,
`std.arith.add` to `std.math.decimal-arith.add`; no relation was renamed and no answer moved. This README's history
and the stage files below are records and keep the names of their day.

**Status:** planned 2026-11-08, on the user's instruction of that day ([§ The instruction](#the-instruction)).
**S1f.6.1, the design, is done (2026-11-11):** three rounds on a prototype and the user's rulings of that day,
recorded in [`design.md` § The rulings](design.md#the-rulings-2026-11-11). **The phase was re-planned to them the
same day** ([§ The design, as ruled](#the-design-as-ruled-2026-11-11)). **S1f.6.11, the import fix, is done
(2026-11-11):** a flat import brings a module's facts, W88's four rows filed and fixed
([its records](s1f.6.11_importing_facts.md#t1-record-2026-11-11)). **S1f.6.2, the tables, is done
(2026-11-11):** `std.digits`, 1 664 rows in seven tables, generated and held to the integers
([its records](s1f.6.2_digits_and_tables.md#t1-record-2026-11-11)). **S1f.6.3, addition, is done
(2026-11-11):** `std.arith` begins, 19 rules, with the tests every later operation joins
([its records](s1f.6.3_addition.md#t1-record-2026-11-11)). **S1f.6.4, subtraction and comparison, is done
(2026-11-11):** `(diff-of X Y D R)` and `(cmp-of X Y O)`, 18 rules more
([its records](s1f.6.4_subtraction.md#t1-record-2026-11-11)). **S1f.6.5, multiplication, is done
(2026-11-11):** `(product-of X Y P)`, 16 rules more
([its records](s1f.6.5_multiplication.md#t1-record-2026-11-11)). **S1f.6.6, division, is done
(2026-11-11):** `(quot-of X Y Q R)` by bisection, and division by zero as a flag and `(false)` from it, 36
rules more; the four operations and comparison are built
([its records](s1f.6.6_division.md#t1-record-2026-11-11)).
**S1f.6.13, `std.num9`, is done (2026-11-11):** the user's own number as a second module, wide rules, 47 of
them, generated ([its records](s1f.6.13_num9.md#t1-record-2026-11-11-the-generator-and-the-module)).
**S1f.6.12's investigation is done and stopped for the user (2026-11-11):** what a theory can detect of a
malformed number without negation, what it costs, and what a name clash does today
([`malformed_numbers.md`](malformed_numbers.md)).
**S1f.6.15, load against calculation, is done (2026-11-11):** three generated examples and
`utils/arith_cost.py`; the tables are the cost up to a hundred requests, and the parse past a thousand sums
([its records](s1f.6.15_load_against_calculation.md#t1-record-2026-11-11)).
**S1f.6.7, negative numbers, is built but for division (2026-11-12):** `std.int`, `(int P N)` over `std.arith`,
36 rules of one premise each ([its records](s1f.6.7_negative_numbers.md#t1-record-2026-11-12)). **Its division
is built since S1f.6.22 (2026-11-12)**, on the user's ruling: three conventions, each a request of its own, the
Euclidean remainder a natural number ([its record](s1f.6.22_integer_division.md#record-2026-11-12)).
**S1f.6.8, rational numbers, is done (2026-11-12):** `std.rat`, `(rat Z D)` in lowest terms over `std.int` and
`std.arith`, 56 rules of one premise each, a zero denominator `std.arith`'s division by zero
([its records](s1f.6.8_rational_numbers.md#t1-record-2026-11-12)).
**S1f.6.9, what is left of the width, is measured and stopped for the user (2026-11-12):** the nesting limit
re-taken on the shipped theory, a state file refused on reload from 999-digit operands, the cost by length, both
on the page; the choice of what to do is Q-M1f.185
([its records](s1f.6.9_width.md#t1-record-2026-11-12-the-limit-re-taken)).
**S1f.6.10, the investigation, is done and shown to the user (2026-11-12):** equations by saturation and by
search, cryptarithms, bounded theorem checking, number theory and explanations, each probed with its cost;
sixteen probes banked under `examples/beyond_arith/`; E-29 to E-31 found; what becomes a stage is the user's
([`beyond_arithmetic.md`](beyond_arithmetic.md), [its records](s1f.6.10_beyond_arithmetic.md#t1-record-2026-11-12)).
**The user replied to the phase's questions on 2026-11-12** ([`questions.md`](questions.md), `c828dac3`; what stayed open is [`questions_round2.md`](questions_round2.md), replied and committed as `dc88520b`). Every reply and what it became is [`replies_2026-11-12.md`](replies_2026-11-12.md): thirteen stages planned, S1f.6.21 to S1f.6.33, the builds of S1f.6.12, S1f.6.16 and S1f.6.17 ruled, and S1f.6.14 moved to P1f.8 (now M1g's P1g.10).
**Depends on:** [P1f.5](../p1f.5_the_no_owner_items/README.md), for the language the theory is written in.
[S1f.5.4](../p1f.5_the_no_owner_items/s1f.5.4_digits_are_names.md) makes `0` to `9` plain names, so
`(num9 0 0 0 1 2 3 0 7 8)` is nine names and not nine ints. It lands with S1f.5.5's swap, and T1f.5.4.3 retires
the int kind. [S1f.5.7](../p1f.5_the_no_owner_items/s1f.5.7_eq_and_neq_are_relations.md),
[S1f.5.11](../p1f.5_the_no_owner_items/s1f.5.11_closed_keywords.md) and
[S1f.5.14](../p1f.5_the_no_owner_items/s1f.5.14_nl_required.md) settle the guards, the keywords and a
required `:why`. S1f.6.1 can run once the swap has landed, since it writes programs in the final grammar. The
theory stages, S1f.6.2 onward, run after P1f.5 closes, so the theory is written once.
**Blocks:** nothing.

## The instruction

The user, 2026-11-08:

> plan a phase p1f.6 - calculator in ein; define a theory that allows making fixed length digital numbers like
> this (num9 0 0 0 1 2 3 0 7 8) == 123078, where single digits are separate atoms used in all theory and define
> operations - sum, mul, sub with remainder and division with reminder, so sub and div produces 2 numbers,
> implement just simple school style summation, multiplication and division, so the program in ein can import
> this and calculate some simple arithmetic, first stage would be design proposal that will show user a couple of
> target example programs; also propose extension with negative numbers (e.g. numbers containing 2 parts -
> positive and negative, that can be simplified until one of them is zero; also propose extension with rational
> numbers in this case containing integer numerator and natural denominator; with overflow detection or width
> extension if it is possible, I'm not sure now but maybe relation could have variadic arguments then can write
> numbers in reverse order e.g. (num 8 7 0 3 2 1) == 123078; additional stage in the end of the phase -
> investigation of what can be done with this theory except simple arithmetic, can it be used for any kind of
> theorems proving, or solving equations, or numbers theory exploration, or anything else.

and, the same day, on how it computes:

> I expect ideally solving simple arithmetic will utilize only saturation with hardcoded summation, subtraction,
> multiplication and divisiono tables like it is done in electronics, so search will not be involved at all

## The design, as ruled (2026-11-11)

The user's rulings, in their words, are [`design.md` § The rulings](design.md#the-rulings-2026-11-11). What they
make of the plan:

- **The number** is nested, of any length, in reading order: `z`, or `(n PREFIX DIGIT)`, ten times the prefix
  plus the digit. 123078 is `(n (n (n (n (n (n z 1) 2) 3) 0) 7) 8)`. Every answer is in normal form, with no
  high zero. The user's own `(num9 0 0 0 1 2 3 0 7 8)` ships as a second module.
- **Three modules:** `std.digits`, the tables as plain facts, the stdlib's first library of facts; `std.arith`,
  the theory over nested numbers; `std.num9`, the fixed-width theory.
- **The protocol:** a request fact, `(add X Y)`, answered by a result fact that repeats its operands,
  `(sum-of X Y S)`, read by a goal with a variable in the result's place. Subtraction and division answer two
  numbers. Named constants, `(let NAME NUMBER)`. **No expression layer in the core:** it waits for the user.
- **Division by zero** derives a flag, and a second rule derives `(false)` from the flag. A malformed number
  will do the same where the theory can detect it.
- **A library of facts is importable flat,** by an engine fix that runs before the theory.

## The principle: a circuit, not a search

The theory computes arithmetic the way hardware does. The digit tables are the gates and the rules are the
wiring. Every step is a table lookup whose outcome the tables decide, so **nothing is guessed**. In Ein's terms:

- **No search.** The answer is root's fixpoint. On every arithmetic program H is empty, R is {∅} and k_s is 1,
  with no fork and no hypothesis. `ein solve`, `ein saturate` and `ein test` read the same facts.
- **Monotone.** No `not` premise anywhere in the theory, and no `absent` in any computation: a comparison is
  derived outright, from a borrow or from a table, so choosing a quotient digit needs no negation. **One
  `absent` since [S1f.6.12](s1f.6.12_malformed_numbers.md#build-record-2026-11-13)** (the user's reply of
  2026-11-12, round 2's item 5 (b)), in the rule that flags a name no `let` states: over `let`, which a program
  writes and no rule derives and which the theory closes, so the premise is decided at root, the static check
  calls every program that imports the theory monotone, and it has two strata where it had one.
- **Demand-driven.** A program states a request, `(add X Y)`. Rules answer only that request, and the requests
  the circuit makes itself. Nothing enumerates the numbers, and a conversion between shapes is a request too.
- **Measured as a circuit.** Each operation reports its firings, the circuit's size, and its depth, the longest
  chain of firings. Ein's saturator is an agenda with no rounds of its own; the depth is the rounds a
  synchronous circuit would take (`design.md`, round 1).

**How the nested theory is wired** (design K, [`design.md` § Round 3](design.md#round-3-2026-11-11-several-representations-at-once)):

- **One walk from the units,** each column through its table, its digit put in front of an accumulator. The
  accumulator holds the answer in the other order, most significant digit outermost.
- **High zeros are dropped there,** where they are outermost, and the answer is turned round into the public
  shape.
- **Every call carries its continuation,** a tag that holds what the caller needs next.
- **Division bisects** the divisor's nine multiples: at most four trial subtractions for a quotient digit.

**The rule of the theory: no rule joins two facts on a number.** A rule has one premise that holds numbers, and
its other premises are table rows looked up by digits. It is a stated rule of `std.arith`, in the module's
header, and a test holds it. The reason is measured: the engine's fact index does not key on a nested fact
(E-3 of [`findings.md`](findings.md)), so an ordinary join on a number scans its relation, and a batch of
requests costs quadratically. With the rule, 4 000 nine-digit divisions take 2.2 s on the prototype; round 2's
theory, written with ordinary joins, did not finish 1 000 in 45 s (`python3 r3_table.py`, 2026-11-11).

What breaks the principle is a finding, not a cost to accept. If an operation cannot be computed without a
hypothesis, or H is not empty on an arithmetic program, the stage stops and shows the user.

**Where search does belong** is S1f.6.10. Once arithmetic is a circuit, an *inverse* question, such as which x
makes x + 4567 = 127645, is a puzzle. Ein's search answers puzzles, and that stage asks what it makes of these.

## What this is not

- **Not native numbers.** The user's ruling of 2026-11-06 stands
  ([Q-M1e2.24](../../m1e2_correctness/open_questions.md)): Ein has no numbers, no ranges, and no arithmetic
  predicate or token. This phase writes a theory *in* Ein, over ten digit names.
- **Not an engine change, except the ones a ruling made or may make.** One is ruled: a flat import brings a
  module's facts ([S1f.6.11](s1f.6.11_importing_facts.md)). One is a question S1f.6.12 puts to the user: the
  loader holding a nested fact to its relation's declared arity (E-2). Variadic relations are no longer asked
  for.
- **Not a language of function calls.** The user, 2026-11-11: *"Ein is not Lisp, is it?"*. A computation is a
  request fact and a result fact, composed by rules. Nested expressions are a question, not a plan.
- **Not the harness's arithmetic.** A test may hold the theory's answers to Rust's integers; that is the
  oracle for the theory, and it never enters a program.

## The stages

**The investigations are closed** (the user, 2026-11-12: *"enough investigations, just write down results in documentation"*). Their results are the pages named in the table, [`malformed_numbers.md`](malformed_numbers.md), [`binary_arithmetic.md`](binary_arithmetic.md), [`binary_logic.md`](binary_logic.md), [`word_problems.md`](word_problems.md) and [`beyond_arithmetic.md`](beyond_arithmetic.md), with [`findings.md`](findings.md); no further probe is run, and what is left of each stage is a build that waits for the user's ruling on its page's questions.

**The questions that wait for the user are in one file, [`questions.md`](questions.md)** (2026-11-12, on the user's instruction: *"write questions into one file in phase plans leaving places for my replies"*): every open question of the pages above, of the stage files and of the defaults taken during the phase, the blocking ones first, each with a place for the reply, and the findings that need an owner.

| ID | title | depends on | status |
|---|---|---|---|
| [S1f.6.1](s1f.6.1_design_proposal.md) | The design proposal: representations, the protocol, and target programs computed on a prototype | S1f.5.5's swap | **done 2026-11-11**: [`design.md`](design.md), three rounds and the rulings |
| [S1f.6.11](s1f.6.11_importing_facts.md) | A flat import brings a module's facts: the engine fix, with its register rows, its census and its sweep | — | **done 2026-11-11**: W88 ×4 filed and fixed, the sweep moved the witnesses alone, Q-M1f.135 to .144 defaulted, E-8 found |
| [S1f.6.2](s1f.6.2_digits_and_tables.md) | The digits and the hardcoded tables, as a library of facts, `std.digits`, generated with a `--check` | S1f.6.11 | **done 2026-11-11**: 1 664 rows in seven tables, each held to Rust's integers; closed by written facts; Q-M1f.145 to .148 defaulted, E-9 and E-10 found |
| [S1f.6.3](s1f.6.3_addition.md) | Addition, and the machinery every circuit shares: the number, the protocol, named constants, accumulators and tags, and the test of the rule of the theory | S1f.6.2 | **done 2026-11-11**: `stdlib/arith.ein`, 19 rules; sums held to Rust's integers; search-free on the binary; no rule joins two facts on a number, on the text and on the matcher's count, a gate step; Q-M1f.149 to .154 defaulted, E-11 found |
| [S1f.6.4](s1f.6.4_subtraction.md) | Subtraction with its two numbers, and comparison | S1f.6.3 | **done 2026-11-11**: 18 rules more, 37; `(sub X Y)` answered by `(diff-of X Y D R)`, two walks side by side, and `(cmp X Y)` by `(cmp-of X Y lt\|eq\|gt)`; every operation a row of the harness against the integers, and measured in batches of its own; Q-M1f.155 to .158 defaulted, E-12 found |
| [S1f.6.5](s1f.6.5_multiplication.md) | Multiplication | S1f.6.3 | **done 2026-11-11**: 16 rules more, 53; `(mul X Y)` answered by `(product-of X Y P)`, the school method from the multiplier's top digit, tagged so a later theory can call it; 123 × 456 is 72 firings; the DOT sweep runs several files at a time; Q-M1f.159 to .162 defaulted |
| [S1f.6.6](s1f.6.6_division.md) | Division with remainder, the quotient digit by bisection; division by zero as a flag and `(false)` from it | S1f.6.4, S1f.6.5 | **done 2026-11-11**: 36 rules more, 89; `(div X Y)` answered by `(quot-of X Y Q R)`, 127645 ÷ 25 in 220 firings; a division by zero derives the flag and the flag `(false)`, every surface shown in the record; the two families of nine rules held alike by a test; Q-M1f.163 to .165 defaulted |
| [S1f.6.12](s1f.6.12_malformed_numbers.md) | Malformed numbers, and the names the theory occupies: an investigation first, then the detection | the prototype for its first task; S1f.6.6 for the build | **T1 done 2026-11-11, stopped for the user**: [`malformed_numbers.md`](malformed_numbers.md), on the shipped theory: a digit's place, a number's end and a twice-named constant can be flagged with no negation for twenty firings a request; a name no `let` states and a foreign fact cannot; a wrong count of arguments is the loader's, and a census finds nothing that loads refused; six questions; E-27 found **The build ruled 2026-11-12** (replies 4 (a), 6 (a), 7 (a), 9 (b); round 2's item 5 (b)): [§ The build, as ruled](s1f.6.12_malformed_numbers.md#the-build-as-ruled-2026-11-12); the arity check is [S1f.6.23](s1f.6.23_the_loaders_arity_check.md). **Built 2026-11-13** ([its record](s1f.6.12_malformed_numbers.md#build-record-2026-11-13)): 26 rules and 7 relations more in `std.arith`, 114 rules and its closing rule; a malformed operand derives `(malformed REQUEST)`, a `let` that cannot be read `(malformed (let NAME NUMBER))`, a name no `let` states `(undeclared NAME REQUEST)` through the theory's one `absent`, and each flag `(false)`; the check is a pass at the operand, a firing a digit of each operand; `std.int`, `std.rat`, `std.num9` and the binary example unchanged and silent; Q-M1f.205 to .209 defaulted, Q-M1f.210 open (the two modules over it); E-27 closed, E-36 to E-38 found. |
| [S1f.6.13](s1f.6.13_num9.md) | Fixed-width numbers as a second module, `std.num9`, with overflow and conversions | S1f.6.2, S1f.6.6 | **done 2026-11-11**: `stdlib/num9.ein`, generated, 47 rules; a sum, a difference and a comparison in one firing, a product and a quotient in eleven; overflow a flag and never a number, a division by zero through `std.arith`'s rule; conversions each way; exhaustive against the integers at width 2; no rule joins two facts on a number, so 4 000 divisions take 1.24 s where the prototype took 8.1; Q-M1f.167 to .173 defaulted, **.173 for the user to confirm**: a wide rule takes up to 162 of the engine's 256 registers, and the tripwire on that bound now measures the module apart; E-28 found |
| [S1f.6.7](s1f.6.7_negative_numbers.md) | Extension: negative numbers as a positive and a negative part, simplified until one is zero | S1f.6.6 | **built but for division, 2026-11-12** (`f1fe6918`): `stdlib/int.ein`, 36 rules and its closing rule, every rule one premise, a call's tag its request; simplification, negation, addition, subtraction, multiplication and comparison held to Rust's signed integers, every pair of one-digit parts exhaustively; 123078 + −4567 in 59 firings; Q-M1f.174 to .178 defaulted. **Integer division is open: Q-M1f.179, the remainder convention, for the user**, the three written as rules and compared in the record **Integer division ruled 2026-11-12** (replies 1 (d), 2 (a)): three conventions, each its own request; **built 2026-11-12** by [S1f.6.22](s1f.6.22_integer_division.md). |
| [S1f.6.8](s1f.6.8_rational_numbers.md) | Extension: rational numbers, an integer numerator over a natural denominator | S1f.6.7 | **done 2026-11-12** (`f8f605a3`): `stdlib/rat.ein`, 56 rules and its closing rule, every rule one premise; `(rat-add X Y)` answered by `(rat-sum-of X Y S)` and so on, in lowest terms by Euclid's chain over `std.arith`'s division, and it needs no division of integers; a zero denominator, and a division by a rational that is zero, derive `std.arith`'s flag and `(false)`; held to exact fractions, 28 operands exhaustively; 1/2 + 1/3 is 492 firings and 1/2 against 1/3 is 76; Q-M1f.180 to .184 defaulted |
| [S1f.6.9](s1f.6.9_width.md) | What is left of the width: how long a number may be (the nesting limit, the size of a state) | S1f.6.6 | **measured and stated 2026-11-12** (`55aeaf08`)**, stopped for the user**: a program writes a number of 999 digits and no longer; a state is two to seven levels deeper than its request, so the state file of a 999-digit sum is refused on reload, 998 for a product, 997 for a sum of integers, 994 for a rational's numerator; the engine itself saturates facts 396 000 deep; a derived fact of a 998-digit computation is about 40 KB of text, a 998-digit sum's state 86 MB, 998 digits by 998 five million facts and 5.9 GB; the page statement written, flagging by rule and an engine change probed and not built. **Q-M1f.185, the choice, for the user**; E-4 re-taken **Ruled 2026-11-12** (reply 44 (a)): say it on the page and leave it, which is done; **closed**, nothing left to build. |
| [S1f.6.20](s1f.6.20_word_problems.md) | An application: elementary-school word problems, the calculator imported among other relations; the words' meaning as rules, and a missing quantity detected and named | S1f.6.6 | **T1 done 2026-11-11** (`50b34105`)**, stopped for the user**: [`word_problems.md`](word_problems.md), 29 programs on a prototype under `build/s1f.6.20/` (gitignored, so the page carries the theory in full); twelve questions, open. **Banked 2026-11-12**, on the user's instruction of that day, with the variants the page recommends: [`examples/word_problems/`](../../../examples/word_problems/README.md), three modules and thirteen problems, each a corpus entry with its claim, every answer re-taken and the page's ([its record](s1f.6.20_word_problems.md#t2-record-2026-11-12-the-prototype-banked)); examples and not stdlib, and not a ruling on the questions. The remainder variant, 13 apples, is not banked: a test of the gate does not end on it (E-33, Q-M1f.198) **Closed 2026-11-13**: what the replies of 2026-11-12 asked for is [S1f.6.28](s1f.6.28_word_problems_second_pass.md); nothing is left under this id. |
| [S1f.6.14](s1f.6.14_expressions.md) | Optional: nested expressions, `(calc E)` and `(let NAME E)` | S1f.6.6, and the user's ruling | **moved to P1f.8 on 2026-11-12** (reply 10; P1f.8's algebra is M1g's P1g.10 since 2026-11-13): [S1g.10.2](../../m1g_logic/p1g.10_school_algebra/s1g.10.2_expressions_and_evaluation.md#moved-here-from-s1f614-2026-11-12) |
| [S1f.6.15](s1f.6.15_load_against_calculation.md) | Long numbers and many calculations: examples that set load time against calculation time (the user, 2026-11-11) | S1f.6.6 | **done 2026-11-11**: `long_numbers.ein`, `many_calculations.ein` and `factorial.ein`, generated, sized to what every per-commit surface affords; `utils/arith_cost.py` and its tables in `stdlib/README.md`: 20.6 ms of load a program, a `.einb` that buys none of it back, the parse ahead of the calculation past a thousand sums, and `ein solve` 30 to 90 times `ein saturate` on long numbers; Q-M1f.166 defaulted, E-24 to E-26 found |
| [S1f.6.16](s1f.6.16_binary_arithmetic.md) | Arithmetic with binary numbers: whether the rules get simpler, and whether the theory is generic in its base (the user, 2026-11-11) | S1f.6.6 | **T1 done 2026-11-11** (`c78387af`)**, stopped for the user**: [`binary_arithmetic.md`](binary_arithmetic.md), three ways built on a prototype under `build/s1f.6.16/`; nothing shipped; seven questions. **Way (b) banked as examples 2026-11-12** (`9612b350`; the user's instruction of that day): [`examples/binary_arithmetic/`](../../../examples/binary_arithmetic/README.md), the tables and the theory generated from `std.arith`, the conversions and five programs with claims, no answer different from the page's; not a ruling, the questions stay open **The module ruled 2026-11-12** (replies 13 (b), 14 (b), 15 (a), 16, 17 (b)): [§ The build, as ruled](s1f.6.16_binary_arithmetic.md#the-build-as-ruled-2026-11-12), after the layout; a trial of parameterised shared rules first. **The trial done 2026-11-14, and the build stopped for the user** at the trial's own rule ([record](s1f.6.16_binary_arithmetic.md#record-2026-11-14-t1f6165-the-trial-and-the-stop)): the 78 shared rules written once, every relation a parameter, do the shipped rules' work to the count (firings, depth, candidates; every decimal read-out of 143 programs the same), but take 3 to 5 % more saturation and 1 to 4 ms more load, and decimal's states trade 78 `(arith-rule R)` facts for 78 activators; nothing built. [The question](s1f.6.16_binary_arithmetic.md#the-question-for-the-user): (a) the shared file at that cost, (b) the generated copy, (c) an engine stage first; recommended (a). E-60, E-61. |
| [S1f.6.17](s1f.6.17_binary_logic.md) | Binary logic: truth tables, gates, a circuit as a program, and six applications tried; the starting point of [M1g](../../m1g_logic/README.md) (the user, 2026-11-11) | S1f.6.16 | **T1 done 2026-11-11** (`c78387af`)**, stopped for the user**: [`binary_logic.md`](binary_logic.md), the sixteen functions, thirty derived properties and six applications on a prototype under `build/s1f.6.17/`; nothing shipped; eight questions. **The recommended variants banked as examples 2026-11-12** (`9612b350`; the user's instruction of that day): [`examples/binary_logic/`](../../../examples/binary_logic/README.md), the one table, the properties as generated facts with the rules that derive them as their check, three small theories and the six applications each a program, no answer different from the page's; not a ruling, the questions stay open **The modules ruled 2026-11-12** (replies 19 (c), 20 (b), 21 (b), 22 (a), 23 (a)): [§ The build, as ruled](s1f.6.17_binary_logic.md#the-build-as-ruled-2026-11-12), as `std.bool.*`, after the layout. |
| [S1f.6.10](s1f.6.10_beyond_arithmetic.md) | Investigation: what else the theory can do, from equations and puzzles to bounded theorem checking and number theory | S1f.6.8, S1f.6.9 | **done 2026-11-12** (`47982ced`)**, shown to the user**: [`beyond_arithmetic.md`](beyond_arithmetic.md). One unknown is solved by saturation alone, the tables read backwards; unknown digits as hypotheses are a search, 10n + 1 saturations under `--single` a column at a time; SEND + MORE = MONEY unique in 17 747 saturations, 0.5 s, or no answer in 600 s by the order its letters are declared; the laws hold on every instance over 0 to 9, and Ein checks the full adder's 200 rows where the induction on columns is outside it; primes to 5 000 and the rest agree with Python, memory the limit; a refuted equation's core is a three-row proof. Sixteen probes banked, `examples/beyond_arith/`; Q-M1f.186 to .189 defaulted, E-29 to E-31 found; eight questions, **what becomes a stage is the user's** |
| [S1f.6.21](s1f.6.21_the_gate_reduced.md) | The gate reduced: big graphs skipped, and no counters step | — | **done 2026-11-12** (replies 45, 46 (b)): a view past 1 000 edges is skipped and counted, the counters step is out of the gate and run by hand |
| [S1f.6.22](s1f.6.22_integer_division.md) | Integer division, under three conventions | S1f.6.7 | **done 2026-11-12** (replies 1 (d), 2 (a)): `int-div-euclid`, `int-div-trunc` and `int-div-floor` in `stdlib/int.ein`, 34 rules of one premise each, answered by `int-quot-euclid-of` (the remainder a natural number), `int-quot-trunc-of` and `int-quot-floor-of`; held to `i128` under every case the other operations are; a division by zero is `std.arith`'s flag; Q-M1f.200 defaulted |
| [S1f.6.23](s1f.6.23_the_loaders_arity_check.md) | The loader holds a form to its relation's declared count of arguments; an engine stage | — | **done 2026-11-12** (reply 3 (a)): a form headed by a declared relation whose arguments are not as many as its declaration's types is refused at load, on every command, one line a form; W89's seven rows filed and fixed; no program that loaded is refused; Q-M1f.201 to .204 defaulted; E-2 closed, E-34 and E-35 found |
| [S1f.6.24](s1f.6.24_division_by_zero_at_the_call.md) | Division by zero is flagged at the call, whoever made it | S1f.6.12's build | **done 2026-11-13** (reply 48; round 2's item 48, agreed): `std.arith` flags a division by zero at the tagged call, `(division-by-zero (div-call TAG X Y))` and `(false)`, whoever made the call, by one rule more, `div-call-by-zero`; a public request keeps its flag beside the call's (design (ii), [Q-M1f.211](../open_questions.md#q-m1f211--which-fact-carries-the-flag-of-a-division-by-zero-at-the-call), defaulted) and `ein solve` reads as it did; `std.int`'s and `std.rat`'s rules stay, none reaching a call with a zero divisor; a rule of a program's own that calls the division by zero, silent until now, is dead with its own fact as the core. Q-M1f.211 to .213 defaulted, E-39 filed, the register unmoved; [its record](s1f.6.24_division_by_zero_at_the_call.md#record-2026-11-13) |
| [S1f.6.25](s1f.6.25_the_section_j_fixes.md) | The findings fixed in this phase: E-5, E-8, E-11, E-20, E-22; an engine stage | — | **done 2026-11-13** (section J left empty, so each fix on the default): E-5, `--trace`'s *Derives* line names every conclusion of a firing; E-8, no import brings a module's `(query …)`, `(solve …)` or `(config …)` (W90); E-11, a name a selection lists is live in the tree-shake, so a state file re-writes the program as loaded (W91, schema `/22`), the workaround facts kept; E-20, one `note:` line where a selection brings a relation's declaration and no rule that derives it; E-22, a count of commitments is exact and reported up to 2^63 − 1, *or more*, exit 0 (W92). Q-M1f.214 to .218 defaulted; found E-40, E-41 and E-42; [its record](s1f.6.25_the_section_j_fixes.md#record-2026-11-13) |
| [S1f.6.26](s1f.6.26_what_root_owes.md) | A read-out row for what root owes | — | **done 2026-11-13** (reply 30 (a)): where the `∅` row reads *not in R — root owes n*, `owed` rows under it name each instance through its rule's `:why`, at most four on stdout and one line for the rest, every one in `answer.txt` and in the summary's new `region.root_owed` (E-19); a query's `over` row over an empty R says *root is dead* or *root is alive and owes n instances* (the row half of E-18); no claim's outcome, verdict word or exit code moved, and R, N and H are the same on every tracked program (the sweep, 849 files). Q-M1f.219, a query's `(false)` on a program that only owes, defaulted to no change and left to M1g's P1g.7; Q-M1f.220, the rows' form, defaulted: the cut is the warning's, four rows and a line, and not the listings' 100 lines, on T1's measurement. Found E-43, E-44 and E-45; [its record](s1f.6.26_what_root_owes.md#record-2026-11-13) |
| [S1f.6.27](s1f.6.27_ein_test_by_the_cheapest_route.md) | `ein test` decides a claim by the cheapest route that decides it; an engine stage | S1f.6.25 | **done 2026-11-13** (round 2's item 39 (a)): `ein test` reads a file's claims by their shapes and asks the least question that can decide them, `--single` for a claim of at most one class and `-n k+1` for one of k over the one part of R they are about, describe for a solve claim or claims over more than one part, and the describing question, of the program loaded again, for what the first left undecided; no option chooses it, and under `ein test` a state found whose class a claim does not list refutes it. `-v` names the question, `--json-report` is `ein-test-report/5` with `route` and `fell_back_from`, and a claim still NOT CHECKED says what was asked first (E-30). `claim_routes.rs` holds each question's verdict to the description's and the oracle's on 1 710 claims over 459 programs, with no disagreement; no corpus claim's outcome moved (the sweep, 849 files, 465 claims). E-31: N is taken only by a run that prints it or its count, so `ein solve --count` on x × 7 at five digits went from 5.02 s to 0.21 s with every surface byte for byte; a bare describe still pays, Q-M1f.224. SEND + MORE = MONEY is banked, `examples/beyond_arith/send_more_money.ein`, held by `--single` in 0.46 s where it was NOT CHECKED after 7.39 s, and slow by its bare `solve`. Q-M1f.221 to .224 defaulted; found E-46, fixed; [its record](s1f.6.27_ein_test_by_the_cheapest_route.md#record-2026-11-13) |
| [S1f.6.28](s1f.6.28_word_problems_second_pass.md) | Word problems, second pass: a reading is always chosen, and *what is not listed is zero* | S1f.6.26 | **done 2026-11-13** (replies 28, 29, 31, 32, 35; round 2's items 31 and 32): **E-33 resolved** with the fact-grain test's meaning unchanged, the closure computed only where a relation-grain site reads it and giving up on the safe side on a flagged program, where the test fails naming it (Q-M1f.225; every verdict of the 743 files and the 437 cubes the same); `readings.ein` rebuilt to owe a reading always, so `boxes12.ein` is two states and one answer and **the 13-apple problem is banked**, `boxes13.ein`, two states, 4 and 5, every mode against the oracle and the arithmetic apart from the engine; what the traces say of *why divided* written down; `listed_is_all.ein` and `kinds_listed.ein`, *what is not listed is zero* adopted for `has`; `before.ein`, a witness whose right answer, 7, the theory does not give (M1h P1h.1's); the directory's README says the banked forms are *"not only and not final"*. E-47 and E-48 filed, Q-M1f.225 and .226 defaulted ([the record](s1f.6.28_word_problems_second_pass.md#record-2026-11-13)) |
| [S1f.6.29](s1f.6.29_the_biggest_calculation.md) | The biggest calculation: a generator with size arguments | S1f.6.15 | **done 2026-11-13 (closed on the user's word, the solve half partly measured)** (reply 49; round 2's item 49: about 16 GB is the bound, run by no test and no CI step; *"just write down generated examples and close"*): `utils/gen_arith_examples.py --op OP … -o FILE` writes a program of a stated size, the same arguments the same bytes, never checked in and run by no test or tier; `utils/arith_cost.py --biggest` searches the sizes, each run in a 16.0 GB cgroup (Q-M1f.227); `stdlib/README.md` § The biggest calculation: under `ein saturate` 999 digits by 999 (14.9 s, 6.13 GB, the parser's limit), 998 by 499 (the parser's), 43 430 requests (60.4 s, at the cap) and 1 081! (47.0 s, at the cap); under `ein solve` 547 by 547 and 626 by 313 at the 600 s limit, 30 379 requests in 277.2 s at the cap, searched to there, its chain not measured ([the record](s1f.6.29_the_biggest_calculation.md#record-2026-11-13)) |
| [S1f.6.30](s1f.6.30_name_clashes_between_imported_theories.md) | Name clashes between imported theories: investigation, proposals, and a stop | — | planned 2026-11-12 (reply 8); **stopped for the user at T5, 2026-11-13**: T1 built (the tagged calls an interface, held by a test), the probes, the census and six proposals on [`import_clashes.md`](import_clashes.md); **ruled and closed 2026-11-13**: the user's model (h), full names and short names ([`import_clashes.md` § 7](import_clashes.md#7-the-ruling-2026-11-13-h-full-names-and-short-names)), Q-M1f.231 to .236 ruled, .237 to .240 defaulted; T6 is [S1f.6.35](s1f.6.35_imports_by_full_name.md) |
| [S1f.6.31](s1f.6.31_the_stdlib_layout.md) | The stdlib layout: `std.math.*` and `std.bool.*` | S1f.6.22, S1f.6.12's build, S1f.6.24; the layout reply | **done 2026-11-14** (replies 16, 38, 42, 52; round 2's layout, *"Confirm others"*): the five modules that exist under `stdlib/math/`, `std.arith` as `std.math.decimal-arith` with its entry `decimal-arith`, `std.digits`, `std.int`, `std.rat`, `std.num9` as `std.math.*`, their programs under `tests/stdlib/math/` so the suite mirrors the library; restated after S1f.6.35 first, a full name moving with its module; no engine change (T1's probe, all three resolutions), no relation renamed, no alias; the readers that saw one directory, the manifest's script and `Source::modules()` among them, read every depth, and two tests fail on a module the manifest or the coverage gate does not see; the sweep, every tracked program under `ein solve` and `ein saturate --dump` on both layouts, equal but for the names (1 712 of 1 712); 4 787 shape digests moved on 144 files, every one a calculator file; Q-M1f.261 and .262 defaulted, E-58 and E-59 found and fixed ([its record](s1f.6.31_the_stdlib_layout.md#record-2026-11-14)). `std.bool.*` is S1f.6.17's build |
| [S1f.6.32](s1f.6.32_gcd_and_lcm.md) | gcd and lcm, as `std.math.gcd` | S1f.6.31, S1f.6.24 | planned 2026-11-12 (reply 38) |
| [S1f.6.33](s1f.6.33_lowest_terms_on_request.md) | `std.rat` reduces only when asked | S1f.6.32 | planned 2026-11-12 (round 2's item 54 (b)) |
| [S1f.6.34](s1f.6.34_hypothesis_mechanisms_in_conflict.md) | A conflict between the hypothesis mechanisms is reported; an engine stage | — | **done 2026-11-14**: every command refuses at load, on root's fixpoint, a program two of whose hypothesis declarations contradict one another, one line per conflict, and notes a list that filters nothing or a name another declaration says (`defined_behaviour.md` § 4.27); *forbid*'s first remedy names the hrules where the program declares one; the corpus's 24 programs in conflict corrected or, one, the refusal's fixture; E-47 closed, E-51 and E-52 filed, Q-M1f.242 to .246 defaulted. Planned 2026-11-13 (the user's ruling on E-47: *"report conflict if any hypothesis mechanism constradicts itself"*) |
| [S1f.6.35](s1f.6.35_imports_by_full_name.md) | Imports by full name: one copy of a module, short names local to a file; an engine stage and a language change | S1f.6.30's ruling, S1f.6.34 | planned 2026-11-13 (the user's model (h), [`import_clashes.md` § 7](import_clashes.md#7-the-ruling-2026-11-13-h-full-names-and-short-names)); **T1 done 2026-11-14, stopped there for the coordinator**: on a prototype, (h) read literally moves R on 22 tracked programs and refuses the acceptance's own; read with a declared relation and a closing rule ([Q-M1f.247](../open_questions.md#q-m1f247--what-the-closure-ii-follows-a-relation-of-any-kind-or-a-declared-one), [.248](../open_questions.md#q-m1f248--a-closing-rule-under-the-closure-ii)) on 4, each by the ruling ([.249](../open_questions.md#q-m1f249--the-answers-h-moves-under-the-recommended-reading)); E-53 filed; no source moved. **The user ruled .247 and .248 the same day** (any relation, a closing rule concludes: the reading V2) and asked to see the moves first: five programs shown, two of them claims (`typing/03`, the E-20 witness), and E-53's fix to `std.algebra` tried three ways, F1 recommended; **stopped again there**, T2 not started. **The user ruled the same day that every import brings its module whole** (*"in all cases we import everything, the difference only in names"*), and the stage ran to its end: **done 2026-11-14**, the resolver rewritten, every program and module listing the names it writes, R equal before and after on 512 of 516 loading programs (the four moves shown and accepted, Q-M1f.249), import invariance a gate test, W93 filed and fixed (E-53, F1), E-20, E-41, E-49 and E-50 closed, E-54 to E-56 filed; defaults Q-M1f.250 to .253 |
| [S1f.6.36](s1f.6.36_no_search.md) | Judging root without a search: `(solve … :no-search)` and `--no-search`; an engine stage | S1f.6.34, S1f.6.35 | **done 2026-11-14**: `(solve :no-search)`, a flag, states the lattice is root alone (nothing proposed, R {∅} or empty; a note beside an hrule or a list, silent beside an obligation or a closure, no S1f.6.34 conflict relaxed), and `--no-search` on `ein solve` and `ein test` declines to discover R, reporting root as the full run reaches it and no R (`lever_invariance.rs`, every corpus program); `ein oracle` refuses it; the 21 of S1f.6.34's 24 that meant root alone say it, every workaround gone, the mutation survivors the banked set; 506 digests moved over exactly those 21; defaults Q-M1f.254 to .260 (T1 shown and accepted the same day), E-57 filed. Planned 2026-11-14 (the user: *"some explicit flag in (solve ) ? instead of abusing bugs; or cmdline flag that disable search; or both"*, and *"flag that disables search doesn't change R, it disables discovering of R"*) |

No stage carries an estimate (the user's ruling of 2026-11-08). The numbers 18 and 19 are unassigned.

## The order

1. **S1f.6.11, the import fix, first.** The tables are plain facts imported flat, so nothing of the theory can be
   written as ruled before a flat import brings facts. **Done 2026-11-11.**
2. **S1f.6.2, the tables,** as the stdlib's first library of facts. **Done 2026-11-11.**
3. **S1f.6.3 to S1f.6.6, the four operations and comparison, in order.** All four **done 2026-11-11**;
   [what the next stages add where](s1f.6.3_addition.md#for-the-stages-that-follow) is its record's last section. S1f.6.3 writes the machinery the others
   call, and the test of the rule of the theory that each later stage extends to its own rules.
4. **S1f.6.12's investigation** may run at any time, on the prototype, since its first task stops for the user;
   its build follows S1f.6.6. **The investigation is done (2026-11-11) and stopped for the user.**
5. **S1f.6.13, `std.num9`,** after S1f.6.6, since its conversions name `std.arith`'s numbers. It touches no file
   of the nested theory. **Done 2026-11-11.**
6. **S1f.6.20, the word problems,** any time after S1f.6.6. It is the first program that uses the calculator
   among other relations, and where the need for nested expressions shows or does not.
7. **S1f.6.14, expressions,** only on the user's ruling, after S1f.6.20's examples.
8. **S1f.6.7 and S1f.6.8, the two extensions.** Each is a module over the natural-number theory and does not
   edit it. Negative numbers come first, because a rational's numerator is one.
9. **S1f.6.9, what is left of the width.** It may run beside the extensions.
10. **S1f.6.10, the investigation, last.** **Done 2026-11-12.**
11. **The stages of 2026-11-12,** in the order of [`replies_2026-11-12.md` § The order](replies_2026-11-12.md#the-order): those independent of the stdlib layout first, then the layout move, then what is born under the new names.
12. **S1f.6.34, the conflicts between the hypothesis mechanisms** (2026-11-13), independent of the layout: after
    S1f.6.30, the last of the stages of 2026-11-12 that are, and before S1f.6.31's move.
13. **S1f.6.35, imports by full name** (2026-11-13), after S1f.6.34 and **before S1f.6.31's move** (the user:
    *"6. before"*), so the move renames full names once.
14. **S1f.6.36, `:no-search` and `--no-search`** (2026-11-14), after S1f.6.35 and before S1f.6.31: it revisits
    S1f.6.34's corrected fixtures.

## Findings stay in the phase

What a stage finds in the engine or the language outside its scope is a row of [`findings.md`](findings.md),
not of F25 (the user's ruling of 2026-11-11: F25 was for P1f.7 alone). Each row is shown to the user and is
fixed, ruled or given an owner before the phase closes. After the re-plan each of E-1 to E-7 names the stage
that takes it, or says *open, shown, no owner yet*.

## Acceptance

- **Importable.** A program imports `std.arith` and computes the four operations and comparison on nested
  numbers of any length written `(n (n z 1) 2)`. Subtraction and division each give two numbers. A program
  imports `std.num9` and computes the same on `(num9 0 0 0 1 2 3 0 7 8)`.
- **A library of facts is importable flat,** `std.digits` being the first, and the register's rows for the old
  behaviour are fixed.
- **Search-free, held by a test.** On every arithmetic program in the corpus:
  - H has no candidate, k_s is 1 and R is {∅}, and no fork happens;
  - the answer is root's fixpoint, and it is the same under `ein solve`, `ein saturate` and `ein test`;
  - the theory writes no `not` premise, and exactly one `absent`, over `let`, which no rule derives (S1f.6.12;
    none until then).
- **Right, held by a test.** Every table row, and every answer on a sample of operands of every length up to a
  stated bound, agrees with the harness's integers; operands with high zeros are read by value; every answer is
  in normal form. `std.num9` is covered exhaustively at a width small enough to exhaust.
- **The rule of the theory, held by a test.** No rule of `std.arith` joins two facts on a number, checked on the
  rules, and a batch of requests costs in proportion to its count, by a deterministic measure of the matcher's
  work and not by the clock, held by a test run by hand (`cargo test -p ein-infer --features counters --test
  arith_batch`; out of the gate since [S1f.6.21](s1f.6.21_the_gate_reduced.md), the user's reply 46 (b)).
- **Division by zero derives its flag and then `(false)`,** each by its own rule, and never a number. Overflow
  exists in `std.num9` only and is reported there, never as a truncated number.
- **Malformed numbers** are flagged where the user ruled after S1f.6.12's investigation, and named on the page
  where they cannot be. **Built 2026-11-13:** `(malformed REQUEST)`, `(malformed (let NAME NUMBER))` and
  `(undeclared NAME REQUEST)`, each `(false)` by a rule of its own; what stays silent is listed in
  `stdlib/README.md` and held by a test.
- **The stdlib's own gates hold.** Every rule of each module is fired by a program under `tests/stdlib/`
  (`stdlib_coverage.rs`), every program there states an `:expect`, `utils/stdlib_mutants.py` covers the new
  modules, and each generator's `--check` is in the gate.
- **The extensions.** Negative and rational numbers each have a module with the same four operations, the same
  tests, and a normal form.
- **The expression layer** is built only if the user rules it in.
- **The investigation** is a page whose every claim carries the command that re-takes it. Any wrong answer it
  finds in the engine is a register row before anything fixes it.
- **Every relation and rule carries a `:why`.** S1f.5.14 makes it required.
- **`findings.md` has no row without a ruling, a fix or an owner.**

## Risks, each with what checks it

- **One ordinary join and a batch is quadratic again.** The rule of the theory is a discipline an author can
  break without any answer changing. S1f.6.3's test is what fails, and every later stage extends it to its own
  rules before it is done.
- **A circuit is a sequence.** With a continuation on every call, depth equals firings: 220 for the prototype's
  division, where round 2's theory had 82. A trace of a division is a long list of steps; each stage records its
  circuit's size and depth, and S1f.6.9 states where a number is too long to be practical.
- **A core names table rows.** With the tables as plain facts, a refutation that passed through arithmetic lists
  the rows it used beside the request (`design.md` § G). The ruled `(false)` on division by zero reads no table,
  so its core is the request alone; S1f.6.2 records one core that does name rows, and making cores canonical is
  F22's.
- **H is not empty by default.** A relation a rule derives is a hypothesis candidate unless something closes
  it. Each module closes its own relations, and the search-free test holds it. A program's own unary or binary
  relation beside the theory is the program's to close (the user's ruling on round 1's finding 2).
- **The import fix moves an answer nobody expected.** Round 2's scan found no tracked program importing a module
  that holds a fact, and did not scan the Rust test suites' inline programs. S1f.6.11's census does, and its
  sweep compares every tracked program before and after. **It did not** (2026-11-11): no inline module imported flat
  holds a fact, and the sweep moved the four witnesses alone.
- **Detection of a malformed number costs every well-formed request.** S1f.6.12's investigation measures that
  before anything is built, and the user rules on its table.
- **The nesting limit.** The parser nests to 1 000, so a request holds at most 999 digits, a rule can derive
  more, and such a state file does not load back (E-4). S1f.6.9 puts the choice to the user. **Re-taken
  2026-11-12:** a state is deeper than its request, so the state file fails from 999-digit operands already, and
  the page says so; Q-M1f.185 is the choice.
- **The names of the theory clash with a program's.** `n`, `z` and the result relations mean what the theory
  says once it is imported flat. S1f.6.12 establishes what a clash does today and what the module's page and
  the loader should say.
