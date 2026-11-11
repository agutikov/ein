# P1f.6 — the design proposal: a calculator in Ein, shown on a running prototype

**Stage:** [S1f.6.1](s1f.6.1_design_proposal.md) of [P1f.6](README.md). Written 2026-11-11.
**This page stops for the user.** Nothing under `stdlib/`, `examples/`, `tests/` or `corpus/` was written, and the
engine is unchanged.

**The prototype** is under `build/s1f.6/prototype/`, which git ignores, so its files are named here as paths and
not as links. Its `README.md` lists every script and the command that re-takes each number on this page. Every
answer below is pasted from a run of `ein.rs/target/release/ein`, built from the tree at `25b4af05`, on
2026-11-11. None is written by hand.

## The result in one paragraph

A theory of 20 rules and 21 relations (`grep -c '^(rule' programs/arith9.ein`, 2026-11-11) computes the user's
four operations on `(num9 d8 … d0)` numbers by saturation alone. All eight target programs run. On each one
`ein solve` reports *universe: no candidate*, k_s 1 and R {∅}; `ein test` holds every `:expect`; and
`ein saturate` stores the same answer. The theory writes no `absent` and no `not`, and the dump reports
*stratified, 1 stratum*. Held to Python's integers, it is right on every operand pair at width 2 and on a random
sample at widths 4 and 9 ([§ Right](#right-held-to-pythons-integers)). Nothing needs a hypothesis. Three things
the user should know before ruling are in [§ Findings](#findings): a flat import drops a module's facts, so the
tables cannot be written as facts; the theory can close its own relations and not the importing program's; and a
number written with the wrong count of digits loads and is answered by silence.

## The target programs

Each program is the whole file. Its import is file-relative on the prototype, `(import arith9 :symbols (arith))`;
question 8 decides what a program writes in the end.

### One per operation

`programs/add.ein`, 123078 + 4567:

```lisp
(import arith9 :symbols (arith))

(add (num9 0 0 0 1 2 3 0 7 8) (num9 0 0 0 0 0 4 5 6 7))

(query :goal (sum-of ?x ?y ?s)
       :expect (model (sum-of (num9 0 0 0 1 2 3 0 7 8) (num9 0 0 0 0 0 4 5 6 7) (num9 0 0 0 1 2 7 6 4 5))))
```

`ein solve programs/add.ein`, whole (`out/add/solve.txt`):

```text
solve · programs/add.ein
──────────────────────────────────────────────────────────────
  asked             all of R — describe
  states (k_s)      1
  commitments (k_c) 1
  exhausted         yes — every commitment is classified: this is all of R
  ∅                 in R — root is alive and owes nothing
  minimal line      1 state — ∅
  maximal line      1 state
  components        1
  sub-cubes         1, of dimension 0
  merged            none — every state is reached by one commitment, so k_c = k_s
  N                 0 minimal dead sets
  lattice           monotone (the static check) — R is convex: no hole
  universe          no candidate: R is {∅} or empty, decided by root alone
  route             the state route, every alive state once — L1 dead is upward-closed · L3 a commitment's state is its closure's · L4 the lines are local
  work              1 saturation · 0 facts beyond root · 2 layers

  (region
    ;; state 1 of 1 — ∅, root itself · minimal and maximal
    (model))

  query             (sum-of ?x ?y ?s)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?s = (num9 0 0 0 1 2 7 6 4 5) · ?x = (num9 0 0 0 1 2 3 0 7 8) · ?y = (num9 0 0 0 0 0 4 5 6 7)
  as :expect        (model (sum-of (num9 0 0 0 1 2 3 0 7 8) (num9 0 0 0 0 0 4 5 6 7) (num9 0 0 0 1 2 7 6 4 5)))
  :expect           holds
```

`ein test -v programs/add.ein` (`out/add/test.txt`):

```text
ok           programs/add.ein
  :goal (sum-of ?x ?y ?s) — holds (all of R, 1 class over 1 states)
1 file, 1 expectation: 1 held, 0 FAILED, 0 not checked, 0 errors  (0.02 s)
```

`ein saturate --dump programs/add.ein` stores `(sum-of … (num9 0 0 0 1 2 7 6 4 5) :rule add-fits)` in 6 firings,
five of them the four tables and the closing rule (`out/add/saturate.txt`). `--trace -G` ends on the one step that is the answer
(`out/add/trace-G.md`):

```text
### Step 6 — `add-fits`

> ripple-carry: each column's digit and carry from the table, no carry out of the top

Premises: `add(num9(0, 0, 0, 1, 2, 3, 0, 7, 8), num9(0, 0, 0, 0, 0, 4, 5, 6, 7))`, `fa(8, 7, 0, 5, 1)`, `fa(7, 6, 1, 4, 1)`, `fa(0, 5, 1, 6, 0)`, `fa(3, 4, 0, 7, 0)`, `fa(2, 0, 0, 2, 0)`, `fa(1, 0, 0, 1, 0)`, `fa(0, 0, 0, 0, 0)`, `fa(0, 0, 0, 0, 0)`, `fa(0, 0, 0, 0, 0)`

Derives `sum-of(num9(0, 0, 0, 1, 2, 3, 0, 7, 8), num9(0, 0, 0, 0, 0, 4, 5, 6, 7), num9(0, 0, 0, 1, 2, 7, 6, 4, 5))` — (num9 0 0 0 1 2 3 0 7 8) + (num9 0 0 0 0 0 4 5 6 7) = (num9 0 0 0 1 2 7 6 4 5)
```

The other three operations have the same shape: one request fact, one query with its `:expect`. The request and
the binding row `ein solve` prints for each (`out/<program>/solve.txt`):

| program | request | the row `ein solve` prints |
|---|---|---|
| `sub.ein`, 123078 − 4567 | `(sub (num9 0 0 0 1 2 3 0 7 8) (num9 0 0 0 0 0 4 5 6 7))` | `?d = (num9 0 0 0 1 1 8 5 1 1) · ?r = (num9 0 0 0 0 0 0 0 0 0)` |
| `sub-rev.ein`, 4567 − 123078 | `(sub (num9 0 0 0 0 0 4 5 6 7) (num9 0 0 0 1 2 3 0 7 8))` | `?d = (num9 0 0 0 0 0 0 0 0 0) · ?r = (num9 0 0 0 1 1 8 5 1 1)` |
| `mul.ein`, 123 × 456 | `(mul (num9 0 0 0 0 0 0 1 2 3) (num9 0 0 0 0 0 0 4 5 6))` | `?p = (num9 0 0 0 0 5 6 0 8 8)` |
| `div.ein`, 127645 ÷ 25 | `(div (num9 0 0 0 1 2 7 6 4 5) (num9 0 0 0 0 0 0 0 2 5))` | `?q = (num9 0 0 0 0 0 5 1 0 5) · ?r = (num9 0 0 0 0 0 0 0 2 0)` |

Subtraction answers `(diff-of X Y D R)` and division `(quot-of X Y Q R)`: two numbers each, as the user asked.

Long division, as `ein saturate --dump programs/div.ein` stores it (`out/div/saturate.txt`). Each `div-at` fact
is one step of the school method: the step counter, the dividend's digits still to bring down, the remainder,
and the quotient so far.

```text
  (div-at (num9 0 0 0 1 2 7 6 4 5) (num9 0 0 0 0 0 0 0 2 5) 5 (num9 2 7 6 4 5 0 0 0 0) (num9 0 0 0 0 0 0 0 0 1) (num9 0 0 0 0 0 0 0 0 0) :rule div-step)
  (div-at (num9 0 0 0 1 2 7 6 4 5) (num9 0 0 0 0 0 0 0 2 5) 4 (num9 7 6 4 5 0 0 0 0 0) (num9 0 0 0 0 0 0 0 1 2) (num9 0 0 0 0 0 0 0 0 0) :rule div-step)
  (div-at (num9 0 0 0 1 2 7 6 4 5) (num9 0 0 0 0 0 0 0 2 5) 3 (num9 6 4 5 0 0 0 0 0 0) (num9 0 0 0 0 0 0 0 0 2) (num9 0 0 0 0 0 0 0 0 5) :rule div-step)
  (div-at (num9 0 0 0 1 2 7 6 4 5) (num9 0 0 0 0 0 0 0 2 5) 2 (num9 4 5 0 0 0 0 0 0 0) (num9 0 0 0 0 0 0 0 0 1) (num9 0 0 0 0 0 0 0 5 1) :rule div-step)
  (div-at (num9 0 0 0 1 2 7 6 4 5) (num9 0 0 0 0 0 0 0 2 5) 1 (num9 5 0 0 0 0 0 0 0 0) (num9 0 0 0 0 0 0 0 1 4) (num9 0 0 0 0 0 0 5 1 0) :rule div-step)
  (div-at (num9 0 0 0 1 2 7 6 4 5) (num9 0 0 0 0 0 0 0 2 5) 0 (num9 0 0 0 0 0 0 0 0 0) (num9 0 0 0 0 0 0 0 2 0) (num9 0 0 0 0 0 5 1 0 5) :rule div-step)
  …
  (muld-of (num9 0 0 0 0 0 0 0 2 5) 5 (num9 0 0 0 0 0 0 1 2 5) 0 :rule muld-row)
  …
  (quot-of (num9 0 0 0 1 2 7 6 4 5) (num9 0 0 0 0 0 0 0 2 5) (num9 0 0 0 0 0 5 1 0 5) (num9 0 0 0 0 0 0 0 2 0) :rule div-done)
```

### Overflow, and division by zero

`programs/overflow.ein` asks 999999999 + 1 and 123078 × 123078, and claims two flags and no number:

```lisp
(add (num9 9 9 9 9 9 9 9 9 9) (num9 0 0 0 0 0 0 0 0 1))
(mul (num9 0 0 0 1 2 3 0 7 8) (num9 0 0 0 1 2 3 0 7 8))

(query :goal (overflow ?request)
       :expect (model (overflow (add (num9 9 9 9 9 9 9 9 9 9) (num9 0 0 0 0 0 0 0 0 1)))
                      (overflow (mul (num9 0 0 0 1 2 3 0 7 8) (num9 0 0 0 1 2 3 0 7 8)))))
(query :goal (sum-of ?x ?y ?s)      :expect (model))
(query :goal (product-of ?x ?y ?p)  :expect (model))
```

```text
  query 1 of 3      (overflow ?request)
    class 1 of 1    1 state (1)
                      ?request = (add (num9 9 9 9 9 9 9 9 9 9) (num9 0 0 0 0 0 0 0 0 1))
                      ?request = (mul (num9 0 0 0 1 2 3 0 7 8) (num9 0 0 0 1 2 3 0 7 8))
  :expect           holds
  query 2 of 3      (sum-of ?x ?y ?s)
    class 1 of 1    1 state (1) — the goal has no row: undetermined here
  :expect           holds
  query 3 of 3      (product-of ?x ?y ?p)
    class 1 of 1    1 state (1) — the goal has no row: undetermined here
  :expect           holds
```

`programs/div-zero.ein` asks 127645 ÷ 0:

```text
  query 1 of 2      (division-by-zero ?request)
                      ?request = (div (num9 0 0 0 1 2 7 6 4 5) (num9 0 0 0 0 0 0 0 0 0))
  :expect           holds
  query 2 of 2      (quot-of ?x ?y ?q ?r)
    class 1 of 1    1 state (1) — the goal has no row: undetermined here
  :expect           holds
```

An overflow and a division by zero are a fact that names its request, and no number is stored for either. Root
stays alive, so other requests in the same program are answered.

### A chain: (123 × 456) ÷ 25

The program writes one rule of its own. The product is the dividend, and the same saturation computes both
(`programs/chain.ein`):

```lisp
(import arith9 :symbols (arith))

(mul (num9 0 0 0 0 0 0 1 2 3) (num9 0 0 0 0 0 0 4 5 6))

(rule then-divide ()
  :match  (product-of (num9 0 0 0 0 0 0 1 2 3) (num9 0 0 0 0 0 0 4 5 6) ?p)
  :assert (div ?p (num9 0 0 0 0 0 0 0 2 5))
  :why    "the product {?p} is the dividend: ask for it divided by 25")

(query :goal (quot-of ?x ?y ?q ?r)
       :expect (model (quot-of (num9 0 0 0 0 5 6 0 8 8) (num9 0 0 0 0 0 0 0 2 5) (num9 0 0 0 0 0 2 2 4 3) (num9 0 0 0 0 0 0 0 1 3))))
```

```text
  universe          no candidate: R is {∅} or empty, decided by root alone
  query             (quot-of ?x ?y ?q ?r)
    class 1 of 1    1 state (1)
                      ?q = (num9 0 0 0 0 0 2 2 4 3) · ?r = (num9 0 0 0 0 0 0 0 1 3) · ?x = (num9 0 0 0 0 5 6 0 8 8) · ?y = (num9 0 0 0 0 0 0 0 2 5)
  :expect           holds
```

56088 ÷ 25 is 2243, remainder 13. A chain is a rule whose `:match` reads one answer and whose `:assert` states the
next request. That is the whole protocol for composing operations, and it is what the theory itself does inside
multiplication and division.

### The search-free check, and what each run costs

`./run.sh` (2026-11-11) runs every program under `ein solve`, `ein saturate --dump --events` and `ein test -v`,
and writes `out/table.md`:

| program | k_s | universe | `ein test` | firings | wiring | lookups | depth | facts stored | solve wall ms |
|---|---:|---|---|---:|---:|---:|---:|---:|---:|
| add | 1 | no candidate | 1 expectation: 1 held | 6 | 1 | 9 | 1 | 1402 | 21 |
| sub | 1 | no candidate | 1 expectation: 1 held | 6 | 1 | 9 | 1 | 1402 | 22 |
| sub-rev | 1 | no candidate | 1 expectation: 1 held | 6 | 1 | 9 | 1 | 1402 | 22 |
| mul | 1 | no candidate | 1 expectation: 1 held | 20 | 15 | 117 | 12 | 1420 | 21 |
| div | 1 | no candidate | 1 expectation: 1 held | 26 | 21 | 261 | 12 | 1432 | 21 |
| overflow | 1 | no candidate | 3 expectations: 3 held | 21 | 16 | 126 | 10 | 1424 | 21 |
| div-zero | 1 | no candidate | 2 expectations: 2 held | 17 | 12 | 90 | 2 | 1423 | 21 |
| chain | 1 | no candidate | 1 expectation: 1 held | 42 | 37 | 378 | 25 | 1452 | 23 |

Every `ein solve` and `ein test` exits 0, every read-out has *∅ in R* and *1 saturation · 0 facts beyond root*,
and no `solve` wrote to stderr. How to read the circuit columns (`depth.py`, over the `--events` log):

- **firings** is the saturator's own count. Five of them on every program are the four tables and the closing rule.
- **wiring** is the rest: the circuit's size for this request.
- **lookups** counts the premises of the wiring that are table rows: gate uses.
- **depth** is the longest chain of wiring firings. Ein's saturator is a FIFO agenda and has no rounds of its
  own, so this is the number of rounds a synchronous circuit would take. It is what the phase calls *rounds*.
- **solve wall** is the whole `ein solve` process, best of ten.

**The artefacts**, per program, under `build/s1f.6/prototype/out/<program>/`: `solve.txt`, `summary.json`
(`--json-summary`), `saturate.txt`, `events.jsonl`, `test.txt`, `circuit.txt`, `trace.md` and `trace-G.md`
(`--trace -G`, no diagrams). `trace.md` is 2 857 lines for `add` and 4 161 for `chain`
(`wc -l out/*/trace.md`, 2026-11-11), nearly all of it the five table steps drawn as graphs; `trace-G.md` is 73
lines for `add` and is the one to read.

## How the prototype is built

`gen.py W` writes the theory for one width. `programs/arith9.ein` is 488 lines.

**The gates** are three digit tables, plus two small ones:

| table | rows | reads as |
|---|---:|---|
| `(fa a b c s c')` | 200 | a + b + carry c is digit s, carry c′ |
| `(fs a b c d c')` | 200 | a − b − borrow c is digit d, borrow c′ |
| `(md a b c p c')` | 900 | a × b + carry c is digit p, carry c′ (the carry in is 0 to 8) |
| `(pred k j)` | 9 | the step before step k is step j: the step counter, named by the digits |
| `(nz d)` | 9 | d is not zero |

**A table is a rule with an empty `:match`**, a ROM: `(rule fa-table () :match () :assert (and (fa 0 0 0 0 0) …))`.
It fires once at root (M1f S1f.5.9). The tables are not written as facts because a flat import would drop them
([finding 1](#1-a-flat-import-keeps-a-modules-declarations-and-drops-its-facts)).

**The wiring**, fifteen rules:

- **Addition** is one rule with a premise per column, `add-fits`, the carry out of the top column written as the
  constant `0`. Its twin, `add-overflows`, writes `1` there and concludes `(overflow (add X Y))`.

  ```lisp
  (rule add-fits ()
    :match  (and (add (num9 ?x8 ?x7 ?x6 ?x5 ?x4 ?x3 ?x2 ?x1 ?x0) (num9 ?y8 ?y7 ?y6 ?y5 ?y4 ?y3 ?y2 ?y1 ?y0))
                (fa ?x0 ?y0 0 ?s0 ?c1)
                (fa ?x1 ?y1 ?c1 ?s1 ?c2)
                …
                (fa ?x8 ?y8 ?c8 ?s8 0))
    :assert (sum-of (num9 ?x8 … ?x0) (num9 ?y8 … ?y0) (num9 ?s8 ?s7 ?s6 ?s5 ?s4 ?s3 ?s2 ?s1 ?s0))
    :why    "ripple-carry: each column's digit and carry from the table, no carry out of the top")
  ```

- **Subtraction** is two ripple-borrow subtractors, X − Y and Y − X, and the one with no borrow out of the top
  answers: `sub-ge` gives `(diff-of X Y D 0)` and `sub-le` gives `(diff-of X Y 0 R)`. When X = Y both fire and
  conclude the same fact. No comparator is needed: the borrow is the comparison, as in hardware.
- **A number times a digit**, `muld-row`: `(muld X d)` is answered by `(muld-of X d P c)`, one `md` premise per
  column, `c` the digit carried out of the top.
- **Multiplication** is the school method from the multiplier's top digit down. `mul-start` asks for the row of
  each multiplier digit and states the first step. `mul-step` shifts the sum so far one place and adds the next
  row through the `fa` table. `mul-done` reads the product when the counter is 0. Three rules derive the
  overflow, one for each place a digit can fall off: the shift, the row's carry, the addition's carry.
- **Division** is long division. `div-start` asks for the divisor's ten multiples and states the first step.
  `div-step` brings down a digit, and its premises hold two subtractors: the remainder minus a multiple must not
  borrow, and what is left minus the divisor must borrow. Exactly one of the ten multiples passes both, and it
  is the quotient digit. The saturator tries the ten rows as a join over stored facts; nothing is proposed and
  nothing is refuted. `div-done` reads the quotient and the remainder. `div-by-zero` matches a zero divisor.
  With a zero divisor no multiple passes `div-step`, so the steps stop by themselves and no number is stored.

**The last rule closes everything.** `arith` has an empty `:match` and asserts `(__closed__ R)` for every relation
of the theory, and a `(circuit <rule>)` fact for every rule. The second half is why a program can import the whole
theory by one name ([finding 1](#1-a-flat-import-keeps-a-modules-declarations-and-drops-its-facts)).

**What the prototype does not build:** a comparator as an operation of its own (subtraction and division use the
borrow); hardware's modulo-and-borrow subtraction (question 3 (b)); double-width results (question 4 (b)); a
width above 9, since the step counter is named by the digits; and negative or rational numbers. None of these
was run, and no number on this page is about them.

### Right, held to Python's integers

`check.py` writes a program of requests, runs `ein saturate --dump`, reads the answers out of the dump and
compares each with Python, overflow and division by zero included. Python is the oracle for the theory and
enters no program. On 2026-11-11:

```text
$ python3 check.py 2 all
width 2: 10000 operand pairs x 4 operations = 40000 requests; 0 wrong, 0 answers to no request; 86455 firings, saturate 550.20 ms, wall 1.26 s
$ python3 check.py 9 500 1
width 9: 506 operand pairs x 4 operations = 2024 requests; 0 wrong, 0 answers to no request; 17315 firings, saturate 708.20 ms, wall 0.84 s
$ python3 check.py 4 500 2
width 4: 497 operand pairs x 4 operations = 1988 requests; 0 wrong, 0 answers to no request; 10125 firings, saturate 147.94 ms, wall 0.23 s
```

Width 2 is exhaustive: every pair of operands, all four operations. Widths 4 and 9 are random operands of every
length, plus the corners.

### Cost, and how it scales with the width

`python3 scale.py` (2026-11-11) runs one long-case request per operation at four widths. *load* is parse, load
and compile; *wall* is the whole `ein saturate --dump` process, best of ten.

| width | program | answer | firings | wiring | lookups | depth | facts stored | by the wiring | load ms | saturate ms | wall ms |
|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 3 | the import alone | — | 5 | 0 | 0 | 0 | 1394 | 0 | 14.5 | 0.98 | 19 |
| 3 | add: 99 + 1 | 100 | 6 | 1 | 3 | 1 | 1396 | 1 | 15.2 | 0.98 | 20 |
| 3 | sub: 100 − 1 | 99 and 0 | 6 | 1 | 3 | 1 | 1396 | 1 | 14.4 | 1.00 | 19 |
| 3 | mul: 99 × 9 | 891 | 12 | 7 | 15 | 6 | 1404 | 9 | 15.3 | 0.99 | 20 |
| 3 | div: 999 ÷ 7 | 142 and 5 | 20 | 15 | 51 | 6 | 1420 | 25 | 14.2 | 1.07 | 19 |
| 4 | the import alone | — | 5 | 0 | 0 | 0 | 1395 | 0 | 16.3 | 1.08 | 21 |
| 4 | add: 999 + 1 | 1000 | 6 | 1 | 4 | 1 | 1397 | 1 | 16.1 | 1.11 | 20 |
| 4 | sub: 1000 − 1 | 999 and 0 | 6 | 1 | 4 | 1 | 1397 | 1 | 15.7 | 1.17 | 20 |
| 4 | mul: 99 × 99 | 9801 | 13 | 8 | 24 | 7 | 1406 | 10 | 14.3 | 1.12 | 19 |
| 4 | div: 9999 ÷ 7 | 1428 and 3 | 21 | 16 | 76 | 7 | 1422 | 26 | 14.3 | 1.22 | 19 |
| 6 | the import alone | — | 5 | 0 | 0 | 0 | 1397 | 0 | 15.0 | 1.32 | 20 |
| 6 | add: 99999 + 1 | 100000 | 6 | 1 | 6 | 1 | 1399 | 1 | 15.7 | 1.35 | 21 |
| 6 | sub: 100000 − 1 | 99999 and 0 | 6 | 1 | 6 | 1 | 1399 | 1 | 16.7 | 1.38 | 22 |
| 6 | mul: 999 × 999 | 998001 | 15 | 10 | 48 | 9 | 1410 | 12 | 18.3 | 1.40 | 24 |
| 6 | div: 999999 ÷ 7 | 142857 and 0 | 23 | 18 | 138 | 9 | 1426 | 28 | 14.9 | 1.55 | 20 |
| 9 | the import alone | — | 5 | 0 | 0 | 0 | 1400 | 0 | 15.7 | 1.68 | 21 |
| 9 | add: 99999999 + 1 | 100000000 | 6 | 1 | 9 | 1 | 1402 | 1 | 16.6 | 1.74 | 22 |
| 9 | sub: 100000000 − 1 | 99999999 and 0 | 6 | 1 | 9 | 1 | 1402 | 1 | 16.0 | 1.76 | 22 |
| 9 | mul: 99999 × 9999 | 999890001 | 18 | 13 | 99 | 12 | 1416 | 15 | 15.2 | 1.81 | 20 |
| 9 | div: 999999999 ÷ 7 | 142857142 and 5 | 26 | 21 | 261 | 12 | 1432 | 31 | 16.4 | 2.06 | 23 |

What the table says:

- **A run's cost is the tables, not the arithmetic.** The import alone is 19 to 21 ms of wall and about 1 400
  stored facts at every width; one request adds 1 to 31 facts and at most 0.4 ms of saturation. A stage that
  wants a cheaper run looks at loading 1 300 table rows, not at the circuit.
- **Addition and subtraction** are one firing and W lookups at any width, depth 1.
- **Multiplication** is W + 2 firings plus one per distinct multiplier digit, depth W + 3. Its lookups grow as
  W², the array multiplier's size.
- **Division** is W + 12 firings (ten multiples, W steps, a start and an end), depth W + 3. Its lookups are
  2 W² + 11 W: 51, 76, 138 and 261 at widths 3, 4, 6 and 9.
- **At volume** the marginal request at width 9 costs about 0.35 ms of saturation: 2 024 mixed requests in
  708 ms (`python3 check.py 9 500 1`).

## Findings

None of the three is a wrong answer about arithmetic, and none stopped the prototype. Each shapes a question
below. The witnesses are under `build/s1f.6/prototype/findings/` and `q6/`. The engine was not changed.

### 1. A flat import keeps a module's declarations and drops its facts

`findings/tables-as-facts.ein` is a module with two table rows written as facts, `(fa 9 1 0 0 1) (fa 0 0 1 1 0)`,
and one rule, `add2`, that reads them. Three programs import it and ask 9 + 1:

| program | import | `ein solve`'s query row | `fa` facts in `saturate --dump` |
|---|---|---|---:|
| `flat-import-drops-facts.ein` | `:symbols (add sum-of num2 fa add2)` | `1 state (1) — the goal has no row: undetermined here` | 0 |
| `flat-import-unlisted-rule.ein` | `:symbols (add sum-of num2 fa)` | `1 state (1) — the goal has no row: undetermined here` | 0 |
| `qualified-import-keeps-facts.ein` | `:as n` | `?s = (n.num2 1 0) · ?x = (n.num2 0 9) · ?y = (n.num2 0 1)` | 2 |

Every run exits 0 and prints no warning. This is `ein_ir::imports`' `select` as written: `:symbols` keeps the
listed declarations and what they name, and a fact is no declaration. It is how `std.algebra` is meant to be
imported, since its rules are activated by the importing program's own facts. A table theory is the first module
whose content is facts, so it meets the rule from the other side. There are three ways through, and question 8
asks which:

- **Tables as ROM rules, and one rule that names every rule** (the prototype). `(import arith9 :symbols (arith))`
  brings the whole theory under flat names, because `arith`'s `:assert` names every relation and every rule.
  It works today, with no engine change. The cost is that a table row is *derived by `fa-table`* and not
  *given*, and the `(circuit …)` facts exist only to be named.
- **A qualified import**, `(import arith9 :as n)`, keeps facts and prefixes every declared name:
  `(n.add (n.num9 0 0 0 1 2 3 0 7 8) (n.num9 …))`. Run on the prototype theory, `findings/qualified9.ein` and
  `whole9.ein` (a bare `(import arith9)`, the prefix `arith9.`) both answer the sum with H empty.
- **An import that keeps a module's facts under flat names.** That is an engine change and a ruling, outside this
  phase's *adds nothing to the engine*.

### 2. The theory closes its own relations, and cannot close the importing program's

`q6/a-own-relation.ein` imports the closed theory, declares `(relation total Number)` and derives
`(total ?s)` from the sum. Its `ein solve`:

```text
  states (k_s)      2 048
  universe          11 candidates, 0 dead alone
  :expect           FAILED — the claim lists 1 class, the query has 2048: (model (total (num9 0 0 0 1 2 7 6 4 5)) (total 0)) is not listed
    (model (total (num9 0 0 0 1 2 7 6 4 5)) (total 0) (total 1) (total 2) (total 3) (total 4) (total 5) (total 6) (total 7) (total 8) (total 9) (total __closed__)) is a class the claim does not list
```

The blind enumerator proposes `(total d)` for each of the ten digits, which are names of the program, and
`(total __closed__)`. With `(__closed__ total)` added by the program (`q6/a-own-relation-closed.ein`) the
universe has no candidate and the claim holds. So mechanism (a) of question 6 makes the *theory* search-free. A
program that declares a relation of arity one or two beside it must close that relation itself, or it is
searching. `chain.ein` declares none, which is why it needed nothing. That `__closed__` is proposed as an
object is roughness for [F25](../../followups/f25_roughness_found_outside_zebra.md), not for the user: it moves
no zebra program.

### 3. A number with the wrong count of digits loads, and is answered by silence

`findings/short-number.ein` asks `(add (num9 0 0 1 2 3 0 7 8) (num9 0 0 0 0 0 4 5 6 7))`, the first number
eight digits long. `num9` is declared with nine argument types. The program loads, exits 0, and both
`(sum-of ?x ?y ?s)` and `(overflow ?r)` print *the goal has no row: undetermined here*. A declared relation's
arity is not checked against a nested fact of it, and a request no rule matches derives nothing. An `:expect`
catches it; a program without one sees no answer and no error. The theory could answer every request it cannot
read with a `(malformed …)` fact only by negation, which the principle rules out, so this stays a property of
the protocol unless the loader checks arity. It is a row for F25 too.

## The questions, each with the recommendation

### 1. The number's shape

**Recommend (a), the user's `(num9 d8 … d0)`,** one relation per width. It is what the prototype runs, a number
reads as written, and one rule destructures all nine digits at once, which is what makes addition a single
firing. (b), the nested little-endian list, also runs: it is adder variant **d** of question 9, of any width,
at 2 W + 3 firings per sum against 1. (c), a variadic relation, is a grammar change and S1f.6.9's. The fixed
width costs one generated theory per width, and a width above 9 needs step names beyond the digits.

### 2. The protocol

**Recommend what the prototype does:** a request fact, `(add X Y)`, answered by a result fact that repeats the
operands, `(sum-of X Y S)`, and a flag that holds the request as a value, `(overflow (add X Y))`.

- The result repeats its operands, so several requests in one program are told apart and a chain rule matches
  the answer to its own question.
- A chain is a rule from an answer to the next request (`chain.ein`).
- Inside the theory, a sub-circuit is a request only where its answer is reused: `(muld X d)`, the row of one
  digit, asked once and read by every step. Everything else reads the tables directly in the rule's premises.
  So the theory never states an `(add …)` or `(sub …)` of its own, and a program's `(sum-of ?x ?y ?s)` query
  sees only the sums the program asked for.
- The step relations, `mul-at`, `div-at`, `muld` and `muld-of`, are visible in the state. They are the
  explanation `--trace` prints.

### 3. Subtraction's two numbers

**Recommend (a), the pair (d r),** with at most one of the two non-zero. It is what the prototype computes, by two
subtractors and no comparator, in one firing. It is the normal form S1f.6.7's negative numbers want. (b) was not
built.

### 4. The width of a result

**Recommend (a), the operands' width with a flag.** Multiplication's three overflow rules and addition's one are
built and held to Python at three widths. (b), double width, was not built; S1f.6.9 takes up extension.

### 5. Overflow and division by zero

**Recommend a fact naming its request,** `(overflow (add X Y))` and `(division-by-zero (div X Y))`, with no number
stored. Root stays alive, so the other requests of the program are answered, and a claim can state both the flag
and the absence of a number (`overflow.ein`'s three `:expect`s). `(false)` was not tried: it would make R empty
for the whole program.

### 6. How H stays empty

Each mechanism was tried on 123078 + 4567 against a copy of the theory with no closure
(`python3 gen.py 9 --open`), under `q6/` (2026-11-11):

| program | mechanism | universe | k_s | the `:expect` |
|---|---|---|---:|---|
| `none.ein` | nothing | 562 candidates over 10 relations (`ein solve --minimal -H`) | — | bare `ein solve` stops after about 20 s, exit 1: *a state's commitments pass 2^128: the state route cannot count them*; `ein test` counts 1 error |
| `programs/add.ein` | (a) the theory derives `(__closed__ R)` for each of its relations, from a rule with an empty `:match` | no candidate | 1 | holds |
| `a2-program-closes.ein` | the program writes the ten `(__closed__ R)` facts | no candidate | 1 | holds |
| `b-no-hypothesis.ein` | (b) `(solve :no-hypothesis (add sub mul div muld pred nz overflow division-by-zero circuit))` | no candidate | 1 | holds |
| `b-partial.ein` | (b) with `nz` forgotten | 1 candidate | 2 | holds, on two states |
| any of them | (c) `ein saturate` | not consulted | — | the same stored facts |

Only relations of arity one and two are candidates; the read-out's `outside` row names the rest. Ten of the
theory's relations are in reach.

**Recommend (a).** A program that imports the theory writes nothing and is search-free under every command, and
a `(__closed__ R)` that a rule derives at root works exactly as a written one. (b) works, and it makes every
program repeat a list of the theory's internal names. A forgotten name is not refused, it is searched:
`b-partial.ein` has two states and its claim still holds, so nothing tells the author. (c) answers, and gives up
`ein solve` as the one tool. (a) covers the theory's relations only ([finding 2](#2-the-theory-closes-its-own-relations-and-cannot-close-the-importing-programs)):
the theory's page must say that a program closes the unary and binary relations it declares itself.

### 7. The tables

**Recommend (a), generated,** with a `--check` in the gate. The prototype's `gen.py` already writes all 1 318 rows
and the wide rules. At width 9 `div-step` has 22 premises; no one should type it. A test holds every row to the harness's integers either way. The stage should also decide
whether the *rules* are generated per width with the tables, as the prototype's are. They have to be under
shape (a) of question 1.

### 8. Where the theory lives

**Recommend (a), `stdlib/arith.ein` as `std.arith`,** with the import form as the open part, which
[finding 1](#1-a-flat-import-keeps-a-modules-declarations-and-drops-its-facts) decides:

- **(a1)** `(import std.arith :symbols (arith))` and flat names, `(add (num9 …) (num9 …))`. Tables are ROM rules
  and the `arith` rule names every rule. This is the prototype, it needs no engine change, and it reads as the
  user's sketch does. Recommended.
- **(a2)** `(import std.arith :as n)` and `(n.add (n.num9 …) (n.num9 …))`. Tables may be facts. Every name in
  every program and every answer carries the prefix.
- **(a3)** A ruling that a flat import keeps a module's facts. An engine change.

The user's sketch writes a bare `(import std.arith)`. On today's engine that is the qualified form, with the
prefix `std.arith.` on every name. One more point for the stdlib's own gates: `stdlib_coverage.rs` asks that every
rule be fired by a program under `tests/stdlib/`, and with one theory per width that is 20 rules per width.

### 9. How a circuit is written (measured)

One ripple-carry adder, four ways, in one module (`adders.py`; `q9/w4/` and `q9/w9/`):

- **a** — one rule, a premise per column. The prototype's `add-fits`.
- **b** — one rule per column, a `b-carry` fact passed from column to column, and one rule that collects the
  digits.
- **c** — one rule for every column, over step names: both operands in shift registers, one table premise.
- **d** — one rule for every column, recursive over a nested little-endian number `(d d0 (d d1 … e))`.

All four answer, with H empty, k_s 1 and their `:expect` held (`q9/w9/add-*.solve.txt`). One sum, the longest
carry chain, 99999999 + 1 at width 9 and 999 + 1 at width 4 (`python3 adders.py W q9/wW`, then `depth.py`):

| variant | rules | width 4: wiring firings · depth | width 9: wiring firings · depth |
|---|---:|---:|---:|
| a | 1 | 1 · 1 | 1 · 1 |
| b | W + 2 | 6 · 6 | 11 · 11 |
| c | 3 | 6 · 6 | 11 · 11 |
| d | 5 | 11 · 11 | 21 · 21 |

A thousand random sums per variant (`python3 q9_batch.py 4` and `9`, saturation's own time, best of five):

| variant | width 4: firings · facts by the wiring · ms | width 9: firings · facts by the wiring · ms |
|---|---:|---:|
| a | 1 002 · 1 000 · 7.9 | 1 002 · 1 000 · 35.5 |
| b | 6 002 · 10 000 · 72.7 | 11 002 · 20 000 · 322.4 |
| c | 6 002 · 6 000 · 9.9 | 11 002 · 11 000 · 26.2 |
| d | 7 935 · 6 934 · 139.0 | 17 919 · 16 918 · 929.1 |

**Recommend a for the four operations.** It is the smallest and shallowest circuit: one firing and one stored
fact per sum at any width, where the others take W + 2 or 2 W + 3 firings and store every intermediate. In
time, **c** is level with it (faster at width 9, slower at width 4): a nine-premise join costs about what nine
one-premise firings do. **b** and **d** are 9 and 26 times slower than **a** at width 9. Variant **d** shares
work between requests with a common tail, which is why its firings are under 23 a sum; it is the only
width-free form, and S1f.6.9 should start from it. The shift-register form, **c**, is what the prototype uses
where a circuit must iterate, in `mul-step` and `div-step`: one rule, over step names.

## What the stop asks of the user

1. The nine recommendations above: 1 (a), 2 the request and result facts as built, 3 (a), 4 (a), 5 a fact,
   6 (a), 7 (a), 8 (a) with (a1), 9 variant a with c for the iterated circuits.
2. Question 8's import form is the one that needs a ruling and not just a nod, since (a1) rests on a ROM rule
   and a rule that exists to name the others, and (a3) is an engine change.
3. Nothing on the two rows for F25, [finding 2](#2-the-theory-closes-its-own-relations-and-cannot-close-the-importing-programs)'s
   `(total __closed__)` and [finding 3](#3-a-number-with-the-wrong-count-of-digits-loads-and-is-answered-by-silence)'s
   unchecked arity: they are parked under the ruling of 2026-11-11, since neither moves a zebra program. They
   are named here because every program that uses the theory will meet them. This stage did not edit F25; the
   coordinator files the rows.
