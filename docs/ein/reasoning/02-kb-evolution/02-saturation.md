# Saturation

Saturation is the one computation every answer is made of. Root is the program's own facts saturated by its rules;
every node of the search is root's fixpoint with a commitment written in, saturated again; the candidate set H is
read off root's fixpoint, and a node's dead and owing bits off its own ([solution semantics](06-solution-semantics.md)).
This page states what saturation computes, the order it computes it in and why that order decides no answer, what it
derives before anything is supposed, and the invariant that lets H be taken once, at root.

## The fixpoint

`sat(K)` is the least knowledge base containing `K` that every rule of the program leaves unchanged. A rule is
applied once per activator fact of its parameters, and each application, a rule with its activator and the bindings
of its premises, fires at most once ([values and their order](../01-static-ir/03-values-and-order.md)). Firing adds the
facts its conclusion states, and stored negations among them; it never removes one. Saturation is so **inflationary**:
within one world the fact set only grows, and nothing is retracted.

It is monotone in its input only where no rule reads an absence. With `(absent P)` a larger input can block a firing
a smaller one admits, so what `sat` is on a program that reads absences is fixed by the next section and by
[`absent`: negation as failure](03-absent-and-naf.md), and *the least fixpoint* above is read on the positive part of
the program, with the boundary deciding the rest.

A program can describe a closure with no end: a rule may build a fact out of a fact, so nesting can grow without
bound. Everything stated about an answer is about programs whose saturation reaches a fixpoint.

## Two phases: the closure and the boundary

Saturation alternates two phases, and that shape is the contract:

```text
saturate(W):
  loop
    closure:   fire every positive match to quiescence; a match whose rule reads an absent is parked
    boundary:  judge the parked matches against the quiescent W, in the agenda's order;
               admit the first whose every guard passes, or stop if none does
```

- **The closure** consults no negation. It fires the purely positive part of each rule; a match whose rule carries
  an `(absent …)` premise is parked rather than fired.
- **The boundary** speaks only at the closure's quiescence. It judges the parked matches against that fixpoint and
  admits **at most one**, which fires at once into an empty queue and resumes the closure. One admission per round is
  a soundness requirement: a batch would let one admission invalidate another's guard after its verdict was taken,
  and on `p ← absent q; q ← absent p` derive both.
- A parked match whose guard can still turn from failing to passing, a `forall`'s, stays parked; one whose guard
  has found a match it can never lose is dropped. The fixpoint is the first quiescence at which the boundary admits
  nothing.

## The agenda: the order rules fire in

Every saturation, root's, each node's, the oracle's and every reader's, takes its candidates in one order, the
**agenda**: first what leads to `(false)`, a rule concluding `(false)`, a stored negation, or a fact whose negation
some rule can conclude or the world holds; then the rest; and within each half first in, first out, in the written
order of the rules and the order matches arrive. The boundary walks its parked matches in the same order, and a
resumed saturation keeps it ([Q-M1f.313](../../../decisions/engine/Q-M1f.313-the-agenda-fires-first-what-leads-to-false.md)).

**On a monotone program the order decides no answer.** The fixpoint of a monotone program does not depend on the
order its rules fire in, so every question and every claim answers alike under the agenda and under the written
order alone. Taking first what can make a contradiction finds a dead node's contradiction sooner, and that is all the
order is for. What it does decide is *when* a fact is derived: where a dead node stops, how much of a derivation a
node narrates, and which of a fact's equally valid derivations its record keeps first, which a core is explained
from. A core is an explanation, and it may move with the order
([Q-M1f.6](../../../decisions/answer/Q-M1f.6-a-core-is-an-explanation.md)). Of a program that may not be monotone
the engine claims no answer under any order ([monotonicity](08-monotonicity.md)).

## A node resumes root's saturation

A node's saturation is root's, continued: it starts from root's fixpoint, with what root has fired and what it has
parked, adds the commitment's facts, and saturates to quiescence. It does not derive root's fixpoint again, and every
match it inherits parked is judged again in its own world before it can be admitted. The fixpoint it reaches is the
one a fresh saturation of root's facts and the commitment reaches, and so is every answer and every count; what may
differ is how many firings it counts and which derivation of a fact derived several ways its record keeps first
([Q-M1a.18](../../../decisions/engine/Q-M1a.18-a-node-resumes-roots-saturation.md)).

