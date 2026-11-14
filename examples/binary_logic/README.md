# Binary logic

Six things to do with a circuit, each a program over the stdlib's `std.bool` modules. M1f P1f.6, stage
[S1f.6.17](../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.17_binary_logic.md).

The theory is in the stdlib since the stage's build (2026-11-14), as the user ruled it on 2026-11-12:
[`std.bool.gates`](../../stdlib/bool/gates.ein), the sixteen functions of two inputs as one table
`(gate2 NAME A B OUT)` and as a relation each `(And A B OUT)`;
[`std.bool.props`](../../stdlib/bool/props.ein), thirty relations known of them, derived by rules;
[`std.bool.circuit`](../../stdlib/bool/circuit.ein), a circuit as a program; and
[`std.bool.inputs`](../../stdlib/bool/inputs.ein), inputs as hypotheses that each owe one value, and the
values a program requires. [`stdlib/README.md`](../../stdlib/README.md#stdbool-the-sixteen-functions-of-two-inputs-and-circuits-m1f-s1f617)
has a section on each. The plan page,
[`binary_logic.md`](../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/binary_logic.md), has the
prototype these were banked from, every read-out, and what a search costs as the inputs grow.

**A function's name is `std.bool.gates`'.** Each of the sixteen is declared there, as the relation of
the second layout, so a program that writes one, as a gate's function too, lists it:
`(import std.bool.gates :symbols (Xor And Or))`. A name it writes and does not list is its own, and a gate
of that function computes nothing ([`std.bool.gates`' header](../../stdlib/bool/gates.ein); E-62 of the
phase's [`findings.md`](../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/findings.md)).

## What is here

| file | what it shows | answer |
|---|---|---|
| [`full_adder.ein`](full_adder.ein) | the binary full adder from five gates, as facts of `std.bool.circuit`'s `gate`: a module the programs below import | alone, R {∅} |
| [`bits.ein`](bits.ein) | a copy of [`../binary_arithmetic/bits.ein`](../binary_arithmetic/bits.ein), for the table `fa2`: an import cannot reach a sibling directory. Written by that directory's generator, and kept until `std.math.bits` exists ([S1f.6.16](../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.16_binary_arithmetic.md)'s build) | a module of facts |
| [`simulation.ein`](simulation.ein) | **1, simulation**: the adder at a = 1, b = 1, carry in = 0 | sum 0, carry 1; search-free |
| [`truth_table.ein`](truth_table.ein) | **2, the truth table**, inputs as hypotheses | R is the 8 assignments, and the query's 8 classes are the rows; 30 saturations |
| [`adder_against_table.ein`](adder_against_table.ein) | the truth table from written test vectors, and the gate-built adder against the calculator's table `fa2` | 8 of 8 agree, none differs; search-free |
| [`satisfiable.ein`](satisfiable.ein) | **3, satisfiability**: sum 1 and carry 1? | one state, a = b = carry in = 1 |
| [`unsatisfiable.ein`](unsatisfiable.ein) | the same and not both a and b | R is empty, by owing: the claim is the owing frontier, eleven nodes |
| [`equivalent.ein`](equivalent.ein) | **4, equivalence**: the adder's carry against the majority form, by a miter | R is empty, by owing: no input tells them apart |
| [`not_equivalent.ein`](not_equivalent.ein) | the carry against a or b | two states, the two inputs where they differ |
| [`knights.ein`](knights.ein) | **5, knights and knaves**: A says B is a knave, B says A and C are of a kind; what is C? | two states, one projection class: C is a knave |
| [`synthesis.ein`](synthesis.ein) | **6, synthesis**: which function fits the adder's missing last gate? | two states, `Or` and `Xor`; 16 candidates, 14 dead alone |
| [`simplify.ein`](simplify.ein) | three laws read from `std.bool.props`' facts | *x and 1* is *x*, *y or 1* is 1, the two `Xor` gates are one |
| [`post.ein`](post.ein) | Post's criterion on seven sets of functions, over `std.bool.props`' classes | five complete, two inside a class |

Every claim of these programs was computed in Python by enumeration, with no table of Ein's read, and is
checked by `ein test`. Each holds as it did before the build, R equal on every program but for the names'
prefixes, and each answers alike imported selectively, whole and by an alias
(`ein-cli/tests/import_invariance.rs`). What changed is where the theory is: a program that imports
`std.bool.circuit` holds the table and its sixteen layout rules, 65 firings at root, and one that imports
`std.bool.props` the property rules too, 405 more (`simplify.ein` and `post.ein`, which read generated facts
until the build).

## Running one

```sh
ein solve examples/binary_logic/truth_table.ein
ein solve --single examples/binary_logic/satisfiable.ein
ein test  examples/binary_logic
ein solve examples/binary_logic/unsatisfiable.ein --dump-answer /tmp/unsat   # nogoods.json has the reasons
```

## Which variants, and why

- **The table for a search, a relation a function for a rule that names one.** Both layouts are
  `std.bool.gates`' (reply 19 (c)). In the table a function is a value, so a rule about any function needs no
  parameter, and a search can make the function a hypothesis, as `synthesis.ein` does; every program here
  reads the table.
- **The names** are the stage file's proposal, which the user's reply 19 left standing; any spelling other
  than Ein's own `and`, `or` and `not` would do (the user, 2026-11-11).
- **The properties as rules** (reply 20 (b)): a program that reads them pays their firings, once, at root.
  The generated facts are the rules' check now, the claims of `tests/stdlib/bool/props/`.
- **Inputs as two candidates each, by an obligation** (`std.bool.inputs`): the obligations rung proposes
  them, the simplest of the three ways the plan page shows equal (a default,
  [Q-M1f.190](../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f190--which-variants-of-the-binary-prototypes-are-banked-where-the-page-recommends-none)).
  One candidate an input with `absent` is refused or wrong, and is on the page only.

## What to know before reading a read-out

- **An unsatisfiable search reads as owing.** In `unsatisfiable.ein` and `equivalent.ein` every total
  assignment is dead, the partial ones are alive and owe an input, and the read-out says *empty: every alive
  node owes*; `ein solve --single` says `Open`. The claims in those two files are the ones that hold: the
  owing frontier, and `(false)` for the query. It is E-23 of the phase's
  [`findings.md`](../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/findings.md), and question
  24 of the phase, not ruled; the build does not change it.
- **The sizes are small on purpose.** A node of the search is a partial assignment, so describing R over n
  inputs is 3^n + n saturations: 30 at three inputs, and the default budget stops it near twelve. That is a
  demonstration and not a SAT solver (the user's reply to round 2's item 25, *"yes, both"*); a cheaper search
  for *exactly one value per object* is M1g's
  [S1g.6.6](../../plans/m1g_logic/p1g.6_engine_requirements/s1g.6.6_exactly_one_value_per_object.md). Every
  corpus program is solved, dumped and reloaded on each commit.

## The full adder against the binary table

`adder_against_table.ein` is the stage file's acceptance line, *the gate-built full adder equals the
table*, as a corpus program with a claim. Its turn into a test against `std.math.bits`' table is
T1f.6.17.7, which waits on [S1f.6.16](../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.16_binary_arithmetic.md)'s
build: until then the program reads `bits.ein`, the copy here.
