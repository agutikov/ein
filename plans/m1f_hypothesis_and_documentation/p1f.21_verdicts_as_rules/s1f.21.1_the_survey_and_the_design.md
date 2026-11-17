# S1f.21.1 — the survey and the design

**Phase:** [P1f.21](README.md)
**Status:** next.
**Writes:** this file's § Record, [`design.md`](design.md) beside it, and nothing under `docs/`, `ein.rs/`,
`stdlib/`, `examples/`, `tests/` or `corpus/`: it reads the engine and runs it.

## What it answers

**1. What the kernel does today**, for each of the three verdicts, with the place in the code, the test that
holds it and a run that shows it:

- *contradiction*: where a fact beside its stored negation makes a state dead; whether `(false)` is then a fact
  the state holds or only a verdict; what a core of such a state rests on; what the trace and the events show.
- *alive / dead*: the predicate, where it is read, and what reads it (the routes, the read-out, a claim).
- *owed / discharged*: how a rule concluding `(open R)` is told apart at load, where it is read (never in the
  agenda, once on a quiescent state), what an instance is (the rule, its activator, its bindings), what is
  counted, what `owe` events carry, what hypothesis generation reads of it, what a claim can say of it.

**2. Each verdict as a rule.** Written in Ein and run:

- *from a fact and its negation, `(false)`*: can a program write it today (a variable for a whole fact in a
  `:match`, beside `(not …)` of the same variable)? If it loads, does a program that carries it answer as the
  same program without it does, on every corpus program that has a stored negation? If it does not load, what
  exactly the language lacks, and what the nearest rule Ein can write is (one rule a relation, a rule over
  `written`, a relation-polymorphic rule a count of arguments).
- *dead from `(false)`, alive from its absence*: what such rules would be, and why the second is no rule a
  program may write today (a refutation may not rest on an absence a commitment can fill, Q-M1e2.5) or is.
- *owed where a trigger holds and no witness does; discharged from nothing owed*: the stdlib's four obligation
  rules read this way; what *discharged* is as a rule over them.

For each: the rule, whether it loads, what it derives, and in what the kernel's own doing differs from firing it
(order, the agenda's first half, provenance, what a core names).

**3. `(open R)` as a fact.** The instruction's two properties, each shown on a program and not argued:

- **it duplicates**: a state that owes two instances over one relation, where the count is two and the tuple
  one;
- **it can be removed**: a state that owes and a larger state that does not, where every other fact of the
  smaller is a fact of the larger.

And what today's engine does with a rule that concludes `(open R)` beside another literal, with a fact `(open R)`
a program writes, and with a rule that reads `(open ?R)` in its `:match` (Q-M1f.66).

**4. `(open R <justification>)`, designed.** At least these, each on two real corpus programs (one of
`examples/lattice/` with obligations, and `examples/zebra2-obligations.ein`), with the knowledge base each state
would hold and the answers computed (by a rule set in Ein that emulates it where Ein can, by hand from
`--events` where it cannot; say which):

- **what the justification is**: the candidates (the obligation rule's activator fact; the trigger's matched
  facts; the instance, as the tuple of the rule's name and its bindings; the fact that would pay it), what each
  makes of property (1), and which of them is an object the knowledge-base graph already has;
- **what becomes of property (2)**: whether an emitted `(open R j)` stays in every larger state and a state
  *owes* where it holds one with no payment beside it (so no fact is ever removed and *owes* is an absence), or
  is not emitted where it is paid (so the fact set is not monotone in the commitment), and what each does to
  the routes' licences, to the monotone verdict and to the forbid check;
- **who emits it**: the agenda, as any conclusion (then an obligation reads an absence during saturation, and
  what the order does to it), or the pass on the quiescent state as today (then the facts are added after the
  fixpoint, and what may read them);
- **what reads it**: hypothesis generation's obligation rung, the tree route's choice of an owed instance, the
  read-out's rows, the events, the answer dump, a claim, a core;
- **what changes for a program**: the stdlib's four rules, a program's own obligations, `:expect`, the tests
  under `tests/stdlib/`, the corpus's goldens;
- **what it costs and what it buys**: stated as what would have to be built and checked, with no estimate, and
  what would become sayable that is not today (a rule reading what is owed; a core of an owing state; the
  obligation's place in the graph).

**5. The question, and a recommendation.** One page the user can rule from: the options side by side on the two
programs, what each leaves true of *the answer is a region R* and of *no option changes R*, the recommendation
and why, and what S1f.21.2 writes in the reasoning pages under each ruling.

## Acceptance

`design.md` holds 1 to 5, every claim of 1 to 3 beside the command that shows it and its output, every program
it shows kept in the page (the scratch directory does not last). The link check and the release-voice check
pass. Nothing outside this folder is changed.

## Record

(the stage writes what it did here)
