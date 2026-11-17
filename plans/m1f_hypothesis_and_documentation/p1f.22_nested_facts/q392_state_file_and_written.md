# Q-M1f.392, from the start — saving an answer and loading it back gives a different answer

**Phase:** [P1f.22](README.md). Second version, 2026-11-17, on the user's word (*"I still do not understand, show
an issue, how to reproduce it, explain for dummies"*). The question is
[Q-M1f.392](../open_questions.md#q-m1f392--a-state-file-that-reads-back-dead-through-written).
The outputs are runs of `ein.rs/target/gate/ein` taken for this page.

**Ruled 2026-11-17: it is a bug.** The user: *"the way you describe it in short … it is a bug and has to be
verified with simple unit test. Where from comes (written ...)???? Why? What for?"* So it is fixed, and a unit
test holds it. The two questions are answered in
[§ Where `written` comes from](#where-written-comes-from-why-and-what-for), and the fix proposed, with the test, in
[§ The fix, and the unit test](#the-fix-and-the-unit-test).

## The issue in one sentence

**`ein` can save a solution to a file. For some programs, loading that saved file back says "there is no
solution".**

## Reproduce it: three commands

The program is [`q392_demo.ein`](q392_demo.ein), beside this page. From the repository's root:

```sh
EIN=ein.rs/target/gate/ein
P=plans/m1f_hypothesis_and_documentation/p1f.22_nested_facts/q392_demo.ein

$EIN solve $P                                # 1. solve the program
$EIN solve $P --dump-answer build/q392       # 2. solve it again, saving the solution as files
$EIN solve build/q392/models/01.ein          # 3. load the saved solution
```

What they print (the lines that matter):

```text
1.   states (k_s)      1
     ∅                 in R — root is alive and owes nothing          ← one solution

2.   (writes build/q392/models/01.ein)

3.   states (k_s)      0
     empty             every node is dead — root's own fixpoint derives (false)   ← no solution
     unsat core        (note (bad a))
     refuted by        no-bad
```

Step 1 says: **one solution**. Step 3, loading that very solution back, says: **no solution**. That is the issue.
The file that step 2 writes starts with a comment that promises the opposite: *"It loads, and saturating it
derives nothing new"*.

## The program, line by line

```lisp
(item a)

(rule make ()
  :match  (item ?x)
  :assert (note (bad ?x))
  :why    "a note about {?x}")

(rule no-bad ()
  :match  (written (bad ?x))
  :assert (false)
  :why    "the author typed (bad {?x})")
```

- `(item a)` is the only fact the author typed.
- The rule `make` sees `(item a)` and concludes `(note (bad a))`. So **the rule made** a fact with `(bad a)` inside
  it. The author never typed `(bad a)`.
- The rule `no-bad` is a **spell-check on what the author typed**. `(written X)` is a fact the loader adds by
  itself for every fact `X` that the author typed *inside* another fact. So `no-bad` reads: *if the author typed a
  `(bad …)` inside some fact, stop: the program is wrong.*

## Why step 1 says "one solution"

`(bad a)` exists only inside `(note (bad a))`, and that fact was **made by a rule**, not typed. The loader adds
`(written …)` only for what was typed. So there is no `(written (bad a))`, the spell-check has nothing to object
to, and the program has a solution. Everything the solution holds:

```text
(item a)
(note (bad a))        ← made by the rule make
```

This part is intended and was ruled earlier: the check is on the *text*, not on what rules compute (it is how the
standard library says *"this is not a number"* about a number the author mistyped, without checking every
intermediate number the calculator builds).

## What step 2 saves

A saved solution is an ordinary program file: the original program, followed by **every fact of the solution,
typed out**. The end of `build/q392/models/01.ein`:

```lisp
;; ── the state ──────────────────────────────────────────────────────────

(item a)
(note (bad a)) ; rule make: a note about a
```

Look at the last line. `(note (bad a))` is now **typed in a file**. The words *rule make* after the `;` are a
comment; the loader does not read comments.

## Why step 3 says "no solution"

Step 3 loads that file like any program. The loader sees `(note (bad a))` typed in the file, so, by its own rule,
it adds `(written (bad a))`. Now the spell-check fires:

```text
(item a)
(note (bad a))                       ← this time: typed in the file
(written (bad a))                    ← so the loader adds this  (it was not there in step 1)
(false)                              ← so no-bad fires: no solution
```

**Nothing about the solution changed. What changed is who appears to have typed `(note (bad a))`**: in step 1 a
rule made it; in the saved file it looks like the author typed it. The file forgot where the fact came from.

The same thing with no saved file, to show it is only about *typed or made*: a program in which the author types
`(note (bad a))` directly has no solution, the same answer as step 3:

```lisp
(item a)
(note (bad a))
(rule no-bad () :match (written (bad ?x)) :assert (false) :why "typed")
```

```text
  states (k_s)      0
  unsat core        (note (bad a))
  refuted by        no-bad
```

## In an everyday picture

A customs officer inspects **only what you bring in your luggage**, not what you make at home. You make a thing at
home (the rule makes `(note (bad a))`): nobody inspects it, fine. Then you pack **everything you own** into
suitcases (the saved file), travel out and come back in (load the file). Now the thing you made at home is in your
luggage, and the officer inspects it and rejects it. Same thing, same you; what changed is that it now arrives as
luggage.

## Who meets this

Only a program in which **all three** are true:

1. a rule makes a fact that has another fact inside it, put together from a variable (`(note (bad ?x))`);
2. the program has a check on typed text, a rule over `written` (in practice: it imports the standard library's
   numbers, which check the numerals an author types);
3. the thing the rule made is one the check would reject if typed.

The realistic case: a rule assembles a malformed number, e.g. `(reading (n z ?d))` with `?d` bound to a letter.
The run is fine (assembled numbers are not checked), the saved file reloads as *"no solution: not a decimal
number"*.

**Not affected:** the answer `ein solve` prints (steps 1 and 2 are right); programs with no rule over `written`;
programs whose rules only assemble well-formed things. No program in the repository's examples or tests is known
to meet it.

## Where `written` comes from, why, and what for

**Where from.** Nobody types `(written …)`, and a program that tries is refused. **The loader makes it**, the part
of `ein` that reads a program file: for every fact the file's text has *inside* another fact, the loader adds one
`(written THAT-FACT)`. One line shows it:

```lisp
(price Apple (n (n z 4) 2))
```

```text
$ ein saturate --dump …        (what is held after loading)
  (price Apple (n (n z 4) 2))
  (written (n (n z 4) 2) :rule "#written")       ← added by the loader
  (written (n z 4) :rule "#written")             ← added by the loader
```

**Why it exists.** It was built for one ruling of yours, of 2026-11-14 (Q-M1f.210 (c)): *"if decimal number
contains letters, signs, whitespaces and hieroglyphs - it is not a decimal number from the beginning. It is a
string, cypher, whatever else but not a decimal number."* A mistyped number is the number module's problem, and is
to be flagged **where it is written**, with no calculation started.

The difficulty in building that is the very subject of this phase. A number in Ein is a nested fact: 42 is
`(n (n z 4) 2)`, and it only ever stands inside something else, `(price Apple (n (n z 4) 2))`. A fact inside
another is **in that fact's context and not in the global one, and rules are global**: a rule of the number module
sees a number only if it writes the wrapping, `(price ?x (n ?p ?a))`. The module cannot write the wrapping: it
does not know that your program has a relation `price`, or how deep the number sits. Before `written`, a mistyped
number that nobody computed with was simply never seen.

**What it is, in this phase's words: `written` is a context that the loader makes.** The loader takes every fact
the text nests, wherever it is, and puts it into one context that everybody knows the name of: `(written …)`. Then
the number module writes its check wrapped in *that* context, once:

```lisp
(rule … :match (and (written (n ?p ?a)) …) :assert (not-decimal …))     ; std.math.decimal
```

and it sees every numeral the program's text writes, in any relation, at any depth. It is the loader doing for a
library what a library's rule cannot do for itself.

**What for, exactly.** Twenty rules of the standard library read it, and nothing else in the repository does:

| module | rules over `written` | what they flag |
|---|---|---|
| `std.math.decimal` | 4 | a typed decimal numeral that is not one: a letter for a digit, a wrong prefix |
| `std.math.binary` | 4 | the same for a binary numeral |
| `std.math.int` | 6 | a typed integer whose parts are not what an integer has |
| `std.math.rat` | 6 | a typed rational whose parts are not what a rational has |

(`grep -n '(written' stdlib/math/*.ein` lists them.) It decides nothing else: no search, no answer, no other rule.

**Why it reads only what was typed, and not what rules build.** Also ruled then (Q-M1f.275, Q-M1f.280): the check
is of *what the author wrote*. A number a rule assembles has no text to check, and the calculator builds a nested
fact at every step, so checking those would cost a check a step and would make `written` differ from state to
state of a search. So `written` is decided once, when the program is read.

**What there would be instead of it.** Two other ways to meet the same ruling, neither built:

- *check where a number is used*: the arithmetic checks its operands. That was the state before, and it misses
  every number nobody computes with;
- *a pattern that matches a fact anywhere inside any fact*: a new kind of premise, which would let a library write
  the check with no help from the loader. The language has no such premise.

So the bug of this page is not in what `written` is for. It is that **a saved solution is a text, and the loader
cannot tell the lines the author typed from the lines the save wrote**.

## The fix, and the unit test

**The unit test** is the three commands of this page, as a test: solve [`q392_demo.ein`](q392_demo.ein), save the
answer, load the saved state, and require **one solution holding the same facts**. It fails on today's engine
(step 3 says *no solution*) and passes with the fix. A second case in the same test does it with the standard
library's number instead of the made-up `no-bad`: a rule assembles `(reading (n z ?d))` from `(digit-read x)`. It
goes beside the test that already reloads every corpus program's saved states, `ein-cli/tests/answer_round_trip.rs`,
which did not catch this because no corpus program builds, by a rule, something a check on typed text rejects.

**The fix proposed: a saved line says where it came from, and the loader believes it.**

- Saving writes each fact of the solution that a rule concluded with the clause the language already has for it,
  `(note (bad a) :rule make)`, and each fact the solution *supposed* (a hypothesis it committed to) with a mark of
  the same kind. Today the save writes that origin as a comment, which is lost.
- Loading adds `written` only for facts that carry no such clause, that is, for what an author typed. A fact that
  says a rule concluded it is not typed text, so it is not put into the `written` context.

The program's own text is in the saved file too, above the solution, so everything the author typed is still
checked on reload exactly as it was.

| | before | after |
|---|---|---|
| step 3 on the demo | no solution | **one solution, the same facts** |
| a program with no rule over `written` | unchanged | unchanged |
| a program an author types with a bad numeral | flagged | flagged |
| an author who types `:rule x` on a fact by hand | the clause is ignored | that fact's inside is not checked: the author has said a rule made it |

The last row is the one thing to accept: the clause is taken on trust. The module's own page already says *"a flag
means malformed; no flag does not mean well-formed"*, and a fact a rule really makes is not checked either.

**What it touches**: the code that saves a state, the loader's `written` step, the page of the saved answer
(`10-answer-dump.md`) with the file's first comment, and the two decisions that say what `written` is stated for
(Q-M1f.275, Q-M1f.279). It is S1f.22.3's first task.

## The options as first put

Kept for the record; the ruling above takes the first row's aim, *the promise holds, always*.

| | what changes | step 3 on the demo then says | cost |
|---|---|---|---|
| **(a) the saved file remembers what was made by a rule** | the last line is saved as `(note (bad a) :rule make)`, and the loader adds no `written` for a fact marked so | **one solution** — the promise holds, always | a change to the code that saves and the code that loads; someone could hand-write `:rule x` on a typed fact to dodge a check |
| **(b) the check applies to made facts too** | `written` is added for facts rules make as well | *no solution* — but so does **step 1**: the program itself is now rejected | reverses an earlier ruling; the calculator would check every intermediate number it builds; changes the answers of some programs |
| **(c) leave it, and say so** | nothing in the engine; the file's first comment and two pages say *"…except a program that checks typed text and makes, by a rule, something that check rejects"* | *no solution*, as today, but documented | three sentences |
