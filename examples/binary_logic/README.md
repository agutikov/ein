# Binary logic

The sixteen functions of two inputs as a table of facts, what is known of them, a circuit as a program, and
six things to do with one. M1f P1f.6, stage
[S1f.6.17](../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.17_binary_logic.md).

These are **examples, not a stdlib module**. The theory modules live in this directory and the programs
import them file-relative. The plan page,
[`binary_logic.md`](../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/binary_logic.md), has the
prototype these were banked from, every read-out, and what a search costs as the inputs grow.

## The modules

| file | what it is |
|---|---|
| [`gates.ein`](gates.ein) | **the table**: `(gate2 NAME A B OUT)`, 64 rows, all sixteen functions: `Zero And Andn Fst Randn Snd Xor Or Nor Xnor Nsnd Orn Nfst Rorn Nand One`; each one's second name, `(column-name Xor f0110)`; and `inv`, `le`, `bit`. Generated |
| [`props.ein`](props.ein) | **what is known of the sixteen**, as facts: thirty relations, among them `swap-of`, `complement-of`, `dual-of`, `commutative`, `associative`, `identity-of`, `absorbing-of`, Post's five classes from inside and from outside, and `complete-alone`. Generated |
| [`props_rules.ein`](props_rules.ein) | the 31 monotone rules that derive every fact of `props.ein` from the 64 rows. Generated (the generator holds the text) |
| [`circuit.ein`](circuit.ein) | a circuit: `(gate INSTANCE FUNCTION IN1 IN2 OUT)`, `(val WIRE BIT)`, and one rule that propagates |
| [`inputs.ein`](inputs.ein) | inputs as hypotheses: `(in WIRE BIT)`, owed and functional, so an input has exactly one value |
| [`require.ein`](require.ein) | `(require WIRE BIT)`: a state where the wire carries the other bit is a contradiction |
| [`full_adder.ein`](full_adder.ein) | the binary full adder from five gates, as facts |
| [`bits.ein`](bits.ein) | a copy of [`../binary_arithmetic/bits.ein`](../binary_arithmetic/bits.ein), for the table `fa2`: an import cannot reach a sibling directory. Written by that directory's generator |

## The programs

| file | what it shows | answer |
|---|---|---|
| [`props_check.ein`](props_check.ein) | the check of `props.ein`: the rules derive exactly the generated facts. Generated | 30 claims, all held; search-free |
| [`simulation.ein`](simulation.ein) | **1, simulation**: the adder at a = 1, b = 1, carry in = 0 | sum 0, carry 1; search-free |
| [`truth_table.ein`](truth_table.ein) | **2, the truth table**, inputs as hypotheses | R is the 8 assignments, and the query's 8 classes are the rows; 30 saturations |
| [`adder_against_table.ein`](adder_against_table.ein) | the truth table from written test vectors, and the gate-built adder against the calculator's table `fa2` | 8 of 8 agree, none differs; search-free |
| [`satisfiable.ein`](satisfiable.ein) | **3, satisfiability**: sum 1 and carry 1? | one state, a = b = carry in = 1 |
| [`unsatisfiable.ein`](unsatisfiable.ein) | the same and not both a and b | R is empty, by owing: the claim is the owing frontier, eleven nodes |
| [`equivalent.ein`](equivalent.ein) | **4, equivalence**: the adder's carry against the majority form, by a miter | R is empty, by owing: no input tells them apart |
| [`not_equivalent.ein`](not_equivalent.ein) | the carry against a or b | two states, the two inputs where they differ |
| [`knights.ein`](knights.ein) | **5, knights and knaves**: A says B is a knave, B says A and C are of a kind; what is C? | two states, one projection class: C is a knave |
| [`synthesis.ein`](synthesis.ein) | **6, synthesis**: which function fits the adder's missing last gate? | two states, `Or` and `Xor`; 16 candidates, 14 dead alone |
| [`simplify.ein`](simplify.ein) | three laws read from the property facts | *x and 1* is *x*, *y or 1* is 1, the two `Xor` gates are one |
| [`post.ein`](post.ein) | Post's criterion on seven sets of functions | five complete, two inside a class |
| [`gen_gates.py`](gen_gates.py) | writes `gates.ein`, `props.ein`, `props_rules.ein` and `props_check.ein` | `--check` exits 1 if one differs |

Every claim of the hand-kept programs was computed in Python by enumeration, with no table of Ein's read,
and is checked by `ein test`.

## Running one

```sh
ein solve examples/binary_logic/truth_table.ein
ein solve --single examples/binary_logic/satisfiable.ein
ein test  examples/binary_logic
ein solve examples/binary_logic/unsatisfiable.ein --dump-answer /tmp/unsat   # nogoods.json has the reasons
```

## Which variants, and why

- **One table, not a relation a function.** The page recommends `(gate2 NAME A B OUT)`: a function is then a
  value, so a rule about any function needs no parameter, and a search can make the function a hypothesis,
  as `synthesis.ein` does. The other layout, sixteen relations of four rows, is on the page.
- **The names** are the stage file's proposal, on the user's ruling of 2026-11-11 that any spelling other
  than Ein's own `and`, `or` and `not` will do.
- **The properties as generated facts, the rules as their check.** The page recommends it: facts cost nothing
  when a program loads, and the rules that derive each from the rows can never disagree with the rows.
  `props_check.ein` holds the two equal, as a corpus program with a `test` cell, so that check is in the
  gate. The `forall` and `absent` restatements of eight properties are on the page and not here; `post.ein`
  uses `forall` where it is needed, over a set's members.
- **Inputs as two candidates each, by an obligation.** The page tried three ways to propose the candidates
  (the obligations rung alone, an hrule, `:hypothesis-relations`), which give one answer at one cost, and
  recommends none over the others; the simplest is here, the obligations rung alone (a default,
  [Q-M1f.190](../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f190--which-variants-of-the-binary-prototypes-are-banked-where-the-page-recommends-none)). One candidate an input
  with `absent` is refused or wrong, and is on the page only.
- **The six applications**, each as a program, with the full adder against the table and the simplifier.

**The page's questions stay open.** Banking the recommended variants is the user's instruction for the
examples (2026-11-12, *"bank binary arithmetic and logic prototypes into examples too"*) and not a ruling on
the layout, on which property facts ship and how, on the one-sided facts, on how an unsatisfiable search
should read, or on what ships from the stage.

## What to know before reading a read-out

- **An unsatisfiable search reads as owing.** In `unsatisfiable.ein` and `equivalent.ein` every total
  assignment is dead, the partial ones are alive and owe an input, and the read-out says *empty: every alive
  node owes*; `ein solve --single` says `Open`. The claims in those two files are the ones that hold: the
  owing frontier, and `(false)` for the query. It is E-23 of the phase's
  [`findings.md`](../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/findings.md).
- **The sizes are small on purpose.** A node of the search is a partial assignment, so describing R over n
  inputs is 3^n + n saturations: 30 at three inputs, and the default budget stops it near twelve. Every
  corpus program is solved, dumped and reloaded on each commit. The page has the measurements to 24 inputs,
  Post's criterion over all 65 535 sets and the three-input classes; those stay with the prototype under
  `build/s1f.6.17/`.

## Regenerating

```sh
python3 examples/binary_logic/gen_gates.py --check   # exit 1 if a generated file is stale
python3 examples/binary_logic/gen_gates.py           # write them again
```

The generator is run by hand and is not in the gate.
