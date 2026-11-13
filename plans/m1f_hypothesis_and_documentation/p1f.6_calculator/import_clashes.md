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

## 0. In short

- **Today** (§ 2). A flat import shares by name. That is what connects theories, the user's *"already
  connected"*: one `fa`, one `n` (P1, P6, where every shipped theory loads flat beside every other). And it is
  what clashes: a program's `done`, `sum-of`, `b` or rule `add-column` beside the calculator is refused (P2, P4),
  and a written `(done laundry today)` joins the calculator's `done` in silence (P4d). `:as` is the way out and
  shares nothing: the tables twice, numbers that do not meet (P3, P5, P10); a missing prefix is silence (E-49).
- **The census** (§ 3). `std.arith` occupies 74 relations and 116 rules, 48 relations its own. Of 325 pairs of
  stdlib modules and example theories, 2 cannot be imported flat together, and `std.algebra`'s `difference`
  cannot be activated beside the calculator (E-50). 24 corpus programs use a name of the calculator's, `b` and
  `n` most, `done` none; the example `inputs` occupies `is-a`, which 218 programs use.
- **The proposals** (§ 4): (a) a module says what it exports, (b) a module is one copy whoever imports it, (c)
  a rename on import, (d) a convention, (e) nothing, (f) (a) to (c) as one mechanism, (g) four ways for E-32.
  Each computed on a prototype where module text can be one.
