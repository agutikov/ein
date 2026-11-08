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
  Half of the old two days for *beside and the swap*. Done ([§ T3 record](#t3-record-2026-11-08)).
- A day and a quarter for recovery: the patched `lrpar`, its budget and its tests (F11, in T3). Unchanged. Done.
- Half a day for the nesting limit (F10, in T3). Half of the old day for *the nesting limit and the tests that
  move*; the tests are the swap's. Done.
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
**T3 done on 2026-11-08** ([§ T3 record](#t3-record-2026-11-08)): the generated parser is in `ein-ir` beside the
hand-written one, reached from its tests and from the `generated-parser` cargo feature, off in every shipped build;
the builder holds on 340 of 414 corpus files with their `:priority` taken out and 204 as written, the trees
differing on none, and 133 files differing only by their `:priority`, which the grammar does not have (the
user's ruling of 2026-11-08: S1f.5.15 deletes it before the swap); a macro takes a `:why` (the same day);
recovery is deterministic, with `lrpar` 0.15.0 vendored and patched and a budget of 30 000 steps; the nesting
limit is 1 000. No answer, golden or exit code moved.
**The generated grammar is the phase's final one**, on the user's rulings of 2026-11-08
([§ The order](#the-order-re-planned-2026-11-08)): [S1f.5.15](s1f.5.15_rule_priority.md) runs between T3 and T8,
so the grammar never has `:priority`; `:why` stays optional in the grammar, and the loader flag that requires it
is [S1f.5.14](s1f.5.14_nl_required.md)'s, the phase's last stage; and the vendored `lrpar` patch is not sent
upstream ([F11](#f11-grmtools-recovery-is-not-deterministic-and-this-stage-makes-it-so)).
**T8 done on 2026-11-08** ([§ T8 record](#t8-record-2026-11-08)). **T9 done on 2026-11-08, and the stage stopped
there for the user's review** ([§ T9 record](#t9-record-2026-11-08), [the attribution](s1f.5.5_attribution.md)):
every one of the 167 files the swap moves, of 513, is credited to a ruling, none to no ruling; the census's 55 are all
refused; the name-rule build moves 45 of S1f.5.4's 46, the 46th being Q1's witness, whose move T8's rename took away;
and the generated build answers byte for byte as the name-rule build wherever the name rule alone moves a file.
No engine code, test, golden, fixture or register row moved. **The user ruled the stop the same day**
([§ The review stop ruled](#the-review-stop-ruled-2026-11-08)): `arg-kinds` scoped, the engine's five names
reserved (T11), the `.einb` budget restated in T4, and the rest as recommended. T10, T11 and then T4 run next.
**T10 done on 2026-11-08** ([§ T10 record](#t10-record-2026-11-08)): W37 filed, `open` and S1f.5.4's (item 7), and
of the three fixture candidates `syntax/wrappers` moved, the other two refused at the parse by today's parser and
so T4's; item 8, the scripts' `:priority`, landed as S1f.5.15's T1f.5.15.9.
**T11 stopped on 2026-11-08 before any edit** ([§ T11 record](#t11-record-2026-11-08)): the coordinator's reading of
item 4 would refuse `(imply1 functional __closed__)`, which the stdlib documents and a test holds, and T11's test
cannot hold under it; where a program may write a dunder is
[Q-M1f.7](../open_questions.md#q-m1f7--where-may-a-program-write-the-kernels-dunders), put to the user. **The user
ruled (b), *"Never bound"*, the same day (`feb5e839`), and T11 is done**: a dunder is refused where a program binds
a name and legal wherever one is used, and the three writeback names are refused wherever a program writes them.
**T4, the swap, done on 2026-11-09** ([§ T4 record](#t4-record-2026-11-09)): the generated parser is the only one,
`lex.rs` and `parse.rs` gone, and it moved exactly T9's list, re-taken on the tree it leaves against T9's generated
build; W29's six rows, W32's seven and W35 are `fixed`, and 23 fixed rows restated. T5, T6 and T7 run next.
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
   *Since 2026-11-08 the generated grammar has no `:priority` at all: the user ruled that S1f.5.15 removes
   it before the swap ([§ The order](#the-order-re-planned-2026-11-08)), and T3 built the grammar without it.*

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
`:assert`, and `:using (and s1 c15)`. *Since T3 (2026-11-08) `RuleClause` has no `:priority`, on the user's
ruling, and a macro takes a `:why` ([§ T3 record](#t3-record-2026-11-08)).*

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
`ein.rs/crates/` only; the patch's `HashSet` is probed for duplicates and never iterated. Sending the patch upstream was the
user's call, since it is outward-facing, and **the user ruled it on 2026-11-08: not sent** (*"don't send the lrpar
patch upstream either"*, as on the Graphviz bug, [S1f.5.17 § Notes](s1f.5.17_ci_green_again.md#notes)). It stays
vendored, `ein.rs/vendor/lrpar/ein.patch`; if a grmtools release ever makes recovery deterministic on its own, the
vendored copy can go.

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
| [S1f.5.4](s1f.5.4_digits_are_names.md) | digits are names, and a name is any run of name characters (F7, F8, F2) | the name rule, which the paper grammar states; its step 1, done (`8b12ae07`, `638b1062`), as T9's first cross-check; T1f.5.4.2: the register's 12 rows, the fixtures, the printer, the JSON writer, S1f.5.3's interim (T4), and the loader's internal names (T8) | T1f.5.4.3, the int kind retired, its `:priority` guard moot once S1f.5.15 has landed before the swap; T1f.5.4.4, the pages |
| [S1f.5.1](s1f.5.1_bare_underscore_refused.md) | a bare `_` refused, and `_` valid anywhere in a name (W-22, F3) | T1f.5.1.1, its step 1 (T9); T1f.5.1.2, the refusal, the printer, the fixtures, W29's six rows and W20's and W24's `_` rows (T4) | T1f.5.1.3, F3 traced, `_` held in every position, `Slot::Any` and `Node::Wildcard` retired; T1f.5.1.4, the pages |
| [S1f.5.2](s1f.5.2_strings_are_natural_language.md) | a string is natural language, only in `:why`, `:source` and `:goal-text` (F4, F5, G-6) | T1f.5.2.1, its step 1 (T9); the refusal, the escapes, the fixtures and the restated W16, W18 and W28 rows (T4) | T1f.5.2.2, a string held in every position, the Unicode test and the two examples; T1f.5.2.3, the string's value arms retired; T1f.5.2.4, the pages |
| [S1f.5.11](s1f.5.11_closed_keywords.md) | each form takes a fixed set of keywords (D-15, G-15, G-22) | T1f.5.11.1, its step 1 (T9); the lists and the refusal inside terms, and W32's seven rows (T4) | T1f.5.11.2, the compiler's and the expander's drops gone, a keyword held in every form and place; T1f.5.11.3, the pages |

**The grammar is final from the start: the user's rulings of 2026-11-08, after the re-plan.** The user:
*"collect all requirement for the parser in this phase and implement grammar that will already meet all of them
instead of making wrong parser that meets our old wrong parser and then updating it step by step"*. The paper
grammar already meets every stage of the phase that has a grammar part
([P1f.5 § The order](README.md#the-order) lists them). Only two items would have edited it later, and the user
ruled both:
- **`:priority`**: *"S1f.5.15 before swap"*. [S1f.5.15](s1f.5.15_rule_priority.md) runs after T3 and before T8,
  removing the keyword from every program both parsers read, so the generated grammar never has it. T3 built it
  so ([§ T3 record](#t3-record-2026-11-08)). If S1f.5.15 ends in the written refusal its acceptance allows, the
  production comes back then, before T9.
- **NL `:why`**: *":why is optional in grammar and loader has a flag that makes it required, start with default
  false so not break everything, then after other stages finished swith to default true and fix remaining ein
  code"*; and, asked whether the flag can change an answer, *"yes, the flag only decides loading"*. T3 gave a
  macro its `:why`, so every form that can carry a template may. The flag is
  [S1f.5.14](s1f.5.14_nl_required.md)'s, which runs last, and the grammar is not edited for it.

**The sequence.** T3, the generated parser beside the old one; S1f.5.15, `:priority` removed; T8, the swap's
prerequisite; T9, every difference attributed, and the one review stop; T10, the fixture moves that move no
answer; T4, the swap. After it, one at a time on the generated parser: T5, then the four stages' code tasks above,
each moving no answer, then T6, then the pages, T7 and each stage's, which share `00_ebnf.md`, `01_grammar.md`,
`02_patterns.md` and `defined_behaviour.md`.

**Nothing edits the hand-written lexer or parser for S1f.5.4, S1f.5.1, S1f.5.2 or S1f.5.11.** It gets two
edits before it goes, both this stage's and neither a reading of the language: T3's hook, if the comparison
needs one, and T8's spelling of the loader's internal names. S1f.5.15's refusal of `:priority`, if it takes one,
is a loader line, not a parser edit.

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

### T3 record, 2026-11-08

**What was built.** The paper grammar as step 2 left it, with the two rulings below, in
`ein.rs/crates/ein-ir/src/ein.l` and `ein.y`, and its builder, `src/grammar/build_ast.rs`, compiled by `ein-ir`'s
new `build.rs` (`CTLexerBuilder` with CPCT+ recovery; a conflict fails the build). The builder is the probe's,
with one difference: a `Loc` is taken from a line table built once per parse (`Lines`), counting as the
hand-written lexer counts (`\n` alone ends a line, a column is a character), where the probe asked `lrlex`'s
`line_col`, which walks the line from its start for every token. The module is `ein_ir::grammar`:
`grammar::parse(ast, text, filename) -> Result<Vec<NodeId>, Refusal>`, `NESTING_LIMIT`, `RECOVERY_STEP_BUDGET`
and the report types. The lexer is built once a process (a `OnceLock`).

**Two rulings of 2026-11-08 came after step 2, and the grammar carries both.** The user's principle: *"collect
all requirement for the parser in this phase and implement grammar that will already meet all of them instead of
making wrong parser that meets our old wrong parser and then updating it step by step"*, so the generated
grammar is the phase's final one.
- **No `:priority`.** S1f.5.15, which deletes it and derives the order from the rule graph, runs after T3 and
  before T8, T9 and the swap. So `RuleClause` has no `:priority` and a rule's `:priority` is refused as any
  keyword the form does not take; with it went `DIGITS`, a token only so that `:priority` could take digits
  alone, so a run of digits is a `NAME` like any other, and `b::count`, the action that kept `:priority`'s
  value an `Int`: the generated parser builds no `Int`. The hand-written parser and the shipped path still
  take `:priority` until S1f.5.15.
- **A macro takes a `:why`**, as a relation and a rule do: `(macro NAME (params…) BODY :why "…")`, once, after
  the body. The macro reader keeps it, `ein_ir::macros::Macro::why` (`generated_parser::a_macro_carries_its_why`);
  the hand-written parser refuses it. The loader's flag that will require a `:why` is S1f.5.14's, not built here.

**The hook: a cargo feature, `generated-parser` on `ein-ir`, off by default.** With it on, `ein_ir::parse` is
the generated parser, so `cargo build --release -p ein-cli --features ein-ir/generated-parser` is T9's second
binary; the hand-written parser is `ein_ir::parse::hand_written` by name in either build, so both can run in one
process. The only edit to `parse.rs` is that split, which changes nothing it accepts. No shipped build turns the
feature on, so no command's output, option or `EIN_*` name moves, and `ein --version`'s `features` line does not
list it (it lists the binary's). Under the feature a refusal is `ParseError` with the first error's position and
the whole report as its context, a stand-in until T4 writes the error text. Built with it (before the two
rulings), `ein solve` printed the same bytes on `zebra.ein` and `zebra2.ein` as today's binary, timing lines
apart, and `ein saturate`'s `parse:` line on `zebra2.ein` read 3.52 ms against 0.20 ms (best of five, one parse
a process, so the lexer's construction is in it); both files write `:priority`, which the feature build now
refuses until S1f.5.15.

**The builder holds** on every corpus file both parsers accept, taken both ways: as written, and with each
`:priority` clause taken out (`ein_corpus::mutants::without_priority`), which is what the corpus reads as once
S1f.5.15 has run. Of the **414 files** under `examples/`, `stdlib/`, `tests/` and `corpus/`, 137 write
`:priority` (482 clauses; `grep -l`, `grep -o`):

| | the same tree | the same but for digits | trees differ | only the generated parser refuses | both refuse | only the hand-written refuses |
|---|---:|---:|---:|---:|---:|---:|
| as written | 189 | 15 | **0** | 204 | 5 | 1 |
| without `:priority` | 322 | 18 | **0** | 68 | 5 | 1 |

So **340 agree** without `:priority` (the census: 331 of 390 at `ec004a48`) and 204 as written; **133 files
differ only by their `:priority`**, attributed to S1f.5.15, pending. Digits are a name by S1f.5.4's ruling;
the one file the hand-written parser alone refuses is `range-in-match`, which S1f.5.3 moved out of the digits
class since the census. `cargo test -p ein-ir --test generated_parser the_builder_holds_on_the_corpus` holds
it both ways, the counts, the 133, and every other file not the same tree banked with how it differs
(`ein-ir/tests/golden/generated_parser_corpus.txt`). Each of those is on the census's list
([`grammar_vs_today.txt`](s1f.5.5_parser_probe/census/grammar_vs_today.txt), 74 files, each still banked) or is
one of its four parse-negatives, but 14 register witnesses S1f.5.0 filed after `ec004a48`, each refused for the
shape its row is about: W29's six `underscore-*` (S1f.5.1's `_`), W32's seven `kwpair-in-*` and `where-*`
(S1f.5.11's keywords) and W35's `absent-empty` (default 7). T9 attributes them.

**The vendored `lrpar`**, `ein.rs/vendor/lrpar/`: the crate as published (git `0fd68c80cb13`), grmtools'
`COPYRIGHT`, `LICENSE-APACHE` and `LICENSE-MIT` at that commit, which the published crate does not carry, and
`ein.patch`, the whole difference (104 changed lines in `cpctplus.rs` and `parser.rs`; `patch -p1 < ein.patch` on
a fresh copy reproduces `src/`). The probe's patch, less its step printer, plus the cut: each `ParseError` says
the steps recovery had taken (`recovery_steps`) and whether the budget stopped it there (`recovery_cut`).
**It sits outside the workspace**: `[workspace] exclude = ["vendor"]`, used through `[patch.crates-io]`, so the
generated parser and any grammar loaded at run time get it. None of the gate's checks reads it, by construction
rather than by a skip list: `cargo fmt --all`, clippy, rustdoc and `cargo test` take members, and the published
crate is not rustfmt-clean (`rustfmt --check --edition 2024 vendor/lrpar/src/lib/mod.rs` exits 1);
`check_hashmap_iteration.py` reads `ein.rs/crates/`, and `doc_audit.py` the pages it names. **Dependencies**:
`cfgrammar`, `lrlex` and `lrpar` pinned `=0.15.0`; `ein-ir`'s run-time crates go from 8 to 46, 56 with the build
(`cargo tree -p ein-ir -e normal --prefix none --target x86_64-unknown-linux-gnu | sort -u | wc -l`, `-e
normal,build`), and `Cargo.lock` gains 49 packages, every licence MIT, Apache-2.0 or both, within `deny.toml`'s
list. A change to the grammar or to the vendored code generator reaches the generated parser only once
`src/ein.y` is touched: `CTParserBuilder` keys its cache on the grammar and on `lrpar`'s build stamp.

**The report.** A refused file reports every error recovery reaches, in file order: `file:line:col: unexpected
X`, with the token's `%epp` description where it has one (`unexpected `a?x`: a token that is not a name`),
then at most three ranked repairs, the first the one the parse went on with, each a sequence of `Insert`,
`Delete` and `Shift`, then `and n more repairs`, and, where recovery was cut, a last line, *recovery stopped
here, at its budget of 30000 steps: errors after this one are not reported*. The probe's `four_mistakes.ein`
reports its three errors (`cargo run --release -p ein-ir --example generated_parser -- check FILE…`). **Not in
it: the tokens expected at an error**, which § The error text asks for. Read off the state table they overstate:
the table is Pager's merged LR(1), whose reductions on a lookahead are taken before the error is found, so at
`four_mistakes.ein`'s `a?x`, a fact's argument, the state offered `:why`, `:goal` and nine more keywords no fact
takes (tried on this build, then dropped). The parser's data is private to the generated module besides. The
ranked repairs are exact, since CPCT+ checks each by parsing on; the expected set is T4's, with the error text
(§ Notes).

**The budget: 30 000 steps**, the stage's proposal, re-taken on the final grammar: the smallest round budget at
which 99 % of refused corpus mutants finish recovery, its worst case inside the 500 ms clock it replaces, where
50 000's is not. `cargo run --release -p ein-ir --example generated_parser -- mutants 40000 --without-priority`,
40 000 mutants of the 398 corpus files under 6 000 bytes with their `:priority` taken out, as the corpus will be
(`ein_corpus::mutants`, the census's generator, now shared), with the vendored constant set to each budget in
turn, the six sweeps at once on six cores:

| budget | refusals cut (of 17 118) | slowest | most one parse grew the peak RSS |
|---:|---:|---:|---:|
| 10 000 | 408 | 0.096 s | 68 MiB |
| 20 000 | 232 | 0.196 s | 146 MiB |
| **30 000** | **144** | **0.299 s** (0.205 s alone) | **217 MiB** |
| 50 000 | 79 | 0.514 s | 382 MiB |
| 300 000 | 13 | 2.57 s | 2.9 GiB |

Steps per refusal, at 300 000: median 45, 90 % 1 067, **99 % 26 446**, 99.9 % 241 352. As written, `:priority`
and all, the same 40 000 at 30 000 steps: 25 414 refused, 139 cut, 99 % 16 518, the slowest 0.207 s, 210 MiB.
**Memory, not time, is what binds**: 7 to 10 KiB a step at the worst, so the million steps the probe's patch
tried reached 10 to 14 GiB on the grammar of step 2 and three runs were killed for memory (2026-11-08); a budget
in steps also bounds a parse's memory, which the clock did not. The hand-made worst cases, cut at 30 000:
`zebra2.ein` with every `)` deleted reports 35 errors in 0.171 s (76, still cut, in 6.9 s at 300 000), and
`zebra.ein` with every `)` deleted 31 in 0.294 s (`build/s1f.5.5/t3/hard/`). No mutant's report differed between
two parses at any budget.

**The three tests of F11.**
- **Twice, the same** (`generated_parser::a_report_is_the_same_twice`, about 4 s on a debug build): every corpus
  file the generated parser refuses and 1 000 seeded mutants, each parsed twice in one process and compared byte
  for byte, 842 refusals. With the ranking unpatched (`lrpar`'s `HashSet` drain and `sort_unstable_by` put back)
  **173 of the 842 differ** (183 of 494 on step 2's grammar, `:priority` and all); patched, none.
- **The fuzzer** (`fuzz_properties::a_generated_refusal_is_the_same_twice_and_within_its_budget`): the same
  property over the fuzzer's stream, and every recovery within the budget, all of it spent only where the
  report says it was cut. It reads every fourth input, and at most 10 000 spread over the stream: three in four
  are refused and a refusal costs milliseconds, most of them lexing, so the nightly's 1 000 000 would take an
  hour a seed. 500 inputs a commit (the binary's four tests in 3.3 s, debug), 10 000 a seed in the nightly (40 s
  at `EIN_FUZZ_SEED=1`, release). Unpatched, it fails at iteration 0.
- **The cut** (`generated_parser::a_report_cut_at_the_budget_says_so`): `ein-ir/tests/golden/recovery_cut.ein`,
  two mistakes recovery repairs and then a comment whose second line lost its semicolons, needs 68 400 steps
  (the 300 000 build); at 30 000 it reports the two with their repairs, the third with none, and the cut line, in
  `golden/recovery_cut.txt`.

**The nesting limit: 1 000**, the stage's default. A `(` that opens level 1 001 is refused before the parse, one
line on its position: *this `(` nests deeper than the limit of 1000 parentheses* (`generated_parser::
the_nesting_limit`). At the limit, a program with a macro, a rule whose `:match` nests a call of it 1 000 deep
and a fact nested 1 000 deep parses, dumps and loads, the loader expanding the call, on the test's own thread,
a debug build's 2 MiB; 1 001 and 100 000 deep are refused with the line. Its headroom: the same program, read by
the hand-written parser, dumps and loads on a 2 MiB debug thread at 3 000 deep and overflows at 3 250. Built
with the feature, `ein saturate`, `ein solve`, `ein render program` and `ein solve --dump-answer` answer it,
exit 0, on a debug and a release build, and refuse 1 001 and 100 000 deep with the line, exit 1. The shipped
binary still aborts at 100 000 deep until the swap.

**What did not move.** No answer, golden, exit code, CLI option or `EIN_*` name: every command still parses
with the hand-written parser, which still takes `:priority` and refuses a macro's `:why`. `fuzz_seeds.txt` is
unchanged, since no seed file was added. The gate's numbers are in the commit.

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

### T8 record, 2026-11-08

**What was renamed.** The names the parser, the loader and the engine make up for something a program writes no
name for, each now starting with `#`, listed in `ein_core::text::INTERNAL_NAMES`:

| was | now | names | made by |
|---|---|---|---|
| `@empty` | `#empty` | the empty form `()` | `ein_core::text::EMPTY_HEAD`: `parse.rs`' `alt_generic_list`, `build_ast.rs`' `empty` |
| `<nested>` | `#nested` | a nested form whose head is not a name, `(p (_ a))` | `NESTED_HEAD`: `from_ir.rs`' `fact_args` |
| `@params` | `#params` | a rule's or a macro's parameter list | `PARAMS_HEAD`: `parse.rs`' `params`, `build_ast.rs`' `params` |
| `<query>` | `#query` | the rule a query's `:goal` compiles to | `ein_core::terms::QUERY_RULE` |
| `@list` | `#list` | a fact's `:using` list, in the generated parser's tree | `LIST_HEAD`: `build_ast.rs`' `premises` |

**Why `#`.** Neither lexer reads it in a name. The name rule leaves it out by the user's ruling (*"`;`,`#`,`|` -
for comments, also not allowed in names"*); `ein.l` reads a `#` that does not open `#|` as part of a `BAD` token,
*a token that is not a name*; and the hand-written lexer starts no token with it. Ein's text uses `#` only in
`#|`…`|#`, the block comment, and no internal name has a `|` after its `#`. Of the other characters the list leaves
out, `?`, `:`, `;`, `|`, `\`, `"` and the parentheses each have a job, and `,` reads as punctuation in every
surface that lists names. `#` was the stage's suggestion and Q1's.

**The fifth name, `@list`**, which T3's builder gives a `:using` list, was as spellable as the four and was read
the same way, so it moved with them. Three places read a synthetic head by its spelling, and each now reads the
constants: the dumper (`dump.rs`' `is_headless`) and `(import … :symbols …)` (`imports.rs`' `symbol_list`) tested
a leading `@`, so under the name rule a form a program writes as `(@a b)` would have dumped as `(b)`; both test
`ein_core::text::is_internal`. `from_ir.rs`' pattern walk left `@empty` and `@params` out of a pattern's relations
by spelling, so a rule matching a relation a program names `@params` would not have listed it.

**The printer, and what reads it back.** `ein_core::text::head` prints `#nested` as `_` and `form` prints
`#empty` with no arguments as `()`, by the constants, as before. A name with a `#` prints quoted, `"#empty"`, as
`"@empty"` did, and the hand-written loader reads a string in a fact as the name it spells, so a state file that
writes `(q "#empty")` reloads to that value. W34's state files still do not load, for the line `()` the row is
about (`DO_NOT_LOAD` in `answer_round_trip.rs` and `answer_dump.rs`, unchanged). `#query` is never printed as
reloadable text: no rendering or CLI output in the sweeps below names `<query>`, before or after.

**What moved, named before it landed.** A grep of the four spellings over code, goldens, docs and the corpus,
and a dump of every rendering `corpus_shapes` digests (15 104) on the tree before the change, listed it; the
same dump after the change, and the CLI sweep below, checked it:
- **`W34/assert-empty`** (open, S1f.5.9's): its `[case.now]` and `[case.region]` print the candidate `(q
  "#empty")`, where they printed `(q "@empty")`. Its `wrong` keeps `(q "@empty")`, the answer at `ec004a48`; its
  title, `open_because`, the witness's header (its 35 lines kept, so no `Loc` moves) and its corpus note say
  `#empty`. No row's status moved: 103 fixed, 35 open and 1 not wrong
  (`grep '^status' corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-08).
- **`alive_invariant`'s `KNOWN_R1`**: its two lines print `"#empty"`.
- **`corpus_shapes.md5`: 94 lines on eleven files**, more than the task named, each a rendering that prints a
  renamed head. `@params` in the five IR DOT views and `dot[constraints]` of `stdlib/macro.ein` and the four
  macro fixtures under `examples/broken/load/` (30 lines); `@empty` in the seven IR and rule DOT views of W34's,
  W35's and W31's four witnesses, the plan of each but `empty-match-hrule`'s, and on W34's own in its
  `saturate` events, `naf`, `hyp`, `dot[kb-since]`, four read-outs, the region trace and lattice, five core views
  and, with W35's, the oracle (64). Of the 94, 50 differ by the spelling alone. The other 44 also differ by what
  a renderer derives from a name, each on a line that names the renamed head: a relation's
  colour is `sha1(name) mod 10` (`ein_render::palette::hash_color`), so `#empty` is cyan where `@empty` was grey,
  and a DOT fact node's id hashes its fact's text (three ids, each a renamed head's fact).
- **No golden printed a firing's `:rule <query>`**, which the task expected might move.

**The CLI sweep.** The binaries built before and after the change, on every `.ein` under `examples/`, `tests/`,
`stdlib/`, `corpus/` and `plans/` (513; `find examples tests stdlib corpus plans -name '*.ein' | wc -l`,
2026-11-08), each through `ein saturate FILE --dump`, `ein solve FILE --json-summary J -H --dump-answer D --events E
--trace T -G`, `ein render program FILE` and, where it states an `:expect`, `ein test FILE`. No exit code of the
1 687 runs moved. Of the 935 755 files they write, 934 802 are the same bytes, 426 differ only by the sweep's own
output directory in the `run` event's `argv`, and 504 only by a time. **23 moved by the rename**: ten of W34's
witness (its `saturate --dump`, `solve`'s stdout, summary, events, trace and five answer files, `models/02.ein`
among them), and `ein render program`'s view of thirteen programs that write `()` or a macro's parameter list
(`stdlib/macro.ein`, the four macro fixtures, the six empty-form witnesses of W31, W34 and W35, and two probes under
`plans/`, `empty_arg.ein` and `heads.ein`), six of them with the name's colour. None names `<query>` or `#query`.

**The tests.**
- `ein-ir/tests/generated_parser.rs`' **`no_name_the_loader_makes_up_is_a_name`**: no name of
  `INTERNAL_NAMES` is one `NAME` of the generated lexer (`ein_ir::grammar::is_one_name`, added for it), and both
  parsers refuse each as a fact's argument, a nested fact's head and a fact's head; each old spelling is one
  `NAME`.
- The same file's **`the_loaders_names_and_the_names_a_program_writes_are_two`**: `(p ())` and `(p (@empty))`
  load as two facts into one interner, and so do `(p (_ a))` and `(p (<nested> a))`. The generated grammar
  refuses `()` and `_` as a fact's argument (a fact holds names and facts), so the stage's twin `(p ()) (p
  (@empty))` does not parse through it whole: the hand-written parser reads the first of each pair and the
  generated one the second, the two parsers T9 compares, and T4 removes the first half with its parser. Through
  the generated parser, `(@params b)` dumps with its head, and a rule matching `(@empty ?x)` and `(@params ?x)`
  lists both among the relations it reads.
- `ein-infer/tests/rule_semantics.rs`' **`an_empty_conclusion_and_one_named_at_empty_are_two_facts`**, the twin
  that outlives the hand-written parser: through the generated parser, `:assert ()` and `:assert (@empty)` store
  two facts, the second's relation the name `@empty`. It holds whether or not `:assert ()` stores a fact, so
  S1f.5.9's fix of W34 leaves it standing.
- `ein_core::text`'s `every_internal_name_carries_the_mark_and_prints_quoted`.

With the old spellings put back in `ein_core`, each of the three fails at its collision (2026-11-08).

**The `generated-parser` build**, which the gate does not build: `cargo test -p ein-ir --features generated-parser
--no-fail-fast` passes 80 tests and fails 47, and on an export of the commit before (`git archive HEAD`) the same
47 fail, one more fails only because its golden names the checkout's root, and the two new tests do not exist
(2026-11-08). The 47 are suites in today's language read through the generated grammar: each of the nineteen unit
tests among them parses a program the grammar refuses (a string, `_`, a variable in a fact, a keyword its form
does not take, `:match X`). They are T4's to move with the swap.

**Not renamed, and parked for T9's review with Q1.** The engine's three writeback rule names, `<forced-positive>`,
`<monotonic-unconditional>` and `<lookahead-dies-immediately>` (`ein_core::terms::ENGINE`), are names under the
name rule too, and so are the dunders `__closed__` and `__symmetric__`, which a program can write today. None is
the loader's, and nothing reads the first three by name: a record is a terminal by its empty premises
(`Prov::is_terminal`), so a rule a program names `<forced-positive>` would share a printed name with the
writeback and nothing else. The dunders are reserved names
[`reserved_engine_strings.md`](../../../docs/kernel/inference/reserved_engine_strings.md) documents.

### Task T1f.5.5.9 — every difference attributed, and one review stop

On the tree with T3, S1f.5.15 and T8 landed. **The old parser is the reference**, and a difference can be read as
one only while it exists, so this comes before the swap. **No difference is `:priority`'s**: S1f.5.15 has removed
it from every program before this runs (the user's ruling of 2026-11-08), so T3's 133 files that differed only by
their `:priority` read as the same tree, and the table below has no row for it. If S1f.5.15 ended in a written
refusal, its production is back and those files are attributed to it here.

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
| **S1f.5.4**, the name rule (F7, F8, F2) | a run of digits is a name (`(p 1) (p 01)` two facts, `-7`, `1st`, `1..5`); the other name characters; a reserved word only as a whole token (`rule-x`, `neq-1`); `=` a name; a token ends at a separator (`a?x`, `?x?y`, `"a"b`, `a:b`, `Жук` refused) | [its step 1](s1f.5.4_digits_are_names.md#the-sweep-46-programs-move-and-465-answer-as-they-did): **the files credited to it, alone or beside another ruling, are exactly its 46 of 511** |
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

### T9 record, 2026-11-08

**What ran**, on `git archive` snapshots of `efd2cc74` (T3, S1f.5.15 and T8 landed), each answer the release
binary's with `EIN_STDLIB` the checkout's: **today** (the snapshot as committed); **the generated build**, the swap
prototyped in three files (the `generated-parser` feature on by default, so every command reads its program
through the generated parser; `ein_core::text::is_symbol` the name rule; every name a JSON string in
`summary.json`'s bindings, Q4), whose diff, `build/s1f.5.5/t9/swap-prototype.patch`, is T4's starting point; and
**the name-rule build**, S1f.5.4's step-1 prototype ported to the snapshot with Q4's writer, the name rule alone,
the counterfactual S1f.5.4's credit is read off. Three runs: the CLI sweep, every `.ein` under the five roots (513;
`find examples tests stdlib corpus plans -name '*.ein' | wc -l`) through `ein solve FILE --json-summary J -H
--dump-answer D`, `ein saturate FILE --dump` and, where it states an `:expect`, `ein test FILE` on the three
binaries (115 s); the parse, both parsers and the name-rule parser on every file and on 40 000 mutants from the
census's generator (`ein_corpus::mutants`, seed 0), with today's loader on whatever today's parser accepts; and
`cargo test --workspace` on the generated build, then blessed. The scripts are S1f.5.4's step-1 scripts extended,
under `build/s1f.5.5/t9/scripts/`. **The attribution is [`s1f.5.5_attribution.md`](s1f.5.5_attribution.md)**, one
row a file, with the tests, the goldens and the commands to re-take it.

**Every difference is attributed.** 346 files answer as they did on every command, 167 move, and each is credited to
the rulings that alone move it, **none to no ruling**: S1f.5.4 49 (45 by the name-rule build, 4 by the tree alone),
the paper grammar's own 48, S1f.5.15 43, S1f.5.2 28, S1f.5.11 28 and S1f.5.1 24, a file once per ruling it reads by.
On the 40 000 mutants the generated parser accepts only what the name-rule parser accepts, with its tree, on every
one; the name rule's share is exact (where today's parser and the name-rule parser differ), and what the generated
parser refuses past it, 6 827 mutants, is credited by its first error: S1f.5.11 2 141, the paper grammar's own
2 114, S1f.5.2 1 547 and S1f.5.1 1 025 (`mutants.py`).

**The cross-checks.**
- **S1f.5.4's 46: holds as meant, and read literally fails on one file.** The name-rule build moves 45 of the 46 and
  nothing outside them. The 46th, `assert-empty`, moved in step 1 only because its candidate printed `(q @empty)`
  bare, step 1's finding; T8's rename (Q1 (a)) printed it `(q "#empty")` on both builds, so it moves under neither.
  Put to the user at the stop (item 5 of the review's § 0).
- **The generated parser's name rule is the prototype's**: on the 20 of the 45 it accepts, the generated build
  answers byte for byte as the name-rule build on every command. The other 25 it refuses for a string, a
  `:priority` or a keyword, the combined grammar's answer, credited to both rulings.
- **The census's 55: all refused.** The 14 other corpus files the generated parser refuses that today's accepts are
  witnesses filed since, each for its row's shape: W29's six, W32's seven and W35.

**No difference is unattributed, and no cross-check fails as meant, so the swap does not stop.**

**What the brief did not expect, each a report and not a stop** (each with a recommendation in the review's § 0):
- **43 records under `plans/` are refused at their `:priority`.** The brief read *every program* as the four corpus
  roots; S1f.5.15's T5 left the records as written, saying the swap refuses the keyword. Credited to S1f.5.15.
- **Three scripts still write `:priority`**: `utils/region_synth.py`, which `region_dump`'s
  `a_random_region_written_as_a_program_comes_back` and `detector`'s
  `a_monotone_verdict_is_one_state_per_node_on_random_programs` read, `guard_split.py` and `fuzz_engine.py`. T4 takes
  the clause out of the three (S1f.5.15's leftover).
- **A template cannot name every variable: a wrong answer with no register row.** `ein_core::why::render_why` reads a
  reference as `{?`, an alphanumeric, then alphanumerics, `_` and `-`, so `{?isR*}` prints unfilled in three stdlib
  `:why`s today (`stdlib/typing.ein`, `stdlib/algebra.ein` twice; `ein solve
  tests/stdlib/typing/02_type_hierarchy_converse.ein --trace T` on today's binary), and after the swap so does every
  variable the name rule adds: `zebra2.ein` with `_`-led names answers as `zebra2.ein` does (k_s 1, k_c 261 982, N
  366, H 56, the state and N the same once the underscores are taken off) and prints *“The {?\_who\_water\_} drinks
  water in {?\_h\_water\_}; …”*. Reported here; a row is filed before anything fixes it (recommended: owned by
  S1f.5.4, the reference reading the name rule's characters up to the first `}`).
- **W34's printed region no longer reads back** (`claim_round_trip`): it holds the leaked `(q "#empty")`, a string
  a claim may not write after the swap. Recommended: the witness joins that test's exceptions until S1f.5.9.
- **A bare name after `:hypothesis-relations` or `:no-hypothesis`** is refused, the paper grammar taking a list;
  `search_semantics`' `lattice_relation_keyword_accepts_a_bare_symbol` pins it, and no corpus file writes it.
- **`value_label`'s `KwPair` arm is reached by no corpus file** (S1f.5.11's question): every `.ein` under the five
  roots through six `ein render` views on a build that reports the arm, 0 lines.
- **A `.einb` opens past its millisecond.** A container's cold open re-parses its PROGRAM section, so the generated
  parser's speed lands on it: a saturated `zebra2` opens in 3.7 ms on the generated build against 0.76 to 1.0 ms
  today (`cargo test --release -p ein-einb --test roundtrip`, three runs each), past design/10 § 6's 1 ms, and
  `roundtrip`'s dev-profile 5 ms fails at 5.4. The cost the user accepted with grmtools; recommended, T4 restates
  the budget, the container keeping the parsed tree left for when the millisecond is wanted.

**The tests and goldens T4 moves.** `cargo test --workspace --no-fail-fast` on the generated build
(`build/s1f.5.5/t9/proto-test.log`, 21 min 19 s): **834 passed and 109 failed, in 39 of 107 test binaries**, each row
of [the attribution's table](s1f.5.5_attribution.md#the-tests-t4-moves) with its ruling and what T4 does: 47 in
`ein-ir` (**T8's 47**, the same count, each a suite in today's language read through the generated grammar: its 19
unit tests, `ir_semantics` 11, `kb_semantics` 5, `load_semantics` 4, `grammar_decisions` 3, `imports_semantics` 3,
`dump_goldens` and `fuzz_properties` one each), 28 in `ein-infer`, 23 in `ein-cli`, 7 in `ein-render`, 3 in
`ein-core`, and `ein-einb`'s `a_saturated_zebra2_is_small_and_opens_cold_in_under_a_millisecond`. Beyond the stage's
list: the three named above (`region_dump`, `detector`'s random programs, `claim_round_trip`), `search_semantics`'
bare name, the `.einb` cold open above, and `config_reference`'s and `load_semantics`'
`require_why_decides_loading_only`, which the stage's `--require-why` bullet already covers (the stdlib's two macros
get their `:why`). Blessed (`EIN_BLESS=1`, 23 min 49 s: 841 passed, 102 failed, nothing new failing), **eight goldens
move**: `corpus_exits.txt` (87 of 1 390 cells), `corpus_shapes.md5` (89 files), `grammar_decisions.txt`,
`fuzz_seeds.txt`, `config_coercion.txt`, the two `branching/12` traces, and `load_error_order.txt`, the one S1f.5.4's
step 1 did not list ([§ The goldens T4 moves](s1f.5.5_attribution.md#the-goldens-t4-moves)).

**The register: 37 of 139 rows, exactly the stage's list.** `wrong_answers::every_row_answers_what_the_register_records`
on the generated build: 14 `open` rows the swap fixes (W29's six, W32's seven, W35) and 23 `fixed` rows it restates
(W7, W15 ×2, W16 ×3, W18 ×5, W20 ×5, W21, W24 ×3, W28 ×3); W34 does not move. S1f.5.4's proposal A for the W7 and W15
witnesses holds on the generated build: four states, the oracle the same.

**The review stop.** `build/s1f.5.5/review/index.md` (gitignored, 207 programs, every artefact both binaries write
for each, raw and rendered; 2.7 GB, `du -sh`, 2026-11-08): S1f.5.4's review rebuilt on the generated parser
(`features/05`, `std.elim`, `zebra2.ein` unmoved, `(p 1) (p 01)`, the 46, Q1 to Q4) and extended to every ruling: S1f.5.1's fixtures and W20's
three as written and rewritten, `zebra2.ein` with `_`-led names, its table of seventeen positions each computed, and
the refusal as the parse error; S1f.5.2's witnesses, `zebra2.ein`'s text untouched, a probe with Cyrillic, CJK and
an emoji on every surface, the escapes (`\u{1F993}` is 🦓 on the generated build, `u{1F993}` today) and the
refusal; W32's seven refusals and `wrappers.ein` rewritten; the 55 and where each goes, the six rewritten fixtures
and the refused programs with every error; the register's 37 rows with their new `now` and `right`; T8's leftover
(the engine's writeback names are read by nothing, only printed; the dunders are the documented way to ask for the
kernel's behaviour; recommended: the three writeback names renamed with `#` before the swap, the dunders kept); and
T4's expected tokens (recommended: the repairs stand, and the error text folds the eleven position words into *a
name* when it picks the three repairs to show). Its § 0 holds the questions, each with a recommendation. **Nothing was
edited beyond this record**: no engine code, test, golden, fixture or register row moved. T10 and T4 wait for the
user.

### The review stop ruled, 2026-11-08

The user read the stop the same day and ruled its § 0. None of Q1 to Q4, C1, C2 or the paper grammar's
defaults was overruled.

- **Item 1, `arg-kinds`:** *"Scope the solve"*. The corpus carries Q-M1f.2 (a)'s rewrite scoped with
  `(solve :no-hypothesis (r))`, R {∅}, exit 0. This is Q3's (b), and it overtakes Q3 (a). The 2^128 limit stays
  parked.
- **Item 4, the engine's names:** *"they are also reserved names"*. That rules out both the rename to `#` and
  leaving them as ordinary names. The five keep their spellings: `<forced-positive>`,
  `<monotonic-unconditional>`, `<lookahead-dies-immediately>`, `__closed__` and `__symmetric__`. Each is
  reserved in the sense
  [`06_reserved_names.md`](../../../docs/kernel/ir/03-ein-lang/06_reserved_names.md) and
  [`reserved_engine_strings.md`](../../../docs/kernel/inference/reserved_engine_strings.md) give the word: written
  only with its fixed meaning, never redefined. Read as follows (the coordinator's reading, for the user to
  correct):
  - **The three writeback names** have no meaning a program can write. A program that writes one, anywhere, is
    refused, so a printed `:rule <forced-positive>` is always the engine's.
  - **The dunders** keep the positions the corpus and the stdlib write them in: the relation of a fact, of a
    `:match` pattern and of an `:assert` (`(__closed__ is-a)`, `(__symmetric__ knows)`, `:assert (__closed__ ?R)`).
    Any other use is refused: a relation the program declares, a rule's name, or an argument value.
  - This is T1f.5.5.11 below. **T11 stopped on this reading's dunder half on 2026-11-08**, before any edit:
    [Q-M1f.7](../open_questions.md#q-m1f7--where-may-a-program-write-the-kernels-dunders). **The user ruled (b) the
    same day, *"Never bound"***, which corrects the reading's dunder half: a dunder is refused where a program
    binds a name (a declared relation, a rule's, an hrule's or a macro's name, an import's `:as`) and legal wherever
    one is used, as a relation or as a value, `(imply1 functional __closed__)` among them. The writeback half
    stands.
- **Item 12, a `.einb` cold open:** *"Restate the budget"*. T4 restates design/10 § 6's millisecond and
  `roundtrip`'s dev-profile 5 ms at what the generated parser costs, as the generator's accepted price. Keeping
  the parsed tree in the container, a format bump, waits until the millisecond is wanted.
- **Items 2, 3, 5 to 11:** *"Accept all as recommended"*.
  - **Item 2:** the W16, W18 and W28 rows whose witness writes `"1"` are restated once, to the refusal, each
    witness kept as the refusal's witness in its position. The two `activator-text` controls retire, and the
    two `activator-text-key` seeds stay as seeds refused at the parse.
  - **Item 3:** S1f.5.1's and S1f.5.2's refusal sentences become notes on the parse error.
  - **Item 5:** cross-check 1 holds as meant, and `assert-empty` is Q1's witness.
  - **Item 6:** the ranked repairs stand. The error text folds the eleven words a position gives a meaning to
    into *a name* when it picks the three repairs to show.
  - **Item 7:** a register row is filed for the template that cannot name every variable, `open` and owned by
    S1f.5.4, before anything fixes it (T10's commits).
  - **Item 8:** the 43 records under `plans/` stay as written. `utils/region_synth.py`, `guard_split.py` and
    `fuzz_engine.py` lose `:priority`, credited to S1f.5.15, before the swap if that moves no answer.
  - **Item 9:** W34's witness joins `claim_round_trip`'s exceptions until S1f.5.9 fixes W34.
  - **Item 10:** a list is the one form after `:hypothesis-relations` and `:no-hypothesis`, and
    `lattice_relation_keyword_accepts_a_bare_symbol` is restated to the refusal (T4).
  - **Item 11:** `value_label`'s `KwPair` arm goes with T1f.5.11.2.

### Task T1f.5.5.11 — the engine's names reserved

On item 4's ruling, in its own commit before the swap, like T8, if it moves no corpus answer; otherwise in T4's,
named there.

- **The rule:** a program that writes one of the three writeback names, in any position, is refused. A program
  that uses a dunder other than as the relation of a fact, a pattern or an `:assert` is refused.
- **Where:** in the generated grammar, with its own refusal text, and in the loader where the hand-written
  parser still reads the program until T4. Whichever holds the rule, both parsers agree.
- **The tests:** one per name and position, and T9's probe `build/s1f.5.5/t9/probes/engine-names.ein` refused
  at each of its five names. *Restated by Q-M1f.7 (b), 2026-11-08:* the probe writes its dunders as the relation
  of a fact, a use, so it is refused at its three writeback names; a probe binding each dunder in each binding
  position is refused at each, and each use stays legal.
- **The page:** `reserved_engine_strings.md` says the five are refused as a program's own names.

### T11 record, 2026-11-08

**Stopped before any edit, on [Q-M1f.7](../open_questions.md#q-m1f7--where-may-a-program-write-the-kernels-dunders)**,
put to the user. No engine code, grammar, test, golden, fixture, page or register row moved.

**The writeback half meets nothing.** No `.ein` under `examples/`, `tests/`, `stdlib/`, `corpus/` or `plans/`, and
no inline program of the Rust suites, writes `<forced-positive>`, `<monotonic-unconditional>` or
`<lookahead-dies-immediately>`; the one place in the crates that spells them is `ein_core::terms` (`grep`,
2026-11-08). Today's parser refuses each at its `<` (*2:4: unexpected input*, `(p <forced-positive>)` and its two
twins), and T9's generated build loads each, as a value, as a relation and as a rule's name, whose firings then
print as the engine's writebacks (T9's probe). No `(trace …)` the engine writes is read back through the parser
on any command (`trace_to_ir` has no caller outside the crate's exports), so refusing them in every position a
program writes costs no engine output.

**The dunder half meets two things the reading did not see**, Q-M1f.7's two bullets: the stdlib documents
`(imply1 functional __closed__)`, a dunder as an argument value, which `algebra_semantics` holds and both builds
derive `(__closed__ foo)` from; and the task's test, T9's probe refused at each of its five names, cannot hold
under the reading, which keeps the probe's two dunder facts legal. Neither moves a corpus answer: no corpus
program writes a dunder anywhere but as the relation of a fact, a premise or a conclusion, the positions every
reading keeps (`branching/12`, `features/06`, `stdlib/closure.ein`, `tests/stdlib/closure/02` and `03`). The
positions no reading stated (a macro's name, an import's `:as`, a goal, a claim, a nested fact's relation, a
`:no-hypothesis` list, a fact's `:rule __symmetric__`) all load on both builds today (the probes under
`build/s1f.5.5/t11/`, 2026-11-08), and a dunder declared as a relation or named as a rule loads too, its firings
printing `:rule __symmetric__`, the native mirror's provenance.

**What T11 does once Q-M1f.7 is ruled**, under every option: the three writeback names are tokens of their own in
`ein.l`, before `NAME`, that no production takes, so the generated grammar refuses each where it stands with its
own `%epp` text, as it refuses `_`; the dunders' binding positions join the loader's *shadows a reserved kernel
name* check (`ein_core::is_reserved`'s callers in `from_ir.rs` and `macros.rs`, and the import alias), which both
parsers feed, so they agree without an edit to `lex.rs` or `parse.rs`; under (a) a loader line refuses a dunder
as an argument too. The pages: `reserved_engine_strings.md` and `06_reserved_names.md`.

**Built the same day, on the user's ruling of Q-M1f.7, (b) *"Never bound"* (`feb5e839`)**, in one commit, as
planned above; no edit to `lex.rs` or `parse.rs`.

- **The dunders.** `ein_core::RESERVED`, the names a declarator may not bind, gains `__closed__` and
  `__symmetric__` (eleven names), so the loader's existing check refuses each as a declared relation and as a
  rule's, an hrule's or a macro's name, *… '__closed__' shadows a reserved kernel name*, through both parsers and
  through every import route (`imports_semantics`' `every_ein_core_reserved_name_is_unbindable_through_a_qualified_import`
  reads the list, so it holds the dunders too). An import's `:as` takes the same check, *(import M :as __closed__)
  — alias '__closed__' shadows a reserved kernel name*, which also refuses an alias that is one of the other nine;
  no program writes one (`grep ':as '` over every `.ein`, 2026-11-08: four aliases, each `m` or `"m"`). `qualify` leaves a module's
  dunder unrenamed, so the loader refuses it there too. Every use stays legal.
- **The writeback names.** `ein.l`'s `WRITEBACK` token, `[<](forced\-positive|monotonic\-unconditional|lookahead\-dies\-immediately)[>]`,
  before `NAME`, so that the one of two matches of one length the lexer takes is it, and `ein.y` lists it under
  `%expect-unused` with its `%epp`, so the generated parser refuses each where it stands: *unexpected
  `<forced-positive>`: the engine's writeback rule name (reserved)*. A longer run, `<forced-positive>x`, is the
  name it spells, and the name in a string or a comment is text. The hand-written lexer has no token that starts
  with `<`, so it refuses the same files at the same column.
- **The tests**, `ein-ir/tests/engine_names.rs`, five, every program through both parsers: each writeback name in
  eighteen positions refused, at the name, by both; the name in a string, a comment and a longer run not refused;
  T9's probe, verbatim, refused at its four occurrences of the three names and nowhere else (the generated parser:
  5:11, 7:7, 8:7, 8:61; the hand-written: 5:11); each dunder in the five binding positions refused with the
  loader's line; and one program using both dunders as the relation of a fact, a nested fact, a premise, a
  conclusion, a goal and a claim's fact, and as a value (an argument, `(imply1 functional __closed__)` from
  `std.algebra`, a `:no-hypothesis` list, a fact's `:rule`), loaded by both. `ir_semantics`'
  `the_kernel_names_a_macro_may_not_shadow` takes the two dunders, and the comments that counted nine count eleven
  (`ir_semantics`, `kb_semantics`; `lex.rs`'s, which T4 deletes, stay).
- **The pages**: `reserved_engine_strings.md` (a table of the five: what a program may write and what refuses
  it), `06_reserved_names.md` (the guard's list, the table of `RESERVED`, the two dunder rows) and
  `defined_behaviour.md` § 4.2 (*the nine names*, eleven since). `(imply1 functional __closed__)`, its comment in
  `stdlib/algebra.ein`, its row in `07_stdlib_api.md` and `algebra_semantics`' test stay as they are.
- **What moved:** no corpus answer, golden, fixture or register row (no corpus program binds a dunder or writes a
  writeback name: the `grep` above). The gate below is the check.

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

### T10 record, 2026-11-08

Two commits, before the swap and before T11.

**W37 filed** (`b7561e93`), item 7 of the stop: `W37/template-var-star`, `open` and owned by S1f.5.4, its witness
`examples/ein-bugs/wrong-answers/template-var-star.ein`, a corpus entry. A `:goal-text` that names `{?n*}` prints as
written, *“{?n\*} is a q”*, where *“a is a q”* is right, and the rule's `:why` likewise on `--trace`, as the
stdlib's three `{?isR*}` templates do today. The register's schema `/15` records what a query's `:goal-text`
prints (`goal_text`, read off the read-out, which the summary does not carry). T9's generated build parses the
witness and answers the same, so the swap does not move the row; S1f.5.4's
[T1f.5.4.5](s1f.5.4_digits_are_names.md#task-t1f545--a-template-names-every-variable-w37) fixes it after the swap.
The register reads **103 rows fixed, 36 open and 1 not-wrong** (`grep '^status' corpus/wrong_answers.toml | sort |
uniq -c`, 2026-11-08). Item 8 landed between the two, as S1f.5.15's
([its T9 record](s1f.5.15_rule_priority.md#t9-record-2026-11-08), `4c272aae`).

**The fixture moves that move no answer** (`9e54d3a8`). Each of the three candidates was put through the test, today's
binary (`a50cf081`'s engine) on the fixture as written and on Q-M1f.2 (a)'s rewrite
([`s1f.5.5_parser_probe/programs/`](s1f.5.5_parser_probe/programs/)), every declared run, `ein saturate --dump`
and `ein solve -H --json-summary --dump-answer` (`build/s1f.5.5/t10/wrappers/`, 2026-11-08):

| candidate | the rewrite | today's binary on it | lands |
|---|---|---|---|
| `broken/load/derivation_cycle` | `:using ((r c d))` | refused **at the parse**, `7:24: unexpected input`, exit 1, where the fixture is refused at load with *derivation cycle: (r a b) -> (r c d) -> (r a b)*, its `.expected` | T4 |
| `syntax/unary-and-nary` | `:using ((u a))` | refused **at the parse**, `12:32: unexpected input`, exit 1, where the fixture saturates, exit 0 | T4 |
| `syntax/wrappers` | the inner facts' pairs gone | `render rules` exit 1 (*no rule forms*) and `saturate`, `saturate --dump` and `solve` exit 0, as written; the same three stored facts, R {∅} (k_s 1, k_c 1), N 0, H 0, the summary and the state file's state the same but for the file's name | **here** |

Step 2's evidence said both parsers refuse `derivation_cycle`'s rewrite with today's line; T9 found today's parser
refuses `:using ((…))` at the parse, so the line is the generated build's alone, and the attribution's three test
rows that name T10's rewrite of it (`cli_semantics`' two, `render_doors`') are T4's. **One fixture moves:**
`syntax/wrappers.ein` loses `:source "(1)"` and `:rule sym :using (x)` from its inner facts, which today's loader
dropped, its header saying so; the rest of the program and its node-kind shapes stay. **The goldens it moves, named
before the bless:** `corpus_shapes.md5` (its parse-view digests, the pairs no longer drawn) and
`generated_parser_corpus.txt` (`wrappers` from *the hand-written parser accepts, the generated one refuses* to *both
accept, same tree*). `corpus_exits.txt` does not move (its two cells keep their exit codes). With it the swap
carries one file fewer: `wrappers`' row of the attribution (S1f.5.11's) and its name in
`every_positive_entry_answers_under_at_least_one_run`'s row.

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
- **No `:priority`.** With S1f.5.15 landed no program writes one, and the grammar refuses it like any keyword a
  rule does not take (T3), so `:priority 110` puts no name in the program and S1f.5.4's guard (T1f.5.4.3) has
  nothing to guard. `from_ir.rs`' `priority` match, the trap S1f.5.4's step 1 found, is S1f.5.15's to remove with
  the keyword. Until the user's rulings of 2026-11-08 this bullet kept `:priority DIGITS`, built as `Node::Int`
  by `b::count`, which T3 dropped. If S1f.5.15 ended in a written refusal, the production is back, and the guard
  is S1f.5.4's again.
- **A macro's `:why` under `--require-why`** (added 2026-11-08, when S1f.5.14's
  [T1f.5.14.6](s1f.5.14_nl_required.md#t1f5146-record-2026-11-08) landed the loader's flag early). The flag asks a
  `(macro …)` for its `:why` only under `#[cfg(feature = "generated-parser")]`, since until this commit the shipped
  parser refuses one. This commit deletes that `cfg` in `from_ir.rs`' `load_query_with`, with its `not` twin
  `WHY_REQUIRED_OF`, and the first arm of `load_semantics`' `a_macro_needs_a_why_only_where_its_parser_reads_one`
  and of `require_why_decides_loading_only`; and it writes the stdlib's two macros their `:why`, `forall` and
  `unknown` in `stdlib/macro.ein`, with `MANIFEST.sha256` updated, so that the stdlib still loads under the flag.
  The flag's line names a macro from then on (`relation, rule, hrule and macro`), and `defined_behaviour.md`
  § 4.10, `configuration.md` § 3.4 and the option's help are restated. The default stays false, so no answer moves.
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

### T4 record, 2026-11-09

**One commit, on `977829bf`** (T11 landed). The user's go-ahead: *"ok, continue with T4 after T11"*. Every ruling of
[§ The review stop ruled](#the-review-stop-ruled-2026-11-08) is carried; T9's prototype
(`build/s1f.5.5/t9/swap-prototype.patch`) was the start.

**The parser.** `ein_ir::parse` is `ein_ir::grammar::parse`, and `ParseError` is the report type T3 called
`Refusal`. `lex.rs` and `parse.rs` are deleted with the hand-written lexer's unit tests, T3's hook
(`parse_as_hand_written`) and the `generated-parser` feature, which has nothing left to switch. The generated
parser bumps the `parse_call` and `parse_bytes` counters the hand-written one did. `grammar::RESERVED_WORDS` (the
twelve words never a name, `ein_core::text::LEXER_KEYWORDS`' source) and `grammar::POSITION_WORDS` (the eleven a
position gives a meaning to) are public. `ein-ir/tests/generated_parser.rs` keeps the parser's own tests (twice the
same, the cut, the macro's `:why`, the internal names, the nesting limit) and loses
`the_builder_holds_on_the_corpus`, which held the generated tree to the hand-written one, with its golden,
`generated_parser_corpus.txt`: with one parser there is no second tree, and T9 attributed every file where the two
differed before the old one went. `engine_names.rs` and the `generated_parser` example read the one parser.

**The error text.** Each error recovery reaches is one line, `file:line:col: unexpected X`, with the token's
`%epp` description where it has one, then its notes, then at most three ranked repairs and `and n more repairs`; a
report cut at the budget says so after its last error. No line names *the tokens expected*: the repairs stand for
them (the review stop's item 6; T3's [§ Notes](#notes) item). The duplicated `unexpected input` header T9's
prototype carried, a stand-in from T3's hook, is gone.
- **The fold** (item 6). Where a place takes a name, each of the eleven position words is a name there too, so the
  repairs that differ only in which of them, or which name, they insert are one repair, *Insert a name*, listed at
  the rank of the first. Read literally, *fold the eleven into a name* where a repair inserting a name is ranked
  beside them folds nothing in many places: CPCT+ ranks the words and not `NAME`. At `(tag ?x (box a :k v))`'s `:k`
  it ranks fourteen repairs, *Insert* each of the eleven words, a variable or a `(`, then *Delete `:k`*, and none
  inserting a name (T9's proto build with the shown repairs raised to 100, 2026-11-09). So a place also takes a
  name where four of the words or more are repairs there: every production of `ein.y` that takes four of them
  takes `NAME` (`Name`, `DeclName`, `FactHead`, `PremiseHead`), and the places that take fewer and no name, a
  flag's `true` or `false`, a claim's `model`, `merged` or `owing`, a solve's `region`, keep their words, so no
  report says *Insert a name* where no name parses. `(eq ?x)`'s three repairs, *Insert a variable*, *Insert
  `owing`*, *Insert `merged`* and ten more on T9's prototype, are *Insert a variable* and *Insert a name*.
- **The notes** (item 3 and the paper grammar's default 5): `_ is reserved and names nothing: where any value will
  do, write a variable used once` at a `_` or a `?_` (S1f.5.1's sentence); `"a" is a string, and a string is
  natural-language text, written only after :why, :source or :goal-text: write a name bare` at a string, quoting it
  (S1f.5.2's); `:no-hypothesis chooses the lattice and belongs on (solve …) since M1e2 S1e2.1a.4 — one program,
  one lattice` at a lattice keyword in a `(query …)`; and `config flag :lattice-order was retired at M1e2
  S1e2.1a.5: …` at a retired flag in a `(config …)`, read from `ein_core::config::RETIRED`, the loader's two lines
  the grammar no longer lets a program reach.
- **What moved with it**: the thirty `.expected` under `examples/broken/` (the four parse-negatives, the twenty-five
  moved and `kwpair-below-the-filter`), each the report with the path written `{FILE}`, and the four
  `broken/load/reserved_open_*` whose import of `reserved_open_direct.ein` is refused at its parse;
  `grammar_decisions.txt`, `recovery_cut.txt` (the `_`'s note) and the goldens below.

**The printer.** `ein_core::text::is_symbol` is the name rule: a run of name characters, not a bare `_`, not one of
the twelve reserved words and, beyond T9's prototype, not one of the engine's three writeback names, which the
lexer reads as `WRITEBACK` since T11. `grammar.rs`' `the_printers_symbol_is_the_lexers`, moved from `lex.rs`, holds
`is_symbol` to `is_one_name` (or a position word, a name wherever a name goes) over every reserved and position
word with every boundary class, and every ASCII character in every position of a short name;
`the_reserved_words_are_the_printers` holds the two lists to one. `(p 1) (p 01)` print as two facts and reload to
themselves.

**The JSON writer** (S1f.5.4's Q4 (a)): every binding in `summary.json` is a string, so `int-goal-binding`'s `8`,
`-7` and `99999999999999999999999` are `"8"`, `"-7"` and `"99999999999999999999999"`; its header and corpus note
say so, and `cli_semantics`' `a_goal_binding_of_digits_is_a_json_string` (until now
`an_integer_goal_binding_stays_a_json_number`) holds it. **No schema id moves**: no reader of `ein-summary/2` reads
a binding's JSON type (`feature_matrix.py` stores them, and no Rust reader does; `grep`, 2026-11-09).

**A macro's `:why`.** The `cfg` arms in `from_ir.rs` (`WHY_REQUIRED_OF` and its twin, the macro check in
`load_query_with`) and the first arms of `load_semantics`' two tests are gone; the flag's line names `relation,
rule, hrule and macro` in every build. `stdlib/macro.ein`'s `forall` and `unknown` carry their `:why`, and
`MANIFEST.sha256` moved with them (`utils/stdlib_manifest.py --write`). Restated: the option's help (so
`help_shape.txt`), `configuration.md` § 3.4 and its opening note, and `defined_behaviour.md` § 4.10, where a `:why`
that is not a string is now a parse error. The default stays false; no answer moved.

**S1f.5.3's interim, gone.** `ein_corpus::REFUSED_UNTIL_S1F_5_4` and `corpus_cli`'s
`the_files_refused_until_digits_are_names_are_refused` are deleted, and its readers (`every_positive_entry…`, the
round trip, `SEEDED`, the dump fixed point) read both files. `cli_semantics`' `a_range_is_no_token` is
`a_range_is_the_name_it_spells`: `(r 1..5 a)`, `(relation q 1..* Node)`, a `:match` over `1..3` and an `:assert`
writing `2..*` load, read and print as written; `:card 1..*` is refused at `:card`, a keyword no form takes; and
`(config :lattice-sanity-check 1..5)` is refused at the name, a flag taking `true` or `false`. The pages that said
*until S1f.5.4 makes it one name* are restated: `00_ebnf.md`'s fact 3, `01_grammar.md`'s `INT` row and its no-`RANGE`
paragraph, `defined_behaviour.md` § 4.8 (and § 4.6, whose integer rows the name rule makes unreachable),
`examples/README.md`'s `syntax/` row, Q-M1e2.13 and Q-M1e2.24.

**The register** (`corpus/wrong_answers.toml`, held by `wrong_answers.rs`, `register_region.rs` and
`ein-corpus`'s well-formedness tests). **14 rows flipped to `fixed`**: W29's six, W32's seven and W35, each `now`
the parse refusal's first line, each `right`'s *at load, with a line naming the place* restated *at the parse,
with the file, line and column*, each `[case.region]` dropped (a refused `now` states none); `W29/core-two-ways`
dropped its core and `W32/kwpair-in-goal` its query. **23 fixed rows restated**, as the attribution and item 2 say:
W16's two `"1"` witnesses, W18's five and W28's three to the refusal (S1f.5.2), W20's three matching witnesses to
the refusal (S1f.5.1), each witness kept as the refusal's in its position, W16's two `control` blocks dropped with
the retired controls; `W20/range-in-match`'s `right`, a pattern over the name `1..3`, R the cube of 8, `(hit r)`
exactly where `(n 1..3)` holds; `W7/binding-key-layout` and W15's two on the scoped witnesses, R four states,
their regions re-banked (`EIN_BLESS=1 cargo test -p ein-infer --test register_region`); `W21/typed-blind-solve`'s
`now` and `right`, N 25 → 51 sets, the twenty-six new singletons each `color-of` with `0` or `1` on a side and its
own core (F8); and the refusal's line in `W16/activator-all-int` (`[activator 1]`), `W20/wild-in-head`, and W24's
`wild-in-guard`, `bare-premise` and `kwpair-in-guard`, the last four refused at the parse since. **Which
`[case.fixed]` names this commit:** the stage's lead sentence, *its `[case.fixed]` names this commit*, read as: a
row whose `right` this commit turned into another kind of answer, an answer into a refusal or a refusal into an
answer, names it (the fourteen flipped, and W16's two, W18's five, W28's three and W20's four), its earlier fix kept
in its note; a row whose `right` it re-took on a scoped witness or a grown N (W7, W15's two, W21) or whose `now`
alone moved keeps its fix. Each of the 28 carries the null id, `0000000`, the fixing commit not being able to name
itself; the stage's next commit writes the hash, as S1e2.2.24's did. `ein-corpus`' three schema tests whose live
example the swap turned into a refusal (schema /7's `now` held against a refusal, /12's claim and classes, /13's
core and query beside a refusal) read the rows as they stood at `977829bf`, embedded. W34 does not move (T8's), and
**W37 still holds** (`wrong_answers` green). The register reads **117 rows fixed, 22 open and 1 not-wrong**
(`grep '^status' corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-09).

**The fixtures.**
- **Moved to `examples/broken/`** (Q-M1f.2 (a)), each a `parse-negative` entry, its runs and note kept, its
  `.expected` the report: `broken/compile/`'s four arity fixtures and `broken/load/`'s twenty-one (`config_bad_value`,
  `config_retired_flag`, `config_unknown_flag`, `empty_absent`, `expect_is_a_pattern`, `expect_unknown_keyword`,
  `hrule_reserved_name`, `import_alias_not_a_name`, `import_symbols_empty`, `import_symbols_not_a_list`,
  `macro_reserved_name`, `open_arity`, `open_in_match`, `open_mixed_assert`, `query_lattice_keyword`,
  `region_top_level`, `relation_malformed`, `relation_needs_a_name`, `relation_reserved_name`,
  `rule_reserved_name`, `solve_unknown_keyword`); `reserved_open_direct` stays. And `ein-bugs/kwpair-below-the-filter`
  (S1f.5.11's default), its header and its fuzz find's note saying why; the seed stays a seed.
- **Retired**: `broken/load/relation_parameter_int` with its `.expected` and corpus entry (Q2 (a)), and the two
  `activator-text` controls, `-int-only` and `-string-only` (item 2); the two `activator-text-key` seeds stay,
  refused at the parse, and the two witnesses lose their `test` cell with their claim.
- **Rewritten**: `broken/load/derivation_cycle` (`:using ((r c d))`, its cycle line reached again), and
  `syntax/unary-and-nary` (`:using ((u a))`), `syntax/arg-kinds` (Q-M1f.2 (a)'s rewrite, scoped `(solve
  :no-hypothesis (r))`, R {∅}, item 1), `syntax/is-a-shapes` (`7` for the `?x` and the `_`) and `syntax/negation`
  (`(not (plain))`); each new relation carries a `:why`.
- **Scoped** (S1f.5.4's proposal A): the four `binding-key-*` witnesses, each `:expect` restated to its R (four
  states; `-int-only` two) and its header saying why, and the two `binding-key-layout` seeds.
- `SEEDED` is 27 rows (S1f.5.4's moves `stdlib/elim`'s `0` and `1`, `relation-decls`' `7`, `range-in-match`'s
  `1..3`, and `arg-kinds`' `a` and `b`; out with their files' refusal, `eq_arity_high`'s `A` and `B`,
  `kwpair-below-the-filter`'s four, W18's `lbl` twice, W28's `"1"`, W32's two `v` and W29's `a`); `detector.rs`'
  flagged count 49 → 48 (`kwpair-in-macro` refused); no `slow` flag moves (the scoped witnesses cost what they did).

**The tests.** Every test T9 named moved as the attribution says, and the ones it did not name are below. The ein-ir
suites (T8's 47 and the rest) are restated in the final grammar: a shape the grammar refuses is asserted as the
parse error, `line:col: what`; a program that wrote old syntax incidentally is rewritten so the test keeps its
subject. Beyond T9's list: `cli_semantics`' `a_file_that_starts_like_a_container_but_is_not_one_is_text_in_either_build`
and `primitive_arity`'s `the_shape_pinned_primitives_refuse_a_wrong_arity`, which matched `unexpected input`, the
prototype's stand-in header; `imports_semantics`' `one_module_under_two_qualifications_does_not_leak_either_way`,
moved by the stdlib macros' `:why`; `config_reference`' `the_argument_surface_is_stable` (the help);
`golden_dot`'s three fixtures, whose `:using` and trace shapes the grammar refuses; and `ein-corpus`' three schema
tests above. `load_semantics`' `load_errors_accumulate_in_pass_order` and `from_ir`'s
`every_pass_accumulates_rather_than_stopping_at_the_first_problem` gained a program the loader still reads, their
old ones being parse errors now, so they still test the loader's pass order. `grammar_decisions`'
`a_keyword_is_a_whole_word_in_every_place` reads the hand-written parser's nineteen literals as written, and its
`-1` parses now read the name `keyword-1`: 348 of them parse (`cargo test -p ein-ir --test grammar_decisions`,
2026-11-09), where 435 read the keyword and `-1` until now, which overrides Q-M1e2.17 (i)(a).
`answer_round_trip`'s witnesses are two programs written in the test (`7` beside `07`, `-7` and `1..5`; every
decorator and the brackets, bare, nested and derived) and `arg-kinds`. W34's witness joins `claim_round_trip`'s
`DO_NOT_READ_BACK` until S1f.5.9 (item 9). `search_semantics`' bare name is restated to the refusal, its first
repair the list (item 10). `region_synth.py` needed nothing: `region_dump`'s and `detector`'s random-program tests
pass (T1f.5.15.9 took `:priority` out).

**The `.einb` cold open** (item 12): design/10 § 6's millisecond and `roundtrip`'s 5 ms are restated to 6 ms
(release) and 20 ms (dev). Measured on this commit, a saturated `zebra2` opens in 3.83 to 3.89 ms release and 5.40
to 5.48 ms dev (`cargo test [--release] -p ein-einb --test roundtrip a_saturated_zebra2 -- --nocapture`, three runs
each, 2026-11-09), the generated parser reading `PROGRAM`'s 10 960 bytes.

**The cross-check: no answer moved that T9 did not list.** Every `.ein` under the five roots (511, `find examples
tests stdlib corpus plans -name '*.ein' | wc -l`, 2026-11-09), on the tree as this commit leaves it, through T9's
generated build (`build/s1f.5.5/t9/proto`) and this commit's release build: `ein solve FILE --json-summary J -H
--dump-answer D`, `ein saturate FILE --dump` and, where the file states an `:expect`, `ein test FILE`, comparing
exit codes, the summary (its `source` and `work` aside), the stored facts, `ein test`'s output and N's
cores (`build/s1f.5.5/t4/xcheck/`, 77 s). **505 files answer the same on every command, 6 differ only in the width
of `saturate`'s timing lines, and 137 differ only in their stderr**, the error text above. T9 attributed every
move from today's parser to its generated build; this commit moves none past it.

**Fourteen goldens move**, T9's eight and six more, each read: `corpus_exits.txt` (80 cells, each a move above,
the moved fixtures' cells under their new paths and the retired files' gone), `corpus_shapes.md5` (158 files: the
refused files' views, a refused file keeping its parse row; the moved and retired fixtures; the rewritten and scoped
ones; S1f.5.4's name rule on `branching/12`, `features/05`, `activator-all-int`, `int-goal-binding`,
`mixed-type-hypothesis`, `relation-decls`, `stdlib/elim` and `tests/stdlib/elim/01`, `02`; and the
`ir[resolve]`, `ir[minimize]` and `ir[expand]` views of every program importing `std.macro`, which inline its
macros' new `:why`), `grammar_decisions.txt`, `fuzz_seeds.txt` (the seeds the grammar refuses, each with its
report, and the moved fixtures, which are seeds by their directory), `config_coercion.txt`, `load_error_order.txt`
(its four programs parse errors now, and a fifth the loader reads), the two `branching/12` traces
(`trace_ambiguous.md`, `trace_one-hypothesis.md`: N and the universe 25 → 51), and beyond T9's list
`recovery_cut.txt` (the `_`'s note), `help_shape.txt` (`--require-why`'s help), and `golden_dot`'s
`ir_render_trace_a.dot` and `ir_render_trace_dag.dot` (the fixture's second step names its step and a given, `:using
(and s1 c15)`, where it wrote a fact the grammar refuses there). `generated_parser_corpus.txt` is deleted with
the test that read it. The `.expected` beside the thirty parse-negatives and the four `reserved_open_*` are the new
reports. The bless ran `EIN_BLESS=1 cargo test --workspace --no-fail-fast`: 939 passed, none failed (23 min 19 s,
2026-11-09).

**The nightly's two scripts**, run on this commit's release build: `utils/stdlib_mutants.py --check` kills 167 of
217, as on 2026-11-08, its survivors the banked set, and `utils/stdlib_census.py --check` covers 77 of 77 rules
(2026-11-09).

**The gate.** On `977829bf`, before this commit, 14 min 14 s (`build/gate-s1f.5.5-T11.log`, T11's gate on the tree it
committed). On this commit, 19 min 59 s (`build/gate-s1f.5.5-T4.log`), green: 939 tests, every static check, 5 min 45 s
more. Not traced here; the suites start thousands of processes that each read a program, and the generated parser
costs more a parse than the hand-written one did (the `.einb` open above).

**Found here, each a report**: `ein_render::trace`'s `trace_to_ir` and `parse_trace_steps` have no caller, which the
paper grammar's default 8 says go at the swap; they stay, moving nothing, for T5 or T7 to delete. A `\u{…}` beyond
Unicode's range still reads as U+FFFD (`build_ast.rs`' `unescape`), which no file writes; refusing it is S1f.5.2's
after the swap.

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
- **The grammar is the phase's final one** (the user's rulings of 2026-11-08). It has no `:priority`: a rule's
  `:priority` is refused like any keyword a rule does not take, and no program writes one, since S1f.5.15 runs
  before the swap. If S1f.5.15 ends in a written refusal, the production comes back, and with it this item as it
  stood until then: `:priority` takes a run of digits and nothing else, and puts no name in the program. **Every
  form that can carry a template takes `:why`**, optional: a relation, a rule, an hrule and a macro, with
  `:goal-text` on a query and `:source` on a fact. Whether a form must carry one is S1f.5.14's loader flag, not
  the grammar.
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

## Notes

- **Parked for T4 by T3, 2026-11-08: the tokens expected at an error.** § The error text has each error report
  *the tokens expected there*. Read off the state table, which is what `lrpar` exposes beside an error (its
  state), the set overstates: Pager's merged tables reduce on a lookahead before the error is found, so at
  `four_mistakes.ein`'s `a?x`, a fact's argument, it offered `:why`, `:goal` and nine more keywords no fact
  takes. The exact set is what parses on from the error's stack, which is how CPCT+ checks a repair, and the
  stack and the table are inside `lrpar` (the generated module keeps the table private). Two ways: a third
  change to the vendored crate, recording that set on each `ParseError`; or the ranked repairs stand for it,
  since each `Insert` in them is a token the parse goes on with. **Recommended: the repairs stand**, and the
  error text says no more; T4 decides, with the error text, and T3's report leaves the line out. *Ruled 2026-11-08
  as recommended (the review stop's item 6), and built at T4* ([§ T4 record](#t4-record-2026-11-09)): no line
  names the tokens expected, and the eleven position words are folded into *a name* where a place takes one.
