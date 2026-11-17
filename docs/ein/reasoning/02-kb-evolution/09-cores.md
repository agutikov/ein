# Cores: what a contradiction rests on

A dead node's **core** is the set of facts its contradiction rests on: the givens and the commitments from which,
through the derivations the node recorded, its contradiction follows. It is an explanation of why the node is dead,
not part of the answer. This page states what a contradiction is, what a fact records about how it came to be, the
proof graph those records form, what a core is over that graph and how far its minimality goes, and what a
refutation tells a search.

## A contradiction

A world holds a **contradiction** when it holds `(false)`, which a rule concludes to say a state is impossible, or a
fact beside its stored negation, `F` and `(not F)`. A world that holds one is **dead**; one that holds none is
**alive**. A fact's origin plays no part: `F` and `(not F)` are a contradiction whether each was given, derived or
committed.

A world never loses a fact, so a contradiction once derived is never retracted: a dead world stays dead however far
its saturation runs. That is what lets a contradiction be detected as each fact lands rather than at quiescence, and
lets a route stop a dead node at its first contradiction on a monotone program
([saturation](02-saturation.md#a-node-resumes-roots-saturation)).

## A derivation record

Every fact a world holds records how it came to be there
([Q18](../../../decisions/engine/Q18-provenance-is-per-fact-an-or-of-derivations-over.md)):

| kind | what it records |
|---|---|
| **given** | the fact as the program states it, with its `:source` where the program writes one |
| **a rule's firing** | the rule, the activator it was applied with, the positive premises it read, its bindings, the facts a `forall` it read ranged over, and the `(absent …)` queries that had to fail for it |
| **a commitment** | the hypothesis a node assumed |

A fact's identity is its relation and its arguments, so a fact derived twice is one fact; the second derivation is
recorded beside the first, as an **alternative**. A given and a commitment take none: a clue that happens to be
re-derivable is still a clue.

## The proof graph

- A **fact** is an **OR-node**: it holds if any of its recorded derivations holds.
- A **derivation** is an **AND-node** over its premises: it holds only if all of them do. A derivation through a
  `forall` rests on each fact the universal read
  ([Q-M1e2.15](../../../decisions/answer/Q-M1e2.15-a-derivation-through-a-forall-rests-on-each-fact.md)); one made
  through an activator a node derived or committed rests on that activator too, while root's activators, stated or
  derived, are the program's and no core follows them
  ([Q-M1e2.12](../../../decisions/answer/Q-M1e2.12-a-dead-nodes-core-follows-an-activator-only.md)).
- The **primary** derivation is the first one recorded; the others are alternatives.
- The **terminals** are the givens and the commitments. Every walk over the graph grounds out on them.

```text
   given A   given B        given C
       \      /                |
     primary (rule R₁)    alternative (rule R₂)
              \               /
                  fact F

   following the primary alone, F rests on {A, B}; searching both, on {C}
```

The graph is legitimately **cyclic**: once re-derivations are recorded, symmetric and transitive closure make
`(R a b)` and `(R b a)` justify each other in any ordinary puzzle. Every walk over recorded provenance is therefore
cycle-safe, and a fact is never its own reason.

## The core

**Which frontier.** Over an OR-node, which terminals a fact rests on is a choice, and two choices differ:

- *one derivation per fact*, the primary, unioned over the contradiction's facts: cheap, the shape a trace
  renders, but it overstates the conflict and moves with the order derivations happened to be recorded in;
- *a search across every recorded derivation*: the combination of derivations, one per fact, whose terminals are
  fewest. A saving can lie *between* facts, two facts that both derive from the same pair, which no per-fact choice
  sees.

**A core is the second**: the smallest frontier over every derivation of its contradiction the node records
([Q-M1e2.11](../../../decisions/answer/Q-M1e2.11-a-core-is-the-smallest-over-every-derivation.md)). A
contradiction's facts are explained together, a pair's two sides as one AND-node. On a program whose root is alive a
node's core names a fact of the node's commitment, since root's facts refute nothing, and root's fixpoint with the
core's commitment facts written in is dead.

**An empty core and no core are two answers.** A dead node whose refutation rests on no fact, `(rule r () :match ()
:assert (false))`, has the empty core: the empty set suffices. A node that is not dead has no core at all
([Q-M1e2.27](../../../decisions/read-out/Q-M1e2.27-in-eins-json-is-the-empty-set-and-null-no-object.md)).

**A core names facts.** The absences a derivation rested on are recorded, and a core does not count them: an
absence is no fact a node could be shown. What reads them is the check of a program that may not be monotone
([monotonicity](08-monotonicity.md)).

**`refuted by`**, beside a core, names for each fact of the contradiction the rule of the last step of a derivation
from the core: the fact's first recorded derivation where the core derives it, else the first alternative that it
derives ([Q-M1e2.21](../../../decisions/read-out/Q-M1e2.21-refuted-by-names-the-first-recorded-derivation.md)).

## How far the minimality goes

Every statement that a core is *smallest* carries three bounds.

1. **Not a subset-minimal unsatisfiable set.** No proper subset of a core is checked for satisfiability. The
   textbook way to get one, deleting a fact and saturating again, is unsound under `absent`: removing a fact can make
   a guard pass and fabricate a contradiction the full world never had
   ([`absent`: negation as failure](03-absent-and-naf.md#what-follows)). A core is found by search over the recorded
   derivations instead.
2. **Over the derivations recorded.** The derivations are the firings the saturation made, so minimality is
   relative to the rule set and the order it fired in. A store may keep a bounded number of derivations per fact,
   those with the fewest premises and beside them the one with the smallest closure; no list of fixed length is every
   derivation, so a smaller core can lie among the derivations a store did not keep.
   `(config :record-alternative-justifications false)` keeps each fact's first derivation alone; it is the one option
   that may change a core, and it changes nothing else
   ([Q-M1e2.11](../../../decisions/answer/Q-M1e2.11-a-core-is-the-smallest-over-every-derivation.md)).
3. **Budgeted.** The smallest frontier over an AND/OR graph is exponential to find in the worst case, so the search
   runs under a budget. A search the budget cut is still sound, every frontier it returns a real set of terminals
   the contradiction follows from, and every surface that prints its core says it was cut.

**A core is an explanation.** A dead node usually has many refutations, and which one a saturation meets first is a
matter of order. A core may vary with the rules' written order and with the agenda's, as long as it is a valid
core; R, every state and every count do not depend on it
([Q-M1f.6](../../../decisions/answer/Q-M1f.6-a-core-is-an-explanation.md)). Taking one is a search the
classification of a node does not need, so a run takes a core only where a surface prints it, and which cores it
takes moves no answer
([Q-M1e2.15](../../../decisions/answer/Q-M1e2.15-a-derivation-through-a-forall-rests-on-each-fact.md)); every core a
run prints is taken against the one preparation of root every node of that run comes from
([Q-M1f.25](../../../decisions/answer/Q-M1f.25-every-core-a-run-prints-is-taken-against-its-one.md)).

## What a refutation tells a search

On a **monotone** program `dead` is closed upward: a world only grows, so every superset of a dead commitment is
dead, and one dead node decides every node above it. A route may use that as a licence and conclude *dead* for a
node it never saturates ([finding R](07-finding-r.md)). The licence is exactly what an `absent` over a relation a
commitment can still extend would break, `{(p A)}` dead and `{(p A), (q A)}` alive, which is why a refutation may
not rest on one ([`absent`: negation as failure](03-absent-and-naf.md#a-refutation-resting-on-an-absent)). On a program
that may not be monotone no route takes the licence, and each node is computed
([monotonicity](08-monotonicity.md)).

## What holds it

| claim | test |
|---|---|
| every core a run prints names a fact of its commitment and, with root, derives its contradiction; `refuted by` names its derivation's last step | `ein-infer/tests/corpus_cores.rs`: `every_core_names_its_commitment_and_derives_its_contradiction` |
| a rule with no premise refutes root with the empty core | `ein-infer/tests/corpus_cores.rs`: `a_rule_with_no_premise_refutes_root_with_the_empty_core` |
| a node that is not dead has no core | `ein-cli/tests/answer_round_trip.rs`: its `no_core_unless_dead` check, in `every_state_file_reloads_to_its_own_facts_by_value_*`, over every corpus dump |
| no core is printed unqualified from a cut search | `ein-infer/tests/corpus_cores.rs`: `no_core_is_printed_unqualified_from_a_cut_search` |
| every budget cap stays sound and reports itself, each by name | `ein-infer/tests/explain_semantics.rs`: `every_budget_cap_stays_sound_and_reports_itself`, `each_cap_a_search_stops_at_is_named` |
| the search finds a saving between two facts that a per-fact choice misses | `ein-infer/tests/explain_semantics.rs`: `the_search_finds_a_shared_optimum_a_greedy_pick_would_miss` |
| the search terminates on a cyclic graph and grounds out on givens | `ein-infer/tests/explain_semantics.rs`: `the_search_terminates_and_grounds_out_on_a_cyclic_justification_graph` |
| a given explains itself; a consistent world and an underivable fact explain to nothing | `ein-infer/tests/explain_semantics.rs`: `a_given_explains_itself`, `a_consistent_kb_and_an_underivable_target_explain_to_nothing` |
| a full list keeps the smallest-closure arrival, and a pair's two sides report one core | `ein-infer/tests/explain_semantics.rs`: `a_full_list_keeps_the_smallest_closure_and_the_pair_report_one_core` |
| a bounded store can hide a smaller core: a saving between two facts behind a full list is refused | `ein-infer/tests/explain_semantics.rs`: `a_saving_between_two_facts_behind_a_full_list_is_still_refused` |
| a core follows an activator only a node derives, and covers both sides of a pair and what a `forall` read | `ein-infer/tests/region.rs`: `a_dead_nodes_core_follows_an_activator_only_a_node_derives`, `a_contradictions_core_covers_both_sides_and_what_a_forall_read` |
| the core is the one option-sensitive output: every line of every mode is the same under every flag but the core's under `:record-alternative-justifications` | `ein-cli/tests/lever_invariance.rs`: `no_config_flag_moves_a_line_of_any_mode`, `the_core_exemption_is_one_flag_and_the_core_rows` |
| every question and claim answers alike under two orders, a core held as a valid explanation | `ein-infer/tests/agenda.rs`: `every_question_and_claim_answers_alike_under_the_agenda_and_the_written_order` |
| a run that prints no core takes none and answers alike | `ein-infer/tests/explore.rs`: `a_run_that_prints_no_core_takes_none_and_answers_alike` |
| a member of N's core is the one an eager run takes | `ein-cli/tests/answer_round_trip.rs`: its `eager_cores` check, in the same tests |
| a dead root's core names the injected clue, with its `:source` | `ein-infer/tests/acceptance.rs`: `the_contradictions_view_is_the_verdict_explained`, `the_injected_clash_carries_the_source_that_names_it` |
