# Static IR in code

Where `ein.rs` implements the two pages of [`reasoning/01-static-ir/`](../../reasoning/01-static-ir/), the tests
that hold it to them, and the commands that show each notion. The layout is the folder's
([README](README.md#how-a-page-is-laid-out)).

- [The running example](#the-running-example)
- [The knowledge base](#the-knowledge-base)
- [The rules](#the-rules)

**The words.** The reasoning calls every node of a knowledge base's graph an **object**: an **atom**, a node
with a name and no outgoing arrow, or a **fact**, a node with outgoing arrows, the first of which points at its
relation. In the code an object is a `Value`, an atom a `Symbol` and a fact a `FactId`; the code's own names say
*value* and, for the atoms the blind enumerator guesses about, *object* (`candidate_objects`). This page uses
the reasoning's words and names the code's items as they are spelt.

## The running example

The two pages return to one small program, the *three-house example*. They show it as formulas and graphs; this
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

**Where each drawing of the reasoning comes from.** The drawings of the two pages are files under
[`reasoning/01-static-ir/diagrams/`](../../reasoning/01-static-ir/diagrams/): each a source, `.dot` or `.puml`,
named for its page and what it shows, and the `.svg` drawn from it, which the page shows. Every command below
writes DOT on stdout ([rendering](../../user/04-reference/12-rendering.md) is the legend); a source that comes
from one is the digraph the command names, with the layout hints and comment lines taken out.

| page | drawing | command |
|---|---|---|
| the knowledge base | the graph of the four facts; the relation, its extent and two facts about it; the declaration of `lives-in`; what an arrow points at | **none**: drawn by hand, see below |
| the knowledge base | the definitions as a class diagram | by hand, in PlantUML |
| the knowledge base | the two forms side by side | by hand; its right half is what `ein render program three-houses.ein` draws of two facts in its digraph `facts` |
| the rules | the pattern, the match, the result of rewriting, the rule `transitive` with its activator pattern | **none**: drawn by hand |
| the rules | `pet-follows-owner`, `nobody-home` | `ein render rules three-houses.ein` |
| the rules | `pet-follows-owner` as one graph | `ein render rules --rule-mode overlay three-houses.ein` |
| the rules | `domain-elimination` | `ein render rule --name domain-elimination stdlib/bijection.ein` |

**No command draws the graph as the reasoning defines it.** In that graph a relation is a node, the atom under
the first arrow of each of its facts, and a fact has no label. Every view `ein render` has writes the relation's
name on the fact instead: as the label of an arrow, or, under `ein render program --levi`, in the octagon of the
fact, with numbered arrows to the arguments only. So `--levi` draws the name `left-of` on two octagons where the
graph has one atom, and numbers the arrows by argument where the reasoning counts them from the relation's. The
drawings of the graph on the two pages are written by hand in the reasoning's vocabulary: a box for an atom, an
unlabelled octagon for a fact, filled where the knowledge base holds it, and a number on each arrow for its
place. The other hand-drawn schematics are the chain of objects in their order, the gluing square and the four
levels.

## The knowledge base

The page: [the knowledge base](../../reasoning/01-static-ir/01-knowledge-base.md).

### Where it lives

| notion | code |
|---|---|
| an object: an atom or a fact | `ein_core::value::Value`, four bytes, tagged `Sym` or `Fact` ([`03-design/03-data-model.md` § 3](../03-design/03-data-model.md#3-values-and-fact-ids)) |
| one name, one node: an atom | interning: `ein-core/src/intern.rs`; `ein_core::terms::Terms::intern_text`, one `Symbol` per distinct text |
| one list of arrows, one node: a fact | an interned row of its relation and its arguments, `Terms::intern_fact`; the facts as rows, `ein_core::facts` |
| the knowledge base: the facts held | `ein_core::kb::Kb` ([`03-design/03-data-model.md`](../03-design/03-data-model.md)); a fact that is only pointed at is interned in `Terms` and in no `Kb` |
| the relation of a fact, its arguments | the row's first field and the rest; a relation is a `Symbol`, never a `Fact` |
| the extent of a relation | `Kb`'s index of the held facts by relation (`facts_by_relation`) |
| the name characters, and a name as a run of them | the lexer's `NAME` rule in `ein-ir/src/ein.l`; `ein_core::text::is_name_char` and `is_symbol`, the printer's copy of it, held equal to the lexer's by a test |
| the reserved words | `ein_core::terms::RESERVED` (what no declaration binds), `STRUCTURAL` and `PREDICATES` (the words of the pattern language); the grammar's own list in `ein-ir/src/grammar.rs` |
| a name the loader makes up | `ein_core::text::INTERNAL_MARK`, `#`, and `INTERNAL_NAMES` ([`07-reserved-engine-strings.md`](../07-reserved-engine-strings.md)) |
| the kernel's two markers | `ein_core::terms::CLOSED`, `SYMMETRIC` |
| a stored negation, falsity | a fact of the relation `not`, one argument, and the fact of `false`, none: `ein_core::terms::Terms`' `kernel` symbols |
| what is an object and what is not | the grammar: `FactHead`, `ValueHead`, `NestedFact`, `Negated` in `ein-ir/src/ein.y`; after macro expansion, `refuse_malformed_primitives` in `ein-ir/src/from_ir.rs` |
| a declaration as facts | `ingest_relation` in `ein-ir/src/from_ir.rs` |
| the declared count of arguments | `declared_counts` and `wrong_counts` in `ein-ir/src/from_ir.rs` |
| reading a text into $K_0$ | `ein-ir/src/from_ir.rs`: `load`, its passes and their accumulated refusals ([`03-design/04-ir-frontend.md` § 9](../03-design/04-ir-frontend.md#9-the-loader)) |
| `written` | `state_written` and `written_in_conclusions` in `ein-ir/src/from_ir.rs`; `ein_core::terms::WRITTEN`, and `WRITTEN_STEP`, the made-up rule name `#written` its facts are derived by |
| full names and a file's short names | `ein-ir/src/imports.rs`: `resolve_imports`, `short_names` ([`03-design/04-ir-frontend.md` § 8](../03-design/04-ir-frontend.md#8-import-resolution)) |
| equality, computed and never stored | `ein-infer/src/predicates.rs`; the store's equality classes, which nothing writes and no matcher reads ([`03-design/03-data-model.md` § 10](../03-design/03-data-model.md#10-equality-classes)) |
| the total order of the objects | `ein_core::terms::Terms::cmp_semantic` and `cmp_fact_semantic`; the names' order, the interner's rank table, `rank` in `ein-core/src/intern.rs` ([`03-design/03-data-model.md` § 2.1](../03-design/03-data-model.md#21-the-rank-table)) |
| the text of an object, which reads back | the one printer, `ein-core/src/text.rs`; `Terms::display` and `Terms::fact_text` |
| the quoted form | `ein_core::repr::repr` and `repr_str`, with printability in `ein-core/src/printable.rs`; its escapes, code point by code point, are [`03-design/01-determinism-and-order.md` § 7](../03-design/01-determinism-and-order.md#7-the-value-renderer) |
| a contradiction, and a dead set of facts | `ein_infer::contradiction::detect`, `has_contradiction`, `contradicts` |

### What holds it

| claim | test |
|---|---|
| a run of digits is a name, as written, in every place a name is written; `1` and `01` are two names | `ein-cli/tests/cli_semantics.rs`: `a_run_of_digits_is_a_name_in_every_place`, `a_range_is_the_name_it_spells`; `ein-core/tests/values_semantics.rs`: `a_run_of_digits_is_a_name_as_written` |
| no name the loader makes up is a name a program can write | `ein-ir/tests/generated_parser.rs`: `no_name_the_loader_makes_up_is_a_name` |
| a kernel marker is refused wherever a program binds it | `ein-ir/tests/engine_names.rs`: `a_dunder_is_refused_wherever_a_program_binds_it` |
| one module reached through several imports is one copy; a short name is local to its file | `ein-ir/tests/imports_full_names.rs`: `the_users_dependency_graph`, `a_short_name_is_local_to_its_file`; `ein-ir/tests/imports_semantics.rs`: `one_module_under_two_spellings_is_one_copy` |
| a form of a declared relation at another count is refused, and the labels are no type check | `ein-cli/tests/declared_counts.rs`: `a_form_of_another_count_is_refused_on_every_command`, `what_is_not_a_declared_relations_form_loads` |
| a relation name declared twice is refused at load | `ein-ir/tests/load_semantics.rs`: `the_load_negative_fixtures_are_byte_identical` |
| a declaration is stored as the fact with its labels and the fact of one argument; a bare one as one fact | `ein-infer/tests/compile_semantics.rs`: `a_bare_declaration_is_declared_with_an_empty_signature_and_stores_one_fact`; the loader's unit test `a_declaration_stores_itself_as_facts_so_rules_can_read_signatures` in `ein-ir/src/from_ir.rs` |
| a relation nothing declares is a relation | the loader's unit test `an_undeclared_head_vivifies_a_relation_but_a_predicate_does_not` in `ein-ir/src/from_ir.rs` |
| the loader states `written` for what a file nests and for a conclusion's ground nested facts, and a program may not write it | `ein-cli/tests/written_facts.rs`: `what_the_loader_states`, `a_program_does_not_write_written` |
| a nested fact pairs with its negation by its arrows, at every depth | `ein-infer/tests/search_semantics.rs`: `a_nested_positive_pairs_through_structural_identity` |
| a form that is no object is refused, and every form a knowledge base could hold loads | `ein-cli/tests/value_rule.rs`: `a_form_that_is_no_value_is_refused_on_every_command`, `what_a_state_could_hold_loads` |
| `P` beside `(not P)` is a contradiction whatever either half's origin, and every pair is reported | `ein-infer/tests/search_semantics.rs`: `mutual_negation_is_detected_whatever_either_sides_origin`, `every_pair_is_reported_not_only_the_first` |
| matching merges no names | `ein-infer/tests/naf_semantics.rs`: `matching_does_not_resolve_equality_classes` |
| names order by code point, whatever order they were read in | `ein-core/tests/values_semantics.rs`: `the_rank_table_orders_names_by_code_point`; the term store's unit test `semantic_order_sorts_names_by_text` in `ein-core/src/terms.rs` |
| a name sorts before a fact; facts order by relation, then arguments, then count | the term store's unit tests `semantic_order_sorts_names_by_text`, `facts_order_as_their_identity_tuples_do` in `ein-core/src/terms.rs` |
| a name is printed bare exactly where it reads back as that name | the grammar's unit test `the_printers_symbol_is_the_lexers` in `ein-ir/src/grammar.rs`; `ein-core/tests/values_semantics.rs`: `display_renders_every_value_shape` |
| every knowledge base an answer's files hold reads back as it was written | `ein-cli/tests/answer_round_trip.rs` |
| no keyword pair reaches an object's label in a drawing | the renderer's unit test `no_keyword_pair_reaches_a_values_label` in `ein-render/src/dot_util.rs`; `ein-ir/tests/grammar_pages.rs`: `every_row_of_the_grammars_table_is_refused_at_the_parse` |
| the quoted form of every shape of object, and its escape for every code point | `ein-core/tests/format_tables.rs`: `repr_of_every_reachable_value_shape`, `repr_escape_table_code_point_by_code_point` |

### Commands that show it

```sh
ein saturate three-houses.ein --dump     # the facts held, the declarations' two facts each among them, and a census of the names
ein render program three-houses.ein      # the facts drawn with labelled arrows: the digraph `facts`
ein render program --levi three-houses.ein   # each fact an octagon with its relation's name in it, arrows to its arguments
```

The census `ein saturate` prints counts, among the names, how often each stands as a fact's relation, its
*head-participations*, and how often as an argument, its *arg-participations*. It reads the facts held and
their own arguments: a name that stands only inside a nested fact, or as the relation of a fact that is only
pointed at, is in neither count.

**A declaration is a fact a rule's `:match` reads, and its labels are ordinary names**
([Q-M1f.372](../../../decisions/language/Q-M1f.372-ein-has-no-types-a-declarations-slot-names-are.md)). The rule
of [the rules § Relation-polymorphic](../../reasoning/01-static-ir/02-rewriting-rules.md#relation-polymorphic) that
gives a label a meaning, as a program. A variable in a relation's place must be a parameter, so the rule takes
`?R` and an activator names the relation:

```lisp
(relation is-a Thing Kind :why "{?1} is a {?2}")
(relation lives-in Person House :why "{?1} lives in {?2}")
(relation fits Thing Kind :why "{?1} fills a first place labelled {?2}")

(is-a Norwegian Person)
(is-a House-1 House)
(lives-in Norwegian House-1)
(lives-in House-1 Norwegian)

(rule first-fits (?R)
  :match  (and (relation ?R ?T1 ?T2) (?R ?x1 ?x2) (is-a ?x1 ?T1))
  :assert (fits ?x1 ?T1)
  :why    "{?R} labels its first place {?T1} and {?x1} is one")

(first-fits lives-in)
```

`ein saturate FILE --dump` lists the facts the declarations are stored as, `(relation lives-in Person House)` and
`(relation lives-in)` among the background facts, and one derived fact, `(fits Norwegian Person :rule
first-fits)`. `(lives-in House-1 Norwegian)` loads, and the rule concludes nothing from it. With the rule
written `(rule first-fits () …)` the program is refused at load: *its :match writes (?R ?x1 ?x2) with ?R as its
relation, which no parameter binds*.

**The one place a label is read by name** is the blind enumerator's, and the two programs that show it are
with it ([hypotheses and the answer § 1.4](03-hypotheses-and-the-answer.md#14-a-program-for-each-case)).

**The two orders.** Where a run tabulates the facts its listed states differ in, `ein solve FILE --maximal
--models key` or `--models grid`, the facts are in the order of the objects. `ein show hyps FILE` lists H by
text, which is another order
([hypotheses and the answer](03-hypotheses-and-the-answer.md#3-solution-semantics)).

## The rules

The page: [the rules](../../reasoning/01-static-ir/02-rewriting-rules.md).

### Where it lives

| notion | code |
|---|---|
| a rule: parameters, pattern, conclusion, reason | the grammar's `rule` form in `ein-ir/src/ein.y`; the record, `ein-core/src/entities.rs`; registered by `ingest_rules` in `ein-ir/src/from_ir.rs` |
| a match: a pattern's variables given objects | the register matcher, `ein-infer/src/match_.rs`, over a plan (`ein_infer::plan::Plan`) |
| equality and inequality, and a variable only they bind | `ein-infer/src/predicates.rs`; the guard that ranges over the program's names, lowered in `ein-infer/src/compile.rs` |
| an absence as a condition on a match | lifted at compile time to an `ein_infer::plan::NafGuard` ([`03-design/06-saturation.md`](../03-design/06-saturation.md)); when it is judged is [saturation and negation](02-saturation-and-negation.md#3-absence) |
| the universal premise and the open fact | the macros `forall` and `unknown` of `stdlib/macro.ein`, expanded by `ein-ir/src/macros.rs` |
| a conclusion read flat | `ein_ir::Ast::conjuncts`, the one function every reader of a conclusion goes through |
| the instance of a conclusion | `build_conclusion` in `ein-infer/src/firing.rs` |
| a rule applied under an activator | `ein_infer::compile::compile_rule`, to a plan per activator, cached under `PlanKey` ([`03-design/05-matcher.md`](../03-design/05-matcher.md)) |
| a conclusion's variable bound by every way of matching | `refuse_unbound_conclusions` in `ein-ir/src/from_ir.rs` |
| a variable in a relation's place is a parameter | `validate_relation_parameters` in `ein-ir/src/from_ir.rs` |
| the names of a program | `intern_program_names` in `ein-ir/src/from_ir.rs`; `ein_infer::invariant::program_names` |
| the structural rules of the standard library | `domain-elimination` and `range-elimination` in `stdlib/bijection.ein`; `slot-elimination` and `slot-fill` in `stdlib/slots.ein` |
| a rule drawn as its match and its conclusion | `ein-render/src/rules.rs` |

The four levels are a reading of the one store and have no type of their own in the code: a thing named is a
`Symbol`, a fact a row, a relation and a rule an entry of the program's registries (`ein-core/src/program.rs`),
and a declaration's facts are ordinary rows of the relation `relation`. What a rewriting does when it is carried
out, a firing, is [saturation and negation](02-saturation-and-negation.md#1-firing-a-rule).

### What holds it

| claim | test |
|---|---|
| a pattern matches into a nested fact | `ein-infer/tests/rule_semantics.rs`: `a_nested_pattern_binds_inside_the_nested_fact` |
| `(not P)` in a premise matches a stored negation, not the absence of `P` | `ein-infer/tests/rule_semantics.rs`: `not_in_a_premise_is_stored_negation_not_negation_as_failure` |
| `(unknown P)` admits exactly the facts neither held nor denied | `ein-infer/tests/naf_semantics.rs`: `unknown_admits_exactly_the_undecided_pairs` |
| `eq` and `neq` are computed from the bindings, each admitting what the other rejects; a variable only a guard binds ranges over the program's names; a conclusion of either is checked and never stored | `ein-infer/tests/rule_semantics.rs`: `eq_admits_exactly_the_bindings_neq_rejects`, `a_guards_free_variable_ranges_over_the_programs_names`, `an_eq_or_neq_conclusion_is_checked_never_stored` |
| a `forall` over an empty domain holds | `ein-infer/tests/rule_semantics.rs`: `a_forall_over_an_empty_domain_is_vacuously_true` |
| a rule with parameters and no activator has no application, and an activator applies it over the relation it names alone | `ein-infer/tests/rule_semantics.rs`: `a_parametrised_rule_with_no_activator_is_dormant` |
| a rule with no parameters matches an empty pattern once, and once per activator with parameters | `ein-infer/tests/rule_semantics.rs`: `an_empty_match_fires_once_or_once_per_activator` |
| an activator gives a parameter a nested fact | `ein-infer/tests/compile_semantics.rs`: `a_value_parameter_an_activator_binds_to_a_nested_fact_is_bound` |
| a relation parameter bound to a fact, written or derivable, is refused at load | `ein-cli/tests/cli_semantics.rs`: `a_derived_activator_binding_a_relation_parameter_to_a_fact_is_refused_at_load`; `ein-infer/tests/compile_semantics.rs`: `a_relation_parameter_a_rule_binds_to_a_nested_fact_is_refused` |
| a rule name declared twice is refused at load | `ein-ir/tests/load_semantics.rs`: `the_load_negative_fixtures_are_byte_identical`, over `examples/broken/load/rule_duplicate_name.ein` and `hrule_duplicate_name.ein` |
| `domain-elimination` concludes the last object left, and nothing while two survive | `ein-infer/tests/stdlib_semantics.rs`: `domain_elimination_forces_the_last_surviving_value`, `range_elimination_forces_the_last_surviving_object`, `domain_elimination_says_nothing_while_two_values_survive` |
| a declaration is stored as facts of `relation` | `ein-infer/tests/compile_semantics.rs`: `a_bare_declaration_is_declared_with_an_empty_signature_and_stores_one_fact` |
| every stdlib rule is a rule written in Ein, and each is applied by a program of the suite | `ein-infer/tests/stdlib_coverage.rs`: `every_stdlib_rule_is_activated_by_a_program` |

### Commands that show it

```sh
ein render rules three-houses.ein                                   # each rule: its match over its assert
ein render rules --rule-mode overlay three-houses.ein               # each rule as one graph: the match solid, what the assert adds dashed
ein render rule --name domain-elimination stdlib/bijection.ein      # one rule by name
ein saturate three-houses.ein --dump                                # each rule with its parameters and its stratum
```

**The levels in a real program.** In [`examples/zebra2.ein`](../../../../examples/zebra2.ein): `Red` and `House-1`
are L0; `(color-loc Red House-1)` is L1 and names both; `(relation color-loc Color House)` and
`(bijective color-loc)` are L2; the rules `std.bijection` brings are L3, applied to `color-loc` through the
activators its setup rule derives from `(bijective color-loc)`, and reading L1 `color-loc` facts.
`ein render constraints examples/zebra2.ein` draws the L2 property facts, and `ein render kb` the L0 and L1 of the
saturated root.
