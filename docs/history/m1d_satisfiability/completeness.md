# The completeness argument for a per-obligation tree

**Stage:** [S1d.10.6](s1d.10.6_the_traversal.md) — [T1d.10.6.1](s1d.10.6_the_traversal.md#task-t1d1061--the-completeness-argument), written 2026-08-26
**Status:** the argument, before the code, as the stage's first acceptance
bullet requires. Nothing in `ein.rs` implements this traversal yet.

The stage asks for three parts and says the third is the one that can fail:

> the branch is jointly exhaustive *by the obligation's meaning*; the recursion
> terminates *because the owed set strictly decreases and nothing adds to it*;
> and the leaves are models *iff* saturation determines everything no
> obligation owes. Name what makes the third true, and what a program looks
> like for which it is false.

Two of the three hold. **The third is false in general, it is false today, and
the tree is not what makes it false** — which is the finding this document
exists for.

Every number below is from the out-of-process emulator described in
[README §1](README.md#1-the-proof-costs-83-517-what-the-answer-does):
six lines of policy over `ein solve -m 0 --json-summary`, a fresh process per
node.

---

## 1. The branch is jointly exhaustive at its node

A `total-owed` instance says *`a` has an `R`-arrow to some member of `B`*. Its
candidate set — `{(R a b) : b ∈ ext(B)}`, less what is already excluded — is
therefore **jointly exhaustive**: every model of the program satisfies at least
one of them, because the obligation says a model must. A `surjective-owed`
instance is the same statement read from the other argument, `{(R a b) : a ∈
ext(A)}` for a fixed `b`.

So branching on one instance and recursing into each alternative visits every
model that passes through that node. No clause, no death and no depth cap enters
the argument. That is the difference the milestone named — *committing to one
alternative excludes its siblings without anybody having to refute them* — and
it is why the tree needs no `--max-set-size`.

**Mutual exclusivity is a separate and weaker property, and it is not needed.**
Where `R` is also `functional` the alternatives are pairwise exclusive and the
branch is a partition. Where it is not, a model may satisfy two alternatives and
be reached by two paths; that is a duplicate, not a gap, and `state_key`
already dedupes it — the lattice has the same property and the same answer.

**The precondition is C4, and the rung already enforces it.**
[`oblgen.rs`](../../../ein.rs/crates/ein-infer/src/oblgen.rs)'s own words: *"a
branch is jointly exhaustive only while the candidate set cannot grow underneath
it, so an obligation whose guard scans a relation the rung itself proposes is
**declined**"*. A declined obligation takes the call back to the blind
generator, which is what keeps completeness a property of the ladder rather than
of the puzzle. The tree inherits that check unchanged; it does not need a new
one.

Measured on `examples/zebra2-minus-15-obligations.ein`: **mean branch width
5.0**, which is `ext(House) = 5` exactly, and `stuck = 0` — no node ever owed
something it could not branch on.

## 2. Termination — measured on every edge, not argued

Each commitment discharges at least one owed instance, and the owed set is
finite at root. The bound is therefore the root's own debt, and it does not
mention the depth cap: on the phase's entry `owed = 46` at root and the tree
bottoms out at **depth 6**, where the lattice needs 22 layers.

*"and nothing adds to it"* is the half that could fail, so it was measured
rather than assumed. The emulator records the owed count at every node and
compares it with its parent's:

| policy | nodes | dead | models | max depth | edges where owed **strictly** decreased |
|---|---:|---:|---:|---:|---|
| most-owed relation first | 171 | 105 | 32 | 6 | **65 of 65** |
| fewest-owed relation first | 196 | 125 | 32 | 7 | **70 of 70** |
| report order (the rung's default) | 206 | 133 | 32 | 6 | **72 of 72** |

Every edge, every policy, no exception — and the four policies reproduce
[the reconnaissance's table](README.md#1-the-proof-costs-83-517-what-the-answer-does)
node for node, which is the check that the emulator was rebuilt correctly rather
than remembered.

**What would break it**, stated as a property of a program rather than as a
worry: a rule that asserts an `is-a` fact into a type an obligation ranges over,
because a new member of `B` adds a `surjective-owed` instance underneath a
branch that was chosen without it. On the zebra family this cannot happen —
`is-a` is **closed**, no rule positively concludes it — and a program where it
can is one where the termination bound has to be measured rather than derived.
The emulator's per-node owed trace is the instrument that would catch it.

## 3. The leaves — where the argument fails, and it is not the tree's fault

The stage's third part reads *"the leaves are models **iff** saturation
determines everything no obligation owes"*, with `uncovered` as its structural
half. Measured, that framing turns out to be **stronger than what is true and
weaker than what matters**.

### 3a. `uncovered ≠ 0` does not mean the tree loses models

It means the *rung* does not propose those relations — and the lattice is
running on the same rung. Since [S1d.2.5](../p1d.2_obligations/hypotheses_from_obligations.md)
a program that declares an obligation has its candidates generated by the
obligations rung, not by the blind enumerator, whichever way the search then
walks them. On `zebra2-minus-15-obligations` the `rung` event says
`candidates 230` and layer 1 enters **96**, every one of them a `*-loc` fact.

So **the tree and the lattice search the same candidate space** and differ only
in how they traverse it. Whatever the rung excludes, both exclude. A tree cannot
be less complete than the lattice it replaces for the reason this part worried
about, because the exclusion happens one level below the traversal.

### 3b. What *is* excluded, measured

[S1d.3.1](../p1d.3_model_sets/model_set_census.md)'s `EIN_LEFTOVER=1` probe is
the instrument, and split by relation it is exact:

| program | `k` | leftover at the model | by relation |
|---|---:|---:|---|
| `examples/zebra2-obligations.ein` | 1 | **3 678** | `is-a` 930, `is-a*` 900, `next-to` 922, `right-of` 926 |
| the `uncovered.ein` fixture below | 3 | 33–34 | `is-a` 12, **`knows` 12**, `seats` 9–10 |

The zebra row is the argument holding. The leftover is **exactly the four
`uncovered` relations and nothing else** — not one `*-loc` fact remains
proposable, so the obligations and saturation between them decide every
attribute arrow, which is the claim
[S1d.2.5 §6](../p1d.2_obligations/hypotheses_from_obligations.md) settled by
comparing model sets. Of the four, `is-a` and `right-of` are **closed** — no
rule positively concludes them, so a hypothesis about them could never be
confirmed — and `is-a*` and `next-to` are derived by the puzzle's own rules
from what is already decided.

### 3c. And the program for which it is false

The stage asked what one looks like. It looks like this, and it is 25 lines:

```lisp
(import std.algebra :symbols (total-owed surjective-owed))

(relation is-a   T T         :why "{?1} is a {?2}")
(relation seats  Guest Chair :why "{?1} sits on {?2}")
(relation knows  Guest Guest :why "{?1} knows {?2}")

(is-a Ann Guest) (is-a Bob Guest)
(is-a C1 Chair)  (is-a C2 Chair)

;; `seats` is owed from both ends — every guest a chair, every chair a guest.
(total-owed      seats is-a)
(surjective-owed seats is-a)

;; `knows` is owed by nobody, and is NOT closed: this rule positively concludes
;; it, so the closure pass cannot rule it out. Nothing determines whether
;; anyone knows anyone.
(rule knows-is-mutual ()
  :match  (knows ?a ?b)
  :assert (knows ?b ?a)
  :why    "{?1} knows {?2}, so {?2} knows {?1}."
  :priority 100)

(query :goal (seats ?g ?c))
```

`ein solve -e` reports **`k = 3`, `exhausted = true`** — *these are the models*
— and none of the three mentions `knows`. Add `(knows Ann Bob)` to the program
and it reports **three more**, all consistent, all exhausted. So there are at
least six and the run said three, with the word that S1d.3.3 made normative for
saying *these are the models*.

**The tree agrees with it.** The emulator finds 11 nodes, 0 dead, max depth 3,
and the same seatings — because both searches are asking the rung, and the rung
owes nothing about `knows`.

### 3d. So the third part restated

> A leaf is a model of the program **as the rung scopes it**. It is a model of
> the *program* when every hypothesis-eligible relation no obligation owes is
> either **closed** — no rule concludes it, so no hypothesis about it could ever
> be confirmed — or **decided** by saturation from what the obligations settle.
> `uncovered` counts the relations that have to be checked; `EIN_LEFTOVER=1`
> checks them.

`uncovered ≠ 0` is not a defect and not a warning about the tree. It is the
signal that the leftover probe has something to report, and the probe is what
answers. On the zebra family the answer is *closed or derived*; on the fixture
above it is `knows`.

**And this is shipped behaviour, not a tree's risk.** The phase README's risk
list already names it —

> A puzzle where they are not is a puzzle where the tree is **complete for the
> obligations and incomplete for the models** … The trap is that on this corpus
> it would not fire.

— and the fixture above is that puzzle, built. What the measurement adds is
that the trap fires for the **lattice too**, today, at `exhausted = true`.
Whether that verdict should qualify itself the way
[T1d.10.5.2b](s1d.10.5_contract.md#task-t1d1052b--contradiction-and-what-a-cap-may-say)
made a truncated `Contradiction` qualify itself is a question for
[S1d.10.5](s1d.10.5_contract.md)'s vocabulary and not for this stage — but it is
the same question, one level up: **a claim stated without the qualifier that
licenses it.**

## What the stage may now build on

- Parts 1 and 2 hold, and part 2 holds by measurement on every edge of every
  policy rather than by argument.
- Part 3 is **not a reason to hold the tree back**. It is a property of the
  obligations rung, it is already in the shipped engine, and the tree neither
  worsens nor improves it.
- The comparison that would catch a regression is therefore **not** tree-vs-
  lattice on model *count* — they cannot differ for this reason. It is
  `EIN_LEFTOVER=1` on the leaves, which is a property of the rung and can be run
  today, and the fact-for-fact model-set equality
  [T1d.10.6.5](s1d.10.6_the_traversal.md#task-t1d1065--measure-both-regimes)
  already asks for.
- One cost worth recording before someone plans around it: **re-taking S1d.2.5's
  blind-arm comparison at `-e` is not affordable.**
  `EIN_OBLIGATION_CHOICE=off ein solve -e examples/zebra2-obligations.ein` did
  not finish in **10 minutes**, where the rung arm answers in 0.03 s. The blind
  enumerator proposes 3 734 candidates at layer 1 against the rung's 56, and
  `Σₖ C(3734, k)` is what that costs. Any future comparison of the two arms needs
  a depth cap and has to say so.
