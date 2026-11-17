# Syntax — the lexical rules, every form, and the patterns

A program is text: a flat sequence of S-expression forms
([Q3](../../../decisions/language/Q3-ein-is-an-s-expression-language.md)), each classified by its head. This page is
the reference for every form a program writes, the keywords each takes and what the loader holds it to; § 3 is the
pattern language a rule's `:match` and `:assert` are written in. What parses, exactly, is
[`01-grammar.md`](01-grammar.md), and where this page and its two blocks disagree, the blocks are right. What each form
*means* is its concept page under [`../03-syntax-and-semantics/`](../03-syntax-and-semantics/README.md); the reserved words
are [`03-reserved-names.md`](03-reserved-names.md), and every refusal's line is [`07-diagnostics.md`](07-diagnostics.md).

## 1. Lexical rules

| terminal | regex | examples | role |
|---|---|---|---|
| `NAME` | ``[A-Za-z0-9_.\-$%^*+=<>!&@~/'`\[\]{}]+`` | `has-color`, `House-1`, `is-a*`, `7`, `-7`, `1..5`, `=`, `<=`, `a/b`, `f'` | an atom: a value, a relation, a rule, a type, a step. **The name rule**: a run of ASCII name characters, each name printed as written and reloading to itself, so `7` and `07` are two names. No character has a meaning of its own, `*` among them. A reserved word is never a name, and a position word is one wherever a name goes ([`01-grammar.md` § 1](01-grammar.md#1-lexical-grammar), fact 1). A token ends only at a space, a paren, a comment or the end of input, so `a?x` and `a,b` are each one token that is not a name, refused |
| `VAR` | ``\?[A-Za-z0-9_.\-$%^*+=<>!&@~/'`\[\]{}]+`` | `?a`, `?house`, `?T`, `?R*`, `?_x`, `?a.b` | a variable: `?` and a name's characters, bound by a `:match` and reused in an `:assert` |
| `KEYWORD` | `:` and a name's characters | `:match`, `:rule`, `:source` | a form's clause marker, followed by its value, but `(solve …)`'s `:no-search`, a flag. **Each form takes a fixed set** ([below](#the-keywords-each-form-takes)), and any other keyword, `:where`, `:cardinality` and `:mode` among them, is refused at the parse, as is any keyword pair inside a term or a nested fact ([Q-M1e2.28](../../../decisions/language/Q-M1e2.28-each-form-takes-a-closed-list-of-keywords.md)) |
| `TEXT` | `"…"`, the escapes `\n \t \r \" \\` and `\u{…}` | `"condition (10)"`, `"{?r} is transitive"`, `"Жилец {?who}"` | **natural-language text, and nothing else**: the value of `:why` (a template), `:source` (a source sentence) or `:goal-text` (a template), in any Unicode, carried byte for byte by every surface that prints it ([Q-M1e2.23](../../../decisions/language/Q-M1e2.23-a-string-is-natural-language-text-never-a-value.md)). Written anywhere else, as a fact's or a rule's argument, a name or a keyword's value, it is a parse error at the string with a note saying so, and the name written bare is what it meant: `(p "1")` is refused, `(p 1)` is the fact |

**There is no `_`, no number and no range.** A bare `_` and `?_` are refused wherever either is written, with the
note *_ is reserved and names nothing: where any value will do, write a variable used once*
([Q-M1e2.22](../../../decisions/language/Q-M1e2.22-a-bare-is-reserved-and-refused.md)); a variable used once is
what to write where any value will do, and `_` anywhere else in a word is a name character, so `_a_in_b_` is a name
and `?_x` a variable. Ein has no numbers and no ranges
([Q-M1e2.24](../../../decisions/language/Q-M1e2.24-ein-has-no-numbers-and-no-ranges.md)): `1..5`, `7` and `07` are
three names, and arithmetic is a theory in the standard library ([`04-stdlib.md`](04-stdlib.md)).

