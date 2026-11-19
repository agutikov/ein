# Your answers — part 2: `open`, what a state owes, and the hypotheses

[← the part](README.md) · [← the bundle](../README.md) · [← the phase](../../README.md)

Your answers of 2026-11-20 to items 47 to 51 are filed in your words in
[the phase's README](../../README.md#the-answers-to-items-47-to-51) and in the ledger. Four questions came out of
them. Write your answer after **Your answer:** — a letter for each numbered question, *default*, or your own
words. An item left blank keeps its defaults. This file is yours.

## 52 · Q-M1f.574: a hypothesis first held above root: how the program is found not monotone

[The item](52-Q-M1f.574/README.md)

Your *"right?"*: by your definition yes. Today's static check calls such a program monotone: a rule with no
`absent` that concludes a hypothesis from a fact only a commitment brings is no site of it.

1. **How it is detected.**
   - **(a)** Statically: a rule that concludes `(open F)` and reads a relation a commitment can change is a site
     of the static check, as an obligation's trigger is today.
   - **(b)** In flight: a state the run builds that holds an `(open F)` root does not.
   - **(c)** Both. **Default.**
2. **The run's answer, once found in flight.**
   - **(a)** As computed, under the WARNING, nothing claimed. **Default.**
   - **(b)** Computed again by the cube.

**Your answer:**

**Notes:**

---

## 53 · Q-M1f.575: what one `:assert` holds beside `(open R)` and beside `(open F)`

[The item](53-Q-M1f.575/README.md)

Your sentence: *"It can assert (open R) - and then it works as before, plus it cant assert (open (fact)) with
whatever other fact, in the same assert."* *cant* reads as *can* and as *can't*.

- **(a)** `(open R)` is a rule's whole `:assert`, as today. `(open F)` is a conclusion like any fact: it stands in
  a conjunction with facts and with other `(open F)`. **Default.**
- **(b)** Both stand in any conjunction: one rule concludes `(open R)`, `(open F)` and facts together.
- **(c)** Each is a rule's whole `:assert`.

**Your answer:**

**Notes:**

---

## 54 · Q-M1f.576: the union's edges

[The item](54-Q-M1f.576/README.md)

1. **An obligation whose candidates cannot be read** (its guard or trigger reads what a commitment can change),
   **beside a rule that concludes `(open F)`.**
   - **(a)** It writes no candidate and calls no one; the rule's candidates stand. **Default.**
   - **(b)** It calls the blind enumerator, as it does alone: `06_friends_tables.ein` goes from 19 candidates to
     923 or more.
2. **The blind enumerator.**
   - **(a)** It writes its guesses only where no rule concludes `(open F)` at root and no obligation is activated,
     as today. **Default.**
   - **(b)** Always.
3. **The two lists of `(solve …)` and a closure** bind every writer as they bind today. **Default: so.**

**Your answer:**

**Notes:**

---

## 55 · Q-M1f.577: a view of a state without its `open` facts

[The item](55-Q-M1f.577/README.md)

- **(a)** `-P` leaves out the `(open …)` facts as it leaves out the `(not …)`; `-p` lists everything. No new flag.
  **Default.**
- **(b)** A flag of its own, on `ein solve`, `ein saturate --dump` and the drawing of a knowledge base.
- **(c)** Nothing now.

**Your answer:**

**Notes:**
