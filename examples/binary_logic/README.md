# Binary logic

Six things to do with a circuit, each a program over the stdlib's `std.bool` modules
([Q-M1f.328](../../docs/decisions/stdlib/bool/Q-M1f.328-boolean-functions-and-circuits-ship-as-four-std.md)):
[`std.bool.gates`](../../stdlib/bool/gates.ein), the sixteen functions of two inputs as one table
`(gate2 NAME A B OUT)` and as a relation each `(And A B OUT)`;
[`std.bool.props`](../../stdlib/bool/props.ein), thirty relations known of them, derived by rules;
[`std.bool.circuit`](../../stdlib/bool/circuit.ein), a circuit as a program; and
[`std.bool.inputs`](../../stdlib/bool/inputs.ein), inputs as hypotheses that each owe one value, and the
values a program requires. [`stdlib/README.md`](../../stdlib/README.md) has a section on each, and
[`04-stdlib/18-boolean-functions.md`](../../docs/ein/user/04-reference/04-stdlib/18-boolean-functions.md) is their reference.

**A function's name is `std.bool.gates`'.** Each of the sixteen is declared there, as the relation of
the second layout, so a program that writes one, as a gate's function too, lists it:
`(import std.bool.gates :symbols (Xor And Or))`. A name it writes and does not list is its own, and a gate
of that function computes nothing ([`std.bool.gates`' header](../../stdlib/bool/gates.ein);
[Q-M1f.263](../../docs/decisions/stdlib/bool/Q-M1f.263-every-program-that-imports-std-bool-gates.md)).

## What is here

| file | what it shows | answer |
|---|---|---|
| [`full_adder.ein`](full_adder.ein) | the binary full adder from five gates, as facts of `std.bool.circuit`'s `gate`: a module the programs below import | alone, R {∅} |
| [`simulation.ein`](simulation.ein) | **1, simulation**: the adder at a = 1, b = 1, carry in = 0 | sum 0, carry 1; search-free |
| [`truth_table.ein`](truth_table.ein) | **2, the truth table**, inputs as hypotheses | R is the 8 assignments, and the query's 8 classes are the rows |
| [`adder_against_table.ein`](adder_against_table.ein) | the truth table from written test vectors, and the gate-built adder against the calculator's table, `std.math.bits`' `bin-fa` | 8 of 8 agree, none differs; search-free |
| [`satisfiable.ein`](satisfiable.ein) | **3, satisfiability**: sum 1 and carry 1? | one state, a = b = carry in = 1 |
| [`unsatisfiable.ein`](unsatisfiable.ein) | the same and not both a and b | R is empty, by owing: the claim is the owing frontier |
| [`equivalent.ein`](equivalent.ein) | **4, equivalence**: the adder's carry against the majority form, by a miter | R is empty, by owing: no input tells them apart |
| [`not_equivalent.ein`](not_equivalent.ein) | the carry against a or b | two states, the two inputs where they differ |
| [`knights.ein`](knights.ein) | **5, knights and knaves**: A says B is a knave, B says A and C are of a kind; what is C? | two states, one projection class: C is a knave |
| [`synthesis.ein`](synthesis.ein) | **6, synthesis**: which function fits the adder's missing last gate? | two states, `Or` and `Xor`; 16 candidates, 14 dead alone |
| [`simplify.ein`](simplify.ein) | three laws read from `std.bool.props`' facts | *x and 1* is *x*, *y or 1* is 1, the two `Xor` gates are one |
| [`post.ein`](post.ein) | Post's criterion on seven sets of functions, over `std.bool.props`' classes | five complete, two inside a class |

Every claim of these programs was computed outside Ein, in Python, by enumeration, with no table of Ein's
read, and is checked by `ein test`. Each answers alike imported selectively, whole and by an alias
(`ein-cli/tests/import_invariance.rs`). A program that imports `std.bool.circuit` holds the table and its
sixteen layout rules at root, and one that imports `std.bool.props` the property rules too, which it pays
for once, at root.

## Running one

```sh
ein solve examples/binary_logic/truth_table.ein
ein solve --single examples/binary_logic/satisfiable.ein
ein test  examples/binary_logic
ein solve examples/binary_logic/unsatisfiable.ein --dump-answer unsat   # the answer as files
```

## Which variants, and why

- **The table for a search, a relation a function for a rule that names one.** Both layouts are `std.bool.gates`'
  ([Q-M1f.325](../../docs/decisions/stdlib/bool/Q-M1f.325-a-boolean-function-is-an-argument-in-one-table.md)). In the
  table a function is an argument, so a rule about any function needs no parameter, and a search can make the
  function a hypothesis, as `synthesis.ein` does; every program here reads the table.
- **The names** are any spelling other than Ein's own `and`, `or` and `not`.
- **The properties as rules**
  ([Q-M1f.326](../../docs/decisions/stdlib/bool/Q-M1f.326-the-properties-of-the-sixteen-functions-are.md)):
  a program that reads them pays their firings, once, at root. The claims of `tests/stdlib/bool/props/`, the
  properties as generated facts, are the rules' check.
- **Inputs as two candidates each, by an obligation** (`std.bool.inputs`): the obligations rung proposes
  them, the simplest of the forms whose answer is the same
  ([Q-M1f.190](../../docs/decisions/corpus/Q-M1f.190-each-circuit-example-takes-the-simplest-form.md)).
  One candidate an input with `absent` is refused or answers otherwise.

## What to know before reading a read-out

- **An unsatisfiable search reads as owing.** In `unsatisfiable.ein` and `equivalent.ein` every total
  assignment is dead, the partial ones are alive and owe an input, and the read-out says *empty: every alive
  node owes*; `ein solve --single` says `Open`. The claims in those two files are the ones that hold: the
  owing frontier, and `(false)` for the query.
- **The sizes are small on purpose.** A node of the search is a partial assignment, so describing R over n
  inputs is 3^n + n saturations, and the default budget bounds n. These are demonstrations, not a SAT solver.
  Every corpus program is solved, dumped and reloaded by the corpus tests.

## The full adder against the binary table

`adder_against_table.ein` holds *the gate-built full adder equals the table* as a corpus program with a
claim, and `ein-infer/tests/bool_theory.rs`' `the_full_adder_from_gates_is_the_bits_table` as a test: it
drives this directory's `full_adder.ein` on all eight inputs and holds its sum and carry to `std.math.bits`'
`bin-fa`, row for row, and both to Rust's sum. `synthesis.ein` reads `bin-fa` too.
