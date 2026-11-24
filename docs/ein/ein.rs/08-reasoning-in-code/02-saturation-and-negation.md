# Saturation and negation in code

Six pages of the reasoning state what a firing, a fixpoint, an absence, a monotone program, a core and a claim
are, with no word about this engine:
[firing a rule](../../reasoning/02-kb-evolution/01-firing.md),
[saturation](../../reasoning/02-kb-evolution/02-saturation.md),
[absence](../../reasoning/02-kb-evolution/03-absent-and-naf.md),
[monotone and non-monotone programs](../../reasoning/02-kb-evolution/08-monotonicity.md),
[cores](../../reasoning/02-kb-evolution/09-cores.md) and
[satisfiability](../../reasoning/02-kb-evolution/10-satisfiability.md). This page is their other half: for each
notion, **where it lives** in `ein.rs`, **the algorithm** the engine uses for it, **the tests** that fail when the
engine stops doing what the reasoning says, and **the commands** that show it. The design contracts carry the
algorithms in full ([`03-design/06-saturation.md`](../03-design/06-saturation.md),
[`05-matcher.md`](../03-design/05-matcher.md), [`07-search.md`](../03-design/07-search.md)); this page is the map
from a statement of the reasoning to them.

Every test below is run as its file, from the repository's root:

```sh
cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-infer --test naf_semantics
```

with the crate after `-p` and the file's name after `--test`. In the commands, `ein` is the built binary.

---

## 1. Firing a rule

The reasoning: [firing a rule](../../reasoning/02-kb-evolution/01-firing.md).

### 1.1 Where it lives

