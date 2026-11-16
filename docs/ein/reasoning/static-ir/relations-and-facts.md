# Relations and facts

A **knowledge base** is a program and a set of facts. The program is what the text declares: its relations, its
rules and hrules, its macros (expanded as the text is read), its queries and its `(solve …)` and `(config …)`
forms. The facts are what holds: the facts the text writes, the facts the loader states about the text, and the
facts saturation derives. This page states what the store of facts is, which operations it has, and what each fact
records of how it came to be held. What a single name or fact *is* is [names and atoms](names-and-atoms.md).

The knowledge base is a **hypergraph with provenance**: its nodes are objects, types, relations, rules and facts,
and a fact is one edge over any number of arguments, labelled by its relation. A relation of three arguments is one
edge, not a construction of binary ones. Every fact carries how it came to be held
([Q1](../../../decisions/language/Q1-the-knowledge-base-is-a-typed-labelled.md)).

## A fact, its negation, and neither

For any fact `P` a knowledge base holds one of three things:

| `P` is | the store holds | a pattern that matches it |
|---|---|---|
| **asserted** | the fact `P` | `P` |
| **negated** | the fact `(not P)`, a fact of `not` whose one argument is `P` | `(not P)` |
| **open** | neither | `(unknown P)`, which is `(and (absent P) (absent (not P)))` |

A stored negation is a fact like any other: it is written or derived, recorded with its provenance, matched by a
pattern of its shape. **`(not P)` in a pattern matches a stored negation and nothing else**; it is never negation
as failure. Negation as failure is `(absent P)`, a test that no fact of a shape is held, and it is read against a
saturated world, never against a half-built one ([`absent`](../kb-evolution/absent-and-naf.md)). `(not N)` of a
name `N` is the negation of no fact.

Holding both `P` and `(not P)` is a contradiction, and so is holding `(false)`. A knowledge base that holds one is
**dead**; which facts a contradiction rests on is its core ([cores](../kb-evolution/cores.md)).

## The store only grows

No operation removes a fact. Rules add facts; a stored negation adds a fact; nothing deletes one. Two statements of
one fact are one fact, and the first statement's provenance is kept, whether the two are in one file, in a module
and the program, or in two modules
([Q-M1f.139](../../../decisions/imports/Q-M1f.139-a-fact-stated-twice-keeps-its-first-statement.md)). What a
search assumes is undone by discarding a fork (below), never by retracting from the store.

## Reading a program into a store

Each top-level form of the text is a declaration, a relation, a rule, an hrule, a macro or an import, or a
program's question, a query, a `(solve …)` or a `(config …)`, or a **fact**. A fact's provenance is what the text
says of it:

- written with `:source "…"`, it is **given**, and the source sentence is its provenance;
- written with `:rule R :using (P₁ … Pₙ)`, it is **derived**, as the text says, from those premises by that rule;
- written with neither, it is given with no source, a **background** fact.