**A dead node may stop at its first contradiction.** A world never loses a fact, so a contradiction once derived is
never retracted, and every firing after it adds nothing to the node's status. On a monotone program a route may stop a
dead node's saturation at the firing that makes its first contradiction; the answer is the one the node run to
quiescence gives. The oracle, root, a program that may not be monotone, and any reader that needs the whole state run
to quiescence, and a core is always taken over the whole fixpoint ([finding R](07-finding-r.md)).

## What saturation derives before anything is supposed

Propagation decides everything it can before anything is supposed: H is proposed at root's fixpoint, never while a
saturation runs, and a node is judged as its rules leave it
([Q19](../../../decisions/hypotheses/Q19-hypotheses-are-proposed-at-a-fixpoint-after.md)). So what a solver by hand
concludes without guessing, saturation has to conclude too, and that includes the negatives.

**Negative completion.** A stored negation can be derived from positive evidence and a declared property, with no
search behind it. If `R` is functional and `(R a b)` holds, every `(R a b′)` with `b′ ≠ b` is false; if `R` is
injective, every `(R a′ b)` with `a′ ≠ a` is; a negative carries across a declared equivalence; and when every value
but one is excluded for an object, the last one holds (domain elimination), as when every object but one is excluded
for a value (range elimination). The standard library states these as rules over a relation's declared properties
(`std.bijection`, `std.elim`), so a puzzle writes one property per constraint and saturation fans it out. On the
Zebra puzzle that closes at depth zero every step of the human walkthrough: each *therefore* is reached from the
clues and the ontology without a hypothesis.

**The same inference over one generic relation.** A puzzle may instead link every attribute through one
equivalence relation, which is a bijection only between each pair of types. `std.slots` states the same inference
for that encoding, from a property scoped by a family of types, rule for rule: placement through an index,
exclusivity within a type, the negative across a link, elimination for a value and for a slot, the unique-neighbour
spatial rules. Two things are worth knowing about it. Elimination for a value and fill for a slot are both needed and
are not each other's mirror: the conclusions are the same edge, but they read different negatives and fire at
different times. And a symmetric relation needs its negative mirrored: a negative derived in one argument order
reaches no rule that reads the other. Both encodings of the Zebra puzzle reach the same model by the same
walkthrough.

**The closed-world reads are on what saturation determines.** Both libraries read under `absent` only on the
position structure, the neighbour, pruning and endpoint guards, which the clues fix before any hypothesis; they never
read an attribute the search decides. That is the bet a closed-world read makes, and it is exactly the line the
language draws: a refutation may not rest on an `absent` over a relation a commitment can still extend
([`absent`: negation as failure](03-absent-and-naf.md)).

## The alive-set invariant

The candidate set H is taken once, at root's fixpoint, and every route reads subsets of it. That is sound only if
nothing a node derives names what root's fixpoint and the program did not: a name that first appeared inside a node
would be an object no candidate is about, in a part of the lattice H was not built for, and a model naming it would be
missing from R. The invariant, *no rule asserts a name the program lacks*, has three clauses:

1. **No new objects.** A constant a rule writes, in its `:assert` or its `:match`, is a name of the program, as a
   constant a fact writes at any depth is
   ([Q-M1e2.9](../../../decisions/language/Q-M1e2.9-a-programs-names-are-the-constants-its-facts.md)), and the blind
   enumerator's objects are seeded with every name of the program
   ([Q-M1e.21](../../../decisions/hypotheses/Q-M1e.21-a-rule-brings-in-no-name.md)). A variable can carry only a
   value a match already found, so nothing but a constant can bring a name in, and every constant is the
   program's. A constant only a query writes is not a name of the program: adding a query never changes R.
2. **No new relations.** The relations a run can write are fixed at load: the declared ones, the heads of the
   program's facts, and every constant head a rule can conclude. Nothing adds a relation during a run.
3. **Hypotheses connect names.** A candidate's arguments are names of the program or facts built of them; a relation
   written as a parameter and bound to a value that names no relation, a nested fact, is refused where it is written
   or where a rule could derive it.

Three readings of the invariant, each stronger than the one before:

| | claim |
|---|---|
| **static** | every name of the program is among the blind enumerator's objects, read off the facts and the rules at load |
| **at the fixpoint** | nothing a run derives names what the loaded program, its facts and its rules' constants, did not |
| **in the search** | nothing a node reaches names what root's fixpoint and the rules' constants did not |

