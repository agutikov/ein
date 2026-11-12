# P1f.6 — round 2: what your replies left open

**Phase:** [P1f.6](README.md). Written 2026-11-12, on your instruction of that day: *"there are questions I did
not answered because either didn't understand or wanted to discuss - create new list with them and additional
explanations and discussions"*.

[`questions.md`](questions.md) holds your replies as you typed them (`c828dac3`) and is not edited further.
This page takes up every reply that was a question back, a *don't understand*, or a ruling that needs one more
decision before it can be built. Each item quotes your reply, explains, says **what is done meanwhile**, and
leaves a `**Reply:**` line. An empty reply takes what *meanwhile* says.

Numbers are the old file's question numbers. Figures quoted are copied from `questions.md` and its sources.

| group | items |
|---|---|
| **Blocks a build until you reply** | [the stdlib layout](#layout), [54](#q54) |
| **Explained; a default is taken unless you object** | [5](#q5), [16](#q16), [23](#q23), [25](#q25), [31](#q31), [32](#q32), [39](#q39), [41](#q41), [47](#q47), [48](#q48), [49](#q49) |
| **Discussion, nothing built from it now** | [24, 28 and 37](#q24), [30](#q30), [33](#q33) |
| **Answers to what you asked in passing** | [18](#q18) |
| **Left empty in the first file** | [55, 73, sections I and J](#empty) |

---

## Blocks a build

<a id="layout"></a>

### The stdlib layout: `std.math` and what goes where

Your replies to 16, 38, 42 and 52 together: *"rename arith to decimal-arith"*, *"it's time to make a std.math
with subfolders"*, *"std.math.decimal-arith.overflow"*, *"not std.crypt, maybe std.math.arith.crypt"*. Two of
them name the same module differently (`arith` and `decimal-arith`), and a rename moves every program, golden
and page of the calculator, so it should happen once. A module name's dots are directories already
(`std.math.x` is `stdlib/math/x.ein`), so no engine change is expected.

**Proposed:**

| module | file | what it is | entry rule |
|---|---|---|---|
| `std.math.digits` | `stdlib/math/digits.ein` | the decimal tables, today's `std.digits` | none, facts |
| `std.math.bits` | `stdlib/math/bits.ein` | the binary tables | none, facts |
| `std.math.positional` | `stdlib/math/positional.ein` | the rules both bases share ([item 16](#q16)) | none, imported by the two below |
| `std.math.decimal-arith` | `stdlib/math/decimal-arith.ein` | today's `std.arith`: `add`, `sum-of`, … unchanged | `decimal-arith` |
| `std.math.binary-arith` | `stdlib/math/binary-arith.ein` | `bin-add`, `bin-sum-of`, … (your 14 (b)) | `binary-arith` |
| `std.math.convert` | `stdlib/math/convert.ein` | decimal ⇄ binary (your 17 (b)) | `convert` |
| `std.math.int` | `stdlib/math/int.ein` | today's `std.int`, with the three divisions | `int-arith` |
| `std.math.rat` | `stdlib/math/rat.ein` | today's `std.rat` | `rat-arith` |
| `std.math.num9` | `stdlib/math/num9.ein` | today's `std.num9`, kept as the record of another representation (your 9) | `arith9` |
| `std.math.gcd` | `stdlib/math/gcd.ein` | `gcd` and `lcm` (your 38) | `gcd` |
| `std.math.crypt` | later | the cryptarithm theory (your 42), after [item 39](#q39) and P1f.10 | |
| `std.logic.gates`, `.props`, `.circuit`, `.inputs` | `stdlib/logic/` | see [item 23](#q23) | |

`std.algebra`, `std.slots` and the other zebra modules stay where they are.

- **(a) As proposed.**
- **(b) One level deeper where a module has parts,** `std.math.decimal.arith`, `std.math.decimal.digits`,
  `std.math.binary.arith`, `std.math.binary.bits`: the directory says the representation, the file the
  subject. Closer to your *"std.math.decimal-arith.overflow"*.
- **(c) Another layout:** write it.

Also say whether the decimal requests keep their bare names (`add`, `sub`, `sum-of`) while binary's take
`bin-`. Proposed: yes, decimal stays bare, since it is what every program so far writes.

**Meanwhile:** nothing is renamed or moved. The stages that do not depend on the layout run first (integer
division, the arity check, the detection rules, the engine fixes of section J), written against today's
names; the move is one mechanical stage after your reply.

**Reply:**

<a id="q54"></a>

### 54. Lowest terms: always, or only when asked?

> gcd cost is log2? of what, of number length or value? Euclid's chain complexity? I think we don't need to
> reduce every rational, can ask explicitely for it?

**The cost.** Euclid's chain takes at most about five division steps per decimal digit of the smaller number
(Lamé's bound), so the count of steps is linear in the number's *length*, logarithmic in its value. Each step
is one long division, whose work is the length times the digits of its quotient, plus nine multiples of the
divisor that the shipped division computes first. Over a whole chain that comes to about the square of the
length, with a large constant: the nine multiples are computed again for every step. That constant is what
shows on small numbers: 1/2 + 1/3 is 492 firings, the comparison of the same two, which reduces nothing, 76.

**Asking explicitly is possible**, and `rat-simplify` exists already. What changes if reduction is not
automatic:

- An answer is no longer canonical: 1/2 + 1/2 is `4/4`, and two equal results are two different facts. A rule
  that wants *the same number* must ask `rat-cmp`, which already compares by cross products and never needed
  lowest terms.
- A chain of operations grows: denominators multiply at every step unless something reduces.
- A sum falls to about a fifth of its firings.

- **(a) Leave it:** every answer in lowest terms.
- **(b) Reduce only on request:** the operations answer unreduced, `rat-simplify` reduces, `rat-cmp` is the
  equality. The tests that hold *every answer is in lowest terms* are restated.
- **(c) Both, by request name:** `rat-add` reduced as today, and a raw family beside it.

**Recommended: (b),** which is what you lean to; it also removes the reason gcd lives inside `std.rat`, since
`rat-simplify` can then call `std.math.gcd`.

**Meanwhile:** `std.rat` is untouched.

**Reply:**

---

## Explained, with a default taken

<a id="q5"></a>

### 5. A name that no `let` states

> Why absent? Why not (false)? It's calculator semantic - used but not declared variable (constant in our
> case). Or that means it can be deduced, and then after declaration appears finished calculation?

Your second sentence is the reason. `(add foo X)` is answered by a rule that looks `foo` up in `(let foo ?v)`.
With no such fact the rule does not fire, and that is all a monotone theory can do: to a rule, *no `let` yet*
and *no `let` ever* are the same state, and a fact added later completes the calculation. To derive anything
from the missing `let`, a flag or `(false)`, a rule must have the premise *there is no `(let foo …)`*, and in
Ein that premise is spelt `(absent (let ?x ?v))`. So `absent` is not an alternative to `(false)`; it is the
only way to reach it.

Here it is harmless: `let` is written by the program and derived by no rule, and the theory closes it, so the
negation is over a relation nothing can extend. The loader's *forbid* does not refuse that.

- **(a) Silence,** as today.
- **(b) The calculator's semantics:** `(undeclared NAME REQUEST)` derived through one `absent` over `let`,
  and `(false)` from it, as a division by zero. The theory's claim becomes *one `absent`, over a relation no
  rule derives*.

**Meanwhile: (b),** built with the detection rules of your 4 (a), since your reply reads as wanting the
calculator to refuse an undeclared name. The cost measured on the prototype was some hundreds of comparisons
a request for asking *is this a name*; the build re-measures and reports it.

**Reply:**

<a id="q16"></a>

### 16. The rules both bases share, in one file

> yes, extract common rules into separate file the both import, btw rename arith to decimal-arith

This is neither of the options asked (generate one file from the other), and it is better if it works: 51
rules written once, with no generator. The shared rules must then not name what differs between the bases:
the constructor (`n` or `b2`), the tables, and the request names. Ein has the mechanism: a rule with
parameters is compiled once per activator fact, so `(rule add-column (?num ?table) …)` activated for
`(n fa)` and for `(b2 fa2)` gives two specialised plans and should cost nothing at run time. It is not the
generic theory of 13 (c), which carried the base in every request and paid for it in firings.

What is not known until it is tried: whether a parameter may stand as the head of a nested number pattern in
every place the walks need it, and whether the firings stay at today's count.

**Meanwhile:** the stage starts with that trial. If the parameterised rules cost what the shipped ones do,
they are the shared file. If they cannot be written or cost more, the stage stops and shows you the
measurement beside the generated copy (the old (a)), which is what `examples/binary_arithmetic/` has today.
It runs after the layout reply.

**Reply:**

<a id="q23"></a>

### 23. Where the logic modules live

> (a) and examples/binary_logic already made

(a) was *all four small modules and the six applications ship*. The applications are in
`examples/binary_logic/`, as you say. The four modules are there too, as example files; shipping them means
moving them into the stdlib with a test program per rule under `tests/stdlib/` and the mutation census, as
every stdlib module has. With your other replies they would be:

- `std.logic.gates`: the table `(gate2 And 1 1 1)`, and the relation-per-function layout derived from it by
  sixteen rules (your 19 (c));
- `std.logic.props`: the property **rules** (your 20 (b), *"in separate reusable file"*), with the left and
  right identity and absorbing forms (your 21 (b));
- `std.logic.circuit` and `std.logic.inputs`.

- **(a) Into the stdlib as `std.logic.*`,** the examples importing them from there.
- **(b) They stay example files** until M1g, which starts from them.

**Meanwhile: (a),** after the layout reply, since `std.logic` is part of it.

**Reply:**

<a id="q25"></a>

### 25. The search over inputs that each take exactly one value

> Is this about SAT, binary circuits or what? I don't see how (a) and (b) conflict, are they?

It is about SAT, in Ein's form. To ask *is there an input for which this circuit outputs 1*, each input is a
hypothesis that owes exactly one of two values. The engine then explores commitments: for n inputs, 2ⁿ
saturations to find one answer and 3ⁿ to describe all of R (each input is 0, 1 or not yet chosen). Nothing in
the engine knows that *exactly one value per object* is a special, very common shape, which a SAT solver
exploits with unit propagation and learned clauses.

You are right that they do not conflict. (a) said *this phase needs no engine work*, (b) said *M1g will*. Both
hold.

**Meanwhile: both.** P1f.6 treats the search form as a demonstration, and M1g's engine-requirements phase
gets a stage for a cheaper route on this shape, with the table of `questions.md` 25 as its witness.

**Reply:**

<a id="q31"></a>

### 31. A reading is always chosen

> reading always chosen, it can't be not chosen: result depends on formula, formula depends on selected arith
> operations, selection depend on reading. […] btw there could be the case when answer is correct, but
> solution is not, right?

Taken as option (b), *always owed*. Its consequence is the case you name: with 12 apples and boxes of 3 both
readings answer 4, so R is **two states with one answer**: two solutions, one of which may be the wrong
reasoning with the right number. The query's projection shows one class, 4; the states show the two readings.
That is the honest output under your ruling, and it is the first corpus witness of *answer correct, solution
not necessarily*.

The banked examples use (a), *never owed*, today, so they change: `readings.ein` owes a reading, `boxes12`
becomes two states, and the 13-apple problem (4 or 5) should come back. One obstacle is known: the 13-apple
variant was left out of the banking because one prototype test of the engine's suite
(`fact_grain_never_calls_a_violated_cube_monotone`) does not end on such a program (E-33 of
[`findings.md`](findings.md)). The stage that rebuilds the readings has to resolve that first, without
changing what the test means.

*Why did you divide and not subtract* is answered by the trace only as far as the rules go (*need* with *a
box holds* means division); a theory of solutions is M1h's and M2's, as you say.

**Meanwhile:** the examples are rebuilt to *always owed*.

**Reply:**

<a id="q32"></a>

### 32. *What is not listed is zero*

> what is not listed is zero - good idea, it could be a reusable universal assumption.

It is the closed-world assumption for one relation, and it needs `absent` over `has`. To keep it an
*assumption* and not the theory's meaning of the words, it is built as a module of its own that a problem
imports when it adopts it, `examples/word_problems/listed_is_all.ein`: with it, *Alice has 3 pears* gives
*Alice has 0 apples*, and the answer carries a `rests-on` fact naming the assumption (your 29 (a)). Without
the import the amount stays a gap, as banked.

**Meanwhile:** built that way, with the apples-and-pears problem in both forms.

**Reply:**

<a id="q39"></a>

### 39. `ein test` and the cheapest route

> I don't understand the question, explain in more details and examples

`ein solve` has modes that each do the least work for their question: `--single` asks *is there exactly one
answer*, `-n 3` asks for three states, a bare `ein solve` describes the whole region R. `ein test`, which
checks a file's `:expect` claims, has one way only: it describes all of R and then compares.

On a program that searches, describing R can cost far more than deciding the claim. SEND + MORE = MONEY
carries the claim *the answer is exactly this assignment*:

| command | what it does | result |
|---|---|---|
| `ein solve --single send.ein` | finds the assignment, shows no other exists | *Solution — R is one state*, 0.5 s |
| `ein test send.ein` | tries to describe all of R, runs out of budget | *NOT CHECKED*, 7.5 s, exit 1 |

So a claim the engine can decide in half a second fails in the test runner, and the puzzle cannot be in the
corpus with its answer stated.

The proposal: `ein test` reads the shape of each claim and runs the cheapest mode that decides it. A claim of
one state is `--single`'s question; a claim of exactly k states is `-n k+1`'s (find up to k+1: a (k+1)-th
refutes the claim); only a claim about the whole region describes it. It is your rule *minimum work per
question*, applied to the runner.

- **(a) Yes, as an engine stage.**
- **(b) No:** a search program too large to describe carries no claim.

**Meanwhile: (a), planned as a stage of this phase** and run after the section J fixes; the cryptarithm
theory (your 42) waits for it.

**Reply:**

<a id="q41"></a>

### 41. The tagged calls

> I don't understand the question

There are two ways for a rule of yours to use the calculator.

**The public way.** Your rule asserts a request, `(add X Y)`, and the theory answers `(sum-of X Y S)`. To use
the answer, a second rule of yours must find the `sum-of` fact that belongs to *your* numbers:

```
:match (and (price ?item ?x) (tax ?item ?y) (sum-of ?x ?y ?s))
```

That premise joins two facts on a number. The engine's index does not key on a nested fact, so it scans every
`sum-of` there is; with a thousand requests each lookup walks a thousand facts.

**The tagged way,** which the theory uses inside itself. Your rule starts the addition under a name of its
own, the tag, and the answer comes back under that tag:

```
:assert (add-walk (total ?item) ?x ?y 0 z)      ; ask
:match  (done (total ?item) ?s)                 ; read: one premise, no join
```

The reading rule has one premise, so nothing is scanned. `std.int`, `std.rat`, the conversions and eight
banked examples are written this way. Checking the laws of arithmetic over the digits took 16.8 s the public
way and 0.42 s the tagged way.

**The question** is only whether those inner relations (`add-walk`, `mul-call`, `div-call`, `cmp-walk`,
`done`, `div-done`, `order-is`) are *promised*: documented as what a theory built on the calculator calls,
and held by a test so they cannot change shape unnoticed. Today the module's header calls them internal.

- **(a) Yes, they are an interface.**
- **(b) No:** outside programs use requests and pay for the join.

**Meanwhile: (a).** They are what every theory over the calculator already uses, and your reply to 8 asks
that theories connect conveniently. The stage on import name clashes (your 8) takes their names into account.

**Reply:**

<a id="q47"></a>

### 47. The comparison table's column order

> I don't understand the question

A small speed detail; it changes no answer. Comparison reads a table of 300 rows, `(cl ORDER-SO-FAR A B
ORDER-AFTER)`. When a rule looks a row up, the matcher uses the row's *first* argument to narrow the
candidates, and here the first argument is one of three words (`lt`, `eq`, `gt`), so it is offered 100 rows
to find the one that fits. Had a digit been first it would be offered 30. The whole effect: a nine-digit
comparison considers 901 candidate rows where a sum considers 181, and 4 000 comparisons still take 74 ms.

**Meanwhile: (a), leave it.** The general cure is the engine choosing the most selective argument, which is
row 57 of section J and goes to M1g with it.

**Reply:**

<a id="q48"></a>

### 48. How far the division-by-zero flag reaches

> I don't understand the question: (div ?x 0) must emit division-by-zero no matter where it happens (except
> theories where it is allowed - like wheels, meadows, transreal arithmetic, etc...).

The question was about two places where *a division by zero happens* without a program writing `(div X 0)`.

1. **A division called from inside another theory.** Through the tagged call of [item 41](#q41),
   `(div-call TAG X z)`. Today that derives nothing; only the public request `(div X z)` is flagged, and each
   theory above writes a rule of its own (`std.rat` does). By your reply this is wrong: **the flag moves to
   the division itself**, so any call with a zero divisor derives `(division-by-zero CALL)` and `(false)`,
   whoever made it.
2. **A rational written over zero,** `(rat 3 z)`, used in a product or a comparison. No division is
   performed, yet as built any request with such an operand is flagged, and the program is dead. That agrees
   with your principle (3/0 *is* a division by zero) and stays. A `let` that names such a rational and that
   no request uses is not flagged, since nothing is computed that nobody asks for.

A theory in which division by zero is allowed (a wheel, a meadow) would be a module of its own that does not
import the flag's rule.

**Meanwhile:** 1 is changed as said; 2 stays as built.

**Reply:**

<a id="q49"></a>

### 49. The biggest calculation

> Let's have biggest solvable arithmetic calculation in let's say 100+-50 seconds on this host. make a
> generator in python that takes some size arguments and generate an example.

Two things to know before the number is chosen. **Memory ends a calculation before time does:** a stored
fact costs about a kilobyte and every step is stored, so the longest product a program can write, 998 digits
by 998, is 9.8 s and 5.9 GB under `ein saturate`; a hundred seconds of saturation would need tens of
gigabytes. And **`ein solve` is many times dearer than `ein saturate`** on the same program, because it
writes the state as text (a 100-digit product: 125 ms against 4.0 s).

**Meanwhile:** `utils/gen_arith_examples.py` takes size arguments (operation, digits, count of requests,
factorial's n) and writes a program to a path you give. The stage finds, for `ein saturate` and for
`ein solve` separately, the sizes that land in your window or hit memory first, and records the commands and
figures in `stdlib/README.md`. The big file is **generated on demand and not checked in**: every file under
`examples/` is solved, dumped and reloaded per commit.

Say if you want it checked in and run nightly instead.

**Reply:**

---

## Discussion

<a id="q24"></a>

### 24, 28 and 37. Open against false; arithmetic against algebra

> solution of `x × 7 = 862` belongs yet to arithmetic or already to algebra? this question open vs. false is
> deep. What would give more benefits, I expect `open` will, but why? Can we change it quickly later? We
> already have core for dead state, do we have anything like this for open? […] It looks like an entire
> separate phase.

> Is it possible at all to solve it by saturation without algebra rules that allows infer x*7=862 ->
> x=862/7 ? Again is it arithmetics or algebra?

**Three methods for one question.** The question, *which x*, is algebra's: it has an unknown. The methods
differ:

1. **Arithmetic read backwards.** The multiplication table is a relation, `(md A B CARRY-IN DIGIT
   CARRY-OUT)`, and a rule may ask it *which A, with 7, gives a units digit 2*: only 6. Carry 4; the next
   column asks which digit times 7 plus 4 ends in 6, and so on. It is the school multiplication run from the
   answer to the operand, a firing a column, with no rewriting and no search. So yes, it is possible without
   an algebra rule, and it was run (eleven rules; the banked examples under `examples/beyond_arith/`). For
   862 the last column cannot close, and the result is a **dead root** with a three-row core.
2. **Search.** Each digit of x is a hypothesis that owes one of ten values; the engine tries assignments.
   This is where **`Open`** appears.
3. **Algebra.** A rule rewrites the equation, x × 7 = 862 ⇒ x = 862 ÷ 7 with remainder zero, and the rest is
   arithmetic. This is P1f.8's subject, and nothing of it is built.

Your remark on `(let x <number>)` is right and is why method 2 guesses *digits*, ten candidates a place, and
never numbers from the KB.

**Why the same question is `false` one way and `Open` another.** In 1 the program's own facts contradict
each other, so the root state is dead. In 2 no fact contradicts another at the root: the root is alive and
merely *owes* the digits. Every complete way of paying is dead, so R is empty, but the root itself is not.
`Open` was introduced (M1d) to mean *not yet a model*: information is missing and could arrive. Here the
search has shown more, *no way of paying exists among the candidates*, and the word does not say it.

**What explains an open state, as a core explains a dead one.** Two things exist today:

- *what is owed*: the obligation instances (which requirement, of which object). This is what the hypothesis
  generator reads, as you remember: it proposes the facts that would pay a debt.
- *why it cannot be paid*: the minimal dead sets N, each with its own core. Together they are the proof: for
  every way of paying the debt, one of them refutes it. For x × 7 = 862 that is 28 dead sets. They are
  written to `nogoods.json` and are not in the read-out, and nothing assembles them into one object, *this
  debt, and for each alternative the refutation that kills it*. That object would be the open state's core.

**Which gives more.** `Open` keeps more: the partial state stays inspectable, the gap is named, and it
separates a text that contradicts itself from a requirement nothing satisfies, which are different faults in
a word problem (*gave 7 of 5* against *how many together, Alice's amount unstated*). `false` gives one proof
object and propagates: a hypothesis whose every completion is dead becomes dead itself, which is what a
solver learns from. They are not exclusive: the reading *a debt that provably cannot be paid within the
candidates is a refutation* turns the second into the first exactly where the search has finished.

**Can it be changed later: yes, cheaply.** R is the same set under both readings, empty. Only the label on
states *outside* R and the explanation printed would change, so no answer, claim or register row about R
moves. That is the reason not to decide now.

**Proposed:** nothing is built from 24, 28 (b) stays the banked form as you said, and 37's inverse requests
are **not** added to the stdlib now. Two plans take the subject:

- a phase of [M1g](../../m1g_logic/README.md), *unsatisfiable against owing*: the open state's core, the
  read-out row for N, and whether a finished search may call an unpayable debt dead. M1g's first probes
  already met it.
- [P1f.8](../p1f.8_school_algebra_and_geometry/README.md) compares the three methods on school equations,
  where the choice between them is the subject.

**Reply:**

<a id="q30"></a>

### 30. Is a simple ambiguous problem already non-monotone?

> that simple problem couldn't have non-monotone solution […] Or that means ambiguity == non-monotone
> solution == required additional information?

Not quite, and the difference is worth having for M1h. **Ambiguity is monotone**: R has several states and
nothing is retracted. What is non-monotone is one sentence: *Alice's amount is **not stated***. It is true
of the problem as given and false as soon as anything supposes or supplies the amount; any statement of the
form *X is unknown* is retracted by learning X. So:

- the gap written **as a fact** (*not stated*, by `absent`) is non-monotone, for every problem, however
  simple;
- the gap written **as a debt** (the state owes the amount) is monotone, because it is the engine and not a
  rule that says *missing*, outside the facts.

That is why the two forms of `questions.md` 28 behave differently, and why your (a) here, a read-out row
that names what root owes, lets the monotone form say what the non-monotone one says. So: *required
additional information* = a debt; *ambiguity* = several states; neither needs non-monotone rules unless the
program wants the missing thing as a fact to reason from.

**Meanwhile:** your (a) is built, a read-out row for what root owes when R is not empty. The rest is M1h's.

**Reply:**

<a id="q33"></a>

### 33. Which assumptions a model has to state

> Is this a finite set of assumptions? What is it limited by? Used names/words? Probably we don't need to say
> that things are not alive and don't run out of the boxes […] But the same time we assume things fit boxes,
> while problem doesn't have this word. Btw, if we add rules and ontology that restricts throwing things away
> and after that the solution will not touch those rules and facts (see my answer 27) - that means this is
> not required information?

**In the world the set is not finite** (the frame problem). **In a theory it is bounded by the rules, not by
the text's words.** An assumption has to be stated exactly where a rule's conclusion depends on it. *Things
fit boxes* is needed although the text never says *fit*, because the rule that reads *a box holds 3* as a
division is valid only under it. *Things are not alive* is needed by no rule, so it is not stated. Each rule
is a modelling decision about what a word means, and its assumptions are what make that reading valid: the
set grows with the vocabulary the theory covers, a few per rule.

**Your slicing observation is the reason option (a) was recommended.** If an assumption is a fact that the
dependent rules take as a premise, then the slice of an answer (your phase from 27) contains exactly the
assumptions that answer rests on, with no extra mechanism, and one the derivation never touches is not
required for *that* answer.

One thing slicing by derivation does not show, and M1h's slicing phase should: a **restriction that was
checked and held**. *Nobody gives away what they do not have* derives nothing when it holds; it would only
have derived a contradiction. So it leaves no trace in the answer's derivation, yet the answer is valid only
because it passed. *Untouched* then has two meanings, *not applicable to this problem* and *applicable,
tested, satisfied*, and only the first is superfluous information.

**Meanwhile:** nothing is built; both points go into M1h's slicing phase as its opening questions.

**Reply:**

---

## Answers in passing

<a id="q18"></a>

### 18. What division is shipped

> I expected you implement https://en.wikipedia.org/wiki/Long_division ; What is shipped now?

Long division is what is shipped: the dividend's digits are brought down one at a time from the top, each
step finds one quotient digit and carries the remainder to the next. The one detail that is the theory's
own is how a step finds its digit. A pupil guesses and corrects; a rule cannot guess, so the divisor's nine
multiples are computed once at the start, and each step finds the largest multiple that fits by bisection,
at most four trial subtractions. The *simpler one* of the question replaced that with repeated subtraction
of the divisor, up to nine a digit and no multiples. Left as shipped, as you say.

---

<a id="empty"></a>

## Left empty in the first file

- **Section I** (the table of other defaults): no objection named, so all stand.
- **Section J** (findings that need an owner): every line was left empty, and the file said an empty line
  takes the recommendation. That makes these **fixes in this phase**, each a stage: the trace's *Derives*
  line (58, E-5); a qualified import bringing the module's queries and config (59, E-8); the dumped program
  dropping a rule only a selection names (61, E-11); a flat import that leaves the lower theory's rules
  behind, said in one line (66, E-20); `ein solve` exiting 1 with no read-out past 2¹²⁸ commitments (68,
  E-22), filed as a register row first since an answer that exits non-zero is against your rule; and a
  second measure for guards (71, E-27), with the detection rules. The others go to the owners the table
  named: P1f.10 (56), M1g's S1g.6.2 (57, 63, 69, 70) and S1g.6.3 (60, 62), F23 (67), Q-M1f.8 (64), the
  read-out stage of 30 (65), and the `ein test` stage of 39 (72). If you did not mean to take them all,
  name the rows to drop.
- **55, may a number print as a number** (`17` for `(n (n z 1) 7)`, in text surfaces only): empty, so open;
  nothing is built. It touches your ruling that Ein has no numbers, so no default is taken.
- **73, split commit `271451fb`:** empty; left as it is, which was the recommendation.

**Reply:**

---

## What was taken from your other replies

For the record, the replies that were rulings and are acted on without a further question:

| reply | what happens |
|---|---|
| 1 (d), 2 (a) | three integer divisions in `std.int`, `int-div-euclid`, `int-div-trunc`, `int-div-floor`; the Euclidean remainder a natural number |
| 3 (a) | the loader refuses a wrong count of arguments: register rows, the check, a sweep |
| 4 (a), 9 (b) | the detection rules in the decimal theory; `num9` stays silent |
| 7 | a program that states false facts (`2 + 2 = 5`): a question recorded for P1f.8 |
| 8 | a stage: name clashes between imported theories, investigation, proposals, fix |
| 10 | expressions move to P1f.8, with checks of used names and of recursive use |
| 12 | no promotion; a types theory as a stdlib module is M1g's |
| 13 (b), 14 (b), 15 (a), 17 (b) | binary arithmetic ships as a theory of its own, `bin-` names, one `z`, conversions a module of their own |
| 14 | a P1f.8 stage: compound numbers without a hardcoded representation |
| 19 (c), 20 (b), 21 (b), 22 (a) | the logic table in both layouts, properties as rules in a file of their own, one-sided forms too |
| 26, 27, 29, 34, 35 | the word problems stay as banked, in `examples/`, with only the readings needed now |
| 40 | P1f.10: discover the hypothesis structure first, then order by it |
| 45 | the drawing test skips big graphs; Graphviz itself is not tested |
| 46 (b) | the extra gate step is removed; the batch test is run by hand |
| 44, 50, 51, 53, 43 | confirmed as built |
| your TODOs | M1h (temporal and causal theories first, then slicing, gaps and assumptions, formula against calculation, the school text, requirements verification, TRIZ), M15 (an MCP server), M1j (`nlp/` and `smt/` as one proof of concept) are written as plans |
