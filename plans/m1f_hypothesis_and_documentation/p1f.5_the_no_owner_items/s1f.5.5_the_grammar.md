# S1f.5.5 — The grammar revisited: each position says what it holds, and the parser is generated from it

**Phase:** [P1f.5](README.md) (The no-owner items)
**Was S1e2.5.5**, in M1e2's P1e2.5, until 2026-11-08 ([§ The ids](README.md#the-ids)).
**Estimate:** 12.75 days, re-taken on 2026-11-08 on the user's re-plan
([§ The order](#the-order-re-planned-2026-11-08)), which gave this stage the grammar parts of S1f.5.4, S1f.5.1,
S1f.5.2 and S1f.5.11. It was 10, re-taken at step 1 the same morning ([the census § The
estimate](s1f.5.5_census.md#the-estimate-re-taken)), and 8.5 before that.
- One day for the census (T1). Done.
- One day for the grammar on paper and the real programs (T2). Done, with the builder that makes today's AST.
- One day for the generated parser beside the old one (T3): the builder holds on 331 files, `Loc` for `Loc`.
  Half of the old two days for *beside and the swap*.
- A day and a quarter for recovery: the patched `lrpar`, its budget and its tests (F11, in T3). Unchanged.
- Half a day for the nesting limit (F10, in T3). Half of the old day for *the nesting limit and the tests that
  move*; the tests are the swap's.
- A quarter of a day for the swap's prerequisite (T8): the loader's four internal names renamed out of the name
  space, and the test that holds it. New: S1f.5.4's Q1.
- A day and a quarter for every difference attributed, with the review material and the stop (T9). New: it is
  S1f.5.1's, S1f.5.2's and S1f.5.11's step 1s, which were half a day, half a day and a quarter apart, done in
  one sweep; S1f.5.4's, done, is its first cross-check and its review builder is re-used.
- A quarter of a day for the fixture moves that move no answer (T10). New: Q-M1f.2 (a)'s, ahead of the swap.
- Two and a half days for the swap (T4). One for the parser in and the old one out, with the error text and its
  goldens (the old swap's half). Half a day for the tests that move (the old nesting line's other half). One for
  the register, the fixtures, the printer and the JSON writer of five rulings at once, about what the four
  stages' own register, fixture and printer work came to together. Their lexer and loader changes are not
  written at all, since the grammar states each, and one commit blesses the parse goldens once where five would
  each have blessed them.
- Two days for the grammar-pages test and the loader's refusals: 67 refusal sites move (T5). Unchanged; no
  loader line is added first for it to delete, since S1f.5.11's lists are never written into the loader.
- Half a day for `--diff-models`, which reads its file through the parser (S-15, T6). Unchanged.
- A day and a quarter for the pages (T7). Unchanged; each language stage keeps its own pages.

**Status:** specified 2026-11-06 on the user's instruction of that day; extended 2026-11-07 with the
user's name rule and instruction to generate the parser
([Q-M1e2.25](../../m1e2_correctness/open_questions.md#q-m1e225--how-strict-is-the-grammar-and-what-holds-the-parser-to-it)).
**Steps 1 and 2 done on 2026-11-08, on the user's go-ahead of that day; stopped there, to show the user
before any edit**: the census is [`s1f.5.5_census.md`](s1f.5.5_census.md), and the final grammar, the
real programs with their answers and the two questions are [`s1f.5.5_paper_grammar.md`](s1f.5.5_paper_grammar.md).
No engine code, test, golden or fixture has moved. **The user read it the same day and ruled both questions**
([Q-M1f.1](../open_questions.md#q-m1f1--g-14-is-a-bad-conclusion-that-never-fires-refused) (b),
[Q-M1f.2](../open_questions.md#q-m1f2--the-55-files-the-grammar-refuses-that-todays-parser-accepts) (a)),
overruling none of its ten defaults, so the paper grammar stands. **The generator is grmtools, ruled by the user on 2026-11-07** for its features
([§ The generator](#the-generator-grmtools-ruled)), **with its error recovery on, ruled the same day**: this
stage makes recovery deterministic first, with a patch to `lrpar` ([F11](#f11-grmtools-recovery-is-not-deterministic-and-this-stage-makes-it-so)).
**Re-planned on 2026-11-08, on the user's ruling: the generated parser first**
([§ The order](#the-order-re-planned-2026-11-08)). Until then S1f.5.3, S1f.5.4, S1f.5.1 and S1f.5.2 were to land
on the hand-written parser before T3. Since then T3 runs next, the grammar parts of
[S1f.5.4](s1f.5.4_digits_are_names.md), [S1f.5.1](s1f.5.1_bare_underscore_refused.md),
[S1f.5.2](s1f.5.2_strings_are_natural_language.md) and [S1f.5.11](s1f.5.11_closed_keywords.md) are this stage's
T9 and T4, credited to each by name, and each of the four keeps its id and its non-grammar part, which runs after
the swap.
**Depends on:** the user's choice of generator and rulings on what each position holds, all given. Its steps 1
and 2 ran first. Of the language stages only [S1f.5.3](s1f.5.3_range_retired.md) ran on the hand-written parser,
and it is done (2026-11-08); since the re-plan the swap waits for no other stage.

## The instruction

The user, 2026-11-06: *"We have to revisit Ein grammer; previously when where using parser generators
and EBNF it was easier, now seems that our hand-written parser is way too permissive"*.

The user, 2026-11-07, on route A or B: *"propose parser generator in Rust"*.

## The case

**The grammar is too permissive, and the parser inherited that.** Taken on 2026-11-06.

The generated parser's `grammar.lark` (deleted at `4c1a5b39`) holds the same rules the hand-written
one implements: the same `SYMBOL`, `WILDCARD`, `RANGE`, `INT` and `STRING` regexes, `%ignore /\s+/` with
no separator required, and one `?value` for every position:

```text
?value: not_form | and_form | or_form | neq_form | generic_list
      | SYMBOL | VAR | WILDCARD | INT | RANGE | STRING
```

`00_ebnf.md`'s `Value` is that production, and M1a's S1a.1.1 built the hand-written parser *at
accept/reject parity* with the old one (`2168f430`). So `1a` splitting and `(r _ ?x)` parsing are not
what the rewrite introduced.

**What the rewrite did lose is the single source of truth.** Under a generator the grammar file *was*
the parser. Now there are three statements: `00_ebnf.md`, `lex.rs` and `parse.rs` (1 547 lines), and
`from_ir.rs` (2 835 lines; `wc -l`). Nothing derives one from another, and what holds the parser to the
page is a 78-case table, a keyword sweep and the corpus's digests (`00_ebnf.md` § 5).

**One `Value` for every position pushes the real grammar into the loader.** A fact's argument, a
pattern's argument, a keyword's value and a guard's argument all parse alike, so *what a position may
hold* is decided after the parse, one refusal at a time, each added when a defect found it: 46 lines
of refusal text and 19 `Found::` sites in `from_ir.rs` (the agent's grep), and some shapes nothing
checks. This review's items are nearly all that shape:

| item | what the grammar lets through |
|---|---|
| W18, W20, W24 | fixed by refusals in the loader, one shape at a time |
| W-22, F3 | `_` as any value |
| F4 | a string as any value |
| F6 | a range as any value |
| F7 | an int as a value of its own kind |
| G-6 | a variable in a fact |
| F2 | no separator between tokens |
| G-15, G-22 | a keyword pair beside a pattern, dropped |
| G-21 | a conclusion of a computed predicate |
| G-14 | `:assert` checked only when it fires |

**M2 depends on it.** M2's formalizer constrains the LLM with a GBNF *generated from* `00_ebnf.md`
([S2.2.3](../../m2_nl_to_ir/p2.2_formalizer/s2.2.3_gbnf.md)), which README's plan calls making valid IR
free. It makes only what the grammar calls valid free: `(r _ ?x)`, `(p 1..5)` and `(p 1a)` are all
valid today, so a model can emit each.

**F10: deep nesting aborts.** `ein solve` on a fact nested 100 000 deep exits 134 with *has overflowed
its stack*, where 10 000 exits 0 (2026-11-07, release build). The corpus's deepest program nests 8.

## The name rule

The user, 2026-11-07: *"`0`, `1` - any alphanumeric plus connectors signs (`_`,`-`) plus decorators
signs (`$`, `%`, `^`, `*`, ...) string without space considered a NAME, except variables that marked with
`?` that sits in front of the name"*.

The user, the same day, on the agent's readings: *"`?` and `:` - only for variables and kvargs, not
allowed in other places; `?_` - refused; `;`,`#`,`|` - for comments, also not allowed in names; `,` commas
also not allowed; better specify list of allowed chars in name; names STAY ASCII FOREVER! никаких Жуков в
коде! В NL строках - пожалуйста, в строках что угодно, но не в коде. `:priority` planned to be deleted in
plans/m1f_hypothesis_and_documentation/p1f.5_documentation_and_other/s1f.5.6_rule_priority.md"* (the
Russian: *no "Жук"s in the code; in natural-language strings, anything, but not in code*).

The user, the same day, on the agent's list: *"escape char `\` is forbidden in names, but others ' ` [ ] { }
lets allow for now, so 'name' is an atom with name = "'name'", or f' is also an allowed atom and {} is also"*.

**This collapses the lexical grammar.** A token is a run of characters up to a separator. There is no
contextual tokenizing any more, and `00_ebnf.md` § 3, *why this is not a plain CFG over a token stream*,
goes with it. So do `SYMBOL`'s lookahead and § 1's four load-bearing facts. It is a plain grammar over a
token stream, which is what makes a generator fit.

**A name is spelled from a list of allowed characters**, as the user asked, and never from a list of
excluded ones:

```text
Letter    ::= [A-Za-z]
Digit     ::= [0-9]
Connector ::= '_' | '-' | '.'
Decorator ::= '$' | '%' | '^' | '*' | '+' | '=' | '<' | '>' | '!' | '&' | '@' | '~' | '/'
Bracket   ::= "'" | '`' | '[' | ']' | '{' | '}'       /* allowed for now: the user, 2026-11-07 */
NameChar  ::= Letter | Digit | Connector | Decorator | Bracket
NAME      ::= NameChar+, except a reserved word or a bare _
VAR       ::= '?' NameChar+, except ?_
KEYWORD   ::= ':' NameChar+, and only one the form takes
STRING    ::= '"' … '"'     any Unicode, with escapes (S1f.5.2)
```

Every other character is out of a name:

| out of a name | because |
|---|---|
| `?`, `:` | ruled: the front of a variable and of a keyword, and nowhere else |
| `;`, `#`, `\|` | ruled: comments |
| `,` | ruled |
| `(`, `)`, `"` | structure and strings |
| `\` | ruled: the escape character |
| space, tab, a control character | they separate tokens or are refused |
| anything outside ASCII | ruled: **names are ASCII forever**. Any Unicode belongs in a string (S1f.5.2) |

The list only widens what a name may hold today: today's `SYMBOL` is a letter followed by
`[A-Za-z0-9_*.-]*` (`00_ebnf.md` § 1), and the corpus writes five characters besides letters and digits,
`-`, `_`, `.`, `*` and `=` (390 files under `examples/`, `tests/`, `stdlib/` and `corpus/`, comments and
strings stripped, 2026-11-07). Of the decorators, `$`, `%`, `^` and `*` are the user's. The other nine are
the agent's reading of the user's *"..."*, which the user left standing.

**A bracket or a quote is a plain name character, for now.** None of them means anything: `'name'` is the
name `'name'`, quotes included, and differs from `name`; `f'`, `{}`, `[a]` and `` `x` `` are names too. They
are allowed *for now*, so a later ruling may give one a job; the corpus writes none of them today.

What it moves, on the corpus: **nothing in how a token reads**. Every token of the 474 `.ein` files in
the tree is already a `SYMBOL`, `VAR`, `KEYWORD`, `INT`, `RANGE`, `STRING`, `_` or `=` (`census.py`,
2026-11-07). The names it adds are names no program writes yet.

| written | today | after | because |
|---|---|---|---|
| `0`, `01`, `1st`, `1_a`, `1-2`, `1.5` | int, split or refused | a name | the ruling |
| `$x`, `%`, `^`, `a^b`, `*`, `-`, `+1` | refused | a name | the ruling |
| `-7` | an int | **a name**, and `(p -7)` and `(p 7)` differ | the ruling; `-` is a connector |
| `1..5`, `1..*` | a range | **a name**, with no range reading | the ruling; `.` is a name character today (`std.elim`). S1f.5.3's default 1 is withdrawn |
| `neq-1`, `rule-x`, `rule.x`, `not-a` | a keyword and the rest, or refused | **a name** | a reserved word is a whole token. This overrides [Q-M1e2.17](../../m1e2_correctness/open_questions.md#q-m1e217--where-does-a-keyword-end) (i)(a) |
| `=` | its own token | a name | `(= a b)` is then a fact of a relation named `=`; the loader's reading stays |
| `__`, `___`, `_-`, `_*` | refused | a name | S1f.5.1's default 1 is withdrawn |
| `'name'`, `f'`, `{}`, `[a]` | refused | a name, quotes and brackets included | the ruling, *"for now"* |
| `a\b` | refused | refused | `\` is the escape character |
| `_` | the wildcard | refused | [Q-M1e2.22](../../m1e2_correctness/open_questions.md#q-m1e222--does-ein-need-a-wildcard) |

**What the user ruled on the agent's readings**, 2026-11-07:

1. **`?` and `:` mark the front of a variable and of a keyword, and appear nowhere else.** So `a?x`,
   `?x?y`, `??x` and `a:b` are refused. Read as names, `(r a?x)` would hide a missing space and silently
   turn a variable into a constant.
2. **`?_` is refused**, as `_` is. Two uses of it in one rule would bind the same value, which is the
   opposite of what the spelling suggests. `?__` and `?_x` are variables. The probe's draft does not yet
   do this.
3. **`;`, `#`, `|` and `,` are not name characters.**
4. **Names are ASCII, forever.** `Жук` is a parse error as a name. Any text may go in a string, which
   S1f.5.2 confines to the natural-language keywords, such as `:why`. This replaces [S1f.5.2](s1f.5.2_strings_are_natural_language.md)'s default 3, which had
   left widening names for later.
5. **`:priority` is [S1f.5.15](s1f.5.15_rule_priority.md)'s
   to delete**, so this phase gives it nothing new. Until then its value is a run of digits, as every
   corpus use is (`grep -rhoE ':priority +-?[^ )]+'` over the four roots, 2026-11-07). The by-position
   grammar refuses any other value as a by-product. Today the loader drops a value that is not an int
   in silence (`from_ir.rs`, the `priority` match), and reads `-5` as an int, which no program writes.

## What it changes

**A grammar by position.** `Value` goes, and each position gets the production for what it holds:

```text
Name       ::= NAME | 'relation' | 'step'
FactArg    ::= Name | '(' Name FactArg* ')'                 /* no variable, no _, no string */
Fact       ::= '(' FactHead FactArg* FactClause* ')'        /* FactHead: a Name other than relation */
Term       ::= '(' 'and' TArg+ ')' | '(' 'or' TArg+ ')' | '(' 'not' TArg ')'
             | '(' 'neq' Item Item ')' | '(' (Name | VAR) Item* ')'
TArg       ::= VAR | Term                                   /* (not ?h), (and ?G (absent ?B)) */
Item       ::= Name | VAR | Term
RuleClause ::= ':match' Term | ':assert' Term | ':why' STRING | ':priority' DIGITS
```

The probe's draft holds the whole of it, in grmtools' notation
([`ein.y`](s1f.5.5_parser_probe/c_grm/src/ein.y)) and in five others, the shortest being LALRPOP's 80
lines. It accepts every puzzle, stdlib module and `tests/` program as written. Three shapes real programs
write corrected the first draft: `:hrules (guess (color-loc Color House) guess …)`, `(not ?h)` in an
`:assert`, and `:using (and s1 c15)`.

**Closed keywords.** Each form names the keywords it takes, with what each holds. An unknown keyword is
refused at the parse, rather than dropped. **The lists are ruled** (2026-11-07,
[Q-M1e2.28](../../m1e2_correctness/open_questions.md#q-m1e228--which-keywords-does-each-form-take)), and
[S1f.5.11 § The lists](s1f.5.11_closed_keywords.md#the-lists) states them. Here each keyword becomes a token of
its form at the swap, credited to S1f.5.11's ruling; since the re-plan of 2026-11-08 the lists are never
written into the loader first, and the lines the loader has today, for `query`, `solve` and `config`, go in T5.
A term and a nested fact take none. The draft's query takes `:no-hypothesis`, which the list does not, and step 2 drops
it. Which keywords a form *requires*, and that each appears once, stays the loader's, since a grammar can
state either only by listing every order.

**A token ends at a separator.** A space, a paren, a comment or the end of input ends a token, which
settles F2. Under a longest-match lexer this follows from one catch-all bad token, plus a second for a
string glued to what follows, rather than from a check on each token.

**Refusals move up.** A shape the grammar excludes needs no loader line, and those lines go. What stays in
the loader is what a grammar cannot see: arity against a signature, a variable no earlier premise binds,
*forbid*.

**A nesting limit.** A stated limit, refused at the parse with a line, closes F10. The LR parser itself
survives any depth, but macro expansion, the loader and the dumper recurse. Default: 1 000, which step 3
checks on a debug build.

**Unused productions go.** `BranchOpen`, `BranchClose`, `BranchRef`, `ContradictionDecl` and `SymmetryDecl`
have no exerciser (`00_ebnf.md` § 5). Step 1 settles the rest of `(trace …)`.

## The generator: grmtools, ruled

The user, 2026-11-07: *"Let's take grmtools now because of features, if speed will ever become an issue -
will add super-optimized hand-written parser"*.

**What is chosen.** grmtools 0.15: `lrlex` for the tokens and `lrpar` for the productions, with LR(1)-power
tables by Pager's method. The grammar is two files, `ein.rs/crates/ein-ir/src/ein.l` and `ein.y`, in Yacc's
notation with one Rust action per alternative. The shipping parser is generated from them at compile time,
by `CTParserBuilder` in `ein-ir`'s `build.rs`, so a conflict in the grammar fails the build. The same two files
load at run time through `RTParserBuilder`, which is F2's to use and nothing in this stage does. Step 2 starts
from the probe's draft, [`ein.l`](s1f.5.5_parser_probe/c_grm/src/ein.l) and
[`ein.y`](s1f.5.5_parser_probe/c_grm/src/ein.y).

**Why.** It supports the most of what Ein's plans ask of a parser
([§ Which supports the most](#which-supports-the-most)). An ambiguous grammar fails its build, `cfgrammar` and
`lrlex` read its two files through public APIs, and the grammar loads at run time: 390 of 390 corpus files
agree with the compiled build. Its error recovery is the strongest of the six, and is on: a refused file
reports every error recovery reaches, each with ranked repairs, the same on every run
([F11](#f11-grmtools-recovery-is-not-deterministic-and-this-stage-makes-it-so)).

**What it costs, accepted with the choice.**

- **Speed.** 2.1 to 2.8 ms on `zebra2.ein`, 27 KB, against today's 0.165 ms. 1.9 ms of it is `lrlex`'s lexing.
  If speed ever matters, the user will add a super-optimized hand-written parser. That parser would be held
  to the grammar as T1f.5.5.3 holds the generated one to the old: both run over the corpus and the fuzzer,
  and the generated one is the reference. It need not recover: a file it refuses goes to the generated
  parser for the report, which costs nothing on a file that parses. Nothing here builds it.
- **41 crates at run time, 51 with the build**, and about 24 s on a clean build alone at `-j 4`.
- **Two files in Yacc's notation**, 320 lines in the draft with its actions. A token no production uses, such
  as the catch-all for a bad token, is declared under `%expect-unused`.

**It meets the gate.** The generated parser builds under `#![forbid(unsafe_code)]`, and its own module says
`#![deny(unsafe_code)]`. It passes clippy at `-D warnings` and holds no `HashMap` (the probe's generated
`ein.y.rs`, 9 213 lines, 2026-11-07). Rustdoc at `-D warnings` is step 3's to check.

### F11: grmtools' recovery is not deterministic, and this stage makes it so

Found 2026-11-07, while the generator was being chosen. CPCT+'s report can differ between two runs on the
same file, for two reasons in `lrpar` 0.15:

- **The ranking.** `cpctplus.rs` de-duplicates repair sequences through a `HashSet`, then sorts them by
  length alone with `sort_unstable_by`, so repairs of equal length come out in the set's random order. Over 20
  runs on the probe's [`four_mistakes.ein`](s1f.5.5_parser_probe/four_mistakes.ein), the output came out two
  ways, 14 and 6, with `Insert :priority, Delete :bogus` and `Delete :bogus, Delete 3` swapped (the probe's
  `recover` command, 2026-11-07). The parse goes on with the first repair, so later errors can move as well.
- **The clock.** Recovery stops at a wall-clock budget of 500 ms per parse, `RECOVERY_TIME_BUDGET` in `lrpar`'s
  `parser.rs`, a constant. On a loaded machine the same file can report fewer errors.

**The first error is unaffected.** Its position and the tokens expected there come from the tables before
recovery starts, and under `RecoveryKind::None` no recovery code runs.

**Ruled the same day: recovery is on.** The user, 2026-11-07: *"no, enable recovery, let's benefit from
full-featured grmtools"*. That overrides the agent's default of recovery off. So this stage makes recovery
deterministic before it ships, with a patch to `lrpar`:

- **The ranking.** Duplicate repair sequences are dropped keeping the first of each, in the order the search
  found them, and the sort is stable (`sort_by`), so repairs of equal rank keep the search's order. The
  search itself is already deterministic: its queue is an `IndexMap`, read in insertion order
  (`dijkstra.rs`).
- **The budget.** Recovery is bounded by a count of search steps over the whole parse, not by the clock, so
  a file reports the same errors on every machine and under any load. A step is one check of the budget, at
  each node the search expands and each node the repair collection and ranking visit, which are the places
  the clock was read.
- **The budget's size.** **Proposed: 30 000 steps**, re-taken on the final grammar. On the probe's
  2 113 refused mutants (2026-11-07), half need under 70 steps, 99 % under 18 795, and 19 reach 30 000.
  The slowest file at that budget, `zebra2.ein` with every `)` deleted, takes 0.33 s and reports 71 errors,
  as many as at a million steps, which took 24 s. It takes 0.78 s at 50 000 steps and 2.0 s at 100 000,
  since a step costs more the deeper a search goes. The clock's 500 ms is the budget it replaces.
- **A cut is said.** Where recovery stops at the budget, the report says so after the last error it
  reached, as a core cut at its budget does since W23. Today's `lrpar` gives such an error no repairs,
  which reads as *no repair exists*. The shipped patch must tell the two apart, and the probe's does not
  yet.

**Tried on the probe** (2026-11-07). The patch,
[`lrpar_deterministic.patch`](s1f.5.5_parser_probe/lrpar_deterministic.patch), changes 73 lines of
`cpctplus.rs` and `parser.rs`, the probe's step printer included. Over 200 runs on `four_mistakes.ein`
the unpatched crate printed two reports, 97 and 103 times, and the patched one printed one of the two every
time. Over three runs of 5 000 seeded mutants of the corpus, 2 113 of them refused, the unpatched crate's
report differed between runs on 1 032 and the patched one's on none. The patch as tried set a million
steps, and four refused mutants spent all of it, the slowest in 9.4 s. Two builds generate the same parser
source, apart from a build-time stamp in a comment.

**How it ships.** `lrpar` 0.15.0 with the patch, vendored as `ein.rs/vendor/lrpar/` and used through
`[patch.crates-io]` in the workspace's `Cargo.toml`, so the compiled parser and a grammar loaded at run time
both get it. It is `#![forbid(unsafe_code)]` as published. `utils/check_hashmap_iteration.py` reads
`ein.rs/crates/` only; the patch's `HashSet` is probed for duplicates and never iterated. **Sending the patch upstream is the
user's call**, since it is outward-facing; if a grmtools release carries it, the vendored copy goes.

**What it costs.** Nothing on a file that parses: `lrpar` builds its recoverer at the first error. On a
refused file, recovery's time, bounded by the budget.

### The measurements

Measured on 2026-11-07 with one grammar written for six generators
([`s1e2.5.5_parser_probe/`](s1f.5.5_parser_probe/README.md), which has the commands and every number).
All six accept the same 352 of the corpus's 390 files and agree on 40 000 mutants.

| | hand-written today | LALRPOP | pest | parol | **grmtools** |
|---|---|---|---|---|---|
| grammar format | Rust code: `lex.rs` and `parse.rs` | one `.lalrpop` file: BNF productions with a Rust type and an action per alternative, macros such as `Cl<K,V>`, and a `match` block of literal and regex tokens. `build.rs` reads it | one `.pest` file of PEG rules with no actions. Rust code walks the tree it returns. A derive macro reads it, or `pest_meta` and `pest_vm` at run time | one `.par` file: EBNF with `{ }`, `[ ]` and `( )`, scanner states, and no actions. It generates the AST types and a trait the program implements | two files, Lex `.l` for the tokens and Yacc `.y` for the productions, with Rust actions in the `.y`. `build.rs` reads them, or the program at run time |
| parsing strategy | a longest-match lexer, then recursive descent. An overlap goes to the alternative tried first (`defined_behaviour.md` § 1.4) | table-driven LR(1), built by the lane-table method (LALR(1) on request). Its lexer takes the longest match and breaks ties by the `match` block's tiers; it is built from the regexes at run time, which is the 150 µs | PEG: recursive descent that backtracks and takes the first alternative that matches. No separate lexer, and no memoising | LL(k): a predictive pushdown automaton with a lookahead automaton per non-terminal, and no backtracking. A separate scanner. Its LALR(1) mode was not used | table-driven LR by Pager's method, which has LR(1)'s power in merged tables. A lex-style lexer: the longest match, then the earlier rule |
| error recovery | none: it stops at the first error, with one `ParseError` | none unless the grammar writes the `!` symbol. Where it does, the parser drops tokens, records an `ErrorRecovery` and goes on. The draft writes none. At the first error it gives the token, its position and the terminals it expected | none: it reports the furthest position reached, with the rules expected there (`positives`) and those not wanted (`negatives`) | automatic in LL mode, and on by default since 0.24: it inserts or deletes tokens, chosen by edit distance. None in LALR mode. The probe left it on | automatic, by CPCT+ (Diekmann and Tratt, *Don't Panic!*), and on by default: it reports every error in a run, each with ranked repairs (insert, delete, shift). The probe turned it off (`RecoveryKind::None`) to time the parse alone |
| target use cases, as each describes itself | Ein's language alone | *"usability as its primary goal"*: compact grammars, with macros, for a language whose grammar is fixed when the program is built | *"a general purpose parser … with a focus on accessibility, correctness, and performance"*: a grammar kept apart from the code, tried in a web editor or loaded at run time | language implementations that want their AST types generated, a top-down parse that is easy to follow in a debugger, and parsers for Rust and C# from one file | Lex and Yacc grammars brought to Rust unchanged, grammars loaded at run time, and tools that must report every syntax error with a repair |
| an overlapping alternative added | — | **build fails** | **builds in silence** | build fails | build fails |
| `zebra2.ein`, 27 KB (µs) | 165 | 240, plus 150 once per run | 1 168 | 855 | 2 756 |
| nested 1 000 000 deep | aborts | parses | refuses, with an error | aborts | parses |
| crates added, run time / with build | 0 / 0 | 5 / 45 | 15 / 20 | 41 / 101 | 41 / 51 |
| clean build, alone, `-j 4` | — | 9.9 s | 7.3 s | 18.5 s | 23.7 s |
| grammar | 1 547 lines of Rust | 80 lines, one file | 65 lines, one file | 60 lines, one file | 320 lines, two files |

The rows on format, strategy, recovery and use are read from each crate's source and README at the version
the probe locked: LALRPOP 0.23.2, pest 2.9.5, parol 5.0.4, grmtools 0.15.0. Recovery was tried for grmtools
alone. In the timings every candidate but parol parsed with recovery off or absent. parol's was left on and
refused the same files as the others, and the timed file has no error, so recovery cost nothing in the
timings.

### Which supports the most

The user, 2026-11-07: *"What parser will support more and richer features?"* Read from each crate's source
at the version the probe locked, with grmtools' two distinctive features tried on Ein's draft the same day
([the probe](s1f.5.5_parser_probe/README.md#what-it-found)).

| feature | LALRPOP | pest | parol | grmtools | peg |
|---|---|---|---|---|---|
| grammars it accepts | LR(1); LALR(1) on request | PEG, plus a stack (`PUSH`, `POP`, `PEEK`, `DROP`) for some context-sensitive shapes | LL(k), and LALR(1) | LR(1) power, by Pager's method; Yacc's | PEG; left recursion with `#[cache_left_rec]` |
| ambiguity found at build | **yes**, with an example | no | **yes**: LL(k) decidability, LALR conflicts | **yes**, conflicts counted against `%expect` | no |
| notation | parameterised macros, conditional macros, `#[inline]`, `#[precedence]` and `#[assoc]`, fallible actions, inferred types | silent and atomic rules, tags, built-in Unicode classes, implicit whitespace and comments, doc comments, a Pratt parser | scanner states (`%scanner`, `%push`, `%pop`, `%enter`), user types for tokens and productions, comment declarations | Yacc precedence (`%left`, `%right`, `%nonassoc`, `%prec`), `%parse-param`, `%epp` names for messages, `%avoid_insert` | rule arguments and generics, `precedence!{}`, `quiet!{}`, `expected!()`, input of any token type |
| error recovery | at `!` points the grammar writes | none | automatic, LL mode | **automatic, CPCT+**: 3 of 4 mistakes in one run, each with ranked repairs; deterministic with this stage's patch ([F11](#f11-grmtools-recovery-is-not-deterministic-and-this-stage-makes-it-so)) | none |
| grammar loaded at run time | no | **yes**, `pest_vm` | no | **yes**, `RTParserBuilder`: 390 of 390 files agree, 1.3 ms to build | no |
| public reader of the grammar file | no (the probe's 167-line script) | `pest_meta` | the library, and a JSON export with a schema | `cfgrammar` | no |
| tree it returns | the actions' | a generic tree | **generated, typed AST** | the actions', or a generic tree | the actions' |
| tooling | conflict reports | web editor, debugger | **a CLI of 15 tools** (decidability, k, left factoring, left recursion, first and follow sets, formatter, export, scaffold), a language server for `.par` files | none checked here | a trace feature |
| output languages | Rust | Rust | Rust and C# | Rust | Rust |

Four needs are named in the plans:

1. **Ambiguity fails the build**, the review's need (Q-M1e2.25): LALRPOP, parol, grmtools.
2. **One grammar file feeds `00_ebnf.md` and M2's GBNF**: a public reader in pest, parol, grmtools.
3. **The grammar changes at run time**, [F2](../../followups/f2_self_modifying_language.md)'s versioned
   grammars: pest, grmtools.
4. **Every error of a run, with a fix**, for the diagnostics of [M20](../../m20_gui/README.md)'s editor:
   grmtools best, with F11's patch; parol second; LALRPOP where the grammar writes `!`.

Only grmtools has all four, and the user chose it on that ground.

**Not chosen.**

- **LALRPOP**, the agent's first proposal. It has the fastest generated parse, 240 µs, and the richest
  notation, at 5 run-time crates. It has no grammar at run time and no public reader: the probe's 167-line
  script stood in for one.
- **parol** has three of the four needs and the most tooling, at 101 crates, and no grammar at run time.
- **pest** has no ambiguity check, and 7 times LALRPOP's parse time.
- **peg** keeps its grammar in a Rust macro, with no reader and no check.

**This overrides two records.** `docs/history/m1a_rust/design/04_ir_frontend.md` § 1, *hand-written, not
generated*, and `12_toolchain_and_layout.md` § 2's *Not used: any parser generator*. Those pages are a
record and do not change. Of § 1's four reasons: recursive descent was to be about 500 lines and is 1 547;
the speed is spent by the user's choice, with a hand-written parser in reserve; the codegen budget is what
this spends; and error-text control turns out not to need hand-writing, since the message pinned is
`unexpected input` at a position.

### One source

`ein.l` and `ein.y` become the grammar. A test in `ein-ir`, `tests/grammar_pages.rs`, reads them through
`cfgrammar` and `lrlex`, grmtools' public readers, and writes `00_ebnf.md`'s grammar blocks. It fails when
the page differs, and `EIN_BLESS=1` rewrites the page, as `ein-cli/tests/embedding.rs` does for
`docs/api/rust.md`. So the page cannot drift from the parser. M2's GBNF is written from the same reader at
[S2.2.3](../../m2_nl_to_ir/p2.2_formalizer/s2.2.3_gbnf.md), which decides how a reserved word stays out of a
name, since GBNF has no lookahead. Actions stay one call each, which keeps `ein.y` readable.

### The error text

`defined_behaviour.md` § 1.1 to § 1.4 are quirks the first generated parser left and the hand-written one
reproduces: `-1:-1` at end of input, a ±40-character window cut before the line is trimmed, `death_position`'s
column held back past a pending comment match, and ambiguity resolved to the earlier alternative.
**Default: all four go.** Each parse error reports its true token position and the tokens expected there,
with `%epp` giving each token a readable name. This takes
[Q-M1a.3](../../../docs/history/m1a_rust/open_questions.md#q-m1a3--parse-error-messages) option (c).

**With recovery on, a refused file reports every error recovery reaches**, in file order, each with its
ranked repairs: *insert `)`*, *delete `a?x`*. The parse goes on after each error with the first repair, as
CPCT+ does. On the probe's 5 000 mutants of the corpus, the 2 113 refused report 4 083 errors, and 923 of
those mutants report more than one (2026-11-07). The agent's readings, which stand unless the user objects:

- **A refused file stays refused.** Repairs are advice, and no repaired program is loaded, solved or
  dumped. The exit codes do not change. Loading a repaired file would accept text that nobody wrote, which is
  what the user ruled the parser too permissive for.
- **Three repairs per error, then a count.** One error had 752 repairs on the mutants, and 933 of the 4 083
  had more than three. The first is the one the parse went on with, and *and n more* says how many are left
  out.
- **Errors only from the parse.** A file with a parse error does not reach the loader, as today. The loader's
  own refusals still stop at the first.
- **Where it was cut.** A report cut at the step budget ends with a line saying so, and with the line and
  column it stopped at.

The four `examples/broken/*.expected` files and `grammar_decisions.txt` move, and step 3 lists them.
[M2](../../m2_nl_to_ir/README.md)'s formalizer can read every error and its repairs back to the model, where
one error per round costs a round per mistake. That is M2's to use, and this stage only makes it possible.

## Questions it has to settle

Each goes to the user with real programs, not in this stage's prose:
- **G-15 and G-22:** ruled 2026-11-07 with D-15
  ([Q-M1e2.28](../../m1e2_correctness/open_questions.md#q-m1e228--which-keywords-does-each-form-take)): a keyword pair inside a
  pattern, or beside an `:assert`'s conjuncts, is refused, as the draft refuses it. A term takes no keyword.
  [S1f.5.11](s1f.5.11_closed_keywords.md) refuses it in the loader first.
- **G-21:** ruled 2026-11-07: a rule may conclude `eq` or `neq`, as the draft allows; what it means is
  [S1f.5.7](s1f.5.7_eq_and_neq_are_relations.md#conclusions-g-21)'s.
- **G-14:** with `:assert` checked by the grammar, does a bad conclusion that never fires get refused? Its
  unbound-variable part is ruled: refused at load, by
  [S1f.5.10](s1f.5.10_a_conclusion_variable_must_be_bound.md) (F13). **Ruled 2026-11-08, (b)**
  ([Q-M1f.1](../open_questions.md#q-m1f1--g-14-is-a-bad-conclusion-that-never-fires-refused)): refused at the
  parse whether or not the rule fires, and the one shape no grammar sees, an activator a rule derives binding a
  relation parameter to a fact, refused at load by a static check, which is
  [S1f.5.18](s1f.5.18_an_activator_binds_a_nested_fact.md)'s.
- **The 34 files** the draft refuses that today's parser accepts: which stay as refusal fixtures, and which
  are rewritten. Re-taken at step 1: the final grammar refuses 55, the draft's 34 and 20 refusal fixtures
  whose refusal it takes over from the loader, and one syntax fixture
  ([Q2](s1f.5.5_paper_grammar.md#q2--the-55-files-the-grammar-refuses-that-todays-parser-accepts)).
  **Ruled 2026-11-08, (a)**
  ([Q-M1f.2](../open_questions.md#q-m1f2--the-55-files-the-grammar-refuses-that-todays-parser-accepts)): the
  26 refusal fixtures stay and move to `examples/broken/`, except `reserved_open_direct`; `derivation_cycle`
  and the five syntax fixtures are rewritten; the 23 others are the owning stages'. Since the re-plan of
  2026-11-08 the 23 land in T4's commit, each credited to its owning stage's ruling, and of the rest whatever
  moves no answer lands before it (T10).
- **W35, `(absent ())` holds**, filed by [S1f.5.0](s1f.5.0_the_known_wrong_answers_filed.md) on
  [Q-M1f.3](../open_questions.md#q-m1f3--three-wrong-answers-the-census-found-with-no-row): this stage owns it,
  and the swap fixes it, since the grammar takes `()` only as a whole `:match` or `:assert` (default 7). Its
  row turns `fixed` in T4's commit.
  G-14 is [Q1](s1f.5.5_paper_grammar.md#q1--g-14-with-assert-checked-by-the-grammar-is-a-bad-conclusion-that-never-fires-refused).

## The method

### Task T1f.5.5.1 — what each position holds today

A census: every position in the grammar, what the corpus writes there, what the parser accepts, what the
loader then refuses and what it drops. Every refusal line in `from_ir.rs` and `compile.rs` is classified:
*the grammar's*, or *the loader's*. **The estimate is re-taken here.**

### Task T1f.5.5.2 — the grammar on paper, and real programs

The final `ein.l` and `ein.y`, from the probe's grmtools draft, with `zebra2.ein`, one stdlib module and every kind of
`examples/syntax/` fixture written against it, each answer computed on a prototype. **Stop and show the
user before any edit** (the standing rule on syntax), with the questions above.

### The order, re-planned 2026-11-08

Until 2026-11-08 S1f.5.3, S1f.5.4, S1f.5.1 and S1f.5.2 were each to land on the hand-written parser in its own
commits, S1f.5.11's lists in the loader beside them, so that every move of an answer was attributed to its
ruling before T3, and the swap was to move no answer. The coordinator told the user that the generated parser
waited for four language stages built on today's parser, S1f.5.4, S1f.5.1, S1f.5.2 and S1f.5.11. **The user,
2026-11-08:** *"It is the same work done twice, why not switch to grmtools
first and remove hand-written parser and then verify requirement met (strings, names, etc...) ?"* The
coordinator proposed the variant below, and the user: *"yes, re-plan it that way"*. S1f.5.3 had already run,
on the hand-written parser, and is done.

**What moved here, credited by name.** No stage id is renumbered. Each of the four keeps its id and is cut to
its non-grammar part, which runs after the swap, and its grammar part, its step-1 sweep and its review are
this stage's:

| stage | its ruling | what is this stage's | what the stage keeps, after the swap |
|---|---|---|---|
| [S1f.5.4](s1f.5.4_digits_are_names.md) | digits are names, and a name is any run of name characters (F7, F8, F2) | the name rule, which the paper grammar states; its step 1, done (`8b12ae07`, `638b1062`), as T9's first cross-check; T1f.5.4.2: the register's 12 rows, the fixtures, the printer, the JSON writer, S1f.5.3's interim (T4), and the loader's internal names (T8) | T1f.5.4.3, the int kind retired with the `:priority` guard; T1f.5.4.4, the pages |
| [S1f.5.1](s1f.5.1_bare_underscore_refused.md) | a bare `_` refused, and `_` valid anywhere in a name (W-22, F3) | T1f.5.1.1, its step 1 (T9); T1f.5.1.2, the refusal, the printer, the fixtures, W29's six rows and W20's and W24's `_` rows (T4) | T1f.5.1.3, F3 traced, `_` held in every position, `Slot::Any` and `Node::Wildcard` retired; T1f.5.1.4, the pages |
| [S1f.5.2](s1f.5.2_strings_are_natural_language.md) | a string is natural language, only in `:why`, `:source` and `:goal-text` (F4, F5, G-6) | T1f.5.2.1, its step 1 (T9); the refusal, the escapes, the fixtures and the restated W16, W18 and W28 rows (T4) | T1f.5.2.2, a string held in every position, the Unicode test and the two examples; T1f.5.2.3, the string's value arms retired; T1f.5.2.4, the pages |
| [S1f.5.11](s1f.5.11_closed_keywords.md) | each form takes a fixed set of keywords (D-15, G-15, G-22) | T1f.5.11.1, its step 1 (T9); the lists and the refusal inside terms, and W32's seven rows (T4) | T1f.5.11.2, the compiler's and the expander's drops gone, a keyword held in every form and place; T1f.5.11.3, the pages |

**The sequence.** T3, the generated parser beside the old one; T8, the swap's prerequisite; T9, every
difference attributed, and the one review stop; T10, the fixture moves that move no answer; T4, the swap. After
it, one at a time on the generated parser: T5, then the four stages' code tasks above, each moving no answer,
then T6, then the pages, T7 and each stage's, which share `00_ebnf.md`, `01_grammar.md`, `02_patterns.md` and
`defined_behaviour.md`.

**Nothing edits the hand-written lexer or parser for S1f.5.4, S1f.5.1, S1f.5.2 or S1f.5.11.** It gets two
edits before it goes, both this stage's and neither a reading of the language: T3's hook, if the comparison
needs one, and T8's spelling of the loader's internal names.

### Task T1f.5.5.3 — the generated parser, beside the old one

The full paper grammar, [`ein.l`](s1f.5.5_parser_probe/grammar/ein.l) and
[`ein.y`](s1f.5.5_parser_probe/grammar/ein.y) as step 2 left them, with its ten defaults and every ruling of this
phase, compiled by `CTParserBuilder` in `ein-ir`'s `build.rs`. Its actions build `ein_ir::Ast` with its `Loc`s,
through [`build_ast.rs`](s1f.5.5_parser_probe/grammar/build_ast.rs). It sits beside `lex.rs` and `parse.rs`,
reachable from T9's comparison and from its own tests, and **not from the shipped binary's default path**: no
command's output, option or `EIN_*` name changes until the swap. The binary reads two `EIN_*` names, and
`config_reference.rs` pins them, so the hook is a cargo feature off by default or an entry point only the tests
call, T3's to choose. Step 2's `EIN_PARSER=grm` was a probe's. No answer and no golden moves in T3.

**The nesting limit** (F10). A stated limit, default 1 000, refused at the parse with a line on every command.
It is checked on a debug build through what recurses after the parse: macro expansion, the loader and the
dumper. A test holds a program one past it.

**Recovery, made deterministic** (F11). `lrpar` 0.15.0 is vendored with the probe's patch, minus its step
printer, and the budget is set from a sweep of corpus mutants on the build. Three tests hold it:
- **Twice, the same.** A test in `ein-ir` parses every refusal fixture and a seeded set of mutants twice in
  one process and compares the reports byte for byte. Each `HashSet` gets its own hash keys, so the
  unpatched `lrpar` fails it; over three runs it differed on 1 032 of the probe's 2 113 refused mutants.
- **The fuzzer.** `fuzz_properties` gains the same property, and that every recovery ends within the budget.
- **The cut.** One fixture is cut at the budget and its report says so, in a golden.

### Task T1f.5.5.8 — the swap's prerequisite: the loader's names out of the name space

S1f.5.4's [Q1](s1f.5.4_digits_are_names.md#q1--the-synthetic-names), **decided 2026-11-08 by the coordinator on
the step-1 recommendation, (a)**, open to the user's review at T9's stop. Under the name rule `@empty`,
`<nested>`, `@params` and `<query>` are names a program can write, so the swap would store `(p ())` and
`(p (@empty))` as one fact, and `(p (_ a))` and `(p (<nested> a))` as one
([the finding](s1f.5.4_digits_are_names.md#the-finding-two-synthetic-names-become-spellable)). The four are
renamed to spellings the name rule refuses, with a character outside [the list](#the-name-rule) such as `#`:
`ein_core::text`'s `EMPTY_HEAD` and `NESTED_HEAD`, `terms.rs`' `QUERY_RULE`, and the places that build `@empty`
and `@params`, today's `parse.rs` and T3's `build_ast.rs` alike. That is a spelling of the loader's for no
language stage's ruling, so the two parsers T9 compares agree on it.

**A test holds it:** no internal name is a `NAME` of the generated lexer, and through the generated parser
`(p ()) (p (@empty))` stores two facts. **What moves**, named before it lands: `W34/assert-empty`'s `now`, whose
candidate prints the renamed head (it stays `open`, [S1f.5.9](s1f.5.9_an_empty_match_fires.md)'s);
`alive_invariant`'s `KNOWN_R1` line; and any golden that prints a firing's `:rule <query>`. It is its own
commit, so the swap carries no rename. If the user's review at T9 overrules Q1, this commit is replaced before
the swap.

### Task T1f.5.5.9 — every difference attributed, and one review stop

On the tree with T3 and T8 landed. **The old parser is the reference**, and a difference can be read as one only
while it exists, so this comes before the swap.

**What runs.**
- **The sweep:** every `.ein` under `examples/`, `tests/`, `stdlib/`, `corpus/` and `plans/` (511 on
  2026-11-08, `find examples tests stdlib corpus plans -name '*.ein' | wc -l`), each through `ein solve FILE
  --json-summary J -H --dump-answer D`, `ein saturate FILE --dump` and, where it states an `:expect`, `ein test
  FILE`, on today's binary and on a build with the generated parser, the swap's printer and the swap's JSON
  writer. Timing lines are removed, and exit codes, stderr, k_s, k_c, the states, N's sets, H's candidates and
  the stored facts are compared, as S1f.5.4's step 1 did (`build/s1f.5.4/run_sweep.sh`, `analyze.py`).
- **The parse itself:** both parsers on every corpus file and on the census's 40 000 seeded mutants (`c_ein
  mutate`), comparing accept, reject and the dumped AST, as `2168f430` did the last time.
- **`cargo test --workspace`** on a snapshot with the swap prototyped, then blessed (`EIN_BLESS=1`), so that every
  test and golden T4 moves is named here.

**Every difference is attributed** to one ruling or more:

| ruling | what it moves | its cross-check |
|---|---|---|
| **S1f.5.4**, the name rule (F7, F8, F2) | a run of digits is a name (`(p 1) (p 01)` two facts, `-7`, `1st`, `1..5`); the other name characters; a reserved word only as a whole token (`rule-x`, `neq-1`); `=` a name; a token ends at a separator (`a?x`, `?x?y`, `"a"b`, `a:b`, `Жук` refused); `:priority` takes digits only | [its step 1](s1f.5.4_digits_are_names.md#the-sweep-46-programs-move-and-465-answer-as-they-did): **the files credited to it, alone or beside another ruling, are exactly its 46 of 511** |
| **S1f.5.1**, a bare `_` refused (W-22, F3) | `_` and `?_` refused in every position | W29's six witnesses, W20's three `wild-*` and `wild-in-head`, W24's `wild-in-guard` and `bare-premise`, `arg-kinds` and `is-a-shapes`; and `zebra2.ein` renamed with `_`-led names and variables (`_Old_Gold_`, `?_h_water_`), R unmoved ([its step 1](s1f.5.1_bare_underscore_refused.md#task-t1f511--w29-re-taken-every-move-listed-real-programs-shown)) |
| **S1f.5.2**, strings only in `:why`, `:source` and `:goal-text` (F4, F5, G-6) | a string as any value refused; `?x` in a fact refused; `\u{…}` its code point, and any other escape refused | the 13 files its case names (`arg-kinds` and 12 witnesses of W16, W18 and W28), `is-a-shapes` for its `?x`, the two `activator-text-key` fuzz seeds, and the probes under `plans/` that write one; every `:why`, `:source` and `:goal-text` in the tree reads as before, `zebra2.ein` byte for byte |
| **S1f.5.11**, closed keywords (D-15, G-15, G-22) | a keyword its form does not take, and any keyword inside a term or a nested fact, refused | W32's seven witnesses, `wrappers.ein`, `kwpair-below-the-filter` and its fuzz find, and the five refusal fixtures whose line moves ([its census](s1f.5.11_closed_keywords.md#what-moves-in-the-corpus)) |
| **the paper grammar's own**: step 2's defaults 1 to 10, W35, F10, the error text and recovery | `()` only as a whole `:match` or `:assert`; `:using` a list of facts; `(not plain)` refused; the shapes and arities the loader checked; a refused file's report | **the census's 55** ([Q-M1f.2](../open_questions.md#q-m1f2--the-55-files-the-grammar-refuses-that-todays-parser-accepts)): the generated parser refuses each (the old one now refuses `arg-kinds` too, since S1f.5.3), and every other file it refuses that the old one accepts is credited above; W35 |
| **Q1 to Q4**, decided 2026-11-08 | Q4's JSON strings (`int-goal-binding`); Q1's rename is T8's, and moves nothing here | — |

Two rules settle a file that two rulings meet. **C1 and C2**, from S1f.5.4's step 1, taken by the coordinator on
2026-11-08: `_x`, `__`, `_-1` and `?_x` are S1f.5.4's, and the bare `_` and `?_` S1f.5.1's; no keyword difference
is S1f.5.4's. **Where two rulings meet one program, the combined grammar's answer is the one attributed**, under
every ruling it reads by. `activator-text-int-and-string`, for one, moved on S1f.5.4's prototype for its `1` and is
refused here for its `"1"`, so it is credited to both, and S1f.5.4's proposal for its row gives way to S1f.5.2's
refusal.

**The swap stops** on any difference no ruling explains, and on a cross-check that fails: a file of the 46 that
does not move, or one credited to S1f.5.4 outside them; or one of the 55 that the generated parser accepts. The
agent reports it and edits nothing. The attribution is banked as this stage's record, a page beside the census,
one row a file.

**One review stop, for every ruling at once.** Stop and show the user real programs before the swap (the
standing rule: real examples first), with every artefact both binaries write for each, raw and rendered (the
standing rule on review artefacts): S1f.5.4's review, `build/s1f.5.4/review/`, re-built on the generated parser
and extended to the other rulings. What it shows is each stage's step 1 as the stage wrote it:
- **S1f.5.4:** `features/05`, `std.elim`, `zebra2.ein` unmoved, `(p 1) (p 01)`, the 46, and the four decisions,
  Q1 to Q4.
- **S1f.5.1:** `arg-kinds`, `is-a-shapes` and W20's three matching witnesses, as written and rewritten;
  `zebra2.ein` with `_`-led names, R unmoved; its table of positions, each row computed; its refusal as the parse
  error and its repair.
- **S1f.5.2:** `arg-kinds`, one witness each of W16, W18 and W28; `zebra2.ein`'s `:why`, `:source` and
  `:goal-text` untouched and its answer unmoved; a probe with Cyrillic, CJK and an emoji in the three keywords, on
  every surface that carries them; the escapes; and its refusal as the parse error.
- **S1f.5.11:** the refusal of each of W32's seven, and `wrappers.ein` rewritten; and whether `value_label`'s
  `KwPair` arm is still reached, by a form's own pairs.
- **The paper grammar's own:** the 55 and where each goes; `derivation_cycle` and the five `examples/syntax/`
  fixtures rewritten; and the refused programs (`refused/*.ein`, `four_mistakes.ein`) with every error recovery
  reaches.
- **The register:** each row T4 flips or restates, with its new `now` and `right`; and which W16, W18 and W28
  witnesses stay as the refusal's witnesses and which retire, as S1f.5.2 said its step 1 would.
- **What the re-plan leaves open**, each with the agent's recommendation:
  - **`arg-kinds` and Q3.** S1f.5.4's Q3 was decided on the program as written today, which the grammar refuses
    for its `"a string"`, `_` and `?x`. After the swap the corpus carries Q-M1f.2 (a)'s rewrite,
    [`programs/arg-kinds.ein`](s1f.5.5_parser_probe/programs/arg-kinds.ein), and on S1f.5.4's prototype its `ein
    solve` stops with *a state's commitments pass 2^128: the state route cannot count them*, exit 1, writing no
    state, where `saturate` exits 0 (`build/s1f.5.4/protobin/ein`, 2026-11-08). That is the limit S1f.5.4 parked
    ([§ Notes](s1f.5.4_digits_are_names.md#notes)). The round trip dumps every corpus program whether or not
    its entry declares `solve`, and would read a witness that writes nothing. **Recommended:** the rewrite
    scoped with `(solve :no-hypothesis (r))`, Q3's (b), R {∅}, since the fixture's point is the node kinds as
    written; the limit stays parked.
  - **S1f.5.4's proposals that S1f.5.2's ruling overtakes.** S1f.5.4's step 1 proposed, and the coordinator
    accepted, `W16/int-and-string` and `-string-then-int` kept with a scope, and W28's three restated as one
    name. Their witnesses, the two `activator-text` controls and the two `activator-text-key` seeds all write
    `"1"`, which the same swap refuses. **Recommended:** each row restated once, to the refusal, and its witness
    kept as the refusal's witness in its position, as S1f.5.2 planned; W18's five likewise, so that their
    `[case.region]` re-bank is moot.
  - **The refusal sentences.** S1f.5.1 and S1f.5.2 each proposed a line: *`_` is reserved and names nothing:
    where any value will do, write a variable used once*, and *"a" is a string, and a string is natural-language
    text, written only after :why, :source or :goal-text: write a name bare*. A refusal is now the parse error
    with its repairs. **Recommended:** each kept as a note on the parse error, as default 5 keeps two.

The user may overrule any decision of 2026-11-08 here: Q1 to Q4, C1 and C2. Nothing is edited until the user
has read it.

### Task T1f.5.5.10 — the fixture moves that move no answer

[Q-M1f.2](../open_questions.md#q-m1f2--the-55-files-the-grammar-refuses-that-todays-parser-accepts) (a), ruled
2026-11-08, moves and rewrites fixtures. Whatever of it moves no answer on today's binary lands here, in its own
commit before the swap, so that the swap carries only what the grammar moves. **The test:** a rewrite lands here
only where every declared run, `ein saturate --dump` and `ein solve` give the same stored facts, R, N and exit
code on the rewritten file as on the original, under today's binary, with the goldens it moves named first. A
digest is not an answer.

Candidates, on step 2's evidence: `broken/load/derivation_cycle` (`:using ((r c d))`, refused by both parsers
with today's line), `syntax/unary-and-nary` (the same four stored facts; its derived fact's premises go from none
to `(u a)`, which T10 checks no core or claim reads) and `syntax/wrappers` (today's loader drops the nested
pairs). **Not here**, each for the reason that keeps it for T4: the 26 refusal fixtures, since a
fixture's group is its directory and each belongs under `examples/broken/` only once the parser refuses it;
`arg-kinds`, `is-a-shapes` and `negation`, whose rewrite moves a stored fact or does not parse today; and
`kwpair-below-the-filter`, which loads today. It edits no parser. If nothing passes the test, this task is empty
and says so.

### Task T1f.5.5.4 — the swap

**One commit**, after T9's stop and T10. `parse` is the generated parser, and `lex.rs` and `parse.rs` go, with the
hand-written lexer's unit tests and T3's hook. **No answer moves that T9 did not list**, and **the commit body
lists every moved answer, grouped by ruling**: S1f.5.4, S1f.5.1, S1f.5.2, S1f.5.11, the paper grammar's own, and
Q1 to Q4. It carries:

- **The error text.** Each parse error reports its true position, the tokens expected there and its ranked
  repairs, every error recovery reaches, three repairs and then a count, with default 5's notes (and S1f.5.1's and
  S1f.5.2's, if T9's review keeps them). The four `examples/broken/*.expected` and `grammar_decisions.txt` move.
- **The printer.** `ein_core::text::is_symbol` is the name rule, so a name prints bare where the generated lexer
  reads it back as one name (`(p 1)`, `(hit 1)`, `it's`), and `the_printers_symbol_is_the_lexers` holds the two to
  one answer. Without it every name of digits prints quoted, the same value.
- **`:priority 110` stays out of the program's names.** The grammar's action builds today's `Node::Int` for
  `:priority DIGITS` (`b::count`), and `from_ir.rs`' `priority` match reads it as it does today, so H is unmoved
  on every program that writes no plain digit, which T9 checked. That match drops any other value in silence, so
  it is the trap, and its guard is S1f.5.4's T1f.5.4.3, when the int kind goes.
- **The JSON writer.** S1f.5.4's [Q4](s1f.5.4_digits_are_names.md#q4--a-name-of-digits-in-summaryjson), decided
  2026-11-08, (a): every name is a JSON string in `summary.json`'s bindings (`region_answer.rs`' `binding`), so
  `(p 007)` and `(p 7)` stay two. `int-goal-binding`'s note is restated (`8`, `-7` and
  `99999999999999999999999` become strings), and the summary's schema id moves if its readers need it, named in
  the commit.
- **S1f.5.3's interim, emptied and turned here**, where S1f.5.4's T2 had it until the re-plan.
  `ein_corpus::REFUSED_UNTIL_S1F_5_4` goes, with `corpus_cli`'s
  `the_files_refused_until_digits_are_names_are_refused`, and its four readers (`every_positive_entry_answers…`,
  the round trip's witnesses, `alive_invariant`'s `SEEDED`, the dump fixed point) read both files again with no
  other edit. `cli_semantics`' `a_range_is_no_token` becomes its plain-name counterpart: `1..5`, `1..*` and `2..*`
  in a fact, a declaration, a rule's `:match` and `:assert` and a `(config …)` each read as the name it spells,
  and `:card 1..*` is refused at the parse as a keyword no form takes, since S1f.5.11's ruling lands in the same
  commit. The code comments, `syntax/arg-kinds`' header and corpus note, and the pages that say *until S1f.5.4
  makes it one name* (`00_ebnf.md`'s fact 3, `01_grammar.md`, `defined_behaviour.md` § 4.8, `examples/README.md`'s
  `syntax/` row, Q-M1e2.13 and Q-M1e2.24) are restated with it.
- **The register.** Each row moves in this commit. Its `owner` stays the stage whose ruling fixes it, and its
  `[case.fixed]` names this commit.
  - **Flipped to `fixed`:** W29's six (S1f.5.1's ruling), W32's seven (S1f.5.11's) and W35 (this stage's,
    default 7). Each `now` is the parse refusal, exit 1 (`ein test`: 2), and each `right`'s *refused at load,
    with a line naming the place* is restated *at the parse, with the file, line and column*, the exit codes
    unchanged. `W29/core-two-ways` drops its core and `W32/kwpair-in-goal` its class and claim, which a refused
    `now` may not record; `core-two-ways`' twin keeps its R.
  - **Restated, S1f.5.4's** ([its § The register](s1f.5.4_digits_are_names.md#the-register-12-rows-move),
    accepted 2026-11-08): `W7/binding-key-layout`, `W15/binding-key-layout` and `-swapped` scoped with `(solve
    :no-hypothesis (edge noted) …)`, four states, the oracle the same; `W16/activator-all-int`'s line, with
    `[activator 1]`; `W20/range-in-match`'s `right`, a pattern over the name `1..3`, R the cube of 8; and
    `W21/typed-blind-solve`'s `now`, N 25 → 51 (F8, accepted). Rows are restated, never given a status `retired`.
  - **Restated, S1f.5.2's:** the W16, W18 and W28 rows whose witness writes a string as a value, each `right` the
    refusal, overtaking S1f.5.4's proposals for `W16/int-and-string`, `-string-then-int` and W28's three, as T9
    showed.
  - **Restated, S1f.5.1's:** `W20/wild-in-match`, `-absent` and `-hrule`, whose `right` moves to the refusal,
    their twins keeping R; `W20/wild-in-head`, `W24/wild-in-guard` and `W24/bare-premise`, refused still, with the
    new line.
  - **Restated, S1f.5.11's:** `W24/kwpair-in-guard`'s line.
  - Not here: `W34/assert-empty`'s `now` moved at T8; W30, W31, W33 and W34 stay `open`, their stages' after the
    swap.
- **The fixtures**, Q-M1f.2 (a)'s and the four stages', each as T9 listed: the 26 refusal fixtures moved to
  `examples/broken/`, each `.expected` the new report, `reserved_open_direct` staying; what T10 did not land of
  `derivation_cycle` and the five syntax fixtures; `arg-kinds` as T9's review settles it; `kwpair-below-the-filter`
  to `examples/broken/` as a refusal, its fuzz find's note saying why (S1f.5.11's default, a parse error now and
  not a load one); `broken/load/relation_parameter_int` retired with its `.expected` and corpus entry (S1f.5.4's
  [Q2](s1f.5.4_digits_are_names.md#q2--relation_parameter_int), decided 2026-11-08, (a)); the `binding-key-*`
  witnesses and their two fuzz seeds scoped, as S1f.5.4 proposed; the four `activator-text-*` and their two seeds
  as T9's review settles; `SEEDED`'s lines (S1f.5.4's moves, `underscore-nested-fact-arg`'s `a`,
  `kwpair-in-pattern`'s and `kwpair-in-assert`'s `v`); `detector.rs`' count of flagged programs, which
  `kwpair-in-macro` leaves; and the slow flags `the_slow_flag_still_describes_the_sweep` reads.
- **The tests and goldens T9 named.** At least `corpus_exits.txt`, `corpus_shapes.md5`, `grammar_decisions.txt`
  and its 53 460-parse keyword test (which tests the reading of Q-M1e2.17 (i)(a) that the name rule overrides),
  `fuzz_seeds.txt`, `config_coercion.txt`, the two `branching/12` traces, `ir_semantics`' table, the
  int-against-string cases of `rule_semantics`, `compile_semantics` and `compile.rs`, `values_semantics`, and the
  `terms` and `text` unit tests. And `answer_round_trip`'s `the_witnesses_reload_to_their_states`: four of its five
  witnesses write a string as a value and the fifth is `arg-kinds`, so its witnesses are restated to programs in
  the final grammar whose names exercise the printer (`7` beside `07`, decorators, brackets).
- **The gate's cost**, before and after, in the commit body: S1f.5.4's
  [Q3](s1f.5.4_digits_are_names.md#q3--arg-kinds-and-the-round-trip), decided 2026-11-08, (a), scopes `arg-kinds`
  only if the gate visibly slows, and T9 shows what its rewrite costs.

### Task T1f.5.5.5 — the grammar pages and the loader's refusals

After the swap. `ein-ir/tests/grammar_pages.rs`, in `cargo test`. Each loader line the grammar now covers goes,
with its test moved to the grammar's table: the census's 67 sites, the lines the four rulings made unreachable
among them (`Found::WildHead`, `NestedKw`, `NotAForm`, `WildGuard`, the closed forms' unknown-keyword lines and
`QUERY_KEYWORDS`). What each ruling leaves past its loader lines is its own stage's after this task: S1f.5.1's
wildcard, S1f.5.2's string value arms, S1f.5.4's int kind and S1f.5.11's keyword drops, each retired moving no
answer, and each stage's positions added to this table.

### Task T1f.5.5.6 — `--diff-models` compares by value (S-15)

Ruled by the user on 2026-11-07, as recommended. `ein solve --diff-models FILE` compares each fact of the
file with the states' facts as text, in `ein_render::models::parse_expected`: a line that starts with `(` is
kept as written. So the same fact in another spelling does not match. Re-taken on `916d97ac`, 2026-11-07,
on the inventory's probe, whose one state holds `(p a)` and `(q "x y")`:

| the file writes | today | after |
|---|---|---|
| `(p a)` | `1 matched` | `1 matched` |
| `(p  a)` | `0 matched … 1 missing` | `1 matched` |
| `( p a )` | `0 matched … 1 missing` | `1 matched` |
| `(p a) ; the fact` | `0 matched … 1 missing` | `1 matched` |
| `(p a`, with no `)` | `0 matched … 1 missing`, with no error | refused before the run, exit 2, with the line and column |

S1f.5.2 and S1f.5.4 take away the two value cases, `(p "a")` and `(p 01)`. These five rows are what is left.

**The fix.** The file keeps its line format: `;` comments, an optional `closes` line, and `model` lines. The
facts between two `model` lines are read by the generated parser as facts, so a fact may span lines and carry
a comment. Each is printed by the one printer (`Terms::display`, W28's), which prints a value one way and
reloads to it, and is compared with the states' facts, which the same printer writes. A fact the parser
refuses is a usage error, exit 2, before the run, as an unreadable file is today, with every error its
recovery reaches. It runs after the swap, T1f.5.5.4. **No answer moves, and no register row:** `--diff-models`
exits 0 either way, so this is a report, as P1e2.3's surfaces were.

**Its test** is the five rows, in `ein-cli/tests/region_modes.rs` beside the option's other tests, with each
file written to the gate's scratch.

### Task T1f.5.5.7 — the pages

`00_ebnf.md` (generated blocks, § 1's facts and § 3 gone, the header), `01_grammar.md`, `02_patterns.md`,
`defined_behaviour.md` § 1, § 4.8 and § 4.9, `docs/kernel/ir/02-data-model/03_implementation.md`, `CLAUDE.md`'s
`ein-ir` line, a line in M2's S2.2.3 on what its generator now reads, and `configuration.md`'s sentence on a
`--diff-models` file that does not parse. Q-M1e2.17 says what overrode it. The editor grammar,
`utils/vscode-ein/`, follows the name rule, as S1f.5.3 kept it in step with the token it retired. Each language
stage's own pages land beside these, one at a time: S1f.5.4's T1f.5.4.4, S1f.5.1's T1f.5.1.4, S1f.5.2's
T1f.5.2.4 and S1f.5.11's T1f.5.11.3. `docs/history/` is a record, and does not change.

## Acceptance

- `ein.l` and `ein.y` are the grammar: each position has its own production, and no `Value` is shared between a
  fact, a pattern and a keyword.
- `00_ebnf.md`'s blocks are generated from it, and the gate fails if they differ.
- A conflict in the grammar fails the build, as the probe showed for grmtools.
- A refused file reports every error recovery reaches, each with its line and column, the tokens expected
  and at most three ranked repairs with a count of the rest. A report cut at the budget says where it stopped.
- That report is the same byte for byte on every run, under any load and on every machine: `lrpar` is
  patched (F11), and a test and the fuzzer parse each refused input twice and compare.
- A file with a parse error is refused, whatever its repairs. No repaired program is loaded.
- A file that parses pays nothing for recovery.
- Every shape this review refused is refused by the grammar, with the file, line and column, and
  `a?x`, `?x?y` and `"a"b` no longer split.
- A program nested past the limit is refused with a line, on every command, and no input aborts the parser.
- **The name rule**, S1f.5.4's ruling, whose acceptance moved here on 2026-11-08. `(p 1) (p 01)` is two facts,
  each printing as written and reloading to itself. `1st`, `1_a`, `7`, `-7`, `1..5`, `$x`, `%`, `a^b`, `a/b`,
  `<=`, `!x`, `@x`, `~`, `'name'`, `f'`, `{}` and `[a]` load as names, print as written and reload to themselves,
  and `'name'` and `name` are two names; `?a.b` is a variable. `a?x`, `?x?y`, `??x`, `a:b`, `a,b`, `a#b`, `a|b`,
  `a\b`, `Жук` and `"a"b` are refused, and no token splits in silence; `Жук` in a `:why` loads.
- **Where the paper grammar says more**, from S1f.5.4's step 1: `_x`, `__`, `_-1` and `?_x` are a name and a
  variable; `rule-x`, `rule.x`, `neq-1` and `not-a` are names, a reserved word being a whole token, which
  overrides [Q-M1e2.17](../../m1e2_correctness/open_questions.md#q-m1e217--where-does-a-keyword-end) (i)(a);
  `=` is a name, so `(= a b c)` is a fact; `(1 a)` and `(relation 1 T)` are a fact of, and a declaration of, the
  relation `1`; and a keyword is `:` and a run of name characters, one its form does not take refused.
- `:priority` takes a run of digits and nothing else, and `:priority 110` puts no name in the program: H is
  unmoved on every program that writes no plain digit.
- **A bare `_`**, S1f.5.1's ruling: `_` and `?_` alone are refused in every position, on every command that reads
  a program. `_a_in_b_`, `_______01` and `A________B`, and the same as variables, load, match and print as
  written, and reload to the same value. `zebra2`'s R is unmoved under the renaming.
- **Strings**, S1f.5.2's ruling: a string outside `:why`, `:source` and `:goal-text` is refused on every command,
  `(p "1")` among them, and so is a variable in a fact, `(r ?x)`. Every `:why`, `:source` and `:goal-text` in
  the tree reads as it did. `\u{…}` means its code point, and an unknown escape is refused. Every
  `--dump-answer` state file reloads to its state by value, with no quoted value.
- **Closed keywords**, S1f.5.11's ruling: each form takes exactly the keywords its row of
  [S1f.5.11 § The lists](s1f.5.11_closed_keywords.md#the-lists) names, and any other is refused, `:where`,
  `:cardinality`, `:mode`, `:k`, `:G` and `:notes` among them. No keyword pair parses inside a term or a nested
  fact.
- The loader's internal names are no name a program can spell, and a test holds it (T8).
- **Every difference between the two parsers was attributed to a ruling before the old one went** (T9): the
  files credited to S1f.5.4 are its step 1's 46, and the census's 55 are among the files the grammar refuses.
- `lex.rs` and `parse.rs` are gone, and the swap's commit moved exactly T9's list, each answer under its ruling
  in the commit body.
- W29's six rows, W32's seven and W35 are `fixed` by the swap, and every row a ruling restates answers its
  restated `right`.
- `--diff-models` matches a fact written with other spacing or a trailing comment, and refuses a file whose
  fact does not parse, with the line and column (S-15).
- No corpus answer moved outside T9's list.
- `./run_tests.sh` is green.
