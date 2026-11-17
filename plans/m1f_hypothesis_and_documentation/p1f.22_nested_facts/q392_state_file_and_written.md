# Q-M1f.392, from the start — saving an answer and loading it back gives a different answer

**Phase:** [P1f.22](README.md). Second version, 2026-11-17, on the user's word (*"I still do not understand, show
an issue, how to reproduce it, explain for dummies"*). The question is
[Q-M1f.392](../open_questions.md#q-m1f392--a-state-file-that-reads-back-dead-through-written); it is **not ruled**.
The outputs are runs of `ein.rs/target/gate/ein` taken for this page.

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

## What can be done about it

| | what changes | step 3 on the demo then says | cost |
|---|---|---|---|
| **(a) the saved file remembers what was made by a rule** | the last line is saved as `(note (bad a) :rule make)`, and the loader adds no `written` for a fact marked so | **one solution** — the promise holds, always | a change to the code that saves and the code that loads; someone could hand-write `:rule x` on a typed fact to dodge a check |
| **(b) the check applies to made facts too** | `written` is added for facts rules make as well | *no solution* — but so does **step 1**: the program itself is now rejected | reverses an earlier ruling; the calculator would check every intermediate number it builds; changes the answers of some programs |
| **(c) leave it, and say so** | nothing in the engine; the file's first comment and two pages say *"…except a program that checks typed text and makes, by a rule, something that check rejects"* | *no solution*, as today, but documented | three sentences |

## The one question

**Should a saved solution always load back as that solution?**

- **Yes, always** → (a).
- **It is fine to list this as a known exception** → (c). (There is already one such exception: a solution nested
  deeper than the parser's limit of parentheses does not load back either.)

The default I took was (c), because no real program hits it. If you would rather the promise had no exception, say
(a); it is a small build.
