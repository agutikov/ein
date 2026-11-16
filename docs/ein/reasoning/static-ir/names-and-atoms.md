# Names and atoms

Everything a program is about is named. A **name** (an *atom*) is the token written in the program; the node of the
knowledge base it denotes is the thing named. **Identity is by name**: two occurrences of one name anywhere in one
program denote one node, and there is no second, hidden identity behind the name. This page states what a name is,
what kinds of thing have one, and what makes two of them the same thing.

## What a name is

A name is a run of the name characters: ASCII letters and digits, the connectors `_`, `-` and `.`, and the
decorators the grammar lists. Digits are name characters like any other, so **Ein has no numbers, no ranges and no
integer kind**: `1`, `01` and `-7` are three names, `1..5` is the name it spells, and a name of digits is compared,
ordered and proposed as any other name is
([Q-M1e2.24](../../../decisions/language/Q-M1e2.24-ein-has-no-numbers-and-no-ranges.md)). Arithmetic is a theory of
rules over numerals written in such names, never an operation of the engine.

A **string** is natural-language text, written only as the value of `:why`, `:source` or `:goal-text`; it is never
a value and never a name ([Q-M1e2.23](../../../decisions/language/Q-M1e2.23-a-string-is-natural-language-text-never-a-value.md)).

Two families of name are not the program's to spell:

- **A name the loader makes up** for something a program writes no name for, the empty form `()`, a nested form
  whose head is not a name, a query's goal, starts with `#`, which is no name character, so no name a program
  writes can collide with one ([Q-M1f.339](../../../decisions/language/Q-M1f.339-every-name-the-loader-makes-up-starts-with.md)).
- **The kernel's two markers**, `__closed__` and `__symmetric__`, are names a program may use as a fact's relation
  or as a value, and may never bind: no declaration, rule, hrule, macro or import alias takes one
  ([Q-M1f.7](../../../decisions/language/Q-M1f.7-the-dunders-are-never-bound-and-the-writeback.md)).

**A name has one full name.** A name a module declares, a relation, a rule, an hrule or a macro, is the module's
name, a dot and the declared name, `std.math.decimal-arith.add`. The short name a file writes, `add`, is an alias
local to that file. Identity is by full name: two modules that each declare `add` declare two names, and one module
reached through several imports is one copy with one set of full names
([Q-M1f.231](../../../decisions/imports/Q-M1f.231-every-name-a-module-declares-has-a-full-name.md)).

## The kinds of named thing

Every node of the knowledge base is one of five kinds: an object, a type, a relation, a rule, or a fact
([Q1](../../../decisions/language/Q1-the-knowledge-base-is-a-typed-labelled.md)). The first four are named; a fact is
not, and is identified by what it says.

| kind | what it is | identified by |
|---|---|---|
| **object** | a thing the puzzle is about: `Norwegian`, `House-1`, `Red` | its name |
| **type** | a name an object's membership fact points at: `House`, `Color` | its name; a type is an object like any other |
| **relation** | what a fact says of its arguments: `co-located`, `is-a` | its full name |
| **rule** | a statement that derives facts from facts ([rules as statements](rules-as-statements.md)) | its full name |
| **fact** | a relation applied to arguments, `(co-located Norwegian House-1)` | its relation and its arguments, in order |

**Ein imposes no type system.** No variable, relation, rule or fact carries a type slot. An object's type is a fact
of the program's own membership relation, `(is-a House-1 House)`, and a variable is typed by the premises that bind
it ([Q23](../../../decisions/language/Q23-a-premise-types-a-variable-and-no-entity-has.md)). A relation's declaration
names a type for each argument, its *signature*, and those names are read in two ways: a rule reads them as types,
through the declaration's stored fact (below), and the loader reads only how many there are. A form of a declared
relation has as many arguments as its declaration has types, and nothing checks what is written in each place
([Q-M1f.315](../../../decisions/language/Q-M1f.315-a-form-of-a-declared-relation-has-as-many.md)). A declaration with
no type, `(relation flag)`, declares a relation of no argument
([Q-M1f.201](../../../decisions/language/Q-M1f.201-a-bare-relation-r-declares-no-argument.md)).

A name is declared once. A program that declares one relation name twice, or one rule or hrule name twice, is
refused at load.

## A relation is an object

