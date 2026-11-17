# The grammar

Ein parses with a lexer and a parser generated from two grammar files, `ein.l` and `ein.y`, and the two blocks on
this page are those files as the parser generator reads them. **They are what parses.** Each block is generated from
its file and never edited by hand, and the test suite fails when a block and its file differ. The prose around them
says what the blocks cannot show; where the two disagree, the block is right. What each form means, and what the
loader holds it to, is [`02-syntax.md`](02-syntax.md).

**Notation.** The blocks write the two files this way:

| | |
|---|---|
| §1, `NAME ::= regex` | a lexer rule: the token's name, and its regular expression in Rust's `regex` syntax (`[a-z]`, `[^"\\]`, `\(`, `{0,5}`, `?`, `\|`); a quoted name, `'rule'`, is a token that is its own text |
| §1, `/* in …; then …; "…" */` | the start conditions the rule is tried in (every rule is tried in `INITIAL` unless it says otherwise), the one it enters, and the name a parse error gives the token |
| §2, `A ::= B C \| D` | a production: concatenation, alternation, and `/* empty */` for the empty alternative; a list is a left-recursive rule, `Names ::= /* empty */ \| Names Name` |
| §2, `'x'`, `NAME` | a token, by the name of the lexer rule that makes it |

The lexer and the parser are separate layers. The lexer turns the whole text into tokens before the parser reads one,
and the parser is an LR(1) parser over those tokens, with no lookahead into the text and no token read by what is
being parsed: a word is the token the lexer makes of it wherever it is written.

## 1. Lexical grammar

The lexer tries every rule its current start condition allows at each position and takes the **longest** match;
between two matches of one length, the rule written first. Every terminal is ASCII: a character that is not ASCII is
written only inside a string (`TEXT`) or a comment, and anywhere else no rule matches it, so the lexer stops there and
the error names it (fact 6). Names are ASCII; a string is natural-language text in any Unicode (fact 4).

