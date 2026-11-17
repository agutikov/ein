# S1f.21.1 — the survey and the design

**Phase:** [P1f.21](README.md)
**Status:** done 2026-11-17. The question is put as [Q-M1f.390](../open_questions.md#q-m1f390--what-an-obligation-emits-the-verdict-with-its-justification-or-as-a-fact), not ruled.
**Writes:** this file's § Record, `design.md` beside it, and nothing under `docs/`, `ein.rs/`,
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

**2026-11-17.** [`design.md`](design.md) holds the five answers in the stage's order; the engine was read at
`c38b4048` and run as the main tree's gate build, and nothing under `docs/`, `ein.rs/`, `stdlib/`, `examples/`,
`tests/` or `corpus/` was changed. Every program the page relies on is in it whole, and every script that took
a count is in its Appendix B.

**What it found.**

- **Contradiction** ([§ 1.1](design.md#11-contradiction), [§ 2.1](design.md#21-from-a-fact-and-its-negation-false)).
  The kernel reads a fact beside its negation and derives nothing: the state holds no `(false)`
  (`ein saturate --dump`). Ein cannot write the rule of every fact, since no premise is a variable and
  `(not ?f)` is written in a conclusion only; it writes one rule a relation, or one a count of arguments with
  its activators derived from the declarations. With that set appended, 560 of the 567 corpus programs `ein
  solve` answers answer the same, and seven are refused by *forbid*.
- **Alive and dead** ([§ 2.2](design.md#22-dead-from-false-alive-from-its-absence)). *Dead from `(false)`* runs,
  and does not fire in a route's node stopped at its first contradiction, nor in a state dead by a pair. *Alive
  from its absence* loads, makes the program one the static check flags, and leaves `(alive)` in a dead node.
- **Owed and discharged** ([§ 2.3](design.md#23-owed-and-discharged), [§ 2.4](design.md#24-the-verdicts-together-which-may-be-a-fact)).
  As ordinary rules under another relation's name, the verdicts are the kernel's one state at a time, on all 16
  commitments of `examples/lattice/05_owed_seats_bounded.ein`; with the rule for *discharged* written first, 9 of
  the 16 are wrong; across a fork the debt outlives its payment in 15 of 16 nodes, reported as stale facts. The
  requirement, the payment and *discharged* are three rules that read no absence a larger state falsifies: the
  static check stays monotone, and the unpaid requirements are the kernel's `owes` at all 16 nodes and at all 120
  nodes `ein solve` enters on `examples/zebra2-obligations.ein`.
- **`(open R)` as a fact** ([§ 3](design.md#3-open-r-as-a-fact)). Both properties, on programs; `open` is a
  parse error everywhere but the verdict; and how an owed instance, a hypothesis and the verdict stand to a
  relation's extent, in the two terms the user ruled the same day.
- **The design** ([§ 4](design.md#4-open-r-justification-designed)). Four candidates for the justification, of
  which the instance alone makes a tuple a debt; why no fact meaning *owed now* is a fact of a state; who can
  emit what; every reader; what changes for a program; what each option builds and buys.

**The question.** [Q-M1f.390](../open_questions.md#q-m1f390--what-an-obligation-emits-the-verdict-with-its-justification-or-as-a-fact),
four options, the recommended default (b): the verdict carries its justification and stays a verdict.
[`design.md` § 5](design.md#5-the-question-and-a-recommendation) is the page to rule from, with the strongest
argument against the recommendation and what was not determined.

**Beside the stage's own files.** The question's entry is in `../open_questions.md`, which the stage's *Writes*
line does not name: the instruction that launched the stage asked for it there. [`findings.md`](findings.md) is
the phase's findings page, V-1 to V-4. The [README](README.md)'s status, its row for this stage and its section
on the question say what is done; its row for S1f.21.2 now says which verdicts Ein can state as rules and which
it cannot, where it said *if S1f.21.1 finds Ein can say it*, and its row for S1f.21.3 says when there is nothing
to build. Nothing else of S1f.21.2 to S1f.21.4 was changed.

**Not done.** No time was measured, only facts. `std.slots`' two obligations were emulated at root on their four
fixtures and on no search. The slow corpus entries were swept with the others under a 60 s limit a run, and none
timed out. What option (d) asks of the copy-on-write layers and the container was not read.

**Checks.** `python3 utils/doc_audit.py --links --check --tracked`: 0 findings in links, 0 in decision ids.
`python3 utils/check_release_voice.py`: none. `python3 utils/check_doc_blocks.py` and `python3
utils/render_diagrams.py --check`: both hold. The ten diagrams are sources under [`diagrams/`](diagrams/), each
drawn by `dot -Tsvg` beside it; `region-owed-seats.dot` is the engine's (`ein render region`).
