# Values and their order

Where what a run reports depends on an order, the order is stated here, as a function of the values themselves, so
that two implementations given one program report the same thing in the same order. Each section states an order or
an identity a reader can observe, and why it is fixed rather than left to the implementation.

## No output depends on how names were numbered

An implementation may number names and facts as it reads them, and may keep them in hashed collections. **None of
that reaches an answer or a printed line.** Every order a reader sees is an order of the values' content: the
answer, every rendering of it, every narration, every core a run reports, and every dump are the same whatever
order names were first met in, whatever numbers they were given, and however many workers shared the run. Where a
collection's iteration order could reach an output, the collection is sorted by content at that point; a
collection read only for membership needs no order.

## Values are totally ordered

A value is a name or a fact ([names and atoms](names-and-atoms.md#an-argument-is-a-name-or-a-fact)), and any two
values compare:

- **a name before a fact**;
- **two names by their text**, character by character in code-point order. Names are ASCII, so this is ASCII
  order: digits before upper case before lower case. A name of digits is text like any other, so `10` sorts before
  `2` and `(p 10)` before `(p 2)`;
- **two facts by their relation's name first, then their arguments pairwise in this same order, then their
  arity**, so a fact whose argument list is a prefix of another's sorts first: `(adjacent b b)`, `(co-located a)`,
  `(co-located a a)`, `(co-located a b)`.

It is the order a run uses where it compares values as values: the facts the listed states differ in, which the
`--models key` and `--models grid` forms tabulate, and the values a static check's findings are listed by. It is
stated rather than inherited because two values of different kinds, a name and a nested fact, can meet in one
argument position of one relation, and the order must say which comes first.

**The candidates of H are ordered by their text**: each candidate written as Ein text, `(relation arg …)` with a
nested fact written the same way inside it, and the texts compared character by character. That is the universe's
order, in which [`ein show hyps`](../kb-evolution/hypothesis-space.md) lists H.

## A state is its facts

**Two commitments whose saturated states hold the same facts reach one state.** The identity of a state is its set
of facts, compared as a set, element by element: never a hash or digest of it. A digest may label a state for a
reader, and is never what decides that two states are one, so two different states can never be counted as one
because their digests collide. What a state's identity leaves out is everything a fact's identity leaves out: how
each fact came to be held, and in which order. So the answer's two counts differ exactly where saturation derives a
candidate: two commitments that differ only in a candidate one of them derives are one state
([solution semantics](../kb-evolution/solution-semantics.md)).

## A rule application's identity

A rule with parameters is applied once for each **activator**, a fact whose relation is the rule's name and whose
arguments bind its parameters: `(transitive co-located)` applies `transitive` with `?R` bound to `co-located`. A
rule with no parameters has one application. A **firing** is one application matched once, and its identity is
**its rule, its activator, and the values its variables are bound to**
([Q-M1e.16](../../../decisions/engine/Q-M1e.16-a-firings-identity-keys-on-every-activator.md)). One identity fires
at most once.

**The activator half is every argument of the activator fact, as values**, names and nested facts alike
([Q-M1f.43](../../../decisions/engine/Q-M1f.43-a-plans-activator-is-every-argument.md)). So:

- two activators of one rule that differ in **any** argument are two applications, and each fires; `1` and `01`
  are two names, and `(note edge 1)` and `(note edge 01)` are two applications;
- an activator binds each parameter to its argument **whatever value it is**: `(act (f a))` binds the parameter to
  the fact `(f a)`, and a premise `(p ?y)` under it matches `(p (f a))` alone;
- **the order activators are written in decides nothing**: adding an activator never removes a conclusion, and
  exchanging two lines of a program derives the same facts.

An identity that kept only some of an activator's arguments would merge two applications that bind their
variables in different places, and would drop a valid firing as a repeat of another, depending on which activator
was written first. The program below is the shape where that would show:

```lisp
(relation edge  Node Node :why "there is an edge from {?1} to {?2}")
(relation holds Node :why "{?1} holds")
(relation noted Node Node :why "{?1} is noted with {?2}")

(rule note (?R ?f)
  :match  (and (?R ?a ?b) (holds ?f))
  :assert (noted ?a ?f)
  :why    "{?R} runs from {?a} to some node and {?f} holds, so {?a} is noted with {?f}")

(edge 1 2) (edge 2 3)
(holds 1)  (holds 3)

(note edge 1)
(note edge (src Y))
```

Its root holds `(noted 1 1)` and `(noted 2 1)`, from `(note edge 1)`, which binds `?f` to `1`. `(note edge (src
Y))` binds `?f` to the fact `(src Y)`, which nothing `holds`, so it derives nothing; and it removes nothing the
first activator derives. Written in either order, the program derives the same two facts
(`examples/ein-bugs/binding-key-both.ein` and its swapped twin, with one file for each activator alone).

## How a value is printed

**On every surface that prints a value**, the read-out, a dump, a trace, an event, a drawing's label, a value is
printed as Ein text that reads back as the same value: a name bare, a nested fact as `(relation arg …)`. A name a
program writes always lexes as a name, so it always prints bare; a name the loader made up, which starts with `#`,
prints in `"` and does not read back, which is how a reader tells it apart. Every state an answer's files hold
reads back as the state it was written from.

**A drawing of the program labels a value as written.** No keyword pair reaches a value's label, since a value holds
none, and no drawing refuses or aborts on a program the parser accepts.

**A message that quotes a value** (a diagnostic naming a rule, a static check's warning) quotes it in one fixed form,
three shapes:

- a **name or a text** is quoted in `'`, unless it contains a `'` and no `"`, in which case in `"`. The chosen quote
  and `\` are escaped with a `\`; tab, newline and carriage return are `\t`, `\n` and `\r`; any other control
  character and DEL is `\xNN`; any other character that is not printable is `\xNN` below U+0100, `\uNNNN` below
  U+10000 and `\UNNNNNNNN` above, in lower-case hex; every other character is itself. *Not printable* is by Unicode
  general category, `Cc` `Cf` `Cs` `Co` `Cn` `Zl` `Zp` `Zs`, the ASCII space excepted;
- a **list** is `(a, b)`, a list of one `(a,)` and of none `()`;
- a **fact** is `Fact(relation_name='p', args=('A',))`, its arguments a list, a nested fact the same way.

Names are ASCII, so a name never needs more than the first two escapes; a text, which may hold any Unicode, can.

The quoted form is total over every value, so it serves as an order too, the texts compared character by
character: the facts of a contradiction's core, drawn as the edges into `⊥` in a refutation's drawing, which a
trace embeds, are listed in the order of their quoted forms, whatever order they were found in.

## What holds it

| claim | test |
|---|---|
| no rendering of any corpus program depends on the order names were numbered in | `ein-render/tests/id_order_invariance.rs`: `every_rendering_reproduces_its_digest_and_none_depends_on_the_id_order`, `an_id_order_that_reaches_an_output_is_caught` |
| no rendering depends on how many workers ran | `ein-render/tests/jobs_invariance.rs`: `jobs_does_not_move_any_observable` |
| H is listed in the universe's order, as the binary lists it | `ein-cli/tests/show_hyps.rs`, which holds the listing on the hypothesis space's page to the binary's |
| names order by code point, whatever order they were read in | `ein-core/tests/values_semantics.rs`: `the_rank_table_orders_names_by_code_point`; the term store's unit test `semantic_order_sorts_names_by_text` in `ein-core/src/terms.rs` |
| a name sorts before a fact; facts order by relation, then arguments, then arity | the term store's unit tests `semantic_order_sorts_names_by_text`, `facts_order_as_their_identity_tuples_do` in `ein-core/src/terms.rs` |
| a state's identity is its fact list, never a digest; two different closures have different identities | `ein-infer/tests/search_semantics.rs`: `state_key_is_identity_not_a_digest`, `distinct_closures_have_distinct_keys` |
| two activators differing in any argument both fire, in either order, and the order they are written in loses no derivation | `ein-infer/tests/rule_semantics.rs`: `activators_differing_only_by_an_int_argument_both_fire`, `activators_differing_only_by_a_nested_fact_argument_both_fire`, `an_int_beside_a_nested_fact_in_one_position_loses_no_derivation`, `an_int_and_a_string_with_one_text_are_two_activators`; the corpus programs `examples/ein-bugs/binding-key-{both,swapped,int-only,nested-only}.ein` and `activator-text-{int-and-string,string-then-int}.ein` |
| an activator binds a parameter to a nested fact | `ein-infer/tests/compile_semantics.rs`: `a_value_parameter_an_activator_binds_to_a_nested_fact_is_bound` |
| a name is printed bare exactly where it reads back as that name | the grammar's unit test `the_printers_symbol_is_the_lexers` in `ein-ir/src/grammar.rs`; `ein-core/tests/values_semantics.rs`: `display_renders_every_value_shape` |
| every state an answer's files hold reads back as that state | `ein-cli/tests/answer_round_trip.rs` |
| no keyword pair reaches a value's label in a drawing | the renderer's unit test `no_keyword_pair_reaches_a_values_label` in `ein-render/src/dot_util.rs`; `ein-ir/tests/grammar_pages.rs`: `every_row_of_the_grammars_table_is_refused_at_the_parse` |
| the quoted form of every value shape, and its escape for every code point | `ein-core/tests/format_tables.rs`: `repr_of_every_reachable_value_shape`, `repr_escape_table_code_point_by_code_point` |
| a refutation's edges into `⊥` are in the order of the facts' quoted forms, whatever order they were collected in | `ein-render/tests/presentation_semantics.rs`: `the_bottom_edges_of_a_refutation_are_sorted_not_collection_ordered` |
