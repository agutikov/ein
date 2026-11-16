# `absent`: negation as failure

`(absent P)` is the one place a rule reads what a world lacks. Everything else a rule does is positive: it reads
facts and adds facts, and a knowledge base only grows. This page states what `(absent P)` means, the one moment at
which it is asked, what follows from that for the rest of the engine, and what it does not give.

The one distinction that matters: **`(absent P)` is a query about a saturated world, never a fact.** It is asked,
answered and acted on, and its answer is not stored, cached or carried to another world. Of the three readings of
negation as failure a reasoner could take, closed-world, stratified and world-relative epistemic, Ein takes the
third; the worked examples below are where the other two give other answers.

The surface forms that write it, `(absent P)` in a `:match` and the `forall` and `unknown` macros of `std.macro`, are
the grammar's ([`syntax.md`](../../user/reference/syntax.md#premise-forms-in-match)).

## Two halves: a positive closure and a boundary

Saturation is split along negation ([saturation](saturation.md)):

- **the closure** fires the purely positive part of every rule to quiescence and consults no negation at all. A
  rule's `(absent …)` premises are set aside; a match whose premises carry one is **parked**, not fired;
- **the boundary**, at each quiescence of the closure, judges the parked matches against that fixpoint, admits at
  most one whose every guard passes, and hands it back to the closure.

So the deductive part of Ein is monotone, and the non-monotone part, absence, is confined to one place with one
world to look at. Nothing is ever retracted: a world's facts only grow.

## Worlds

A **world** is one knowledge base under saturation: root, the program's own facts saturated by its rules, or a
node, root's fixpoint with a commitment `C` of hypotheses written in and the saturation resumed
([solution semantics](solution-semantics.md)). Within one saturation a world is **append-only**: `W(t)`, its fact
set after `t` firings, is contained in `W(t + 1)`. Two worlds are related only by the fork that made one from the
other; a question asked in one is not an answer in another.

## The definition

**Boundary epistemic negation as failure.** For a match considered in world `W` at a **positive quiescence**, a
point at which the closure has run to a fixpoint, with bindings `θ`:

> `(absent P)` holds ⟺ there is no extension `θ′ ⊇ θ` under which `P θ′` matches a stored fact of `W`; that is,
> `W ⊭ ∃x̄. P θ`, where `x̄` are the variables of `P` that `θ` leaves unbound.

`θ` is the match's bindings **as far as the premises written before the guard bind them**. Where a guard is
written decides which question it asks: `(and (absent (P ?x)) (Q ?x))` asks whether there is no `P` at all, and
`(and (Q ?x) (absent (P ?x)))` whether there is no `P` for this `x`. Setting the guard aside until the boundary
asks exactly the question it would have asked in place.

Three consequences of the wording:

- **Membership, of a saturated store.** `absent` reads the store, not the theory. Since the store it reads is the
  positive fixpoint, everything positively derivable is there, so for the positive part of a program membership
  and derivability coincide. They do not coincide on what a later boundary round admits: a fact admitted through
  one guard is not in the world an earlier guard was judged in.
- **A variable inside the guard is existential.** The guard is ¬∃ over the variables of its pattern that nothing
  bound before it. A conjunction inside a guard is one existential: `(absent (and (g ?x ?y) (h ?y)))` fails for
  `x` exactly when some `y` has both. The `forall` macro's universal is this existential negated twice:
  `(forall ?b G B)` is `(absent (and G (absent B)))`, *no `b` satisfies `G` and not `B`*.
- **One question, one world.** A guard is asked of the world it is judged in and its answer is acted on at once,
  so it has no interval in which to go stale.

`(unknown P)` is two guards, `(and (absent P) (absent (not P)))`: neither `P` nor its stored negation holds.

## The boundary

There is **one** point at which a guard is evaluated, and it is the boundary.

- **At quiescence, once.** The closure runs to its fixpoint with the guards set aside, so a match the closure finds
  says nothing about negation. At quiescence each parked match is judged: its guards are asked of the world, under
  their bindings, and the match is admitted if every guard passes. That judgement is the decision, and there is no
  other.
- **One admission per round, into an empty queue.** At quiescence the closure has nothing left to fire, so the one
  admitted match fires at once, against exactly the world its guards were judged in, and the closure resumes.
  This is a soundness requirement, not a throttle: on `p ← absent q; q ← absent p` both guards pass in the world
  that holds neither, so admitting both would derive `{p, q}`, which is a model of that program under no reading.
  Admitting one and re-judging the other against the world the first made derives one of the two.
- **Never after a firing.** Nothing is retracted. A fact admitted through a guard stays, whatever the world later
  comes to hold.
- **The fixpoint is a quiescence that admits nothing.** A saturation that ends with parked matches still parked
  ends because none of them passes; a world whose closure has quiesced while a parked match would pass is not at
  its fixpoint.

How a guard's answer can change while the world grows, and what that means for a match parked on it:

- a guard with **no nested `absent`** is anti-monotone in the world: once its pattern matches it matches forever, so
  a match rejected by it can never pass and is dropped;
