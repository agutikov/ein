# S1f.10.6 — Obligations derived under a hypothesis

**Phase:** [P1f.10](README.md) (The structure of the hypothesis set)
**Estimate:** 3 days
**Depends on:** [S1f.10.1](s1f.10.1_exclusion_census.md) — its census answers
*does any corpus entry derive an obligation activator under a hypothesis?*, and
the answer decides whether this stage is describing a live shape or bounding an
empty one.
**Blocks:** [S1f.10.3](s1f.10.3_the_restricted_join.md) may not ship its
restricted join before this stage has stated the **domain** the group structure
is valid over.
**Answers:** [Q-M1e.11](../../m1e_review_processing/open_questions.md#q-m1e11--what-happens-to-an-obligation-derived-under-a-hypothesis)
and the review's
[`Q6`](../../m1e_review_processing/review/open-questions.md), and hands
[S1e.2.1](../../m1e_review_processing/p1e.2_high/s1e.2.1_correctness.md) T3 the probe its guard is
missing.
**Source:** [D2](../../m1e_review_processing/p1e.1_open_questions/s1e.1.1_search_soundness_probes/d2_q6_which_decline_to_construct.md),
converted to a stage on 2026-08-28 on the user's instruction. Everything below
the ruling was D2's; the parts about what the search should *do* are new.

## Context

This stage is P1f.10's own founding sentence, examined. The phase says *the
search enumerates subsets of a **fixed** `alive` set* and builds a branch
structure on it that is computed once at load. An obligation's activator is an
ordinary fact, a rule head can derive one **inside a fork**, and the facts that
would discharge the new obligation are new candidates. So the set is not fixed
in general, and what is open is how far from fixed it can get.

### What is settled, and is not this stage's

**The premise the tree traversal states is false.** `tree()` probes the
generation rung once at root
([`solve.rs:889-914`](../../../ein.rs/crates/ein-infer/src/solve.rs)) on the
stated premise that *"the mode is a property of the program rather than of the
node, so asking once is asking enough"*, and the repo's own doc comment already
refutes it — `activators_for`
([`compile.rs:54-69`](../../../ein.rs/crates/ein-infer/src/compile.rs)):

> A parameterised one consults the **fork's** `rule_apps_by_rule`, not the
> load-time KB's, because a fork derives activators of its own during
> saturation.

**And the guard is decided**, 2026-08-28, by the user: *the obligation has to
be re-evaluated after each saturation* — the mode is re-read at every node.
[S1e.2.1](../../m1e_review_processing/p1e.2_high/s1e.2.1_correctness.md) T3 writes it, and it costs
nothing that is not already paid: `tree_node` builds a `HypGenStats`, calls
`generate_one_branch`, keeps the candidate list and **drops `hs.rung.mode`**
(`solve.rs:945-956`). The change is to stop discarding a value, not to add a
call.

### What is open

Two things, and the second is why this is a stage rather than a fix.

**1. Is a flip constructible?** Rung 2's gate is
`!program().obligations.is_empty()` — static — so a flip has to come from
`oblgen::generate` returning `Declined`, and there are exactly three
fact-dependent ways
([`oblgen.rs:232-262`](../../../ein.rs/crates/ein-infer/src/oblgen.rs)):

| # | condition | what the probe needs |
|---|---|---|
| **1** | a bare `(open)` plan — no relation to branch on | an obligation rule asserting `(open)` whose activator is derived only in a fork |
| **2** | a projection that will not resolve for that activator | the same, with a head shape `project()` returns `None` for |
| **3** | **C4** — an accepted obligation *scans* a relation the rung itself proposes | a derived activator adding an obligation on `S`, where an existing obligation's guard reads `S` |

Sketch for **3**, the likeliest — root has one obligation on `r`; committing a
hypothesis derives `(marker …)`, which activates a second obligation whose
relation is scanned by the first:

```lisp
;;; SKETCH — not run. The shape, not the syntax.
(rule owe-r (?isa ?D)
  :match  (and (?isa ?a ?D) (absent (and (?isa ?b ?D) (r ?a ?b) (s ?b))))
  :assert (open r))            ; guard scans `s` …

(rule owe-s (?isa ?D)          ; … and this rung proposes `s`
  :match  (and (marker ?D) (?isa ?b ?D) (absent (s ?b)))
  :assert (open s))

(rule derive-marker ()         ; the activator, derived only under a hypothesis
  :match  (r ?a ?b)
  :assert (marker T))
```

At root there is no `(r …)`, so no `(marker T)`, so `owe-s` has no activator
and the rung reports `Obligations` — the tree accepts. One commitment later the
marker exists, `owe-s` gets a plan, C4 sees `owe-r` scanning `s`, and the call
**declines**, falling through to the blind enumerator with no event the tree
reads and no assert.

**2. What should the search do about it?** A guard says *stop*. It does not say
what a branch structure means when the set underneath it grows, and three
properties of the growth shape the answer:

- **It is monotone.** The KB is append-only, so an obligation can appear under
  a hypothesis and can never be retracted under a deeper one. Nothing has to
  handle a set that shrinks.
- **C4 is the contract, and it is silent here.**
  [`domain_contract.md`](../../../docs/history/m1d_satisfiability/domain_contract.md)
  says *a branch is jointly exhaustive only while the candidate set cannot grow
  underneath it* — and says nothing about what to do when it does.
- **Discharge changes meaning.** M1d S1d.2.6 scoped `Open` so that *a state is
  judged by discharge when it has been told what it owes*. A node told at depth
  3 what root was not owes what its ancestors did not, so `complete` at that
  node is a different predicate from `complete` at its parent.

### The loss mechanism, stated correctly

The review's `CO-H3`(c) says the tree "would treat a non-exhaustive branch set
as exhaustive and miss models". Reading `tree_node`, that is not the first
thing that happens: the tree enters **every** candidate the blind rung returns
and recurses, because `one_branch` is a parameter the blind rung ignores. What
you get first is the `d!`-per-path walk P1.5b deleted.

The **missed model** comes from `complete` changing meaning at that node. A
node whose obligations are all discharged is a solution under rung 2 and is
*not* complete under rung 3 while the blind enumerator still proposes anything
— and `branching/06` is the standing proof that it proposes junk long after any
real debt is settled
([D8](../../m1e_review_processing/p1e.1_open_questions/s1e.1.1_search_soundness_probes/d8_branching06_untyped_models.md)).

So the assertion is two things, not one:

1. `tree_models ⊇ lattice_models`, compared **fact for fact** (the
   [`tree_traversal.rs`](../../../ein.rs/crates/ein-infer/tests/tree_traversal.rs)
   idiom — *"never of `k`"*), and
2. **no node emitted a `rung` event with a mode other than `obligations`**.

(2) is the one that survives even if (1) happens to pass by luck.

## The four candidate answers

Q-M1e.11's table, which this stage picks from and writes down:

| | what | consequence |
|---|---|---|
| **A** | re-derive the branch at the node where the set grew, and continue | the honest one; needs `complete` to be relative to the node's own obligation set, not root's |
| **B** | decline the traversal at the flip and fall back to the lattice | safe, and it throws away the descent so far. The lattice has no such premise to lose, which is why it is the fallback |
| **C** | refuse at load — every obligation activator must be root-derivable | a diagnostic instead of a wrong answer, the shape of [Q-M1e.9](../../m1e_review_processing/open_questions.md#q-m1e9--is-dead-really-upward-closed-under-absent)'s option B. It forbids a program nobody has yet written |
| **D** | accept the loss and state it | needs a witness first, which is what T1 and T2 are for |

**The recommendation is A if the flip is constructible and C if it is not**,
and the reason is this phase's rather than the traversal's: a group structure
whose domain is *programs whose obligation activators are root-derivable* is a
structure with a stated domain, and a structure with a stated domain is one
S1f.10.3 may spend. What is not admissible is shipping a structure whose
validity nobody wrote down.

## Acceptance

- **A ruling on Q-M1e.11 written into
  [`open_questions.md`](../../m1e_review_processing/open_questions.md#q-m1e11--what-happens-to-an-obligation-derived-under-a-hypothesis)**,
  with the date, naming which of A–D and why — and the same sentence written
  where the structure is *computed*, not only in a plan file
  ([Q-M1e.1](../../m1e_review_processing/open_questions.md#q-m1e1--what-is-the-standard-of-proof-for-refuted)'s
  third rule).
- **The probe is banked either way.** Constructible: a fixture under
  `examples/` or `tests/` whose `rung` events show the flip, with a corpus
  entry, and it is what makes S1e.2.1 T3's guard removable by a later
  milestone. Not constructible: the three decline conditions each tried and
  written up, and the reason stated at
  [`oblgen.rs:232-262`](../../../ein.rs/crates/ein-infer/src/oblgen.rs) — *"a
  guard added without a probe is a guard nobody can ever remove."*
- **A test named for the premise** — *the tree's branch sets stay jointly
  exhaustive* — running in the gate, which is worth more than the
  `debug_assert` it may replace because the assert only fires in a debug build.
- **P1f.10's domain sentence exists.** One paragraph in
  [`README.md`](README.md) § The set that is not fixed saying which of the two
  admissible readings the phase took, cited by S1f.10.2 and S1f.10.3.
- **Not one answer moves.** Like every stage of this phase: the corpus's model
  sets are identical fact for fact, at the same `-m` and `-e`, before and
  after. A ruling that would move one is a finding, not a fix, and it goes back
  to the milestone.

## Tasks

### Task T1f.10.6.1 — Is a derived activator read at all?

Half a day, and it is first because everything else is built on it. Activators
are ordinary facts, so a rule head can produce one; whether the loader and
`oblgen` read a **derived** activator the same way they read a declared one is
the actual unknown. Check with `ein saturate --dump` before building anything
on top of it, and record the answer even if it is *yes, identically* — that is
the sentence `oblgen`'s doc comment is missing.

### Task T1f.10.6.2 — Construct the flip

Condition **3 (C4)** first, from the sketch above: it is the likeliest to fire
and the closest to a shape a real program could reach. If it does not fire
within a day, try condition 1 (a bare `(open)` plan) — simplest to write,
proves constructibility fastest, weakest as a shape anyone would ship — and
record condition 2 as untried with the reason.

Timebox: one day. *Not constructible* is a result, and under
[Q-M1e.1](../../m1e_review_processing/open_questions.md#q-m1e1--what-is-the-standard-of-proof-for-refuted)
it is one only if the attempt is banked.

### Task T1f.10.6.3 — Does the tree then miss a model?

Run the probe under `EIN_TRAVERSAL=tree` and under the default lattice with
`-e`, and diff the model sets **fact for fact** — the same comparison
[S1d.10.6](../../../docs/history/m1d_satisfiability/README.md#s1d106--the-traversal)
used to verify the 86-vs-17 204 592 result. A tree set that is a strict subset
is the bug; a tree set that agrees while a `rung` event shows a flip is the
*harmless* case, and finding out why is part of the task.

### Task T1f.10.6.4 — Rule, and write the domain

Pick from A–D, write it into `open_questions.md` and beside the code, and write
P1f.10's domain sentence. Then say what
[S1e.2.1](../../m1e_review_processing/p1e.2_high/s1e.2.1_correctness.md) T3's guard becomes:

| outcome | what the guard becomes |
|---|---|
| constructible, and the tree misses models | the re-probe stays, and the fixture is its regression test; A or B decides what happens *at* the flip |
| constructible, and the tree still agrees | the re-probe stays as a cheap invariant, and the write-up says why the agreement is not luck |
| not constructible | the re-probe stays — it is already free — and C's load-time refusal is the alternative worth pricing, because a guard whose trigger nobody can construct is a guard nobody can remove |

## Notes

**The ordering wrinkle, stated rather than hidden.** S1e.2.1 T3 ships the guard
in [P1e.2](../../m1e_review_processing/p1e.2_high/README.md); this stage builds the probe in P1f.10,
which runs later. So `CO-H3`(c) is `fixed` before its regression test exists,
which is the exact shape
[Q-M1e.1](../../m1e_review_processing/open_questions.md#q-m1e1--what-is-the-standard-of-proof-for-refuted)
warns about — with the difference that the fix here is *applying a ruling*
rather than closing a risk by argument. Two ways out, and the milestone should
pick one deliberately: this stage runs early (P1f.10 before P1e.2, which the
phase order does not otherwise require), or T3 records that its test is owed
and names this stage. **The second is recommended**, because the guard is free
and the probe is not.

**Taken, 2026-08-29 — and the guard has shipped.**
[S1e.2.1](../../m1e_review_processing/p1e.2_high/s1e.2.1_correctness.md) T3
wrote it into `tree_node`
([`solve.rs`](../../../ein.rs/crates/ein-infer/src/solve.rs)): the mode is
re-read on every node the tree expands, and any mode but `Obligations` narrates
a `traversal` event with the mode as its reason and stops descending. It cost
what the stage predicted — nothing, since `tree_node` already built the
`HypGenStats` and dropped `hs.rung.mode` — and it moved no number: the headline
re-take is 86 enterings and the same 32 models, and **0** declined events on
that run, which is the corpus saying it cannot flip the mode rather than the
guard saying nothing happened. The comment at the site names this stage as the
owner of the missing probe, in those words. What this stage still owes is
therefore exactly one thing: a program that reaches that line with a mode other
than `Obligations`. If T1f.10.6.1 cannot build one, that is C's case above and
the sentence to write is that the guard's trigger is not constructible — which
is a finding, not a licence to remove it.

**Why this left S1e.1.1.** The stage that held it is *three soundness probes*,
and two of them — Q4 and Q5 — are about a search path that exists. This one is
about what a hypothesis set *is* when the theory can extend it mid-search,
which is the subject of this phase and of nothing else in the milestone. The
handover is recorded at both ends: a stub in
[S1e.1.1](../../m1e_review_processing/p1e.1_open_questions/s1e.1.1_search_soundness_probes/d2_q6_which_decline_to_construct.md)
and Q-M1e.11's owner line.