- **Recommended** (§ 5): (f), in two steps, (a) and (b) first; (g1) after.
- **The questions** (§ 6): [Q-M1f.231](../open_questions.md#q-m1f231--which-way-the-import-mechanism-closes-the-gap)
  to [Q-M1f.236](../open_questions.md#q-m1f236--e-49-and-e-50), open, the user's. **The stage stops here**:
  T6, the fix, is planned to the ruling.

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

## 4. The proposals (T4)

Five, as the stage file lists them, and a sixth that is three of them as one mechanism. Each is shown on the
probes of § 2, **computed on a prototype written as module text** where one can be written (`build/s1f.6.30/
proto.py` writes them, and `build/s1f.6.30/proposals.sh` runs the probes on them, the release binary at
`2ef68530`, 2026-11-13), and marked *not computed* where it needs the loader. What would move is measured by
two sweeps of the 133 corpus programs that bring `std.arith` (`build/s1f.6.30/sweep.py VARIANT`: `ein solve`
bare, and `ein saturate --dump`, which prints every fact, each with the checkout's stdlib and with the
prototype's under `$EIN_STDLIB`). The digests of `ein-render`'s `corpus_shapes.md5` cannot be taken under a
prototype, since that test refuses `$EIN_STDLIB`: the 133 programs hold 5 520 of its 24 479 digests, and
those that render the resolved program, the KB, a rule or a trace can move, the program's own parse cannot.

The prototypes are of `std.arith` alone, the module the question is about; `std.int` and `std.rat` would take
the same treatment.

### (a) What a module exports

**The change.** A module says which of its names a program sees: a top-level form, say `(export add sub cmp
mul div let n sum-of … done operand-is)`. Every other name it declares, relations and rules, is its own: the
loader renames it under the module's name, `std.arith.strip`, before the import is spliced, so no program
meets it. A module that writes no `export` exports everything, as today.

- **`ein.y`:** one top-level form, `(export NAME+)`, beside `import`; `01_grammar.md`'s form table, one row.
- **`ein_ir::imports`:** after a module is resolved and before `select`, its declared names that its `export`
  does not list are renamed `MODULE.NAME` by `qualify`'s own `rename_atoms`, with that mapping alone; a
  qualified import prefixes what is left, as today.

**The prototype,** `proto/exports/stdlib/arith.ein`: of `std.arith`'s 74 relations, 26 exported (the requests
and `let`, the results, the flags, `n`, the interface's eleven) and 48 renamed `std.arith.X`; of its 116 rules,
the entry `arith` and `division-by-zero-is-false` exported (`std.num9` selects that rule by name: an export
list must hold what another module selects) and 114 renamed. `ein test` over `examples/arith/` and
`tests/stdlib/{arith,int,rat}/` holds 176 of 178 expectations, where the checkout's stdlib holds 182 of 182;
the two it does not, and the one file it cannot check, are the three claims that name the theory's own tags,
`(done (sum a b) …)`, which would be written `(done (std.arith.sum a b) …)`. The first prototype hid
`operand` and `operand-is` too and failed sixteen claims of `std.int` and `std.rat`, which call them: that is
how T1 learned they are the interface.

| probe | today | under (a) |
|---|---|---|
| P4e, the program's rule `add-column` | refused | **loads**: `(col units 2 1)` and `(sum-of (n z 7) (n z 5) (n (n z 1) 2))` |
| P4f, the program's relation `b` | refused | **loads**: `(b one)` and the sum |
| P2a, `chores`' `done`; P4a, the program's `done` | refused on `done` | **refused on `done`**: `done` is exported, an answer of a tagged call |
| P4b, the program's `sum-of` | refused | refused: a result is exported |
| P4d, `(done laundry today)` | joins the calculator's `done` | joins it: `(done (std.arith.sum (n z 2) (n z 3)) (n z 5))` beside it |
| P4g, a malformed request | `refuted by malformed-is-false` | `refuted by std.arith.malformed-is-false` |
| P11, `std.algebra`'s rule `difference` activated beside the calculator | refused by the count | **loads**: the tag is `std.arith.difference`; the test's claim *holds* |

**What moves.** On the 133 programs: `ein solve`'s read-out on 17, R on none: 14 by the `refuted by` line, the
malformed-request and undeclared-name tests of `tests/stdlib/arith/` (22 to 29, 31 to 33, 35 to 37), and 3
whose claims name a tag (`07_only_what_is_asked`, `08_sub`, `15_mul_long_and_named`); `ein saturate --dump`
on 111; `corpus_shapes.md5`, up to 5 520 digests. Of the 24 corpus programs that meet a name of `std.arith`
(§ 3), the meeting goes for 14 and stays for 10, on `n`, `add` and `sum-of`. **E-8:** no change. **E-20:** no
change; the closure still follows names. **E-32:** no change.

### (b) What `:as` shares: a module loaded once, whoever imports it

**The change.** A qualified import prefixes the names its module declares itself, and not those of the
modules it imports, which keep the names the rest of the program gives them. One module, imported by two
paths or under two prefixes, is one copy.

- **`ein.y`:** nothing.
- **`ein_ir::imports`:** `qualify`'s mapping takes the module's own declarations, not the resolved list's; the
  modules it imports are resolved once for the program, by their path, and spliced once.

**The question it raises,** which the prototype answers by hand and the mechanism must answer by rule: what a
module's names are called when the only import that brings it is qualified. Bare, they meet the program's
names (P4 again, on `fa`); under the module's own name, `std.digits.fa`, they need (a)'s renaming. So (b) is
whole only with (a).

**The prototypes.** `proto/share/arith_shared.ein` is `std.arith` with its `(import std.digits …)` deleted,
so its tables are the importer's; `rat_shared.ein` is `std.rat` with its two imports deleted and every name of
`std.int` and `std.arith` it writes read as the program's `i.` one.

| probe | today | under (b) |
|---|---|---|
| P3b, `succ` flat, the calculator `:as a` | `fa` and `a.fa`, `a.fs`, `a.md`, `a.cl`: 2 044 facts | `fa`, `fs`, `md`, `cl` once: 1 842 facts; `(bumped 6)` as before |
| P5, the calculator flat and `:as a` | 3 656 facts, 232 rules | **1 978 facts**, 232 rules; `(sum-of … (n z 5))` and `(a.sum-of … (a.n z 5))` as before |
| P10, `std.int :as i`, `std.rat :as r`, `i`'s sum handed to `r` | 3 943 facts, 432 rules; **no answer** for `(r.rat-simple-of ?x ?s)` | 2 021 facts, 245 rules; **`(r.rat-simple-of (r.rat (i.int (i.n z 3) z) (i.n z 1)) …)`: connected**, the rationals' integers being `i`'s |

The P10 program under (b), `proto/share/b10_stack_shared.ein`:

```lisp
(import std.int :as i)
(import rat_shared :as r)
(i.int-add (i.int z (i.n z 2)) (i.int (i.n z 5) z))
(r.rat-add (r.rat (i.int (i.n z 1) z) (i.n z 2)) (r.rat (i.int (i.n z 1) z) (i.n z 3)))
(rule int-to-rat ()
  :match  (i.int-sum-of ?x ?y ?s)
  :assert (r.rat-simplify (r.rat ?s (i.n z 1)))
  :why    "the integer sum {?s} as a rational, over one")
(query :goal (r.rat-simple-of ?x ?s))
```

```text
  query 3 of 3      (r.rat-simple-of ?x ?s)
                      ?s = (r.rat (i.int (i.n z 3) z) (i.n z 1)) · ?x = (r.rat (i.int (i.n z 3) z) (i.n z 1))
```

**What moves.** No corpus program's answer: no tracked program imports, qualified, a theory that imports
another (the corpus's qualified imports are the import mechanism's fixtures, whose modules import nothing).
**E-8:** no change. **E-20:** no change alone. **E-32:** no change, but a module's identity becomes its path,
which a fix of E-32 needs too.

### (c) Renaming a name on import

**The change.** `(import chores :symbols (chores) :rename ((done chore-done)))`: the listed names of the
module are renamed for this import, everywhere its forms write them.

- **`ein.y`:** a third key of an import's clause, `:rename`, a list of pairs of names, beside `:symbols`.
- **`ein_ir::imports`:** `import_spec` reads it; `resolve` applies the mapping to the module's resolved forms by
  `rename_atoms` before `select`; a left-hand name the module does not declare is refused, as a `:symbols` name
  is.

**The prototypes,** the texts such an import would make: `proto/rename/chores_renamed.ein` (`done` as
`chore-done`) and `arith_done_renamed.ein` (`std.arith` with `done` as `arith-done`).

| probe | today | under (c) |
|---|---|---|
| P2a, `std.arith` and `chores` flat | refused on `done` | **loads**, `chores` renamed: `(sum-of … (n z 5))` and `(to-do dishes)`, the program writing `(chore-done laundry)` |
| P4a, the program's `done` | refused | **loads**, `std.arith`'s renamed: `(sum-of … (n z 5))` and `(done laundry)` |
| P4a and `std.int` flat | refused | **refused**: *conflicting definitions of 'strip-none': same name, different body* |

**The last row is computed and is the trap.** A rename rewrites one import's copy of a module; `std.int`
imports `std.arith` itself, unrenamed, so the program holds two calculators whose rules differ in one name and
are refused on the first rule that writes it. A renamed theory cannot sit beside a theory built on it. Under
(b), one copy, a rename would be a second name for the one relation, and the trap goes: *not computed*.

**What moves.** Nothing: no program writes `:rename`. **E-8, E-20, E-32:** no change.

### (d) A convention only: the module renames its own

**The change.** None to the language: question 8's option (b), the theory's names prefixed in its own text.
Two depths:

- **(d1)** the circuit's own relations, forty-seven, `b` as `arith-b`, `strip` as `arith-strip`, the tags as
  `arith-sum`; rules and the interface unchanged (`proto/convention/stdlib`);
- **(d2)** and the interface's eleven, `arith-done`, `arith-add-walk`, `std.int` and `std.rat` following,
  fifty-eight (`proto/convention-all/stdlib`).

| probe | today | (d1) | (d2) |
|---|---|---|---|
| P4f, the program's `b` | refused | **loads** | **loads** |
| P2a, P4a, `done` | refused | refused: `done` is the interface's | **load** |
| P4e, the program's rule `add-column` | refused | refused: rules keep their names | refused |
| P4g's core | `refuted by malformed-is-false` | unchanged | unchanged |
| P11, `std.algebra`'s `difference` | refused | **loads**, the claim held: the tag is `arith-difference` | loads |

**What moves.** (d1): `ein solve` on 3 of the 133, the three claims that name a tag; `ein saturate --dump` on
111; `corpus_shapes.md5` up to 5 520 digests; R on none. Of the 24 corpus programs meeting a name, 12 are
spared, 12 not (`n`, `add`, `sum-of`, `add-column`). (d2): besides, every program that writes the interface,
18 files by T1's test (eight under `examples/beyond_arith/`, `conv.ein`, `std.int`, `std.rat`, six under
`tests/stdlib/arith/`), rewritten; until they are, `ein solve` moves on 13. And `examples/binary_arithmetic/
bin.ein`, generated from `std.arith`, follows its generator. **E-8, E-20, E-32:** no change. It is what every
other shipped module already does (`int-`, `rat-`, `…9`, `…2`), and it reaches only the names the module
chooses: a program's `n`, `add` or `sum-of` still meets the calculator's.

### (e) Nothing, and how to mix theories today

Nothing moves. What a program does today, each line a probe of § 2:

1. **Import theories flat and give your own relations names of their own**, as every shipped module does
   (P6): the tables and the numbers are then shared, and the theories connect (P1).
2. **Where a name is taken, import one of the two `:as`** (P3a, P3d), and write its relations, its tagged
   calls *and its numbers* with the prefix (P7a): `(a.add-walk TAG (a.n z 7) (a.n z 3) 0 z)`. A query with no
   row is the first sign of a missing prefix (P7b, P7c, E-49).
3. **Do not import one theory twice, flat and `:as`** (P5): every table twice, and two kinds of number that
   do not meet (P3c).
4. **A theory over the calculator imports it flat, by its entry, in its own text**, as `std.int` and `std.rat`
   do; two theories that must hand numbers to each other are imported flat into the program (P10's are not).
5. **Two modules in sibling directories** cannot import each other (P9, E-32): a copy, as
   `examples/binary_logic/bits.ein` is, or a program above both that imports them by dotted name.
6. **`std.elim` and `std.bijection`** are alternatives, never one program's (E-50); nor `std.algebra`'s
   `difference` beside the calculator.

### (f) One mechanism: a module's names are its own, and an import shows what it exports

(a), (b) and (c) as one design, the one most module systems have:

- every name a module declares lives under the module's name, `std.arith.strip`, `std.arith.done`, and is
  printed so where a surface shows a fact or a rule the program did not write;
- an import shows the module's **exports**, bare for a flat import, under the alias for `:as`; a module loaded
  by two imports is **one copy** (b), so the tables and the numbers are shared whatever the prefixes;
- **two flat imports that export one name are refused only where the program writes that name**, with a line
  that names both modules and the two remedies, `:as` and `:rename`; where the program does not write it, the
  two theories each keep their own and nothing clashes (P2a loads unless the program writes `done`);
- **`:rename` is a second name for the one relation** (c), so a renamed theory sits beside a theory built on it.

**Computed:** each part on its prototype, above: P4e and P4f load under (a), P10 connects under (b), P2a loads
under (c); its answers are theirs. **Not computed:** the refusal where an exported name is written and two
modules export it, and the rename of the one copy, which need the loader.

- **`ein.y`:** `export`, and `:rename`.
- **`ein_ir::imports`:** the resolution keeps one copy of a module by its path and names its declarations under
  the module's name; an import maps the exported names to the bare or aliased ones; the check of two bodies
  becomes a check of two exporters of a name the program writes.
- **What moves:** as (a), every surface that prints a non-exported name (`ein saturate --dump` on 111 of 133,
  `refuted by` on 14, three claims); nothing else, since a module that writes no `export` exports everything.
- **E-20 goes:** a module's own imports are the module's, loaded as it wrote them; an importer selects among
  its exports and cannot leave the lower theory's rules behind. The note S1f.6.25 added would not fire.
  *Not computed.* **E-8:** no change. **E-32:** a module named by its path is a module wherever it lies, so
  `(import ../x/m)` costs nothing more (g).

### (g) A sibling directory (E-32)

Named here, ruled with the rest, not fixed as a side effect. Today a file-relative import reaches the
importing file's directory and below (P9), so `examples/binary_logic/bits.ein` is a copy of
`examples/binary_arithmetic/bits.ein`, and the probes of P6 reach both through symlinks.

- **(g1) `..` in a module name**, `(import ../binary_arithmetic/bits …)`: `locate` resolves the path segment by
  segment and refuses a path outside the checkout's root; the copy goes. No grammar: `ein.l`'s `NAME` takes
  `.`, `/` and `-` already, which is why P9 is refused by the resolver and not by the parser.
- **(g2) A root.** A dotted name resolves under the nearest directory above the importing file that holds a
  marker (`corpus/`, or a file such as `ein.toml`): `(import examples.binary_arithmetic.bits …)`. No grammar.
- **(g3) A search path,** `$EIN_PATH`, as `$EIN_STDLIB` is for the stdlib. No grammar; a program's meaning
  then depends on the environment.
- **(g4) Nothing:** the copy stays, generated by `gen_binary.py` with a `--check`.

## 5. The recommendation

**(f), in two steps, and (g1) after the first.** The user named the clash as the import mechanism's gap, and
(f) is the mechanism closing it: what connects theories today, a shared name, stays where a module means to
share it, and stops where it does not.

1. **First (a) and (b), together.** A module may write `(export …)`; what it does not export is named under the
   module, `std.arith.strip`; a module is one copy per program, whoever imports it. **What it buys, computed:**
   the clash on every name the calculator does not export goes (P4e, P4f, P11; 14 of the 24 corpus programs
   that meet a calculator name), and `:as` shares the tables and the numbers (P5 1 978 facts where 3 656, P10
   connected). **What it costs, computed:** `ein solve` moves on 17 of the 133 programs, 14 `refuted by` lines
   that name a rule of the calculator's own and 3 claims that name a tag, R on none; every dump and trace of
   111; up to 5 520 digests of `corpus_shapes.md5`. `std.int`, `std.rat`, `std.num9`, the binary theory and
   `words` each write their list in the same step, and [S1f.6.31](s1f.6.31_the_stdlib_layout.md)'s move, which
   renames modules, would then rename what the hidden names print (`std.math.decimal-arith.strip`): the two
   are best done in one order, (a) and (b) before the move or after it, and not across it.
2. **Then an exported name that two flat imports both export:** refused only where the program writes it, with
   a line that names the two modules and the two remedies, `:as` and `:rename`; and `:rename` a second name of
   the one copy, so (c)'s computed trap goes. P2a then loads where the program does not write `done`, and is
   refused with a line that says what to do where it does. *Not computed.*
3. **E-32 by (g1),** `..` in a module name, once a module is its path: the copy of `bits.ein` goes.

And with them: **E-49** gets a `note:` line, as E-20 did, where a program writes bare a name that only an
alias exports, or declares nothing for a relation that a qualified theory exports (*not computed*); **E-50**'s
first pair stays two alternatives, said in both headers, since a program activates their rules by name and so
both export them; the logic's `complement` and `identity` are renamed by its generator, an example's; and
`std.algebra`'s `difference` is cured by step 1, the tag being the calculator's own (P11 loads under (a)).

**Why not the others.** (d) is a convention the shipped modules already keep (`int-`, `rat-`, `…9`, `…2`),
and it is the theory dodging by spelling, which the reply set aside: (d1) spares 12 of the 24 programs and
moves every dump; (d2) spares P2a too and rewrites 18 files, and `n`, `add`, `sum-of` still meet a program's.
(c) alone has the computed trap. (b) alone leaves a module that only a qualified import brings with no name of
its own. (e) leaves P4e as it is: a cryptarithm with a rule `add-column` cannot check its sums with the
calculator.

## 6. The questions

Filed in [`open_questions.md`](../open_questions.md), **open, the user's**, none defaulted: the stage stops for
them.

- **[Q-M1f.231](../open_questions.md#q-m1f231--which-way-the-import-mechanism-closes-the-gap). Which way does
  the import mechanism close the gap?** (a), (b), (c), (d1), (d2), (e) or (f) of § 4. *Recommended: (f), in
  two steps, (a) and (b) first.*
- **[Q-M1f.232](../open_questions.md#q-m1f232--how-a-module-says-what-it-exports-and-what-stdarith-exports).
  How does a module say what it exports, and what does `std.arith` export?** A form `(export NAME+)`; the
  entry's names; or a mark on a name. And the list: the prototype's 26 relations and 2 rules, `done` among
  them as the interface's answer, or the interface renamed as it is exported. *Recommended: the form, and the
  prototype's list.*
- **[Q-M1f.233](../open_questions.md#q-m1f233--how-a-name-a-module-does-not-export-is-printed). How is a name
  a module does not export printed** in a state, a trace and a core? `std.arith.strip`; bare, the module said
  apart; or a name the loader makes up. *Recommended: `std.arith.strip`, as a bare qualified import prints it
  today.*
- **[Q-M1f.234](../open_questions.md#q-m1f234--two-flat-imports-that-export-one-name). Two flat imports that
  export one name:** refused at load, as today; refused only where the program writes it; or the later wins.
  *Recommended: where the program writes it.*
- **[Q-M1f.235](../open_questions.md#q-m1f235--a-module-in-a-sibling-directory-e-32). A module in a sibling
  directory (E-32):** (g1) `..`, (g2) a root, (g3) a search path, (g4) the copy. *Recommended: (g1), after (b).*
- **[Q-M1f.236](../open_questions.md#q-m1f236--e-49-and-e-50). E-49 and E-50:** a `note:` where a bare name is
  one only an alias exports, or nothing; the two alternatives said in their headers, the logic's two rules
  renamed, `difference` by step 1. *Recommended: the note, and the three as said.*


## 7. The ruling, 2026-11-13: (h), full names and short names

The user ruled in conversation on 2026-11-13, after reading § 0 to § 6, with a model of their own that the
page had not proposed. In their words, in order:

> flat import has to import listed symbols unrenamed, but remain (private) symbols standard way - with
> std.module... path; would that solve our issue?

> let's call symbols full names `std.math.arith.decimal...` and call short names short; […] 1. it names what
> it explicitely calls, all other names are pulled-in automatically (if required) with full names; but they
> still work, engine must remember that std.math.arith.decimal.add here == just `add`; yes, it is alias; 2. if
> it happens in the same file - it's an error, not correct ein code, just like ordinary namespaces; 3. Yes, we
> can accept it now, silent re-definition, like variables shadowing, it's a mistake. 4. Ok, is that an issue?
> 5. It make sense that if user imports specific name then he wants only them and don't want any background
> side effects of this import, and if import whole module with or without :as - then wants everything; 6.
> before; […] full names can call global and short names can call local, because they are local in the file,
> OK?

> 1 - yes (ii); module canonical identity should work fine, std has one defined root, other imports are
> relative to the program file, `..` going outside the folder is restricted so there is always one single
> path; ignore potential problems with symlinks and hardlinks - it is out of the scope, if user is smart
> enough to make them - import problems are his own;

The numbers in the second quotation answer the coordinator's list of what (h) left open; (ii) in the third is
the closure's direction, below. **The model, (h):**

- **A full name is the name.** Every name a module declares has one full name, its module's canonical name
  and the short name, `std.math.decimal-arith.add` once [S1f.6.31](s1f.6.31_the_stdlib_layout.md) has moved
  the modules (`std.arith.add` until then). It is the same whoever imports the module, through whatever path,
  so a module is **one copy per program**: every path to it reaches the same declarations under the same
  names, and they are one. Full names are global: any file may write one.
- **A short name is an alias, local to the file that imports.** `(import M :symbols (add sum-of))` lets that
  file write `add` for M's full name `….add`. It renames nothing in M. A module's own file writes its own
  declarations short; an importer never sees another file's short names (**no re-export**).
- **What an import brings.** A selection brings the listed names and **what they require** (ruling 1 (ii)):
  the closure follows a kept rule to the relations it writes, as today, **and a listed or kept relation to the
  rules that read it or conclude it**, transitively through M's own names, so `:symbols (add sum-of)` brings
  the circuit that answers `add` and nothing that no listed name needs: *"no background side effects"*. A
  whole-module import, bare or `:as`, brings everything (5). What a program holds is the union over its import
  edges, each by full name.
- **`:as a`** is a second alias prefix: `a.add` is the full name `….add`; it makes no copy.
- **Errors, in one file** (2): one short name bound to two full names, by two imports or by an import and the
  file's own declaration, is refused at load. Binding one short name to one full name twice is not an error.
- **Shadowing is the writer's** (3): a short name a file writes and no import of it lists is the file's own,
  whatever a module it imports declares under that short name; nothing is said. E-49 is accepted as this.
- **Module identity** (ruling 1, made precise the same day):

> imports are "relative to the program file" - means examples/word_problems may contain subdirectory
> word-theory with files there that would be imported with (import word-theory.gaps :as gaps) having global
> names like word-theory.gaps.gaps

  The stdlib has one root, `std.`; every other module name is a path **relative to the program file's
  directory**, a dot for each directory, for every import in the program, those written inside a module in a
  subdirectory included: `word-theory/gaps.ein` imports its sibling as `word-theory.words`, not `words`. (Today
  a module's own imports resolve against its own directory, `ein_ir::imports::locate`.) A declaration's full
  name is its module's name and its short name: `word-theory.gaps.gaps`, which `:as gaps` lets the program
  write `gaps.gaps`. `..` stays refused, so one file has one name in a program. Symlinks and hard links are out
  of scope. A short name a module declares holds no dot (Q-M1f.241), so a full name splits one way; no tracked
  program declares one with a dot (2026-11-13).
- **Before the layout move** (6): the fix lands before [S1f.6.31](s1f.6.31_the_stdlib_layout.md), so the move
  renames full names once, after them.

**How the user's dependency question works** (*"A <import- B <import- C <import- Prog, A <import- Prog, A
<import- E <import- Prog; How it works now and how it should work?"*), probed on the release binary at
`2ef68530` on the coordinator's scratch modules (a module `A` with a relation `a-in`, a rule `a-step` that
concludes `a-out`, an entry rule `a` naming `a-step`, and a fact `(a-in seed)`; `B`, `C` and `E` each import
the one below flat by its entry and pass its output on by a rule of their own):

| program | today | under (h) |
|---|---|---|
| all flat, the user's graph | A's forms arrive three times, are equal, and merge: **one copy**; the program's `(a-in mine)` joins A's | one copy, by full name; the program writes `a-in` only if its own import of A lists it |
| the program imports C alone and writes `a-in`, reads `a-out` | **the short names leak**: both connect to A's rule, A never imported by the program | `a-in` and `a-out` are the program's own (3); A's are `A.a-in`, reachable by full name |
| the program declares its own `a-out` and imports C alone | refused, *conflicting definitions of 'a-out'* | loads: its `a-out` is its own |
| the program imports A qualified, the rest flat | **two copies**, `A.a-in`/`A.a-step` and `a-in`/`a-step`, each with its seed, not connected | one copy |
| `(import C :as cc)` beside a flat A | **two copies**: C's whole subtree prefixed, `cc.a-in` | one copy; `cc.` aliases C's own names |
| a qualified import inside a qualified one | the prefix follows the path: `Bq.A.a-in` | the defining module's: `A.a-in` |
| a cycle X → Y → X | refused, *import cycle: X.ein -> Y.ein -> X.ein* | refused, as today |
| `(A.a-in x)` written with A imported flat | a new relation `A.a-in`, unconnected, in silence | A's own `a-in`, by its full name |
| a program declares `(relation std.arith.mine …)` | loads: a name with dots is a name | refused where the program imports that module (default below) |

**Defaults taken for what the user's words leave open,** each the user's to revise (Q-M1f.237 to .241):

- **How a name prints** (Q-M1f.233, which the user asked *"is that an issue?"* of and the coordinator answered:
  a one-time move of outputs, R unmoved): a state, a trace and a core print the **main file's short name**
  where it has one, the full name otherwise.
- **A full name of a module no import brings** is refused at load, naming the module; it is not an implicit
  import.
- **A file declaring a name under the full name of a module it imports** is refused at load. A `--dump-answer`
  state file, which imports nothing and declares the full names it was loaded with, loads as today.
- **Entry rules and `__closed__` lists stay** in the shipped modules: (ii) makes an entry rule unnecessary for an
  import, and keeping them moves nothing a program reads.
- **A declared short name holds no dot,** refused at load, so a full name has one reading.

**What it decides of the other questions:** Q-M1f.231, (h); Q-M1f.232, no export list, the importer chooses;
Q-M1f.234, refused in one file; Q-M1f.235, `..` stays refused and E-32 stays as stated, the copy of `bits.ein`
going with S1f.6.16's build, which moves the binary tables into the stdlib; Q-M1f.236, E-49 accepted as shadowing, and E-50 dissolved by full names: `std.bijection` and `std.elim`
and the logic's two rules coexist, and a program that lists a clashing short name from both is refused (2).
**The fix is [S1f.6.35](s1f.6.35_imports_by_full_name.md),** planned to this section, after S1f.6.34 and
before S1f.6.31.