Relations are nodes of the graph, not labels on its edges, and they take part in facts as themselves:

- **A relation's properties are facts beside it.** `(symmetric next-to)` and `(includes right-of next-to)` are facts
  whose arguments are relation names; each activates a rule that reads them
  ([Q27](../../../decisions/language/Q27-a-relations-properties-are-facts-beside.md)).
- **A declaration is stored as facts.** `(relation co-located Person House)` is stored as the fact
  `(relation co-located Person House)` and as `(relation co-located)`, so a rule reads a signature with
  `(relation ?R ?A ?B)` and asks *is R a relation* with `(relation ?R)`, whatever R's arity. A declaration with no
  type stores the one fact `(relation flag)`, which is both.
- **A relation nothing declares is still a relation.** A fact whose head no declaration names makes that head a
  relation, with no declared count; a rule may read a relation that nothing in the program makes, and then never
  fires ([Q-M1f.120](../../../decisions/language/Q-M1f.120-a-rule-may-read-a-relation-nothing-makes.md)).
- **What a file writes is stated as facts.** For each fact nested in a fact a file writes, at any depth, and each
  nested fact a rule's conclusion writes with no variable in it, the loader states `(written T)`, derived from the
  fact it was read in. No program writes `written`; a module that defines a representation reads it to check its
  own terms where they are written
  ([Q-M1f.275](../../../decisions/language/Q-M1f.275-written-states-every-fact-a-file-nests.md),
  [Q-M1f.280](../../../decisions/language/Q-M1f.280-a-program-does-not-write-written.md),
  [Q-M1f.272](../../../decisions/language/Q-M1f.272-a-representation-module-reads-what-a-file-writes.md)).

There is one place where a relation's name is set apart from the other names, and it is a device of tractability,
not a layer of the semantics: the blind enumerator, which proposes facts about the program's objects when nothing
better proposes hypotheses, leaves out the names of the program's relations and rules, the names a signature
declares as types, the reserved words and the kernel's markers. A relation's name that only a rule's conclusion
heads is not declared, and stays an object
([Q-M1f.306](../../../decisions/hypotheses/Q-M1f.306-the-blind-enumerators-objects-are-the-programs.md),
[Q-M1e.12](../../../decisions/hypotheses/Q-M1e.12-the-blind-enumerator-is-untyped-and-a-type-it.md); the ladder
is [hypothesis generation](../kb-evolution/hypothesis-generation.md)). Nothing stops a rule from concluding a fact
about a relation, and nothing reads such a fact differently from any other.

## An argument is a name or a fact

An argument of a fact is a **name** or a **fact**: `(hypothesis (co-located Norwegian House-2))` is a fact of
`hypothesis` whose one argument is the fact `(co-located Norwegian House-2)`. The nested fact is an ordinary value,
identified by its structure at every depth, and a pattern matches into it: `(hypothesis (co-located ?a ?b))` binds
`?a` and `?b` ([Q40](../../../decisions/language/Q40-a-fact-may-hold-a-fact-as-an-argument.md)).

**A value is a name, or a fact a state could hold**: a fact of an ordinary relation, or the stored negation of one.
A fact inside a fact, a pattern inside a pattern, and a conclusion's or a guard's argument are values, so `eq`,
`neq`, `absent`, `and`, `or` and `false` with an argument never head a nested form, and `(not N)` of a name `N` is
the negation of no fact
([Q-M1f.113](../../../decisions/language/Q-M1f.113-a-value-is-a-name-or-a-fact-a-state-could-hold.md)). An
equality is computed, a negation as failure is a test of a state, and a connective joins premises: none is a fact
any state holds, so none can be a value.

## The program's names

**The names of a program** are the constants written in an argument position of its facts, at any depth, and the
constants written anywhere in its rules, premises and conclusions alike. A constant that only a query writes is not
one: a query asks about the answer and does not change it
([Q-M1e2.9](../../../decisions/language/Q-M1e2.9-a-programs-names-are-the-constants-its-facts.md)). **A rule
brings in no name**: the names are fixed when the program is read, before anything is derived, so no state any
search reaches holds a name the program lacks
([Q-M1e.21](../../../decisions/hypotheses/Q-M1e.21-a-rule-brings-in-no-name.md)). This is the alive-set invariant,
and [saturation](../kb-evolution/saturation.md) states what it buys.

