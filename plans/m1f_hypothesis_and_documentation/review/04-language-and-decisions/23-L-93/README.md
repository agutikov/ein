# L-93 — two stale sentences of the pages

## 1. The question

Two sentences of the documentation say what no longer holds, and were read, not run, when they were found:

1. [`04-stdlib/08-natural-numbers.md` § How long a number may be](../../../../../docs/ein/user/04-reference/04-stdlib/08-natural-numbers.md#how-long-a-number-may-be)
   cites the diagnostics page for the parser's limit of 1 000 parentheses, and the diagnostics page does not state it.
2. [`ein.rs/07-reserved-engine-strings.md` § Names the parser and the loader make up](../../../../../docs/ein/ein.rs/07-reserved-engine-strings.md#names-the-parser-and-the-loader-make-up)
   gives the internal name `#nested` the example `(r (_ a))`, and `_` is refused at the parse.

The question is whether either is worth a stage; each is a sentence. Recorded at the milestone's
[`findings.md`](../../../findings.md), row **L-93** (from P1f.22's N-13, whose third sentence is gone).

## 2. What is so today

All runs use `build/review-bin/ein-head-b5fb8d70`; the greps and the sections they find are
[`runs/greps.txt`](runs/greps.txt).

### 2.1 The parser's limit

- The ledger's grep, `grep -n -i 'nested past\|1 000 parenth' docs/ein/user/04-reference/07-diagnostics.md`, prints
  nothing, and a wider one (`nests deeper\|parenthes\|nesting`) nothing either: **the diagnostics page states no
  nesting limit**. § 1 there says every parse error is `<file>:<line>:<col>: unexpected X`, which the limit's line is
  not.
- The limit is stated by [Q-M1f.185](../../../../../docs/decisions/engine/Q-M1f.185-a-state-deeper-than-the-parsers-limit.md),
  by [`ein.rs/03-design/04-ir-frontend.md` § 3.4](../../../../../docs/ein/ein.rs/03-design/04-ir-frontend.md#34-the-nesting-limit)
  and by `stdlib/README.md` § How long a number may be, which quotes the line; it is
  `ein_ir::grammar::NESTING_LIMIT` (1 000) and the line is pinned by `ein-ir/tests/generated_parser.rs`'
  `the_nesting_limit` (read, not run).
- **The witnesses**: [`programs/nested-1000.ein`](programs/nested-1000.ein), one fact `(p (p … a))` 1 000 deep, loads
  ([`runs/nested-1000.solve.txt`](runs/nested-1000.solve.txt)); [`programs/nested-1001.ein`](programs/nested-1001.ein),
  1 001 deep, is refused, exit 1 ([`runs/nested-1001.solve.txt`](runs/nested-1001.solve.txt)):

  <!-- output -->
  ```text
  plans/m1f_hypothesis_and_documentation/review/04-language-and-decisions/23-L-93/programs/nested-1001.ein:4:3001: this `(` nests deeper than the limit of 1000 parentheses
  ```
- The page's number holds: a `let` of a numeral of 999 digits loads
  ([`programs/numeral-999-digits.ein`](programs/numeral-999-digits.ein), [run](runs/numeral-999-digits.solve.txt)),
  of 1 000 digits is refused with the same line ([`programs/numeral-1000-digits.ein`](programs/numeral-1000-digits.ein),
  [run](runs/numeral-1000-digits.solve.txt), `…:8:3007: this ( nests deeper than the limit of 1000 parentheses`).

### 2.2 `#nested`

The page's row: `| #nested | a nested form whose head is not a name, (r (_ a)) | (_ …) |`. Every way to write a
nested form whose head is not a name is refused before the loader:

| program | line | exit |
|---|---|---|
| [`head-underscore.ein`](programs/head-underscore.ein), `(r (_ a))`, the page's example | `…:3:5: unexpected _: _ (reserved)` + `note: _ is reserved and names nothing: where any value will do, write a variable used once` | 1 ([run](runs/head-underscore.solve.txt)) |
| [`head-variable.ein`](programs/head-variable.ein), `(r (?x a))` | `…:3:5: unexpected ?x: a variable` | 1 ([run](runs/head-variable.solve.txt)) |
| [`head-form.ein`](programs/head-form.ein), `(r ((f b) a))` | `…:3:5: unexpected (`, `repair: Insert a name` | 1 ([run](runs/head-form.solve.txt)) |
| [`head-string.ein`](programs/head-string.ein), `(r ("x" a))` | `…:3:5: unexpected "x": a string` + its note | 1 ([run](runs/head-string.solve.txt)) |
| [`head-variable-in-rule.ein`](programs/head-variable-in-rule.ein), `(r (?x a))` in a `:match`, `?x` no parameter | `kb load error: rule 't': its :match writes (?x a) with ?x as its relation, which no parameter binds — …` | 1 ([run](runs/head-variable-in-rule.solve.txt)) |

The loader's own unit test says so: *"no fact reaches the loader with a nested head that is not a name, and the
`#nested` rename such a head collapses to is reached by no program"* (`from_ir.rs:6381`, in
[`runs/greps.txt`](runs/greps.txt)). The name is still `ein_core::text::NESTED_HEAD`, one of `INTERNAL_NAMES`.

## 3. The options, with the proposed re-wordings

### 3.1 The stdlib page

**(a) Point at the decision**, as `stdlib/README.md` does. Proposed, for the paragraph's first sentence:

```markdown
The theory has no width. A program can write a number of at most 999 digits, since a numeral nests a level a digit
and the parser refuses a program nested past 1 000 parentheses, with the line *this `(` nests deeper than the limit
of 1000 parentheses*
([Q-M1f.185](../../../../decisions/engine/Q-M1f.185-a-state-deeper-than-the-parsers-limit.md)); a rule may derive a
longer one.
```

**(b) Put the limit on the diagnostics page** and keep the stdlib page's link, aimed at it. A section at the end of
§ 1, which renumbers nothing:

````markdown
### 1.7 A text nested past 1 000 parentheses is refused before it is parsed

A top-level form is at depth 1. One pass over the tokens counts depth before the parse, and a text whose `(` passes
1 000 is refused with one line at that `(`, and no other:

```text
<file>:<line>:<col>: this `(` nests deeper than the limit of 1000 parentheses
```

Every command exits 1 (`ein test`: 2). A rule may derive a fact deeper than the limit, and a state that holds one is
answered and saved; its saved file loads again, since what the parser reads of it is the program
([Q-M1f.185](../../../decisions/engine/Q-M1f.185-a-state-deeper-than-the-parsers-limit.md)).
````

and in the stdlib page the link becomes:

```markdown
([diagnostics](../07-diagnostics.md#17-a-text-nested-past-1-000-parentheses-is-refused-before-it-is-parsed))
```

The line is already pinned by `generated_parser.rs`' `the_nesting_limit`, which (b) would cite.

**Either way, the paragraph's third sentence** (found while building, § 7) needs re-wording; proposed:

```markdown
A step of a circuit holds its operands inside its tag, two levels deeper than the request for a sum, a difference
and a comparison and three for a product and a quotient, so a state of operands near that length holds facts past
the limit. It is answered and printed all the same, and the file `--dump-answer` saves for it loads again: a saved
state is its program and the choices made, and the program is within the limit.
```

### 3.2 The reserved-strings page

Proposed row:

```markdown
| `#nested` | a nested form whose head is not a name. The parse refuses every such form, `(r (_ a))`, `(r (?x a))` and `(r ((f b) a))` among them, so no program reaches it | `(_ …)` |
```

(or the row and the constant dropped together, which is a code change: `NESTED_HEAD` and its place in
`INTERNAL_NAMES`.)

### 3.3 Or leave both

Each is one sentence a reader can check against the binary in a second; the link check passes on both, since each
link resolves.

## 4. The default

**None taken** in the ledger: *"The user's call whether either is worth a stage; each is a sentence."*

## 5. Files

- [`runs/greps.txt`](runs/greps.txt) — the ledger's greps and wider ones, re-taken, with the sections they find and
  the code's own words (`text.rs`, the loader's unit test).
- [`programs/nested-1000.ein`](programs/nested-1000.ein), [`programs/nested-1001.ein`](programs/nested-1001.ein) and
  [`runs/nested-1000.solve.txt`](runs/nested-1000.solve.txt), [`runs/nested-1001.solve.txt`](runs/nested-1001.solve.txt) — at and past the limit.
- [`programs/numeral-999-digits.ein`](programs/numeral-999-digits.ein), [`programs/numeral-1000-digits.ein`](programs/numeral-1000-digits.ein)
  and their runs [`999`](runs/numeral-999-digits.solve.txt), [`1000`](runs/numeral-1000-digits.solve.txt) — the page's 999 digits.
- [`programs/head-underscore.ein`](programs/head-underscore.ein), [`head-variable.ein`](programs/head-variable.ein),
  [`head-form.ein`](programs/head-form.ein), [`head-string.ein`](programs/head-string.ein),
  [`head-variable-in-rule.ein`](programs/head-variable-in-rule.ein) and their `runs/*.solve.txt` — every nested head
  that is not a name, refused.
- [`programs/sum-999-digits.ein`](programs/sum-999-digits.ein) — a sum of two 999-digit numerals (§ 7);
  [`runs/sum-999-digits.solve.txt`](runs/sum-999-digits.solve.txt), its saved answer
  [`runs/sum-999-digits.dump-answer/`](runs/sum-999-digits.dump-answer/) and the saved file read again,
  [`runs/sum-999-digits.reload.solve.txt`](runs/sum-999-digits.reload.solve.txt) (exit 0); the same under the earlier
  binary `ein-0a4a97a3`: [`solve`](runs/sum-999-digits.before-0a4a97a3.solve.txt),
  [`reload`](runs/sum-999-digits.before-0a4a97a3.reload.solve.txt) (exit 1), and the head of its 132 MB state file,
  [`runs/sum-999-digits.before-0a4a97a3.state-file-head.txt`](runs/sum-999-digits.before-0a4a97a3.state-file-head.txt)
  (the file itself was not kept).

## 6. Re-run

```sh
B=build/review-bin/ein-head-b5fb8d70; D=plans/m1f_hypothesis_and_documentation/review/04-language-and-decisions/23-L-93
grep -n -i 'nested past\|1 000 parenth' docs/ein/user/04-reference/07-diagnostics.md
grep -n '(r (_ a))' docs/ein/ein.rs/07-reserved-engine-strings.md
for p in nested-1000 nested-1001 numeral-999-digits numeral-1000-digits head-underscore head-variable head-form head-string head-variable-in-rule; do $B solve $D/programs/$p.ein; echo "exit $?"; done
$B solve $D/programs/sum-999-digits.ein --dump-answer /tmp/s999 && $B solve /tmp/s999/models/01.ein; echo "exit $?"
EIN_STDLIB=$PWD/build/wt-rev-0a4/stdlib build/review-bin/ein-0a4a97a3 solve $D/programs/sum-999-digits.ein --dump-answer /tmp/o999
EIN_STDLIB=$PWD/build/wt-rev-0a4/stdlib build/review-bin/ein-0a4a97a3 solve /tmp/o999/models/01.ein; echo "exit $?"   # 132 MB file
```

## 7. Found while building

- **A third stale sentence in the same paragraph** of `08-natural-numbers.md`: *"so a state written by
  `--dump-answer` from operands near that length is refused when it is loaded again, though every answer is right and
  printed"*. It held before saved files were their program (the earlier binary `ein-0a4a97a3` writes a 132 MB state
  file for [`sum-999-digits.ein`](programs/sum-999-digits.ein) and refuses it on reading,
  `…/models/01.ein:9076:3056: this ( nests deeper than the limit of 1000 parentheses`, exit 1); today the saved file
  (145 KB) loads again, exit 0, as Q-M1f.185 and `stdlib/README.md` § How long a number may be (its point 2) say. The
  proposed re-wording is in § 3.1.
- **The code says the same as the reserved-strings page**: `ein-core/src/text.rs:137–139`, `NESTED_HEAD`'s doc
  comment, *"(`(r (?x a))`, `(r (_ a))`): it is no name text can spell, and `_` in head position loads as it"*. Both
  examples are refused at the parse (§ 2.2), and `_` loads nowhere.

## 8. Your call

- **Stdlib page**: (a) link Q-M1f.185 with the line quoted, or (b) a § 1.7 on the diagnostics page and the link aimed
  at it; and the third sentence re-worded (§ 3.1) either way.
- **Reserved-strings page**: the row re-worded (§ 3.2), or the row and the constant dropped.
- Or neither, left as they are.
- **Default**: none taken.
