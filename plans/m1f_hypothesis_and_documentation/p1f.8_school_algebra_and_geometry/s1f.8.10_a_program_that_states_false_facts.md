# S1f.8.10 — A program that states false facts: `2 + 2 = 5`

**Phase:** [P1f.8](README.md)
**Status:** planned 2026-11-12, on the user's reply of that day. Not started. **An investigation that ends in
a page shown to the user;** it builds nothing in the engine or the stdlib without a ruling.
**Depends on:** the calculator as shipped. It is independent of the phase's other stages and may run before
[S1f.8.1](s1f.8.1_survey_and_design.md)'s stop, since it writes nothing outside `plans/` and `build/` until
its page has been read.

## The reply

Question 7 of the calculator's [`questions.md`](../p1f.6_calculator/questions.md#q7) asked whether a module
may reserve its names, so that a program's written facts of them are refused. Its witness: a program that
writes `(done (sum (n z 2) (n z 2)) (n z 5))` gets `2 + 2 = 5`, *an answer nobody computed*. The user,
2026-11-12:

> (a), btw, `2 + 2 = 5` is interesting question - what happens to the program that states false facts? If it
> is not solved already by-product in existing theories - I expect it belongs to either p1f.8 or M1g

(a) there was *not in this phase*: the calculator's page lists the names and says only requests and `let`
are a program's to state. The *btw* is this stage.

## The question

A program states, as a fact, something a theory it imports would compute otherwise. What happens, what
should happen, and **is it already solved as a by-product of the existing theories**, as the reply asks?
The cases, each a different thing though all read *2 + 2 = 5*:

| the false statement | written as | what the plan expects today, to be computed and not assumed |
|---|---|---|
| a false **result**, with the request beside it | `(add 2 2)` and `(sum-of 2 2 5)` | two `sum-of` facts for one request, 4 derived and 5 written; no contradiction, since nothing says a sum is unique |
| a false result with **no** request | `(sum-of 2 2 5)` alone | one fact, never checked: nothing asks |
| a false **internal** fact | `(done (sum 2 2) 5)`, the witness of question 7 | the circuit takes it as its own hand-over, and the public answer is 5 |
| a false **table row** | `(fa 2 2 0 5 0)` beside `std.digits` | a second row for one column; where a table is closed, what closure says of a written extra row |
| a false statement in a **problem's own vocabulary** | *Bob has 5 apples* and *Bob has 7 apples* at one moment | the word theory's question, [S1f.6.20](../p1f.6_calculator/s1f.6.20_word_problems.md)'s `contra.ein` is its neighbour |
| a false **equation as data** | `(equation (Plus (num 2) (num 2)) (num 5))` | not a false fact at all: a statement the program *talks about*, which [S1f.8.4](s1f.8.4_linear_equations.md) answers *no solution* |
| an **identity** that is false | *prove (a+b)² = a² + b²* | [S1f.8.3](s1f.8.3_simplification_and_identities.md): two normal forms that differ |

The last two rows are why the question belongs here: algebra is where *false* stops being a property of a
fact and becomes an answer about a statement. The first four are about what a theory can say of facts written
past it.

## What would make it *solved as a by-product*

Three mechanisms exist, and the stage tries each on the first four rows before proposing anything:

- **`functional`,** `std.algebra`'s restriction: a relation declared functional in its last argument makes
  two results for one request `(false)`. Whether the calculator's result relations can carry it without
  breaking the rule of the theory (the restriction joins two `sum-of` facts on their operands, which are
  numbers) is measured on a batch.
- **A checking rule,** monotone: a written result that differs from the derived one in some digit, read by
  the tables, as the calculator's detection rules read a malformed digit
  ([S1f.6.12's build](../p1f.6_calculator/s1f.6.12_malformed_numbers.md#the-build-as-ruled-2026-11-12)).
  It can only fire where the request was made.
- **Provenance:** a written fact and a derived one differ in where they came from. What a written fact may
  claim is [F24](../../followups/f24_written_provenance.md)'s question (written provenance), and name
  reservation, question 7's option (b), is the loader's form of the same thing. The stage names what it
  would take and does not build it.

## Tasks

1. **The seven programs, computed:** each under `ein solve`, `ein saturate --dump` and `--trace`, with
   every surface; what is in R, whether root is alive, and what the read-out says of the false statement.
2. **The three mechanisms tried** on rows 1 to 4, on a copy of the stdlib under `build/`: what each catches,
   what each costs a well-formed batch, what each cannot see.
3. **The page,** with the answer to the reply's *if*: which rows are already answered by an existing theory,
   which by a mechanism that exists and is not applied, and which by nothing. A recommendation a row, and
   **where each belongs**: this phase (rows 6 and 7 are S1f.8.3's and S1f.8.4's subjects already), F24, or
   [M1g](../../m1g_logic/README.md) (a false axiom in a proof system, and *ex falso*, are P1g.3's).
4. **Shown to the user.** Anything built follows the ruling, as a task of the stage it belongs to.

## Acceptance

- Each row has a program and its computed answer.
- The page says, for each, whether an existing theory answers it, and names the owner of what is left.
- Nothing in the stdlib or the engine changed without a ruling.

## What it must not do

- **Not reserve names or refuse written facts in the loader** (the user's (a) to question 7).
- **Not add a uniqueness restriction to the calculator** before its cost on a batch is shown.

## The standing rules

A wrong answer of the engine met by a probe is a row of
[`corpus/wrong_answers.toml`](../../../corpus/wrong_answers.toml), filed `open` before its fix; note that a
false *written* fact answered as written is the program's and not the engine's. Findings go to this phase's
[`findings.md`](findings.md); questions to [`open_questions.md`](../open_questions.md) under the next free
id. Real computed examples come first.
