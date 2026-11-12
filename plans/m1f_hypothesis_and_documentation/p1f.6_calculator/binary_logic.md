# Binary logic: the sixteen functions, gates, and six applications

**Stage:** [S1f.6.17](s1f.6.17_binary_logic.md), task T1. **Phase:** [P1f.6](README.md).
**Status:** prototype, 2026-11-11. Nothing here ships; it is shown to the user, who rules. **The recommended
variants banked as examples 2026-11-12**, [`examples/binary_logic/`](../../../examples/binary_logic/README.md);
the questions stay open.

The user, 2026-11-11: *"binary logic, truth tables, can be there any applications for it?"*, then *"there are
16 different boolean functions of 2 arguments … I want to have them all and in 2 forms"*, then *"plan add
symmetry relations and other useful information"*.

**The answer, measured.** All sixteen load as facts under the proposed names, in either layout, and none of
the names fails. Everything the stage file lists as *known about the sixteen*, thirty relations and
properties, is **derived in Ein from the 64 rows by monotone rules**, with no `absent`, and agrees with a
Python count and with facts generated beside the rows; the re-count corrected three of the stage file's hand
counts. Post's criterion, read from Ein's facts, agrees with an independent enumeration on all 65 535 sets of
functions. All six applications have a program and a computed answer. Three are saturation alone. Three are
a search, and the search is small: describing R costs 3ⁿ saturations for n inputs and finding one state 2ⁿ,
so a truth table by search ends near 14 inputs and a satisfiability question between 16 and 20.

Every number is from a run of 2026-11-11, with the command that re-takes it. The prototype is under
`build/s1f.6.17/` (gitignored): `g.py` holds the sixteen columns, `gen.py` writes the modules under `p/`,
`check_props.py`, `post.py`, `scale.py` and `scale2.py` take the measurements, and every read-out quoted here
is whole under `out/`. It runs the binary and the stdlib that
[`binary_arithmetic.md`](binary_arithmetic.md) froze: the release binary as built at 19:17 that day, against
the stdlib of `a621dc75`.

**Where it is now (2026-11-12).** The recommended variants are banked as examples, on the user's instruction
of that day, *"bank binary arithmetic and logic prototypes into examples too"*:
[`examples/binary_logic/`](../../../examples/binary_logic/README.md).

