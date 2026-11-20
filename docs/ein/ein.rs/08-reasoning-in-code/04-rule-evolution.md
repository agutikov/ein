# Rule evolution in code

Where `ein.rs` implements the three pages of [`reasoning/03-rule-evolution/`](../../reasoning/03-rule-evolution/),
the tests that hold it to them, and the commands that show each notion. The layout is the folder's
([README](README.md#how-a-page-is-laid-out)).

- [Rules as data](#rules-as-data)
- [Analysis of rules](#analysis-of-rules)
- [Transformations of a rule set](#transformations-of-a-rule-set)

## Rules as data

The page: [rules as data](../../reasoning/03-rule-evolution/01-rules-as-data.md).

### Where it lives

| notion | code |
|---|---|
| a form is a declaration or a fact, by its head | the grammar, `ein-ir/src/ein.y`: one production per declarator, and every other head a fact; the declarators are reserved words of the lexer, `ein-ir/src/ein.l` |
| each form's closed list of keywords | the grammar's productions; an unknown keyword is a token no production uses, so it is a parse error ([`03-design/04-ir-frontend.md` § 6](../03-design/04-ir-frontend.md#6-parse-errors)) |
| a declaration held as facts of `relation` | `ingest_relation` in `ein-ir/src/from_ir.rs` |
| a pattern over `relation` is an ordinary premise | no special case: the compiler and the matcher read a `relation` row as any row (`ein-infer/src/compile.rs`, `ein-infer/src/match_.rs`) |
| the property algebra | rules of `stdlib/algebra.ein` and `stdlib/bijection.ein`, each applied by the fact that names it; the modules are the user's [standard library reference](../../user/04-reference/README.md) |
| no property has a kernel counterpart: symmetry is a rule | `std.algebra`'s `symmetric`; the kernel closes no relation under the swap, `ein-infer/tests/stdlib_semantics.rs`' `the_kernel_mirrors_no_relation_and_symmetric_is_a_rule`, and reads no theory's name ([`07-reserved-engine-strings.md` § No theory's name](../07-reserved-engine-strings.md#no-theorys-name)) |
| `forall` and `unknown` | macros of `stdlib/macro.ein`, read from the module by `stdlib_macro_names` in `ein-ir/src/imports.rs` and expanded by `expand_macros` in `ein-ir/src/macros.rs` |
| a rule is not data | the grammar: `rule` is a reserved word, never a name, so no pattern is headed by it; a conclusion is facts (`ein_ir::Ast::conjuncts`), and the program's registries (`ein-core/src/program.rs`) are written by the loader alone |
| no declaration binds a reserved word | the guard on declarators in `ein-ir/src/from_ir.rs`, over `ein_core::terms::RESERVED` |

### What holds it

| claim | test |
|---|---|
| any head outside the declarators is a fact, and a keyword a fact does not take is refused at the parse | `ein-ir/tests/ir_semantics.rs`: `any_head_outside_the_declarator_set_is_a_generic_fact` |
| no accepted program carries a keyword its form does not list | `ein-ir/tests/fuzz_properties.rs`: `no_accepted_program_carries_a_keyword_its_form_does_not_list` |
| a declaration is stored as the fact with its labels and its membership fact; a bare declaration is its membership fact | the loader's unit test `a_declaration_stores_itself_as_facts_so_rules_can_read_signatures` in `ein-ir/src/from_ir.rs` |
| a declarator binds no reserved word | `ein-ir/tests/kb_semantics.rs`: `the_reserved_name_guard_is_on_declarators_and_matches_whole_names`; `ein-ir/tests/imports_semantics.rs`: `every_ein_core_reserved_name_is_unbindable_through_a_qualified_import` |
| every rule of `std.algebra` and `std.bijection`, the property rules among them, is activated by a program of the stdlib suite | `ein-infer/tests/stdlib_coverage.rs`: `every_stdlib_rule_is_activated_by_a_program`; the programs under `tests/stdlib/algebra/` and `tests/stdlib/bijection/` |
| `forall` and `unknown` are `std.macro`'s macros, read from the module | `ein-ir/tests/imports_semantics.rs`: `stdlib_macro_names_reads_the_module_rather_than_a_hardcoded_list`; `tests/stdlib/macro/01_forall.ein`, `02_unknown.ein` |
| no run reaches a name the program does not have | `ein-infer/tests/alive_invariant.rs`: `no_run_reaches_a_name_the_program_does_not_have` |

### Commands that show it

**The homoiconic line.** The page's derivation, as a program. Save it as `kinds.ein`:

```lisp
(import std.algebra :symbols (includes transitive))
(relation is-a T T :why "{?3} is a {?4}")
(relation is-a* T T :why "{?3} is, at some remove, a {?4}")
(is-a Object   T)
(is-a Relation Object)     ;; a relation is an object
(is-a Fact     Object)     ;; a fact is an object
(is-a Rule     Object)     ;; a rule is an object
(is-a is-a Relation)       ;; the membership relation classifies itself
(includes is-a is-a*)
(transitive is-a*)
```

```sh
ein saturate kinds.ein --dump     # the derived facts, each with the rule that derived it
ein render kb --fold kinds.ein    # the drawing on the page, each fact of three arrows folded
```

The dump's `DERIVED` section holds more than the two rules derive: what the imported module's own rules derive, and
root's `open` facts, the blind enumerator's hypotheses, as part of the state
([Q-M1f.572](../../../decisions/read-out/Q-M1f.572-a-states-open-facts-are-part-of-it-listed-drawn.md)). The facts the
page derives are the two rules':

```sh
ein saturate kinds.ein --dump | grep -E ':rule (includes|transitive)\)'
```

<!-- output -->
```text
  (is-a* Object T :rule includes)
  (is-a* Relation Object :rule includes)
  (is-a* Fact Object :rule includes)
  (is-a* Rule Object :rule includes)
  (is-a* is-a Relation :rule includes)
  (is-a* Relation T :rule transitive)
  (is-a* Fact T :rule transitive)
  (is-a* Rule T :rule transitive)
  (is-a* is-a Object :rule transitive)
  (is-a* is-a T :rule transitive)
```

`(is-a* is-a Object :rule transitive)` is the page's conclusion.

**The quantifier macros**, as `stdlib/macro.ein` declares them:

```lisp
(macro forall (?b ?G ?B)
  (absent (and ?G (absent ?B)))
  :why "every {?b} the guard {?G} holds of satisfies {?B}: none satisfies the guard and not the body")

(macro unknown (?P)
  (and (absent ?P) (absent (not ?P)))
  :why "{?P} is unknown: neither it nor its negation holds")
```

**Where it stops.** Three probes, each a file of a few lines:

| the file writes | what `ein` answers |
|---|---|
| a rule whose `:match` is `(rule ?n)` | a parse error at the word: ``unexpected `rule` `` |
| `(relation relation T :why "…")` | a parse error at the second word: ``unexpected `relation` `` |
| a rule that concludes `(relation ?x)` from `(p ?x)`, beside `(p a)` and a second rule that reads `(relation ?R)` | it loads; `ein saturate FILE --dump` lists `(relation a :rule …)` among the derived facts, the second rule fires on `a` as on the declared relations, and the dump's `;; Relations — declared` list does not grow |

## Analysis of rules

The page: [analysis of rules](../../reasoning/03-rule-evolution/02-analysis-of-rules.md).

### Where it lives

| notion | code |
|---|---|
| the dependency graph, over the compiled plans of root | `ein_infer::strata::edges`: an edge from each relation a plan reads facts of to each it concludes facts of, negative at an odd `absent` depth, and a positive one from the relation of a rule's activators; the reads and writes of a plan are `ein-infer/src/monotone.rs`' `reads_of` and `writes_of` |
| the strata, and each rule's stratum | `ein_infer::strata::graph` and `rule_strata` |
| a negative cycle and its parity | `ein_infer::strata::components`, `Cycle`, and `Cycle::stratum_text`, the diagnostic's wording |
| the diagnostic on a run | the field `negative_cycles` of `ein_infer::monotone::Monotonicity`; the read-out's rows in `ein-render/src/read_out.rs`; the dump's line in `ein-cli/src/saturate.rs` |
| the monotonicity check's cycles: the same graph restricted to the relations whose held extent ([the knowledge base](../../reasoning/01-static-ir/01-knowledge-base.md#the-relation-its-extent-a-fact-of-it)) a commitment can change | `ein-infer/src/detector.rs`, reading `strata::edges` |
| the order of the agenda, which no stratum sets | `ein_infer::saturator::AGENDA` ([`03-design/06-saturation.md`](../03-design/06-saturation.md)) |

The graph is over the **compile cache**, not the source: the conclusion of a relation-polymorphic rule names no
relation until an activator binds its parameter, so its edges exist only per plan, and the graph is asked of a
run that has saturated root.

### What holds it

| claim | test |
|---|---|
| a read under an `absent` puts what the rule concludes one stratum higher; a rule that reads under an `absent` facts of the relation it concludes a fact of is an odd negative cycle; each rule's stratum is the highest of what it concludes | `ein-infer/tests/strata.rs`: `an_absent_read_is_a_stratum_and_a_self_read_is_a_negative_cycle` |
| `examples/features/07_unstratifiable.ein` has a negative cycle, and `examples/features/01_not_and_absent.ein`, an `absent` over the held extent of a relation no rule concludes a fact of, has none | `ein-infer/tests/strata.rs`: `the_unstratifiable_fixture_has_a_negative_cycle_and_its_sibling_none` |
| `ein solve` names each negative cycle and its parity as a possibility, still answers and exits 0; a cycle that never fires is reported in the same words; a stratified program prints no `strata` row | `ein-cli/tests/strata_diagnostic.rs`: `solve_names_each_negative_cycle_and_still_answers` |
| `ein saturate --dump` says whether a stratification exists | `ein-cli/tests/strata_diagnostic.rs`: `saturate_dump_says_whether_a_stratification_exists` |
| the engine answers a program with an odd cycle, with a fixpoint supported at the boundary | `ein-infer/tests/naf_semantics.rs`: `an_unstratifiable_loop_converges_to_a_supported_model` |
| the agenda's order and the written order alone give every question and every claim the same answer on every monotone program of the corpus | `ein-infer/tests/agenda.rs`: `every_question_and_claim_answers_alike_under_the_agenda_and_the_written_order` |

### Commands that show it

**The page's example.** Save it as `strata.ein`:

```lisp
(relation p T :why "{?3} is p")
(relation q T :why "{?3} is q")
(relation r T :why "{?3} is r")
(relation s T :why "{?3} is s")
(p a)
(rule one () :match (p ?x) :assert (s ?x) :why "{?x} is p, so it is s")
(rule g ()   :match (and (s ?x) (absent (q ?x))) :assert (r ?x) :why "{?x} is s and not known q, so it is r")
```

```sh
ein saturate strata.ein --dump
```

<!-- output -->
```text
;; Rules — the saturation agenda's (3)
;;   one  :params ()  stratum 0
;;   g  :params ()  stratum 1
;;   std.core.inconsistent  :params ()  :ein inconsistent

;; Strata — stratified, 2 strata
```

#### Where the diagnostic is shown

- **`ein solve`** prints a `strata` row under `lattice` only on a program with no stratification: how many cycles
  pass through an `absent`, and that root's fixpoint may depend on the boundary's admission order and need not be
  unique. A `negative cycle` row under it names each cycle: its relations, the rules on its negative edges, and
  what its parity allows. (`cycle`, without *negative*, is the monotonicity check's word for its own cycles.) The
  rows are [`08-read-out.md`](../../user/04-reference/08-read-out.md)'s.
- **`ein saturate --dump`** prints the stratum of each rule of the saturation agenda beside it, and one
  `;; Strata` line: *stratified, k strata*, or *not stratifiable* with the same cycles.
- A stratified program prints no `strata` row.

The even cycle of the page is [`examples/features/07_unstratifiable.ein`](../../../../examples/features/07_unstratifiable.ein):

```sh
ein solve examples/features/07_unstratifiable.ein
```

<!-- output -->
```text
  strata            not stratifiable: 1 cycle through an absent — root's fixpoint may depend on the boundary's admission order, and need not be unique
    negative cycle  p, q — derive-p: q →¬ p · derive-q: p →¬ q — even: several fixpoints may exist, and which one root reaches may depend on the order the boundary admits in
```

A cycle that never fires is reported in the same words.
[`examples/syntax/rule-guards.ein`](../../../../examples/syntax/rule-guards.ein) writes no fact, so its rule
`g`, which reads facts of `s` under an `absent` and concludes a fact of `s`, never fires, and its one fixpoint
is root's empty one:

```sh
ein solve examples/syntax/rule-guards.ein
```

<!-- output -->
```text
  strata            not stratifiable: 1 cycle through an absent — root's fixpoint may depend on the boundary's admission order, and need not be unique
    negative cycle  s — g: s →¬ s — odd: a fixpoint supported at the boundary need not exist, and the one root reaches may depend on the boundary's order
```

## Transformations of a rule set

The page: [transformations of a rule set](../../reasoning/03-rule-evolution/03-transformations.md).

### Where it lives

| the map of the page | code |
|---|---|
| resolve | `resolve_imports` in `ein-ir/src/imports.rs`: every module read once, each declared name under its full name, a file's short names by `short_names` ([`03-design/04-ir-frontend.md` § 8](../03-design/04-ir-frontend.md#8-import-resolution)) |
| expand | `expand_macros` and `expand_rule_clauses` in `ein-ir/src/macros.rs`, a rewrite of the AST before the load; what an expansion writes is held to the grammar's shapes by `refuse_malformed_primitives` in `ein-ir/src/from_ir.rs` ([`03-design/04-ir-frontend.md` § 7](../03-design/04-ir-frontend.md#7-macro-expansion)) |
| flatten | `ein_ir::Ast::conjuncts`, which every reader of a conclusion calls |
| instantiate | `ein_infer::compile::compile_rule`, once per activator, cached under `PlanKey`; `ein_infer::Engine::compile_all` for the activators a state holds, and a plan for an activator that arrives during a saturation ([`03-design/05-matcher.md`](../03-design/05-matcher.md)) |
| the order of firing | `ein_infer::saturator::AGENDA` |

The engine applies no other rewrite to a rule set. The levers that change how a run computes, and never what, are
[`02-algorithms.md`](../02-algorithms.md)'s.

### What holds it

| claim | test |
|---|---|
| every corpus program answers alike under every import form | `ein-cli/tests/import_invariance.rs`: `every_corpus_program_answers_alike_under_every_import_form` |
| `send_more_money_written.ein` is `send_more_money.ein` with one line of eight facts reordered, and `--single` and the maximal line do the same work and list the same states on both | `ein-cli/tests/cli_semantics.rs`: `the_written_send_is_the_by_column_send_with_one_line_reordered`; `ein-infer/tests/order.rs` |
| the agenda and the written order alone answer every question and every claim alike on every monotone program of the corpus | `ein-infer/tests/agenda.rs`: `every_question_and_claim_answers_alike_under_the_agenda_and_the_written_order` |
| a rule with parameters gets an instance for an activator derived at run time, on a fork | `ein-infer/tests/explain_semantics.rs`: `a_runtime_derived_activator_gets_a_plan_on_a_fork` |
| a macro's body may call another macro, and a parameter in head position is substituted | `ein-ir/tests/ir_semantics.rs`: `a_macro_body_may_invoke_another_macro`, `a_macro_parameter_in_head_position_is_substituted` |

### Commands that show it

```sh
cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-cli --test import_invariance   # Proposition 1, over every corpus program that imports
cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-infer --test agenda            # Proposition 3, over every monotone corpus program
ein solve examples/beyond_arith/send_more_money.ein --single                                      # Proposition 2 on one pair: this program,
ein solve examples/beyond_arith/send_more_money_written.ein --single                              # and the same with its letters declared in another order
```

The second program of the pair is generated from the first, and a test holds it to being the first with one line
of facts reordered.
