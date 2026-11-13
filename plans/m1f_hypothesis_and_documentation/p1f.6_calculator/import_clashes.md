# Name clashes between imported theories: the probes, the census, and the proposals

**Phase:** [P1f.6](README.md). **Stage:** [S1f.6.30](s1f.6.30_name_clashes_between_imported_theories.md), tasks
T2 to T5, 2026-11-13. **What it is for:** the user's reply to [question 8](questions.md#q8), 2026-11-12:

> usage of imported theories should be convenient, and allow mixing and connecting theories, if they have
> common things (like digits.ein) they are already connected, potential names clash is a gap of import
> mechanism, plan a stage for investigation, proposals and fix

**This page changes nothing.** It shows what mixing two theories does today, on real programs run on the
release binary, counts the names every module occupies, and shows each proposal on the same programs. Nothing
in `ein.l`, `ein.y` or `ein_ir::imports` moves before the user rules, and no relation of a shipped module is
renamed: the stage stops at this page.

Every run below is the release binary built at `2ef68530`, the last commit that moved engine source, with the
checkout's stdlib unless a run names a prototype's (`$EIN_STDLIB`), each in a scope of its own of 2 GB and no
swap, killed after 60 s (`build/s1f.6.30/run.sh`). The scratch is under `build/s1f.6.30/`, which is not
tracked, so every program and every output the page relies on is quoted here. A program is quoted without its
comment lines; an output is trimmed to the lines that differ from one program to the next, and every program
that loads has the same head: one state, R = {∅}, no candidate, one saturation (§ 2, P1b, quotes it once).

## 1. How an import works today

