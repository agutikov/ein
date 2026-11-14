# S1f.6.38 — A number is well-formed from the start: the check is its representation's

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-14, on the user's ruling of that day. Not started. **T1 is a design shown to the user
before anything is built** (the user's standing rule: real programs with computed answers first).
**Depends on:** [S1f.6.12](s1f.6.12_malformed_numbers.md)'s build, whose checks it moves;
[S1f.6.31](s1f.6.31_the_stdlib_layout.md)'s layout. **Runs before [S1f.6.16](s1f.6.16_binary_arithmetic.md)'s
build**, whose design ([§ The design](s1f.6.16_binary_arithmetic.md)) put the parameterised checks in
`std.math.positional`, the shared arithmetic, which this ruling says they are not: the build follows T1's design
once the user has seen it.

## The ruling

[Q-M1f.210](../open_questions.md#q-m1f210--stdint-and-stdrat-and-the-flags) asked what `std.math.int` and
`std.math.rat` do with a malformed request (E-38): (a) silent, as today; (b) three rules a module from its tags to
`(bad-request REQUEST FAULT)`, flagging an undeclared name and a digit as an operand; (c) (b), and each part of an
integer and a rational handed to the decimal check. The user, 2026-11-14:

> Now Q-M1f.210: yes (c), but have to notice that malformed decimal numbers is a problem of decimal module,
> malformed binary numbers - of binary module, not arithmetic, integer or rationals or whatever else. If there is
> a malformed number, I expect it has to be detected right away, without need to start calculations with it.
> Thing about it like written numbers on paper, if decimal number contains letters, signs, whitespaces and
> hieroglyphs - it is not a decimal number from the beginning. It is a string, cypher, whatever else but not a
> decimal number.

Read as three requirements:

1. **Whose check.** Whether `(n (n z 1) x)` is a decimal number is the decimal representation's question, and a
   binary numeral's is binary's. Not the shared arithmetic's (`std.math.positional`), not `std.math.int`'s or
   `std.math.rat`'s, nor any module that only carries numbers inside its own terms.
2. **When.** As soon as the number is written, by what it is, **without a calculation started with it**: not a
   pass at a request's operand (S1f.6.12's placement, `check` handed from `operand-number`), not a walk that finds
   it on the way.
3. **Where it is written does not matter.** A numeral written as a request's operand, inside an integer, inside a
   rational, as a `let`'s value, is the same numeral and is checked the same way, once.

What (c) keeps of its own: the faults that are **not** of a numeral, which S1f.6.12's table mixes in, belong to
the module that reads the operand, and int and rat flag them as decimal does (the (b) part): a name no `let`
states, a digit where a number goes, a `let` that cannot be read.

## Today

S1f.6.12's table ([§ What the theory flags now](s1f.6.12_malformed_numbers.md#what-the-theory-flags-now)):
`operand-number` and `operand-name` hand each number they find to `(check TAG NUMBER)`, `check-step` walks it a
place a firing, a fault is `(bad-operand TAG FAULT)`, ten rules by operation lift it to `(bad-request REQUEST
FAULT)`, and the flags `(malformed REQUEST)` / `(undeclared NAME REQUEST)` derive `(false)`. So the check runs
only where a request asks for an operand; int and rat reach `operand` for a named operand and nothing more (E-38);
a malformed part of an integer or a rational is never seen.

## Tasks

1. **T1f.6.38.1 — the design, on real programs, shown before anything is built; the stage stops after it.** On a
   copy of the stdlib under `build/s1f.6.38/` and a release binary of its own:
   - **Sort every fault** of S1f.6.12's table into *a numeral's* (a digit's place holding no digit, a number
     ending in a digit or a name where `z` goes, …) and *an operand's or a program's* (a name no `let` states, a
     digit where a number goes, a `let` stated twice or of `z` or a digit), and say whose module each is under
     the ruling.
   - **How a representation module sees every numeral a program writes, right away,** whatever term it sits in.
     What Ein has, each tried with a real program and its computed answer, its cost against S1f.6.12's pass,
     and what a trace, a core and `refuted by` read: rules whose premises have a variable head over what the
     program writes (a walk of written terms, not of requests); a check over root's written facts only, numbers
     a rule derives being well-formed by construction; and, if neither can, the engine change it would take (a
     declared constructor's argument positions checked at load, say), described and not built. Say which is
     *right away* in the user's sense and which only looks like it.
   - **The layout it gives,** module by module, with S1f.6.16's design beside it: where the decimal numeral's
     check lives (`std.math.digits`, which is facts today, a new `std.math.decimal`, or `decimal-arith` as
     decimal's combining module), where binary's, what leaves `std.math.positional`, what `std.math.int` and
     `std.math.rat` keep (the (b) part only), and what `std.math.num9` does (silent by reply 9).
   - **What a program sees:** the flag a malformed numeral derives (is it still `(malformed REQUEST)`, or a flag
     on the numeral itself, and `(false)` either way), on the probes of S1f.6.12 and E-38's 26
     (`build/s1f.6.12/build/t9/`), written as integers and rationals too; and that a well-formed program's
     answers and firings are what they were, or by how much they move.
   Recorded as the record's first section, every open choice with a proposed default (filed as Q-M1f rows,
   defaulted, the user's to revise); committed, records only; **then the stage stops for the user.**
2. **T1f.6.38.2 onward — the build, after the user has seen T1:** written then, with S1f.6.16's build, which it
   shapes. Its acceptance will hold at least: a malformed numeral is flagged and `(false)` wherever it is written,
   with no request reading it; int and rat flag an undeclared name and a digit as an operand (E-38 closed); no
   module but the representation's names a numeral's fault; no well-formed program's answer moves.

## What it must not do

- **Not leave a numeral's check in the arithmetic** (`positional`, the operations), in int or in rat.
- **Not make a malformed program a load error** without the user's word: today it loads, is dead and claims
  `(solve :expect (false))` (AGENTS.md § Edits that go together); a load refusal is a proposal in T1, not a
  default.
- **Not build before the user has seen T1.**

## The standing rules

A wrong answer met on the way is a register row before its fix. Findings go to [`findings.md`](findings.md),
questions to [`open_questions.md`](../open_questions.md), each under the next free id. The gate runs once over the
batch (the user's cadence of 2026-11-13).