The third is the one R's soundness rests on, and the first is what makes it hold for a rule's constant: a constant a
rule writes only inside a commitment is an object from root on. The static reading answers for every run the program
could have, not only for the one that happened.

**The objects are not the list of names.** Four kinds of name are no object of the blind enumerator, each left out by
name: a relation signature's type roles, the kernel's words (`not`, `false`, `and`, `or`, `absent`, `eq`, `neq`), every
name the program calls a relation or a rule, and the kernel's two dunders, `__closed__` and `__symmetric__`, which a
program may write as a value and which stay names of the program
([Q-M1f.306](../../../decisions/hypotheses/Q-M1f.306-the-blind-enumerators-objects-are-the-programs.md)).

## What holds it

| claim | test |
|---|---|
| saturating a fixpoint again fires nothing | `ein-infer/tests/explain_semantics.rs`: `saturation_is_idempotent` |
| nothing is retracted: a fact enabled by a firing stays | `ein-infer/tests/naf_semantics.rs`: `an_unstratifiable_loop_converges_to_a_supported_model` |
| a world with a parked match that would pass is not at its fixpoint | `ein-infer/tests/naf_semantics.rs`: `the_boundary_decides_whether_a_saturator_is_stalled` |
| a rule with no `absent` parks nothing and never reaches the boundary | `ein-infer/tests/naf_semantics.rs`: `a_guard_free_rule_parks_nothing` |
| within each half of the agenda, rules fire first in, first out, in their written order | `ein-infer/tests/explain_semantics.rs`: `the_agenda_is_fifo_in_the_written_order` |
| every question and every claim of every monotone corpus program answers alike under the agenda and under the written order alone | `ein-infer/tests/agenda.rs`: `every_question_and_claim_answers_alike_under_the_agenda_and_the_written_order` |
| a node saturated from root's snapshot and the same facts saturated afresh reach one state, at the boundary too | `ein-infer/tests/explain_semantics.rs`: `fork_and_direct_saturation_agree`, `fork_and_direct_agree_at_the_naf_boundary` |
| an activator first derived in a node gets its rule applied in that node | `ein-infer/tests/explain_semantics.rs`: `a_runtime_derived_activator_gets_a_plan_on_a_fork` |
| a dead node stopped at its first contradiction gives every question the answer the node run to quiescence gives | `ein-infer/tests/first_contradiction.rs`: `every_question_answers_alike_with_the_stop_and_without` |
| a functional or injective positive excludes every other value or object; elimination forces the last survivor and says nothing while two survive | `ein-infer/tests/stdlib_semantics.rs`: `a_functional_positive_excludes_every_other_value`, `an_injective_positive_excludes_every_other_object`, `domain_elimination_forces_the_last_surviving_value`, `range_elimination_forces_the_last_surviving_object`, `domain_elimination_says_nothing_while_two_values_survive` |
| both Zebra encodings define and use the walkthrough's rules and reach the same walkthrough | `ein-render/tests/idea08_acceptance.rs`: `the_zebra2_trace_exhibits_the_walkthrough_rules`, `the_generic_trace_exhibits_the_walkthrough_rules`, `both_encodings_reach_the_same_walkthrough` |
| every name of the program is a candidate object | `ein-infer/tests/alive_invariant.rs`: `every_name_of_the_program_is_a_candidate_object` |
| no run reaches a name the program does not have, against either baseline | `ein-infer/tests/alive_invariant.rs`: `no_run_reaches_a_name_the_program_does_not_have` |
| a program and its twin that writes the same name in a fact answer one R | `ein-infer/tests/alive_invariant.rs`: `the_pair_answers_one_region` |
| a relation parameter bound to a fact, written or derivable, is refused at load | `ein-cli/tests/cli_semantics.rs`: `a_derived_activator_binding_a_relation_parameter_to_a_fact_is_refused_at_load`; `ein-infer/tests/compile_semantics.rs`: `a_relation_parameter_a_rule_binds_to_a_nested_fact_is_refused` |
| the search interns no name the load did not | `ein-infer/tests/interning.rs`: `the_search_interns_no_names`, `a_rule_only_constant_is_interned_at_load` |
| a dunder is a name of the program where it is written as a value, and no object | `ein-infer/tests/dunder_objects.rs`: `the_programs_names_hold_a_dunder_where_it_is_written_as_a_value`, `a_dunder_written_as_a_value_is_no_object` |
