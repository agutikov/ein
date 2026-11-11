# S1f.6.12 T1 — malformed numbers and occupied names: what the theory can detect, and what it costs

**Phase:** [P1f.6](README.md) · **Stage:** [S1f.6.12](s1f.6.12_malformed_numbers.md), its first task and its
second task's probes.
**Status:** taken 2026-11-11, on the shipped `std.arith` (89 rules, `8860d616`). **Stops for the user:**
[§ 8](#8-the-questions-for-the-user) has the questions. Nothing here is built into the stdlib or the engine.

The user, 2026-11-11: *"with theory loaded names `n`, `d`, `z`, `sum-of` etc became occupated and must not be used
with other meaning, so if theory can detect malformed numbers - do it, what cons/objections?"*

Everything on this page was run. The probes, the prototype theories and the scripts are under `build/s1f.6.12/`
(gitignored), and each table names the command that re-takes it.

## 1. The answer in one paragraph

**Most of what a person mistypes can be detected by the theory, with no `absent` and no `not`, for twenty firings a
request.** A digit place that holds anything but a digit, a number that ends in a digit or a named constant where
`z` goes, a digit written where a number goes, and a name bound to two numbers are each flagged by a rule that
derives `(malformed REQUEST)`, and a second rule derives `(false)` from the flag, as for a division by zero. **Three
things cannot be detected by a theory at all**, and the reason is the same for each: saying *this is not of that
shape* is a negation. They are a name no `let` states, a fact of another relation where a number goes, and a number
or a request with the wrong count of arguments. The third is what a **loader check** catches, and it would refuse
no program the corpus loads today; the first needs one `absent` or stays silent; the second stays silent. The
objections that survived the experiment are in [§ 7](#7-the-objections-each-weighed).

## 2. What a malformed request does today

`ein solve` on each, a program of the import, one request and a query for its answer
(`build/s1f.6.12/today/<name>.ein`, each with its `.solve.txt` and `.dump.txt`; 2026-11-11). **Every one exits 0,
writes nothing to stderr, reports `states (k_s) 1`, and answers the query with**

```text
    class 1 of 1    1 state (1) — the goal has no row: undetermined here
```

which is what a program that asked nothing prints. Where the circuit stops is in root's state for whoever dumps it:

| | the request | what is wrong | where the circuit stops (`ein saturate --dump`) |
|---|---|---|---|
| m1 | `(add (n (n z 1) x) (n z 5))` | a name that is no digit, in a digit's place | the walk's first fact: no row of `fa` has `x` |
| m1b | `(add (n (n z x) 1) (n z 5))` | the same, a column up | after one column: `(add-walk … (n z x) z 0 (b 6 z))` |
| m2 | `(add (n z (n z 1)) (n z 5))` | a fact in a digit's place | the walk's first fact |
| m3 | `(add (n 1 2) (n z 5))` | no `z`: the number ends in the digit 1 | `(add-walk … 1 z 0 (b 7 z))`: the walk reads `1` as what is left |
| m3b | `(add (n end 1) (n z 5))` | no `z`: the number ends in a name | `(add-walk … end z 0 (b 6 z))` |
| m4 | `(add foo (n z 5))` | a name no `let` states | `(operand (add-first foo …) foo)`: the operand is never found |
| m4b | `(let a b)`, `(add a (n z 5))` | a name bound to a name | `(add-walk … b (n z 5) 0 z)` |
| m9 | `(add 7 (n z 5))` | a digit where a number goes: 7 is `(n z 7)` | the operand is never found |
| m5 | `(add (pair 1 2) (n z 5))` | a fact of another relation as an operand | the operand is never found |
| m5b | `(add (n (pair 1 2) 3) (n z 5))` | the same, inside a number | after one column |
| m8 | `(add (b 1 z) (n z 5))` | the theory's own second shape, as an operand | the operand is never found |
| m6 | `(add (n 1) (n z 5))` | `n` with one argument | the operand is never found |
| m6b | `(add (n z 1 2) (n z 5))` | `n` with three | the operand is never found |
| m6c | `(add (n z 5))` | a request with one operand | nothing: no rule matches `add` with one argument |
| m6d | `(add (n z 5) (n z 1) (n z 2))` | a request with three | nothing |
| m7 | `(add (n (n (n z 0) 0) 7) (n (n z 0) 5))` | **nothing: high zeros are read by value** | answered, `(n (n z 1) 2)` |

`m6c` and `m6d` say something about the loader that E-2 of [`findings.md`](findings.md) did not: **a top-level fact
with the wrong count of arguments for its declared relation loads too**, not only a nested one. `add` is declared
`(relation add Operand Operand …)`.

## 3. What a rule can say without negation

Three tools, each tried (`build/s1f.6.12/neq/g1.ein`, `ein solve`, 2026-11-11):

1. **Ten `neq` guards: "none of the ten digits".** `(neq ?a 0) … (neq ?a 9)` over a bound `?a` is computed, monotone
   (M1f S1f.5.7), and true of every other name **and of every fact**: of `x`, `z`, `(pair 1 2)` and `(pair x y)`,
   and not of `3`. So one rule covers a wrong name and a fact in a digit's place. One table of the ten digits with
   one guard, `(digit ?d) (neq ?a ?d)`, is not cheaper: it holds nine times for a digit that is right.
2. **A positive lookup: "is a digit", "is a named constant".** `(nz ?m)` and `(named ?m)` with `?m` bound are one
   interned lookup each (E-3's note), whatever `?m` is.
3. **A guard over the program's names: "is a name other than `z`".** `(neq ?v z) (eq ?v ?m)`, with `?v` bound by
   nothing else, ranges `?v` over the program's names and keeps the one that is `?m`: it held of `3` and `x` and not
   of the two facts. It is the only way found to say *a name* as against *a fact*, and it costs the count of the
   program's names every time it is asked ([§ 5](#5-what-detection-costs-a-well-formed-request)).

**What no rule can say:**

- ***not of the shape `(n P D)`***, or *a fact of another relation*. A pattern names its relation; a relation
  written as a variable inside a pattern is refused at load (W79); and `neq` compares with a value, not with a
  shape (a form as a guard's argument is refused, W24).
- ***no `let` states this name.*** That a `let` exists is a fact; that none does is its absence.
- **A wrong count of arguments, in general.** A rule *can* match one wrong count it names: `(operand ?k (n ?a))` and
  `(operand ?k (n ?a ?b ?c))` load and match `(n 1)` and `(n z 1 2)` (`build/s1f.6.12/misc/arity-pattern.ein`). So
  a theory could flag one argument and three, the likely slips, and no other count.

## 4. The table: each malformation, and what detects it

The prototypes are the shipped `stdlib/` with rules added to `arith.ein` by `build/s1f.6.12/proto.py`:
**b1**, a checking pass at the operand with the first two tools; **b2**, b1 and the third tool; **a**, no pass, the
ten guards on `add-walk`'s own columns, addition alone, to price a guard inside a walk. *Output* is `ein solve`
under `EIN_STDLIB=build/s1f.6.12/stdlib-b1` (or `-b2`).

| | malformation | detected, with no `absent` and no `not`? | by | recommendation |
|---|---|---|---|---|
| m1, m1b | a name that is no digit, in a digit's place | **yes**, b1 | `check-digit`: `(and (check ?k (n ?p ?a)) (neq ?a 0) … (neq ?a 9))` | **build** |
| m2 | a fact in a digit's place | **yes**, b1 | the same rule: `neq` holds of a fact | **build** (no rule of its own) |
| m3 | no `z`: the number ends in a digit, `(n 1 2)` | **yes**, b1 | `check-prefix-digit`: `(and (check ?k ?m) (nz ?m))`, and `(check ?k 0)` | **build**: it is the slip a person makes |
| — | no `z`: the number ends in a named constant, `(n price 1)` | **yes**, b1 | `check-prefix-named`: `(and (check ?k ?m) (named ?m))` | **build** |
| m3b | no `z`: the number ends in another name, `(n end 1)` | yes, **b2 only** | `check-prefix-name`: `(and (check ?k ?m) (neq ?v z) (eq ?v ?m))` | **do not build**: its cost grows with the program's names, § 5. Stays silent, named on the module's page |
| m9 | a digit where a number goes, `(add 7 X)` | **yes**, b1 | `operand-digit`: `(and (operand ?k ?x) (nz ?x))`, and `(operand ?k 0)` | **build** |
| m4b | a name bound to a name, `(let a b)` | b1 where `b` is itself named or a digit; **b2** otherwise | the prefix rules above, on what the `let` gave | as m3 and m3b |
| m4 | **a name no `let` states** | **no** | — | **the user's to rule, Q2**: silence as today, or one `absent` over `let` |
| m5, m5b, m8 | a fact of another relation where a number goes | **no** | — | stays silent, named on the page. No loader check short of typed arguments reaches it |
| m6, m6b | `n` with the wrong count of arguments | one and three arguments, by a pattern each; **not in general** | — | **the loader, Q1**: refused at load, which is better than a flag |
| m6c, m6d | a request with the wrong count of operands | no | — | **the loader, Q1** |
| m7 | high zeros | — | — | not malformed: read by value, as shipped |

And three that are not malformed numbers but the same family, found by T2's probes
([§ 6](#6-the-names-the-theory-occupies-what-a-clash-does-today)):

| | what | today | detected? | recommendation |
|---|---|---|---|---|
| c8 | **a name stated twice with two numbers**, `(let price 5)`, `(let price 7)` | **two answers to one request**: `(sum-of price 1 6)` and `(sum-of price 1 8)`, exit 0, no word | **yes**, b1: `let-twice`, `(and (let ?x ?v) (let ?x ?w) (neq ?v ?w))`, a join on the name, which the index keys | **build**: today's answer is wrong in the plain sense |
| c5 | **`z` or a digit as a constant's name**, `(let z 5)` | `(add z 5)` has two answers, 5 and 10: `z` stands for itself and for 5 | **yes**, b1: `let-zero`, `let-digit` | **build** |
| c9, c10 | a written fact of a result or of the circuit, `(sum-of 2 2 5)`, `(done (sum 2 2) 5)` | a second answer, or a forged one: `2 + 2 = 5` | no: a written fact and a derived one are one fact | the occupied-names question, Q4 |

**What a detection reads as** (`m1` under b1; `build/s1f.6.12/today/m1-nondigit.ein`):

```text
  states (k_s)      0
  empty             every node is dead — root's own fixpoint derives (false)
  unsat core        (add (n (n z 1) x) (n z 5))
  refuted by        malformed-is-false
```

and in `--trace -G`, the steps that matter:

```text
### Step 12 — `malformed-add-first`
> the first operand of (n (n z 1) x) + (n z 5) is not a number
### Step 14 — `malformed-is-false`
> (add (n (n z 1) x) (n z 5)) is not a request the theory can read
```

The flag is `(malformed (add (n (n z 1) x) (n z 5)))`, the request as written, and the unsat core is the request
alone, or the request and the `let` where the bad number came through a name (`o6`: `(add a (n z 1))` and `(let a
(n z x))`). A `let` nobody uses is not checked (`o7`): the theory computes on demand, and so does its check. All
five operations flag, either operand (`o1` to `o5`). **Every well-formed program still answers as before under b1
and b2:** `ein test tests/stdlib/arith examples/arith`, 67 of 67 claims held (2026-11-11).

**A foreign fact that happens to use the theory's names is not flagged.** `(n alice bob)` beside a sum, and `z` in
a relation of the program's own, load and answer under b1 and b2 as today (`c3`, `c4`): the check starts from an
operand that was asked for, never from a fact of `n` because it exists.

## 5. What detection costs a well-formed request

**Where the check runs.** As a pass of its own, at the one place every operand of every request goes through:
`operand-is`. `check-start` asks, `check-step` goes one place down a firing, and the flag rules watch the walk. Its
cost is a firing a digit of each operand. The alternative, guards on each circuit's own walks, costs no firing and
was priced on addition (prototype **a**): but there are seven walks (`add-walk`, `sub-walk`, `cmp-walk`, `mul-walk`,
`top-first`, and the two ends of each), a flag raised inside a row of a multiplication or a trial of a division
names a tag some calls deep and not the request, and a number that ends wrong is met only where a walk ends. The
pass is one mechanism for five operations and names the request by ten small rules, one per operand tag.

**Firings and time,** the release binary, batches of nine-digit operands (a divisor of four), best of five
(`python3 build/s1f.6.12/cost.py`, 2026-11-11):

| operation | requests | shipped: firings a request | ms | a: firings a request | ms | b1: firings a request | ms | b2: firings a request | ms |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| add | 250 | 27.0 | 8.7 | 27.0 | 9.1 | 47.0 | 14.9 | 47.0 | 24.1 |
| add | 1000 | 27.0 | 32.6 | 27.0 | 35.2 | 47.0 | 58.7 | 47.0 | 97.8 |
| add | 4000 | 27.0 | 138.9 | 27.0 | 147.3 | 47.0 | 260.8 | 47.0 | 407.3 |
| sub | 250 | 37.0 | 12.4 | 37.0 | 12.3 | 57.0 | 18.6 | 57.0 | 28.0 |
| sub | 1000 | 37.0 | 50.8 | 37.0 | 50.5 | 57.0 | 75.5 | 57.0 | 114.5 |
| sub | 4000 | 37.0 | 231.1 | 37.0 | 218.3 | 57.0 | 325.5 | 57.0 | 482.2 |
| cmp | 250 | 16.0 | 6.5 | 16.0 | 6.5 | 36.0 | 13.1 | 36.0 | 22.7 |
| cmp | 1000 | 16.0 | 25.5 | 16.0 | 25.4 | 36.0 | 52.8 | 36.0 | 91.0 |
| cmp | 4000 | 16.0 | 104.4 | 16.0 | 104.4 | 36.0 | 222.8 | 36.0 | 377.9 |
| mul | 250 | 486.3 | 180.5 | 486.3 | 185.5 | 506.3 | 189.1 | 506.3 | 199.1 |
| mul | 1000 | 486.0 | 781.5 | 486.0 | 803.7 | 506.0 | 812.4 | 506.0 | 850.9 |
| mul | 4000 | 486.2 | 3828.7 | 486.2 | 3905.9 | 506.2 | 3984.7 | 506.2 | 4112.5 |
| div | 250 | 448.3 | 172.4 | 448.3 | 171.6 | 463.3 | 185.7 | 463.3 | 194.3 |
| div | 1000 | 447.3 | 728.4 | 447.3 | 728.6 | 462.3 | 795.2 | 462.3 | 824.3 |
| div | 4000 | 447.5 | 3469.4 | 447.5 | 3495.8 | 462.5 | 3840.8 | 462.5 | 3946.9 |

| 1 000 sums beside | shipped: ms | a: ms | b1: ms | b2: ms |
|---|---:|---:|---:|---:|
| 0 `let`s | 33.0 | 35.0 | 58.8 | 97.5 |
| 1000 `let`s | 33.4 | 35.4 | 60.0 | 264.6 |
| 4000 `let`s | 37.1 | 39.2 | 63.8 | 761.2 |

**The matcher's own count,** `candidates` of `ein_core::counters`, which is what `arith_batch.rs` holds
(`EIN_STDLIB=build/s1f.6.12/stdlib-<k> cargo test -p ein-infer --features counters --test arith_batch --
--nocapture`, 2026-11-11), a request at 250, 1 000 and 4 000 requests:

| operation | shipped | a | b1 | b2 |
|---|---:|---:|---:|---:|
| add | 181.4 | 181.4 | 181.4 | 181.4 |
| sub | 362.0 | 362.0 | 362.0 | 362.0 |
| cmp | 901.0 | 901.0 | 901.0 | 901.0 |
| mul | 9 603.9, 9 598.7 at 4 000 | the same | the same | the same |
| div | 6 034.0, 6 030.9 at 4 000 | the same | the same | the same |

The four columns are equal to the last digit, and that is a finding about the measure: **`candidates` counts the
facts the matcher offers to a relation premise, and a guard is none.** A rule seeded by one fact whose other
premises are guards, which is what every check rule is, offers nothing; so the counter reads a detection as free,
b2's too, and `arith_batch.rs`, which holds the theory's proportion on that counter, would pass on b2. The firings
and the clock are what price a detection. E-27 of [`findings.md`](findings.md).

What they say:

- **b1 costs twenty firings a request,** one a digit of each operand and one to start each, whatever the operation:
  47 for a sum where 27, 36 for a comparison where 16, 506 for a product where 486 (fifteen for the division, whose
  divisor here has four digits). In time, at 4 000 requests: 261 ms where 139 for a sum, 326 where 231 for a
  difference, 223 where 104 for a comparison, 3.98 s where 3.83 for a product, 3.84 s where 3.47 for a quotient.
- **A batch still costs in proportion to its count** under b1: twenty firings more a request at 250, at 1 000 and
  at 4 000, and 4 000 sums take 17.5 times what 250 take, where the shipped theory's take 16.0 times.
  `arith_batch.rs` passes on it unchanged. `let-twice` joins two `let`s on the name, which the index keys.
- **The guards themselves are nearly free on a digit that is right:** prototype **a** adds no firing to a sum and
  a twentieth to its time at most (147 ms where 139 at 4 000). The cost of b1 is its walk, not its ten comparisons.
- **b2 is the one that does not scale.** Its guard ranges over the program's names once a digit, so its cost grows with
  the program and not only with the request: 1 000 sums take 97.5 ms beside no named constant, 265 ms beside 1 000
  `let`s nobody asks about and 761 ms beside 4 000, where b1 takes 58.8, 60.0 and 63.8 ms and the shipped theory 33.0,
  33.4 and 37.1. So the general test for *a name* is not affordable in a checking pass, and m3b is recommended
  silent.

**What would halve b1:** checking each operand once per distinct number is not available (that is a join on a
number), but the check could ride on the first walk of an operation instead of a walk of its own where the
operation has one walk over each operand (a sum, a difference, a comparison). That is the mixed design, three more
flag paths, and the build's to weigh once the user has ruled what is detected.

## 6. The names the theory occupies: what a clash does today

`build/s1f.6.12/clash/<name>.ein`, each with its `.solve.txt` and `.solve.err` (2026-11-11). The import is flat,
`(import std.arith :symbols (arith))`, but for the last two.

| | the program | today |
|---|---|---|
| c1 | declares its own `(relation n Person Count …)` | **refused at load**, exit 1: `kb load error: conflicting definitions of 'n': same name, different body (at None)` |
| c2, c6 | declares its own `sum-of`, or `done`, an internal name | refused, the same line |
| c12 | writes a rule named as one of the theory's, `add-column` | refused, the same line |
| c11 | writes a rule named as one of the theory's **relations**, `sum` | loads: a rule's name and a relation's do not clash |
| c3 | states `(n alice bob)` and never declares `n` | loads; the fact sits beside the theory's and the sum is answered |
| c7 | states `(done laundry)` and `(b 1 2)` | loads; no rule matches them |
| c4 | uses `z` as a name of its own, `(likes z alice)` | loads; `z` is a name like any other, and its query answers `?a = z` |
| c5 | `(let z (n z 5))`, `(add z (n z 5))` | **two answers**, 5 and 10 |
| c8 | `(let price …)` twice, two numbers | **two answers** |
| c8b | `(let price …)` twice, one number | one answer: a fact stated twice is one fact |
| c9 | writes `(sum-of (n z 2) (n z 2) (n z 5))` beside the request | **two answers**, 4 and 5 |
| c10 | writes `(done (sum (n z 2) (n z 2)) (n z 5))` and no request | **an answer nobody computed**: `2 + 2 = 5` |
| q1 | `(import std.arith :as a)`, and its own `(relation n …)` | loads; the theory's names are `a.n`, `a.add`, `a.sum-of`, and nothing clashes |
| q2 | the same, and writes `(a.n a.z 1)` | **silence**: `z` is a name, not a declaration, so a qualified import does not prefix it, and `a.z` is another name. A malformed number, by the prefix |

So:

- **A second declaration of an occupied name is already refused**, for every relation and rule of the theory,
  internal ones too, and the refusal names the name. Its message has no place, `(at None)`: E-21 of
  [`findings.md`](findings.md), found by S1f.6.20 the same day.
- **Facts are not.** A program may state facts of any of the theory's relations, and the theory will compute with
  them: a result, a hand-over, a `let`. Nothing distinguishes a written `(sum-of …)` from a derived one.
- **`z` and the digits are names, and names cannot be occupied:** they are shared with every other use, harmlessly
  (c4), except as a constant's name (c5).
- **A qualified import is the existing way out** for a program that wants `n` for itself, at the price of writing
  `(a.n (a.n z 1) 7)` for 17, and with q2's trap.

## 7. The objections, each weighed

1. **Cost.** Confirmed and bounded: twenty firings a request for b1, linear, 0.03 ms a request: nine tenths more
   time on a sum, four tenths more on a difference, twice on a comparison, and a twentieth to a tenth on a product or
   a quotient. Not confirmed for the guards inside a walk, which are nearly free.
2. **Completeness: a program cannot read the absence of a flag as *well-formed*.** Confirmed, and it is the main
   objection. With b1 built and the loader check ruled in, what stays silent is a name no `let` states (unless Q2),
   a number that ends in a name that is neither a digit nor a constant, and a fact of another relation in a
   number's place. The module's page would have to say *a flag means malformed; no flag does not mean
   well-formed*, with that list.
3. **`(false)` for a typing mistake empties R for the whole program.** It is the ruled meaning, as for a division
   by zero, and the witness shows what the user gets for it: the request, as written, as the unsat core. Under
   today's silence the same mistake is a query with no row and nothing to say why.
4. **Rules that fire on other people's facts.** Dropped: the check is asked for by an operand and never ranges
   over `n`'s facts, and `c3`, `c4` and `c7` answer under the prototypes as they do today.
5. **A core names table rows.** New, and small: where the flag rests on a table lookup, the core lists the row,
   `(cmp (n 1 2) z)` and `(nz 1)` for `o3`. It is the known cost of tables as plain facts
   ([S1f.6.2](s1f.6.2_digits_and_tables.md)), and F22's to make canonical.
6. **Twenty-two rules and three relations more** in a module of 89, each to be activated by a program under
   `tests/stdlib/arith/` and held by the mutation sweep; ten of them are the tag-to-request rules, alike but for
   the operation.
7. **Two more occupied names,** `malformed` and `check` (and `bad-operand`), and one more rule that concludes
   `(false)`, which `the_theory_writes_no_negation` holds by name.
8. **`neq` in the theory.** The shipped module writes no guard at all, and its static test bans `neq` with
   `absent`, `not` and `or`. `neq` is monotone and computed, so the ban is a taste and not a need; the build would
   lift it for the named rules.

## 8. The questions for the user

**Q1. The loader: should a fact have as many arguments as its relation declares?** Today neither a top-level fact
nor a nested one is held to its declaration (m6 to m6d, and E-2). The check as an engine change, specified and not
built:

- *what:* every form headed by a **declared** relation's name, wherever a fact, a pattern or a conclusion is
  written (a top-level fact, a nested fact at any depth, a rule's `:match` and `:assert`, an hrule's, a query's
  goal and claims), has exactly as many arguments as the declaration has types; keyword pairs are not arguments.
  A relation nobody declares has no count and is not checked.
- *where:* at load, on every command, one line at the form, in Ein's words: the relation, the count declared, the
  count written.
- *what it would refuse today:* **nothing that loads.** Of 9 779 such forms in the 699 `.ein` files git tracks, 12
  in 6 files have another count than declared, and each of the six is refused already for another reason
  (`python3 build/s1f.6.12/arity_census.py`, 2026-11-11). The Rust suites' inline programs were not scanned; the
  build's census would.
- *what it costs:* register rows first (m6 and m6c are the witnesses: answered by silence), a sweep, and the pages,
  as S1f.6.11's fix had.

*Recommended: yes.* It turns four rows of the table from silence into a refusal that names the place, for every
theory and not this one alone, and `std.num9`'s nine digits are the case it was first met on.

**Q2. A name no `let` states: silence, or one `absent`?** `(add foo X)` cannot be flagged monotonically. The one
way is `(absent (let ?x ?v))` over an operand that is a name, and saying *a name* costs the guard over the
program's names once an operand, some hundreds of comparisons a request. `let` is written and never derived, so
the negation is stratified and *forbid* does not refuse it; but the theory would no longer be *no `absent`, one
stratum*, and `arith_search_free.rs`'s monotone claim would read *one `absent`, over a relation nothing derives*.

*Recommended: silence, named on the page*, unless the user wants the rule of the theory relaxed for this one
premise. A misspelt constant is a likely slip, so this is the row where silence costs a user most.

**Q3. Build b1?** The rules of § 4 marked *build*: a digit's place, a number's end where it is a digit or a
constant, a digit as an operand, a name stated twice, `z` or a digit as a constant's name; `(malformed REQUEST)`
and the rule from it to `(false)`; twenty firings a request.

*Recommended: yes*, with the check as a pass of its own first, and the cheaper mixed placement measured by the
build before it is chosen.

**Q4. May a module reserve its names, so that a program's facts of them are refused?** Declarations are refused
already. Facts are not, and c9 and c10 are what that allows. A reservation would be a language change: a module
says which of its relations a program may state facts of (the requests and `let`), and the loader refuses a
written fact of any other. It is also what would make a *written* fact distinguishable from a derived one, which
is [F24](../../followups/f24_written_provenance.md)'s question from the other side.

*Recommended: not in this phase.* The module's page lists the names (it does, since S1f.6.3) and says that only
the requests and `let` are for a program to state; the question goes to F24 or M1g with c9 and c10 as its
witnesses.

**Q5. The internal names: a common prefix?** The stage file's default was to give the circuit's own relations
names unlikely to clash. T2 found that a clash of declarations is refused with the name in the message, and that
the user-facing risk is a *fact* of an internal name, which Q4 covers. Renaming sixty relations moves every state,
every trace and every golden of the calculator.

*Recommended: no rename.* `done`, `b`, `sum`, `order` and `short` are the likeliest to be wanted by a program, and
a program that wants one imports the theory `:as`.

**Q6. `std.num9`.** Its malformations are a digit's place, checked by the same ten guards on each of nine slots,
and the count of digits, which is Q1's loader check and nothing else. Whether it gets the flag is the build's,
after the rulings above.
