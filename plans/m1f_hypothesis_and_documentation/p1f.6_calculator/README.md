# P1f.6 — A calculator in Ein: school arithmetic as a theory over digit atoms, computed by saturation alone

**Milestone:** [M1f](../README.md)
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
- **Monotone.** No `absent` and no `not` premise anywhere in the theory. A comparison is derived outright, from
  a borrow or from a table, so choosing a quotient digit needs no negation. One stratum.
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

| ID | title | depends on | status |
|---|---|---|---|
| [S1f.6.1](s1f.6.1_design_proposal.md) | The design proposal: representations, the protocol, and target programs computed on a prototype | S1f.5.5's swap | **done 2026-11-11**: [`design.md`](design.md), three rounds and the rulings |
| [S1f.6.11](s1f.6.11_importing_facts.md) | A flat import brings a module's facts: the engine fix, with its register rows, its census and its sweep | — | **done 2026-11-11**: W88 ×4 filed and fixed, the sweep moved the witnesses alone, Q-M1f.135 to .144 defaulted, E-8 found |
| [S1f.6.2](s1f.6.2_digits_and_tables.md) | The digits and the hardcoded tables, as a library of facts, `std.digits`, generated with a `--check` | S1f.6.11 | **done 2026-11-11**: 1 664 rows in seven tables, each held to Rust's integers; closed by written facts; Q-M1f.145 to .148 defaulted, E-9 and E-10 found |
| [S1f.6.3](s1f.6.3_addition.md) | Addition, and the machinery every circuit shares: the number, the protocol, named constants, accumulators and tags, and the test of the rule of the theory | S1f.6.2 | **done 2026-11-11**: `stdlib/arith.ein`, 19 rules; sums held to Rust's integers; search-free on the binary; no rule joins two facts on a number, on the text and on the matcher's count, a gate step; Q-M1f.149 to .154 defaulted, E-11 found |
| [S1f.6.4](s1f.6.4_subtraction.md) | Subtraction with its two numbers, and comparison | S1f.6.3 | **done 2026-11-11**: 18 rules more, 37; `(sub X Y)` answered by `(diff-of X Y D R)`, two walks side by side, and `(cmp X Y)` by `(cmp-of X Y lt\|eq\|gt)`; every operation a row of the harness against the integers, and measured in batches of its own; Q-M1f.155 to .158 defaulted, E-12 found |
| [S1f.6.5](s1f.6.5_multiplication.md) | Multiplication | S1f.6.3 | **done 2026-11-11**: 16 rules more, 53; `(mul X Y)` answered by `(product-of X Y P)`, the school method from the multiplier's top digit, tagged so a later theory can call it; 123 × 456 is 72 firings; the DOT sweep runs several files at a time; Q-M1f.159 to .162 defaulted |
| [S1f.6.6](s1f.6.6_division.md) | Division with remainder, the quotient digit by bisection; division by zero as a flag and `(false)` from it | S1f.6.4, S1f.6.5 | **done 2026-11-11**: 36 rules more, 89; `(div X Y)` answered by `(quot-of X Y Q R)`, 127645 ÷ 25 in 220 firings; a division by zero derives the flag and the flag `(false)`, every surface shown in the record; the two families of nine rules held alike by a test; Q-M1f.163 to .165 defaulted |
| [S1f.6.12](s1f.6.12_malformed_numbers.md) | Malformed numbers, and the names the theory occupies: an investigation first, then the detection | the prototype for its first task; S1f.6.6 for the build | **T1 done 2026-11-11, stopped for the user**: [`malformed_numbers.md`](malformed_numbers.md), on the shipped theory: a digit's place, a number's end and a twice-named constant can be flagged with no negation for twenty firings a request; a name no `let` states and a foreign fact cannot; a wrong count of arguments is the loader's, and a census finds nothing that loads refused; six questions; E-27 found |
| [S1f.6.13](s1f.6.13_num9.md) | Fixed-width numbers as a second module, `std.num9`, with overflow and conversions | S1f.6.2, S1f.6.6 | **done 2026-11-11**: `stdlib/num9.ein`, generated, 47 rules; a sum, a difference and a comparison in one firing, a product and a quotient in eleven; overflow a flag and never a number, a division by zero through `std.arith`'s rule; conversions each way; exhaustive against the integers at width 2; no rule joins two facts on a number, so 4 000 divisions take 1.24 s where the prototype took 8.1; Q-M1f.167 to .173 defaulted, **.173 for the user to confirm**: a wide rule takes up to 162 of the engine's 256 registers, and the tripwire on that bound now measures the module apart; E-28 found |
| [S1f.6.7](s1f.6.7_negative_numbers.md) | Extension: negative numbers as a positive and a negative part, simplified until one is zero | S1f.6.6 | planned |
| [S1f.6.8](s1f.6.8_rational_numbers.md) | Extension: rational numbers, an integer numerator over a natural denominator | S1f.6.7 | planned |
| [S1f.6.9](s1f.6.9_width.md) | What is left of the width: how long a number may be (the nesting limit, the size of a state) | S1f.6.6 | planned; one choice **for the user** |
| [S1f.6.20](s1f.6.20_word_problems.md) | An application: elementary-school word problems, the calculator imported among other relations; the words' meaning as rules, and a missing quantity detected and named | S1f.6.6 | planned 2026-11-11; its first task **stops for the user** |
| [S1f.6.14](s1f.6.14_expressions.md) | Optional: nested expressions, `(calc E)` and `(let NAME E)` | S1f.6.6, and the user's ruling | **waits for the user**; S1f.6.20 is where the need will show |
| [S1f.6.15](s1f.6.15_load_against_calculation.md) | Long numbers and many calculations: examples that set load time against calculation time (the user, 2026-11-11) | S1f.6.6 | **done 2026-11-11**: `long_numbers.ein`, `many_calculations.ein` and `factorial.ein`, generated, sized to what every per-commit surface affords; `utils/arith_cost.py` and its tables in `stdlib/README.md`: 20.6 ms of load a program, a `.einb` that buys none of it back, the parse ahead of the calculation past a thousand sums, and `ein solve` 30 to 90 times `ein saturate` on long numbers; Q-M1f.166 defaulted, E-24 to E-26 found |
| [S1f.6.16](s1f.6.16_binary_arithmetic.md) | Arithmetic with binary numbers: whether the rules get simpler, and whether the theory is generic in its base (the user, 2026-11-11) | S1f.6.6 | planned; T1 stops for the user |
| [S1f.6.17](s1f.6.17_binary_logic.md) | Binary logic: truth tables, gates, a circuit as a program, and six applications tried; the starting point of [M1g](../../m1g_logic/README.md) (the user, 2026-11-11) | S1f.6.16 | planned; T1 stops for the user |
| [S1f.6.10](s1f.6.10_beyond_arithmetic.md) | Investigation: what else the theory can do, from equations and puzzles to bounded theorem checking and number theory | S1f.6.8, S1f.6.9 | planned; a report |

No stage carries an estimate (the user's ruling of 2026-11-08). The numbers 15 to 19 are unassigned.

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
10. **S1f.6.10, the investigation, last.**

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
  - the theory writes no `absent` and no `not` premise.
- **Right, held by a test.** Every table row, and every answer on a sample of operands of every length up to a
  stated bound, agrees with the harness's integers; operands with high zeros are read by value; every answer is
  in normal form. `std.num9` is covered exhaustively at a width small enough to exhaust.
- **The rule of the theory, held by a test.** No rule of `std.arith` joins two facts on a number, checked on the
  rules, and a batch of requests costs in proportion to its count, by a deterministic measure of the matcher's
  work and not by the clock.
- **Division by zero derives its flag and then `(false)`,** each by its own rule, and never a number. Overflow
  exists in `std.num9` only and is reported there, never as a truncated number.
- **Malformed numbers** are flagged where the user ruled after S1f.6.12's investigation, and named on the page
  where they cannot be.
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
  more, and such a state file does not load back (E-4). S1f.6.9 puts the choice to the user.
- **The names of the theory clash with a program's.** `n`, `z` and the result relations mean what the theory
  says once it is imported flat. S1f.6.12 establishes what a clash does today and what the module's page and
  the loader should say.
