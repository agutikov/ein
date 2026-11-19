# 44 · Q-M1f.566: how the *must* is emitted, beside the hypotheses' `(open H)`

[← part 2](../README.md) · [← the phase](../../../README.md) · [your answers](../ANSWERS.md) ·
[the ledger](../../../../open_questions.md#q-m1f566--how-a-requirement-is-emitted-the-must-beside-open) ·
[the analysis](../../../open_and_owe.md#2-how-a-requirement-is-counted-and-how-it-could-be-emitted)

**Raised** by your answer to item 34, 2026-11-19: *"Obligations MUST, hypotheses MAY be. As I understand now our
obligation rules emit facts that are hypotheses and obligation options at the same time. How we can count the exact
obligations and how we can define and emit them? Maybe in form of questions? How to define them? Maybe something
like (owe (?R ?a ?_x)) where ?R and ?a binded and ?_x not - so the emitted thing is actually a query."*

**Not ruled. The default is (b).** Nothing is built: every run below is today's engine, the gate build of master at
`edbe6c3b` (`build/review-bin/ein-head-edbe6c3b`), on the programs in [`programs/`](programs/). `tools/run.sh`
re-takes them all into `runs/` (both ignored, so on this machine only); a path below written as code is a file
there.

**On your word, 2026-11-19** (*"show more examples of obligation rules from existing ein code"*): § 4 counts every
obligation rule the corpus writes and shows eleven of them, each with its requirement as each option writes it;
§ 5 is what they show that the seats program does not.

## 1. The question

An obligation rule today says the *must* and the *may* in one conclusion, `(open ?R)`, and writes neither: the
kernel reads both off the rule's match. You ruled that `(open H)` is the *may*, a hypothesis. This asks how the
*must* is written: the requirement, of which a state is discharged when every one is met.

## 2. The four options, each on the seats program

Two guests, two chairs; each guest is owed a chair and each chair a guest
(`examples/lattice/05_owed_seats_bounded.ein`).

| | what a state holds of a requirement | the program | today |
|---|---|---|---|
| **(a)** | nothing. The kernel counts the rule's instances with no witness; only the verdict's word changes, to `owe` | [`a-seats.ein`](programs/a-seats.ein) | runs: it is today |
| **(b)** | the question, the hole's domain in its place: `(owe seats Ann Chair)`, `(owe seats Guest C1)` | [`b-seats-owe.ein`](programs/b-seats-owe.ein) | runs, as four ordinary rules |
| **(c)** | the rule's instance as a term: `(debt-due seats (total-owed-of seats is-a Guest Chair Ann))` | [`c-seats-debts.ein`](programs/c-seats-debts.ein) | runs: it is `std.debts`, one import and one fact |
| **(d)** | your form, a hole in the fact: `(owe (seats Ann ?_x))` | [`d-owe-hole.ein`](programs/d-owe-hole.ein) | **refused at load** |

What root's saturation derives (`runs/b-seats-owe.saturate-dump.txt`, `runs/c-seats-debts.saturate-dump.txt`):

```lisp
;; (b)                          ;; (c)
(owe seats Ann Chair)           (debt-due seats (total-owed-of seats is-a Guest Chair Ann))
(owe seats Bob Chair)           (debt-due seats (total-owed-of seats is-a Guest Chair Bob))
(owe seats Guest C1)            (debt-due seats (surjective-owed-of seats is-a Guest Chair C1))
(owe seats Guest C2)            (debt-due seats (surjective-owed-of seats is-a Guest Chair C2))
```

and (d), `runs/d-owe-hole.solve.txt`:

> kb load error: rule 'need': its :assert writes ?_x, which no premise or parameter binds — to conclude it of the
> objects a relation holds of, bind it with a premise such as (R ?_x), or of every name of the program with
> (eq ?_x ?_x)

A fact holds no variable: an object is an atom or a fact
([Q-M1f.379](../../../../../../docs/decisions/language/Q-M1f.379-an-object-is-a-node-of-the-knowledge-base-graph.md)).
(d) asks for a third kind of node, and for a held fact read as a pattern, which is rules as data.

## 3. The same answer, and the same count at every node

In (b) and (c) the kernel's own obligations are kept beside the written facts, so both are read on the same
states. **$R$ is the same in the three programs**: the read-outs differ in the count of facts beyond root, and (b)'s
adds the `outside` row for its two relations (`runs/a-seats.solve.vs-b-seats-owe.diff`,
`runs/a-seats.solve.vs-c-seats-debts.diff`; the oracle's the same way).

At every node of the cube, the requirements held with no answer held are what the kernel owes (`runs/counts.md`,
written by `tools/counts.py` from `ein oracle --dump DIR --states`):

| layer | commitment | status | kernel `owes` | (b) `owe` · `answered` · unanswered | (c) `debt-due` · `debt-paid` · unpaid |
|---:|---|---|---:|---|---|
| 0 | ∅ | owing | 4 | 4 · 0 · 4 | 4 · 0 · 4 |
| 1 | each of the four single seatings | owing | 2 | 4 · 2 · 2 | 4 · 2 · 2 |
| 2 | the two matchings | **R** | 0 | 4 · 4 · 0 | 4 · 4 · 0 |
| 2 | the four clashing pairs | dead+owing | 1 | 4 · 3 · 1 | 4 · 3 · 1 |
| 3, 4 | the five larger sets | dead | 0 | 4 · 4 · 0 | 4 · 4 · 0 |

**16 of 16 nodes** for (b), and 16 of 16 for (c); the file has the sixteen rows.

## 4. The obligation rules existing code writes

**25 distinct obligation rules, in 27 files** of `stdlib/`, `examples/` and `tests/`; 20 of them are in a file that
loads, 5 in a witness of a refusal. `runs/obligation-rules.md` has every one, its text and its files
(`tools/census.py`). Of the 20:

| | rules |
|---|---:|
| the rule takes its relation as a parameter, `(open ?R)` | 6 |
| the rule names its relation, `(open likes)` | 13 |
| the rule names none, `(open)` | 1 |
| inside its `absent`, a guard on the hole and the fact, `(absent (and (is-a ?b Chair) (seats ?a ?b)))` | 18 |
| inside its `absent`, the fact alone, `(absent (?R ?a ?b))` | 2 |

Eleven of them, on programs. Each of the seven `e*.ein` is a corpus program, whole and as it is, with a block after it
that states the same requirement as (b) writes it and as (c) writes it, the kernel's own obligation kept
(`tools/make_examples.py`). The (d) column is your form with the hole alone, as it would be written: it runs
nowhere.

| rule | from | one instance, as the kernel says it | (b) | (c) | (d), the hole alone |
|---|---|---|---|---|---|
| `total-owed` | `stdlib/algebra.ein`; § 2 | *seats owes Ann a Chair.* | `(owe seats Ann Chair)` | `(debt-due seats (total-owed-of seats is-a Guest Chair Ann))` | `(owe (seats Ann ?_b))` |
| `surjective-owed` | `stdlib/algebra.ein`; § 2 | *seats owes C1 a preimage in Guest.* | `(owe seats Guest C1)` | `(… (surjective-owed-of seats is-a Guest Chair C1))` | `(owe (seats ?_a C1))` |
| `slot-owed-room` | `stdlib/slots.ein`; [`e2`](programs/e2-slot-owed-room.ein) | *Bob is in no slot yet — some Seat is still owed.* | `(owe co-loc Bob Seat)` | `(… (slot-owed-room-of co-loc instance type Attribute Seat Bob Who))` | `(owe (co-loc Bob ?_i))` |
| `slot-owed-fill` | `stdlib/slots.ein`; [`e3`](programs/e3-slot-owed-fill.ein) | *slot S2 has no Who yet*, and *slot S2 has no Pet yet* | `(owe co-loc S2 Who)` `(owe co-loc S2 Pet)` | two terms, `… S2 Who)` and `… S2 Pet)` | `(owe (co-loc S2 ?_v))`, **one fact for the two** |
| `owes-food` | six files of `examples/ein-bugs/wrong-answers/`; [`e4`](programs/e4-owes-food.ein) | *Ann owes a liked food* | `(owe likes Ann food)` | `(debt-due likes (owes-food-of Ann))` | `(owe (likes Ann ?_b))` |
| `reading-owed` | `examples/word_problems/readings.ein`, a module; read in `boxes13.ein` | *a reading of need is owed: the question asks how many boxes are needed for […]* | `(owe reads-as need reading-of)`, not run | `(debt-due reads-as (reading-owed-of boxes apples (together Bob Alice apples now)))`, not run | `(owe (reads-as need ?_m))` |
| `group-seated-owed` | `examples/lattice/06_friends_tables.ein`; [`e6`](programs/e6-group-seated-owed.ein) | *Cat's group Trio sits, and Eve has no seat* | `(owe sits Eve Seat)` | `(debt-due sits (group-seated-owed-of Cat B1 Trio Eve))` | `(owe (sits Eve ?_t))` |
| `owe-1`, `owe-2` | `examples/lattice/synth/local_not_global.ein`, and eight more rules of those names; [`e7`](programs/e7-owe-1-owe-2.ein) | *with A: one of B is required* | `(owe h K1)` | `(debt-due h (owe-1-of A))` | `(owe (h ?_x))`, **the same fact for both rules** |
| `owed` | `examples/ein-bugs/wrong-answers/kw-or-in-obligation.ein`; [`e8`](programs/e8-owed-no-domain.ein) | *orz holds of a, and a has no tag yet: one is still owed* | `(owe tag a Node)` | `(debt-due tag (owed-of tag Node Node a))` | `(owe (tag a ?_b))`, exact |
| `sorted-somewhere` | `tests/stdlib/bijection/06_blind_enumeration.ein`; [`e9`](programs/e9-sorted-somewhere.ein) | owes nothing there: each guest is of a declared sort | `(owe is-a Ann Sortal)` | `(debt-due is-a (sorted-somewhere-of Ann))` | `(owe (is-a Ann ?_k))` |

The rules, as their files write them:

```lisp
;; stdlib/slots.ein
(rule slot-owed-room (?R ?isa ?sub ?super ?index)
  :match  (and (?isa ?a ?Ta) (?sub ?Ta ?super) (neq ?Ta ?index)
               (absent (and (?isa ?i ?index) (?R ?a ?i))))
  :assert (open ?R)
  :why    "{?a} is in no slot yet — some {?index} is still owed.")
(rule slot-owed-fill (?R ?isa ?sub ?super ?index)
  :match  (and (?isa ?i ?index) (?sub ?Tv ?super) (neq ?Tv ?index)
               (absent (and (?isa ?v ?Tv) (?R ?i ?v))))
  :assert (open ?R)
  :why    "slot {?i} has no {?Tv} yet — one is still owed.")

;; examples/ein-bugs/wrong-answers/oblig-derived-domain-declared.ein: the domain is a relation, `food`
(rule owes-food ()
  :match  (and (person ?a)
               (absent (and (food ?b) (likes ?a ?b))))
  :assert (open likes)
  :why    "{?a} owes a liked food")

;; examples/word_problems/readings.ein: the trigger is a fact inside a fact, the domain a relation with a constant
(rule reading-owed ()
  :match  (and (wanted (needed ?c ?k ?q))
               (absent (and (reading-of need ?m) (reads-as need ?m))))
  :assert (open reads-as)
  :why    "a reading of need is owed: the question asks how many {?c} are needed for [{?q}], and the problem does not say what need means")

;; examples/lattice/06_friends_tables.ein: required only once a friend of the group sits
(rule group-seated-owed ()
  :match  (and (sits ?a ?s) (member ?a ?g) (member ?b ?g)
               (absent (and (is-a ?t Seat) (sits ?b ?t))))
  :assert (open sits)
  :why    "{?a}'s group {?g} sits, and {?b} has no seat")

;; examples/lattice/synth/local_not_global.ein: required only with a commitment, of a relation of one argument
(rule owe-1 ()
  :match  (and (h A) (absent (and (need K1 ?x) (h ?x))))
  :assert (open h)
  :why    "with A: one of B is required")

;; examples/ein-bugs/wrong-answers/kw-or-in-obligation.ein: no guard on the hole
(rule owed (?R)
  :match  (and (relation ?R ?A ?B) (orz ?a) (absent (?R ?a ?b)))
  :assert (open ?R) :why "orz holds of {?a}, and {?a} has no {?R} yet: one is still owed")

;; tests/stdlib/bijection/06_blind_enumeration.ein: no relation named, and the hole is a kind
(rule sorted-somewhere ()
  :match  (and (is-a ?x Guest)
               (absent (and (sort ?k Sortal) (is-a ?x ?k))))
  :assert (open)
  :why    "{?x} belongs to no declared sort.")
```

**The counts, on runs** (`runs/examples.md`, written by `tools/examples.py`; the kernel's `owes` is asserted the
same in the corpus program and in the example). A node is every node of the cube where the cube is small, else the
program with a commitment's facts written after it, read under `--no-search`.

| program | nodes not dead | the kernel's `owes` there | (b)'s unanswered agrees | (c)'s unpaid agrees |
|---|---:|---|---:|---:|
| `e2-slot-owed-room` | 1, root | 3 | 1 | 1 |
| `e3-slot-owed-fill` | 1, root | 3 | 1 | 1 |
| `e4-owes-food` | 2 of the cube's 64 | 1 at ∅, 0 at `{(likes Ann Pie)}` | 2 | 2 |
| `e6-group-seated-owed` | 5 written commitments | 0, 1, 2, 2, 0 | **4** | 5 |
| `e7-owe-1-owe-2` | 4, the cube | 0, 1, 1, 0 | 4 | 4 |
| `e8-owed-no-domain` | 1, root | 1 | 1 | 1 |
| `e9-sorted-somewhere` | 1, root | 0 | 1 | 1 |
| together | 15 | | **14** | **15** |

`e6` and `e7` are programs the static check flags as not monotone: nothing is claimed of their answer, and
nothing here is about it. What is compared is what one state owes and holds.

## 5. What the examples show that the seats program does not

**1. One rule, several instances, one question: a corpus program where (b) and (c) count differently.** In the
friends' tables, the rule's instance is four bindings, `?a ?s ?g ?b`, and its question is about `?b` alone. With
Cat at B1 and Dan at B2 (`runs/examples.md`):

| | count | what is held |
|---|---:|---|
| the kernel today, (a) | **2** | *Cat's group Trio sits, and Eve has no seat*; *Dan's group Trio sits, and Eve has no seat* |
| (b) | **1** | `(owe sits Eve Seat)` |
| (c) | **2** | `(debt-due sits (group-seated-owed-of Cat B1 Trio Eve))` `(debt-due sits (group-seated-owed-of Dan B2 Trio Eve))` |

So (c) is exact to today's count, and (b) counts questions. [`one-question-twice.ein`](programs/one-question-twice.ein)
is the same with two rules in place of one rule's two instances: the kernel and (c) count 4, (b) counts 2
(`runs/counts.md`). $R$ does not depend on it, since one witness answers every instance of a question.

**2. What stands for the hole in (b) is a name the rule's author picks.** It is a kind for the library's four
rules and for the friends' (`Chair`, `Seat`, `Pet`). In the others it is a relation's name (`food`), a constant of
the guard (`K1`, for *what pays K1*), a kind reached through a second fact (`Sortal`), or, where the hole has no
guard, the label of the declaration (`Node`), which Ein does not check. Two rules ask one question exactly when
their authors give it one name.

**3. In (d) the hole alone is not the requirement.** Of the eleven rules, the fact with a hole says what is required
for one, `owed`, the one with no guard. In the other ten the guard is part of the requirement, and without it two
requirements become one fact: slot S2's *a Who* and *a Pet* are both `(owe (co-loc S2 ?_v))`, and `owe-1`'s *one of
B* and `owe-2`'s *one of A* are both `(owe (h ?_x))`. With the guard kept, what is held is a conjunction with a
hole, `(owe (and (instance ?_v Pet) (co-loc S2 ?_v)))`: a premise, held as a fact.

**4. A requirement can appear with a commitment.** `group-seated-owed` requires nothing at root and a seat for
each friend once one of the group sits; `owe-1` requires only where `(h A)` is held. Held as facts, in (b) and in
(c), these are derived where their trigger is, which the runs above show node by node. Both programs are flagged
today for exactly this: *discharged* at a state does not say *discharged* above it. It is also where a stored
`(debts-discharged)` goes stale (§ 6).

**5. A requirement need not name a relation, or ask anything.** `sorted-somewhere` concludes `(open)`: under (b),
(c) or (d) its author names what it asks about, here `is-a`. And an obligation with no `absent` loads
([`no-absent.ein`](programs/no-absent.ein), a probe; the corpus's one such rule is in a file refused for another
reason): root owes, nothing can pay, and $R$ is empty (`runs/no-absent.solve.txt`). It is a *must* that is no
question: (b) and (d) write a question and it asks none, where (c)'s term, the rule and its bindings, is there
all the same.

## 6. What no option stores: *still owed*

*Required* only grows as a commitment grows, and so does *answered*. Their difference shrinks, so it is no held
fact: it is read off a state at its fixpoint, as the kernel's pass does today over rule instances, and as § 3's
table does over facts.

`std.debts` does store the end of it, `(debts-discharged)`. On the seats program it is held exactly where the
kernel owes nothing, 16 of 16 (`runs/counts.md`). It goes stale in a program where a commitment adds a requirement
([V-28](../../../findings.md#v-28--stddebts-says-debts-discharged-is-kept-by-every-larger-state)), which § 5's
fourth point is.

## 7. What each option takes

| | what changes |
|---|---|
| (a) | the verdict's word, and nothing else: no fact, no count, no read-out row |
| (b) | each obligation states its question, written by its author: 20 rules of the corpus, 4 of them the library's. The kernel's pass reads held `owe` facts in place of rule instances; the count is of questions (§ 5, first point). **Which argument is the hole is not in the fact**: `(owe seats Ann Chair)` and `(owe seats Guest C1)` are told apart by the rule that emitted each, which is what you said the engine can keep for itself |
| (c) | each obligation states its instance; every count as today, and the term can be made by the kernel from the rule, with nothing for an author to pick. The term names the rule and its bindings, so a requirement reads as its rule's internals, and one question asked by two instances is two facts |
| (d) | a node that holds a variable, a held fact matched as a pattern, and for 18 of the 20 rules a conjunction held under `owe` (§ 5, third point): a later milestone's, rules as data |

## 8. The default, and why

**(b).** It is the question, as you put it; it runs today with no new kind of node; it reads as the rule's `:why`
reads, *seats owes Ann a Chair*; and one question asked twice is one requirement, which is what counting
requirements means. (c) is the choice if the count must stay today's, instance by instance, or if no author is to
pick a name: the examples make that case stronger than the seats program did, since outside the library the
hole's domain is seldom a kind.

## 9. Files

- [`programs/`](programs/): the five programs of § 2 and § 5; the seven `e*.ein`, each a corpus program with its
  block; `no-absent.ein`.
- `runs/`: for each program its read-out (`.solve.txt`, `.solve-p.txt` or `.no-search.solve.txt`), `ein show hyps`,
  `ein saturate --dump`, the events and the summary, and the knowledge base drawn (`.kb.dot`, `.kb.svg`); for the
  programs of § 2 the oracle's read-out and its dump with every node's state, the answer and the lattice as files;
  `a-seats.owe-events.jsonl`, root's four `owe` events; `counts.md`; the four diffs of § 3; `examples.md`, with
  every node of § 4's table and what is unanswered and unpaid at each; `nodes/`, each node's program and run;
  `obligation-rules.md`, the 25 rules; `e5-reading-owed.boxes13.*`, the readings rule read in place.
- `tools/run.sh`, `tools/counts.py`, `tools/make_examples.py`, `tools/examples.py`, `tools/census.py`.

## 10. Your call

(a), (b), (c) or (d), in [`ANSWERS.md`](../ANSWERS.md) under item 44. Blank keeps (b).
