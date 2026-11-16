# Rules as data — what of Ein already describes itself in Ein

How much of Ein can be stated in Ein, with only the forms a program may already write, and reasoned about by Ein's
own rules? The constraint is the point: a self-description that needs a second language beside Ein is no longer Ein
describing itself, as a grammar of EBNF written in EBNF needs nothing but EBNF.

The answer is a line through the four levels a knowledge base is read at
([Rules as statements](../static-ir/rules-as-statements.md)): objects, facts and relations are data rules read and
write, and a rule is not. This page states what is on each side of the line.

## 1. The toolbox: declarators and facts

A program has two kinds of form. A **declarator** heads a form the loader reads as a declaration or an
instruction: `relation`, `rule`, `hrule`, `macro`, `import`, `query`, `solve` and `config`
([`06_reserved_names.md`](../../../kernel/ir/03-ein-lang/06_reserved_names.md) lists them). **Any other head is a
fact.** Each form takes a closed list of keywords, and a fact takes only `:source`, `:rule` and `:using`
([Q-M1e2.28](../../../decisions/language/Q-M1e2.28-each-form-takes-a-closed-list-of-keywords.md)); a keyword pair is
only ever a form's own clause, never part of a term
([Q-M1f.14](../../../decisions/engine/Q-M1f.14-a-keyword-pair-is-only-ever-a-forms-clause.md)).

So there is no schema sub-language to describe Ein with. A form such as `(level L0 :name objects :nodes atoms)` is
not read as a schema with slots: `level` is a fact's head, and `:name` is a keyword a fact does not take, refused at
the parse. To describe Ein in Ein there are exactly **the declarators** and **facts over relations a program
names**, and they are enough for three of the four levels.

## 2. The four levels, written in Ein

### Objects: the kinds are names in a membership relation

Ein has no form that declares an object. An object is any name that fills an argument, and *X is one of these* is
a fact of a membership relation the program declares like any other, `is-a` by convention
([Q23](../../../decisions/language/Q23-a-premise-types-a-variable-and-no-entity-has.md)):

```lisp
(relation is-a T T :why "{?1} is a {?2}")
(is-a Object   T)
(is-a Relation Object)     ;; a relation is an object
(is-a Fact     Object)     ;; a fact is an object
(is-a Rule     Object)     ;; a rule is an object
```

Ein's own kinds are names under a common top, as every node of the knowledge base is a vertex of one graph.

### The homoiconic line

`is-a` is a declared relation and not a reserved word, so a program may say that it is one:

```lisp
(is-a is-a Relation)       ;; the membership relation classifies itself
```

With `std.algebra`'s closure rules activated, `(includes is-a is-a*)` and `(transitive is-a*)`, the facts above
entail `(is-a* is-a Object)`: `includes` copies `(is-a is-a Relation)` and `(is-a Relation Object)` into `is-a*`,
and `transitive` joins them. The membership relation is, by its own rules, an object. That is the fixed point of
Ein's model, *an instance is an instance of instance*, reached with a declaration, three facts and two stdlib rules,
and it is inference about Ein performed by Ein.

### Facts, relations, properties and rules

| level | what it is | how a program writes it |
|---|---|---|
| a fact | a relation applied to objects | `(color-loc Red House-1)` |
| a relation | a name and its argument types | `(relation color-loc Color House :why "…")` |
| a relation's property | a fact naming the relation | `(bijective color-loc)`, `(symmetric next-to)` |
| a rule | a pattern, a conclusion and a reason | `(rule symmetric (?rel) :match (?rel ?a ?b) :assert (?rel ?b ?a) :why "…")` |

### A declaration is a fact a rule can read

The loader stores each declaration as facts of the kernel relation `relation`: the signature, `(relation R A B)`,
and the **membership fact** `(relation R)`, which a bare declaration already is. Matching is arity-coupled, so
`(relation ?R ?A ?B)` reads the binary declarations only, and `(relation ?R)` asks of any declared relation whether
it is one. A pattern over `relation` is an ordinary premise, so **a relation is data the rules read**.

A rule may conclude a fact of `relation` too, `(relation a)`; it is a fact like any other, and it declares nothing:
a program's relations are fixed when it is loaded.

## 3. What already describes itself