[`01_grammar.md` § Import](../../../docs/kernel/ir/03-ein-lang/01_grammar.md#import) states it, and
`ein_ir::imports` does it, before the loader runs: each `(import …)` is replaced by forms of the module.

- **A flat import, `:symbols (…)`,** keeps the listed names and their closure (every name a kept declaration
  writes), unrenamed, and the module's facts of every relation in the closure. A module's own imports are
  resolved first, so a theory's closure reaches into what the theory imports.
- **A qualified import, bare or `:as a`,** keeps every declaration and fact of the module *as resolved*, its
  own imports' included, and prefixes every name any of them **declares**, wherever it stands: a relation at
  the head of a fact, a rule named as a value, a relation inside a `__closed__`. A name nobody declares, `z`,
  the digits, `lt`, `eq`, `gt`, the names inside facts, is left as written.
- **Then one list.** Two declarations of one kind and one name are one if their bodies are equal, and the
  load is refused if they differ: *conflicting definitions of 'X': same name, different body*. A fact that
  arrives twice by one module is one fact.

So flat imports share by *name*: two theories that import `std.digits` get the same `fa`, and a diamond
collapses. A qualified import shares nothing: it renames its whole subtree, the tables a theory imports
among it.

## 2. The probes (T2)

### The modules the probes import

Three small theories written for the probes, as a user would write them, under `build/s1f.6.30/p/`.

```lisp
;; succ.ein: the digit after a digit, read off the full adder.
(import std.digits :symbols (fa __closed__))
(relation next-digit Digit Digit :why "after {?1} comes {?2}")
(relation succ-rule Rule :why "{?1} is a rule of succ")
(rule next-digit-of ()
  :match  (fa ?a 1 0 ?b 0)
  :assert (next-digit ?a ?b)
  :why    "{?a} + 1 is {?b}, with no carry")
(rule succ ()
  :match  ()
  :assert (and (__closed__ next-digit) (__closed__ succ-rule) (succ-rule next-digit-of))
  :why    "the next digit is computed, never guessed")

;; order.ein: which digit is below which, read off the comparator.
(import std.digits :symbols (cl __closed__))
(relation below Digit Digit :why "{?1} is below {?2}")
(relation order-rule Rule :why "{?1} is a rule of order")
(rule below-of ()
  :match  (cl eq ?a ?b lt)
  :assert (below ?a ?b)
  :why    "{?a} against {?b} is lt")
(rule digit-order ()
  :match  ()
  :assert (and (__closed__ below) (__closed__ order-rule) (order-rule below-of))
  :why    "the order of the digits is computed, never guessed")

;; chores.ein: a theory written without the calculator in mind.
(relation chore Task :why "{?1} is a chore")
(relation done Task :why "{?1} is done")
(relation to-do Task :why "{?1} is still to do")
(relation chores-rule Rule :why "{?1} is a rule of chores")
(rule still-to-do ()
  :match  (and (chore ?t) (absent (done ?t)))
  :assert (to-do ?t)
  :why    "{?t} is a chore and is not done")
(rule chores ()
  :match  ()
  :assert (and (__closed__ chore) (__closed__ done) (__closed__ to-do) (__closed__ chores-rule)
               (chores-rule still-to-do))
  :why    "what is to do follows from what is done")
```

### The table

| | the program | today |
|---|---|---|
| **P1a** | `examples/arith/num9_both_ways.ein`: `std.arith` and `std.num9` flat, both over `std.digits`, `std.num9` taking `n` and the division-by-zero flag from `std.arith`; a rule of the program compares the two theories' sums | loads; `:expect holds`: the two sums are one number. **Connected, by the shared `n`** |
| **P1b** | `succ` and `order` flat, both over `std.digits`; a rule of the program joins them on a digit | loads; nine `step-up` facts. `fa` is 200 rows, `cl` 300: **the tables arrive once** |
| **P2a** | `std.arith` and `chores` flat: each declares `done` | **refused**, exit 1: *conflicting definitions of 'done': same name, different body (at None)* |
| **P2b** | E-21's witness, `std.arith` and the binary prototype (a) flat, the same names over other tables | **refused**: *… of 'div-m1' …* (on 2026-11-11 it was `div-digits`: the first name that differs moves with the theory) |
| **P3a** | P2a with `std.arith :as a` | loads; `(a.sum-of (a.n z 2) (a.n z 3) (a.n z 5))` and `(to-do dishes)` |
| **P3d** | P2a with `chores :as c` | loads; `(sum-of (n z 2) (n z 3) (n z 5))` and `(c.to-do dishes)` |
| **P3b** | `succ` flat and `std.arith :as a`: do the tables arrive twice, and do the two still connect? | loads; **`fa` and `a.fa`, 200 rows each, and `a.fs`, `a.md`, `a.cl`**: the calculator's tables twice over. A rule joins `(a.sum-of … (a.n z ?d))` to `(next-digit ?d ?e)`: `(bumped 6)`. **Connected by the digits**, which no import renames |
| **P3c** | `std.num9` flat and `std.arith :as a`; a request of `a` given `std.num9`'s number | loads; each theory answers its own; `(a.add (n z 2) (n z 3))` is **answered by silence**: no `a.sum-of`, and no `a.malformed` either. **Not connected by the numbers**: `n` and `a.n` are two shapes |
| **P4a** | the program declares `(relation done Task …)` beside `std.arith` flat | **refused**: *… of 'done' …* |
| **P4b** | the program declares `(relation sum-of Person Number Number …)` | **refused**: *… of 'sum-of' …* |
| **P4c** | the program writes `(done laundry)` and declares nothing | **refused** since S1f.6.23: *the fact (done laundry) gives done 1 argument, and (relation done Tag Number) declares 2* |
| **P4d** | the program writes `(done laundry today)` | loads: **the program's fact sits in the calculator's `done`** beside `(done (sum (n z 2) (n z 3)) (n z 5))` |
| **P4e** | the program has a rule named `add-column`, as `examples/beyond_arith/cryptarithm.ein` has, beside `std.arith` flat | **refused**: *… of 'add-column' …* |
| **P4f** | the program declares `(relation b T …)`, as five witnesses under `examples/ein-bugs/` do | **refused**: *… of 'b' …* |
| **P5** | `std.arith` flat and again `:as a` | loads; two calculators, `sum-of` and `a.sum-of`; every table twice: 3 656 facts and 232 rules before saturation, where one calculator is 1 839 and 118 |
| **P6a** | `std.arith`, `std.int`, `std.rat`, `std.num9` flat | loads; a sum of each: 5, 3, 5/6, nine-digit 5 |
| **P6b** | P6a and `examples/binary_arithmetic/`'s `bin` and `conv` flat | loads; `(sum-of2 … (b2 (b2 (b2 z 1) 0) 1))`, 5 in binary |
| **P6c** | P6b and `examples/binary_logic/`'s gates, properties and circuit flat | loads; eight `commutative` functions. **No clash among all of them** |
| **P6d** | the logic's two writings of one set of properties, `props` (facts) and `props_rules` (rules), flat | loads: their 30 declarations are equal, one each |
| **P7a** | `std.arith :as a`; a rule of the program calls `(a.add-walk (fee-of ?i) ?p (a.n z 3) 0 z)` and reads `(a.done (fee-of ?i) ?s)` | loads; `(total tea (a.n (a.n z 1) 0))`: 7 + 3 |
| **P7b** | P7a with the call and the answer written without the prefix | loads, **answered by silence**: `(total ?i ?s)` has no row, and nothing says why |
| **P7c** | P7a with the numbers written without the prefix, `(n z 7)` | loads, **answered by silence**, the same |
| **P8** | `std.arith :as a` and `(a.div (a.n z 7) z)`: what a core and a trace print | `unsat core (a.div (a.n z 7) z)`, `refuted by a.division-by-zero-is-false`; a trace step is `` ### Step 1 — `a.add-ask` `` |
| **P9** | `(import ../succ …)` from a sibling directory (E-32) | **refused**: *(import ../succ) — module not found at /succ.ein (None)*; `(import ..succ …)` is *… not found at ./p9/succ.ein*, the importing file's own directory |
| **P10** | `std.int :as i` and `std.rat :as r`; a rule hands `i`'s sum to `r` | loads; each answers its own, and `(r.rat-simplify (r.rat (i.int …) (r.n z 1)))` is **answered by silence**: `r`'s integers are `r.int`. The tables arrive twice: `i.fa` and `r.fa`, 3 943 facts and 432 rules |

**What the table says, in four lines.**

- **Flat imports connect by name, and that is the user's *"already connected"*** (P1a, P1b, P6): one `fa`, one
  `n`, one `z`, and a fact of the program meets every theory that reads it. Every shipped module and example
  theory loads flat beside every other in P6, because each was written with names of its own (`int-`, `rat-`,
  `…9`, `…2`); `std.arith`'s are the bare ones.
- **The same sharing is the clash** (P2a, P4a, P4b, P4e, P4f), and its quiet twin (P4d): a name the program
  uses and a theory declares are one name. A declaration is refused, with the first name and no place (E-21);
  a fact joins the theory's relation in silence.
- **`:as` is today's way out, and it shares nothing** (P3, P5, P10): it renames the theory's tables and its
  numbers with it, so the tables arrive once per prefix and the theories connect only through names nobody
  declares, the digits.
- **A mistake under `:as` is silence** (P3c, P7b, P7c): a name written without its prefix, or a number of the
  wrong theory, is a relation or a value of the program's own, and nothing derives from it.

### The programs and their outputs

**P1b**, `p1b_two_over_digits.ein`:

```lisp
(import succ :symbols (succ))
(import order :symbols (digit-order))
(relation step-up Digit Digit :why "{?1} steps up to {?2}")
(rule connected ()
  :match  (and (next-digit ?a ?b) (below ?a ?b))
  :assert (step-up ?a ?b)
  :why    "{?b} comes after {?a} and is above it")
(__closed__ step-up)
(query :goal (step-up ?a ?b))
```

```text
solve · ./p1b_two_over_digits.ein
──────────────────────────────────────────────────────────────
  asked             all of R — describe
  states (k_s)      1
  commitments (k_c) 1
  exhausted         yes — every commitment is classified: this is all of R
  ∅                 in R — root is alive and owes nothing
  …
  universe          no candidate: R is {∅} or empty, decided by root alone
  work              1 saturation · 0 facts beyond root · 2 layers

  query             (step-up ?a ?b)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?a = 0 · ?b = 1
                      ?a = 1 · ?b = 2
                      …
                      ?a = 8 · ?b = 9
```

`ein saturate`, the facts by relation before saturation: `fa` 200, `cl` 300, `__closed__` 8; 522 facts.

**P1a** is `examples/arith/num9_both_ways.ein` as tracked; its claim, `(cmp-of 127645 127645 eq)` written as
numbers, *holds*.

**P2a**, `p2a_two_dones.ein`, and its three variants. Each prints nothing on stdout and exits 1:

```lisp
(import std.arith :symbols (arith))
(import chores :symbols (chores))
(add (n z 2) (n z 3))
(chore laundry) (chore dishes)
(done laundry)
(query :goal (sum-of ?x ?y ?s))
(query :goal (to-do ?t))
```

```text
kb load error: conflicting definitions of 'done': same name, different body (at None)
```

P4a, P4b, P4e and P4f are the same message with `done`, `sum-of`, `add-column` and `b`; each program is
`(import std.arith :symbols (arith))`, `(add (n z 2) (n z 3))`, a query, and its one declaration:

```lisp
(relation done Task :why "{?1} is done")                             ; P4a, with (done laundry)
(relation sum-of Person Number Number :why "{?1}'s sum is {?2} and {?3}")   ; P4b
(rule add-column ()                                                   ; P4e, as cryptarithm.ein's
  :match  (and (column ?c ?a ?b) (fa ?a ?b 0 ?d ?k))
  :assert (col ?c ?d ?k)
  :why    "{?a} + {?b} is digit {?d}, carry {?k}")
(relation b T :why "{?1} is b")                                       ; P4f, with (b one)
```

P2b is `build/s1f.6.16/q/both_flat.ein`, `(import std.arith :symbols (arith))` and `(import arith2 :symbols
(arith))`, re-run: *conflicting definitions of 'div-m1' …*.

**P4c**, `(done laundry)` with no declaration:

```text
kb load error: ./p4c_undeclared_done.ein:5:2: the fact (done laundry) gives done 1 argument, and (relation done Tag Number) declares 2 — a form headed by a declared relation has as many arguments as its declaration has types
```

**P4d**, `(done laundry today)`, and `(query :goal (done ?t ?a))`:

```text
  query 2 of 2      (done ?t ?a)
    class 1 of 1    1 state (1)
                      ?a = (n z 5) · ?t = (sum (n z 2) (n z 3))
                      ?a = today · ?t = laundry
```

**P3a**, `p3a_arith_as.ein`:

```lisp
(import std.arith :as a)
(import chores :symbols (chores))
(a.add (a.n z 2) (a.n z 3))
(chore laundry) (chore dishes)
(done laundry)
(query :goal (a.sum-of ?x ?y ?s))
(query :goal (to-do ?t))
```

```text
  query 1 of 2      (a.sum-of ?x ?y ?s)
                      ?s = (a.n z 5) · ?x = (a.n z 2) · ?y = (a.n z 3)
  query 2 of 2      (to-do ?t)
                      ?t = dishes
```

**P3d** is P3a the other way round, `(import std.arith :symbols (arith))` and `(import chores :as c)`, with
`(c.chore …)` and `(c.done laundry)`: `?s = (n z 5)`, `?t = dishes` under `(c.to-do ?t)`.

**P3b**, `p3b_tables_twice.ein`:

```lisp
(import succ :symbols (succ))
(import std.arith :as a)
(a.add (a.n z 2) (a.n z 3))
(relation bumped Digit :why "the sum's digit, bumped, is {?1}")
(rule bump ()
  :match  (and (a.sum-of ?x ?y (a.n z ?d)) (next-digit ?d ?e))
  :assert (bumped ?e)
  :why    "the sum is the digit {?d}, and after it comes {?e}")
(__closed__ bumped)
(query :goal (bumped ?e))
```

`?e = 6`. Before saturation: `fa` 200, `a.fa` 200, `a.fs` 200, `a.md` 900, `a.cl` 300; 2 044 facts, 119
rules.

**P3c**, `p3c_num9_flat_arith_as.ein`:

```lisp
(import std.num9 :symbols (arith9))
(import std.arith :as a)
(to9 (n (n z 4) 2))
(a.add (a.n z 2) (a.n z 3))
(a.add (n z 2) (n z 3))
(query :goal (as9 ?x ?y))
(query :goal (a.sum-of ?x ?y ?s))
(query :goal (a.malformed ?r))
```

```text
  query 1 of 3      (as9 ?x ?y)
                      ?x = (n (n z 4) 2) · ?y = (num9 0 0 0 0 0 0 0 4 2)
  query 2 of 3      (a.sum-of ?x ?y ?s)
                      ?s = (a.n z 5) · ?x = (a.n z 2) · ?y = (a.n z 3)
  query 3 of 3      (a.malformed ?r)
    class 1 of 1    1 state (1) — the goal has no row: undetermined here
```

Both tables, `fa` and `a.fa`, `fs` and `a.fs`, `md` and `a.md`, `cl` and `a.cl`: 3 510 facts, 164 rules.

**P5**, `(import std.arith :symbols (arith))` and `(import std.arith :as a)`, a request to each: `?s = (n z
5)` and `?s = (a.n z 5)`; every table twice, 3 656 facts and 232 rules. P3a alone is 1 839 and 118.

**P6a to P6d.** The program sits in `build/s1f.6.30/p/p6/`, beside two symlinks to `examples/binary_arithmetic`
and `examples/binary_logic`, since a file-relative import cannot climb to them (E-32, P9):

```lisp
(import std.arith :symbols (arith))
(import std.int :symbols (int-arith))
(import std.rat :symbols (rat-arith))
(import std.num9 :symbols (arith9))
(import binary_arithmetic.bin :symbols (bin))
(import binary_arithmetic.conv :symbols (conv))
(import binary_logic.gates :symbols (gate2 column-name inv le bit __closed__))
(import binary_logic.props :symbols (commutative identity-of __closed__))
(import binary_logic.circuit :symbols (circuit))
(add (n z 2) (n z 3))
(add2 (b2 (b2 z 1) 0) (b2 (b2 z 1) 1))
(query :goal (sum-of ?x ?y ?s))
(query :goal (sum-of2 ?x ?y ?s))
(query :goal (commutative ?f))
```

```text
  query 1 of 3      (sum-of ?x ?y ?s)
                      ?s = (n z 5) · ?x = (n z 2) · ?y = (n z 3)
  query 2 of 3      (sum-of2 ?x ?y ?s)
                      ?s = (b2 (b2 (b2 z 1) 0) 1) · ?x = (b2 (b2 z 1) 0) · ?y = (b2 (b2 z 1) 1)
  query 3 of 3      (commutative ?f)
                      ?f = And
                      … Nand, Nor, One, Or, Xnor, Xor, Zero
```

P6a, the first four imports with a request each: `(sum-of … (n z 5))`, `(int-sum-of (int z (n z 2)) (int (n
z 5) z) (int (n z 3) z))`, `(rat-sum-of … (rat (int (n z 5) z) (n z 6)))`, `(sum9-of … (num9 0 0 0 0 0 0 0
0 5))`. P6b, the first six and `(to-bin (n z 5))`: `(bin-of (n z 5) (b2 (b2 (b2 z 1) 0) 1))`. 2 387 facts
and 375 rules for P6c, the tables once: `fa` 200, `fa2` 8.

**P7a**, `p7a_qualified_call.ein`:

```lisp
(import std.arith :as a)
(relation price Item Number :why "{?1} costs {?2}")
(relation total Item Number :why "{?1} costs {?2} with the fee")
(relation fee-of Item :why "the fee for {?1}")
(__closed__ price) (__closed__ total) (__closed__ fee-of)
(price tea (a.n z 7))
(rule ask-total ()
  :match  (price ?i ?p)
  :assert (a.add-walk (fee-of ?i) ?p (a.n z 3) 0 z)
  :why    "add the fee, 3, to {?p}")
(rule read-total ()
  :match  (a.done (fee-of ?i) ?s)
  :assert (total ?i ?s)
  :why    "with the fee, {?i} costs {?s}")
(query :goal (total ?i ?s))
```

```text
  query             (total ?i ?s)
                      ?i = tea · ?s = (a.n (a.n z 1) 0)
```

**P7b** writes `(add-walk …)` and `(done …)` for `(a.add-walk …)` and `(a.done …)`; **P7c** writes `(price
tea (n z 7))` and `(n z 3)` for the `a.n` ones. Each loads, exit 0, nothing on stderr, and:

```text
  query             (total ?i ?s)
    class 1 of 1    1 state (1) — the goal has no row: undetermined here
```

**P8**, `(import std.arith :as a)`, `(a.div (a.n z 7) z)`, `(query :goal (a.quot-of ?x ?y ?q ?r))`:

```text
  empty             every node is dead — root's own fixpoint derives (false)
  unsat core        (a.div (a.n z 7) z)
  refuted by        a.division-by-zero-is-false
```

and P3a's `--trace`, the names a trace prints under a prefix:

```text
### Step 1 — `a.add-ask`
Derives `a.operand(a.add-first(a.n(z, 2), a.n(z, 3)), a.n(z, 2))` — wanted, for (a.add-first (a.n z 2) (a.n z 3)): the number (a.n z 2) stands for
…
### Step 14 — `a.turn-end`
Derives `a.done(a.sum(a.n(z, 2), a.n(z, 3)), a.n(z, 5))` — the answer for (a.sum (a.n z 2) (a.n z 3)) is (a.n z 5)
### Step 15 — `a.add-answer`
Derives `a.sum-of(a.n(z, 2), a.n(z, 3), a.n(z, 5))` — (a.n z 2) + (a.n z 3) = (a.n z 5)
```

**P9**: `build/s1f.6.30/p/p9/sub/p9_sibling.ein`, `(import ../succ :symbols (succ))`, and
`p9/p9_up.ein`, `(import ..succ :symbols (succ))`; the two lines above, exit 1. E-32 as filed.

**P10**, `p10_stack_as.ein`:

```lisp
(import std.int :as i)
(import std.rat :as r)
(i.int-add (i.int z (i.n z 2)) (i.int (i.n z 5) z))
(r.rat-add (r.rat (r.int (r.n z 1) z) (r.n z 2)) (r.rat (r.int (r.n z 1) z) (r.n z 3)))
(rule int-to-rat ()
  :match  (i.int-sum-of ?x ?y ?s)
  :assert (r.rat-simplify (r.rat ?s (r.n z 1)))
  :why    "the integer sum {?s} as a rational, over one")
(query :goal (i.int-sum-of ?x ?y ?s))
(query :goal (r.rat-sum-of ?x ?y ?s))
(query :goal (r.rat-simple-of ?x ?s))
```

```text
  query 1 of 3      (i.int-sum-of ?x ?y ?s)
                      ?s = (i.int (i.n z 3) z) · ?x = (i.int z (i.n z 2)) · ?y = (i.int (i.n z 5) z)
  query 2 of 3      (r.rat-sum-of ?x ?y ?s)
                      ?s = (r.rat (r.int (r.n z 5) z) (r.n z 6)) · ?x = … · ?y = …
  query 3 of 3      (r.rat-simple-of ?x ?s)
    class 1 of 1    1 state (1) — the goal has no row: undetermined here
```

No probe met a wrong answer of the engine. Three met one finding, filed in [`findings.md`](findings.md):
P3c, P7b and P7c, a name of a qualified theory written without its prefix, or a value of the wrong theory,
answered by silence (E-49).

## 3. The census (T3)

Over the twelve stdlib modules and the fourteen example theories, the files another file imports as a theory:
`examples/binary_arithmetic/` `bits`, `bin`, `conv`; `examples/binary_logic/` `bits`, `gates`, `props`,
`props_rules`, `circuit`, `full_adder`, `inputs`, `require`; `examples/word_problems/` `words`, `gaps`,
`readings`. The fixtures of the import mechanism itself, under `tests/imports/` and the `import-*` witnesses,
are left out: they exist to break it. Taken by `build/s1f.6.30/census.py`, a reader of Ein's text and a copy
of `ein_ir::imports`' flat selection and its check of two bodies, 2026-11-13; **held to the engine on every
pair** below (`--engine`: a program of the two flat imports, loaded by `ein saturate`, refused exactly where the
census finds a clash, with one of the names it finds: 325 of 325 agree).

### What each module declares, by role

A relation's role is read off the rules a flat import of the module brings: an **input** is read by a rule
and derived by none, so a program states it (the requests, `let`); a **result** is derived and read by no rule
of the module, so a program reads it; **internal** is derived and read, a step of the circuit; a **value**
appears only nested inside another fact, a tag or a number's shape; a **table** is stated by the module.
**Interface** is T1's: what a theory built on the module calls and reads.

| module | relations | input | result | interface | internal | table | value | unused | rules, hrules, macros |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `std.algebra` | 0 | | | | | | | | 40 |
| `std.arith` | 74 | 6 | 6 | 11 | 16 | | 35 | | 116 |
| `std.bijection` | 0 | | | | | | | | 8 |
| `std.closure` | 0 | | | | | | | | 1 |
| `std.digits` | 7 | | | | | 7 | | | 0 |
| `std.elim` | 0 | | | | | | | | 4 |
| `std.int` | 47 | 9 | 10 | 5 | 1 | | 22 | | 71 |
| `std.macro` | 0 | | | | | | | | 2 |
| `std.num9` | 22 | 7 | 10 | | 2 | | 3 | | 47 |
| `std.rat` | 48 | 7 | 8 | 5 | 5 | | 23 | | 57 |
| `std.slots` | 0 | | | | | | | | 20 |
| `std.typing` | 0 | | | | | | | | 4 |
| `binary_arithmetic.bits` | 3 | | | | | 3 | | | 0 |
| `binary_arithmetic.bin` | 54 | 6 | 6 | | 19 | | 23 | | 69 |
| `binary_arithmetic.conv` | 11 | 2 | 3 | | 2 | | 4 | | 12 |
| `binary_logic.bits` | 3 | | | | | 3 | | | 0 |
| `binary_logic.gates` | 5 | | | | | 5 | | | 0 |
| `binary_logic.props` | 30 | | | | | 30 | | | 0 |
| `binary_logic.props_rules` | 32 | | 20 | | 12 | | | | 32 |
| `binary_logic.circuit` | 3 | 1 | 1 | | 1 | | | | 3 |
| `binary_logic.full_adder` | 0 | | | | | | | | 0 |
| `binary_logic.inputs` | 3 | 1 | 1 | | | 1 | | | 2 |
| `binary_logic.require` | 2 | 1 | 1 | | | | | | 2 |
| `word_problems.words` | 26 | 7 | 5 | | 4 | 1 | 8 | 1 | 37 |
| `word_problems.gaps` | 5 | | 3 | | 2 | | | | 7 |
| `word_problems.readings` | 5 | | 1 | | 1 | 1 | | 2 | 7 |

**`std.arith`'s 74, by name.** What a program states: `add`, `sub`, `cmp`, `mul`, `div`, `let`. What it reads:
`sum-of`, `diff-of`, `cmp-of`, `product-of`, `quot-of`, and the flags `division-by-zero`, `malformed`,
`undeclared` (internal by the roles, since each derives `(false)` by a rule of the module). The number: `n`. The
interface, T1's eleven: `add-walk`, `sub-walk`, `cmp-walk`, `mul-call`, `div-call`, `operand`; `done`, `short`,
`order-is`, `div-done`, `operand-is`. **The circuit's own, forty-eight:** `b`, `named`, `check`, `bad-operand`,
`bad-request`, `no-let`, `not-a-number`, `strip`, `turn`, `top-first`, `top-first-is`, `mul-walk`, `mul-step`,
`multiples`, `div-step`, `div-choose`, `div-digit`, the tags `sum`, `add-first`, `add-second`, `difference`,
`shortfall`, `sub-first`, `sub-second`, `order`, `cmp-first`, `cmp-second`, `product`, `mul-first`,
`mul-second`, `mul-digits`, `mul-row-of`, `mul-sum`, `quotient`, `div-first`, `div-second`, `div-m1` to
`div-m9`, `div-digits`, `div-trial`, and `arith-rule`. With its 116 rules, every one a name it occupies
flat. Question 8's *"sixty relations renamed"* is these forty-eight less `arith-rule`, forty-seven, or
fifty-eight with the interface.

**The shipped stack's other modules occupy names of their own:** `std.int`'s and `std.rat`'s begin `int` and
`rat`, `std.num9`'s end in `9`, the binary theory's in `2`. The zebra modules declare rules only, generic over
a relation parameter, activated by a program's facts, `(functional owns Person Pet)`.

### The pairs that cannot be imported flat together

**Two pairs of 325**, each by rules of one name and two bodies:

- **`std.bijection` and `std.elim`**: `domain-elimination`, `typecheck-arg-0`, `typecheck-arg-1`. Two
  formulations of one inference, positional and signature-driven; `std.elim`'s header says *"the two
  coexist"*, in the library, and a program cannot hold both: *conflicting definitions of 'typecheck-arg-0'*.
- **`std.algebra` and `examples/binary_logic/props_rules.ein`**: `complement` and `identity`, a generic rule
  of the algebra and a rule of the logic's properties.

**And one name a rule of one and a relation of the other:** `std.algebra`'s rule `difference`, activated by a
fact `(difference R S T)`, and `std.arith`'s tag `difference`, a relation of two arguments, which `std.int`,
`std.rat`, `conv` and `words` bring. No load refuses the pair, and a program that activates the rule beside the
calculator is refused by the count: **P11**, `tests/stdlib/algebra/03_boolean.ein` with `(import std.arith
:symbols (arith))` added,

```text
kb load error: ./p11_algebra_difference_and_arith.ein:13:2: the fact (difference owns rents owns-only) gives difference 3 arguments, and (relation difference Operand Operand) declares 2 — a form headed by a declared relation has as many arguments as its declaration has types
```

Every other pair loads flat, and **the calculator clashes with nothing shipped** but that one name: a clash
with it needs a name of a program's own, or of a theory written without it in mind (P2a's `chores`). Neither
pair, nor P11, is said on a page: [E-50](findings.md), filed.

