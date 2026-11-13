# Grammar — lexical rules and top-level forms

> **Source of truth for syntax: [`00_ebnf.md`](00_ebnf.md)** — the complete
> grammar, `ein.l` and `ein.y` as the parser is generated from them (a
> hand-written EBNF until M1f S1f.5.5's swap). This document explains intent
> and structure; that one is what parses.
>
> The engine parses with the parser grmtools generates from
> [`ein-ir/src/ein.l` and `ein.y`](../../../../ein.rs/crates/ein-ir/src/ein.y),
> and `00_ebnf.md`'s two blocks are those files, written from them by a test
> that fails when they differ (M1f S1f.5.5, 2026-11-09). Until S1f.5.5's swap
> the engine parsed by recursive descent, an implementation and not a
> specification, and the grammar was stated there by hand rather than left to
> the parser.

This was [`docs/ir.md` §1 + §2](../../README.md) before the kernel-
documentation split.

---

## §1 Lexical rules

| terminal   | regex                          | examples                                | role                                              |
|------------|--------------------------------|-----------------------------------------|---------------------------------------------------|
| `NAME`     | ``[A-Za-z0-9_.\-$%^*+=<>!&@~/'`\[\]{}]+`` | `has-color`, `House-1`, `is-a*`, `7`, `-7`, `1..5`, `=`, `<=`, `a/b`, `f'` | atoms; a pattern's or a fact's relation; rule / type / step names. **The name rule** (M1f [S1f.5.4](../../../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.4_digits_are_names.md), the user's ruling of 2026-11-06): a run of ASCII name characters, ASCII forever, each printed as written and reloading to itself, so `7` and `07` are two names; no character has a meaning of its own, `*` among them (the transitive-closure convention `R*`, S1.5.8c.2). A reserved word is never a name and a position word is one wherever a name goes ([`00_ebnf.md` § 1](00_ebnf.md#1-lexical-grammar), fact 1); a token ends only at a space, a paren, a comment or the end of input, so `a?x` and `a,b` are each one token that is not a name, refused (fact 5), and `Жук` is refused at its `Ж`, a character that is not ASCII outside a string (fact 6, [below](#what-a-programs-text-may-hold)). Until S1f.5.5's swap this row was `SYMBOL`, `[A-Za-z][A-Za-z0-9_*-]*`, a letter first, a run of digits an `INT` and `=` a reserved head |
| `VAR`      | ``\?[A-Za-z0-9_.\-$%^*+=<>!&@~/'`\[\]{}]+`` | `?a`, `?house`, `?T`, `?R*`, `?_x`, `?a.b` | pattern variables — bound by `:match`, reused in `:assert`: `?` and a name's characters. Until the swap `\?[A-Za-z][A-Za-z0-9_*-]*` |
| `KEYWORD`  | `:[a-z][A-Za-z0-9_-]*`         | `:match`, `:rule`, `:source`             | a form's clause markers, each **always** followed by its value. **Each form takes a fixed set** ([§ 2's table](#the-keywords-each-form-takes)), each keyword a token of the forms that take it, and any other keyword, `:where`, `:cardinality` and `:mode` among them, is refused at the parse, as is any keyword pair inside a term or a nested fact (M1f [S1f.5.11](../../../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.11_closed_keywords.md), the user's ruling of 2026-11-07, [Q-M1e2.28](../../../../plans/m1e2_correctness/open_questions.md#q-m1e228--which-keywords-does-each-form-take)). Until S1f.5.5's swap a keyword was one token for every form: only `query`, `solve` and `config` refused one they did not know, and every other form, and every term, dropped one in silence (D-15, G-15, G-22; W32's seven rows); S1f.5.11's T1f.5.11.2 retired the compiler's and the macro expander's drops |
| `WILDCARD` | —                              | —                                        | **none since M1f S1f.5.5's swap**: a bare `_` and `?_` are tokens no production takes, refused at the parse wherever either is written, with the note *`_ is reserved and names nothing: where any value will do, write a variable used once`* (S1f.5.1's ruling, the user's of 2026-11-06, [Q-M1e2.22](../../../../plans/m1e2_correctness/open_questions.md#q-m1e222--does-ein-need-a-wildcard)); `_` anywhere else in a word is a name character, so `_a_in_b_`, `_______01` and `A________B` are names and `?_x` a variable. A variable used once is what to write where any value will do. Until the swap `WILDCARD` was the argument wildcard of a rule's pattern: in a `:match`, at any depth, `_` matched any value and bound nothing (M1e2 [S1e2.2.20](../../../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.20_wildcard_and_range_in_a_rule.md), 2026-11-05, Q-M1e2.13 (a)); written as a relation, `(_ a)`, as a guard's argument or as a premise it was refused at load (Q-M1e2.14 (c), S1e2.2.24); in an `:assert` the run stopped at the rule's first firing; and in a fact, a nested fact or an activator it was dropped in silence, `(r _ a)` stored as `(r a)` (W-22, W29). Until S1e2.2.20 the rule compiler lowered it as a slot that matched nothing (W20). S1f.5.1's T1f.5.1.3 retired the AST's `Wildcard` node and the compiler's `Slot::Any` (`ef5374a1`) |
| `INT`      | —                              | —                                        | **none since M1f S1f.5.5's swap**: a run of digits is a `NAME` like any other (S1f.5.4's ruling, the user's of 2026-11-06), so `7`, `07` and `-7` are three names, each printed as written, and a fact of one is never a fact of another. Until the swap `INT`, `-?[0-9]+`, lexed *integer atoms*, `007` and `7` one value; `:priority 10`, the one keyword that took one, went with S1f.5.15. The int value kind went with [S1f.5.4](../../../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.4_digits_are_names.md)'s T1f.5.4.3 (2026-11-09): a value is a name or a nested fact |
| `TEXT`     | `"…"`, the escapes `\n \t \r \" \\` and `\u{…}` | `"condition (10)"`, `"{?r} is transitive"`, `"Жилец {?who}"` | **natural-language text, and nothing else since M1f S1f.5.5's swap**, where this row was `STRING` (S1f.5.2's ruling, the user's of 2026-11-06, [Q-M1e2.23](../../../../plans/m1e2_correctness/open_questions.md#q-m1e223--what-is-a-string-for)): the value of `:why` (a template), `:source` (a source sentence) or `:goal-text` (a template), in any Unicode, which the read-out, `--trace`, `saturate --dump`, a state file and `render program` carry byte for byte (`ein-cli/tests/natural_language_text.rs`). Written anywhere else, a fact's or a rule's argument, a name, a keyword's value, it is a parse error at the string with a note saying so, and the name written bare is what it meant: `(p "1")` is refused, `(p 1)` is the fact. A `\u{…}` names a Unicode scalar value ([`00_ebnf.md` § 1](00_ebnf.md#1-lexical-grammar), fact 4). **Until the swap** a string was also a **fact's argument**, top-level or nested, the name its body spells: `"foo"` and `foo` were one value (M1e2 S1e2.2.16, 2026-11-04); and in a **rule** the symbol it names at any depth, a name of the program as that atom is (M1e2 [S1e2.2.18](../../../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.18_strings_in_rules.md), 2026-11-04), where until then the rule compiler did not lower one (W18) |

**There is no `RANGE`**, and no number of any kind: Ein has no numbers and no
ranges, and none is planned (the user, 2026-11-06,
[Q-M1e2.24](../../../../plans/m1e2_correctness/open_questions.md#q-m1e224--are-there-numbers-in-ein)).
M1f [S1f.5.3](../../../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.3_range_retired.md)
retired the token on 2026-11-08, so `1..5` was the `INT` `1` and then `..5`,
no token, and a parse error wherever it was written, until S1f.5.5's swap
made it one name (S1f.5.4's ruling): `(r 1..5 a)` is a fact of the name
`1..5` since. Until S1f.5.3 `RANGE`, `[0-9]+\.\.([0-9]+|\*)`, was *UML-style
cardinality* for a `:cardinality` no program wrote and no code read: in a fact
it stored the name `1..5`, as a keyword's value it was dropped, and in a rule
the loader refused it (M1e2 S1e2.2.20, Q-M1e2.13 (a)).

The regexes above are the readable form.
[`00_ebnf.md` §1](00_ebnf.md#1-lexical-grammar) is the exact one, `ein.l` as
the lexer reads it: every token's rule, the order that settles a tie between
a reserved word and `NAME`, and the start condition that ends a token at a
separator. Until S1f.5.5's swap it stated three lookaheads this table could
not show: `SYMBOL`'s reserved-word exclusion, `WILDCARD`'s, and `STRING`'s
refusal of an escaped newline; the first two are a tie and a token no
production takes since, and the third is a `BAD` token.

**Comments** — SMT-LIB-compatible: `; line` to end of line, `#| block |#`
non-nesting.

### What a program's text may hold

A program is UTF-8 text (or a `.einb` container, which stores one), and **a
character that is not ASCII is written only inside a string literal, after
`:why`, `:source` or `:goal-text`, or in a comment** (M1f
[S1f.5.22](../../../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.22_unicode_only_in_a_string.md),
the user's ruling of 2026-11-10, *"Unicode is not allowed anywhere else than
inside string literals, and the error must say it explicitely"*, and
[Q-M1f.65](../../../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f65--may-a-comment-hold-unicode)
(b), *"Comments may hold Unicode"*). Anywhere else, in a name, a variable, a
keyword, between two tokens or in place of a paren, the lexer stops at it, and
every command refuses the program with one line that names the character, its
code point and where Unicode is allowed:

```text
x.ein:7:7: 'é' (U+00E9) is not ASCII: Unicode is allowed only inside a string literal (after :why, :source or :goal-text) or in a comment
x.ein:7:3: '\u{a0}' (U+00A0) is not ASCII: Unicode is allowed only inside a string literal (after :why, :source or :goal-text) or in a comment
```

A character a reader cannot see where it stands, a space, a control, a
zero-width or bidirectional format character, a combining mark or a variation
selector, is quoted as the escape a string writes it with, as the second line
quotes a no-break space
([Q-M1f.92](../../../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f92--how-the-line-quotes-a-character-a-reader-cannot-see));
the line names the file's first such character, where the lexer stops
([Q-M1f.95](../../../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f95--how-many-characters-the-refusal-names)).
A separator is ASCII's whitespace, `\t \n \v \f \r` and the space, and
`\x1c` to `\x1f`. **Until S1f.5.22** a character that is not ASCII glued to a
token made it one token that is not a name, *`unexpected `café`: a token that
is not a name`*, and a Unicode space, U+00A0, U+3000, U+2028 or U+0085, was a
separator, so a program writing one loaded and was answered as if it were a
space (W73, [`corpus/wrong_answers.toml`](../../../../corpus/wrong_answers.toml)).
A byte that is not UTF-8 is refused before the text is read, `x.ein:2:4: byte
0xff is not UTF-8: a program is UTF-8 text, or a .einb container` (since
S1f.5.5's T1f.5.5.13).

**An imported module is read as the program a command is given is read**
(`ein_ir::read`, since S1f.5.22, the close review's finding 2), so a module
that is not UTF-8, one that cannot be read and one that is a directory are
refused with the line their path gives as a program, behind `kb load error: `
and naming the path the import resolves to, and a module that is a `.einb`
container with a line of its own; a module that holds a character that is not
ASCII outside a string is refused at it. Until then each was `(import m) —
module not found at m.ein (None)`, a path that exists reported as missing
(W74); a module that does not exist keeps that line, its `(None)` F23's D-2.

**Naming convention** — hyphenated lowercase for relations and rule
names (`has-color`, `triangle-composition`); PascalCase or `Foo_N`
for types and instances (`Person`, `House-1`, `Norwegian`). Convention
only; the grammar accepts either.

---

## §2 Top-level forms

A program is a **flat sequence of forms** (P1.7c). Each top-level form is
classified by its **head** against the closed declarator set; anything
whose head is not a declarator is a **fact**. Source of truth for the set:
[`06_reserved_names.md`](06_reserved_names.md) (the parser + loader both
key on it).

| head | role | shape |
|------|------|-------|
| `relation` | declare a relation-type + its arg-type signature | `(relation <name> <T1> <T2> [<T3> …] [:why "…"])` |
| `rule` | a saturation rewrite rule | `(rule <name> (<param-vars>*) :match … :assert … …)` |
| `hrule` | a hypothesis-generation rule (drives the blind enumerator) | same shape as `rule` |
| `query` | what to ask the engine — and, with `:expect`, what the answer should be | `(query :goal … [:goal-text …] [:over …] [:expect …])`; several per file |
| `solve` | the problem's lattice: what may be guessed, stated once (M1e2 S1e2.1a.4) | `(solve [:hrules …] [:hypothesis-relations …] [:no-hypothesis …] [:expect …])`; at most one per file |
| `config` | solver knobs | `(config [:lattice-sanity-check true\|false] [:record-alternative-justifications true\|false])` |
| `import` | bring a module's declarations and facts into the program ([§ Import](#import)) | `(import <module> [:as <alias> \| :symbols (<name>+)])` |
| `trace` | **engine output** — derivation log (engine-emitted, not authored) | `(trace (step …)*)` |
| *anything else* | **a fact** | `(not <fact> …)` · generic `(<NAME> <args>* [:source "…"] [:rule <name> :using (<fact>+)])` |

#### The keywords each form takes

Each form takes the keywords its production in
[`ein.y`](../../../../ein.rs/crates/ein-ir/src/ein.y) lists, and no other; a
term and a nested fact take none. Any other keyword, or a keyword pair inside
a term, is refused at the parse, from every command that reads the program,
with the file, line and column: *unexpected `:where`: a keyword this form does
not take*, and the repairs (M1f
[S1f.5.11](../../../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.11_closed_keywords.md)'s
lists, the user's ruling of 2026-11-07,
[Q-M1e2.28](../../../../plans/m1e2_correctness/open_questions.md#q-m1e228--which-keywords-does-each-form-take)).
Which keywords a form requires, and that each appears once, stay the loader's.

| form | keyword | holds | required |
|---|---|---|---|
| `relation` | `:why` | a string: the template that renders the relation's facts | at load, by default ([§ `:why`](#relation-declarator)) |
| `rule`, `hrule` | `:match` | a premise: the LHS | yes |
| | `:assert` | a conclusion: the RHS | yes |
| | `:why` | a string: the template that explains a firing | at load, by default |
| a fact, negated or not | `:source` | a string: the fact is given, and where it is stated | no |
| | `:rule`, `:using` | a rule's name and its premises: the fact is derived | no |
| `query` | `:goal` | a premise: the pattern asked about | yes |
| | `:goal-text` | a string: the headline template | no |
| | `:over` | `all`, `minimal` or `maximal`: which states | no |
| | `:expect` | a claim: the answer stated | no |
| `solve` | `:hrules`, `:hypothesis-relations`, `:no-hypothesis` | what may be guessed: the lattice | no |
| | `:expect` | a claim about the region | no |
| `config` | `:lattice-sanity-check`, `:record-alternative-justifications` | the flags, `true` or `false` ([`configuration.md`](../../configuration.md)) | no |
| `import` | `:symbols` | the names imported, flat: each selects the module's declaration of it and its facts of that relation ([§ Import](#import)) | no |
| | `:as` | an alias | no |
| `macro` | `:why` | a string, after the body (the user's ruling of 2026-11-08) | at load, by default |
| `trace` | none; its `step` takes `:rule`, `:using`, `:derives`, `:source`, `:assumes` | one derivation step | no |
| a term, a nested fact | none | | |

Until S1f.5.5's swap (2026-11-09) only `query`, `solve` and `config` refused a
keyword they did not list, at load. Every other form dropped one, and so did
every term: a keyword pair in a `:match` or a `:goal` was dropped by the
compiler (Q32), one in a macro call by the expander's arity filter, and an
annotation such as `(relation s Node :cardinality 1..1)` or `(q c :why
"text")` by the loader, so a written condition or conclusion read as nothing
(W32's seven rows, fixed by the swap). `:cardinality` and a query's `:mode`,
which nothing read, are refused like any other, and `:priority` went with
S1f.5.15. The grammar's table holds a row for each form and each place in a
term (`ein-ir/tests/grammar_pages.rs`), and the fuzzer holds the lists on every
program it accepts (`fuzz_properties`'
`no_accepted_program_carries_a_keyword_its_form_does_not_list`).

The block wrappers `(ontology …)` / `(facts …)` / `(reasoning …)` /
`(rules …)` were **removed in P1.7c** (S1.7c.4); a former-wrapper head now
simply reads as a fact (e.g. `(facts X)` is a fact whose relation is
`facts`).

### Where a fact came from — its provenance annotation

A fact's origin is read off the annotation it carries, and nothing else
(see [`../01-ein-graph/01_kb.md` §3](../01-ein-graph/01_kb.md)):

| the fact carries… | it is… | meaning |
|---|---|---|
| `:rule` / `:using` | **derived** | the engine produced it (a rule firing) |
| `:source "(N)"` | **given** | an explicit, numbered problem statement |
| neither | **background** | an implicit assumption (schema, `is-a` enumeration, property tag) |

The distinction is **presentation only** — the engine treats every fact
alike, and fact identity is `(relation_name, args)` regardless. There is no
way to override it, and no need for one: write the annotation that is true —
a numbered condition gets `:source`, a property tag gets nothing. See
[`../02-data-model/01_entities.md` §4](../02-data-model/01_entities.md)
and [`plans/ideas/04-nlp-to-graph-to-solver-pipeline.md` §Ontology
deduction by common sense](../../../../plans/ideas/04-nlp-to-graph-to-solver-pipeline.md)
for how the NL frontend recovers the ONTOLOGY-vs-FACT split from context.

### Relation declarator

```lisp
(relation <name> [<T1> <T2> ...]         ; signature: zero or more type atoms
  [:why "<text>"])                        ; the one keyword a relation takes
```

`relation` declares a relation-type node + its arg-type signature
(structural / spatial relations are plain `(relation …)`; a pattern-based
derivation is an ordinary `rule`; `a-priori` removed in S1.7.6). The loader
auto-stores each declaration as an ordinary fact `(relation R T1 T2 …)` so
rules can introspect signatures via a `(relation ?R ?A ?B)` pattern in
`:match`. Because of that, `relation` is a position word, a name wherever a
name goes, `(relation ?R ?A ?B)` a pattern; as a form's head it is the
declarator, `Relation ::= '(' 'relation' DeclName Names RelationClauses ')'`,
so the malformed wrapped-arg form `(relation R (T1 T2))` is a parse error
since M1f S1f.5.5's swap. Until then `relation` was not `SYMBOL`-excluded, so
that form parsed as a fact and the loader rejected it.

**Arity — name + *zero or more* type atoms.** The grammar is
`Relation ::= '(' 'relation' DeclName Names RelationClauses ')'`, its clauses
`:why` alone ([`00_ebnf.md` § 2](00_ebnf.md#2-phrase-grammar)), and the loader
reads what it parsed. So `(relation adult Person)` is a legal **unary**
declaration and `(relation opaque)` a legal **bare** one — a relation node
with no declared arg types. Only the *name* is mandatory: `(relation)` and
the wrapped-arg form `(relation R (T1 T2))` are parse errors (P1.3 R10), the
second because a type is a name. **Until M1f S1f.5.5's swap** the
hand-written grammar was `relation_decl: "(" "relation" SYMBOL SYMBOL*
kw_pair* ")"`, and the loader refused `(relation)` and checked that every
argument after the name was a type atom or a keyword pair, so the inner group
could not pass itself off as a bare declaration (S1f.5.5's T5 deleted both
lines, the grammar stating each).

**A declaration's count is held** (M1f S1f.6.23, W89, the user's ruling of
2026-11-12). The number of type atoms is the number of arguments a form of
the relation has, and the loader refuses a program that writes one with
another count: a fact, a fact inside a fact at any depth, a pattern or a
conclusion of a rule or an hrule, a goal's pattern and a fact a claim lists,
one line a form, `t.ein:3:2: the fact (likes Bob) gives likes 1 argument, and
(relation likes Person Drink) declares 2 — …`
([`defined_behaviour.md` § 4.26](../../defined_behaviour.md#426-a-form-of-a-declared-relation-has-the-declared-count-of-arguments--m1f-s1f623)).
A bare `(relation opaque)` declares none, so `(opaque)` is its one form. A
keyword pair is no argument. A relation nothing declares has no count and is
not held, a relation written as a variable, `(?R ?a ?b)`, is not read, and
the type atoms themselves stay opaque: `(likes Tea Ann)` loads. Until the
stage nothing held a form to its declaration, and one of another count was
stored, or matched nothing, in silence.

A bare declaration is **not a hypothesis target**: signature *presence* is
the kernel's "declared domain relation" signal (the table below), and an
empty signature deliberately fails it. Write `(relation R T)` — the
don't-care atom — when you want a guessable relation with no meaningful
type.

The blind enumerator fills **arity 1 and 2**
([`hypgen.rs`](../../../../ein.rs/crates/ein-infer/src/hypgen.rs),
`_fill_slot`): a unary relation yields one candidate per focal object,
a binary one yields |objects| per focal object. Arity ≥ 3 declarations
load, store facts and saturate normally but are never guessed — the one
remaining M1 arity cut, and an *engine* limit rather than a grammar rule.
See [`../01-ein-graph/03_ein_model.md` §5.1](../01-ein-graph/03_ein_model.md)
for unary relations as subsets.

**The membership fact.** Because matching is arity-coupled,
`(relation ?R ?A ?B)` sees only *binary* declarations. Every declaration
therefore also stores the arity-1 fact `(relation R)` — the
arity-independent "is `?R` a declared relation?" pattern:

```lisp
(relation likes Person Drink :why "{?1} likes {?2}")
;; stores BOTH  (relation likes Person Drink)   ← the signature mirror
;;        and   (relation likes)                ← the membership fact
(rule needs-a-relation () :match (relation ?R) :assert (…) :why "{?R} is a declared relation")
```

For a bare declaration the two coincide, so exactly one fact is stored.
Auto-vivified relations — property-tag carriers like `symmetric`, which
have no declaration — get neither, so `(relation ?R)` means precisely
*declared* relation.

**`:why` render template.** A `:why "<tmpl>"` string turns a fact
of this relation into natural-language text, its *statement*: written after each
fact of the relation in a `--dump-answer` state file and after each one `--trace`
derives, since M1f S1f.5.14's T1f.5.14.2
([`09_natural_language.md`](09_natural_language.md) states every text a
program writes, where each is printed, the ladder a fact falls back on, and
whose words are whose). Until M1e2 S1e2.1c.5 `ein solve`'s result table printed it (the *rendered query
facts* column), and from then until T1f.5.14.2 nothing did. It reuses the rule `:why` engine
but references the fact's argument **slots positionally**: `{?1}` is arg 0,
`{?2}` is arg 1, … (a leading digit is the relation-template form;
rule/goal `:why` uses letter-led var names). A relation with **no** `:why`
renders as its raw IR s-expression `(R a b)` — there is no built-in
relation→verb vocabulary, so untemplated relations stay in IR. Optional in
the grammar, on a relation, a rule, an hrule and a macro alike, and required at
load by default since M1f S1f.5.14's T1f.5.14.7; `--no-require-why` loads a
program without one (T1f.5.14.6, T1f.5.14.7,
[`defined_behaviour.md` § 4.10](../../defined_behaviour.md#410-a-form-with-no-why--m1f-t1f5146-t1f5147)). Example:

```lisp
(relation drink-loc Drink House :why "{?1} is drunk in {?2}")
;; (drink-loc Water House-1)  →  "Water is drunk in House-1"
(relation right-of House House)            ; no :why: refused at load; with --no-require-why, renders as (right-of …)
```

A schema + implicit-assumption example (all flat forms; the property tags
and enumerations carry no annotation, so they read as background):

```lisp
(is-a House Attribute) (is-a Color Attribute)
(relation co-located Attribute Attribute :why "{?1} and {?2} are in the same house")
(is-a Norwegian Nationality)
(is-a House-1 House)
(symmetric  co-located)
(transitive co-located)
(right-of House-2 House-1 :source "condition (1)")  ; "five in a row" — structural, but it *is* condition (1)
```

#### What the signature means — userspace types, kernel structure

The kernel imposes **no type system**
(S1.7.23),
yet a signature is not inert. It plays **two unrelated roles**, and they
never mix: rules read the signature's *content* as types; the kernel reads
its *shape* as structure. This subsection is the definitive statement of
that split.

**As types — userspace only.** The mirror fact is the whole mechanism:
everything type-like keys off `(relation ?R ?A ?B)` **in rules**, never in
the engine — [`std.bijection`](../../../../stdlib/bijection.ein)'s
typecheck stack (`(and (relation ?R ?A ?B) (bijective ?R)
(typecheck-hierarchy ?isa))`),
[`std.algebra`](../../../../stdlib/algebra.ein)'s converse
domain/range check, `std.typing`'s hierarchy wiring, zebra2's
`disjunctive-prune`. The kernel never resolves a signature atom to
anything: `Relation.signature` holds *opaque atoms*, and since S1.7.23
there are no `Type` entities to resolve them **to**
([`../02-data-model/01_entities.md` §1.1](../02-data-model/01_entities.md)).

**As structure — kernel, three signals.** The engine reads the
signature's shape only, and only in these places:

| signal | site | effect |
|---|---|---|
| signature **non-empty** | `ein-infer/hypgen.rs` (candidate enumeration), `closed.rs` (`emit_closed`) | marks a *declared domain relation* — eligible for hypothesis generation and for `__closed__` auto-inference. Property / rule-name relations (auto-vivified, empty signature) are skipped by both. |
| signature **length** | `ein-infer/hypgen.rs` (the slot fill) | length 1 → one candidate per focal object; length 2 → the pairwise fill; length ≥ 3 → unenumerated (the M1 arity cut noted above). |
| signature **atoms** | `ein-infer/hypgen.rs` (`candidate_objects`) | the declared type-role names (`Attribute`, `House`, `T`, …) are subtracted from the candidate-**object** pool, so the enumerator never guesses *about* a type node. |

Two further reads are ergonomic rather than semantic: the declaration's
`:why` render template (above) and the signature column of `ein saturate`'s
relation table (`ein-cli/saturate.rs`).

So "are the signature's type atoms used explicitly through rules, or
implicitly in the kernel?" — **both, in different roles.** As *types* they
exist only for userspace rules; as *structure* (present / length-2 /
name-set) they steer hypothesis generation. No kernel site ever interprets
an atom like `Attribute` as a type. See also
[`08_self_describing.md` §3](08_self_describing.md) for the userspace half
in its own right, and
[`06_reserved_names.md` §Not reserved](06_reserved_names.md) for why `T` is
a convention rather than a kernel atom.

**Why `relation` is a kernel declarator and not a stdlib word** (decided
2026-08-17): because demoting it would remove no kernel *interpretation* —
the three structural signals above are what makes hypothesis generation
terminate, so they survive any demotion, and the kernel would go on
hardcoding the name `relation`, merely as a reserved *fact head* (the
`__closed__` / `not` / `false` category) rather than a declarator. The
reserved set would be renamed, not shrunk. What demotion *would* cost is
concrete: the malformed / shadowed-name / duplicate-declaration checks lose
their load-time `loc` (a conflicting re-declaration would become two stored
facts and an ambiguous signature lookup feeding the enumerator, with no
diagnostic), and `:why` would be refused, since a fact takes `:source`,
`:rule` and `:using` and no other keyword (S1f.5.11; until S1f.5.5's swap a
fact discarded one it did not read, and `:why` would have been dropped in
silence). Userspace gives up nothing for this: the
declaration is *already* published as the mirror fact — and, since the
same decision, as the **membership fact** above, which is what the
question "can a rule check that `?R` is a relation?" was really after. The
friction was arity-coupling in the *published* schema, not the declarator;
publishing more fixed it without demoting anything.

### Facts — `(NAME args*)`, the flat default

A fact is any top-level form whose head is **not** a declarator. Three
shapes:

```lisp
(not <fact> [:source "<text>"])           ; negative            (reserved head `not`, one fact)
(<name> <arg>* [:source "<text>"])        ; relation instance / enumeration / property tag / equality
```

| kind | example | semantics |
|---|---|---|
| **relation instance** | `(co-located Englishman Red :source "(2)")` | a relation holds between specific entities |
| **equality**          | `(= Red Colour-1 :source "(?)")` | a fact of the relation `=`, a name since M1f S1f.5.5's swap; until then `=` was a reserved head taking two expressions, `(= (color House-1) Red)` among them |
| **negative**          | `(not (drinks Spaniard Coffee) :source "(?)")` | the wrapped fact does *not* hold |

`not` is a **shape-pinned reserved head**: `NotFact ::= '(' 'not'
NestedFact FactClauses ')'`, one fact, so a wrong arity is a parse error
(`=` was one too, two arguments, until M1f S1f.5.5's swap made it a name,
S1f.5.4's ruling, so `(= a b c)` is a fact). `and` / `or` / `neq` are kernel
meta-primitives that belong inside
`:match` premises (a guard is a premise, `(neq ?a ?c)`, since Q32 dropped the
`:where` clause), never a top-level fact head. **`eq` and `neq` are relations
whose facts are computed** since M1f
[S1f.5.7](../../../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.7_eq_and_neq_are_relations.md) (below,
[§ Premise order around a guard](#premise-order-around-a-guard)), so no fact
changes them: `(neq …)` is no fact the grammar takes, and a written `(eq a b)`,
or `(not (eq a b))`, is checked as a conclusion with no premise, a true one not
stored and a false one `(false)`, written where it stands (Q-M1f.33, Q-M1f.31),
and one at any other arity, `(eq a)`, is refused at load since M1f S1f.5.9 (W61);
until S1f.5.7 each was stored as an ordinary fact, which no guard read. **A
value is a name, or a fact a state could hold**, an atom of an ordinary
relation or its negation (the user's ruling of 2026-11-11,
[the value rule](../../../../plans/m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/README.md#the-value-rule)): so `(eq a b)` is written
only as a fact of its own, never inside another, `(q (eq a b))`, as `(q (neq a
b))` never was (M1f S1f.7.10); no fact or value is headed `absent`, which is
negation as failure, a premise's test, and the stored negation of a fact is
`(not …)` (S1f.7.7); and `false` takes no argument, `(false)` alone, as a
fact and as a value (S1f.7.6). Each is a parse error with a note
([`defined_behaviour.md` § 4.22](../../defined_behaviour.md#422-a-value-is-a-name-or-a-fact-a-state-could-hold--m1f-s1f77-s1f76-s1f710-s1f73)); until those stages each was
stored. And the
declarators `rule` / `hrule` / `query` / `config` / `trace` cannot be fact
heads either (each is a reserved word, never a name). Everything else —
`instance`, `type`, `symmetric`, `co-located`, `lives-in`, … — is an
ordinary generic `(NAME value*)` fact, open-world: introduce a relation by
declaring it with `(relation …)` and asserting it.

The origin of each fact follows the
[annotation rule](#where-a-fact-came-from--its-provenance-annotation): a
`:source` makes it a given problem statement; `:rule` / `:using` make it
an engine derivation; no annotation makes it a background assumption.
Rule-application facts (`(symmetric co-located)`,
`(implies right-of next-to)`) carry no annotation → background: the
puzzle text never says "co-located is
symmetric"; that's universal context, the *meta* of the relation, while a
`rule` is the meta of the *engine*.

`all-different` is **not** a kernel primitive; pairwise distinctness within
a category is derived by `type-exclusivity` from the `(is-a X T)`
facts. Genuinely puzzle-specific structural shapes (parity, budget, …) just
take their own head: `(budget-total X Y)`.

A flat explicit-conditions example (each numbered condition is a given
via its `:source`; the un-annotated property tag is background):

```lisp
(lives-in Norwegian House-1 :source "condition (10)")
(co-located Englishman Red  :source "condition (2)")
(symmetric  co-located)                              ; property tag — implicit, background
```

**A fact is not a macro's call** (M1f
[S1f.7.12](../../../../plans/m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/s1f.7.12_a_macro_call_written_as_a_fact.md);
[`defined_behaviour.md` § 4.24](../../defined_behaviour.md#424-a-macros-call-is-no-fact--m1f-s1f712)).
A `(macro …)` is called in a rule's or an hrule's `:match` and `:assert` and
in a query's `:goal`, where the loader replaces the call by the macro's body.
A fact is not expanded. So a fact headed by a macro's name, `(settled Ann)`
beside `(macro settled (?g) (paid ?g))`, is refused at load, and so is one
that holds the call at any depth, `(says (settled Ann))`, `(not (settled
Bob))`, one a claim lists, and a `(relation …)` declared under a macro's name.
The same holds of `std.macro`'s `unknown` and `forall`, imported or not. The
author writes the fact itself, `(paid Ann)`. A macro's name written as a
value, `(word settled)`, is a name.

### Derived facts — engine working memory

The engine *derives* facts at runtime and dumps them as flat forms
annotated with `:rule` (which rule fired) + `:using` (which premises it
consumed) instead of `:source` — so they read back as derivations:

```lisp
(<name> <arg>* :rule <RuleName> :using (<premise-fact>+))
(not <fact>    :rule <RuleName> :using (<premise-fact>+))
```

A hand-authored puzzle has none; they appear in engine dumps, which
round-trip through `parse` / `dump` **exactly**: the dump carries the
provenance, the provenance *is* the origin, and no further annotation is
needed to reproduce it. Example derived facts:

```lisp
(co-located Blue House-2 :rule square-fwd :using ((co-located Ivory House-1) (right-of House-2 House-1)))
(not (co-located Norwegian House-2) :rule type-exclusivity :using ((co-located Norwegian House-1)))
```

> **`:using` is a list of facts** since M1f S1f.5.5's swap (2026-11-09,
> the paper grammar's default 2): `FactClause ::= ':using' '(' NestedFacts
> ')'`, the data model's `(rel, args)` premise ids, each fact resolved by the
> loader to the premise the derivation record names. **Until the swap** the
> hand-written grammar took `:using (atom-id-1 atom-id-2 ...)`, a list whose
> first element the loader read as a head and dropped, and rejected the
> compact form `:using ((rel a b) (rel c d))`, a keyword's value having to
> be a headed list; both waited on a P1.1 grammar tweak or a `:id <atom>`
> annotation system (S1.2.3 T1.2.3.4), and rule-kind provenance was populated
> by the engine programmatically alone. The engine's dumps still write
> `:rule` with no `:using` (`ein saturate --dump`), or the provenance as a
> comment (a `--dump-answer` state file).

### Rules

```lisp
(rule <name> (<param-vars>*)      ; parameter list — mandatory, `()` for non-generic
  :match <pattern>                ; LHS — structural pattern (see 02_patterns.md)
  :assert <conclusion>            ; RHS — what to derive
  :why "<text>")                  ; reason template for trace
```

A rule is a top-level `(rule …)` form (a hypothesis-generation rule is the
same shape headed `hrule`). There is no `(rules …)` block (P1.7c).

Each rule has one `:match` and one `:assert`. The pattern sub-language
is in [`02_patterns.md`](02_patterns.md). **A rule has no priority**
since M1f [S1f.5.15](../../../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.15_rule_priority.md)
(2026-11-08, the user's ruling on Q-M1f.6: *"plain FIFO, core is an
explanation, F22 makes it canonical"*): the saturation agenda and the
`absent` boundary are FIFO, the order rules are written in and facts
arrive in. Until then `:priority <INT>` ordered the agenda, lower
first, which resolved [Q15](../../../../plans/open_questions.md#q15--rule-ordering)
(rule ordering) with static per-rule numbers. A `:priority` written
anyway is refused at the parse since S1f.5.5's swap, as every keyword a
rule does not take is (S1f.5.11); from S1f.5.15 to the swap the loader
did not read it and dropped it, as it dropped any (D-15). No corpus
program writes one (`ein-corpus`'s `no_corpus_program_writes_a_priority`).

The **parameter list** is mandatory. Two cases (mapping onto the
three rule types from
[`../01-ein-graph/02_rules.md`](../01-ein-graph/02_rules.md)):

| parameters | rule type | how it fires |
|---|---|---|
| **non-empty** `(?p1 ?p2 …)` containing **relation variables** | **T2 — relation-polymorphic** | fires only when bound to a rule-application fact, e.g. `(symmetric co-located)` binds `?rel = co-located` for the `symmetric` rule |
| **empty `()`** | **T1 first-order OR T3 structural** | fires universally on every match; free vars in `:match` are bound by the matcher |

A relation-polymorphic rule with no matching application facts never
fires — the parameters are the gate. A non-generic rule needs no
application fact.

**An empty `:match` is the empty conjunction, which is true** (M1f
[S1f.5.9](../../../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.9_an_empty_match_fires.md), the user's
ruling of 2026-11-07 on P1f.5's F12: *"1 fire; 2 same; 3 reject (and) while
it is the same as () and `and` should have arguments"*). `:match ()` holds
once: a rule fires once at root, or once per activator fact with only its
parameters bound, `(rule r (?x) :match () :assert (hit ?x))` with `(r a)`
concluding `(hit a)`, and an hrule's proposes its conclusion likewise. `()`
is its one spelling: `and` and `or` take one premise or more, so `(and)` and
`(or)` are parse errors, each with a note naming the rule and the clause it is
written in and, for `(and)` as a whole `:match` or `:assert`, saying `()` is
the spelling (Q-M1f.45). **`:assert ()`, the empty conclusion, asserts
nothing**: an application derives nothing and is no firing, as one whose every
conclusion holds. A rule whose `:match` is `()` and whose conclusion is
`(false)` makes root dead, its refutation resting on no fact, so root's core is
`[]`, the empty set, which the text surfaces print `∅` (Q-M1f.48). Until S1f.5.9 `()` was read as a premise over the parser's
synthetic `#empty`, which no fact holds, so such a rule never fired and such an
hrule proposed nothing, in silence (W31), and `:assert ()` stored a fact under
that head, whose name joined H (W34).

**An activator binds each parameter to its argument, any value**: a name, or a
nested fact, `(act (f a))` binding `?y` to `(f a)`, which the rule's patterns
match and its conclusions write as they would a name (M1f
[S1f.5.18](../../../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.18_an_activator_binds_a_nested_fact.md), the user's
ruling of 2026-11-08 on S1f.5.5's census; until then a nested fact bound
nothing, and its parameter matched every value, W33). A parameter that names a
relation, `?rel` above, binds a name: one its activator binds to a fact is
refused at load, where the activator is written down and, since S1f.5.18, where
a rule could derive it, which the loader reads off the program
([`defined_behaviour.md` § 4.9](../../defined_behaviour.md#49-a-rule-shape-that-reads-nothing--m1e2-s1e2224)).
One that can be bound to `eq`, the relation of equality between two values,
heads premises and conclusions of two arguments: one of another arity, `(?R
?x)` under `(r eq)`, is `(eq ?x)`, which the grammar refuses where it is
written, and is refused at load since M1f S1f.5.9, read the same way
([`defined_behaviour.md` § 4.14](../../defined_behaviour.md#414-eq-at-another-arity--m1f-s1f59));
until then the run stopped where the plan was compiled, or the conclusion
stored `(eq a)` (W60).

Concrete: the rule

```lisp
(rule symmetric (?rel)
  :match  (?rel ?a ?b)
  :assert (?rel ?b ?a)
  :why    "{?rel} is symmetric.")
```

is applied via the fact `(symmetric co-located)`, which substitutes
`?rel = co-located` and then matches `(co-located ?a ?b)` against
working memory. One generic rule per property replaces N per-relation
property-rules.

#### Premise forms in `:match`

In addition to ordinary fact patterns, `:match` accepts three
NAF / quantifier-style premises:

| premise            | semantics                                           | added in    |
|--------------------|-----------------------------------------------------|-------------|
| `(not P)`          | matches a STORED `(not P)` fact in the KB           | S1.5.8c.1   |
| `(absent P)`       | negation-as-failure — holds iff no fact matches P in the world at the closure boundary (see below) | S1.5.8c.1   |
| `(unknown P)`      | `std.macro` macro for `(and (absent P) (absent (not P)))` — the third-state match: P is neither asserted nor negated | S1.5.8c.3b |
| `(forall ?b (G) (B))` | `std.macro` macro for `(absent (and G (absent B)))` — for every binding of `?b` satisfying guard G, body B must hold | S1.5.8c.3a |

The three-state model: at any moment, a potential fact P is
**asserted** (matched by `(P)`), **negated** (matched by `(not P)`),
or **open** (matched by `(unknown P)`). The earlier overloaded
`(not P)` meaning (default NAF) was dropped in S1.5.8c — NAF must
now be written explicitly as `(absent P)`. `forall` and `unknown` are
load-time `(macro …)` expansions in terms of `absent` (the
[`std.macro`](../../../../stdlib/macro.ein) module since
S1.5.9 — import them; see
[`06_reserved_names.md` §macro sugar](06_reserved_names.md#pattern-macro-sugar-forall--unknown--not-reserved));
the compiler itself sees only `absent` + nested patterns, and lifts
each *top-level* one out of the match plan (below).

**What a premise may be.** A premise is a form: a fact pattern, a guard
(`eq`, `neq`, the relations of equality and non-equality, which `(not (eq a
b))` and a pattern whose relation a parameter binds to `eq` are too, since M1f
S1f.5.7), an `(absent …)`, or an `(and …)` or `(or …)` of premises. A
pattern's relation is a name, or one of the rule's parameters, which its
activator binds to a name; its arguments are values, variables or patterns
(and `_` until M1f S1f.5.5's swap refused a bare `_`, S1f.5.1). Since M1e2
[S1e2.2.24](../../../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.24_silent_patterns_refused.md)
(W24, the user's rulings of 2026-11-05) the loader refuses, with one line
naming the rule and the clause
([`defined_behaviour.md` § 4.9](../../defined_behaviour.md#49-a-rule-shape-that-reads-nothing--m1e2-s1e2224)):
an atom (a run of digits among them, an integer until M1f S1f.5.4), a
variable, `_` or a string written where a premise goes (`_` and a string at
the parse since M1f S1f.5.5's swap, S1f.5.1 and S1f.5.2); a relation written as a variable no parameter binds, at the top level
or inside a pattern, in a `:match` or an `:assert`; a keyword pair inside a
pattern inside a pattern (at the parse since M1f S1f.5.5's swap, which
refuses a keyword pair anywhere in a term, S1f.5.11); and an activator that
binds a relation parameter to a nested fact, written down, or, since M1f
S1f.5.18, one a rule could derive (Q-M1f.1 (b)). Until then each read nothing, in silence: a premise that is
not a form was dropped, so the rule fired as if it were not written, and the
rest matched nothing, where the same relation variable at the top level
stopped compilation in the Python IR's spelling. A keyword pair at a
pattern's top level, or as a premise of an `(and …)`, was Q32's, dropped by
the compiler, until the swap refused it at the parse; S1f.5.11's T1f.5.11.2
retired the drop.

#### Conclusion forms in `:assert`

A conclusion is ordinarily a fact pattern, with `(and …)` concluding several
at once (A13 multi-assert). Two heads are **verdicts about the state** rather
than facts, and neither is stored as one:

| conclusion | semantics | added in |
|---|---|---|
| `(false)` | direct ⊥ — this branch is contradictory | S1.5.8c |
| `(open)` / `(open ?R)` | this state is **unfinished**: it owes a witness, and `(open ?R)` names the relation whose extent is incomplete | M1d S1d.2.3 |

**`open` is written there and nowhere else** (M1f
[S1f.5.23](../../../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.23_open_is_reserved.md),
the user's ruling of 2026-11-10 on
[Q-M1f.66](../../../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f66--may-open-be-written-as-a-value),
*"Refuse it"*): a rule's or an hrule's whole `:assert`, `(open)` or `(open ?R)`,
`?R` a parameter or a relation's name. It is one of the words that are never a
name ([`00_ebnf.md` § 1](00_ebnf.md#1-lexical-grammar), fact 1), so no fact, nested
fact, stored negation, pattern, value, type, object, activator argument, goal,
claim, `(solve …)` list, `:over`, import, trace step or macro body writes it, and
each is refused at the parse, at the `open`, with the note *open is reserved: it
is the verdict a rule's :assert states, written as the whole :assert, (open) or
(open ?R), and nowhere else, so it names no relation and no value*
([`defined_behaviour.md` § 4.17](../../defined_behaviour.md#417-open-is-written-only-as-a-rules-verdict--m1f-s1f523)).
A macro's body writes no `open` either, called or not
([Q-M1f.86](../../../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f86--may-a-macros-body-write-open)):
the verdict is written in the rule. Until S1f.5.23 `open` was a word a
position gives a meaning to, a name wherever a name goes, and the loader
refused it only in a rule's `:match`, inside a conclusion and beside one, so
`(p (open x))` loaded (the close review's finding 3), and so did `open` as a
fact's relation, an activator's relation parameter, a value anywhere, which made
it a name of the program the blind enumerator proposed facts about, and a
macro's body stating the verdict for a rule (W70 to W72).

**A conclusion of `eq` or `neq`** — `(neq ?x ?y)`, `(eq ?x w)`, `(not (eq ?x
?y))`, or a pattern whose relation a parameter binds to `eq` — **is checked
against its definition, never stored** (M1f
[S1f.5.7](../../../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.7_eq_and_neq_are_relations.md), the user's G-21
of 2026-11-07): one that holds adds nothing, and one that does not is `(false)`,
derived by the rule from the premises its firing matched, so its state is dead,
its core those premises and `refuted by` the rule. A firing whose every
conclusion holds derives nothing and is no firing. An hrule's instance of either
is no hypothesis (Q-M1f.32). Until S1f.5.7 each was stored as an ordinary fact,
so a false one sat in the KB while a guard computed the opposite (W30).

**A conclusion `(not ?x)` negates the fact `?x` holds**, and a name has no
negation: where the program shows that `?x` can hold a name, the rule or the
hrule is refused at load, and where only a run meets the name the firing
concludes nothing for it (M1f
[S1f.7.4](../../../../plans/m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/s1f.7.4_the_negation_of_a_name_concluded.md);
[`defined_behaviour.md` § 4.23](../../defined_behaviour.md#423-not-of-a-name-is-the-negation-of-no-fact-and-an-hrule-may-read-not--m1f-s1f74-s1f72)).
Until S1f.7.4 it stored `(not k)`, which no program can write (W84).

**Every variable a conclusion writes is bound** on every way the `:match` can
hold (M1f
[S1f.5.10](../../../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.10_a_conclusion_variable_must_be_bound.md),
the user's ruling of 2026-11-07 on F13): by a parameter, which the activator
binds; by a relation premise, at any depth of its arguments; or by an `eq` or
`neq` guard, which binds a variable nothing else binds over the program's
names. A variable seen only inside an `(absent …)`, a `forall` or an `unknown`
is not bound outside it, `()` binds nothing, and a top-level `(or …)` must
bind it in every disjunct. An hrule's `:assert` is held to the same rule. A
rule or an hrule that writes one nothing binds is refused at load, one line
per variable naming the two explicit spellings of what the implicit form could
mean: a premise such as `(is-a ?y T)`, *every object of a type*, or `(eq ?y
?y)`, *every name of the program*
([`defined_behaviour.md` § 4.13](../../defined_behaviour.md#413-a-conclusions-variable-nothing-binds--m1f-s1f510)).
Until S1f.5.10 it loaded, and the run stopped where a firing reached the
variable, or answered where none did (W57).

`(false)` is stored — a contradiction survives any extension, so a dead state
stays dead — and `open` is not, because openness exists to be *destroyed* by
an extension and a stored one would survive its own discharge. It is a tally
on the search-lattice node instead, read once per quiescent KB **after** the
fixpoint; a rule that asserts it is an *obligation* and never enters the
saturation agenda. `(open ?R)` names the incomplete relation and nothing
else: the witness domain and the slot to fill are projected out of the rule's
own `(absent …)`, and where it does not resolve the rule is refused at load
rather than guessed, the line placed where the `(open …)` is written (since M1f
S1f.5.23; five more shapes were refused at load until then, each a parse error
since).
The form, the refusals and their messages are in
[`06_reserved_names.md` § the verdict atom](06_reserved_names.md#the-verdict-atom--open-m1d-p1d2-s1d23-read-since-s1d24)
and [`defined_behaviour.md` §4.2](../../defined_behaviour.md).
**The tally is read twice.** Since S1d.2.4
[`obligations::tally`](../../../../ein.rs/crates/ein-infer/src/obligations.rs)
reports it — on `--events` as one `owe` line per undischarged instance, in
`--json-summary`'s `owes` block, and as the trace's *Outstanding obligations*
section — and since S1d.2.6 the **verdict** reads it: a state that is
consistent, quiescent and complete by the generator's test while still owing a
witness is `Open`, not `Solution`. Since S1d.2.5 the search reads it a third
way, as the middle rung of the hypothesis ladder: the facts that would
discharge what the state owes are what it branches on.

#### NAF evaluation timing — the closure/world boundary

`(absent P)` is a **query over the current fork-local world, answered
at a positive fixpoint** (S1.21.8). The compiler *lifts* every
top-level `(absent …)` out of the rule's match plan
([`compile.rs`](../../../../ein.rs/crates/ein-infer/src/compile.rs)),
so what the matcher runs is purely positive and a match whose disjunct
carries guards is **parked** instead of fired. When the positive
closure quiesces, the saturator builds a
[the boundary phase](../../../../ein.rs/crates/ein-infer/src/saturator.rs) over that
fixpoint and asks the guards there — once, decisively — admitting the
firing iff every one passes. That is the *only* evaluation point: the
old fire-time re-check (`match.absents_still_pass`, S1.5a.1) is
**deleted, not bypassed**, because exactly one candidate is admitted
per boundary round into an empty queue, so it fires against precisely
the world its guard was judged in.

For a rule author, three consequences:

- **`(absent P)` asks "is `P` missing from the finished positive
  derivation?", not "has `P` not been derived *yet*?"** A rule that
  used to fire because its watched fact simply had not been produced
  yet no longer does.
- **The order does not decide what is derivable.** On a stratified
  rule set the result is order-independent; the order, FIFO since M1f
  S1f.5.15 removed `:priority`, still orders firings (hence the trace)
  and decides which derivation of a fact arrives first, which is what a
  core is explained from (F22 makes a core canonical). The band
  discipline zebra2 once needed for soundness — every producer of a
  watched relation at a strictly lower number than every watcher —
  went from load-bearing to advisory at S1.21.8 and with the keyword at
  S1f.5.15.
- **Non-stratified rule sets are still answered by operational order.**
  `p ← absent q` together with `q ← absent p` has two models; the
  engine picks one (by boundary-admission order) and does not report
  that the other exists. `naf_deps::derived_naf_warnings` names the shape
  that can cause it — NAF over a *rule-derived* relation — (`(config
  :warn-derived-naf true)` narrated it until M1e2 S1e2.2.5 retired the
  flag). **The static stratification check exists since M1f S1f.5.15's
  T4**: a program whose rules' dependency graph has a cycle through an
  `absent` is told so, a `strata` row in `ein solve`'s read-out and a
  `;; Strata` line in `ein saturate --dump`, each negative cycle named
  by its relations and the rules on its edges. It is a diagnostic, not
  a refusal, and it says what the graph licenses, never a fact about the
  run: the answer stands, and may depend on the boundary's order and
  need not be unique ([`inference/implementation.md`](../../inference/implementation.md)'s
  `strata.rs`).

A **nested** `(absent …)` — what `forall` desugars to,
`(absent (and G (absent B)))` — is not lifted separately: the whole
double negation is one query, evaluated as a unit against that same
world.

The normative definition — worlds, the single evaluation point E1, the
corollaries the engine relies on, and what is explicitly *not*
provided (stratification, stable models, retraction) — is
[`inference/absent_semantics.md`](../../inference/absent_semantics.md);
the operational narrative is in the
[inference README](../../inference/README.md).

#### Premise order around a guard

Lifting a guard to the boundary is **not** reordering it. Where an
`(absent …)` sits among the positive premises still decides *what it
asks* — a rule of the language, unchanged by S1.21.8:

| written                        | asks                                                              |
|--------------------------------|-------------------------------------------------------------------|
| `(and (absent (P ?x)) (Q ?x))` | is there **no `P` at all**? — `?x` is still free where the guard stands |
| `(and (Q ?x) (absent (P ?x)))` | is there **no `P` for this `?x`**? — `?x` was bound by the preceding `(Q ?x)` |

The compiler records the variables bound by the premises that
*preceded* each guard
([`NafGuard::scope_of`](../../../../ein.rs/crates/ein-infer/src/plan.rs)),
and the boundary projects the completed bindings back down to that set
— so a lifted guard is exactly as strong as it was in place, no more
and no less. Write the binding premise **before** the guard when you
mean "for this `?x`".

A **guard**, `(eq a b)` or `(neq a b)`, is a premise over the relations of
equality and non-equality (M1f
[S1f.5.7](../../../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.7_eq_and_neq_are_relations.md), the user's ruling
of 2026-11-07 on Q-M1e2.26): `(eq x x)` for every value, and `(neq x y)` for
every two different values, computed and never stored. Its arguments are
values: a variable, a name (a run of digits is a name, an integer until M1f
S1f.5.4), or a fact that holds no variable, `(f a)`, the nested fact it spells
as in `(p (f a))`; a form that holds a variable is refused at load (Q-M1f.30),
and `_` and a keyword pair at the parse. **A premise means the same in any
order of its conjuncts**: a guard whose variable a relation premise of its
conjunction binds waits for that premise and is a test after it, so `(and
(neq ?x a) (p ?x))` is `(and (p ?x) (neq ?x a))`. A variable **nothing** in
the conjunction binds is bound by the guard, where it is written, as a relation
premise binds it: `eq` binds it to the other side's value, and `neq`, or `eq`
with both sides free, ranges it over **the program's names**, every constant a
fact writes in an argument, at any depth, and every constant of a rule's
`:match` or `:assert` (Q-M1e2.9 (a), `invariant::program_names`), the list H's
candidates are built from; a name only a query writes is none of them. An
`(absent …)` after such a guard reads its variable, and one before it does not
(Q-M1f.34). Under an `(absent …)`, the enclosing premises before it bind, as
they scope the `absent` itself. From M1e2 S1e2.2.24 (W24, Q-M1e2.18 (i) and
(ii)) until S1f.5.7 the loader refused a form as a guard's argument and a guard
on a variable no premise before it binds; until S1e2.2.24 the unbound variable
compared as a null, so `(and (neq ?x a) (p ?x))` fired for every `?x`, `a`
included, in silence.

### Solve

```lisp
(solve
  [:hrules (<activator> …)]                ; hypothesis-generator activators
  [:hypothesis-relations (<rel> …)]        ; speculate only these
  [:no-hypothesis (<rel> …)]               ; …or never these
  [:expect <claim>])                       ; the expected region
```

**The lattice is stated once** (M1e2
[S1e2.1a.4](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.4_the_language.md)).
The three keywords choose the candidate list, and the candidate list is the
lattice the answer is a region of. Until then they sat on `(query …)`, so two
queries in one file could ask about two different lattices. Now a program has
one lattice and one R, and every query asks about it. That is the user's ruling
(b) of 2026-10-16: *"we solve once and then queries answer questions about
region and about specific solutions"*.

- **At most one per file.** A second is a load error. With none, the lattice is
  the default ladder's, unrestricted.
- **`(solve)` is valid**, as `(config)` is: the default lattice, stated.
- **It takes four keywords**, the three above and `:expect`, and any other is
  refused at the parse (S1f.5.11), as a query's are. On a `(query …)` each of
  the three is refused at the parse too, with a note that names `(solve …)`
  (a load error until M1f S1f.5.5's swap).
- **`solve` lexes as a keyword**, as `query` and `config` do, so it cannot be
  a relation name ([`00_ebnf.md`](00_ebnf.md)).
- `:hrules` is read by the hrule rung, the other two by the candidate filter
  ([`06_reserved_names.md` § Hypothesis / query control](06_reserved_names.md#hypothesis--query-control)).
- **Every name a list holds names something the program has** (M1f S1f.7.8,
  W76), or the program is refused at load, one line per name, on every
  command
  ([`defined_behaviour.md` § 4.20](../../defined_behaviour.md#420-a-solve--list-names-what-the-program-has--m1f-s1f78)):
  - `:hrules (h (a b) (c d) …)`: `h`, and every bare name after it, is an
    hrule the program declares. Each `(a b)` is an activator, which binds
    `h`'s parameters once, so it has as many arguments as `h` has parameters.
    An argument at a parameter that names a relation, one that heads a form
    of the hrule as `?R` does in `(?R ?a ?b)`, is a relation of the program.
    An argument at any other parameter is a value, a type or an object, and
    is not checked.
  - `:hypothesis-relations` and `:no-hypothesis`: every name is a relation of
    the program. `()` names none.
  - **A relation of the program** is a name the program writes as a relation
    outside the three lists: one it declares or states a fact of, `is-a`,
    `eq` and `neq`, the relation of a form in a rule's, an hrule's or a
    query's `:goal`, or a name it passes where a relation's name is held,
    `(bijective color-loc)`. A relation only a rule's `:assert` derives is
    one.

#### The expected region — `(solve … :expect …)`

```lisp
<claim> ::= (region <item>*)              ; R, by its nodes
          | (false)                       ; every node dead
<item>  ::= (model <fact>*)               ; one state of R
          | (merged <to> <from>)          ; a state several commitments reach
          | (owing (model <fact>*) …)     ; R empty, and these alive nodes owe
<to>    ::= (model <fact>*)               ; one of the states listed
<from>  ::= (model <fact>*)               ; its one minimal commitment
          | (or (model <fact>*) …)        ; …or its several
```

The forms the user ruled on 2026-10-16, after reading them on eight real
programs
([`region_examples.md`](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/region_examples.md)).
How a claim reads:

1. **A `(model …)` is a state of R.** Root's facts are part of it.
2. **Naming a relation closes it, across the whole claim.** Every relation the
   claim names positively is listed at its complete extent in every model, so
   `(model)` is a state with none of them. Stored negatives are not closed, and
   a listed `(not …)` is checked for presence.
3. **The models are R's states, one to one**, in any order. Their number is
   the claimed k_s.
4. **`(merged …)` is all or nothing.** A claim with none is about states. A
   claim with one lists every state more than one commitment reaches, each
   with exactly its minimal commitments, and so pins k_c too. `<to>` repeats
   a listed state, and the `<from>` models are **commitments**: the candidates
   committed, with nothing derived and nothing of root's.
5. **`(owing …)`** says R is empty and lists the alive, owing nodes with no
   alive node one fact up. The read-out has to say why R is empty, and this is
   how it says it in a form that reads back (ruled (A), 2026-10-16).
6. **`(false)`** says every node is dead. **`(region)`** says R is empty, of
   either kind.

The words are this keyword's alone. A top-level `(region …)`, `(merged …)` or
`(owing …)` is a load error, because pasted outside `(solve … :expect …)` it
would load as a fact of a new relation and check nothing. `model` is not
reserved, as before.

**Who checks it.** `ein test` does, against R as `ein solve` describes it:
route 1 on a monotone program, and the cube on one the static check flags,
every node a fork of root (what a node is there is open, F21's D2, and the
read-out's `lattice` block warns of it). The claim is never checked against the
walk. A route that stops at its budget, or a flagged program past the
oracle's limit, leaves the claim `NOT CHECKED`. Until
[S1e2.1a.5](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.5_the_read_out.md)'s
switch, bare `ein solve` was the walk and did not check it; the walk was
deleted at S1e2.1c.5 (2026-11-03). `ein_infer::claim::print`
writes any R in this form, and `ein-infer/tests/claim_round_trip.rs` reads
every corpus region it prints back as its own claim.
`tests/region/` has one program per form.

### Query

```lisp
(query
  :goal <expr>                             ; what to find / verify
  [:goal-text "<text>"]                    ; NL headline template (optional)
  [:over all|minimal|maximal]              ; which states (M1e2)
  [:expect <expectation>])                 ; what the answer should be (M1c)
```

**The keywords are an allow-list.** Those four are what a `(query …)` may
carry; anything else is refused **at the parse** since M1f S1f.5.5's swap
(S1f.5.11), the obsolete `:mode` among them, which loaded and was ignored
until then, and was a **load error** from M1c S1c.1.2 to the swap. The three
lattice keywords were here until M1e2 S1e2.1a.4 moved them to
[`(solve …)`](#solve), and on a query each is refused with a note that says
so. It used to be
a silent no-op, which was survivable while a query only asked a question and
became untenable the moment one could carry a *test*: a mistyped `:expect`
that loaded, checked nothing and said nothing is the exact failure the keyword
exists to prevent.

**A file may carry several `(query …)` blocks**, each an independent question
over the same ontology, rules, facts and, since M1e2 S1e2.1a.4, lattice. Before M1c
[S1c.1.2](../../../history/m1c_external_validation/README.md#s1c12--how-a-program-states-what-it-expects)
the **last** one silently won and the rest were discarded at load. `ein solve`
answers every one of them, printing `query <i> of <n>` above each. Since M1e2
S1e2.1a.5 they are read off **one** run — R does not depend on the query, so a
query is a projection of it — and the flags that name a single output path
(`--events`, `--trace`, `--json-summary`, `--dump-answer`) write that one run,
where each query used to be its own walk and the flags were refused.
`(config …)` keeps last-wins: a config is
a *setting*, a query is *content*, and the two want opposite rules.

The engine answers idea 03's three task classes —
[`docs/ideas/03-three-task-classes.md`](../../../../plans/ideas/03-three-task-classes.md)
— *solve* (a unique model), *gaps* (under-determined: many models), and
*contradictions* (inconsistency + provenance). But these are **three answers
to one problem, read off the result `k`** (the count of distinct complete
models: `k = 1` / `k > 1` / `k = 0`) — **not** chosen up front. There is one
`ein solve`, no `--mode` flag, and the engine's goal test is hardwired to
SOLVE. A `:mode` keyword on the query was **obsolete** (vestigial; the
engine ignored it) and is refused at the parse since M1f S1f.5.5's swap, as
any keyword a query does not take is (S1f.5.11's default).

**A `:goal` is a premise, read as a rule's `:match` is** (M1f
[S1f.7.5](../../../../plans/m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/s1f.7.5_a_macro_called_in_a_goal.md),
[S1f.7.11](../../../../plans/m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/s1f.7.11_a_goal_over_a_relation_nothing_makes.md),
[S1f.7.1](../../../../plans/m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/s1f.7.1_a_goals_variable_relation_in_a_nested_pattern.md);
[`defined_behaviour.md` § 4.21](../../defined_behaviour.md#421-a-goal-is-held-to-what-a-rules-match-is-held-to--m1f-s1f75-s1f711-s1f71)).
Three things follow, each refused or done at load, on every command:

- **A macro call in a goal is expanded**, as in a rule: `(query :goal (and
  (is-a ?g Guest) (unknown (paid ?g))))` asks which guest it is unknown of
  whether they paid, and its class lists the expansion's instances, `(model
  (absent (not (paid Bob))) (absent (paid Bob)) (is-a Bob Guest))`. The
  read-out prints the goal as written. A `std.macro` call the program did not
  import, a call of another arity and an expansion a rule would be refused for
  are refused, one line each. A claim lists facts, so a macro call in an
  `:expect` is not expanded and names no relation.
- **A goal asks about relations of the program.** The relation of every
  pattern it writes, at any depth, under `(absent …)` and `(not …)` too, is one
  the program declares, states a fact of, or writes in a rule or an hrule, or
  one a `(solve :hrules …)` activator has its hrule propose; `is-a`, `eq`,
  `neq` and the kernel's `relation` are every program's. A goal over any other
  name, `(payed ?g)` for `paid`, is refused at the pattern: no state holds a
  fact of it, so the goal would have no row, and under `(absent …)` would hold
  of everything. A claim's relations are held to the same list, so a claim
  may list a fact only a rule's `:assert` derives.
- **A goal names its relations.** A relation written as a variable is a
  rule's, whose parameter its activator binds; a query has no activator, so
  `(?R ?x)` is refused in a goal at the top level (M1f S1f.5.21) and inside a
  pattern, `(says (?R ?g))` (S1f.7.1).

**`:expect` — what the answer should be (M1c
[S1c.1.2](../../../history/m1c_external_validation/README.md#s1c12--how-a-program-states-what-it-expects)).**
A query may state its own answer, and the engine checks it: `ein solve` exits
**1** when the claim is false, and prints what disagreed. Three shapes, and the
verdict is *implied* by which one is used rather than asserted beside it:

```lisp
:expect (model <fact>*)                 ; exactly one model — Solution
:expect (or (model …) (model …) …)      ; that SET of models — k is the count
:expect (false)                         ; Contradiction — the kernel's own ⊥
```

Three rules, and the third is the one that gives the form its teeth:

1. **The goal's relations are mandatory.** An expectation that does not pin
   what the query asked is not an expectation, and does not load.
2. **More is allowed.** Any ground fact may be listed, including a `(not …)`,
   so a test can pin the consequence at a distance rather than restating the
   rule it is about.
3. **Naming a relation closes it.** If `:expect` mentions `pet-loc` at all,
   the listed `pet-loc` facts are that relation's *complete* extent in the
   model. Relations it never mentions are unconstrained.

Rule 3 sits between the two useless extremes: a per-fact assertion cannot
catch a **surplus** fact, and a whole-state golden pins 250 facts of `is-a*`
and activator noise that no test means to assert. Two clarifications it needs:
**stored negatives are not closed** — closing `pet-loc` says nothing about the
extent of `(not (pet-loc …))`, and a listed `(not …)` is checked for presence
like any other fact — and `(or …)` compares model **sets**, never sequences,
because the order a search finds models in is not observable.

**A goal may ask about `eq` and `neq`** (M1f
[S1f.5.7](../../../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.7_eq_and_neq_are_relations.md)), the relations
of equality and non-equality: a guard in a goal reads as it does in a premise,
so `(query :goal (neq w ?y))` answers a row for each name of the program but
`w`, `(eq w ?y)` answers `?y = w`, `(neq a b)` holds and `(eq a b)` has no
row, and a class lists the guard's instances beside the goal's other atoms,
`(model (neq w a) (p a))`. A claim may name either, and naming one closes it
over the class, which holds the goal's instances only. *the goal holds: it
has no variable* is printed only for a goal with no variable; one whose every
variable is an `(absent …)`'s own prints *the goal holds: it binds no
variable*. Until S1f.5.7 a guard's free variable bound nothing, `(neq w ?y)`
holding with no row bound, and a claim naming `eq` or `neq` was refused at
load. A region claim, `(solve … :expect (region …))`, lists a state's stored
facts, and a state stores neither, so one that names `eq` or `neq` is refused
at load.

The value is checked at *load*: an expectation that is not one of the three
shapes, that names a relation the program does not have (`eq` and `neq`
excepted, above; a relation only a rule's `:assert` derives is the program's
since M1f S1f.7.11), that omits the
goal's relations, or that contains a `?var` (an expectation is an answer, not
a pattern) is a load error. `examples/features/10_expect.ein` is the worked
fixture and `examples/broken/load/expect_*.ein` are the four refusals.

**`ein test` is the runner** (M1c
[S1c.1.3](../../../history/m1c_external_validation/README.md#s1c13--ein-test)).
`ein solve` checks an `:expect` because ignoring one would be worse than not
having the keyword; `ein test <file|dir>…` exists so that a corpus of them is a
**status code** rather than something to read:

```sh
ein test examples/features/          # every .ein under it, sorted
ein test puzzle.ein -v               # …and say what held, with the verdict and k
```

Three differences from `solve`, and each is the form's semantics rather than a
preference:

- **It runs the question each claim is about.** A claim is about a set — all
  of R, a line, a query's classes over one — so a run that asked a part of it
  (`-n`) confirms nothing. Since M1e2 S1e2.1a.5 `ein test` runs, per file, the
  mode the file's claims are about (all of R when any is), and has no `-n`
  and no line flag; deciding is the behaviour.
- **It runs only what the expectations need.** A query with no `:expect` states
  nothing and is never solved, so a directory of programs costs one load each
  plus a search for each claim.
- **Exit 1 means a claim is false**, so a *load* error takes **2** — the code
  that already means "this run is not a verdict". A runner that cannot tell a
  broken file from a false claim is the one failure a test runner must not
  have, and for the same reason a selection that checked nothing exits 2 as
  well.

A claim a run did not decide has one more spelling under either command:
**`NOT CHECKED`**, which is not a pass. Under `test` it takes the same exit
code a false claim does, and the only thing that can still produce it is a
budget, `--region-budget` or the cube's `--region-limit` — a partial R is a
lower bound, not R. Under `solve` it is a `note:` on stderr and the run exits
0, because the run answered the question it was asked (M1e2 S1e2.1a.5; until
then the walk's `--max-set-size` was the cap, and `solve` exited 1).

**`:goal-text` headline template (optional).** A `:goal-text "<tmpl>"` string
renders the one-line natural-language answer for `ein solve`'s result table,
referencing the goal's **own variables** by name — the same `{?var}` engine as
rule `:why`, bound from the solution. A reference is the variable as the
program writes it after its `?`, up to the first `}`, so `{?isR*}` names
`?isR*` and `{?_x_}` names `?_x_` (M1f S1f.5.4's T1f.5.4.5, W37; until then a
reference took a letter or digit and then letters, digits, `_` and `-`, and
printed any other as written), and a template naming a variable that holds a
`}`, which no reference can reach, is refused at load
([`defined_behaviour.md` § 4.11](../../defined_behaviour.md#411-a-template-reference-no-reference-can-reach--m1f-t1f545)). The vocabulary lives entirely in the
puzzle; nothing is hardcoded in the renderer. The value is a **single** string
literal (ein-lang does not concatenate adjacent strings). Example (the Zebra
goal binds `?who_water` / `?h_zebra` / …):

```lisp
(query
  :goal (and (drink-loc Water ?h_water) (nation-loc ?who_water ?h_water)
             (pet-loc Zebra ?h_zebra)   (nation-loc ?who_zebra ?h_zebra))
  :goal-text "The {?who_water} drinks water in {?h_water}; the {?who_zebra} owns zebra in {?h_zebra}")
;;  →  "The Norwegian drinks water in House-1; the Japanese owns zebra in House-5"
```

Every text a program writes, where each is printed and what a fact reads as
where it has none: [`09_natural_language.md`](09_natural_language.md).
The division of labour: a `(relation … :why)` template renders an individual
*fact* to text (a state file's comment after the fact, `--trace`'s *Derives*
line); the `(query … :goal-text)` template renders the *headline result* from
the goal bindings, a projection class's sentence on the read-out.

### Import

```lisp
(import onto)                        ; whole module, every declared name prefixed onto.
(import onto :as o)                  ; the same, prefixed o.
(import onto :symbols (is-a things)) ; flat: the listed names, their closure, and their facts
```

A module is a `.ein` file: `std.x` resolves under the
[stdlib](../../../../stdlib/README.md#importing), any other name against the
importing file's directory. An import is replaced, before the loader runs, by
forms of the module, and the three shapes differ in which forms and under
which names. `:as` and `:symbols` are mutually exclusive.

**A flat import, `:symbols`, brings declarations and facts** (M1f
[S1f.6.11](../../../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.11_importing_facts.md),
the user's ruling of 2026-11-11, *"importing facts is mandatory"*; W88). The
selection is a set of names, the **closure** of the list: every listed name,
and every name a selected declaration writes, followed through the module's
declarations. Of the module's top-level forms it keeps, unrenamed and in the
module's order:

- every **declaration** (`relation`, `rule`, `hrule`, `macro`) whose name is
  in the closure;
- every **fact** whose relation is in the closure, with its `:source`. A
  stored negation, `(not (likes Milk Tea))`, is a fact of `likes`.

So `(import onto :symbols (is-a))` brings `is-a`'s declaration and the
module's `is-a` facts, and `(import onto :symbols (things))`, where the rule
`things` reads `is-a`, brings the rule, the declaration and the same facts. A
relation needs no declaration, so a listed name may be a relation the module
only states facts of. A listed name the module neither declares nor states a
fact of is refused, *not provided by the module*.

What a flat import does not bring: a declaration or a fact no name in the
closure reaches, so a rule nobody names stays behind even where the imported
facts would fire it; the names written *inside* a fact, which the closure
does not follow (`(owns Ann cat)` brings no declaration of `cat`); and the
module's `(query …)`, `(solve …)` and `(config …)`, which are the module's
own as a program. `(__closed__ r)` is a fact of `__closed__` like another: a
library that closes its own relations has the marker listed, or writes it in
a rule the selection names.

**One line where a selection leaves a relation's rules behind** (M1f
[S1f.6.25](../../../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.25_the_section_j_fixes.md),
E-20). The closure brings the *declaration* of every relation a kept rule
writes or reads, and not the rules that derive it: a theory built on a
theory, imported by its entry, brings the lower theory's relations and none
of its rules unless its entry names the lower one's. That is the rule above
and it is not changed. Where it happens the loader says so, on stderr, on
every command that loads the program, once:

```text
note: p.ein:1:9: (import upper :symbols (upper)) brings the declaration of double-of and no rule that derives it: upper has such a rule, its own or one it imports, and the selection names none — a selection brings the names it lists and what their declarations write, so list the rule, or the entry that names it, if double-of is to be derived
```

It is a diagnostic, never a refusal: the program loads and answers as it
would without the line, and no exit code moves. It is printed when all four
hold: the relation's declaration came through the closure, **its name not
listed** (a listed relation is asked for on purpose); the module, with what
it imports, has a rule or an hrule whose `:assert` concludes a fact of it;
the selection keeps none of them; and the resolved program holds no rule that
concludes one either, another import's or its own. So a library of facts,
`(import std.digits :symbols (fa nz __closed__))`, prints nothing, having no
rule. The line and its condition are `ein-cli`'s
[`import_left_behind.rs`](../../../../ein.rs/crates/ein-cli/tests/import_left_behind.rs);
`defined_behaviour.md` § 4.25 states it.

**An imported fact is a fact of the program.** It is stored as if written
where the import stands: background, or given where it carries a `:source`.
Its constants are names of the program, so they are among the objects the
blind enumerator proposes facts about
([`solution_semantics.md` § 1.2](../../inference/solution_semantics.md#12-names));
a program that wants no guesses about a library's relation closes it. A fact
that arrives twice, through a diamond of flat imports or one module imported
flat twice, is one fact. The same fact stated with two `:source`s keeps its
first statement and loses the second in silence, as it does written twice in
one file
([F24](../../../../plans/followups/f24_written_provenance.md)'s question).

**A qualified import, bare or `:as`, brings every declaration and every
fact** and prefixes each name the module *declares*, wherever it stands, the
relation heading a fact included: the module's `(is-a Tea Drink)` is `(o.is-a Tea Drink)`, a relation
apart from the program's own `is-a`. A relation the module does not declare
and every constant are left as written. So a library of facts, an ontology or
a table, is imported flat, where its facts join the program's vocabulary; a
qualified import keeps a library's theory apart under its own names.

**No import brings a module's `(query …)`, `(solve …)`, `(config …)` or
`(trace …)`** (M1f
[S1f.6.25](../../../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.25_the_section_j_fixes.md),
W90). They are what a *program* asks, how it is solved and how it is
configured, and a module that is also a program of its own keeps them to
itself: the importer states its own, or none. Until that stage a qualified
import kept all of them, so the importer answered the module's queries,
renamed, was solved under the module's lattice and configured by the module's
flags, and two qualified imports of a module holding a `(solve …)` were
refused, *2 (solve …) forms — a program states its lattice once*.

Every cell of this is `ein-ir`'s
[`imports_semantics.rs`](../../../../ein.rs/crates/ein-ir/tests/imports_semantics.rs)
(`what_each_import_form_brings_of_a_module`), and `ein-cli`'s
[`import_facts.rs`](../../../../ein.rs/crates/ein-cli/tests/import_facts.rs)
holds that a flat import of facts answers, on every command that loads a
program, as the same forms written in place.

### Trace

```lisp
(trace
  (step <id> :rule <name>                  ; engine derivation step
             :using (<premise-ids>)        ;   or (and <id>+)
             :derives <fact>
             [:source "<text>" | :assumes <fact>]))
```

A trace holds `step` events alone since M1f S1f.5.5's swap (2026-11-09, the
paper grammar's default 8): `Step ::= '(' 'step' Name StepClauses ')'`, its
keywords `:rule`, `:using`, `:derives`, `:source` and `:assumes` (S1f.5.11's
list). **Until the swap** the block above also showed four events the
hand-written grammar kept for the renderer
([Q21](../../../../plans/open_questions.md#q21--ir--dot-structural-isomorphism)),
which no `.ein` file wrote and nothing in the engine emitted: `(branch-open
<id> :on <expr> :choices (<sub-ids>))`, `(branch-close <id> :choose
<sub-id>)`, `(contradiction <id> :using (<step-ids>) :assumption <step-id>)`
and `(symmetry-class <id> :over (<entities>) :note <STRING>)`. Each is a
parse error since, at its head, a name where a trace wants `step`.

Per [Q21](../../../../plans/open_questions.md#q21--ir--dot-structural-isomorphism),
`(trace …)` is the **same IR** as input — same parser, same AST,
same dumper. The engine can reason about its own traces; rules can
match `(step …)` forms ([TMS/ATMS analogue](../../../lib/09-cognitive-architectures-neurosymbolic.md)).
Per [Q18](../../../../plans/open_questions.md#q18--provenance-granularity)
each derived edge's provenance tuple `(rule, premise_edges, source)`
is literally a `(step …)` form — provenance and trace are the same
data structure under different views.

---

## See also

- [`02_patterns.md`](02_patterns.md) — the pattern sub-language used
  inside `:match` / `:assert`.
- [`03_examples.md`](03_examples.md) — worked Zebra fragments.
- [`04_dot_rendering.md`](04_dot_rendering.md) — DOT rendering of
  each form.
- [`../01-ein-graph/`](../01-ein-graph/) — what these forms *mean*
  in graph terms.
- [`../02-data-model/`](../02-data-model/) — the entities the
  loader produces from these forms.