| this page | banked as |
|---|---|
| § 1, layout 1, the one table | [`gates.ein`](../../../examples/binary_logic/gates.ein), written by [`gen_gates.py`](../../../examples/binary_logic/gen_gates.py), which is `g.py` and `gen.py` |
| § 2, the thirty relations, as generated facts | [`props.ein`](../../../examples/binary_logic/props.ein) |
| § 2, the rules that derive them | [`props_rules.ein`](../../../examples/binary_logic/props_rules.ein), and [`props_check.ein`](../../../examples/binary_logic/props_check.ein), whose thirty claims hold exactly when the rules derive the generated facts and no other |
| § 2, Post's criterion on written sets | [`post.ein`](../../../examples/binary_logic/post.ein), seven sets, over the generated facts |
| § 3, the circuit theory; § 5.2 and § 5.3, inputs and requirements | [`circuit.ein`](../../../examples/binary_logic/circuit.ein), [`inputs.ein`](../../../examples/binary_logic/inputs.ein), [`require.ein`](../../../examples/binary_logic/require.ein) |
| § 4, the adder from five gates, against the table | [`full_adder.ein`](../../../examples/binary_logic/full_adder.ein), [`adder_against_table.ein`](../../../examples/binary_logic/adder_against_table.ein) |
| § 5, the six applications | `simulation`, `truth_table` (the obligations rung alone, this page's `tt_o`), `satisfiable` and `unsatisfiable`, `equivalent` and `not_equivalent`, `knights`, `synthesis` |
| § 6, the simplifier | [`simplify.ein`](../../../examples/binary_logic/simplify.ein), reading the generated facts |

They are examples and not a stdlib module, and the generator is run by hand, not in the gate; what the gate
holds is `props_check.ein`'s claims. Not banked, and still here and under `build/s1f.6.17/`: layout 2, the
`forall` and `absent` restatements of eight properties, the 140 written sets and the enumeration of all
65 535, the three-input classes, the hrule and `:hypothesis-relations` forms of the truth table, the
`absent` encoding that is refused or wrong, and the scale runs. **The banking is not a ruling: every
question at the foot of this page stays open.**

The banked programs were run on the release binary and the stdlib of 2026-11-12, and every answer this page
reports stands: the rules derive the thirty relations in 405 firings (`ein saturate
examples/binary_logic/props_check.ein`); simulation, sum 0 and carry 1; the truth table, 8 states and 30
saturations; satisfiable, 1 state, 10 minimal dead sets, 30 saturations; unsatisfiable, R empty by owing,
eleven owing nodes, 28 saturations, `Open` under `--single`; equivalent, R empty by owing, 30 saturations;
not equivalent, 2 states; knights, 2 states and one class, 26 saturations; synthesis, `Or` and `Xor`, 14 of
16 dead alone, 18 saturations; the simplifier's three answers (`ein solve` on each file, 2026-11-12). The
claims in the banked files were computed apart from the engine, in Python by enumeration over the functions'
columns (`build/s1f.6.16-17/mk_logic.py`), and `ein test examples/binary_logic` holds all of them.

## 1. The sixteen, as generated facts, in two layouts

**Layout 1, one table** (`p/gates.ein`, 64 rows, generated by `gen.py` from the four-bit columns):

```lisp
(relation gate2 Function Bit Bit Bit :why "{?1} of {?2} and {?3} is {?4}")
(relation column-name Function Column :why "{?1} is the function whose outputs at 00, 01, 10 and 11 spell {?2}")
(gate2 Zero 0 0 0) (gate2 Zero 0 1 0) (gate2 Zero 1 0 0) (gate2 Zero 1 1 0)   (column-name Zero f0000)
(gate2 And 0 0 0) (gate2 And 0 1 0) (gate2 And 1 0 0) (gate2 And 1 1 1)   (column-name And f0001)
(gate2 Andn 0 0 0) (gate2 Andn 0 1 0) (gate2 Andn 1 0 1) (gate2 Andn 1 1 0)   (column-name Andn f0010)
…
(gate2 Nand 0 0 1) (gate2 Nand 0 1 1) (gate2 Nand 1 0 1) (gate2 Nand 1 1 0)   (column-name Nand f1110)
(gate2 One 0 0 1) (gate2 One 0 1 1) (gate2 One 1 0 1) (gate2 One 1 1 1)   (column-name One f1111)
```

with three small tables beside it that the rules below read: `(inv 0 1) (inv 1 0)`, negation; `(le 0 0) (le 0
1) (le 1 1)`, the order of the bits; `(bit 0) (bit 1)`.

**Layout 2, a relation a function** (`p/gates_rel.ein`): `(And 0 0 0) (And 0 1 0) (And 1 0 0) (And 1 1 1)`,
sixteen relations of four rows.

**Both load, and every proposed name loads:** `Zero And Andn Fst Randn Snd Xor Or Nor Xnor Nsnd Orn Nfst Rorn
Nand One`, as a value in layout 1 and as a relation in layout 2 (`ein solve p/gates.ein`, `p/gates_rel.ein`:
*no candidate*, k_s 1). So do the one-input `Not` and `Id`, the connective names `Bot Top Imp Iff Nimp Rimp
Nrimp`, the systematic names `f0000` to `f1111`, and a formula `(Imp (And p q) p)` as a nested fact
(`p/names1.ein`). What does not load is the lower-case word, as the stage file expected:

```text
names_and.ein:2:8: unexpected `and`
  repair: Delete `and`, Shift `0`, Insert `(`
```

for `(gate2 and 0 0 0)`, and the same for `or` and `not`. `eq` and `false` do load as a bare value there
(`p/names_eq.ein`), which is no reason to use them.

**Every four-bit column is named exactly once.** A rule reads a function's four rows into one fact,

```lisp
(rule column-of ()
  :match  (and (gate2 ?f 0 0 ?a) (gate2 ?f 0 1 ?b) (gate2 ?f 1 0 ?c) (gate2 ?f 1 1 ?d))
  :assert (column ?f ?a ?b ?c ?d)
  :why    "the four rows of {?f}")
```

and `python3 check_props.py` reads the sixteen derived `column` facts back: *columns derived: 16 · each of
the 16 named exactly once: True · second names agree: True*.

## 2. What is known about the sixteen

Thirty relations: the stage file's, with the one-sided forms of *identity* and *absorbing*, the two *ignores*,
and five *escapes*, each the outside of one of Post's classes. Each was produced three ways and compared
(`python3 check_props.py`, `out/check_props.txt`):

- **Python**, from the columns (`g.py`'s `properties()`);
- **generated facts**, written beside the rows (`p/props_gen.ein`) and read back from `ein saturate --dump`;
- **derived in Ein** from the 64 rows by rules (`p/props.ein`, 31 rules and a closing rule, 405 firings,
  0.7 ms).

| fact | holds of | Python | generated | derived by rules |
|---|---|---:|---:|---:|
| `(swap-of F G)` | every function: four pairs and eight their own | 16 | 16 | 16 |
| `(complement-of F G)` | every function: eight pairs | 16 | 16 | 16 |
| `(dual-of F G)` | every function: six pairs and four their own | 16 | 16 | 16 |
| `(left-negated F G)`, `(right-negated F G)` | every function, each | 16, 16 | 16, 16 | 16, 16 |
| `(commutative F)` | `Zero And Xor Or Nor Xnor Nand One` | 8 | 8 | 8 |
| `(associative F)` | `Zero And Xor Or Xnor One Fst Snd` | 8 | 8 | 8 |
| `(idempotent F)` | `And Or Fst Snd` | 4 | 4 | 4 |
| `(reads-first F)`, `(reads-second F)` | twelve each; ten read both | 12, 12 | 12, 12 | 12, 12 |
| `(ignores-first F)`, `(ignores-second F)` | `Zero One Snd Nsnd`; `Zero One Fst Nfst` | 4, 4 | 4, 4 | 4, 4 |
| `(identity-of F E)` | `And`/1, `Or`/0, `Xor`/0, `Xnor`/1 | 4 | 4 | 4 |
| `(right-identity-of F E)`, `(left-identity-of F E)` | the four, and `Andn`/0, `Orn`/1, `Fst`/0, `Fst`/1 on the right; `Randn`/0, `Rorn`/1, `Snd`/0, `Snd`/1 on the left | 8, 8 | 8, 8 | 8, 8 |
| `(absorbing-of F E)` | `And`/0, `Or`/1, `Zero`/0, `One`/1 | 4 | 4 | 4 |
| `(right-absorbing-of F E)`, `(left-absorbing-of F E)` | eight each | 8, 8 | 8, 8 | 8, 8 |
| `(preserves-zero F)`, `(preserves-one F)` | eight each | 8, 8 | 8, 8 | 8, 8 |
| `(monotone F)` | `Zero And Fst Snd Or One` | 6 | 6 | 6 |
| `(affine F)` | `Zero One Fst Snd Nfst Nsnd Xor Xnor` | 8 | 8 | 8 |
| `(self-dual F)` | `Fst Snd Nfst Nsnd` | 4 | 4 | 4 |
| `(escapes-zero F)`, `-one`, `-monotone`, `-affine`, `-self-dual` | the outside of each class | 8, 8, 10, 8, 12 | the same | the same |
| `(complete-alone F)` | `Nand`, `Nor` | 2 | 2 | 2 |

**No disagreement between the three, on any of the thirty.**

### The hand counts, re-counted

Three of the stage file's were wrong, and its table is corrected, each change marked there:

| the stage file said | the rows say |
|---|---|
| `swap-of`: *six pairs* | **four** pairs, `Andn`/`Randn`, `Fst`/`Snd`, `Nfst`/`Nsnd`, `Orn`/`Rorn`, and eight functions their own swap. The six listed two of the four twice, once under their connective names |
| `dual-of`: `Andn`/`Rorn`, `Randn`/`Orn` | **`Andn`/`Orn`, `Randn`/`Rorn`**. The pairs written were the complements. The dual of *a and not b* is *a or not b* |
| *the sixteen are ten up to the order of the inputs* | **twelve**: four pairs and eight alone. Ten is how many read both inputs |

Every other count on the stage page stands: eight complement pairs, eight commutative, eight associative,
four idempotent, the reads, eight and eight preserving, six monotone, eight affine, four self-dual, two
complete alone. Two things the count adds: *idempotent* is exactly *preserves-zero and preserves-one*, the
same four functions by another name; and *identity* and *absorbing* are one-sided for eight functions each
and two-sided for four, which the stage file's one fact did not say.

### Which properties a monotone rule derives: all of them

The stage file expected a split, the properties that are *a finite conjunction of rows* against those stated
as *no row disagrees*, which would need `absent`. **Over a table that is complete there is no such split.** A
function has four rows and they are all written, so *for every input* is a conjunction of four premises:

```lisp
(rule commutative-r ()
  :match  (column ?f ?a ?b ?b ?d)
  :assert (commutative ?f)
  :why    "{?f} gives the same at 01 and at 10")
(rule dual ()
  :match  (and (column ?f ?a ?b ?c ?d) (inv ?a ?a1) (inv ?b ?b1) (inv ?c ?c1) (inv ?d ?d1) (column ?g ?d1 ?c1 ?b1 ?a1))
  :assert (dual-of ?f ?g)
  :why    "{?g} is {?f} with both inputs and the output negated")
(rule affine-r ()
  :match  (and (column ?f ?a ?b ?c ?d) (gate2 Xor ?a ?b ?x) (gate2 Xor ?c ?d ?x))
  :assert (affine ?f)
  :why    "{?f}'s four outputs have an even number of ones")
(rule escapes-monotone-r ()
  :match  (and (gate2 ?f ?a ?b 1) (le ?a ?c) (le ?b ?d) (gate2 ?f ?c ?d 0))
  :assert (escapes-monotone ?f)
  :why    "{?f} is 1 at {?a}{?b} and 0 at {?c}{?d}, which is not lower")
(rule complete-alone-r ()
  :match  (and (escapes-zero ?f) (escapes-one ?f) (escapes-monotone ?f) (escapes-affine ?f) (escapes-self-dual ?f))
  :assert (complete-alone ?f)
  :why    "{?f} is outside each of Post's five classes")
```

`affine` reads `Xor`'s own rows: the table is used to state a property of the table. Associativity is two
rules, one that an instance holds at three bits and one whose eight premises are the eight instances. The
negative side is monotone too, because *not in the class* is one witness: `Nand` is not monotone since it is 1
at 00 and 0 at 11. So `complete-alone` needs no negation. The program is one stratum, *monotone (the static
check)*, search-free.

**The `forall` and `absent` forms work too** (`p/post.ein`, `python3 post.py`, `out/post.txt`): eight
properties restated as *no row disagrees*, each derived and each equal to the monotone one.

```lisp
(rule commutative-fa ()
  :match  (and (column-name ?f ?n) (forall ?a (gate2 ?f ?a ?b ?x) (gate2 ?f ?b ?a ?x)))
  :assert (commutative-u ?f)
  :why    "every row of {?f} agrees with its swap")
(rule monotone-fa ()
  :match  (and (column-name ?f ?n) (forall ?a (and (gate2 ?f ?a ?b 1) (le ?a ?c) (le ?b ?d)) (gate2 ?f ?c ?d 1)))
  :assert (monotone-u ?f)
  :why    "no row of {?f} above a 1 is 0")
```

```text
commutative-u        8 agrees with the rows       self-dual-u          4 agrees with the rows
associative-u        8 agrees with the rows       preserves-zero-u     8 agrees with the rows
ignores-first-u      4 agrees with the rows       preserves-one-u      8 agrees with the rows
monotone-u           6 agrees with the rows       affine-u             8 agrees with the rows
```

Where `absent` earns its place is **a set of functions**, whose size is not fixed: *every member of S is
monotone* cannot be a rule of four premises.

So [F1b](../../followups/f1b_logical_formulation.md)'s *"are rules able to detect properties from facts?"*
has, on the smallest case, the answer **yes, and monotonically, when the facts are a closed table**; `absent`
is needed where the collection quantified over is open-ended.

### Post's criterion, checked three ways

A set of functions builds every function exactly when it has a member outside each of the five classes. The
stage file stated it from memory; it is checked here by enumeration, not against a source.

- **In Ein, for sets as data.** `(member S F)` facts; `(complete S)` by one monotone rule of ten premises,
  a member and its `escapes` fact for each class; `(inside S CLASS)` by `forall` over the members; and
  `(incomplete S)` from any `inside`. Written for 140 sets, the sixteen singletons, the 120 pairs and four
  named ones: **complete 58, incomplete 82, both 0, neither 0**. *Complete or inside a class, never both and
  never neither* is Post's theorem, observed on every set written.

  ```text
    and-or ['And', 'Or'] incomplete, inside monotone-class, one-preserving, zero-preserving
    and-or-not ['And', 'Or', 'Nfst'] complete
    imp-zero ['Rorn', 'Zero'] complete
    xor-and-one ['Xor', 'And', 'One'] complete
    s-Nand ['Nand'] complete
    s-Nor ['Nor'] complete
  complete pairs: 53 of 120
  ```

- **An independent enumeration, in Python.** For each of the 65 535 non-empty sets, the closure of the two
  projections under composition with the set's members, by brute force, complete when it reaches all sixteen
  columns. No class is consulted.
- **Against the criterion read from Ein's facts**, the five `escapes` relations as `ein saturate --dump`
  printed them:

  ```text
  all 65535 non-empty sets: brute-force closure agrees with Post's criterion over Ein's facts on 65535; complete: 64800
  minimal complete sets: 32 by size {1: 2, 2: 24, 3: 6, 4: 0, 5: 0}
  complete alone (closure): ['Nand', 'Nor']
  Ein on the 140 written sets: complete 58, incomplete 82, both 0, neither 0; agree with the closure: 140
  ```

So `Nand` and `Nor` are the two that suffice alone, by the facts and by the closure; 24 pairs and 6 triples
are complete with nothing to spare; and *is {Imp, Zero} complete?* is five lookups, as the stage file said.

### Three inputs

There are 256 functions of three inputs. Up to the order of the inputs they are 80; up to order and negating
inputs, 22; with the output's negation too, 14 (`python3 classes3.py`, `out/classes3.txt`; the same count
gives 12, 6 and 4 for two inputs, which is the corrected *twelve* above). It
was cheap in Python and was not done in Ein: a three-input function has no name here, and writing 256 of them
as facts is what the stage file said the module does not do.

## 3. A circuit as a program

Wires are names, a gate instance is a fact, and one rule propagates (`p/circuit.ein`):

```lisp
(relation gate Instance Function Wire Wire Wire :why "{?1} is a {?2} gate from {?3} and {?4} to {?5}")
(relation val Wire Bit :why "{?1} carries {?2}")
(rule propagate ()
  :match  (and (gate ?g ?f ?a ?b ?o) (val ?a ?x) (val ?b ?y) (gate2 ?f ?x ?y ?z))
  :assert (val ?o ?z)
  :why    "{?g}: {?f} of {?x} and {?y} is {?z}, on {?o}")
```

**Layout 2 needs one rule as well, and an activator a gate type** (`p/sim_rel.ein`): the function's name is a
value in the instance and a relation in the lookup, which a relation parameter allows.

```lisp
(rule propagates (?F)
  :match  (and (gate ?g ?F ?a ?b ?o) (val ?a ?x) (val ?b ?y) (?F ?x ?y ?z))
  :assert (val ?o ?z)
  :why    "{?g}: {?F} of {?x} and {?y} is {?z}, on {?o}")
(propagates Xor) (propagates And) (propagates Or)
```

Both give the same answer on the same circuit (application 1 below).

**The recommended layout is 1, the one table.** A function is then a value everywhere: a rule about *any*
function, every rule of § 2 and of § 6, matches `(gate2 ?f …)` with no parameter and no activator; `Xor`
appears as an argument in `affine-r`; and the synthesis of application 6 makes the function a hypothesis,
which a relation cannot be. Layout 2 would need an activator for each rule and each function, 16 × 31 facts
for § 2. Its one advantage is a shorter row. The two can coexist, since layout 2 is derivable from layout 1
by sixteen rules, but nothing in this prototype wanted it.

## 4. The binary full adder, from gates, against S1f.6.16's table

Five gates (`p/fa_gates.ein`):

```lisp
(gate x1 Xor a b ab)        ; a xor b
(gate x2 Xor ab cin sum)    ; the sum bit
(gate a1 And a b g1)        ; both inputs
(gate a2 And ab cin g2)     ; one input and the carry
(gate o1 Or g1 g2 cout)     ; the carry out
```

`p/fa_cases.ein` runs all eight inputs in **one saturation**: a value carries the test vector it belongs to,
`(at VECTOR WIRE BIT)`, and a rule compares each vector's two outputs with the row of `fa2`, the binary full
adder of [`binary_arithmetic.md`](binary_arithmetic.md)'s option (b), imported from that prototype's
`bits.ein`. `ein solve p/fa_cases.ein` (`out/fa_cases.txt`):

```text
  states (k_s)      1
  universe          no candidate: R is {∅} or empty, decided by root alone
  work              1 saturation · 0 facts beyond root · 2 layers

  query 1 of 3      (agrees ?v)
                      ?v = v000
                      ?v = v001
                      …
                      ?v = v111
  query 2 of 3      (differs ?v)
    class 1 of 1    1 state (1) — the goal has no row: undetermined here
  query 3 of 3      (and (vector ?v ?a ?b ?c) (at ?v sum ?s) (at ?v cout ?k))
                        “0 + 0 + 0: digit 0, carry 0”
                        “0 + 0 + 1: digit 1, carry 0”
                        “0 + 1 + 0: digit 1, carry 0”
                        “0 + 1 + 1: digit 0, carry 1”
                        “1 + 0 + 0: digit 1, carry 0”
                        “1 + 0 + 1: digit 0, carry 1”
                        “1 + 1 + 0: digit 0, carry 1”
                        “1 + 1 + 1: digit 1, carry 1”
```

**Eight of eight agree, none differs:** the calculator's own gate, derived from five smaller ones. 56
firings.

## 5. The six applications

### 1. Simulation: saturation, no search

`p/sim.ein`, the adder at a = 1, b = 1, carry in = 0 (`out/sim.txt`; layout 2's `out/sim_rel.txt` is the same
below its first line):

```text
  states (k_s)      1
  universe          no candidate: R is {∅} or empty, decided by root alone
  work              1 saturation · 0 facts beyond root · 2 layers
  query             (and (val sum ?s) (val cout ?c))
                      ?c = 1 · ?s = 0
                        “the sum bit is 0 and the carry out is 1”
```

### 2. The truth table of a circuit

**Without search, it is § 4's program:** the 2ⁿ inputs written as test vectors, one saturation. That is the
cheap way and the page says so before the other.

**With the inputs as hypotheses,** *an input has exactly one value* is two declarations of
[`std.algebra`](../../../stdlib/algebra.ein) over a typed relation (`p/inputs.ein`):

```lisp
(relation in Input Bit :why "the input {?1} is set to {?2}")
(is-a 0 Bit) (is-a 1 Bit)
(total-owed in is-a)     ; every input owes a value: a state with an input unset is not an answer
(functional in)          ; and none has two
(rule in-drives () :match (in ?w ?v) :assert (val ?w ?v) :why "the input {?w} is set to {?v}")
```

and a program says which wires are inputs, `(is-a a Input) (is-a b Input) (is-a cin Input)`. Three ways to
have the candidates were tried, and they give one answer:

| the candidates come from | file | universe | R | work |
|---|---|---|---|---|
| the obligations rung alone: what `total-owed` says the state owes | `tt_o.ein` | 6 candidates | 8 states | 30 saturations |
| an hrule, `(hrule set-input () :match (and (is-a ?w Input) (is-a ?v Bit)) :assert (in ?w ?v))`, named in `(solve :hrules (set-input))` | `tt_h.ein` | 6 | 8 states | 30 |
| `(solve :hypothesis-relations (in))`, a filter over the obligations rung | `tt_s.ein` | 6 | 8 states | 30 |

`ein solve p/tt_o.ein` (`out/tt_o.txt`):

```text
  states (k_s)      8
  commitments (k_c) 8
  exhausted         yes — every commitment is classified: this is all of R
  ∅                 not in R — root owes 3 instances
  minimal line      8 states, from 8 commitments — the least guess is 3 candidates
  N                 3 minimal dead sets
  lattice           monotone (the static check) — R is convex: no hole
  universe          6 candidates, 0 dead alone
  work              30 saturations · 171 facts beyond root · 5 layers

  query             (and (in a ?a) (in b ?b) (in cin ?c) (val sum ?s) (val cout ?k))
  over              all of R — 8 states → 8 projection classes
    class 1 of 8    1 state (1)
                      ?a = 0 · ?b = 0 · ?c = 0 · ?k = 0 · ?s = 0
                        “0 + 0 + 0: digit 0, carry 0”
    class 2 of 8    1 state (2)
                      ?a = 0 · ?b = 0 · ?c = 1 · ?k = 0 · ?s = 1
                        “0 + 0 + 1: digit 1, carry 0”
    …
    class 8 of 8    1 state (8)
                      ?a = 1 · ?b = 1 · ?c = 1 · ?k = 1 · ?s = 1
                        “1 + 1 + 1: digit 1, carry 1”
```

**R is the eight assignments, and the query's projection classes are the table's rows.** N is the three
pairs `{(in w 0), (in w 1)}`, refuted by `functional`. The 30 saturations are the 27 partial assignments, each
alive, and those three.

**What each half of *exactly one* does,** shown by leaving one out:

- **Without the obligation** (`tt_h_nt.ein`, the hrule alone): R is 27 states, every partial assignment, and
  the query has 9 classes, the eight rows and *19 states — the goal has no row: undetermined here*. `:over
  maximal` on the query gives the eight rows back. So the obligation is what makes R the table; without it
  the table is R's maximal line.
- **Without the obligation and without an hrule** (`tt_s_nt.ein`, the blind enumerator scoped to `in`): 1 190
  candidates, every pair of the program's names, and `ein solve` had not answered in 60 s. A relation's
  declared types do not narrow the blind enumerator; the obligation's guard does.

**One candidate an input, and *0* by `absent`, does not work.** It would make a commitment an assignment,
2ⁿ nodes for 3ⁿ. With the theory's `one-value` rule it is refused at load, which is *forbid* (M1e2 S1e2.2.5):

```text
rule 'one-value': concludes (false) from val, which rule 'not-raised' derives resting on (absent …) over hi, a relation
a commitment can still extend — so it calls a state that has not yet decided hi a contradiction, and a commitment would
lift it (dead is not upward-closed under absent; forbidden since M1e2 S1e2.2.5). …
```

(`p/tt_absent.ein`, exit 1.) With no rule that concludes `(false)` it loads, is *not monotone: 1 site*, and
is wrong in the way the read-out itself names (`p/tt_absent2.ein`, `out/tt_absent2.txt`): *stale facts — a
fact whose absent guard the node's own fixpoint falsifies*, `{(hi a)}, in R: (val a 0) ← not-raised (absent
(hi a))`, so a raised input carries both values and the table has rows such as *0 + 0 + 0: digit 0, carry 1*.
It also proposes `(hi X)` for every one of the program's 35 names. What a node is on such a program is
[F21](../../followups/f21_non_monotone_programs_decided.md)'s open question; until it is ruled, the two-candidate
encoding is the one that is right.

### 3. Satisfiability

The truth-table program with outputs required (`p/require.ein`: `(require WIRE BIT)`, and a rule that a wire
carrying the other value is `(false)`).

**Satisfiable,** `p/sat.ein`, *is there an input with sum 1 and carry 1?*:

```text
  states (k_s)      1
  minimal line      1 state, from 1 commitment — the least guess is 3 candidates
  N                 10 minimal dead sets
  universe          6 candidates, 0 dead alone
  work              30 saturations · 178 facts beyond root · 5 layers
    ;; state 1 of 1 — 3 candidates · minimal and maximal · from {(in a 1), (in b 1), (in cin 1)}
                        “a = 1, b = 1, carry in = 1”
```

**Unsatisfiable,** `p/unsat.ein`, the same with *and not both a and b* (`(require g1 0)`):

```text
  states (k_s)      0
  exhausted         yes — every commitment is classified: this is all of R
  empty             every alive node owes — no commitment pays every debt
  universe          6 candidates, 0 dead alone
  work              28 saturations · 156 facts beyond root · 4 layers

  owing             {(in a 0), (in b 0)} — owes 1
  owing             {(in a 0), (in b 1)} — owes 1
  …                 (eleven nodes, each two inputs set)
  query             (and (in a ?a) (in b ?b) (in cin ?c))
  over              all of R — R is empty: no state to answer from
```

- **Unsatisfiable reads as *empty: every alive node owes*, not as *every node is dead***, as
  [M1g's probe 1b](../../m1g_logic/README.md#first-probes-2026-11-11) found. Every full assignment is dead;
  the partial ones are alive and cannot be completed. `ein solve --single` says *Open — R is empty: every
  alive node owes*, *decided: yes*; `-n 1` says *0 states — none: R is empty*.
- **The cores say why, and they are in the dump, not in the read-out.** The read-out of an R that is empty
  this way prints the owing frontier and no N row. `--dump-answer DIR` writes `nogoods.json`, ten minimal dead
  sets, each with its core and its rule (`out/unsat_dump/nogoods.json`):

  ```text
  {"set": ["(in a 1)", "(in b 1)"], "core": ["(gate a1 And a b g1)", "(gate2 And 1 1 1)", "(in a 1)", "(in b 1)", "(require g1 0)"], "refuted_by": ["required"]}
  {"set": ["(in a 0)", "(in b 0)", "(in cin 0)"], "core": ["(gate x1 Xor a b ab)", "(gate x2 Xor ab cin sum)", "(gate2 Xor 0 0 0)", "(in a 0)", "(in b 0)", "(in cin 0)", "(require sum 1)"], "refuted_by": ["required"]}
  ```

  A core names the gates, the table rows and the requirement a refutation used: a clause, with its reason.
  The first is learned from two inputs, before the third is set.

### 4. Equivalence of two circuits

A miter: the two outputs go into an `Xor` that is required to be 1. `p/equiv.ein`, the adder's carry against
the majority form *or(and(a, b), and(cin, or(a, b)))*:

```text
  states (k_s)      0
  empty             every alive node owes — no commitment pays every debt
  work              30 saturations · 215 facts beyond root · 4 layers
```

**R is empty: no input tells them apart, so they are equivalent.** And two that are not, `p/inequiv.ein`,
the carry against *a or b*: R is where they differ.

```text
  states (k_s)      2
                        “at a = 0, b = 1, carry in = 0 the carry is 0 and a or b is 1”
                        “at a = 1, b = 0, carry in = 0 the carry is 0 and a or b is 1”
```

§ 4 is the same question answered without search, by running every vector.

### 5. A puzzle of truth and lies

`p/knights.ein`. *A says: "B is a knave." B says: "A and C are of the same kind." What is C?* A wire an
islander, 1 for a knight; a statement is a gate over those wires; and each speaker's wire is required to
equal the value of what they said.

```lisp
(is-a A Input) (is-a B Input) (is-a C Input)
(gate sa Nfst B B said-A)        ; "B is a knave": not B
(gate sb Xnor A C said-B)        ; "A and C are of the same kind"
(gate ka Xnor A said-A honest-A) ; A is a knight exactly when what A said is true
(gate kb Xnor B said-B honest-B)
(require honest-A 1) (require honest-B 1)
```

```text
  states (k_s)      2
  work              26 saturations · 139 facts beyond root · 5 layers

  query 1 of 2      (in C ?c)
  over              all of R — 2 states → 1 projection class
    class 1 of 1    2 states (1, 2)
                      ?c = 0
                        “C is 0 (1 a knight, 0 a knave)”

  query 2 of 2      (and (in A ?a) (in B ?b) (in C ?c))
  over              all of R — 2 states → 2 projection classes
                        “A = 0, B = 1, C = 0”
                        “A = 1, B = 0, C = 0”
```

**Two states and one projection class: C is a knave whoever of A and B lies.** The puzzle's answer is the
class count, which is the read-out as it is.

### 6. Synthesis of one missing gate

`p/synth.ein`. The adder's last gate is a hole, `(hole o1 g1 g2 cout)`; its type is the hypothesis, `(kind
o1 F)`, owed and functional as an input's value is; the eight test vectors are facts; and a state in which
some vector's carry is not the table's is `(false)`.

```text
  states (k_s)      2
  ∅                 not in R — root owes 1 instance
  N                 15 minimal dead sets
  universe          16 candidates, 14 dead alone
  work              18 saturations · 169 facts beyond root · 3 layers
    (model (kind o1 Or))
    (model (kind o1 Xor))
                        “the missing gate can be Or”
                        “the missing gate can be Xor”
```

**Two functions fit, `Or` and `Xor`,** because *both inputs* and *one input and the carry* are never 1
together: the adder's last gate may be either, which is true and is what a textbook's two drawings of the
adder show. Fourteen of the sixteen are *dead alone*.
The search is sixteen candidates and no input is guessed: the vectors are facts. The stage file's *fewer
gates up to `swap-of`* was not tried: with the hole's two wires fixed, a function and its swap are different
answers.

### Which are search-free, which are a search, and what a search costs

| application | search? | cost |
|---|---|---|
| 1 simulation | no | one saturation; a firing a gate |
| 2 truth table, from written vectors | no | one saturation; 2ⁿ vectors written, a firing a gate a vector |
| 2 truth table, inputs as hypotheses | yes | 3ⁿ + n saturations to describe R |
| 3 satisfiability | yes | the same to describe R; 2ⁿ to 2ⁿ⁺¹ saturations for one state or for *none* |
| 4 equivalence | yes, or no by vectors | as 3; or one saturation over 2ⁿ vectors |
| 5 knights and knaves | yes | as 3, n the islanders |
| 6 synthesis of one gate | yes, small | 16 candidates a hole, 18 saturations; the inputs are vectors, not guesses |
| § 2's properties, § 6's simplifier | no | one saturation |

### Scale: inputs 4 to 24

`python3 scale.py` and `scale2.py` (`out/scale.json`, `out/scale2.json`, every read-out under
`out/scale_*.txt`). n inputs and a chain of n − 1 gates over them, 2n candidates. Three programs: **tt**, a
parity chain, R the 2ⁿ assignments; **sat**, an `And` chain required to be 1, one satisfying input; **unsat**,
that and the first input's negation required too. The machine was shared with another agent's builds, so the
seconds are coarse; the saturation counts are exact.

**At the default budget** (`--region-budget` 2²⁰ facts beyond root). Saturations · seconds; ✂ where the run
stopped at the budget and said so:

| | n = 4 | 8 | 12 | 16 | 20 | 24 |
|---|---:|---:|---:|---:|---:|---:|
| tt, describe | 85 · 0.01 | 6 569 · 0.12 | 94 462 · 1.65 ✂ | 34 130 · 0.64 ✂ | 87 462 · 1.94 ✂ | 187 386 · 4.92 ✂ |
| tt, `-n 1` | 16 · 0.01 | 256 · 0.01 | 4 096 · 0.14 | 27 971 · 1.14 ✂ | 27 971 · 1.20 ✂ | 27 971 · 1.25 ✂ |
| sat, describe | 85 · 0.01 | 6 569 · 0.12 | 94 462 · 1.64 ✂ | 34 130 · 0.62 ✂ | 87 462 · 1.94 ✂ | 187 386 · 4.95 ✂ |
| sat, `-n 1` | 31 · 0.01 | 511 · 0.02 | 8 191 · 0.29 | 27 971 · 1.13 ✂ | 27 971 · 1.19 ✂ | 27 971 · 1.25 ✂ |
| unsat, describe | 58 · 0.01 | 4 382 · 0.11 | 72 835 · 1.29 ✂ | 30 039 · 0.55 ✂ | 78 987 · 1.73 ✂ | 172 159 · 4.52 ✂ |
| unsat, `-n 1` | 17 · 0.01 | 257 · 0.02 | 4 097 · 0.16 | 25 487 · 1.13 ✂ | 25 487 · 1.17 ✂ | 25 487 · 1.22 ✂ |

`--count` costs what describe costs, and `--single` what `-n 1` costs, one saturation more or less.

- **Describing R is 3ⁿ + n saturations:** 85, 6 569, and 531 453 at n = 12 below. A node is a partial
  assignment, each input unset, 0 or 1, and every one is alive.
- **One state is 2ⁿ saturations, and *none* is 2ⁿ + 1:** `-n 1` on the truth table takes 16, 256, 4 096; on
  the one-solution program 2ⁿ⁺¹ − 1; on the unsatisfiable one 2ⁿ + 1, and it then says *exhausted: yes —
  this is all of R*.
- **What bites first is the budget, at 12 inputs for describe and 16 for one state**, and it bites well: exit
  0, a note on stderr, and a row that says what was established, for instance at n = 12

  ```text
    states (k_s)      0   (a lower bound: complete up to states of 5 candidates)
    exhausted         no — the state route stopped before the states of 6 candidates: their 101 376 extensions project
                      1 115 136 facts on top of the 1 065 528 spent, past --region-budget 1048576
  ```

  A lower bound of 0 states is true and is no table. The commitments limit, `--region-limit`, never bites:
  the program is monotone, so no question needs the cube.

**With the budget raised to 2³⁰,** a limit of 150 s and of 8 GiB a run. Saturations · seconds · peak memory:

| | n = 10 | 12 | 14 | 16 | 20 | 24 |
|---|---:|---:|---:|---:|---:|---:|
| tt, describe | 59 059 · 1.1 · 76 MiB | 531 453 · 11.7 · 572 MiB | 4 782 983 · 122 · 5.1 GiB, k_s 16 384 | no answer in 150 s, 8 GiB | | |
| sat, describe | 59 059 · 1.2 · 70 MiB | 531 453 · 14.6 · 547 MiB | no answer in 150 s, 5.2 GiB | | | |
| unsat, describe | 39 376 · 0.9 · 54 MiB | 354 306 · 8.9 · 335 MiB | 3 188 660 · 112 · 3.1 GiB | no answer in 150 s, 7.6 GiB | | |
| tt, `-n 1` | | | | 65 536 · 2.9 · 100 MiB | 1 048 576 · 57 · 1.4 GiB | no answer in 150 s |
| sat, `-n 1` | | | | 131 071 · 7.6 · 149 MiB | no answer in 150 s | |
| unsat, `-n 1` | | | | 65 537 · 3.4 · 92 MiB | no answer in 150 s | |

**So *small* is: a truth table by search to 12 inputs in seconds and 14 in two minutes and 5 GiB; one
satisfying input, or the answer that there is none, to 16 inputs in seconds, and 20 not within the limit
except where the first assignment tried is a state.** Memory grows with the nodes, and at 16 inputs
describing R passed 8 GiB.

**The same truth table from written vectors,** one saturation (`scale.py`'s last block, `ein saturate`):

| n | vectors | firings | saturation | parse | the program's text |
|---:|---:|---:|---:|---:|---:|
| 4 | 16 | 64 | 0.14 ms | 3 ms | 1.7 KB |
| 8 | 256 | 2 048 | 2.4 ms | 13 ms | 42 KB |
| 12 | 4 096 | 49 152 | 64 ms | 290 ms | 1.2 MB |
| 16 | 65 536 | 1 048 576 | 2.3 s | 6.4 s | 30 MB |

n · 2ⁿ firings, against 3ⁿ saturations: at 12 inputs, 64 ms against 11.7 s; at 16, 2.3 s, and 6.4 s to parse
the vectors, against no answer. **Where every input is to be tried, writing the inputs beats guessing
them.** Search earns its place where the answer is a few states and the reasons, as in the puzzle and the
synthesis.

## 6. What a simplifier does with the property facts

`p/simplify.ein`: six gates and three laws, each a rule that reads a property and names no function.

```lisp
(rule drop-identity-right ()
  :match  (and (gate ?g ?f ?a ?c ?o) (const ?c ?e) (identity-of ?f ?e))
  :assert (same-wire ?o ?a)
  :why    "{?e} is neutral for {?f}: {?g}'s output is its other input, {?a}")
(rule constant-by-absorbing ()
  :match  (and (gate ?g ?f ?a ?c ?o) (const ?c ?e) (absorbing-of ?f ?e))
  :assert (const ?o ?e)
  :why    "{?e} decides {?f}: {?g}'s output is the constant {?e}")
(rule normal-swapped ()
  :match  (and (gate ?g ?f ?b ?a ?o) (before ?a ?b) (commutative ?f))
  :assert (normal ?g ?f ?a ?b ?o)
  :why    "{?f} is commutative: {?g}'s inputs are put in order")
```

On `(gate g1 And x one p)`, `(gate g2 Or y one q)`, `(gate g3 Xor a b r)`, `(gate g4 Xor b a s)`, `(gate g5
Andn a b t)`, `(gate g6 Andn b a u)`, with `(const one 1)` and `(before a b)` (`out/simplify.txt`, 72
firings, search-free; the property facts are derived in the same saturation by § 2's rules):

```text
  query 1 of 3      (same-wire ?o ?w)
                        “p is x”
                        “r is s”
                        “s is r”
  query 2 of 3      (const ?w ?e)
                        “one is the constant 1”
                        “q is the constant 1”
  query 3 of 3      (same-gate ?g ?h)
                      ?g = g3 · ?h = g4
                      ?g = g4 · ?h = g3
```

*x and 1* is *x*; *y or 1* is 1; the two `Xor` gates are one gate. The two `Andn` gates are **not** merged,
rightly: `Andn` is not commutative, and no rule said it by name. De Morgan's laws would be the same shape
over `(dual-of And Or)` and `(complement-of And Nand)`; they were not written. A simplifier that *rewrites*,
removing `g4` and re-pointing what read `s`, needs retraction or a new circuit as its output, which is
[M1g's normal forms](../../m1g_logic/p1g.2_propositional_logic/s1g.2.3_normal_forms.md)' subject; this one
derives what is equal and stops.

**On `std.algebra`'s names.** The stage file asks whether these properties should be `std.algebra`'s
(`symmetric`, `functional`, …) applied to a gate's table read as a relation. They should stay apart.
`std.algebra`'s `symmetric` is a **rule that makes** a relation symmetric, activated by a fact, and its
`functional` **refutes** a relation that is not; `(commutative Xor)` here is a **finding about** a table that
is already complete. Activating `(symmetric R)` on a table that is not symmetric would add rows to it. The
same word for *impose* and for *observe* would be a trap.

## What did not work, and findings

1. **One candidate an input with `absent`** is refused by *forbid* or, without a refutation, gives stale
   facts and wrong rows; § 5.2. Not a new finding: it is F21's question, with a small clean witness.
2. **A module that builds on another, imported by its own entry alone, is half a theory, in silence.**
   `inputs.ein` imports `circuit.ein`; a program that imported `inputs` got `in-drives` and the *declaration*
   of `val`, and not `propagate`: R was the eight assignments and every row of the table read *the goal has
   no row: undetermined here* (`out/tt_o_half_theory.txt`). The repair is a fact in `inputs`' closing rule
   that names `circuit`'s entry. [`binary_arithmetic.md`](binary_arithmetic.md) met the same thing, and
   [S1f.6.20](s1f.6.20_word_problems.md) filed it first: E-20 of [`findings.md`](findings.md), with a note.
3. **An unsatisfiable program answers `Open` under `--single`,** with *decided: yes*. The word is right by
   its definition, an alive node owes, and it is not what a reader of a satisfiability question expects:
   every total assignment is dead and the run knows it. E-23.
4. **The read-out of an R that is empty by owing prints no N row,** though the run has N, ten sets here, and
   `--dump-answer` writes it. For satisfiability N is the explanation. E-23 too; whether the read-out
   should carry the row is [`read_out.md`](../../../docs/kernel/inference/read_out.md)'s.
5. **The blind enumerator is not narrowed by a relation's declared types:** 1 190 candidates for `in Input
   Bit` over three inputs and two bits. Known, [E-1](findings.md) and [E-7](findings.md)'s family; the
   obligation is what narrows.

## Names: which failed

**None.** The stage file's sixteen proposed gate names load as written, as values and as relations, with the
connective names and `Not`. Nothing was renamed for the prototype. The lower-case `and`, `or` and `not` are
refused at the parse in a value's place, as expected.

## Questions for the user

1. **The layout.** One table, `(gate2 NAME A B OUT)`, is recommended (§ 3). Agreed, or both?
2. **The names.** The sixteen as proposed; `gate2` for the table; `f0110` as the second name, held in
   `(column-name Xor f0110)`. Any to change? `Randn` and `Rorn` are the least pronounceable.
3. **Which property facts ship, and how.** All thirty derive from the rows by rules in 405 firings. Shipped
   as **rules** they cost that on every load and can never disagree with the rows; shipped as **generated
   facts** they cost nothing at run time and a test holds them to the rows. The recommendation: generated
   facts, with the rules kept as the test that derives every one.
4. **The one-sided facts.** `identity-of` and `absorbing-of` as the stage file has them hold of four
   functions each; the left and right forms of eight each. Are the one-sided ones wanted (`Andn` has a right
   identity, 0, and no left one)?
5. **The corrections** to the stage file's hand counts, § 2: four swap pairs, the two dual pairs, twelve up
   to input order. Confirm.
6. **Unsatisfiable as `Open`.** Is *empty: every alive node owes* the wanted reading of *no input satisfies
   it*, or should a program be able to say that a state with a debt no extension can pay is dead? It is the
   same question as M1g's probe 1b.
7. **The search's size.** 3ⁿ for a truth table and 2ⁿ for one satisfying input is what the engine's routes
   cost on this encoding; a truth table of written vectors is one saturation. Is the search form wanted as
   more than a demonstration, that is, is a cheaper route for *exactly one of two candidates per object* a
   requirement for [M1g](../../m1g_logic/README.md)'s valuations?
8. **What ships from this stage:** the table and the properties; the circuit theory (`gate`, `val`,
   `propagate`, `one-value`); the inputs theory (`in`, owed and functional) and `require`; and which of the
   six applications as examples. The recommendation: all four modules small, and the six as
   `examples/logic/`, the adder against the table as a test.