## Identity, stated once

```text
object, type     its name
relation, rule   its full name
fact             (relation, arguments), each argument compared the same way, at any depth
```

What is **never** part of identity: where a thing is written, how a fact came to be held (its provenance), its
`:why` text, and the knowledge base that holds it. Two facts with one relation and one argument list are one fact,
however many ways it was stated or derived ([relations and facts](relations-and-facts.md)); and a fact held at root
and the same fact held in a fork of root are one fact. Identity is equality; the total order over values, which
every sort a reader can observe uses, is stated in [values and their order](values-and-order.md).

**Where a fact came from is not what it is.** Reading a fact's provenance sorts the facts into the *given* (a fact
the program writes with a `:source`), the *background* (one written with none, a declaration's mirror) and the
*derived* (a rule's conclusion, a commitment). The partition is read, never stored, and only for presentation:
every rule, every check and every contradiction treats a fact alike whatever its origin, and `(not X)` contradicts
`X` wherever either came from.

## What holds it

| claim | test |
|---|---|
| a run of digits is a name, as written, in every place a name is written; `1` and `01` are two names | `ein-cli/tests/cli_semantics.rs`: `a_run_of_digits_is_a_name_in_every_place`, `a_range_is_the_name_it_spells`; `ein-core/tests/values_semantics.rs`: `a_run_of_digits_is_a_name_as_written` |
| no name the loader makes up is a name a program can write | `ein-ir/tests/generated_parser.rs`: `no_name_the_loader_makes_up_is_a_name` |
| a kernel marker is refused wherever a program binds it | `ein-ir/tests/engine_names.rs`: `a_dunder_is_refused_wherever_a_program_binds_it` |
| one module reached through several imports is one copy; a short name is local to its file | `ein-ir/tests/imports_full_names.rs`: `the_users_dependency_graph`, `a_short_name_is_local_to_its_file`; `ein-ir/tests/imports_semantics.rs`: `one_module_under_two_spellings_is_one_copy` |
| a form of a declared relation at another count is refused, and the signature is no type check | `ein-cli/tests/declared_counts.rs`: `a_form_of_another_count_is_refused_on_every_command`, `what_is_not_a_declared_relations_form_loads` |
| a relation name, a rule name or an hrule name declared twice is refused at load | `ein-ir/tests/load_semantics.rs`: `require_why_refuses_each_form_without_one`, and `the_load_negative_fixtures_are_byte_identical` over `examples/broken/load/rule_duplicate_name.ein` and `hrule_duplicate_name.ein` |
| a declaration is stored as its signature fact and its arity-1 fact; a bare one as one fact | `ein-infer/tests/compile_semantics.rs`: `a_bare_declaration_is_declared_with_an_empty_signature_and_stores_one_fact`; the loader's unit test `a_declaration_stores_itself_as_facts_so_rules_can_read_signatures` in `ein-ir/src/from_ir.rs` |
| an undeclared head is a relation | the loader's unit test `an_undeclared_head_vivifies_a_relation_but_a_predicate_does_not` in `ein-ir/src/from_ir.rs` |
| the loader states `written` for what a file nests and a conclusion's ground nested facts, and a program may not write it | `ein-cli/tests/written_facts.rs`: `what_the_loader_states`, `a_program_does_not_write_written` |
| a pattern matches into a nested fact, and a nested fact pairs with its negation by structure | `ein-infer/tests/rule_semantics.rs`: `a_nested_pattern_binds_inside_the_nested_fact`; `ein-infer/tests/search_semantics.rs`: `a_nested_positive_pairs_through_structural_identity` |
| a form that is no value is refused, and every form a state could hold loads | `ein-cli/tests/value_rule.rs`: `a_form_that_is_no_value_is_refused_on_every_command`, `what_a_state_could_hold_loads` |
| every name of the program is a candidate object, and no run reaches a name the program does not have | `ein-infer/tests/alive_invariant.rs`: `every_name_of_the_program_is_a_candidate_object`, `no_run_reaches_a_name_the_program_does_not_have` |
| a contradiction is a pair whatever either half's origin | `ein-infer/tests/search_semantics.rs`: `mutual_negation_is_detected_whatever_either_sides_origin` |
