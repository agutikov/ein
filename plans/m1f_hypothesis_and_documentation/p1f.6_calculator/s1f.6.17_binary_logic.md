# S1f.6.17 — Binary logic: truth tables, gates, and what they are good for

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-11, on the user's instruction of that day. **T1 done 2026-11-11, as a prototype and a
page shown to the user: [`binary_logic.md`](binary_logic.md).** Nothing shipped; T2 and T3 wait for the user's
rulings on that page's questions. T1 re-counted this page's tables from the rows and corrected three hand
counts, each marked below. **The recommended variants banked as examples 2026-11-12** (`9612b350`), on the user's instruction
of that day: [`examples/binary_logic/`](../../../examples/binary_logic/README.md); the
[record](#record-2026-11-12-the-recommended-variants-banked-as-examples) is below. It is not T2: no stdlib module,
and the questions stay open.
**The modules are ruled and planned (2026-11-12):** the user's replies to the page's questions are in
[§ The build, as ruled](#the-build-as-ruled-2026-11-12), which replaces task 2 below. **Built 2026-11-14 but
T1f.6.17.7**, which waits on S1f.6.16's build: the four modules `std.bool.gates`, `std.bool.props`,
`std.bool.circuit` and `std.bool.inputs` under `stdlib/bool/`, their programs under `tests/stdlib/bool/`, the
applications over them, every claim as it was; the [record](#record-2026-11-14-the-build) is below.
**Depends on:** [S1f.6.16](s1f.6.16_binary_arithmetic.md), for the two digit names and the binary adder it
rebuilds from gates.

## The instruction

The user, 2026-11-11: *"binary logic, truth tables, can be there any applications for it?"*

## What it is

The calculator's tables are truth tables already: a full adder's eight rows are a function of three bits. This
stage goes one level down, to the gates the tables are made of, and asks what Ein does with them.

- **The gates as tables of facts:** `not`'s two rows, and the four rows each of the two-input functions. The
  names are the stage's to choose with care: `and`, `or`, `not`, `eq` and `false` are the engine's own words,
  and the value rule keeps `and` and `or` from ever heading a value, so a gate is not called `and`.
  **The user ruled the same day that any other spelling will do:** *"for "and" element, function can use
  _and, _and_, __and, AND, And, l_and, And2 - whatever"*. So the stage picks one convention for all the
  gates, shows it on the examples, and does not stop for it.
- **A circuit as a program:** wires are names, a gate instance is a fact, and one rule per gate type
  propagates values. Saturation is then circuit simulation, search-free, like the calculator.
- **The binary full adder built from gates,** and its eight outputs held equal to S1f.6.16's table: the
  calculator's own gate, derived instead of written.

## All sixteen functions of two inputs (the user, 2026-11-11)

> about naming (And, Or, Not, Imp, Iff): there are 16 different boolean functions of 2 arguments, completely
> defined by array of 4 bool result values indexed by input values; I want to have them all and in 2 forms -
> binary gates in plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.17_binary_logic.md (And, Nand,
> XNand - all 16 possible) and in preposition logic - conjunction, disjunction, implication, pierse arrow -
> also should be 16, if not - explain why.

**There are sixteen in both forms, and each has a name in both.** A function of two bits is its column of four
outputs, and 2⁴ = 16. Propositional logic has the same sixteen as binary connectives, since a connective *is*
its truth table; a textbook writes five or so only because the rest are definable from them (and all sixteen
from `Nand` alone, or from `Nor` alone).

| table (a b = 00 01 10 11) | gate | in propositional logic | proposed gate name | proposed connective |
|---|---|---|---|---|
| 0000 | constant 0 | contradiction, ⊥ | `Zero` | `Bot` |
| 0001 | AND | conjunction, a ∧ b | `And` | `And` |
| 0010 | a AND NOT b (inhibit) | nonimplication, a ↛ b | `Andn` | `Nimp` |
| 0011 | a (buffer on the first input) | projection on the first | `Fst` | `Fst` |
| 0100 | NOT a AND b | converse nonimplication, a ↚ b | `Nand`-like, proposed `Randn` | `Nrimp` |
| 0101 | b (buffer on the second input) | projection on the second | `Snd` | `Snd` |
| 0110 | XOR | exclusive disjunction, a ⊕ b (non-equivalence) | `Xor` | `Xor` |
| 0111 | OR | disjunction, a ∨ b | `Or` | `Or` |
| 1000 | NOR | joint denial, Peirce's arrow, a ↓ b | `Nor` | `Nor` |
| 1001 | XNOR | biconditional, a ↔ b (equivalence) | `Xnor` | `Iff` |
| 1010 | NOT b | negation of the second | `Nsnd` | `Nsnd` |
| 1011 | a OR NOT b | converse implication, a ← b | `Orn` | `Rimp` |
| 1100 | NOT a | negation of the first | `Nfst` | `Nfst` |
| 1101 | NOT a OR b | implication, a → b | proposed `Rorn` | `Imp` |
| 1110 | NAND | alternative denial, Sheffer's stroke, a ↑ b | `Nand` | `Nand` |
| 1111 | constant 1 | tautology, ⊤ | `One` | `Top` |

The names are the coordinator's proposal, on the user's *"whatever"*; the stage shows them on programs and
the user may change any.

- **`XNand` is not one of them under that name.** The gate usually meant is XNOR, 1001, equivalence. Read
  literally, *not (a xor-and b)* names no standard function.
- **Six of the sixteen do not depend on both inputs:** the two constants, the two projections and the two
  one-input negations. They are functions of two inputs all the same, and the module has them, so that *every
  column of four bits has a name* is true and a test holds it. The other ten are the ones that read both.
- **`Randn` and `Rorn` are `Andn` and `Orn` with the inputs swapped,** as `Rimp` is `Imp` swapped and `Nrimp`
  is `Nimp` swapped; `Fst`/`Snd` and `Nfst`/`Nsnd` are the same pairing. So the sixteen are **twelve** up to the
  order of the inputs, four pairs and eight that are their own swap, and a spelling that says *swapped* by one
  letter keeps the family readable. (This said *ten* until T1 counted it, 2026-11-11: ten is how many read
  both inputs, another count.)
- **One table, generated.** `(gate2 NAME A B OUT)`, 64 rows, written by the tables' generator with its
  `--check`, each row held to the function's four-bit column by a test; or a relation per function, which the
  stage compares on the prototype. A systematic second name, the column itself, `f0110` for `Xor`, costs
  nothing and makes *all sixteen* checkable by enumeration.
- **Of one input there are four** (identity, negation, the two constants), and of none, two.

## What is known about the sixteen, as facts (the user, 2026-11-11)

> Six of the sixteen don't read both inputs, The sixteen are ten up to swapping the inputs - I know, plan add
> symmetry relations and other useful information

So the module holds, beside the 64 rows, **what is true of each function and between functions**, as facts a
program can read and a rule can use. Each is listed here with how many functions or pairs it holds of, counted
by hand from the table and **to be re-counted by the stage's test**, which derives every one from the rows
and does not trust this page.

**Relations between two functions:**

| fact | meaning | holds of |
|---|---|---|
| `(swap-of F G)` | G is F with its inputs exchanged: G(a, b) = F(b, a) | **four** pairs: `Andn`/`Randn`, `Fst`/`Snd`, `Nfst`/`Nsnd`, `Orn`/`Rorn` (the first and the last are the connectives `Nimp`/`Nrimp` and `Rimp`/`Imp`); the other eight are their own swap. Sixteen facts. (*six pairs* until T1's count, which had listed two of the four twice, under their connective names) |
| `(complement-of F G)` | G is F with its output negated | eight pairs: `Zero`/`One`, `And`/`Nand`, `Or`/`Nor`, `Xor`/`Xnor`, `Fst`/`Nfst`, `Snd`/`Nsnd`, `Andn`/`Rorn`, `Randn`/`Orn` |
| `(dual-of F G)` | De Morgan's dual: G(a, b) = not F(not a, not b) | `And`/`Or`, `Nand`/`Nor`, `Xor`/`Xnor`, `Zero`/`One`, **`Andn`/`Orn`, `Randn`/`Rorn`**; the four that are their own dual are below. (The last two pairs read `Andn`/`Rorn`, `Randn`/`Orn` until T1's count: those are the complements, the row above. The dual of *a and not b* is *a or not b*) |
| `(left-negated F G)`, `(right-negated F G)` | G is F with one input negated: the family a gate with an inverted input belongs to | every function, twice |

**Properties of one function:**

| fact | meaning | holds of |
|---|---|---|
| `(commutative F)` | F(a, b) = F(b, a): its own swap | eight: `Zero And Xor Or Nor Xnor Nand One` |
| `(associative F)` | F(F(a, b), c) = F(a, F(b, c)) | eight: `Zero And Xor Or Xnor One Fst Snd` |
| `(idempotent F)` | F(a, a) = a | four: `And Or Fst Snd` |
| `(reads-first F)`, `(reads-second F)` | the output depends on that input | ten read both; `Fst Nfst` the first only, `Snd Nsnd` the second only, `Zero One` neither |
| `(identity-of F E)`, `(absorbing-of F E)` | E is neutral for F, or decides it: F(a, E) = a, or F(a, E) = E | e.g. 1 and 0 for `And`, 0 and 1 for `Or`, 0 for `Xor` |
| `(preserves-zero F)`, `(preserves-one F)` | F(0, 0) = 0; F(1, 1) = 1 | eight each |
| `(monotone F)` | raising an input never lowers the output | six: `Zero And Fst Snd Or One` |
| `(affine F)` | F is a sum modulo 2 of inputs and a constant | eight: `Zero One Fst Snd Nfst Nsnd Xor Xnor` |
| `(self-dual F)` | F is its own dual | four: `Fst Snd Nfst Nsnd` |
| `(complete-alone F)` | every function can be built from F alone | two: `Nand`, `Nor` |

The last five rows but one are **Post's five classes**, and the last is what they are for: a set of functions
can build every function exactly when, for each of the five classes, it has a member outside that class
(Post's theorem; stated here from memory, to be checked against a source by the stage). So the module can
*derive* `(complete-alone Nand)`, and answer *is {And, Or} complete?* (no: both are monotone) or *is {Imp,
Zero}?* by five lookups, where a program without these facts would have to search for constructions.

**How they are produced.** Not written by hand. The generator writes the 64 rows, and each property is either
- **generated beside the rows and held by a test** against the rows, as the tables of `std.digits` are, or
- **derived in Ein from the rows by a rule,** which is the more interesting half: `(commutative F)` is *for
  every a and b the two rows agree*, a universal statement over four rows, written with `std.macro`'s
  `forall` or as a rule with four table premises. That is
  [F1b](../../followups/f1b_logical_formulation.md)'s open question *"are rules able to detect properties
  from facts?"* on the smallest possible case, and [F7](../../followups/f7_rule_induction.md) B′'s *instance
  to type properties*. The stage tries both, and says which properties a monotone rule can derive (a property
  that is a finite conjunction of rows) and which need `absent` (a property stated as *no row disagrees*).

**What a program does with them:** a rule that simplifies a circuit or a formula reads them
(`(commutative F)` lets an argument order be normalised; `(complement-of And Nand)` and `(dual-of And Or)` are
De Morgan's laws as data; `(identity-of F E)` removes a constant input), which is what
[M1g's normal forms](../../m1g_logic/p1g.2_propositional_logic/s1g.2.3_normal_forms.md) need; the synthesis
application below searches fewer gates up to `swap-of`; and the relation properties of
[`std.algebra`](../../../stdlib/algebra.ein) (`symmetric`, `transitive`, `functional`) are the same idea one
level up, so the stage says whether these names should be those, applied to a gate's table read as a
relation.

**Of the three-input functions** there are 256, and the full adder's sum and carry are two of them. The
module does not list them; the stage counts how many classes they fall into under input permutation and
negation, as a first use of the facts above, if that is cheap, and says so if it is not.

## The applications, each tried and each answered with a program or a *no*

1. **Simulation:** inputs given, outputs derived. No search.
2. **The truth table of a circuit.** The inputs are **hypotheses**: one candidate per input and value, the
   theory saying an input has exactly one value. Then R is every input assignment, a query's projection
   classes are the table's rows, and `ein solve` prints the truth table. This is where search belongs, and it
   is Ein's existing answer shape used as it is.
3. **Satisfiability:** is there an input for which the output is 1? The same program with the output
   required: R is the satisfying assignments, and empty means unsatisfiable, with the cores saying why. A
   small SAT solver falls out of the engine, and the stage measures how small *small* is: the commitments
   limit, and what the region costs at 8, 16 and 24 inputs.
4. **Equivalence of two circuits:** the gate-built adder against the table, or two forms of one function.
   Equivalent when no input tells them apart, which is an unsatisfiability.
5. **Puzzles of truth and lies** (knights and knaves,
   [idea 09](../../ideas/09-puzzles-beyond-zebra.md)): a statement is a circuit over who is truthful.
6. **Synthesis:** which gate, put here, gives this table? The gate's type as the hypothesis. Tried on a
   one-gate hole and reported.

## Tasks

1. **T1, a prototype and a page, shown to the user:** each application above as a real program with its
   computed answer and every artefact, the names proposed, and what did not work.
2. **What ships,** as ruled: a small module of gate tables and propagation rules, its test programs, and the
   examples.
3. **The findings,** to [`findings.md`](findings.md). Expected, and to be confirmed: the commitments limit on
   a truth table of many inputs; and that an input's *exactly one value* wants a typed obligation.

## Acceptance

- Each application has a program and its computed answer, or a stated reason it has none.
- The gate-built full adder equals the table, held by a test.
- The page says which applications are search-free and which are a search, and what each costs.

## Where it leads

Binary logic is propositional logic's semantics: a formula is a circuit, a valuation an input assignment, a
tautology a circuit that is 1 on every row. Formulas as objects, proof systems, first-order logic and Peano
arithmetic are not this stage's and not this phase's; they are [M1g](../../m1g_logic/README.md), which the
user approved on 2026-11-11 and which runs after M1f; this stage is its starting point.

## Record, 2026-11-12: the recommended variants banked as examples

The user, 2026-11-12: *"bank binary arithmetic and logic prototypes into examples too"*. Done with
[S1f.6.16](s1f.6.16_binary_arithmetic.md#record-2026-11-12-way-b-banked-as-examples)'s in one commit.

**What is banked,** under [`examples/binary_logic/`](../../../examples/binary_logic/README.md):

| file | what | answer on the release binary of 2026-11-12 |
|---|---|---|
| `gates.ein` | the one table, `(gate2 NAME A B OUT)`, 64 rows, the sixteen names as proposed, `column-name`, `inv`, `le`, `bit`; generated | a module of facts |
| `props.ein` | the thirty relations as generated facts | a module of facts |
| `props_rules.ein` | the 31 rules that derive them | a module |
| `props_check.ein` | the rules against the generated facts | 30 claims held; 405 firings; search-free |
| `circuit.ein`, `inputs.ein`, `require.ein`, `full_adder.ein`, `bits.ein` | the three theories, the adder's five gates, and a copy of S1f.6.16's tables | modules; alone, R {∅} |
| `simulation.ein` | application 1 | sum 0, carry 1; search-free |
| `truth_table.ein` | application 2, by search | 8 states, 8 classes, 6 candidates, 30 saturations |
| `adder_against_table.ein` | § 4, and application 2 by vectors | 8 agree, none differs; search-free |
| `satisfiable.ein` | application 3 | 1 state, a = b = carry in = 1; N 10; 30 saturations; `Solution` under `--single` |
| `unsatisfiable.ein` | application 3 | R empty, *every alive node owes*, eleven owing nodes; 28 saturations; `Open` under `--single` |
| `equivalent.ein` | application 4 | R empty by owing, twelve owing nodes; 30 saturations; `Open` under `--single` |
| `not_equivalent.ein` | application 4 | 2 states |
| `knights.ein` | application 5 | 2 states, one class for C: 0; 26 saturations |
| `synthesis.ein` | application 6 | 2 states, `Or` and `Xor`; 16 candidates, 14 dead alone; 18 saturations |
| `simplify.ein` | § 6 | p is x; q is the constant 1; g3 and g4 one gate; search-free |
| `post.ein` | § 2, Post's criterion on seven sets | five complete, two inside a class; search-free |

(`ein test examples/binary_logic`, 50 claims held, and `ein solve` on each, 2026-11-12.) Every run exits 0.

**Checked apart from the engine.** Every claim of the hand-kept programs was written by a script that
enumerates in Python over the functions' columns and reads no table of Ein's
(`build/s1f.6.16-17/mk_logic.py`): the simulation, the eight rows, the satisfying assignment, the owing
frontiers (the maximal partial assignments that violate no requirement), the two counterexamples, the two
islands, the two functions that fit, the simplifier's facts, and for `post.ein` which sets are complete by
closing the two projections under composition. `ein test` holds them all. `props_check.ein` is the same
comparison for the thirty relations: Python's sets as claims, Ein's derivation from the rows.
**No answer differs from the page.** One count differs and is not an answer: the truth table reports 179
facts beyond root where the page's `tt_o` had 171, since the banked program derives a `row` fact a state, so
that its claim lists rows.

**Variants.** Layout 1, the one table, as the page recommends. The properties as generated facts with the
deriving rules as their check, as it recommends in its third question. Inputs as two candidates each under
an obligation. The six applications, the adder against the table, and the simplifier reading the generated
facts. The names as the stage file proposes them.

**Defaults taken,** each the user's to revise ([`open_questions.md`](../open_questions.md)): the truth
table's candidates come from the obligations rung alone, the simplest of the three ways the page shows
equal; `post.ein` is seven written sets over the generated facts with `forall`, and reads no `-u` relation;
the check of the properties is a corpus program's claims, and the generator is run by hand; `bits.ein` is a
copy; three inputs everywhere.

**Not banked,** and why: layout 2 and the `sim_rel` form, not recommended; the `forall` and `absent`
restatements of eight properties and the 140 written sets, which are the page's cross-check of the monotone
rules; the enumeration of all 65 535 sets and the three-input classes, which are Python; the hrule and
`:hypothesis-relations` truth tables, equal to the banked one; the `absent` encodings, refused or wrong
(F21's); the name probes; and the scale runs to 24 inputs, which stop at the budget or take minutes.

**The corpus and the goldens, the commit, `9612b350`, and what was checked after it** are in
[S1f.6.16's record](s1f.6.16_binary_arithmetic.md#record-2026-11-12-way-b-banked-as-examples), for both
directories.

**Not done:** no stdlib module; the acceptance's *held by a test* is here a corpus program with a claim,
`adder_against_table.ein`. **The page's eight questions stay open**; banking the recommended variants is the
instruction for the examples and not a ruling on them.

## The build, as ruled (2026-11-12)

**Status:** planned 2026-11-12, not started. It replaces task 2 of [§ Tasks](#tasks).
**Depends on:** [S1f.6.31](s1f.6.31_the_stdlib_layout.md), the layout; and
[S1f.6.16's build](s1f.6.16_binary_arithmetic.md#the-build-as-ruled-2026-11-12) for `std.math.bits`, which
the full adder built from gates is held equal to.

### The rulings

The user's replies of 2026-11-12, each to the question of [`questions.md`](questions.md) named:

| question | the reply, as typed | what it makes of the build |
|---|---|---|
| [19](questions.md#q19), one table or a relation a function | *"(c)"* | both: the table `(gate2 And 1 1 1)`, and the relation-per-function layout `(And 1 1 1)` derived from it by sixteen rules |
| [20](questions.md#q20), the property facts as rules or as generated facts | *"(b), and I think we want to have those rules in separate reusable file"* | rules, in a module of their own; a program that loads it pays the 405 firings, and one that does not pays nothing |
| [21](questions.md#q21), the one-sided identity and absorbing facts | *"(b)"* | the left and right forms too |
| [22](questions.md#q22), three corrections to the stage file's hand counts | *"(a)"* | confirmed: four swap pairs and eight functions their own swap; the duals `Andn`/`Orn` and `Randn`/`Rorn`; twelve functions up to the order of the inputs |
| [23](questions.md#q23), what ships | *"(a) and examples/binary_logic already made"* | all four small modules and the six applications. [Round 2, item 23](questions_round2.md#q23) reads *ships* as *into the stdlib, with a test program per rule and the mutation census*, option (a) there. **Replied in round 2: *"std.bool"***, read as (a) under that name |
| [25](questions.md#q25), a cheaper search for *exactly one value per object* | *"Is this about SAT, binary circuits or what? I don't see how (a) and (b) conflict, are they?"* | both hold ([round 2, item 25](questions_round2.md#q25); **replied there: *"yes, both"***): here the search form is a demonstration, and the cheaper route is [M1g's S1g.6.6](../../m1g_logic/p1g.6_engine_requirements/s1g.6.6_exactly_one_value_per_object.md) |

**The library's name.** Round 2 proposed `std.logic.*`. The user's replies in round 2, committed as `dc88520b`: to the
layout (quoted in full in [S1f.6.31 § The reply](s1f.6.31_the_stdlib_layout.md#the-reply)), *"I think std.bool
suits better here than std.logic"*, and to item 23, *"std.bool"*. So the modules are **`std.bool.gates`, `std.bool.props`, `std.bool.circuit` and
`std.bool.inputs`,** under `stdlib/bool/`. `std.logic` is left free, which suits
[M1g](../../m1g_logic/README.md): propositional logic, in the reply's own list, is another subject than
Boolean algebra.

The gates' names stand as proposed (`Zero And Andn Fst Randn Snd Xor Or Nor Xnor Nsnd Orn Nfst Rorn Nand One`,
the table `gate2`); question 19 left a line for an objection and none was typed.

### The modules

| module | what it holds | notes |
|---|---|---|
| `std.bool.gates` | the table `(gate2 NAME A B OUT)`, 64 rows, generated; `column-name`, `inv`, `le`, `bit`; and **sixteen rules** that derive the relation-per-function layout from the table (reply 19 (c)) | the table is generated, the sixteen rules written by hand; the per-function relations are derived only in a program that selects them |
| `std.bool.props` | the thirty relations known of the sixteen, **as rules** (reply 20 (b)): the 31 monotone rules of the banked `props_rules.ein`, plus the left and right forms of `identity-of` and `absorbing-of` (reply 21 (b)), which hold of eight functions each where the two-sided forms hold of four (`questions.md` 21) | 405 firings on the prototype for the thirty (`questions.md` 20); the count with the one-sided forms is measured |
| `std.bool.circuit` | `gate`, `val`, one propagation rule | |
| `std.bool.inputs` | an input owes exactly one value, with `require` | the obligation is `std.algebra`'s kind; proposed by the obligations rung alone |

### Tasks

1. **T1f.6.17.4 — the four modules,** moved from `examples/binary_logic/` into `stdlib/bool/` and
   restated to the rulings: the sixteen layout rules; the properties as rules, with the one-sided forms; each
   module's header with the names it occupies, its closing rule or its `__closed__` facts, a `:why` on every
   relation and rule. Whether the per-function relation names (`And`, `Or`, …) may be relation names beside
   the same names as values of `gate2` is checked on the release binary first: a name is both today in no
   stdlib module.
2. **T1f.6.17.5 — the generated facts become the check.** Today `props.ein` is generated facts and
   `props_check.ein`'s thirty claims hold exactly when the rules derive them and no other. With the rules
   shipping, the direction turns: `gen_gates.py` (moved to `utils/`, its `--check` in the gate through a
   cargo test) writes the table and the *expected* property facts, and a test holds the rules to them, the
   one-sided forms included, each count re-taken from the rows as reply 22 confirmed them.
3. **T1f.6.17.6 — the tests a stdlib module has:** a program per rule or tight family under
   `tests/stdlib/bool/`, each with its `:expect` (a library with no refutation rule, so the
   idiom for a negative case is *naming a relation closes it*); `stdlib_coverage.rs`; the mutation census with no unexplained
   survivor; a harness in Rust that computes every property by enumeration over the functions' columns and
   holds the rules to it.
4. **T1f.6.17.7 — the full adder from gates against `std.math.bits`' table, as a test,** which is the stage
   file's acceptance line; today it is a corpus program with a claim, `adder_against_table.ein`.
5. **T1f.6.17.8 — the six applications import the stdlib.** `examples/binary_logic/`'s programs keep their
   claims and import `std.bool.*`; the theory files beside them are retired; `unsatisfiable.ein` and
   `equivalent.ein` go on claiming the owing frontier and `(false)` for the query (E-23; question 24 is not
   ruled, and its subject is [M1g's P1g.7](../../m1g_logic/p1g.7_unsatisfiable_against_owing/README.md)).
   The sizes stay the corpus's: three inputs for every search.
6. **T1f.6.17.9 — the pages:** [`binary_logic.md`](binary_logic.md) gets a closing section, what shipped and
   what each ruling made of its questions; [`stdlib/README.md`](../../../stdlib/README.md) a section a
   module; `examples/binary_logic/README.md`; `AGENTS.md`; [M1g's README](../../m1g_logic/README.md) is told
   where its starting point now lives.

### Acceptance

- The four modules are in the stdlib under `std.bool`, each rule activated by a program under
  `tests/stdlib/bool/`, every property held to an enumeration written in Rust.
- Both layouts of the sixteen functions are available, the second derived from the first.
- The one-sided identity and absorbing facts are derived, and the three corrected counts are held by a test.
- The gate-built full adder equals the binary adder's table, held by a test.
- Every application's claim holds as it did.

### What it must not do

- **Not build a cheaper search for inputs** (reply 25: M1g).
- **Not change what an unsatisfiable search reads as** (question 24: not ruled).
- **Not name anything `std.logic`.**
- **Not write formulas as objects, valuations or proof systems:** those are M1g's.

### The standing rules

A wrong answer of the engine met on the way is a row in `corpus/wrong_answers.toml`, filed `open` before its
fix. Findings go to [`findings.md`](findings.md) under the next free id; questions to
[`open_questions.md`](../open_questions.md) under the next free id. Config never changes an answer. The gate
is `./run_tests.sh`, then `./run_tests.sh --nightly` after each code commit; `utils/ci_container.sh` is not
needed unless a view of the new modules trips the DOT sweep, which S1f.6.21 makes skip big graphs.

## Record, 2026-11-14: the build

Run on the coordinator's word of 2026-11-14, after S1f.6.31, S1f.6.35 and S1f.6.36, on master at `6de252e3`, tasks
T1f.6.17.4, .5, .6, .8 and .9. **T1f.6.17.7 waits on S1f.6.16**, whose build stopped at its trial for the user's
choice (`6de252e3`, [its record](s1f.6.16_binary_arithmetic.md#record-2026-11-14-t1f6165-the-trial-and-the-stop)):
no `std.math.*` module was made, `adder_against_table.ein` keeps its claim and reads `bits.ein`, the copy beside it,
and only its imports moved, from the retired theory files to the stdlib's.

**The commits:** `24f59537`, the build (T1f.6.17.4 to .6 and .8, the goldens, the module-facing pages); this record,
`binary_logic.md`'s closing section, the findings, the questions and M1g's pointer in the commit after it.

### The plan, read through what changed since it was written

- **S1f.6.35, every import whole.** The plan's *"the per-function relations are derived only in a program that
  selects them"* was written when a selection brought only what it named; now no import selects the sixteen layout
  rules apart from the table, so every program that imports `std.bool.gates` derives the second layout
  ([Q-M1f.263](../open_questions.md#q-m1f263--the-sixteen-layout-rules-of-stdboolgates-now-that-an-import-brings-every-rule),
  defaulted). Every declared name has a full name, and T1f.6.17.4's check of a name that is both a relation and a
  value was run under that resolver (below).
- **S1f.6.31, the layout.** `stdlib/bool/<module>.ein` and `tests/stdlib/bool/<module>/`, read by the manifest, the
  coverage gate, the census and the mutation sweep at every depth with no change to any of them.
- **S1f.6.34 and S1f.6.36.** `std.bool.inputs` closes `is-a` and `require` and owes `in`, which nothing closes or
  lists, so no mechanism conflicts with another (`defined_behaviour.md` § 4.27); its obligation is proposed by the
  obligations rung, as banked. No program needs `:no-search`: the ones with no candidate are judged at root already.
- **The gate cadence** (the user, 2026-11-13) replaces the stage file's last line: this stage ran the checks listed
  below on its working tree, and not `./run_tests.sh`, which is the coordinator's, once a batch.

### T1f.6.17.4 — the four modules, done

**The check first: a name both a relation and a value.** No stdlib module had one. On the release binary, a module
declaring a relation `And` whose table holds the value `And`, `(gate2 And 1 1 1)`, and a rule that reads the table
through a gate's function: listed, `(import g :symbols (gate2 And))`, the program's `And` is the module's in both
places and the claim holds, the relation's four rows derived beside it; whole, `g.And`, and by an alias, `h.And`,
alike. **Unlisted, the program's `And` is its own**, a name no row has, and the gate computes nothing, with no line
printed: S1f.6.35's *a short name no import lists is the file's own, shadowing in silence*. So the sixteen names may
be relation names, as reply 19 (c) has them, and each becomes `std.bool.gates`', which every program that writes one
lists. That is E-62 (`knights.ein` with `Nfst` unlisted answers four states where two) and
[Q-M1f.264](../open_questions.md#q-m1f264--a-functions-name-a-program-writes-and-does-not-list-e-62), defaulted to
the resolver as ruled, with the rule said in the module's header, `stdlib/README.md` and the examples' README.

**The modules**, moved with `git mv` from `examples/binary_logic/` and restated to the rulings, each with a header
that names what it occupies, an entry that is its closing rule, and a `:why` on every relation and rule:

| module | from | what changed |
|---|---|---|
| [`std.bool.gates`](../../../stdlib/bool/gates.ein) | `gates.ein` | the sixteen relations of the second layout, `(And A B OUT)`, and sixteen hand-written rules `Zero-rows` to `One-rows` that derive them from the table (reply 19 (c)); the table's rows generated between two marked lines of a file otherwise hand-written; a closing rule, `gates`, where the banked file closed its tables by written facts |
| [`std.bool.props`](../../../stdlib/bool/props.ein) | `props_rules.ein` | written by hand now, no longer by the generator: the same 31 rules, the one-sided identity and absorbing forms among them already: reply 21 (b) asked for what the prototype's rules derived, which the plan's *"plus"* did not know; the entry `props-derived` renamed `props` |
| [`std.bool.circuit`](../../../stdlib/bool/circuit.ein) | `circuit.ein` | the entry names `gates`, the module below |
| [`std.bool.inputs`](../../../stdlib/bool/inputs.ein) | `inputs.ein`, `require.ein` | `require` and its rule `required` inside it (the ruled table's *"with `require`"*), `requirements`, the banked entry, gone; the entry `inputs` closes `is-a` and `require` and names `circuit` |

What a program pays at root for each, `ein saturate` on a program that imports it and writes nothing else
(2026-11-14): `std.bool.gates` 65 firings, the sixteen layout rules' 64 and its entry; `std.bool.props` 470, 405 of
them its own; `std.bool.circuit` 66; `std.bool.inputs` 67. The 405 is the prototype's 405 (`questions.md` 20): the
one-sided forms were in it.

### T1f.6.17.5 — the generated facts become the check, done

`gen_gates.py` moved to [`utils/`](../../../utils/gen_gates.py) and turned round. It writes `std.bool.gates`' rows,
and, from the sixteen columns, every relation `std.bool.props` derives as the **expected** facts: the claims of seven
programs under `tests/stdlib/bool/props/`, a family of relations each, every relation claimed whole, and of
`tests/stdlib/bool/gates/01_a_relation_a_function.ein`, the second layout's sixteen. `props.ein`, the facts, and
`props_check.ein`, the program that held the rules to them, are gone from the examples: the rules ship and the facts
check them. Its `--check` is in the gate through `ein-infer/tests/bool_theory.rs`, as `gen_arith_tables.py`'s is
through `digit_tables.rs`; `--count` prints each relation's count, re-taken from the columns.

The counts reply 22 confirmed, re-taken from the rows (`python3 utils/gen_gates.py --count`, 2026-11-14, and held by
`bool_theory.rs` on the facts the rules derive): `swap-of` four pairs and eight functions their own swap, twelve
functions up to the order of the inputs; `dual-of` with `Andn`/`Orn` and `Randn`/`Rorn`, `Andn`/`Rorn` and
`Randn`/`Orn` the complements. And the one-sided forms: each holds **eight facts over seven functions**, `Fst` (or
`Snd`) holding both bits on its free side, where `questions.md` 21 said *eight functions*; the two-sided four
facts, four functions.

### T1f.6.17.6 — the tests a stdlib module has, done

- **Twelve programs under `tests/stdlib/bool/`**, a directory a module, eight written by the generator (`gates/01` and
  `props/01` to `07`) and four by hand (`circuit/01_propagate`, `circuit/02_one_value`, `inputs/01_in_drives`,
  `inputs/02_required`). Each states R, `(solve :expect …)`, and its relations whole; `one-value` and `required`, the
  two rules that refute, each fire in one file and are loaded and silent in another. `tests/README.md` has the
  table, and what would fail each.
- **The harness in Rust**, [`ein-infer/tests/bool_theory.rs`](../../../ein.rs/crates/ein-infer/tests/bool_theory.rs),
  eight tests: the table against the sixteen columns, none missing and none twice; the second layout against the
  rows; **every property against its definition**, computed from the columns with `affine` as *some* `c0 ^ c1·a ^
  c2·b` and not the parity the rule reads, `monotone` over every pair of points, `self-dual` as its own dual; the
  three corrected counts and the one-sided forms on the derived facts; each closing rule; no `absent`, `not` or `or`
  in any rule, `neq` and `(false)` in the two that refute alone; the generator's `--check`. Made to fail once on
  purpose: `affine-r` reading `Or`'s rows for `Xor`'s, *affine: the rules miss [(affine Fst), …]*, then restored.
- **The coverage gate**, `stdlib_coverage.rs`, with no change: every one of the 55 rules is activated by a program
  under `tests/stdlib/bool/`.
- **The mutation census** (`python3 utils/stdlib_mutants.py`, 2026-11-14): 458 of 547 mutants killed, where 431 of
  502 were; of `std.bool`'s 45, 27 killed and **18 survive, each equivalent to its rule**: thirteen exchange an
  `(inv …)` premise, and `inv` is its own converse; five exchange the conclusion of `swap`, `complement`, `dual`,
  `left-neg` and `right-neg`, each relation an involution, so the rule's matches come in mirrored pairs and the
  conclusion the other way round writes the same facts. No claim made of facts can tell either kind from its rule,
  so none is a hole; `tests/README.md` says so, and the 18 are banked with the 71 before them, none of which moved.
  Every mutant of `std.bool.circuit` and `std.bool.inputs` is killed, as is every one that exchanges an `le`, a
  one-sided relation or a value of `val`.

### T1f.6.17.8 — the six applications import the stdlib, done

The eleven programs of `examples/binary_logic/` and `full_adder.ein` import `std.bool.*`, each listing the names it
writes from the module that declares it, a function's name from `std.bool.gates` (E-62); `simplify.ein` and
`post.ein` read `std.bool.props`' derived facts where they read the generated ones. Seven theory files are retired:
`gates.ein`, `props_rules.ein`, `circuit.ein` and `inputs.ein` became the stdlib's, `require.ein` went into
`std.bool.inputs`, `props.ein` and `props_check.ein` became the generator's expected facts. `full_adder.ein` and
`bits.ein` stay, the adder an example's module and the table T1f.6.17.7's.

**Every claim holds as it did, and R is equal** (`ein solve --json-summary` on each program before, from `HEAD`'s
tree, and after, compared with every module prefix taken off; 2026-11-14):

| program | R, the queries, the claims | saturations | facts beyond root | facts at root |
|---|---|---:|---:|---|
| `simulation` | equal | 1 | 0 | 127 → 260 |
| `truth_table` | equal | 30 | 179 | 139 → 277 |
| `adder_against_table` | equal | 1 | 0 | 257 → 390 |
| `satisfiable` | equal | 30 | 178 | 146 → 276 |
| `unsatisfiable` | equal | 28 | 156 | 147 → 277 |
| `equivalent` | equal | 30 | 215 | 149 → 279 |
| `not_equivalent` | equal | 30 | 197 | 147 → 277 |
| `knights` | equal | 26 | 139 | 145 → 275 |
| `synthesis` | equal | 18 | 169 | 242 → 374 |
| `simplify` | equal | 1 | 0 | 395 → 764 |
| `post` | equal | 1 | 0 | 407 → 776 |

What grew is root, once: the second layout's 64 facts and the declarations, closures and rule names of the
modules that now come whole (`simulation`: the sixteen relations' 64 facts, 17 more `__closed__`, 34 `relation`
facts, 17 `gates-rule`, one `circuit-rule`), and in `simplify` and `post`, which read generated facts until now,
the table and `assoc-at`'s 96 steps. The search is the same search: the same candidates, the same saturations, the
same facts beyond root. `ein test examples/binary_logic`: 20 claims held of 20, the 30 of `props_check.ein` now
the generated programs'. Each answers alike imported selectively, whole and by an alias
(`ein-cli/tests/import_invariance.rs`, which reads every corpus program that imports). The sizes stay the
corpus's, three inputs for every search.

### T1f.6.17.9 — the pages, done

[`binary_logic.md`](binary_logic.md) § What shipped, each of its eight questions with the reply and what it made;
its table of banked files re-pointed to where they went, the text kept (Q-M1f.262's default);
[`stdlib/README.md`](../../../stdlib/README.md) § `std.bool`, a subsection a module, the layout's and the module
table's rows, and the coverage count re-taken (423 of 423, its command and date);
[`examples/binary_logic/README.md`](../../../examples/binary_logic/README.md), rewritten for programs over the stdlib;
`examples/README.md`'s line; [`tests/README.md`](../../../tests/README.md) § `stdlib/bool/`; `utils/README.md`'s
line for the generator; `AGENTS.md`, the stdlib paragraph's `std.bool` locations and the generated-files rule, the
generator now in the gate; [M1g's README](../../m1g_logic/README.md), where its starting point lives. `corpus/corpus.toml`:
seven entries gone with their files, four stdlib modules and twelve tests added, four notes restated.

### T1f.6.17.7 — the full adder against `std.math.bits`' table: waits on S1f.6.16

Not this build's. `std.math.bits` is S1f.6.16's build, which stopped at its trial and, since, at its design
(`1302db46`) for the user's choice; nothing here makes a `std.math.*` module. `adder_against_table.ein` stays the
acceptance line's corpus program with its claim, 8 vectors agree and none differs, reading `fa2` from
`examples/binary_logic/bits.ein`, the copy `examples/binary_arithmetic/gen_binary.py` writes; only its imports
moved, from the retired `gates.ein` and `full_adder`'s re-export to `std.bool.gates`, `std.bool.circuit` and
`full_adder` bare, its R and claims equal (T1f.6.17.8's table). When S1f.6.16 builds, T7 turns the program into a
test against `std.math.bits` and the copy goes.

### The goldens, named before the bless and counted after

Named before: the exit table, the cells of the seven retired example files out and those of the four stdlib modules
and twelve tests in, nothing else; the shape digests, every rendering of the seven retired files out, those of the
sixteen new files in, and the renderings of the twelve `examples/binary_logic/` files that import `std.bool` moved,
the imports and root's facts being new, `bits.ein`, which imports nothing, unmoved; nothing outside
`examples/binary_logic/`, `stdlib/bool/` and `tests/stdlib/bool/`.

- **`ein-cli/tests/golden/corpus_exits.txt`**: 19 cells out, the seven retired files'; 76 in, sixteen of the stdlib
  modules and sixty of the tests, every one 0 but the twelve tests' `render rules`, 1, as every test program's is
  (no rule of its own); no other cell moved, the rest of the diff the order of the blocks.
- **`ein-render/tests/golden/corpus_shapes.md5`**: of 24 290 renderings, 287 out (the seven retired files, 41 each;
  four of them are the moved modules' old paths), 656 in (the sixteen new files), and **371 moved, on exactly the
  twelve files named**, `bits.ein` not among them: `dot` 144, `ir` 48, `cores` 33, `load`, `saturate`, `match`,
  `plan`, `naf`, `hyp`, `hyp+closed`, `oracle`, `explain`, `explain-noalts` and `region-trace` 12 each, `states` 10,
  `read-out` 3, `region-lattice` 1. The `states` renderings grew by root's facts. The four read-out and lattice
  digests are `synthesis.ein`'s under `--maximal`, `--single` and `-n 2`: the same answer at another cost, since
  the candidates' order follows their names' spelling and the program lists three of the sixteen (E-64); its
  describe is unchanged, 18 saturations.
- **`ein-render/tests/dot_wellformed.rs`'s `SKIPPING`**: two directories more, `stdlib/bool` and
  `tests/stdlib/bool/props`, each view past 1 000 edges a `kb-since` of a program that holds `std.bool.props`' facts
  (1 490 edges); `examples/binary_logic` was there already, for `props_rules.ein`, and holds four such views now.
- **`tests/mutation_survivors.txt`**: 18 lines in, the equivalent mutants above, and none out; 89 survivors where 71.

### The checks, on the working tree before the commit

`cargo test --release --workspace`: every test but the three goldens above before their bless, and those after
(`corpus_cli`, `corpus_shapes`, `dot_wellformed` re-run green); among them `import_invariance.rs`, every corpus
program that imports answering alike selective, whole and by an alias, the new ones included; `stdlib_coverage.rs`;
`answer_round_trip.rs`, every dump reloaded. `cargo fmt --check`, `cargo clippy --workspace --all-targets -D
warnings`, `cargo doc --no-deps -D warnings`: clean. `python3 utils/doc_audit.py --links --check` over the gate's
pages: no finding. `python3 utils/stdlib_manifest.py`: 16 modules match. `python3 utils/stdlib_census.py --check`:
423 rules, none at zero, `std.bool`'s 55 among them. `python3 utils/stdlib_mutants.py`: 458 of 547, the 18 new survivors the equivalent ones, banked; `--check` against the bank, re-run after it: exit 0, no survivor new and none killed.
`python3 utils/gen_gates.py --check`: exit 0, and inside the gate through `bool_theory.rs`. No other generator's
input was touched. Not run, on the coordinator's word: `./run_tests.sh` and `--nightly`.

### Acceptance

- **The four modules are in the stdlib under `std.bool`**, each rule activated by a program under
  `tests/stdlib/bool/` (`stdlib_coverage.rs`, and the census, 55 of 55), **every property held to an enumeration
  written in Rust** (`bool_theory.rs`'s `every_property_is_its_definition`, thirty relations and the step). Done.
- **Both layouts of the sixteen functions are available, the second derived from the first**: the table and the
  sixteen rules of `std.bool.gates`, `each_relation_of_a_function_is_its_rows`. Done.
- **The one-sided identity and absorbing facts are derived, and the three corrected counts are held by a test**:
  `the_one_sided_forms_are_derived` and `the_three_corrected_counts`, on the facts the rules derive. Done.
- **The gate-built full adder equals the binary adder's table, held by a test**: waits on S1f.6.16 (T1f.6.17.7);
  until then the corpus program `adder_against_table.ein` holds it against the copied table, as before.
- **Every application's claim holds as it did**: 20 of 20, R equal on all eleven programs but for the names. Done.

What it must not do, not done: no cheaper search for inputs (reply 25), no change to what an unsatisfiable search
reads as (question 24), nothing named `std.logic`, no formula as an object.

### Defaults, findings, and what is the user's

- [Q-M1f.263](../open_questions.md#q-m1f263--the-sixteen-layout-rules-of-stdboolgates-now-that-an-import-brings-every-rule):
  the sixteen layout rules derived in every program that imports `std.bool.gates`, (a) as the ruled table has them,
  where the plan's *"only in a program that selects them"* is S1f.6.35's to overrule. **Defaulted, the user's to
  revise.**
- [Q-M1f.264](../open_questions.md#q-m1f264--a-functions-name-a-program-writes-and-does-not-list-e-62): a
  function's name written and not listed is the program's own, in silence (E-62); (a) kept as S1f.6.35 ruled, (b) a
  guard in `std.bool.circuit` the cheapest remedy. **Defaulted, the user's to revise.**
- [Q-M1f.265](../open_questions.md#q-m1f265--the-names-and-file-shapes-stdbools-build-chose): the entries
  `gates`, `props`, `circuit`, `inputs`; the rules `Zero-rows` to `One-rows`; `gates.ein` hand-written but for its
  rows; the expected facts as eight generated programs. **Defaulted, the user's to revise.**
- **E-62** (a function's name unlisted computes nothing, in silence) and **E-64** (a route's cost follows the
  names' spelling, so the import form), in [`findings.md`](findings.md); neither is a wrong answer. No wrong answer
  of the engine was met: every application's R is the one before the build.
- The count `questions.md` 21 gave, *the left and right forms hold of eight each*, is eight **facts** over seven
  functions; recorded, not a question.
