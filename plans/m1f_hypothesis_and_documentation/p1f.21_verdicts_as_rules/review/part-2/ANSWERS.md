# Your answers — part 2: `open`, what a state owes, and the hypotheses

[← the part](README.md) · [← the bundle](../README.md) · [← the phase](../../README.md)

Your answers of 2026-11-19 to items 44, 45 and 46 are filed in your words in
[the phase's README](../../README.md#the-answers-to-items-44-45-and-46) and in the ledger. Five questions came out
of them. Write your answer after **Your answer:** — a letter for each numbered question, *default*, or your own
words. An item left blank keeps its defaults. This file is yours.

## 47 · Q-M1f.569: what may read an `open` fact, and when

[The item](47-Q-M1f.569/README.md)

A fork drops the `open` facts and keeps what a rule concluded from them.

1. **May a rule's `:match` read an `open` fact?**
   - **(a)** Not in this phase: the kernel and the read-out read them.
   - **(b)** Yes. They are written by a pass at each state's fixpoint, and the rules that read them run after it. A
     program whose rule reads `(open R)`, an obligation's candidate, or the absence of either is flagged as not
     monotone. **Default.**
   - **(c)** Yes, and at the fork the kernel also drops what was concluded from a dropped fact.
2. **A written `(open P)` at a fork.**
   - **(a)** It is there in every state. **Default.**
   - **(b)** It is dropped with the rest.
3. **Every rule that concludes `(open …)` is read at every state**, a hypothesis rule among them; $H$ stays
   root's. **Default: so.**

**Your answer:**

**Notes:**

---

## 48 · Q-M1f.570: a rule that concludes `(open F)` beside an obligation

[The item](48-Q-M1f.570/README.md)

- **(a)** Override, as a hypothesis rule does today: where a rule concludes `(open F)` at root, the obligations
  count and write no `(open F)`. No answer moves. **Default.**
- **(b)** Union: $H$ is every `(open F)` root holds. `14_seats_nothing_proposes.ein` goes from 4 states to 8,
  `13_witness_of_two.ein` from 5 to 8.
- **(c)** Union, and a flag of `(solve …)` turns the obligations' proposals off.

**Your answer:**

**Notes:**

---

## 49 · Q-M1f.571: what `absent` and `open` take, and what each is under an ordinary relation

[The item](49-Q-M1f.571/README.md)

1. **`absent`'s argument.**
   - **(a)** A premise: a pattern, or a conjunction, which may hold an `absent` (the universal). Only the direct
     `(absent (absent P))` is refused. **Default.**
   - **(b)** No `absent` anywhere inside an `absent`: `forall` and `std.debts`' rule go.
2. **`(open (open P))`.**
   - **(a)** Refused: directly under `open`, neither kernel word is written. **Default, your verdict.**
   - **(b)** Legal, `(open P)` being a fact.
3. **Under an ordinary relation**, as in `(think Ann (absent (p A)))`.
   - **(a)** Both `open` and `absent` are ordinary names there, plain facts. **Default, your word in item 44.**
   - **(b)** `open` is a plain fact there, and `absent` stays refused as an argument, as today.
4. **The rule the defaults give**, [§ 4 of the item](49-Q-M1f.571/README.md#4-the-rule-the-defaults-give): is it
   the rule? **Default: yes.**

**Your answer:**

**Notes:**

---

## 50 · Q-M1f.572: whether a state's `open` facts are shown and counted with its facts

[The item](50-Q-M1f.572/README.md)

- **(a)** Like any fact: listed, drawn and counted with a state's facts.
- **(b)** Held, and shown apart: a state's listing, its counts and its identity are of its other facts; a saved file
  leaves them out. **Default.**
- **(c)** Held, and shown only by `ein show hyps` and to a rule that reads them.

**Your answer:**

**Notes:**

---

## 51 · Q-M1f.573: `open` as an obligation or as a hypothesis, and which hypotheses are candidates

[The item](51-Q-M1f.573/README.md)

1. **A variable under `open`.**
   - **(a)** Read at load by what it is: a parameter that names a relation is the obligation's; a variable the
     `:match` binds is a hypothesis, its value a fact. **Default.**
   - **(b)** A hypothesis is always written with its fact, `(open (?R ?x))`.
2. **A hypothesis root has already decided.**
   - **(a)** `(open F)` is held, and F is no candidate where root holds F or its negation, as today. **Default.**
   - **(b)** Every `(open F)` root holds is a candidate.
3. **$H$ is root's `open` facts**: one first concluded above root is held and not proposed. **Default: so.**

**Your answer:**

**Notes:**
