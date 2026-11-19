# `open` is *may*, and what is owed is *must*: the user's answer to item 34, question by question

[← the phase](README.md) · [the runs](absent_open_not/run.py) · [the table of `absent`, `open` and `not`](absent_open_not.md)

**The user**, 2026-11-19, answering review item 34 (Q-M1f.549), verbatim:

> 1st (open ...) is a context for hypotheses. Not only obligations. Obligations MUST, hypotheses MAY be. As I
> understand now our obligation rules emit facts that are hypotheses and obligation options at the same time.
> How we can count the exact obligations and how we can define and emit them? Maybe in form of questions? How
> to define them? Maybe something like (owe (?R ?a ?_x)) where ?R and ?a binded and ?_x not - so the emitted
> thing is actually a query. Also a question about emitted open structure: the implied connector between KB
> facts is `and` - all facts are true so ein search is truth maintenance system. What is a connector between
> facts inside (open ...) ? Between new invented (owe ...) it is also `and` because we need to discharge all
> obligations. But what about (open ...) ? Can we say anything about them or not? Probably not - they are
> limited by restriction rules, not by obligation rules. If we will emit (open H) facts as pure hypotheses,
> and obligations would be emitted different way, can we then drop hrules as special form according to
> plans/m1f_hypothesis_and_documentation/p1f.21_verdicts_as_rules/one_way_each.md and use rules that emit open
> facts in :hrules?

## The answer, short

1. **Yes**: an obligation rule today says the *must* and the *may* in one conclusion, and writes neither. Its
   `(open ?R)` names a relation; the requirement and its options are read off the rule's match by the kernel.
2. **A requirement can be a fact, and it is a question**, as you say. What cannot be held is *still owed*: it
   shrinks as a commitment grows, so it is a reading at the fixpoint. Your form `(owe (?R ?a ?_x))` is refused
   today, since a fact holds no variable; two other ways of writing the same question run today.
3. **The connectors**: held facts, *and*; requirements, *and*, each an *or* inside; `open` facts, **none**. You are
   right that nothing is said between them: the rules that refute and the rules that require say all of it.
4. **Yes, the `hrule` form can go**, and it is not the proposal you declined: that one replaced *may* by *must*.
   This keeps *may* and spells it as a rule. Two programs already answer byte for byte the same that way.

