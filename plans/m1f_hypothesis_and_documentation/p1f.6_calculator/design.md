# P1f.6 — the design proposal: a calculator in Ein, shown on a running prototype

**Stage:** [S1f.6.1](s1f.6.1_design_proposal.md) of [P1f.6](README.md). Written 2026-11-11.
**This page stops for the user.** Nothing under `stdlib/`, `examples/`, `tests/` or `corpus/` was written, and the
engine is unchanged.

**The prototype** is under `build/s1f.6/prototype/`, which git ignores, so its files are named here as paths and
not as links. Its `README.md` lists every script and the command that re-takes each number on this page. Every
answer below is pasted from a run of `ein.rs/target/release/ein`, built from the tree at `25b4af05`, on
2026-11-11. None is written by hand.

**Round 2** (the same day): the user answered the questions and asked for more experiments. They are in
[§ Round 2](#round-2-2026-11-11-the-users-answers-and-the-experiments-they-asked-for) at the end, and each
question and finding below carries its state after it. The rest of the page is round 1, kept as written.


> **Moved 2026-11-11.** Where this page says a finding is a row for F25, read
> [`findings.md`](findings.md): the user ruled the same day that F25 was for P1f.7 alone, and the rows this
> stage's two rounds filed there, R-9 to R-15, are this phase's E-1 to E-7.

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

**State after round 2: to be fixed,** on the user's *"importing facts is mandatory"*; the specification is
[§ G](#g-the-flat-import-finding-a-specification-for-importing-facts), and the fix is not made here.

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

**State after round 2: ruled, left as it is** (the user: *"it's ok for now"*).

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

**State after round 2: follows question 1** ([§ A](#a-little-endian-nested-numbers-questions-1-and-4-finding-3)).

`findings/short-number.ein` asks `(add (num9 0 0 1 2 3 0 7 8) (num9 0 0 0 0 0 4 5 6 7))`, the first number
eight digits long. `num9` is declared with nine argument types. The program loads, exits 0, and both
`(sum-of ?x ?y ?s)` and `(overflow ?r)` print *the goal has no row: undetermined here*. A declared relation's
arity is not checked against a nested fact of it, and a request no rule matches derives nothing. An `:expect`
catches it; a program without one sees no answer and no error. The theory could answer every request it cannot
read with a `(malformed …)` fact only by negation, which the principle rules out, so this stays a property of
the protocol unless the loader checks arity. It is a row for F25 too.

## The questions, each with the recommendation

### 1. The number's shape

**State after round 2: open.** The user asked for the little-endian nested list; it is built, with all four
operations ([§ A](#a-little-endian-nested-numbers-questions-1-and-4-finding-3)).

**Recommend (a), the user's `(num9 d8 … d0)`,** one relation per width. It is what the prototype runs, a number
reads as written, and one rule destructures all nine digits at once, which is what makes addition a single
firing. (b), the nested little-endian list, also runs: it is adder variant **d** of question 9, of any width,
at 2 W + 3 firings per sum against 1. (c), a variadic relation, is a grammar change and S1f.6.9's. The fixed
width costs one generated theory per width, and a width above 9 needs step names beyond the digits.

### 2. The protocol

**State after round 2: open.** Comparison, nested expressions, names and the goal form were tried
([§ B](#b-comparison-question-2), [§ C](#c-the-protocol-and-nested-expressions-question-2)).

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

**State after round 2: ruled (a),** the user leaning to it and no objection found
([§ D](#d-subtractions-pair-question-3)).

**Recommend (a), the pair (d r),** with at most one of the two non-zero. It is what the prototype computes, by two
subtractors and no comparator, in one firing. It is the normal form S1f.6.7's negative numbers want. (b) was not
built.

### 4. The width of a result

**State after round 2: follows question 1.** Nested numbers have no width
([§ A](#a-little-endian-nested-numbers-questions-1-and-4-finding-3)).

**Recommend (a), the operands' width with a flag.** Multiplication's three overflow rules and addition's one are
built and held to Python at three widths. (b), double width, was not built; S1f.6.9 takes up extension.

### 5. Overflow and division by zero

**State after round 2: ruled for division by zero,** `(false)` and the flag fact both, built in
[§ E](#e-division-by-zero-as-false-question-5); overflow under a fixed width is open.

**Recommend a fact naming its request,** `(overflow (add X Y))` and `(division-by-zero (div X Y))`, with no number
stored. Root stays alive, so the other requests of the program are answered, and a claim can state both the flag
and the absence of a number (`overflow.ein`'s three `:expect`s). `(false)` was not tried: it would make R empty
for the whole program.

### 6. How H stays empty

**State after round 2: ruled (a).**

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

**State after round 2: ruled (a).**

**Recommend (a), generated,** with a `--check` in the gate. The prototype's `gen.py` already writes all 1 318 rows
and the wide rules. At width 9 `div-step` has 22 premises; no one should type it. A test holds every row to the harness's integers either way. The stage should also decide
whether the *rules* are generated per width with the tables, as the prototype's are. They have to be under
shape (a) of question 1.

### 8. Where the theory lives

**State after round 2: ruled (a).** The import form is (a1) until an import brings facts;
[§ G](#g-the-flat-import-finding-a-specification-for-importing-facts) specifies that fix.

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

**State after round 2: open by the user's choice,** every variant kept runnable
([§ F](#f-all-adder-variants-question-9)).

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

## Round 2 (2026-11-11): the user's answers and the experiments they asked for

The user read round 1 (committed as `0355ab2a`) and answered each question; several answers ask for more
experiments before a ruling. This section quotes each answer and gives the experiment. Round 1 above is kept as
the record; each question there now carries its state. Everything here was run on the same binary on
2026-11-11, and the files are under `build/s1f.6/prototype/`: `le/` (the width-free theory, its targets,
experiments and protocol programs), `n9/` (round 2 on `num9`), `g/` (the import comparison). Nothing was
committed, and the engine, grammar, `stdlib/`, `examples/`, `tests/` and `corpus/` are untouched.

**Round 2 in one paragraph.** A second theory, over little-endian nested numbers of any length, computes the
four operations, comparison and nested expressions by saturation alone: every program below has *universe: no
candidate*, k_s 1 and R {∅}, and it agrees with Python on every request checked, with every answer in normal
form. It removes the width, the overflow and the wrong digit count. It costs 15 to 20 times the firings of
`num9`, which no single program notices, and it is quadratic at volume, because the engine's fact index does not
key on a nested fact ([§ what it costs](#what-it-costs)). Nested expressions work as one fact,
`(calc (add N1 (div N2 N3)))` or `(let x …)`, on top of round 1's request facts. The user's goal form,
`:goal (= ?x (add …))`, is refused, and no goal drives a computation. Division by zero as `(false)` is built
and its surfaces are shown. The flat-import finding became a fix specification on the user's second answer:
an ontology library, facts only, must be importable, and § G says what that takes.

### A. Little-endian nested numbers (questions 1 and 4, finding 3)

> 1: "try little-endian nested list, looks promising"
>
> 4: "another point for little-endian nested list, but is it possible to simplify leading zeros?"
>
> finding 3: "another point for little-endian nested lists?"

**The spelling proposed.** A number is `z`, zero, or `(d DIGIT NUMBER)`, a digit under a number, units first.
123078 is

```lisp
(d 8 (d 7 (d 0 (d 3 (d 2 (d 1 z))))))
```

The **normal form** has no high zero: zero is `z` and never `(d 0 z)`, and the innermost digit of any other
number is not 0. `d` and `z` are this page's choice and the user's to rename; any two names work. Two
alternatives were not built: the terminator `0`, so that zero is the digit's own name, and no terminator at all,
the top digit standing innermost, `(d 8 (d 7 (d 0 (d 3 (d 2 1)))))`, which makes every digit a number and every
rule two cases longer.

**The theory** is `le/nat.ein`, written by `gen_le.py`: 80 rules and 50 relations
(`grep -c '^(rule' le/nat.ein`), no `absent`, no `not`, one stratum. It holds the same three tables as `num9`,
the four operations, comparison and the expression layer of [§ C](#c-the-protocol-and-nested-expressions-question-2).
Where `num9` has one wide rule per operation, this has a recursion over the digits:

- **A ripple** is a request with a carry, `(addc X Y c)`, `(subc X Y c)` or `(mulc X d c)`. One rule takes the
  units column through the table and asks for the rest with that column's carry. It leaves a *frame* fact,
  `(fr <the rest's request> <this column's digit> <this request>)`. When the rest is answered, one of three rules
  puts the digit under it.
- **Those three rules are the normal form.** A digit goes under an answer that is not zero; a digit that is not 0
  goes under zero; and 0 under zero is zero. No answer is ever built with a high zero, whatever the operands.
- **Subtraction and comparison** subtract both ways. A borrow out of the top is a fact, `(borrowed <request>)`,
  so the ripple that does not borrow answers, and which one borrows is the comparison.
- **Multiplication** is X × (d b Y) = X × b + 10 × (X × Y), where times ten is a 0 put under the number.
- **Division** is long division. The recursion reaches the dividend's top digit first, which is where long
  division starts. At each digit it subtracts the divisor's nine multiples, and the digit is the multiple that
  fits where the next one borrows.

**The eight targets, on both shapes.** `./run2.sh le/programs le/out` (2026-11-11) beside round 1's table. Every
program has k_s 1, *universe: no candidate*, every `:expect` held, exit 0.

| program | answer on `le/nat.ein` | wiring firings: num9 → LE | depth: num9 → LE | facts stored: num9 → LE | solve wall ms: num9 → LE |
|---|---|---:|---:|---:|---:|
| add | `(d 5 (d 4 (d 6 (d 7 (d 2 (d 1 z))))))` | 1 → 15 | 1 → 15 | 1402 → 1569 | 21 → 22 |
| sub | `(d 1 (d 1 (d 5 (d 8 (d 1 (d 1 z))))))` and `z` | 1 → 28 | 1 → 15 | 1402 → 1589 | 22 → 25 |
| sub-rev | `z` and `(d 1 (d 1 (d 5 (d 8 (d 1 (d 1 z))))))` | 1 → 28 | 1 → 15 | 1402 → 1589 | 22 → 23 |
| mul | `(d 8 (d 8 (d 0 (d 6 (d 5 z)))))` | 15 → 51 | 12 → 36 | 1420 → 1623 | 21 → 24 |
| div | `(d 5 (d 0 (d 1 (d 5 z))))` and `(d 0 (d 2 z))` | 21 → 416 | 12 → 82 | 1432 → 2125 | 21 → 26 |
| overflow | no overflow: both have an answer | 16 → 194 | 10 → 107 | 1424 → 1838 | 21 → 25 |
| div-zero | `(division-by-zero (div … z))` | 12 → 3 | 2 → 3 | 1423 → 1551 | 21 → 23 |
| chain | `(d 3 (d 4 (d 2 (d 2 z))))` and `(d 3 (d 1 z))` | 37 → 427 | 25 → 110 | 1452 → 2150 | 23 → 26 |

The artefacts are under `le/out/<program>/`, the same files as round 1's.

**Held to Python** (`check_le.py`, five requests a pair: add, sub, mul, div, cmp; 2026-11-11):

```text
$ python3 check_le.py 3 200 1
up to 3 digits: 165 operand pairs x 5 operations = 825 requests; 0 wrong, 0 answers not in normal form; 18065 firings, 26734 facts, saturate 388.08 ms, wall 0.48 s
$ python3 check_le.py 9 200 2
up to 9 digits: 200 operand pairs x 5 operations = 1000 requests; 0 wrong, 0 answers not in normal form; 100830 firings, 145744 facts, saturate 12309.25 ms, wall 12.79 s
$ python3 check_le.py 9 100 3 --pad
up to 9 digits, some operands with high zeros: 101 operand pairs x 5 operations = 505 requests; 0 wrong, 0 answers not in normal form; 57199 firings, 82924 facts, saturate 4016.72 ms, wall 4.29 s
$ python3 check_le.py 30 30 4
up to 30 digits: 35 operand pairs x 5 operations = 175 requests; 0 wrong, 0 answers not in normal form; 99402 firings, 146413 facts, saturate 12177.52 ms, wall 13.01 s
```

**Leading zeros** (`le/x/zeros.ein`). The answer to *is it possible to simplify leading zeros* is yes, and the
theory does it in every answer. What it cannot do is make two spellings one value.

- **Every answer is in normal form**, whatever the operands. 5 written `(d 5 (d 0 (d 0 z)))` plus 7 written
  `(d 7 (d 0 z))` is `(d 2 (d 1 z))`. 100 − 99 is `(d 1 z)`. `(d 5 (d 0 z))` times `(d 0 (d 0 z))` is `z`.
- **Operands with high zeros are accepted and read by value.** `(cmp (d 5 (d 0 (d 0 z))) (d 5 z))` answers
  `eq`, and `(div (d 7 z) (d 0 z))` answers `(division-by-zero (div (d 7 z) (d 0 z)))`.
- **Two spellings of one number are two values to Ein.** The theory's comparison says `eq`; Ein's own `eq` and
  `neq`, which compare structure, say different. The query `(and (cmp-of ?x ?y eq) (neq ?x ?y))` has the row
  `?x = (d 5 (d 0 (d 0 z))) · ?y = (d 5 z)`, and `(and (cmp-of ?x ?y eq) (eq ?x ?y))` has no row. An answer fact
  repeats its operands as they were written, so `(sum-of (d 5 (d 0 (d 0 z))) …)` and `(sum-of (d 5 z) …)` are
  two facts with one sum.
- **So the rule for an author is: write normal forms.** A program that does so never sees the difference, since
  the theory never produces anything else.

**Does overflow disappear? Yes.** 999999999 + 1 and 123078 × 123078 both have an answer (`le/programs/overflow.ein`,
two `:expect`s held). The theory has no overflow rule and no width.

**What bounds a number** (`python3 le/x/limits.py`, 2026-11-11, which writes `le/x/deep.ein`):

- **A written number: 999 digits inside a request.** A request holding a number of 1 000 digits is refused at
  the parse: `deep.ein:2:5001: this ( nests deeper than the limit of 1000 parentheses`.
- **A derived number is not bounded by the parser.** 999 nines plus 1 is answered, 1 000 digits, with H empty,
  in 2 006 firings (`ein saturate le/x/deep.ein`) and 0.5 s of wall.
- **A state file holding such a number does not load back.** `ein solve le/x/deep.ein --dump-answer DIR` writes
  `models/01.ein`, 32 MB, and `ein saturate` on it is refused at the same parse limit. It is 32 MB because every
  step fact holds its operands whole, and they are a thousand digits long.
- **Size:** 80 digits times 80 digits takes 13 417 firings and 0.33 s of saturation; 80 digits divided by 40
  takes 34 486 firings and 1.2 s (the same script, one request each).

**Does the silence on a malformed number disappear? The wrong digit count does; silence does not.** Any count of
digits is a number, so finding 3's witness has no analogue. The analogues are a name that is no digit, a number
that does not end in `z`, and a `d` of one argument. `le/x/malformed.ein` asks all three and one good sum:
it loads, exits 0, and the query lists the good sum alone. The reason is the same as in round 1: a rule cannot
conclude *nothing matched* without negation.

#### What it costs

- **Firings.** 15 to 28 where `num9` has 1 for a sum or a difference, 51 against 15 for the product, 416 against
  21 for the division. Each digit is a firing down and a firing up, where `num9` has one rule for all nine.
- **One program does not notice.** Every target is within 5 ms of its `num9` run, because loading the tables is
  most of both.
- **At volume it is quadratic.** Sums of two random nine-digit numbers, saturation's own time
  (`python3 q9_scale.py`, 2026-11-11):

  | sums | `num9` (variant a) | round 1's variant d | `le/nat.ein` (variant e) |
  |---:|---:|---:|---:|
  | 250 | 9.3 ms | 69.8 ms | 43.2 ms |
  | 1 000 | 36.4 ms | 959.7 ms | 521.1 ms |
  | 4 000 | 142.1 ms | 13 750 ms | 7 338 ms |

  Four times the requests is four times the time on `num9` and fourteen times on either nested form.
- **Why.** The engine's fact index does not key on a value that is a nested fact:
  `ein-infer/src/match_.rs`, `probe_key`, *"A nested fact: the index does not hold one, so the scan moves on"*.
  A premise whose only bound slots hold numbers is matched by scanning every fact of its relation. On `num9` the
  wide rules join on digits, which are names, so they are indexed. On nested numbers every join is on a number
  or on a request, so every join is a scan, and the relations grow with the requests. `perf record` on a batch
  of 40 divisions put 91 % of the run in the matcher's `unify`, `try_candidate` and `walk`.
- **Does a better circuit close the gap? Half of it.** Round 1's variant d joined on a number nested inside the
  pattern. The frames of `nat.ein` join on a whole request held in a top-level slot, which is the best a theory
  can do, and that is the factor of two in the table. One rule that joined a relation with itself made a batch
  of 40 divisions take 431 ms; split into three two-premise rules it takes 51 ms. Past that, any key a theory
  can build is a digit or a nested fact, and Ein has no fresh names.
- **What would close it** is an index entry for a nested fact, which is interned and has an id. That is an
  engine change, not made and not this phase's. Until then the width-free theory is the right tool for a
  program of some tens of requests and the wrong one for a table of thousands.

### B. Comparison (question 2)

> "what we forgot from school arithmetic is at least numbers comparison"

Both shapes now answer `(cmp X Y)` with `(cmp-of X Y lt)`, `eq` or `gt`, derived outright from the subtractor's
borrow, with no negation and no new table.

- **`num9`** (`n9/arith9r2.ein`, `python3 gen.py 9 --r2`): three rules. X − Y borrows out of the top: `lt`.
  Y − X borrows: `gt`. `(cmp ?x ?x)`: `eq`, since a fixed-width number has one spelling. 303 random pairs agree
  with Python, 0 wrong (`python3 n9/cmp_check.py`, 2026-11-11).
- **Nested numbers:** four rules over the two ripples subtraction already runs. `eq` is *the difference is zero*,
  so it is by value and holds across spellings.

`n9/cmp.ein`:

```lisp
(cmp (num9 0 0 0 1 2 3 0 7 8) (num9 0 0 0 0 0 4 5 6 7))
(cmp (num9 0 0 0 0 0 4 5 6 7) (num9 0 0 0 1 2 3 0 7 8))
(cmp (num9 0 0 0 0 0 4 5 6 7) (num9 0 0 0 0 0 4 5 6 7))

(query :goal (cmp-of ?x ?y ?o)
       :expect (model (cmp-of … gt) (cmp-of … lt) (cmp-of … eq)))     ; written out in the file; holds
```

`le`, `ge`, `max` and `min` were not built. Each is a rule or two over `cmp-of`, and `le/p/branch.ein` below
shows a program branching on the three outcomes without them.

### C. The protocol and nested expressions (question 2)

> "the protocol - idk honestly, what we forgot from school arithmetic is at least numbers comparison, so maybe
> :goal (= ?x (add <n1> (div <n2> <n3>)), seems without declaring constants like (const x1 (num9 0 0 ...)) it
> would probably work, but not convenient, then what about nested calculations (add (sub (mul (div ...) - yet too
> many questions to decide, do experiments, show me more examples"

Four protocols were tried on real programs. The programs are under `le/p/` and `le/goal/`, written by
`gen_r2_programs.py`, each with its `ein solve` output beside it as `<name>.solve.txt`. Every program under
`le/p/` is search-free: `ein test le/p` reports *15 files, 16 expectations: 16 held, 0 FAILED, 0 not checked,
0 errors … 2 files state no expectations* (the theory and `calc-all-values.ein`), and each read-out has k_s 1
and *universe: no candidate*. The numbers below are nested; the same
expression layer runs on `num9` (`n9/calc.ein`, `n9/word-problem.ein`, both held).

#### (iii) The user's goal form: what is refused, and why no goal can compute

The form as written, `le/goal/goal-equals.ein`:

```lisp
(query :goal (= ?x (add (d 8 (d 7 (d 0 (d 3 (d 2 (d 1 z)))))) (div (d 5 (d 4 (d 6 (d 7 (d 2 (d 1 z)))))) (d 5 (d 2 z))))))
```

```text
kb load error: goal-equals.ein:4:15: query: its :goal writes (= ?x (add (d 8 (d 7 (d 0 (d 3 (d 2 (d 1 z)))))) (div (d 5 (d 4 (d 6 (d 7 (d 2 (d 1 z)))))) (d 5 (d 2 z))))), and the program names no relation = — a relation of the program is one it declares, states a fact of, or writes in a rule or an hrule, so no state holds a fact of =
exit 1
```

`=` is a name like any other, and a goal over a relation nothing makes is refused (M1f S1f.7.11). Three
neighbours, each run:

| goal | what happens |
|---|---|
| `(eq ?x (add N1 (div N2 N3)))` | Loads, exit 0, and answers `?x = (add (d 8 …) (div (d 5 …) (d 5 (d 2 z))))`: the expression itself. Ein's `eq` is equality of structure; it binds `?x` to the form and computes nothing |
| `(value-of (add N1 (div N2 N3)) ?x)`, with no `calc` fact | Loads, exit 0, *the goal has no row: undetermined here* |
| `(sum-of N1 N2 ?x)`, with no `add` fact | The same: no row |

**Nothing is demand-driven from a goal.** A goal is matched against the states of R after the run; it is not a
premise, and no rule can read it. So a calculation has to be asked for by a fact. The closest working spelling
of the user's line is one fact and one goal:

```lisp
(let x (add N1 (div N2 N3)))
(query :goal (is x ?v))
```

Making a goal ask is an engine change: the loader would have to turn a goal's sub-terms into facts, or the
matcher would have to evaluate. It is outside this phase.

#### (i) Round 1's request and answer facts

Unchanged, and the layer everything else stands on: `(add X Y)` is answered by `(sum-of X Y S)`, and a chain is
a rule from an answer to the next request. It is the right form for a recursion the program writes itself.
`le/p/gcd.ein`, Euclid's algorithm as four rules over `div`:

```lisp
(relation gcd Number Number :why "asked: the greatest common divisor of {?1} and {?2}")
(relation gcd-of Number Number Number :why "the greatest common divisor of {?1} and {?2} is {?3}")
(__closed__ gcd)

(gcd (d 1 (d 7 (d 0 (d 1 z)))) (d 2 (d 6 (d 4 z))))

(rule gcd-base ()   :match (gcd ?a z)            :assert (gcd-of ?a z ?a)       :why "…")
(rule gcd-divide () :match (gcd ?a (d ?p ?q))    :assert (div ?a (d ?p ?q))     :why "…")
(rule gcd-next ()   :match (and (gcd ?a ?b) (quot-of ?a ?b ?q ?r))                       :assert (gcd ?b ?r)        :why "…")
(rule gcd-answer () :match (and (gcd ?a ?b) (quot-of ?a ?b ?q ?r) (gcd-of ?b ?r ?g))     :assert (gcd-of ?a ?b ?g)  :why "…")

(query :goal (gcd-of (d 1 (d 7 (d 0 (d 1 z)))) (d 2 (d 6 (d 4 z))) ?g) :expect …)
```

```text
                      ?g = (d 1 (d 2 z))
  :expect           holds
```

gcd(1071, 462) is 21. The program closes its own binary relation `gcd`, as round 1's finding 2 says it must.
`le/p/gcd-big.ein` is the same four rules on 123456789012345678 and 987654321987654; its `:expect`, Python's
`math.gcd`, holds. `le/p/factorial.ein` is 25! by five rules over `sub` and `mul`:

```text
                      ?f = (d 0 (d 0 (d 0 (d 0 (d 0 (d 0 (d 4 (d 8 (d 9 (d 5 (d 8 (d 9 (d 0 (d 3 (d 3 (d 3 (d 4 (d 0 (d 0 (d 1 (d 2 (d 1 (d 1 (d 5 (d 5 (d 1 z))))))))))))))))))))))))))
  :expect           holds
```

That is 15511210043330985984000000, twenty-six digits, which no `num9` holds.

#### (ii) Nested expressions, evaluated on demand

One fact holds an expression tree. `(calc E)` asks for its value, answered by `(value-of E V)`. The heads inside
an expression are `add`, `sub`, `mul`, `div`, `mod` and `cmp`, over numbers and names. Three rules per head do
it: ask for both sides' values; when both have one, state the round-1 request; when that is answered, the
expression has its value. `le/p/calc.ein`:

```lisp
(calc (add (d 8 (d 7 (d 0 (d 3 (d 2 (d 1 z)))))) (div (d 5 (d 4 (d 6 (d 7 (d 2 (d 1 z)))))) (d 5 (d 2 z)))))

(query :goal (value-of (add (d 8 (d 7 (d 0 (d 3 (d 2 (d 1 z)))))) (div (d 5 (d 4 (d 6 (d 7 (d 2 (d 1 z)))))) (d 5 (d 2 z)))) ?v)
       :expect …)
```

123078 + 127645 ÷ 25 is 128183, and the claim holds. The goal has to repeat the expression, which is the
inconvenience the user foresaw; names, below, remove it. Four deep, `le/p/calc-deep.ein`,
((127645 ÷ 25) × 3 − 15000) + 7:

```lisp
(calc (add (sub (mul (div N127645 N25) N3) N15000) N7))        ; the numbers written out in the file
```

```text
                      ?v = (d 2 (d 2 (d 3 z)))
  :expect           holds
```

What the two-number operations mean inside an expression:

- **Division is two heads.** `(div a b)` is the quotient and `(mod a b)` the remainder. Both state the same
  request, so the division runs once. `le/p/calc-div-mod.ein` computes `q`, `r` and `(add (mul q N25) r)`, and
  gets `q = (d 5 (d 0 (d 1 (d 5 z))))`, `r = (d 0 (d 2 z))` and the dividend back.
- **Subtraction is the difference where there is one.** `(sub a b)` has a value when a ≥ b. Otherwise the
  expression gets no value and a flag, `(negative (sub a b))`, and nothing that contains it gets a value
  either. `le/p/calc-negative.ein`: `low` is 4567 − 123078, `more` is `low` + 1, `fine` is 123078 − 4567.
  `(is ?name ?v)` lists `fine` alone, and `(negative ?e)` lists the one subtraction. The pair is still there for
  whoever wants it: the `diff-of` query in the same file prints both rows.
- **A division by zero** inside an expression leaves it without a value and derives the flag naming the request
  the expression reduced to: `le/p/calc-div-zero.ein`, 1 + 7 ÷ (5 − 5), gives no `is` row and
  `?request = (div (d 7 z) z)`.
- **Comparison is a head too.** `(cmp a b)` has the value `lt`, `eq` or `gt`, a name and not a number.

What a query with a variable for the expression sees (`le/p/calc-all-values.ein`): `(value-of ?e ?v)` lists
five rows, the expression, its sub-expression and the three numbers, each a number being its own value. And
`(sum-of ?x ?y ?s)` lists the one sum the expression reduced to. An expression's intermediate requests are
ordinary request facts and are visible as such.

#### (iv) Names

`(let NAME E)` asks for E's value and answers `(is NAME V)`. A name may then stand in a later expression.
Constants are the same form with a number for E. `le/p/let.ein`:

```lisp
(let a (d 8 (d 7 (d 0 (d 3 (d 2 (d 1 z)))))))
(let b (d 5 (d 4 (d 6 (d 7 (d 2 (d 1 z)))))))
(let x (add a (div b (d 5 (d 2 z)))))

(query :goal (is x ?v) :expect …)        ; ?v = (d 3 (d 8 (d 1 (d 8 (d 2 (d 1 z)))))), holds
```

A word problem, `le/p/word-problem.ein`: *a school orders 12 boxes of 144 pencils; 250 go to the office; the
rest is shared equally among 31 classes. How many does each class get, and how many are left over?*

```lisp
(let pencils   (mul (d 2 (d 1 z)) (d 4 (d 4 (d 1 z)))))
(let to-share  (sub pencils (d 0 (d 5 (d 2 z)))))
(let per-class (div to-share (d 1 (d 3 z))))
(let left-over (mod to-share (d 1 (d 3 z))))

(query :goal (is ?what ?v) :expect …)
```

```text
                      ?v = (d 1 (d 2 z)) · ?what = left-over
                      ?v = (d 7 (d 4 z)) · ?what = per-class
                      ?v = (d 8 (d 2 (d 7 (d 1 z)))) · ?what = pencils
                      ?v = (d 8 (d 7 (d 4 (d 1 z)))) · ?what = to-share
  :expect           holds
```

47 each, 21 left over. Without names (`le/p/word-problem-no-names.ein`) the shared sub-expression
`(sub (mul N12 N144) N250)` is written four times, twice in facts and twice in goals, and computed once; both
claims hold.

**A comparison decides a branch** (`le/p/branch.ein`): a rule reads a name's value and writes the next `let`.

```lisp
(let cost    (mul (d 7 (d 3 z)) (d 9 (d 2 (d 1 z)))))
(let budget  (d 0 (d 0 (d 0 (d 5 z)))))
(let verdict (cmp cost budget))

(rule it-fits ()         :match (is verdict lt) :assert (let change (sub budget cost))  :why "…")
(rule it-is-exact ()     :match (is verdict eq) :assert (let change (sub budget cost))  :why "…")
(rule it-does-not-fit () :match (is verdict gt) :assert (let missing (sub cost budget)) :why "…")
```

```text
                      ?v = (d 0 (d 0 (d 0 (d 5 z)))) · ?what = budget
                      ?v = (d 3 (d 7 (d 7 (d 4 z)))) · ?what = cost
                      ?v = (d 7 (d 2 (d 2 z))) · ?what = change
                      ?v = lt · ?what = verdict
  :expect           holds
```

Only the branch taken is computed: no `missing` is asked for.

**A recursion written as expressions** (`le/p/power.ein`): 2 to the power 64, where the name of each step is
itself a form, `(let (pow b n) (mul b (pow b (sub n 1))))`. Three rules of the program's own; the answer is
`(d 6 (d 1 (d 6 … (d 8 (d 1 z))…)))`, 18446744073709551616, and the claim holds.

#### What each protocol costs

`ein saturate --events`, then `depth.py` (2026-11-11), nested numbers:

| program | protocol | wiring firings | depth | facts stored | saturate ms |
|---|---|---:|---:|---:|---:|
| `le/programs/chain.ein`, (123 × 456) ÷ 25 | (i), one rule of the program's | 427 | 110 | 2150 | 5.5 |
| `calc.ein`, 123078 + 127645 ÷ 25 | (ii) | 441 | 105 | 2159 | 5.8 |
| `calc-deep.ein` | (ii), four deep | 475 | 128 | 2206 | 7.0 |
| `word-problem.ein` | (iv) | 395 | 110 | 2108 | 5.2 |
| `branch.ein` | (iv) and three rules | 83 | 57 | 1668 | 2.2 |
| `gcd.ein` | (i), four rules | 903 | 166 | 2827 | 10.6 |
| `gcd-big.ein` | (i), four rules | 22 900 | 3 290 | 34 434 | 830 |
| `factorial.ein`, 25! | (i), five rules | 2 926 | 2 501 | 5 727 | 59 |
| `power.ein`, 2⁶⁴ | (ii), three rules | 5 825 | 5 476 | 9 633 | 140 |

The expression layer adds three firings per operator and one per number. The arithmetic under it is the cost.
On `num9` the same `calc` is 32 wiring firings and the word problem 60 (`n9/`).

#### The recommendation for question 2

**Both layers, one on the other.**

- **(i) stays the base:** `(add X Y)` answered by `(sum-of X Y S)`, and so on. A program's own recursion is
  written against it (`gcd`, `factorial`), and it is what the theory itself is built from.
- **(ii) and (iv) are what a program normally writes:** `(let name E)` read by `(is name ?v)`, and `(calc E)`
  where no name is wanted. They cost three small rules per operator and need no engine change.
- **(iii) is not available.** It needs the engine to compute from a goal.

The honest alternatives: (i) alone is the smallest theory and makes every nesting a rule of the program's;
(ii) without `let` makes every goal repeat its expression. Two things are the user's to rule inside (ii): that
`sub` in an expression is strict with a `negative` flag, where it could be the truncated difference; and the
names `calc`, `let`, `is`, `value-of`, `div` and `mod`.

### D. Subtraction's pair (question 3)

> "I lean towards (a), any objections?"

No objection to (a) as what `(sub X Y)` answers. Two observations, neither against it:

- **The pair does not compose inside an expression,** since an expression has one value. Round 2 takes `sub`
  there as the difference when the first is not the smaller, and a `(negative (sub a b))` flag otherwise
  ([§ C](#ii-nested-expressions-evaluated-on-demand)). The pair stays available through the request.
- **On nested numbers (a) costs two ripples,** 28 firings where a sum takes 15, because both directions are
  subtracted and the one that does not borrow answers. A comparison first and one subtraction after would cost
  the same.

### E. Division by zero as `(false)` (question 5)

> "division by zero is definietely (false), (division-by-zero ...) also good, but why it wouldn't show the
> reasoning?"

Built on both shapes (`python3 gen_le.py --false`, `python3 gen.py 9 --r2 --false`): the rule `div-by-zero`
concludes `(and (division-by-zero (div N V)) (false))`. `le/f/div-zero-false.ein` asks 127645 ÷ 0 and, beside
it, 1 + 2:

```lisp
(import nat-false :symbols (nat))

(div (d 5 (d 4 (d 6 (d 7 (d 2 (d 1 z)))))) z)
(add (d 1 z) (d 2 z))

(solve :expect (false))
(query :goal (division-by-zero ?request))
(query :goal (sum-of ?x ?y ?s))
```

**`ein solve`**, whole (`le/f/div-zero-false.solve.txt`), exit 0:

```text
solve · div-zero-false.ein
──────────────────────────────────────────────────────────────
  asked             all of R — describe
  states (k_s)      0
  commitments (k_c) 0
  exhausted         yes — every commitment is classified: this is all of R
  empty             every node is dead — root's own fixpoint derives (false)
  lattice           monotone (the static check) — R is convex: no hole
  universe          no candidate: R is {∅} or empty, decided by root alone
  route             the state route, every alive state once — L1 dead is upward-closed · L3 a commitment's state is its closure's · L4 the lines are local
  work              1 saturation · 0 facts beyond root · 1 layer

  unsat core        (div (d 5 (d 4 (d 6 (d 7 (d 2 (d 1 z)))))) z)
  refuted by        div-by-zero

  (false)

  query 1 of 2      (division-by-zero ?request)
  over              all of R — R is empty: no state to answer from
  as :expect        (false)

  query 2 of 2      (sum-of ?x ?y ?s)
  over              all of R — R is empty: no state to answer from
  as :expect        (false)

  (solve :expect)   holds
```

**The other surfaces:**

| surface | what it shows |
|---|---|
| `ein test -v` on `(solve :expect (false))` | `solve · :expect (false) — holds (state route, k_s = 0, k_c = 0)`, 1 held, exit 0 |
| `ein test -v` on a claim that lists the flag, `le/f/div-zero-false-flag-claim.ein` | `FAILED — the claim lists 1 class, the query has 0: (model (division-by-zero (div … z))) is no class of all of R`, exit 1 |
| `--trace -G`, `le/f/div-zero-false.trace.md`, 130 lines | Root's derivation in full, thirteen steps. Step 11 is `div-by-zero`, with its two premises and *Derives `division-by-zero(div(d(5, …), z))` — (div (d 5 …) z) divides by zero*. Step 13 derives the sum 1 + 2. Then *N — 1 minimal dead sets*: ∅, its core and *Refuted by `div-by-zero`* |
| `--dump-answer DIR` | `answer.txt`, `answer.json`, `index.json` and `nogoods.json`, the last holding `"set": []`, the core, `"core_status": "within-budget"` and `"refuted_by": ["div-by-zero"]`. No `models/`: there is no state |
| `ein saturate --dump` | All three derived facts: `(division-by-zero (div … z) :rule div-by-zero)`, `(false :rule div-by-zero)` and `(sum-of (d 1 z) (d 2 z) (d 3 z) :rule add-answer)` |

**So the reasoning is shown, in two places.** `ein solve` names the request that divides by zero as the core,
and the rule as *refuted by*. `--trace` has the whole derivation, the flag fact included. What `(false)` takes
away is everything else:

- **Every query of the program loses its answer,** the ones that have nothing to do with the division too. The
  sum 1 + 2 is derived, and `ein solve` cannot print it: *R is empty: no state to answer from*.
- **The flag fact cannot be claimed or queried.** Only `(false)` can be claimed. The fact is in `ein saturate`'s
  dump and in the trace.
- **One core is shown for several divisions by zero.** `le/f/div-zero-false-in-expression.ein` has one written
  outright and one three levels down an expression. `ein saturate --dump` stores both flags,
  `(division-by-zero (div (d 9 z) z))` and `(division-by-zero (div (d 7 z) z))`; `ein solve` prints the core
  `(div (d 9 z) z)` and does not mention the other.
- **The trace's *Derives* line names a firing's first conclusion,** so step 11 shows the flag and not the
  `(false)` beside it.
- **The core stays short because the tables are rules.** A table row derived by a rule with an empty `:match`
  rests on nothing written, so the core is the request alone ([§ G](#g-the-flat-import-finding-a-specification-for-importing-facts) measures the
  other case).

**Recommendation.** For division by zero, what the user ruled: both conclusions, `(division-by-zero …)` and
`(false)`. It is built and it reads well: the core is exactly the offending request. The cost above is the
meaning of `(false)` and not a defect, and a program that wants to go on past a zero divisor compares first
(`cmp` against `z`) and branches. **For overflow under a fixed width, recommend the fact alone.** A sum that
does not fit nine digits is a limit of the spelling, not a contradiction in the problem, and it would empty R
for programs whose other answers are right. On nested numbers the question does not arise.

### F. All adder variants (question 9)

> "implement all versions, will compare later"

All five are in the prototype and runnable:

| variant | what | run |
|---|---|---|
| a | one rule, a premise per column (`num9`) | `python3 adders.py 9 q9/w9`, then `ein solve q9/w9/add-a.ein` |
| b | one rule per column, carry facts (`num9`) | `ein solve q9/w9/add-b.ein` |
| c | one rule over step names, shift registers (`num9`) | `ein solve q9/w9/add-c.ein` |
| d | recursive over a nested number, joining on the nested operands | `ein solve q9/w9/add-d.ein` |
| e | round 2: variant d with frames and a normal form, `le/nat.ein`'s adder | `ein solve le/programs/add.ein` |

`python3 q9_batch.py 9` runs a thousand sums through each (2026-11-11). Its line for the new one:
*width 9 variant e: 1000 sums of 1000 · 17889 firings · 25827 facts stored by the wiring · saturate 516.2 ms*,
against 35.7 ms for a, 327.1 for b, 31.6 for c and 956.1 for d.

The other operations exist in two variants each after round 2, one per number shape: subtraction,
multiplication, division, comparison and the expression layer are in `n9/arith9r2.ein` (wide rules, and shift
registers where the circuit iterates) and in `le/nat.ein` (recursion with frames). Only addition has more.

### G. The flat-import finding: a specification for importing facts

> "flat import drops facts - do we need it? if yes - then have to fix"

and, later the same day:

> "the flat import - together with theory libs that are rule libs we probably would want to have ontology libs
> that are facts libs, does it make sense? and then importing facts is mandatory"

So the answer to *do we need it* is yes, and this section is the specification of the fix. The fix is not made
here. It does make sense, and it is the split
[`solution_semantics.md` § 1.1](../../../docs/kernel/inference/solution_semantics.md#11-context-and-problem)
already states in the user's words: *theory = relations and their properties defined by rules, ontology =
simple facts like `(is-a Tee Drink)`, theory + ontology = context*. That page ends *"The boundary is not in the
language"*. Today the import is where it shows: a library can carry a theory into a program and cannot carry an
ontology into it flat. The digit tables are an ontology in exactly this sense, facts every solution needs and no
problem states.

#### 1. What each import form does today

**From the pages.** `stdlib/README.md` § Importing: *"`:symbols` keeps the listed names plus their dependency
closure"*. `docs/history/m1a_rust/design/04_ir_frontend.md` § 6: whole-module, *"every defined name prefixed"*;
aliased, *"prefixed `m.`"*; selective, *"listed declarations, flat and unrenamed"*. No page says what any form
does with a module's facts, and no test in `ein-ir/tests/imports_semantics.rs` holds it. The stdlib never met
the case: none of its seven modules holds a fact.

**From probes.** `g/imp/onto.ein` is a small ontology library: `(relation is-a Thing Type)`, a relation nothing
mentions, the facts `(is-a Tea Drink)`, `(is-a Milk Drink :source "the ontology")` and `(likes Tea Milk)` (whose
relation is not declared), two rules over `is-a`, and a macro. Five programs import it and add
`(is-a Coffee Drink)`; `ein saturate --dump` on each (2026-11-11):

| of the module | `(import onto)` | `(import onto :as o)` | `:symbols`, the name listed | `:symbols`, the name not listed |
|---|---|---|---|---|
| a declared relation | kept, renamed `onto.is-a` | kept, renamed `o.is-a` | kept, `is-a` | dropped, unless a kept declaration names it |
| a rule | kept, renamed `onto.things` | kept, `o.things` | kept, `things` | **dropped**, though the program's facts would fire it |
| a macro | kept, called as `(onto.both …)` | kept, `(o.both …)` | kept, `(both …)` | dropped |
| a fact of a declared relation | kept, **its relation renamed**: `(onto.is-a Tea Drink)` | kept, `(o.is-a Tea Drink)` | **dropped** | **dropped** |
| a fact of a relation the module does not declare | kept as written, `(likes Tea Milk)` | kept as written | **dropped** | **dropped** |
| a fact's `:source` | kept: the fact is *given* | kept | — | — |
| a constant in a fact, `Tea` | unrenamed | unrenamed | — | — |

Four things follow, each seen in the dumps:

- **A flat import brings no fact, by construction.** `ein_ir::imports`' `select` keeps a form only if it is a
  declaration whose name is in the closure. This is the round-1 finding.
- **A qualified import brings the facts into a different relation.** The module's `(is-a Tea Drink)` becomes
  `(o.is-a Tea Drink)`, while the program's own `(is-a Coffee Drink)` stays `is-a`, a second relation. The
  module's rule derives `(thing Tea)` and `(thing Milk)` and not `(thing Coffee)`. So a qualified ontology does
  not join the program's vocabulary: the prefix that keeps two libraries apart also keeps the library and the
  program apart.
- **What is renamed is every atom that is a declared name of the module,** wherever it stands, as a head or as
  an argument (`rename_atoms`). A relation the module uses and does not declare (`likes`, and the `thing` its
  rule asserts) is not renamed, and no constant is, unless it happens to share a declared name.
- **An unlisted rule is dropped silently** under `:symbols`: `(import onto :symbols (is-a))` loads the relation
  and neither rule.

Three more probes:

- **An imported fact's constants become names of the program** (`g/imp/names.ein`). With the ontology imported
  as `o` and one unary relation of the program's left open, `ein solve -H` lists 39 candidates over 3 relations,
  among them `(o.is-a Coffee Tea)` and `(o.is-a Coffee Milk)`: `Tea`, `Milk` and `Drink` are objects the blind
  enumerator proposes facts about. It also proposes `(o.is-a Coffee liquid)` and `(o.is-a Coffee thing)`, the
  relation names the module's rules assert, which is round 1's `__closed__` roughness again.
- **One module under two names is two copies** (`g/imp/twice.ein`): `(a.is-a Tea Drink)` and
  `(b.is-a Tea Drink)`, two relations. The undeclared `(likes Tea Milk)` is one fact.
- **A fact stated twice keeps its first statement.** The same file states `(likes Tea Milk :source "the
  program")` after the imports; the dump lists `(likes Tea Milk)` as background, the module's spelling, and the
  program's `:source` is gone without a word.

#### 2. The proposed semantics, and the open points

**The proposal in one sentence: a flat import brings the module's facts, unrenamed, and a fact needs no name to
come.** The points below are the design; each has its options and a recommendation.

**(a) Which facts does `:symbols` bring?** A fact has no name to list.

| option | rule | for | against |
|---|---|---|---|
| a1 | every fact of the module, always | simplest to say; an ontology library is imported for its facts | a module that mixes rules with facts gives its facts to an importer who listed one rule |
| a2 | the facts whose relation is in the selection's closure | selection stays a selection; `(import onto :symbols (is-a))` brings the `is-a` facts and not `likes` | a fact of a relation the module does not declare can never be listed, unless a relation's name is listable without a declaration |
| a3 | a2, and the listed names may also be relations the module only states facts of | everything is reachable; nothing comes unasked | one more thing `:symbols` can name |
| a4 | a new form for the whole module, flat, `(import onto :all)` or a bare `(import onto)` redefined | no selection to define | a grammar change, or a change to what the bare form means for the 6 corpus imports that use it |

**Recommend a3.** A listed name selects the module's declaration of it, if any, and every top-level fact whose
relation it is. The closure then follows names as it does today, so a rule that reads `is-a` brings the `is-a`
facts with it. It adds nothing an importer did not ask for, which is what `:symbols` has always meant, and the
calculator's `(import std.arith :symbols (arith))` keeps working because the closing rule names every relation.

**(b) What does a qualified import do to the names inside a fact?** Today it renames declared names and leaves
the rest, so the module's facts land in `o.is-a`.

- b1, keep today's rule. A qualified ontology stays its own vocabulary, which the probe shows is not what an
  ontology is for.
- b2, never rename a relation that heads a fact. Then `(o.is-a …)` rules and `is-a` facts would split inside
  the module itself.
- b3, leave qualified imports as they are and say on the page that an ontology is imported flat.

**Recommend b3.** Renaming is what a qualified import is; the fix belongs to the flat form. Constants are not
renamed under any form and should stay so: `Tea` in the library is `Tea` in the program, which is the point of
an ontology.

**(c) Do imported constants become names of the program?** They do today under a qualified import, and they
enter H's objects. The options are to keep that, or to mark library constants so the blind enumerator skips
them. **Recommend keeping it:** an imported fact is a fact of the program, and
[`solution_semantics.md` § 1.2](../../../docs/kernel/inference/solution_semantics.md#12-names) defines the
program's names as the constants its facts and rules write. A program that does not want guesses about a
library relation closes it, or the library closes its own, as the calculator does. The cost to know: importing
an ontology of n constants widens the blind universe of every open relation by n.

**(d) The provenance of an imported fact.** Today a fact keeps its own `:source` and is otherwise background,
the same as if it were written in the program. Options: keep that; or stamp a fact that has no `:source` with
the module's name. **Recommend keeping it for the fix, and leaving the stamp to
[F24](../../followups/f24_written_provenance.md),** which already owns what a written provenance may claim. Under
§ 1.1's table an imported fact with no `:source` reads as *context*, which is right for an ontology.

**(e) Duplicates.**

- The same module imported flat twice, or through a diamond: declarations are already idempotent (identical
  ones collapse); a fact is a member of a set, so the second copy is the first. Nothing to add.
- The same fact from two modules, or from a module and the program, with different `:source`s: today the first
  statement wins in silence (the `twice.ein` probe). **Recommend leaving the behaviour and naming it for F24,**
  since it is the same question as a fact written twice in one file.
- A module under two qualified names stays two copies, by (b).

**(f) An unlisted rule.** Not part of this fix. A selection that drops a rule the importer did not name is what
`:symbols` says. The calculator's closing rule, which names every rule, stays the way one symbol brings a
theory.

#### 3. What the fix would move

`python3 build/s1f.6/prototype/scan_imports.py` (2026-11-11) reads every tracked `.ein` file under `stdlib/`,
`examples/`, `tests/` and elsewhere: **663 files, 173 flat imports, 3 aliased, 6 bare, and no import, under any
form, of a module that holds a top-level fact.** The seven stdlib modules hold none. So under a1, a2 or a3 no
corpus program loads one more fact, and no corpus answer can change. Under a4 with the bare form redefined, the
6 bare imports would lose their prefixes, which moves their output and is a reason against it. Not scanned: the
programs written inline in the Rust test suites, which the fix's own gate run would reach.

What it needs besides the code in `select`: the rule stated in `stdlib/README.md` § Importing and in
`01_grammar.md`'s import row, a line in `solution_semantics.md` § 1.1 that an import can bring ontology as well
as theory, a case per row of the table above in `imports_semantics.rs`, and a corpus fixture that imports an
ontology library flat.

#### 4. The calculator's tables as plain facts

The same program three ways (`g/`, 2026-11-11): 123078 + 4567 on nested numbers, plus one rule of the program's
that concludes `(false)` on a sum of six digits, so that a core is printed. The first is today's workaround.
The second is what a qualified import gives today. The third is the theory pasted into the program, which is
what a flat import that kept facts would load.

| | tables as rules with an empty `:match`, flat import (`rom-flat.ein`) | tables as facts, `:as n` (`facts-qualified.ein`) | tables as facts, inlined (`facts-inline.ein`) |
|---|---|---|---|
| a request reads | `(add (d 8 …) …)` | `(n.add (n.d 8 …) …)` | `(add (d 8 …) …)` |
| a table row is | derived, `:rule fa-table` | background | background |
| background facts, derived facts | 101, 1 469 | 1 418, 148 | 1 418, 148 |
| saturation | 1.08 ms, 21 firings | 0.28 ms, 17 firings | 0.27 ms, 17 firings |
| `ein solve` wall, best of ten | 23 ms | 21 ms | 20 ms |
| `--trace` | 3 360 lines | 654 lines | 654 lines |
| `--trace -G` | 194 lines | 162 lines | 162 lines |
| the core of the `(false)` | the request alone | the request and seven table rows | the request and seven table rows: `(fa 0 5 1 6 0)`, `(fa 1 0 0 1 0)`, `(fa 2 0 0 2 0)`, `(fa 3 4 0 7 0)`, `(fa 7 6 1 4 1)`, `(fa 8 7 0 5 1)`, `(nz 1)` |

- **Cost:** no difference that matters. Loading 1 300 rows is the cost either way, about 20 ms.
- **Trace:** plain facts remove the four table steps, each drawn as a graph of up to 900 nodes: 3 360 lines
  become 654. Without diagrams the two are within 32 lines.
- **Provenance:** a table row is a background fact, which is what it is. As a rule's conclusion it is labelled
  *derived by `fa-table`*, and the four ROM rules and their firings appear in every count.
- **Cores change, and not for the better.** A written fact can be in a core; a fact derived from nothing
  cannot. With tables as facts every refutation that passed through arithmetic lists the table rows it used
  beside the request. This is the one thing the workaround does better, and it bears on question 5: with
  `(false)` on division by zero the core is the request alone on both, since that rule reads no table; a
  program's own `(false)` over a sum is where the rows appear. Whether a core should name library facts is a
  question for whoever makes cores canonical, F22.

**So for the calculator:** once the fix exists, the tables are written as plain facts and imported flat, the
four ROM rules go, and the closing rule stays for the `(__closed__ …)` facts and to name the rules. Until then
the prototype's empty-`:match` rules compute the same answers at the same cost.

### What round 2 found in the engine

None is a wrong answer about arithmetic, and none moves a zebra program, so under the ruling of 2026-11-11 each
is a row for [F25](../../followups/f25_roughness_found_outside_zebra.md) and not a question. This stage did not
edit F25. The first bears on question 1, which is why it is spelled out above.

1. **The fact index does not key on a nested fact,** so a join on a value that is a fact scans the relation
   ([§ what it costs](#what-it-costs)). Witness: `python3 q9_scale.py`.
2. **A state file holding a fact nested past 1 000 does not load back.** Witness: `le/x/deep.ein` under
   `--dump-answer`; *this ( nests deeper than the limit of 1000 parentheses*.
3. **The trace's *Derives* line names the first conclusion of a firing only.** Witness: step 11 of
   `le/f/div-zero-false.trace.md`, and every table step.

### The state of each question after round 2

| question | the user's answer | state | what remains |
|---|---|---|---|
| 1, the number's shape | "try little-endian nested list, looks promising" | **open** | Both shapes are built and agree with Python. Nested numbers remove the width, overflow and the digit count, and cost 15 to 20 times the firings and a quadratic batch. The recommendation is nested numbers, `z` and `(d DIGIT NUMBER)`, with the index gap known |
| 2, the protocol | "do experiments, show me more examples" | **open** | Recommendation: request facts as the base, `calc` and `let` on top; the goal form is refused today |
| 3, subtraction's two numbers | "I lean towards (a), any objections?" | **ruled (a)** unless the user says otherwise | No objection; inside an expression `sub` is strict, with a `negative` flag |
| 4, the width of a result | "another point for little-endian nested list, but is it possible to simplify leading zeros?" | **follows question 1** | With nested numbers there is no width; every answer is in normal form |
| 5, overflow and division by zero | "division by zero is definietely (false), (division-by-zero ...) also good" | **ruled for division by zero**: both conclusions | Overflow under a fixed width: recommend the fact alone; moot under nested numbers |
| 6, how H stays empty | "(a), it doesn't matter now actually" | **ruled (a)** | — |
| 7, the tables | "(a)" | **ruled (a)** | — |
| 8, where the theory lives | "(a)" | **ruled (a)** | The import form is (a1), tables as rules with an empty `:match`, until a flat import brings facts (§ G) |
| 9, how a circuit is written | "implement all versions, will compare later" | **open, by the user's choice** | Five adders runnable; the other operations in two variants |
| finding 1, the flat import | "do we need it? if yes - then have to fix"; then "importing facts is mandatory" | **ruled: needed; the fix is specified, not made** | § G's open points (a) to (f), each with a recommendation; who makes the fix and when |
| finding 2, the program's own relations | "I think it's ok for now … author knows what he's doing" | **ruled: left as it is** | — |
| finding 3, the wrong digit count | "another point for little-endian nested lists?" | **follows question 1** | Yes for the digit count; a non-digit name or a missing `z` is still answered by silence |

**What the stop asks now:** question 1, on the costs above; question 2, on the examples; inside question 2,
whether `sub` in an expression is strict; question 5's overflow half if `num9` stays; whether the spelling
`z` and `d` stands; and § G's points (a) to (f) on importing facts, with where that fix is scheduled.
