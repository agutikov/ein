# Ein — defined behaviour

Thirteen behaviours whose only statement, until 2026-08-21, was the source that
implements them. This page is where they are *stated* — and, since M1c
S1c.1.2, where the diagnostics that name no exception class are stated too
([§4.1](#41-the-first-errors-that-name-no-exception-class--m1c-s1c12)).

> **Audience: both.** §1 and §4 are what you meet when a program is wrong;
> §2 and §3 are what a reimplementer needs and a puzzle author can skip.

## Why this page exists

A behaviour defined as *whatever the implementation does* is **undefined**,
and undefined behaviour in a specification repo is worse than a quirk: nobody
can tell a regression from a fix.

So each one is written down here, as ein's own defined behaviour. Two
consequences of that framing are worth being explicit about:

- **These are normative.** A change to any of them is a change to the kernel,
  and needs the same treatment as a change to `(absent …)`: a reason, a
  fixture, and an entry in whatever ledger is current.
- **Several of these are strange in isolation.** The argument for keeping
  them is that they are what every checked-in fixture, golden and example
  output in the repo was baselined against. Where a better behaviour is
  known, it is named below.

## 1. Parse and load diagnostics

**A refused file reports every error recovery reaches**, in file order, one
line each, `{file}:{line}:{col}: unexpected X`, where `X` is the token as
written and, where the token has a name, the name after it (`a name`, `a
variable`, `a keyword this form does not take`, `a token that is not a
name`); a character no token starts with is `no token starts with X`, and
since M1f S1f.5.22 a character that is not ASCII outside a string literal and
a comment is its own line, which names it and where Unicode is allowed
([§ 1.6](#16-a-character-that-is-not-ascii-is-written-only-in-a-string-or-a-comment--m1f-s1f522)). Under
each error come its notes, where a ruling says more than the token does (a
bare `_`, a string as a value, a query's lattice keyword, a retired
`(config …)` flag, and since M1f S1f.5.23 an `open` that is not a rule's
verdict, § 4.17), then at most three ranked repairs, `repair: Insert `)``
or `repair: Delete `a?x``, and `and n more repairs`; a report cut at the
recovery budget, 30 000 steps, ends with a line saying where it stopped.
Every command exits 1 on one (`ein test`: 2, the file an `ERROR` row), and no
repaired program is loaded. Positions are 1-based, lines split at `\n` and
columns counted in characters (`ein_ir::grammar`, M1f S1f.5.5's swap,
2026-11-09).

```text
$ ein solve t.ein               # file is: (p a?x) (q "1")
t.ein:1:4: unexpected `a?x`: a token that is not a name
  repair: Delete `a?x`
t.ein:1:12: unexpected `"1"`: a string
  note: "1" is a string, and a string is natural-language text, written only after :why, :source or :goal-text: write a name bare
  repair: Delete `"1"`
  repair: Insert `:source`
```

**Until the swap** a parse error was one line, `{file}:{line}:{col}:
unexpected input`, and a context block, the source line and a caret, from the
hand-written parser, which stopped at the first error; § 1.1 to § 1.3 were
that block's quirks, and § 1.4 its parser's. Each went at the swap, the
stage's default ([S1f.5.5 § The error
text](../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md#the-error-text),
[Q-M1a.3](../history/m1a_rust/open_questions.md#q-m1a3--parse-error-messages)
option (c)), and each section says what it was.

### 1.1 The end of input is where the text ends

An input that ends while a form is open reports the position past its last
character: the line after a final newline, column 1.

```text
$ ein solve t.ein               # file is: (relation r T and a newline
t.ein:2:1: unexpected end of input
  repair: Insert `)`
```

Since M1f S1f.5.5's T1f.5.5.13 (2026-11-09): `lrpar` makes its end-of-input
token at the end of the last token, and the report said `1:14` for the file
above until then, wherever a newline, a space or a comment followed
(`generated_parser::the_end_of_input_is_where_the_text_ends`). **Until the
swap** it reported line `-1`, column `-1`, and a context block rendering the
file's last line with the caret one past its end, which every
`examples/broken/` fixture was baselined against.

### 1.2 There is no context block

The report names the position and the token, and prints no source line.
**Until the swap** the block took 40 characters either side of the error
position and *then* cut to the current line, so an error past column 40
rendered a truncated line with no ellipsis: `(relation aaa…60 a's… T){`
showed `aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa T){` and a caret under the `{`,
the line's `(relation` cut. The same file reports `long.ein:1:74: unexpected
`{`: a name` since: `{` is a name character (S1f.5.4), and a name where a
form's `)` goes is the error.

### 1.3 The reported column is the token's

An error is placed at the start of the token the parse could not take, or of
the character no token starts with. **Until the swap** it was the first
position at or after the failure with no *pending* whitespace-or-comment
match: whitespace and comments were matched at every position the scanner
passed, inside a string too, so `(y";"{` reported `1:6` and `(y";"{?`
`1:7`, the `;` in the string a comment one character longer
(`parse::death_position`, which went with `parse.rs`). Both report `1:2`
since, at `y"`, one token that is not a name: a token ends only at a space, a
paren, a comment or the end of input ([`00_ebnf.md`
§ 1](ir/03-ein-lang/00_ebnf.md#1-lexical-grammar), fact 5).

### 1.4 No ambiguity, and a tie goes to the earlier rule

`ein.y` has no conflict: the parser's tables are built from it at build time
and a conflict fails the build, so no input has two parses. The lexer's one
choice is between two rules that match the same length, where the rule
written first wins: each reserved word and keyword is written before `NAME`
and `KEYWORD`, so `neq` is the token `'neq'` and `neqw`, the longer match, a
`NAME` ([`00_ebnf.md` § 1](ir/03-ein-lang/00_ebnf.md#1-lexical-grammar),
fact 1). `(relation R A B)` is a declaration and `(relation)` a parse error,
a declaration needing a name.

**Until the swap** the grammar had alternatives the parser tried in order,
the one written first winning: `relation` was written before the generic
fact and was a `SYMBOL` too, so `(relation)` was a fact of `relation`. From
M1e2 S1e2.2.22 (2026-11-05, the user's ruling on
[Q-M1e2.17](../../plans/m1e2_correctness/open_questions.md#q-m1e217--where-does-a-keyword-end))
a reserved word was a keyword only as a whole word, `SYMBOL`'s exclusion a
lookahead with a word boundary, so `rulex` was a symbol where `rule-x` was a
parse error; until then a literal matched as a prefix, and
`(rulex (?a) :match (q ?a) :assert (q ?a))` was a rule named `x` and
`(neqw ?x)` in a rule the guard `(neq w ?x)` (W22). Since the swap `rule-x`
is a name too, the name rule's `-` a name character, which overrides
Q-M1e2.17 (i)(a).

### 1.5 Loader messages about a top-level form end in `at None`

Every error the loader raises interpolates the form's source location, and a
*top-level* form is built without one:

```text
unknown relation `foo` at None
```

**Thirteen of the 35 `examples/broken/load/` fixtures** have a message that
ends this way, and `grep -L 'at None' examples/broken/load/*.expected` names
the twenty-two that do not (2026-11-10, `nested_or` among them since M1f
S1f.5.21, `eq_in_a_macro_body` since S1f.5.24 and `open_compound_witness` since
S1f.5.23; the page said fourteen and twenty-one before S1f.5.23, and 31 and seventeen when it was re-taken on 2026-11-09): the error is raised somewhere that
*has* a location, or nowhere that has one at all — `macro_arity_mismatch`
(inside macro expansion, on a nested node — it prints a real position,
`<file>:6:20`, which was the Python IR's `Loc(file=…, line=6, col=20)` until
M1f S1f.5.21 took that half of F23's D-2, Q-M1f.75), `nested_or` (since
S1f.5.21, § 4.15), `eq_in_a_macro_body` (since S1f.5.24, § 4.16, whose line
starts with the macro body's position), `open_compound_witness` (since
S1f.5.23, § 4.2, whose line starts with where its `(open ?R)` is written), `unbound_head`, `derivation_cycle`, `unimported_std_macro`,
the two `import_*` that name no module found, the two `expect_*` M1c added,
the solve, query and region claims' five, and the four `reserved_open_*`,
whose import of `reserved_open_direct.ein` is refused at its parse since M1f
S1f.5.5's swap, and `reserved_open_symbols` at its own `:symbols (open)` since
S1f.5.23, `open` being no name. On 2026-08-31 it was 11 of 41; the swap moved twenty-one of
them to `examples/broken/`, each refused at the parse since.

*Why.* [Q-M1a.6](../history/m1a_rust/open_questions.md#q-m1a6--at-none-in-loader-messages)
— a genuine usability bug, kept deliberately so that fixing it is one
visible re-baseline rather than noise. It is the clearest
candidate on this page for being **fixed** rather than documented; what it
costs is re-blessing every message in that directory at once.

### 1.6 A character that is not ASCII is written only in a string or a comment — M1f S1f.5.22

A character that is not ASCII is written only inside a string literal, after
`:why`, `:source` or `:goal-text`, or in a `;` or `#| … |#` comment (the
user's ruling of 2026-11-10, *"Unicode is not allowed anywhere else than
inside string literals, and the error must say it explicitely"*, and
[Q-M1f.65](../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f65--may-a-comment-hold-unicode)
(b), *"Comments may hold Unicode"*). Anywhere else no lexer rule matches it,
the lexer stops there, and the file is refused with one line at the
character, on every command (exit 1, `ein test` 2, `kb save` writing no
container):

```text
$ ein solve t.ein               # file is: (p café)
t.ein:1:7: 'é' (U+00E9) is not ASCII: Unicode is allowed only inside a string literal (after :why, :source or :goal-text) or in a comment
```

- **The character, quoted, and its code point**, `'é' (U+00E9)`, `'🦓'
  (U+1F993)`; one a reader cannot see where it stands, a space, a control, a
  zero-width or bidirectional format character, a combining mark or a
  variation selector, as the escape a string writes it with, `'\u{a0}'
  (U+00A0)`, `'\u{202e}' (U+202E)`
  ([Q-M1f.92](../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f92--how-the-line-quotes-a-character-a-reader-cannot-see),
  `ein_ir::grammar::not_ascii`).
- **One line, at the file's first such character**: `lrpar` lexes a file
  before it parses it and stops at the first lexing error, so no parse error
  and no repair follows
  ([Q-M1f.95](../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f95--how-many-characters-the-refusal-names)).
  A `#` before one is passed over: no rule starts at that `#`, and the line
  is about the character.
- **Every place**: a name, a variable, a keyword, a declared relation's name
  or type, a nested fact, a rule, an hrule, a macro's name or body, a query's
  goal, `:over` or claim, a `(solve …)` list, a `(config …)` flag, a trace
  step, an import's module, between two tokens, a line's or the file's first
  character (a byte-order mark among them,
  [Q-M1f.96](../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f96--a-byte-order-mark)),
  in place of a paren, glued after a string, and in an imported module, whose
  line names the module's resolved path. The grammar's table holds a row per
  place (`grammar_pages`), and `ein-cli/tests/unicode_only_in_a_string.rs`
  every place on twenty commands, and twelve characters in every place
  through the parser.
- **Not refused**: any character inside a string literal, whatever the string
  is written as (a string where a value goes is refused as one, § 4.12, and an
  escape it does not read as one, at its quote), and in a comment. A
  byte that is not UTF-8 is refused before the text is read (§ 4).

**Until S1f.5.22** a character that is not ASCII glued to a token made it one
token that is not a name, *`unexpected `café`: a token that is not a name`*
and a repair, and the skip rule's whitespace was `\s`, Unicode's, so a
no-break space, U+3000, U+2028 and U+0085 were separators: a program writing
one between two tokens loaded and was answered as if it were a space, and one
inside what reads as one name, `k`, U+00A0, `j`, was two names (W73, five rows,
[`corpus/wrong_answers.toml`](../../corpus/wrong_answers.toml)). The census
of T1f.5.22.1 found no corpus file writing one outside a string or a comment
(454 files, 2026-11-10), so no corpus answer moved.

## 2. Values and order

<!-- The heading read `Int < Sym < Fact` until M1f S1f.5.4, and the history cites its anchor. -->
<a id="21-fact-arguments-are-totally-ordered-int--sym--fact"></a>

### 2.1 Fact arguments are totally ordered: `Sym < Fact`

Within a tag, symbols order by **lexicographic rank** — position in the sorted
symbol list, never the order the interner assigned ids in — and facts by
relation name, then arguments element-wise, then arity, so a shorter argument
tuple that is a prefix of a longer one sorts first. A run of digits is a
symbol, so `(p 10)` sorts before `(p 2)`. Until M1f S1f.5.4 retired the int
kind (T1f.5.4.3, 2026-11-09) the order was `Int < Sym < Fact`, integers by
numeric value at any width; since S1f.5.5's swap no program could write one.

*Why it is stated.* The search layer sorts its alive set, so a puzzle with
mixed-kind arguments in one relation slot, or with an `(hrule … :assert (not
…))` head, puts a symbol beside a fact, or two facts, in one sort — and the
cross-tag half of this order is consulted exactly there.
`Terms::cmp_semantic` is the order, and
[`examples/ein-bugs/mixed-type-hypothesis.ein`](../../examples/ein-bugs/mixed-type-hypothesis.ein)
and
[`nested-fact-hypothesis.ein`](../../examples/ein-bugs/nested-fact-hypothesis.ein)
are two puzzles that reach it.

### 2.2 Where a value is sorted for display, it is sorted by its `repr`

State-dump node order and explanation tie-breaks go through one `repr` for the
three shapes a fact argument can take (four until M1f S1f.5.4 retired the
int), and so do the shape digests and the
diagnostics that quote a value Python's way (`KeyError: '…'`). Where a value
is **printed** on a surface, the read-out, a dump, the trace, the events or a
DOT label, it is printed as Ein text since M1e2 S1e2.3.8 (`ein_core::text`): a
name bare where it lexes as one `SYMBOL` and quoted in `"` elsewhere, `\\` `\"`
`\n` `\t` `\r` escaped, a nested fact as `(rel …)` (and an integer as its
canonical decimal, until M1f S1f.5.4), so the text loads back to the same value
(`grammar::the_printers_symbol_is_the_lexers`, `answer_round_trip.rs`). Since
M1f S1f.5.5's swap a name a program writes always lexes as one, a string being
natural-language text and no value (S1f.5.2, § 4.12), so a name printed quoted is
one the loader made up, which does not load back: `"#empty"` was one, which a run
reached through `:assert ()` until M1f S1f.5.9 made the empty conclusion assert
nothing (W34), and no run reaches one since. Until then
the DOT labels printed a nested fact as its `repr` and a name as its raw text.
The `repr`:

- a **string** is quoted in `'`, unless it contains a `'` and no `"`, in which
  case in `"`. The chosen quote and `\` are backslash-escaped; tab, newline and
  carriage return are `\t` `\n` `\r`; any other non-printable code point is
  `\xNN` below U+0100, `\uNNNN` below U+10000 and `\UNNNNNNNN` above, in
  lower-case hex;
- a run of digits is a **string** like any other name, `'7'` (an
  **integer**, its canonical decimal text bare, until M1f S1f.5.4);
- a **tuple** is `(a, b)` — one item is `(a,)`, none is `()`;
- a nested **fact** is `Fact(relation_name='p', args=('A',))`.

*Non-printable* is by Unicode **general category** — `Cc` `Cf` `Cs` `Co` `Cn`
`Zl` `Zp` `Zs`, ASCII space excepted — not by `is_control()`:
`ein-core/src/printable.rs` is generated from the Unicode Character Database
(737 ranges, Unicode 16.0.0) by
[`utils/gen_unicode_printable.py`](../../utils/gen_unicode_printable.py). Run
it after a Unicode upgrade; a category that moves surfaces as a changed row of
`ein-core/tests/format_tables.rs`'s `repr_escapes.txt`.

Until M1f S1f.5.4 integers were carried as canonical decimal **text**, not
as an `i64`, since the grammar accepted `-?[0-9]+` at any width. Since
S1f.5.5's swap a run of digits is a name, kept as written, so there is no
width and no canonical form: `7`, `07` and `-7` are three names.

### 2.3 Formatted floats follow one fixed-point spec

`--hyp-stats` percentages, `--timing` columns and `--stats` elapsed times are
`[[fill]align][sign][0][width][.precision]f`: `precision` digits after the
point, six when none is given, rounded from the exact binary value with ties
to even. Three points where the obvious Rust spelling is wrong: NaN prints
`nan` (not `NaN`), a NaN never carries a `-` while an infinity does, and an
**empty** spec is not `.6f` — the `f` is required. Anything outside that
subset is rejected rather than guessed at.

### 2.4 Canonical state identity is the sorted fact list itself, never a hash

Two commitment-set branches that saturate to the same closed KB collapse to
one lattice node, and the key that decides this is the **representation**: a
collision costs a comparison, not a wrong answer. `state_digest` exists only
for display and is never identity.

Any total order over facts serves for identity, so the key sorts by interned
id. Where the key is *displayed* — `--dump-states` orders its nodes — the
`repr` order of §2.2 is used instead, because that is the order every
checked-in dump was baselined against.

### 2.5 A render view labels an IR form as written

The DOT views that draw the program rather than a KB — `render constraints`,
`render program`, `render rules` — label an IR value with
`ein_render::value_label`, which recurses into a nested form. A value holds no
keyword pair since M1f S1f.5.5's swap (S1f.5.11's ruling: a term and a nested
fact take no keyword), and a form's own pairs are its clauses, which each view
reads or filters before it labels the form's arguments; the `(query …)` and
`(solve …)` note nodes give their keywords as `:key value`. What is defined is
that no keyword pair reaches a value's label, and no view aborts on a program
the parser accepts.

**From M1e2 S1e2.3.2 to M1f S1f.5.11**, a keyword pair below the form a view
drew was labelled `:key value`, so `(tracex (step s1 :rule from-condition …))`
drew its step as `(step s1 :rule from-condition :using (c10) :derives
(lives-in N H1))`; until S1e2.3.2 one aborted the process, exit 101 on both
`render constraints` and `render program`, on a program the loader accepted.
The user ruled that rendering on 2026-11-03 (option (a) of
[the stage's question](../../plans/m1e2_correctness/p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.2_a_keyword_pair_panics_two_views.md#the-question)),
against filtering the pairs at every depth. S1f.5.5's swap refused the witness,
`examples/ein-bugs/kwpair-below-the-filter.ein`, at the parse, and it moved to
[`examples/broken/kwpair-below-the-filter.ein`](../../examples/broken/kwpair-below-the-filter.ein);
S1f.5.11's T1f.5.11.2 retired `value_label`'s arm for a pair, shown unreachable
first, an `unreachable!` naming the grammar's productions. Held by
`dot_util::tests::no_keyword_pair_reaches_a_values_label` and by the grammar's
table, whose rows refuse a keyword pair in each place in a term
(`ein-ir/tests/grammar_pages.rs`).

## 3. Search

### 3.1 `--shuffle` is the Mersenne Twister, exactly

`--shuffle` (`-z`), with `--seed` (`-d`) pinning the permutation, seeds
MT19937 by the standard `init_by_array` over the seed's absolute value, split
into 32-bit words least significant first — and shuffles each layer with a
downward Fisher–Yates, swapping position `i` with `randbelow(i + 1)` for `i`
from the last index down to 1, where `randbelow(n)` takes the top
`bit_length(n)` bits of a draw and rejects until the result is below `n`.
Generator state carries across layers.

The absolute value is observable: `--seed=-7` and `--seed 7` are **the same
generator**, and two exhaustive runs of one puzzle under them produce
`summary.json` files that differ in exactly one field — the seed the run
recorded.

*Why fix it exactly.* Shuffle-invariance is a property the engine
claims: a `--shuffle` run must reach the same verdict, `k` and models as an
unshuffled one. Fixing the exact generator is what lets a shuffled run be
compared against a recorded one at all, and `--shuffle` runs are precisely the
ones where a silent ordering difference would be easiest to dismiss.

### 3.2 A rule application's identity is its activator's every argument

> **Fixed 2026-11-04** — M1e2
> [S1e2.2.7](../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.7_binding_key_layout.md)
> (W7), which took
> [Q-M1e.16](../history/m1e_review_processing/open_questions.md#q-m1e16--the-binding-key-compares-two-register-layouts-as-one)'s
> first fix. Until then this section stated a **preserved defect**, the one
> item on this page that was a latent bug rather than a quirk: the identity
> kept an activator's *symbol* arguments only, and a program that put an
> integer and a nested `Fact` in one activator position lost a derivation in
> a release build and tripped an assertion in a debug one.
>
> **Corrected 2026-08-29** before that — M1e
> [S1e.1.4](../history/m1e_review_processing/README.md#s1e14--q-m1a8s-real-trigger-q3),
> the review's `Q3`. Until then this section said *integer*, and the engine
> had never done what it described; the probe that refuted it is
> `ein-infer/tests/rule_semantics.rs::activators_differing_only_by_an_int_argument_both_fire`.

The identity of a firing is `(rule, activator, bindings)`, and three keys read
one activator:

| key | what it keeps of the activator |
|---|---|
| the compile cache key | **every** argument, as a value |
| the identity's activator half | the same list: **every** argument |
| the identity's bindings half (the plan's register file) | every argument that **binds a parameter** — **every** argument since M1f S1f.5.18, a nested fact as a name; every name (and every integer, until M1f S1f.5.4 retired the int) until then |

So two activators of one rule that differ in **any** argument have different
identities, and both fire. **"Differ" is by value since M1e2 S1e2.2.16**
(2026-11-04). Until then the compile key rendered every argument that is not
a symbol and interned its text, so the int `1` and the string `"1"`, two
values the loader keeps apart, gave one key and one plan: of the activators
`(note edge 1)` and `(note edge "1")` the second compiled nothing, its two
conclusions were lost in a release build, whichever came second, and a
debug build aborted in `Engine::compile_all`'s check that its shortcut
skipped no activator (W16;
`examples/ein-bugs/activator-text-int-and-string.ein` and its swapped twin,
`rule_semantics.rs::an_int_and_a_string_with_one_text_are_two_activators`).
`compile.rs` called it *the one collision* and kept it. A string and an atom
with one text were one value, a symbol, so they were one key, as they were one
fact, until M1f S1f.5.5's swap made a string natural-language text, no value
(S1f.5.2, § 4.12). A name reaches both halves, since it binds its
parameter, as an integer did until M1f S1f.5.4, and so does a nested
**`Fact`** since M1f [S1f.5.18](../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.18_an_activator_binds_a_nested_fact.md)
(2026-11-09, W33, the user's ruling of 2026-11-08 on S1f.5.5's census): an
activator binds a parameter to its argument whatever value it is, so `(act (f
a))` binds `?y` to `(f a)` and `(p ?y)` matches `(p (f a))` alone
(`compile_semantics.rs::a_value_parameter_an_activator_binds_to_a_nested_fact_is_bound`).
Until then a nested `Fact` bound nothing and reached the activator half alone:
its parameter was a free variable of the plan, so the rule fired on every `p`
(W33), and a conclusion that alone read the parameter stopped the run with
§ 4's retired `KeyError` (W58). Two activators differing only in a nested `Fact`
compiled to plans equal in every field but that list, and a rule that reads the
parameter nowhere still concludes the same from each: a **redundant** firing,
one more firing and nothing derived differently
(`rule_semantics.rs::activators_differing_only_by_a_nested_fact_argument_both_fire`).
`Plan::activator_args`, what an event and a static check's label print, is the
same every argument since S1f.5.18 (Q-M1f.43), where it kept the names alone.

**Why the activator half is the whole list.** A plan's register layout is a
function of the rule and of *which* parameters its activator bound, and a
name bound its parameter where a nested `Fact` did not until M1f S1f.5.18, as
an integer did until M1f S1f.5.4. While the
activator half kept the symbols alone, an integer in the position another
activator gives a nested `Fact` put two plans with two *register layouts* in
one identity space — `?f` is register 1 in one and register 3 in the other —
so the identity compared `(?R ?f ?a ?b)` against `(?R ?a ?b ?f)` position by
position. A vector that is a legitimate match of both then suppressed a
firing whose conclusion nothing else derives:

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

(note edge 1)                     ; an int in the second position (a name since M1f S1f.5.4)
(note edge (src Y))               ; a Fact  in the second position
```

```text
$ ein saturate examples/ein-bugs/binding-key-both.ein --dump    # until S1e2.2.7, release
  (noted 1 1 :rule note)
  (noted 2 1 :rule note)
  (noted 2 3 :rule note)

$ ein saturate examples/ein-bugs/binding-key-both.ein --dump    # since, either build
  (noted 1 1 :rule note)
  (noted 2 1 :rule note)
  (noted 1 3 :rule note)          ← lost above, silently
  (noted 2 3 :rule note)

$ ein saturate examples/ein-bugs/binding-key-both.ein --dump    # since M1f S1f.5.18
  (noted 1 1 :rule note)
  (noted 2 1 :rule note)
```

The last is W33's fix and not a lost derivation: `(note edge (src Y))` binds `?f`
to `(src Y)`, which no fact `holds`, so it derives nothing, where until then `?f`
was `(holds ?f)`'s to bind and the `Fact` activator derived every `(noted ?a
?f)`, which the second block shows; the `int` (a name since S1f.5.4)
activator's two are the whole answer.

Adding an activator **removed** a conclusion, swapping the two lines put it
back, and `Engine::check_layout`, which asserts the invariant under
`debug_assertions`, aborted a debug build on both orders. Since the
activator half is the compile key's list, two plans of one rule with one
activator half have one compile key and are one plan, so the identity never
compares two layouts; `check_layout` stays, as the guard that the two keys
have not parted again.

**Where it is held.** The program is the corpus family
`examples/ein-bugs/binding-key-{both,swapped,int-only,nested-only}.ein`: both
activators in either order, and each alone. Each file's `:expect` states
every state's `noted` facts (restated by M1f S1f.5.18, whose `:expect` stated
W33's wrong answer, and `nested-only`'s by a query, no model naming `noted`
since), so `ein test` checks them in the corpus sweep,
which runs a debug build, and the order-swap pair answers alike.
`rule_semantics.rs::an_int_beside_a_nested_fact_in_one_position_loses_no_derivation`
states the same in-process, in both profiles. No other corpus program has an
activator argument that is not a symbol: of the 10 744 plans compiled under
`ein solve`, `ein solve --single`, `ein oracle` and `ein saturate` over every
corpus file, 92 have a compile key that differs from their symbol list, and
every one is on W7's files (2026-11-04, an instrumented build, S1e2.2.7's
T1), so the fix moved no other answer.

### 3.3 The M1 alive-set invariant, operationally

> **New 2026-08-31** — M1e
> [S1e.3.3](../history/m1e_review_processing/README.md#s1e33--state-model-medium),
> the review's `ST-M1`. The invariant is stated in
> [`inference/README.md` § M1 invariant](inference/README.md); until this
> stage nothing evaluated it, and the three mechanisms that cite it as their
> warrant — the per-KB `alive` recompute, the `state_key` dedup that produces
> `k`, and M1d's tree exhaustiveness-by-discharge — rested on the stdlib's
> conventions alone.
>
> **Restated 2026-11-04** — M1e2
> [S1e2.2.4](../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.4_fresh_name_unreachable.md).
> A constant a rule's `:assert` names is a **name of the program**, not a new
> one: the user's ruling of 2026-10-16 on
> [Q-M1e.21](../history/m1e_review_processing/open_questions.md#q-m1e21--a-rule-may-name-an-object-the-search-can-never-hypothesise-about),
> (b), *"rules are part of root, so rules can't assert new names"*. The
> universe is seeded with every such constant, and the baseline below counts
> them. Until then this section read clause 1 against the facts' names alone,
> and *What a breach cost* is the program on which that cost a model.
>
> **Widened 2026-11-04** — M1e2
> [S1e2.2.14](../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.14_unread_names.md).
> The user ruled the same day that a name is any constant the program writes
> in a fact, at any depth, or anywhere in a rule, `:match` included, and not
> one only a query writes
> ([Q-M1e2.9](../../plans/m1e2_correctness/open_questions.md#q-m1e29--which-constants-are-names-of-the-program)
> (a)). R0 below is that list; a name only a `:match` or a nested fact wrote
> was missing from the candidates until then (M1e2's W15).
>
> **A string, 2026-11-04** — M1e2
> [S1e2.2.18](../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.18_strings_in_rules.md).
> A string a rule writes is the symbol it names, as a fact's is, so it is a
> name of the program as that atom would be: `lbl` in `(tag ?x "lbl")`. The
> compiler did not lower a string until then, and the names list did not
> read one (M1e2's W18). A string that is a keyword's value inside a clause is
> text, and not a name.

The prose form is three clauses about a rule library. The evaluable form has
to name the sets and the moment, and there are **three readings**, ordered by
strength:

| | claim | how it is checked |
|---|---|---|
| **R0** | every name of the program — every constant a fact names at any depth, or a rule's `:match` or `:assert` names — is among the blind enumerator's objects, whether or not the names index holds it | statically, at load, over the facts and the registered rules: [`ein_infer::invariant::program_names`](../../ein.rs/crates/ein-infer/src/invariant.rs) is the one list, and `hypgen::candidate_objects` reads it |
| **R1** | nothing a run *derives* names what the loaded program — its facts and its rules' constants — did not | a post-fixpoint scan, `Universe::breaches` |
| **R2** | nothing the **search** reaches names what **root's fixpoint** and the rules' constants did not | the same scan, against a baseline taken at root's fixpoint |

**R2 is the one the engine's soundness rests on**, because the universe H is
taken once, at root's fixpoint
([`solution_semantics.md` § 1.4](inference/solution_semantics.md#14-candidate-and-universe)),
and every route reads subsets of it. A name that arrived later would extend
the blind enumerator's candidate-object pool into a region no route revisits.
**R0 is what makes R2 hold for a rule's constant**: the pool is
`kb.names()`, a function of the KB's facts' top level, **∪ every name of the
program** (the facts' arguments at any depth, the rules' `:match` and
`:assert` constants), so a constant that a rule asserts only inside a
commitment is an object from root on. It is free and total: it reads the rules once, so it
answers for *every run the program could have* rather than for the one that
happened, and the same read cost **7 µs** on `examples/zebra2.ein` (84 facts,
30 rules) when S1e.3.3 measured it. A *variable* leaf contributes nothing,
because a variable can only carry a value the match already found in the KB —
and that induction is confirmed rather than assumed: swept over every state a
bounded describe meets on the corpus, the scan finds **nothing**, against
either baseline (`ein-infer/tests/alive_invariant.rs`). The one shape outside
the induction is a relation head written as a parameter and bound to a value
that is not a name, a nested fact, and it is **refused**: by the loader where
the activator is written down, by the compiler where a rule derives it
([§ 4.9](#49-a-rule-shape-that-reads-nothing--m1e2-s1e2224)). Until M1f
S1f.5.4 an integer was the other such value, whose decimal text the compiler
would have interned as a name, refused the same two ways
([§ 4.6](#46-a-relation-named-by-a-number--m1e2-s1e224)); since S1f.5.5's swap
a run of digits is a name of the program. No corpus program has either.

**The baseline.** *Everything the loaded program reaches*, not the interner's
length. `Ann` in `examples/ein-bugs/mixed-type-hypothesis.ein` is interned at
**load** by `from_ir::intern_program_names`, so the interner never grows
during the search (`ein-infer/tests/interning.rs` measures exactly that); what
makes `Ann` a name of the program is that a rule names it. So the baseline is:
every symbol the loaded facts mention at any depth, every constant a rule's
`:match` or `:assert` names, plus the relation registry's names and the
symbols in their signatures.

**What counts as a new relation.** The registry — *declared ∪ auto-vivified* —
**∪ every constant head a rule can `:assert`**, and the widening is not a
convenience. `from_ir` vivifies an undeclared **fact** head at load and does
not vivify an undeclared **rule-`:assert`** head, so **49** relation names over
33 corpus files have facts and no registry entry — all but one a stdlib
activator (`total` derived from `(bijective …)`, `slot-endpoint-fwd`,
`converse-illtyped-dom`, …; the exception is a puzzle's own
`explicitly-dislikes`). Reading the registry as the whole answer would make the
check fire on the standard library on its first run. Nothing can add
a relation *during* a run in any case: `Program` is an `Arc` every fork shares
and `program_mut` panics once it is shared, so clause 2 is closed by the type
system and this check is about the *facts*, not about the registry.

**What a breach cost, until the seed.** Three corpus programs name a constant
in a rule and in no fact: `mixed-type-hypothesis.ein` (`Ann`, from an
`hrule`, whose rung builds that program's H), `tests/stdlib/algebra/07_schroder.ein`
(`G`, from a probe rule that fires in root's saturation) and
`examples/ein-bugs/wrong-answers/fresh-name-default.ein` (`Z`). The first two
cost nothing. On the third, and on
[`examples/ein-bugs/alive-set-fresh-name.ein`](../../examples/ein-bugs/alive-set-fresh-name.ein),
the same program, `spawn` names `Z` only on a committed `q`, so no candidate
was about `Z`: the walk reported `k = 0, exhausted = true`, *No solution — the
constraints are contradictory*, and the region R = {∅}, where
`{(q A Z), (q B Z)}` is the maximal state (M1e2's W4,
[`corpus/wrong_answers.toml`](../../corpus/wrong_answers.toml)). Its twin
`…-declared.ein` is the same file plus the single fact `(seen Z)`, which
changes no rule and no constraint and does nothing but put the name in a fact;
it reached that state. Since S1e2.2.4 the two describe one R, four states
(`alive_invariant.rs::the_pair_answers_one_region`).

**What is reported, and what is not.** Nothing, since S1e2.2.4. From S1e.3.3
until then a `warn` event, category `alive-set-invariant`, narrated each
constant only a rule names, at root under `--events`; under the ruling none of
them is new, so the line would say something false, and it left. The set is
named by a test (`alive_invariant.rs::every_name_of_the_program_is_a_candidate_object`:
46 names on 25 programs that the names index does not hold, 2026-11-04, three
of them W14's witnesses, banked by S1e2.2.15, three W7's family, banked by
S1e2.2.7, one W16's `activator-all-nested.ein`, banked by S1e2.2.16, and two
W18's `lbl`, a string a rule writes, banked by S1e2.2.18; it was `Ann`, `G`
and `Z` until S1e2.2.14), and `ein solve FILE -H` lists the
candidates the seed makes.
The F5 typed form is what would make a violation unrepresentable rather than
detected.

## 4. Errors and exit codes

The engine reports a failure it cannot diagnose by printing one line that
names the failure's class, and exiting non-zero:

| input | last line | exit |
|---|---|---|
| missing input file | `FileNotFoundError: [Errno 2] No such file or directory: '<path>'` (`ein saturate` printed `error: <path> not found` until M1f T1f.5.5.13) | 1 |
| an input file that is neither UTF-8 text nor a `.einb` container | `<path>:<line>:<col>: byte 0x<hh> is not UTF-8: a program is UTF-8 text, or a .einb container`, at its first byte that is not, since M1f T1f.5.5.13; `UnicodeDecodeError: 'utf-8' codec can't decode bytes in '<path>'` until then, Python's spelling | 1 (`ein test`: 2, the file an `ERROR` row) |
| an imported module that exists and cannot be read as text: not UTF-8, unreadable, a directory, a `.einb` container | `kb load error: ` and the line its path gives as a program, naming the path the import resolves to, `kb load error: m.ein:3:4: byte 0xff is not UTF-8: …`, `kb load error: IsADirectoryError: [Errno 21] Is a directory: 'm.ein'`; a container `kb load error: m.ein: a .einb container, and an imported module is Ein text: import the .ein it was saved from`; since M1f S1f.5.22 (§ 4.18), `kb load error: (import m) — module not found at m.ein (None)` until then | 1 (`ein test`: 2) |
| a rule that will not compile: since M1f S1f.5.24 none, every refusal the compiler made a program reach being the loader's (§ 4.15, § 4.16); from S1f.5.21 until then only a macro's body that wrote `eq` or `absent` at another arity (§ 4.3) | `kb load error: <file>:<line>:<col>: rule '<r>': its :match writes (eq ?x), and eq takes exactly 2 arguments, not 1 — …` since S1f.5.24, on every command, the views and `kb save` among them (§ 4.16); the same line bare, on every command that compiled, from S1f.5.21; `ein.inference.compile.CompileError: …` and the IR's repr until then | 1 |
| a conclusion no fact can hold: since M1f S1f.5.24 none at a firing, a macro's body that is a bare parameter, called in an `:assert`, being refused at load (§ 4.16); from S1f.5.5's swap until then only that, the grammar refusing every other shape at the parse (a keyword pair, a `_`, a name, a variable or a string where a conclusion or its argument goes) | `kb load error: <file>:<line>:<col>: rule '<r>': its :assert concludes ?y, which is not a fact — …` since S1f.5.24; `rule '<r>': its :assert …` at the firing until then, naming no class (below) | 1 |
| an argument the CLI rejects | `error: invalid value 'x' for '--solutions <N>': invalid int value: 'x'` | 2 |

**`KeyError` left the table at M1f
[S1f.5.18](../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.18_an_activator_binds_a_nested_fact.md)** (2026-11-09, Q-M1f.37):
`KeyError: "unbound var ?y in :assert — bindings: {…}"`, exit 1, its message
printed as its `repr` (§2.2), quotes and all, where a conclusion read a variable
its match left unbound. Since M1f S1f.5.10 a conclusion variable no premise or
parameter binds is refused at load (§ 4.13), and since S1f.5.18 a parameter its
activator binds to a nested fact is bound (§ 3.2, W33 and W58), so no input
reaches it, and `ein_infer::firing::FireError::UnboundVar` is gone.

**Every command reads a program through one path since M1f S1f.5.5's
T1f.5.5.13** (2026-11-09, `ein-cli`'s `common::read_program`, `ein_ir::read`'s
since M1f S1f.5.22, which reads every imported module too, § 4.18), so a file that
cannot be read, or is not UTF-8, is refused with the same line by `solve`,
`saturate`, `oracle`, `test`, each `render` view and `kb save`, at each
command's load-error exit. Until then the read exited the process on either,
exit 1, so `ein test` stopped its whole selection at the first such file and
reported a refuted claim's code; it counts the file an `ERROR` and goes on
since (`ein-cli/tests/program_read.rs`).

**A conclusion no fact can hold names no class since 2026-11-04** (M1e2
[S1e2.2.18](../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.18_strings_in_rules.md)).
Its row read *`:assert` head that is not a fact*, `TypeError: <message>`:
`ein saturate` printed `TypeError: expected NestedPattern at :assert
top-level, got Atom` (or `Var`, `SForm`) and `TypeError: an unrecognised
`:assert` slot has no value to store`, and `ein solve` the message alone, a
Python class, the Python IR's class names, and no word of which rule. It is
one line from every command now, the rule and the slot as the program writes
them, in three forms:

- *A record since S1f.5.5's T5:* `rule 's': its :assert writes _ as an
  argument of tag, and no fact can hold it — an argument is a name, a string,
  an int, a variable its :match binds, or a fact (relation argument …)` (a
  wildcard `_`, a keyword pair such
  as `:source "x"`, at the conclusion's top level; a range until M1e2
  S1e2.2.20, which refused one in a rule at load, and M1f S1f.5.3, which
  retired the token, § 4.8; a keyword pair inside a nested conclusion until
  S1e2.2.24, § 4.9; no program reaches the line since M1f S1f.5.5's T5
  deleted it, the grammar's `Term` holding a name, a variable or a fact, and
  the *string* and the *int* it lists are no value since S1f.5.5's swap,
  S1f.5.2's and S1f.5.4's rulings);
- `rule 's': its :assert concludes foo, which is not a fact — a conclusion is
  a fact (relation argument …), or an (and …) of facts` (an atom or a
  variable where a fact belongs, through a macro whose body is a bare
  parameter from M1f S1f.5.5's swap until S1f.5.24, which refuses it at load
  with the same words, its position first, § 4.16, so no program reaches the
  firing's line since; a string too until the swap, which refuses one there,
  § 4.12);
- `rule 's': its :assert writes (?x a b), whose relation ?x its activator
  does not bind — a relation written as a variable is one of the rule's
  parameters, which its activator binds to a name`, which no program the
  loader accepts reaches since M1e2 S1e2.2.24: a relation variable no
  parameter binds is refused at load, and a parameter bound to a nested fact
  at load or by the compiler (§ 4.9).

The run still stops at the rule's first firing, exit 1, so a rule that never
fires is never checked; an hrule's template that builds no fact still
proposes nothing, in silence. `ein-cli/tests/cli_semantics.rs`'s
`a_conclusion_no_fact_can_hold_is_one_line_in_eins_spelling` holds the five
shapes that reached the first line until M1f S1f.5.5's swap, each a parse
error since, under `ein saturate` and `ein solve` (four shapes until the swap,
eight until M1e2 S1e2.2.20, the range among them, and seven until S1e2.2.24,
a nested keyword pair and the two relation variables among them), and
`ein-infer`'s `compile_semantics::the_lines_a_macro_body_reaches_are_the_loaders`
the second, which a macro reached until S1f.5.24 and the loader refuses since. A **string** reached this line
until the same stage, which lowered a string in a rule as the loader lowers
one in a fact, the symbol it names (M1e2's W18;
[`01_grammar.md`'s `STRING` row](ir/03-ein-lang/01_grammar.md)).

The last row is the odd one out and belongs to §5 rather than here: it is
`clap`'s wording, and what is fixed about it is the **exit code and the
stream**, which is what a script can depend on.

**The step limit left this table on 2026-11-03** (M1e2
[S1e2.3.7](../../plans/m1e2_correctness/p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.7_four_lines_that_misname.md),
N3, on the user's ruling of that day). Its row read *`--max-firings` budget
exhausted*, `ein.inference.saturator.SaturatorStepLimitError: <message>`,
exit 1. No command spells `--max-firings`. No command reaches that error
either: it is the library's own budget on `Saturator::saturate`, and `ein
saturate` drives the saturator one step at a time. The binary's limit is `ein
saturate --max-steps N`. It prints one line on stdout and exits **0**, because
a KB stopped at N firings is what the flag asks for, not a failure. So it is
§ 5's, [*`ein saturate --max-steps N` stops at N
firings*](#5-the-cli-surface).

The class names are the strongest candidate on this page for deliberate
change. Two changed in M1e2: the step limit's row left the table (S1e2.3.7),
and `TypeError` gave way to the rule's own words (S1e2.2.18). Of the two left
then, `ein.inference.compile.CompileError` went at M1f
[S1f.5.21](../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.21_a_compile_error_in_eins_words.md)
(the close review's finding 1): a compile refusal is one line in Ein's words on
every command, and the refusals a program reached are the loader's (§ 4.15).
`FileNotFoundError` is the one Python spelling a command still prints, from
the read path and not the compiler, and it is
[F23](../../plans/followups/f23_diagnostics_in_eins_words.md)'s D-1
(`KeyError` was another until M1f S1f.5.18 retired its line); S1e2.3.7's note
names the ones no command prints.

### 4.1 The first errors that name no exception class — M1c S1c.1.2

Five arrived on 2026-08-23 with
[`:expect`](ir/03-ein-lang/01_grammar.md#query), and what they decided is:
**a diagnostic not in the table above names no exception class.** It is a
`kb load error:` like any other loader message, in ein's own words.

| input | message | exit |
|---|---|---|
| a `(query …)` keyword it does not take | a parse error at the keyword, `<file>:<l>:<c>: unexpected `:<k>`: a keyword this form does not take` † | 1 |
| a lattice keyword on a `(query …)` (M1e2 S1e2.1a.4) | a parse error at the keyword, its note `:<k> chooses the lattice and belongs on (solve …) since M1e2 S1e2.1a.4 — one program, one lattice` † | 1 |
| a `(solve …)` keyword it does not take | a parse error at the keyword † | 1 |
| more than one `(solve …)` | `<n> (solve …) forms — a program states its lattice once` | 1 |
| a top-level `(region …)`, `(merged …)` or `(owing …)` | a parse error at the word † | 1 |
| a `(solve … :expect …)` that is not `(false)` or `(region …)`, or a region item that is not `(model …)`, `(merged …)` or `(owing …)` of their shapes | a parse error at the first token that is not † | 1 |
| `(owing …)` beside states of R | `` (region …) — `(owing …)` says R is empty, and the region lists states `` | 1 |
| a `(merged …)` whose `<to>` is not a listed state | `(merged …) — its state is not one the region lists` | 1 |
| a claim naming a relation the program does not have (§ 4.21; one only a rule's `:assert` derives is the program's since M1f S1f.7.11) | `(solve … :expect …) names <r>, and the program names no relation <r> — a relation of the program is one it declares, states a fact of, or writes in a rule or an hrule` | 1 |
| a claim naming `eq` or `neq` (M1f S1f.5.7) | `(solve … :expect …) names <r>, which a state never stores — eq and neq are computed from their definition, so a region claim, which lists a state's facts, holds neither` | 1 |
| `:expect` that is not `(false)` / `(model …)` / `(or (model …) …)` | a parse error at the first token that is not † | 1 |
| `:expect` naming a relation the program does not have (`eq` and `neq`, which a goal may ask about, excepted since M1f S1f.5.7) | `:expect names <r>, and the program names no relation <r> — a relation of the program is one it declares, states a fact of, or writes in a rule or an hrule` (§ 4.21, M1f S1f.7.11) | 1 |
| a `:goal` whose `eq` or `neq` compares a form that holds a variable (M1f S1f.5.7, Q-M1f.30) | `query: its :goal writes (neq ?x (f ?y)), and the form (f ?y) holds a variable — a guard compares two values, each a variable, a name, or a fact that holds no variable` | 1 |
| `:expect` omitting a relation the `:goal` asks about | `:expect does not name <r>, which the query's :goal asks about` | 1 |
| a `?var` or `_` inside an `:expect` | a parse error at the variable, or at the `_` with S1f.5.1's note † | 1 |

They accumulate and `; `-join with every other loader error, as §1.5's do, and
each is pinned by a fixture in
[`examples/broken/load/`](../../examples/broken/load/).

† **A shape, the grammar's since M1f S1f.5.5's swap** (2026-11-09): each form
takes a fixed set of keywords (S1f.5.11), and a claim's and a region's shapes
are productions of [`ein.y`](../../ein.rs/crates/ein-ir/src/ein.y), so the
parse refuses the file first, every error with its line, column and ranked
repairs (§ 1), and the loader never reads it. The loader's lines for these
rows, `QUERY_KEYWORDS`' and `SOLVE_KEYWORDS`' allow-lists among them, were
deleted at T1f.5.5.5, and each shape is a row of `ein-ir`'s grammar table
([`grammar_pages.rs`](../../ein.rs/crates/ein-ir/tests/grammar_pages.rs)).
Their fixtures moved to [`examples/broken/`](../../examples/broken/) at the swap.

**And one non-error that is also new**: a claim that is *false* prints
`:expect FAILED` with the disagreement, writes one `<file>: … :expect FAILED —
<why>` line on stderr, and exits **1** — a result, not a usage error, so it
takes §4's code rather than §5's. `ein solve` on a file with several
`(query …)` blocks runs the program once and reads every query off that run,
exiting 1 if any claim fails. Until M1e2 S1e2.1a.5 each query was its own walk,
and the flags that name a single output path were refused on such a file with
exit 2; a region run is one run however many queries it answers, so one path
holds it, and the summary's `queries` lists them all.

A third label joins `holds` and `FAILED`: **`NOT CHECKED`**, for a claim the
run did not decide — it asked another part of R (`-n`, a line the claim is not
about) or stopped (`--region-budget`, `--region-limit`). Under `ein solve` it
is a `note:` on stderr and the run exits **0**, because the run answered what
it was asked and an answer is not an error. Under `ein test` it takes **1**,
because `ein test` asks the question the claims are and then, of what that
leaves undecided, the describing one (M1f S1f.6.27,
[`01_grammar.md` § Query](ir/03-ein-lang/01_grammar.md#query)), so a claim it
could not decide is a claim it failed to check — and a green line for a claim
nobody checked is what the whole form exists to prevent. **Under `ein test` a
run that did not exhaust what a claim is about still refutes it**, by a class
of a state found that the claim does not list, or for `(solve … :expect
(false))` by an alive root: `FAILED`, where `ein solve` under the same
question prints the note
([Q-M1f.222](../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f222--a-claim-refuted-by-a-state-a-run-found)).
The words are S1e2.1a.5's
([`read_out.md` § 9](../../plans/m1e2_correctness/p1e2.1a_the_solution_region/read_out.md)).

### 4.2 The verdict atom's refusals — M1d S1d.2.3

Ten more, and by §4.1's rule they name no exception class either. They all
concern `open`, the reserved verdict atom
([`06_reserved_names.md` § the verdict atom](ir/03-ein-lang/06_reserved_names.md)),
and every one of them is a place the engine would otherwise have had to guess.
`<k>` is `rule` or `hrule`, `<n>` the rule's name. **Since M1f S1f.5.23 the
first five are refused at the parse**, `open` being no name and `ein.y`'s
`Assert` the one production that takes it (§ 4.17): each line below is their
record, and `validate_open` holds as an invariant that no `(open …)` stands but
a rule's whole `:assert`. **The last five stay the loader's**, and each starts
with where the `(open …)` is written, `<file>:<line>:<col>: <k> '<n>': …`, where
it ended ` at <loc>`, which read `at None` (Q-M1f.87).

| input | message | exit |
|---|---|---|
| `(open …)` in a `:match` | ``<k> '<n>': `(open …)` is a verdict about the KB and is legal only in :assert — the third-state probe for a fact is `(unknown …)` at <loc>`` | 1 |
| `(open …)` with two or more arguments | ``<k> '<n>': `open` takes the incomplete relation and nothing else — `(open)` or `(open ?R)`, not <n> arguments at <loc>`` | 1 |
| an `:assert` concluding `open` *and* something else | ``<k> '<n>': a rule asserting `open` may assert nothing else — it is read after the fixpoint, where a derivation would be too late at <loc>`` | 1 |
| `(open …)` nested inside a conclusion | ``<k> '<n>': `(open …)` is a whole conclusion, not a term inside one at <loc>`` | 1 |
| `open`'s argument is neither a variable nor a name | ``<k> '<n>': `open`'s argument names a relation — a rule parameter or a relation name at <loc>`` | 1 |
| `(open ?R)` where `?R` is not a parameter | ``<k> '<n>': `(open ?R)` names a relation the activator does not bind — `?R` is not in the parameter list at <loc>`` | 1 |
| no `(absent …)` holds a positive `?R` premise | ``<k> '<n>': `(open ?R)` needs an `(absent …)` in :match holding a positive `?R` premise — that premise is the witness the obligation owes at <loc>`` | 1 |
| two or more `(absent …)` do | ``<k> '<n>': <m> `(absent …)` guards hold a positive `?R` premise — which one states the obligation is not decidable at <loc>`` | 1 |
| the `?R` premise binds no variable of its own | ``<k> '<n>': `(open ?R)`'s `?R` premise binds no variable of its own, so the obligation is ground — that is a plain `absent` check and not something a witness could discharge at <loc>`` | 1 |
| two positive `?R` premises each bind one | ``<k> '<n>': <m> positive `?R` premises each bind a witness variable — a compound witness has no single slot to branch on at <loc>`` | 1 |

Four are pinned by fixtures, `open_in_match`, `open_arity` and
`open_mixed_assert` in [`examples/broken/`](../../examples/broken/) since M1f
S1f.5.5's swap, each a parse error, and `open_compound_witness` in
[`examples/broken/load/`](../../examples/broken/load/), whose line reads
`{FILE}:10:12: rule 'bad': 2 positive …` since S1f.5.23; the five the loader
keeps by `ein-ir`'s `the_verdict_atom_refuses_every_shape_it_cannot_resolve`
and `grammar_pages::the_lines_the_grammar_does_not_cover_are_still_reached`,
and the five the grammar took by `grammar_pages`' table and `DELETED`. **Nine of
the ten until M1e S1e.4.7**: row 5, an `open` whose argument is neither a
variable nor a name, had no case anywhere in the repo, so the sentence that
said *all ten* was the second false thing in this paragraph and the harder one
to see — the count above said *nine* while the table held ten, and the
citation said the shortfall was covered.
Binding the name is the eleventh and is not new: `(relation open …)`,
`(macro open …)` and `(rule open …)` took §1's existing *shadows a reserved
kernel name* message, because `open` joined `ein_core::RESERVED` — the nine
names a declarator may not **bind** (eleven since M1f S1f.5.5's T1f.5.5.11,
which added the kernel's two dunders, and the guard an import's `:as` too) —
and each is a parse error since S1f.5.5's swap, no declarator taking `open`.
It stayed out of `ein-ir`'s `LEXER_KEYWORDS`, the words never a name, while
`(open ?R)` was an ordinary fact head, until M1f S1f.5.23, which made it the
thirteenth of `ein_ir::grammar::RESERVED_WORDS` (§ 4.17). Both sets were called
`RESERVED` until M1e S1e.4.2.

### 4.3 Kernel meta-primitive arity — M1e S1e.2.1

Two more, and they are the first on this page whose *predecessor was not a
message at all*. `(eq ?x)` used to reach the matcher and take the process down
with a `panicked at … assertion failed` and exit **101** — a refusal carrying a
stack trace, which is the one thing
`corpus_cli::every_refusal_carries_a_diagnostic` forbids — and `(eq ?x A B)` /
`(absent A B)` used to *fire*, having dropped every argument past the ones they
read. Both are `CompileError`s now, refused before a `Step::Guard` exists.

| input | message | exit |
|---|---|---|
| `eq` or `neq` at any arity but 2 | ``<file>:<line>:<col>: rule '<r>': its :match writes (eq ?x), and eq takes exactly 2 arguments, not <n> — it compares two values, so write one guard per pair`` | 1 |
| `absent` at any arity but 1 | ``<file>:<line>:<col>: rule '<r>': its :match writes (absent …), and absent takes exactly 1 argument, not <n> — negation as failure is over one premise, so wrap several in an (and …)`` | 1 |

They are the compiler's, raised where it compiles the rule, and print the same
first line on every command that compiles (`ein solve` in each mode, `oracle`,
`saturate`, `test` with a claim, `render lattice`, `render kb`), naming no
class and no IR repr since M1f
[S1f.5.21](../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.21_a_compile_error_in_eins_words.md):
until then each was ``` `(eq …)` takes exactly 2 arguments and was given <n>:
<repr> at Loc(file='…', line=…, col=…) — … ```, behind
`ein.inference.compile.CompileError:` on `ein saturate` and the file's name on
`ein render kb`. **The position is real**: a premise has one, and it is where
the form is written, a macro's body, so the line starts with it, as a parse
error does; in a `.einb` it names the container, `<program stored in x.einb>`
(Q-M1f.15). The views and `kb save` executed no rule, and accepted the program
(Q-M1f.67). **Since M1f S1f.5.24 both rows are the loader's** (§ 4.16): the
same words behind `kb load error: `, on every command, the views and `kb save`
among them, since a rule only a commitment activated, an hrule, an obligation
and a rule no activator reached loaded and some commands answered (W66); the
compiler's two arms are invariants. `cli_semantics::a_compile_refusal_reads_the_same_on_every_command`
holds it, on `examples/broken/load/eq_in_a_macro_body.ein`, under
`broken/compile/` until then.

**Why only two rows for seven wrong cells.** The
[S1e.1.6 sweep](../history/m1e_review_processing/README.md#s1e16--what-nothing-pins-q8-q9-q10) found
the rule behind them:
[`00_ebnf.md` §2](ir/03-ein-lang/00_ebnf.md)'s *Kernel meta-primitives
(shape-pinned)* block names **four** productions and the engine has **seven**
primitives, so `eq`, `absent` and `false` are ordinary `GenericList`s whose
arity is checked by whatever reads them. `neq` is pinned and its wrong arities
have always been a positioned **parse** error; `false` is unpinned and needs no
row, being silent in a `:match` at every arity, which is what a reader expects.
That leaves `eq` and `absent`, one row each, at every arity but their own.

**Since M1f S1f.5.9 only a macro's body reaches the `eq` row.** Through a
relation parameter its activator binds to `eq`, `(?R ?x)` under `(r eq)`, the
row was reached where the plan was compiled, at root or only under a commitment
or an hrule's `(solve :hrules …)` activator (W60); the loader refuses that since
(§ 4.14).

Four fixtures pin them in
[`examples/broken/compile/`](../../examples/broken/compile/README.md) —
`eq_arity_low`, `eq_arity_high`, `absent_arity_zero`, `absent_arity_high` —
and all 21 cells of the sweep, the fourteen that were right included, are
`ein-cli/tests/primitive_arity.rs`.

### 4.4 An artefact flag's failure — M1e S1e.3.5

> **New 2026-08-31** —
> [S1e.3.5](../history/m1e_review_processing/README.md#s1e35--error-handling-medium),
> the review's `EH-M1`. The behaviour is older; what is new is that it is a
> ruling with a test rather than an accident with five spellings.

Seven options name a file or a directory the run will write: `solve --events`,
`--json-summary`, `--trace`, `--dump-answer` and `--dump-lattice` (M1e2
S1e2.1.2, S1e2.1.3), `oracle --dump` (S1e2.1a.9), and `test --json-report`
(plus `test`'s own `--events` / `--json-summary`). The seventh was
`--dump-states` until M1e2 S1e2.1a.5 retired it with the walk.
What happens when the write fails:

| | |
|---|---|
| **the message** | `error: --<flag> <path>: <os error>`, on stderr, for every one of them. The flag because a run may carry three; the path because a shell that expanded a variable to the wrong place can then see it; the OS error because that is the only part that was ever there |
| **the exit code** | **unchanged** — 0 on a run that answered. The four flags whose `--help` says *additive: stdout, stderr and the exit code are unchanged* keep saying it: the answer on stdout is correct and complete, and the artefact is paperwork |
| **the exception** | **`--dump-answer`**, **`--dump-lattice`** and **`oracle --dump`** exit 1 when they cannot make their directory, without solving: the directory is made before the run, so a failure there means the run cannot do what it was asked. `--dump-states` took this arm first, until it was retired. Its files are written after the answer exists, and a failure there is the additive arm's — the message, and the exit code unchanged |
| **an empty path** | a **usage error**: exit 2, refused by the value parser before anything runs, for all seven |
| **a pipe whose reader has gone** | since M1f S1f.5.25 the write ends the process by `SIGPIPE`, whichever file it goes to, `/dev/stdout` or a FIFO: the disposition is process-wide (§ 4.19, [Q-M1f.100](../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f100--an-artefact-flag-that-writes-to-a-pipe-whose-reader-has-gone)); until then the message, and the exit code unchanged |

**The empty path is the one that was a defect rather than a policy.** It
reached all five options; three then failed with a bare *No such file or
directory*; and `--dump-states ""` **succeeded** — `create_dir_all("")` is
`Ok`, `""/00_timeline.jsonl` is a relative path, and the run dropped four
artefacts into whatever directory the caller happened to be in.

**What a consumer gets, said plainly.** A pipeline that asks for
`--json-summary` and gets exit 0 with no file has to read stderr to find out.
That is the cost of the additive reading and it is the reason this is a ruling
rather than a note: the alternative is a distinct exit code, and exit 2
already means *usage error* and *budget abort* in this tool
([TE-M4](../history/m1e_review_processing/README.md#s1e36--tests-medium)).
A third meaning is a CLI-vocabulary change and belongs with that overload
rather than beside it —
[Q-M1e.22](../history/m1e_review_processing/open_questions.md#q-m1e22--should-a-failed-artefact-write-have-an-exit-code-of-its-own).
Pinned by `ein-cli/tests/artefact_contract.rs`, three tests over all seven
options.

### 4.5 The store's own limits — M1e S1e.4.1

> **New 2026-09-01** —
> [S1e.4.1](../history/m1e_review_processing/README.md#s1e41--correctness-low),
> the review's `CO-L1`. Three refusals: the store is bounded by a `u32` and a
> `u16` rather than by memory, and two of the three bounds were unchecked.

`ein-core` interns into two id spaces of 2³⁰ (`Value` packs a 2-bit tag
beside a 30-bit payload) and two arenas addressed by a `u32` start. Every one
of the four is a `Result` at the site that grows it, so hitting one is a load
error a reader can act on. (Three id spaces until M1f S1f.5.4 retired the int
pool, whose limit read *too many distinct integer literals*.)

| input | message | exit |
|---|---|---|
| more than 2³⁰ distinct symbols / facts | ``too many distinct {symbols,facts} — the limit is 1073741824`` | 1 |
| more than 2³² bytes of symbol text | ``too much symbol text — the limit is 4294967296 bytes`` | 1 |
| more than 2³² stored fact arguments | ``too many stored fact arguments — the limit is 4294967296`` | 1 |
| a fact with more than 65 535 arguments | ``a fact takes at most 65535 arguments`` | 1 |

**The last one was reachable, and it was a panic.** `(p A0 … A65535)` is a
three-line program and 447 KB of source; before this stage it reached
`u16::try_from(args.len()).expect("a fact's arity fits u16")` and took the
process down with a stack trace and exit **101** — the failure shape
[§4.3](#43-kernel-meta-primitive-arity--m1e-s1e21) ruled out for `(eq ?x)`
one phase earlier, and the one
`corpus_cli::every_refusal_carries_a_diagnostic` forbids. 65 535 arguments
solve; 65 536 are refused. Pinned by
`ein_core::facts::tests::a_fact_wider_than_the_arity_field_is_refused_and_one_narrower_is_not`.

**The two arena bounds are unreachable and guarded anyway**, because what they
prevent is not a crash but a *wrong answer*: a wrapped span start is another
value's identity, which is the silent failure the interner's own doc comment
says the module exists to rule out. Which limit binds first is measurable and
is not the one the comment used to name — at the corpus's own mean symbol
length (7.55–21.00 bytes, `ein kb save`'s symbol-table header, 2026-09-01) the
**byte** arena fills at 19–53 % of the id ceiling, not the other way round.
`ein_core::intern::tests::the_arena_bound_is_the_byte_bound_and_not_the_id_bound`
is what holds that.

### 4.6 A relation named by a number — M1e2 S1e2.2.4

A rule may name the relation it matches or asserts by a parameter, `(rule
mirror (?R) :match (?R ?a ?b) :assert (?R ?b ?a))`, and its activator binds
it: `(mirror q)`. An activator that binds such a parameter to an **integer**
would have the compiler name a relation by the integer's decimal text, a name
the program does not have, which no rule's text predicts and so no candidate
can hold ([§ 3.3](#33-the-m1-alive-set-invariant-operationally)). The user
ruled on 2026-10-16 that rules cannot assert new names
([Q-M1e.21](../history/m1e_review_processing/open_questions.md#q-m1e21--a-rule-may-name-an-object-the-search-can-never-hypothesise-about)
(b)), so both places that can see the binding refused it, in ein's own words,
**until M1f S1f.5.5's swap** (T1f.5.5.4, 2026-11-09). Since the swap a run of
digits is a name (S1f.5.4's ruling), so `(mirror 3)` names the relation `3` as
`(mirror q)` names `q`, a name the program has, and neither row below is
reached: its fixture, `examples/broken/load/relation_parameter_int.ein`,
retired with its `.expected` and corpus entry (S1f.5.4's Q2 (a), decided
2026-11-08), and `compile_semantics.rs`' test, now
`a_relation_parameter_a_rule_binds_to_a_nested_fact_is_refused`, holds that
`3` names a relation and keeps the nested-fact half below. The two lines,
which S1f.5.4's T1f.5.4.3 retired with the int kind (2026-11-09), were:

| input | message | exit |
|---|---|---|
| a fact, or a literal in a rule's `:assert`, that activates rule `<r>` with an integer where a parameter names a relation | `rule '<r>': its parameter ?<p> names a relation, and <activator> binds it to the integer <n> at <loc> — a relation is named by a symbol, not a number, and a rule cannot assert a name the program does not have` (a loader message; `<loc>` is the integer's) | 1 |
| the same activator, derived by a rule through a variable | `the relation head ?<p> of <form> at <loc> is bound to the integer <n> — …` (a `CompileError`, § 4's second row) | 1 |

The first was pinned by `examples/broken/load/relation_parameter_int.ein`,
the second by `ein-infer/tests/compile_semantics.rs`'s
`a_relation_parameter_a_rule_binds_to_a_number_is_refused`. An activator of
the wrong arity activates nothing and is not checked; a name binds a name (a
string did too until M1f S1f.5.5's swap refused one, § 4.12, and a range until
S1f.5.3 retired the token), and a nested fact binds its parameter since M1f
S1f.5.18 (§ 3.2), as it bound none until then. Where the parameter names a
relation it names none, and since M1e2 S1e2.2.24 that is refused in both places
too, as *binds it to the fact (a b)* (§ 4.9): at load where the activator is
written down, and since M1f S1f.5.18 where a rule could derive it, the
compiler's line the backstop behind both. Until S1e2.2.24 it was read as no
binding, and the relation it would name matched nothing, in silence.

### 4.7 A refutation resting on an `absent` a commitment can fill — M1e2 S1e2.2.5

*Forbid*, the user's ruling of 2026-11-02
([S1e2.4.1 § 5](../../plans/m1e2_correctness/p1e2.4_what_the_switch_left/s1e2.4.1_refutation_under_absent.md#5-the-ruling)),
with its F2 of 2026-11-04, *via, per activator*: a rule that concludes
`(false)` or `(not …)` may not rest on an `(absent P)` over a relation a
commitment can still extend, directly, through a fact a rule derived under
one, or through its activator. *Extendable* is `D⁺`: the static check's `D`
([`monotonicity.md`](inference/monotonicity.md)) followed through every rule
with parameters whose activator relation it holds, since a node compiles a
plan of such a rule that root never had, and what that plan writes a
commitment can change too. The check reads root's plans and, from its
pattern, every such rule, and an `absent` such a rule reads itself, over a
relation in `D⁺` or over the relation its parameter names, taints what it
derives. `D⁺` is `D` on a program with no activator site, and no reader but
this one uses it: the static check, its sites and the obligations rung's
decline read `D`. Over such a relation the
shape calls a state that has not yet decided `P` a contradiction, so `dead`
is not upward-closed. [`ein_infer::forbid`](../../ein.rs/crates/ein-infer/src/forbid.rs)
reads it after root's fixpoint, where the static check runs, so it refuses
**every command that takes the universe**: `ein solve` in every mode
(`--monotone`, `--count` and `--no-search` too), every `(solve :expect)` check, `ein test`,
`ein oracle` and `ein render lattice`. `ein saturate` and `ein render
rules|constraints|program|kb` take no universe and do not refuse. A plan has
an activator when its rule has parameters, whatever the activator's
arguments; until M1e2 S1e2.2.16 the check asked whether the activator had a
**symbol** argument, so one whose every argument is an int or a nested fact
read as none and its refutation loaded (W16,
`examples/ein-bugs/activator-all-int.ein`). The `[activator …]` label still
prints the symbols, so such a line names none. **Until M1e2 S1e2.2.17**
(W17, 2026-11-04) *extendable* was `D` itself, a rule with parameters was
read from its pattern only when its activator relation was in `D`, and such a
rule's own `absent` tainted nothing, so a refutation whose activator chain
first appears at a node loaded with a dead hole: one or two activators past
`D` (`examples/ein-bugs/wrong-answers/forbid-chain-1.ein`, `-2.ein`), an
`absent` in the plan a node compiles (`-node-absent.ein`, and over its
parameter's relation, `-param-absent.ein`), and an `absent` over a relation
only such a plan writes (`-node-writes.ein`). Each is refused since, and
`forbid-chain-no-absent.ein`, the chain with no `absent`, loads with its R
unmoved. One line per refusing rule, sorted by rule:

| cause | message | exit |
|---|---|---|
| the rule reads a relation in `D` inside an odd number of `absent`s | `rule '<r>'[ [activator <a…>]]: concludes <(false)\|(not …)> from (absent …) over <R>, a relation a commitment can still extend — <the rest>` | 1 (`ein test`: 2) |
| it reads `<P>`, which rule `<d>` derives resting on such an `absent` | `… concludes <c> from <P>, which rule '<d>' derives resting on (absent …) over <R>, a relation a commitment can still extend — <the rest>` | 1 (2) |
| ″, the `absent` over the relation a parameter of rule `<d>` names (since M1e2 S1e2.2.17) | `… concludes <c> from <P>, which rule '<d>' derives resting on (absent …) over the relation its parameter ?<p> names, and a commitment can derive its activator, so a node can compile it over a relation a commitment can still extend — <the rest>` | 1 (2) |
| its activator is such a fact | `… concludes <c> from its activator, which rule '<d>' derives resting on (absent …) over <R>, … — <the rest>` | 1 (2) |
| a rule with parameters, whose activator a commitment can derive, reads its parameter's relation under an odd `absent` | `… concludes <c> from (absent …) over the relation its parameter ?<p> names, and a commitment can derive its activator, so a node can compile it over a relation a commitment can still extend — <the rest>` | 1 (2) |

`<the rest>` names the three forms that say what was meant (F4): close the
relation, `(solve :no-hypothesis (<R>))`, if no commitment is meant to extend
it; state a requirement as a decision or an obligation (`functional` +
`total-owed`, or `(open ?R)` with hrules); or conclude from a stored negative,
as `std.algebra`'s `total` does. **Where the program declares an hrule**, no
list filters a candidate (§ 4.27), so since M1f S1f.6.34 (E-47) the first form
is *let no hrule propose <R>, if no commitment is meant to extend it — hrule
'h' proposes it, and no list filters an hrule's candidates*, naming every
active hrule that proposes `<R>`, and *let no hrule propose what <R> is
derived from …* where none proposes it directly; until then it named
`(solve :no-hypothesis (<R>))` there too, a form that lifted nothing and that
§ 4.27 refuses beside such an hrule. **Where no hrule is declared and an
obligation owes `<R>`** the first form is *let no obligation owe <R>, if no
commitment is meant to extend it — obligation 'o' owes it, and no list may
filter what an obligation owes*, § 4.27 refusing the list there too
(`ein-infer/tests/refutation_under_absent.rs`'s
`beside_an_hrule_the_remedy_names_the_hrule_and_not_the_list` and
`beside_an_obligation_the_remedy_names_the_obligation`). Pinned by
`ein-infer/tests/refutation_under_absent.rs`, which holds the refused set of
the corpus and the register exactly and the two written activator programs,
and by `corpus/wrong_answers.toml`'s fifteen refused rows (`python3 -c
"import tomllib; print(sum(1 for r in tomllib.load(open('corpus/wrong_answers.toml','rb'))['case'] if r['right'].get('refused')))"`,
2026-11-04).

### 4.8 A wildcard as a relation in a rule — M1e2 S1e2.2.20

**At the parse since M1f S1f.5.5's swap** (T1f.5.5.4, 2026-11-09), on the
user's ruling of 2026-11-06
([Q-M1e2.22](../../plans/m1e2_correctness/open_questions.md#q-m1e222--does-ein-need-a-wildcard)):
a bare `_`, and `?_`, is refused wherever it is written, a relation among
the places, with the file, line and column and S1f.5.1's note, *`_ is reserved
and names nothing: where any value will do, write a variable used once`*, from
every command that reads the program, exit 1 (`ein test`: 2); S1f.5.5's T5
deleted the line below (`Found::WildHead`), and S1f.5.1's T1f.5.1.3 retired
the AST's `Wildcard` node and the compiler's `Slot::Any` (`ef5374a1`). What
follows is the section as it stood from S1e2.2.20 to the swap.

A wildcard `_` stands for an argument, and a relation is named by a name or
by a parameter its activator binds (Q29). So, on the user's ruling of
2026-11-05
([Q-M1e2.14](../../plans/m1e2_correctness/open_questions.md#q-m1e214--how-does-a-head-wildcard-match-every-relation)
(c)), the loader refuses a `_` written as a form's relation, `(_ a)`, at any
depth of a rule's `:match` or `:assert`, in a rule, an hrule or an
obligation. One line per slot, `; `-joined with every other loader error, in
ein's own words:

| input | message | exit |
|---|---|---|
| a wildcard as a relation | `<rule\|hrule> '<r>': its <:match\|:assert> writes (_ a) with the wildcard _ as its relation — a relation is named by a name or by a parameter its activator binds, and _ stands for an argument` | 1 (`ein test`: 2) |

A keyword's value inside a clause, `(n ?x :where v)`, was no slot, which
the compiler dropped, and loaded, until the swap refused a keyword pair inside
a term at the parse (S1f.5.11); S1f.5.11's T1f.5.11.2 retired the drop. A `_` as an argument matched any value, until
the swap
([`01_grammar.md`'s `WILDCARD` row](ir/03-ein-lang/01_grammar.md#1-lexical-rules)).
Pinned by `ein-cli/tests/cli_semantics.rs`'
`a_wildcard_relation_in_a_rule_is_refused_at_load` (at the parse since the
swap) and `a_bare_underscore_names_nothing_in_every_place` (T1f.5.1.3), by
the rows of `ein-ir/tests/grammar_pages.rs`' table that write `_` and `?_`
(`from_ir.rs`' unit test, until T5 moved it there), and by
`corpus/wrong_answers.toml`'s `W20/wild-in-head`.

**A range was refused here too, until M1f S1f.5.3** (2026-11-08). From
S1e2.2.20, on the user's ruling of 2026-11-04
([Q-M1e2.13](../../plans/m1e2_correctness/open_questions.md#q-m1e213--what-does-a-wildcard-or-a-range-in-a-rules-pattern-mean)
(a)), the loader refused a range, `1..3` or `2..*`, written where a pattern's
argument goes or as a whole clause, with *`rule 'r': its :match writes 1..3
as an argument of n, and no fact can hold it — a range is cardinality
metadata, and a rule matches and concludes facts`*. Ein has no numbers and no
ranges (the user, 2026-11-06,
[Q-M1e2.24](../../plans/m1e2_correctness/open_questions.md#q-m1e224--are-there-numbers-in-ein)),
and S1f.5.3 retired the token, so `1..3` was a parse error wherever it was
written (§ 1), from every command that read the program, exit 1 (`ein
test`: 2), until S1f.5.5's swap made it one name (S1f.5.4's ruling): since,
`(n 1..3)` in a rule is a pattern over the name `1..3`, which matches the
facts that hold it, and `:card 1..*` is refused at the parse at `:card`, a
keyword no form takes (S1f.5.11). Pinned by `cli_semantics.rs`'
`a_range_is_the_name_it_spells` (`a_range_is_no_token` until the swap) and
`W20/range-in-match`, whose `right` is the cube of 8 since.
**Until S1e2.2.20** (W20) a range was a slot that matched nothing, in silence,
and one in an `:assert` stopped the run at the rule's first firing with
§ 4's *conclusion no fact can hold* line; a `_` head stopped compilation with
§ 4's compile row as it read then, `ein.inference.compile.CompileError:
unbound relation head ?Wildcard() in a premise of `SForm(head=Wildcard(), …)`
…`, the Python IR's spelling, which no `_` reaches since. A variable head no parameter binds,
`(?r a)`, still did until S1e2.2.24, which refuses it at load (§ 4.9).

### 4.9 A rule shape that reads nothing — M1e2 S1e2.2.24

**Since M1f S1f.5.5's swap** (2026-11-09) the grammar refuses three of the
shapes below at the parse, with the file, line and column, wherever a program
writes them: `_` (S1f.5.1), a keyword pair inside a term (S1f.5.11), and a
premise that is not a form (`ein.y`'s `Premise`). The loader's lines stay for
the rest, which the grammar cannot see (a form as a guard's argument is a
`Term`, and which variables a premise binds is the loader's), and for a
premise that is not a form where a macro's body puts it, `(macro id (?x) ?x)`
called as `(id foo)`, which S1f.5.5's T5 kept
(`grammar_pages::the_lines_the_grammar_does_not_cover_are_still_reached`).

Each shape below loaded until 2026-11-05 and read nothing, in silence: no
fact could make it hold, or it held whatever the facts. On the user's rulings
of that day
([Q-M1e2.16](../../plans/m1e2_correctness/open_questions.md#q-m1e216--what-does-the-close-do-with-the-inventorys-items)
(1) and [Q-M1e2.18](../../plans/m1e2_correctness/open_questions.md#q-m1e218--what-else-in-a-rule-reads-nothing-in-silence)),
the loader refuses each in a rule, an hrule and an obligation, in the same
pass as § 4.8's, one line per slot, `; `-joined with every other loader
error, in ein's own words; exit 1 (`ein test`: 2):

| input | message |
|---|---|
| `_` as a guard's argument (a parse error since M1f S1f.5.5's swap, § 4.8) | `<rule\|hrule> '<r>': its :match writes (eq ?x _), and _ names no value — a guard compares two values, and _ stands for an argument of a pattern` |
| a form that holds a variable as a guard's argument (since M1f S1f.5.7, Q-M1f.30; every form, and a keyword pair, until then, the keyword pair a parse error since M1f S1f.5.5's swap, S1f.5.11) | `… writes (eq ?x (f ?y)), and the form (f ?y) holds a variable — a guard compares two values, each a variable, a name, or a fact that holds no variable` (`… and the form (f a) is no value — a guard compares two values, each a variable a premise before it binds or a name` until S1f.5.7) |
| ~~a guard on a variable no premise before it binds~~ — **loads since M1f S1f.5.7** (the user's ruling of 2026-11-07, Q-M1e2.26): the guard waits for the premise that binds it, or binds it over the program's names | `… writes (neq ?x a), and no premise before it binds ?x — a guard compares two values, each a variable a premise before it binds or a name`, until S1f.5.7 |
| a premise that is not a form: an atom (a run of digits among them, an integer until M1f S1f.5.4), a variable, `_`, a string (`_` and a string a parse error since M1f S1f.5.5's swap, § 4.8 and § 4.12) | `… its :match is foo, and foo is no premise — a premise is a pattern (relation argument …), a guard (eq a b) or (neq a b), an (absent …), or an (and …) or (or …) of premises` (`… writes _ as a premise of and, …` inside a form) |
| a relation written as a variable no parameter binds, at the top level or inside a pattern, in either clause | `… its <:match\|:assert> writes (?r b) with ?r as its relation, which no parameter binds — a relation is named by a name or by a parameter its activator binds` |
| a keyword pair inside a pattern inside a pattern, in either clause (a parse error since M1f S1f.5.5's swap, S1f.5.11, which refuses a keyword pair anywhere in a term) | `… writes :k v as an argument of a, and no fact can hold it — a keyword pair is no value, so a pattern inside a pattern that holds one matches nothing` |
| an activator, a fact or a literal in an `:assert`, that binds a relation parameter to a nested fact | `rule '<r>': its parameter ?<p> names a relation, and (r (a b)) binds it to the fact (a b) — a relation is named by a symbol, not a fact` |
| the same activator, which a rule could derive through a variable (since M1f S1f.5.18, below) | `rule '<d>': its :assert writes (r ?f), whose ?f is rule '<r>''s parameter ?<p>, which names a relation, and (item ?f) can bind ?f to a fact, as an argument of the declared relation item — a relation is named by a symbol, not a fact` (`… as an argument of (g ?f)` inside a fact pattern, `… of the relation ?S names` under a parameter's relation; `hrule '<d>'` for an hrule; `… where its :match holds through <arm>` where some disjuncts of an `(or …)` bind it where a name is held) |
| a fact, or a literal fact in an `:assert`, written where a relation name is passed on (since M1f S1f.5.18, below) | `the fact (marker (a b)) writes (a b) as ?m of (marker ?m), which rule 'fan' passes to rule '<r>''s parameter ?<p>, which names a relation — a relation is named by a symbol, not a fact` (`rule '<d>': its :assert writes (marker (a b)), whose (a b) is ?m of …` for the literal) |

**A derived activator is refused at load since M1f
[S1f.5.18](../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.18_an_activator_binds_a_nested_fact.md)** (2026-11-09,
`from_ir::refuse_derived_relation_activators`), on the user's ruling of
2026-11-08 on Q-M1f.1 (b): a program in which some rule could derive an activator
binding a relation parameter to a fact is refused, whether or not a run derives
it. *Could* is read off the program (Q-M1f.41, taken by default on the user's
instruction of 2026-11-09). A rule's parameter that heads a form holds relation
names, and so does every argument of `(relation …)`; an argument of a
**declared** relation holds any value of its type, a fact among them, as does an
argument inside a fact pattern and one of a relation a parameter names; and an
argument of an **undeclared** relation, a marker such as `(bijective
color-loc)`, or a rule's own parameter holds relation names where the program
passes a variable bound there on to a position that does, to a fixpoint, so the
stdlib's fan-outs load and a fact written there is refused. A conclusion written
`(?T …)` is read as writing into every rule of its arity whose name the program
writes as an argument (Q-M1f.42). Until S1f.5.18 such a program loaded: where a
run derived the activator it stopped where the plan was compiled, with the
compiler's line, `the relation head ?<p> of <form> at <loc> is bound to the fact
(a b) — a relation is named by a symbol, not a fact.` (a `CompileError`, § 4's
second row, as § 4.6's integer), and where none did it answered (W59's two
rows). That line stays, the backstop behind both loader checks, which no program
the loader accepts reaches and a direct caller of `compile_rule` does. Pinned by
`ein-ir`'s `from_ir.rs`
`a_derived_activator_binding_a_relation_parameter_to_a_fact_is_refused_at_load`,
`ein-cli/tests/cli_semantics.rs`'s test of the same name, `compile_semantics.rs`'
`a_relation_parameter_a_rule_binds_to_a_nested_fact_is_refused`,
`examples/broken/load/derived_activator.ein` and `derived_activator_never.ein`,
and the register's two W59 rows.

Since M1f S1f.5.7 a guard's variable needs no binder before it: a premise
means the same in any order of its conjuncts, so a guard waits for the relation
premise that binds its variable, and binds one nothing binds, `eq` to the other
side and `neq` over the program's names (§ 01_grammar.md's *Premise order
around a guard*). From S1e2.2.24 until then a guard's variable counted as bound
by the rule's parameters and by the variables of the positive patterns written
before it; under an `(absent …)`, by those the enclosing premises bound before
it and the `absent`'s own earlier premises. What stayed until M1f S1f.5.5's swap: a keyword pair at a
pattern's top level or as a premise of an `(and …)`, which the compiler
dropped (Q32); `_` as a pattern's argument; `_` and a keyword pair at a
conclusion's top level, which stopped the firing with § 4's line. Each is a
parse error since the swap (`_` § 4.8, S1f.5.1; the keyword pair S1f.5.11),
and S1f.5.11's T1f.5.11.2 retired the compiler's drop. **Until S1e2.2.24** (W24): a guard compared `_`, a
form or a keyword pair as a raw IR node, equal to no value, and a variable
not yet bound as a null, so `(eq ?x _)` never held and `(and (neq ?x a) (p
?x))` fired for `a`; a premise that is not a form was dropped, so the rule
fired as if it were not written; a pattern inside a pattern whose relation
no parameter binds, holds a keyword pair, or names a parameter an activator
bound to a nested fact, matched nothing. The same relation variable at the
top level, and an `(absent ?x)`, stopped compilation with § 4's second row in
the Python IR's spelling (`unbound relation head ?r in a premise of
`SForm(…)``, `` `(absent …)` sub-plan compiled to no steps in SForm(…) ``),
which no program the loader accepts reaches since. Pinned by `ein-ir`'s
`from_ir.rs` `a_rule_shape_that_reads_nothing_is_refused_at_load`,
`ein-cli/tests/cli_semantics.rs`'s test of the same name,
`ein-infer/tests/compile_semantics.rs`'s
`a_relation_parameter_a_rule_binds_to_a_nested_fact_is_refused` (named for the
integer until S1f.5.5's swap), `examples/broken/empty_absent.ein` (a
compile-negative fixture until S1e2.2.24, a load-negative until S1f.5.5's
swap, refused at the parse since) and `examples/broken/load/unbound_head.ein`,
and `corpus/wrong_answers.toml`'s seven W24 rows, three of which, `_` in a
guard, a premise that is not a form and a keyword pair in a guard, are refused
at the parse since the swap (S1f.5.1, S1f.5.11, the paper grammar).

### 4.10 A form with no `:why` — M1f T1f.5.14.6, T1f.5.14.7

`:why` is optional in the grammar, and the loader makes it required: the
user's rulings of 2026-11-08 (*":why is optional in grammar and loader has a
flag that makes it required, start with default false so not break everything,
then after other stages finished swith to default true"*; *"yes, the flag only
decides loading"*). **By default since M1f S1f.5.14's T1f.5.14.7** (2026-11-10),
on every command that loads a program and in `ein_ir::load` and its siblings
(`ein_ir::LoadOptions::require_why`, true by default), the loader refuses every
`(relation …)`, `(rule …)`, `(hrule …)` and `(macro …)` it reads, the stdlib's
and an imported module's included, that carries no `:why "…"`, one line per
form,
`; `-joined with every other loader error in pass order, in ein's own words;
exit 1 (`ein test`: 2):

| input | message |
|---|---|
| a relation, rule, hrule or macro with no `:why` | `<relation\|rule\|hrule\|macro> '<name>' has no :why "…", which this load requires of every relation, rule, hrule and macro` |

A `:why` that is not a string is refused at the parse since S1f.5.5's swap
(T1f.5.5.4), where the loader's strings-only guard read it as none until then.
Until the swap a `(macro …)` was asked for one only in a build with `ein-ir`'s
`generated-parser` feature, the one parser that read a macro's `:why`, and the
line read `… every relation, rule and hrule` elsewhere; the swap made the
generated parser the only one, deleted the `cfg`, and wrote the stdlib's two
macros, `forall` and `unknown`, their `:why`. `--no-require-why`
([`configuration.md` § 3.4](configuration.md#34-the-option-that-decides-loading)),
or `require_why: false`, loads such a program, and nothing else moves; until
T1f.5.14.7 that was the default, and `--require-why` turned the refusal on.
Pinned by `ein-ir/tests/load_semantics.rs`'
`require_why_refuses_each_form_without_one`,
`the_default_requires_a_why_and_false_loads_without`,
`a_macro_needs_a_why_only_where_its_parser_reads_one` and
`require_why_decides_loading_only`, and by `ein-cli/tests/config_reference.rs`'
`require_why_decides_loading_only`.

### 4.11 A template reference no reference can reach — M1f T1f.5.4.5

A template reference, `{?name}` in a `:why` or a `:goal-text`, reads the name
rule's characters up to the first `}` (`ein_core::why::render_why`), so it
names every variable a program writes since M1f S1f.5.4's T1f.5.4.5
(2026-11-09, W37): `{?isR*}`, `{?_x_}` and `{?a.b}` are filled, where until
then a reference took one letter or digit and then letters, digits, `_` and
`-`, and printed any other as written. `}` is a name character too, so a
variable holding one, `?a}b`, is one no reference can name. The loader refuses
a template that writes a reference to one, taken by default on the user's
instruction of 2026-11-09
([Q-M1f.11](../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f11--a-template-reference-to-a-variable-no-reference-can-name)):
a rule's, an hrule's or a macro's `:why` and a query's `:goal-text`, each read
against its own variables, one line a reference, `; `-joined with every other
loader error; exit 1 (`ein test`: 2):

| input | message |
|---|---|
| a template that writes `{?a}b}` where `?a}b` is a variable of its form | `<rule\|hrule\|macro> '<name>': its :why writes {?a}b}, a reference to ?a}b, which no reference can name — a reference reads a variable's name up to its first }` (`query: its :goal-text writes …` for a query) |

A reference to no variable of the form, `{?a}` there, stays text, as an
unbound one always has. A relation's `:why` names its slots by position,
`{?1}`, and is not read. Pinned by `ein-ir/tests/load_semantics.rs`'
`a_template_naming_a_variable_no_reference_reaches_is_refused` and
`ein-core`'s `why` unit tests.

### 4.12 A string is natural-language text, and an escape names a character — M1f S1f.5.2

A string is the value of `:why`, `:source` or `:goal-text` and nothing else,
on the user's ruling of 2026-11-06
([Q-M1e2.23](../../plans/m1e2_correctness/open_questions.md#q-m1e223--what-is-a-string-for)),
which M1f S1f.5.5's swap built into the grammar (2026-11-09, `ein.y`'s `Text`).
Written anywhere a value or a name goes it is a parse error at the string, and
a variable written in a fact at the variable (G-6), on every command that reads
a program; exit 1 (`ein test`: 2):

| input | message |
|---|---|
| a string as a value or a name: `(p "1")`, `(rule r () :match (p "a") …)`, `(relation "p" T)`, `(config :lattice-sanity-check "true")` | `<file>:<l>:<c>: unexpected `"1"`: a string`, then `note: "1" is a string, and a string is natural-language text, written only after :why, :source or :goal-text: write a name bare` |
| a variable in a fact: `(r ?x)` | `<file>:<l>:<c>: unexpected `?x`: a variable` |
| an escape a string does not read: `\x41`, `\u0416`, `\u{}`, `\u{110000}`, `\u{D800}` | `<file>:<l>:<c>: unexpected `"…"`: a string with an unknown escape`, at the opening quote, then one `note: \u{110000}, at <l>:<c>, is no escape a string reads: a string's escapes are \n, \t, \r, \", \\ and \u{…}, one to six hex digits naming a Unicode scalar value, 0 to 10FFFF and not D800 to DFFF` per escape |

Each is followed by the error's ranked repairs (§ 1). A string reads `\n` `\t`
`\r` `\"` `\\` and `\u{…}`, and holds any other character as itself, a raw
newline and any Unicode among them
([`00_ebnf.md` § 1](ir/03-ein-lang/00_ebnf.md#1-lexical-grammar), fact 4). Until
the swap a string was also the name it spells wherever a value went (M1e2
S1e2.2.16 in a fact, S1e2.2.18 in a rule), a variable in a fact the name `?x`,
and an unknown escape lost its backslash in silence; until S1f.5.2's T1f.5.2.5
a `\u{…}` naming no character read as U+FFFD (W38). Pinned by `ein-ir`'s
grammar table (`grammar_pages.rs`, a row per place), `generated_parser.rs`'
`a_strings_escapes_are_the_rulings`, and `ein-cli/tests/cli_semantics.rs`'
`a_string_is_natural_language_and_no_value`; the text a program writes reaches
every surface that carries it byte for byte (`natural_language_text.rs`).

### 4.13 A conclusion's variable nothing binds — M1f S1f.5.10

A rule's or an hrule's `:assert` that writes a variable some way of satisfying
its `:match` leaves unbound is refused at load, on the user's ruling of
2026-11-07 on P1f.5's F13 (*"yes, file F13 with the refusal"*), one line per
variable, in the order the `:assert` writes them, `; `-joined with every other
loader error, in ein's own words; exit 1 (`ein test`: 2), on every command that
loads the program (`saturate`, every `solve` mode, `oracle`, `test`, `render
lattice`, `render kb`, `kb save`):

| input | message |
|---|---|
| a variable no premise or parameter binds | `<rule\|hrule> '<r>': its :assert writes ?y, which no premise or parameter binds — to conclude it of every object of a type, bind it with a premise such as (is-a ?y T), or of every name of the program with (eq ?y ?y)` |
| one that some disjuncts of a top-level `(or …)` bind and others do not | `… writes ?v0, which no premise or parameter binds where its :match holds through (r1 ?v1 ?v3) — …`, the disjuncts that leave it unbound `or`-joined |

A variable is bound, on each way the `:match` can hold, by a **parameter**,
which the activator binds; by a **relation premise**, at any depth of its
arguments, a stored `(not P)` among them; or by an **`eq` or `neq` guard**,
which binds a variable nothing else binds over the program's names (M1f
S1f.5.7). One seen only inside an `(absent …)` is not bound outside it, and
`forall` and `unknown` expand to one; `()` binds nothing; a top-level `(or …)`
is one way per disjunct. The line's two spellings say what the implicit form
could mean, *every object of a type* and *every name of the program*, each
written out. An obligation's `(open ?R)` names a relation, which § 4.2 checks.

**Until S1f.5.10** the loader did not check it, and the run found the
variable where a firing reached it (W57): root's firing stopped every command,
exit 1, with § 4's `KeyError` line (`ein solve` printing the message alone); a
firing only a commitment made stopped `ein solve` and `ein oracle` while `ein
saturate` and `ein solve --minimal` answered; an hrule's template proposed
nothing, in silence; and a rule that never fired, its `:match` empty (which
fires since M1f S1f.5.9), its activator never written or its `(absent …)` never
holding, loaded and answered. **Nothing reaches § 4's retired `KeyError` line since M1f
S1f.5.18**: the one input S1f.5.10 left, a value parameter its activator binds
to a nested fact, read only by a conclusion, which the compiler did not seed
(W33's cause), is bound since (`W58`, Q-M1f.37), and the line and
`FireError::UnboundVar` are gone. Pinned by `ein-ir`'s `from_ir.rs`
`a_conclusion_variable_nothing_binds_is_refused_at_load`, `ein-cli`'s
`cli_semantics.rs` test of the same name, `fuzz_properties`'
`no_firing_meets_an_unbound_variable`, `examples/ein-bugs/unbound-assert-var.ein`
and `corpus/wrong_answers.toml`'s four W57 rows.

### 4.14 `eq` at another arity — M1f S1f.5.9

`eq` is the relation of equality, between exactly two values (M1f S1f.5.7, the
user's ruling of 2026-11-07 on Q-M1e2.26), and the grammar refuses `(eq ?x)`
where a premise or a conclusion is written. Two ways around the grammar are
refused at load since M1f
[S1f.5.9](../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.9_an_empty_match_fires.md)'s T1f.5.9.4,
on the coordinator's brief and taken by default on the user's instruction of
2026-11-09 (Q-M1f.46, Q-M1f.47), one line each, `; `-joined with every other
loader error, exit 1 (`ein test`: 2), on every command that loads the program:

| input | message |
|---|---|
| a rule's or an hrule's relation parameter that can be bound to `eq`, heading a premise (at any depth of `and`, `or`, `not` and `absent`) or a top-level conclusion whose arity is not 2 | `<rule\|hrule> '<r>': its parameter ?R heads (?R ?x), a form of 1 argument, and the fact (r eq) binds ?R to eq — eq is the relation of equality, which holds between exactly two values`; where a rule derives the activator, `… and ?R can be bound to eq by <where eq comes from> — eq is the relation of equality, which holds between exactly two values` |
| a written fact of `eq`, or of `(not (eq …))`, at any arity but 2 | `the fact (eq a) writes eq with 1 argument — eq is the relation of equality, which holds between exactly two values` |

**Which parameters can hold `eq`** is read as § 4.9's check reads relation
names (S1f.5.18, Q-M1f.41): a position that holds relation names can hold
`eq` where a fact, a conclusion or a `(solve :hrules …)` activator writes `eq`
there, or a `(relation …)` declaration's signature writes it as a type; or
where a conclusion writes a variable that its `:match` binds, in some
disjunct, only at positions that can, or only with an `eq` or `neq` guard
while the program writes `eq` as a name; to a fixpoint. `<where eq comes from>`
names the first such source and each conclusion it passes through: *the fact
(mark eq), through rule 'mk''s conclusion (r ?f)*, *rule 'mk''s conclusion (r
eq)*, or *an eq or neq guard of rule 'mk', over the program's names, eq among
them, through its conclusion (r ?f)*. A parameter heading a pattern inside a
pattern's argument fills a value, and since M1f S1f.7.10 a value is no form of
`eq`, so it is refused at every arity (§ 4.22; until then `(p (eq a))` was a
value and was not read). Not read is a
declared relation of another arity, `(?R ?x)` under `(r q)` with `q` binary,
which matches nothing, as `(q ?x)` written directly does, matching being
arity-coupled. `neq` is no name a program can write, so no parameter holds it.

**Until S1f.5.9** (W60's four rows, W61's): the premise loaded and the run
stopped where the activator's plan was compiled, § 4.3's line in the Python
IR's spelling, at root on every command, or under a commitment, or at an
hrule's activator, where `ein saturate` answered and `ein solve` stopped; the
conclusion stored `(eq a :rule r)`; and the written fact stored `(eq a)`,
Q-M1f.33's check reading two arguments only. Pinned by `ein-ir`'s `from_ir.rs`
`eq_at_another_arity_is_refused_at_load`, `ein-cli`'s `cli_semantics.rs`
test of the same name, and `corpus/wrong_answers.toml`'s W60 and W61 rows.

### 4.15 The compiler's refusals a program reached, at load — M1f S1f.5.21

Three refusals the compiler made, which a program reached, are the loader's
since M1f
[S1f.5.21](../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.21_a_compile_error_in_eins_words.md),
on the user's ruling on the close review's finding 1 (*"new stage in p1f.5"*):
one line each, `; `-joined with every other loader error, exit 1 (`ein test`:
2), on every command that reads the program, the four views and `kb save`
among them. The compiler's arms for them are invariants (Q-M1f.9 (a)'s pattern).

| input | message |
|---|---|
| an `(or …)` written as a premise, of an `(and …)`, an `(absent …)` or a top-level `(or …)`, in a rule's, an hrule's or an obligation's `:match` or a query's `:goal`, written or produced by a macro's expansion (Q-M1f.74) | `rule 'r': its :match writes (or (p ?x) (q ?x)) as a premise of and — an (or …) is a whole :match, each of its premises one way it holds: lift it to the top of the :match, or split the rule`; a goal's `query: its :goal writes … — an (or …) is a whole :goal, … or ask two queries` |
| a `:match` that writes more than 256 distinct variables, the rule's parameters among them, or a `:goal` that does (`ein_ir::MAX_MATCH_VARIABLES`, the register file's size, Q-M1f.73) | `rule 'wide': its :match writes 257 distinct variables, and a rule binds at most 256 — split the rule`, with *its parameters among them* where it has any; a goal's `query: its :goal writes 257 distinct variables, and a goal binds at most 256 — ask two queries` |
| a query's `:goal` premise whose relation is a variable, under `and`, `or` and `absent` | `query: its :goal writes (?R Rex Animal) with ?R as its relation, which nothing binds — a relation is named by a name, or by a rule's parameter its activator binds, and a query has no activator` |

**Until S1f.5.21** each was the compiler's, met where a plan was compiled, in
the Python IR's spelling (*nested `(or …)` in a `:match` premise:
SForm(head=Atom(name='or'), …). Only a TOP-LEVEL `(or …)` is supported
(S1.8.A13 …)*), printed bare by `ein solve`, behind
`ein.inference.compile.CompileError:` by `ein saturate` (and by `ein solve` on
a goal), and behind the file's name by `ein render kb`. A plan is compiled per
activator, by the hypothesis layer for an hrule, and for a goal only where a
state is answered from it, so the commands disagreed: under a commitment
`saturate` and `solve --minimal` answered what describe stopped on, in an
hrule or a goal `saturate` answered, and a rule no activator reached or a goal
of an empty R loaded on every command (W62, W63, W64, sixteen rows with W65's).

**The views refuse what the loader refuses** since the same stage (W65,
Q-M1f.71): `ein render program`, `rules`, `rule` and `constraints` draw a
program as written, its imports unresolved, and read it through the loader
first, so a program it refuses is refused with its line and exit 1, as every
other command refuses it. Until then they parsed it and never loaded it,
so each drew a program every other command refused, exit 0; and `kb save`, which
loads and never compiles, wrote a container of a program every command that
opens one refused. Each view takes `--no-require-why`
([`configuration.md` § 3.4](configuration.md#34-the-option-that-decides-loading)).

Pinned by `ein-ir`'s `from_ir::tests::the_compilers_refusals_a_program_reached_are_refused_at_load`,
`ein-cli`'s `cli_semantics::a_compile_refusal_reads_the_same_on_every_command`
(every witness on seventeen commands, one first line and one exit) and
`render_doors::every_view_refuses_what_the_loader_refuses`,
`corpus_cli::no_cell_prints_the_python_spelling`, `ein-infer`'s
`compile_semantics::no_corpus_program_is_refused_in_the_python_spelling`,
`fuzz_properties::the_compiler_refuses_nothing_the_loader_accepts`, and
`corpus/wrong_answers.toml`'s W62 to W65 rows. **What is not refused**: a
top-level `(or …)`, an `(or …)` as a pattern's argument (no premise), a macro
whose body is an `(or …)` called as a whole `:match`, and a macro no rule uses;
a query's pattern *inside* a pattern whose relation is a variable, `(p (?R
?x))`, loads and matches nothing, as the compiler lowers it, which no
`CompileError` ever reported (found by S1f.5.21's T1, outside its case).

### 4.16 A malformed primitive a macro's expansion writes, at load — M1f S1f.5.24

A macro's body is a template the grammar never reads expanded, and a call takes
any premise as an argument, so a kernel primitive can reach a rule in a shape
the grammar refuses where a rule writes it. Since M1f
[S1f.5.24](../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.24_a_macro_body_checked_where_a_rule_is.md),
on the user's ruling of 2026-11-10 on Q-M1f.67 (*"the ultimate goal in this
case is not allow execution of (eq <arg>)"*), the loader holds a rule's, an
hrule's and an obligation's `:match` and `:assert`, after expansion, to the
grammar's shapes by position (`ein_ir::from_ir`'s `refuse_malformed_primitives`,
Q-M1f.80, Q-M1f.81), and refuses a form in another, one line each,
`; `-joined with every other loader error, exit 1 (`ein test`: 2), on every
command that reads the program, the views and `kb save` among them, before any
path compiles, fires, proposes or owes the rule. **A macro no rule calls is not
read**, so it loads whatever its body (the ruling).

| a macro's expansion writes | message, behind `kb load error: <file>:<line>:<col>: <rule>: its <clause> writes <form>, and …` |
|---|---|
| `eq` or `neq` at any arity but 2, in a `:match` or as a conclusion | `eq takes exactly 2 arguments, not <n> — it compares two values, so write one guard per pair` |
| `absent` at any arity but 1 | `absent takes exactly 1 argument, not <n> — negation as failure is over one premise, so wrap several in an (and …)` |
| `not` of what is no pattern: a conjunction, a disjunction, a `not`, a `neq`, a name, or in a `:match` a variable (`std.macro`'s `unknown` called with a conjunction among them) | `not takes a pattern (relation argument …), not <arg> — it is the stored negation of one fact; negation as failure is (absent …)`; as a conclusion, *a pattern (relation argument …) or a variable* |
| a connective where a value goes: an `(and …)`, an `(or …)` or a `(neq …)` as a guard's argument, a conclusion's or a negated pattern's | `<value> is a connective where a value goes — <c> heads no fact, and a value is a name, a variable or a fact (relation argument …)` |
| `false` with an argument, as a conclusion | `false takes no argument, not <n> — (false) concludes that the state is impossible, and names nothing` |
| an `(or …)` as a conclusion | `an (or …) is no conclusion — a rule concludes every conclusion it writes, and a disjunction is a :match's, one way it holds per premise: write one rule per disjunct` |
| a conclusion that is no form: a body that is its parameter, called with a variable or a name | `kb load error: <file>:<line>:<col>: <rule>: its :assert concludes ?y, which is not a fact — a conclusion is a fact (relation argument …), or an (and …) of facts` |

**The position** is where the expanded form is written (Q-M1f.82): the macro
body's for a form the body writes, `(eq ?a)`; for one the grammar builds for a
keyword, `(not …)`, `(neq …)` or `(or …)`, which keeps none, its first part's,
the call's argument, so a form `std.macro`'s `unknown` writes is placed in the
program's file; in a `.einb`, the stored program, `<program stored in x.einb>`.
**The rule** is named as the loader names it, `hrule 'h'` for an hrule, where
the compiler's line said `rule 'h'`.

**Until S1f.5.24** the compiler refused `eq` and `absent` at another arity where
it compiled the plan (§ 4.3), the firing refused a conclusion that is no form
where it fired (§ 4's table), and nothing read the rest. A plan is compiled per
activator, by the hypothesis layer for an hrule and by the obligation pass for
an obligation, so under a commitment `saturate`, `solve --minimal`, `--single`
and `-n 1` answered what describe stopped on, in an hrule or an obligation
`saturate` answered, and a rule no activator reached loaded on every command
(W66, W69); a `not` of no fact and a connective as a guard's value were
matched (W67), and `(eq k)`, `(false k)`, an `(or …)`, `(not k)` and a
conjunction as a value were concluded, or proposed by an hrule (W68): sixteen
register rows, filed and fixed by the stage. The views and `kb save` accepted
every such program. The compiler's two arms and the firing's line are
invariants since (Q-M1f.84).

**An `(and …)` a macro's call leaves inside a conclusion's `(and …)` is no
malformed primitive**: the grammar's `Conclusion` nests, and since M1f
[S1f.6.37](../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.37_conclusion_shapes.md)
every reader of a conclusion reads it flat at any depth (`ein_ir::Ast::conjuncts`,
the user's ruling of 2026-11-14), so `(and (r a) (both a))`, `both` writing
`(and (p ?x) (q ?x))`, concludes `(r a)`, `(p a)` and `(q a)`. Until then the
compiler split the top `(and …)` alone and stored the inner one as one fact
headed `and` (E-63, W94; `ein-cli/tests/conclusion_shapes.rs`).

Pinned by `ein-ir`'s `from_ir::tests::a_macros_expansion_is_held_to_the_grammars_shapes`
(each shape at each site, and every shape the grammar takes, written and through
a macro, loading), `ein-cli`'s `macro_bodies.rs` (eighteen shapes at every site,
eighty programs, and the sixteen witnesses, each on twenty commands: one first
line, no container, no step of the rule in `--events`; and every malformed
macro, called by no rule, answered by every command),
`grammar_pages::the_lines_the_grammar_does_not_cover_are_still_reached`,
`fuzz_properties::the_compiler_refuses_nothing_the_loader_accepts`, and
`corpus/wrong_answers.toml`'s W66 to W69 rows. **What is not refused**: a
connective as a premise pattern's argument, `(p (and (q ?x)))`, which the
grammar takes so a macro can be called there and which reads nothing; and a
macro called in a query's `:goal`, which is not expanded (both found by
S1f.5.24's T2, outside its case). That was the state at S1f.5.24: the first is
refused since M1f S1f.7.3 (§ 4.22), the second expanded since S1f.7.5
(§ 4.21), and a conclusion `(not ?x)` whose variable holds a name, the third
its T2 found, is refused since S1f.7.4 (§ 4.23).

### 4.17 `open` is written only as a rule's verdict — M1f S1f.5.23

`open` is the verdict a rule's `:assert` states, `(open)` or `(open ?R)`
(§ 4.2), and since M1f
[S1f.5.23](../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.23_open_is_reserved.md),
on the user's ruling of 2026-11-10 on the close review's finding 3 and on
Q-M1f.66 (*"Refuse it"*), it is written nowhere else: it is one of the thirteen
words that are never a name (`ein_ir::grammar::RESERVED_WORDS`), and `ein.y`'s
`Assert` is the one production that takes it. Every other place is a parse
error at the `open`, on every command that reads the program, the views and
`kb save` among them, exit 1 (`ein test`: 2), with a note (Q-M1f.88):

```text
$ ein solve t.ein               # file is: (relation p T) (p (open x))
t.ein:1:20: unexpected `open`
  note: open is reserved: it is the verdict a rule's :assert states, written as the whole :assert, (open) or (open ?R), and nowhere else, so it names no relation and no value
  repair: Delete `open`
```

The places: a fact's, a nested fact's or a stored negation's relation; a
pattern's relation anywhere in a rule, an hrule or a query's `:goal`, and a
claim's fact; a value, as a fact's argument, an object, a type, a premise's,
a guard's or a conclusion's argument, a macro call's, an activator binding a
relation parameter, a goal's or a claim's; a name in a `(solve …)` list, a
query's `:over`, an import, a fact's `:rule` or `:using`, or a trace step; the
verdict's own argument, `(open open)`; and a macro's body, called or not
(Q-M1f.86). A container whose stored program writes one is refused where it is
read, its stored program read back through the parser (Q-M1f.15), and a
`--diff-models` file that lists one is refused at its read. What the loader
still refuses of the verdict is § 4.2's last five, each placed where the
`(open …)` is written (Q-M1f.87).

**Until S1f.5.23** `open` was a word a position gives a meaning to, a name
wherever a name goes, and the loader refused it only in a rule's `:match`,
inside a conclusion and beside one, at no position (`at None`). So of the
fifty-two places a program can write it, thirty-eight loaded and were
answered: `(p (open x))` (the finding) and every fact of `open`; a goal's
pattern and a claim's fact; a relation parameter bound to it, which read a
relation `open`, or an obligation's, which every state owed; `open` as a value,
a name of the program the blind enumerator proposed facts about; and a
macro's body stating the verdict for a rule. Twenty-four register rows, W70 to
W72, filed and fixed by the stage (Q-M1f.89). No corpus program wrote `open`
but as a rule's whole `:assert`, so no corpus answer moved.

Pinned by `ein-cli`'s `open_reserved.rs` (the fifty-two places and the
twenty-four witnesses, each on twenty commands: one first line at an `open`,
the note under it, no container, nothing fired; the verdict loading where a
rule's or an hrule's `:assert` states it, answered by every command and its
container; and a container that stores `open` as a name refused), `ein-ir`'s
`grammar_pages` table (a row per place, banked in `grammar_table.txt`) and
`DELETED`, `grammar::tests::the_reserved_words_are_the_printers`, and
`corpus/wrong_answers.toml`'s W70 to W72 rows. **What is not refused**: the
position words a fact's head admits, each a name by Q-M1e2.17 (`eq` checked,
S1f.5.7's). A written `(false k)` and a written `(absent k)`, which S1f.5.23's
T2 found outside its case, are refused since M1f S1f.7.6 and S1f.7.7
(§ 4.22).

### 4.18 An imported module is read as the program is — M1f S1f.5.22

An imported module on disk, a file-relative one or a `std.*` one under a
checkout or `$EIN_STDLIB`, is read through the read every command makes of
the program it is given (`ein_ir::read::read_program`), and a module that
exists and cannot be read as text is refused with that read's line, behind
`kb load error: `, on every command (exit 1, `ein test` 2), naming the path
the import resolves to
([Q-M1f.93](../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f93--which-path-a-modules-read-line-names)):

| the module | the line |
|---|---|
| not UTF-8 | `kb load error: m.ein:3:4: byte 0xff is not UTF-8: a program is UTF-8 text, or a .einb container`, at its first such byte |
| unreadable | `kb load error: PermissionError: [Errno 13] Permission denied: 'm.ein'` |
| a directory | `kb load error: IsADirectoryError: [Errno 21] Is a directory: 'm.ein'` |
| a `.einb` container | `kb load error: m.ein: a .einb container, and an imported module is Ein text: import the .ein it was saved from` ([Q-M1f.91](../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f91--a-module-that-is-a-einb-container)) |
| holding a character that is not ASCII outside a string | `kb load error: <its resolved path>:2:5: 'é' (U+00E9) is not ASCII: …` (§ 1.6), as any parse error in a module names its resolved path |
| missing | `kb load error: (import m) — module not found at m.ein (None)`, unchanged |

The first three are the lines the module's path gives to `ein solve` as a
program, the directory's and the unreadable file's in the Python spelling
F23's D-1 owns for both; the missing module's `(None)` is the import's
position, which a top-level form does not carry, F23's D-2 with every other
`at None` (§ 1.5,
[Q-M1f.94](../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f94--a-directorys-and-an-unreadable-modules-line-and-a-missing-ones-none)).

**Until S1f.5.22** the resolver read a module with
`std::fs::read_to_string(…).ok()` behind `is_file()`, so each of the first
four was `(import m) — module not found at m.ein (None)`, a path that exists
reported as missing with a Rust `None` (the close review's finding 2; W74,
nine rows, on a module that is a directory, the one way git can keep). What
pins it: `ein-cli/tests/unicode_only_in_a_string.rs`'s
`an_imported_module_is_read_as_the_program_is`, each way on twenty commands,
and each line held to the one the module's path gives as a program.

### 4.19 A closed stdout ends `ein` by `SIGPIPE` — M1f S1f.5.25

> **New 2026-11-10** — [S1f.5.25](../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.25_a_closed_stdout.md), on the user's
> ruling of that day on the close review's finding 5,
> [Q-M1f.68](../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f68--what-ein-does-when-its-reader-closes-stdout) (b):
> *"why not handle sigpipe properly?"*.

A write to a pipe whose reader has gone, `ein saturate x.ein | head`, a pager
quit before the end, a closed socket, ends the process by `SIGPIPE` at that
write, as it ends `cat`, `grep` and `head`. Nothing is printed, and a shell
reports 141, 128 + 13, so `set -o pipefail` reads `ein` as it reads them.
That holds for every command and every option that writes to stdout, the help
and `--version` among them:

| | until S1f.5.25 | since |
|---|---|---|
| a write to stdout after its reader has gone | `thread 'main' (…) panicked at …:` and *failed printing to stdout: Broken pipe (os error 32)* on stderr, exit 101, the close review's finding 5 | ended by `SIGPIPE`, nothing on stderr |
| the help, `clap`'s | exit 0, its write's failure ignored and the help lost | ended by `SIGPIPE` |
| an artefact flag naming a pipe whose reader has gone, `--events /dev/stdout` or a FIFO | `error: --<flag> <path>: Broken pipe (os error 32)`, the exit unchanged (§ 4.4) | ended by `SIGPIPE` ([Q-M1f.100](../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f100--an-artefact-flag-that-writes-to-a-pipe-whose-reader-has-gone)) |
| a run whose writes all landed in the pipe before its reader closed | exit 0 | exit 0, as `cat`'s |

**Under `| head` it was a race.** A run whose only write landed before
`head` exited answered 0, `ein solve x.ein | head` among them, its read-out
one `print!`; one that wrote again after it panicked. What differs now is the
second case alone: a run that finishes writing before its reader closes has
seen no closed pipe, and exits 0 as `cat` does.

**It is not an exit code.** *Exit non-zero only on error* is a rule about the
codes Ein returns, and a process the kernel ends returns none (§ 5's table is
the codes). The disposition is the binary's: Rust's runtime sets `SIGPIPE` to
ignored before `main`, and `ein-cli/src/main.rs`'s `restore_sigpipe` restores
the default as `main`'s first statement, one audited `libc::signal` call on
Unix, the second `unsafe` site beside `ein-einb`'s `cast.rs`. A program that
embeds the engine links the library, not `main.rs`, and keeps its own
disposition ([`docs/api/rust.md`](../api/rust.md)). Windows has no `SIGPIPE`
([Q-M1f.101](../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f101--a-closed-stdout-on-windows)).

What pins it: `ein-cli/tests/closed_stdout.rs`, sixty-eight runs, every
subcommand and every option that writes to stdout, each read in full, with
its reader closed before the run, which ends every one by `SIGPIPE` with
nothing on stderr, and with a reader that closes after the first line, which
ends by `SIGPIPE` every run that must write again; and the register's eighteen
W75 rows, a command or a `solve` question each, run with the reader closed
before the run (schema `/21`).

### 4.20 A `(solve …)` list names what the program has — M1f S1f.7.8

The three lists of a `(solve …)`, `:hrules`, `:hypothesis-relations` and
`:no-hypothesis`, choose the candidates, and so the lattice the answer is a
region of. Since M1f
[S1f.7.8](../../plans/m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/s1f.7.8_a_solve_list_naming_nothing.md)
every name one holds names something the program has, or the loader refuses
the program, one line per name, placed at the name, on every command that
reads it, the views and `kb save` among them, exit 1 (`ein test`: 2)
(`ein_ir::from_ir`'s `refuse_solve_names`, Q-M1f.114 to .116, defaulted):

| the list writes | the line, after `kb load error: <file>:<line>:<col>: solve: its ` |
|---|---|
| `:hrules (gues …)`, and no hrule is named `gues` | `:hrules names gues, and the program declares no hrule gues — it declares guess` |
| `:hrules (guess (at Color))`, and `guess` takes three arguments | `:hrules gives hrule 'guess' (at Color), 2 arguments, and guess takes 3, (?R ?T1 ?T2) — an activator binds each parameter once` |
| `:hrules (guess (att Color House))`, `?R` naming a relation, and no relation is named `att` | `:hrules gives hrule 'guess' (att Color House), whose att is its parameter ?R, which names a relation, and the program names no relation att — a relation of the program is one it declares, states a fact of, or writes in a rule, an hrule or a goal` |
| `:hypothesis-relations (like)` or `:no-hypothesis (like)`, and no relation is named `like` | `:no-hypothesis names like, and the program names no relation like — a relation of the program is one it declares, states a fact of, or writes in a rule, an hrule or a goal` |

**A relation of the program** is a name the program writes as a relation
outside the three lists: one it declares or states a fact of; `is-a`, `eq` and
`neq`; the relation of any form in a rule's, an hrule's or an obligation's
`:match` or `:assert`, in a query's `:goal` or in a fact's argument; and a
name a fact or a conclusion passes where a relation's name is held, as
`(bijective color-loc)` passes `color-loc` (§ 4.9's positions). So a relation
only a rule's or an hrule's `:assert` derives is one, an imported rule's
included, where a claim's relation must be declared or a fact's. The test is
*named elsewhere*, not *made by something*, because an activator's hrule makes
the relation it is given, `att` as it makes `at`: what tells a misspelt name
is that the program writes it nowhere else (Q-M1f.114). A parameter names a
relation where it heads a form of the hrule, or is passed on to a position
that holds one. A program the loader has already refused gets no line from
this check.

**What is not refused here.** A list that names what the program has and
contradicts another of its hypothesis declarations, an hrule that proposes a
relation `:no-hypothesis` names among them, is § 4.27's. An activator's
argument at a parameter that names no relation is a value, a type or an object, which no declaration lists, so
`(guess (at Colour House))` loads and proposes nothing, and the read-out's
`universe` row says *no candidate*; so does a parameterised hrule that no
activator names. A bare name in `:hrules` selects nothing: every hrule
without parameters proposes whether the list names it or not. All three are
[F25](../../plans/followups/f25_roughness_found_outside_zebra.md)'s (R-2,
R-3).

**Until S1f.7.8** a name that matched nothing was read as one that matches
none, in silence, and one misspelt character moved H and R: on the stage's
witnesses `gues` for `guess` and an activator of two arguments left H empty
and R {∅} where the program meant four states, `att` for `at` proposed three
facts of a relation `functional at` does not read, R eight states, `like` in
`:hypothesis-relations` left H empty, and `like` in `:no-hypothesis` excluded
nothing. Five register rows, W76, filed and fixed by the stage. One corpus
fixture named hrules it did not declare, `examples/syntax/query-full.ein`,
and declares them since; no other corpus answer moved.

Pinned by `ein-cli`'s `solve_lists.rs` (nine shapes and the five witnesses,
each on twenty commands: one first line, the same on each, and no container;
and fourteen programs whose lists name what the program has, each loading)
and `corpus/wrong_answers.toml`'s W76 rows.

### 4.21 A goal is held to what a rule's `:match` is held to — M1f S1f.7.5, S1f.7.11, S1f.7.1

A query's `:goal` is a premise. Since M1f
[S1f.7.5](../../plans/m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/s1f.7.5_a_macro_called_in_a_goal.md),
[S1f.7.11](../../plans/m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/s1f.7.11_a_goal_over_a_relation_nothing_makes.md)
and
[S1f.7.1](../../plans/m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/s1f.7.1_a_goals_variable_relation_in_a_nested_pattern.md)
the loader reads it as it reads a rule's `:match`, on every command that reads
the program, the views and `kb save` among them (Q-M1f.117 to .121,
defaulted):

- **A macro call in a goal is expanded** (`ein_ir::from_ir`'s `expand_goal`),
  as a rule's `:match`'s is, so `(unknown (paid ?g))` in a goal is `(and
  (absent (paid ?g)) (absent (not (paid ?g))))`. The expansion is what is
  matched, what a claim's facts are held to and what a class lists; the
  read-out, the summary and `ein test` print the goal as written (Q-M1f.117).
  A macro call in a claim is not expanded: a claim lists facts, and one headed
  by a macro's name names no relation (Q-M1f.118).
- **The expanded goal gets the checks a rule's `:match` gets**: one walk reads
  both (`refuse_no_fact_holds`, `refuse_nested_disjunctions`,
  `refuse_wide_matches`, `refuse_malformed_primitives`,
  `macros::unimported_macro_errors`).
- **Every relation a goal asks about, and every relation a claim lists a fact
  of, is a relation of the program** (`program_relations`): § 4.20's notion
  without the goals, so one the program declares, states a fact of, or writes
  in a rule, an hrule or an obligation, at any depth, a rule's `:assert`
  among them, and one a `(solve :hrules …)` activator has its hrule propose.
  `is-a`, `eq`, `neq` and the kernel's own relations, `(relation ?R ?T)`, are
  every program's.

| the goal writes | the line, after `kb load error: ` |
|---|---|
| `(unknown (paid ?g))`, and the program does not import `unknown` | `query: '(unknown …)' is a std.macro pattern macro used without importing it — add (import std.macro :symbols (unknown)); unexpanded it would silently never match` |
| `(settled ?g Guest)`, and the macro `settled` takes one argument | `query: its :goal: macro settled/1 invoked with 2 args at <file>:<line>:<col>` |
| a call of a macro that expands to itself | `query: its :goal: macro expansion exceeded depth 50 — a macro likely expands to itself` |
| a call whose expansion leaves a name where a premise goes | `query: its :goal writes Ann as a premise of and, and Ann is no premise — a premise is a pattern (relation argument …), a guard (eq a b) or (neq a b), an (absent …), or an (and …) or (or …) of premises` |
| a call whose expansion writes a kernel primitive in another shape (§ 4.16) | `<file>:<line>:<col>: query: its :goal writes (eq ?g), and eq takes exactly 2 arguments, not 1 — it compares two values, so write one guard per pair` |
| `(payed ?g)`, at any depth, under `(absent …)` and `(not …)` too, and no relation is named `payed` | `<file>:<line>:<col>: query: its :goal writes (payed ?g), and the program names no relation payed — a relation of the program is one it declares, states a fact of, or writes in a rule or an hrule, so no state holds a fact of payed` |
| `(says (?R ?g))`, a relation written as a variable inside a pattern, under `(absent …)` too | `query: its :goal writes (?R ?g) with ?R as its relation, which nothing binds — a relation is named by a name, or by a rule's parameter its activator binds, and a query has no activator` |

A line that names a place names where the pattern writes its relation. A
program the loader has already refused gets no line about its relations, as
in § 4.20. A relation only a rule's `:match` reads is a relation of the
program, so a goal over it loads and has no row: a library rule may read a
relation its importer supplies, and a rule that can never fire is reported
nowhere ([F25](../../plans/followups/f25_roughness_found_outside_zebra.md)'s
R-6, Q-M1f.120).

**Until these stages** a macro call in a goal was read as a pattern over a
relation of the macro's name: the goal had no row in any state, `:expect
(model)` held of it, and the claim that lists what the macro means was
refused at load, *lists …, which no atom of the query's :goal matches*. A
goal over a relation the program names nowhere had no row, as a goal that is
undetermined has none, and under `(absent …)` it held of everything. A
pattern inside a pattern whose relation is a variable was lowered as a slot
that matches nothing, so `(says (?R ?g))` had no row beside `(says (paid
Bob))`, and under `(absent …)` the goal held in every state; a rule writing
the shape is refused since M1e2 S1e2.2.24 (§ 4.9), and a goal's top-level one
since M1f S1f.5.21 (§ 4.15). And a claim listing a fact of a relation only a
rule's `:assert` derives was refused, *which no declaration or fact makes a
relation*. Ten register rows, W77 to W79, filed and fixed by the stages. The
read-out's class of a goal holding an `(and …)` inside its `(and …)`, which a
macro's expansion writes, lists the inner premises one by one since, as the
loader reads them for a claim; until then it listed `(and …)`, which no claim
can write.

Pinned by `ein-cli`'s `goal_premises.rs` (nineteen shapes and the six
witnesses whose right answer is a refusal, each on twenty commands: one first
line, the same on each, and no container; nine programs answered, the goal
printed as written; and sixteen goals over relations of the program, each
loading, a container answering as its text) and `corpus/wrong_answers.toml`'s
W77, W78 and W79 rows.

### 4.22 A value is a name, or a fact a state could hold — M1f S1f.7.7, S1f.7.6, S1f.7.10, S1f.7.3

The user's ruling of 2026-11-11
([the value rule](../../plans/m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/README.md#the-value-rule), Q-M1f.113): *a value is a
name, or a fact a state could hold: an atom of an ordinary relation, or its
negation.* A fact inside a fact, a pattern inside a pattern, and a
conclusion's or a guard's argument are values. Four stages hold the language
to it, on every command that reads the program, the views and `kb save`
among them (Q-M1f.122 to .126, defaulted):

- **`absent` heads no fact, value or conclusion**
  ([S1f.7.7](../../plans/m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/s1f.7.7_an_absent_fact.md)). It is negation as failure, a
  test a premise or a goal makes of a state, `(absent <premise>)`. A claim's
  class still lists what a goal's `(absent …)` says a state does not hold,
  `(model (absent (q a)) (p a))` (`ein.y`'s `ClaimAtom`).
- **`false` takes no argument**
  ([S1f.7.6](../../plans/m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/s1f.7.6_a_false_fact_with_an_argument.md)): a fact, a value,
  a premise, a goal and a conclusion write it only as `(false)`. `(false)` as
  a value, `(q (false))`, stays (Q-M1f.123).
- **`(eq …)` is no value**, as `(neq …)` never was
  ([S1f.7.10](../../plans/m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/s1f.7.10_an_eq_fact_as_a_value.md)): `eq` is computed and
  never stored (§ 4.14, M1f S1f.5.7). A fact `(eq a b)` and `(not (eq a b))`,
  a premise and a conclusion of either, and a claim's, stay, each checked
  (`ein.y`'s `EqFact` and `Negated`); `(not (eq a b))` inside a fact or a
  pattern is refused (Q-M1f.124).
- **A premise pattern's argument is what a value can be**
  ([S1f.7.3](../../plans/m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/s1f.7.3_a_connective_as_a_patterns_argument.md)): a name, a
  variable, a pattern of an ordinary relation, or `(not <pattern>)`, a stored
  negation. `and`, `or`, `neq`, `eq`, `absent` and `false` with an argument
  there are refused, in a rule, an hrule, an obligation and a goal. The
  grammar takes a connective as a pattern's argument, since a macro such as
  `forall` is called as a pattern and takes premises, so this refusal is the
  loader's, read off the clause once its macros are expanded
  (`ein_ir::from_ir`'s `Shapes::arg`, § 4.16's walk; Q-M1f.125).

The grammar refuses each shape a program writes, at the parse (`ein.y`'s
`FactHead`, `ValueHead`, `NestedFact`, `Premise`, `Pattern` and `Negated`;
[`00_ebnf.md`](ir/03-ein-lang/00_ebnf.md)), and the error carries a note:

| the program writes | the error is at | the note |
|---|---|---|
| `(absent k)`, `(absent (p a))`, `(q (absent k))`, `(not (absent k))`, `:assert (absent (p ?x))` in a rule or an hrule, `:assert (q (absent ?x))` | `absent`: *unexpected \`absent\`* | `absent is negation as failure, a test a premise makes of a state, (absent <premise>) in a :match or a :goal, so no fact, value or conclusion is of it: the stored negation of a fact is (not …)` |
| `(false k)`, `(q (false k))`, `(not (false k))`, a premise or a goal `(false ?s)`, `(absent (false ?s))`, a pattern's argument `(q (false ?s))`, `:assert (q (false ?x))`, a claim's `(false k)` | the argument: *unexpected \`k\`: a name* | `false takes no argument: (false) says that the state is impossible, and names nothing` |
| `(q (eq k j))`, `(q (not (eq k j)))`, `:assert (q (eq ?x ?y))`, a guard's `(eq ?y (eq ?x b))`, a claim's `(p (eq a b))`; `neq` in each | `eq`: *unexpected \`eq\`* | `eq is computed and never stored, so (eq a b) is no value: write it as a premise or a conclusion of its own, where it is checked, and never inside another fact or pattern` |

The loader refuses what the grammar cannot see, one line per form, in
§ 4.16's spelling, placed where the expanded form is written:

| the clause holds, once its macros are expanded | the line, after `kb load error: <file>:<line>:<col>: ` |
|---|---|
| a pattern's argument `(and …)`, `(or …)` or `(neq …)`, under `(absent …)` too | `rule 'r': its :match writes (q (and (p ?x))), and (and (p ?x)) is a connective where a value goes — and heads no fact, and a value is a name, a variable or a fact (relation argument …)` |
| a pattern's argument `(eq ?a ?b)`, or a value of it in a conclusion | `rule 'r': its :match writes (q (eq ?a ?b)), and (eq ?a ?b) is written where a value goes — eq is computed and never stored, and a value is a name, a variable, or a fact a state could hold, (relation argument …) or its negation` |
| a pattern's argument `(absent …)`, as `forall` called inside a pattern leaves | `rule 'r': its :match writes (q (absent (q ?x))), and (absent (q ?x)) is written where a value goes — absent is negation as failure, a premise's test that no fact is of, and a value is a name, a variable, or a fact a state could hold, (relation argument …) or its negation; the stored negation of a fact is (not …)` |
| a value `(false ?x)` | `rule 'r': its :match writes (q (false ?x)), and (false ?x) is written where a value goes — false takes no argument, and a value is a name, a variable, or a fact a state could hold, (relation argument …) or its negation` |
| a premise `(false ?x)` | `rule 'r': its :match writes (false ?x), and false takes no argument, not 1 — (false) holds where the state is impossible, and names nothing` |
| a conclusion `(absent ?x)` | `rule 'r': its :assert writes (absent ?x), and absent is negation as failure, a premise's test, and no fact is of it — a conclusion is a fact (relation argument …); the stored negation of a fact is (not …)` |

A goal's line reads `query: its :goal writes …`. **A relation parameter that
an activator can bind to one of the three words** is read as § 4.14 reads one
that can hold `eq` (`refuse_eq_at_another_arity`, once per word): it is
refused for each premise or conclusion it heads that the grammar refuses
written out, `(absent ?x)` at any arity and `(false ?x)` at one argument or
more, and for each form it heads where a value goes, `(q (?R ?x ?y))` under
`(mk eq)` (Q-M1f.126). These lines name the rule and have no place:

```
kb load error: rule 'mk': its parameter ?R heads (?R ?x), a form of 1 argument, and the fact (mk absent) binds ?R to absent — absent is negation as failure, a premise's test, and no fact is of it
kb load error: rule 'mk': its parameter ?R heads (?R ?x ?x), written where a value goes, and the fact (mk eq) binds ?R to eq — eq is computed and never stored, and a value is a name, or a fact a state could hold
```

The three words stay names: `(h absent)`, `(h false)` and `(mk eq)` are
facts.

**Until these stages** each shape loaded. A fact `(absent (paid Ann))`,
written where `(not (paid Ann))` was meant, was stored as a fact of a
relation `absent` that no premise can read, and the state stayed alive
beside `(paid Ann)`; a written `(absent Bob)` made `Bob` a name of the
program, so the blind enumerator's H grew; a rule's `:assert (absent …)`
stored one and an hrule's proposed one, splitting R on a fact nothing reads.
A written `(false Bob)` was stored with its argument and killed root; a
premise `(false ?s)` matched nothing, so `(absent (false ?s))` held and its
rule fired. A value `(eq Ann Bob)` was stored, a pattern's argument `(eq ?a
?b)` matched it by its shape, deriving a conclusion of two names that are
not equal, and `(not ?s)` concluded of a stored `(eq k k)` stored `(not (eq k
k))` and no `(false)`, where a concluded `(not (eq k k))` is checked. And a
connective as a pattern's argument was lowered as a pattern over a relation
`and`, `or` or `neq`, which no fact holds: the rule never fired, and under an
`(absent …)` it fired for every binding. Nineteen register rows, W80 to W83,
filed and fixed by the stages.

Pinned by `ein-cli`'s `value_rule.rs` (fifty-five shapes and the nineteen
witnesses, each on twenty commands: one stderr, the same on each, its first
line at the shape and its note under it, and no container; nineteen programs
that keep loading, `(false)`, a stored negation as a value, `(not (eq a b))`
written of its own, `absent` in a claim's class and the three words as names
among them; and the rule of a witness written as it was meant, answered),
`ein-ir`'s
`grammar_pages` table (a row per place the grammar takes the refusal, banked
in `grammar_table.txt`) and `corpus/wrong_answers.toml`'s W80 to W83 rows.

### 4.23 `not` of a name is the negation of no fact, and an hrule may read `not` — M1f S1f.7.4, S1f.7.2

Two stages of M1f
[P1f.7](../../plans/m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/README.md),
both about `not`, the one head § 4.22's rule keeps as a value's.

**A conclusion `(not ?x)` is of a variable bound to a fact**
([S1f.7.4](../../plans/m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/s1f.7.4_the_negation_of_a_name_concluded.md),
Q-M1f.127 and Q-M1f.128, defaulted). The grammar takes `(not Var)` as a
conclusion, as `(hypothesis ?h)` then `(not ?h)` writes it, and whether `?h`
holds a fact depends on the data. A name has no negation: `(not k)` is a parse
error as a fact and as a conclusion, and § 4.16 refuses one a macro writes.
So the loader refuses the conclusion wherever the program shows that the
variable can hold a name (`ein_ir::from_ir`'s `refuse_negated_names`), one
line per conclusion, in § 4.16's spelling, at the variable, on every command
(exit 1, `ein test`: 2):

```
kb load error: <file>:<line>:<col>: rule 'deny': its :assert writes (not ?g), and ?g can be a name: (paid ?g) binds ?g where the fact (paid Ann) holds Ann — not takes a pattern (relation argument …) or a variable bound to a fact: it is the stored negation of one fact, and a name is no fact; negation as failure is (absent …)
```

A position, a relation and an argument's index, *holds a name* where the
program shows one there, and the line's middle says which:

| the program shows | the line's middle |
|---|---|
| a written fact, at any depth and under a `(not …)` | `(paid ?g) binds ?g where the fact (paid Ann) holds Ann` |
| a `(relation …)` declaration, matched as a fact | `(relation ?R ?T) binds ?R where a declaration names a relation or a type` |
| a rule's or an hrule's conclusion that writes a name, or a variable that can hold one, to a fixpoint | `(seen ?g) binds ?g where rule 'greet' writes (seen ?g)` |
| the rule's own activator, a written fact | `?g is its parameter, which the fact (deny Bob) binds to Bob` |
| an hrule's activator in `(solve :hrules …)` | `?g is its parameter, which (solve :hrules (suppose (Bob))) binds to Bob` |
| a declared relation the hypotheses are taken from: the program declares no hrule, the `(solve …)` lists leave the relation in, and nothing writes the closure marker | `(seen ?g) binds ?g where a commitment can hold a name, seen being a declared relation the hypotheses are taken from` |
| nothing but a guard binds the variable | `(neq ?g Ann) binds ?g over the program's names, nothing else binding it`, `(eq ?g Ann) binds ?g to a name` |

The variable *can hold a name* where, in one disjunct of the `:match`, every
relation premise that binds it binds it at such a position. A premise under
`(absent …)` binds nothing, and a guard beside a relation premise only
filters. So `(and (paid ?g) (claim ?c))` with `(not ?c)` loads where `claim`
holds facts alone, and a position that holds both, `(claim (paid Bob))` beside
`(claim Ann)`, is refused. The three
`examples/saturation/hypothesis-contradiction/` programs load: their
`hypothesis` is no declared relation, and holds facts.

A pattern or a conclusion whose relation is a parameter, `(?R ?g)`, is read as
one of each relation a written activator names there: `(rule deny (?R) :match
(?R ?g) :assert (not ?g))` beside `(deny paid)` is refused, *(?R ?g) binds ?g
where the fact (paid Ann) holds Ann*.

**What the loader does not read is the firing's.** Where the activator is
derived through a variable, `(marks paid)` and a rule concluding `(deny ?R)`
of what is marked, the parameter's relation is not read, and the program
loads. Where a binding then gives `(not ?g)` a name, there is no fact to
conclude: the firing stores nothing for that conclusion, and stores the rule's
other conclusions; an hrule proposes nothing for it
(`ein_infer::firing::build_fact`). No command stops, no `(false)` is
concluded, and no state holds the negation of a name. A state file of such a
program writes the derived activator as a fact, so the loader reads it there:
[F25](../../plans/followups/f25_roughness_found_outside_zebra.md)'s R-1.

**An hrule's `:match` may read `not`**
([S1f.7.2](../../plans/m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/s1f.7.2_an_hrule_that_reads_not.md),
Q-M1f.129, defaulted): a stored negation as a premise, `(not (paid ?g))`,
under `(absent …)`, and through `std.macro`'s `unknown`. It is matched against
the state the hypotheses are taken at, and `ein solve` answers what `ein
oracle` answers, in a build with debug assertions as in a release build.
Where an hrule reads `not`, every hrule is matched before any candidate is
emitted (`ein_infer::hrule`'s `Hrules::candidates`), so nothing the hypothesis
pipeline writes between two candidates is visible to the enumeration. No
command's pipeline writes there: H is taken config-free since M1e2 S1e2.3.1,
with the lookahead's kill cache off.

**Until these stages** a rule whose `(not ?g)` met a name stored `(not Ann)`,
which no program can write, and the state file `ein solve --dump-answer` wrote
for that state did not reload, *unexpected \`Ann\`: a name*; an hrule's put
`(not Ann)` in H, so R held two states that differed in a fact nothing reads.
And an hrule whose `:match` read `not` stopped a build with debug assertions,
exit 101, *an hrule reads \`not\`, which hypgen's kill cache writes
mid-enumeration*, on every solve question, `ein oracle` and `ein test`, where
a release build answered. Ten register rows, W84 and W85, filed and fixed
by the stages.

Pinned by `ein-cli`'s `negated_names.rs` (eighteen refused shapes and W84's
seven witnesses, each on twenty commands: one stderr, the same on
each, and no container; ten programs that keep loading, and the three corpus
programs, each storing the negation it stored; and three programs whose name
only a run meets, each storing and proposing none, no state file holding one,
`ein solve` and `ein oracle` counting the same states, read as text and
through a `.einb`), `ein-cli`'s `hrule_reads_not.rs` (W85's three
witnesses on thirteen commands under `cargo test`'s debug assertions, and two
hrules of which one reads what the other could be asked about), `ein-ir`'s
`from_ir` unit test `a_conclusion_negating_a_name_is_refused_at_load`, and
`corpus/wrong_answers.toml`'s W84 and W85 rows.

### 4.24 A macro's call is no fact — M1f S1f.7.12

A stage of M1f
[P1f.7](../../plans/m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/README.md),
[S1f.7.12](../../plans/m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/s1f.7.12_a_macro_call_written_as_a_fact.md)
(Q-M1f.130 to .133, defaulted).

**A macro is called where the loader expands it**: in a rule's or an hrule's
`:match` and `:assert`, at any depth, and in a query's `:goal` (§ 4.21). There
the call is replaced by the macro's body. Nothing else is expanded, so a form
headed by a macro's name anywhere else is no call. In a fact it is refused at
load (`ein_ir::from_ir`'s `refuse_macro_facts`), one line per call, at the
call's name, on every command (exit 1, `ein test`: 2):

```
kb load error: <file>:<line>:<col>: the fact (settled Ann) is a call of the macro settled, (macro settled (?g) (paid ?g)) — a macro is called in a :match, an :assert or a :goal, where its call is replaced by its body; a fact is not expanded, and no fact is of a macro: write the fact itself
```

| the program writes | the line, after the position |
|---|---|
| a fact headed by a macro's name, at any arity | `the fact (settled Ann) is a call of the macro settled, (macro settled (?g) (paid ?g)) — …` |
| the call inside a fact, at any depth, under `(not …)`, as an `is-a` fact's argument or in a fact's `:using` | `the fact (says (settled Ann)) holds (settled Ann), a call of the macro settled, (macro settled (?g) (paid ?g)) — …` |
| an imported macro, flat or qualified (`std.macro.unknown`) | `the fact (unknown (paid Bob)) is a call of the macro unknown, (macro unknown (?P) (and (absent ?P) (absent (not ?P)))) — …` |
| a `std.macro` macro the program does not import, `unknown` or `forall` | `the fact (unknown (paid Bob)) is a call of std.macro's macro unknown, which the program does not import — …` |
| the call inside a fact a claim lists, a query's or a `(solve … :expect (region …))`'s | `query: its :expect lists (settled Ann), a call of the macro settled, (macro settled (?g) (paid ?g)) — a claim lists facts, and …` |
| a `(relation …)` declared under a macro's name | `relation 'settled': settled is a macro too, (macro settled (?g) (paid ?g)) — a premise (settled …) is the macro's call, replaced by its body, and a fact (settled …) is refused, so nothing could be of the relation; give one of the two another name` |

Every line but the last ends *a macro is called in a :match, an :assert or a
:goal, where its call is replaced by its body; a fact is not expanded, and no
fact is of a macro: write the fact itself*.

**The call is not expanded into the fact its body spells** (the stage's
default (a)). A macro's body is a pattern: `unknown`'s is two `absent`s, a
test of a state, and spells no fact. Expanding only a body that is one ground
form would give a macro a second meaning.

**What stays.** A call as a conclusion, `:assert (settled ?g)`, is expanded,
as it was before the stage, and stores the body's fact, `(paid Ann)`; an
expansion that is no conclusion is refused by § 4.16
([Q-M1f.130](../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f130--a-macros-call-as-a-conclusion-refused-or-expanded-as-it-is)).
A macro's name written as a value, `(word settled)`, is a name. A claim headed
by the call, `(model (settled Ann))`, names no relation of the program and
keeps § 4.21's line (Q-M1f.118); the line above is for a program nothing else
refuses. A relation named `unknown` or `forall` in a program that does not
import `std.macro` is not refused where it is declared: no fact of it can be
written, by the fourth row, and no premise read, since S1.8a.f20.

**Until this stage** `(settled Ann)` beside the macro was stored as a fact of a
relation `settled`, which no premise could read, a premise `(settled ?g)`
being the call. `(paid Ann)` was not stored, so a rule over the macro derived
nothing, and no line said so. `(not (settled Bob))` beside `(paid Bob)` left
root alive. A claim that listed `(says (settled Ann))` loaded and failed, the
listed fact *no class*. Eight register rows, W86, filed and fixed by the
stage.

Pinned by `ein-cli`'s `macro_facts.rs` (fifteen refused shapes and W86's eight
witnesses, each on twenty commands: one stderr, the same on each, and no
container; seven programs that keep loading, each storing what its call's
body spells and no fact of the macro's name), `ein-ir`'s `from_ir` unit test
`a_macros_call_written_as_a_fact_is_refused_at_load`, and
`corpus/wrong_answers.toml`'s W86 rows.

### 4.25 An import brings a module's facts — M1f S1f.6.11, S1f.6.35

A stage of M1f
[P1f.6](../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/README.md),
[S1f.6.11](../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.11_importing_facts.md),
on the user's ruling of 2026-11-11, *"importing facts is mandatory"*
(Q-M1f.135 to .144, defaulted); since
[S1f.6.35](../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.35_imports_by_full_name.md),
on the user's ruling of 2026-11-14, *"in all cases we import everything, the
difference only in names"* (§ 4.28).

**Every import brings every declaration and every fact of its module**, with
the module's own imports, one copy per program, under full names
([`01_grammar.md` § Import](ir/03-ein-lang/01_grammar.md#import)):

| of the module | `(import M)` | `(import M :as o)` | `(import M :symbols (n …))` |
|---|---|---|---|
| a declared relation, a rule, an hrule, a macro | kept, written `M.name` | kept, written `o.name` | kept, written `name` if listed and `M.name` if not |
| a fact of a declared relation | kept: `(M.is-a Tea Drink)` | kept: `(o.is-a Tea Drink)` | kept: `(is-a Tea Drink)` where `is-a` is listed |
| a fact of a relation no file declares | kept as written | kept as written | kept as written; the relation may be listed, a name only (Q-M1f.250) |
| a stored negation, `(not (likes Milk Tea))` | kept, as a fact is | kept, as a fact is | kept, as a fact is |
| a constant in a fact | as written | as written | as written |
| `(query …)`, `(solve …)`, `(config …)`, `(trace …)` | **never brought** since M1f S1f.6.25 (W90) | never brought | never brought |

- **No import brings what a module states as a program** (M1f
  [S1f.6.25](../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.25_the_section_j_fixes.md),
  E-8, W90;
  [Q-M1f.215](../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f215--a-qualified-import-and-what-a-module-states-as-a-program)):
  its queries, its `(solve …)`, its `(config …)` and a `(trace …)` are
  dropped where the module is read. The importer's lattice, flags and queries
  are the ones it writes.
- **A listed name** is one the module declares or states a fact of. Any other
  is refused, on every command: `kb load error: (import M :symbols …) — not
  provided by the module: n at …`.
- **An imported fact is a fact of the program**: its provenance is its own, a
  `:source` or none, with no stamp of the module; its constants are names of
  the program, and enter the blind enumerator's objects.
- **A fact of a module arrives once**, the module being one copy, whoever and
  however many import it. The same fact stated twice with two `:source`s, by a
  module and the program or by two modules, keeps its first statement and
  loses the second with no word, as it does written twice in one file: stated
  here, not ruled, and
  [F24](../../plans/followups/f24_written_provenance.md)'s to decide.

**From S1f.6.11 to S1f.6.35** a selection kept the listed names, the closure
of what their declarations write and the facts of every relation in it,
unrenamed, so an unlisted rule stayed behind even where the facts would fire
it, and the loader said so in a `note:` line where a selection brought a
relation's declaration and no rule that derives it (S1f.6.25, E-20,
Q-M1f.217); a qualified import renamed a copy of the module's subtree under
its prefix, so the facts of `o.is-a` never joined the program's `is-a`, and two
prefixes were two copies. Nothing is left behind since, and the line is
retired. **Until S1f.6.11** a selection kept declarations alone, and a
module's facts were dropped with no word; four register rows, W88, filed and
fixed by that stage.

Pinned by `ein-ir`'s `imports_semantics.rs`
(`what_each_import_form_brings_of_a_module`, a case per cell of the table;
`a_selection_names_and_the_module_comes_whole`;
`a_diamond_of_flat_imports_yields_each_fact_once`), `ein-cli`'s
`import_facts.rs` (W88's four witnesses, each with a claim, on twelve
commands: the import answers as the forms written in its place, and its saved
container as the file) and `imports_by_full_name.rs` (`nothing_is_left_behind`),
and `corpus/wrong_answers.toml`'s W88 rows.

### 4.26 A form of a declared relation has the declared count of arguments — M1f S1f.6.23

A stage of M1f
[P1f.6](../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/README.md),
[S1f.6.23](../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.23_the_loaders_arity_check.md),
on the user's ruling of 2026-11-12 on question 3 of the phase's
`questions.md`, *"(a)"*: *every form headed by a declared relation, at any
depth, in a fact, a rule or a query, refused at load with one line naming the
relation, the count declared and the count written* (Q-M1f.201 to .204,
defaulted).

**A `(relation R T₁ … Tₙ)` declares that a form of `R` has n arguments**, and
the loader refuses a program that writes one with another count
(`ein_ir::from_ir`'s `wrong_counts`), on every command, at each command's
load-error exit (1; `ein test`: 2), one line a form, placed at the form:

```text
$ ein solve t.ein       # (relation likes Person Drink :why "…") (likes Ann Tea) (likes Bob)
kb load error: t.ein:3:2: the fact (likes Bob) gives likes 1 argument, and (relation likes Person Drink) declares 2 — a form headed by a declared relation has as many arguments as its declaration has types
```

| where the form is written | the line, after its place |
|---|---|
| a fact | `the fact (likes Bob) gives likes 1 argument, and (relation likes Person Drink) declares 2 — …` |
| a fact inside a fact, at any depth, under `(not …)`, in a fact's `:using` | `the fact (add (n 1) (n z 5)) holds (n 1), which gives n 1 argument, and (relation n Number Digit) declares 2 — …` |
| a rule's or an hrule's `:match` or `:assert`, at any depth, under `(absent …)` and `(not …)`, as its macros expand | `rule 'fans': its :match writes (likes ?who), which gives likes 1 argument, and … declares 2 — …` |
| a query's `:goal`, as its macros expand | `query: its :goal writes (likes ?who), which gives likes 1 argument, and … declares 2 — …` |
| a fact a claim lists, a query's `:expect` or `(solve … :expect …)`'s | `solve: its :expect lists (likes Ann), which gives likes 1 argument, and … declares 2 — …` |

- **What declares a count** is the program's own `(relation …)` and an
  imported one: an import is resolved before the loader reads a form, so a
  flat import's relation is held under its name and `(import M :as o)`'s
  under `o.name`.
- **A bare `(relation flag)` declares none**, so `(flag)` is its one form and
  `(flag up)` is refused
  ([Q-M1f.201](../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f201--what-a-bare-relation-r-declares-of-its-count)).
- **A keyword pair is no argument**: `(likes Ann Tea :source "said")` has two.
- **A macro's body is held where a rule or a goal calls it**, on its
  expansion; a macro nothing calls is not read, as § 4.16 has it.
- **A relation nothing declares has no count** and is not held: `(says Ann)`
  beside `(says Ann Bob)` loads. That includes the kernel's own, which no
  program declares: `(__closed__ r extra)` loads, and `(relation R ?A ?B)`,
  the declaration's stored fact, is a pattern at any count
  ([Q-M1f.203](../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f203--the-kernels-own-relations-and-an-activators-count)).
  `eq` is held to two arguments by § 4.14.
- **A relation written as a variable is not read**, `(?R ?a ?b)`, whatever
  the program shows its parameter holds: a rule over a parameter is generic
  over counts, matching is by count, and `(mirror likes)` on a rule that
  reads `(?R ?a)` matches nothing, as it did
  ([Q-M1f.202](../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f202--a-relation-written-through-a-parameter-and-the-declared-count)).
  So a rule can still *derive* a fact of a declared relation at another
  count through a parameter; no written form is one.
- **An activator is not a declared relation's form**: `(functional r extra)`
  against a rule `functional (?R)` activates nothing, in silence, as before
  (E-34 of the phase's `findings.md`); an activator in a `(solve :hrules …)`
  list is held to its hrule's parameters by § 4.20.
- **It is no type check**: `(likes Tea Ann)` loads. What is written in an
  argument's place is not held to the type declared for it.
- **No flag turns it off.**

**Until this stage** no form was held to its declaration. A fact of another
count was stored beside the rest; matching is by count, so a premise or a
goal of another count matched nothing and a conclusion of another count
derived facts no premise of the declared count read; and the calculator's
`(add (n z 5))`, one operand where `add` declares two, and a `std.math.num9`
number of eight digits were answered by silence, exit 0. A `--dump-answer`
state file printed such a fact's English with a place nothing filled, *Bob
likes {?2}*. No program of the corpus that loaded was refused by the check
(`python3 build/s1f.6.23/sweep.py`, 2026-11-12). Seven register rows, W89,
filed and fixed by the stage.

Pinned by `ein-cli`'s `declared_counts.rs` (a case per row of the table and
per bullet, on twenty commands, the line's text included; W89's seven
witnesses), `ein-ir`'s `fuzz_properties.rs`
(`no_accepted_program_holds_a_declared_relation_at_another_count`, read off
the loaded store and the compiled plans), and `corpus/wrong_answers.toml`'s
W89 rows.

### 4.27 A program's hypothesis mechanisms do not contradict one another — M1f S1f.6.34

Five declarations steer what a commitment may be about: an `(hrule …)` (**H**, with its `:hrules` activators), an
obligation rule, `:assert (open ?R)` (**O**, with its activators), `(solve :hypothesis-relations …)` (**HR**),
`(solve :no-hypothesis …)` (**NH**) and `(__closed__ R)`, written or derived (**C**); the blind enumerator runs where
none of the generating ones does ([`06_reserved_names.md` § Hypothesis / query
control](ir/03-ein-lang/06_reserved_names.md#hypothesis--query-control)). The user ruled on 2026-11-13, on E-47,
*"report conflict if any hypothesis mechanism constradicts itself"*, and gave the pairs the same day. Since M1f
[S1f.6.34](../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.34_hypothesis_mechanisms_in_conflict.md)
a program whose two declarations say opposite things of one relation is refused, one line per conflict, and a
declaration that filters no candidate, or says what another already says, is one `note:` line on stderr:

| pair | refused where | `note:` where |
|---|---|---|
| H + O | never: H generates, O judges (`zebra2.ein`) | — |
| H + HR | an active hrule proposes a relation the list leaves out | the list, which filters no candidate beside an hrule |
| H + NH | an active hrule proposes a relation the list names | the list, the same |
| H + C | an active hrule proposes a closed relation | — |
| O + HR | no hrule is declared, and an obligation owes a relation the list leaves out | — |
| O + NH | no hrule is declared, and an obligation owes a relation the list names | — |
| O + C | an obligation owes a closed relation | — |
| HR + NH | a relation is in both | no hrule: a `:no-hypothesis` name `:hypothesis-relations` already leaves out |
| HR + C | a listed relation is closed | — |
| NH + C | never | no hrule: a `:no-hypothesis` name a closure already closes |

**Read on root's fixpoint.** A derived closure (`std.closure`'s `infer-closure`, a module's closing rule) and a
derived activator (`bijective-setup`'s `(total-owed R isa)`) exist only there. An hrule is **active** where the
generator compiles a plan of it, one without parameters always and one with them once per `:hrules` activator, and
the relation it proposes is its `:assert`'s; an obligation is a plan root's activators compile, **owed at root or
not** (Q-M1f.243), and the relation it owes is its `(open ?R)`'s. `:hypothesis-relations ()` names none, so beside an
active hrule or an obligation it leaves every relation out. An activator only a commitment derives is not seen.

**On every command, at load** (Q-M1f.245): `ein-cli`'s one load path loads the program a second time into tables of
its own, saturates that root and judges it (`ein_infer::mechanisms::check`), wherever the program could declare two
mechanisms at all (two of H, O, HR and NH as written, or one beside `__closed__` written anywhere); the command's own
tables are untouched, and a root that does not saturate is the command's to report. The views and `kb save` refuse
as `solve` does, and no container is written. A library caller calls `check`; `ein_infer`'s routes do not.

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

The exit is each command's load-error exit, 1 (`ein test`: 2, the file an `ERROR` row). The position is the list's
name, the list where it names none (`()` at its keyword), the hrule's `:assert`, or the written activator or closure
by its form; a line about two derived declarations has none. An activator or a closure says where it comes from,
*written at FILE:L:C* or *which rule 'r' derives*. The notes, each `note: FILE:L:C: solve: its :KEY …`, are *filters
no candidate — the program declares hrule 'guess', and no list filters an hrule's candidates*, *names near, which its
:hypothesis-relations already leaves out* and *names at, which (__closed__ at), written at FILE:L:C, already
closes*, printed once a process and moving no exit, answer or stdout (Q-M1f.242).

**Until S1f.6.34** each pair answered in silence: an hrule's candidates untouched by the lists and a closure (H + HR,
H + NH, H + C), and a state owing a relation the lists or a closure took out of the obligations rung *stuck*, R
empty (O + HR, O + NH, O + C). E-47 met H + NH on S1f.6.28's word problems, where *forbid*'s first remedy (§ 4.7)
named the list the program already wrote. The corpus carried twenty-four programs in conflict, every one a
deliberate fixture that switched an obligation's generating role off to ask about root alone: fourteen answer as
before, nine state the R their edit gives, and `tests/stdlib/closure/03_closed_and_owing.ein` is
`examples/ein-bugs/closed-and-owing.ein`, the refusal's fixture (Q-M1f.246). No program that loads with no conflict
moved.

**`(solve :no-search)`** (M1f [S1f.6.36](../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.36_no_search.md),
the user's words of 2026-11-13, *"maybe some explicit flag in (solve ) ? instead of abusing bugs"*) states that the
lattice is root alone: no rung runs, so every generating mechanism proposes nothing, and an obligation judges root
alone. It is the sanctioned way to say what the twenty-four programs said with a list; twenty-one of them say it with
the keyword since (Q-M1f.260). Beside an obligation or a closure it is silent; beside an hrule, `:hypothesis-relations`
or `:no-hypothesis` it is one `note:` each, *FILE:L:C: solve: its :no-search states the lattice is root alone, so hrule
'guess' proposes no candidate* and *FILE:L:C: solve: its :no-hypothesis filters no candidate — the program states
:no-search, whose lattice is root alone*, which replace the notes above. **It relaxes no conflict**: H, O and the lists
are read as declared, so every refused pair is refused beside it with the same line, E-47's witness among them with
`:no-search` in its list's place (H + C) ([Q-M1f.254](../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f254--s1f634s-conflicts-beside-solve-no-search)).

Pinned by `ein-cli`'s `mechanism_conflicts.rs` (eleven refused shapes, each on twenty commands with one line, the
same on each, and no container; twelve that load, six silent and six with their note, under five commands and
reopened from a container; E-47's witness and its twin, two lines each and none of *forbid*'s; the corpus's
fixture; since S1f.6.36 each refused shape with `:no-search` beside it, the same line, and `:no-search` beside each
mechanism, silent or its note), and by `ein-infer`'s `obligation_reports.rs`.

### 4.28 An import is by full name, and a short name names one declaration in a file — M1f S1f.6.35

A stage of M1f
[P1f.6](../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/README.md),
[S1f.6.35](../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.35_imports_by_full_name.md),
on the user's model (h) of 2026-11-13 and its ruling of 2026-11-14
([`import_clashes.md` § 7](../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/import_clashes.md#7-the-ruling-2026-11-13-h-full-names-and-short-names)).
Every import brings its module whole (§ 4.25); a name a module declares has
one full name, `M.name`, the module's name a path from the program file's
directory or `std.x`; a file writes short the names it declares, lists or
prefixes with an alias, and any other short name it writes is its own
([`01_grammar.md` § Import](ir/03-ein-lang/01_grammar.md#import)). Each of
the following is refused at load, on every command, one `kb load error:`
line at the command's load-error exit (1; `ein test` 2), placed where the
form or the name is written:

| the program | the line |
|---|---|
| one short name bound to two full names in one file, by two imports or by an import and the file's own declaration (Q-M1f.234) | `p.ein:2:9: 'done' is bound to two full names in the program, m.done and n.done — one short name names one declaration in a file: list it from one module, or write the other by its full name`; `… infer-closure (its own declaration) and std.closure.infer-closure …` |
| one alias prefixing two modules in one file | `p.ein:2:9: 'x' prefixes two modules in one file, m and n — one short name names one thing in a file` |
| a prefixed or full name that names nothing its module declares (Q-M1f.251) | `p.ein:2:2: 'x.nope' names nothing: m declares no 'nope' — a prefixed name is a full name of a declaration of the module it names (Q-M1f.251)` |
| a full name of a module no import of the program brings (Q-M1f.238), with an import of another module or with none | `p.ein:1:2: 'std.math.decimal-arith.add' is a full name of std.math.decimal-arith, which no import of the program brings — a module enters a program by an import of it: write (import std.math.decimal-arith …) (Q-M1f.238)` |
| the main file declaring a name under the full name of a module the program imports (Q-M1f.239) | `p.ein:2:11: the program declares 'm.mine', a full name of m, which the program imports — a declaration's full name is its own file's, and a full name of m is one m declares (Q-M1f.239)` |
| a module declaring a name with a dot (Q-M1f.241) | `d.ein:2:11: d declares 'y.z', a name with a dot — a module's declared name holds no dot, so that a full name such as d.x reads one way (Q-M1f.241)` |
| a module's name with an empty segment or a `/`, `..` among them (Q-M1f.252) | `(import ..m) — a module's name is its path from the program's directory, a name per directory joined by dots, none empty and none holding a '/': no '..', no path (Q-M1f.252) at None` |
| a module's own import of a file-relative module written from the module's directory | `(import words) — module not found at <program's directory>/words.ein (None)`: every module is found from the program file's directory |

What is **not** refused: one short name bound to one full name twice; a
short name the file writes that no import of it lists, which is the file's
own whatever a module declares under it (the user: *"silent re-definition,
like variables shadowing, it's a mistake"*, Q-M1f.236, E-49); a dotted name
the main file declares where no module of the program has its prefix, which
is how a `--dump-answer` state file declares the full names it was loaded
with; and a dotted name whose prefix is no module at all, a name like another.

**What prints** is the main file's listed short name, else its first alias's
spelling, else the full name (Q-M1f.237, Q-M1f.253), every surface alike:
a state, a trace, a core, `refuted by`, `--events`. A spelling another file
writes as a name of its own is not taken.

**Until S1f.6.35** a flat import shared names with the program and with every
other flat import: a program's `done`, `b` or rule `add-column` beside the
calculator was refused, *conflicting definitions of 'done': same name,
different body (at None)* (E-21), where two bodies differed, and two equal
ones were one; a program's undeclared `(done laundry today)` joined the
calculator's `done`; an empty segment of a module's name was dropped, so
`(import ..succ)` read `succ.ein`; and a module's own imports were found from
its own directory.

Pinned by `ein-ir`'s `imports_full_names.rs` (`the_refusals`, one case each;
`the_users_dependency_graph`; `a_short_name_is_local_to_its_file`;
`a_module_is_found_from_the_programs_directory`; `what_prints`),
`imports_semantics.rs`'s `module_names_mangle_into_these_file_paths` and the
`import_conflicting_definitions` fixture of `examples/broken/load/`, and
`ein-cli`'s `imports_by_full_name.rs` (`a_refusal_is_one_line`) and
`import_invariance.rs`, every corpus program alike under its selective, whole
and `:as` imports.

## 5. The CLI surface

Everything a script or a habit can depend on is fixed: the five subcommands
(`oracle` since M1e2 S1e2.1a.9) and the `render` sub-subcommands, the
delegated dispatch (`ein saturate --help` prints `saturate`'s own help;
`saturate` appears in `ein --help` though the top parser never parses it),
every option's long name, short key,
metavar, arity, default, choices and mutually-exclusive group, its help
*string*, the accept/reject verdict on every invocation, the exit code, and
which stream each byte goes to.

**`ein test` is the fourth**, added by M1c
[S1c.1.3](../history/m1c_external_validation/README.md#s1c13--ein-test),
and its exit codes **deliberately differ** from the other three's:

| code | `solve` / `oracle` / `render` / `saturate` | `test` |
|---|---|---|
| 0 | success | every expectation held, and at least one was checked |
| 1 | a load error — or *forbid*'s refusal (§ 4.7), a false `:expect`, or an `oracle` cube that contradicts a monotone static verdict | an expectation is **false**, or was not checked (under `--no-search`, false only) |
| 2 | a usage error, or `-T` | a load error or *forbid*'s refusal, a usage error, `-T` — or nothing to check |

**An answer is never an exit code** on the other three: since M1e2
S1e2.1a.5 a `solve --single` exits 0 whatever its word, and a run its budget
stopped writes one `note:` to stderr and exits 0 (the user's ruling of
2026-10-16, [`read_out.md` § 9](../../plans/m1e2_correctness/p1e2.1a_the_solution_region/read_out.md#9-exit-codes)).

**`--no-search` declines to discover R, and is no code either** (M1f
[S1f.6.36](../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.36_no_search.md)).
`ein solve --no-search` reports root and exits 0, or 1 where root decides a
claim false; `ein test --no-search` decides what root decides, and a claim it
leaves NOT CHECKED, having been asked not to search, moves no exit code
([Q-M1f.257](../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f257--ein-test---no-searchs-exit-code)).
`ein oracle --no-search` and `ein solve --no-search` beside `-n`, `--single`,
`--minimal`, `--maximal`, `--count` or `--monotone` are usage errors, exit 2
([Q-M1f.258](../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f258--which-commands-and-options-take---no-search)).

**A closed stdout is no code either.** Since M1f S1f.5.25 a write to a pipe
whose reader has gone ends any of the five subcommands by `SIGPIPE`, with
nothing on stderr, which a shell reports as 141 (§ 4.19); until then it
panicked, exit 101, where the write came after the reader's exit.

**Most of the rest of § 5 is the walk's surface as M1c–M1e defined it** — the
verdict words' qualifiers, `--max-set-size 0`, `--models` on the `Ambiguity`
arm — and no command reached the walk after the switch, and M1e2 S1e2.1c.5
deleted it (2026-11-03), so those paragraphs are its record. What carried over,
and where it is held: `-m` and `-E` are retired and refused with what replaced
them; `-n 0` is still refused, exit 2, with the message below; the qualifier
rule is the region read-out's, where every count that is not exact says *a
lower bound* and how far it is complete, and a run that stopped never says R
is empty (`cli_semantics::a_stopped_run_does_not_report_r_empty`);
`--models list|key|grid` draws the states a run lists, and `key` is inert
below two (`model_set_report.rs`). Since M1e2 S1e2.3.3 (the user's ruling (a)
of 2026-11-03) the forms after the read-out (`-H`, `--models key|grid`, `-p`,
`-P`, `-f`, `--diff-models`) draw the states stdout showed and share its 100
lines, at least one item each, and `--dump-answer DIR` writes each uncut, over
every listed state, as `DIR/forms/<form>.txt` (`forms_cut.rs`). Past a 16 MiB pair table `key` declines
before it allocates the table, with the numbers and exit 0, and so does
`--diff-models`' near pairing (M1e2 S1e2.4.2, `ein-render/tests/key_declines.rs`;
the bound is [`configuration.md` § 3](configuration.md)'s).

1 is taken there, and a runner that cannot tell a broken file from a false
claim is the one failure mode a test runner must not have. 2 dominates 1 in a
multi-file run for the same reason: with one file unloaded, "every expectation
was checked and some are false" would be a false description of the run.

**M1d [S1d.2.6](../../docs/history/m1d_satisfiability/README.md#s1d26--verdicts-counters-corpus)
added a fourth verdict word and no exit code.** `Open` — a consistent,
quiescent state with an obligation the program stated still unwitnessed —
exits **0**, exactly as `Solution`, `Ambiguity` and `Contradiction` do: the
verdict channel and the *claim* channel are separate, and the claim channel is
`:expect`. A program whose expectation holds against an open state exits 0 and
one whose expectation is false exits 1, both for the reasons they already did.
`:expect` did not grow a word for the verdict, so no claim can assert openness;
all three of its forms are assertions about **facts**, and an `open` conclusion
is by construction never a fact. Twelve corpus entries changed word and **zero**
changed exit code.

**M1d [S1d.3.3](../../docs/history/m1d_satisfiability/README.md#s1d33--the-verdict)
added the 49th option and one rendering rule.** `solve --models {list,key}`
chooses the projection of a model **set** that goes to stdout — the blocks, or
the smallest set of slots that tells the models apart and the table of
combinations that occur. It is read by the `Ambiguity` arm alone, defaults to
`list`, and reaches nothing recorded: the verdict, every counter, the JSON
summary, the event stream, `-p` and `:expect` are byte-identical under either
value.

The rule beside it is **normative and about what a count may claim**:

| the search | what a report of the model set may say |
|---|---|
| `exhausted = true` | *these are the models* — `solutions (k) 9`, `Ambiguous — distinct complete models` |
| `exhausted = false` | *these are models **found*** — `solutions (k) 5   (a lower bound — the search did not exhaust)`, `Ambiguous — distinct complete models found` |

`Solution` has always carried the same distinction, as *"(not certified —
pass --exhaustive)"*. `Ambiguity` did not, and the difference matters more
there: an unqualified `k = 1` is a guess about uniqueness, where an unqualified
`k = 5` on a file with nine models is **wrong**.

**M1d [T1d.10.5.2b](../../docs/history/m1d_satisfiability/README.md#s1d105--what-exhausted-means)
extended the rule to the empty model set, which is where it was worth most.**
A `k = 0` is a claim like any other and `exhausted` is what licenses it, so the
table above now has four rows rather than two:

| verdict | `exhausted = true` | `exhausted = false` |
|---|---|---|
| `Solution` | `Solution` | `(not certified — pass --exhaustive)` |
| `Ambiguity` | *these are the models* | *models **found***, `(a lower bound …)` |
| `Contradiction` | *No solution — the constraints are contradictory* | *No model found — the search did not exhaust the lattice*, `(none found …)` |
| `Open` | *the requirement is unmet, not refuted* | *…and the search did not exhaust* |

The fixture is `examples/saturation/type-exclusivity/pets.ein`, and it is why
this is a defect and not a nicety: it said **the constraints are
contradictory** at `-m 5` and `-m 8`, and it has **35 models** at `-m 10`. The
claim channel had refused to be fooled by the same run since M1c —
`:expect (false)` there comes back `NOT CHECKED`, not `FAILED`, because
`expect.rs` will not settle a claim on a stopped search — so the two channels
were saying *refuted* and *nobody knows* about one solve. They agree now.

**The unsat core is renamed with it**, for the same reason and in the same
breath: an unsat core explains why a program has **no model**, which a
truncated run has not shown. Its block header is `refuted so far (n facts)`
when `exhausted = false` — which is also what makes the empty one legible,
where `unsat core (0 facts)` read as *the empty set is contradictory* rather
than *nothing died*.

**Words moved; nothing else did.** 26 corpus cells across 13 files change what
they print, all of them `Contradiction` at `exhausted = false`; the 48
exhausted `Contradiction` cells keep every word; **no exit code, no counter, no
`--json-summary` field and no `:expect` outcome moves**, and `verdict.type`
stays `Contradiction`, which is the same shape S1d.3.3 held to. `Open` at
`exhausted = false` is unreachable on today's corpus — every `Open` in it is
exhausted — so that row is written to be right rather than because a cell moved.

What remains of [Q-M1d.1](../../docs/history/m1d_satisfiability/open_questions.md#q-m1d1--may-the-search-stop-before-the-lattice-is-exhausted)
after this is the *stopping* half, not the vocabulary half: whether a search
may stop early on purpose, and what would license it. The word it used to be
about is settled: a refutation said under a depth cap is not a refutation, and
it no longer says it is.

**M1d [S1d.4.1](../../docs/history/m1d_satisfiability/README.md#s1d41--what-closure-costs)
added the 50th option and no behaviour at all.** `ein test --json-report
FILE.json` writes **one row per `(query …)` of the whole selection** — the
claim's shape (`model` / `or` / `false`), how many models it lists, the
relations its `:goal` closes, the outcome, and what the run found — since M1e2
S1e2.1a.1 including whether the program is monotone, by the static check,
and how many sites make it not. It is
additive in the strict sense the other artefact flags are — stdout, stderr, the
exit code and *what is solved* are identical with it and without it, so a query
stating no `:expect` is still never solved and its row says so.

Two things about it are contract rather than convenience:

- **It takes any selection**, where `--events` and `--json-summary` are refused
  over more than one run. Those name one *path* for one *run*; a report has no
  run to be more than one of, and one invocation over the three corpus roots is
  what the census reads.
- **A file that did not load carries no claim.** Its row is `queries = 0`,
  `outcome = "error"`, `expect = null`. Three `examples/broken/load/` fixtures
  contain the token `:expect` and exist to be refused; a claim is a property of
  a *program*, so they are not in the numerator of "what fraction of the corpus
  claims a model set".

The schema is `ein-test-report/5`, deliberately not `ein-summary/2`: a summary
is one run's counters, a report is one row per query, and a consumer reading
the same version marker on both would be right to expect the same fields.
**`/5` since M1f S1f.6.27**, additively: every row carries `route`, the
question its claim's outcome was read off, spelled as `ein solve`'s flags
(`--single`, `-n 4`, `--maximal -n 3`) and `describe` for the bare run, and
`fell_back_from`, the question asked first where it left the claim undecided
and the outcome is the describing run's; both are `null` on a row no run
checked. `ein test -v` names the same two in a query claim's line, *holds
(describe, after -n 3: all of R, 2 classes over 7 states)*, and
`test_cli::the_verbose_header_and_the_report_row_agree_field_for_field` holds
the line to the row over the corpus. Where a file was asked two questions,
`--events` and `--json-summary`, which name one run, hold the last. `/3`
(M1e2 S1e2.1a.5) replaced a query row's `ran`, the walk's counters, with
`answer`: what its classes are over, whether that is exact, and how many
classes and states; `/4` (M1e2 S1e2.3.10) added `outside` and
`types_as_objects` to every row, what the universe leaves out beside a claim
a run checked.
**`/2` since M1e2 S1e2.1a.4.** A file's `(solve … :expect …)` claim has a row
of its own, first among the file's rows, as its header is first among the
file's lines. Every row says which kind it is in `place` (`query` or
`solve`). A solve row has `query = 0`, an `expect` with the claim's `shape`
(`region` or `false`), `models`, `merged` and `owing`, a `ran` of `null`
(that field is the walk's), and a `region` with the `route`, `monotone`,
`k_s`, `k_c` and `ms` of the R it was checked against.

**M1d [S1d.4.3](../../docs/history/m1d_satisfiability/the_vocabulary.md)
moved no byte of stdout and added one line to stderr.** A `:expect` that does
not hold under `ein solve` now writes

```text
<file>: :expect NOT CHECKED — expected Ambiguity with k = 2, got Solution with k = 1
```

to **stderr**, beside the unchanged report under the solution table. The rule
it settles is which stream carries what, and both halves are the answer:

| | stream | why |
|---|---|---|
| the `:expect` block — label, disagreements, the derivation that put a surplus fact there, the models projected through the `:goal` | **stdout** | it is what the run *found*. A false claim is a **result**, not a refusal of the input, and its report belongs under the table it is about |
| one line naming the file, the label and the first disagreement | **stderr** | an exit 1 with an empty stderr is a run nobody can diagnose from a pipeline |

Before it, `solve` produced exactly that undiagnosable shape, which is why
`examples/features/11_expect_ambiguity.ein` — then the corpus's only fixture
that reached `Outcome::NotChecked` under a declared run, joined at M1e S1e.3.1
by `examples/ein-bugs/complete-records-stale.ein`, whose claim is a `Solution`
and so is checkable only when the search exhausts — **could not declare plain
`solve`**: `corpus_cli::every_refusal_carries_a_diagnostic` forbids it. Both
declare it now, and `corpus_exits.txt` banks the 1. `ein test` is unchanged:
it prints its own per-file report and has always been readable on stdout alone.

**M1d [T1d.10.5.0](../../docs/history/m1d_satisfiability/README.md#s1d105--what-exhausted-means)
closed the one door in that rule with no guard on it.** `--max-set-size 0` is a
**truncation**: a run that explores no layer over a non-empty frontier reports
`exhausted = false`, and the table above then applies to it unchanged — its
`k = 0` means *no model within the cap*, never *no model*. Before it the layer
loop `1..=max_set_size` never ran at zero, so `truncated` was never set and
`exhausted` kept its `true` default; **51 of the 150 corpus entries that load —
every one that reaches the search** — stated a refutation with an empty unsat
core, a certified exhaustion claim and a success exit code.

Two things the fix decides rather than assumes:

- **A cap of zero answers; it does not refuse.** A program whose root is
  already complete has no lattice to exhaust and reports its verdict exactly at
  a cap of zero — `Solution` on `examples/branching/01_saturate_only.ein`,
  `Open — owes 1` on `tests/stdlib/algebra/23_total_owed.ein` (on
  `tests/region/04_owing.ein` since M1f S1f.6.34, which let `23`'s obligation
  propose), both `exhausted = true`. The other **99** entries are that class and not one of
  them moved. The alternative — the `Aborted` shape `--max-enterings 0` uses —
  would decline a question the engine answers exactly, and P1d.10's own
  reconnaissance asks it once per node.
- **An empty `unsat_core` on a `Contradiction` stays constructible.** A search
  that entered nothing has no dead commitment to cite, and the emptiness is not
  this cap's shape to begin with: **12** corpus entries report `Contradiction`
  with a 0-fact core under their ordinary `solve` run, every one of them at
  `exhausted = false`. Whether that pair may keep the *word* is
  [Q-M1d.1](../../docs/history/m1d_satisfiability/open_questions.md#q-m1d1--may-the-search-stop-before-the-lattice-is-exhausted)'s
  and is untouched here — no exit code moved, and `corpus_exits.txt` is
  unchanged.

**M1e [S1e.1.5](../history/m1e_review_processing/README.md#s1e15---n-0-q7)
answered the same question for the other budget flag, and answered it the other
way.** `--solutions 0` is **refused**, exit 2:

```text
$ ein solve x.ein -n=0
error: invalid value '0' for '--solutions <N>': invalid solution count: '0' (expected 1 or more; leave -n out to describe all of R)
```

The asymmetry with `--max-set-size 0` above is the point, and it is not a
preference. A cap of zero *has* an answer the engine gives exactly — explore no
layer, and say so with `exhausted = false`. A stop-after of zero has none:
`stop_after` is compared with `>=` **after** a model is recorded, so `Some(0)`
cuts at the **first** model and `-n 0` was `-n 1` under another name, measured
byte-identical on stdout and on every `--json-summary` field across a unique
model, a nine-model ambiguity and a contradiction. Neither reading a user could
mean was available — *record nothing* is not what it did, and *no limit* was
already spelled `--exhaustive`, and is leaving `-n` out since M1e2 S1e2.1a.5 —
which is the argument `--jobs 0` had been
refused with since [S1a.7.5](../history/m1a_rust/README.md#s1a75--the---jobs-contract).

Two things it settles that are easy to state loosely:

- **Every negative goes with it.** The CLI clamped `-n` to zero with `max(0)`,
  so `--solutions=-7` was `-n 1` too. `-n -7` with a space was already `clap`'s
  *unexpected argument*; the `=` spelling was the open door.
- **A non-integer keeps its message.** `error: invalid value 'x' for
  '--solutions <N>': invalid int value: 'x'` is §4's own example row, so the
  validator refuses the *range* and leaves the *type* alone.

Held by
`ein-cli/tests/cli_semantics.rs::solutions_takes_a_count_of_one_or_more_and_nothing_else`.

**`-g/--progress-every 0` logs no node line, and a negative is refused**
(M1e2 [S1e2.3.4](../../plans/m1e2_correctness/p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.4_the_inherited_questions_and_the_record.md),
the user's ruling (b) of 2026-11-03, which closed
[Q-M1e.17](../history/m1e_review_processing/open_questions.md#q-m1e17--three-int_arg-options-silently-reinterpret-a-negative)).
Under `-v` the step lines print and the node lines do not, which is what `ein
saturate --progress-every 0` already meant; until then the region modes
clamped 0 and every negative to 1 and logged every node with no word. A
negative is a usage error, exit 2, in both spellings (`-g=-1` and `-g -1`,
since the option takes negative numbers as values):

```text
$ ein solve x.ein -v -g=-1
error: invalid value '-1' for '--progress-every <PROGRESS_EVERY>': invalid progress interval: '-1' (expected 0 or more nodes; 0 logs none)
```

Held by
`cli_semantics.rs::progress_every_is_zero_for_no_node_lines_and_refuses_a_negative`.

**`ein saturate --max-steps N` stops at N firings, says where in Ein's
spelling, and exits 0** (M1e2
[S1e2.3.7](../../plans/m1e2_correctness/p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.7_four_lines_that_misname.md),
N3, the user's ruling of 2026-11-03). The limit is checked before each step.
So N firings run, and then the run stops without asking whether the KB is at
its fixpoint, even when the N-th firing was the last there was. One line on
stdout names the flag as typed, the firings it let run and the last of them:
its activation as a program writes it, its bindings as `?v = value`, and its
facts as s-expressions. A redundant last firing says it *re-derived* what the
KB already held. The state AFTER, the firing breakdowns, a per-rule breakdown
*at limit* and `--dump` follow, and they are the KB where the run stopped.
Nothing goes to stderr, and the exit code is 0, as for every answer
([`read_out.md` § 9](../../plans/m1e2_correctness/p1e2.1a_the_solution_region/read_out.md#9-exit-codes)).
0 and a negative stop before the first firing.

```text
$ ein saturate --max-steps 1 examples/saturation/transitive/colocation-chain.ein
…
!! saturator step limit hit: --max-steps 1 stopped saturation after 1 firing, before it checked for a fixpoint. The last firing: (transitive co-located) with ?rel = co-located · ?a = A · ?b = B · ?c = C derived (co-located A C) from (co-located A B) (co-located B C)
```

Until then the line spelt the firing as a Python repr, `Firing(rule='transitive',
activator=('co-located',), …)`. It sent the reader to `Saturator._last_firing`,
which no crate defines, and it said *without reaching fixed point* on this
run, whose one firing is the whole saturation. The help said the flag *raises
SaturatorStepLimitError*. Held by
`cli_semantics.rs::the_step_limit_is_one_line_in_eins_spelling_and_exits_0`.

**M1d P1d.10 added the 51st option, and it moves no byte of stdout.** `ein
solve --layer-progress` streams the **per-layer census** to stderr and nothing
per entering — three lines a layer, where `--verbose` had one and a half:

```text
  layer 3: alive=96 root_facts=339
  layer 3 gen:  frontier=2911 joined=60260 −dead=0 −clause=16171 (26.8%) cand=44089
  layer 3 test: entered=44089 alive=33940 complete=7256 models=4 dead=10149 dead_pre=0 …
  layer 3 done: survivors=26684 enterings=48745 solution-nodes=32
```

The numbers are the sixteen the `layer` event already carried; what is new is
that they reach a reader who is *watching a run* rather than parsing a JSONL
file afterwards. Three identities hold per layer and are a test
(`cli_semantics::the_layer_progress_rows_add_up`):

| identity | which half of the loop it accounts for |
|---|---|
| `joined − dropped_dead − dropped_clause = cand` | generation: what the prefix join proposed and what the learned clauses took off it |
| `entered = alive + dead` | testing |
| `alive = complete + survivors` | where the consistent forks went |

The third is the one no counter stated before. `alive_enterings` counts every
consistent fork, and only the **incomplete** ones reach the next frontier — so
`complete` is this layer's solution-outcome enterings and `models` is what they
collapse to under `state_key`. On `examples/zebra2.ein` layer 1 that is
**13 → 1**: thirteen commitments each complete the puzzle, and all thirteen are
the same model.

`--verbose` still prints the per-entering line and now prints these rows too;
`--layer-progress` is the same dumper with the entering firehose silenced,
which is what makes it usable on a run that enters 618 076 times (6 180 lines
at the default `--progress-every`). Both cost what `-v` costs — the dumper
reads each fork for the `state_key` dedup, so neither is free — and both are
additive: stdout, the exit code and every counter are identical with them and
without.

**Free, and `clap`'s:** wrapping, indentation, headings, ordering within a
section, and the wording of a usage diagnosis. The structure — `{subcommand →
{option → short, metavar, arity, default, choices, group, help}}` — is what the gate checks, which catches a lost option
on its own line rather than inside an 89-line text blob.

## 6. What is *not* defined, and is filed

**Nothing, since M1e2 S1e2.2.12** (2026-11-04). M1e2's switch left one that
nobody had seen, the second item below, and the first was genuinely
**under-determined** while the walk printed it. The region read-out sorts a
class's goal rows and prints every one (M1e2 T1e2.1a.5.2), so no row is
chosen, and its witness is the corpus entry
[`examples/ein-bugs/d3-goal-row-order.ein`](../../examples/ein-bugs/d3-goal-row-order.ein)
since M1e2 S1e2.3.4, which `ein-render/tests/id_order_invariance.rs` permutes
on every commit. The two items are kept as the record of what was filed:

- **The goal row a solve table prints.** If a model satisfies `(query :goal
  …)` more than once, the table prints the first row of an *unsorted* match,
  and which row that is depends on the order facts entered the KB. It moves
  under a permuted id space, in one build — so it is a
  [design/02](../history/m1a_rust/design/02_determinism_and_order.md)
  violation in its own right and not a consequence of anything else.
  `summary.json`'s `goal_bindings` carries **all** rows and sorts them, which
  is why no counter and no JSON field can see this. The fix is available and
  is a decision, not a repair — print the lex-smallest row, at the cost of one
  visible change to a checked-in fixture. Reproducer and notes:
  [`corpus/fuzz_findings/`](../../corpus/fuzz_findings/README.md).
- **A dead node's core and the rules it names.** A resumed fork's delta was
  sorted by `FactId`, which is interning order, so a dead node's derivations
  arrived in an order the program does not decide: the rules `refuted_by`
  names and the `absent`s a refutation rests on moved under a permuted id
  space on 34 corpus files (`EIN_ID_SEEDS=8`, 2026-11-04, with every core
  compared), and the core, where a full alternatives list
  keeps whichever arrive first, on
  [`examples/ein-bugs/fork-delta-order.ein`](../../examples/ein-bugs/fork-delta-order.ein)
  (M1e2 `W11/fork-delta-order`). Seen in `--dump-answer`'s `nogoods.json`,
  `--dump-lattice`'s `nodes.json`, `ein oracle --dump` and `--events`, which
  no sweep compared until M1e2 S1e2.2.12; fixed by it (`10858b96`): the delta is
  in the order its commitment was written, and the sweep compares every core
  and asserts that nothing moves.

## See also

- [`../../docs/history/m1a_rust/open_questions.md`](../history/m1a_rust/open_questions.md)
  — Q-M1a.3 / .4 / .5 / .6 / .8 / .13 / .14 / .15, the resolutions this page
  restates.
- [`inference/reserved_engine_strings.md`](inference/reserved_engine_strings.md)
  — the other kind of thing that is defined only because the engine does it.
- [`inference/implementation.md`](inference/implementation.md) — where each
  behaviour above is implemented.
- [`standard_of_proof.md`](standard_of_proof.md) — its opposite number: this
  page states what the engine **does**, that one states what it takes to know
  it, and what an argument for leaving a behaviour alone has to rest on.