- a guard **with a nested `absent`**, a `forall`'s, can turn from failing to passing as the world grows: adding a
  `B` makes the inner `absent` fail and the outer one pass. Its match stays parked and is judged again at a later
  quiescence, where a relation the guard reads has grown.

## What follows

Each of these is enforced where it applies; this page is the shared reason.

- **Root is never written by a node.** A fact a node derives may rest on an absence that holds in that node and not
  in root's future, so nothing a node derives is written back into root. Every node is root's fixpoint and its own
  commitment, and nothing else ([finding R](finding-r.md)).
- **A derivation records the absences it rested on.** A firing admitted at the boundary records each query that had
  to fail, one pattern per guard with a position the query left free recorded as free, beside the positive premises
  it read. Negative dependence is so visible: what a fact `Y` depends on is its positive premises and those
  absences together. A core names facts only and does not count the absences
  ([cores](cores.md)); the check of a program that may not be monotone reads them
  ([monotonicity](monotonicity.md)).
- **Deleting a fact can fabricate a contradiction.** Under `absent`, removing a fact can make a guard pass and so
  derive a contradiction the full world never had. A smallest explanation of a contradiction is therefore found by
  search over the derivations a world recorded, never by deleting facts and saturating again
  ([cores](cores.md)).
- **`absent` is world-relative.** The same ground query can answer one way in root and another in a node, so a
  guard's answer is never cached across worlds nor stored as a fact; a match a node inherits parked from root is
  judged again in the node's own world before it can be admitted. A fact root admitted through a guard is, though,
  part of root's fixpoint, and every node, a fork of root, holds it; where a commitment can fill the absence that
  fact rested on, the node holds it beside the fact that denies its premise. That is the case in which a node's state
  depends on the path to it, and a program that can reach it is not monotone
  ([monotonicity](monotonicity.md),
  [Q-M1e2.2](../../../decisions/answer/Q-M1e2.2-a-node-is-a-fork-of-root-an-assumption-on.md)).
- **The answer inherits the semantics.** A node's dead and owing bits are read off its fixpoint, and the candidate
  set H off root's, so the region R, every count and every claim are downstream of the same boundary queries.

## What is not provided

- **Stable or well-founded models.** The fixpoint is *supported at the boundary*: every admitted fact had its
  guards pass when it was admitted. It need not be stable. `p ← seed ∧ absent q; q ← p` has no stable model and
  converges to `{p, q}`; `p ← absent q; q ← absent p` has two, and the boundary's order picks one.
- **Retraction or truth maintenance.** A world never loses a fact. What a search learns from a dead node it learns
  by not entering it, not by retracting what led there.
