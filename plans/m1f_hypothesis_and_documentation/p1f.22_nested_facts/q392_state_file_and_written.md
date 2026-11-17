# Q-M1f.392, in detail — a state file that reads back dead, through `written`

**Phase:** [P1f.22](README.md). Written 2026-11-17 on the user's word (*"Q-M1f.392 - I don't understand - write
with details into separate file in plans"*). The question is
[Q-M1f.392](../open_questions.md#q-m1f392--a-state-file-that-reads-back-dead-through-written); it is **not ruled**.
Every output below is a run of the gate build, `ein.rs/target/gate/ein`, taken for this page.

The short form: **`ein solve --dump-answer` writes each state of the answer as a `.ein` file, and promises that
loading that file gives the same state back. For one kind of program the file gives back a *dead* state instead.**
Nothing is wrong with the answer `ein solve` printed; what breaks is the promise about the file.

It takes four things to see why, and each is small.

## 1. What `written` is

A fact under an arrow of another is not held, so no rule's premise sees it (the rest of this phase). That leaves a
module that defines a *representation* with no way to check its own terms. The decimal numeral is the case that
made the relation: 42 is `(n (n z 4) 2)`, a fact of `n` that nobody states and that only ever stands under an
arrow, as in `(price Apple (n (n z 4) 2))`. If a program writes `(price Apple (n z x))`, with a letter where a
digit goes, `std.math.decimal` wants to say *this is no number* at once, before any arithmetic is asked for
(Q-M1f.210, Q-M1f.272: a malformed value is its representation's problem, flagged where it is written).

A rule can only read held facts. So **the loader holds one fact about each nested fact a file writes**:

```lisp
(price Apple (n (n z 4) 2))
```

`ein saturate --dump` on that line holds

```text
(price Apple (n (n z 4) 2))
(written (n (n z 4) 2) :rule "#written")
(written (n z 4) :rule "#written")
```

`(written T)` says *the program's text wrote `T` under an arrow*. It does not say `T` holds: `(n z 4)` is still not
held. It is a held fact that *points at* `T`, and a rule may match it:

```lisp
(rule no-bad ()
  :match  (written (bad ?x))
  :assert (false)
  :why    "bad {?x} is written")
```

`std.math.decimal`'s five checking rules are of this shape, over `(written (n ?p ?a))`.

## 2. What `written` deliberately does not cover

**A fact that a rule assembles under an arrow is not `written`.** The text wrote a pattern with a variable in it,
not the fact:

```lisp
(item a)
(rule make ()
  :match  (item ?x)
  :assert (note (pair (bad ?x) 2))
  :why    "note {?x}")
(rule no-bad ()
  :match  (written (bad ?x))
  :assert (false)
  :why    "bad {?x} is written")
(query :goal (note ?n))
```

This is `f13.ein`. The rule `make` fires and the state holds `(note (pair (bad a) 2))`, in which `(bad a)` stands
under an arrow. Nobody wrote `(bad a)`: a rule built it. So there is no `(written (bad a))`, `no-bad` does not
fire, and the program is alive:

```text
$ ein saturate --dump f13.ein          (the held facts)
  (item a)
  (note (pair (bad a) 2) :rule make)

$ ein solve f13.ein
  states (k_s)      1
  ∅                 in R — root is alive and owes nothing
```

That is ruled, not an accident (Q-M1f.275's second half, Q-M1f.280): `written` is decided once, when the program is
read, so it is the same in every state the search visits, and the calculator, whose every step nests its operands
in a fact, does not pay for a `written` fact per step. The standard library's page says it in so many words: *a
numeral a rule assembles from a variable … is no term a file writes*, and is silent.

So the same nested fact `(bad a)` is treated two ways, by **who put it there**:

| the fact that nests `(bad a)` | `(written (bad a))` | `no-bad` |
|---|---|---|
| written in the file: `(note (pair (bad a) 2))` | held | fires: dead |
| concluded by the rule `make` | not held | does not fire: alive |

## 3. What a state file is

`ein solve FILE --dump-answer DIR` writes, for each state of the answer, a file `DIR/models/NN.ein`. It is **the
program, and then every fact the state holds, written out as plain facts**. For `f13.ein`:

```lisp
;;; state 1 of 1 — written by `ein solve --dump-answer` (ein-answer/2).
;;; …
;;; Below: the program as loaded, imports inlined, and then every fact of
;;; the state (2 facts; the program's own relation declarations are not
;;; repeated). It loads, and saturating it derives nothing new: the state is
;;; a fixpoint of the program (10-answer-dump.md, every state file loads).

(item a)
(rule make () :match (item ?x) :assert (note (pair (bad ?x) 2)) :why "note {?x}")
(rule no-bad () :match (written (bad ?x)) :assert (false) :why "bad {?x} is written")
(query :goal (note ?n))
;; ── the state ──────────────────────────────────────────────────────────

(item a)
(note (pair (bad a) 2)) ; rule make: note a
```

The point of the file is that it is an ordinary program: `ein solve DIR/models/01.ein` loads it, and the header
promises that the result is the state itself, *"saturating it derives nothing new"*. That promise is what makes a
dumped answer something another tool, or a person, can load and continue from; `10-answer-dump.md` states it as
*"derives no fact and no `(false)`"*, and `ein-cli/tests/answer_round_trip.rs` reloads every corpus program's
states to hold it.

## 4. Where the two meet

Look at the last line of the state file. The fact `(note (pair (bad a) 2))` was **concluded by a rule** in the run
that wrote the file. In the file it is **a fact a file writes**. The words *rule make* are in a comment, which the
loader does not read. So on reload the loader does what § 1 says it does for every fact a file writes:

```text
$ ein saturate --dump f13.ans/models/01.ein     (the held facts)
  (item a)
  (note (pair (bad a) 2))
  (written (pair (bad a) 2) :rule "#written")       ← new: the first run held neither of these
  (written (bad a) :rule "#written")                ←
  (false :rule no-bad)                              ← and so no-bad fires

$ ein solve f13.ans/models/01.ein
  states (k_s)      0
  empty             every node is dead — root's own fixpoint derives (false)
  unsat core        (note (pair (bad a) 2))
  refuted by        no-bad
```

**The state was alive when it was dumped, and the file of it loads dead.** Step by step:

| | the run of `f13.ein` | the reload of `models/01.ein` |
|---|---|---|
| where `(note (pair (bad a) 2))` comes from | the rule `make` | a line of the file |
| `(written (bad a))` | not held | held |
| `no-bad` | does not fire | fires |
| the state | alive, 1 state | dead, 0 states |

The same with the standard library instead of a made-up `no-bad` (`h2.ein`): a rule assembles the numeral
`(n z ?d)` from a fact `(digit-read x)`, giving `(reading (n z x))`. The run is alive, since an assembled numeral is
not checked (§ 2). The state file writes `(reading (n z x))` as a line; the reload states `(written (n z x))`;
`std.math.decimal` flags it: `states (k_s) 0`, `refuted by std.math.decimal.not-decimal-is-false`.

## What it takes for a program to hit this

All three together:

1. a rule **concludes a fact that nests a fact built from a variable** (so the nested fact is not `written` in the
   run);
2. some rule **reads `written`** (in practice: the program imports `std.math.decimal` or `std.math.binary`, or has
   a checking rule of its own);
3. that rule's pattern **matches the assembled fact**, i.e. the assembled term is one the check would have refused
   had a file written it.

A program that only assembles *well-formed* terms is not affected in what it concludes: the reload holds `written`
facts the run did not, but no checking rule fires on a well-formed term, so every other fact of the state reads
back as it was. No program of the corpus is known to hit it: `answer_round_trip.rs`, which reloads every corpus
program's dumped states, is a test of the gate. Neither S1f.22.1 nor this page ran that test by itself, so *no
corpus program hits it* rests on the gate and not on a run made for the question.

So this is not a wrong answer of `ein solve`: R is right. It is a state file that does not keep its promise, for a
program that builds a malformed term by rule in a program that checks written terms.

## The options, each as what it would do to `f13.ein`

**(a) The state file says which facts the state derived, and the loader states no `written` for them.**
The grammar already has the clause: a fact may be written `(note (pair (bad a) 2) :rule make)`. Today the loader
reads that clause as a remark and treats the fact as written like any other (`k1.ein` in the analysis: still dead).
Under (a) the dump writes the clause on every derived fact, and a fact that carries it gets no `written`.

- `f13.ans/models/01.ein` then reloads alive, as the state was. The promise holds with no exception.
- Costs: the dump's writer; the loader's `written` step; a mark for a *committed* fact too (a hypothesis the state
  holds was not written by the program either, and the grammar has no clause for that yet). And a consequence to
  accept: a hand-written program could put `:rule x` on a malformed numeral to slip past the check. A state file
  becomes a slightly different kind of file from a program, where today `10-answer-dump.md` says it *"is the
  state's facts, not a second kind of program that re-states where each came from"*.

**(b) `written` follows every held fact**, derived and committed ones too.
Then the run itself states `(written (bad a))` when `make` fires, `no-bad` fires in the run, and `f13.ein` *itself*
is dead. Run and reload agree, by making the run stricter.

- Costs: it reverses a ruling (Q-M1f.275: an assembled term is not `written`; Q-M1f.280: `written` is decided at
  root, never per state). Every step of the calculator nests its operands in its tag, so every step would state
  `written` facts and run the checks over them: the calculator's cost per step rises, and `written` stops being
  the same in every state of a search. It also changes R of any program that today assembles a malformed term by
  rule and is alive.

**(c) Say it.** The engine does not change. `10-answer-dump.md`, `12-rendering.md` and the header's sentence gain
the exception: *a state that holds a fact a rule assembled under an arrow, in a program with a rule over `written`,
can read back with `written` facts it did not hold, and such a rule then fires.*

- `f13.ans/models/01.ein` still reloads dead; the page now says that it may.
- There is one exception of this kind already: a state nested deeper than the parser's limit of parentheses does
  not load again, and the pages say so (Q-M1f.185). (c) adds a second.

| | the reload of `f13`'s state | the run of `f13.ein` | changes |
|---|---|---|---|
| today | dead | alive | — |
| (a) | **alive** | alive | the dump's writer and the loader; a state file marks derived facts |
| (b) | dead | **dead** | reverses Q-M1f.275 / .280; the calculator pays per step; R of some programs |
| (c) | dead | alive | three sentences |

## What I would do, and why it is your call

The default taken in the analysis is **(c)**: no corpus program hits this, and (a) builds a mechanism for it.
If *"every state file loads, and is the state"* is a promise you want to hold **without exception**, the way is
**(a)**, and it is not large; I would not choose (b), which fixes a file's promise by changing what programs mean.

The question to answer is therefore one: **is the round trip a promise with listed exceptions (c), or one with
none (a)?**