### The names real programs use that a module declares

Every tracked program under `examples/`, `stdlib/` and `tests/` that does not bring the module, at any depth,
and the names it declares, states a fact of or derives that the module declares itself: each a name a flat
import of the module would meet in that program. *Declare* is refused by the load unless the two bodies are
one; *write* joins the module's relation, or is refused by its count.

| module | programs | the names, programs each (declaring) |
|---|---:|---|
| `std.arith` | 24 | `b` 8 (5), `n` 8 (2), `check` 3 (0), rule `add-column` 2 (2: `cryptarithm.ein` and `send_more_money.ein`), `add` 2 (1), `difference` 2 (0: `std.algebra` and its test, the rule above), `sum-of` 1 (1) |
| `std.algebra` | 35 | `functional` 21 (18), `symmetric` 17 (16), `transitive` 16 (16), `total` 7, `injective` 4 (4), `bijective-properties` 3 (3), `surjective` 3, and six of one each |
| `std.bijection` | 11 | `domain-elimination` 7 (5), `typecheck-arg-0` and `-1` 6 (4), `injective-negative` 5, `functional-negative` 4, `range-elimination` 3 |
| `std.elim` | 9 | `typecheck-arg-0` and `-1` 7 (4), `domain-elimination` 6 (5), `no-room-left` 1 |
| `std.num9` | 4 | `big` 4 (4) |
| `binary_logic.inputs` | 222 | **`is-a` 218 (108)**, `in` 4 (4) |
| `word_problems.words` | 37 | **`holds` 28 (28)**, `wanted` 4 (2), `together` 3, `asked` 1 (1), `apart` 1 |
| `binary_logic.props`, `props_rules` | 4, 6 | `column` 4 (4), and the properties' names in one program each |
| `binary_logic.circuit` | 6 | `gate` 4 (3), `val` 2 (2) |

(`std.digits`, `std.macro`, `bits` and `bin` one program each.) Most of the zebra modules' rows are programs
that write their own copy of a generic rule, which the stdlib's README describes as the *variant copies*;
those load flat only where the copy is equal, body and `:why`. **The rows that say the most are the theories':**
the example `inputs` declares `is-a`, the corpus's most used name, so a zebra program cannot import the logic's
inputs flat; `words` declares `holds`; and `std.arith`'s one-letter `b` and `n` are the names of its most met,
eight programs each, where `done`, the clash the question named, is met by none.
