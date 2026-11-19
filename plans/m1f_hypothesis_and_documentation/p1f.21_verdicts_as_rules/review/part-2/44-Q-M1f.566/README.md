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

## 4. Where (b) and (c) count differently: two rules that ask one question

[`one-question-twice.ein`](programs/one-question-twice.ein) activates two obligations that ask the same of each
guest, a chair: `std.algebra`'s `total-owed` and a rule of its own, `seated-owed`. Nothing is seated. At root
(`runs/counts.md`, `runs/one-question-twice.solve.txt`):

| | count | what is held |
|---|---:|---|
| the kernel today, (a) | **4** | *seats owes Ann a Chair*, *seats owes Bob a Chair*, *Ann is not seated*, *Bob is not seated* |
| (b) | **2** | `(owe seats Ann Chair)` `(owe seats Bob Chair)`, each concluded by both rules and held once |
| (c) | **4** | two `(debt-due seats (total-owed-of …))` and two `(debt-due seats (seated-owed-of …))` |

So (c) is exact to today's count, and (b) counts questions: the read-out's *root owes 4 instances* would read 2.
$R$ does not depend on it, since one witness answers both rules.

## 5. What no option stores: *still owed*

*Required* only grows as a commitment grows, and so does *answered*. Their difference shrinks, so it is no held
fact: it is read off a state at its fixpoint, as the kernel's pass does today over rule instances, and as § 3's
table does over facts.

`std.debts` does store the end of it, `(debts-discharged)`. On this program it is held exactly where the kernel
owes nothing, 16 of 16 (`runs/counts.md`). It goes stale in a program where a commitment adds a requirement
([V-28](../../../findings.md#v-28--stddebts-says-debts-discharged-is-kept-by-every-larger-state)).

## 6. What each option takes

| | what changes |
|---|---|
| (a) | the verdict's word, and nothing else: no fact, no count, no read-out row |
| (b) | an obligation states its question; the kernel's pass reads held `owe` facts in place of rule instances; the count is of questions (§ 4). **Which argument is the hole is not in the fact**: `(owe seats Ann Chair)` and `(owe seats Guest C1)` are told apart by the rule that emitted each, which is what you said the engine can keep for itself. A program in which one name is both a member and a kind would need it |
| (c) | an obligation states its instance; every count as today. The term names the rule and its bindings, so a requirement reads as its rule's internals, and the same requirement stated by two rules is two facts |
| (d) | a node that holds a variable, and a held fact matched as a pattern: a later milestone's, rules as data |

## 7. The default, and why

**(b).** It is the question, as you put it; it runs today with no new kind of node; it reads as the rule's `:why`
reads, *seats owes Ann a Chair*; and one question asked twice is one requirement, which is what counting
requirements means. (c) is the choice if the count must stay today's, instance by instance.

## 8. Files

- [`programs/`](programs/): the five programs above.
- `runs/`: for each program its read-out (`.solve.txt`, `.solve-p.txt`), `ein show hyps`, `ein saturate --dump`,
  the oracle's read-out and its dump with every node's state (`.oracle/`), the events and the summary, the answer
  and the lattice as files, and the knowledge base drawn (`.kb.dot`, `.kb.svg`); `a-seats.owe-events.jsonl`, root's
  four `owe` events; `counts.md`; the four diffs of § 3.
- `tools/run.sh`, `tools/counts.py`.

## 9. Your call

(a), (b), (c) or (d), in [`ANSWERS.md`](../ANSWERS.md) under item 44. Blank keeps (b).
