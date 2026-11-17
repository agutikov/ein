# Static IR in code

Where `ein.rs` implements the four pages of [`reasoning/01-static-ir/`](../../reasoning/01-static-ir/), the tests
that hold it to them, and the commands that show each notion. The layout is the folder's
([README](README.md#how-a-page-is-laid-out)).

- [The running example](#the-running-example)
- [Names and atoms](#names-and-atoms)
- [Relations and facts](#relations-and-facts)
- [Values and their order](#values-and-their-order)
- [Rules as statements](#rules-as-statements)

## The running example

The four pages return to one small program, the *three-house example*. They show it as formulas and graphs; this
is its source. Save it as `three-houses.ein`:

```lisp
(relation is-a Thing Kind :why "{?1} is a {?2}")
(relation lives-in Person House :why "{?1} lives in {?2}")
(relation owns Person Pet :why "{?1} owns {?2}")
(relation pet-at Pet House :why "{?1} is kept at {?2}")
(relation left-of House House :why "{?1} is left of {?2}")
(relation vacant House :why "{?1} is vacant")

(rule pet-follows-owner ()
  :match  (and (owns ?p ?x) (lives-in ?p ?h))
  :assert (pet-at ?x ?h)
  :why    "{?p} owns {?x} and lives in {?h}, so {?x} is kept at {?h}")

(rule transitive (?R)
  :match  (and (?R ?a ?b) (?R ?b ?c) (neq ?a ?c))
  :assert (?R ?a ?c)
  :why    "{?R} is transitive: {?a} to {?b} to {?c}")

(rule nobody-home ()
  :match  (and (is-a ?h House) (absent (lives-in ?p ?h)))
  :assert (vacant ?h)
  :why    "nobody is known to live in {?h}")

(is-a Norwegian Person) (is-a Dog Pet)
(is-a House-1 House) (is-a House-2 House) (is-a House-3 House)
(transitive left-of)
(owns Norwegian Dog :source "The Norwegian owns the dog")
(lives-in Norwegian House-1 :source "The Norwegian lives in the first house")
(left-of House-1 House-2 :source "The first house is left of the second")
(left-of House-2 House-3 :source "The second house is left of the third")
```

The pages use its **root**, the state its own facts saturate to, and never its answer: its third rule reads an
`absent` over `lives-in`, a relation the blind enumerator may extend, so a bare `ein solve` flags the program as
not monotone. `ein saturate` stops at the root, and `--dump` prints it by origin:

```sh
ein saturate three-houses.ein --dump
```

<!-- output -->
```text
;; ── GIVEN (4 facts) ──
;;   left-of (2)
  (left-of House-1 House-2 :source "The first house is left of the second")
  (left-of House-2 House-3 :source "The second house is left of the third")
  (lives-in Norwegian House-1 :source "The Norwegian lives in the first house")
  (owns Norwegian Dog :source "The Norwegian owns the dog")

;; ── DERIVED (4 facts) ──
  (left-of House-1 House-3 :rule transitive)
  (pet-at Dog House-1 :rule pet-follows-owner)
;;   vacant (2)
  (vacant House-2 :rule nobody-home)
  (vacant House-3 :rule nobody-home)
```

**Where each drawing of the reasoning comes from.** Every command writes DOT on stdout
([rendering](../../user/04-reference/12-rendering.md) is the legend); a page pastes the digraph it names, with
the layout hints and comment lines taken out, and where it says so, a part of it.

| page | drawing | command |
|---|---|---|
| names and atoms | four facts, each a node with numbered arrows | `ein render program --levi three-houses.ein`, the digraph `facts` |
| names and atoms | the declaration of `lives-in` as a fact | `ein render kb three-houses.ein`, a part |
| relations and facts | the saturated state | `ein render kb --colour-by origin three-houses.ein`, without the `relation` facts |
| relations and facts | two firings with their premises | `ein solve three-houses.ein --no-search --trace trace.md`, the drawings of steps 1 and 2 in `trace.md`, set in one digraph |
| values and their order | the rule `note` and its state | `ein render rules examples/ein-bugs/binding-key-both.ein` and `ein render kb examples/ein-bugs/binding-key-both.ein`, the second without the `relation` facts |
| rules as statements | `pet-follows-owner`, `transitive`, `nobody-home` | `ein render rules three-houses.ein` |
| rules as statements | `pet-follows-owner` as one graph | `ein render rules --rule-mode overlay three-houses.ein` |
| rules as statements | `domain-elimination` | `ein render rule --name domain-elimination stdlib/bijection.ein` |

The drawings a page draws by hand are schematics no command writes: the cycle of recorded derivations, the
forks of root, the chain of values, the gluing square, the four levels.

## Names and atoms

The page: [names and atoms](../../reasoning/01-static-ir/01-names-and-atoms.md).

### Where it lives

| notion | code |
|---|---|
| the name characters, and a name as a run of them | the lexer's `NAME` rule in `ein-ir/src/ein.l`; `ein_core::text::is_name_char` and `is_symbol`, the printer's copy of it, held equal to the lexer's by a test |
| a name as one node: interning | `ein-core/src/intern.rs`; `ein_core::terms::Terms::intern_text`, one `Symbol` per distinct text |
| the reserved words | `ein_core::terms::RESERVED` (what no declaration binds), `STRUCTURAL` and `PREDICATES` (the words of the pattern language); the grammar's own list in `ein-ir/src/grammar.rs` |
| a name the loader makes up | `ein_core::text::INTERNAL_MARK`, `#`, and `INTERNAL_NAMES` ([`07-reserved-engine-strings.md`](../07-reserved-engine-strings.md)) |
| the kernel's two markers | `ein_core::terms::CLOSED`, `SYMMETRIC` |
| full names and a file's short names | `ein-ir/src/imports.rs`: `resolve_imports`, `short_names` ([`03-design/04-ir-frontend.md` § 8](../03-design/04-ir-frontend.md#8-import-resolution)) |
| a value: a name or a fact | `ein_core::value::Value`, four bytes, tagged `Sym` or `Fact`; a fact is an interned row, `Terms::intern_fact` ([`03-design/03-data-model.md` § 3](../03-design/03-data-model.md#3-values-and-fact-ids)) |
| what is a value and what is not | the grammar: `FactHead`, `ValueHead`, `NestedFact`, `Negated` in `ein-ir/src/ein.y`; after macro expansion, `refuse_malformed_primitives` in `ein-ir/src/from_ir.rs` |
| a declaration held as facts | `ingest_relation` in `ein-ir/src/from_ir.rs` |
| the declared count of arguments | `declared_counts` and `wrong_counts` in `ein-ir/src/from_ir.rs` |
| `written` | `state_written` and `written_in_conclusions` in `ein-ir/src/from_ir.rs`; `ein_core::terms::WRITTEN`, and `WRITTEN_STEP`, the made-up rule name `#written` its facts are derived by |
| the names of a program | `intern_program_names` in `ein-ir/src/from_ir.rs`; `ein_infer::invariant::program_names` |
| the blind enumerator's objects | `ein_infer::hypgen::candidate_objects`; the rows that name a relation or a slot name bound as an object, `ein-infer/src/relation_names.rs` and `types_as_objects.rs` |
| the origin of a fact, read off its provenance | the three buckets of `ein saturate --dump` in `ein-cli/src/saturate.rs`; `ColourBy::Origin` in `ein-render/src/kb_dot.rs` |

### What holds it

| claim | test |
|---|---|
| a run of digits is a name, as written, in every place a name is written; `1` and `01` are two names | `ein-cli/tests/cli_semantics.rs`: `a_run_of_digits_is_a_name_in_every_place`, `a_range_is_the_name_it_spells`; `ein-core/tests/values_semantics.rs`: `a_run_of_digits_is_a_name_as_written` |
| no name the loader makes up is a name a program can write | `ein-ir/tests/generated_parser.rs`: `no_name_the_loader_makes_up_is_a_name` |
| a kernel marker is refused wherever a program binds it | `ein-ir/tests/engine_names.rs`: `a_dunder_is_refused_wherever_a_program_binds_it` |
| one module reached through several imports is one copy; a short name is local to its file | `ein-ir/tests/imports_full_names.rs`: `the_users_dependency_graph`, `a_short_name_is_local_to_its_file`; `ein-ir/tests/imports_semantics.rs`: `one_module_under_two_spellings_is_one_copy` |
| a form of a declared relation at another count is refused, and the slot names are no type check | `ein-cli/tests/declared_counts.rs`: `a_form_of_another_count_is_refused_on_every_command`, `what_is_not_a_declared_relations_form_loads` |
| a relation name, a rule name or an hrule name declared twice is refused at load | `ein-ir/tests/load_semantics.rs`: `the_load_negative_fixtures_are_byte_identical`, over `examples/broken/load/rule_duplicate_name.ein` and `hrule_duplicate_name.ein` |
| a declaration is stored as the fact with its slot names and the fact of one argument; a bare one as one fact | `ein-infer/tests/compile_semantics.rs`: `a_bare_declaration_is_declared_with_an_empty_signature_and_stores_one_fact`; the loader's unit test `a_declaration_stores_itself_as_facts_so_rules_can_read_signatures` in `ein-ir/src/from_ir.rs` |
| an undeclared head is a relation | the loader's unit test `an_undeclared_head_vivifies_a_relation_but_a_predicate_does_not` in `ein-ir/src/from_ir.rs` |
| the loader states `written` for what a file nests and for a conclusion's ground nested facts, and a program may not write it | `ein-cli/tests/written_facts.rs`: `what_the_loader_states`, `a_program_does_not_write_written` |
| a pattern matches into a nested fact, and a nested fact pairs with its negation by structure | `ein-infer/tests/rule_semantics.rs`: `a_nested_pattern_binds_inside_the_nested_fact`; `ein-infer/tests/search_semantics.rs`: `a_nested_positive_pairs_through_structural_identity` |
| a form that is no value is refused, and every form a state could hold loads | `ein-cli/tests/value_rule.rs`: `a_form_that_is_no_value_is_refused_on_every_command`, `what_a_state_could_hold_loads` |
| every name of the program is a candidate object, and no run reaches a name the program does not have | `ein-infer/tests/alive_invariant.rs`: `every_name_of_the_program_is_a_candidate_object`, `no_run_reaches_a_name_the_program_does_not_have` |
| a contradiction is a pair whatever either half's origin | `ein-infer/tests/search_semantics.rs`: `mutual_negation_is_detected_whatever_either_sides_origin` |

### Commands that show it

**A declaration is a fact a rule's `:match` reads, and its slot names are ordinary names.** The page's rule that
gives a slot name a meaning, as a program. A variable in a relation's place must be a parameter, so the rule takes
`?R` and an activator names the relation:

```lisp
(relation is-instance Object Type :why "{?1} is an instance of {?2}")
(relation lives-in Person House :why "{?1} lives in {?2}")
(relation typed-first Object Type :why "{?1} fills a first place declared {?2}")

(is-instance Norwegian Person)
(is-instance House-1 House)
(lives-in Norwegian House-1)
(lives-in House-1 Norwegian)

(rule first-typed (?R)
  :match  (and (relation ?R ?T1 ?T2) (?R ?x1 ?x2) (is-instance ?x1 ?T1))
  :assert (typed-first ?x1 ?T1)
  :why    "{?R} declares {?T1} first and {?x1} is one")

(first-typed lives-in)
```

`ein saturate FILE --dump` lists the facts the declarations are stored as, `(relation lives-in Person House)` and
`(relation lives-in)` among the background facts, and one derived fact, `(typed-first Norwegian Person :rule
first-typed)`. `(lives-in House-1 Norwegian)` loads, and the rule concludes nothing from it. With the rule
written `(rule first-typed () …)` the program is refused at load: *its :match writes (?R ?x1 ?x2) with ?R as its
relation, which no parameter binds*.

**The one place a slot name is read by name.** Two programs that differ only in the slot names of one
declaration, `(relation lives-in Person House …)` against `(relation lives-in A B …)`, over the same four lines:

```lisp
(relation is-instance Object Type :why "{?1} is an instance of {?2}")
(relation lives-in Person House :why "{?1} lives in {?2}")

(is-instance Norwegian Person)
(is-instance House-1 House)
```

```sh
ein show hyps FILE
```

The first lists 4 candidates and the second 22: with the declaration naming `A` and `B`, `Person` and `House` are
no longer slot names of any declaration, so the blind enumerator takes them for objects and proposes
`(lives-in Person House-1)` and the like. No rule reads the slot names in either program
([Q-M1f.306](../../../decisions/hypotheses/Q-M1f.306-the-blind-enumerators-objects-are-the-programs.md)).

## Relations and facts

The page: [relations and facts](../../reasoning/01-static-ir/02-relations-and-facts.md).

### Where it lives

| notion | code |
|---|---|
| the knowledge base: a program, a state, its provenance | `ein_core::kb::Kb`; the program's registries, `ein_core::program`; the facts as interned rows, `ein_core::facts` ([`03-design/03-data-model.md`](../03-design/03-data-model.md)) |
| a stored negation | a fact of the relation `not`, one argument; matched by a plan step of its shape (`ein-infer/src/compile.rs`) |
| a contradiction, and a dead state | `ein_infer::contradiction::detect`, `has_contradiction`, `contradicts` |
| the state only grows; a fact stated twice | `Kb`'s writes, which add and never remove; the loader's path keeps the first statement's record |
| reading a text into a state | `ein-ir/src/from_ir.rs`: `load`, its passes and their accumulated refusals ([`03-design/04-ir-frontend.md` § 9](../03-design/04-ir-frontend.md#9-the-loader)) |
| a cycle among the derivations the text states | `ein_core::walks::detect_provenance_cycles` |
| a fork | `ein_core::kb::Kb::fork`: immutable layers below one writable top ([`03-design/03-data-model.md` § 5.1](../03-design/03-data-model.md#51-the-fork)); the fork's own derivation records, `ein_core::prov::ProvArena`'s fork region |
| a node resumes root's saturation | `ein_infer::saturator::Saturator::resume` |
| a derivation record | `ein_core::prov::Prov` and `ProvKind`; `Prov::is_terminal`, `Prov::same_justification` |
| the derivations of a fact, the primary first | `Kb::record_justification`, `Kb::primary`, `Kb::alternatives`, and the bound `ein_core::kb::MAX_ALT_JUSTIFICATIONS` |
| the derivation graph and the frontier | `ein_core::walks`: `walk_premises`, `build_derivation_dag`, `is_frontier`, `unsat_core` ([`03-design/03-data-model.md` § 8.4](../03-design/03-data-model.md#84-derivation-walks)) |
| equality and inequality, computed | `ein-infer/src/predicates.rs`; the store's equality classes, which nothing writes and no matcher reads ([`03-design/03-data-model.md` § 10](../03-design/03-data-model.md#10-equality-classes)) |

### What holds it

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

### Commands that show it

```sh
ein saturate three-houses.ein --dump                       # the state by origin: BACKGROUND, GIVEN, DERIVED
ein render kb --colour-by origin three-houses.ein          # the same state drawn: given black, derived blue and dashed
ein solve three-houses.ein --no-search --trace trace.md    # root's firings, each with its premises, in trace.md
```

How many derivations a run keeps for one fact is a `(config …)` flag, `:record-alternative-justifications`
([`06-config.md`](../../user/04-reference/06-config.md)): `false` keeps the primary alone. It changes no state and
no answer; it may change a dead node's core.

## Values and their order

The page: [values and their order](../../reasoning/01-static-ir/03-values-and-order.md).

### Where it lives

| notion | code |
|---|---|
| no output depends on the numbering of ids | the contract and its rules, [`03-design/01-determinism-and-order.md`](../03-design/01-determinism-and-order.md) |
| the total order of the values | `ein_core::terms::Terms::cmp_semantic` and `cmp_fact_semantic`; the names' order, the interner's rank table, `rank` in `ein-core/src/intern.rs` ([`03-design/03-data-model.md` § 2.1](../03-design/03-data-model.md#21-the-rank-table)) |
| the order of the candidates, by text | `order_key` in `ein-infer/src/region.rs`, which H is sorted by ([`03-design/01-determinism-and-order.md` § 4.2](../03-design/01-determinism-and-order.md#42-the-hypothesis-set-and-the-search)) |
| a state's identity is its fact set | `ein_infer::canon::state_key`, the sorted list of fact ids, compared whole |
| a firing's identity | `ein_infer::firing::BindingKey`: the plan, the activator, the registers; the activator half is the compile key's, `ein_infer::compile::PlanKey` |
| a value as Ein text that reads back | the one printer, `ein-core/src/text.rs`; `Terms::display` and `Terms::fact_text` |
| the quoted form | `ein_core::repr::repr` and `repr_str`, with printability in `ein-core/src/printable.rs`; its escapes, code point by code point, are [`03-design/01-determinism-and-order.md` § 7](../03-design/01-determinism-and-order.md#7-the-value-renderer) |
| a drawing's labels | `ein-render/src/dot_util.rs` |

### What holds it

| claim | test |
|---|---|
| no rendering of any corpus program depends on the order names were numbered in | `ein-render/tests/id_order_invariance.rs`: `every_rendering_reproduces_its_digest_and_none_depends_on_the_id_order`, `an_id_order_that_reaches_an_output_is_caught` |
| no rendering depends on how many workers ran | `ein-render/tests/jobs_invariance.rs`: `jobs_does_not_move_any_observable` |
| H is listed in the universe's order, as the binary lists it | `ein-cli/tests/show_hyps.rs`, which holds the listing on the hypothesis space's page to the binary's |
| names order by code point, whatever order they were read in | `ein-core/tests/values_semantics.rs`: `the_rank_table_orders_names_by_code_point`; the term store's unit test `semantic_order_sorts_names_by_text` in `ein-core/src/terms.rs` |
| a name sorts before a fact; facts order by relation, then arguments, then arity | the term store's unit tests `semantic_order_sorts_names_by_text`, `facts_order_as_their_identity_tuples_do` in `ein-core/src/terms.rs` |
| a state's identity is its fact list, never a digest; two different closures have different identities | `ein-infer/tests/search_semantics.rs`: `state_key_is_identity_not_a_digest`, `distinct_closures_have_distinct_keys` |
| two activators differing in any argument both fire, in either order, and the order they are written in loses no derivation | `ein-infer/tests/rule_semantics.rs`: `activators_differing_only_by_an_int_argument_both_fire`, `activators_differing_only_by_a_nested_fact_argument_both_fire`, `an_int_beside_a_nested_fact_in_one_position_loses_no_derivation`, `an_int_and_a_string_with_one_text_are_two_activators`; the corpus programs `examples/ein-bugs/binding-key-both.ein`, `binding-key-swapped.ein`, `binding-key-int-only.ein`, `binding-key-nested-only.ein`, `activator-text-int-and-string.ein` and `activator-text-string-then-int.ein` |
| an activator binds a parameter to a nested fact | `ein-infer/tests/compile_semantics.rs`: `a_value_parameter_an_activator_binds_to_a_nested_fact_is_bound` |
| a name is printed bare exactly where it reads back as that name | the grammar's unit test `the_printers_symbol_is_the_lexers` in `ein-ir/src/grammar.rs`; `ein-core/tests/values_semantics.rs`: `display_renders_every_value_shape` |
| every state an answer's files hold reads back as that state | `ein-cli/tests/answer_round_trip.rs` |
| no keyword pair reaches a value's label in a drawing | the renderer's unit test `no_keyword_pair_reaches_a_values_label` in `ein-render/src/dot_util.rs`; `ein-ir/tests/grammar_pages.rs`: `every_row_of_the_grammars_table_is_refused_at_the_parse` |
| the quoted form of every value shape, and its escape for every code point | `ein-core/tests/format_tables.rs`: `repr_of_every_reachable_value_shape`, `repr_escape_table_code_point_by_code_point` |
| a refutation's edges into `⊥` are in the order of the facts' quoted forms, whatever order they were collected in | `ein-render/tests/presentation_semantics.rs`: `the_bottom_edges_of_a_refutation_are_sorted_not_collection_ordered` |

### Commands that show it

**The activator example.** The page's rule `note` with its two activators is
[`examples/ein-bugs/binding-key-both.ein`](../../../../examples/ein-bugs/binding-key-both.ein); `binding-key-swapped.ein`
writes the two activators in the other order, and there is one file for each activator alone.

```sh
ein saturate examples/ein-bugs/binding-key-both.ein --dump
ein saturate examples/ein-bugs/binding-key-swapped.ein --dump
```

Both print the same derived facts:

<!-- output -->
```text
;; ── DERIVED (3 facts) ──
;;   noted (2)
  (noted 1 1 :rule note)
  (noted 2 1 :rule note)
  (written (src Y) :rule "#written")
```

**The two orders.** `ein show hyps FILE` lists H in the candidates' order, by text. Where a run tabulates the
facts its listed states differ in, `ein solve FILE --maximal --models key` or `--models grid`, the facts are in
the values' order.

## Rules as statements

The page: [rules as statements](../../reasoning/01-static-ir/04-rules-as-statements.md).

### Where it lives

| notion | code |
|---|---|
| a rule: parameters, pattern, conclusion, reason | the grammar's `rule` form in `ein-ir/src/ein.y`; the record, `ein-core/src/entities.rs`; registered by `ingest_rules` in `ein-ir/src/from_ir.rs` |
| a conclusion read flat | `ein_ir::Ast::conjuncts`, the one function every reader of a conclusion goes through |
| a conclusion's variable bound by every way of matching | `refuse_unbound_conclusions` in `ein-ir/src/from_ir.rs` |
| a variable in a relation's place is a parameter | `validate_relation_parameters` in `ein-ir/src/from_ir.rs` |
| a rule applied once per activator | `ein_infer::compile::compile_rule`, to an `ein_infer::plan::Plan` per activator, cached under `PlanKey` ([`03-design/05-matcher.md`](../03-design/05-matcher.md)) |
| satisfaction of a pattern | the register matcher, `ein-infer/src/match_.rs`; equality and inequality, `ein-infer/src/predicates.rs` |
| an absence as a condition on a match | lifted at compile time to an `ein_infer::plan::NafGuard`, judged at the boundary on a quiescent KB ([`03-design/06-saturation.md`](../03-design/06-saturation.md)) |
| a firing adds and records | `ein-infer/src/firing.rs`: `fire`, `build_conclusion` |
| the universal premise and the undecided fact | the macros `forall` and `unknown` of `stdlib/macro.ein`, expanded by `ein-ir/src/macros.rs` |
| the structural rules of the standard library | `domain-elimination` and `range-elimination` in `stdlib/bijection.ein`; `slot-elimination` and `slot-fill` in `stdlib/slots.ein` |
| a rule drawn as its match and its conclusion | `ein-render/src/rules.rs` |

The four levels are a reading of the one store and have no type of their own in the code: an object is a
`Symbol`, a fact a row, a relation and a rule an entry of the program's registries (`ein-core/src/program.rs`),
and a declaration's facts are ordinary rows of the relation `relation`.

### What holds it

| claim | test |
|---|---|
| a rule with parameters and no activator never fires, and an activator applies it over the relation it names alone | `ein-infer/tests/rule_semantics.rs`: `a_parametrised_rule_with_no_activator_is_dormant` |
| a rule with no parameters fires once over an empty pattern, and once per activator with parameters | `ein-infer/tests/rule_semantics.rs`: `an_empty_match_fires_once_or_once_per_activator` |
| `eq` and `neq` are computed from the bindings, each admitting what the other rejects | `ein-infer/tests/rule_semantics.rs`: `eq_admits_exactly_the_bindings_neq_rejects` |
| `domain-elimination` concludes the last value left, and nothing while two survive | `ein-infer/tests/stdlib_semantics.rs`: `domain_elimination_forces_the_last_surviving_value`, `range_elimination_forces_the_last_surviving_object`, `domain_elimination_says_nothing_while_two_values_survive` |
| a `forall` over an empty domain holds | `ein-infer/tests/rule_semantics.rs`: `a_forall_over_an_empty_domain_is_vacuously_true` |
| a declaration is stored as facts of `relation` | `ein-infer/tests/compile_semantics.rs`: `a_bare_declaration_is_declared_with_an_empty_signature_and_stores_one_fact` |
| every stdlib rule is a rule written in Ein, and each is applied by a program of the suite | `ein-infer/tests/stdlib_coverage.rs`: `every_stdlib_rule_is_activated_by_a_program` |
| no run reaches a name the program does not have | `ein-infer/tests/alive_invariant.rs`: `no_run_reaches_a_name_the_program_does_not_have` |

### Commands that show it

```sh
ein render rules three-houses.ein                                   # each rule: its match over its assert
ein render rules --rule-mode overlay three-houses.ein               # each rule as one graph: the match solid, what the assert adds dashed
ein render rule --name domain-elimination stdlib/bijection.ein      # one rule by name
ein saturate three-houses.ein --dump                                # the three rules' firings, and each rule's stratum
```

**The levels in a real program.** In [`examples/zebra2.ein`](../../../../examples/zebra2.ein): `Red` and `House-1`
are L0; `(color-loc Red House-1)` is L1 and names both; `(relation color-loc Color House)` and
`(bijective color-loc)` are L2; the rules `std.bijection` brings are L3, applied to `color-loc` through the
activators its setup rule derives from `(bijective color-loc)`, and reading L1 `color-loc` facts.
`ein render constraints examples/zebra2.ein` draws the L2 property facts, and `ein render kb` the L0 and L1 of the
saturated root.
