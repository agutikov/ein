# Natural language

Every relation, rule, hrule and macro a program declares says in English what it means, and **a `:why` reads the
form it is written on**: a declaration's, the declaration; a rule's, the firing. A fact of a relation carries no text
of its own and takes none from the declaration: it reads by where it came from, a `:source` or its rule's `:why`. The
engine invents no prose: a sentence is always one the program wrote, filled in, and where no form carries one, a fact
reads as written, which is true. This page is what a program writes, what the engine prints from it and where, and
whose words are whose.

## What a program writes

```lisp
(relation knows Person Person :why "a {?3} {?2} a {?4}")

(rule knows-back ()
  :match  (knows ?a ?b)
  :assert (knows ?b ?a)
  :why    "{?a} knows {?b}, so {?b} knows {?a}")

(knows Alice Bob :source "Alice says she knows Bob")

(query :goal (knows ?x Alice) :goal-text "{?x} knows Alice")
(solve :no-search)
```

Four texts: the relation's `:why`, which reads the declaration it is written on, `{?n}` its n-th arrow, so this one
reads *a Person knows a Person*; the rule's `:why`, filled by the firing's variables; the fact's `:source`, the
sentence of the problem it states; and the query's `:goal-text`, filled per answer. A string is natural-language
text, any Unicode, with the escapes `\n \t \r \" \\ \u{…}`, and is written only after `:why`, `:source` and
`:goal-text`: it is never an object, and no fact has one as an argument
([Q-M1e2.23](../../../decisions/language/Q-M1e2.23-a-string-is-natural-language-text-never.md)).

The three keywords are the built-in connection between a form and its English, and that is all they are: a text
is no node of the knowledge base, no rule reads one, and none has a drawing or a formula of its own
([Q-M1f.371](../../../decisions/docs/Q-M1f.371-a-core-notion-is-shown-four-ways-in-the-users.md)). What a relation, a
rule or a fact *is* is drawn and defined on its own page; this page is how each reads.

## What the engine prints

The same program, as `ein solve FILE` prints its query's answer, each row of the projection with its `:goal-text`:

```text
  query             (knows ?x Alice)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?x = Bob
                        “Bob knows Alice”
```

and as `--trace OUT.md` writes a firing, its reason quoted, a given premise with its source, and the conclusion as
written, since a fact has no text of its own:

```text
### Step 1 — `knows-back`

> Alice knows Bob, so Bob knows Alice

Premises: `knows(Alice, Bob)` — from Alice says she knows Bob

Derives `knows(Bob, Alice)`.
```

The declaration's own reading, *a Person knows a Person*, is printed where a rule reads the declaration, after it
on the step's premises line (§ 3).

The views carry the texts where they draw what the texts are about: `ein render kb FILE` labels a given fact's
arrow with its source, *knows Alice says she knows Bob*, and a derived one with its rule, *knows by knows-back*;
`ein render program FILE` draws a query as a note holding its goal and its `:goal-text`; and the trace's drawing of
a firing writes the rule's reason in the rule's box ([facts](03-facts.md), [rules](05-rules.md)).

A template is filled by replacing each reference `{?v}` that the form can bind with the value bound, printed as
written; a reference it cannot bind is left as it is.

## 1. The four texts

| text | on | fills | printed by |
|---|---|---|---|
| `:why` | `(relation R …)` | **the declaration's arrows**: `{?1}` the atom `relation`, `{?2}` the relation's name `R`, `{?3}` its first label, … — the declaration's **statement** | `--trace`'s premises line, after the declaration where a rule reads it, and its *Derives* line where a rule concludes it |
| `:why` | `(rule …)`, `(hrule …)` | the firing's variables: a parameter, or a variable a positive premise binds — a derived fact's **reason** | `--trace`'s step, a rule node of the derivation views, an obligation's owe line |
| `:why` | `(macro …)` | a parameter | nothing (§ 5): it documents the pattern |
| `:goal-text` | `(query …)` | the goal's variables, per row | the read-out's projection rows, in `“…”` |
| `:source` | a fact the program states | — | `--trace`'s premises line |

A reference nothing can bind is printed as written, `{?zz}`, and so is a `{?5}` past a declaration's arrows: a
template with a misspelt variable loads, and shows the misspelling where it is printed.

**A fact of a relation gets no text from the declaration**
([Q-M1f.58](../../../decisions/read-out/Q-M1f.58-a-fact-reads-in-english-in-the-trace.md)): `(relation owns Person
Pet :why "a {?3} {?2} a {?4}")` reads *a Person owns a Pet*, and the fact `(owns Ann Dog)` reads by its `:source`,
its rule's `:why`, or as written, `owns(Ann, Dog)`. A fact carries no `:why` of its own: a stated fact takes
`:source` alone.

## 2. Every declaration carries a `:why`

