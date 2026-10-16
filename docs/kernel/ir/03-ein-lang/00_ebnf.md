# The ein-lang grammar, in EBNF

> **This is the source of truth for what parses.**
> [`01_grammar.md`](01_grammar.md) explains intent and structure — the
> terminal table, the form set, what each declarator means. This file is the
> grammar.
>
> The engine parses by **recursive descent**
> ([`ein-ir/src/{lex,parse}.rs`](../../../../ein.rs/crates/ein-ir/src/parse.rs)),
> which is an implementation and not a specification, so the grammar is
> stated here. It is checked against that parser, and
> [§5](#5-what-keeps-this-document-honest) is what pins it: a 78-case decision
> table.

**Notation** — W3C EBNF, as in the XML specification:

| | |
|---|---|
| `A ::= B` | production |
| `'x'` | literal, matched over the *character* stream |
| `A B` | concatenation · `A \| B` alternation · `( … )` grouping |
| `A?` `A*` `A+` | zero-or-one, zero-or-more, one-or-more |
| `[a-z]` `[^"\\]` | character class, and its complement |
| `A - B` | matches `A` but not `B` |
| `/* … */` | comment |

Two layers, and the boundary between them is not the usual one — see
[§3](#3-why-this-is-not-a-plain-cfg-over-a-token-stream) before assuming a
separate scanner pass.

## §1 Lexical grammar

Terminals, over the character stream. Every one is ASCII-anchored: a
non-ASCII character can appear only inside a `STRING` or a comment.

```ebnf
Char          ::= /* any character */

/* Trivia — skipped everywhere between tokens, never a token itself. */
Trivia        ::= Space | LineComment | BlockComment
Space         ::= /* one character with Unicode White_Space, plus U+001C-U+001F */
LineComment   ::= ';' [^#xA]*
BlockComment  ::= '#|' ( Char* - ( Char* '|#' Char* ) ) '|#'   /* non-nesting */

/* Atoms. */
SYMBOL        ::= ( '__' )? [A-Za-z] [A-Za-z0-9_*.-]*
                  /* …and the match is rejected outright when the input at
                     this position begins with one of the twelve
                     LEXER_KEYWORDS followed by a non-word character (or by
                     end of input). The lookahead is START-anchored, so
                     `std.rule` is one SYMBOL and `rule.x` is not a SYMBOL at
                     all. */
LEXER_KEYWORDS ::= 'not' | 'and' | 'or' | 'neq' | 'rule' | 'hrule'
                 | 'query' | 'solve' | 'config' | 'trace' | 'macro' | 'import'
                  /* Twelve since M1e2 S1e2.1a.4 added 'solve'. */
                  /* This production was called RESERVED until M1e S1e.4.2,
                     and so was a DIFFERENT set in ein-core — the nine names a
                     declarator may not BIND, which is where `open` lives.
                     Reading the two together said the lexer's eleven had
                     become twelve (they did, with 'solve', which is a
                     declarator and not `open`). They overlap in four (`not` `and` `or`
                     `neq`) and neither contains the other:
                     06_reserved_names.md § What is reserved, and by which
                     set has the table.

                     `relation` is deliberately NOT here: rules match
                     `(relation ?R ?A ?B)` patterns, so it must lex as an
                     ordinary SYMBOL. `(relation R (T1 T2))` therefore parses
                     — as a fact — and the LOADER rejects it. Same for
                     `open`, whose `(open ?R)` is an ordinary fact head. */
WordChar      ::= /* `_`, or one character with Unicode Alphabetic or a
                     numeric general category (Nd, Nl, No) — the class the
                     word boundary after a LEXER_KEYWORD tests */

VAR           ::= '?' [A-Za-z] [A-Za-z0-9_*-]*    /* no '.', unlike SYMBOL */
KEYWORD       ::= ':' [a-z] [A-Za-z0-9_-]*        /* lower-case first character */
WILDCARD      ::= '_'
                  /* …only where the next character is not [A-Za-z0-9_], so
                     the leading `__` of a dunder atom is never two wildcards. */
EQ            ::= '='
RANGE         ::= [0-9]+ '..' ( [0-9]+ | '*' )    /* digit-anchored */
INT           ::= '-'? [0-9]+
STRING        ::= '"' ( [^"\\] | '\\' ( Char - #xA ) )* '"'
```

`(` and `)` carry no terminal name: they appear as literals in §2, which is
the only place they occur.

Four terminal facts that are load-bearing and easy to lose:

1. **`SYMBOL`'s reserved-word exclusion is a negative lookahead with a word
   boundary**, not a set difference. `neq` is not a `SYMBOL`; `neq_test`
   is. `rule-x` is not (the `-` is not a word character, so the boundary
   holds and the exclusion fires); `rulex` is.
2. **`__dunder__` atoms lex as `SYMBOL`.** `(?:__)?` still requires a letter
   after it, so a bare `_` stays `WILDCARD` and `_x` is neither. `WILDCARD`'s
   own lookahead is what stops `__closed__` from starting as two wildcards.
3. **`RANGE` is tried before `INT`** — both start on a digit — and it never
   collides with a dotted `SYMBOL`, which is letter-anchored.
4. **`STRING` takes a raw newline but not an escaped one.** `[^"\\]` accepts
   `#xA`; `'\\' ( Char - #xA )` refuses it: a backslash escapes any
   character but a newline. Unescaping
   is *minimal*: `\n` `\t` `\r` map, every other `\X` is `X`, and there is no
   `\xNN`, no `\uNNNN`, no octal — so `"\d"` is `d`.

## §2 Phrase grammar

`Program` is the start symbol. Trivia may appear between any two terminals.

```ebnf
Program       ::= Form*

Form          ::= RelationDecl
                | RuleDecl | HruleDecl
                | QueryForm | SolveForm | ConfigForm
                | TraceForm
                | MacroDecl | ImportForm
                | FactForm                        /* anything else */

/* ── Declarators (the closed reserved set) ───────────────────────── */
RelationDecl  ::= '(' 'relation' SYMBOL SYMBOL* KwPair* ')'
RuleDecl      ::= '(' 'rule'  SYMBOL RuleParams KwPair+ ')'
HruleDecl     ::= '(' 'hrule' SYMBOL RuleParams KwPair+ ')'
RuleParams    ::= '(' VAR* ')'                    /* MANDATORY; `()` when non-generic */
QueryForm     ::= '(' 'query'  KwPair+ ')'
SolveForm     ::= '(' 'solve'  KwPair* ')'        /* `(solve)` is valid: the default lattice */
ConfigForm    ::= '(' 'config' KwPair* ')'        /* `(config)` is valid: all defaults */
MacroDecl     ::= '(' 'macro' SYMBOL MacroParams Value ')'
MacroParams   ::= '(' VAR+ ')'
ImportForm    ::= '(' 'import' SYMBOL KwPair* ')' /* SYMBOL is a dotted logical name */

/* ── Facts: the flat default, i.e. any non-declarator head ───────── */
FactForm      ::= EqFact | NotForm | GenericFact
EqFact        ::= '(' EQ Value Value KwPair* ')'
GenericFact   ::= '(' SYMBOL Value* KwPair* ')'

/* ── Trace: engine output, same IR as input ──────────────────────── */
TraceForm     ::= '(' 'trace' TraceEvent* ')'
TraceEvent    ::= StepDecl | BranchOpen | BranchClose
                | BranchRef | ContradictionDecl | SymmetryDecl
StepDecl          ::= '(' 'step'            SYMBOL KwPair* ')'
BranchOpen        ::= '(' 'branch-open'     SYMBOL KwPair* ')'
BranchClose       ::= '(' 'branch-close'    SYMBOL KwPair* ')'
BranchRef         ::= '(' 'branch-ref'      SYMBOL ')'
ContradictionDecl ::= '(' 'contradiction'   SYMBOL KwPair* ')'
SymmetryDecl      ::= '(' 'symmetry-class'  SYMBOL KwPair* ')'

/* ── The value sub-language (the interior of every KwPair) ───────── */
KwPair        ::= KEYWORD Value                   /* KEYWORD is never a Value */
Value         ::= NotForm | AndForm | OrForm | NeqForm
                | GenericList
                | SYMBOL | VAR | WILDCARD | INT | RANGE | STRING
GenericList   ::= '(' ')'
                | '(' ListHead ListItem* ')'
ListHead      ::= SYMBOL | VAR | WILDCARD | EQ    /* never KEYWORD */
ListItem      ::= Value | KwPair

/* ── Kernel meta-primitives (shape-pinned) ───────────────────────── */
NotForm       ::= '(' 'not' Value KwPair* ')'
NeqForm       ::= '(' 'neq' Value Value ')'
AndForm       ::= '(' 'and' Value+ KwPair* ')'
OrForm        ::= '(' 'or'  Value+ KwPair* ')'
```

`NotForm` carries trailing `KwPair*` so that a *derived* `(not …)` fact's
provenance (`:rule type-exclusivity :using (c10)`) attaches naturally; in
`:match` / `:assert` position the loader rejects them.

## §3 Why this is not a plain CFG over a token stream

The lexical and phrase layers are **not** separated by a scanner pass. The
parser tokenizes *contextually*: at each position it asks "does the terminal I
want start here?", and a word may lex differently depending on what is being
parsed. `ein-ir` commits by recursive descent, and settles a word both
readings could take by trying the **literal reading before the `SYMBOL`
reading** at every head position where both could match — the reason
`LITERALS` is a data table in `lex.rs` and not a `match` arm.

The consequence to keep in mind when reading §2: a quoted literal like
`'rule'` is a *character* match, not a token class, and the input `(rule.x A)`
is a **parse error** rather than a fact with head `rule.x` — `rule.x` is not a
`SYMBOL` (start-anchored exclusion), and the literal `rule` matches instead,
after which `.x` scans as nothing. Symmetrically, `(std.rule A)` is a fact:
the exclusion never fires because the input does not *begin* with a reserved
word.

## §4 What the grammar deliberately does not enforce

Everything below parses and is rejected later, by the loader
([`from_ir.rs`](../../../../ein.rs/crates/ein-ir/src/from_ir.rs)) or by the
compiler. A parse error and a load error are different exit paths with
different messages, and the corpus has a fixture directory for each
([`examples/broken/`](../../../../examples/broken/)).

> **What those messages look like is specified, and it is strange.** A parse
> error reports `-1:-1` at EOF, a ±40-character context window applied before
> the line is trimmed, and a column held back past any pending
> whitespace-or-comment match. A loader error about a top-level form ends
> `at None`. All five are defined behaviour, stated in
> [`defined_behaviour.md` §1](../../defined_behaviour.md), and every
> `.expected` file under `examples/broken/` is baselined against them.

- **Which keywords each form requires** — `:match` / `:assert` in a rule,
  `:goal` in a query, `:rule` / `:using` / `:derives` in a step. Since M1c
  [S1c.1.2](../../../history/m1c_external_validation/README.md#s1c12--how-a-program-states-what-it-expects)
  a `(query …)` also has a keyword **allow-list**, so an unrecognised one is a
  load error rather than an ignored pair — the grammar still admits any
  `KEYWORD Value`.
- **The shape of a `:expect` value.** `(model <fact>*)`,
  `(or (model …) …)` and `(false)` are read structurally by the loader; to the
  grammar they are an ordinary `GenericList` with a `SYMBOL` head and an
  `OrForm`, which is exactly why they cost no production here. The shape the
  form was first proposed with — a bare list of facts, `:expect ((p A) (q B))`
  — is **not** in this grammar: `ListHead` is a `SYMBOL`, `VAR`, `WILDCARD` or
  `EQ` and never a list, so admitting it would have changed the shape of every
  form to buy one keyword its ergonomics. Also loader-checked: that an
  expectation's facts are **ground**, that the relations it names exist, and
  that it names the goal's.
- **Arity, ground-vs-pattern, and type-checking** of a fact's arguments
  against its `(relation …)` signature.
- **Unbound variables**: a `:assert` variable that no `:match` premise binds.
- **`(relation R (T1 T2))`** — the wrapped-signature form, which parses as a
  `GenericFact` because `relation` is a `SYMBOL`.
- **The computed-predicate registry** (`eq` / `neq` — and their *arity*,
  which for `eq` is the compiler's since M1e S1e.2.1), and the `__closed__` /
  `__symmetric__` dunder triggers, which are ordinary atoms here and kernel
  meaning at load
  ([`06_reserved_names.md`](06_reserved_names.md)).

## §5 What keeps this document honest

A specification with no test is a wish. Two checked-in artefacts pin §3:

| | |
|---|---|
| [`ein-ir/tests/grammar_decisions.rs`](../../../../ein.rs/crates/ein-ir/tests/grammar_decisions.rs) | a **78-case decision table**, one line each, against `tests/golden/grammar_decisions.txt`. It is where every sharp edge in §1 is nailed down: `(rule-x A)` vs `(rulex A)` vs `(std.rule X)` vs `(neq_test X)`, `_` vs `_x` vs `__closed__`, `1..5` / `1..*` / `1..`, `"a⏎b"` vs `"a\⏎b"`, `()` vs `(x ())`, an unterminated block comment. Plus the four `examples/broken/*.ein` messages, whose `.expected` files are the engine's defined output — [`defined_behaviour.md` §1](../../defined_behaviour.md) |
| `corpus_shapes.md5`'s `ir[parse]` lines | every corpus file's parse, digested — the *structure* of an acceptance, not just the fact of one |

**Corpus coverage of §2, counted** (files under `examples/` + `stdlib/`
containing at least one): `relation` 90 · `rule` 88 · `query` 67 · `and` 65 ·
`import` 42 · `not` 43 · `neq` 39 · `hrule` 23 · `config` 18 · `or` 10 ·
`macro` 5 · `trace` 3 · `=` 3 · `step` 1. Terminals: `VAR` 105 · `STRING` 95 ·
`INT` 84 · dunder `SYMBOL` 11 · `RANGE` 4 · `WILDCARD` 3.

**Five productions have no exerciser at all**, and it is better to write that
down than to let a reader assume otherwise: `BranchOpen`, `BranchClose`,
`BranchRef`, `ContradictionDecl` and `SymmetryDecl` appear in no `.ein` file
in the tree, and nothing in the engine emits them — they are reachable only
through the parser's own unit tests. They are the trace vocabulary the
renderer has not needed yet ([Q21](../../../../plans/open_questions.md#q21--ir--dot-structural-isomorphism)),
kept in the grammar so that a `(branch-open …)` never silently misclassifies
as a fact.

## §6 Derived artefacts

M2's GBNF for constrained LLM decoding is derived from this grammar
([`plans/ideas/01`](../../../../plans/ideas/01-self-modifying-constraint-language.md)).
The source is §1 + §2, and the translation is by hand or by a generator that
reads this document.

