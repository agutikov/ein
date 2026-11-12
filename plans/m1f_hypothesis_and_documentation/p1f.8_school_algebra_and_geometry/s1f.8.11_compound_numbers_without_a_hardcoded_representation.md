# S1f.8.11 — Compound numbers without a hardcoded representation

**Phase:** [P1f.8](README.md)
**Status:** planned 2026-11-12, on the user's reply of that day. Not started. **An investigation that stops
for the user** with real programs computed on a prototype; it changes no shipped module before a ruling.
**Depends on:** the calculator's extensions as shipped (`std.int`, `std.rat`), and binary arithmetic in the
stdlib ([S1f.6.16's build](../p1f.6_calculator/s1f.6.16_binary_arithmetic.md#the-build-as-ruled-2026-11-12)),
since a second representation of the natural numbers is what the question is asked about. Until that has
shipped, the banked `examples/binary_arithmetic/` theory can stand in for the prototype.

## The reply

Question 14 of the calculator's [`questions.md`](../p1f.6_calculator/questions.md#q14) asked for the binary
theory's names. The user, 2026-11-12, in full:

> a is ambiguous, add2 looks like add with 2 operands etc. Take (b), but ints and rats - different question.
> binary and decimal is numbers representation, integers, rationals, ... complex - are numbers types based on
> naturals of some representation. Add new stage into p1f.8 to investigate how to make compound numbers
> without hardcoded representation so can switch between binary and decimal, and maybe later use any new
> natural numbers there, or even not numbers...

The first sentence is ruled and built in the calculator's phase (`bin-` names). The rest is this stage, and
its distinction is the user's: **a representation** (binary, decimal) is how a natural number is written;
**a number type** (integer, rational, later complex) is built *from* naturals, of whatever representation.

**The same question, asked again in round 2.** On the trial of rules shared between the bases
([round 2, item 16](../p1f.6_calculator/questions_round2.md#q16)) the user replied, as it stood in the working copy when this plan was written (2026-11-12, not yet committed):

> So we potentially can import std.math.rat and std.math.binary-arith and then connect them with single fact?

That is this stage's target stated as a program: two imports and one fact. The first direction below,
parameters, is the one under which it would be literally so, the fact being the activator.

## What is hardcoded today

`std.int` is `(int P N)` with P and N numbers of `std.arith`, and its 36 rules each start one of
`std.arith`'s walks under a tag: `add-walk`, `mul-call`, `cmp-walk`, the subtraction walk. `std.rat` is
`(rat Z D)` over `std.int` and `std.arith`'s division. So the compound types name the decimal theory's
relations in every rule. An integer over binary naturals would today be a second copy of `std.int` with
other names in it, which is the copy the user's reply to question 16 did not want for the bases themselves
(*"extract common rules into separate file the both import"*).

## The question

How a compound type says *a natural number, and its operations* without saying which. The directions to
try, each on real programs, none chosen before the measurement:

- **Parameters.** A rule with parameters is compiled once per activator fact, the mechanism the calculator's
  phase tries first for the bases' shared rules (the trial of S1f.6.16's build). `std.int`'s rules would
  take the underlying theory's call relations as parameters, and a program would activate *integers over
  decimal* or *integers over binary*. The trial's result bears directly on this: if a parameter cannot
  stand where a walk needs it, or costs firings, it fails here for the same reason.
- **An interface of calls.** The compound type calls a fixed set of tagged relations (*add*, *subtract both
  ways*, *multiply*, *divide*, *compare*), and each representation provides rules that answer them, with
  the representation read off the number's own head (`n` or `b2`). The calculator's tagged calls become an
  interface in [S1f.6.30](../p1f.6_calculator/s1f.6.30_name_clashes_between_imported_theories.md); this
  asks whether one interface can be answered by two theories at once, and what the dispatch costs.
- **Conversion at the boundary.** One compound theory over one representation, and numbers of another
  converted on the way in and out (`std.math.convert`). No genericity; a cost a request.
- **A type's declaration as data,** which is where it meets
  [M1g's types theory](../../m1g_logic/p1g.8_a_theory_of_types/README.md) (the user's reply to question
  12: *"types theory deserves it's own separate theory file in stdlib"*). This stage does not build that
  theory; it says what the compound numbers would need of it.

**"or even not numbers..."** The reply's last words. What the compound constructions need of the thing
underneath is small and can be stated: the pair `(int P N)` needs an addition, a comparison and a
subtraction that says which side was larger; the fraction needs a multiplication and a division with
remainder besides. Anything that provides those can stand underneath: polynomials (this phase's own
[S1f.8.3](s1f.8.3_simplification_and_identities.md) builds their normal form), or the integers themselves
under the rationals, as today. The stage writes that list down as the requirement and tries one
non-number, the smallest it can find in the phase, to see whether the list is right.

## Tasks

1. **What is hardcoded, counted:** for `std.int` and `std.rat`, every rule that names a relation of the
   natural-number theory, and which relation; the list of calls the two modules make in all.
2. **Each direction on a prototype** under `build/`, with the same targets: an integer sum and product and
   a rational sum over decimal naturals and over binary naturals, in one program and in two. Answers
   computed and held to Python; firings and depth beside the shipped modules'; the batch's cost by hand
   with the counters; what a trace and a core read as.
3. **Switching:** what a program writes to compute the same integers over the other representation, under
   each direction; and a program that mixes the two.
4. **One non-number underneath,** as far as it goes, with what broke.
5. **The page and the stop:** the directions side by side, a recommendation, what each would change in the
   shipped `std.math.int` and `std.math.rat` and in every program that imports them, and the questions.
   Complex numbers are named as the next compound type and not built.

## Acceptance

- Each direction has programs with computed answers, or a stated reason it could not be written.
- The requirement a compound type has of what is underneath is written as a list and tried on a non-number.
- The user has the page; nothing shipped moved.

## What it must not do

- **Not change `std.int`, `std.rat` or their names** before a ruling.
- **Not build a types theory** (M1g's) or promotion between number kinds (question 12: *"keep as is now"*).
- **Not add a representation.** Binary and decimal are the two; *"any new natural numbers"* is what the
  design must allow, not what the stage adds.

## The standing rules

A wrong answer of the engine met by a probe is a row of
[`corpus/wrong_answers.toml`](../../../corpus/wrong_answers.toml), filed `open` before its fix. Findings go
to this phase's [`findings.md`](findings.md); questions to [`open_questions.md`](../open_questions.md) under
the next free id. A design change shows real computed examples first, which is the whole of this stage.