<!-- generated: ein.l -->
```text
/* ein.l: a token's regular expression, the start conditions it is
   tried in, the one it enters, and the name an error gives it.
   Generated from the grammar file; not edited by hand. */
%x AFTER_WORD
'('          ::= \(    /* in INITIAL, AFTER_WORD; then INITIAL */
')'          ::= \)    /* in INITIAL, AFTER_WORD; then INITIAL */
'not'        ::= not    /* then AFTER_WORD */
'and'        ::= and    /* then AFTER_WORD */
'or'         ::= or    /* then AFTER_WORD */
'neq'        ::= neq    /* then AFTER_WORD */
'rule'       ::= rule    /* then AFTER_WORD */
'hrule'      ::= hrule    /* then AFTER_WORD */
'query'      ::= query    /* then AFTER_WORD */
'solve'      ::= solve    /* then AFTER_WORD */
'config'     ::= config    /* then AFTER_WORD */
'trace'      ::= trace    /* then AFTER_WORD */
'macro'      ::= macro    /* then AFTER_WORD */
'import'     ::= import    /* then AFTER_WORD */
'relation'   ::= relation    /* then AFTER_WORD */
'step'       ::= step    /* then AFTER_WORD */
'eq'         ::= eq    /* then AFTER_WORD */
'absent'     ::= absent    /* then AFTER_WORD */
'false'      ::= false    /* then AFTER_WORD */
'true'       ::= true    /* then AFTER_WORD */
'open'       ::= open    /* then AFTER_WORD */
'model'      ::= model    /* then AFTER_WORD */
'region'     ::= region    /* then AFTER_WORD */
'merged'     ::= merged    /* then AFTER_WORD */
'owing'      ::= owing    /* then AFTER_WORD */
':why'       ::= :why    /* then AFTER_WORD */
':source'    ::= :source    /* then AFTER_WORD */
':goal-text' ::= :goal\-text    /* then AFTER_WORD */
':match'     ::= :match    /* then AFTER_WORD */
':assert'    ::= :assert    /* then AFTER_WORD */
':rule'      ::= :rule    /* then AFTER_WORD */
':using'     ::= :using    /* then AFTER_WORD */
':goal'      ::= :goal    /* then AFTER_WORD */
':over'      ::= :over    /* then AFTER_WORD */
':expect'    ::= :expect    /* then AFTER_WORD */
':hrules'    ::= :hrules    /* then AFTER_WORD */
':hypothesis-relations' ::= :hypothesis\-relations    /* then AFTER_WORD */
':no-hypothesis' ::= :no\-hypothesis    /* then AFTER_WORD */
':no-search' ::= :no\-search    /* then AFTER_WORD */
':lattice-sanity-check' ::= :lattice\-sanity\-check    /* then AFTER_WORD */
':record-alternative-justifications' ::= :record\-alternative\-justifications    /* then AFTER_WORD */
':symbols'   ::= :symbols    /* then AFTER_WORD */
':as'        ::= :as    /* then AFTER_WORD */
':derives'   ::= :derives    /* then AFTER_WORD */
':assumes'   ::= :assumes    /* then AFTER_WORD */
UNDERSCORE   ::= _    /* then AFTER_WORD; "_ (reserved)" */
VARUNDERSCORE ::= \?_    /* then AFTER_WORD; "?_ (reserved)" */
WRITEBACK    ::= [<](forced\-positive|monotonic\-unconditional|lookahead\-dies\-immediately)[>]    /* then AFTER_WORD; "the engine's writeback rule name (reserved)" */
NAME         ::= [A-Za-z0-9_.\-$%^*+=<>!&@~/'`\[\]{}]+    /* then AFTER_WORD; "a name" */
VAR          ::= \?[A-Za-z0-9_.\-$%^*+=<>!&@~/'`\[\]{}]+    /* then AFTER_WORD; "a variable" */
KEYWORD      ::= :[A-Za-z0-9_.\-$%^*+=<>!&@~/'`\[\]{}]+    /* then AFTER_WORD; "a keyword this form does not take" */
TEXT         ::= "([^"\\]|\\[ntr"\\]|\\u\{(0{0,5}[0-9A-Fa-f]|0{0,4}[1-9A-Fa-f][0-9A-Fa-f]|0{0,3}[1-9A-Fa-f][0-9A-Fa-f]{2}|0{0,2}([1-9A-Ca-cEeFf][0-9A-Fa-f]{3}|[Dd][0-7][0-9A-Fa-f]{2})|0?[1-9A-Fa-f][0-9A-Fa-f]{4}|10[0-9A-Fa-f]{4})\})*"    /* then AFTER_WORD; "a string" */
BADESCAPE    ::= "([^"\\]|\\[^\n])*"    /* then AFTER_WORD; "a string with an unknown escape" */
BADTEXT      ::= "([^"\\]|\\[^\n])*"[\x00-\x7F--[\s();\x1c-\x1f]]+    /* then AFTER_WORD; "a string glued to what follows" */
BAD          ::= ([\x00-\x7F--[\s();#\x1c-\x1f]]|#[\x00-\x7F--[|\s();\x1c-\x1f]])+    /* then AFTER_WORD; "a token that is not a name" */
(skipped)    ::= ([\x09-\x0D\x20\x1C-\x1F]|;[^\n]*|#\|([^|]|\|+[^|#])*\|+#)+    /* in INITIAL, AFTER_WORD; then INITIAL */
```
<!-- /generated: ein.l -->

Six facts about the terminals that the block states and does not explain:

1. **A reserved word is a token only as a whole word.** The thirteen reserved words, `not`, `and`, `or`, `neq`,
   `rule`, `hrule`, `query`, `solve`, `config`, `trace`, `macro`, `import` and `open`, and the ten position words,
   `relation`, `step`, `eq`, `absent`, `false`, `true`, `model`, `region`, `merged` and `owing`, are rules written
   before `NAME`, and `NAME` is longest match: `neq` is the token `'neq'`, a tie the earlier rule wins, while `neqw`,
   `neq_test`, `neq-1`, `rule-x`, `rule.x`, `not-a` and `open-slot` are each the one `NAME` they spell, wherever they
   are written ([Q-M1e2.17](../../../decisions/language/Q-M1e2.17-a-reserved-word-is-a-keyword-only-as-a-whole.md)).
   A reserved word is never a name. A position word is a token of its own and a name too, wherever §2's `Name` goes.
   - **`open` is written only as a rule's verdict**: §2's `Assert` is the one production that takes it, a rule's
     whole `:assert`, `(open)` or `(open ?R)`
     ([Q-M1f.66](../../../decisions/language/Q-M1f.66-open-is-written-only-as-a-rules-verdict.md)). An error found
     at any other `open` carries the note *open is reserved: it is the verdict a rule's :assert states, written as the
     whole :assert, (open) or (open ?R), and nowhere else, so it names no relation and no value*.
   - **`eq`, `absent` and `false` head no value.** §2's `ValueHead` lists the words that may head a fact inside a
     fact, a pattern inside a pattern and a conclusion's argument, and those three are not among them: `absent` heads
     only a premise's connective and a claim's class atom, `false` is written only as `(false)`, and `(eq a b)` only as
     a fact, a premise or a conclusion of its own. An error found at an `absent` that opens a form, at an `eq` or `neq`
     that opens a form inside another, and at an argument written after `(false`, carries a note saying so
     ([`07-diagnostics.md`](07-diagnostics.md), *a value is a name, or a fact a state could hold*).
   - **A keyword is `:` and a run of name characters**, read the same way: `:why` is the token `':why'`, and `:where`
     a `KEYWORD`, which no production of §2 takes, so a keyword a form does not take is a parse error that says so
     ([Q-M1e2.28](../../../decisions/language/Q-M1e2.28-each-form-takes-a-closed-list-of-keywords.md)).
2. **A bare `_` and `?_` name nothing**
   ([Q-M1e2.22](../../../decisions/language/Q-M1e2.22-a-bare-is-reserved-and-refused.md)). `UNDERSCORE` and
   `VARUNDERSCORE` are tokens no production of §2 takes: they exist so that the parse error can say why, wherever
   either is written, with the note *_ is reserved and names nothing: where any value will do, write a variable used
   once*. Anywhere else in a word `_` is a name character: `_a_in_b_`, `_______01`, `A________B` and `__closed__` are
   names, and `?_x` is a variable.
3. **There is no number and no range**
   ([Q-M1e2.24](../../../decisions/language/Q-M1e2.24-ein-has-no-numbers-and-no-ranges.md)): `7`, `07`, `-7` and
   `1..5` are names (fact 5), each printed as written, and a fact of one is never a fact of another.
4. **A string is natural-language text, with five escapes and `\u{…}`**
   ([Q-M1e2.23](../../../decisions/language/Q-M1e2.23-a-string-is-natural-language-text-never-a-value.md)). `TEXT`
   is the value of `:why`, `:source` or `:goal-text` and nothing else (§2's `Text`): written anywhere a value goes it
   is a parse error at the string, with a note saying so, and the name written bare is what it meant. It holds any
   character but `"` and `\` as itself, a raw newline among them, and reads `\n` `\t` `\r` `\"` `\\` and `\u{…}`, one
   to six hex digits naming a Unicode scalar value, 0 to 10FFFF and not D800 to DFFF (`\u{1F993}` is 🦓). Any other
   escape, `\x41`, `Ж`, `\u{}`, and a `\u{…}` that names no character, `\u{110000}` or a surrogate, make the
   string a `BADESCAPE`, refused at its opening quote with a note per escape naming it and its line and column. A
   backslash before a newline is no escape: the string is a `BAD` token there.
5. **A token ends at a separator.** After any token but `(` and `)` the lexer is in `AFTER_WORD`, where it tries `(`,
   `)` and the skip rule alone: a space, a paren, a comment or the end of input ends a token, and any other ASCII
   character continues it. So `a?x`, `?x?y` and `a,b` are each one `BAD` token, *a token that is not a name*, and
   `"a"b` one `BADTEXT`, *a string glued to what follows*, each a parse error at the token with the file, line and
   column; none splits in silence. **The name rule**: a `NAME` is a run of the ASCII name characters `ein.l` lists,
   letters, digits and `` _ . - $ % ^ * + = < > ! & @ ~ / ' ` [ ] { } ``, and a `VAR` is `?` and such a run, so `7`,
   `07`, `-7`, `1..5`, `=`, `<=`, `a/b` and `f'` are names and `?a.b` is a variable.
6. **A character that is not ASCII is written only inside a string or a comment**
   ([Q-M1f.65](../../../decisions/language/Q-M1f.65-a-comment-may-hold-any-character.md)). `TEXT` and `BADESCAPE`
   take any character between their quotes, and the skip rule's comments any to their end; `BAD`, `BADTEXT`'s tail
   and the skip rule's whitespace are ASCII, `\x00-\x7F` less the separators, which are `\x09-\x0D`, the space and
   `\x1C-\x1F`. So no rule matches a character that is not ASCII anywhere else, in either start condition: the lexer
   stops at the file's first one, and the report names it, its code point and where Unicode is allowed
   ([Q-M1f.95](../../../decisions/diagnostics/Q-M1f.95-a-file-is-refused-at-its-first-character-that-is.md)),
   quoting a character a reader cannot see as the escape a string writes it with
   ([Q-M1f.92](../../../decisions/diagnostics/Q-M1f.92-a-character-a-reader-cannot-see-is-quoted-as.md)):

   ```text
   x.ein:2:7: 'é' (U+00E9) is not ASCII: Unicode is allowed only inside a string literal (after :why, :source or :goal-text) or in a comment
   x.ein:2:3: '\u{a0}' (U+00A0) is not ASCII: Unicode is allowed only inside a string literal (after :why, :source or :goal-text) or in a comment
   ```

   A `#` before such a character is passed over, the character being what the line is about.

## 2. Phrase grammar

`Program` is the start symbol. The skip rule's whitespace and comments may appear between any two tokens, and no
production names them.

<!-- generated: ein.y -->
```text
/* ein.y: every rule and its alternatives. Generated from the grammar
   file; not edited by hand. The start rule is Program. */
Program ::= /* empty */ | Program Form
Form ::= Relation | Rule | Query | Solve | Config | Trace | Macro | Import | NotFact | Fact
Name ::= NAME | 'relation' | 'step' | 'eq' | 'absent' | 'false' | 'true' | 'model' | 'region'
       | 'merged' | 'owing'
DeclName ::= NAME | 'step' | 'true' | 'model' | 'region' | 'merged' | 'owing'
FactHead ::= NAME | 'step' | 'eq' | 'true' | 'model'
ValueHead ::= NAME | 'relation' | 'step' | 'true' | 'model' | 'region' | 'merged' | 'owing'
PremiseHead ::= ValueHead | Var
ConclusionHead ::= ValueHead | Var
Var ::= VAR
Text ::= TEXT
Bool ::= 'true' | 'false'
Names ::= /* empty */ | Names Name
NamesPlus ::= Name | NamesPlus Name
Vars ::= /* empty */ | Vars Var
VarsPlus ::= Var | VarsPlus Var
NameList ::= '(' ')' | '(' Name Names ')'
NameListPlus ::= '(' Name Names ')'
Fact ::= '(' FactHead FactArgs FactClauses ')' | '(' 'false' FactClauses ')'
NotFact ::= '(' 'not' NestedFact FactClauses ')' | '(' 'not' EqFact FactClauses ')'
NestedFact ::= '(' ValueHead FactArgs ')' | '(' 'false' ')'
EqFact ::= '(' 'eq' FactArgs ')'
FactArg ::= Name | NestedFact | '(' 'not' NestedFact ')'
FactArgs ::= /* empty */ | FactArgs FactArg
FactClause ::= ':source' Text | ':rule' Name | ':using' '(' NestedFacts ')'
FactClauses ::= /* empty */ | FactClauses FactClause
NestedFacts ::= NestedFact | NestedFacts NestedFact
Relation ::= '(' 'relation' DeclName Names RelationClauses ')'
RelationClauses ::= /* empty */ | RelationClauses ':why' Text
Rule ::= '(' 'rule' DeclName Params RuleClauses ')'
       | '(' 'hrule' DeclName Params RuleClauses ')'
Params ::= '(' Vars ')'
RuleClause ::= ':match' Match | ':assert' Assert | ':why' Text
RuleClauses ::= /* empty */ | RuleClauses RuleClause
Match ::= '(' ')' | Premise
Premise ::= '(' PremiseHead Args ')' | '(' 'false' ')' | Connective
Premises ::= Premise | Premises Premise
Connective ::= '(' 'and' Premises ')' | '(' 'or' Premises ')' | '(' 'not' Negated ')'
             | '(' 'absent' Premise ')' | Guard
Negated ::= Pattern | '(' 'eq' Term Term ')'
Guard ::= '(' 'eq' Term Term ')' | '(' 'neq' Term Term ')'
Args ::= /* empty */ | Args Arg
Arg ::= Name | Var | '(' ArgHead Args ')' | '(' 'false' ')' | Connective
ArgHead ::= ValueHead | Var
Pattern ::= '(' ValueHead Terms ')' | '(' Var Terms ')' | '(' 'false' ')'
AnyHead ::= Name | Var
Term ::= Name | Var | Pattern | '(' 'not' Pattern ')'
Terms ::= /* empty */ | Terms Term
Assert ::= '(' ')' | Conclusion | '(' 'open' ')' | '(' 'open' AnyHead ')'
Conclusion ::= '(' ConclusionHead Terms ')' | '(' 'and' Conclusions ')' | '(' 'not' Negated ')'
             | '(' 'not' Var ')' | '(' 'false' ')' | Guard
Conclusions ::= Conclusion | Conclusions Conclusion
Query ::= '(' 'query' QueryClausesPlus ')'
QueryClause ::= ':goal' Premise | ':goal-text' Text | ':over' Name | ':expect' Claim
QueryClausesPlus ::= QueryClause | QueryClausesPlus QueryClause
Claim ::= Model | '(' 'or' Models ')' | '(' 'false' ')'
Model ::= '(' 'model' ClaimFacts ')'
Models ::= Model | Models Model
ClaimFact ::= NestedFact | EqFact | '(' 'not' NestedFact ')' | '(' 'not' EqFact ')'
            | '(' 'absent' ClaimAtom ')' | '(' 'neq' FactArg FactArg ')'
ClaimAtom ::= NestedFact | EqFact | '(' 'not' NestedFact ')' | '(' 'not' EqFact ')'
ClaimFacts ::= /* empty */ | ClaimFacts ClaimFact
Solve ::= '(' 'solve' SolveClauses ')'
SolveClause ::= ':hrules' Hrules | ':hypothesis-relations' NameList | ':no-hypothesis' NameList
              | ':no-search' | ':expect' RegionClaim
SolveClauses ::= /* empty */ | SolveClauses SolveClause
Hrules ::= '(' ')' | '(' Name HruleItems ')'
HruleItem ::= Name | NameListPlus
HruleItems ::= /* empty */ | HruleItems HruleItem
RegionClaim ::= '(' 'region' RegionItems ')' | '(' 'false' ')'
RegionItem ::= Model | '(' 'merged' Model MergedFrom ')' | '(' 'owing' Models ')'
RegionItems ::= /* empty */ | RegionItems RegionItem
MergedFrom ::= Model | '(' 'or' Models ')'
Config ::= '(' 'config' ConfigClauses ')'
ConfigClause ::= ':lattice-sanity-check' Bool | ':record-alternative-justifications' Bool
ConfigClauses ::= /* empty */ | ConfigClauses ConfigClause
Macro ::= '(' 'macro' DeclName MacroParams Template ')'
        | '(' 'macro' DeclName MacroParams Template ':why' Text ')'
MacroParams ::= '(' VarsPlus ')'
Template ::= Var | '(' 'and' Templates ')' | '(' 'or' Templates ')' | '(' 'not' Template ')'
           | '(' 'neq' TemplateItem TemplateItem ')' | '(' AnyHead TemplateItems ')'
Templates ::= Template | Templates Template
TemplateItem ::= Name | Template
TemplateItems ::= /* empty */ | TemplateItems TemplateItem
Import ::= '(' 'import' Name ImportClauses ')'
ImportClause ::= ':symbols' NameListPlus | ':as' DeclName
ImportClauses ::= /* empty */ | ImportClauses ImportClause
Trace ::= '(' 'trace' Steps ')'
Steps ::= /* empty */ | Steps Step
Step ::= '(' 'step' Name StepClauses ')'
StepClause ::= ':rule' Name | ':using' StepUsing | ':derives' Derived | ':source' Text
             | ':assumes' Derived
StepClauses ::= /* empty */ | StepClauses StepClause
StepUsing ::= NameListPlus | '(' 'and' NamesPlus ')'
Derived ::= NestedFact | '(' 'not' NestedFact ')'
```
<!-- /generated: ein.y -->

Four things the productions say that are easy to read past:

- **A `Conclusion` is a conjunction of literals, read flat.** `Conclusions` nests, and an `(and …)` inside an
  `(and …)` concludes each of its members, as the outer one does, at any depth. No production takes `or` in a
  conclusion, or a connective inside a `Term`, so a rule concludes no disjunction and no compound fact
  ([`02-syntax.md` § Rules](02-syntax.md#rules)).
- **A stored negative, `NotFact`, takes a fact's clauses**, so a derived `(not …)` fact's provenance, `:rule
  type-exclusivity :using ((c10 a))`, attaches as a fact's does. A `(not …)` in a `:match` or an `:assert` is a
  `Connective` or a `Conclusion` and takes no clause: a keyword pair there is a parse error.
- **Each form's keywords are its own productions**, `RelationClauses`, `RuleClause`, `QueryClause`, `SolveClause`,
  `ConfigClause`, `ImportClause`, `StepClause` and `FactClause`, and each keyword's value is the production after it:
  a claim is a `Claim` or a `RegionClaim`, a flag a `Bool`, a `:hrules` value `Hrules`, and `:no-search` takes none
  ([`02-syntax.md` § The keywords each form takes](02-syntax.md#the-keywords-each-form-takes)).
- **A `(trace …)` holds `step` events alone**: `(trace (branch-open …))` is a parse error at its head, a name where
  a trace wants `step`.

## 3. What the grammar leaves to the loader

Everything below parses and is refused later, at load. A parse error and a load error are reported differently, and
both are specified in [`07-diagnostics.md`](07-diagnostics.md): a parse error is one line per error recovery reaches,
`file:line:col: unexpected X`, the token's name after it where it has one, then the error's notes, at most three
ranked repairs and a count of the rest; a loader error about a top-level form ends `at None`.

- **That a form's keywords are written, and once.** The grammar states which keywords each form takes and refuses any
  other, but takes them in any order and number: a rule with no `:match`, or with two, parses, and the loader refuses
  it, `(rule r) missing :match or :assert at None`. A query takes at least one clause of its four, any one
  (`QueryClausesPlus`).
- **What a claim names.** An expectation's facts have a fact's shape at the parse (`ClaimFact`); that the relations it
  names exist, and that it names the goal's, is the loader's.
- **How many arguments a form of a declared relation has.** The loader holds every form of a declared relation to the
  count of types its declaration lists ([`02-syntax.md` § Relation declarator](02-syntax.md#relation-declarator)). The types
  themselves are not checked.
- **Unbound variables**: a conclusion's variable that no way of satisfying the `:match` binds
  ([`02-syntax.md` § Conclusion forms in `:assert`](02-syntax.md#conclusion-forms-in-assert)).
- **What a macro's body expands to.** A `Template` takes any head at any arity, and a call takes any premise as an
  argument (`Arg`'s `Connective`), so a premise that is a name, `eq` or `absent` at another arity, a `not` of no fact,
  a connective where a value goes, `false` with an argument, an `(or …)` as a conclusion and a conclusion that is a
  bare variable reach the loader through a macro. The loader refuses each after expansion, holding the expanded rule
  to this grammar's shapes by position. A macro no rule calls is not read, so it loads whatever its body
  ([Q-M1f.67](../../../decisions/language/Q-M1f.67-a-macros-body-is-checked-where-it-is-called.md)).
- **A connective as a pattern's argument.** `Arg` takes a `Connective`, since a macro such as `forall` is called as a
  pattern and takes premises, and the grammar cannot tell a macro's call from a pattern. Once the macros are expanded,
  a connective that is still a pattern's argument, `(q (and (p ?x)))`, `(q (eq ?a ?b))` or `(q (absent (p ?x)))`, is
  a pattern no fact matches, and the loader refuses it, in a rule, an hrule, an obligation and a goal; `(not
  <pattern>)`, a stored negation, stays. The same reading refuses what a macro's expansion writes of the shapes §2
  refuses written out: a fact, value or conclusion of `absent`, `false` with an argument, and `(eq …)` as a value.
- **Where an `(or …)` sits, how wide a clause is, and a goal's relations.** `Connective` takes an `(or …)` wherever a
  premise goes, so an `(or …)` written as a premise of an `(and …)`, under an `(absent …)` or as a whole `:match`
  parses; so do a `:match` or a `:goal` of any width, and a goal's premise whose relation is a variable
  (`PremiseHead`'s `Var`). The loader refuses each shape no plan can be compiled from, a macro's expansion included.
- **The kernel's names.** A dunder, `__closed__` or `__symmetric__`, is a `NAME` the loader refuses where a program
  binds one ([Q-M1f.7](../../../decisions/language/Q-M1f.7-the-dunders-are-never-bound-and-the-writeback.md)).
  Where `open` stands is the grammar's; what the loader still refuses of it is the program's to tell, an `(open ?R)`
  whose `?R` is no parameter or whose projection does not resolve, each line placed where the `(open …)` is written
  ([`03-reserved-names.md` § The verdict atom](03-reserved-names.md#the-verdict-atom--open)).

Every program under [`examples/broken/`](../../../../examples/broken/) is refused, and its report, the engine's
defined output for it, is kept beside it as its `.expected` file: a parse error at the top, and one refused at load in
`load/` and `compile/`.