A fact an import brings keeps the provenance its module writes, as if the program had written it in place
([Q-M1f.138](../../../decisions/imports/Q-M1f.138-an-imported-fact-keeps-its-own-provenance.md)). A head that no
declaration names is a relation with no declared count, and its facts load like any other
([names and atoms](names-and-atoms.md#a-relation-is-an-object)).

Reading refuses a program, and names every reason at once, where it declares one name twice, where a form of a
declared relation has another count of arguments than the declaration has types, and where the `:using` chains the
text writes make a fact its own premise. A **cycle of derivations the engine records** is legitimate, and is not
refused: once a symmetric relation's two facts are each derived from the other, `(R a b)` and `(R b a)` justify
each other, and a walk over provenance breaks such a cycle by not expanding a fact twice.

The facts the loader states about the text, beside those it writes: each declaration as facts of `relation`, and
`(written T)` for what the text nests ([names and atoms](names-and-atoms.md#a-relation-is-an-object)).

## A fork

A **fork** of a knowledge base is a knowledge base of its own that starts as its parent stands: the same program,
and every fact the parent holds. From then on the two diverge:

- what the fork adds is the fork's alone: no fact a fork derives reaches its parent or a sibling;
- what the parent adds after the fork is not the fork's;
- the derivations a fork records are the fork's, and stay in it: a derivation recorded under a commitment may rest
  on the commitment, which the parent never assumed, and would otherwise name it in an explanation of the parent.

Every node a search reads is a fork of root, root's fixpoint with the node's commitment added and saturated
([Q-M1e2.2](../../../decisions/answer/Q-M1e2.2-a-node-is-a-fork-of-root-an-assumption-on.md)), and a node's
saturation continues root's rather than repeating it, reaching the fixpoint a fresh saturation of the grown
knowledge base reaches ([Q-M1a.18](../../../decisions/engine/Q-M1a.18-a-node-resumes-roots-saturation.md)). That
nothing a node derives reaches root is what lets every node be read independently of every other
([finding R](../kb-evolution/finding-r.md)).

## Provenance: a fact is an OR of derivations

Every fact records how it came to be held: given, with its source where the text writes one; derived by a rule's
firing; or committed as a hypothesis. A **derivation** by a firing records its rule, the activator the rule was
applied under, the positive premises it matched, the `(absent …)` queries that had to find nothing for it to be
admitted, and, for a `forall`, the facts each body matched over its domain
([Q18](../../../decisions/engine/Q18-provenance-is-per-fact-an-or-of-derivations-over.md)).

So the proof structure is an **AND/OR graph**: a fact is an OR over its derivations, and each derivation is an AND
over its premises, with its activator where a node derived or committed the activator, and with the facts its
`forall`s read
([Q-M1e2.12](../../../decisions/answer/Q-M1e2.12-a-dead-nodes-core-follows-an-activator-only.md),
[Q-M1e2.15](../../../decisions/answer/Q-M1e2.15-a-derivation-through-a-forall-rests-on-each-fact.md)).

- **The primary derivation** is the first one recorded. Deriving the fact again never changes it, so everything
  that reads one derivation of a fact reads one stable story.
- **A terminal takes no other derivation.** A given fact and a commitment are the frontier of every explanation: a
  given that some rule could also derive is still given. A firing with no premise, an engine's writeback, is a
  terminal too, and a walk over provenance grounds out on it.
- **Two firings of one rule over the same premises from the same activator are one derivation.** The bindings a
  firing used are read for display and are not part of what makes two derivations different.
- **What is recorded is what saturation attempted.** A derivation no rule firing reached is in no record. An
  implementation may bound how many derivations it keeps for one fact; `(config :record-alternative-justifications
  false)` keeps the primary alone. Neither changes what is held, nor the answer: what they may change is a dead
  node's core, which is an explanation and not part of the answer
  ([Q-M1e2.30](../../../decisions/answer/Q-M1e2.30-no-configuration-option-lever-or-flag-changes-r.md),
  [Q-M1f.6](../../../decisions/answer/Q-M1f.6-a-core-is-an-explanation.md)).

**The derivation graph of a fact** is its premises, walked transitively to the terminals: through the primary
derivation alone, one proof a reader can follow, or through every recorded derivation, the AND/OR graph. The
terminals the second reaches bound every explanation: no core of a contradiction names a fact outside the union of
the terminals of every recorded derivation of the facts it rests on. Choosing the *smallest* set of terminals is a
search over the AND/OR graph, not a walk, and is [cores](../kb-evolution/cores.md)' subject.

## Equality

No fact makes two names one. `eq` is name identity, a relation computed from its two arguments and never stored,
and `neq` its complement; nothing merges two names into one node, and a pattern matches a name only where it is
written ([Q1](../../../decisions/language/Q1-the-knowledge-base-is-a-typed-labelled.md),
[Q-M1e2.26](../../../decisions/language/Q-M1e2.26-eq-and-neq-are-the-relations-of-equality-and-non.md)).

## What holds it

| claim | test |
|---|---|
| `(not P)` in a premise matches a stored negation, not the absence of `P` | `ein-infer/tests/rule_semantics.rs`: `not_in_a_premise_is_stored_negation_not_negation_as_failure` |
| `(unknown P)` admits exactly the facts neither held nor denied | `ein-infer/tests/naf_semantics.rs`: `unknown_admits_exactly_the_undecided_pairs` |
| `P` beside `(not P)` is a contradiction whatever either half's origin, and every pair is reported | `ein-infer/tests/search_semantics.rs`: `mutual_negation_is_detected_whatever_either_sides_origin`, `every_pair_is_reported_not_only_the_first` |
| a fact stated twice keeps its first statement | the store's unit test `the_loader_path_keeps_the_first_occurrence_and_does_not_index` in `ein-core/src/kb.rs` |
| an imported fact keeps the provenance its module writes | `ein-ir/tests/imports_semantics.rs`: `what_each_import_form_brings_of_a_module`; `ein-cli/tests/import_facts.rs`: `a_flat_import_of_facts_answers_as_the_facts_written_in_place_on_every_command` |
| a written derivation cycle is refused at load, and every load refusal is reported together | `ein-ir/tests/load_semantics.rs`: `the_load_negative_fixtures_are_byte_identical` (`examples/broken/load/derivation_cycle.ein`), `load_errors_accumulate_in_pass_order` |
| a fork and its parent diverge after the fork, each seeing only its own additions | the store's unit test `a_fork_reads_base_then_delta_and_the_two_histories_diverge` in `ein-core/src/kb.rs` |
| a derivation recorded in a fork does not reach its parent | the store's unit test `a_forks_alternatives_do_not_leak_into_its_parent` in `ein-core/src/kb.rs` |
| a fork saturated reaches the state a direct saturation reaches | `ein-infer/tests/explain_semantics.rs`: `fork_and_direct_saturation_agree`, `fork_and_direct_agree_at_the_naf_boundary` |
| only a firing with a premise is recorded as another derivation, and a terminal takes none | the store's unit tests `only_a_real_firing_is_recorded_as_an_alternative`, `a_terminal_primary_takes_no_alternatives_at_all` in `ein-core/src/kb.rs` |
| a firing admitted at the boundary records what had to be absent, each derivation its own | `ein-infer/tests/naf_semantics.rs`: `a_boundary_admitted_firing_records_what_was_absent`, `an_alternative_justification_carries_its_own_negative_premises` |
| every explanation lies inside the union over every recorded derivation; with other derivations not recorded, the primary-only frontier is what remains | `ein-infer/tests/search_semantics.rs`: `an_explanation_is_a_subset_of_the_or_aware_closure_and_not_of_the_primary_only_union`, `turning_alternative_recording_off_restores_the_primary_only_frontier` |
| the recorded derivations survive a rebuild of the store's indexes | `ein-infer/tests/explain_semantics.rs`: `a_rebuild_preserves_the_alternative_justifications` |
| matching merges no names | `ein-infer/tests/naf_semantics.rs`: `matching_does_not_resolve_equality_classes` |