The regexes above are the readable form; [`01-grammar.md` § 1](01-grammar.md#1-lexical-grammar) is the exact one, with the
order that settles a tie between a reserved word and `NAME` and the start condition that ends a token at a separator.

**Comments**, as SMT-LIB writes them: `; line` to the end of the line, and `#| block |#`, not nesting.

### What a program's text may hold

A program is UTF-8 text, or a `.einb` container, which stores one, and **a character that is not ASCII is written
only inside a string literal, after `:why`, `:source` or `:goal-text`, or in a comment**
([Q-M1f.65](../../../decisions/language/Q-M1f.65-a-comment-may-hold-any-character.md)). Anywhere else, in a name, a
variable, a keyword, between two tokens or in place of a paren, every command refuses the program with one line that
names the character, its code point and where Unicode is allowed:

```text
x.ein:7:7: 'é' (U+00E9) is not ASCII: Unicode is allowed only inside a string literal (after :why, :source or :goal-text) or in a comment
x.ein:7:3: '\u{a0}' (U+00A0) is not ASCII: Unicode is allowed only inside a string literal (after :why, :source or :goal-text) or in a comment
```

A character a reader cannot see where it stands, a space, a control, a zero-width or bidirectional format character,
a combining mark or a variation selector, is quoted as the escape a string writes it with, as the second line quotes a
no-break space ([Q-M1f.92](../../../decisions/diagnostics/Q-M1f.92-a-character-a-reader-cannot-see-is-quoted-as.md));
the line names the file's first such character
([Q-M1f.95](../../../decisions/diagnostics/Q-M1f.95-a-file-is-refused-at-its-first-character-that-is.md)), a
byte-order mark among them ([Q-M1f.96](../../../decisions/language/Q-M1f.96-a-byte-order-mark-is-refused-as-any-character.md)).
A separator is ASCII's whitespace, `\t \n \v \f \r` and the space, and `\x1c` to `\x1f`. A byte that is not UTF-8 is
refused before the text is read:

```text
x.ein:2:4: byte 0xff is not UTF-8: a program is UTF-8 text, or a .einb container
```

**An imported module is read as the program a command is given is read**
([Q-M1f.91](../../../decisions/imports/Q-M1f.91-an-imported-module-is-ein-text-and-a-einb.md)): a module that is not
UTF-8, one that cannot be read and one that is a directory are refused with the line their path gives as a program,
behind `kb load error: ` and naming the path the import resolves to; a module that is a `.einb` container is refused
with a line of its own; a module that holds a character that is not ASCII outside a string is refused at it. A module
that does not exist is `kb load error: (import m) — module not found at m.ein (None)`.

**Naming convention**: hyphenated lower case for relations and rules (`has-color`, `triangle-composition`), and
PascalCase or `Foo-N` for types and instances (`Person`, `House-1`, `Norwegian`). A convention only; the grammar
takes either.

## 2. Top-level forms

A program is a **flat sequence of forms**. Each top-level form is classified by its **head** against the closed set of
declarators, and **anything whose head is not a declarator is a fact**. The set is
[`03-reserved-names.md`](03-reserved-names.md)'s.

| head | role | shape |
|---|---|---|
| `relation` | declare a relation and its signature | `(relation <name> <T1> <T2> … :why "…")` |
| `rule` | a saturation rule | `(rule <name> (<param-vars>*) :match … :assert … :why "…")` |
| `hrule` | a hypothesis rule: what may be guessed | the shape of a `rule` |
| `query` | a question about the answer, and with `:expect` what the answer should be | `(query :goal … [:goal-text …] [:over …] [:expect …])`; several per file |
| `solve` | the problem's lattice, stated once, and the expected region | `(solve [:hrules …] [:hypothesis-relations …] [:no-hypothesis …] [:no-search] [:expect …])`; at most one per file |
| `config` | the two solver flags | `(config [:lattice-sanity-check true\|false] [:record-alternative-justifications true\|false])` |
| `macro` | a pattern alias, expanded at load | `(macro <name> (<param-vars>+) <body> :why "…")` |
| `import` | bring a module into the program ([§ Import](#import)) | `(import <module> [:as <alias> \| :symbols (<name>+)])` |
| `trace` | a derivation log, written by the engine, not by an author | `(trace (step …)*)` |
| *anything else* | **a fact** | `(not <fact> …)`, or `(<NAME> <args>* [:source "…"] [:rule <name> :using (<fact>+)])` |

### The keywords each form takes

Each form takes the keywords its production in the grammar lists, and no other; a term and a nested fact take none
([Q-M1e2.28](../../../decisions/language/Q-M1e2.28-each-form-takes-a-closed-list-of-keywords.md)). Any other
keyword, or a keyword pair inside a term, is refused at the parse, from every command that reads the program, with the
file, line and column, *unexpected `:where`: a keyword this form does not take*, and the repairs. Which keywords a form
requires, and that each appears once, is the loader's.

| form | keyword | holds | required |
|---|---|---|---|
| `relation` | `:why` | a string: the template that renders the relation's facts | at load, by default ([§ Relation declarator](#relation-declarator)) |
| `rule`, `hrule` | `:match` | a premise: what the rule reads | yes |
| | `:assert` | a conclusion: what it derives, or proposes | yes |
| | `:why` | a string: the template that explains a firing | at load, by default |
| a fact, negated or not | `:source` | a string: the fact is given, and where it is stated | no |
| | `:rule`, `:using` | a rule's name and its premises: the fact is derived | no |
| `query` | `:goal` | a premise: the pattern asked about | yes |
| | `:goal-text` | a string: the headline template | no |
| | `:over` | `all`, `minimal` or `maximal`: which states | no |
| | `:expect` | a claim: the answer stated | no |
| `solve` | `:hrules`, `:hypothesis-relations`, `:no-hypothesis` | what may be guessed: the lattice | no |
| | `:no-search` | nothing: a flag, the lattice root alone | no |
| | `:expect` | a claim about the region | no |
| `config` | `:lattice-sanity-check`, `:record-alternative-justifications` | `true` or `false` ([`06-config.md`](06-config.md)) | no |
| `import` | `:symbols` | the names the file writes short | no |
| | `:as` | an alias | no |
| `macro` | `:why` | a string, after the body | at load, by default |
| `trace` | none; its `step` takes `:rule`, `:using`, `:derives`, `:source`, `:assumes` | one derivation step | no |
| a term, a nested fact | none | | |

A `:why` is required on every relation, rule, hrule and macro; `--no-require-why` loads a program without one
([Q-M1f.62](../../../decisions/cli/Q-M1f.62-no-require-why-loads-a-program-without-a-why.md)), and changes nothing
else.

### Where a fact came from — its provenance annotation

A fact's origin is read off the annotation it carries, and nothing else
([`../03-syntax-and-semantics/03-facts.md`](../03-syntax-and-semantics/03-facts.md)):

| the fact carries… | it is… | meaning |
|---|---|---|
| `:rule` / `:using` | **derived** | the engine produced it, a rule firing |
| `:source "(N)"` | **given** | an explicit problem statement |
| neither | **background** | an implicit assumption: a schema, an `is-a` enumeration, a property tag |

The distinction is **presentation only**: the engine treats every fact alike, and a fact's identity is its relation
and its arguments, whatever it carries. Write the annotation that is true: a numbered condition gets `:source`, a
property tag gets nothing.

### Relation declarator

```text
(relation <name> [<T1> <T2> …]           ; the signature: zero or more type names
  :why "<template>")                      ; the one keyword a relation takes
```

`relation` declares a relation and its signature. The loader also stores each declaration as an ordinary fact,
`(relation R T1 T2 …)`, its **mirror**, so that a rule can read a signature with a `(relation ?R ?A ?B)` pattern.
That is why `relation` is a position word: a name wherever a name goes, `(relation ?R ?A ?B)` a pattern, and as a
form's head the declarator, so `(relation)` and the wrapped form `(relation R (T1 T2))` are parse errors.

**A name and zero or more type names.** `(relation adult Person)` is a legal **unary** declaration, and
`(relation opaque)` a legal **bare** one, a relation with no argument
([Q-M1f.201](../../../decisions/language/Q-M1f.201-a-bare-relation-r-declares-no-argument.md)): `(opaque)` is its one
form.

**A declaration's count is held**
([Q-M1f.315](../../../decisions/language/Q-M1f.315-a-form-of-a-declared-relation-has-as-many.md)). The number of type
names is the number of arguments every form of the relation has, and the loader refuses a program that writes one
with another count, a fact, a fact inside a fact at any depth, a pattern or a conclusion of a rule or an hrule, a
goal's pattern and a fact a claim lists alike
([Q-M1f.204](../../../decisions/language/Q-M1f.204-the-declared-count-is-held-in-a-using-a-claim.md)), one line a
form:

```text
kb load error: t.ein:2:2: the fact (likes Bob) gives likes 1 argument, and (relation likes Person Drink) declares 2 — a form headed by a declared relation has as many arguments as its declaration has types
```

A keyword pair is no argument. A relation nothing declares has no count and is not held, a relation written as a
variable, `(?R ?a ?b)`, is not read
([Q-M1f.202](../../../decisions/language/Q-M1f.202-a-relation-written-as-a-variable-is-held-to-no.md)), and the type
names are not checked: `(likes Tea Ann)` loads.

**The membership fact.** A pattern matches facts of its own count, so `(relation ?R ?A ?B)` sees only binary
declarations. Every declaration therefore also stores `(relation R)`, the pattern for *is `?R` a declared relation?*
at any count:

```lisp
(relation likes Person Drink :why "{?1} likes {?2}")
;; stores (relation likes Person Drink), the mirror,
;;    and (relation likes), the membership fact
```

For a bare declaration the two coincide, so one fact is stored. A relation no form declares, a property tag such as
`symmetric`, gets neither from the loader. A rule may conclude the membership fact too: `(rule r () :match (p ?x)
:assert (relation ?x) …)` over `(p a)` derives `(relation a :rule r)`, a fact like any other, with no declaration
behind it. So `(relation ?R)` matches every declared relation and every name a rule has concluded it of.

**What a file writes, stated.** As it stores each declaration's mirror, the loader states each fact nested in a fact
a file writes, at any depth, as `(written T)`, and each fact a rule's or an hrule's conclusion nests that holds no
variable ([Q-M1f.275](../../../decisions/language/Q-M1f.275-written-states-every-fact-a-file-nests.md)); a stored
negation's denied fact is no term, and what it nests is. A rule reads it, `:match (written (n ?p ?a))`, which is how a
module checks every term of its own a file writes, wherever it sits; no program writes it
([Q-M1f.280](../../../decisions/language/Q-M1f.280-a-program-does-not-write-written.md),
[`03-reserved-names.md` § `written`](03-reserved-names.md#written--what-a-file-writes-stated-by-the-loader)):

```lisp
(p (q a (k b)))
;; stores (p (q a (k b))), and states (written (q a (k b))) and (written (k b))
```

**The `:why` template** turns a fact of the relation into natural-language text, its *statement*, written after each
fact of the relation in a `--dump-answer` state file and after each one `--trace` derives. It refers to the fact's
argument **slots by position**: `{?1}` is the first argument, `{?2}` the second; a rule's and a goal's template refer
to variables by name
([Q31](../../../decisions/language/Q31-a-why-template-refers-to-a-variable-as-var.md)). Every text a program writes,
where each is printed and what a fact reads as where it has none, is
[`../03-syntax-and-semantics/14-natural-language.md`](../03-syntax-and-semantics/14-natural-language.md).

```lisp
(relation drink-loc Drink House :why "{?1} is drunk in {?2}")
;; (drink-loc Water House-1)  →  "Water is drunk in House-1"
```

A schema and its implicit assumptions, all flat forms; the property tags and the enumerations carry no annotation,
so they read as background:

```lisp
(is-a House Attribute) (is-a Color Attribute)
(relation co-located Attribute Attribute :why "{?1} and {?2} are in the same house")
(is-a Norwegian Nationality)
(is-a House-1 House)
(symmetric  co-located)
(transitive co-located)
(right-of House-2 House-1 :source "condition (1)")  ; "five in a row": structural, and condition (1)
```

#### What the signature means — userspace types, kernel structure

Ein imposes **no type system**
([Q23](../../../decisions/language/Q23-a-premise-types-a-variable-and-no-entity-has.md)), and yet a signature is not
inert. It plays two roles that never mix: rules read the signature's *content* as types, and the engine reads its
*shape* as structure.

**As types, in rules only.** Everything type-like keys off the mirror fact `(relation ?R ?A ?B)` in a rule:
`std.bijection`'s type check, `std.algebra`'s converse domain and range check, `std.typing`'s hierarchy
([`04-stdlib.md`](04-stdlib.md)). The engine never resolves a type name to anything: a signature holds opaque names.

**As structure, in the engine**, in three places and no other, each about what may be guessed
([`../03-syntax-and-semantics/09-hypotheses.md`](../03-syntax-and-semantics/09-hypotheses.md)):

| the signature is… | so the blind enumerator… |
|---|---|
| **not empty** | may guess the relation's facts: a declaration with no type is never guessed, so write `(relation R T)`, `T` any name, for a guessable relation with no meaningful type |
| **one or two types long** | proposes one candidate per object for a unary relation, and one per pair of objects for a binary one; a relation of three arguments or more loads, stores facts and saturates, and is never guessed |
| **its type names** | never proposes a fact *about* a name a signature declares as a type ([Q-M1e.12](../../../decisions/hypotheses/Q-M1e.12-the-blind-enumerator-is-untyped-and-a-type-it.md)) |

The `:why` template and the signature column of `ein saturate`'s relation table read a declaration too, as
presentation.

### Facts — `(NAME args*)`, the flat default

A fact is any top-level form whose head is **not** a declarator:

```text
(not <fact> [:source "<text>"])           ; a stored negative: one fact, the head `not` reserved
(<name> <arg>* [:source "<text>"])        ; a relation instance, an enumeration, a property tag
```

| kind | example | semantics |
|---|---|---|
| **relation instance** | `(co-located Englishman Red :source "(2)")` | a relation holds between these arguments |
| **negative** | `(not (drinks Spaniard Coffee) :source "(?)")` | the wrapped fact does *not* hold |
| **a fact of `=`** | `(= Red Colour-1)` | an ordinary fact of the relation named `=` ([§ 3](#is-a-fact-head-not-a-unifier)) |

An argument is a name or a fact, `(hypothesis (co-located Norwegian House-2))`
([Q40](../../../decisions/language/Q40-a-fact-may-hold-a-fact-as-an-argument.md)). **A value is a name, or a fact a
state could hold**, an atom of an ordinary relation or its negation
([Q-M1f.113](../../../decisions/language/Q-M1f.113-a-value-is-a-name-or-a-fact-a-state-could-hold.md)), so:

- `not` is shape-pinned: `(not <fact>)`, one fact; a wrong count is a parse error.
- `and`, `or`, `neq` and `absent` are premises of a rule's `:match`, never a fact's head, and never a value. A guard is
  a premise, `(neq ?a ?c)`; there is no `:where` clause.
- **`eq` and `neq` are relations whose facts are computed**
  ([Q-M1e2.26](../../../decisions/language/Q-M1e2.26-eq-and-neq-are-the-relations-of-equality-and-non.md)), so no
  fact changes them. `(neq …)` is no fact the grammar takes; a written `(eq a b)`, or `(not (eq a b))`, is checked as
  a conclusion with no premise, a true one not stored and a false one `(false)`, written where it stands
  ([Q-M1f.33](../../../decisions/language/Q-M1f.33-a-written-fact-of-eq-is-a-conclusion-with-no.md)); one at any
  other count, `(eq a)`, is refused at load
  ([Q-M1f.47](../../../decisions/language/Q-M1f.47-a-written-fact-of-eq-at-another-arity-than-two.md)). `(eq a b)`
  is written only as a form of its own, never inside another, `(q (eq a b))`
  ([Q-M1f.124](../../../decisions/language/Q-M1f.124-eq-a-b-is-written-only-as-a-form-of-its-own.md)).
- No fact or value is headed `absent`, which is negation as failure, a premise's test; the stored negation of a fact is
  `(not …)` ([Q-M1f.122](../../../decisions/language/Q-M1f.122-absent-heads-a-premise-and-a-claims-class-atom.md)).
- `false` takes no argument, `(false)` alone, as a fact and as a value
  ([Q-M1f.123](../../../decisions/language/Q-M1f.123-false-with-no-argument-is-a-value.md)).
- The declarators `rule`, `hrule`, `query`, `solve`, `config`, `macro`, `import` and `trace` are reserved words, never
  a fact's head.

Everything else, `instance`, `type`, `symmetric`, `co-located`, `lives-in`, …, is an ordinary `(NAME value*)` fact:
introduce a relation by declaring it and stating its facts. A fact that activates a rule, `(symmetric co-located)`,
`(implies right-of next-to)`, carries no annotation and reads as background: the puzzle never says *co-located is
symmetric*; it is the relation's context. Pairwise distinctness in a category is no primitive either: a rule derives it
from the `(is-a X T)` facts.

```lisp
(lives-in Norwegian House-1 :source "condition (10)")
(co-located Englishman Red  :source "condition (2)")
(symmetric  co-located)                              ; a property tag: background
```

**A fact is not a macro's call.** A `(macro …)` is called in a rule's or an hrule's `:match` and `:assert` and in a
query's `:goal`, where the loader replaces the call by the macro's body; a fact is not expanded. So a fact headed by a
macro's name, `(settled Ann)` beside `(macro settled (?g) (paid ?g))`, is refused at load, and so is one that holds the
call at any depth, `(says (settled Ann))`, `(not (settled Bob))`, one a claim lists, and a relation declared under a
macro's name ([Q-M1f.131](../../../decisions/language/Q-M1f.131-no-relation-is-declared-under-a-macros-name.md),
[Q-M1f.133](../../../decisions/language/Q-M1f.133-a-macros-call-inside-a-claims-fact-or-a-using-is.md)); the same
holds of `std.macro`'s `unknown` and `forall`, imported or not. Write the fact itself, `(paid Ann)`. A macro's name
written as a value, `(word settled)`, is a name.

### Derived facts — engine working memory

The engine writes a fact it derives as a flat form annotated with `:rule`, the rule that fired, and `:using`, the
premises it read, in place of `:source`, so that it reads back as a derivation:

```text
(<name> <arg>* :rule <rule-name> :using (<premise-fact>+))
(not <fact>    :rule <rule-name> :using (<premise-fact>+))
```

```lisp
(co-located Blue House-2 :rule square-fwd :using ((co-located Ivory House-1) (right-of House-2 House-1)))
(not (co-located Norwegian House-2) :rule type-exclusivity :using ((co-located Norwegian House-1)))
```

`:using` is a list of facts, each resolved by the loader to the premise the derivation names. A program an author
writes has none of these; `ein saturate --dump` writes `:rule` with no `:using`, and a `--dump-answer` state file
writes the provenance as a comment.

### Rules

```text
(rule <name> (<param-vars>*)      ; the parameter list: required, () for a rule with none
  :match <premise>                ; what it reads (§ 3)
  :assert <conclusion>            ; what it derives
  :why "<template>")              ; why, for the trace
```

A rule is a top-level `(rule …)` form, and an hrule the same shape headed `hrule`. Each has one `:match` and one
`:assert` ([Q4](../../../decisions/language/Q4-a-rule-is-a-pattern-a-conclusion-and-a-reason.md)). **A rule has no
priority**: the saturation agenda fires first the rules whose conclusion leads to `(false)`, then the rest, in the
order they are written and their facts arrive within each half
([Q-M1f.313](../../../decisions/engine/Q-M1f.313-the-agenda-fires-first-what-leads-to-false.md)). On a monotone
program that order decides no answer; it orders the trace, and the derivation a core is explained from.

**The parameter list** is required:

| parameters | the rule | how it fires |
|---|---|---|
| **not empty**, `(?p1 ?p2 …)` | relation-polymorphic | only where a fact headed by the rule's name, its **activator**, binds the parameters: `(symmetric co-located)` binds `?rel = co-located` for the rule `symmetric` |
| **empty**, `()` | first-order | on every match; the variables of its `:match` are bound by the matcher |

A rule with parameters and no activator never fires: the parameters are the gate. The kinds of rule, and what a rule
means, are [`../03-syntax-and-semantics/05-rules.md`](../03-syntax-and-semantics/05-rules.md).

```lisp
(rule symmetric (?rel)
  :match  (?rel ?a ?b)
  :assert (?rel ?b ?a)
  :why    "{?rel} is symmetric.")
```

applied through the fact `(symmetric co-located)`, which binds `?rel = co-located` and then matches `(co-located ?a
?b)`. One rule per property serves every relation that has it.

**An empty `:match` is the empty conjunction, which is true**
([Q-M1e2.36](../../../decisions/language/Q-M1e2.36-an-empty-match-is-true-and-fires-once.md)). `:match ()` holds
once: a rule fires once at root, or once per activator with only its parameters bound, `(rule r (?x) :match ()
:assert (hit ?x))` with `(r a)` concluding `(hit a)`, and an hrule proposes its conclusion likewise. `()` is its one
spelling: `and` and `or` take one premise or more, so `(and)` and `(or)` are parse errors, each with a note naming the
rule and the clause it is written in
([Q-M1f.45](../../../decisions/diagnostics/Q-M1f.45-the-parse-error-at-and-or-or-carries-a-note.md)). **`:assert
()`, the empty conclusion, asserts nothing**: an application derives nothing and is no firing. A rule whose `:match`
is `()` and whose conclusion is `(false)` makes root dead, its refutation resting on no fact, so root's core is the
empty set, printed `∅` ([Q-M1f.48](../../../decisions/read-out/Q-M1f.48-a-text-surface-writes-the-empty-core.md)).

**A rule concludes a conjunction of literals**: facts and stored negations `(not P)`, with `(false)` and checks
(`eq`, `neq`, `(not (eq …))`) among them, or `()`, or an obligation, `(open)` or `(open ?R)`, as the whole `:assert`.
An `(and …)` is read flat at any depth, as a premise's is, so `(and (r a) (and (p a) (q a)))` concludes what `(and (r
a) (p a) (q a))` does. A rule concludes no disjunction, whole or inside an `(and …)`: *one of* is the hypothesis
layer's, an hrule's proposals or an obligation and what discharges it, each alternative a state of the answer. Nor
does it conclude a compound fact, a connective inside a fact or under a conclusion's `not`. Both are refused at the
parse, and through a macro's expansion at load
([Q-M1f.74](../../../decisions/language/Q-M1f.74-a-nested-or-a-macros-expansion-writes-is-refused.md)). **An hrule
proposes one fact per match**, each a commitment of its own, so its conclusion, read flat, is one fact; one of more is
refused at load ([Q-M1f.271](../../../decisions/language/Q-M1f.271-an-hrules-conclusion-is-one-fact.md)): write one
hrule per conclusion.

**An activator binds each parameter to its argument, any value**: a name, or a nested fact, `(act (f a))` binding
`?y` to `(f a)`, which the rule's patterns match and its conclusions write as they would a name. A parameter that
names a relation, `?rel` above, binds a name: an activator that binds one to a fact is refused at load, where the
activator is written and where a rule could derive it
([Q-M1f.1](../../../decisions/language/Q-M1f.1-a-rule-that-could-derive-an-activator-binding.md)). One that can be
bound to `eq` heads premises and conclusions of two arguments only
([Q-M1f.46](../../../decisions/language/Q-M1f.46-a-relation-parameter-that-can-hold-eq-heads-only.md)): `(?R ?x)`
under `(r eq)` is refused at load.

#### Premise forms in `:match`

Beside fact patterns, a `:match` takes the premises of negation and quantification:

| premise | holds when |
|---|---|
| `(not P)` | a **stored** `(not P)` fact matches |
| `(absent P)` | negation as failure: no fact matches `P` in the state, asked at the closure boundary ([below](#naf-evaluation-timing--the-closureworld-boundary)) |
| `(unknown P)` | `std.macro`'s macro for `(and (absent P) (absent (not P)))`: `P` is neither stated nor denied |
| `(forall ?b G B)` | `std.macro`'s macro for `(absent (and G (absent B)))`: for every binding of `?b` that satisfies `G`, `B` holds |

At any moment a fact `P` is **asserted**, matched by `P`; **negated**, matched by `(not P)`; or **open**, matched by
`(unknown P)`. Negation as failure is written `(absent P)`, and only so. `forall` and `unknown` are macros of
[`std.macro`](../../../../stdlib/macro.ein), imported to be used
([`03-reserved-names.md` § Pattern macros](03-reserved-names.md#pattern-macros-forall-and-unknown--not-reserved)).

**What a premise may be.** A premise is a form: a fact pattern, a guard (`eq`, `neq`, and `(not (eq a b))`, which is
`(neq a b)` ([Q-M1f.31](../../../decisions/language/Q-M1f.31-not-eq-x-y-is-neq-x-y-in-every-place.md))), an
`(absent …)`, or an `(and …)` or `(or …)` of premises. A pattern's relation is a name, or one of the rule's
parameters, which its activator binds to a name; its arguments are values, variables or patterns. The loader refuses,
one line naming the rule and the clause ([`07-diagnostics.md`](07-diagnostics.md), *a rule shape that reads nothing*): a
name or a variable written where a premise goes; a relation written as a variable no parameter binds, at the top
level or inside a pattern, in a `:match` or an `:assert`; and an activator that binds a relation parameter to a nested
fact. A string, a bare `_` and a keyword pair anywhere in a premise are refused at the parse.

#### Conclusion forms in `:assert`

A conclusion is ordinarily a fact pattern, with `(and …)` concluding several at once, read flat. Two heads are
**verdicts about the state**, and neither is stored as a fact:

| conclusion | says |
|---|---|
| `(false)` | this state is contradictory |
| `(open)` / `(open ?R)` | this state is **unfinished**: it owes a witness, and `(open ?R)` names the relation whose extent is incomplete |

**`open` is written there and nowhere else**
([Q-M1f.66](../../../decisions/language/Q-M1f.66-open-is-written-only-as-a-rules-verdict.md)): a rule's or an
hrule's whole `:assert`, `(open)` or `(open ?R)`, `?R` a parameter or a relation's name. It is one of the words that
are never a name, so no fact, nested fact, stored negation, pattern, value, type, object, activator argument, goal,
claim, `(solve …)` list, `:over`, import, trace step or macro body writes it
([Q-M1f.86](../../../decisions/language/Q-M1f.86-a-macros-body-never-writes-open.md)); each is refused at the
parse, at the `open`, with the note *open is reserved: it is the verdict a rule's :assert states, written as the whole
:assert, (open) or (open ?R), and nowhere else, so it names no relation and no value*.

**A conclusion of `eq` or `neq`**, `(neq ?x ?y)`, `(eq ?x w)`, `(not (eq ?x ?y))`, or a pattern whose relation a
parameter binds to `eq`, **is checked against its definition, never stored**: one that holds adds nothing, and one
that does not is `(false)`, derived by the rule from the premises its firing matched, so its state is dead, its core
those premises and `refuted by` the rule. A firing whose every conclusion holds derives nothing and is no firing. An
hrule's instance of either is no hypothesis
([Q-M1f.32](../../../decisions/hypotheses/Q-M1f.32-an-hrules-eq-or-neq-instance-is-no-hypothesis.md)).

**A conclusion `(not ?x)` negates the fact `?x` holds**, and a name has no negation: where the program shows that
`?x` can hold a name, the rule or the hrule is refused at load
([Q-M1f.127](../../../decisions/language/Q-M1f.127-a-conclusion-not-x-is-refused-where-the-program.md)), and where
only a run meets the name, the firing concludes nothing for it
([Q-M1f.128](../../../decisions/language/Q-M1f.128-a-firing-that-meets-the-negation-of-a-name.md)).

**Every variable a conclusion writes is bound** on every way the `:match` can hold
([Q-M1e2.37](../../../decisions/language/Q-M1e2.37-a-conclusions-variable-is-bound-by-every-way.md)): by a parameter,
which the activator binds; by a relation premise, at any depth of its arguments; or by an `eq` or `neq` guard, which
binds a variable nothing else binds, over the program's names. A variable seen only inside an `(absent …)`, a
`forall` or an `unknown` is not bound outside it, `()` binds nothing, and a top-level `(or …)` must bind it in every
disjunct. An hrule's `:assert` is held to the same rule. A rule or an hrule that writes one nothing binds is refused
at load, one line per variable naming the two explicit spellings of what the implicit form could mean: a premise such
as `(is-a ?y T)`, *every object of a type*, or `(eq ?y ?y)`, *every name of the program*.

`(false)` is stored, since a contradiction survives any extension and a dead state stays dead; `open` is not, because
openness exists to be destroyed by an extension, and a stored one would survive its own discharge. It is a tally on
the state, read once the state has reached its fixpoint, and a rule that asserts it is an **obligation**, which never
fires during saturation. `(open ?R)` names the incomplete relation and nothing else: the witness domain and the slot
to fill are read off the rule's own `(absent …)`, and where they cannot be, the rule is refused at load rather than
guessed. What an obligation means, and how a state's debts are reported, is
[`../03-syntax-and-semantics/08-obligations.md`](../03-syntax-and-semantics/08-obligations.md); its refusals are
[`03-reserved-names.md` § The verdict atom](03-reserved-names.md#the-verdict-atom--open).

#### NAF evaluation timing — the closure/world boundary

`(absent P)` is a **question asked of the state, answered at a positive fixpoint**. While the rules whose premises
are all positive fire, a firing that waits on an `(absent …)` is held back; when the positive closure has nothing left
to derive, each held-back firing's `(absent …)` is asked once, against that fixpoint, and one firing that passes is
admitted, after which the closure runs again. For a rule's author that means:

- **`(absent P)` asks *is `P` missing from the finished positive derivation?***, not *has `P` not been derived
  yet?*
- **The order does not decide what is derivable.** On a stratified rule set the closure's result does not depend on
  the order, and on a monotone program neither does the boundary's; the agenda's order still orders the firings, and
  so the trace, and decides which derivation of a fact arrives first, which is what a core is explained from
  ([Q-M1f.6](../../../decisions/answer/Q-M1f.6-a-core-is-an-explanation.md)).
- **A rule set that is not stratified is answered by operational order.** `p ← absent q` beside `q ← absent p` has two
  models; the engine reaches one, by the order the boundary admits firings in, and does not report that the other
  exists. A program whose rules have a cycle through an `absent` is told so: a `strata` row in `ein solve`'s read-out
  ([`08-read-out.md`](08-read-out.md)) and a `;; Strata` line in `ein saturate --dump`, each cycle named by its relations
  and the rules on its edges. It is a diagnostic, not a refusal: the answer stands, and may depend on that order.

A **nested** `(absent …)`, what `forall` expands to, `(absent (and G (absent B)))`, is one question, asked as a unit
against the same fixpoint. The definition, and what is not provided (stable models, retraction), is
[`03-absent-and-naf.md`](../../reasoning/02-kb-evolution/03-absent-and-naf.md).

#### Premise order around a guard

Where an `(absent …)` stands among the positive premises decides *what it asks*:

| written | asks |
|---|---|
| `(and (absent (P ?x)) (Q ?x))` | is there **no `P` at all**? `?x` is still free where the guard stands |
| `(and (Q ?x) (absent (P ?x)))` | is there **no `P` for this `?x`**? `?x` was bound by the `(Q ?x)` before it |

Write the binding premise **before** the guard to mean *for this `?x`*.

A **guard**, `(eq a b)` or `(neq a b)`, is a premise over the relations of equality and non-equality: `(eq x x)` for
every value, and `(neq x y)` for every two different values, computed and never stored. Its arguments are values: a
variable, a name, or a fact that holds no variable, `(f a)`, the nested fact it spells as in `(p (f a))`; a form that
holds a variable is refused at load
([Q-M1f.30](../../../decisions/language/Q-M1f.30-a-guard-argument-holds-no-variable-inside-a-fact.md)). **A guard's
place among the relation premises of its conjunction does not change what the premise means; the order of two guards
on a variable no relation premise binds does**
([Q-M1f.34](../../../decisions/language/Q-M1f.34-a-guard-that-binds-its-variable-binds-it-where.md)):

- A guard whose variable a relation premise of its conjunction binds waits for that premise and is a test after it,
  so `(and (neq ?x a) (p ?x))` is `(and (p ?x) (neq ?x a))`.
- A variable **nothing** in the conjunction binds is bound by the guard, where it is written: `eq` binds it to the
  other side's value, and `neq`, or `eq` with both sides free, ranges it over **the program's names**, every constant
  a fact writes in an argument, at any depth, and every constant of a rule's `:match` or `:assert`
  ([Q-M1e2.9](../../../decisions/language/Q-M1e2.9-a-programs-names-are-the-constants-its-facts.md)); a name only a
  query writes is none of them.
- A guard on that variable after it is a test, so the first of two such guards decides what the variable holds: `(and
  (p ?a) (neq ?m z) (eq ?m ?a))` binds `?a` to each name `p` holds of but `z`, and to no nested fact, since `neq`
  ranges `?m` over the names and `eq` tests it, while `(and (p ?a) (eq ?m ?a) (neq ?m z))` binds it to each value `p`
  holds of but `z`, a nested fact too. The first order is how a rule says *a name, and not a fact*
  ([Q-M1f.208](../../../decisions/stdlib/math/decimal/Q-M1f.208-two-guards-say-that-an-operand-is-a-name.md)).
- An `(absent …)` after such a guard reads its variable, and one before it does not. Under an `(absent …)`, the
  enclosing premises before it bind, as they scope the `absent` itself.

### Solve

```text
(solve
  [:hrules (<hrule> <activator>…)]         ; the hrules that generate, and their activators
  [:hypothesis-relations (<rel> …)]        ; speculate only these
  [:no-hypothesis (<rel> …)]               ; …or never these
  [:no-search]                             ; a flag: the lattice is root alone
  [:expect <claim>])                       ; the expected region
```

**The lattice is stated once.** The lists choose the candidates, and the candidates make the lattice the answer is a
region of ([Q-M1e2.7](../../../decisions/answer/Q-M1e2.7-the-answer-is-a-region-r-of-the-hypothesis.md)): a program
has one lattice and one answer, and every query asks about it. What each list does is
[`../03-syntax-and-semantics/09-hypotheses.md`](../03-syntax-and-semantics/09-hypotheses.md).

- **At most one per file.** A second is a load error, `kb load error: 2 (solve …) forms — a program states its
  lattice once`. With none, the lattice is the default ladder's, unrestricted, and `(solve)` states the same.
- **It takes five keywords**, and any other is refused at the parse. On a `(query …)` each of the four that choose the
  lattice is refused at the parse, with a note that names `(solve …)`.
- **`:no-search` is a flag**, the one keyword that takes no value: **the program states that its lattice is root
  alone**. Nothing is proposed, whatever hrules, obligations or lists it also declares, so R is {∅} where root is alive
  and discharged and empty otherwise; an obligation judges root and proposes nothing. It is part of the program, so
  every command reads it, and `ein solve`'s `universe` row says *the program states :no-search*. A value after it is
  refused at the parse. `ein solve --no-search` is another thing, a command declining to discover R
  ([`05-cli.md`](05-cli.md)).
- **`solve` is a reserved word**, as `query` and `config` are, so it names no relation.
- **The lists do not contradict the program's other hypothesis declarations**, read on root's fixpoint, or the program
  is refused at load, one line per conflict, on every command
  ([Q-M1f.245](../../../decisions/hypotheses/Q-M1f.245-the-hypothesis-mechanisms-are-judged-at-load-on.md)): no
  active hrule proposes a relation `:hypothesis-relations` leaves out or `:no-hypothesis` names; with no hrule, no
  obligation owes one; no relation is in both lists; and no listed relation is closed. A list that filters nothing, or
  a `:no-hypothesis` name another list or a closure already says, is one `note:` line on stderr
  ([Q-M1f.242](../../../decisions/hypotheses/Q-M1f.242-a-hypothesis-declaration-that-filters-nothing-is.md)).
  `:no-search` relaxes no conflict
  ([Q-M1f.254](../../../decisions/hypotheses/Q-M1f.254-solve-no-search-relaxes-no-conflict.md)): beside an obligation
  or a closure it is silent, and beside an hrule or a list one `note:` says that the hrule proposes, or the list
  filters, nothing. Every pair is [`03-reserved-names.md` § Hypothesis and query control](03-reserved-names.md#hypothesis-and-query-control).
- **Every name a list holds names something the program has**
  ([Q-M1f.114](../../../decisions/language/Q-M1f.114-a-relation-a-solve-list-names-is-one-the-program.md)), or the
  program is refused at load, one line per name, on every command:
  - `:hrules (h (a b) (c d) …)`: `h`, and every bare name after it, is an hrule the program declares
    ([Q-M1f.115](../../../decisions/language/Q-M1f.115-an-hrules-list-names-declared-hrules.md)). Each `(a b)` is an
    activator, which binds `h`'s parameters once, so it has as many arguments as `h` has parameters. An argument at a
    parameter that names a relation, one that heads a form of the hrule as `?R` does in `(?R ?a ?b)`, is a relation of
    the program; an argument at any other parameter is a value, a type or an object, and is not checked
    ([Q-M1f.116](../../../decisions/language/Q-M1f.116-an-activators-argument-at-a-parameter-that-names.md)).
  - `:hypothesis-relations` and `:no-hypothesis`: every name is a relation of the program. `()` names none.
  - **A relation of the program** is a name the program writes as a relation outside the three lists: one it declares
    or states a fact of, `is-a`, `eq` and `neq`, the relation of a form in a rule, an hrule or a query's `:goal`, or a
    name it passes where a relation's name is held, `(bijective color-loc)`. A relation only a rule's `:assert` derives
    is one.

#### The expected region — `(solve … :expect …)`

```text
<claim> ::= (region <item>*)              ; R, by its nodes
          | (false)                       ; every node dead
<item>  ::= (model <fact>*)               ; one state of R
          | (merged <to> <from>)          ; a state several commitments reach
          | (owing (model <fact>*) …)     ; R empty, and these alive nodes owe
<to>    ::= (model <fact>*)               ; one of the states listed
<from>  ::= (model <fact>*)               ; its one minimal commitment
          | (or (model <fact>*) …)        ; …or its several
```

A `(model …)` is a state of R, root's facts included; the models are R's states, one to one, in any order; naming a
relation closes it, across the whole claim, and stored negatives are not closed; `(merged …)` is all or nothing;
`(owing …)` says R is empty and lists the alive, owing nodes with no alive node one fact up; `(false)` says every node
is dead, and `(region)` that R is empty, of either kind. How each reads, on real programs, is
[`../03-syntax-and-semantics/12-query.md`](../03-syntax-and-semantics/12-query.md).

The words `region`, `merged` and `owing` are this keyword's alone: a top-level `(region …)`, `(merged …)` or
`(owing …)` is a parse error, since pasted outside `(solve … :expect …)` it would check nothing. `model` is a position
word, a name wherever a name goes. A region claim lists a state's stored facts, and a state stores no fact of `eq` or
`neq`, so a region claim that names either is refused at load. `ein test` checks a region claim against R as
`ein solve` describes it ([`05-cli.md`](05-cli.md)); [`tests/region/`](../../../../tests/region/) holds one program per form.

### Query

```text
(query
  :goal <premise>                          ; what to find, or verify
  [:goal-text "<template>"]                ; the headline template
  [:over all|minimal|maximal]              ; which states
  [:expect <claim>])                       ; what the answer should be
```

**The keywords are an allow-list.** Those four are what a `(query …)` takes, and any other is refused at the parse,
the three lattice keywords with a note that names `(solve …)`: a keyword that loaded and checked nothing is the failure
`:expect` exists to prevent.

**A file may carry several `(query …)` blocks**, each a question about the same program and the same lattice, and
`ein solve` answers every one, printing `query <i> of <n>` above each, all read off one run: the answer does not
depend on the query, so a query is a projection of it. `(config …)` keeps the last one written: a config is a setting,
a query is content. Whether a program has one model, several or none are three readings of the one answer, the region
R, never three modes chosen up front: there is one `ein solve`. What a query means is
[`../03-syntax-and-semantics/12-query.md`](../03-syntax-and-semantics/12-query.md).

**A `:goal` is a premise, read as a rule's `:match` is.** Three things follow, each refused or done at load, on every
command:

- **A macro call in a goal is expanded**, as in a rule
  ([Q-M1f.117](../../../decisions/language/Q-M1f.117-a-macro-call-in-a-goal-is-expanded-and-the-goal.md)): `(query
  :goal (and (is-a ?g Guest) (unknown (paid ?g))))` asks which guest it is unknown of whether they paid. The read-out
  prints the goal as written. A `std.macro` call the program did not import, a call of another count and an expansion
  a rule would be refused for are refused, one line each. A claim lists facts, so a macro call in an `:expect` is not
  expanded and names no relation
  ([Q-M1f.118](../../../decisions/language/Q-M1f.118-a-claim-is-not-expanded-so-a-claim-headed.md)).
- **A goal asks about relations of the program**
  ([Q-M1f.119](../../../decisions/language/Q-M1f.119-a-goal-and-a-claim-ask-only-about-relations.md)). The relation of
  every pattern it writes, at any depth, under `(absent …)` and `(not …)` too, is one the program declares, states a
  fact of, writes in a rule or an hrule, or one a `(solve :hrules …)` activator has its hrule propose; `is-a`, `eq`,
  `neq` and `relation` are every program's. A goal over any other name, `(payed ?g)` for `paid`, is refused at the
  pattern: no state holds a fact of it, so the goal would have no row, and under `(absent …)` would hold of everything.
  A claim's relations are held to the same list, so a claim may list a fact only a rule's `:assert` derives.
- **A goal names its relations.** A relation written as a variable is a rule's, bound by its activator; a query has
  none, so `(?R ?x)` is refused in a goal, at the top level and inside a pattern, `(says (?R ?g))`.

**`:expect` — what the answer should be**
([Q-M1c.1](../../../decisions/language/Q-M1c.1-a-query-states-the-answer-it-expects-with-expect.md)). A query may
state its own answer, and the engine checks it. Three shapes, the verdict implied by which one is written:

```text
:expect (model <fact>*)                 ; exactly one class: one answer
:expect (or (model …) (model …) …)      ; that set of classes
:expect (false)                         ; no answer
```

1. **The goal's relations are required.** An expectation that does not pin what the query asked does not load.
2. **More is allowed.** Any ground fact may be listed, a `(not …)` among them, so a claim can pin a consequence at a
   distance rather than restate the rule it is about.
3. **Naming a relation closes it**
   ([Q-M1c.2](../../../decisions/language/Q-M1c.2-an-expectation-is-ground-and-naming-a-relation.md)): if `:expect`
   names `pet-loc` at all, the listed `pet-loc` facts are that relation's complete extent in the class. Relations it
   never names are not constrained. Stored negatives are not closed, and a listed `(not …)` is checked for presence;
   `(or …)` compares sets of classes, never sequences.

The value is checked at load: an expectation that is not one of the three shapes, that names a relation the program
does not have, that leaves out the goal's relations, or that holds a variable is refused, since an expectation is an
answer and not a pattern. [`examples/features/10_expect.ein`](../../../../examples/features/10_expect.ein) is the
worked example. `ein solve` exits 1 when a claim is false and prints what disagreed; `ein test` is the runner, and asks
each file the least question that decides its claims ([`05-cli.md`](05-cli.md)).

**A goal may ask about `eq` and `neq`**: a guard in a goal reads as it does in a premise, so `(query :goal (neq w
?y))` answers a row for each name of the program but `w`, `(eq w ?y)` answers `?y = w`, `(neq a b)` holds and `(eq a
b)` has no row, and a class lists the guard's instances beside the goal's other atoms, `(model (neq w a) (p a))`. A
claim may name either, and naming one closes it over the class, which holds the goal's instances only. *the goal
holds: it has no variable* is printed only for a goal with no variable; one whose every variable is an `(absent …)`'s
own prints *the goal holds: it binds no variable*.

**`:goal-text`, the headline template.** A `:goal-text "<template>"` renders the one-line answer for each class,
referring to the goal's **own variables** by name, the `{?var}` references a rule's `:why` uses, bound from the
class. A reference is the variable as the program writes it after its `?`, up to the first `}`, so `{?isR*}` names
`?isR*`; a template naming a variable that holds a `}`, which no reference can reach, is refused at load
([Q-M1f.11](../../../decisions/language/Q-M1f.11-a-template-may-not-refer-to-a-variable-whose.md)). The value is one
string; strings are not concatenated.

```lisp
(query
  :goal (and (drink-loc Water ?h_water) (nation-loc ?who_water ?h_water)
             (pet-loc Zebra ?h_zebra)   (nation-loc ?who_zebra ?h_zebra))
  :goal-text "The {?who_water} drinks water in {?h_water}; the {?who_zebra} owns zebra in {?h_zebra}")
;;  →  "The Norwegian drinks water in House-1; the Japanese owns zebra in House-5"
```

A relation's `:why` renders one *fact*; a query's `:goal-text` renders the *headline* from the goal's bindings, one
class's sentence on the read-out.

### Import

```lisp
(import onto)                        ; the module whole; the file writes its names in full, onto.is-a
(import onto :as o)                  ; the module whole; o.is-a is onto.is-a
(import onto :symbols (is-a things)) ; the module whole; the file writes is-a and things short
```

**Every import brings its module whole**, its declarations and its facts, with the module's own imports, all the way
down, **one copy per program**, and the three shapes differ only in **names**
([Q-M1f.247](../../../decisions/imports/Q-M1f.247-every-import-brings-its-module-whole.md)). `:as` and `:symbols`
are mutually exclusive. **The answer does not depend on the import form**
([Q-M1f.249](../../../decisions/imports/Q-M1f.249-the-answer-does-not-depend-on-the-import-form.md)). What a module
is, and how a library is built of them, is [`../03-syntax-and-semantics/11-macros.md`](../03-syntax-and-semantics/11-macros.md).

**A full name is the name.** Every name a module declares, a relation, a rule, an hrule, a macro, has one **full
name**, the module's name and the name, `std.math.decimal-arith.add`, `onto.is-a`
([Q-M1f.231](../../../decisions/imports/Q-M1f.231-every-name-a-module-declares-has-a-full-name.md)): the same through
every import path, so a module read by two imports, under two aliases, is one copy. A full name is global: any file
may write one of a module the program brings. The main file has no module name, so what it declares keeps its name. A
name no file declares, a type, a digit, a relation a module states facts of and never declares, has no full name and
is one name in every file.

**A module's name is its path.** `std.x.y` is `x/y.ein` under the [standard library](04-stdlib.md); any other name is a
path **from the program file's directory**, a dot per directory, for the imports a module writes too
([Q-M1f.235](../../../decisions/imports/Q-M1f.235-a-module-has-one-path-from-the-program-files.md)): from a program in
`examples/word_problems/`, `(import word-theory.gaps :as gaps)` reads `word-theory/gaps.ein`, and that module imports
its sibling as `word-theory.words`. A name with an empty segment or a `/` is refused, so `..` is
([Q-M1f.252](../../../decisions/imports/Q-M1f.252-a-modules-name-has-no-empty-segment-and-no-slash.md)): one file has
one name. A cycle is refused.

**A short name is local to the file that binds it.** A file writes short its own declarations, the names its imports'
`:symbols` list, and, under `:as o`, `o.` and a name the module declares; a bare `(import M)` binds no short name, and
the file writes `M.x`. Nothing is re-exported
([Q-M1f.232](../../../decisions/imports/Q-M1f.232-a-module-has-no-export-list.md)): a module's short names are its
own, and a name a module only imports is listed from the module that declares it. A listed name the module neither
declares nor states a fact of is refused, *not provided by the module*; one it states facts of and does not declare
names the relation those facts are of
([Q-M1f.250](../../../decisions/imports/Q-M1f.250-a-listed-name-a-module-states-facts-of-names.md)). **A short name a
file writes that it neither declares nor imports is its own**, whatever a module of the program declares under it
([Q-M1f.236](../../../decisions/imports/Q-M1f.236-a-short-name-a-file-writes-and-lists-from-no.md)).

**One short name, one full name, in a file.** One short name bound to two full names in one file, by two imports or by
an import and the file's own declaration, is refused
([Q-M1f.234](../../../decisions/imports/Q-M1f.234-one-short-name-bound-to-two-full-names-in-one.md)); so is one
alias prefixing two modules. Binding one name to one full name twice is no error. A prefixed name that names nothing
its module declares is refused
([Q-M1f.251](../../../decisions/imports/Q-M1f.251-a-prefixed-name-that-names-nothing-its-module.md)), and so is a
full name of a module no import of the program brings
([Q-M1f.238](../../../decisions/imports/Q-M1f.238-a-full-name-of-a-module-no-import-brings-is.md)): a module enters a
program by an import of it. The main file may not declare a name under the full name of a module it imports
([Q-M1f.239](../../../decisions/imports/Q-M1f.239-a-program-declares-no-name-under-the-full-name.md)); a
`--dump-answer` state file, which imports nothing and declares the full names it was loaded with, is the main file's
own declarations. A module's declared name holds no dot
([Q-M1f.241](../../../decisions/imports/Q-M1f.241-a-modules-declared-name-holds-no-dot.md)), so a full name reads
one way.

**What prints** is the main file's spelling: the short name it lists, else its first alias's, `o.is-a`, else the full
name, `std.math.decimal-arith.strip`
([Q-M1f.237](../../../decisions/imports/Q-M1f.237-a-name-prints-as-the-main-file-writes-it-else.md),
[Q-M1f.253](../../../decisions/imports/Q-M1f.253-under-an-alias-a-name-prints-as-the-main-file.md)). A spelling
another file writes as a name of its own is not taken, so that two names never print as one.

**An imported fact is a fact of the program.** It is stored as if written where the import stands: background, or
given where it carries a `:source`
([Q-M1f.138](../../../decisions/imports/Q-M1f.138-an-imported-fact-keeps-its-own-provenance.md)). Its constants are
names of the program
([Q-M1f.137](../../../decisions/imports/Q-M1f.137-an-imported-facts-constants-are-names.md)), so they are among the
objects the blind enumerator proposes facts about; a program that wants no guesses about a library's relation closes
it. A fact of a module arrives once, the module being one copy
([Q-M1f.143](../../../decisions/imports/Q-M1f.143-a-modules-fact-arrives-once-however-many-imports.md)). The same fact
stated with two `:source`s keeps its first statement
([Q-M1f.139](../../../decisions/imports/Q-M1f.139-a-fact-stated-twice-keeps-its-first-statement.md)), as it does
written twice in one file.

**No import brings a module's `(query …)`, `(solve …)`, `(config …)` or `(trace …)`**
([Q-M1f.215](../../../decisions/imports/Q-M1f.215-no-import-brings-a-modules-query-solve-config.md)). They are what a
*program* asks, how it is solved and how it is configured, and a module that is also a program keeps them to itself:
the importer states its own, or none.

### Trace

```text
(trace
  (step <id> :rule <name>                  ; one derivation step
             :using (<premise-ids>)        ;   or (and <id>+)
             :derives <fact>
             [:source "<text>" | :assumes <fact>]))
```

A trace holds `step` events alone, each taking `:rule`, `:using`, `:derives`, `:source` and `:assumes`; any other
event, `(branch-open …)` among them, is a parse error at its head. A `(trace …)` is the **same language** as the
program: one parser and one printer, so a rule can match `(step …)` forms
([Q21](../../../decisions/cli/Q21-every-form-has-a-fixed-dot-shape-and-its-layout.md)), and a derived fact's
provenance, `(rule, premises, source)`, is literally a `(step …)` form
([Q18](../../../decisions/engine/Q18-provenance-is-per-fact-an-or-of-derivations-over.md)).

## 3. Patterns

A rule's `:match` and `:assert` are written in a small pattern language: **positive and conjunctive**, with two
computed relations, `eq` and `neq`, each written as a premise, `(neq ?a ?c)`, and negation as failure, `(absent …)`.
A rule takes `:match`, `:assert` and `:why` and no other keyword; a term takes none, so a filter clause is written as a
premise beside the patterns, never as `:where`
([Q-M1e2.28](../../../decisions/language/Q-M1e2.28-each-form-takes-a-closed-list-of-keywords.md)). What a pattern
means, as a graph and in words, is [`../03-syntax-and-semantics/05-rules.md`](../03-syntax-and-semantics/05-rules.md).

### The primitives, and where a malformed one is caught

A malformed primitive is caught in one of two places: by the **grammar**, where a rule writes it, and by the
**loader**, after macro expansion, where a macro's body or a call's argument writes it
([Q-M1f.81](../../../decisions/language/Q-M1f.81-a-malformed-primitive-is-a-form-the-grammar.md)). Either way every
command refuses the program before anything is compiled, fired, proposed or owed.

| primitive | arguments | refused by |
|---|---|---|
| `not` | 1 | the **grammar**: one pattern, and no keyword after it. A conclusion `(not ?x)` of a variable that can hold a name is refused by the **loader** |
| `eq` / `neq` | 2 | the **grammar** |
| `absent` | 1 | the **grammar** |
| `and` / `or` | 1 or more | the **grammar**: `(and)` and `(or)` are parse errors, each with a note naming the rule and the clause, and `(and X)` is legal, if degenerate; the empty conjunction is `:match ()` |
| `false` | 0 | the **grammar**, as a conclusion and as a premise, `(false)` |

**The empty conjunction is `()`**, written as a whole `:match`, and it is true: the rule fires once, or once per
activator, and an hrule proposes its conclusion likewise. `()` as a whole `:assert` is the empty conclusion, which
asserts nothing. `()` is no premise inside another.

**A macro's body is read where it is called.** A body takes any head at any arity, and a call takes any premise as an
argument, so the grammar never reads the expanded rule: `(macro two (?a ?b) (absent ?a ?b))`, `(macro app (?R ?a) (?R
?a))` called `(app eq ?x)`, `(macro neg (?a) (not ?a))` called with a conjunction. The loader holds a rule's, an
hrule's and an obligation's clauses, expanded, to the shapes this table gives a written rule, by position, and refuses
a form in another shape at load, on every command, one line each
([Q-M1f.80](../../../decisions/language/Q-M1f.80-a-malformed-primitive-a-macros-expansion-writes.md)): the place
where the expanded form is written, the rule, the form as expanded and what to write,

```text
kb load error: m.ein:3:16: rule 'r': its :match writes (eq ?x), and eq takes exactly 2 arguments, not 1 — it compares two values, so write one guard per pair
```

A macro no rule calls is not read, so it loads whatever its body
([Q-M1f.67](../../../decisions/language/Q-M1f.67-a-macros-body-is-checked-where-it-is-called.md)). A query's `:goal`
is a premise, and a macro call in one is expanded and read the same way. A macro call written as a fact, or listed in
a claim, is not expanded.

**A pattern's argument is what a value can be**: a name, a variable, a pattern of an ordinary relation, or `(not
<pattern>)`, a stored negation. The grammar takes a connective there so that a macro such as `forall` can be called
with premises; once the macros are expanded, `(and …)`, `(or …)`, `(neq …)`, `(eq …)`, `(absent …)` and `(false …)`
with an argument as a pattern's argument are refused at load, in a rule, an hrule, an obligation and a goal
([Q-M1f.125](../../../decisions/language/Q-M1f.125-a-connective-as-a-patterns-argument-is-refused.md)). The same
rule, *a value is a name, or a fact a state could hold*, refuses at the parse a premise `(false ?s)`, a conclusion
headed `absent`, and `(eq …)`, `(absent …)` or `(false k)` as a conclusion's or a guard's value.

**A conclusion `(not ?x)` negates a fact**, the one a variable holds: `(rule hypothesis-contradiction () :match (and
(hypothesis ?h) (contradiction-under ?h)) :assert (not ?h))` stores `(not (co-located Norwegian House-2))`. A name has
no negation, so the loader refuses the conclusion wherever the program shows that the variable can hold a name: a
written fact, a declaration, another rule's or an hrule's conclusion, the rule's activator, a declared relation the
hypotheses are taken from, or a guard that alone binds it. Where only a run meets the name, the firing stores nothing
for that conclusion and an hrule proposes nothing. **An hrule's `:match` may read `not`**, as a premise, under
`(absent …)` and through `unknown`, and reads the state it was proposed in
([Q-M1f.129](../../../decisions/engine/Q-M1f.129-an-hrule-that-reads-not-reads-the-state-it-was.md)).

**An `(or …)` written as a premise** of an `(and …)` or under an `(absent …)` is a shape no plan is compiled from,
and the loader refuses it, a macro's expansion included; a top-level `(or …)` in a `:match` is one rule per disjunct.

### What a pattern is made of

| construct | example | meaning |
|---|---|---|
| variable | `?a`, `?house` | bound by the match, and reused across the clauses |
| name | `Red`, `House-1` | matches itself |
| named relation pattern | `(co-located ?a ?b)` | matches the relation's facts |
| relation pattern | `(?R ?a ?b …)` | the head is one of the rule's parameters, which its activator binds to a relation's name; the arguments bind by position. A head variable no parameter binds, at the top level or inside a pattern, in either clause, is refused at load |
| conjunction | `(and <p1> <p2> …)` | every premise holds |
| disjunction | `(or <p1> <p2> …)` | one premise holds |
| stored negation | `(not <p>)` | a stored `(not p)` fact matches |
| negation as failure | `(absent <p>)` | no fact matches `p`, at the closure boundary |
| equality fact | `(= ?a ?b)` | matches a **stored** `(= …)` fact ([below](#is-a-fact-head-not-a-unifier)) |
| computed equality | `(eq ?a ?b)` / `(neq ?a ?b)` | the relations of equality and non-equality, computed and never stored: a test where both sides are bound, a binding where one is free |
| membership | `(is-a ?a ?T)` | an ordinary relation pattern |
| relation-property tag | `(transitive ?r)` | an ordinary premise matching a stored property fact: a join, not a test |

**A keyword pair anywhere in a premise is refused at the parse**: a term takes no keyword. **A misspelled relation is
not refused**: `(instnce ?a ?T)` is a pattern over a relation nothing declares, and a rule containing it loads and
never fires, since an undeclared relation is allowed and matched like any other. What catches a mistyped head is the
symptom, a rule that never fires: `ein solve --events` lists every firing.

### `=` is a fact head, not a unifier

`=` is a name like any other: `(= a b c)` is a fact of the relation `=`. The two spellings of equality look
interchangeable and are not:

- **`(= ?a ?b)`** is an ordinary relation pattern whose head is `=`. It matches the `(= …)` *facts the state holds*,
  one a rule derived among them, since `:assert (= ?a ?a)` is a legal conclusion, and nothing else. A rule `:match
  (and (p ?a ?b) (= ?a ?b))` does **not** fire on `(p x x)`; it fires once `(= x x)` is stated.
- **`(eq ?a ?b)`** is the computed relation. It holds of `(p x x)` with no stored fact at all.

A stored `(= a b)` licenses no substitution and joins no congruence closure. What `=` *does* have is a drawing: a fact
of `=` with two arguments draws as an equality node, a `doublecircle`, with an edge to each side
([`12-rendering.md`](12-rendering.md#node-shape-legend)), which [`examples/syntax/equality.ein`](../../../../examples/syntax/equality.ein)
shows.

### What is not in the pattern language

- **Negation as failure spelled `(not …)`.** `(not <p>)` in a `:match` matches a **stored** `(not p)` fact, as every
  other pattern matches a stored fact; it is not failure to prove. Negation as failure is `(absent <p>)`.
- **Quantifiers and aggregates as expressions.** `forall` is a macro over nested `absent`s, not a primitive. A count
  or a cardinality has no spelling at all, and none is planned: Ein has no numbers
  ([Q-M1e2.24](../../../decisions/language/Q-M1e2.24-ein-has-no-numbers-and-no-ranges.md)). *Only one candidate
  remains* is ordinary rules: `std.bijection`'s `domain-elimination` and `range-elimination`, and `std.slots`'
  `slot-elimination` and `slot-fill`, each a `forall` decided at the closure boundary ([`04-stdlib.md`](04-stdlib.md)).
- **A host-language escape.** Every step the matcher takes is one the trace can name, and an opaque predicate would
  be an unexplained firing.

### The computed relations — `eq` and `neq`, and only they

| relation | arguments | holds when |
|---|:-:|---|
| `(eq ?a ?b)` | 2 | the two values are one; a free side is bound to the other |
| `(neq ?a ?b)` | 2 | the two values differ; a free side ranges over the program's names but the other |

`eq` and `neq` are relations
([Q-M1e2.26](../../../decisions/language/Q-M1e2.26-eq-and-neq-are-the-relations-of-equality-and-non.md)): `(eq x x)`
for every value, `(neq x y)` for every two different values, computed and never stored, in a goal, a premise and a
conclusion alike. A guard's argument is a value: a variable, a name, or a fact that holds no variable; a form that
holds a variable is refused at load. A premise means the same in any order of its conjuncts, and the order of two
guards on a variable only they bind decides what it holds ([§ Premise order around a guard](#premise-order-around-a-guard)).
`(not (eq ?a ?b))` is `(neq ?a ?b)`. A conclusion of either is checked against its definition: a true one adds
nothing, a false one is `(false)`.

There are two because *a computed relation's truth is computed from its arguments, where a relation's truth is data*:
`(transitive ?r)` and `(symmetric ?r)` are ordinary relation patterns matching a stored property fact the program or
the standard library states. Ein has no numbers, so no numeric, cardinality or aggregate primitive is either.

### The triangle rule, two forms

The two forms differ in which kind of rule they are ([`../03-syntax-and-semantics/05-rules.md`](../03-syntax-and-semantics/05-rules.md)),
and only one loads.

**As a rule with no parameters, which Ein does not load:**

```lisp
(rule triangle-composition ()
  :match (and (?r ?a ?b)
              (?r ?b ?c)
              (transitive ?r))
  :assert (?r ?a ?c)
  :why "since {?r} is transitive, {?a} {?r} {?c}")
```

A relation is named by a name or by a parameter its activator binds
([Q29](../../../decisions/engine/Q29-a-rule-compiles-once-per-activator.md)), so the loader refuses `?r` in each of
the three patterns:

```text
kb load error: rule 'triangle-composition': its :match writes (?r ?a ?b) with ?r as its relation, which no parameter binds — a relation is named by a name or by a parameter its activator binds; …
```

**As a relation-polymorphic rule, gated by its parameter:**

```lisp
(rule transitive (?rel)
  :match (and (?rel ?a ?b)
              (?rel ?b ?c)
              (neq ?a ?c))
  :assert (?rel ?a ?c)
  :why "{?rel} is transitive.")
```

Applied through `(transitive co-located)`, it fires only on the relations a fact tags. It is `std.algebra`'s
`transitive`, word for word: with `(co-located a b)` and `(co-located b c)` it derives `(co-located a c :rule
transitive)`. Its guard is the premise `(neq ?a ?c)`.

## See also

- [`01-grammar.md`](01-grammar.md): the grammar, exactly.
- [`03-reserved-names.md`](03-reserved-names.md): the words a program writes and does not bind.
- [`../03-syntax-and-semantics/`](../03-syntax-and-semantics/README.md): what each form means, in four representations.
- [`../02-guide/07-examples.md`](../02-guide/07-examples.md): worked Zebra fragments.
- [`12-rendering.md`](12-rendering.md): how `ein render` draws each form.
- [`07-diagnostics.md`](07-diagnostics.md): every refusal's line.
