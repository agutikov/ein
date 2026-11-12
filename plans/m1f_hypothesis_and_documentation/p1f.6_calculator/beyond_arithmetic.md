# Beyond arithmetic: inverse questions, search on the tables, bounded theorems and number theory

**Stage:** [S1f.6.10](s1f.6.10_beyond_arithmetic.md). **Phase:** [P1f.6](README.md).
**Status:** an investigation, 2026-11-12. No stdlib module and no engine source was changed. Sixteen of the probes
are banked as programs under [`examples/beyond_arith/`](../../../examples/beyond_arith/); the rest are under
`build/s1f.6.10/` (gitignored) with their numbers here. It is shown to the user, who rules what becomes a stage.

The user, 2026-11-08: *"investigation of what can be done with this theory except simple arithmetic, can it be
used for any kind of theorems proving, or solving equations, or numbers theory exploration, or anything else"*.

**The answer, measured.**

- **Equations.** A table is a relation, so a rule reads it in any direction, and an equation with one unknown is
  solved **by saturation alone**, a firing a column, where the column has one answer (x + 4567 = 127645, x × 7 =
  861). Where a column has two, saturation derives both walks and the next column ends the wrong one, still with
  no hypothesis. With the unknown digits as **hypotheses** the same equations are a search: `ein solve --single`
  finds x and shows there is no other in 10n + 1 saturations for n digits when the constraint is written a
  column at a time, and in about 10ⁿ when it is written as one whole-number check through `std.arith`.
- **Cryptarithms.** SEND + MORE = MONEY is solved and shown unique in 0.5 s, 17 747 saturations, by `--single`.
  `zebra.ein` takes 49. The same program with its letters declared in another order had no answer in 600 s.
- **An equation with no solution** reads three ways, by how it is asked: silence (saturation, several answers a
  column), an R that is empty *because every alive node owes* (search), or a **dead root whose core is the
  proof**, three table rows (saturation, one answer a column).
- **Theorems.** Ein checks a law on every instance of a finite domain, and a table row by row. It cannot say
  *for every number*, and it cannot state an induction. So of *addition is right at every length* Ein checks the
  200 rows of the full adder, once, and the induction on columns is an argument made outside it.
