# Diagnostics — what Ein refuses, and what it prints when it does

Every line a program can meet when it is wrong: where the line is placed, what it says, and the exit code it
takes. § 1 is the parse, which refuses text that is not Ein; § 2 is the rest, a file that cannot be read and each
refusal the loader makes of a program the grammar takes. The commands, their options and the full exit-code table
are [`05-cli.md`](05-cli.md); the grammar itself is [`01-grammar.md`](01-grammar.md) and [`02-syntax.md`](02-syntax.md).

**These lines are normative.** Each is the behaviour, not a description of it: a change to any of them is a change
to Ein, and the test suite holds every one of them to the binary. A few are strange in isolation; where a better
behaviour is known, its section names it.

**How a line is written here.** `<file>`, `<line>` and `<col>` are where the fault is written, 1-based, lines split
at `\n` and columns counted in characters. `<r>` is a rule's name, `<k>` a keyword or the word `rule` or `hrule` as
the line has it, `<n>` a count; a `…` in a quoted line stands for the part this page does not repeat. Every example
is the binary's own output.

**The lines' own words.** A line is quoted as `ein` prints it, and three of its words are not the words of these
pages ([the glossary](13-glossary.md#the-language)). A line's **value** is an object in an argument's place: an
atom, a fact, or, in a pattern, a variable. Its **atom**, in *no atom of the query's :goal*, is a pattern of the
goal. Its **type** and **types** are a declaration's label and labels. Ein has no types, and an object, on these
pages and in the lines, is any node of the knowledge-base graph.

## 1. Parse and load diagnostics

**A refused file reports every error recovery reaches**, in file order, one line each:

```text
<file>:<line>:<col>: unexpected X
```

`X` is the token as written and, where the token has a name, the name after it: `a name`, `a variable`, `a keyword
this form does not take`, `a token that is not a name`, `a string`. A character no token starts with is
`no token starts with X`, and a character that is not ASCII outside a string literal and a comment is a line of its
own, which names it and says where Unicode is allowed ([§ 1.6](#16-a-character-that-is-not-ascii-is-written-only-in-a-string-or-a-comment)).

Under each error come its **notes**, where a ruling says more than the token does: a bare `_` or `?_`
([§ 2.8](#28-_-and-_-are-refused-wherever-they-are-written)), a string as an argument
([§ 2.12](#212-a-string-is-natural-language-text-and-an-escape-names-a-character)), a lattice keyword on a query
([§ 2.1](#21-a-querys-a-solves-and-a-claims-refusals)), a retired `(config …)` flag ([`06-config.md`](06-config.md)), an
`open` that is not a rule's verdict ([§ 2.17](#217-open-is-written-only-as-a-rules-verdict)), and `absent`, `false`
or `eq` where an argument goes ([§ 2.22](#222-an-object-is-an-atom-or-a-fact-a-state-could-hold)). Then come at most three
ranked **repairs**, `repair: Insert `)`` or `repair: Delete `a?x``, and `and n more repairs`. Recovery has a budget
of 30 000 steps; a report it cuts ends with a line at its last error, *recovery stopped here, at its budget of 30000
steps: errors after this one are not reported*.

Every command exits 1 on a parse error (`ein test`: 2, the file an `ERROR` row), and no repaired program is loaded:
a repair is advice, never applied
([Q-M1a.3](../../../decisions/diagnostics/Q-M1a.3-a-parse-error-names-its-position-and-its-token.md)).

```text
$ ein solve t.ein               # file is: (p a?x) (q "1")
t.ein:1:4: unexpected `a?x`: a token that is not a name
  repair: Delete `a?x`
t.ein:1:12: unexpected `"1"`: a string
  note: "1" is a string, and a string is natural-language text, written only after :why, :source or :goal-text: write a name bare
  repair: Delete `"1"`
  repair: Insert `:source`
```

### 1.1 The end of input is where the text ends

An input that ends while a form is open reports the position past its last character: the line after a final
newline, column 1, wherever a newline, a space or a comment follows the last token.

```text
$ ein solve t.ein               # file is: (relation r T and a newline
t.ein:2:1: unexpected end of input
  repair: Insert `)`
```

### 1.2 There is no context block

The report names the position and the token, and prints no source line, however long the line is. `{` is a name
character, so a `{` where a form's `)` goes is a name where none is taken:

```text
$ ein solve long.ein            # file is: (relation aaa…sixty a's… T){
long.ein:1:74: unexpected `{`: a name
  repair: Delete `{`
```

### 1.3 The reported column is the token's

An error is placed at the start of the token the parse could not take, or of the character no token starts with. A
token ends only at a space, a paren, a comment or the end of input ([`01-grammar.md`](01-grammar.md), the lexical
grammar), so `(y";"{` and `(y";"{?` both report `1:2`, at `y"`, one token that is not a name; a `;` inside what reads
as a string there is no comment.

### 1.4 No ambiguity, and a tie goes to the earlier rule

The grammar has no conflict, so no input has two parses. The lexer's one choice is between two rules that match the
same length, where the rule written first wins: each reserved word and keyword is written before the rules for a
name and a keyword, so `neq` is the reserved word and `neqw`, the longer match, a name
([`01-grammar.md`](01-grammar.md), the lexical grammar). A reserved word is one only as a whole word
([Q-M1e2.17](../../../decisions/language/Q-M1e2.17-a-reserved-word-is-a-keyword-only-as-a-whole.md)): `rulex` and
`rule-x` are names, `-` being a name character. `(relation R A B)` is a declaration, and `(relation)` a parse error,
a declaration needing a name:

```text
$ ein solve t.ein               # file is: (relation)
t.ein:1:10: unexpected `)`
  repair: Insert a name
```

### 1.5 Loader messages about a top-level form end in `at None`

A loader line about a top-level form that carries no position of its own ends in ` at None`:

```text
kb load error: duplicate rule/hrule name 'g' at None
```

So do a duplicate relation, rule, hrule or macro, a rule missing its `:match` or `:assert`, an import cycle, a bare
`(import std)`, an import that writes both `:as` and `:symbols`, and a `:symbols` name the module does not provide
(the fixtures under [`examples/broken/load/`](../../../../examples/broken/load/) whose `.expected` holds `at None`).
A line raised where a position is known carries it: a macro called with the wrong count ends
`at <file>:<line>:<col>`, and every refusal of § 2 whose table shows a place starts with `<file>:<line>:<col>: `. A
line about a rule's shape that names the rule and no place, as most of § 2.9's and § 2.15's do, ends in neither.

This is a usability defect, kept so that fixing it is one visible re-baseline of every message it touches rather
than noise in each, and it is the clearest candidate on this page for being **fixed** rather than documented
([Q-M1a.6](../../../decisions/diagnostics/Q-M1a.6-a-loader-message-about-a-top-level-form-names-no.md)).

### 1.6 A character that is not ASCII is written only in a string or a comment

A character that is not ASCII is written only inside a string literal, after `:why`, `:source` or `:goal-text`, or
in a `;` or `#| … |#` comment
([Q-M1f.65](../../../decisions/language/Q-M1f.65-a-comment-may-hold-any-character.md)). Anywhere else no lexer rule
matches it, the lexer stops there, and the file is refused with one line at the character, on every command (exit 1,
`ein test` 2, `ein kb save` writing no container):

```text
$ ein solve t.ein               # file is: (p café)
t.ein:1:7: 'é' (U+00E9) is not ASCII: Unicode is allowed only inside a string literal (after :why, :source or :goal-text) or in a comment
```

- **The character, quoted, and its code point**, `'é' (U+00E9)`, `'🦓' (U+1F993)`; one a reader cannot see where it
  stands — a space, a control, a zero-width or bidirectional format character, a combining mark or a variation
  selector — is quoted as the escape a string writes it with, `'\u{a0}' (U+00A0)`, `'\u{202e}' (U+202E)`
  ([Q-M1f.92](../../../decisions/diagnostics/Q-M1f.92-a-character-a-reader-cannot-see-is-quoted-as.md)).
- **One line, at the file's first such character**: the file is lexed before it is parsed and the lexer stops at
  its first error, so no parse error and no repair follows
  ([Q-M1f.95](../../../decisions/diagnostics/Q-M1f.95-a-file-is-refused-at-its-first-character-that-is.md)). A `#`
  before one is passed over: no rule starts at that `#`, and the line is about the character.
- **Every place**: a name, a variable, a keyword, a declared relation's name or a slot's label, a nested fact, a rule, an
  hrule, a macro's name or body, a query's goal, `:over` or claim, a `(solve …)` list, a `(config …)` flag, a trace
  step, an import's module, between two tokens, a line's or the file's first character — a byte-order mark among
  them, `'\u{feff}' (U+FEFF)`
  ([Q-M1f.96](../../../decisions/language/Q-M1f.96-a-byte-order-mark-is-refused-as-any-character.md)) — in place of a
  paren, glued after a string, and in an imported module, whose line names the module's resolved path.
- **Not refused**: any character inside a string literal, whatever the string is written as (a string where an
  argument goes is refused as one, [§ 2.12](#212-a-string-is-natural-language-text-and-an-escape-names-a-character), and an
  escape it does not read as one, at its quote), and any in a comment. A byte that is not UTF-8 is refused before
  the text is read ([§ 2](#2-errors-and-exit-codes)).

### 1.7 A text nested past 1 000 parentheses is refused before it is parsed

A top-level form is at depth 1. One pass over the tokens counts depth before the parse, and a text whose `(` passes
1 000 is refused with one line at that `(`, and no other:

```text
<file>:<line>:<col>: this `(` nests deeper than the limit of 1000 parentheses
```

Every command exits 1 (`ein test`: 2). A rule may derive a fact deeper than the limit, and a state that holds one is
answered and saved; its saved file loads again, since what the parser reads of it is the program
([Q-M1f.185](../../../decisions/engine/Q-M1f.185-a-state-deeper-than-the-parsers-limit.md)).

## 2. Errors and exit codes

A failure the engine cannot place in a form prints one line and exits non-zero:

| input | the line | exit |
|---|---|---|
| a missing input file | `FileNotFoundError: [Errno 2] No such file or directory: '<path>'` | 1 |
| an input file that is neither UTF-8 text nor a `.einb` container | `<path>:<line>:<col>: byte 0x<hh> is not UTF-8: a program is UTF-8 text, or a .einb container`, at its first byte that is not | 1 (`ein test`: 2, the file an `ERROR` row) |
| an imported module that exists and cannot be read as text | `kb load error: ` and the line its path gives as a program ([§ 2.18](#218-an-imported-module-is-read-as-the-program-is)) | 1 (`ein test`: 2) |
| an argument the command line rejects | `error: invalid value 'x' for '--solutions <N>': invalid int value: 'x'` | 2 |

**Every command reads a program by one path**, so a file that cannot be read, or is not UTF-8, is refused with the
same line by `solve`, `saturate`, `oracle`, `test`, `show`, each `render` view and `kb save`, at each command's
load-error exit. `ein test` counts such a file an `ERROR` and goes on to the next.

**The last row is the command line's own wording.** What is fixed about it is the **exit code and the stream**,
which is what a script can depend on: a usage error exits 2, on stderr, before anything runs ([`05-cli.md`](05-cli.md)).

**A diagnostic names no exception class**, but two: `FileNotFoundError` above, and the lines of an imported module
that is a directory or cannot be opened (§ 2.18), which keep a Python spelling, `IsADirectoryError` and
`PermissionError`. Those are the lines a better behaviour is known for. Everything else is one line in Ein's own
words.

**A refusal of the loader** is `kb load error: ` and its lines, `; `-joined in the order the loader's passes find
them. Each line is one finding, then ` — `, then what to write instead
([Q-M1f.38](../../../decisions/diagnostics/Q-M1f.38-a-refusals-line-names-the-fault-then-what.md)). Each refusal
of a program below is made at load, before any rule fires, any hypothesis is proposed or any obligation is owed, on
every command that reads the program — `ein solve` in every mode, `oracle`, `saturate`, `test`, `show`, each
`render` view and `kb save`, which writes no container — so no command stops part way through a run on a rule's
shape, and no two commands disagree about whether a program loads. The one exception is
[§ 2.7](#27-a-refutation-resting-on-an-absent-a-commitment-can-fill)'s, which reads root's fixpoint and is made by
the commands that take the universe. Each exits at the command's load-error exit: **1, and 2 under `ein test`**. A
view refuses what the loader refuses
([Q-M1f.71](../../../decisions/cli/Q-M1f.71-a-view-refuses-a-program-the-loader-refuses.md)).

**A program's answer is never an error.** A refuted claim exits 1, every answer exits 0, and `ein test`'s codes are
its own: [`05-cli.md`](05-cli.md) has the table, and [`08-read-out.md`](08-read-out.md) the lines a run prints.

### 2.1 A query's, a solve's and a claim's refusals

A `(query …)`, a `(solve …)` and the claims they state are read as the rest of a program is, and a malformed one is
refused at the parse or at load, in Ein's words. Each row marked † is a parse error at the token, with its line,
column and repairs ([§ 1](#1-parse-and-load-diagnostics)), because each form takes a fixed set of keywords
([Q-M1e2.28](../../../decisions/language/Q-M1e2.28-each-form-takes-a-closed-list-of-keywords.md)) and a claim's
and a region's shapes are the grammar's.

| input | the line |
|---|---|
| a `(query …)` keyword it does not take | `<file>:<l>:<c>: unexpected `:<k>`: a keyword this form does not take` † |
| a lattice keyword on a `(query …)`, `:hrules`, `:hypothesis-relations`, `:no-hypothesis` or `:no-search` | a parse error at the keyword, its note `:<k> chooses the lattice and belongs on (solve …)` …, ending `— one program, one lattice` † |
| a `(solve …)` keyword it does not take | a parse error at the keyword † |
| more than one `(solve …)` | `kb load error: <n> (solve …) forms — a program states its lattice once` |
| a top-level `(region …)`, `(merged …)` or `(owing …)` | a parse error at the word † |
| a `(solve … :expect …)` that is not `(false)` or `(region …)`, or a region item that is not `(model …)`, `(merged …)` or `(owing …)` of their shapes | a parse error at the first token that is not † |
| `(owing …)` beside states of R | ``kb load error: (region …) — `(owing …)` says R is empty, and the region lists states`` |
| a `(merged …)` whose state is not a listed one | `kb load error: (merged …) — its state is not one the region lists` |
| a region claim naming a relation the program does not have ([§ 2.21](#221-a-goal-is-held-to-what-a-rules-match-is-held-to)) | `kb load error: (solve … :expect …) names <r>, and the program names no relation <r> — a relation of the program is one it declares, states a fact of, or writes in a rule or an hrule` |
| a region claim naming `eq` or `neq` | `kb load error: (solve … :expect …) names <r>, which a state never stores — eq and neq are computed from their definition, so a region claim, which lists a state's facts, holds neither` |
| a query's `:expect` that is not `(false)`, `(model …)` or `(or (model …) …)` | a parse error at the first token that is not † |
| a query's `:expect` naming a relation the program does not have (`eq` and `neq` excepted, which a goal may ask about) | `kb load error: :expect names <r>, and the program names no relation <r> — a relation of the program is one it declares, states a fact of, or writes in a rule or an hrule` |
| a query's `:expect` listing a fact no pattern of the `:goal` matches | `kb load error: :expect lists (q A H1), which no atom of the query's :goal matches — a query's claim lists its projection classes; a claim about the state is (solve … :expect (region …))'s` |
| a `:goal` whose `eq` or `neq` compares a form that holds a variable | `kb load error: query: its :goal writes (neq ?x (f ?y)), and the form (f ?y) holds a variable — a guard compares two values, each a variable, a name, or a fact that holds no variable` ([Q-M1f.30](../../../decisions/language/Q-M1f.30-a-guard-argument-holds-no-variable-inside-a-fact.md)) |
| a `?var`, `_` or `?_` inside an `:expect` | a parse error at the variable, or at the `_` with its note ([§ 2.8](#28-_-and-_-are-refused-wherever-they-are-written)) † |

A claim's lines are `; `-joined with every other loader line, and each is pinned by a fixture in
[`examples/broken/load/`](../../../../examples/broken/load/) or, for a † row, [`examples/broken/`](../../../../examples/broken/).
What a claim says, and the shapes it takes, are [`12-query.md`](../03-syntax-and-semantics/12-query.md)'s
([Q-M1c.2](../../../decisions/language/Q-M1c.2-an-expectation-is-ground-and-naming-a-relation.md)).

**A false claim is a result, not an error.** A claim the run refutes prints `:expect FAILED` with the disagreement,
writes one `<file>: … :expect FAILED — <why>` line on stderr, and exits **1**. `ein solve` on a file with several
queries runs the program once, reads every query off that run, and exits 1 if any claim fails. A claim the run did
not decide — it asked another part of R (`-n`, a line the claim is not about) or stopped (`--region-budget`,
`--region-limit`) — reads **`NOT CHECKED`**: under `ein solve` that is a `note:` on stderr and exit **0**, because
the run answered what it was asked; under `ein test` it is exit **1**, because `ein test` asks the question its
claims are and then, of what that leaves undecided, the describing one, so a claim it could not decide is one it
failed to check. **Under `ein test` a run that did not exhaust what a claim is about still refutes it**, by a class
of a state found that the claim does not list, or for `(solve … :expect (false))` by an alive root: `FAILED`, where
`ein solve` asked the same question prints the note
([Q-M1f.222](../../../decisions/answer/Q-M1f.222-under-ein-test-a-state-found-refutes-a-claim.md)). Under
`--no-search`, a claim root does not decide moves no exit code ([`05-cli.md`](05-cli.md)).

### 2.2 The verdict's refusals

`open` is the verdict an obligation rule states: its whole `:assert` is `(open)` or `(open ?R)`
([`08-obligations.md`](../03-syntax-and-semantics/08-obligations.md),
[Q-M1d.2](../../../decisions/hypotheses/Q-M1d.2-an-obligation-is-a-rule-that-concludes-open-r.md)). Each refusal
below is a place the engine would otherwise have to guess.

**Five shapes are parse errors**, `open` being no name and a rule's `:assert` the one place that takes it
([§ 2.17](#217-open-is-written-only-as-a-rules-verdict)): an `(open …)` in a `:match`, one with two or more
arguments, an `:assert` that concludes `open` and something else, an `(open …)` nested inside a conclusion, and an
argument that is neither a variable nor a name. Each is the line *``unexpected `open` ``* with its note, or an
unexpected token where the grammar's `(open ?R)` ends; the fixtures are `open_in_match`, `open_arity` and
`open_mixed_assert` in [`examples/broken/`](../../../../examples/broken/).

**Four are the loader's**, because the grammar cannot see them, and each line starts with where the `(open …)` is
written ([Q-M1f.87](../../../decisions/diagnostics/Q-M1f.87-the-loaders-refusals-of-the-verdict-start-where.md)),
behind `kb load error: `:

| input | the line |
|---|---|
| `(open ?R)` where `?R` is not a parameter | ``<file>:<l>:<c>: <k> '<r>': `(open ?R)` names a relation the activator does not bind — `?R` is not in the parameter list`` |
| no `(absent …)` holds a positive `?R` premise | ``… <k> '<r>': `(open ?R)` needs an `(absent …)` in :match holding a positive `?R` premise — that premise is the witness the obligation owes`` |
| two or more `(absent …)` do | ``… <k> '<r>': <m> `(absent …)` guards hold a positive `?R` premise — which one states the obligation is not decidable`` |
| the `?R` premise binds no variable of its own | ``… <k> '<r>': `(open ?R)`'s `?R` premise binds no variable of its own, so the obligation is ground — that is a plain `absent` check and not something a witness could discharge`` |
| two positive `?R` premises each bind one | ``… <k> '<r>': <m> positive `?R` premises each bind a witness variable — a compound witness has no single slot to branch on`` |

The last row is pinned by `open_compound_witness` in [`examples/broken/load/`](../../../../examples/broken/load/),
whose line reads `{FILE}:10:12: rule 'bad': 2 positive …`. Binding the name is refused too, at the parse:
`(relation open …)`, `(macro open …)` and `(rule open …)` declare nothing, `open` being one of the reserved words no
declarator binds ([`03-reserved-names.md`](03-reserved-names.md)).

### 2.3 `eq` and `absent` at another arity

`eq` and `neq` compare two objects, and `absent` is negation as failure over one premise. Written in a rule, a goal or
an hrule at any other arity, each is a parse error, with no note: `(eq ?x)` and `(neq ?x)` at their `)`, and
`(absent (p ?x) (q ?x))` at its second premise. The fixtures are `eq_arity_low`, `eq_arity_high`,
`absent_arity_zero` and `absent_arity_high` in [`examples/broken/`](../../../../examples/broken/). `(false)` takes no
argument, and `(false ?x)` is a parse error with a note
([§ 2.22](#222-an-object-is-an-atom-or-a-fact-a-state-could-hold)).

A macro's body is a template the grammar does not read expanded, so a call can still put either at another arity
where a rule's `:match` or `:assert` is. The loader refuses it there, at the macro body's position
([§ 2.16](#216-a-malformed-primitive-a-macros-expansion-writes)):

| input | the line, behind `kb load error: ` |
|---|---|
| `eq` or `neq` at any arity but 2 | `<file>:<l>:<c>: rule '<r>': its :match writes (eq ?x), and eq takes exactly 2 arguments, not <n> — it compares two values, so write one guard per pair` |
| `absent` at any arity but 1 | `<file>:<l>:<c>: rule '<r>': its :match writes (absent …), and absent takes exactly 1 argument, not <n> — negation as failure is over one premise, so wrap several in an (and …)` |

A relation parameter its activator binds to `eq`, `(?R ?x)` under `(r eq)`, is the loader's too
([§ 2.14](#214-eq-at-another-arity)). The witness is
[`examples/broken/load/eq_in_a_macro_body.ein`](../../../../examples/broken/load/eq_in_a_macro_body.ein). In a
`.einb` the position names the container, `<program stored in x.einb>`
([Q-M1f.15](../../../decisions/diagnostics/Q-M1f.15-a-position-in-a-container-names-its-stored.md)).

### 2.4 An artefact flag's failure

Seven options name a file or a directory the run will write: `ein solve`'s `--events`, `--json-summary`,
`--trace`, `--dump-answer` and `--dump-lattice`, `ein oracle`'s `--dump`, and `ein test`'s `--json-report` (with its
own `--events` and `--json-summary`). What happens when the write fails:

| | |
|---|---|
| **the line** | `error: --<flag> <path>: <os error>`, on stderr, for every one of them: the flag because a run may carry three, the path so that a shell that expanded a variable to the wrong place can see it, the OS error because it says why |
| **the exit code** | **unchanged** — 0 on a run that answered. An option whose help says *additive: stdout, stderr and the exit code are unchanged* keeps saying it: the answer on stdout is correct and complete, and the artefact is paperwork |
| **the exception** | **`--dump-answer`**, **`--dump-lattice`** and **`oracle --dump`** exit 1 when they cannot make their directory, without solving: the directory is made before the run, so a failure there means the run cannot do what it was asked. Their files are written after the answer exists, and a failure there is the additive arm's: the line, and the exit code unchanged |
| **an empty path** | a **usage error**: exit 2, refused by the command line before anything runs, for all seven |
| **a pipe whose reader has gone** | the write ends the process by `SIGPIPE`, whichever file it goes to, `/dev/stdout` or a FIFO ([§ 2.19](#219-a-closed-stdout-ends-ein-by-sigpipe), [Q-M1f.100](../../../decisions/cli/Q-M1f.100-an-artefact-written-to-a-closed-pipe-ends.md)) |

```text
$ ein solve ok.ein --json-summary /nonexistent/x.json
error: --json-summary /nonexistent/x.json: No such file or directory (os error 2)
$ echo $?
0
```

**What a consumer gets, said plainly.** A pipeline that asks for `--json-summary` and gets exit 0 with no file has to
read stderr to find out. That is the cost of the additive reading, and it is a ruling rather than a note: the
alternative is a distinct exit code, and exit 2 already means a usage error
([Q-M1e.22](../../../decisions/cli/Q-M1e.22-a-failed-artefact-write-keeps-the-runs-exit-code.md)).

### 2.5 The store's own limits

The store interns symbols and facts into two id spaces of $2^{30}$ each, and keeps their text and arguments in two arenas
addressed by a 32-bit start. Reaching any of the four is a load error a reader can act on, never a crash:

| input | the line | exit |
|---|---|---|
| more than $2^{30}$ distinct symbols, or facts | ``too many distinct {symbols,facts} — the limit is 1073741824`` | 1 |
| more than $2^{32}$ bytes of symbol text | ``too much symbol text — the limit is 4294967296 bytes`` | 1 |
| more than $2^{32}$ stored fact arguments | ``too many stored fact arguments — the limit is 4294967296`` | 1 |
| a fact with more than 65 535 arguments | ``kb load error: a fact takes at most 65535 arguments`` | 1 |

The last is reachable with a three-line program: `(p A0 … A65535)` is refused, and a fact of 65 535 arguments loads
and is answered. The two arena bounds are guarded, though far away, because what they prevent is not a crash but a
wrong answer: a wrapped start would be another object's identity.

### 2.6 A relation named by a number

A run of digits is a name ([Q-M1e2.24](../../../decisions/language/Q-M1e2.24-ein-has-no-numbers-and-no-ranges.md)),
so a rule that names the relation it matches or asserts by a parameter, `(rule mirror (?R) :match (?R ?a ?b)
:assert (?R ?b ?a))`, takes `(mirror 3)` as it takes `(mirror q)`: `3` names the relation `3`, a name the program
has, and there is no refusal to meet. A rule brings in no name the program does not have
([Q-M1e.21](../../../decisions/hypotheses/Q-M1e.21-a-rule-brings-in-no-name.md)).

What is refused is a parameter that stands for a relation bound to a **fact**, `(mirror (a b))`: a relation is an
atom, and that is [§ 2.9](#29-a-rule-shape-that-reads-nothing)'s line, at load where the activator is written and
where a rule could derive it. An activator of the wrong arity activates nothing and is not checked.

### 2.7 A refutation resting on an `absent` a commitment can fill

A rule that concludes `(false)` or `(not …)` may not rest on an `(absent P)` over facts of a relation that a
commitment can still add to — directly, through a fact a rule derived under one, or through its activator
([Q-M1e2.5](../../../decisions/language/Q-M1e2.5-a-refutation-may-not-rest-on-an-absent.md)). Over such facts the
shape calls a state that has not yet decided `P` a contradiction, and a commitment that decides it would bring the
state back to life: *dead* would not be upward-closed, and no answer about R could be trusted. The line says *a
relation a commitment can still extend* and *close the relation*: what is extended or closed is the relation's held
extent, the facts of it a state holds.

*Extendable* is the static check's set of the relations whose held extent a commitment can add to
([`08-monotonicity.md`](../../reasoning/02-kb-evolution/08-monotonicity.md)), followed through every rule with parameters whose
activator relation it holds, because a commitment can bring such a rule a plan root never had, and what that plan
writes a commitment can change too; an `absent` such a rule reads itself, over a relation in the set or over the
relation its parameter names, taints what it derives. The check runs after root's fixpoint, so it refuses **every
command that takes the universe**: `ein solve` in every mode (`--monotone`, `--count` and `--no-search` too), every
`(solve :expect)` check, `ein test`, `ein oracle`, `ein show hyps`, `ein render lattice` and `ein render h-space`.
`ein saturate` and
`ein render rules|constraints|program|kb` take no universe and do not refuse. A plan has an activator when its rule
has parameters, whatever the activator's arguments, and the `[activator …]` label prints the activator's names.

One line per refusing rule, sorted by rule, with no `kb load error: ` before it:

| cause | the line | exit |
|---|---|---|
| the rule reads an extendable relation inside an odd number of `absent`s | `rule '<r>'[ [activator <a…>]]: concludes <(false)\|(not …)> from (absent …) over <R>, a relation a commitment can still extend — <the rest>` | 1 (`ein test`: 2) |
| it reads `<P>`, which rule `<d>` derives resting on such an `absent` | `… concludes <c> from <P>, which rule '<d>' derives resting on (absent …) over <R>, a relation a commitment can still extend — <the rest>` | 1 (2) |
| the same, the `absent` over the relation a parameter of rule `<d>` names | `… concludes <c> from <P>, which rule '<d>' derives resting on (absent …) over the relation its parameter ?<p> names, and a commitment can derive its activator, so a node can compile it over a relation a commitment can still extend — <the rest>` | 1 (2) |
| its activator is such a fact | `… concludes <c> from its activator, which rule '<d>' derives resting on (absent …) over <R>, … — <the rest>` | 1 (2) |
| a rule with parameters, whose activator a commitment can derive, reads its parameter's relation under an odd `absent` | `… concludes <c> from (absent …) over the relation its parameter ?<p> names, and a commitment can derive its activator, so a node can compile it over a relation a commitment can still extend — <the rest>` | 1 (2) |

`<the rest>` says why — *so it calls a state that has not yet decided <R> a contradiction, and a commitment would
lift it (dead is not upward-closed under absent; …)* — and then names the three forms that say what was meant, behind
*Write what it means:*

1. **close the relation**, `(solve :no-hypothesis (<R>))`, if no commitment is meant to add a fact of it. Where the program
   declares an hrule, no list filters a candidate
   ([§ 2.27](#227-a-programs-hypothesis-mechanisms-do-not-contradict-one-another)), so the form reads *let no hrule
   propose <R>, if no commitment is meant to extend it — hrule 'h' proposes it, and no list filters an hrule's
   candidates*, naming every active hrule that proposes `<R>`, or *let no hrule propose what <R> is derived from …*
   where none proposes it directly. Where no hrule is declared and an obligation owes `<R>`, it reads *let no
   obligation owe <R>, if no commitment is meant to extend it — obligation 'o' owes it, and no list may filter what
   an obligation owes*;
2. **state a requirement as a decision or an obligation**: a yes/no attribute that must be decided (`functional` +
   `total-owed`), or `(open ?R)` with hrules that propose it;
3. **conclude from a stored negative**, `(not …)`, as `std.algebra`'s `total` does.

```text
$ ein solve f.ein     # (q a) and (rule r () :match (and (q ?x) (absent (p ?x))) :assert (false) …), p declared
rule 'r': concludes (false) from (absent …) over p, a relation a commitment can still extend — so it calls a state that has not yet decided p a contradiction, and a commitment would lift it (…). Write what it means: close the relation, (solve :no-hypothesis (p)), if no commitment is meant to extend it; state a requirement as a decision or an obligation, a yes/no attribute that must be decided (functional + total-owed) or (open ?R) with hrules that propose it; or conclude from a stored negative, (not …), as std.algebra's total does
```

The witnesses are [`examples/ein-bugs/naf-upward-closure.ein`](../../../../examples/ein-bugs/naf-upward-closure.ein)
and the refused programs under [`examples/ein-bugs/`](../../../../examples/ein-bugs/).

### 2.8 `_` and `?_` are refused wherever they are written

A bare `_`, and `?_`, names nothing
([Q-M1e2.22](../../../decisions/language/Q-M1e2.22-a-bare-is-reserved-and-refused.md)): wherever it is written — an
argument, a relation, a guard's operand, a conclusion, a claim — it is a parse error at the `_`, with a note, on every
command that reads the program:

```text
$ ein solve t.ein               # file is: (p _)
t.ein:1:4: unexpected `_`: _ (reserved)
  note: _ is reserved and names nothing: where any value will do, write a variable used once
  repair: Delete `_`
```

A relation is named by a name, or by a parameter its activator binds
([Q29](../../../decisions/engine/Q29-a-rule-compiles-once-per-activator.md)). A range is no token either: `1..3` is
one name, matched as any name is, and `:card 1..*` is refused at the parse at `:card`, a keyword no form takes
([Q-M1e2.24](../../../decisions/language/Q-M1e2.24-ein-has-no-numbers-and-no-ranges.md)).

### 2.9 A rule shape that reads nothing

Each shape below would read nothing: no fact could make it hold, or it would hold whatever the facts. The grammar
refuses three of them at the parse wherever a program writes them — `_`
([§ 2.8](#28-_-and-_-are-refused-wherever-they-are-written)), a keyword pair inside a term
([Q-M1e2.28](../../../decisions/language/Q-M1e2.28-each-form-takes-a-closed-list-of-keywords.md)), and a premise
that is not a form. The loader refuses the rest, which the grammar cannot see, in a rule, an hrule and an obligation,
one line per slot, `; `-joined with every other loader line
([Q-M1e2.18](../../../decisions/language/Q-M1e2.18-a-keyword-pair-is-no-guards-argument-and-a-fact.md)):

| input | the line, behind `kb load error: ` |
|---|---|
| a form that holds a variable as a guard's argument | `<rule\|hrule> '<r>': its :match writes (eq ?x (f ?y)), and the form (f ?y) holds a variable — a guard compares two values, each a variable, a name, or a fact that holds no variable` ([Q-M1f.30](../../../decisions/language/Q-M1f.30-a-guard-argument-holds-no-variable-inside-a-fact.md)) |
| a premise that is not a form, where a macro's body puts it: `(macro id (?x) ?x)` called as `(id foo)` | `rule '<r>': its :match is foo, and foo is no premise — a premise is a pattern (relation argument …), a guard (eq a b) or (neq a b), an (absent …), or an (and …) or (or …) of premises` (`… writes foo as a premise of and, …` inside a form) |
| a relation written as a variable no parameter binds, at the top level or inside a pattern, in either clause | `rule '<r>': its <:match\|:assert> writes (?r b) with ?r as its relation, which no parameter binds — a relation is named by a name or by a parameter its activator binds` |
| an activator, a fact or a literal in an `:assert`, that binds a relation parameter to a nested fact | `rule '<r>': its parameter ?<p> names a relation, and (r (a b)) binds it to the fact (a b) — a relation is named by a symbol, not a fact` |
| the same activator, which a rule could derive through a variable | `rule '<d>': its :assert writes (r ?f), whose ?f is rule '<r>''s parameter ?<p>, which names a relation, and (item ?f) can bind ?f to a fact, as an argument of the declared relation item — a relation is named by a symbol, not a fact` (`… as an argument of (g ?f)` inside a fact pattern, `… of the relation ?S names` under a parameter's relation; `hrule '<d>'` for an hrule; `… where its :match holds through <arm>` where some disjuncts of an `(or …)` bind it where a name is held) |
| a fact, or a literal fact in an `:assert`, written where a relation name is passed on | `the fact (marker (a b)) writes (a b) as ?m of (marker ?m), which rule 'fan' passes to rule '<r>''s parameter ?<p>, which names a relation — a relation is named by a symbol, not a fact` (`rule '<d>': its :assert writes (marker (a b)), whose (a b) is ?m of …` for the literal) |

**A derived activator is refused whether or not a run derives it**
([Q-M1f.1](../../../decisions/language/Q-M1f.1-a-rule-that-could-derive-an-activator-binding.md)). *Could* is read
off the program ([Q-M1f.41](../../../decisions/language/Q-M1f.41-a-position-holds-a-fact-unless-the-program.md)): a
rule's parameter that heads a form holds relation names, and so does every argument of `(relation …)`; an argument of
a **declared** relation holds any object, a fact among them, as does an argument inside a fact pattern and
one of a relation a parameter names; and an argument of an **undeclared** relation, a marker such as
`(bijective color-loc)`, or a rule's own parameter, holds relation names where the program passes a variable bound
there on to a position that does, to a fixpoint, so the stdlib's fan-outs load and a fact written there is refused. A
conclusion written `(?T …)` is read as writing into every rule of its arity whose name the program writes as an
argument ([Q-M1f.42](../../../decisions/language/Q-M1f.42-a-conclusion-whose-relation-is-a-parameter.md)). The
witnesses are `derived_activator.ein`, `derived_activator_never.ein` and `unbound_head.ein` in
[`examples/broken/load/`](../../../../examples/broken/load/).

**A guard's variable needs no binder before it.** A guard waits for the relation premise that binds its variable, so
its place among the relation premises does not change what the premise means; one whose variable nothing else binds
binds it where it is written, `eq` to the other side and `neq` over the program's names, so the order of two guards
on such a variable does ([Q-M1f.34](../../../decisions/language/Q-M1f.34-a-guard-that-binds-its-variable-binds-it-where.md),
[`02-syntax.md`](02-syntax.md), premise order around a guard).

### 2.10 A form with no `:why`

The loader requires a `:why "…"` of every `(relation …)`, `(rule …)`, `(hrule …)` and `(macro …)` it reads, a
module's and the stdlib's included, and refuses each that carries none, one line per form, `; `-joined with every
other loader line:

| input | the line, behind `kb load error: ` |
|---|---|
| a relation, rule, hrule or macro with no `:why` | `<relation\|rule\|hrule\|macro> '<name>' has no :why "…", which this load requires of every relation, rule, hrule and macro` |

A `:why` that is not a string is a parse error. `--no-require-why` loads such a program, on every command that loads
one, and changes nothing else ([`06-config.md`](06-config.md),
[Q-M1f.62](../../../decisions/cli/Q-M1f.62-no-require-why-loads-a-program-without-a-why.md)).

### 2.11 A template reference no reference can reach

A template reference, `{?name}` in a `:why` or a `:goal-text`, reads a variable's name up to its first `}`, so it
names every variable a program writes: `{?isR*}`, `{?_x_}` and `{?a.b}` are filled. `}` is a name character too, so a
variable holding one, `?a}b`, is one no reference can name, and the loader refuses a template that writes a
reference to one ([Q-M1f.11](../../../decisions/language/Q-M1f.11-a-template-may-not-refer-to-a-variable-whose.md)):
a rule's, an hrule's or a macro's `:why` and a query's `:goal-text`, each read against its own variables, one line a
reference:

| input | the line, behind `kb load error: ` |
|---|---|
| a template that writes `{?a}b}` where `?a}b` is a variable of its form | `<rule\|hrule\|macro> '<name>': its :why writes {?a}b}, a reference to ?a}b, which no reference can name — a reference reads a variable's name up to its first }` (`query: its :goal-text writes …` for a query) |

A reference to no variable of the form, `{?a}` there, stays text. A relation's `:why` names its declaration's arrows
by number, `{?3}`, and is not read ([`14-natural-language.md`](../03-syntax-and-semantics/14-natural-language.md)).

### 2.12 A string is natural-language text, and an escape names a character

A string is the value of `:why`, `:source` or `:goal-text` and nothing else
([Q-M1e2.23](../../../decisions/language/Q-M1e2.23-a-string-is-natural-language-text-never.md)). Written
anywhere an argument or a name goes it is a parse error at the string, and a variable written in a fact a parse error at
the variable, on every command that reads a program:

| input | the line |
|---|---|
| a string as an argument or a name: `(p "1")`, `(rule r () :match (p "a") …)`, `(relation "p" T)`, `(config :lattice-sanity-check "true")` | `<file>:<l>:<c>: unexpected `"1"`: a string`, then `note: "1" is a string, and a string is natural-language text, written only after :why, :source or :goal-text: write a name bare` |
| a variable in a fact: `(r ?x)` | `<file>:<l>:<c>: unexpected `?x`: a variable` |
| an escape a string does not read: `\x41`, `\Ж`, `\u{}`, `\u{110000}`, `\u{D800}` | `<file>:<l>:<c>: unexpected `"…"`: a string with an unknown escape`, at the opening quote, then one `note: \u{110000}, at <l>:<c>, is no escape a string reads: a string's escapes are \n, \t, \r, \", \\ and \u{…}, one to six hex digits naming a Unicode scalar value, 0 to 10FFFF and not D800 to DFFF` per escape |

Each is followed by the error's ranked repairs (§ 1). A string reads `\n`, `\t`, `\r`, `\"`, `\\` and `\u{…}`, and
holds any other character as itself, a raw newline and any Unicode among them ([`01-grammar.md`](01-grammar.md), the
lexical grammar). The text a program writes reaches every surface that carries it byte for byte.

### 2.13 A conclusion's variable nothing binds

A rule's or an hrule's `:assert` that writes a variable some way of satisfying its `:match` leaves unbound is refused
at load ([Q-M1e2.37](../../../decisions/language/Q-M1e2.37-a-conclusions-variable-is-bound-by-every-way.md)), one
line per variable, in the order the `:assert` writes them:

| input | the line, behind `kb load error: ` |
|---|---|
| a variable no premise or parameter binds | `<rule\|hrule> '<r>': its :assert writes ?y, which no premise or parameter binds — to conclude it of the objects a relation holds of, bind it with a premise such as (R ?y), or of every name of the program with (eq ?y ?y)` |
| one that some disjuncts of a top-level `(or …)` bind and others do not | `… writes ?v0, which no premise or parameter binds where its :match holds through (r1 ?v1 ?v3) — …`, the disjuncts that leave it unbound `or`-joined |

A variable is bound, on each way the `:match` can hold, by a **parameter**, which the activator binds, a nested fact
among the objects it may be bound to; by a **relation premise**, at any depth of its arguments, a stored `(not P)` among them;
or by an **`eq` or `neq` guard**, which binds a variable nothing else binds over the program's names. One seen only
inside an `(absent …)` is not bound outside it, and `forall` and `unknown` expand to one; `()` binds nothing; a
top-level `(or …)` is one way per disjunct. The line's two spellings say what the implicit form could mean, *the
objects a relation holds of* and *every name of the program*, each written out
([Q-M1f.38](../../../decisions/diagnostics/Q-M1f.38-a-refusals-line-names-the-fault-then-what.md)). An obligation's
`(open ?R)` names a relation, which [§ 2.2](#22-the-verdicts-refusals) checks. The witness is
[`examples/ein-bugs/unbound-assert-var.ein`](../../../../examples/ein-bugs/unbound-assert-var.ein).

### 2.14 `eq` at another arity

`eq` is the relation of equality, between exactly two objects
([Q-M1e2.26](../../../decisions/language/Q-M1e2.26-eq-and-neq-are-the-relations-of-equality-and-non.md)), and the
grammar refuses `(eq ?x)` where a premise or a conclusion is written (§ 2.3). Two ways around the grammar are
refused at load, one line each:

| input | the line, behind `kb load error: ` |
|---|---|
| a rule's or an hrule's relation parameter that can be bound to `eq`, heading a premise (at any depth of `and`, `or`, `not` and `absent`) or a top-level conclusion whose arity is not 2 | `<rule\|hrule> '<r>': its parameter ?R heads (?R ?x), a form of 1 argument, and the fact (r eq) binds ?R to eq — eq is the relation of equality, which holds between exactly two values`; where a rule derives the activator, `… and ?R can be bound to eq by <where eq comes from> — …` ([Q-M1f.46](../../../decisions/language/Q-M1f.46-a-relation-parameter-that-can-hold-eq-heads-only.md)) |
| a written fact of `eq`, or of `(not (eq …))`, at any arity but 2 | `the fact (eq a) writes eq with 1 argument — eq is the relation of equality, which holds between exactly two values` ([Q-M1f.47](../../../decisions/language/Q-M1f.47-a-written-fact-of-eq-at-another-arity-than-two.md)) |

**Which parameters can hold `eq`** is read as § 2.9 reads relation names: a position that holds relation names can
hold `eq` where a fact, a conclusion or a `(solve :hrules …)` activator writes `eq` there, or a `(relation …)`
declaration's signature writes it as a label; or where a conclusion writes a variable that its `:match` binds, in some
disjunct, only at positions that can, or only with an `eq` or `neq` guard while the program writes `eq` as a name; to
a fixpoint. `<where eq comes from>` names the first such source and each conclusion it passes through: *the fact (mark
eq), through rule 'mk''s conclusion (r ?f)*, *rule 'mk''s conclusion (r eq)*, or *an eq or neq guard of rule 'mk',
over the program's names, eq among them, through its conclusion (r ?f)*. A parameter heading a pattern inside a
pattern's argument stands where an argument goes, and no argument is a form of `eq`, so it is refused at every arity
([§ 2.22](#222-an-object-is-an-atom-or-a-fact-a-state-could-hold)). Not read is a declared relation of another arity,
`(?R ?x)` under `(r q)` with `q` declared with two labels, which matches nothing, as `(q ?x)` written directly does, matching being by
arity. `neq` is no name a program can write, so no parameter holds it.

### 2.15 A nested `(or …)`, a wide `:match` and a goal's relation variable

Three shapes no plan can be made of are refused at load, one line each
([Q-M1f.72](../../../decisions/diagnostics/Q-M1f.72-a-nested-or-a-wide-match-and-a-goals-relation.md)):

| input | the line, behind `kb load error: ` |
|---|---|
| an `(or …)` written as a premise, of an `(and …)`, an `(absent …)` or a top-level `(or …)`, in a rule's, an hrule's or an obligation's `:match` or a query's `:goal`, written or produced by a macro's expansion ([Q-M1f.74](../../../decisions/language/Q-M1f.74-a-nested-or-a-macros-expansion-writes-is-refused.md)) | `rule 'r': its :match writes (or (p ?x) (q ?x)) as a premise of and — an (or …) is a whole :match, each of its premises one way it holds: lift it to the top of the :match, or split the rule`; a goal's `query: its :goal writes … — an (or …) is a whole :goal, … or ask two queries` |
| a `:match` that writes more than 256 distinct variables, the rule's parameters among them, or a `:goal` that does ([Q-M1f.73](../../../decisions/engine/Q-M1f.73-a-rule-binds-at-most-256-variables-counted.md)) | `rule 'wide': its :match writes 257 distinct variables, and a rule binds at most 256 — split the rule`, with *its parameters among them* where it has any; a goal's `query: its :goal writes 257 distinct variables, and a goal binds at most 256 — ask two queries` |
| a query's `:goal` premise whose relation is a variable, under `and`, `or` and `absent` | `query: its :goal writes (?R Rex Animal) with ?R as its relation, which nothing binds — a relation is named by a name, or by a rule's parameter its activator binds, and a query has no activator` |

The witness of the first is [`examples/broken/load/nested_or.ein`](../../../../examples/broken/load/nested_or.ein).
**What is not refused**: a top-level `(or …)`; an `(or …)` as a pattern's argument where a macro is called
([§ 2.22](#222-an-object-is-an-atom-or-a-fact-a-state-could-hold) has what an argument can be); a macro whose body is an
`(or …)` called as a whole `:match`; a macro no rule uses. A goal's pattern *inside* a pattern whose relation is a
variable is [§ 2.21](#221-a-goal-is-held-to-what-a-rules-match-is-held-to)'s.

### 2.16 A malformed primitive a macro's expansion writes

A macro's body is a template the grammar never reads expanded, and a call takes any premise as an argument, so a
kernel primitive can reach a rule in a shape the grammar refuses where a rule writes it. The loader holds a rule's,
an hrule's and an obligation's `:match` and `:assert`, after expansion, to the grammar's shapes by position, and
refuses a form in another, one line each
([Q-M1f.67](../../../decisions/language/Q-M1f.67-a-macros-body-is-checked-where-it-is-called.md),
[Q-M1f.80](../../../decisions/language/Q-M1f.80-a-malformed-primitive-a-macros-expansion-writes.md),
[Q-M1f.81](../../../decisions/language/Q-M1f.81-a-malformed-primitive-is-a-form-the-grammar.md)). **A macro no rule
calls is not read**, so it loads whatever its body.

| a macro's expansion writes | the line, behind `kb load error: <file>:<line>:<col>: <rule>: its <clause> writes <form>, and …` |
|---|---|
| `eq` or `neq` at any arity but 2, in a `:match` or as a conclusion | `eq takes exactly 2 arguments, not <n> — it compares two values, so write one guard per pair` |
| `absent` at any arity but 1 | `absent takes exactly 1 argument, not <n> — negation as failure is over one premise, so wrap several in an (and …)` |
| `not` of what is no pattern: a conjunction, a disjunction, a `neq`, a name, or a variable in a `:match` or under another `not` (`std.macro`'s `unknown` called with a conjunction among them). A `not` of a `not` of a pattern is a pattern, the negation of a negation | `not takes a pattern (relation argument …), not <arg> — it is the stored negation of one fact; negation as failure is (absent …)`; as a conclusion, *a pattern (relation argument …) or a variable* |
| a connective where an argument goes: an `(and …)`, an `(or …)` or a `(neq …)` as a guard's argument, a conclusion's or a negated pattern's | `<value> is a connective where a value goes — <c> heads no fact, and a value is a name, a variable or a fact (relation argument …)` |
| `false` with an argument, as a conclusion | `false takes no argument, not <n> — (false) concludes that the state is impossible, and names nothing` |
| an `(or …)` as a conclusion | `an (or …) is no conclusion — a rule concludes every conclusion it writes, and a disjunction is a :match's, one way it holds per premise: write one rule per disjunct` |
| a conclusion that is no form: a body that is its parameter, called with a variable or a name | `kb load error: <file>:<line>:<col>: <rule>: its :assert concludes ?y, which is not a fact — a conclusion is a fact (relation argument …), or an (and …) of facts` |

```text
$ ein solve t.ein     # (macro one (?x) (eq ?x) …), called in a rule's :match as (one ?x)
kb load error: t.ein:3:18: rule 'r': its :match writes (eq ?x), and eq takes exactly 2 arguments, not 1 — it compares two values, so write one guard per pair
```

**The position** is where the expanded form is written
([Q-M1f.82](../../../decisions/diagnostics/Q-M1f.82-a-malformed-primitive-a-macro-writes-is-refused.md)): the macro
body's for a form the body writes, `(eq ?a)`; for one the grammar builds for a keyword, `(not …)`, `(neq …)` or
`(or …)`, which keeps none, its first part's, the call's argument, so a form `std.macro`'s `unknown` writes is placed
in the program's file; in a `.einb`, the stored program, `<program stored in x.einb>`. **The rule** is named as the
loader names it, `hrule 'h'` for an hrule.

**An `(and …)` a macro's call leaves inside a conclusion's `(and …)` is no malformed primitive**: a conclusion is read
flat at any depth, so `(and (r a) (both a))`, `both` writing `(and (p ?x) (q ?x))`, concludes `(r a)`, `(p a)` and
`(q a)`. A macro called in a query's `:goal` is expanded and checked as a rule's `:match` is
([§ 2.21](#221-a-goal-is-held-to-what-a-rules-match-is-held-to)).

### 2.17 `open` is written only as a rule's verdict

`open` is the verdict a rule's `:assert` states, `(open)` or `(open ?R)` ([§ 2.2](#22-the-verdicts-refusals)), and
it is written nowhere else ([Q-M1f.66](../../../decisions/language/Q-M1f.66-open-is-written-only-as-a-rules-verdict.md)):
it is one of the words that are never a name ([`03-reserved-names.md`](03-reserved-names.md)), and a rule's whole `:assert`
is the one place the grammar takes it. Every other place is a parse error at the `open`, on every command that reads
the program, with a note
([Q-M1f.88](../../../decisions/diagnostics/Q-M1f.88-a-misplaced-open-is-a-parse-error-with-a-note.md)):

```text
$ ein solve t.ein               # file is: (relation p T) (p (open x))
t.ein:1:20: unexpected `open`
  note: open is reserved: it is the verdict a rule's :assert states, written as the whole :assert, (open) or (open ?R), and nowhere else, so it names no relation and no value
  repair: Delete `open`
```

The places: a fact's, a nested fact's or a stored negation's relation; a pattern's relation anywhere in a rule, an
hrule or a query's `:goal`, and a claim's fact; an argument, as a fact's, a slot's label, a premise's, a
guard's or a conclusion's argument, a macro call's, an activator binding a relation parameter, a goal's or a
claim's; a name in a `(solve …)` list, a query's `:over`, an import, a fact's `:rule` or `:using`, or a trace step;
the verdict's own argument, `(open open)`; and a macro's body, called or not
([Q-M1f.86](../../../decisions/language/Q-M1f.86-a-macros-body-never-writes-open.md)). A container whose stored
program writes one is refused where it is read.
What the loader still refuses of the verdict is § 2.2's last five, each placed where the `(open …)` is written. The
words a fact's head admits by position are names, each by
[Q-M1e2.17](../../../decisions/language/Q-M1e2.17-a-reserved-word-is-a-keyword-only-as-a-whole.md) (`eq` checked,
§ 2.14).

### 2.18 An imported module is read as the program is

An imported module on disk, a file-relative one or a `std.*` one under a checkout or `$EIN_STDLIB`, is read as every
command reads the program it is given, and a module that exists and cannot be read as text is refused with that
read's line, behind `kb load error: `, naming the path the import resolves to
([Q-M1f.93](../../../decisions/diagnostics/Q-M1f.93-a-modules-read-line-names-the-path-its-import.md)):

| the module | the line |
|---|---|
| not UTF-8 | `kb load error: m.ein:3:4: byte 0xff is not UTF-8: a program is UTF-8 text, or a .einb container`, at its first such byte |
| unreadable | `kb load error: PermissionError: [Errno 13] Permission denied: 'm.ein'` |
| a directory | `kb load error: IsADirectoryError: [Errno 21] Is a directory: 'm.ein'` |
| a `.einb` container | `kb load error: m.ein: a .einb container, and an imported module is Ein text: import the .ein it was saved from` ([Q-M1f.91](../../../decisions/imports/Q-M1f.91-an-imported-module-is-ein-text-and-a-einb.md)) |
| holding a character that is not ASCII outside a string | `kb load error: <its resolved path>:2:5: 'é' (U+00E9) is not ASCII: …` ([§ 1.6](#16-a-character-that-is-not-ascii-is-written-only-in-a-string-or-a-comment)), as any parse error in a module names its resolved path |
| missing | `kb load error: (import m) — module not found at m.ein (None)` |

The first three are the lines the module's path gives to `ein solve` as a program; the directory's and the
unreadable file's keep the Python spelling, and the missing module's `(None)` is the import's position, which a
top-level form does not carry ([§ 1.5](#15-loader-messages-about-a-top-level-form-end-in-at-none),
[Q-M1f.94](../../../decisions/diagnostics/Q-M1f.94-a-module-is-refused-with-the-line-its-path-gives.md)).

### 2.19 A closed stdout ends `ein` by `SIGPIPE`

A write to a pipe whose reader has gone — `ein saturate x.ein | head`, a pager quit before the end, a closed socket —
ends the process by `SIGPIPE` at that write, as it ends `cat`, `grep` and `head`
([Q-M1f.68](../../../decisions/cli/Q-M1f.68-a-closed-stdout-ends-ein-by-sigpipe.md)). Nothing is printed, and a
shell reports 141, 128 + 13, so `set -o pipefail` reads `ein` as it reads them. That holds for every command and every
option that writes to stdout, the help and `--version` among them, and for an artefact flag naming a pipe, `--events
/dev/stdout` or a FIFO ([Q-M1f.100](../../../decisions/cli/Q-M1f.100-an-artefact-written-to-a-closed-pipe-ends.md)).
A run whose writes all landed in the pipe before its reader closed has seen no closed pipe, and exits 0, as `cat`
does.

**It is not an exit code.** *Every answer exits 0* is a rule about the codes Ein returns, and a process the kernel
ends returns none ([`05-cli.md`](05-cli.md) has the codes). Windows has no `SIGPIPE`: there, a write to a pipe whose reader
has gone is an error, and the process exits 101
([Q-M1f.101](../../../decisions/cli/Q-M1f.101-off-unix-a-closed-stdout-is-left-to-rusts.md)).

### 2.20 A `(solve …)` list names what the program has

The three lists of a `(solve …)`, `:hrules`, `:hypothesis-relations` and `:no-hypothesis`, choose the candidates, and
so the lattice the answer is a region of ([`09-hypotheses.md`](../03-syntax-and-semantics/09-hypotheses.md)). Every name one
holds names something the program has, or the loader refuses the program, one line per name, placed at the name
([Q-M1f.114](../../../decisions/language/Q-M1f.114-a-relation-a-solve-list-names-is-one-the-program.md),
[Q-M1f.115](../../../decisions/language/Q-M1f.115-an-hrules-list-names-declared-hrules.md)):

| the list writes | the line, after `kb load error: <file>:<line>:<col>: solve: its ` |
|---|---|
| `:hrules (gues …)`, and no hrule is named `gues` | `:hrules names gues, and the program declares no hrule gues — it declares guess` |
| `:hrules (guess (at Color))`, and `guess` takes three arguments | `:hrules gives hrule 'guess' (at Color), 2 arguments, and guess takes 3, (?R ?T1 ?T2) — an activator binds each parameter once` |
| `:hrules (guess (att Color House))`, `?R` naming a relation, and no relation is named `att` | `:hrules gives hrule 'guess' (att Color House), whose att is its parameter ?R, which names a relation, and the program names no relation att — a relation of the program is one it declares, states a fact of, or writes in a rule, an hrule or a goal` |
| `:hypothesis-relations (like)` or `:no-hypothesis (like)`, and no relation is named `like` | `:no-hypothesis names like, and the program names no relation like — a relation of the program is one it declares, states a fact of, or writes in a rule, an hrule or a goal` |

**A relation of the program** is an atom the program writes as a relation outside the three lists: one it declares or
states a fact of; `eq` and `neq`; the relation of any form in a rule's, an hrule's or an obligation's
`:match` or `:assert`, in a query's `:goal` or in a fact's argument; and a name a fact or a conclusion passes where a
relation's name is held, as `(bijective color-loc)` passes `color-loc` (§ 2.9's positions). So a relation only a
rule's or an hrule's `:assert` derives is one, an imported rule's included. The test is *named elsewhere*, not *made
by something*, because an activator's hrule makes the relation it is given, `att` as it makes `at`: what tells a
misspelt name is that the program writes it nowhere else. A parameter names a relation where it heads a form of the
hrule, or is passed on to a position that holds one. A program the loader has already refused gets no line from this
check.

**What is not refused here.** A list that names what the program has and contradicts another of its hypothesis
declarations is [§ 2.27](#227-a-programs-hypothesis-mechanisms-do-not-contradict-one-another)'s. An activator's
argument at a parameter that stands for no relation is any object, which no declaration lists, so
`(guess (at Colour House))` loads and proposes nothing, and the read-out's `universe` row says *no candidate*
([Q-M1f.116](../../../decisions/language/Q-M1f.116-an-activators-argument-at-a-parameter-that-names.md)); so does a
parameterised hrule that no activator names. A bare name in `:hrules` selects nothing: every hrule without parameters
proposes whether the list names it or not.

### 2.21 A goal is held to what a rule's `:match` is held to

A query's `:goal` is a premise, and the loader reads it as it reads a rule's `:match`:

- **A macro call in a goal is expanded**, as a rule's `:match`'s is, so `(unknown (paid ?g))` in a goal is
  `(and (absent (paid ?g)) (absent (not (paid ?g))))`. The expansion is what is matched, what a claim's facts are held
  to and what a class lists; the read-out, the summary and `ein test` print the goal as written
  ([Q-M1f.117](../../../decisions/language/Q-M1f.117-a-macro-call-in-a-goal-is-expanded-and-the-goal.md)). A macro
  call in a claim is not expanded: a claim lists facts, and one headed by a macro's name names no relation
  ([Q-M1f.118](../../../decisions/language/Q-M1f.118-a-claim-is-not-expanded-so-a-claim-headed.md)).
- **The expanded goal gets the checks a rule's `:match` gets**: § 2.9, § 2.15, § 2.16 and the macros' own.
- **Every relation a goal asks about, and every relation a claim lists a fact of, is a relation of the program**
  ([Q-M1f.119](../../../decisions/language/Q-M1f.119-a-goal-and-a-claim-ask-only-about-relations.md)): § 2.20's
  notion without the goals, so one the program declares, states a fact of, writes as the relation of a fact that
  is an argument, or writes in a rule, an hrule or an obligation, at any depth, a rule's `:assert` among them, and
  one a `(solve :hrules …)` activator has its hrule
  propose. `eq`, `neq` and the kernel's own relations, `(relation ?R ?T)`, are every program's; `is-a` is a
  relation of the program only where it writes one, as any relation is.

| the goal writes | the line, after `kb load error: ` |
|---|---|
| `(unknown (paid ?g))`, and the program does not import `unknown` | `query: '(unknown …)' is a std.macro pattern macro used without importing it — add (import std.macro :symbols (unknown)); unexpanded it would silently never match` |
| `(settled ?g Guest)`, and the macro `settled` takes one argument | `query: its :goal: macro settled/1 invoked with 2 args at <file>:<line>:<col>` |
| a call of a macro that expands to itself | `query: its :goal: macro expansion exceeded depth 50 — a macro likely expands to itself` |
| a call whose expansion leaves a name where a premise goes | `query: its :goal writes Ann as a premise of and, and Ann is no premise — a premise is a pattern (relation argument …), a guard (eq a b) or (neq a b), an (absent …), or an (and …) or (or …) of premises` |
| a call whose expansion writes a kernel primitive in another shape (§ 2.16) | `<file>:<line>:<col>: query: its :goal writes (eq ?g), and eq takes exactly 2 arguments, not 1 — it compares two values, so write one guard per pair` |
| `(payed ?g)`, at any depth, under `(absent …)` and `(not …)` too, and no relation is named `payed` | `<file>:<line>:<col>: query: its :goal writes (payed ?g), and the program names no relation payed — a relation of the program is one it declares, states a fact of, or writes in a rule or an hrule, so no state holds a fact of payed` |
| `(says (?R ?g))`, a relation written as a variable inside a pattern, under `(absent …)` too | `query: its :goal writes (?R ?g) with ?R as its relation, which nothing binds — a relation is named by a name, or by a rule's parameter its activator binds, and a query has no activator` ([Q-M1f.121](../../../decisions/diagnostics/Q-M1f.121-a-goals-relation-variable-is-refused-with.md)) |

A line that names a place names where the pattern writes its relation. A program the loader has already refused gets
no line about its relations. A relation only a rule's `:match` reads is a relation of the program, so a goal over it
loads and has no row: a library rule may read a relation its importer supplies, and a rule that can never fire is
reported nowhere ([Q-M1f.120](../../../decisions/language/Q-M1f.120-a-rule-may-read-a-relation-nothing-makes.md)).
A class of a goal that holds an `(and …)` inside its `(and …)`, which a macro's expansion writes, lists the inner
premises one by one, as the loader reads them for a claim.

### 2.22 An object is an atom, or a fact a state could hold

*An object is an atom, or a fact a state could hold: a fact of an ordinary relation, or the stored negation of a
fact a state could hold*
([Q-M1f.113](../../../decisions/language/Q-M1f.113-an-object-is-an-atom-or-a-fact-a-state-could.md)). A fact
inside a fact, a pattern inside a pattern, and a conclusion's or a guard's argument are objects, or patterns of
them, and the lines below call each a *value*. Four rules hold the language to it:

- **`absent` heads no fact, argument or conclusion.** It is negation as failure, a test a premise or a goal makes of a
  state, `(absent <premise>)`. A claim's class still lists what a goal's `(absent …)` says a state does not hold,
  `(model (absent (q a)) (p a))`
  ([Q-M1f.122](../../../decisions/language/Q-M1f.122-absent-heads-a-premise-and-a-member-of-a-claims.md)).
- **`false` takes no argument**: a fact, an argument, a premise, a goal and a conclusion write it only as `(false)`.
  `(false)` as an argument, `(q (false))`, stays
  ([Q-M1f.123](../../../decisions/language/Q-M1f.123-false-with-no-argument-is-an-object.md)).
- **`(eq …)` is no argument**, nor is `(neq …)`: `eq` is computed and never stored (§ 2.14). A fact `(eq a b)` and
  `(not (eq a b))`, a premise and a conclusion of either, and a claim's, stay, each checked; `(not (eq a b))` inside
  a fact or a pattern is refused
  ([Q-M1f.124](../../../decisions/language/Q-M1f.124-eq-a-b-is-written-only-as-a-form-of-its-own.md)).
- **A premise pattern's argument is what an object can be**: an atom, a variable, a pattern of an ordinary relation, or
  `(not <pattern>)`, a stored negation. `and`, `or`, `neq`, `eq`, `absent` and `false` with an argument there are
  refused, in a rule, an hrule, an obligation and a goal. The grammar takes a connective as a pattern's argument,
  because a macro such as `forall` is called as a pattern and takes premises, so this refusal is the loader's, read off
  the clause once its macros are expanded
  ([Q-M1f.125](../../../decisions/language/Q-M1f.125-a-connective-as-a-patterns-argument-is-refused.md)).

The grammar refuses each shape a program writes, at the parse ([`01-grammar.md`](01-grammar.md)), and the error carries a
note:

| the program writes | the error is at | the note |
|---|---|---|
| `(absent k)`, `(absent (p a))`, `(q (absent k))`, `(not (absent k))`, `:assert (absent (p ?x))` in a rule or an hrule, `:assert (q (absent ?x))` | `absent`: *unexpected \`absent\`* | `absent is negation as failure, a test a premise makes of a state, (absent <premise>) in a :match or a :goal, so no fact, value or conclusion is of it: the stored negation of a fact is (not …)` |
| `(false k)`, `(q (false k))`, `(not (false k))`, a premise or a goal `(false ?s)`, `(absent (false ?s))`, a pattern's argument `(q (false ?s))`, `:assert (q (false ?x))`, a claim's `(false k)` | the argument: *unexpected \`k\`: a name* | `false takes no argument: (false) says that the state is impossible, and names nothing` |
| `(q (eq k j))`, `(q (not (eq k j)))`, `:assert (q (eq ?x ?y))`, a guard's `(eq ?y (eq ?x b))`, a claim's `(p (eq a b))`; `neq` in each | `eq`: *unexpected \`eq\`* | `eq is computed and never stored, so (eq a b) is no value: write it as a premise or a conclusion of its own, where it is checked, and never inside another fact or pattern` |

The loader refuses what the grammar cannot see, one line per form, in § 2.16's spelling, placed where the expanded
form is written:

| the clause holds, once its macros are expanded | the line, after `kb load error: <file>:<line>:<col>: ` |
|---|---|
| a pattern's argument `(and …)`, `(or …)` or `(neq …)`, under `(absent …)` too | `rule 'r': its :match writes (q (and (p ?x))), and (and (p ?x)) is a connective where a value goes — and heads no fact, and a value is a name, a variable or a fact (relation argument …)` |
| a pattern's argument `(eq ?a ?b)`, or an argument of that form in a conclusion | `rule 'r': its :match writes (q (eq ?a ?b)), and (eq ?a ?b) is written where a value goes — eq is computed and never stored, and a value is a name, a variable, or a fact a state could hold, (relation argument …) or its negation` |
| a pattern's argument `(absent …)`, as `forall` called inside a pattern leaves | `rule 'r': its :match writes (q (absent (q ?x))), and (absent (q ?x)) is written where a value goes — absent is negation as failure, a premise's test that no fact is of, and a value is a name, a variable, or a fact a state could hold, (relation argument …) or its negation; the stored negation of a fact is (not …)` |
| an argument `(false ?x)` | `rule 'r': its :match writes (q (false ?x)), and (false ?x) is written where a value goes — false takes no argument, and a value is a name, a variable, or a fact a state could hold, (relation argument …) or its negation` |
| a premise `(false ?x)` | `rule 'r': its :match writes (false ?x), and false takes no argument, not 1 — (false) holds where the state is impossible, and names nothing` |
| a conclusion `(absent ?x)` | `rule 'r': its :assert writes (absent ?x), and absent is negation as failure, a premise's test, and no fact is of it — a conclusion is a fact (relation argument …); the stored negation of a fact is (not …)` |

A goal's line reads `query: its :goal writes …`. **A relation parameter that an activator can bind to one of the
three words** is read as § 2.14 reads one that can hold `eq`: it is refused for each premise or conclusion it heads
that the grammar refuses written out, `(absent ?x)` at any arity and `(false ?x)` at one argument or more, and for
each form it heads where an argument goes, `(q (?R ?x ?y))` under `(mk eq)`
([Q-M1f.126](../../../decisions/language/Q-M1f.126-a-relation-parameter-bound-to-absent-false-or-eq.md)). These lines
name the rule and have no place:

```text
kb load error: rule 'mk': its parameter ?R heads (?R ?x), a form of 1 argument, and the fact (mk absent) binds ?R to absent — absent is negation as failure, a premise's test, and no fact is of it
kb load error: rule 'mk': its parameter ?R heads (?R ?x ?x), written where a value goes, and the fact (mk eq) binds ?R to eq — eq is computed and never stored, and a value is a name, or a fact a state could hold
```

The three words stay names: `(h absent)`, `(h false)` and `(mk eq)` are facts.

### 2.23 `not` of a name is the negation of no fact, and an hrule may read `not`

**A conclusion `(not ?x)` is of a variable bound to a fact.** The grammar takes `(not Var)` as a conclusion, as
`(hypothesis ?h)` then `(not ?h)` writes it, and whether `?h` holds a fact depends on the data. A name has no
negation: `(not k)` is a parse error as a fact and as a conclusion, and § 2.16 refuses one a macro writes. So the
loader refuses the conclusion wherever the program shows that the variable can hold a name
([Q-M1f.127](../../../decisions/language/Q-M1f.127-a-conclusion-not-x-is-refused-where-the-program.md)), one line per
conclusion, in § 2.16's spelling, at the variable:

```text
kb load error: <file>:<line>:<col>: rule 'deny': its :assert writes (not ?g), and ?g can be a name: (paid ?g) binds ?g where the fact (paid Ann) holds Ann — not takes a pattern (relation argument …) or a variable bound to a fact: it is the stored negation of one fact, and a name is no fact; negation as failure is (absent …)
```

A position, a relation and an argument's index, *holds a name* where the program shows one there, and the line's
middle says which:

| the program shows | the line's middle |
|---|---|
| a written fact, at any depth and under a `(not …)` | `(paid ?g) binds ?g where the fact (paid Ann) holds Ann` |
| a `(relation …)` declaration, matched as a fact | `(relation ?R ?T) binds ?R where a declaration names a relation or a type` |
| a rule's or an hrule's conclusion that writes a name, or a variable that can hold one, to a fixpoint | `(seen ?g) binds ?g where rule 'greet' writes (seen ?g)` |
| the rule's own activator, a written fact | `?g is its parameter, which the fact (deny Bob) binds to Bob` |
| an hrule's activator in `(solve :hrules …)` | `?g is its parameter, which (solve :hrules (suppose (Bob))) binds to Bob` |
| a declared relation the hypotheses are taken from: the program declares no hrule, the `(solve …)` lists leave the relation in, and nothing writes the closure marker | `(seen ?g) binds ?g where a commitment can hold a name, seen being a declared relation the hypotheses are taken from` |
| nothing but a guard binds the variable | `(neq ?g Ann) binds ?g over the program's names, nothing else binding it`, `(eq ?g Ann) binds ?g to a name` |

The variable *can hold a name* where, in one disjunct of the `:match`, every relation premise that binds it binds it
at such a position. A premise under `(absent …)` binds nothing, and a guard beside a relation premise only filters.
So `(and (paid ?g) (claim ?c))` with `(not ?c)` loads where `claim` holds facts alone, and a position that holds
both, `(claim (paid Bob))` beside `(claim Ann)`, is refused. The programs under
[`examples/saturation/hypothesis-contradiction/`](../../../../examples/saturation/hypothesis-contradiction/) load:
their `hypothesis` is no declared relation, and holds facts. A pattern or a conclusion whose relation is a parameter,
`(?R ?g)`, is read as one of each relation a written activator names there: `(rule deny (?R) :match (?R ?g) :assert
(not ?g))` beside `(deny paid)` is refused, *(?R ?g) binds ?g where the fact (paid Ann) holds Ann*.

**What the loader does not read is the firing's.** Where the activator is derived through a variable, `(marks paid)`
and a rule concluding `(deny ?R)` of what is marked, the parameter's relation is not read, and the program loads.
Where a binding then gives `(not ?g)` a name, there is no fact to conclude: the firing stores nothing for that
conclusion, and stores the rule's other conclusions; an hrule proposes nothing for it. No command stops, no `(false)`
is concluded, and no state holds the negation of a name
([Q-M1f.128](../../../decisions/language/Q-M1f.128-a-firing-that-meets-the-negation-of-a-name.md)).

**An hrule's `:match` may read `not`**: a stored negation as a premise, `(not (paid ?g))`, under `(absent …)`, and
through `std.macro`'s `unknown`. It is matched against the state the hypotheses are taken at, and `ein solve` answers
what `ein oracle` answers
([Q-M1f.129](../../../decisions/engine/Q-M1f.129-an-hrule-that-reads-not-reads-the-state-it-was.md)).

### 2.24 A macro's call is no fact

**A macro is called where the loader expands it**: in a rule's or an hrule's `:match` and `:assert`, at any depth,
and in a query's `:goal` (§ 2.21). There the call is replaced by the macro's body
([`11-macros.md`](../03-syntax-and-semantics/11-macros.md)). Nothing else is expanded, so a form headed by a macro's name
anywhere else is no call. In a fact it is refused at load, one line per call, at the call's name:

```text
kb load error: <file>:<line>:<col>: the fact (settled Ann) is a call of the macro settled, (macro settled (?g) (paid ?g)) — a macro is called in a :match, an :assert or a :goal, where its call is replaced by its body; a fact is not expanded, and no fact is of a macro: write the fact itself
```

| the program writes | the line, after the position |
|---|---|
| a fact headed by a macro's name, at any arity | `the fact (settled Ann) is a call of the macro settled, (macro settled (?g) (paid ?g)) — …` |
| the call inside a fact, at any depth, under `(not …)`, as an `is-a` fact's argument or in a fact's `:using` | `the fact (says (settled Ann)) holds (settled Ann), a call of the macro settled, (macro settled (?g) (paid ?g)) — …` ([Q-M1f.133](../../../decisions/language/Q-M1f.133-a-macros-call-inside-a-claims-fact-or-a-using-is.md)) |
| an imported macro, flat or qualified (`std.macro.unknown`) | `the fact (unknown (paid Bob)) is a call of the macro unknown, (macro unknown (?P) (and (absent ?P) (absent (not ?P)))) — …` |
| a `std.macro` macro the program does not import, `unknown` or `forall` | `the fact (unknown (paid Bob)) is a call of std.macro's macro unknown, which the program does not import — …` ([Q-M1f.132](../../../decisions/language/Q-M1f.132-a-std-macro-macro-the-program-does-not-import.md)) |
| the call inside a fact a claim lists, a query's or a `(solve … :expect (region …))`'s | `query: its :expect lists (settled Ann), a call of the macro settled, (macro settled (?g) (paid ?g)) — a claim lists facts, and …` |
| a `(relation …)` declared under a macro's name | `relation 'settled': settled is a macro too, (macro settled (?g) (paid ?g)) — a premise (settled …) is the macro's call, replaced by its body, and a fact (settled …) is refused, so nothing could be of the relation; give one of the two another name` ([Q-M1f.131](../../../decisions/language/Q-M1f.131-no-relation-is-declared-under-a-macros-name.md)) |

Every line but the last ends *a macro is called in a :match, an :assert or a :goal, where its call is replaced by its
body; a fact is not expanded, and no fact is of a macro: write the fact itself*.

**The call is not expanded into the fact its body spells.** A macro's body is a pattern: `unknown`'s is two
`absent`s, a test of a state, and spells no fact. Expanding only a body that is one ground form would give a macro a
second meaning.

**What stays.** A call as a conclusion, `:assert (settled ?g)`, is expanded, and stores the body's fact,
`(paid Ann)`; an expansion that is no conclusion is refused by § 2.16
([Q-M1f.130](../../../decisions/language/Q-M1f.130-a-macros-call-in-a-conclusion-is-expanded.md)). A macro's name
written as an argument, `(word settled)`, is an atom. A claim headed by the call, `(model (settled Ann))`, names no
relation of the program and keeps § 2.21's line; the line above is for a program nothing else refuses. A relation
named `unknown` or `forall` in a program that does not import `std.macro` is not refused where it is declared: no
fact of it can be written, by the fourth row, and no premise read.

### 2.25 An import brings a module's facts

**Every import brings every declaration and every fact of its module**, with the module's own imports, one copy per
program, under full names ([`11-macros.md`](../03-syntax-and-semantics/11-macros.md) has imports,
[Q-M1f.247](../../../decisions/imports/Q-M1f.247-every-import-brings-its-module-whole.md)):

| of the module | `(import M)` | `(import M :as o)` | `(import M :symbols (n …))` |
|---|---|---|---|
| a declared relation, a rule, an hrule, a macro | kept, written `M.name` | kept, written `o.name` | kept, written `name` if listed and `M.name` if not |
| a fact of a declared relation | kept: `(M.is-a Tea Drink)` | kept: `(o.is-a Tea Drink)` | kept: `(is-a Tea Drink)` where `is-a` is listed |
| a fact of a relation no file declares | kept as written | kept as written | kept as written; the relation may be listed, a name only ([Q-M1f.250](../../../decisions/imports/Q-M1f.250-a-listed-name-a-module-states-facts-of-names.md)) |
| a stored negation, `(not (likes Milk Tea))` | kept, as a fact is | kept, as a fact is | kept, as a fact is |
| a constant in a fact | as written | as written | as written |
| `(query …)`, `(solve …)`, `(config …)`, `(trace …)` | **never brought** | never brought | never brought |

- **No import brings what a module states as a program**: its queries, its `(solve …)`, its `(config …)` and a
  `(trace …)` are dropped where the module is read. The importer's lattice, flags and queries are the ones it writes
  ([Q-M1f.215](../../../decisions/imports/Q-M1f.215-no-import-brings-a-modules-query-solve-config.md)).
- **A listed name** is one the module declares or states a fact of. Any other is refused, on every command:
  `kb load error: (import M :symbols …) — not provided by the module: n at None`.
- **An imported fact is a fact of the program**: its provenance is its own, a `:source` or none, with no stamp of the
  module ([Q-M1f.138](../../../decisions/imports/Q-M1f.138-an-imported-fact-keeps-its-own-provenance.md)); its
  constants are names of the program, and are among the guessable atoms the blind enumerator proposes about
  ([Q-M1f.137](../../../decisions/imports/Q-M1f.137-an-imported-facts-constants-are-names.md)).
- **A fact of a module arrives once**, the module being one copy, whoever and however many import it
  ([Q-M1f.143](../../../decisions/imports/Q-M1f.143-a-modules-fact-arrives-once-however-many-imports.md)). The same
  fact stated twice with two `:source`s, by a module and the program or by two modules, keeps its first statement and
  loses the second with no word, as it does written twice in one file
  ([Q-M1f.139](../../../decisions/imports/Q-M1f.139-a-fact-stated-twice-keeps-its-first-statement.md)).

### 2.26 A form of a declared relation has the declared count of arguments

**A declaration that lists $n$ labels after the relation's name declares that a form of the relation has $n$
arguments**, and the loader refuses a program that
writes one with another count, one line a form, placed at the form
([Q-M1f.315](../../../decisions/language/Q-M1f.315-a-form-of-a-declared-relation-has-as-many.md)):

```text
$ ein solve t.ein       # (relation likes Person Drink :why "…") (likes Ann Tea) (likes Bob)
kb load error: t.ein:1:71: the fact (likes Bob) gives likes 1 argument, and (relation likes Person Drink) declares 2 — a form headed by a declared relation has as many arguments as its declaration has types
```

| where the form is written | the line, after its place |
|---|---|
| a fact | `the fact (likes Bob) gives likes 1 argument, and (relation likes Person Drink) declares 2 — …` |
| a fact inside a fact, at any depth, under `(not …)`, in a fact's `:using` | `the fact (add (n 1) (n z 5)) holds (n 1), which gives n 1 argument, and (relation n Number Digit) declares 2 — …` |
| a rule's or an hrule's `:match` or `:assert`, at any depth, under `(absent …)` and `(not …)`, as its macros expand | `rule 'fans': its :match writes (likes ?who), which gives likes 1 argument, and … declares 2 — …` |
| a query's `:goal`, as its macros expand | `query: its :goal writes (likes ?who), which gives likes 1 argument, and … declares 2 — …` |
| a fact a claim lists, a query's `:expect` or `(solve … :expect …)`'s | `solve: its :expect lists (likes Ann), which gives likes 1 argument, and … declares 2 — …` ([Q-M1f.204](../../../decisions/language/Q-M1f.204-the-declared-count-is-held-in-a-using-a-claim.md)) |

- **What declares a count** is the program's own `(relation …)` and an imported one: an import is resolved before the
  loader reads a form, so a flat import's relation is held under its name and `(import M :as o)`'s under `o.name`.
- **A bare `(relation flag)` declares none**, so `(flag)` is its one form and `(flag up)` is refused
  ([Q-M1f.201](../../../decisions/language/Q-M1f.201-a-bare-relation-r-declares-no-argument.md)).
- **A keyword pair is no argument**: `(likes Ann Tea :source "said")` has two.
- **A macro's body is held where a rule or a goal calls it**, on its expansion; a macro nothing calls is not read, as
  § 2.16 has it.
- **A relation nothing declares has no count** and is not held: `(says Ann)` beside `(says Ann Bob)` loads. That
  includes the kernel's own, which no program declares: `(__closed__ r extra)` loads, and `(relation R ?A ?B)`, the
  declaration's stored fact, is a pattern at any count
  ([Q-M1f.203](../../../decisions/language/Q-M1f.203-a-kernel-marker-and-an-activator-are-held-to-no.md)). `eq` is
  held to two arguments by § 2.14.
- **A relation written as a variable is not read**, `(?R ?a ?b)`, whatever the program shows its parameter holds: a
  rule over a parameter is generic over counts, matching is by count, and `(mirror likes)` on a rule that reads
  `(?R ?a)` matches nothing
  ([Q-M1f.202](../../../decisions/language/Q-M1f.202-a-relation-written-as-a-variable-is-held-to-no.md)). So a rule
  can still *derive* a fact of a declared relation at another count through a parameter; no written form is one.
- **An activator is not a declared relation's form**: `(functional r extra)` against a rule `functional (?R)`
  activates nothing, in silence; an activator in a `(solve :hrules …)` list is held to its hrule's parameters by
  § 2.20.
- **It checks no argument against a label**: `(likes Tea Ann)` loads. Ein has no types, and what is written in an
  argument's place is not held to the label of its slot. The line's last word, *types*, is the engine's word for the
  declaration's labels, and what it counts.
- **No flag turns it off.**

### 2.27 A program's hypothesis mechanisms do not contradict one another

Five declarations steer what a commitment may be about ([`09-hypotheses.md`](../03-syntax-and-semantics/09-hypotheses.md)):
an `(hrule …)` (**H**, with its `:hrules` activators), an obligation rule, `:assert (open ?R)` (**O**, with its
activators), `(solve :hypothesis-relations …)` (**HR**), `(solve :no-hypothesis …)` (**NH**) and `(__closed__ R)`,
written or derived (**C**); the blind enumerator runs where none of the generating ones does
([`03-reserved-names.md`](03-reserved-names.md)). A program whose two declarations say opposite things of one relation is
refused, one line per conflict, and a declaration that filters no candidate, or says what another already says, is
one `note:` line on stderr
([Q-M1f.242](../../../decisions/hypotheses/Q-M1f.242-a-hypothesis-declaration-that-filters-nothing-is.md)):

| pair | refused where | `note:` where |
|---|---|---|
| H + O | never: H generates, O judges | — |
| H + HR | an active hrule proposes a fact of a relation the list leaves out | the list, which filters no candidate beside an hrule |
| H + NH | an active hrule proposes a fact of a relation the list names | the list, the same |
| H + C | an active hrule proposes a fact of a closed relation, one a `(__closed__ R)` names | — |
| O + HR | no hrule is declared, and an obligation owes facts of a relation the list leaves out | — |
| O + NH | no hrule is declared, and an obligation owes facts of a relation the list names | — |
| O + C | an obligation owes facts of a closed relation ([Q-M1f.244](../../../decisions/hypotheses/Q-M1f.244-an-obligated-relations-held-extent-is-not-closed.md)) | — |
| HR + NH | a relation is in both | no hrule: a `:no-hypothesis` name `:hypothesis-relations` already leaves out |
| HR + C | a listed relation is closed | — |
| NH + C | never | no hrule: a `:no-hypothesis` name a closure already closes |

**Read on root's fixpoint.** A derived closure (`std.closure`'s `infer-closure`, a module's closing rule) and a
derived activator (`bijective-setup`'s `(total-owed R isa)`) exist only there. An hrule is **active** where it is
compiled, one without parameters always and one with them once per `:hrules` activator, and the relation whose facts
it proposes is its `:assert`'s; an obligation is a rule root's activators activate, **owed at root or not**
([Q-M1f.243](../../../decisions/hypotheses/Q-M1f.243-an-obligation-is-read-as-activated-owed-or-not.md)), and the
relation it owes is its `(open ?R)`'s. `:hypothesis-relations ()` names none, so beside an active hrule or an
obligation it leaves every relation out. An activator only a commitment derives is not seen.

**On every command, at load**
([Q-M1f.245](../../../decisions/hypotheses/Q-M1f.245-the-hypothesis-mechanisms-are-judged-at-load-on.md)): the
program is judged on its root's fixpoint wherever it could declare two mechanisms at all, and the views and `kb save`
refuse as `solve` does, writing no container.

| the pair | the line, after `kb load error: ` |
|---|---|
| H + HR | `FILE:L:C: solve: its :hypothesis-relations leaves out at, and hrule 'guess' (at Color House) proposes facts of at — no list filters an hrule's candidates, so the program says both that at is guessed and that it is not: drop the list, which filters nothing where an hrule is declared, or the hrule's at` |
| H + NH | `FILE:L:C: solve: its :no-hypothesis names at, and hrule 'guess' (at Color House) proposes facts of at — no list filters an hrule's candidates, …: drop at from the list, …, or the hrule's at` |
| H + C | `FILE:L:C: hrule 'guess' (at Color House) proposes facts of at, and (__closed__ at), written at FILE:L:C, says at is closed — a closed relation is one no commitment extends, and each fact the hrule proposes is a commitment: drop the closure, or the hrule's at` |
| O + HR | `FILE:L:C: solve: its :hypothesis-relations leaves out at, and obligation 'total-owed', activated by (total-owed at is-a), written at FILE:L:C, owes at — an obligation is paid by facts of the relation it owes, so a state that owes at and may not be given one is stuck: add at to the list` |
| O + NH | `FILE:L:C: solve: its :no-hypothesis names at, and obligation 'total-owed', activated by …, owes at — …: drop at from the list` |
| O + C | `[FILE:L:C: ]obligation 'total-owed', activated by (total-owed r is-a), written at FILE:L:C, owes r, and (__closed__ r), which rule 'infer-closure' derives, says r is closed — an obligated relation is not closed: …; drop the closure, or the obligation` |
| HR + NH | `FILE:L:C: solve: its :no-hypothesis names at, and its :hypothesis-relations names at too — the one says guess it, the other never: drop at from one` |
| HR + C | `FILE:L:C: solve: its :hypothesis-relations names at, and (__closed__ at), written at FILE:L:C, says at is closed — a listed relation is one the program guesses, and a closed one it never does: drop at from the list, or the closure` |

The hrule's activator is printed where it has one: an hrule with no parameters reads `hrule 'guess' proposes facts of
at`. The position is the list's name, the list where it names none (`()` at its keyword), the hrule's `:assert`, or
the written activator or closure by its form; a line about two derived declarations has none. An activator or a
closure says where it comes from, *written at FILE:L:C* or *which rule 'r' derives*. The notes, each
`note: FILE:L:C: solve: its :KEY …`, are *filters no candidate — the program declares hrule 'guess', and no list
filters an hrule's candidates*, *names near, which its :hypothesis-relations already leaves out* and *names at, which
(__closed__ at), written at FILE:L:C, already closes*, printed once a process and moving no exit, answer or stdout.
The refusal's fixture is [`examples/ein-bugs/closed-and-owing.ein`](../../../../examples/ein-bugs/closed-and-owing.ein).

**`(solve :no-search)`** states that the lattice is root alone: nothing is proposed, so every generating mechanism
proposes nothing, and an obligation judges root alone. It is how a program says *judge root alone*. Beside an
obligation or a closure it is silent; beside an hrule, `:hypothesis-relations` or `:no-hypothesis` it is one `note:`
each, *FILE:L:C: solve: its :no-search states the lattice is root alone, so hrule 'guess' proposes no candidate* and
*FILE:L:C: solve: its :no-hypothesis filters no candidate — the program states :no-search, whose lattice is root
alone*, which replace the notes above. **It relaxes no conflict**: H, O and the lists are read as declared, so every
refused pair is refused beside it with the same line
([Q-M1f.254](../../../decisions/hypotheses/Q-M1f.254-solve-no-search-relaxes-no-conflict.md)).

### 2.28 An import is by full name, and a short name names one declaration in a file

Every import brings its module whole (§ 2.25); a name a module declares has one full name, `M.name`, the module's name
a path from the program file's directory or `std.x`
([Q-M1f.231](../../../decisions/imports/Q-M1f.231-every-name-a-module-declares-has-a-full-name.md),
[Q-M1f.235](../../../decisions/imports/Q-M1f.235-a-module-has-one-path-from-the-program-files.md)); a file writes
short the names it declares, lists or prefixes with an alias, and any other short name it writes is its own
([`11-macros.md`](../03-syntax-and-semantics/11-macros.md) has imports). Each of the following is refused at load, one
`kb load error:` line, placed where the form or the name is written:

| the program | the line |
|---|---|
| one short name bound to two full names in one file, by two imports or by an import and the file's own declaration ([Q-M1f.234](../../../decisions/imports/Q-M1f.234-one-short-name-bound-to-two-full-names-in-one.md)) | `p.ein:2:9: 'done' is bound to two full names in the program, m.done and n.done — one short name names one declaration in a file: list it from one module, or write the other by its full name`; `… infer-closure (its own declaration) and std.closure.infer-closure …` |
| one alias prefixing two modules in one file | `p.ein:2:9: 'x' prefixes two modules in one file, m and n — one short name names one thing in a file` |
| a prefixed or full name that names nothing its module declares ([Q-M1f.251](../../../decisions/imports/Q-M1f.251-a-prefixed-name-that-names-nothing-its-module.md)) | `p.ein:2:2: 'x.nope' names nothing: m declares no 'nope' — a prefixed name is a full name of a declaration of the module it names (Q-M1f.251)` |
| a full name of a module no import of the program brings, with an import of another module or with none ([Q-M1f.238](../../../decisions/imports/Q-M1f.238-a-full-name-of-a-module-no-import-brings-is.md)) | `p.ein:1:2: 'std.math.decimal-arith.add' is a full name of std.math.decimal-arith, which no import of the program brings — a module enters a program by an import of it: write (import std.math.decimal-arith …) (Q-M1f.238)` |
| the main file declaring a name under the full name of a module the program imports ([Q-M1f.239](../../../decisions/imports/Q-M1f.239-a-program-declares-no-name-under-the-full-name.md)) | `p.ein:2:11: the program declares 'm.mine', a full name of m, which the program imports — a declaration's full name is its own file's, and a full name of m is one m declares (Q-M1f.239)` |
| a module declaring a name with a dot ([Q-M1f.241](../../../decisions/imports/Q-M1f.241-a-modules-declared-name-holds-no-dot.md)) | `d.ein:2:11: d declares 'y.z', a name with a dot — a module's declared name holds no dot, so that a full name such as d.x reads one way (Q-M1f.241)` |
| a module's name with an empty segment or a `/`, `..` among them ([Q-M1f.252](../../../decisions/imports/Q-M1f.252-a-modules-name-has-no-empty-segment-and-no-slash.md)) | `(import ..m) — a module's name is its path from the program's directory, a name per directory joined by dots, none empty and none holding a '/': no '..', no path (Q-M1f.252) at None` |
| a module's own import of a file-relative module written from the module's directory | `(import words) — module not found at <program's directory>/words.ein (None)`: every module is found from the program file's directory |

What is **not** refused: one short name bound to one full name twice; a short name the file writes that no import of
it lists, which is the file's own whatever a module declares under it
([Q-M1f.236](../../../decisions/imports/Q-M1f.236-a-short-name-a-file-writes-and-lists-from-no.md)); a dotted name the
main file declares where no module of the program has its prefix; and a dotted name whose prefix is no module at all,
a name like another.

**What prints** is the main file's listed short name, else its first alias's spelling, else the full name, every
surface alike: a state, a trace, a core, `refuted by`, `--events`
([Q-M1f.237](../../../decisions/imports/Q-M1f.237-a-name-prints-as-the-main-file-writes-it-else.md),
[Q-M1f.253](../../../decisions/imports/Q-M1f.253-under-an-alias-a-name-prints-as-the-main-file.md)). A spelling
another file writes as a name of its own is not taken. The answer does not depend on the import form
([Q-M1f.249](../../../decisions/imports/Q-M1f.249-the-answer-does-not-depend-on-the-import-form.md)).

### 2.29 An hrule proposes one fact per match

An hrule proposes facts, each a commitment of its own, so its conclusion is one fact
([Q-M1f.271](../../../decisions/language/Q-M1f.271-an-hrules-conclusion-is-one-fact.md)): an hrule whose `:assert`,
read flat, is more than one conjunct is refused at load, one `kb load error:` line placed at the conclusion's first
conjunct, read after a macro's expansion:

| the hrule | the line |
|---|---|
| `(hrule h () :match () :assert (and (p a) (q a)) …)`, and an `and` inside it, `(and (and (p a) (q a)) (r a))`, *3 conclusions* | `kb load error: h.ein:3:37: hrule 'h': its :assert writes (and (p a) (q a)), 2 conclusions — an hrule proposes one fact per match, each a commitment of its own: write one hrule per conclusion` |

An `(and …)` of one conjunct, `(and (p ?x))`, is one conclusion and loads. A macro's call that leaves the `(and …)`
is placed as § 2.16 places a form a body writes, at the macro's body.

### 2.30 A saved file is its program, its modules and one commitment of H

A saved file is one `(program …)`, the `(module …)`s it carries and at most one `(commit …)`, and a load reads it as
the node of its program's lattice the commitment names
([`10-answer-dump.md`](10-answer-dump.md#a-state-file-is-its-program-and-the-choices-made);
[Q-M1f.400](../../../decisions/read-out/Q-M1f.400-a-saved-file-states-the-choices-made-in-one.md),
[Q-M1f.401](../../../decisions/read-out/Q-M1f.401-a-saved-file-carries-its-program-and-modules.md),
[Q-M1f.402](../../../decisions/answer/Q-M1f.402-a-saved-file-is-a-node-of-its-programs-lattice.md)). The three heads
stand only at the top of a file, and a form that breaks their shape, `(commit)`, `(module)` or a `(program …)` holding
one of them, is a parse error ([`01-grammar.md`](01-grammar.md)'s table). Each of the following is refused, one line,
at the command's load-error exit:

| the file | the line |
|---|---|
| a program that holds a wrapper | `kb load error: a program holds a (commit …) at None — (program …), (module …) and (commit …) are a saved file's, which opens with its (program …)` |
| a saved file of another shape: a second `(program …)` or `(commit …)`, a module twice or after the `(commit …)`, any other form | `kb load error: a second (commit …) at None — a saved file is one (program …), then the (module …)s it carries, each once, then at most one (commit …), and nothing else`, the finding first: `a second (module m …)`, `a (module …) after the (commit …)`, `a form that is no (program …), (module …) or (commit …)`, placed where the form is written |
| an import the file carries no module for | `kb load error: (import m) — F carries no module m, and a saved file reads no other file (None)` |
| a choice that is no candidate of the program's H | ``(commit …) names (note (bad b)), which is no candidate of its program's H — a saved file's choices are candidates of H, which `ein show hyps` lists (Q-M1f.400)`` |
| a choice named twice | `(commit …) names (note (bad a)) twice — a commitment is a set of candidates of H, each named once (Q-M1f.400)` |

The two refusals of a choice read H, which is built at root's fixpoint, so they are made by the commands that read
the node, `ein solve` in every mode, `oracle`, `saturate` and `render kb`, as § 2.7's is by the commands that take
the universe; `ein show hyps` and `ein render h-space` read the program's H and refuse neither, `ein test` asks a
node nothing and refuses neither
([Q-M1f.403](../../../decisions/answer/Q-M1f.403-a-saved-file-with-a-commitment-checks-none.md)), and `ein kb save`
stores the choices as the file states them, its container refused by those commands as the file is
([Q-M1f.413](../../../decisions/engine/Q-M1f.413-a-nodes-container-opens-as-the-node.md)). What is **not** refused,
and draws no line: a carried module of the standard library that is not the one the binary would read, or one
carried where the binary has no standard library to read, which is loaded as carried; a saved file compares nothing
([Q-M1f.401](../../../decisions/read-out/Q-M1f.401-a-saved-file-carries-its-program-and-modules.md)).

## See also

- [`05-cli.md`](05-cli.md) — the commands, their options and every exit code.
- [`02-syntax.md`](02-syntax.md) and [`01-grammar.md`](01-grammar.md) — what the parse takes.
- [`03-reserved-names.md`](03-reserved-names.md) — the words no declarator binds.
- [`06-config.md`](06-config.md) — `--no-require-why` and the retired `(config …)` flags.
- [`08-read-out.md`](08-read-out.md) — what a run that loads prints.
- [`docs/decisions/diagnostics/`](../../../decisions/diagnostics/README.md) — the decisions about a refusal's line.