- **Stratification as a refusal.** A program whose rules read through an `absent` in a cycle has no stratification;
  it is loaded and answered, and the cycle is reported as a diagnostic ([stratification](#stratification)).

On a **stratified** program the order rules fire in decides nothing derivable: the closure is complete before any
guard is asked, so a producer of a relation a guard reads has always finished before the guard is judged, whatever
the order the rules are written in. On a program **with no stratification** the boundary's order, the agenda's
([Q-M1f.313](../../../decisions/engine/Q-M1f.313-the-agenda-fires-first-what-leads-to-false.md)), picks which of
several supported fixpoints root reaches, and nothing tells the others exist.

## A refutation resting on an `absent`

A rule that concludes `(false)` or a stored negation may not rest on an `(absent P)` over a relation a commitment can
still extend: directly, through a fact a rule derived under such an `absent`, or through the activator its rule was
applied with. A program that writes one is refused, one line per refusing rule, by every command that takes the
candidate set ([Q-M1e2.5](../../../decisions/language/Q-M1e2.5-a-refutation-may-not-rest-on-an-absent.md)).

The reason is what such a rule says. Over an extendable relation it judges a state that has not yet decided `P` as if
`P` were false: it says *contradiction* where the truth is *not yet*, an obligation written as a refutation. And
*dead* stops being closed upward: `{(p A)}` dies on `(and (p ?x) (absent (q ?x))) ⟹ (false)` while `{(p A), (q A)}`
lives, so a search that took a superset of a dead set for dead would lose a state of R. The refusal names the forms
that say what was meant: close the relation if no commitment is meant to extend it; make the attribute one the
search must decide and refute the decided *no*; state the requirement as an obligation, `(open ?R)`; or conclude from
a stored negative, as the stdlib's `total` does. `examples/ein-bugs/naf-upward-closure.ein` is the shape, refused.

*Extendable* is the set of relations a commitment can change, followed through every rule with parameters whose
activator relation it holds, since a plan a node applies that root never had can write relations a commitment
changes. Over a relation no commitment can extend, the same shape is closed-world reasoning and loads.

This is not the stratification hazard and the two do not overlap. A guard over a relation a rule derives can change
its answer during saturation, and judging it at the fixpoint makes that sound. A guard over an extendable relation
is discharged by a commitment, in a world the guard was never asked in.

## Stratification

The relations a program's rules read and write form a graph, and an edge read under an `absent` is negative. A cycle
through a negative edge is what makes a program unstratifiable, and its parity says what it may mean: an even cycle
may have several supported fixpoints, an odd one may have none. Ein reports such a cycle as a diagnostic, never as a
refusal, and only ever as what the graph licenses, since a cycle whose rules never fire changes nothing. The graph,
its strata and what each surface prints of it are the [analysis of rules](../rule-evolution/analysis-of-rules.md).

## Worked examples

Each program is written `head ← body`; every fact is given at load.

| program | what saturation derives | what it shows |
|---|---|---|
| `p ← seed ∧ absent q`; `q ← t`; given `seed`, `t` | `{q}` and no `p`, in either written order of the two rules | the guard is judged against the positive fixpoint, in which `q` already holds; the order the rules are written in decides nothing derivable |
| `p ← seed ∧ absent q`; `q ← p`; given `seed` | `{p, q}` | the fixpoint is supported at the boundary, not stable: this program has no stable model |
| `p ← thing ∧ absent q`; `q ← thing ∧ absent p`; given `thing` (`examples/features/07_unstratifiable.ein`) | exactly one of `p` and `q` | two stable models; the boundary admits one match per round and the other's guard then fails |
| `ok ← seed ?x ∧ absent (and (g ?x ?y) (h ?y))`; seeds `A`, `B`, a `g`–`h` chain from `A` only | `(ok B)` only | a guard over a conjunction is ¬∃ over its unbound variables |
| `gated ← (or (t1 ∧ absent r1) (t2 ∧ absent r2))`; `r2 ← raw`; given `t2`, `raw` | no `gated` | each disjunct's guards are judged with that disjunct, at the boundary |
| `non-neighbour ← anchor ∧ other ∧ absent (next-to …)`, with `next-to` derived from `right-of` and closed by symmetry | no `non-neighbour` | a guard sees the closed extent of what it reads, however many firings built it |

## What holds it

| claim | test |
|---|---|
| a guard is judged against the positive fixpoint; the rules' written order decides nothing derivable | `ein-infer/tests/naf_semantics.rs`: `the_order_does_not_decide_what_is_derivable` |
| an unstratifiable loop converges to a supported fixpoint, and nothing is retracted | `ein-infer/tests/naf_semantics.rs`: `an_unstratifiable_loop_converges_to_a_supported_model` |
| exactly one of `p` and `q` is derived on `examples/features/07_unstratifiable.ein` | the corpus shape digest of its `saturate` run, `ein-render/tests/id_order_invariance.rs` |
| a guard over a conjunction is ¬∃ over its unbound variables | `ein-infer/tests/naf_semantics.rs`: `a_guard_over_a_conjunction_is_a_negated_existential` |
| a disjunct's guards are judged at the boundary with that disjunct | `ein-infer/tests/naf_semantics.rs`: `an_or_disjuncts_guard_is_judged_on_the_boundary` |
| a guard sees the closed extent of the relation it reads | `ein-infer/tests/naf_semantics.rs`: `a_guard_sees_the_closed_extent_of_its_watched_relation` |
| where a guard is written decides which question it asks | `ein-infer/tests/rule_semantics.rs`: `a_binding_guard_binds_where_it_is_written` |
| a rule's parameter is bound when its guards are judged | `ein-infer/tests/naf_semantics.rs`: `a_rule_parameter_is_in_scope_for_its_guards` |
| `(unknown P)` admits exactly the pairs neither stored nor negated | `ein-infer/tests/naf_semantics.rs`: `unknown_admits_exactly_the_undecided_pairs` |
| a world whose closure quiesced while a parked match would pass is not at its fixpoint | `ein-infer/tests/naf_semantics.rs`: `the_boundary_decides_whether_a_saturator_is_stalled` |
| a `forall` that turns from failing to passing is judged again and admitted; a node and a fresh saturation agree at the boundary | `ein-infer/tests/explain_semantics.rs`: `fork_and_direct_agree_at_the_naf_boundary` |
| a match inherited parked from root is judged again in the node exactly where what its guard reads grew | `ein-infer/tests/naf_semantics.rs`: `a_forked_candidate_is_rejudged_exactly_when_its_watched_extent_grew` |
| an admitted firing records the absences it rested on, a free position as free, and a positive firing records none | `ein-infer/tests/naf_semantics.rs`: `a_boundary_admitted_firing_records_what_was_absent`, `a_free_position_in_a_negative_premise_is_recorded_as_free`, `a_nested_guard_records_both_levels` |
| matching is structural: two names are two objects | `ein-infer/tests/naf_semantics.rs`: `matching_does_not_resolve_equality_classes` |
| a refutation resting on an extendable `absent` is refused, and the line names the forms that say what was meant | `ein-infer/tests/refutation_under_absent.rs`: `the_probe_is_refused_and_the_message_names_the_replacements`, `the_written_activator_programs_are_refused` |
| every node is root's fixpoint and its commitment, as the oracle builds it | `ein-infer/tests/explore.rs`: `every_mode_is_the_oracle` |
| a negative cycle is reported and the program still answered | `ein-cli/tests/strata_diagnostic.rs`: `solve_names_each_negative_cycle_and_still_answers` |
