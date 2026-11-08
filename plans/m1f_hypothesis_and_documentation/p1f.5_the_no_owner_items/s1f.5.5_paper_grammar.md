# S1f.5.5 step 2 — the grammar on paper, and real programs (T1f.5.5.2)

**Stage:** [S1f.5.5](s1f.5.5_the_grammar.md). **Written** 2026-11-08, on a snapshot of `ec004a48`.
Step 1 is [`s1f.5.5_census.md`](s1f.5.5_census.md). **Stopped here, to show the user before any
edit** (the standing rule on syntax): every program below is a real corpus program, as written or
rewritten against the final grammar, and every answer was computed by the engine, today's and the
prototype's. Nothing under `ein.rs/`, `corpus/`, `examples/`, `tests/`, `stdlib/` or `docs/` moved.

**The grammar** is [`ein.l`](s1f.5.5_parser_probe/grammar/ein.l) (92 lines) and
[`ein.y`](s1f.5.5_parser_probe/grammar/ein.y) (324 lines, 82 nonterminals), in grmtools' notation, with
one action per alternative, each a call into [`build_ast.rs`](s1f.5.5_parser_probe/grammar/build_ast.rs),
which makes today's `ein_ir::Ast`. It builds with no conflict (`CTParserBuilder` fails on one) and CPCT+
recovery on, against the probe's deterministic `lrpar`.