**Relation signatures are data rules consume.** Because a declaration is a fact, rules read the schema and act on
it: `std.bijection`'s type check reads each declaration's argument types, and `std.algebra`'s converse check reads
two signatures to refuse an ill-typed inverse (`converse-illtyped-dom`, `converse-illtyped-ran`). The engine reads
the same declaration only for its shape, how many arguments a relation takes; it never reads an atom as a type
([Q23](../../../decisions/language/Q23-a-premise-types-a-variable-and-no-entity-has.md); the signature's two
readings are [`01_grammar.md` § Relation declarator](../../../kernel/ir/03-ein-lang/01_grammar.md#relation-declarator)).

**The property algebra is Ein in Ein.** `symmetric`, `transitive`, `functional`, `injective`, `total`,
`surjective`, `bijective`, `converse`, `compose`, `meet`, `join`, `includes` and the rest are not primitives of the
engine. Each is a fact written beside a declaration, and its meaning is an ordinary rule of `std.algebra` or
`std.bijection` that the fact activates
([Q27](../../../decisions/language/Q27-a-relations-properties-are-facts-beside.md),
[Q16](../../../decisions/stdlib/Q16-general-rule-families-are-stdlib-modules.md)):

```lisp
(symmetric next-to)                     ;; a fact …
(rule symmetric (?rel)                  ;; … given its meaning by a rule
  :match  (?rel ?a ?b)
  :assert (?rel ?b ?a)
  :why    "{?rel} is symmetric: {?a} ↔ {?b}.")
```

`std.algebra` is the relation algebra, union, intersection, complement, composition, converse and identity, as
generic rules over the one matcher, and `std.bijection` adds closed-world inference over a bijection. Ein's type
system is the same: types are facts of a membership relation, and what a type check concludes, a rule concludes.
The one kernel counterpart is the mark `(__symmetric__ R)`, which closes a relation under the swap of its arguments
as the `symmetric` rule does, without the rule.

**The quantifier sugar is Ein in Ein.** `forall` and `unknown` are not forms of the kernel: they are macros of
`std.macro`, declared in Ein and expanded where a program calls them:

```lisp
(macro forall (?b ?G ?B) (absent (and ?G (absent ?B))) :why "…")
(macro unknown (?P) (and (absent ?P) (absent (not ?P))) :why "…")
```

## 4. Where it stops: a rule is not data

**Objects, facts and relations reify; rules do not.** A pattern can match a fact `(?R ?a ?b)`, a declaration
`(relation ?R ?A ?B)` and a property `(symmetric ?R)`, but not a rule: `rule` is a declarator, and a pattern
`(rule ?n)` is refused at the parse, so no pattern binds a rule. No conclusion declares a relation or a rule either:
a conclusion is a fact, and a declaration is a form the loader reads. The program's rules are therefore fixed when
it is loaded, and saturation and search change its facts and never its rules; nor does a rule bring in a name the
program does not have ([Q-M1e.21](../../../decisions/hypotheses/Q-M1e.21-a-rule-brings-in-no-name.md)).

That is why the four levels form a stack in which each level is read only by the levels above it
([Rules as statements](../static-ir/rules-as-statements.md)): the arrows *a rule reads a rule* and *a rule writes a
declaration* do not exist.

`relation` describes itself only in part. It is a reserved word a declarator never binds, so `(relation relation …)`
is refused at the parse, and the signature of `relation` is not something a program can declare; the membership
facts of §2 are its whole description in Ein.

What lies past this line, rules that match or generate rules, is a change to the language and not a property of
Ein as it is; [Analysis of rules](analysis-of-rules.md) states what Ein does compute about a rule set, and
[Transformations of a rule set](transformations.md) what it does to one.

## What holds it

| claim | test |
|---|---|
| any head outside the declarators is a fact, and a keyword a fact does not take is refused at the parse | `ein-ir/tests/ir_semantics.rs`: `any_head_outside_the_declarator_set_is_a_generic_fact` |
| no accepted program carries a keyword its form does not list | `ein-ir/tests/fuzz_properties.rs`: `no_accepted_program_carries_a_keyword_its_form_does_not_list` |
| a declaration is stored as its signature fact and its membership fact; a bare declaration is its membership fact | `ein-ir/src/from_ir.rs`: `a_declaration_stores_itself_as_facts_so_rules_can_read_signatures` |
| a declarator binds no reserved word | `ein-ir/tests/kb_semantics.rs`: `the_reserved_name_guard_is_on_declarators_and_matches_whole_names`; `ein-ir/tests/imports_semantics.rs`: `every_ein_core_reserved_name_is_unbindable_through_a_qualified_import` |
| every rule of `std.algebra` and `std.bijection`, the property rules among them, is activated by a program of the stdlib suite | `ein-infer/tests/stdlib_coverage.rs`; the programs under `tests/stdlib/algebra/` and `tests/stdlib/bijection/` |
| `forall` and `unknown` are `std.macro`'s macros, read from the module | `ein-ir/tests/imports_semantics.rs`: `stdlib_macro_names_reads_the_module_rather_than_a_hardcoded_list`; `tests/stdlib/macro/01_forall.ein`, `02_unknown.ein` |
| no run reaches a name the program does not have | `ein-infer/tests/alive_invariant.rs`: `no_run_reaches_a_name_the_program_does_not_have` |