A `(relation …)`, `(rule …)`, `(hrule …)` or `(macro …)` without `:why "…"` is refused at load, one line per form,
on every command ([diagnostics](../04-reference/07-diagnostics.md)). `--no-require-why`, which every command that loads a
program takes, loads one without; it decides whether a program loads, never what a loaded program answers
([configuration](../04-reference/06-config.md)). `:goal-text` and `:source` stay optional: a query's answer reads through
its rows, and only a statement of the problem has a sentence to quote.

A `:why` that only fills the slot is worse than none, so a `:why` says what its form says, in English:

- a relation's is the sentence its declaration reads as: `(relation owns Person Pet :why "a {?3} {?2} a {?4}")`,
  *a Person owns a Pet*;
- a rule's says what it concludes and from what: `:why "{?a} knows {?b}, so {?b} knows {?a}"`;
- a refutation's, a rule that concludes `(false)`, says what cannot be; an obligation's, one that concludes
  `(open …)`, what is still owed;
- an hrule's names the guess it proposes: `:why "guess that {?x} is in box {?b}"`;
- a made-up name, `p` or `r1`, is said as what it is, `"{?2} holds of a {?3}"`, and never given a meaning it does
  not have.

The relations the standard library and the examples declare keep words written to read one fact of them, so many
read stiffly as their declaration: `(relation drink-loc Drink House :why "{?3} is drunk in {?4}")` reads *Drink is
drunk in House*. Their rules' texts are the house style.

## 3. A fact in English: the fallback ladder

A fact has no text of its own: a `:why` reads the form it is written on, and a fact of a relation is no
declaration. What a fact reads by is its **reason**, why it holds, read off its **first recorded justification**:

| the first justification | the reason | the trace writes |
|---|---|---|
| the program states it, with a `:source` | the program's `:source` | the fact as a premise, `` `R(a, b)` — from <source> `` |
| the program states it with none (a background fact) | none | the fact as a premise, `` `R(a, b)` `` |
| a rule's firing | the rule's `:why`, filled with that firing's bindings | the step's quote, `> <reason>`, then `` Derives `R(a, b)`. `` |
| a hypothesis | none: the commitment, which the read-out names | the state's `Assuming **(R a b)**.`, before its steps |

**The ladder.** A fact reads from the program's texts where it has one: its `:source`, its rule's `:why`. Where it
has none it reads as written, `R(a, b)` in the trace and `(R a b)` where Ein text is printed, and **never as
invented prose**: there is no built-in vocabulary that turns a relation into a verb. `ein render kb` labels a fact's
arrow from the same place, a given's `:source` and a derived fact's rule.

**A declaration reads by its own `:why`**, the one fact a `:why` is written on, however a run reaches it: the trace
prints its statement after it on a premises line, where a rule reads it, and on a *Derives* line, where a rule
concludes it. The companion `(relation owns)` the loader stores beside a typed declaration is no form the program
wrote, and reads as written; an untyped declaration, `(relation done :why "…")`, is its own form, `{?2}` its name.

**First recorded.** A fact may be reached several ways, and which arrives first can move with the order rules fire
in. A reason is an explanation, as a core is: it is not part of the answer, and within one run the order is fixed, so
the same run writes the same reasons
([Q-M1f.58](../../../decisions/read-out/Q-M1f.58-a-fact-reads-in-english-in-the-trace.md)).

Steps of `ein solve examples/zebra2.ein --trace OUT.md`, held to a run of that command. A rule the program does not
list in an import prints by its full name, `std.bijection.domain-elimination`, and one it lists by the name it
writes, `includes`; the declaration `std.bijection.domain-elimination` reads is followed by its statement, the
program's `(relation nation-loc Nationality House :why "the {?3} lives in {?4}")`:

<!-- trace: examples/zebra2.ein -->
```text
### Step 27 — `includes`

> is-a* includes is-a: from House →is-a→ Attribute, derive House →is-a*→ Attribute.

Premises: `is-a(House, Attribute)`

Derives `is-a*(House, Attribute)`.

### Step 249 — `std.bijection.domain-elimination`

> every House except House-1 excluded for Norwegian → (nation-loc Norwegian House-1).

Premises: `std.algebra.functional(nation-loc)`, `std.algebra.total(nation-loc)`, `relation(nation-loc, Nationality, House)` — the Nationality lives in House, `is-a(Norwegian, Nationality)`, `is-a(House-1, House)`

Derives `nation-loc(Norwegian, House-1)`.

### Step 253 — `adjacent-via-fwd`

> (nation-loc Norwegian House-1) and unique-next-to neighbour House-2 ⟹ (color-loc Blue House-2).

Premises: `nation-loc(Norwegian, House-1)`, `next-to(House-2, House-1)` — from condition (10)

Derives `color-loc(Blue, House-2)`.
```
<!-- /trace -->

A premises line names its premises' sources once, after the last premise: `— from condition (10)` is the source of
`next-to(House-2, House-1)`, the one premise that has one.