| notion | code |
|---|---|
| an object, a fact, a stored negation | [`ein-core/src/value.rs`](../../../../ein.rs/crates/ein-core/src/value.rs), [`terms.rs`](../../../../ein.rs/crates/ein-core/src/terms.rs), [`facts.rs`](../../../../ein.rs/crates/ein-core/src/facts.rs): atoms and facts interned together, a fact a row of relation and arguments, `(not F)` a fact of `not` |
| a knowledge base, a fork | [`ein-core/src/kb.rs`](../../../../ein.rs/crates/ein-core/src/kb.rs): a stack of immutable layers under one writable top ([`03-data-model.md`](../03-design/03-data-model.md)) |
| a derivation record | [`ein-core/src/prov.rs`](../../../../ein.rs/crates/ein-core/src/prov.rs), `Prov`: kind, source, rule, premises, bindings in bind order, the absences, the branch |
| an application: a rule and its activator | [`ein-infer/src/compile.rs`](../../../../ein.rs/crates/ein-infer/src/compile.rs) compiles one plan per (rule, activator) pair, [`plan.rs`](../../../../ein.rs/crates/ein-infer/src/plan.rs) is the plan, [`engine.rs`](../../../../ein.rs/crates/ein-infer/src/engine.rs) the cache of plans and the record of what fired |
| satisfaction, $K \models \varphi\theta$ | [`ein-infer/src/match_.rs`](../../../../ein.rs/crates/ein-infer/src/match_.rs), the register matcher; `eq` and `neq` in [`predicates.rs`](../../../../ein.rs/crates/ein-infer/src/predicates.rs) |
| a firing and its effect | [`ein-infer/src/firing.rs`](../../../../ein.rs/crates/ein-infer/src/firing.rs): the conclusion instantiated, one `Prov` shared by every fact of one firing, and the key that says an identity already fired |
| dead | [`ein-infer/src/contradiction.rs`](../../../../ein.rs/crates/ein-infer/src/contradiction.rs): `has_contradiction`, a firing of a rule that states `:ein inconsistent` recorded on the KB (`ein_core::Kb::ended`, written by `ein_infer::firing::end`), and `detect`, what an explanation reads of it; the rule every program gets on a held `(false)` is [`stdlib/core.ein`](../../../../stdlib/core.ein) |
| the engine's word, a rule's conclusion in the place of literals | `ein_core::EngineWord`, `ein_core::Rule::ein`; a plan that states it, `Plan::ends` |
| a state's identity | [`ein-infer/src/canon.rs`](../../../../ein.rs/crates/ein-infer/src/canon.rs): the sorted fact list, never a hash of it |
| a firing's identity | `ein_infer::firing::BindingKey`: the plan, the activator, the registers; the activator half is the compile key's, `ein_infer::compile::PlanKey` |
| a fork | `ein_core::kb::Kb::fork`: immutable layers below one writable top ([`03-design/03-data-model.md` § 5.1](../03-design/03-data-model.md#51-the-fork)); the fork's own derivation records, `ein_core::prov::ProvArena`'s fork region |
| no output depends on the numbering of ids | the contract and its rules, [`03-design/01-determinism-and-order.md`](../03-design/01-determinism-and-order.md) |

### 1.2 The algorithm

A rule is compiled once for each activator: the activator's arguments are substituted for the parameters and
the pattern is lowered to a plan of scan, join, guard and absent-guard steps over registers
([`05-matcher.md` § 2](../03-design/05-matcher.md#2-from-a-rule-to-a-plan)). The matcher runs a plan and hands
each match to a callback; the saturator turns a match into a firing, looks every conclusion up, and inserts the
ones the knowledge base lacks with one shared derivation record
([`06-saturation.md` § 6](../03-design/06-saturation.md#6-firing-redundancy-alternatives)). A firing whose every
conclusion is present inserts nothing and is recorded as an alternative derivation.

### 1.3 What holds it

| claim | test |
|---|---|
| one engine, three readings, chosen by the input and never by which question was asked; a contradiction's core names the culprit | `ein-infer/tests/acceptance.rs`: `the_three_readings_agree_with_the_input`, `the_contradictions_view_is_the_verdict_explained` |
| a satisfiable puzzle is never reported unsatisfiable, and an unsatisfiable one never solved | `ein-infer/tests/acceptance.rs`: `a_satisfiable_puzzle_is_never_unsat_and_an_unsat_one_is_never_solved` |
| the two Zebra encodings, [`examples/zebra.ein`](../../../../examples/zebra.ein) with one generic relation and [`examples/zebra2.ein`](../../../../examples/zebra2.ein) with five, reach the same model | `ein-infer/tests/acceptance.rs`: `both_ontologies_reach_the_same_model`; `ein-cli/tests/acceptance_cli.rs`: `solve_zebra_prints_the_same_answer_through_the_generic_encoding` |
| every route computes the R the definition gives | `ein-infer/tests/explore.rs`: `every_mode_is_the_oracle` |
| matching is structural, and two atoms are never resolved as one | `ein-infer/tests/naf_semantics.rs`: `matching_does_not_resolve_equality_classes` |
| nothing is retracted | `ein-infer/tests/naf_semantics.rs`: `an_unstratifiable_loop_converges_to_a_supported_model` |
| a fork and its parent diverge after the fork, each seeing only its own additions | the store's unit test `a_fork_reads_base_then_delta_and_the_two_histories_diverge` in `ein-core/src/kb.rs` |
| no rendering of any corpus program depends on the order its objects were numbered in | `ein-render/tests/id_order_invariance.rs`: `every_rendering_reproduces_its_digest_and_none_depends_on_the_id_order`, `an_id_order_that_reaches_an_output_is_caught` |
| no rendering depends on how many workers ran | `ein-render/tests/jobs_invariance.rs`: `jobs_does_not_move_any_observable` |
| a rule written once with a parameter in a relation's place fires for each relation its activators name | `ein-infer/tests/stdlib_semantics.rs`: `a_bijective_declaration_fans_out_into_the_operational_activators` |
| a firing of a rule that states the engine's word writes no fact and ends the state, and a stopped saturation stops there | `ein-infer/src/saturator.rs`' unit tests `a_rule_that_states_the_key_ends_the_state_and_writes_no_fact`, `a_stopped_saturation_stops_at_the_firing_that_ends_the_state` |
| the kernel reads no `(false)` by its name, and judges no pair by itself | `ein-infer/src/contradiction.rs`' unit tests `a_held_false_is_no_contradiction_by_its_name`, `a_pair_with_no_rule_over_it_is_no_contradiction`, `a_pair_a_rule_refutes_is_read_as_the_pair` |

### 1.4 The drawings of the page

The rule and the two knowledge bases of the reasoning page are the engine's drawings of this program, folded,
the second knowledge base with the rule and the first without it:

```lisp
(relation is-a T T :why "{?3} is a {?4}")
(relation lives-in Person House :why "{?3} lives in {?4}")
(relation next-to House House :why "{?3} is next to {?4}")
(relation near Person House :why "{?3} lives near {?4}")

(is-a Norwegian Person) (is-a House-1 House) (is-a House-2 House)

(rule neighbour ()
  :match  (and (lives-in ?p ?h) (next-to ?h ?g))
  :assert (near ?p ?g)
  :why    "{?p} lives in {?h}, which is next to {?g}, so {?p} lives near {?g}")

(lives-in Norwegian House-1)
(next-to House-1 House-2)
```

```sh
ein render rule --fold --name neighbour FILE     # the rule, its match and its conclusion
ein render kb --fold FILE                        # the knowledge base, saturated
```

The page shows `ein render kb --fold`'s graph without the four declarations' own facts, two for each, as
`(relation lives-in Person House)` and `(relation lives-in)` for `lives-in`, which the view draws as an octagon whose
arrow `1` is at the box `relation` and a loop labelled `relation` on the box `lives-in`. It keeps root's `open` facts,
the blind enumerator's hypotheses, each a loop labelled `open by open` on the octagon of the fact it supposes, which
stands in the frame `open ctx`: `ein saturate --dump FILE` lists 22 without the rule and 21 with it, where root holds
`(near Norwegian House-2)` and does not guess it.

---

## 2. Saturation

The reasoning: [saturation](../../reasoning/02-kb-evolution/02-saturation.md).

### 2.1 Where it lives

| notion | code |
|---|---|
| $\operatorname{sat}$, the two phases | [`ein-infer/src/saturator.rs`](../../../../ein.rs/crates/ein-infer/src/saturator.rs): `Saturator::step` returns one firing, `Saturator::saturate` runs to the fixpoint |
| the agenda | `saturator::AGENDA`, which is `FailFirst::Pairs`; the switches `saturator::fifo` and `saturator::fail_first` exist for a test and a measurement, and no option reaches them |
| a node continues root's saturation | `Saturator::snapshot` and `Saturator::resume` |
| a dead node stopped at its first contradiction | `Saturator::stop_at_first_contradiction` |
| the alive-set invariant | [`ein-infer/src/invariant.rs`](../../../../ein.rs/crates/ein-infer/src/invariant.rs): `program_names`, `rule_constants`, and `Universe::breaches`, the predicate a run can be held to |
| negative completion | Ein rules, not engine code: `std.bijection` and `std.elim` over a relation's declared properties, `std.slots` over one generic relation, in [`stdlib/`](../../../../stdlib/) |

### 2.2 The algorithm

Saturation is a loop of two phases, a semi-naive positive closure and a boundary that admits one parked candidate a
round, both in the agenda's order; a node takes root's snapshot and resumes over a fork of root's knowledge base,
seeded with the commitment's facts. The loop, the closure step, the enqueue pass and the boundary round are told
once, as flows and pseudocode, in
[`02-algorithms.md` § 4](../02-algorithms.md#4-saturation-the-two-phase-loop-and-the-agenda), and
[`06-saturation.md`](../03-design/06-saturation.md) states each as a contract, with the queue, the delta and the
parked set and the events `park`, `admit` and `retire` ([`05-events.md`](../05-events.md)).

### 2.3 What holds it

| claim | test |
|---|---|
| saturating a fixpoint again fires nothing | `ein-infer/tests/explain_semantics.rs`: `saturation_is_idempotent` |
| nothing is retracted: a fact enabled by a firing stays | `ein-infer/tests/naf_semantics.rs`: `an_unstratifiable_loop_converges_to_a_supported_model` |
| a world with a parked match that would pass is not at its fixpoint | `ein-infer/tests/naf_semantics.rs`: `the_boundary_decides_whether_a_saturator_is_stalled` |
| a rule with no `absent` parks nothing and never reaches the boundary | `ein-infer/tests/naf_semantics.rs`: `a_guard_free_rule_parks_nothing` |
| within each half of the agenda, rules fire first in, first out, in their written order | `ein-infer/tests/explain_semantics.rs`: `the_agenda_is_fifo_in_the_written_order` |
| every question and every claim of every monotone corpus program answers alike under the agenda and under the written order alone | `ein-infer/tests/agenda.rs`: `every_question_and_claim_answers_alike_under_the_agenda_and_the_written_order` |
| a node saturated from root's snapshot and the same facts saturated afresh reach one state, at the boundary too | `ein-infer/tests/explain_semantics.rs`: `fork_and_direct_saturation_agree`, `fork_and_direct_agree_at_the_naf_boundary`; the saturator's unit test `a_resumed_saturation_reaches_the_same_fixpoint` |
| two activators differing in any argument both fire, in either order, and the order they are written in loses no derivation | `ein-infer/tests/rule_semantics.rs`: `activators_differing_only_by_an_int_argument_both_fire`, `activators_differing_only_by_a_nested_fact_argument_both_fire`, `an_int_beside_a_nested_fact_in_one_position_loses_no_derivation`, `an_int_and_a_string_with_one_text_are_two_activators`; the corpus programs `examples/ein-bugs/binding-key-both.ein`, `binding-key-swapped.ein`, `binding-key-int-only.ein`, `binding-key-nested-only.ein`, `activator-text-int-and-string.ein` and `activator-text-string-then-int.ein` |
| an activator first derived in a node gets its rule applied in that node | `ein-infer/tests/explain_semantics.rs`: `a_runtime_derived_activator_gets_a_plan_on_a_fork` |
| a dead node stopped at its first contradiction gives every question the answer the node run to quiescence gives | `ein-infer/tests/first_contradiction.rs`: `every_question_answers_alike_with_the_stop_and_without` |
| a functional or injective positive excludes every other filler of the place; elimination forces the last survivor and says nothing while two survive | `ein-infer/tests/stdlib_semantics.rs`: `a_functional_positive_excludes_every_other_value`, `an_injective_positive_excludes_every_other_object`, `domain_elimination_forces_the_last_surviving_value`, `range_elimination_forces_the_last_surviving_object`, `domain_elimination_says_nothing_while_two_values_survive` |
| both Zebra encodings define and use the walkthrough's rules and reach the same walkthrough | `ein-render/tests/idea08_acceptance.rs`: `the_zebra2_trace_exhibits_the_walkthrough_rules`, `the_generic_trace_exhibits_the_walkthrough_rules`, `both_encodings_reach_the_same_walkthrough` |
| every name of the program is a guessable atom | `ein-infer/tests/alive_invariant.rs`: `every_name_of_the_program_is_a_candidate_object` |
| no run reaches a name the program does not have, against either baseline | `ein-infer/tests/alive_invariant.rs`: `no_run_reaches_a_name_the_program_does_not_have` |
| a program and its twin that writes the same name in a fact answer one R | `ein-infer/tests/alive_invariant.rs`: `the_pair_answers_one_region` |
| a parameter in a relation's place bound to a fact, written or derivable, activates nothing, and nothing says so | `ein-cli/tests/cli_semantics.rs`: `an_activator_binding_a_relation_parameter_to_a_fact_activates_nothing_in_silence`; `ein-infer/tests/compile_semantics.rs`: `a_relation_parameter_a_rule_binds_to_a_nested_fact_activates_nothing` |
| the search interns no name the load did not | `ein-infer/tests/interning.rs`: `the_search_interns_no_names`, `a_rule_only_constant_is_interned_at_load` |
| a dunder is a name of the program where it is written as an argument, and not guessable | `ein-infer/tests/dunder_objects.rs`: `the_programs_names_hold_a_dunder_where_it_is_written_as_a_value`, `a_dunder_written_as_a_value_is_no_object` |

The agenda's row is a sweep of the corpus: it holds the order to deciding no answer on the monotone programs
the corpus has, not on every program the static check calls monotone.

### 2.4 Commands and drawings

```sh
ein saturate FILE                 # root's saturation alone: the counts before and after, the phase timings
ein solve FILE --trace OUT.md     # the region trace: root's derivation step by step, a drawing a step
ein solve FILE --events OUT.jsonl # every firing, park, admit and retire, one JSON object a line
```

The derivation drawn on the reasoning page is the two steps `ein solve --trace` draws for this program, each fact
an octagon with its arrows at its relation and its arguments, joined at the fact they share and with each firing
labelled by its rule alone; the cores page adds the two facts of `is-a` and marks the terminals:

```lisp
(relation is-a T T :why "{?3} is a {?4}")
(relation lives-in Person House :why "{?3} lives in {?4}")
(relation next-to House House :why "{?3} is next to {?4}")
(relation near Person House :why "{?3} lives near {?4}")
(relation owns Person Pet :why "{?3} owns {?4}")

(is-a Norwegian Person) (is-a House-1 House) (is-a House-2 House) (is-a Fox Pet)

(rule neighbour ()
  :match  (and (lives-in ?p ?h) (next-to ?h ?g))
  :assert (near ?p ?g)
  :why    "{?p} lives in {?h}, which is next to {?g}, so {?p} lives near {?g}")

(rule fox-keeps-away ()
  :match  (and (owns ?p Fox) (near ?p House-2))
  :assert (false)
  :why    "{?p} owns the fox and lives near the second house, which cannot be")

(lives-in Norwegian House-1 :source "the Norwegian lives in the first house")
(next-to House-1 House-2 :source "the first house is next to the second")
(owns Norwegian Fox :source "the Norwegian owns the fox")
```

Its read-out names the three stated facts as the core and `fox-keeps-away` as what refutes root.

**The activator example.** The page's rule `note` with its two activators is
[`examples/ein-bugs/binding-key-both.ein`](../../../../examples/ein-bugs/binding-key-both.ein); `binding-key-swapped.ein`
writes the two activators in the other order, and there is one file for each activator alone. The rule's drawing
is `ein render rules --fold`' of the file and the state's `ein render kb --fold`'s, without the declarations' facts,
the activators and the fact `(src Y)` the second activator points at; root's two `open` facts, below, are drawn, each
a loop labelled `open by open` on the octagon of `(holds 2)` or `(holds Y)` in the frame `open ctx`.

```sh
ein saturate examples/ein-bugs/binding-key-both.ein --dump
ein saturate examples/ein-bugs/binding-key-swapped.ein --dump
```

Both print the same derived facts, and beside them root's `open` facts, the blind enumerator's hypotheses, which
the dump lists with the state:

<!-- output -->
```text
;; ── DERIVED (4 facts) ──
;;   noted (2)
  (noted 1 1 :rule note)
  (noted 2 1 :rule note)
;;   open (2)
  (open (holds 2) :rule open)
  (open (holds Y) :rule open)
```

---

## 3. Absence

The reasoning: [absence](../../reasoning/02-kb-evolution/03-absent-and-naf.md).

### 3.1 Where it lives

| notion | code |
|---|---|
| a guard set aside | [`ein-infer/src/compile.rs`](../../../../ein.rs/crates/ein-infer/src/compile.rs): every top-level `(absent …)` is lifted out of its disjunct into the plan's guards, so a plan's steps are a positive program ([`05-matcher.md` § 4](../03-design/05-matcher.md#4-predicates-and-guards)) |
| parked, admitted, dropped | [`ein-infer/src/saturator.rs`](../../../../ein.rs/crates/ein-infer/src/saturator.rs): the parked set and the boundary's round, one admission each |
| the absences a derivation rested on | `Prov::absent` in [`ein-core/src/prov.rs`](../../../../ein.rs/crates/ein-core/src/prov.rs); no walk for a core follows them ([`ein-core/src/walks.rs`](../../../../ein.rs/crates/ein-core/src/walks.rs)) |
| a refutation resting on an `absent` | a site of the static check, [`ein-infer/src/monotone.rs`](../../../../ein.rs/crates/ein-infer/src/monotone.rs); [`ein-infer/src/extendable.rs`](../../../../ein.rs/crates/ein-infer/src/extendable.rs): `relations` is the set of relations whose held extent ([the knowledge base](../../reasoning/01-static-ir/01-knowledge-base.md#the-relation-its-extent-a-fact-of-it)) a commitment can extend, $D^{+}$; [`naf_deps.rs`](../../../../ein.rs/crates/ein-infer/src/naf_deps.rs) is the advisory map of which guards read what |
| the dependency graph and its strata | [`ein-infer/src/strata.rs`](../../../../ein.rs/crates/ein-infer/src/strata.rs) |
| the universal and the *unknown* test | the macros `forall` and `unknown` of `std.macro`, which expand to nested `(absent …)` before a rule is compiled ([`02-syntax.md`](../../user/04-reference/02-syntax.md)) |

### 3.2 The algorithm

The compiler splits a match into disjuncts and each disjunct into its positive steps and its guards. A match of
the positive steps of a disjunct with guards is parked with its bindings; the boundary evaluates each guard as a
query over the quiescent knowledge base under those bindings, variables the guard alone holds left free. A guard
with no nested `absent` that fails is retired; one with a nested `absent` stays parked with a stamp of the sizes
of the held extents of the relations it reads, and is judged again when one of them grew. A candidate a node
inherits parked from root keeps its stamp, so it is judged again in the node exactly where what its guard reads
grew.

Call a relation *extendable* where a commitment can extend its held extent: the static check's $D$, followed
through every rule with parameters whose name, the relation of its activators, is extendable, each such rule read
from its pattern (`monotone::through_activators`). A rule that concludes `(false)` or a stored negation resting on
an `absent` over such a relation is read and compiled as any other: its `absent` is a site of the static check, so
the program is flagged and its routes are the cube's. The shape is
[`examples/ein-bugs/naf-upward-closure.ein`](../../../../examples/ein-bugs/naf-upward-closure.ein).

### 3.3 What holds it

| claim | test |
|---|---|
| a guard is judged against the positive fixpoint; the rules' written order decides nothing derivable where the facts a guard reads are concluded by a positive rule | `ein-infer/tests/naf_semantics.rs`: `the_order_does_not_decide_what_is_derivable` |
| an unstratifiable loop converges to a supported fixpoint, and nothing is retracted | `ein-infer/tests/naf_semantics.rs`: `an_unstratifiable_loop_converges_to_a_supported_model` |
| exactly one of `p` and `q` is derived on [`examples/features/07_unstratifiable.ein`](../../../../examples/features/07_unstratifiable.ein) | the corpus shape digest of its `saturate` run, `ein-render/tests/id_order_invariance.rs` |
| a guard over a conjunction is a negated existential over its unbound variables | `ein-infer/tests/naf_semantics.rs`: `a_guard_over_a_conjunction_is_a_negated_existential` |
| a disjunct's guards are judged at the boundary with that disjunct | `ein-infer/tests/naf_semantics.rs`: `an_or_disjuncts_guard_is_judged_on_the_boundary` |
| a guard sees the held extent of the relation it reads as the positive closure leaves it, the test's *closed extent* | `ein-infer/tests/naf_semantics.rs`: `a_guard_sees_the_closed_extent_of_its_watched_relation` |
| where a guard is written decides which question it asks | `ein-infer/tests/rule_semantics.rs`: `a_binding_guard_binds_where_it_is_written` |
| a rule's parameter is bound when its guards are judged | `ein-infer/tests/naf_semantics.rs`: `a_rule_parameter_is_in_scope_for_its_guards` |
| `(unknown P)` admits exactly the pairs neither stored nor negated | `ein-infer/tests/naf_semantics.rs`: `unknown_admits_exactly_the_undecided_pairs` |
| a world whose closure quiesced while a parked match would pass is not at its fixpoint | `ein-infer/tests/naf_semantics.rs`: `the_boundary_decides_whether_a_saturator_is_stalled` |
| a `forall` that turns from failing to passing is judged again and admitted; a node and a fresh saturation agree at the boundary | `ein-infer/tests/explain_semantics.rs`: `fork_and_direct_agree_at_the_naf_boundary` |
| a match inherited parked from root is judged again in the node exactly where what its guard reads grew | `ein-infer/tests/naf_semantics.rs`: `a_forked_candidate_is_rejudged_exactly_when_its_watched_extent_grew` |
| an admitted firing records the absences it rested on, a free position as free, and a positive firing records none | `ein-infer/tests/naf_semantics.rs`: `a_boundary_admitted_firing_records_what_was_absent`, `a_free_position_in_a_negative_premise_is_recorded_as_free`, `a_nested_guard_records_both_levels` |
| a program whose refutation rests on an `absent` over the held extent of an extendable relation loads, is flagged, and is answered with no licence | `ein-infer/tests/non_monotone_loads.rs`: `a_refutation_under_an_absent_loads_flagged_and_unlicensed`, `the_written_activator_programs_load_flagged_and_unlicensed` |
| every node is root's fixpoint and its commitment, as the oracle builds it | `ein-infer/tests/explore.rs`: `every_mode_is_the_oracle` |
| a negative cycle is reported and the program still answered | `ein-cli/tests/strata_diagnostic.rs`: `solve_names_each_negative_cycle_and_still_answers` |

The reasoning page's worked examples are the programs `naf_semantics.rs` writes inline, in the table's order.

### 3.4 Commands and drawings

The rule with its `absent` cluster and the saturated knowledge base of the reasoning page are
`ein render rule --fold --name soup-unless-allergic FILE` and `ein render kb --fold FILE` on, the knowledge base
without its declarations' facts:

```lisp
(relation is-a     T T         :why "{?3} is a {?4}")
(relation seats    Guest Chair :why "{?3} is seated at {?4}")
(relation allergic Guest       :why "{?3} is allergic")
(relation meal     Guest Food  :why "{?3} is served {?4}")

(is-a Ann Guest) (is-a Bob Guest) (is-a C1 Chair) (is-a C2 Chair) (is-a Soup Food)

(rule soup-unless-allergic ()
  :match  (and (seats ?g ?c) (absent (allergic ?g)))
  :assert (meal ?g Soup)
  :why    "{?g} is seated and not allergic, so {?g} has the soup")

(seats Ann C1)
(seats Bob C2)
(allergic Bob)
```

The drawing keeps root's `open` facts, the blind enumerator's hypotheses, each a loop labelled `open by open` on the
octagon of the fact it supposes, in the frame `open ctx`; `ein saturate --dump FILE` lists the 61. Among them is
`(open (allergic Ann))`, so the rule's `absent` reads a relation the enumerator guesses: the program is flagged, `ein
show hyps FILE` says so on stderr's first line and `ein solve FILE` in its read-out's first row, `WARNING`, and
nothing is claimed of its answer
([Q-M1f.564](../../../decisions/answer/Q-M1f.564-a-non-monotone-program-loads-with-a-warning.md)).

`ein solve FILE` prints a `strata` row for a program with a cycle through an `absent`, one `negative cycle` line
a cycle with its parity ([`08-read-out.md`](../../user/04-reference/08-read-out.md)).

---

## 4. Monotone and non-monotone programs

The reasoning: [monotone and non-monotone programs](../../reasoning/02-kb-evolution/08-monotonicity.md).

### 4.1 Where it lives

| notion | code |
|---|---|
| the static check: $D$, a read's class, a site | [`ein-infer/src/monotone.rs`](../../../../ein.rs/crates/ein-infer/src/monotone.rs): `monotone::classify` over the compiled plans, `Site`, `SiteKind`, `ReadClass`, `Monotonicity::is_monotone` |
| fact grain | a prototype beside the engine, [`ein-infer/examples/shared/fact_grain.rs`](../../../../ein.rs/crates/ein-infer/examples/shared/fact_grain.rs), whose verdicts `ein-infer/tests/region.rs` holds on the programs written for it; no route reads it |
| the cube | [`ein-infer/src/region.rs`](../../../../ein.rs/crates/ein-infer/src/region.rs), the oracle: every subset of the candidates saturated as a fork of root |
| the relations a commitment can extend, $D^{+}$ | [`ein-infer/src/extendable.rs`](../../../../ein.rs/crates/ein-infer/src/extendable.rs) |
| the detector | [`ein-infer/src/detector.rs`](../../../../ein.rs/crates/ein-infer/src/detector.rs): the static half off root's plans, the dynamic half off the nodes a run computes; `SPOT_PARENTS` is the number of one-step parents it builds for each printed state, three |
| what a route learns from a dead node | [`ein-infer/src/known_dead.rs`](../../../../ein.rs/crates/ein-infer/src/known_dead.rs), asked only on a monotone program |
| the static question alone | `explore::Checked` in [`ein-infer/src/explore.rs`](../../../../ein.rs/crates/ein-infer/src/explore.rs) |

### 4.2 The algorithm

`monotone::classify` reads root's compiled plans: it closes the candidates' relations under *a saturation plan
that reads facts of one concludes into the set*, classifies every read by that set and its `absent` depth, and
reports a site for each dynamic read at non-monotone polarity and for each rule whose name, the relation of its
activators, is in the set; and three of the hypotheses: a rule that reads `open` (`open-read`), a rule that
concludes `(open F)` and reads a relation of the set at even depth (`hypothesis`), and an obligation whose guard
reads one beside its witness (`obligation-guard`). It costs root's saturation and builds no node. A program with no
site takes the routes with their licences; a flagged one takes the cube
([`07-search.md` § 7](../03-design/07-search.md#7-monotone-and-flagged-programs)). A run that builds a state
holding a hypothesis root does not, where the pass that writes `open` facts runs, finds the program not monotone in
flight (`open::Writers::first_above`), says so at once, once, and answers
([Q-M1f.574](../../../decisions/answer/Q-M1f.574-a-hypothesis-first-held-above-root-makes-its.md)).

The detector's dynamic half reads what the run computes anyway: each node's derivation records for a guard the
node's fixpoint now falsifies (a stale fact), and for each printed state of two commitments or more up to
`SPOT_PARENTS` of its one-step parents, each built, the missing candidate written, saturated, and compared.

### 4.3 The witnesses

The reasoning page states each cause by the shape of a program; these are the corpus programs of those shapes.

| cause | program |
|---|---|
| c1, an `absent` that concludes a positive fact | [`examples/features/07_unstratifiable.ein`](../../../../examples/features/07_unstratifiable.ein)'s `derive-p` |
| c2, an `absent` that refutes | [`examples/ein-bugs/naf-upward-closure.ein`](../../../../examples/ein-bugs/naf-upward-closure.ein)'s `bad` |
| c3, a refutation through a fact derived under an `absent` | [`examples/ein-bugs/wrong-answers/stale-naf-fork.ein`](../../../../examples/ein-bugs/wrong-answers/stale-naf-fork.ein)'s `p-and-q-clash` |
| c4, an `absent` that takes a witness away | [`examples/lattice/08_absent_witness_hole.ein`](../../../../examples/lattice/08_absent_witness_hole.ein) |
| c5, an obligation's trigger | [`06_friends_tables.ein`](../../../../examples/lattice/06_friends_tables.ein), its only site, with its monotone twin [`06_friends_tables_everyone_sits.ein`](../../../../examples/lattice/06_friends_tables_everyone_sits.ein); [`examples/lattice/04_obligation_trigger_hole.ein`](../../../../examples/lattice/04_obligation_trigger_hole.ein), the trigger beside c14's and c15's sites |
| c6, an activator that applies an obligation | [`examples/lattice/09_activator_hole.ein`](../../../../examples/lattice/09_activator_hole.ein) |
| c7, c11, an activator that hides the `absent` | no corpus program: the two written in [`ein-infer/examples/shared/written.rs`](../../../../ein.rs/crates/ein-infer/examples/shared/written.rs), `ACTIVATOR_REFUTES` and `ACTIVATOR_UNDER_ABSENT`, which `ein-infer/tests/non_monotone_loads.rs` loads |
| c8, an even cycle through an `absent` | `07_unstratifiable.ein` and [`examples/ein-bugs/wrong-answers/f07-rules-swapped.ein`](../../../../examples/ein-bugs/wrong-answers/f07-rules-swapped.ein), the same program with its two rules in the other order |
| c9, an odd cycle through an `absent` | [`examples/ein-bugs/d3-unsat-core.ein`](../../../../examples/ein-bugs/d3-unsat-core.ein) |
| c10, the FIFO gap | [`examples/lattice/10_fifo_gap.ein`](../../../../examples/lattice/10_fifo_gap.ein) |
| c12, a FIFO race over a static read | [`examples/lattice/11_fifo_gap_static.ein`](../../../../examples/lattice/11_fifo_gap_static.ein) |
| c13, a rule that reads what a state owes or supposes | [`examples/features/23_a_rule_reads_open.ein`](../../../../examples/features/23_a_rule_reads_open.ein)'s `noted`, an `open-read` site |
| c14, a hypothesis a commitment can bring | `04_obligation_trigger_hole.ein`'s `seat` and `dine`, `hypothesis` sites; found in flight too, on the program `ein-infer/tests/open_facts.rs` writes in `a_hypothesis_first_held_above_root_is_found` |
| c15, a domain a commitment can grow | [`examples/ein-bugs/wrong-answers/oblig-derived-domain.ein`](../../../../examples/ein-bugs/wrong-answers/oblig-derived-domain.ein)'s `owes-food`, an `obligation-guard` site, and its siblings; `04_obligation_trigger_hole.ein`'s `total-owed` |
| a chain of activators that first appears at a node | [`examples/ein-bugs/wrong-answers/activator-chain-1.ein`](../../../../examples/ein-bugs/wrong-answers/activator-chain-1.ein) and its siblings |
| what is no site | [`examples/zebra2.ein`](../../../../examples/zebra2.ein)'s spatial reads; `total-owed`'s witness in [`examples/lattice/05_owed_seats_bounded.ein`](../../../../examples/lattice/05_owed_seats_bounded.ein); a stored negation of a fact of a relation that has candidates in [`examples/ein-bugs/complete-records-stale.ein`](../../../../examples/ein-bugs/complete-records-stale.ein) |
| the one-fact test misreads the lines | [`examples/lattice/synth/local_not_global.ein`](../../../../examples/lattice/synth/local_not_global.ein) |
| the duality fails with no hole | [`examples/lattice/synth/maximal_not_top.ein`](../../../../examples/lattice/synth/maximal_not_top.ein) |

### 4.4 What holds it

| claim | test |
|---|---|
| a program written for a target region has that region, node for node, and the static check calls it monotone exactly when the target is | `ein-infer/tests/region.rs`: `every_synthetic_program_has_the_region_it_was_written_for` |
| `ein solve FILE --monotone` answers from root's fixpoint and the static check, and describes no region | `ein-cli/tests/region_modes.rs`: `the_monotone_question_is_about_the_program` |
| the theorem on the cube: a program the check calls monotone shows no violation of either upward closure; the non-monotone witnesses are flagged, each for its cause | `ein-infer/tests/region.rs`: `monotone_programs_show_no_violation_and_the_non_monotone_three_are_flagged` |
| a *monotone* verdict beside a cube violation fails, on every corpus program and regression witness within the oracle's limit | `ein-infer/tests/region.rs`: `fact_grain_never_calls_a_violated_cube_monotone` |
| the check reads at relation grain: a program whose dynamic reads no commitment can match is flagged all the same | `ein-infer/tests/region.rs`: `fact_grain_on_programs_written_to_test_it` |
| fact grain leaves the set only where no commitment can match a read, on the programs written for it, and agrees with the shipped check on every monotone cube; its verdict on a flagged program is held to no cube | `ein-infer/tests/region.rs`: `fact_grain_on_programs_written_to_test_it`, `fact_grain_never_calls_a_violated_cube_monotone` |
| a *monotone* verdict is one state per node by fork and by every split from a fixpoint that holds root's, with no stale fact; from scratch it departs on `11_fifo_gap_static`'s `{b}` alone | `ein-infer/tests/detector.rs`: `a_monotone_verdict_is_one_state_per_node_by_fork_and_by_every_split`, `a_monotone_verdict_is_one_state_per_node_on_random_programs` |
| zebra2's `absent`s read facts of static relations, those no commitment can add a fact of, so the check calls it monotone | `ein-infer/tests/region.rs`: `zebra2_is_monotone_by_construction` |
| an obligation's witness `absent` is no site | `ein-infer/tests/region.rs`: `monotone_programs_show_no_violation_and_the_non_monotone_three_are_flagged` |
| a fixture per cause, each its program's only site: an `absent` that takes a witness away, an obligation's trigger and an activator; `04_obligation_trigger_hole.ein` holds the trigger beside a `hypothesis` and an `obligation-guard` site; an activator hides the refutation's `absent` from root's rules | `ein-infer/tests/region.rs`: `each_cause_has_a_fixture_whose_only_site_is_of_its_kind` |
| a rule that reads `open` is flagged, and a hypothesis first held above root is found by the static check and in flight | `ein-infer/tests/open_facts.rs`: `a_rule_that_reads_open_is_flagged`, `a_hypothesis_first_held_above_root_is_found` |
| without holes the minimal and maximal lines are E's and N's | `ein-infer/tests/region.rs`: `the_duality_holds_without_holes` |
| on a monotone cube saturation is a closure operator | `ein-infer/tests/region.rs`: `a_monotone_verdict_makes_saturation_a_closure_operator` |
| what an alive state denies is dead above it on every monotone cube, and a flagged program spends no denial | `ein-infer/tests/known_dead.rs`: `what_an_alive_node_denies_is_dead_above_it_on_every_monotone_cube`, `a_flagged_program_spends_no_denial` |
| the rule order of `features/07` moves its universe | no test states it: a flagged program, of whose answer nothing is claimed ([Q-M1f.564](../../../decisions/answer/Q-M1f.564-a-non-monotone-program-loads-with-a-warning.md)) |
| the static half is empty on every monotone corpus program, and names each site's stake, each cycle with its parity and each order finding on a flagged one | `ein-infer/tests/detector.rs`: `the_static_half_is_empty_on_a_monotone_program_and_named_on_a_flagged_one`, `each_site_names_what_it_concludes_and_puts_at_stake`, `a_cycle_through_an_absent_is_found_with_its_parity`, `the_order_check_finds_every_fifo_race` |
| every route on a flagged program gives the oracle's R, and every mode agrees with it | `ein-infer/tests/explore.rs`: `every_mode_is_the_oracle` |
| a line mode past the limit certifies what describe certifies | `ein-render/tests/line_modes_past_the_limit.rs`: `past_the_limit_a_line_mode_certifies_what_describe_does` |

### 4.5 Commands and drawings

```sh
ein solve FILE --monotone          # the static check alone, on root's fixpoint: the sites, cycles and order findings
ein oracle FILE --dump DIR         # the cube: every node classified, as region.json and region.md
ein render region DIR              # the cube drawn; --visited and --answer overlay a solve run's dumps
```

The detector's rows in the read-out are the `lattice` block of
[`08-read-out.md`](../../user/04-reference/08-read-out.md), and the cube's files are
[`11-region-dump.md`](../../user/04-reference/11-region-dump.md).

**The images under the reasoning page's `diagrams/`.** Six of the seven sources are written again with one command,
from the repository's root with `ein` on `PATH`:

```sh
for p in branching/15_lookahead_two_step_on lattice/05_owed_seats_bounded lattice/synth/merged_states \
         lattice/08_absent_witness_hole lattice/04_obligation_trigger_hole lattice/09_activator_hole; do
  f=examples/$p.ein; o=$(mktemp -d)
  d=docs/ein/reasoning/02-kb-evolution/diagrams/08-region-$(basename $p | tr _ -)
  ein oracle $f --dump $o/cube > /dev/null && ein solve $f --dump-lattice $o/lattice --dump-answer $o/answer > /dev/null &&
  ein render region $o/cube --visited $o/lattice --answer $o/answer > $d.dot && rm -r $o
done
```

The command writes each image's source, the `.dot`. The `.svg` beside it is Graphviz's drawing of that source, and
its last line carries the source's hash, so a source that changes without its picture is found
([Q-M1f.368](../../../decisions/docs/Q-M1f.368-a-diagram-is-a-file-and-a-page-shows-its-picture.md)).

| image | program |
|---|---|
| `08-region-15-lookahead-two-step-on` | [`examples/branching/15_lookahead_two_step_on.ein`](../../../../examples/branching/15_lookahead_two_step_on.ein) |
| `08-region-05-owed-seats-bounded` | [`examples/lattice/05_owed_seats_bounded.ein`](../../../../examples/lattice/05_owed_seats_bounded.ein) |
| `08-region-merged-states` | [`examples/lattice/synth/merged_states.ein`](../../../../examples/lattice/synth/merged_states.ein) |
| `08-region-08-absent-witness-hole` | [`examples/lattice/08_absent_witness_hole.ein`](../../../../examples/lattice/08_absent_witness_hole.ein) |
| `08-region-04-obligation-trigger-hole` | [`examples/lattice/04_obligation_trigger_hole.ein`](../../../../examples/lattice/04_obligation_trigger_hole.ein) |
| `08-region-09-activator-hole` | [`examples/lattice/09_activator_hole.ein`](../../../../examples/lattice/09_activator_hole.ein) |
| `08-region-three-components` | [`examples/lattice/synth/three_components.ein`](../../../../examples/lattice/synth/three_components.ein) |

**The two cubes in the page's § 6** are `ein oracle FILE --dump DIR` and `ein render region DIR` with no run
overlaid, shown without the legend, the tooltips and the file's path. The non-monotone one is
`examples/lattice/04_obligation_trigger_hole.ein`. The monotone one is this program, whose two rules the
satisfiability page draws with `ein render rule --fold`:

```lisp
(relation is-a   T T         :why "{?3} is a {?4}")
(relation seats  Guest Chair :why "{?3} is seated at {?4}")

(is-a Ann Guest)
(is-a C1 Chair)  (is-a C2 Chair)

(rule guest-owes-seat ()
  :match  (and (is-a ?g Guest) (absent (seats ?g ?c)))
  :assert (open seats)
  :why    "{?g} is a guest with no seat, so a seat is owed")

(rule one-seat ()
  :match  (and (seats ?g ?c) (seats ?g ?d) (neq ?c ?d))
  :assert (false)
  :why    "{?g} is seated at {?c} and at {?d}, which cannot be")

(rule seat () :match (and (is-a ?g Guest) (is-a ?c Chair)) :assert (open (seats ?g ?c)) :why "guess that {?g} is seated at {?c}")

(query :goal (seats ?g ?c))
```

---

## 5. Cores

The reasoning: [cores](../../reasoning/02-kb-evolution/09-cores.md).

### 5.1 Where it lives

| notion | code |
|---|---|
| the records of a fact, primary and alternatives | [`ein-core/src/prov.rs`](../../../../ein.rs/crates/ein-core/src/prov.rs) and the per-knowledge-base table in [`kb.rs`](../../../../ein.rs/crates/ein-core/src/kb.rs), `Kb::record_justification` |
| what a derivation rests on: premises, a universal's facts, a node's activator | [`ein-core/src/walks.rs`](../../../../ein.rs/crates/ein-core/src/walks.rs) |
| a derivation record, a terminal, two records that are one | `ein_core::prov::Prov` and `ProvKind`; `Prov::is_terminal`, `Prov::same_justification` |
| the primary record and the alternatives, and the bound on them | `Kb::primary`, `Kb::alternatives`, `ein_core::kb::MAX_ALT_JUSTIFICATIONS` |
| the derivation graph, and the terminals a fact reaches | `ein_core::walks`: `walk_premises`, `build_derivation_dag`, `is_frontier`, `unsat_core` ([`03-design/03-data-model.md` § 8.4](../03-design/03-data-model.md#84-derivation-walks)) |
| the origin of a fact, read off its record | the three buckets of `ein saturate --dump` in `ein-cli/src/saturate.rs`; `ColourBy::Origin` in `ein-render/src/kb_dot.rs` |
| a fact stated twice keeps its first record | `Kb`'s loader path |
| a cycle among the derivations the text states | `ein_core::walks::detect_provenance_cycles` |
| a core | [`ein-infer/src/explain.rs`](../../../../ein.rs/crates/ein-infer/src/explain.rs): the label search, `ExplanationBudget` |
| which cores a run takes | `region::Cores` in [`ein-infer/src/region.rs`](../../../../ein.rs/crates/ein-infer/src/region.rs) |
| keeping the first derivation alone | `(config :record-alternative-justifications false)` ([`06-config.md`](../../user/04-reference/06-config.md)), the one option that may change a core |
| the contradiction rule, one a count of arguments | [`stdlib/contradiction.ein`](../../../../stdlib/contradiction.ein): `clash-0` to `clash-6` and `clash-of-0` to `clash-of-6`; what the engine reads of such a rule, with no name, `ein_infer::contradiction::Pairs` |
| a death by a pair, read as the pair | `ein_infer::contradiction::detect`: through the firing of the rule every program gets, to the `(false)` it matched, and through a derivation of that `(false)` from a fact and one holding it, to the two |

### 5.2 The algorithm

The search computes, for each fact reachable from the contradiction, a **label**: the smallest sets of terminals
it follows from, over every derivation the store holds for it, by a least fixpoint from the terminals up, so a
cycle of derivations costs nothing. A derivation's label is the pairwise union of its premises' labels; a fact's
is the smallest of its derivations'. The store keeps a bounded list of derivations a fact, those with the fewest
premises and beside them the arrival with the smallest closure. The budget has four caps, the environments kept
per label, the rounds, the largest environment and the largest premise closure
(`ExplanationBudget`'s `max_environments`, `max_rounds`, `max_env_size`, `max_facts`), and a core carries its
status: `within-budget`, or `cut-` and the caps its search stopped at
([`07-search.md` § 6](../03-design/07-search.md#6-cores-and-explanations); the search as a flow is
[`02-algorithms.md` § 9](../02-algorithms.md#9-core-extraction)). In the JSON a node that is not dead
has `null` for its core and a dead node with the empty core `[]`.

### 5.3 What holds it

| claim | test |
|---|---|
| every core a run prints names a fact of its commitment and, with root, derives its contradiction; `refuted by` names its derivation's last step | `ein-infer/tests/corpus_cores.rs`: `every_core_names_its_commitment_and_derives_its_contradiction` |
| a rule with no premise refutes root with the empty core | `ein-infer/tests/corpus_cores.rs`: `a_rule_with_no_premise_refutes_root_with_the_empty_core` |
| a node that is not dead has no core | `ein-cli/tests/answer_round_trip.rs`: its `no_core_unless_dead` check, in `every_state_file_is_saved_and_loaded_as_itself_0` to `_7`, over every corpus dump |
| no core is printed unqualified from a cut search | `ein-infer/tests/corpus_cores.rs`: `no_core_is_printed_unqualified_from_a_cut_search` |
| every budget cap stays sound and reports itself, each by name | `ein-infer/tests/explain_semantics.rs`: `every_budget_cap_stays_sound_and_reports_itself`, `each_cap_a_search_stops_at_is_named` |
| the search finds a saving between two facts that a per-fact choice misses | `ein-infer/tests/explain_semantics.rs`: `the_search_finds_a_shared_optimum_a_greedy_pick_would_miss` |
| the search terminates on a cyclic graph and grounds out on givens | `ein-infer/tests/explain_semantics.rs`: `the_search_terminates_and_grounds_out_on_a_cyclic_justification_graph` |
| a given explains itself; a consistent world and an underivable fact explain to nothing | `ein-infer/tests/explain_semantics.rs`: `a_given_explains_itself`, `a_consistent_kb_and_an_underivable_target_explain_to_nothing` |
| a full list keeps the smallest-closure arrival, and a pair's two sides report one core | `ein-infer/tests/explain_semantics.rs`: `a_full_list_keeps_the_smallest_closure_and_the_pair_report_one_core` |
| a bounded store can hide a smaller core: a saving between two facts behind a full list is refused | `ein-infer/tests/explain_semantics.rs`: `a_saving_between_two_facts_behind_a_full_list_is_still_refused` |
| a core follows no activator of root's, and covers both sides of a pair and what a `forall` read | `ein-infer/tests/region.rs`: `a_dead_nodes_core_follows_no_activator_of_roots`, `a_contradictions_core_covers_both_sides_and_what_a_forall_read` |
| the core is the one option-sensitive output: every line of every mode is the same under every flag but the core's under `:record-alternative-justifications` | `ein-cli/tests/lever_invariance.rs`: `no_config_flag_moves_a_line_of_any_mode`, `the_core_exemption_is_one_flag_and_the_core_rows` |
| every question and claim answers alike under two orders, a core held as a valid explanation | `ein-infer/tests/agenda.rs`: `every_question_and_claim_answers_alike_under_the_agenda_and_the_written_order` |
| a run that prints no core takes none and answers alike | `ein-infer/tests/explore.rs`: `a_run_that_prints_no_core_takes_none_and_answers_alike` |
| a member of N's core is the one an eager run takes | `ein-cli/tests/answer_round_trip.rs`: its `eager_cores` check, in the same tests |
| only a firing with a premise is recorded as another derivation, and a terminal takes none | the store's unit tests `only_a_real_firing_is_recorded_as_an_alternative`, `a_terminal_primary_takes_no_alternatives_at_all` in `ein-core/src/kb.rs` |
| a record made in a fork does not reach its parent | the store's unit test `a_forks_alternatives_do_not_leak_into_its_parent` in `ein-core/src/kb.rs` |
| a firing admitted at the boundary records what had to be absent, each derivation its own | `ein-infer/tests/naf_semantics.rs`: `a_boundary_admitted_firing_records_what_was_absent`, `an_alternative_justification_carries_its_own_negative_premises` |
| every explanation lies inside what the facts reach over every recorded derivation; with other derivations not recorded, the primary-only reach is what remains | `ein-infer/tests/search_semantics.rs`: `an_explanation_is_a_subset_of_the_or_aware_closure_and_not_of_the_primary_only_union`, `turning_alternative_recording_off_restores_the_primary_only_frontier` |
| the recorded derivations survive a rebuild of the store's indexes | `ein-infer/tests/explain_semantics.rs`: `a_rebuild_preserves_the_alternative_justifications` |
| a fact stated twice keeps its first statement | the store's unit test `the_loader_path_keeps_the_first_occurrence_and_does_not_index` in `ein-core/src/kb.rs` |
| an imported fact keeps the record its module writes | `ein-ir/tests/imports_semantics.rs`: `what_each_import_form_brings_of_a_module`; `ein-cli/tests/import_facts.rs`: `a_flat_import_of_facts_answers_as_the_facts_written_in_place_on_every_command` |
| a written derivation cycle is refused at load, and every load refusal is reported together | `ein-ir/tests/load_semantics.rs`: `the_load_negative_fixtures_are_byte_identical` (`examples/broken/load/derivation_cycle.ein`), `load_errors_accumulate_in_pass_order` |
| a refutation's edges into `⊥` are in the order of the facts' quoted forms, whatever order they were collected in | `ein-render/tests/presentation_semantics.rs`: `the_bottom_edges_of_a_refutation_are_sorted_not_collection_ordered` |
| a dead root's core names the injected clue, with its `:source` | `ein-infer/tests/acceptance.rs`: `the_contradictions_view_is_the_verdict_explained`, `the_injected_clash_carries_the_source_that_names_it` |
| each contradiction rule fires once on its pair, and without the module the pair is alive and nothing fires | `ein-infer/tests/contradiction_rules.rs`: `each_clash_rule_fires_once_and_its_premises_are_the_pair`, `without_the_module_the_pair_is_alive_and_nothing_fires`, `no_clash_rule_fires_where_no_fact_meets_its_negation`; the programs under [`tests/stdlib/contradiction/`](../../../../tests/stdlib/contradiction/) |

### 5.4 Commands

```sh
ein solve FILE                      # a dead root: the rows `unsat core`, `source`, `refuted by`
ein solve FILE --trace OUT.md       # N, each minimal dead set with its core and what refutes it
ein solve FILE --dump-answer DIR    # the members of N with their cores, as files
ein solve FILE --dump-lattice DIR   # every node the run visited, a dead one with its core
ein saturate FILE --dump            # root's facts by origin: BACKGROUND, GIVEN, DERIVED, its open facts among them
ein render kb --colour-by origin FILE   # the same, drawn: given black, derived blue and dashed
```

How many derivations a run keeps for one fact is a `(config …)` flag, `:record-alternative-justifications`
([`06-config.md`](../../user/04-reference/06-config.md)): `false` keeps the primary alone. It changes no state and
no answer; it may change a dead node's core.

---

## 6. Satisfiability

The reasoning: [satisfiability](../../reasoning/02-kb-evolution/10-satisfiability.md).

### 6.1 Where it lives

| notion | code |
|---|---|
| what a state owes | [`ein-infer/src/obligations.rs`](../../../../ein.rs/crates/ein-infer/src/obligations.rs): `tally`, one pass over a quiescent knowledge base, its result `Owes`, read at every state, which [`open.rs`](../../../../ein.rs/crates/ein-infer/src/open.rs) writes as `(open R)`, once for each relation owed, into the part of the state no fork inherits, where its pass runs: at root, and at a state above root that is printed or saved or that uses its `open` facts |
| a claim and its outcome | [`ein-infer/src/claim.rs`](../../../../ein.rs/crates/ein-infer/src/claim.rs) compares, [`expect.rs`](../../../../ein.rs/crates/ein-infer/src/expect.rs) is the outcome and its report, [`ein-cli/src/test.rs`](../../../../ein.rs/crates/ein-cli/src/test.rs) the runner and its exit codes |
| a goal's rows over a state | [`ein-infer/src/verdict.rs`](../../../../ein.rs/crates/ein-infer/src/verdict.rs) |
| the branch on an owed instance | [`ein-infer/src/tree.rs`](../../../../ein.rs/crates/ein-infer/src/tree.rs), the obligation tree, under the licences of [`07-search.md` § 3](../03-design/07-search.md#3-the-licences) |
| what the universe leaves out | [`ein-infer/src/scope.rs`](../../../../ein.rs/crates/ein-infer/src/scope.rs) |
| a claim's models, compared with what its run found | [`ein-infer/src/claim.rs`](../../../../ein.rs/crates/ein-infer/src/claim.rs): `compared`, a region claim's models and R's states or owing nodes as its check reads them, and [`read_out.rs`](../../../../ein.rs/crates/ein-infer/src/read_out.rs)' `query_compared`, a query's; [`ein-render/src/claim_diff.rs`](../../../../ein.rs/crates/ein-render/src/claim_diff.rs): `diff`, the pairing, and `lines` and `json`, what `ein test --diff` prints and writes |

### 6.2 The algorithm

An obligation rule is compiled like any other and kept out of the agenda. After a node's fixpoint the pass runs
each obligation plan once and counts its matches, and where the pass of `open.rs` runs it writes `(open R)` once for
each relation owed; the event is `owe`
([`06-saturation.md` § 6.2](../03-design/06-saturation.md#62-obligations); the pass as a flow is
[`02-algorithms.md` § 10](../02-algorithms.md#10-the-obligation-pass)).

`ein test` asks a file the least question that decides the claims it states, and reads every claim off that
run: **held**, **FAILED**, or **NOT CHECKED**. A failed claim exits 1, and so does one not checked, except under
`--no-search`, where a claim root does not decide moves no exit code
([`05-cli.md`](../../user/04-reference/05-cli.md)). A claim is written `:expect`, on a `(query …)` or on the
`(solve …)` form. Under `--diff` each claim a run decided is followed by its models against what the run found, read
as its check reads them (`claim::compared`, `read_out::query_compared`), paired by `claim_diff::diff`: the matches as
a maximum matching of the pairs the check pairs, so a held claim's diff is all matches, then the near-misses.

The count of what a listed state leaves open is the leftover probe,
`EIN_LEFTOVER=1`, read on a copy of the state ([`06-config.md`](../../user/04-reference/06-config.md)).

The program of the reasoning page's § 5 is
[`examples/ein-bugs/wrong-answers/uncovered.ein`](../../../../examples/ein-bugs/wrong-answers/uncovered.ein), and
[`uncovered-knows.ein`](../../../../examples/ein-bugs/wrong-answers/uncovered-knows.ein) is the same program with
`(knows Ann Bob)` added; its read-out's `outside` row is the sentence that names the relation. The neighbours of
its § 2.3 are in the catalogue:
[answer set programming](../../../lib/02-solvers-csp-sat-smt.md#answer-set-programming-asp) and
[clingo](../../../lib/02-solvers-csp-sat-smt.md#clingo),
[Alloy](../../../lib/03-theorem-proving-formal-methods.md#alloy),
[Z3](../../../lib/02-solvers-csp-sat-smt.md#z3). Ein's analogue of Alloy's scope is `--region-budget`.

### 6.3 What holds it

| claim | test |
|---|---|
| an obligation rule derives nothing and never reaches the firing stream; a state that owes is not a solution | `ein-infer/tests/obligation_reports.rs`: `an_obligation_rule_never_reaches_the_firing_stream`, `a_state_that_owes_is_open_and_not_a_solution` |
| `ein saturate --dump` lists every rule of a program in the section of its kind: the saturation agenda's with its stratum, an obligation rule and a hypothesis rule with how many activators bind its parameters | `ein-cli/tests/saturate_dump_rules.rs`: `an_obligation_rule_is_listed_in_a_section_of_its_own`, `an_obligation_rule_nothing_activates_says_so`, `a_hypothesis_rule_is_listed_with_its_activators`, `every_rule_stands_in_one_section` |
| no option, lever or flag changes what is derived or which states are in R | `ein-cli/tests/lever_invariance.rs`: `every_declared_lever_cell_moves_no_line_of_any_mode` |
| no order of the search, interning order or number of workers moves an answer | `ein-infer/tests/explore.rs`: `every_order_of_the_tree_is_the_oracle`; `ein-render/tests/id_order_invariance.rs`: `every_rendering_reproduces_its_digest_and_none_depends_on_the_id_order`; `ein-render/tests/jobs_invariance.rs`: `jobs_does_not_move_any_observable` |
| a claim that names a relation states the whole of its held extent in every state it lists | `ein-infer/tests/claim_semantics.rs`: `naming_a_relation_closes_it_in_every_model` |
| a claim's diff moves no verdict, no exit code and no other line, and a held claim's is all matches, over every claim of the corpus | `ein-cli/tests/test_cli.rs`: `the_diff_moves_no_verdict_and_no_exit_code` |
| a claim the run did not decide is not checked; a run a budget stopped leaves its claim not checked and says why; under `--no-search` a claim not checked moves no exit code | `ein-infer/tests/expect_semantics.rs`: `a_claim_the_run_did_not_decide_is_not_checked`; `ein-cli/tests/test_cli.rs`: `a_budget_stopped_run_is_not_checked_and_says_why`; `ein-cli/tests/no_search.rs`: `test_decides_what_root_decides_and_a_claim_not_checked_exits_0` |
| every route that branches on an owed instance's alternatives finds the oracle's R | `ein-infer/tests/explore.rs`: `every_mode_is_the_oracle` |
| on the zebra family, branching on what the obligations owe finds what the hypothesis rule finds | `ein-infer/tests/obligation_rung.rs`: `the_theory_finds_what_the_hypothesis_rule_finds` |
| every question names a relation H holds no fact of, in one sentence | `ein-cli/tests/read_out.rs`: `the_universes_scope_is_one_sentence_on_every_surface` |
| what the blind enumerator would still propose at a listed state is a read, off unless asked | `ein-cli/tests/leftover_probe.rs`: `the_leftover_probe_is_off_by_default_is_a_read_and_means_what_it_says` |
| an exhausted run reports the states, an unexhausted one the states found | `ein-cli/tests/model_set_report.rs`: `an_exhausted_run_reports_the_states`, `an_unexhausted_run_reports_states_found` |

### 6.4 Commands

```sh
ein test FILE                        # every claim the file states: held, FAILED or NOT CHECKED
ein solve FILE --maximal -n 5        # at most five states of the line, and whether that is all of it
ein render rule --name NAME FILE     # an obligation or a restriction as its match and its conclusion
```
