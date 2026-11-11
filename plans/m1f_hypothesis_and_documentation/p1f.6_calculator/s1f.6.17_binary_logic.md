# S1f.6.17 — Binary logic: truth tables, gates, and what they are good for

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-11, on the user's instruction of that day. **T1 done 2026-11-11, as a prototype and a
page shown to the user: [`binary_logic.md`](binary_logic.md).** Nothing shipped; T2 and T3 wait for the user's
rulings on that page's questions. T1 re-counted this page's tables from the rows and corrected three hand
counts, each marked below.
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