**What is not rendered.** The read-out's `(model …)` blocks and its `as :expect` line are Ein text a claim can be
pasted from, so they stay s-expressions, as `ein solve`'s stdout and the dump's `answer.txt` do. A saved state holds
no fact of the state, only its program and the choices made, so it carries the texts as the program wrote them and
renders none ([`10-answer-dump.md`](../04-reference/10-answer-dump.md)). `--trace` writes markdown, so there a text's
line break is a hard break and the line after it stays in its block, a rule's `:why` in its quote
([Q-M1f.109](../../../decisions/read-out/Q-M1f.109-a-texts-line-break-in-the-trace-is-a-markdown.md)).

A bare `ein solve` renders no fact, so the texts cost nothing there.

## 4. The four registers

A fact of any relation reads by its reason, its `:source` or its rule's `:why`, whoever wrote that, and otherwise as
written. What depends on who could have written it is the text a relation's declaration reads by, and every
relation is in one of four registers:

| register | the relation is | who can give its declaration a text |
|---|---|---|
| **puzzle** | declared by the program or a module of its own it imports, or the name of one of their rules (an activator) | the program, in the declaration; an activator has none |
| **stdlib** | the name of a standard library rule (an activator, `(bijective color-loc)`), or a relation a standard library rule concludes without declaring it | nobody: no form declares it |
| **kernel** | a name the engine reserves: `__closed__`, `absent`, `and`, `eq`, `false`, `neq`, `not`, `open`, `or`, `relation` | the engine, which does not (§ 6) |
| **undeclared** | written or derived, never declared, and no rule's | nobody, until the program declares it |

So a fact that reads as written says where its text would come from: a program that wants a fact read gives it a
`:source` where it states it, or a `:why` on the rule that derives it; an activator of a standard library rule,
`(symmetric next-to)`, is a stated fact like any other.

## 5. Where a template has nowhere to live

- **A macro's `:why`** is required, like the others, and rendered nowhere. A macro is expanded at load, and a fact's
  justification names the rule whose `:match` used it, whose `:why` says what the pattern meant there. The macro's
  `:why` documents the pattern for a reader of the program
  ([Q-M1f.59](../../../decisions/read-out/Q-M1f.59-a-macros-why-documents-the-pattern-and-is.md)).
- **A relation nobody declares** has no declaration to carry a `:why`, and the standard library cannot declare a
  program's relations. Its facts read as every fact does: one a rule derived by its reason, one the program states
  by its `:source` or as written
  ([Q-M1f.60](../../../decisions/read-out/Q-M1f.60-a-fact-of-a-relation-nobody-declares-has-no.md)).
- **A goal's conjuncts.** `:goal-text` is one string for the whole query; a conjunct's fact has no text of its own,
  and reads as written wherever a surface prints it
  ([Q-M1f.61](../../../decisions/read-out/Q-M1f.61-a-query-has-one-goal-text-and-each-conjunct.md)).

## 6. The domain and the engine: whose words are whose

**Every text a program writes is the domain's**: the program's `:why`s, `:source`s and `:goal-text`s, and the
standard library's, which is a program. The engine fills them and never edits them. **The words around them are the
engine's**: the read-out's labels ([the read-out reference](../04-reference/08-read-out.md)), the trace's headings and its
*Derives*, *Premises* and *Assuming*, the provenance words `ein saturate --dump` writes (`:source`, `:rule`,
`:hypothesis`), the separators (`: `, ` — `), and the verdict and protocol words the engine reserves
([the engine's reserved strings](../../ein.rs/07-reserved-engine-strings.md)). The engine owns no domain
word: it has no sentence for a fact no form gives one, and a fact of a kernel relation, `not` or `relation`, reads as
written, but for a declaration, which reads by the `:why` the program wrote on it. A word the engine adds to an output is the engine's, listed on one of those two pages, and never
part of a vocabulary for the domain.

## 7. What is checked

- Every `{?ref}` in a `:why` of the standard library, and of every program the project ships as an example or a
  test, names something its form can fill: a relation's, an arrow of its declaration past the first, `{?2}` to
  `{?n+2}` for `n` labels; a rule's or an hrule's, a variable it binds (a parameter, or a positive premise's, macros
  expanded); a macro's, a parameter. None writes `{x}`, a reference with its `?` missing, and no declaration writes
  `{?1}`, the atom `relation`, which no text needs.
- A declaration's `:why` reads its own arrows where the trace prints the declaration, and a fact of the relation
  reads by none of it.
- No line of a dump's `answer.txt` carries an unfilled `{?`, over every corpus program the test suite dumps.
- Each text is printed byte for byte on every surface that prints it, Cyrillic, CJK, an emoji and every escape among
  them, and `--no-require-why` decides loading alone.
- § 3's block is lines of the trace its marker's run writes, and § 4's `kernel` register is the engine's reserved
  names, name for name.

The test suite holds this page to the binary.

## See also

- [Relations](02-relations.md) and [rules](05-rules.md): where the `:why` templates are written.
- [Facts](03-facts.md): the record a reason is read from.
- [The read-out reference](../04-reference/08-read-out.md): every row `ein solve` prints, the projection rows among them.
- [The answer dump](../04-reference/10-answer-dump.md): a saved state, its program and the choices made.