**The prototype that computes the answers** is a snapshot of the tree whose `ein-ir` compiles those two
files and reads a program through them when `EIN_PARSER=grm` is set: `parse` calls the generated parser,
and everything after it, macros, imports, the stdlib, the loader and the solver, is today's code
([§ Re-taking it](#re-taking-it)). What the prototype prints that a later stage changes is marked: the
printer quotes a name of digits, `"7"`, until S1f.5.4 makes the printer print it bare.

*Re-planned 2026-11-08, after this page was written* ([S1f.5.5 § The order](s1f.5.5_the_grammar.md#the-order-re-planned-2026-11-08)):
what this page has landing before the swap, on the hand-written parser, lands in the swap's commit instead,
credited to the same rulings: S1f.5.4's printer and `features/05`'s N, and the 23 files of
[Q2](#q2--the-55-files-the-grammar-refuses-that-todays-parser-accepts) the owning stages were to refuse first.
Of Q2's own moves, what moves no answer lands just before the swap (T1f.5.5.10).

## The grammar, by position

The lexical grammar. A token runs to a separator; no token starts inside another.

```text
NameChar   ::= [A-Za-z0-9] | '_' | '-' | '.' | '$' | '%' | '^' | '*' | '+' | '=' | '<' | '>' | '!' | '&'
             | '@' | '~' | '/' | "'" | '`' | '[' | ']' | '{' | '}'
NAME       ::= NameChar+       except a word below, `_`, or digits alone (DIGITS, itself a name)
VAR        ::= '?' NameChar+   except `?_`
STRING     ::= '"' ( [^"\] | '\' [ntr"\] | '\u{' hex{1,6} '}' )* '"'      any Unicode
KEYWORD    ::= one of the keywords below; any other `:word` is refused where it is written
Separator  ::= white space | U+001C–U+001F | '(' | ')' | ';' … end of line | '#|' … '|#'
never a name:          not and or neq rule hrule query solve config trace macro import
names, and words a position gives a meaning to:
                       relation step eq absent false true open model region merged owing
```

The phrase grammar, as `ein.y` states it (`A - B`: an `A` that is not a `B`).

```text
Program      ::= Form*
Form         ::= Relation | Rule | Query | Solve | Config | Trace | Macro | Import | NotFact | Fact
Name         ::= NAME | DIGITS | relation | step | eq | absent | false | true | open | model
               | region | merged | owing
DeclName     ::= Name - (relation | eq | absent | false | open)        /* ein_core::RESERVED */

Fact         ::= '(' (Name - (relation | region | merged | owing)) FactArg* FactClause* ')'
NotFact      ::= '(' not NestedFact FactClause* ')'
NestedFact   ::= '(' Name FactArg* ')'
FactArg      ::= Name | NestedFact | '(' not NestedFact ')'
FactClause   ::= :source STRING | :rule Name | :using '(' NestedFact+ ')'

Relation     ::= '(' relation DeclName Name* (:why STRING)* ')'
Rule         ::= '(' (rule | hrule) DeclName '(' VAR* ')' RuleClause* ')'
RuleClause   ::= :match Match | :assert Assert | :why STRING | :priority DIGITS

Match        ::= '(' ')' | Premise                                   /* () the empty conjunction */
Premise      ::= '(' ((Name - (eq | absent | open)) | VAR) Arg* ')' | Connective
Connective   ::= '(' and Premise+ ')' | '(' or Premise+ ')' | '(' not Pattern ')'
               | '(' absent Premise ')' | Guard
Guard        ::= '(' (eq | neq) Term Term ')'
Arg          ::= Name | VAR | '(' ((Name - (eq | absent)) | VAR) Arg* ')' | Connective
Pattern      ::= '(' (Name | VAR) Term* ')'
Term         ::= Name | VAR | Pattern | '(' not Pattern ')'
Assert       ::= '(' ')' | Conclusion | '(' open (Name | VAR)? ')'
Conclusion   ::= '(' ((Name - (eq | false | open)) | VAR) Term* ')' | '(' and Conclusion+ ')'
               | '(' not (Pattern | VAR) ')' | '(' false ')' | Guard

Query        ::= '(' query QueryClause+ ')'
QueryClause  ::= :goal Premise | :goal-text STRING | :over Name | :expect Claim
Claim        ::= Model | '(' or Model+ ')' | '(' false ')'
Model        ::= '(' model ClaimFact* ')'
ClaimFact    ::= NestedFact | '(' not NestedFact ')' | '(' neq FactArg FactArg ')'
Solve        ::= '(' solve SolveClause* ')'
SolveClause  ::= :hrules Hrules | :hypothesis-relations NameList | :no-hypothesis NameList
               | :expect RegionClaim
Hrules       ::= '(' ')' | '(' Name (Name | '(' Name+ ')')* ')'
NameList     ::= '(' Name* ')'
RegionClaim  ::= '(' region RegionItem* ')' | '(' false ')'
RegionItem   ::= Model | '(' merged Model (Model | '(' or Model+ ')') ')' | '(' owing Model+ ')'
Config       ::= '(' config ((:lattice-sanity-check | :record-alternative-justifications) (true | false))* ')'

Macro        ::= '(' macro DeclName '(' VAR+ ')' Template ')'
Template     ::= VAR | '(' (and | or) Template+ ')' | '(' not Template ')'
               | '(' neq TemplateItem TemplateItem ')' | '(' (Name | VAR) TemplateItem* ')'
TemplateItem ::= Name | Template
Import       ::= '(' import Name (:symbols '(' Name+ ')' | :as DeclName)* ')'
Trace        ::= '(' trace Step* ')'
Step         ::= '(' step Name StepClause* ')'
StepClause   ::= :rule Name | :using ('(' Name+ ')' | '(' and Name+ ')') | :derives Derived
               | :source STRING | :assumes Derived
Derived      ::= NestedFact | '(' not NestedFact ')'
```

*A dated note, 2026-11-08, after this block was accepted: T3 built it with two changes, both the user's rulings of
that day* ([S1f.5.5 § T3 record](s1f.5.5_the_grammar.md#t3-record-2026-11-08), `54715b5f`). **No `:priority`**:
the user chose *"S1f.5.15 before swap"*, so S1f.5.15 removes the keyword from every program before the swap.
`RuleClause` is `:match Match | :assert Assert | :why STRING`, a rule's `:priority` is refused like any keyword a
rule does not take, and `DIGITS` is no token: a run of digits is a `NAME`. **A macro takes a `:why`**, once,
after the body: `'(' macro DeclName '(' VAR+ ')' Template (:why STRING)? ')'`. It is optional there as everywhere,
since the user ruled that `:why` is optional in the grammar and a loader flag, S1f.5.14's, makes it required. The
block above is left as accepted.

What stays the loader's: arity against a signature; which keywords a form requires and that each
appears once; a variable no parameter or premise binds; a relation parameter an activator binds to a
fact; `:over`'s three words; the macro table and imports; one `(solve …)`; a claim's relations and the
goal they must cover; *forbid*.

## Where it differs from the probe's draft, and why

The draft is [`c_grm/src/ein.l`](s1f.5.5_parser_probe/c_grm/src/ein.l) and
[`ein.y`](s1f.5.5_parser_probe/c_grm/src/ein.y), written to choose a generator. Each change is a
ruling the draft predates, or a position the draft left to one `Term`.

| | the draft | the final grammar | why |
|---|---|---|---|
| a name's characters | took `,` and `\` | neither | ruled 2026-11-07 ([the name rule](s1f.5.5_the_grammar.md#the-name-rule)) |
| `?_` | a variable | refused | ruled; the stage noted the draft did not do it |
| a string's escapes | any `\X` | `\n \t \r \" \\ \u{…}`; any other refused at the token | S1f.5.2's reading of *reasonable* |
| `#\|` after a name | part of a bad token | ends the name, as `;` does | a comment is a separator (the stage's *A token ends at a separator*) |
| keywords | `query` took `:no-hypothesis`; `step` took 3; `config` any keyword and any name | S1f.5.11's lists: `query` four, `step` five, `config` its two flags with `true` or `false` | ruled 2026-11-07 ([Q-M1e2.28](../../m1e2_correctness/open_questions.md#q-m1e228--which-keywords-does-each-form-take)) |
| a premise, a conclusion, a guard, a goal, a macro body | one `Term` | five productions; `absent` and `eq` take one and two, `open` is a whole conclusion, `(and)` refused | each position its production; the arity checks of `compile.rs` and `validate_open` move up |
| a macro call's arguments | any `Item` | a premise may stand there | `forall`'s guard and body are premises: this grammar's first version refused ten corpus files for it, three stdlib modules among them |
| a claim | a `Term` | its shapes, `(model …)`, `(or …)`, `(false)`, `(region …)` and its items | the shape checks of `expect.rs` and `claim.rs` move up |
| a declarator's name | any name | not a reserved kernel name | the loader's *shadows a reserved kernel name* moves up |
| a fact's `:using` | a list of names | a list of facts | today the loader drops the list's head and every name: [default 2](#defaults) |
| `(p (not (q a)))` | refused | a fact | today's parser and loader accept it; no ruling refuses it |
| the actions | a probe tree | today's `ein_ir::Ast`, `Loc` for `Loc` | 312 corpus files build the same tree as today's parser ([census](s1f.5.5_census.md#what-it-found)) |

## Defaults

The grammar carries these. Each stands unless the user says otherwise.

1. **Nine words become grammar tokens and stay names**: `eq`, `absent`, `false`, `true`, `open`, `model`,
   `region`, `merged`, `owing`, beside `relation` and `step`. Wherever a name goes each is a name, so
   `(p model)` and `(relation step T)` read as today. Where a position gives one a meaning, the other
   reading is gone: `(eq ?x)` is no pattern of a relation `eq`, and `(region …)` at the top level is no
   fact. No corpus program moves for it.
2. **A fact's `:using` lists its premises, each a fact**: `(r a b :rule x :using ((r c d)))`. Today the
   loader reads the arguments of the list that are facts and drops its head and every name, so
   `:using (x)` names no premise and `:using (p (r c d))` names one. No engine output writes a fact's
   `:using`.
3. **`(not …)` negates a fact.** `(not plain)` is refused; the negation of the nullary fact is
   `(not (plain))`.
4. **A pattern's argument may be a connective**, since a macro is called as a pattern and `forall`
   takes premises. Once macros have expanded, the loader refuses an `and`, `or` or `absent` left as a
   pattern's argument (a line T1f.5.5.5 adds), which today matches nothing in silence.
5. **Two sentences survive as notes on the parse error**: a query's lattice keyword says it belongs on
   `(solve …)`, and a retired `config` flag names the stage that retired it, as today's loader lines do.
6. **`:symbols` lists one name or more**; `:hypothesis-relations` and `:no-hypothesis` may be empty.
7. **`:assert ()` is the empty conclusion, and `(absent ())` is refused**: `()` is a whole `:match` or a
   whole `:assert`, nothing else. The census found `:assert ()` storing a fact named `@empty` today,
   a wrong answer for its owner ([census](s1f.5.5_census.md#findings-outside-the-grammar)).
8. **The trace takes `step` alone, with the five keywords ruled.** `ein_render::trace::trace_to_ir`, which
   writes `:bind` and `:why`, and `parse_trace_steps` have no caller and go at the swap.
9. **A nested `(or …)` in a `:match` stays the compiler's refusal**: it is an engine limit, not a rule
   of the language.
10. **`:over` takes a name, and the loader checks it** is `all`, `minimal` or `maximal`, which stay
    ordinary names.

## The programs

### `examples/zebra2.ein`, as written

The final grammar reads all 543 lines unchanged. Three of its forms, as the file writes them:

```lisp
(rule adjacent-via-fwd (?S ?R1 ?V1 ?R2 ?V2)
  :match  (and (?R1 ?V1 ?h1)
               (?S ?h2 ?h1)
               (absent (and (?S ?h_o ?h1) (neq ?h_o ?h2))))
  :assert (?R2 ?V2 ?h2)
  :why    "({?R1} {?V1} {?h1}) and unique-{?S} neighbour {?h2} ⟹ ({?R2} {?V2} {?h2})."
  :priority 200)

(solve
  :hrules (guess
            (color-loc  Color       House)
            (nation-loc Nationality House)
            (drink-loc  Drink       House)
            (smoke-loc  Cigarette   House)
            (pet-loc    Pet         House)))

(query
  :goal (and (drink-loc  Water      ?h_water) (nation-loc ?who_water ?h_water)
             (pet-loc    Zebra      ?h_zebra) (nation-loc ?who_zebra ?h_zebra))
  :goal-text "The {?who_water} drinks water in {?h_water}; the {?who_zebra} owns zebra in {?h_zebra}")
```

It builds the tree today's parser builds, node for node and `Loc` for `Loc`, and `ein solve` prints the
same bytes under both parsers: k_s 1, k_c 261 982, N 366 minimal dead sets, and the one model
(`EIN_PARSER=grm ein-grm solve examples/zebra2.ein`, against `ein solve`, 2026-11-08). So do `zebra.ein`
and every other puzzle.

### `std.elim`, as written

```lisp
(rule domain-elimination (?R ?isa ?OT ?VT)
  :match  (and (functional ?R 0 1)
               (total      ?R 0)
               (?isa ?a ?OT)
               (?isa ?v ?VT)
               (forall ?v_other
                 (and (?isa ?v_other ?VT) (neq ?v_other ?v))
                 (not (?R ?a ?v_other))))
  :assert (?R ?a ?v)
  :priority 400
  :why    "every {?VT} except {?v} excluded for {?a} → ({?R} {?a} {?v})")
```

The module's text does not change. Its `0` and `1` are names now, as `(functional R 0 1)` in a program
is, which is S1f.5.4's ruling, so the trees differ in those leaves alone. What it moves, computed:

| program | today | the final grammar |
|---|---|---|
| `tests/stdlib/elim/01_domain_elimination.ein` | k_s 1; `ein test`: 1 held | the same |
| `tests/stdlib/elim/02_no_room_left.ein` | k_s 0; `ein test`: 1 held | the same |
| `examples/features/05_stdlib_domain_elim.ein` | k_s 32, k_c 32, N 21, 26 candidates | k_s 32, k_c 32, **N 47, 52 candidates** |

`features/05`'s N is F8's measurement, ruled accepted (a label is a candidate like any other name), and
it moves at S1f.5.4, before the swap.

### `examples/syntax/`, every fixture

Twelve read unchanged, to today's tree and today's answers (`ein saturate --dump`, `ein solve`): `config`,
`constraint-scopes`, `equality`, `nullary`, `query-full`, `relation-decls` (whose `(relation R4 7 T)` is
a name of digits now, which prints `"7"` until S1f.5.4), `rule-forall-and-not`, `rule-guards`,
`rule-nary-and-or`, `rule-shared-guard-var`, `trace-empty` and `trace-steps`. Five are rewritten; each
file is in [`programs/`](s1f.5.5_parser_probe/programs/), and each answer is `saturate --dump`'s stored
facts, today's binary on today's file and the prototype on the rewritten one.

**`arg-kinds`.** Today `(r 7 "a string") (r 1..5 1..*) (r _ ?x) (r (nested a b) c)`; after:

```lisp
(r 7 07)
(r 1..5 1..*)
(r $x a-b_c.d)
(r (nested a b) c)
(r (not (nested a b)) c)
```

| today stores | the final grammar stores |
|---|---|
| `(r 7 "a string")` `(r "1..5" "1..*")` `(r "?x")` `(r (nested a b) c)` | `(r "7" "07")` `(r "1..5" "1..*")` `(r "$x" a-b_c.d)` `(r (nested a b) c)` `(r (not (nested a b)) c)` |

`(r _ ?x)` stored `(r "?x")`, one argument against a two-place signature. The quotes on `"7"`, `"07"`,
`"1..5"` and `"$x"` are today's printer's, which S1f.5.4 changes.

**`is-a-shapes`.** `(is-a a T) (is-a 7 T)`. Today's `(is-a ?x T)` and `(is-a _ T)` stored `(is-a "?x" T)`
and `(is-a T)`; both are refused.

**`negation`.** `(not (r a b)) (not (u a)) (not (plain)) (not (r a b) :source "(9)")`. Today `(not plain)`
stored `(not plain)`; after, `(not (plain))`.

**`unary-and-nary`.** `(quad a b c d :rule mk :using ((u a)))` where the fixture wrote `:using (x)`. The
stored facts are the same four, `(quad a b c d) (u a)` and the two declarations; the derived `(quad …)`'s
premise list is empty today and `[(u a)]` after, which `--dump` does not print.

**`wrappers`.** `(ontology (relation r T T) (is-a a T)) (facts (r a b)) (reasoning (r b c))`, the keyword
pairs gone from the nested facts (S1f.5.11's default). The stored facts are the same three, since the
loader dropped the pairs.

**`examples/broken/load/derivation_cycle.ein`**, rewritten for default 2: `(r a b :rule x :using ((r c
d)))` and its mirror. Both parsers refuse it with today's line, *derivation cycle: (r a b) -> (r c d) ->
(r a b)*.

### What is refused, and what the user sees

One file per kind, each refused in one run with every error recovery reaches, from the patched `lrpar`
(`EIN_PARSER=grm ein-grm saturate FILE`). Today's parser reports at most the first error, and the first
file it loads.

[`refused/fact-args.ein`](s1f.5.5_parser_probe/programs/refused/fact-args.ein), what the five fixtures
lose. Today it loads, exit 0. After, 9 errors:

```text
4:6:   (r 7 "a string")                     Delete "a string"  · Insert :source
5:4:   (r _ ?x)                             Delete _, Delete ?x
6:7:   (is-a ?x T)                          Delete ?x
7:7:   (is-a _ T)                           Delete _
8:6:   (not plain)                          Insert (, Shift plain, Insert )
9:32:  (quad a b c d :rule mk :using (x))   Insert (
10:1, 10:15, 10:29: (facts (r a b :source "(1)"))  three repairs around the nested keyword
```

[`refused/names.ein`](s1f.5.5_parser_probe/programs/refused/names.ein): `a?x`, `?x?y`, `a:b`, `a,b`,
`a#b`, `a|b`, `a\b`, `Жук`, `"a"b`, `?_`, `_`, one line each. Today the parser stops at `a:b`, line 5;
before it, `(p a?x)` reads as `(p a "?x")` and `?x?y` as two names. After, 11 errors, each *Delete* the
token.

[`refused/rules.ein`](s1f.5.5_parser_probe/programs/refused/rules.ein): `(and)`, `(absent)`, `(eq ?x)`,
`(p ?x :where …)`, `(and (open ?R) …)`, `:priority high`, a rule named `absent`, `(config
:enable-path-nogoods true)`, `(query … :no-hypothesis …)`. Today the parser stops at `(and)`. After, 10
errors; recovery after `(and)` reports a second at the next line, and the repairs for `(absent)`,
`(eq ?x)` and `(open ?R)` list every token that could start what is missing, 33, 14 and 14 of them, which
the printer's *three, then a count* (the stage's default) cuts.

The probe's [`four_mistakes.ein`](s1f.5.5_parser_probe/four_mistakes.ein), a valid program with four
mistakes: today *4:2: unexpected input*; after, 3 errors, at `a?x` (merged with the missing `)` before
it), at `:bogus` and at `_`, as the probe found with the draft.

## The questions for the user

### Q1 — G-14: with `:assert` checked by the grammar, is a bad conclusion that never fires refused?

Its unbound-variable part is ruled ([S1f.5.10](s1f.5.10_a_conclusion_variable_must_be_bound.md)).
[`g14/never-fires.ein`](s1f.5.5_parser_probe/programs/g14/never-fires.ein) has three rules whose
conclusion no fact can hold, `(q ?x :k v)`, `(q _)` and `?x`, over a relation no fact or candidate
populates, so no run fires them; [`g14/fires.ein`](s1f.5.5_parser_probe/programs/g14/fires.ein) is the
same with `(never a)`.

| | `never-fires.ein` | `fires.ein` |
|---|---|---|
| today, `ein solve` | exit 0: answers, nothing said | exit 1 at the first firing: *rule 'r': its :assert writes :k v as an argument of q, and no fact can hold it*, the other two unsaid |
| the final grammar | refused at the parse, 3 errors: `Delete :k`, `Delete _`, `Insert (, Shift ?x, Insert )` | the same 3 |

What no grammar sees is a relation parameter that an activator a rule *derives* binds to a fact:
[`g14/derived-activator.ein`](s1f.5.5_parser_probe/programs/g14/derived-activator.ein) derives `(r (a
b))` for `(rule r (?R) :match (box (?R b)) …)`, and both parsers stop the run where it is compiled, *the
relation head ?R of (?R b) … is bound to the fact (a b)*, exit 1. Without `(item (a b))`
([`derived-activator-never.ein`](s1f.5.5_parser_probe/programs/g14/derived-activator-never.ein)) no node
derives it, and both answer, 64 states.

- **(a) Refused at the parse, fired or not.** It is what the grammar does by itself. The one shape left
  stays a refusal where the activator is compiled, as today.
- (b) As (a), and the derived-activator shape refused at load by a static check that some rule could
  derive an activator binding a relation parameter to a fact. It also refuses
  `derived-activator-never.ein`, which never does and answers today.
- (c) Checked only when it fires, as today. The grammar would have to accept any shape in a conclusion,
  which is the permissiveness the stage removes.

**Recommended: (a).** Nothing it refuses can hold a fact, and a refusal that waits for a firing is one
a run may never reach.

### Q2 — the 55 files the grammar refuses that today's parser accepts

**Re-taken: 55, where the stage said 34.** The draft still refuses 34 at `ec004a48`; the final grammar
refuses those and 21 more, the loader refusals it takes over (closed keywords, declarators' names,
`open`, `absent` and `eq` arity, claims, imports) and `syntax/unary-and-nary` (default 2)
([`grammar_vs_today.txt`](s1f.5.5_parser_probe/census/grammar_vs_today.txt)). None is a puzzle, a
stdlib module or a `tests/` program.

| group | files | proposed |
|---|---:|---|
| refusal fixtures under `examples/broken/load/` (22) and `broken/compile/` (4) whose refusal the grammar takes over: `config_bad_value`, `config_retired_flag`, `config_unknown_flag`, `empty_absent`, `expect_is_a_pattern`, `expect_unknown_keyword`, `hrule_reserved_name`, `import_alias_not_a_name`, `import_symbols_empty`, `import_symbols_not_a_list`, `macro_reserved_name`, `open_arity`, `open_in_match`, `open_mixed_assert`, `query_lattice_keyword`, `region_top_level`, `relation_malformed`, `relation_needs_a_name`, `relation_reserved_name`, `reserved_open_direct`, `rule_reserved_name`, `solve_unknown_keyword`; `absent_arity_high`, `absent_arity_zero`, `eq_arity_high`, `eq_arity_low` | 26 | **stay refusal fixtures**, moved to `examples/broken/` beside the four parse-negatives, each `.expected` the new report, as S1e2.2.24 moved two compile-negatives to `broken/load/` when the loader took them. `reserved_open_direct` stays: three load-negatives, `reserved_open_aliased`, `-qualified` and `-symbols`, import it by its path, and their refusal, which is its, now reads as its parse error |
| `broken/load/derivation_cycle.ein` | 1 | **rewritten**, `:using ((r c d))`, so it still holds the cycle check (shown above) |
| `examples/syntax/`: `arg-kinds`, `is-a-shapes`, `negation`, `unary-and-nary`, `wrappers` | 5 | **rewritten** as shown, the shapes they lose held by one parse-negative fixture, `refused/fact-args.ein` |
| register witnesses: `W16` ×2, `W18` ×5, `W20` ×4, `W24` ×3, `W28` ×3 | 17 | **not this stage's**: S1f.5.1, S1f.5.2 and S1f.5.4 refuse each shape on the hand-written parser first and restate each row's `right`, as their files say |
| regression fixtures with no row: `ein-bugs/activator-text-int-only`, `-string-only`, `corpus/fuzz_findings/activator-text-key`, `-swapped`; `ein-bugs/kwpair-below-the-filter` and its fuzz find | 6 | **not this stage's**: S1f.5.4's step 1 says which of the int-and-string four retire, and S1f.5.11's default makes the keyword-pair pair a refusal fixture |

- **(a) As proposed.** The directory says where a program is refused, and every refusal keeps a fixture.
- (b) The 26 stay where they are, `.expected` moved only: fewer moves, and `broken/load/` then holds
  parse errors.
- (c) Rewrite the 26 so each still reaches its old loader line: impossible for most, whose line the
  grammar has replaced.

**Recommended: (a).**

## Re-taking it

```sh
# from the repository root
S=build/s1f.5.5; P=plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.5_parser_probe
mkdir -p $S/snapshot && git archive ec004a48 | tar -x -C $S/snapshot
L=build/pg-probe/lrpar-det                               # lrpar 0.15.0 with lrpar_deterministic.patch (the probe's README)
# today's ein
(cd $S/snapshot/ein.rs && CARGO_TARGET_DIR=$PWD/../../../pg-probe/target-snap cargo build --release -p ein-cli)
# the prototype front end, and the final grammar against today's parser
(cd $P && CARGO_TARGET_DIR=$PWD/../../../../build/pg-probe/target-ein \
   cargo build --release -p c_ein --config "patch.crates-io.lrpar.path=\"$PWD/../../../../$L\"")
# the snapshot's ein-ir with the final grammar behind EIN_PARSER=grm: in its Cargo.toml lrlex, lrpar and
# cfgrammar 0.15 as dependencies and build-dependencies; a build.rs that compiles $P/grammar/ein.l and
# ein.y as c_ein/build.rs does; in lib.rs `extern crate self as ein_ir;`, `#[path] pub mod b;` naming
# $P/grammar/build_ast.rs, and the two `lr*_mod!`; and in parse.rs, `parse` calling the generated parser
# when EIN_PARSER=grm. Then build it as above, with the patched lrpar, and copy the binary to $S/ein-grm.
EIN_STDLIB=$S/snapshot/stdlib EIN_PARSER=grm $S/ein-grm solve $S/snapshot/examples/zebra2.ein
```

Each answer above is that command, or `saturate FILE --dump`, `test FILE`, or `c_ein check FILE`, on the
file named.
