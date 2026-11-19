# Your answers — part 2: `open`, what a state owes, and the hypotheses

[← the part](README.md) · [← the bundle](../README.md) · [← the phase](../../README.md)

Your answers of 2026-11-19 to items 32, 34 and 39 are filed in your words in
[the phase's README](../../README.md#the-answers-to-part-2s-items) and in the ledger. Three questions came out of
them. Write your answer after **Your answer:** — a letter, *default*, or your own words. An item left blank keeps
its default. This file is yours.

## 44 · Q-M1f.566: how the *must* is emitted

[The analysis](../../open_and_owe.md#2-how-a-requirement-is-counted-and-how-it-could-be-emitted)

**Options:**

- **(a)** as today: the requirement is no fact, the kernel counts a rule's instances; only the verdict's word
  changes, to `owe`.
- **(b)** a held fact, the question with the hole's domain in its place: `(owe seats Ann Chair)`. Runs today as
  ordinary rules. Two rules that ask one question owe once. **Default.**
- **(c)** a held fact, the instance as a term: `(owe (total-owed seats is-a Guest Chair Ann))`. Runs today, and is
  how `std.debts` writes it. Exact to today's count.
- **(d)** your form, a hole in the fact: `(owe (seats Ann ?_x))`. Refused today; a third kind of node, and a held
  fact read as a pattern.

In (b), (c) and (d) *discharged* is read at the fixpoint and never stored: *still owed* shrinks.

**Your answer:**

**Notes:**

---

## 45 · Q-M1f.567: the `hrule` form dropped

[The analysis](../../open_and_owe.md#4-can-the-hrule-form-go)

**Options:**

- **(a)** the form stays, as you ruled of Q-M1f.561.
- **(b)** the form goes, a rule concludes `(open F)`; its activators stay in `(solve :hrules …)`. **Default**, your
  question as you put it.
- **(c)** the form goes and the clause with it: an activator is an ordinary fact, `(guess chosen Drink)`.

With (b) or (c): $H$ stays root's `open` facts; an `(open F)` first concluded above root is not proposed.

**Your answer:**

**Notes:**

---

## 46 · Q-M1f.568: four cells of the table

[The table](../../absent_open_not.md#6-the-table-those-two-rules-give-once-open-p-is-a-fact)

**Defaults**; write the number of any you want otherwise:

1. `(absent (absent P))` written directly is refused at the parse; with a conjunction between, the universal, it
   stays. Today it loads and means *P is held*.
2. `(open (open P))` is refused at load, until dependent hypotheses are taken up.
3. `(not (open P))` and `(open (not P))` are legal; the kernel reads nothing in the first.
4. `(open P)` may be written by a program as a fact, a candidate given by hand.

**Your answer:**

**Notes:**