- **Number theory** by saturation works as far as memory goes: primes to 5 000, casting out nines to 9 999,
  perfect numbers to 250, the Collatz trajectory of 837 799, gcd of 31-digit numbers, F(500), each agreeing with
  Python. Every step is stored, a kilobyte a fact, and that is the limit (perfect numbers to 500 ended when the
  machine's 32 GB did).
- **What does not scale** is describing R where the answer is found by guessing: a bare `ein solve` visits every
  partial assignment that is still alive. So `ein test`, which describes, cannot check a claim on SEND + MORE
  that `--single` decides in half a second.

Every number is from a run of 2026-11-12 on the release binary (`./build.sh --engine`), with the command that
re-takes it; a count set beside Python's says so. The seconds were taken on a 32-core machine, and those of § 2's and § 3's tables while
other runs of this stage shared it; the firings and saturations are exact. `ein oracle` agrees wherever it can
reach, which is 20 candidates.

**Where things are.** `build/s1f.6.10/`: `gen1b.py`, `gen1c.py` (§ 1), `gen2.py`, `run2.py` (§ 2), `gen3.py`
(§ 3), `gen4.py`, `gen4t.py` (§ 4), `gen5.py`, `run5.py` (§ 5), `gen7.py`, `run7.py` (§ 7), `t.py` (wall and
peak memory of one command), `bank.py` (wrote the banked programs, each with the claim the engine printed and a
Python check of it); the programs under `p/`, the read-outs under `out/`. Commands are from the repo root, and
`E` is `ein.rs/target/release/ein`.

## 1. Tables run backwards

A table is a relation, not a function. `std.arith` reads the full adder `(fa A B CARRY-IN DIGIT CARRY-OUT)` by
its two operands. A rule may as well read it by one operand, the carry and the **sum's** digit:

```lisp
(rule un-column ()
  :match  (and (un-walk ?t (n ?py ?b) (n ?ps ?d) ?c ?acc) (fa ?a ?b ?c ?d ?k))
  :assert (un-walk ?t ?py ?ps ?k (b ?a ?acc))
  :why    "what plus {?b} plus carry {?c} ends in {?d}? {?a}: write {?a}, carry {?k}")
```

### 1a. x + 4567 = 127645: saturation alone, all of it

[`examples/beyond_arith/inverse_addition.ein`](../../../examples/beyond_arith/inverse_addition.ein), eleven rules
over `fa` and `nz`, no import of `std.arith`. Three requests, 33 firings together (`$E saturate
examples/beyond_arith/inverse_addition.ein`):

```text
“x + (n (n (n (n z 4) 5) 6) 7) = (n (n (n (n (n (n z 1) 2) 7) 6) 4) 5) for x = (n (n (n (n (n (n z 1) 2) 3) 0) 7) 8)”
“x + (n (n (n (n z 4) 5) 6) 7) = (n (n (n (n z 4) 5) 7) 0) for x = (n z 3)”
“no natural x makes x + (n (n (n (n z 4) 5) 6) 7) = (n (n (n z 9) 9) 9)”
```

**No column needs a guess.** Read by an operand's digit, the carry in and the sum's digit, the full adder has
exactly one row, so the walk from the units is a straight line: it is subtraction, found in the adder's table.
The universe has no candidate and R is {∅}. *No natural x* is derived as a fact, by a rule that matches the
walk's end with digits of the known operand still unread, so here even the negative answer is positive.

### 1b. Where a column has two answers, saturation carries both

[`examples/beyond_arith/tables_backwards.ein`](../../../examples/beyond_arith/tables_backwards.ein): x × M = P
over the digit multiplier's table `md`, read by the multiplier, the carry and the product's digit; and x + x = S
over `fa` with both operands the same variable, `(fa ?a ?a ?c ?d ?k)`.

- **By 7** (and 1, 3, 9) a column has one answer: x × 7 = 861 is three firings of the column rule, x = 123.
- **By 4** it has two, five apart, with different carries: at the units of x × 4 = 3284, 1 (carry 0) or 6 (carry
  2). **This is the carry a column cannot decide.** Nothing chooses: the rule fires for both rows, and the walk is
  now two facts. At the next column each of them again has two rows or none, by the parity of its carry, and the
  wrong ones end there: no row fits. x = 821.
- **x + x = S** is the same: a digit or the digit five above it, and the next column's parity decides.

Six requests, 84 firings (`$E saturate examples/beyond_arith/tables_backwards.ein`): 123, 821, 12345 (x × 5 =
61725), 1234 (x + x = 2468), and no answer fact for x × 4 = 3285 and x + x = 1357.

**The walks alive stay few**, so the cost is linear in the length (`python3 build/s1f.6.10/gen1b.py`; x a random
number of that many digits, the answer checked against it):

| digits of x | × 7: firings · walk facts | × 2 | × 4 | × 5 | x + x |
|---:|---:|---:|---:|---:|---:|
| 4 | 14 · 6 | 19 · 11 | 22 · 15 | 34 · 26 | 19 · 11 |
| 8 | 22 · 10 | 31 · 19 | 47 · 35 | 58 · 46 | 28 · 17 |
| 12 | 30 · 14 | 43 · 27 | 67 · 51 | 82 · 66 | 43 · 27 |
| 16 | 38 · 18 | 52 · 33 | 87 · 67 | 106 · 86 | 55 · 35 |
| 20 | 46 · 22 | 67 · 43 | 107 · 83 | 130 · 106 | 67 · 43 |

**What saturation cannot say here is *none*.** x × 4 = 3285 is answered by silence: every walk ended, and *no
walk ended well* is an absence, which a rule cannot match. § 6 has the two ways to get a *no* that is said.

### 1c. Two unknowns: saturation lists every solution, and pays for every prefix

`build/s1f.6.10/p/p1c_split.ein` (three rules): which x and y make x + y = S? `(fa ?a ?b ?c ?d ?k)` with only the
carry and the sum's digit bound has ten rows. Every pair is derived, and the count agrees with S + 1 (`python3
build/s1f.6.10/gen1c.py`, each derived pair's sum checked in Python):

| S | pairs, by Ein | S + 1 | firings | walk facts | saturate |
|---:|---:|---:|---:|---:|---:|
| 12 | 13 | 13 | 124 | 113 | 0.2 ms |
| 999 | 1 000 | 1 000 | 2 111 | 1 113 | 2.3 ms |
| 5000 | 5 001 | 5 001 | 16 112 | 11 113 | 19 ms |
| 20000 | 20 001 | 20 001 | 131 112 | 111 113 | 184 ms |

The walk has 10ⁿ partial pairs for n digits whatever S is: 113 walk facts for the 13 pairs of 12. An
underdetermined equation is an enumeration, and saturation is a breadth-first one.

## 2. Equations by search

The unknown's digits as hypotheses. The theory half is the one [`binary_logic.md`](binary_logic.md) § 5 used for
a circuit's inputs: an hrule proposes every digit at every place, `total-owed` says a state with a place unset is
no answer, `functional` that a place has one digit.

```lisp
(total-owed dig is-a)
(functional dig)
(hrule guess-digit ()
  :match  (and (is-a ?p Place) (is-a ?d Digit))
  :assert (dig ?p ?d)
  :why    "suppose the digit of x at {?p} is {?d}")
(solve :hrules (guess-digit))
```

Two ways to write the constraint (`build/s1f.6.10/gen2.py` writes both):

- **A, the whole number through `std.arith`.** Once every digit is set a rule asks the product, and a product
  that is not the wanted one is `(false)`. The circuit checks; nothing is refuted before the last digit.
- **B, a column at a time over `md`.** A digit is checked as soon as the digits below it are set, so a wrong
  units digit is dead alone.

### x × 7 = 861

[`examples/beyond_arith/equation_by_search.ein`](../../../examples/beyond_arith/equation_by_search.ein) is B at
three digits:

```text
  asked             is R one state? (--single)
  verdict           Solution — R is one state
  decided           yes — R holds no other state
  universe          30 candidates
  route             the obligation tree, over states …
  work              31 saturations · 147 facts beyond root
  (model (dig p0 3) (dig p1 2) (dig p2 1))
```

and a bare `ein solve` describes R: one state, `N 117 minimal dead sets`, `universe 30 candidates, 9 dead alone`,
`work 251 saturations`.

**By the number of digits** (`python3 build/s1f.6.10/run2.py`; x random, x × 7 written on x's places; candidates
are 10 a digit; saturations · seconds; ✂ where the run stopped at the default `--region-budget` and said so):

| digits | encoding | describe R | N | `-n 1` | `--single` |
|---:|---|---:|---:|---:|---:|
| 1 | A | 11 · 0.04 | 9 | 4 | 11 |
| 1 | B | 11 · 0.02 | 9 | 4 | 11 |
| 2 | A | 211 · 0.05 | 189 | 24 | 120 |
| 2 | B | 76 · 0.02 | 63 | 14 | 21 |
| 3 | A | 1 466 · 0.10 | 1 134 | 324 | 1 129 |
| 3 | B | 251 · 0.03 | 117 | 24 | 31 |
| 4 | A | 14 821 · 1.18 | 10 179 | 9 034 · 0.56 | 11 138 · 0.69 |
| 4 | B | 1 636 · 0.09 | 171 | 34 | 41 |
| 5 | A | 161 277 · 26.6 ✂ | ≥ 100 224 | 41 574 · 2.5 ✂ | 41 574 · 2.5 ✂, *not decided* |
| 5 | B | 16 331 · 5.1 | 225 | 44 | 51 |
| 6 | B | no answer in 300 s | | 54 | 61 |

- **`--single` on B is 10n + 1 saturations.** The obligation tree branches on the units place first, nine of its
  ten alternatives die at once, and so on up: the search is the school method, with the wrong digits tried.
- **`--single` on A is about 10ⁿ**: every full assignment is its own refutation. At five digits it stops at the
  budget, *not decided*, exit 0.
- **Describing R is exponential in both**, because R's description visits every alive node and a partial
  assignment that sets only upper digits is alive: 11ⁿ⁻¹ and more nodes for B, 11ⁿ for A. B at six digits had
  not answered in 300 s, where `--single` takes 61 saturations; and most of describe's time is not saturation
  (E-31).

**The oracle agrees** where it reaches (`$E oracle --region-limit 20 -j auto FILE`): at one digit, 1 024
saturations for each encoding, the same state and the same N of 9; at two digits, 1 048 576 saturations each, the same one state and the same N,
63 for B (11.1 s) and 189 for A (58.9 s, 3.1 GiB), and *0 dead nodes with an alive superset*.

### A linear equation with two unknowns: 3x + 5y = 47

[`examples/beyond_arith/equation_two_unknowns.ein`](../../../examples/beyond_arith/equation_two_unknowns.ein): x
below 100 and y below 10, three unknown digits, encoding A. Three rules of the program ask `std.arith` for 3x, 5y
and their sum and refute a sum that is not 47. **Search and the search-free theory meet here: every guess is
checked by a computation that guesses nothing.**

```text
  states (k_s)      3
  N                 1 132 minimal dead sets
  universe          30 candidates, 0 dead alone
  work              1 466 saturations · 60 223 facts beyond root · 5 layers
                        “x = 04, y = 7”
                        “x = 14, y = 1”
                        “x = 09, y = 4”
```

0.14 s. Python's enumeration gives the same three, (4, 7), (9, 4), (14, 1). Of the 1 000 full assignments 997 are
dead, each found separately; N is those and the 135 pairs `functional` refutes.

### An equation with no solution: x × 7 = 862

Three digits, both encodings (`build/s1f.6.10/p/p2a_862.ein`, `p2b_862.ein`; `out/` has the read-outs and
`out/p2b_862.dump/nogoods.json`):

```text
  states (k_s)      0
  exhausted         yes — every commitment is classified: this is all of R
  empty             every alive node owes — no commitment pays every debt
  universe          30 candidates, 9 dead alone
  work              251 saturations · 668 facts beyond root · 4 layers

  owing             {(dig p0 6), (dig p1 6)} — owes 1
                      dig owes p2 a Digit.
  …                 (111 nodes)
```

- **R is empty by owing, not by death.** Every full assignment is dead; the partial ones that are still alive
  cannot be completed. `--single` answers *`Open` — R is empty: every alive node owes*, *decided: yes*, in 31
  saturations; `-n 1` says *0 states — none: R is empty, every alive node owes*, *exhausted: yes*. It is E-23 of
  [`findings.md`](findings.md) again, on an equation: the run has shown *never* and the word reads as *not yet*.
- **The cores are the proof, and they are in the dump and the trace, not in the read-out.** B's N is 118 sets:
  99 pairs that `functional` or the tens column refutes, and the 28 that are the argument, which reads from the
  units up:

  ```text
  {(dig p0 0)}                         core (dig p0 0) (md 0 7 0 0 0) (times 7) (want p0 2)      refuted by col-wrong
  …   nine units digits, each dead alone: the units digit must be 6
  {(dig p0 6), (dig p1 0)} …           nine tens digits beside 6, each dead: the tens digit must be 6
  {(dig p0 6), (dig p1 6), (dig p2 2)} core … (md 2 7 4 8 1) (md 6 7 0 2 4) (md 6 7 4 6 4) (nz 1) (top p2)
                                                                                                  refuted by carry-over-top
  …   ten hundreds digits beside 6 and 6, each dead
  ```

  Each core names the rows of `md` the refutation passed through. Encoding A's N is 1 135 sets, one for each of
  the 1 000 full assignments: the same fact with no argument in it.
- **The oracle agrees** on the two-digit instance, x × 7 = 62
  ([`equation_no_solution.ein`](../../../examples/beyond_arith/equation_no_solution.ein)): 1 048 576
  saturations, 13.2 s, `states 0`, *every alive node owes*. Its claim is the owing frontier, `(solve … :expect
  (region (owing (model (dig p0 6)) (model (dig p1 0)) …)))`, eleven nodes, since a query's `:expect (false)`
  holds of an R that is empty either way (E-18).

## 3. Cryptarithms

`build/s1f.6.10/gen3.py NAME A B C` writes A + B = C: a letter a digit (`val`), `total-owed`, `functional` and
`injective` from `std.algebra`, a leading letter not zero, and each column through `fa` as soon as its letters
and the carry below it are known:

```lisp
(rule add-column ()
  :match  (and (column ?c ?x ?y ?z) (carry-in ?c ?i) (val ?x ?a) (val ?y ?b) (fa ?a ?b ?i ?d ?k))
  :assert (col ?c ?d ?k)
  :why    "{?x} + {?y}: {?a} + {?b} + carry {?i} is digit {?d}, carry {?k}")
(rule column-wrong ()
  :match  (and (col ?c ?d ?k) (column ?c ?x ?y ?z) (val ?z ?v) (neq ?v ?d))
  :assert (false)
  :why    "column {?c} adds up to {?d}, and {?z} is {?v}")
```

**SEND + MORE = MONEY** (`build/s1f.6.10/p/p3_send.ein`, 80 candidates; `python3 build/s1f.6.10/t.py OUT $E solve
build/s1f.6.10/p/p3_send.ein --single`):

```text
  verdict           Solution — R is one state
  decided           yes — R holds no other state
  universe          80 candidates
  work              17 747 saturations · 222 924 facts beyond root
                        “9567 + 1085 = 10652”
```

0.50 s, 29 MiB. Python's enumeration of the 1 814 400 assignments finds the same one solution (`python3
build/s1f.6.10/gen3.py --brute SEND MORE MONEY`).

| program | candidates | `--single` | `-n 1` | describe R |
|---|---:|---|---|---|
| `examples/zebra.ein` | 56 | 49 saturations · 0.17 s | | 445 · 0.90 s |
| `examples/zebra2.ein` | 56 | 45 · 0.11 s | | 374 · 0.52 s |
| SEND + MORE = MONEY, letters declared by column (D E Y N R O S M) | 80 | 17 747 · 0.50 s | 15 854 · 0.41 s | stops at the budget: 336 101 saturations, 7.4 s, 519 MiB, *complete up to states of 3 candidates* |
| the same, letters declared as written (S E N D M O R Y) | 80 | *not decided* at the default budget (131 134 saturations, 3.1 s); no answer in 600 s at `--region-budget 67108864` | | |
| the same by column, with propagation rules (below) | 80 | 639 · 2.10 s | 428 · 1.23 s | |
| the same as written, with propagation rules | 80 | 5 267 · 19.4 s | 3 071 · 10.0 s | |
| TWO + TWO = FOUR | 60 | `Ambiguity` · 1 554 · 0.05 s | `-n 10`: 7 states, *all of them*, 1 878 · 0.07 s | 7 states, N 2 092, 67 046 · 32.9 s |
| I + BB = ILL ([`cryptarithm.ein`](../../../examples/beyond_arith/cryptarithm.ein)) | 30 | | | 1 state, 1 + 99 = 100, N 224, 417 · 0.04 s |

What the table says:

- **It is Ein's home ground, and it is 360 times `zebra.ein`'s search.** The zebra programs carry the stdlib's
  elimination rules, which decide most of the grid at root (18 and 21 of 56 candidates are dead alone). The
  cryptarithm's rules only **check**: a column says nothing until its three letters and the carry below it are
  all known. So the search is generate-and-test with early failure, and 17 747 nodes is its size.
- **Propagation rules buy nodes and cost time.** Reading `fa` backwards inside the search, so that two known
  letters of a column give the third, and deriving that a carry which begins a number is 1, cuts the tree to 639
  nodes, 28 times fewer, and takes four times as long: a node costs 3.3 ms where it cost 28 µs. The profile is
  the matcher (`perf record`: `Matcher::unify`, `try_candidate`, `walk`). Why a node of that program costs a
  hundred times more was not investigated here; E-12, a table premise keyed on a column that does not select,
  is the first thing to rule out.
- **The order the letters are declared in decides whether there is an answer.** The obligation tree branches on
  the owed instances in the order `total-owed` reports them, which is the order of the `(is-a L Letter)` facts.
  Declared by column, the units column's letters are guessed first and a wrong sum dies after three guesses.
  Declared as the puzzle is written, S is guessed first and nothing can be checked before D, E and Y. Same
  program, same R, 0.5 s against no answer in 600 s. **Finding E-29.**
- **Describing R does not scale to it.** A partial assignment that leaves a letter of every column unset is
  alive, and describe visits each: SEND + MORE stops at the budget having found no state. TWO + TWO = FOUR, 60
  candidates, is described whole in 32.9 s, 7 states, the same seven as Python's.
- **So `ein test` cannot check the claim.** `build/s1f.6.10/p/p3_send_expect.ein` is the program with `:expect
  (model (val D 7) … (val Y 2))`: *NOT CHECKED — the claim is about all of R, and this run did not exhaust it*,
  exit 1, 7.5 s; `--single` had decided the same thing, *R holds no other state*, in 0.5 s. **Finding E-30.** It
  is why the corpus holds I + BB = ILL and not SEND + MORE.
- **The oracle agrees** on two 20-candidate cryptarithms (`$E oracle --region-limit 20 -j auto`): A + A = B, 4
  states and N 81, in 1 048 576 saturations against describe's 99; and AB + BA = AB, which has no solution, `0
  states`, *every alive node owes*, against describe's 174.

## 4. Bounded theorem checking

### 4a. The laws, on every instance of a small domain

[`examples/beyond_arith/laws.ein`](../../../examples/beyond_arith/laws.ein) checks commutativity and
associativity of + and ×, distributivity, and the division identity x = q·y + r with r < y. For each instance
`std.arith` computes both sides; `holds2`/`holds3` says they agree and `fails2`/`fails3` that they do not. The
claims are `(query :goal (fails2 ?law ?x ?y) :expect (model))` and the same of `fails3`: nothing fails. The banked
file's domain is 0 to 3; the runs below are `python3 build/s1f.6.10/gen4t.py NAME W [LAWS] [DW]`, then `$E
saturate --dump`, the `holds` facts counted from the dump:

| domain | instances that hold · fail | firings | facts | saturate |
|---|---|---:|---:|---:|
| 0 to 9: pairs, triples; division of 00 to 99 by 1 to 9 | commutes +, ×: 100, 100 · 0; associates +, ×: 1 000, 1 000 · 0; distributes: 1 000 · 0; division identity: 900 · 0 | 271 266 | 274 346 | 0.42 s |
| 00 to 99, pairs only | commutes +, ×: 10 000, 10 000 · 0 | 867 579 | | 1.3 s, 903 MiB |
| division of 000 to 999 by 1 to 9 | division identity: 9 000 · 0 | 1 492 338 | | 2.7 s, 1.7 GiB |

Every instance holds, and R is {∅} with no candidate: it is a computation, and `ein oracle` has nothing to add
to one saturation. **It is a proof for that domain**, in the sense that every case was computed and compared. It
is not a proof of the law: that needs *for every number*, which no Ein rule can say, since a rule's variables
range over the facts a state holds.

**How the check is written decides its cost, by a factor of 40.** The first version (`gen4.py`) used the public
protocol and ordinary joins: `(and (d ?x) (d ?y) (sum-of ?x ?y ?s) (sum-of ?y ?x ?t) (neq ?s ?t))`. Over 0 to 9
it took **16.8 s** for 146 861 firings: a premise with an unbound slot beside a bound number walks its relation
(E-3). The banked version is written the way the theory is: each check is a chain of the theory's **tagged
calls**, one premise a rule, the earlier answer carried in the tag,

```lisp
(rule commute-add-back ()
  :match  (done (ca ?x ?y) ?s)
  :assert (add-walk (cb ?x ?y ?s) ?y ?x 0 z)
  :why    "{?x} + {?y} is {?s}: now ask {?y} + {?x}")
(rule add-commutes ()
  :match  (done (cb ?x ?y ?s) ?s)
  :assert (holds2 add-commutes ?x ?y)
  :why    "{?x} + {?y} and {?y} + {?x} are both {?s}")
```

and takes **0.42 s** for 271 266 firings: more firings, since a tagged call is not shared between instances, and
40 times less time. The rule of the theory, *no rule joins two facts on a number*, is a rule for its users too.
It also means a user's program calls `add-walk`, `mul-call`, `div-call`, `cmp-walk`, `done`, `div-done` and
`order-is`, which the module's header lists as *the circuit's own*: question 4.

### 4b. The column lemma: exactly which part Ein checks

*Addition by the walk is right at every length.* The proof has two parts.

1. **Every row of the full adder is right:** for digits a, b and a carry c of 0 or 1, the row's digit d and carry
   k satisfy a + b + c = 10k + d. 200 cases.
2. **An induction on the columns:** if the walk's accumulator and carry are right after i columns, one more row
   that is right keeps them right; and they are right before the first.

**Ein checks part 1, and only part 1.**
[`examples/beyond_arith/column_lemma.ein`](../../../examples/beyond_arith/column_lemma.ein) derives the table a
second way, from nine facts `(next 0 1)` … `(next 8 9)` and five rules of counting (a + 0 is a; a + (b + 1) is
(a + b) + 1, with the step over ten), and sets each counted row against the written one:

```text
“every row of the full adder agrees with counting”         :expect holds
(disagrees ?a ?b ?c)   — the goal has no row                 :expect (model) holds
“8 + 7 + carry 0: write 5, carry 1”  “8 + 7 + carry 1: write 6, carry 1”
```

601 firings, 0.8 ms, 200 `agrees` facts and no `disagrees` (`$E saturate --dump
examples/beyond_arith/column_lemma.ein`). `(every-row agrees)` is derived by a walk over the 200 rows in order:
over a table that is complete, *for every row* is a chain and needs no negation, as [`binary_logic.md`](binary_logic.md)
found for a gate's four rows. A mutant with the step over ten made wrong fails all three claims, with 90 rows
named (`build/s1f.6.10/p/p4b_mutant.ein`, `$E test`).

What part 1 rests on in turn: that counting is addition, which is a definition, and Ein's matcher. The Rust test
`ein-ir/tests/digit_tables.rs` holds the same rows to the machine's integers; this is the same check made in Ein.

**Part 2 is made outside Ein.** It quantifies over the length of a number and over every accumulator. Ein has no
variable that ranges over what is not in the state, no schema, and no way to assume a hypothesis and discharge it
except a commitment, which is a fact and not a formula.
[M1g](../../m1g_logic/README.md) plans proof systems, first-order logic, Peano and induction as theories in Ein:
there the induction would be a *proof object* that a theory checks, and the 200 rows would be its base lemma.
This probe is the evidence that the lemma half is cheap today.

### 4c. What it cannot do

- **Quantify over unbounded numbers.** Every law above is checked on a domain a rule enumerates.
- **State an induction schema,** or any statement about statements.
- **Reason by cases on an unknown** without a hypothesis for each case. § 2 does that by search, and then the
  theorem is *R is empty* for the negation, with the limits of § 2: 20 candidates for the oracle, and an R that
  is empty by owing.

## 5. Number theory exploration

Each is a program of ten or so rules over `std.arith`'s tagged calls, generated by `build/s1f.6.10/gen5.py` at a
bound, saturated, and **every answer compared with Python** (`python3 build/s1f.6.10/run5.py`; primes to 5 000 and nines to
9 999 are from the first run, `out/run5_first.txt`, their times re-taken alone):

| program | answer | agrees with Python | firings | saturate |
|---|---|---|---:|---:|
| primes, 2 to 100 | 25 primes; 74 composites, each with its smallest divisor | yes | 31 156 | 50 ms |
| primes, 2 to 1 000 | 168; 831 | yes | 849 808 | 1.55 s |
| primes, 2 to 5 000 | 669; 4 330 | yes | 8 693 213 | 21.4 s, 10.6 GiB |
| gcd and lcm, five pairs, to 31 digits | gcd(1071, 462) = 21, lcm 23 562, … | yes | 22 011 | 35 ms |
| casting out nines, 0 to 99 | holds of 100, fails of 0 | yes, with each digit sum | 21 765 | 34 ms |
| casting out nines, 0 to 999 | 1 000 · 0 | yes | 264 203 | 0.46 s |
| casting out nines, 0 to 9 999 | 10 000 · 0 | yes | 3 078 582 | 6.3 s, 3.3 GiB |
| perfect numbers, 1 to 30 | 6 and 28; 5 abundant, 23 deficient | yes, with each sum of divisors | 53 897 | 87 ms |
| perfect numbers, 1 to 100 | 6 and 28; 22 abundant, 76 deficient | yes | 663 377 | 1.18 s |
| perfect numbers, 1 to 250 | 6 and 28; 57 abundant, 191 deficient | yes | 5 140 127 | 12.4 s |
| perfect numbers, 1 to 500 | **no answer:** the process ended when the machine's 32 GB were used | | | |
| Collatz, 27, bound 200 | reaches 1 after 111 steps | yes | 17 651 | 29 ms |
| Collatz, 1 to 30, bound 200 | all reach 1 | yes, each count | 56 123 | 89 ms |
| Collatz, 27 and 97, bound 50 | stopped: 27 at 566, 97 at 334 | yes | 15 591 | 25 ms |
| Collatz, 837 799, bound 1 000 | reaches 1 after 524 steps | yes | 135 837 | 233 ms |

The stored facts are the firings plus some 2 000 in every row. Banked, at sizes the corpus affords:
[`primes.ein`](../../../examples/beyond_arith/primes.ein) (to 30),
[`gcd.ein`](../../../examples/beyond_arith/gcd.ein),
[`casting_out_nines.ein`](../../../examples/beyond_arith/casting_out_nines.ein) (to 40),
[`perfect.ein`](../../../examples/beyond_arith/perfect.ein) (to 12),
[`collatz.ein`](../../../examples/beyond_arith/collatz.ein) (7, and 27 stopped at 20 steps).

What the programs show about writing such things in Ein:

- **Primality is monotone.** *k does not divide n* looks like a negation and is not one: a remainder that is not
  zero is a number written `(n PREFIX DIGIT)`, a positive pattern. And *no divisor up to the root* is a scan in
  order that reaches a quotient smaller than its divisor, a positive event. So `(prime ?n)` is derived with no
  `absent`, in one stratum, and the universe has no candidate.
- **A loop is a chain with a comparison.** Counting from 2 to N, the next trial divisor, the next Collatz step:
  each is a fact that asks a comparison, and one of two rules matches the answer. No rule iterates; saturation
  does.
- **A saturation that might not end needs a bound it can count to.** Nobody knows that every Collatz trajectory
  reaches 1, so the request carries a bound on the steps and a rule stops there, `(collatz-stopped 27 … 274)`.
  Without it a diverging input would saturate for ever, and Ein has no other guard than `-T`.
- **Memory is the limit, not time** (E-26). Trial division to 5 000 is 8.7 million facts, and the sums of divisors to
  500 did not fit in 32 GB. Nothing a circuit derived is forgotten, though only the
  verdict facts are wanted.
- **gcd and lcm are six rules,** Euclid's chain a division a step, where `std.rat` has the same chain inside it
  and does not offer it. Question 5.

## 6. Explanations

### 6a. A worked solution is the trace's sentences

`build/s1f.6.10/p/p6_worked.ein` asks 478 + 356. `$E solve FILE -G --trace T.md`, then `grep '^> ' T.md`, is 16
lines:

```text
> … + … is asked: first, the number (n (n (n z 4) 7) 8) stands for
> add (n (n (n z 4) 7) 8) and (n (n (n z 3) 5) 6) from the units, with no carry in
> 8 + 6 + carry 0 is 4, carry 1: 4 goes in front of the digits so far
> 7 + 5 + carry 1 is 3, carry 1: 3 goes in front of the digits so far
> 4 + 3 + carry 1 is 8, carry 0: 8 goes in front of the digits so far
> no column is left and no carry: the digits are (b 8 (b 3 (b 4 z)))
> the top digit 8 is not zero: no high zero is left
> the digit 8 goes after z   …   turned round: the answer is (n (n (n z 8) 3) 4)
> (n (n (n z 4) 7) 8) + (n (n (n z 3) 5) 6) = (n (n (n z 8) 3) 4)
```

The three column lines are a school-style worked solution, word for word what a rule's `:why` says. What stands
between this and one a pupil could read is known and not new: the numbers print as nested facts (E-14), and the
machinery's steps, the operands looked up and the answer turned round, are interleaved with the reasoning with
nothing to fold them (E-17).

The inverse walk explains itself the same way (`inverse_addition.ein`, `--trace`): *what plus 7 plus carry 0 ends
in 5? 8: write 8, carry 1*.

### 6b. A proof of no solution: three forms, one of them a proof

| asked as | the answer | where the reason is |
|---|---|---|
| saturation, several answers a column (§ 1b, x × 4 = 3285) | silence: no answer fact | nowhere. *No walk ended* is not derivable |
| search (§ 2, x × 7 = 862) | R is empty: *every alive node owes*; `--single` says `Open` | N's cores, 28 of them the argument, in `--trace` and `nogoods.json`; the read-out prints the owing frontier and no N row (E-23) |
| saturation, one answer a column | **root is dead** | **the read-out's unsat core** |

The third is [`examples/beyond_arith/equation_refuted.ein`](../../../examples/beyond_arith/equation_refuted.ein).
By 7 a column has exactly one answer, so the walk of § 1b is a straight line and a carry left over the top
refutes the equation outright: one more rule, `(false)` from that carry, sound because the multiplier is one of
1, 3, 7, 9, which the program states as facts and the rule reads.

```text
  empty             every node is dead — root's own fixpoint derives (false)
  universe          no candidate: R is {∅} or empty, decided by root alone
  work              1 saturation · 0 facts beyond root · 1 layer

  unsat core        (md 2 7 4 8 1)
  unsat core        (md 6 7 0 2 4)
  unsat core        (md 6 7 4 6 4)
  unsat core        (nz 1)
  unsat core        (one-answer 7)
  unsat core        (unmul 7 (n (n (n z 8) 6) 2))
  refuted by        um-no-solution
```

and its trace is the proof in sentences:

```text
> what times 7 plus carry 0 ends in 2? only 6: write 6, carry 4
> what times 7 plus carry 4 ends in 6? only 6: write 6, carry 4
> what times 7 plus carry 4 ends in 8? only 2: write 2, carry 1
> the product is read and a carry of 1 is left: no x times 7 is (n (n (n z 8) 6) 2)
```

**A refutation is a proof exactly where the theory can derive the uniqueness it rests on.** The search form finds
the same three rows, as the cores of its last ten dead sets, after 251 saturations, and then reports the result
as a debt. The saturation form derives it in five firings and reports it as a contradiction with its reasons.
Which form a question gets is the author's choice of encoding, and nothing tells the author which is better.

## 7. What else the probes suggested

- **A question about a set is a join.** `build/s1f.6.10/gen7.py pyth`: the squares of 1 to N, every pair added,
  and a sum that is itself a square is a triple, `(and (squares-sum ?a ?b ?u) (square ?c ?u))`. To 20, 12 ordered
  triples, 4 986 firings, 7 ms; to 100, 104, 133 329 firings, 195 ms; both lists equal Python's (`python3
  build/s1f.6.10/run7.py`). [`pythagorean.ein`](../../../examples/beyond_arith/pythagorean.ein) is it to 13. The
  join is on a number, which the rule of the theory forbids inside a circuit; it is the natural way to ask, and
  at these sizes it costs nothing one can see.
- **A sequence is a chain.** `gen7.py fib`: F(100) = 354224848179261915075 in 3 929 firings, 5.7 ms; F(500), 105
  digits, in 62 291 firings, 89 ms; each F(i) equal to Python's.
  [`fibonacci.ein`](../../../examples/beyond_arith/fibonacci.ein) is it to F(30).
- **Ein as a checker of a certificate.** `equation_refuted.ein`'s core, and each `holds` fact of § 4, is a small
  certificate: the facts a conclusion rests on. A factorisation, a gcd with its chain, a division with its
  identity are all checked by one saturation. This is the cheap direction of every search above: finding x × 7 =
  861 by search is 31 saturations, and checking 123 × 7 = 861 is one.
- **The same puzzle, three costs.** SEND + MORE as a search is 17 747 nodes; with the tables read backwards as
  propagation it is 639; a human does it in a dozen deductions. The rules that close the gap are the ones § 1
  writes: a table read in every direction. A cryptarithm *theory* in the stdlib, with those rules written once,
  would be to arithmetic puzzles what `std.bijection`'s elimination rules are to the zebra.

## What worked, what did not, and why

**Worked.**

1. Reading a table in any direction, in a rule. One unknown: saturation alone.
2. Search over unknown digits with the tables as constraints: right answers, agreed by the oracle and by Python;
   `--single` and `-n` are cheap where the constraint can fail early.
3. Search checked by the search-free theory (3x + 5y = 47): the two compose with no special care.
4. Bounded checking of laws, and of a table against a second definition, with a claim that nothing fails.
5. Number theory as chains over the tagged calls: every answer right, to the sizes memory allows.
6. A refutation at root whose core is a readable proof.

**Did not work, or only up to a size.**

1. **Describing R of a guessing program.** Exponential in the unknowns even when the answer is one state and
   `--single` finds it in a few dozen saturations. Why: R's description classifies every alive node, and a
   partial assignment is alive.
2. **`ein test` on such a program.** It describes, so the claim is NOT CHECKED (E-30).
3. **The obligation tree with the unknowns declared in an unhelpful order** (E-29).
4. **Saying *no solution* by saturation** where a column has several answers: an absence.
5. **Saying *no solution* by search** in a way that reads as one: `Open`, and the cores out of the read-out
   (E-23).
6. **Anything unbounded**: a law for every number, an induction, a trajectory with no bound.
7. **Keeping only the answer.** Every step is stored, so a long exploration ends by memory (E-26).
8. **Ordinary joins on numbers in a check** cost 40 times the tagged form (E-3), and the tagged form uses
   relations the theory calls its own.

## Findings

Three are new rows of [`findings.md`](findings.md). None is a wrong answer: every count agreed with Python and
with the oracle, so no row of `corpus/wrong_answers.toml` was filed.

| id | what | recommendation |
|---|---|---|
| **E-29** | **The obligation tree branches in the order the obligations are written, and that order decides whether a search ends.** SEND + MORE = MONEY: 0.5 s with the letters declared by column, no answer in 600 s declared as written; with propagation rules, 2.1 s against 19.4 s. Nothing in the read-out says the order mattered | **needs an engine change: a choice of the owed instance by the state** (fewest alive alternatives first, which the retired `EIN_OBLIGATION_CHOICE=fail-first` measured on the walk). It is [P1f.10](../p1f.10_hypothesis_structure/)'s subject, and a cryptarithm is its cheapest witness. Until then: a line on the page of any search theory, *declare what is checked first, first* |
| **E-30** | **`ein test` decides a claim by describing R, where a cheaper route decides the same claim.** A query's `:expect (model …)` over a program whose R `--single` shows to be that one state, *R holds no other state*, in 0.5 s is NOT CHECKED after 7.5 s, exit 1 | **needs an engine change, in the runner: ask the question the claim is.** A claim of one class over all of R is `--single`'s question; a claim of k classes is `-n k+1`'s. It is the user's *minimum work per question* applied to `ein test`. **Deserves a stage**; without it no search program past some twenty unknown-value pairs can carry a claim |
| **E-31** | **Describing R spends its time on a hypergraph transversal, not on saturations.** The state route takes N and each state's generators as minimal transversals (`ein_infer::transversal`, MMCS). Equation B at five digits: 16 331 saturations and 5.1 s, of which 94 % is `transversal::Mmcs::rec` (`perf record $E solve build/s1f.6.10/p/p2s_B5.ein`); 39 µs a saturation at four digits (1 636), 315 µs at five, and no answer in 300 s at six, where `--single` takes 61 saturations. TWO + TWO = FOUR: 491 µs a saturation over 67 046 | **an engine row, no owner yet.** Where every alive node owes or is a partial assignment, the maximal alive states are many and their complements' transversals are what costs. Whether a surface that prints only a count of N needs the sets is the *minimum work per question* ruling's to ask |

And five known rows met again, each with a new witness: **E-3** (a check written with ordinary joins, 16.8 s
against 0.42 s, § 4a); **E-18** and **E-23** (an equation with no solution reads as owing, and its proof is out of
the read-out, § 2 and § 6b); **E-24** and **E-26** (what sized the banked programs, and what ended the perfect
numbers at 500); **E-14** and **E-17** (the worked solution, § 6a).

## What deserves a stage

| probe | recommendation | why |
|---|---|---|
| the tables backwards (§ 1) and the refutation at root (§ 6b) | **a stage: `std.arith`'s inverse requests**, `(solve-add Y S)` and `(solve-mul M P)` for a one-digit M, answered by saturation, with *no solution* as a fact where the table makes it derivable | it is the user's *solving equations*, it is search-free, eleven rules were enough, and it is the only form whose *no* is a proof |
| cryptarithms (§ 3) | **a stage, after E-29 and E-30 are ruled:** a small `std.crypt` theory, the column rules with the tables read in every direction, and SEND + MORE as its example | Ein's home ground; today the example cannot carry a claim and its cost hangs on declaration order |
| `ein test` by the cheapest route (E-30) | **a stage, engine** | above |
| the choice of the owed instance (E-29) | **P1f.10's**, with this witness | above |
| bounded laws (§ 4a) | **no stage; keep the banked program.** A second use would be a test of the theory written in Ein, beside `arith_theory.rs`, and that one is held to the machine's integers already | it proves nothing the Rust test does not |
| the column lemma (§ 4b) | **M1g's**: the base lemma of its induction, with this program as the evidence | Ein cannot do the other half today |
| number theory (§ 5) | **no stage of its own; one decision**: whether gcd and lcm are offered by `std.arith` (question 5). The rest are examples | each is a dozen rules a user writes; the limit is E-26, which is the engine's |
| explanations (§ 6a) | **no new stage**: it waits on E-14 and E-17, M1g's S1g.6.3 and S1f.6.20 | nothing new was found |
| equations with unknown digits in several places, by search (§ 2) | **out of reach as a general solver**: 10ⁿ through `std.arith`, and column-wise only where the author can write the columns, which is what `std.arith` already is | a search over digits re-derives a circuit the theory has |

## Questions for the user

1. **Inverse requests in `std.arith`.** Should the theory answer x + Y = S and x × M = P (M one digit), by
   saturation, with *no solution* derived where it can be? **Recommended: yes, as a stage**; § 1 and § 6b are its
   prototype. A multi-digit M is division, which the theory has: x × M = P is `(div P M)` with remainder zero,
   and the stage would say so rather than build it twice.
2. **`ein test` on a search program (E-30).** Should the runner decide a claim by the cheapest route that
   decides it, `--single`'s for a claim of one state? **Recommended: yes, as a stage**, before any theory of
   puzzles ships an example it cannot test.
3. **The order of obligations (E-29).** Is a state-dependent choice wanted for the obligation tree, and is it
   P1f.10's? **Recommended: yes and yes**; SEND + MORE in two declaration orders is the witness, under
   `build/s1f.6.10/p/`.
4. **Are the tagged calls public?** `laws.ein`, `primes.ein` and the other chains call `add-walk`, `mul-call`,
   `div-call`, `cmp-walk` and read `done`, `div-done`, `order-is`. `std.int` and `std.rat` do the same from inside
   the stdlib. The public protocol, `(add X Y)` and `(sum-of X Y S)`, forces a join on a number in any rule that
   uses an answer for a particular purpose, 40 times the cost in § 4a. **Recommended: say on the module's page
   that the tagged calls are the interface for a theory built on it**, and name them; they are stable already,
   since two shipped modules call them. (Defaulted for the banked programs:
   [Q-M1f.189](../open_questions.md#q-m1f189--a-users-program-calling-stdariths-tagged-calls).)
5. **gcd and lcm.** `std.rat` computes a gcd inside and offers none. Six rules offer both. Into `std.arith`, or
   left as the example? **Recommended: into `std.arith`**, as `(gcd X Y)` answered by `(gcd-of X Y G)`, when
   question 1's stage runs.
6. **An unsatisfiable search as `Open`** (E-23, now on an equation). It was question 6 of
   [`binary_logic.md`](binary_logic.md) and M1g's probe 1b; this page adds only that the same question, asked by
   saturation, gets a dead root and a three-row proof. No new recommendation.
7. **A cryptarithm theory** (`std.crypt` or under another name), after 2 and 3? **Recommended: yes, small**: it
   is the first puzzle family beside the zebra that the stdlib's tables make cheap to state.
8. **Where the banked programs live.** They are under `examples/beyond_arith/` and not `examples/arith/`,
   because `arith_search_free.rs` holds every program under `examples/arith/` to *no candidate, R = {∅}*, and
   four of these search ([Q-M1f.186](../open_questions.md#q-m1f186--where-the-probes-of-s1f610-are-banked),
   defaulted). Another name, or one directory with the test told which files search?
