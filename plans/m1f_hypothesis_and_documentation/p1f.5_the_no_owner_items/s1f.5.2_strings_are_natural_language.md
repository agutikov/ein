# S1f.5.2 — A string is natural language: only in `:why`, `:source` and `:goal-text`

**Phase:** [P1f.5](README.md) (The no-owner items)
**Was S1e2.5.2**, in M1e2's P1e2.5, until 2026-11-08 ([§ The ids](README.md#the-ids)).
**Estimate:** 1.5 days, re-taken on 2026-11-08 on the user's re-plan ([P1f.5 § The order](README.md#the-order));
it was 2.5.
- A quarter of a day for a string held in every position, in the grammar's table (T1f.5.2.2).
- Half a day for the Unicode test and the two examples (T1f.5.2.2).
- A quarter of a day for what only a string used (T1f.5.2.3).
- Half a day for the pages (T1f.5.2.4).
- Re-taking the positions, every move and the real programs (half a day), the refusal and the printer (half a
  day), the escapes, which the grammar's `STRING` states, and the register and the fixtures (a quarter) are
  [S1f.5.5](s1f.5.5_the_grammar.md)'s since the re-plan: its attribution sweep and review stop (T1f.5.5.9) and
  its swap (T1f.5.5.4).

**Status:** specified 2026-11-06 on the user's ruling of that day
([Q-M1e2.23](../../m1e2_correctness/open_questions.md#q-m1e223--what-is-a-string-for)).
**Not started.** No engine code, test, golden or fixture has moved.
**Re-planned on 2026-11-08, on the user's ruling** ([P1f.5 § The order](README.md#the-order)): *"It is the same
work done twice, why not switch to grmtools first and remove hand-written parser and then verify requirement met
(strings, names, etc...) ?"* The refusal and the escapes are the generated parser's (`STRING` only after `:why`,
`:source` and `:goal-text`, and `\u{…}` or a refused escape in the token, in
[the paper grammar](s1f.5.5_paper_grammar.md#the-grammar-by-position)), and nothing edits the hand-written lexer
or parser for them. This stage's step 1 is done inside S1f.5.5's attribution sweep and its one review stop
([T1f.5.5.9](s1f.5.5_the_grammar.md#task-t1f559--every-difference-attributed-and-one-review-stop)), and the
refusal, the printer, the fixtures and the restated rows land in S1f.5.5's swap
([T1f.5.5.4](s1f.5.5_the_grammar.md#task-t1f554--the-swap)), credited to this stage's ruling. What is left,
T1f.5.2.2 to T1f.5.2.4, runs after the swap, on the generated parser.
**Depends on:** S1f.5.5's swap, which lands this stage's refusal with S1f.5.1's and S1f.5.4's in one commit, so
the spellings that replace a string (`_rule`, `1`) are names from the commit that refuses it, and no name is ever
left unwritable.

## The case

Taken on 2026-11-06 at `916d97ac`, on the release binary, on throwaway probes.

**A string has two jobs, and one was never decided.** The first grammar gave
`STRING` to *source-sentence provenance + `:why` templates only* (`c44ccfb5`,
2026-05-18). The Rust loader held a fact's string as the name it spells from its
first commit (`687b91bf`, 2026-08-18; earlier history not traced), `01_grammar.md`
said so only at S1e2.2.16 (2026-11-04), and S1e2.2.18 extended it to rules the same
day, as a default it says the user may override (`259c16e4`). No ruling was found.

**The natural-language job is three keywords.** Every string written after a keyword
in the tree (`examples/`, `stdlib/`, `tests/`, `corpus/`, `plans/`, by the agent's
scan):

| keyword | strings | what it holds |
|---|---:|---|
| `:why` | 826 | a template, `"{?R} not functional: {?a} has both {?b} and {?c}."` |
| `:source` | 261 | a source sentence, `"condition (1)"` |
| `:goal-text` | 7 | a template, `"The {?who_water} drinks water in {?h_water}; …"` |
| `:as` | 1 | `"m"`, in `examples/broken/load/import_alias_not_a_name.ein`, which is refused for it |

**The name job has no user.** 13 files under `examples/`, `tests/` and `stdlib/` write
a string as a value: `syntax/arg-kinds` and 12 witnesses of the defects it caused. The
stdlib writes none. M2's contract writes strings only as `:why` and `:goal-text`
(`s2.2.1_contract.md`, `s2.2.4_passes.md`).

**It caused four defects:** W16, W18, W28 and S-15.

**What only a string can write today, and what replaces it:**

| a name that | written today | written after |
|---|---|---|
| holds a space | `"x y"` | `x_y` |
| is digits, or starts with one | `"1"`, `"1st"` | `1`, `1st` (S1f.5.4) |
| is a reserved word | `"not"`, `"rule"` | `not_`, `_rule` (S1f.5.1) |
| looks like a variable or the wildcard | `"?x"`, `"_"` | nothing: such a name carries no meaning |
| holds a non-ASCII letter | `"Жук"` | **nothing, ever**: names are ASCII (default 3, ruled 2026-11-07). The text belongs in a natural-language keyword |

## The ruling

The user, 2026-11-06, on being shown F4: *"I don't see any reason to use strings as
names, do you? I think we should allow use strings only for NL - :why, :source, and
other places where NL text or NL text template with variables lives; Is there any
objections?"* The agent raised none, and withdrew its recommendation to keep them.

The user, the same day, on F5: *"strings literals are unicode with reasonable escape
symbols, so potentially ein program can emit unicode text, use it in test, dont force
using it in examples but place a couple of examples just to show that option"*.

The user, the same day: *"YES, digits are not numbers, digits are plain names, and refuse ?x
in facts; (p 1) and (p 01) are VERY DIFFERENT facts, (p "1") has to be refused"*.

This overrides S1e2.2.18's default. The rulings behind W16 and W28 stay true of a
name and an int, and now have nothing to tell apart in a string.

## What it changes

*Since the re-plan of 2026-11-08 the refusal, the escapes, the printer, the register and the fixtures are S1f.5.5's
swap, credited to this ruling, and the Unicode test, the two examples and what only a string used are this
stage's, after it.*

**The refusal.** A string is refused everywhere but as the value of `:why`, `:source`
or `:goal-text`, on every command that reads a program, with one line. Proposed
wording, for the user to see in step 1:

```text
FILE:L:C: "a" is a string, and a string is natural-language text, written only after :why, :source or :goal-text: write a name bare
```

| position | today | after |
|---|---|---|
| `:why`, `:source`, `:goal-text` | text | unchanged |
| fact argument, nested or not, `is-a`: **`(p "1")`** and `(p "a")` | the name it spells | **refused** (the ruling) |
| a variable as a fact argument, `(r ?x)` (default 1) | a name printed `"?x"` | **refused** (the ruling) |
| rule `:match` and `:assert` argument, guard, hrule | the name it spells (since S1e2.2.18) | refused |
| activator argument, `(chase "link")` | the name it spells | refused |
| query goal, `:expect` | the name it spells | refused |
| `:priority "5"`, `:priority "high"` | loads, effect not checked | refused |
| `(config :x "false")` | acts as `false` | refused |
| any other keyword, `:k "v"` | see step 1 | refused |
| a head, `(relation "p" T)`, a rule's name or parameter, an import, `:hypothesis-relations` | refused | refused, with the same line where it can be |

**The text is Unicode, and that is a feature to hold.** It mostly works today: a probe
with Russian, Japanese and an emoji in all three keywords printed *“Жилец: Ann, дом: H1”*
in the read-out, *Ann живёт в H1 — «日本語», emoji 🦓* in `--trace`, the `:source` sentence
beside its premise, and the text in `--dump-answer`'s `answer.txt` and state files
(2026-11-06). 192 non-ASCII strings in 45 files already pass through (`⟹`, `→`; the
agent's scan of `examples/`, `tests/` and `stdlib/`). On that probe `ein render rules` and
`--events` carried none of it, and step 1 reads whether each carries the text at all.

**The escapes.** `00_ebnf.md`'s fact 4 is *minimal*, and an unknown escape loses its
backslash in silence: `"A\u0416B \x41 \101 \u{1F993}"` stores `Au0416B x41 101 u{1F993}`
(2026-11-06). The ruling asks for *reasonable* ones. The agent's reading, to show in step 1:

| written | means |
|---|---|
| `\n` `\t` `\r` `\"` `\\` | as today |
| `\u{…}`, one to six hex digits | **new:** that code point, for what is awkward to type |
| any other `\X` | **refused at load**, where it is dropped in silence today |
| a raw newline or a raw non-ASCII character | as today: multi-line text needs no escape |

The printer and `dump.rs`' `escape_string_literal` stay the inverse of this.

**The test.** A program under `tests/` whose three keywords hold Cyrillic, CJK, an emoji
and every escape, and a test in `ein-cli/tests/` holding the text byte for byte on every
surface that carries it.

**The two examples, to show the option and no more.** `examples/features/14_unicode_text.ein`
(a small puzzle with Russian `:why`, `:source` and `:goal-text`) and
`examples/features/15_unicode_escapes.ein` (other scripts, an emoji, `\n`, `\u{…}`). Names
stay ASCII. Each gets a `corpus.toml` entry and a line in `examples/README.md`. **No other
example changes.**

**The printer.** A value never prints quoted once default 1 holds, since every name
then lexes bare. `ein_core::text::quote` stays for natural-language text, and a
`--dump-answer` state file must still reload to its state by value
(`ein-cli/tests/answer_round_trip.rs`).

**What only a string used.** The `Node::Str` arms that make a value: `from_ir.rs:567`,
`compile.rs:1016`, `:1083` and `:1270`, `expect.rs:297` and `:338`, `invariant.rs:502`.
Step 1 reads the rest: `trace/ast.rs:207` and `:337` read an engine-written
`(trace …)`, whose strings have to stay readable if they are natural language.

**The register.** No new case, since each move is a restated `right`: W16's 4 rows,
W18's 5 and W28's 3 are refused where a witness writes a string as a value. All land in S1f.5.5's swap, where
S1f.5.4's proposals for the W16 and W28 rows meet this refusal and give way to it, and this stage owns no row of
its own after it. 30 lines of
`corpus/wrong_answers.toml` hold a quoted value (`grep -cE 'models.*\\"|\(.*\\"'`,
2026-11-06). Whether W16's and W28's value-keyed fixes can then be simplified is
step 1's to report, and they stay unless the user says otherwise.

**The fixtures.** `syntax/arg-kinds` loses its strings. The 12 witnesses move to
refused, each keeping a twin or a banked form that shows the fix they held, or
retiring where the case no longer exists. Step 1 lists which.

**The goldens and tests, named before they move** (step 1 completes this list):
`corpus_shapes.md5`'s `ir[parse]` rows; `grammar_decisions.txt`; `lex.rs`' and
`text.rs`' unit tests; `claim`'s and `expect`'s tests; `answer_round_trip.rs`.

**What it does to other items.**
- **S-15 shrinks but stays.** `(p "a")` can no longer be written, and `(p 01)` against a
  stored `(p 1)` becomes a true difference (S1f.5.4). A models file writing `(p  a)` or
  `( p a )` still gets *0 matched* against a stored `(p a)` (2026-11-06), so reading the
  file's facts through the parser is still its fix, which [S1f.5.5](s1f.5.5_the_grammar.md#task-t1f556----diff-models-compares-by-value-s-15)
  makes (ruled 2026-11-07).
- **F2's `"a"b`** closes for a value. A number or a variable still splits.
- **S1f.5.1's default 3** (`"_"`) is overridden: the string is refused.

## Defaults

1. **G-6: a variable in a fact is refused.** Ruled 2026-11-06. It also keeps a state file
   reloadable, since `(r ?x)` stores a name printed `"?x"`. Only `syntax/arg-kinds` and
   `syntax/is-a-shapes` write one. A macro's body is read after expansion, and a rule's
   pattern and a query's goal keep their variables. The range half goes with `RANGE`
   ([S1f.5.3](s1f.5.3_range_retired.md)).
2. **The list is closed.** A new natural-language keyword joins by adding it to the
   list and the grammar. `:as "m"` stays refused.
3. **Names are ASCII, forever.** Ruled 2026-11-07, replacing this default's *parked for
   M2*. The user: *"names STAY ASCII FOREVER! никаких Жуков в коде! В NL строках -
   пожалуйста, в строках что угодно, но не в коде"* (*no "Жук"s in the code; in
   natural-language strings, anything, but not in code*). `00_ebnf.md` already says a
   non-ASCII character appears only inside a `STRING` or a comment, and that stays true
   for good. Which characters a name may hold is
   [S1f.5.5's list](s1f.5.5_the_grammar.md#the-name-rule).
4. **`\u{…}` is the one Unicode escape**, since it reaches every code point. No `\xNN`,
   `\uNNNN` or octal.

## The method

### Task T1f.5.2.1 — every position re-taken, every move listed, real programs shown

**Done inside S1f.5.5 since the re-plan of 2026-11-08**: its attribution sweep credits every string difference to
this ruling, and its one review stop shows what step 4 below lists, the two examples as a probe until T1f.5.2.2
writes them ([T1f.5.5.9](s1f.5.5_the_grammar.md#task-t1f559--every-difference-attributed-and-one-review-stop)).
Two of its questions the paper grammar settled: `:priority` takes `DIGITS`, so `:priority "high"` is a parse
error, and a trace step's `:source` holds a string, as natural language. The task as it stood is that sweep's
brief for this ruling:

1. Re-take the table on a fresh build, release and debug, and settle the two rows it
   leaves open: what `:priority "high"` does today, and any other keyword's string.
2. Read `(trace …)`'s strings, and say whether they are natural language.
3. Prototype on a `git archive` snapshot under `build/`, on top of S1f.5.1. Sweep every
   `.ein` under `examples/`, `tests/`, `stdlib/`, `corpus/` and `plans/`, and run
   `cargo test --workspace` on the snapshot. List every move, and any `.ein` that writes an
   escape the new set refuses.
4. **Stop and show the user real programs before any edit** (the standing rule on
   syntax): `arg-kinds`, one witness each of W16, W18 and W28, `zebra2.ein` with its
   `:why`, `:source` and `:goal-text` untouched and its answer unmoved, and the two new
   examples with what each surface prints.

**Stops if** any program outside `arg-kinds`, `is-a-shapes` and the 12 witnesses moves.

### Task T1f.5.2.2 — held in every position, and shown in Unicode

The refusal, the escapes, the printer, the fixtures and the register rows **moved to S1f.5.5's swap on
2026-11-08** ([T1f.5.5.4](s1f.5.5_the_grammar.md#task-t1f554--the-swap)). What is left, after the swap: a test per
position, a string refused in each and `(p "1")` by name, as rows of the grammar's table S1f.5.5's T5 builds; then
the Unicode test and the two examples, as [§ What it changes](#what-it-changes) says.

### Task T1f.5.2.3 — what only a string used

After the swap and S1f.5.5's T5: the `Node::Str` arms that make a value, retired as a separate commit, so that no
answer moves in it.

### Task T1f.5.2.4 — the pages

After the swap, beside S1f.5.5's T7.

`00_ebnf.md` (fact 4 and the line that says where a non-ASCII character may appear),
`01_grammar.md` (the `STRING` row), `02_patterns.md`,
`defined_behaviour.md` (the lines at 407 and 547, and the refusal), `glossary.md`,
`examples/README.md`, and the docs that show a quoted value. Q-M1e2.20 and S1e2.2.18's
default each say what overrode them. `docs/history/` is a record, and does not change.

## Acceptance

**Since the re-plan of 2026-11-08 the items the grammar settles are
[S1f.5.5's acceptance](s1f.5.5_the_grammar.md#acceptance)**, held at its swap: a string refused outside the three
keywords, `(r ?x)` refused, every `:why`, `:source` and `:goal-text` reading as it did, `\u{…}` its code point and
an unknown escape refused, and every state file reloading by value with no quoted value. This stage's own, after
the swap:

- A test holds each position, a string refused in it, `(p "1")` by name, and a variable in a fact.
- Unicode in the three keywords reaches every surface that carries them byte for byte, and
  a test holds it.
- The two examples are in the corpus, and no other example changed.
- The `Node::Str` arms that make a value are gone, and that commit moved no golden.
- `./run_tests.sh` is green.