Two questions come out of it, [Q-M1f.566](../open_questions.md#q-m1f566--how-a-requirement-is-emitted-the-must-beside-open)
and [Q-M1f.567](../open_questions.md#q-m1f567--the-hrule-form-dropped-for-a-rule-that-concludes-open-f), each
with a default. Each is an item of the review bundle with its options as programs and their runs:
[44](q-m1f.566_how_the_must_is_emitted/README.md) and [45](q-m1f.567_the_hrule_form_dropped/README.md).

## 1. What an obligation rule emits today

```lisp
(rule total-owed (?R ?isa)
  :match  (and (relation ?R ?A ?B)
               (?isa ?a ?A)
               (absent (and (?isa ?b ?B) (?R ?a ?b))))
  :assert (open ?R)
  :why    "{?R} owes {?a} a {?B}.")
```

Activated by `(total-owed seats is-a)`, it says of each guest with no chair that *seats owes Ann a Chair*. Two
things are in it and neither is written:

- **the requirement**, the *must*: one for each binding of what stands outside the `absent`, here `?a`. The kernel
  counts these: an instance is the rule, its activator's arguments and that binding;
- **the options**, the *may*: every fact the pattern inside the `absent` would hold, `(seats Ann C1)`,
  `(seats Ann C2)`. The kernel reads these off the rule and proposes them where nothing else proposes.

So your reading is right: the rule emits hypotheses and requirements at once. `(open ?R)` itself says only *this
relation is not finished*.

## 2. How a requirement is counted, and how it could be emitted

**Today's count** is of instances with no witness at a state's fixpoint. The seats program owes four at root: each
of two guests a chair, each of two chairs a guest ([review item 32](q-m1f.390_what_an_obligation_emits/README.md), § 2).

**Your form**, `(owe (?R ?a ?_x))` with `?_x` left unbound, is refused today:

> kb load error: rule 'need': its :assert writes ?_x, which no premise or parameter binds

A fact holds no variable: an object is an atom or a fact
([Q-M1f.379](../../../docs/decisions/language/Q-M1f.379-an-object-is-a-node-of-the-knowledge-base-graph.md)). There are three
ways to hold the question all the same:

| the way | the fact | what it takes |
|---|---|---|
| **the hole's domain in its place** | `(owe seats Ann Chair)`, which reads as the rule's `:why` does, *seats owes Ann a Chair* | runs today as ordinary rules, below. Which argument is the hole is known from the rule that emitted it, and you said it: *"engine see where it was emitted anyway"* |
| **the instance as a term** | `(owe (total-owed seats is-a Guest Chair Ann))` | runs today, item 32's `seats-emu.ein`. Exact to today's count: two rules that ask one question owe twice |
| **a hole in the fact**, your form | `(owe (seats Ann ?_x))` | a third kind of node beside the atom and the fact, and a held fact read as a pattern, which is rules as data, a later milestone's |

**The first way, run today** (`owe-held` of [the runs](absent_open_not/run.py)): Ann is seated, Bob is not.

```lisp
(rule need (?R ?A ?B) :match (is-a ?a ?A)                                   :assert (owe ?R ?a ?B) …)
(rule paid (?R ?A ?B) :match (and (owe ?R ?a ?B) (is-a ?b ?B) (?R ?a ?b))   :assert (paid ?R ?a ?B) …)
(rule unpaid ()       :match (and (owe ?R ?a ?B) (absent (paid ?R ?a ?B)))  :assert (still-owed ?R ?a ?B) …)
```

The state holds `(owe seats Ann Chair)`, `(owe seats Bob Chair)`, `(paid seats Ann Chair)` and
`(still-owed seats Bob Chair)`.

**What can be held and what cannot.**

- *Required* only grows: a requirement a state has, every state above it has. So `(owe …)` can be a held fact,
  inherited like any other, and **counted as facts**.
- *Paid* only grows too.
- ***Still owed* is their difference and shrinks.** Stored, as `still-owed` is above, it is stale in a child that
  pays it: in 15 of the seats lattice's 16 nodes (item 32, § 7), and a program that reads it is non-monotone.

So the form is: **`(owe Q)` is held; *discharged* is read at the fixpoint**, every held `(owe Q)` answered. That is
the pass the kernel runs today, over facts in place of rule instances, and item 32's emulation gives the kernel's
count at 16 of 16 nodes with the same $R$. `std.debts`, which you ruled merged, is this as a module, in the
second way of the table: `(debt-due R (total-owed-of …))`, `debt-paid` and `(debts-discharged)`.

## 3. The connectors

| what | between them | why |
|---|---|---|
| held facts | ***and*** | a state holds all of them |
| requirements, `(owe Q)` | ***and***, each an ***or*** inside | every one is to be answered; each is answered by any one witness: `(owe seats Ann Chair)` is *`(seats Ann C1)` or `(seats Ann C2)`* |
| hypotheses, `(open H)` | **none** | each may be supposed or not, by itself: n of them are $2^n$ commitments, a space and not a formula |

What limits the hypotheses is said by two kinds of rules and by nothing between the hypotheses themselves:

- a rule that **refutes**: each dead commitment is a *not all of these together*, and its core is the smallest such;
- a rule that **requires**: each requirement is an *at least one of these*.

$R$ is the commitments that break none of the first and meet every one of the second. In a SAT solver's words the
`open` facts are the variables, the cores the negative clauses, the requirements the positive clauses, and
saturation the propagation; in a truth maintenance system's, a commitment is an environment and a core a nogood.
It holds for monotone programs, where a dead commitment's supersets are dead. And it shows where Ein's only *or*
lives: in a requirement.

## 4. Can the `hrule` form go?

**Yes.** A hypothesis rule is a rule whose conclusion stands under `open`, read at root:

```lisp
;; today                                        ;; as a rule
(hrule guess (?R ?T)                            (rule guess (?R ?T)
  :match  (is-a ?x ?T)                            :match  (is-a ?x ?T)
  :assert (?R ?x)                                 :assert (open (?R ?x))
  :why    "guess: is {?x} {?R}?")                 :why    "guess: is {?x} {?R}?")
```

**It runs today, with another word for `open`.** [Review item 34](q-m1f.549_hypotheses_as_facts/README.md)'s
`seats-asked.ein` and its *zebra2* twin derive each candidate by an ordinary rule as a fact, `(question F)`, and
propose what is asked by one generic hypothesis rule: the read-out is byte for byte the program's own.

**It is not what you declined.** [Q-M1f.561](../open_questions.md#q-m1f561--one-way-to-say-must-and-one-to-say-may-hypothesis-rules-removed)
replaced the hypothesis rule by an obligation, *may* by *must*, and a program with a *may* and no *must* lost its
candidates ([`one_way_each.md`](one_way_each.md)). Here *may* stays and only its spelling changes. It does revise
your *"we leave all existing options to control hypotheses as is"*, which is why it is a question and not done.

**What the form does today that a rule would not**, each to be settled:

| | a hypothesis rule today | a rule that concludes `(open F)` |
|---|---|---|
| when it runs | once, over root's fixpoint | in saturation, at every node. Above root it could conclude a new `(open F')`, a dependent hypothesis, which you parked ([V-21](findings.md#v-21--dependent-hypotheses-a-candidate-proposed-above-root)): $H$ stays root's `open` facts, and one first derived above root is not proposed |
| its activators | in `(solve :hrules …)`, no facts | **(i)** the clause kept, listing activators as now, which is your question's form; or **(ii)** ordinary facts, `(guess chosen Drink)`, and the clause gone |
| its conclusion | one fact | any conjunction of `(open F)` and facts |
| the ladder's first rung | a hypothesis rule that is activated | root holds a fact of `open` |
| the word `open` | the obligation's verdict, `(open ?R)` | the obligation needs another word, § 2's `owe` |
