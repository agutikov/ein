# Static IR in code

Where `ein.rs` implements the two pages of [`reasoning/01-static-ir/`](../../reasoning/01-static-ir/), the tests
that hold it to them, and the commands that show each notion. The layout is the folder's
([README](README.md#how-a-page-is-laid-out)).

- [The running example](#the-running-example)
- [The knowledge base](#the-knowledge-base)
- [The rules](#the-rules)

**The words.** The reasoning calls every node of a knowledge base's graph an **object**: an **atom**, a node
with a name and no outgoing arrow, or a **fact**, a node with outgoing arrows, the first of which points at its
relation. A **relation** is that one atom; the facts of it, held or not, are its **extent**, and those a
knowledge base holds are its **held extent**. In the code an object is a `Value`, an atom a `Symbol` and a fact
a `FactId`; the code's own names say *value* and, for the atoms the blind enumerator guesses about, *object*
(`candidate_objects`). This page uses the reasoning's words and names the code's items as they are spelt.

## The running example

The two pages return to one small program, the *three-house example*. They show it as formulas and graphs; this
is its source. Save it as `three-houses.ein`:

```lisp
(relation is-a Thing Kind :why "{?3} is a {?4}")
(relation lives-in Person House :why "{?3} lives in {?4}")
(relation owns Person Pet :why "{?3} owns {?4}")
(relation pet-at Pet House :why "{?3} is kept at {?4}")
(relation left-of House House :why "{?3} is left of {?4}")
(relation vacant House :why "{?3} is vacant")

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
`absent` over the facts of `lives-in` that a state holds, the relation's held extent
([the knowledge base](../../reasoning/01-static-ir/01-knowledge-base.md#the-relation-its-extent-a-fact-of-it)),
which the blind enumerator's candidates may extend, so a bare `ein solve` flags the program as not monotone.
`ein saturate` stops at the root, and `--dump` prints it by origin:

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
| the knowledge base | the graph of the four facts; the relation, its extent and two facts about it; the declaration of `lives-in`; what an arrow points at; the three cases of a fact in a context; a negation beside a context | **none**: drawn by hand, see below |
| the knowledge base | the definitions as a class diagram | by hand, in PlantUML |
| the knowledge base | the two forms side by side | by hand; its left half is what `ein render program three-houses.ein` draws of two facts in its digraph `kb`, its right half what `ein render program --fold three-houses.ein` draws of them |
| the rules | the pattern, the match, the result of rewriting, the rule `transitive` with its activator pattern; a packing and an unpacking, each with its match and its result | **none**: drawn by hand |
| the rules | `pet-follows-owner`, `nobody-home` | `ein render rules three-houses.ein` |
| the rules | `pet-follows-owner` as one graph | `ein render rules --rule-mode overlay three-houses.ein` |
| the rules | `domain-elimination` | `ein render rule --name domain-elimination stdlib/bijection.ein` |

**Every view draws the graph as the reasoning defines it**
([Q-M1f.419](../../../decisions/cli/Q-M1f.419-ein-render-draws-a-fact-as-the-graph-has-it.md)): a box for an atom,
the relation's atom one node under the first arrow of each of its facts, an unlabelled octagon for a fact, a
number on each arrow for its place counted from the relation's, and a dashed frame around a fact that a fact points
at and the knowledge base does not hold, named for the relation of the facts that point at it; a held fact is in no
frame, and a fact is drawn once, its octagon and no arrow (`ein-render/src/fact_graph.rs`, the one drawing of facts
the KB, program and constraint views share; `ein-render/src/dot_util.rs`' `ctx_frames`), laid out in layers by
its first arrow written back from its relation (`fact_graph::first_arrow`). Under `--fold` a fact of three arrows
that nothing points at is one arrow labelled with its relation's name, and a fact of two a labelled loop
(`fact_graph::folds`), which is the drawing with labelled arrows the knowledge base page shows. The drawings of the graph on the two pages are written by hand in
the same vocabulary. The other
hand-drawn schematics are the chain of objects in their order, the gluing square and the four levels.

## The knowledge base

The page: [the knowledge base](../../reasoning/01-static-ir/01-knowledge-base.md).

### Where it lives

| notion | code |
|---|---|
| an object: an atom or a fact | `ein_core::value::Value`, four bytes, tagged `Sym` or `Fact` ([`03-design/03-data-model.md` § 3](../03-design/03-data-model.md#3-values-and-fact-ids)) |
| one name, one node: an atom | interning: `ein-core/src/intern.rs`; `ein_core::terms::Terms::intern_text`, one `Symbol` per distinct text |
| one list of arrows, one node: a fact | an interned row of its relation and its arguments, `Terms::intern_fact`; the facts as rows, `ein_core::facts` |
| the knowledge base: the facts held | `ein_core::kb::Kb` ([`03-design/03-data-model.md`](../03-design/03-data-model.md)); a fact that is only pointed at is interned in `Terms` and in no `Kb` |
| the global context, and the context of a fact | the global context is the held set of a `Kb`. No store keeps a fact's contexts: a fact in a context is an interned row that an argument of some held fact names (`Tag::Fact`), and a pattern reaches it through that held fact, by the matcher's `Nested` slot ([`03-design/05-matcher.md` § 3.1](../03-design/05-matcher.md#31-registers-trail-and-premise-slots)). A premise is matched among held facts only (`Kb::facts_of`, `Kb::facts_with`) |
| the relation of a fact, its arguments | the row's first field and the rest; a relation is a `Symbol`, never a `Fact` |
| the held extent of a relation, $\operatorname{ext}_K(r)$ | `Kb`'s index of the held facts by relation (`facts_by_relation`) |
| the name characters, and a name as a run of them | the lexer's `NAME` rule in `ein-ir/src/ein.l`; `ein_core::text::is_name_char` and `is_symbol`, the printer's copy of it, held equal to the lexer's by a test |
| the reserved words: where the kernel meets a theory written in Ein | `ein_core::terms::RESERVED` (what no declaration binds), `STRUCTURAL` and `PREDICATES` (the words of the pattern language); the grammar's own list in `ein-ir/src/grammar.rs`; the names the kernel itself knows, `ein_core::terms::Kernel`. A relation that is none of these is read by rules only |
| a name the loader makes up | `ein_core::text::INTERNAL_MARK`, `#`, and `INTERNAL_NAMES` ([`07-reserved-engine-strings.md`](../07-reserved-engine-strings.md)) |
| the kernel's one marker, of closure | `ein_core::terms::CLOSED`; no name of a theory is the kernel's, held by [`ein-render/tests/renaming_invariance.rs`](../../../../ein.rs/crates/ein-render/tests/renaming_invariance.rs) |
| a stored negation, falsity | a fact of the relation `not`, one argument, and the fact of `false`, none: `ein_core::terms::Terms`' `kernel` symbols |
| what is an object and what is not | the grammar: `FactHead`, `ValueHead`, `NestedFact`, `NegFact`, `NegPattern`, `Negated` in `ein-ir/src/ein.y`; after macro expansion, `refuse_malformed_primitives` in `ein-ir/src/from_ir.rs` |
| a declaration as facts | `ingest_relation` in `ein-ir/src/from_ir.rs` |
| the declared count of arguments | `declared_counts` and `wrong_counts` in `ein-ir/src/from_ir.rs` |
| reading a text into $K_0$ | `ein-ir/src/from_ir.rs`: `load`, its passes and their accumulated refusals ([`03-design/04-ir-frontend.md` § 9](../03-design/04-ir-frontend.md#9-the-loader)) |
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
| a nested fact pairs with its negation by its arrows, at every depth | `ein-infer/tests/search_semantics.rs`: `a_nested_positive_pairs_through_structural_identity` |
| a fact in a context is not held: a claim over its relation lists only what is held, and a rule that packs a held fact leaves what it points at as it was | the claims of [`examples/beliefs/02_packing.ein`](../../../../examples/beliefs/02_packing.ein), held with every corpus claim by `ein-cli/tests/corpus_cli.rs`: `every_cell_reproduces_its_exit_code`, its `test` run |
| a fact is held, in a fact's context, or both, and the two questions read one each: a pattern of its own the facts held, a pattern under an arrow what stands there | the claims of [`examples/beliefs/07_three_cases.ein`](../../../../examples/beliefs/07_three_cases.ein), by `ein-cli/tests/corpus_cli.rs`: `every_cell_reproduces_its_exit_code` |
| a declaration, a rule's activator, `(false)` and a denial under an arrow declare, apply, refute and contradict nothing | the claims of [`examples/beliefs/11_nothing_acts_in_a_context.ein`](../../../../examples/beliefs/11_nothing_acts_in_a_context.ein), by the same test; that a marker under an arrow marks nothing has no test |
| the negation of a negation is a fact a file writes and a rule matches and builds; built or written, the answer is the same; it contradicts the negation it denies and not the fact under both | `ein-cli/tests/negation_of_a_negation.rs`; the claims of [`03_liar_denial.ein`](../../../../examples/beliefs/03_liar_denial.ein), [`04_liar_denial_contradicted.ein`](../../../../examples/beliefs/04_liar_denial_contradicted.ein) and [`05_liar_denial_beside_the_fact.ein`](../../../../examples/beliefs/05_liar_denial_beside_the_fact.ein) |
| a form that is no object is refused, and every form a knowledge base could hold loads | `ein-cli/tests/value_rule.rs`: `a_form_that_is_no_value_is_refused_on_every_command`, `what_a_state_could_hold_loads` |
| `P` beside `(not P)` is a contradiction whatever either half's origin, and every pair is reported | `ein-infer/tests/search_semantics.rs`: `mutual_negation_is_detected_whatever_either_sides_origin`, `every_pair_is_reported_not_only_the_first` |
| matching merges no two atoms | `ein-infer/tests/naf_semantics.rs`: `matching_does_not_resolve_equality_classes` |
| names order by code point, whatever order they were read in | `ein-core/tests/values_semantics.rs`: `the_rank_table_orders_names_by_code_point`; the term store's unit test `semantic_order_sorts_names_by_text` in `ein-core/src/terms.rs` |
| an atom sorts before a fact; facts order by relation, then arguments, then count | the term store's unit tests `semantic_order_sorts_names_by_text`, `facts_order_as_their_identity_tuples_do` in `ein-core/src/terms.rs` |
| a name is printed bare exactly where it reads back as that name | the grammar's unit test `the_printers_symbol_is_the_lexers` in `ein-ir/src/grammar.rs`; `ein-core/tests/values_semantics.rs`: `display_renders_every_value_shape` |
| a fact of no argument is printed `(r)`, with no space inside, by every surface that prints a fact as text | `ein-cli/tests/fact_of_no_argument.rs`: `every_surface_prints_a_fact_of_no_argument_as_written`, `the_stale_row_and_the_form_print_one_text`, `a_derivation_cycle_names_its_facts_as_written` |
| every knowledge base an answer's files hold reads back as it was written | `ein-cli/tests/answer_round_trip.rs` |
| every drawing draws a fact as the graph has it, an octagon with no name in it whose arrow `1` is at its relation's atom, or under `--fold` a fact of three arrows that nothing points at as one labelled arrow and a fact of two as a labelled loop; no relation is drawn otherwise than another; an object is drawn once: a fact that a drawn fact points at is one octagon, and no arrow, in one frame that names every relation that points at it where it is not held, and in no frame where it is; no node is filled; the variables of a pattern under an arrow are drawn | `ein-render/tests/every_object_once.rs`, the KB view, the program view, the rule view in both modes and the constraint view, in both forms, and every view of every corpus program; the program view's `kb` against the KB view, `the_program_views_kb_is_the_kb_views_drawing_on_the_corpus`; every corpus program's drawings by `ein-render/tests/id_order_invariance.rs`, and Graphviz's reading of them by `ein-render/tests/dot_wellformed.rs` |
| no keyword pair reaches an object's label in a drawing | the renderer's unit test `no_keyword_pair_reaches_a_values_label` in `ein-render/src/dot_util.rs`; `ein-ir/tests/grammar_pages.rs`: `every_row_of_the_grammars_table_is_refused_at_the_parse` |
| the quoted form of every shape of object, and its escape for every code point | `ein-core/tests/format_tables.rs`: `repr_of_every_reachable_value_shape`, `repr_escape_table_code_point_by_code_point` |

### Commands that show it

```sh
ein saturate three-houses.ein --dump     # the facts held, the declarations' two facts each among them, and a census of the names
ein render program three-houses.ein          # the knowledge base as the graph: the digraph `kb`, each fact an octagon, its arrow 1 at its relation
ein render program --fold three-houses.ein   # each fact of three arrows one arrow labelled with its relation
```

The census `ein saturate` prints is of the atoms, under its row `names`, and counts how often each stands as a
fact's relation, its *head-participations*, and how often as an argument, its *arg-participations*. It reads the
facts held and their own arguments: an atom that stands only inside a nested fact, or as the relation of a fact
that is only pointed at, is in neither count. Its row `category = object` is the engine's word for an atom it
files under neither `relation` nor `rule`; in the reasoning's words every node is an object.

**A fact in a context, shown.** A program of one written fact, a rule over what it points at, and a question:

```lisp
(think Alice (= 5 (+ 2 2)))
(rule seen ()
  :match  (= ?a ?b)
  :assert (seen ?a)
  :why    "an equality is held")
(query :goal (= 5 ?x))
```

```sh
ein saturate FILE --dump     # holds (think Alice (= 5 (+ 2 2))); no fact of `=`, none of `seen`
ein solve FILE               # one state; the query's class reads `the goal has no row: undetermined here`
```

With the line `(= 5 (+ 2 2))` added the rule fires and the query answers `?x = (+ 2 2)`: the fact is then held,
and in the context of the `think` fact as before. The same file with `(think Alice (relation p A B))`,
`(think Alice (seen-by Bob))` beside a rule named `seen-by`, or `(think Alice (false))` loads, declares no `p`,
applies no rule and is alive.

**A declaration is a fact a rule's `:match` reads, and its labels are ordinary atoms**
([Q-M1f.372](../../../decisions/language/Q-M1f.372-ein-has-no-types-a-declarations-slot-names-are.md)). The rule
of [the rules § Relation-polymorphic](../../reasoning/01-static-ir/02-rewriting-rules.md#relation-polymorphic) that
gives a label a meaning, as a program. A variable in a relation's place must be a parameter, so the rule takes
`?R` and an activator names the relation:

```lisp
(relation is-a Thing Kind :why "{?3} is a {?4}")
(relation lives-in Person House :why "{?3} lives in {?4}")
(relation fits Thing Kind :why "{?3} fills a first place labelled {?4}")

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
| a rule unpacks a fact by spelling it, one rule a shape, and holds it; a rule packs a held fact under an arrow of another | the claims of [`examples/beliefs/01_common_belief.ein`](../../../../examples/beliefs/01_common_belief.ein) and [`02_packing.ein`](../../../../examples/beliefs/02_packing.ein), by `ein-cli/tests/corpus_cli.rs`: `every_cell_reproduces_its_exit_code` |
| a variable alone is no conclusion | `ein-cli/tests/cli_semantics.rs`: `a_conclusion_no_fact_can_hold_is_one_line_in_eins_spelling` |
| a rule for a context is the global rule wrapped, by hand or by a macro, and neither matches the other's facts | the claims of [`examples/beliefs/08_global_and_wrapped.ein`](../../../../examples/beliefs/08_global_and_wrapped.ein), by `ein-cli/tests/corpus_cli.rs`: `every_cell_reproduces_its_exit_code` |
| the context's relation may be a parameter; `(false)` concluded in a context leaves the state alive and is read by a rule; a rule over every context's `(false)`, a `forall`, is what refutes the state | the claims of [`09_false_in_a_context.ein`](../../../../examples/beliefs/09_false_in_a_context.ein), alive, and [`10_everybody_contradicts.ein`](../../../../examples/beliefs/10_everybody_contradicts.ein), dead, by the same test |
| the elimination of a double negation is a rule, one rule a count with the relation a parameter: the standard library's, `std.negation`, which a program activates, and a program's own where the double negation stands in a context | the claims of [`tests/stdlib/negation/01_eliminate.ein`](../../../../tests/stdlib/negation/01_eliminate.ein), each count, and of [`06_double_negation.ein`](../../../../examples/beliefs/06_double_negation.ein), globally and wrapped; `ein-cli/tests/negation_of_a_negation.rs`: `a_rule_matches_one_and_concludes_one` |
| `(not P)` in a premise matches a stored negation, not the absence of `P` | `ein-infer/tests/rule_semantics.rs`: `not_in_a_premise_is_stored_negation_not_negation_as_failure` |
| `(unknown P)` admits exactly the facts neither held nor denied | `ein-infer/tests/naf_semantics.rs`: `unknown_admits_exactly_the_undecided_pairs` |
| `eq` and `neq` are computed from the bindings, each admitting what the other rejects; a variable only a guard binds ranges over the program's names; a conclusion of either is checked and never stored | `ein-infer/tests/rule_semantics.rs`: `eq_admits_exactly_the_bindings_neq_rejects`, `a_guards_free_variable_ranges_over_the_programs_names`, `an_eq_or_neq_conclusion_is_checked_never_stored` |
| a `forall` over an empty domain holds | `ein-infer/tests/rule_semantics.rs`: `a_forall_over_an_empty_domain_is_vacuously_true` |
| a rule with parameters and no activator has no application, and an activator applies it to the facts of the relation it names and of no other | `ein-infer/tests/rule_semantics.rs`: `a_parametrised_rule_with_no_activator_is_dormant` |
| a rule with no parameters matches an empty pattern once, and once per activator with parameters | `ein-infer/tests/rule_semantics.rs`: `an_empty_match_fires_once_or_once_per_activator` |
| an activator gives a parameter a nested fact | `ein-infer/tests/compile_semantics.rs`: `a_value_parameter_an_activator_binds_to_a_nested_fact_is_bound` |
| a parameter in a relation's place bound to a fact, written or derivable, is refused at load | `ein-cli/tests/cli_semantics.rs`: `a_derived_activator_binding_a_relation_parameter_to_a_fact_is_refused_at_load`; `ein-infer/tests/compile_semantics.rs`: `a_relation_parameter_a_rule_binds_to_a_nested_fact_is_refused` |
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
`ein render constraints examples/zebra2.ein` draws the L2 facts of the saturated root, the facts about relations,
every fact with a relation of the program under an argument arrow (`ein_infer::slice::about_relations`), and `ein
render kb` the L0 and L1 with them.
