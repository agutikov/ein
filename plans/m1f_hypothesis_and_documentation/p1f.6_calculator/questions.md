# P1f.6 — the questions that wait for the user

**Phase:** [P1f.6](README.md). Written 2026-11-12, on the user's instruction of that day: *"write questions into
one file in phase plans leaving places for my replies"*.

Every question of the phase that waits for you is here, once. The evidence stays on the page each question
came from, linked beside it; the numbers quoted here are copied from those pages and carry their dates there.

**How to answer.** Type under the `**Reply:**` line of a question. A letter is enough where the options are
lettered; *ok* takes the recommendation. An empty reply leaves the question open, and nothing that waits on it
is built. Sections [I](#i-defaults-taken-during-the-phase-everything-else) and
[J](#j-findings-that-need-an-owner) are tables with one reply block each.

**What is already ruled is not asked again:** [`design.md` § The rulings](design.md#the-rulings-2026-11-11)
stands. What was merged or dropped on the way here is listed [at the end](#merged-and-dropped).

## Blocking: answer these first

Each group stops a build. The rest of the file can wait.

| what is not built | questions |
|---|---|
| **Integer division** in `std.int`, the last task of S1f.6.7 | [1](#q1), [2](#q2) |
| **Malformed-number detection**, S1f.6.12's build | [3](#q3), [4](#q4), [5](#q5); [9](#q9) follows them |
| **Expressions**, S1f.6.14 | [10](#q10); [11](#q11) and [12](#q12) bear on it |
| **Binary arithmetic**, S1f.6.16's module | [13](#q13) first, then [14](#q14), [15](#q15), [16](#q16), [17](#q17); [41](#q41) for its conversions |
| **Binary logic**, S1f.6.17's modules | [19](#q19), [20](#q20), [21](#q21), [22](#q22), [23](#q23) |
| **Word problems as more than examples**, S1f.6.20's later tasks | [26](#q26) to [36](#q36) |
| **Inverse requests** in `std.arith`, and what S1f.6.10 proposed as stages | [37](#q37), [38](#q38); [39](#q39) and [40](#q40) before [42](#q42) |

Not blocking: the width ([44](#q44)), the defaults (sections H and I), the findings (section J) and the
leftover commit ([73](#q73)).

---

## A. Integer division

`std.int` is built but for division: an integer is `(int P N)`, worth P − N. Source:
[S1f.6.7 § T2 record](s1f.6.7_negative_numbers.md#t2-record-2026-11-12-the-remainder-convention-three-ways-for-the-user),
[Q-M1f.179](../open_questions.md#q-m1f179--the-remainder-of-an-integer-division).

<a id="q1"></a>

### 1. Which remainder does a division of integers answer?

For `(int-div X Y)`, answered by a quotient Q and a remainder R with X = Q·Y + R, the three usual conventions
differ only when an operand is negative.

- **(a) Euclidean:** R is never negative, 0 ≤ R < |Y|. *R is zero* then means *Y divides X* under every sign.
  Nine rules of its own.
- **(b) Truncated:** Q is rounded toward zero and R takes the dividend's sign. What C, Rust and a pocket
  calculator answer. Four rules of its own, and no arithmetic after the sizes are divided.
- **(c) Floored:** Q is rounded down and R takes the divisor's sign. What Python answers. Nine rules.
- **(d) More than one,** each under a request of its own (`int-div-trunc`, `int-div-floor`, `int-div-euclid`),
  about 27 rules.

| X ÷ Y | (a) Euclidean Q, R | (b) truncated Q, R | (c) floored Q, R |
|---|---|---|---|
| −7 ÷ 2 | −4, 1 | −3, −1 | −4, 1 |
| 7 ÷ −2 | −3, 1 | −3, 1 | −4, −1 |
| −7 ÷ −2 | 4, 1 | 3, −1 | 3, −1 |

All three were written as rules and agree with Python on 924 requests (`python3
build/s1f.6.7/intdiv_proto.py`, 2026-11-12); −7 ÷ 2 is 124 firings under (a) and (c) and 112 under (b).

**Recommended: (a).** The remainder is a natural number, which is what the theory below has and what number
theory reads. If your reader is a programmer first, (b) is the cheaper and more familiar one.

**Reply:**

<a id="q2"></a>

### 2. Under the Euclidean convention, is the remainder a natural number or an integer?

Only if [question 1](#q1) is (a). The remainder is never negative there, so the answer `(int-quot-of X Y Q R)` could
give R as a number of `std.arith` (`z`, `(n z 1)`) or as an integer `(int R z)`. Source: [Q-M1f.179](../open_questions.md#q-m1f179--the-remainder-of-an-integer-division), its last
paragraph.

- **(a) A natural number.** A program's rule on it reads as `std.arith`'s does (*the remainder is `z`*); the
  answer then holds an integer and a natural number side by side.
- **(b) An integer,** `(int R z)`. Every part of the answer is one kind of number; a rule that feeds R to
  `std.arith` unwraps it.

**Recommended: none stated in the source.** Its reason for choosing Euclidean, *the remainder is a natural
number*, leans to (a); the prototype answers an integer.

**Reply:**

---

## B. Malformed numbers, and the names the theory occupies

Today a request the theory cannot read derives nothing: exit 0 and no word. You ruled that the theory detects
what it can, and that a detected one derives `(malformed REQUEST)` and then `(false)`, as a division by zero
does. The investigation ran on prototypes of the shipped `std.arith`. Source:
[`malformed_numbers.md` § 8](malformed_numbers.md#8-the-questions-for-the-user).

<a id="q3"></a>

### 3. Should the loader refuse a fact whose count of arguments is not the one its relation declares?

Today `(add (n z 5))`, one operand where `add` declares two, loads and is answered by silence; so does a
`num9` number of eight digits. No rule can flag a wrong count in general, so this is the loader's or nobody's.
It would be an engine change for every program, not the calculator alone: every form headed by a declared
relation, at any depth, in a fact, a rule or a query, refused at load with one line naming the relation, the
count declared and the count written. Source: malformed_numbers.md Q1; E-2 of [`findings.md`](findings.md).

- **(a) Yes.** Register rows first, then the check, a sweep and the pages.
- **(b) No.** A wrong count stays silent, and the module's page says so.

Of 9 779 such forms in the 699 tracked `.ein` files, 12 in 6 files have another count than declared, and each
of the six is refused already for another reason (`python3 build/s1f.6.12/arity_census.py`, 2026-11-11). The
Rust suites' inline programs were not scanned.

**Recommended: (a).** It turns four rows of the page's table from silence into a refusal that names the place.

**Reply:**

<a id="q4"></a>

### 4. Build the recommended detection rules?

The set the page calls *b1*: a digit's place holding anything but a digit; a number that ends in a digit or in
a named constant where `z` goes, `(n 1 2)`; a digit where a number goes, `(add 7 X)`; a name stated twice with
two numbers; `z` or a digit used as a constant's name. The last two give **two answers to one request** today.
All are monotone (ten `neq` guards or a table lookup, no `absent`, no `not`). Twenty-two rules and three
relations more in a module of 89. Source: malformed_numbers.md Q3.

- **(a) Yes,** as a checking pass of its own at the operand, the cheaper placement measured by the build
  before it is chosen.
- **(b) No.** Silence stays, named on the page.

| 4 000 requests, nine-digit operands | shipped: firings a request, ms | with b1 |
|---|---|---|
| sum | 27.0, 138.9 | 47.0, 260.8 |
| comparison | 16.0, 104.4 | 36.0, 222.8 |
| product | 486.2, 3 828.7 | 506.2, 3 984.7 |

(`python3 build/s1f.6.12/cost.py`, 2026-11-11.) Twenty firings a request whatever the operation; a batch still
costs in proportion to its count.

**Recommended: (a).**

**Reply:**

<a id="q5"></a>

### 5. A name that no `let` states: silence, or one `absent` in the theory?

`(add foo X)` where nothing says `(let foo …)`. Saying *no `let` names it* is a negation, so it cannot be
flagged monotonically. A misspelt constant is a likely slip. Source: malformed_numbers.md Q2.

- **(a) Silence,** named on the module's page. The theory stays *no `absent`, one stratum*.
- **(b) One `absent` over `let`.** `let` is written and never derived, so the negation is stratified and is
  not refused; the theory's claim becomes *one `absent`, over a relation nothing derives*, and the check costs
  some hundreds of comparisons a request, since saying *a name* ranges over the program's names.

**Recommended: (a),** unless you want the rule of the theory relaxed for this one premise.

**Reply:**

<a id="q6"></a>

### 6. Should a request that no rule completes say so?

The general form of the question above. With everything recommended here built, three things still stay
silent: a name no `let` states, a number that ends in a name that is neither a digit nor a constant, and a
fact of another relation where a number goes. A natural number where `std.int` expects an integer is silent
too, and the shipped division run over binary tables loaded and answered nothing. *This request has no answer*
is an absence, so no monotone rule can say it. Sources: malformed_numbers.md § 7, objection 2;
[binary_arithmetic.md Q7](binary_arithmetic.md#questions-for-the-user).

- **(a) No general mechanism.** Each module's page says *a flag means malformed; no flag does not mean
  well-formed*, and lists what stays silent.
- **(b) A general report of unanswered requests,** which needs a negation in the theory or a check outside it;
  neither is designed.

**Recommended: (a)** is what `malformed_numbers.md` proposes; `binary_arithmetic.md` asks without recommending.

**Reply:**

<a id="q7"></a>

### 7. May a module reserve its names, so that a program's written facts of them are refused?

A second *declaration* of any of the theory's names is refused at load already. A written *fact* is not: a
program that writes `(done (sum (n z 2) (n z 2)) (n z 5))` gets `2 + 2 = 5`, an answer nobody computed. A
reservation would be a language change: a module says which of its relations a program may state facts of (the
requests and `let`), and the loader refuses the rest. Source: malformed_numbers.md Q4.

- **(a) Not in this phase.** The module's page lists the names and says only requests and `let` are a
  program's to state; the question goes to [F24](../../followups/f24_written_provenance.md) (what a written
  fact may claim) or M1g, with the two witnesses.
- **(b) Yes, now,** as an engine stage of this phase.

**Recommended: (a).**

**Reply:**

<a id="q8"></a>

### 8. Rename the theory's internal relations with a common prefix?

The circuit's own relations have plain names, `done`, `b`, `sum`, `order`, `short` among them, which a program
importing the theory flat may not declare. Source: malformed_numbers.md Q5; [Q-M1f.151](../open_questions.md#q-m1f151--the-names-of-the-circuits-own-relations).

- **(a) No rename.** A clash of declarations is refused with the name in the message, and a program that wants
  one of the names imports the theory `:as`.
- **(b) A prefix on all of them.** Sixty relations renamed; every state, trace and golden of the calculator
  moves.

**Recommended: (a).**

**Reply:**

<a id="q9"></a>

### 9. Does `std.num9` get the same flag?

`std.num9` is the fixed-width module, nine digits in one fact. Its malformations are a digit's place, checked
by the same ten guards on each of nine slots, and the count of digits, which is [question 3](#q3)'s loader check and
nothing else. Source: malformed_numbers.md Q6.

- **(a) Yes,** the digit-place flag, written by the module's generator, if [question 4](#q4) is yes.
- **(b) No:** `std.num9` stays silent on a non-digit.

**Recommended: none in the source,** which leaves it to the build after the rulings above.

**Reply:**

---

## C. Expressions, and what an operand may be

<a id="q10"></a>

### 10. Is there an expression layer: `(calc E)` and `(let NAME E)`?

Today a program composes operations by rules of its own: one rule asks the calculator, another reads the
answer and asks the next thing. An expression layer would let one fact hold a tree, `(calc (add N1 (div N2
N3)))`, evaluated on demand, and `(let NAME E)` name its value. You called it questionable on 2026-11-11
(*"Ein is not Lisp, is it?"*), and it was to be asked again once the word problems showed the need. Source:
[S1f.6.14](s1f.6.14_expressions.md#what-the-user-is-asked-when-the-need-shows);
[word_problems.md Q12](word_problems.md#11-the-questions-for-the-user).

- **(a) No expression layer.** Every composition is a rule of the program's. The smallest theory.
- **(b) `(let NAME E)` only:** named quantities, no anonymous `calc`; say whether E is one level deep or any
  depth.
- **(c) Both,** as round 2 of the design built them, as a module of its own over `std.arith`.

What the word problems showed: every quantity took two rules per word, one that asks and one that reads the
answer, the second repeating the first's premises; the pencils problem is three operations chained through
three such pairs; **no rule needed two operations at once**.

**Recommended: none stated;** S1f.6.14 leaves it to you, and the word-problem evidence does not show a need.
Already settled if it is ruled in: `sub` inside an expression is strict, and division is two heads, `div` and
`mod`.

**Reply:**

<a id="q11"></a>

### 11. What does `(let NAME NUMBER)` let a request do?

You ruled named constants in. The reading built was the coordinator's default. Source: [Q-M1f.149](../open_questions.md#q-m1f149--what-let-name-number-does-as-built);
[`design.md` § What is open](design.md#what-is-open-after-the-rulings), item 2.

- **(a) A request may name an operand** (built): `(add price qty)` is answered by `(sum-of price qty S)`, the
  names repeated. The same sum written with numbers is another request with its own answer. Every request
  costs four firings for the lookup, numbers too.
- **(b) A name is only a label,** and requests take numbers alone.

**Recommended: (a),** as built.

**Reply:**

<a id="q12"></a>

### 12. Is a smaller kind of number accepted where a larger is expected?

Today it is not: `(int-add (n z 5) X)`, a natural number where an integer goes, is answered by silence, and so
is an integer where a rational goes; 5 as a rational is written `(rat (int (n z 5) z) (n z 1))`. Source:
[Q-M1f.175](../open_questions.md#q-m1f175--what-stdint-reads-as-an-operand), [Q-M1f.181](../open_questions.md#q-m1f181--what-stdrat-reads-as-an-operand).

- **(a) No promotion** (built). Each module reads its own shape.
- **(b) Promote:** a natural number is read as `(int N z)`, an integer as a rational over 1. Four rules more
  and two firings an operand for the integers; it can be added later with no answer of today's moving.

**Recommended: (a) for now;** the sources note that an expression layer ([question 10](#q10)) would meet the question
again.

**Reply:**

---

## D. Binary arithmetic

Nothing is in the stdlib; three ways were built on a prototype, and whatever of it is banked under
`examples/` does not settle these questions. What the measurement found: addition, subtraction
and comparison need **no rule changed**, only smaller tables; multiplication and division get simpler rules;
every number is 3.3 times longer. Source:
[`binary_arithmetic.md` § Questions](binary_arithmetic.md#questions-for-the-user).

<a id="q13"></a>

### 13. Which of the three ways to have binary numbers?

- **(a) The same shape and names,** `(n (n z 1) 0)` and `(add X Y)`, over binary tables. A program picks its
  base by the tables it imports, and a number does not say its base; two bases in one program need a prefixed
  import.
- **(b) A theory of its own,** `(b2 (b2 z 1) 0)` and `(add2 X Y)`. Stands beside decimal in one program; a
  number says its base. Its cost is a second copy of 51 rules that differ from `std.arith`'s only in names.
- **(c) One generic theory** whose every request carries its base, `(add 2 X Y)`. Changes every request you
  have ruled on, to `(add 10 X Y)`, and loads both tables into every program.

| | decimal, shipped | (a) | (b) | (c) |
|---|---:|---:|---:|---:|
| rules | 88 | 69 | 67 | 72 |
| table rows | 1 664 | 33 | 28 | 1 652 |
| 123 × 456, firings | 72 | 415 | 120 | 415 at base 2 |
| parse, load and compile | 26.8 ms | 8.0 ms | 5.8 ms | 26 to 41 ms |

(Runs of 2026-11-11, `build/s1f.6.16/`.)

**Recommended: (b).** The smallest and cheapest, and the only one that conversion between the bases and the
logic gates can import plainly. (c)'s generality is the *any base* you set aside (*"skip different bases"*).

**Reply:**

<a id="q14"></a>

### 14. Under (b), what are the binary theory's names?

The prototype's are placeholders: the shape `(b2 PREFIX BIT)`, the requests `add2`, `sub2`, `cmp2`, `mul2`,
`div2`, the results `sum-of2` and so on. Source: binary_arithmetic.md Q2.

- **(a) The prototype's,** a `2` suffix, as `std.num9` took a `9`.
- **(b) A word prefix,** `bin-add` answered by `bin-sum-of`, as `std.int` took `int-` and `std.rat` `rat-`.
- **(c) Something else:** write it in the reply.

**Recommended: none stated.**

**Reply:**

<a id="q15"></a>

### 15. Is zero the same `z` in both bases?

In the prototype it is: `z` is zero in decimal and in binary, the one number whose numeral does not say its
base. Source: binary_arithmetic.md Q3.

- **(a) One `z`.** Nothing to convert for zero; a bare `z` is of no base.
- **(b) A zero of its own for binary.** Every numeral says its base; one more name.

**Recommended: none stated;** (a) is what was built and measured.

**Reply:**

<a id="q16"></a>

### 16. How are the binary rules that equal `std.arith`'s kept equal?

Under (b), 51 of the 67 rules are `std.arith`'s up to their names. Source: binary_arithmetic.md Q4.

- **(a) Generate the binary copy from `std.arith`,** which stays hand-written, and hold the copy by a test, as
  the prototype's `build.py` does.
- **(b) A hand-kept second file,** held equal to `std.arith` by a test.
- **(c) Generate `std.arith` in part too,** so the shared rules are written once for both.

**Recommended: (a),** the page's recommendation for (b): the two cannot drift, and the decimal module stays a
file a reader can follow.

**Reply:**

<a id="q17"></a>

### 17. Where do the decimal ⇄ binary conversions live?

`(to-bin X)` and `(to-dec B)`, eleven rules on the prototype, each a chain of the two theories' operations;
20 of 20 conversions agree with Python. They call the theories' internal tagged calls, which is
[question 41](#q41). Source: binary_arithmetic.md Q5.

- **(a) In the binary module.**
- **(b) In a module of their own,** so a program that only computes in binary does not load decimal's 1 664
  rows.

**Recommended: none stated.** 255 to binary is 826 firings and back is 76.

**Reply:**

<a id="q18"></a>

### 18. Keep decimal's division by bisection, or take the simpler one?

A by-product: the generic theory's division, which subtracts repeatedly and computes no multiples, is 20 rules
where the shipped one is 36. Source: binary_arithmetic.md Q6.

- **(a) Leave the shipped division.**
- **(b) Replace it** with the 20-rule one.

| firings | shipped, 36 rules | simpler, 20 rules |
|---|---:|---:|
| 127645 ÷ 25 | 220 | 173 |
| a quotient, batch of nine-digit dividends | 443.7 | 436.8 |
| a quotient whose digits are all nines | 272.7 | 381.6 |

**Recommended: none stated;** the page records it and says it is not that stage's to act on.

**Reply:**

---

## E. Binary logic

Nothing is in the stdlib, and whatever of the prototype is banked under `examples/` does not settle these
questions. On a prototype: the sixteen two-input boolean functions as facts, thirty properties
derived from their rows, a circuit simulated by saturation, and six applications. Source:
[`binary_logic.md` § Questions](binary_logic.md#questions-for-the-user).

<a id="q19"></a>

### 19. One table for the sixteen functions, or a relation each?

- **(a) One table,** `(gate2 And 1 1 1)`: a function is a value, so one rule speaks of *any* function, and a
  search can guess which function a gate is.
- **(b) A relation a function,** `(And 1 1 1)`: a shorter row; a rule about any function then needs an
  activator fact per rule and function, 16 × 31 for the property rules.
- **(c) Both,** the second derived from the first by sixteen rules.

The names stand as proposed (`Zero And Andn Fst Randn Snd Xor Or Nor Xnor Nsnd Orn Nfst Rorn Nand One`, the
table `gate2`, a second name such as `f0110` held in `(column-name Xor f0110)`): you ruled on 2026-11-11 that
any spelling will do. Say here if one should change; `Randn` and `Rorn` are the least pronounceable.

**Recommended: (a).** Nothing on the prototype wanted the second layout.

**Reply:**

<a id="q20"></a>

### 20. Do the property facts ship as rules or as generated facts?

Thirty relations such as `(commutative Xor)`, `(dual-of And Or)`, `(identity-of Or 0)` are all derived from
the 64 rows by monotone rules, in 405 firings. Source: binary_logic.md Q3.

- **(a) Generated facts,** with the rules kept as the test that derives every one. Nothing at run time.
- **(b) Rules.** They can never disagree with the rows, and every program that loads the module pays the 405
  firings.

**Recommended: (a).**

**Reply:**

<a id="q21"></a>

### 21. Are the one-sided identity and absorbing facts wanted?

`identity-of` and `absorbing-of` as two-sided facts hold of four functions each; the left and right forms hold
of eight each. `Andn` (*a and not b*), for one, has a right identity, 0, and no left one. Source:
binary_logic.md Q4.

- **(a) Two-sided only,** as the stage file has them.
- **(b) The left and right forms too.**

**Recommended: none stated.**

**Reply:**

<a id="q22"></a>

### 22. Confirm three corrections to the stage file's hand counts

Re-counted from the rows, three counts of the stage file were wrong and its table was corrected. Source:
binary_logic.md Q5 and § 2.

| the stage file said | the rows say |
|---|---|
| six swap pairs | **four** pairs (`Andn`/`Randn`, `Fst`/`Snd`, `Nfst`/`Nsnd`, `Orn`/`Rorn`), and eight functions their own swap |
| duals `Andn`/`Rorn`, `Randn`/`Orn` | **`Andn`/`Orn`, `Randn`/`Rorn`**; the pairs written were the complements |
| ten functions up to the order of the inputs | **twelve**: four pairs and eight alone |

- **(a) Confirmed.**
- **(b) One is wrong:** say which.

**Recommended: (a);** each agrees with a Python count and with the facts Ein derives.

**Reply:**

<a id="q23"></a>

### 23. What ships from the logic stage?

Source: binary_logic.md Q8.

- **(a) All four small modules and the six applications:** the table with its properties; the circuit theory
  (`gate`, `val`, one propagation rule); the inputs theory (an input owes exactly one value) with `require`;
  the six applications under `examples/logic/`; and the full adder built from gates, held equal to the binary
  adder's table, as a test.
- **(b) The table and the properties only,** the rest left on the page until M1g.
- **(c) Another cut:** write it in the reply.

**Recommended: (a).**

**Reply:**

<a id="q24"></a>

### 24. An unsatisfiable search reads as *owing*, not as *dead*: is that the wanted reading?

Asked three times: for a circuit, for an equation, and by M1g's first probes. When the unknowns are hypotheses
that each owe a value, every complete assignment is dead but the partial ones are alive and merely cannot be
completed. So R is empty as *every alive node owes*, `ein solve --single` answers `Open` with *decided: yes*,
and the read-out prints the owing nodes and **no row for N**, the minimal dead sets with their cores, which is
the explanation; those are in `--dump-answer`'s `nogoods.json` and `--trace` only. `Open` reads as *not yet*
where the run has shown *never*. Sources: [binary_logic.md Q6](binary_logic.md#3-satisfiability);
[beyond_arithmetic.md Q6](beyond_arithmetic.md#6b-a-proof-of-no-solution-three-forms-one-of-them-a-proof);
E-23 of [`findings.md`](findings.md); [M1g's probe 1b](../../m1g_logic/README.md#first-probes-2026-11-11).

- **(a) Keep the words,** and add the N row to the read-out of an R that is empty by owing, so the reasons are
  printed.
- **(b) A program can say that a debt no extension can pay is a dead state,** so unsatisfiable reads as *every
  node is dead*. A change to what the verdict words mean.
- **(c) Leave everything,** and say on the page how to read it.

The witness: `x × 7 = 862` asked by search is `Open`, its argument 28 dead sets in the dump; asked by
saturation, where the table makes each digit unique, it is a dead root whose core is a three-row proof.

**Recommended: none stated** in any of the three sources; they name the read-out row as
[`read_out.md`](../../../docs/kernel/inference/read_out.md)'s question and (b) as M1g's.

**Reply:**

<a id="q25"></a>

### 25. Is a cheaper search for *exactly one value per object* a requirement?

With each input a hypothesis that takes 0 or 1, describing R costs 3ⁿ saturations for n inputs, and finding
one state 2ⁿ. The same truth table from written input vectors is one saturation. Source: binary_logic.md Q7.

- **(a) The search form is a demonstration.** Truth tables are computed from written vectors; no engine work.
- **(b) It is a requirement for [M1g](../../m1g_logic/README.md)'s valuations,** and a cheaper route for this
  shape of hypothesis is planned there.

| inputs | by search: describe R | by search: one state | written vectors: one saturation |
|---|---|---|---|
| 12 | 531 453 saturations, 11.7 s | 4 096 saturations | 49 152 firings, 64 ms |
| 16 | no answer in 150 s, 8 GiB | 65 536 saturations, 2.9 s | 1 048 576 firings, 2.3 s |

(`python3 scale.py`, `scale2.py` under `build/s1f.6.17/`, 2026-11-11, budget raised to 2³⁰.)

**Recommended: none stated.**

**Reply:**

---

## F. Word problems

A prototype theory of what the words of a school problem mean (*has*, *gave*, *together*, *need*), over the
calculator, with 29 programs. **The problems were banked on 2026-11-12 under `examples/word_problems/`, in the
page's recommended variants, on your instruction; the questions below stay open,** and an answer other than
the recommendation changes those files. [question 10](#q10), on expressions, took the page's twelfth question. Source:
[`word_problems.md` § 11](word_problems.md#11-the-questions-for-the-user).

<a id="q26"></a>

### 26. How is a problem's order of events written?

*Bob has 5 apples. He gave 2 to Alice. How many does Bob have?* Source: word_problems.md Q1 and § 1.

- **(a) Named moments:** `(has Bob apples (n z 5) t0)`, `(gave Bob Alice apples (n z 2) t0 t1)`, `(asked
  (amount Bob apples t1))`. One flat fact a sentence; the order costs two names on each event.
- **(b) Nested events:** a moment is the history that led to it, `(after (gave …) start)`. No names; every
  question and answer repeats the whole history.
- **(c) A chain of sentences:** `(says s2 (gave Bob Alice apples (n z 2)))`, in the text's order. Closest to
  the text; a state stated late needs the tense read.

**Recommended: (a);** (c) is the form to translate *from*, which is what M2 would produce first.

**Reply:**

<a id="q27"></a>

### 27. Does the theory work backwards from the question?

On the prototype nothing is computed that no question needs: an `asked` fact derives what is `wanted`, and
that what it `needs`. It makes the plan of a two-step problem readable and an unused fact silent. It also
means a contradiction in the facts is found only if a question reaches it. Source: word_problems.md Q2.

- **(a) Keep it.**
- **(b) Compute everything forwards** from the facts: every contradiction is found, and the trace holds what
  no question asked.

**Recommended: (a).**

**Reply:**

<a id="q28"></a>

### 28. How is a missing quantity detected?

*Bob has 5, gave 2 to Alice; how many together?* Nothing says what Alice had. Source: word_problems.md Q3 and
§ 3.6.

- **(a) An obligation:** the state owes Alice's amount. The gap is named, R is empty, and nothing else is
  answered, not even what Bob has.
- **(b) A derived fact by `absent`,** in a module of its own: the read-out says what is known, that
  *together* has no answer, and which quantity is not stated. Search-free; two strata.
- **(c) A hypothesis:** the unknown is guessed from a list of numbers the problem file states, and each guess
  gives an answer. Works only over such a list.

**Recommended: (b),** the only form that says all three things in the read-out.

**Reply:**

<a id="q29"></a>

### 29. How is an assumption written and reported?

*Assume that Alice had none.* Source: word_problems.md Q4 and § 3.5.

- **(a) A marked fact,** `assumed`, and a `rests-on` fact beside the answer: *the answer rests on the
  assumption that Alice's amount is `z`*. Search-free; works today.
- **(b) A hypothesis** the search may commit to: the state's own `from {…}` line names the supposition. The
  better read-out; it waits on [question 30](#q30).

**Recommended: (a) today.**

**Reply:**

<a id="q30"></a>

### 30. A named gap together with a supposition: which fix to wait for?

Combining [question 28](#q28) with a hypothesis has two forms, each with a flaw. With (b)'s `absent`, the program is no
longer monotone and one state keeps a stale *not stated* fact. With (a)'s obligation, the program is monotone
but the read-out prints only *root owes 1 instance*, not which or why (E-19 of [`findings.md`](findings.md)).
Source: word_problems.md Q5.

- **(a) A read-out row for what root owes when R is not empty.** A small change to the read-out.
- **(b) Wait for [F21](../../followups/f21_non_monotone_programs_decided.md)'s ruling** on non-monotone
  programs.

**Recommended: (a),** which is smaller and is wanted by the readings of [question 31](#q31) too.

**Reply:**

<a id="q31"></a>

### 31. How is a word with two readings handled?

*13 apples, a box holds 3: how many boxes do they need?* 4 if only full boxes count, 5 if every apple is
packed. The readings are hypotheses in three built forms. Source: word_problems.md Q6 and § 4.3.

- **(a) Never owed** (`readings_plain`): R has three states, no reading (a partial answer), 4, and 5; the gap
  is a derived fact, a query row. Monotone.
- **(b) Always owed** (`readings_always`): R holds only the two readings, 4 and 5; the gap is named in
  `--events` only. Monotone. With 12 apples it still makes two states for one answer.
- **(c) Owed where it matters** (`readings`): as (b) on 13 apples and search-free on 12, but flagged
  non-monotone, so it needs the full cube.

And one more choice: where nothing is left over (12 apples), does the rule answer 4 with no reading chosen (as
built), or is a reading always chosen? Say which with the letter.

**Recommended: (a);** or (b) if R should hold only the readings.

**Reply:**

<a id="q32"></a>

### 32. Is *Alice has 3 pears* also *Alice has no apples*?

*Bob has 5 apples, Alice has 3 pears. How many apples together?* Source: word_problems.md Q7.

- **(a) No: it is not stated,** so it is a gap, the same as *what Alice had before*; the assumption *the
  problem lists everything each has* is available as a fact.
- **(b) Yes:** what is not listed is zero. Needs `absent` over `has`.

**Recommended: (a).**

**Reply:**

<a id="q33"></a>

### 33. How does the theory state what its model leaves out?

*Things are counted by number alone; any thing fits any box; nobody gives away what they do not have.* Source:
word_problems.md Q8 and § 5.

- **(a) A fact with a `:source`, taken as a premise** by the rules whose conclusion depends on it. The trace
  of an answer cites the model exactly where it is used, and a contradiction's core lists it.
- **(b) Text only,** a relation's `:why` or a query's `:goal-text`: stated once, cited by no step.

**Recommended: (a),** one such fact per assumption if the theory ships.

**Reply:**

<a id="q34"></a>

### 34. Where does the word theory live, and what is it called?

Source: word_problems.md Q9 and § 8.

- **(a) `examples/word_problems/`,** the modules beside their problems, named `words` (what the words mean),
  `gaps` (a missing quantity) and `readings` (a word with two readings). Not in the stdlib: imported flat it
  occupies ordinary words, `has`, `gave`, `holds`, `asked`, `value`, `together`.
- **(b) The stdlib,** with a program per rule under `tests/stdlib/` and the mutation census.

**Recommended: (a);** it can move if it proves itself over P1f.8's algebra problems. It is where the files
were banked.

**Reply:**

<a id="q35"></a>

### 35. Which further readings of a word should the theory hold?

Not held today: *gave all* and *gave half*; *together* over everyone in the problem; *how many times more*;
sizes and part-filled boxes; fractions of a share. Source: word_problems.md Q10 and § 8's table.

- **(a) None before the rulings above.**
- **(b) The two a school problem meets first:** *together* over everyone, and *gave all*.
- **(c) Others:** name them in the reply.

**Recommended: (a).**

**Reply:**

<a id="q36"></a>

### 36. The trace as the pupil's page: is folding an imported theory's steps part of this stage?

The stage's acceptance asks that *each derived answer's trace is the reasoning*. Today the two-step problem's
trace is 139 steps, 10 of them the word theory's and the rest the calculator's; `-l`, the goal-relevant slice,
keeps 137. Nothing folds a call into one line, *12 ÷ 3 = 4, remainder 0 (116 steps of `std.arith`)*. A script
on the prototype filters the trace. Source: word_problems.md Q11; E-17 of [`findings.md`](findings.md).

- **(a) Part of S1f.6.20:** an engine change to `--trace` inside this stage.
- **(b) A stage of its own,** in this phase or a later one: say where.
- **(c) The filter script is enough;** the acceptance is reworded.

The same reasoning also reads *(n z 5) + (n z 7) = (n (n z 1) 2)*, which is [question 55](#q55).

**Recommended: none stated.**

**Reply:**

---

## G. Inverse requests, and what the investigation proposed

S1f.6.10 asked what the tables do besides arithmetic. Source:
[`beyond_arithmetic.md` § Questions](beyond_arithmetic.md#questions-for-the-user).

<a id="q37"></a>

### 37. Should `std.arith` answer inverse requests by saturation?

A table is a relation, so a rule can read it in any direction: *which x has x + 4567 = 127645* is solved with
no search, a firing a column, and *no natural x* is a derived fact. Proposed: `(solve-add Y S)` and
`(solve-mul M P)` for a one-digit M. A longer M is division, which the theory has, and the stage would say so
rather than build it twice. Source: beyond_arithmetic.md Q1.

- **(a) Yes, as a stage.**
- **(b) No;** the two banked examples stay as examples.

Eleven rules were enough on the prototype; three such requests took 33 firings. It is the only form in which
*no solution* is a proof: `x × 7 = 862` is a dead root whose core is three rows of the multiplication table.

**Recommended: (a).**

**Reply:**

<a id="q38"></a>

### 38. Do gcd and lcm go into `std.arith`?

`std.rat` computes a greatest common divisor inside itself and offers none to a program. Six rules offer gcd
and lcm. Source: beyond_arithmetic.md Q5.

- **(a) Into `std.arith`,** as `(gcd X Y)` answered by `(gcd-of X Y G)`, when [question 37](#q37)'s stage runs.
- **(b) Left as the banked example.**

**Recommended: (a).**

**Reply:**

<a id="q39"></a>

### 39. Should `ein test` decide a claim by the cheapest route that decides it?

`ein test` always describes all of R. On a search program that can be far dearer than the question. Source:
beyond_arithmetic.md Q2; E-30 of [`findings.md`](findings.md).

- **(a) Yes, as an engine stage:** a claim of one state is `--single`'s question, a claim of k is `-n k+1`'s.
  Your *minimum work per question*, applied to the runner.
- **(b) No.** A search program past some twenty unknown-value pairs carries no claim.

SEND + MORE = MONEY with its answer claimed: `ein test`, *NOT CHECKED*, 7.5 s, exit 1; `ein solve --single` on
the same file, *Solution — R is one state*, 0.5 s (2026-11-12).

**Recommended: (a),** before any theory of puzzles ships an example it cannot test.

**Reply:**

<a id="q40"></a>

### 40. Should the search choose which owed thing to branch on by the state?

`--single` and `-n` branch on what the state owes in the order the program declares it. Nothing chooses by the
state, and nothing says the order mattered. Source: beyond_arithmetic.md Q3; E-29 of
[`findings.md`](findings.md).

- **(a) Yes, and it is [P1f.10](../p1f.10_hypothesis_structure/README.md)'s,** with this witness: fewest alive
  alternatives first.
- **(b) Yes, in this phase.**
- **(c) No;** the page of any search theory says *declare what is checked first, first*.

SEND + MORE = MONEY: letters declared by column, solved and shown unique in 17 747 saturations, 0.50 s;
declared as the puzzle is written, no answer in 600 s (2026-11-12).

**Recommended: (a).**

**Reply:**

<a id="q41"></a>

### 41. Are `std.arith`'s tagged calls a public interface?

The public protocol is a request and a result that repeats its operands: `(add X Y)`, `(sum-of X Y S)`. A rule
that wants an answer for its own purpose must then find it by its operands, which joins on a number, and the
engine answers such a join by a scan. Inside, the theory calls itself with a tag that names the caller,
`(add-walk TAG X Y 0 z)`, `(mul-call TAG X Y)`, `(div-call TAG X Y)`, `(cmp-walk TAG X Y eq)`, answered by
`(done TAG A)`, `(div-done TAG Q R)`, `(order-is TAG O)`. `std.int` and `std.rat` use them; so do eight banked
examples and the prototype conversions; the module's header calls them *the circuit's own*, not a promise.
Sources: beyond_arithmetic.md Q4; [binary_arithmetic.md Q5](binary_arithmetic.md#questions-for-the-user);
[Q-M1f.189](../open_questions.md#q-m1f189--a-users-program-calling-stdariths-tagged-calls).

- **(a) Yes:** the module's page names them as what a theory built on it calls, and a test holds their shapes.
- **(b) No:** outside programs use the public protocol and pay for the join.

The laws of arithmetic checked over the digits: 16.8 s through the public protocol, 0.42 s through the tagged
calls (`build/s1f.6.10/gen4.py` against `gen4t.py`, 2026-11-12).

**Recommended: (a);** they are stable already, since two shipped modules call them.

**Reply:**

<a id="q42"></a>

### 42. A small cryptarithm theory in the stdlib?

The column rules with the tables read in every direction, written once, and SEND + MORE = MONEY as its
example. With such propagation the puzzle is 639 search nodes where the plain search takes 17 747. Source:
beyond_arithmetic.md Q7.

- **(a) Yes, small, after [question 39](#q39) and [question 40](#q40):** today its example could not carry a claim and its
  cost hangs on declaration order.
- **(b) No.**

**Recommended: (a);** say the name if not `std.crypt`.

**Reply:**

<a id="q43"></a>

### 43. Where do the sixteen banked programs of the investigation live?

They are under `examples/beyond_arith/`, taken by default. They could not go under `examples/arith/`: a test
holds every file there to *no search*, and four of these search. They are sized so a bare `ein solve` finishes
(SEND + MORE itself is not among them) and are kept by hand, with no generator. Source: beyond_arithmetic.md
Q8; [Q-M1f.186](../open_questions.md#q-m1f186--where-the-probes-of-s1f610-are-banked), [Q-M1f.187](../open_questions.md#q-m1f187--which-probes-of-s1f610-are-banked-and-at-what-size), [Q-M1f.188](../open_questions.md#q-m1f188--the-banked-probes-are-kept-by-hand).

- **(a) As they are.**
- **(b) One directory,** `examples/arith/`, with the test told which files search.
- **(c) Another name** for the directory: write it.

**Recommended: (a).**

**Reply:**

---

## H. The width, and defaults to confirm one by one

These were decided by default while the stages ran, and their records mark them as yours to confirm, or they
shape what a program writes. Each stands if the reply is empty.

<a id="q44"></a>

### 44. A number too long for its state file to load again: say it, flag it, or change the engine?

The parser refuses nesting past 1 000 parentheses, and a number nests one level a digit. A computation's
internal steps nest a little deeper than its request, so for a sum of two 999-digit numbers the answer is
right and printed, but the state file `--dump-answer` writes is refused when loaded again (998 digits for a
product, 997 for a sum of integers, 994 for a rational's numerator). The engine itself has no depth limit.
Source: [Q-M1f.185](../open_questions.md#q-m1f185--a-state-file-nested-past-the-parsers-limit); [S1f.6.9 § T1 record](s1f.6.9_width.md#t1-record-2026-11-12-the-limit-re-taken).

- **(a) Say it on the page and leave it.** Done already, since it is true whatever is ruled.
- **(b) The theory flags a number past a bound.** Probed: it works and doubles every sum, 50 firings where 28
  at nine digits.
- **(c) An engine change,** so the parser can read whatever a rule can derive. Not built.
  [M1g's S1g.6.2](../../m1g_logic/p1g.6_engine_requirements/s1g.6.2_nested_facts_at_scale.md) lists the
  nesting limit among what it takes, following this ruling.

A state past the limit is at least 40 MB of text; a 998-digit sum is 6.7 s and 86 MB under `ein solve`
(2026-11-12).

**Recommended: (a), until a program needs more.**

**Reply:**

<a id="q45"></a>

### 45. Confirm the three changes to the per-commit drawing test

The test that puts every view of every corpus file through Graphviz could not finish once the digit tables
existed: a view of them is some 7 900 edges, and `dot` had not drawn one after five minutes. Three defaults
followed. Source: [Q-M1f.147](../open_questions.md#q-m1f147--the-dot-sweep-and-a-view-no-layout-finishes), [Q-M1f.153](../open_questions.md#q-m1f153--the-dot-sweeps-bound-under-the-smallest-table), [Q-M1f.161](../open_questions.md#q-m1f161--the-dot-sweep-several-files-at-a-time); E-10.

1. A view of more than 1 000 edges is checked under another layout (`fdp`, stopped after one iteration): the
   same grammar and attribute checks, without `dot`'s layout. No view the corpus had before is past the bound
   (the largest has 361 edges).
2. The bound is 1 000 and not 1 500, so a view holding even the smallest table is past it.
3. The test shares the corpus files among the cores; what it holds is unchanged.

- **(a) Confirmed.**
- **(b) Change:** say what.

**Recommended: (a).** What a view *should* draw of a table is row E-10 of section J.

**Reply:**

<a id="q46"></a>

### 46. Confirm the new gate step

The claim *a batch of requests costs in proportion to its count* is held by the matcher's own counter, which
every ordinary build compiles out. So the gate builds `ein-infer` a second time with the counters for one
test: `cargo test -p ein-infer --features counters --test arith_batch`, in `run_tests.sh` and the per-commit
workflow. Source: [Q-M1f.150](../open_questions.md#q-m1f150--the-counter-and-how-the-measured-test-is-in-the-gate).

- **(a) Confirmed.**
- **(b) No extra gate step:** the static check of the rules' text alone, and the measured test run by hand.

**Recommended: (a).**

**Reply:**

<a id="q47"></a>

### 47. Leave the comparison table's column order?

The matcher looks a table row up by its first bound argument. The comparison table `cl` has *the order so far*
first, which a hundred of its 300 rows share, so a nine-digit comparison offers the matcher 901 candidates
where a nine-digit sum offers 181. Source: [Q-M1f.158](../open_questions.md#q-m1f158--cls-column-order-and-the-index); E-12.

- **(a) Leave the table.** A constant factor; 4 000 comparisons saturate in 74 ms.
- **(b) Put a digit first:** 30 candidates a column. The generator, the table's test and three pages move.
- **(c) The engine picks the most selective argument:** an engine change, row E-12 of section J.

**Recommended: (a).**

**Reply:**

<a id="q48"></a>

### 48. Confirm how far the division-by-zero flag reaches

You ruled the flag and `(false)` from it. Two details were taken by default. Source: [Q-M1f.163](../open_questions.md#q-m1f163--division-called-with-a-tag-and-whose-division-by-zero-is-flagged), [Q-M1f.182](../open_questions.md#q-m1f182--a-zero-denominator-is-stdariths-division-by-zero);
[S1f.6.6 § T2 record](s1f.6.6_division.md#t2-record-2026-11-11-division-by-zero-on-every-surface).

1. **Only the public request is flagged,** `(division-by-zero (div X Y))`, the request as written. A theory
   that calls division internally with a zero divisor gets nothing and writes a rule of its own.
2. **`std.rat` uses the same flag for any zero denominator:** a request any operand of which is written over
   zero, whatever the operation, derives it, and the program is dead, though a product or a comparison with
   3/0 has an arithmetic reading. A `(let NAME (rat Z z))` nobody asks about is not flagged.

Also as built: the unsat core of a division by zero holds the request, the `let`s of named operands, and
table rows where the zero is written with digits.

- **(a) Confirmed, both.**
- **(b) Change:** say which.

**Recommended: (a).**

**Reply:**

<a id="q49"></a>

### 49. Confirm the small sizes of the generated arithmetic examples

You asked for examples with long numbers and many calculations. The three shipped files are small on purpose
(operands of 100 digits for a sum and 10 for a product, 100 requests, 25!), because every corpus program is
solved, dumped and reloaded per commit and `ein solve` writes a state as text: a 100-digit product is 125 ms
under `ein saturate` and 4.0 s under `ein solve`. The sizes you named are run by `utils/arith_cost.py`'s sweep
instead. Source: [Q-M1f.166](../open_questions.md#q-m1f166--the-sizes-of-the-generated-arithmetic-examples); E-24.

- **(a) Confirmed.**
- **(b) Ship the large sizes,** marked slow, checked nightly.

**Recommended: (a).**

**Reply:**

<a id="q50"></a>

### 50. Confirm that `std.num9`'s wide rules are exempt from the register tripwire

A rule may bind at most 256 variables, and a test held the widest rule of the corpus under an eighth of that,
32, *or the bound needs re-deciding*. `std.num9`'s rules are wide by design: its widest binds 162. The default
keeps the bound, keeps the tripwire on everything else, and pins the module's widest at 162. Source: [Q-M1f.173](../open_questions.md#q-m1f173--max_regs-and-the-wide-rules-of-stdnum9);
E-28.

- **(a) Confirmed.** No engine source moved.
- **(b) A narrower division start:** about 92 registers, eight firings more a division.
- **(c) Raise the bound:** an engine change.

By this generator a fixed-width module stops at fourteen digits.

**Recommended: (a).**

**Reply:**

<a id="q51"></a>

### 51. Confirm that overflow in `std.num9` is a fact and not a contradiction

Not asked at the rulings. As built: a result that does not fit nine digits derives `(overflow REQUEST)` and no
number, and the program goes on; a division by zero there is a contradiction, as in `std.arith`. Source:
[Q-M1f.168](../open_questions.md#q-m1f168--overflow-and-division-by-zero-in-stdnum9).

- **(a) Confirmed.**
- **(b) Overflow is `(false)` too:** a program that overflows once loses every other answer.

**Recommended: (a).**

**Reply:**

<a id="q52"></a>

### 52. Confirm the names of the three extension modules

All taken by default and marked yours to revise. Source: [Q-M1f.167](../open_questions.md#q-m1f167--the-names-of-stdnum9), [Q-M1f.174](../open_questions.md#q-m1f174--the-names-of-stdint), [Q-M1f.180](../open_questions.md#q-m1f180--the-names-of-stdrat).

| module | a number | requests and results | entry |
|---|---|---|---|
| `std.num9` | `(num9 0 0 0 1 2 3 0 7 8)` | `add9`/`sum9-of`, `sub9`/`diff9-of`, `cmp9`/`cmp9-of`, `mul9`/`product9-of`, `div9`/`quot9-of`; `to9`, `from9`; `overflow` | `arith9` |
| `std.int` | `(int P N)` | `int-add`/`int-sum-of`, `int-sub`/`int-diff-of`, `int-mul`/`int-product-of`, `int-cmp`/`int-cmp-of`, `int-neg`/`int-neg-of`, `int-simplify`/`int-simple-of` | `int-arith` |
| `std.rat` | `(rat Z D)` | `rat-add`/`rat-sum-of`, `rat-sub`/`rat-diff-of`, `rat-mul`/`rat-product-of`, `rat-div`/`rat-quot-of`, `rat-cmp`/`rat-cmp-of`, `rat-neg`/`rat-neg-of`, `rat-simplify`/`rat-simple-of` | `rat-arith` |

`rat-quot-of` has one answer where `std.arith`'s `quot-of` has two, since a quotient of rationals has no
remainder.

- **(a) Confirmed.**
- **(b) Rename:** say what.

**Recommended: (a).**

**Reply:**

<a id="q53"></a>

### 53. Confirm that a direct importer of the tables lists `__closed__`

`std.digits` closes its tables with written `(__closed__ R)` facts, and a selective import brings them only
when `__closed__` is listed: `(import std.digits :symbols (fa nz __closed__))`. Forgetting it is not refused;
the tables are then open and the search guesses rows. A program that imports `std.arith` never meets this.
Source: [Q-M1f.146](../open_questions.md#q-m1f146--how-stddigits-closes-its-tables-for-a-flat-importer).

- **(a) Confirmed.**
- **(b) A closing rule in the module,** imported by one name; the module is then no longer facts alone.
- **(c) The engine brings `(__closed__ R)` with `R`:** an engine change.

**Recommended: (a).**

**Reply:**

<a id="q54"></a>

### 54. Lowest terms by Euclid over division: leave it, though it is costly?

`std.rat` reduces every answer by Euclid's chain over `std.arith`'s division. Most of a request is that: 1/2 +
1/3 is 492 firings, where the comparison of 1/2 with 1/3, which reduces nothing, is 76. Source: [Q-M1f.183](../open_questions.md#q-m1f183--lowest-terms-by-stdariths-division-and-what-it-costs).

- **(a) Leave it.**
- **(b) A gcd by subtraction and halving,** which needs no division: more steps and cheaper ones. Not
  prototyped; the alternative if rationals are used at volume.

**Recommended: (a).**

**Reply:**

---

## I. Defaults taken during the phase: everything else

Each of these stands unless you object. They are internal to how the modules are built and tested, or follow
directly from a ruling. One reply for the whole table: name the numbers you want reopened.

| Q | the decision taken | |
|---|---|---|
| [Q-M1f.135](../open_questions.md#q-m1f135--which-facts-a-flat-import-brings) | A flat import brings the facts whose relation the selection reaches; a listed name may be a relation the module only states facts of | stands unless you object |
| [Q-M1f.136](../open_questions.md#q-m1f136--what-a-qualified-import-does-to-the-names-inside-a-fact) | A qualified import renames the relation of a fact too; the page says a library of facts is imported flat | stands unless you object |
| [Q-M1f.137](../open_questions.md#q-m1f137--are-an-imported-facts-constants-names-of-the-program) | An imported fact's constants are names of the program | stands unless you object |
| [Q-M1f.138](../open_questions.md#q-m1f138--the-provenance-of-an-imported-fact) | An imported fact keeps its own `:source` and gets no stamp of its module | stands unless you object |
| [Q-M1f.139](../open_questions.md#q-m1f139--the-same-fact-from-two-places) | The same fact from two places keeps its first statement; stated on the page, the question is F24's | stands unless you object |
| [Q-M1f.140](../open_questions.md#q-m1f140--a-rule-of-the-module-the-selection-does-not-name) | A rule of the module that the selection does not name is still dropped | stands unless you object |
| [Q-M1f.141](../open_questions.md#q-m1f141--does-a-selections-closure-follow-the-names-inside-a-kept-fact) | A selection does not follow the names inside a kept fact | stands unless you object |
| [Q-M1f.142](../open_questions.md#q-m1f142--a-stored-negation-and-a-fact-of-a-kernel-relation-under-a-selection) | A stored negation comes with the relation it negates; a `__closed__` fact comes only when `__closed__` is selected | stands unless you object |
| [Q-M1f.143](../open_questions.md#q-m1f143--a-fact-that-arrives-twice-through-flat-imports) | A fact that arrives by two flat imports is one statement | stands unless you object |
| [Q-M1f.144](../open_questions.md#q-m1f144--what-w88s-rows-record) | The register's rows for the import fix record the sentences a query prints; no schema change | stands unless you object |
| [Q-M1f.145](../open_questions.md#q-m1f145--how-the-table-generators-check-is-in-the-gate) | The table generator's `--check` is in the gate through a cargo test, not a step of its own | stands unless you object |
| [Q-M1f.148](../open_questions.md#q-m1f148--one-module-for-the-seven-tables) | One module for the seven tables, not a module a table | stands unless you object |
| [Q-M1f.151](../open_questions.md#q-m1f151--the-names-of-the-circuits-own-relations) | The circuit's internal relations have words for names (`add-walk`, `strip`, `turn`); see also [question 8](#q8) | stands unless you object |
| [Q-M1f.152](../open_questions.md#q-m1f152--are-stdariths-rules-written-by-hand-or-generated), [Q-M1f.164](../open_questions.md#q-m1f164--divisions-families-of-nine-rules-by-hand-or-generated) | `std.arith` is written by hand, division's two families of nine rules too, held alike by a test | stands unless you object |
| [Q-M1f.154](../open_questions.md#q-m1f154--the-bounds-of-the-harness-against-the-integers), [Q-M1f.162](../open_questions.md#q-m1f162--the-longest-sampled-operand-of-a-product) | The tests against Rust's integers: every pair to 120, sampled to 30 digits (19 for a product's operands) | stands unless you object |
| [Q-M1f.155](../open_questions.md#q-m1f155--equal-operands-of-a-subtraction-two-walks-one-fact) | Equal operands of a subtraction are answered by both walks: one fact, two derivations | stands unless you object |
| [Q-M1f.156](../open_questions.md#q-m1f156--comparison-called-with-a-tag), [Q-M1f.159](../open_questions.md#q-m1f159--multiplication-called-with-a-tag) | Comparison and multiplication are callable with a tag, one firing more a request | stands unless you object |
| [Q-M1f.157](../open_questions.md#q-m1f157--each-request-looks-its-operands-up-by-rules-of-its-own) | Each request looks its operands up by rules of its own, three a request | stands unless you object |
| [Q-M1f.160](../open_questions.md#q-m1f160--a-zero-digit-of-the-multiplier) | A zero digit of the multiplier is a row like any other, not skipped | stands unless you object |
| [Q-M1f.165](../open_questions.md#q-m1f165--what-search-free-means-of-a-program-that-divides-by-zero) | A program that divides by zero is still held search-free: no candidate, R empty, decided by root | stands unless you object |
| [Q-M1f.169](../open_questions.md#q-m1f169--where-stdnum9s-multiples-and-rows-live) | `std.num9` computes a divisor's nine multiples in one rule and carries them in the state | stands unless you object |
| [Q-M1f.170](../open_questions.md#q-m1f170--what-stdnum9-reads-as-an-operand) | `std.num9` reads a number written out and nothing else; no named operands | stands unless you object |
| [Q-M1f.171](../open_questions.md#q-m1f171--stdnum9s-step-counter-and-its-comparison) | `std.num9` counts steps and compares through `std.digits`' tables, none of its own | stands unless you object |
| [Q-M1f.172](../open_questions.md#q-m1f172--the-harness-of-stdnum9-exhaustive-at-width-2) | `std.num9` is tested exhaustively at width 2 on the generator's output; one width ships | stands unless you object |
| [Q-M1f.176](../open_questions.md#q-m1f176--a-product-of-integers-without-a-rule-that-looks-at-a-sign) | A product of integers is the four-product formula on the parts as written, no rule that looks at a sign: 142 firings for 123 × −456 | stands unless you object |
| [Q-M1f.177](../open_questions.md#q-m1f177--a-calls-tag-is-its-request-and-what-an-extension-writes-of-stdariths) | In `std.int` and `std.rat` a call's tag is the request itself, and named operands go through `std.arith`'s one `let` | stands unless you object |
| [Q-M1f.178](../open_questions.md#q-m1f178--the-harness-of-stdint-a-sibling-file-exhaustive-on-one-digit-parts), [Q-M1f.184](../open_questions.md#q-m1f184--the-harness-of-stdrat-and-the-sizes-memory-allows) | `std.int` and `std.rat` have test files of their own, exhaustive on small operands | stands unless you object |

**Reply:**

---

## J. Findings that need an owner

[`findings.md`](findings.md) holds what the stages found in the engine or the language outside their scope,
E-1 to E-31. The phase may not close with a row that has no ruling, fix or owner. For each row below choose:

- **(a)** fix in this phase;
- **(b)** a stage of a named later phase;
- **(c)** a followup;
- **(d)** leave as it is, stated on a page.

Rows already asked above are not repeated: E-2 is [question 3](#q3), E-4 [question 44](#q44), E-17 [question 36](#q36), E-19
[question 30](#q30), E-23 [question 24](#q24), E-28 [question 50](#q50), E-29 [question 40](#q40), E-30 [question 39](#q39). E-6 has its owner, F24.
Where a row's own text recommends nothing, the suggestion is marked *suggested*.

<a id="q55"></a>

### 55. May a number print as a number?

Every `:why`, `:goal-text` and read-out row prints 17 as `(n (n z 1) 7)`, so the trace of 17 + 5 is school
arithmetic in every word but its numbers. Printing a nested fact of digit names as `17` would be a rendering
of names and not arithmetic, but it touches your ruling that Ein has no numbers (Q-M1e2.24), so it is yours.
Source: E-14.

- **(a) Yes, as presentation only,** in the text surfaces; the facts, the files and every comparison stay as
  they are. Planned as part of
  [M1g's S1g.6.3](../../m1g_logic/p1g.6_engine_requirements/s1g.6.3_seeing_nested_facts.md).
- **(b) No.** A number prints as the fact it is.

**Recommended: none stated;** the row says only that it is yours to rule.

**Reply:**

### The table

| # | finding | in plain words | recommendation |
|---|---|---|---|
| <a id="q56"></a>56 | E-1, E-7 | The blind guesser proposes `__closed__`, and relation names a rule asserts as values, as if they were objects: `(total __closed__)` is a candidate | *suggested:* (b) [P1f.10](../p1f.10_hypothesis_structure/README.md), which plans how the candidates are built. The rows name no owner; you ruled the case behind them (a program's own open relations) left as it is |
| <a id="q57"></a>57 | E-3, E-12 | The fact index does not key on a nested fact, and keys a premise on its first bound argument whatever it selects; a join on a number scans | (b) [M1g's S1g.6.2](../../m1g_logic/p1g.6_engine_requirements/s1g.6.2_nested_facts_at_scale.md), which lists both. The theory avoids the join by rule |
| <a id="q58"></a>58 | E-5 | `--trace`'s *Derives* line names only a firing's first conclusion | *suggested:* (a), a small change to the trace. No shipped rule depends on it |
| <a id="q59"></a>59 | E-8 | A qualified import (`:as`) brings the module's `(query …)`, `(solve …)` and `(config …)`, so the importer is asked the module's queries and configured by its flags | *suggested:* (a), beside S1f.6.11's import fix, with a register row first if you call it a wrong answer |
| <a id="q60"></a>60 | E-9, E-10 | A rule's drawing that Graphviz warns on; and no view of a library of facts can be drawn at all. What should a view draw of a table: a row a node, or a table a node? | *suggested:* (b) M1g's S1g.6.3, the stage about the graph views, which names E-10 as a neighbour |
| <a id="q61"></a>61 | E-11 | The program a `--dump-answer` state file re-writes drops a rule that only an import's selection names; `std.arith` works around it with one fact | *suggested:* (a). Any library whose entry is a rule with an empty `:match` meets it |
| <a id="q62"></a>62 | E-13 | The graph views draw a nested fact as one opaque node: a number's digits are not the digit names the tables use | (b) M1g's S1g.6.3, as the row says |
| <a id="q63"></a>63 | E-15, E-25 | A library has no cache of its own: every program that imports `std.arith` pays 20.6 ms of load, and a `.einb` of it opens no faster than its text loads | (b) M1g's S1g.6.2, as the rows say |
| <a id="q64"></a>64 | E-16 | Long nested numbers are the parser's worst input: 10 000 nine-digit sums parse in 748 ms and add in 502 | stays with [Q-M1f.8](../open_questions.md#q-m1f8--the-parsers-remaining-cost-within-grmtools-or-past-it), the parser's remaining cost, parked there |
| <a id="q65"></a>65 | E-18 | A query cannot tell an R that is empty because root owes from one that is empty because root is dead; a query's `:expect (false)` holds on a program that only owes | *suggested:* decided with [question 24](#q24) and [question 30](#q30), as one read-out stage |
| <a id="q66"></a>66 | E-20 | A flat import of a theory built on another theory silently leaves the lower theory's rules behind; met three times. The repair is one fact in the upper theory's entry rule | (a): the row asks for one line where a selection brings a relation's declaration and no rule of its module |
| <a id="q67"></a>67 | E-21 | A load error that names no place and ends in `(at None)` | (c) [F23](../../followups/f23_diagnostics_in_eins_words.md), diagnostics in Ein's words; the row calls it a wording row |
| <a id="q68"></a>68 | E-22 | Where a state is reached by more than 2^128 commitments, a bare `ein solve` exits 1 with one stderr line and no read-out, though nothing is in error | (a): your rule of 2026-10-16 is that every answer exits 0 and a run that stops says what it established. Say whether it is a register row |
| <a id="q69"></a>69 | E-24 | `ein solve` writes every fact of a state as text, so a long computation's read-out costs far more than the computation: a 100-digit product, 125 ms to saturate and 4.0 s to solve | *suggested:* (b) an engine stage with M1g's S1g.6.2. The row says a read-out that listed facts only on request, or wrote a shared number once, would lift it |
| <a id="q70"></a>70 | E-26 | A stored fact costs about a kilobyte and a computation stores every step: 4 000 nine-digit products are 2.1 GB | *suggested:* (b) with E-24. Whether intermediate steps may be dropped once the answer is stored is a question the row says nobody has asked |
| <a id="q71"></a>71 | E-27 | The matcher's work counter does not see a guard, so the batch test would pass on a theory whose guard walks the program's names | (a), with S1f.6.12's build if [question 4](#q4) is yes: the row says that build needs a second measure |
| <a id="q72"></a>72 | E-31 | Describing R spends its time on a set computation (minimal transversals), not on saturations: 94 % of 5.1 s on a five-digit equation | *suggested:* (b) one engine stage with [question 39](#q39). The row has no owner |

One line a row; a line left empty takes the recommendation.

**Reply:**

- 56 (E-1, E-7):
- 57 (E-3, E-12):
- 58 (E-5):
- 59 (E-8):
- 60 (E-9, E-10):
- 61 (E-11):
- 62 (E-13):
- 63 (E-15, E-25):
- 64 (E-16):
- 65 (E-18):
- 66 (E-20):
- 67 (E-21):
- 68 (E-22):
- 69 (E-24):
- 70 (E-26):
- 71 (E-27):
- 72 (E-31):

---

## K. A leftover

<a id="q73"></a>

### 73. Split commit `271451fb`?

That commit of 2026-11-11 is titled as an M2 plan change (the reasoning as a program,
`plans/m2_nl_to_ir/README.md`) and also swept in this phase's round-3 design changes: `design.md`,
`findings.md` and the word-problems stage file. Splitting it means rewriting history on `master`.

- **(a) Leave it.**
- **(b) Split it** into the M2 change and the P1f.6 change.

**Recommended: (a).** The content is right; only the commit's title is narrower than its diff.

**Reply:**

---

## Merged and dropped

**Merged,** asked once:

- *An unsatisfiable search reads as owing* ([question 24](#q24)): `binary_logic.md` Q6, `beyond_arithmetic.md` Q6, E-23
  and M1g's probe 1b.
- *Should a request no rule completes say so* ([question 6](#q6)): `binary_arithmetic.md` Q7 and
  `malformed_numbers.md`'s objection on completeness.
- *Are the tagged calls an interface* ([question 41](#q41)): `beyond_arithmetic.md` Q4, the second half of
  `binary_arithmetic.md` Q5, and Q-M1f.189.
- *Where the banked programs live* ([question 43](#q43)): `beyond_arithmetic.md` Q8 with Q-M1f.186 to .188.
- *The expression layer* ([question 10](#q10)): S1f.6.14's question with `word_problems.md` Q12, which is its evidence.
- *The loader's arity check* ([question 3](#q3)) with E-2; *the order of obligations* ([question 40](#q40)) with E-29; *`ein
  test`'s route* ([question 39](#q39)) with E-30; *the trace as the pupil's page* ([question 36](#q36)) with E-17; *a gap with
  a supposition* ([question 30](#q30)) with E-19; *the width* ([question 44](#q44)) with E-4; *the register tripwire*
  ([question 50](#q50)) with E-28.

**Dropped,** not asked:

- **The gate names of `binary_logic.md` Q2.** You ruled on 2026-11-11 that any spelling other than the
  engine's own words will do and that the stage picks one convention and does not stop for it. The proposed
  names stand; [question 19](#q19) has a line for an objection.
- **Everything [`design.md` § The rulings](design.md#the-rulings-2026-11-11) settles:** the number's shape,
  the protocol, subtraction's two numbers, a strict `sub` inside an expression, division by zero as a flag and
  then `(false)`, the tables as generated plain facts, the flat import of facts, that the theory detects
  malformed numbers where it can, and that a program's own open relations beside the theory are left as they
  are.
- **Variadic relations.** No longer asked for since the number became nested.
- **E-6,** a fact stated twice losing its later `:source`: stated on the kernel pages by S1f.6.11, and the
  question is F24's.
