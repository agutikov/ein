# P1g.8 — A theory of types, and a smaller kind of number where a larger is expected

**Milestone:** [M1g](../README.md)
**Status:** planned 2026-11-12, on the user's reply of that day to the calculator's questions. Not started.
No stage file yet: the phase is this README, and its stages are sections below.
**Depends on:** [P1g.1](../p1g.1_survey_and_design/README.md)'s stop; of M1f, the calculator's number kinds
as shipped (`std.arith`, `std.int`, `std.rat`, under `std.math` after
[S1f.6.31](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.31_the_stdlib_layout.md)), and
[P1g.10's S1g.10.7](../p1g.10_school_algebra/s1g.10.7_compound_numbers_without_a_hardcoded_representation.md),
which says what the compound numbers would need of a types theory.

## Where it comes from

Question 12 of the calculator's
[`questions.md`](../../m1f_hypothesis_and_documentation/p1f.6_calculator/questions.md#q12): *Is a smaller
kind of number accepted where a larger is expected?* Today it is not: `(int-add (n z 5) X)`, a natural
number where an integer goes, is answered by silence, and so is an integer where a rational goes; 5 as a
rational is written `(rat (int (n z 5) z) (n z 1))`. The options were (a) no promotion, as built, and (b)
promote: a natural number read as `(int N z)`, an integer as a rational over 1, four rules more and two
firings an operand for the integers. The user, 2026-11-12:

> question about type system inside the theory; lets postpone it till M1g, types theory deserves it's own
> separate theory file in stdlib, and then we will try to use it to solve this; keep as is now

So three things are ruled: nothing changes in the calculator now; **a types theory is a stdlib module of
its own**; and promotion is tried *with it*, as its first use, not written as four ad hoc rules.

## What a types theory is here

A theory **written in Ein**, as every other module of the stdlib is: relations and rules, imported by a
program that wants it. It is not a type checker in the loader (the loader's arity check,
[S1f.6.23](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.23_the_loaders_arity_check.md),
holds a form to its relation's *count* of arguments and stops there), and it is not new syntax.

What exists to build on, which the first stage reads before proposing anything:

- **`is-a`** and the ontology layer: instances of types, the zebra programs' whole vocabulary, and
  `std.algebra`'s rules over typed relations.
- **A relation's declaration** names argument types, `(relation total Number)`, and the blind enumerator
  reads them when it builds H; where it binds a type as an object the read-out says so (`types as
  objects`).
- **Shapes.** A number's kind is today its *head*: `n` or `z`, `int`, `rat`, `b2`. A rule tells kinds apart
  by pattern, and `z` belongs to more than one (it is zero in both bases, the calculator's reply 15).
- **What no rule can say without negation:** *this is not of that shape* (the calculator's
  `malformed_numbers.md`). A types theory that derives `(has-type V T)` positively turns a *not* into a
  lookup of another fact, which is why it bears on malformed operands too.

## The stages

### S1g.8.1 — What a type is in Ein today, and the design, on programs

The inventory above, each item with a program and its output; the candidate shapes of the theory, each on
the same programs with computed answers: a relation `(has-type VALUE TYPE)` derived on request (the
calculator's discipline: nothing enumerates the values) from a value's shape; `(subtype T U)` as facts with
a transitive closure; a *coercion* as a rule from a value of one type to the value of another that denotes
the same thing, `(n z 5)` to `(int (n z 5) z)`. What each costs a request, in firings and on the matcher's
counters, since it sits in front of every operand. **Stops for the user,** with the name of the module and
its relations as questions ([Q-M1g.14](../open_questions.md#q-m1g14--the-types-theory-its-shape-and-its-name)).

### S1g.8.2 — The module

The types theory as ruled, in the stdlib, with what every stdlib module has: a program per rule under
`tests/stdlib/`, each with its `:expect`; the mutation census; a `:why` on every relation and rule; a
harness in Rust that holds its judgements on every value up to a bound. Monotone, search-free on a program
that only asks for types.

### S1g.8.3 — Its first use: a smaller kind of number where a larger is expected

Question 12, tried with the module: a natural number as an integer's operand, an integer as a rational's.
Built as a module over the number modules, which it does not edit, so that a program that does not import
it sees today's answers: *"keep as is now"* stays true of the calculator's core. Held to the same harnesses
as the number modules, with mixed operands added; the cost an operand reported beside option (b)'s
estimate of four rules and two firings. The page says what promotion does to the calculator's claims: *one
answer a request* (a promoted operand and its written-out twin are two requests, as a named operand and a
written one are today), and *a flag means malformed* (a natural number where an integer goes stops being
silent, and becomes well-formed).

### S1g.8.4 — What else it types

Only as far as M1g's own phases need: a formula against a term in P1g.4's many-sorted reading; a proof line
against a formula in P1g.3; S1g.10.7's compound numbers over a representation. Each a short section with a
program, or a sentence saying the types theory does not reach it.

## What it is not

- **Not a static type system for Ein,** and no check at load. If the stages find that a check at load is
  what is wanted, that is a question for the user and an engine stage of
  [P1g.6](../p1g.6_engine_requirements/README.md), with register rows first.
- **Not type theory as a foundation** (dependent types, propositions as types). The name is the user's
  *"types theory"* for a theory of the types Ein's programs already use; the Curry-Howard reading is named
  in [S1g.3.3](../p1g.3_proof_systems/s1g.3.3_natural_deduction.md)'s neighbourhood and not planned.
- **Not engine arithmetic,** and not a change to any number module's relations.

## Acceptance

- The design page was read and ruled on before the module was written.
- The module ships with the stdlib's gates held.
- A program that imports the promotion module computes with mixed operands, right against the harness; a
  program that does not is answered exactly as today (a sweep).
- The page states what the types theory costs an operand.

## The discipline

[P1g.6 § The discipline](../p1g.6_engine_requirements/README.md#the-discipline), where an engine change is
met; otherwise the stdlib's own gates, as in the [milestone's acceptance](../README.md#acceptance).
