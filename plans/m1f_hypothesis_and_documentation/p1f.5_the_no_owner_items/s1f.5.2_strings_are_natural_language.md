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
**In progress since 2026-11-09**, on the generated parser: T1f.5.2.5 done, W38 filed and fixed ([its
record](#t1f525-record-2026-11-09)); T1f.5.2.2 done, the ruling held in every place and the text on every surface
([its record](#t1f522-record-2026-11-09)); T1f.5.2.3 done, no string a value anywhere in the engine, each arm
shown unreachable first, no answer and no golden moved ([its record](#t1f523-record-2026-11-09)); T1f.5.2.4 next.
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

**Named before it moves, 2026-11-09** (`cargo test --workspace --no-fail-fast` on the change, unblessed, 949 tests,
three failing, `build/s1f.5.2/t2-discover.log`, and the grammar's table, run alone first):
- `ein-ir/tests/golden/grammar_table.txt`: 49 rows appended after the 141 S1f.5.4 left, so none renumbers;
- `ein-cli/tests/golden/corpus_exits.txt`: the three new programs' 15 cells, each exit 0;
- `ein-render/tests/golden/corpus_shapes.md5`: the three new programs' rows, and the rows of the two W24 witnesses
  whose views print the guard line (`form-in-guard`, `guard-before-binder`), which is restated (below);
- `ein-cli/tests/test_cli.rs`' `a_query_with_no_expectation_is_never_solved`, which counts the claims `ein test
  examples/features` holds: 4 → 6, the two examples;
- `corpus/wrong_answers.toml`: `W24/form-in-guard`'s and `W24/guard-before-binder`'s `now` (the guard line) and the
  first's `right`, restated with no status moving, and W38's `[case.fixed]`, which names `90774a44`;
- `defined_behaviour.md` § 4.9's two rows that quote the guard line.
No answer of any program moves.

### T1f.5.2.2 record, 2026-11-09

**One commit, on `90774a44`** (T1f.5.2.5). The ruling is held in every place a value goes, and the text it keeps is
held on every surface that carries it.

**The grammar's table** (`ein-ir/tests/grammar_pages.rs`), 49 rows at its end (141 → 190; `cargo test -p ein-ir
--test grammar_pages`, 2026-11-09), each refused at the parse at the place it names: a string as a fact's argument,
`(p "1")`, `(p "a")` and `(p "Жук")` first, a nested fact's, a stored negation's, an `is-a` fact's, an activator's
(`(chase "link")`), a fact's `:rule` and `:using`; a variable in a fact, nested or negated (G-6); a string as a
relation's name or signature, a rule's name or parameter, a premise's relation or argument, a pattern inside a
premise, a guard's argument (`eq`, `neq`), an `absent`'s or a stored negation's premise, a conclusion's relation or
argument, an `(open …)`'s relation, an hrule's premise and conclusion; a query's goal and `:over`, a claim's fact
and `neq`, a region claim's fact, a solve's relation lists and `:hrules`, a config flag's value; a macro's name,
parameters and body, an import's module and symbols, a trace step's name and `:derives`; and a text keyword where its
form takes none (`:why` on a fact or a query, `:source` on a relation), refused at the keyword. `:priority "5"` is
refused at `:priority`, which no rule takes since S1f.5.15. Eight rows above them already wrote a string (a form's
head, an import's alias, an `absent`'s premise, a conclusion's argument and `:source`, `:expect`, a claim's fact, a
config flag's value).

**Every command, `(p "1")` by name** (`cli_semantics::a_string_is_natural_language_and_no_value`): `ein saturate`,
`solve` and `oracle` exit 1 and `ein test` 2 on `(p "1")`, `(p "a")`, `(p (q "x y"))`, a rule's `:match (p "1")` and
a query's goal `(p "1")`, each with the line at the string, *unexpected `"1"`: a string*, and S1f.5.2's note; and on
`(r ?x)`, at the variable, with no note.

**The guard line, restated** (the coordinator's brief: *"…a name, a string or an integer". It is now false twice
over*). `from_ir`'s `GUARD` says *a guard compares two values, each a variable a premise before it binds or a name*,
a program writing neither a string nor an integer there since the swap. Its two tests (`from_ir`'s and
`cli_semantics`' `a_rule_shape_that_reads_nothing_is_refused_at_load`), `defined_behaviour.md` § 4.9's two rows,
and the register's `W24/form-in-guard` (`now`, `right`) and `W24/guard-before-binder` (`now`) say it, each row's
note saying why and keeping its fix, as T4 restated its 23 rows; the register's header says so.

**Unicode, held byte for byte** (`ein-cli/tests/natural_language_text.rs`, seven tests). `tests/text/01_unicode.ein`
writes a relation's and a rule's `:why`, two `:source`s and a `:goal-text`, each with Cyrillic, CJK and an emoji, the
five escapes and `\u{…}`, one `:source` with a raw newline; its claim holds. The test holds: the parser reads the six
texts as written; the read-out and `answer.txt` print the `:goal-text` filled for each row, `“…”`; `--trace` prints
the rule's `:why` filled at each firing and each premise's `:source`; `ein saturate --dump` each `:source` and the
`:goal-text` quoted (`ein_core::text::quote`); a state file writes all six quoted and reads back to them; `ein render
program` draws the `:goal-text` in the query's label; and `ein solve` of a `.einb` prints what `ein solve` of the
`.ein` does. Which surfaces carry a text is what they did before the swap, none added
([Q-M1f.12](../open_questions.md#q-m1f12--which-surfaces-carry-natural-language-text), defaulted).
`tests/README.md` has a `text/` section.

**The two examples**, each with a corpus entry and a clause in `examples/README.md`'s `features/` line, and no other
example changed: `examples/features/14_unicode_text.ein`, a three-house puzzle whose `:why`, `:source` and
`:goal-text` are Russian, solved by two rules in saturation, R {∅} and its solve claim held (*“Anna живёт в доме
H1”*); and `examples/features/15_unicode_escapes.ein`, the escapes and text in Spanish, Japanese and Greek, the
`:goal-text` printed over two lines with 👋 and 🌍, its query claim held. `ein test examples/features` holds six
claims, where it held four (`test_cli`'s count, restated).

**The register's id.** W38's `[case.fixed]` names `90774a44`, where T1f.5.2.5 left the null id, and the header's
paragraph says so. The register reads **119 rows fixed, 21 open and 1 not-wrong** (`grep '^status'
corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-09).

**What moved, as named above, and nothing else.** `grammar_table.txt` (`EIN_BLESS=1 cargo test -p ein-ir --test
grammar_pages`), `corpus_exits.txt` (the three programs' 15 cells, `-p ein-cli --test corpus_cli
every_cell_reproduces`) and `corpus_shapes.md5` (123 new rows, 41 a program, and 14 moved, the two W24 witnesses'
views of their load error, `-p ein-render --test corpus_shapes`). Every `.ein` under the five roots (515, `find
examples tests stdlib corpus plans -name '*.ein' | wc -l`, 2026-11-09) through `90774a44`'s release build and this
commit's (`build/s1f.5.2/xcheck/`), on `solve --json-summary -H --dump-answer`, `saturate --dump` and `test`, and
T12's 2 000 mutants through `saturate`, timings masked (91 s, 2026-11-09): **513 programs the same on every
command**, and the two W24 witnesses, whose stderr differs by the guard line alone on each of the three; and 1 993
mutants the same (1 047 accepted, 767 refused with repairs, 179 without) and seven, each a mutant of a W24 witness,
differing by the guard line alone. The release binary moved nothing else: the three new programs answer alike on
both, the build of `90774a44` reading every text as this one does.

**Taken by default on the user's instruction of 2026-11-09** (*"just continue with default decisions till the end of
the phase"*):
- **The surfaces that carry text are today's**, held byte for byte, none added (Q-M1f.12 (a)).
- **The guard line's restatement**, *each a variable a premise before it binds or a name*: it drops the two kinds no
  program can write and adds nothing a later stage has not ruled (whether a fact is a guard's value is S1f.5.7's).
- **The test program under `tests/text/`**, a third directory beside `region/` and `stdlib/`, since it tests the
  language and not a stdlib rule; and **every escape in each of the three keywords**, `\r` among them, which the
  read-out and the trace print as the character, as they print any text.

**The gate.** On this commit, 15 min 52 s (`build/gate-s1f.5.2-T2.log`), green: 949 tests and every static check,
T1f.5.2.5's 941 and eight: `cli_semantics`' one and `natural_language_text`'s seven; the table's 49 rows are rows of
one test.

### Task T1f.5.2.3 — what only a string used

After the swap and S1f.5.5's T5: the `Node::Str` arms that make a value, retired as a separate commit, so that no
answer moves in it.

**The sites, read 2026-11-09** (`grep -rn 'Node::Str' ein.rs/crates`, on `bbab388a`), the stage's list re-taken after
the swap and T5, which deleted the trace readers it named (`trace/ast.rs`' two):
- **a value, each to go**: the loader's `atomic_value` (a fact's argument), the compiler's `slot` (a pattern's),
  `guard_arg` (a guard's) and `encode_node` (a guard's key, code 4), `invariant::walk_constants` (a rule's constant,
  with the `kw` flag that only it read), `expect::render` (a claim's fact) and `ir_dot`'s `atom_id_for_value` (a DOT
  node for an argument);
- **G-6's half**, the same ruling: `atomic_value`'s `Var` arm and `Terms::value_var`, a variable in a fact stored as
  the name `?x`, which the grammar refuses since the swap;
- **a text, each kept**: the builder that makes the node, `eq_nodes` and `node_repr`, the dumper (`escape_string_literal`,
  the one printer's `quote`), `string_value` (`:why`, `:source`), `macros`' `:why`, `read_out`'s `:goal-text`, and
  `value_label`, which labels a query's `:goal-text` in `ein render program`'s note;
- **one that reads a text as a name**: `from_ir::intern_program_names` interns every leaf of a query's key pairs, a
  `:goal-text`'s text among them, as a name nothing asks for. It goes: a text names nothing.

**Named before it moves, 2026-11-09.** No golden and no answer. `ein-core`'s unit test
`atom_string_and_var_flatten_to_one_text_value`, which built a string's and a variable's values by hand, becomes
`a_name_is_its_texts_one_value`; the interner no longer holds a query's `:goal-text` as a symbol, which only a
`.einb`'s symbol table could show, so the cross-check reads every container both ways.

### T1f.5.2.3 record, 2026-11-09

**One commit, on `bbab388a`** (T1f.5.2.2). A string is no value anywhere in the engine: the arms that made one are
gone, each shown unreachable first, and **no answer and no golden moved**.

**How unreachable was shown.** A copy of `bbab388a` (`git archive`, `build/s1f.5.2/t3/probe/`) with a panic,
`S2-PROBE <id>`, at each of eleven sites (`build/s1f.5.2/t3/probe_patch.py` places them by their text, T1f.5.4.3's
harness): the seven value arms above, `atomic_value`'s `Var` arm and `Terms::value_var`, and the two walks that also
meet a text, `intern_program_names` and `value_label`, each probed only where the string is not a key pair's value
(the copy handles a key pair's text, and `ir_dot`'s `render_note`'s, as the walk did, before the probe). Each probe
logs its id, the thread and argv, so one reached inside a child `ein` would be named (`build/s1f.5.2/t3/hits.log`).
- `cargo test --workspace --no-fail-fast` on it (`build/s1f.5.2/t3/probe-tests.log`, 15 min 55 s): 948 passed and 1
  failed, the failure a probe in a test binary, `ein-core`'s `atom_string_and_var_flatten_to_one_text_value`, which
  builds a variable's value by hand; **no probe in an `ein` process**, no corpus sweep, golden, fuzz property or CLI
  run reached one.
- Every corpus file under 6 000 bytes (402) and 40 000 seeded mutants of them (`ein_corpus::mutants`, seed 0), then
  200 000 more (seed 987654321), each parsed, drawn (`ein render program`'s, `rules`' and `constraints`' functions),
  dumped, loaded, saturated and printed (`s2_sweep`, an example of `ein-render` in the copy,
  `build/s1f.5.2/t3/s2_sweep.rs`): 16 383 refused at the parse, 2 692 at load, 86 at compile, 21 157 saturated and 84
  stopped saturating, and 81 873, 13 094, 348, 104 583 and 504, **no probe and no other panic** (`sweep.txt`,
  `sweep2.txt`). A first run, without `render_note`'s exemption, met 464 strings there, every one a query's
  `:goal-text`, a text: the exemption is what tells the two apart.

And by construction: the grammar builds a `Str` only as the value of `:why`, `:source` or `:goal-text` (`ein.y`'s
`Text`, in six places: a fact's and a trace step's `:source`, a relation's, a rule's and a macro's `:why`, a query's
`:goal-text`), and no fact, pattern, guard or claim takes one; and it builds no variable in a fact (`FactArg`).

**Removed.**
- `ein-ir`: `atomic_value`'s string and variable arms, now one `unreachable!` naming `FactArg`; `expect::render`'s
  string arm (`ClaimFact`); and `intern_program_names` interns a text no more.
- `ein-core`: `Terms::value_var`; `value_text`'s doc, which said `(likes A "foo")` and `(likes A foo)` are one fact
  *today*, says what it was.
- `ein-infer`: `compile::slot`'s and `guard_arg`'s string arms and `encode_node`'s code 4, each an `unreachable!`
  naming the production; `invariant::walk_constants`' string arm and its `kw` flag, which only that arm read.
- `ein-render`: `ir_dot::atom_id_for_value`'s string arm, folded into the arm that panics on a keyword.
Each `unreachable!` is Q-M1f.9's convention; the comments that described a string as a live value (the compiler's
`slot`, `plan::GuardArgKind::Const`, the AST's `Node::Str`) say what it was and what it is.

**Kept, and why.** The builder that makes a `Str` and the arms that read one as text: `eq_nodes`, `node_repr`, the
dumper (`escape_string_literal`, the one printer's `quote`), `string_value` (`:why`, `:source`), the macro reader's
`:why`, `read_out`'s `:goal-text`, and `value_label`, which labels a query's `:goal-text` in `ein render program`'s
note as it did (bare where the text spells one name, quoted otherwise; no corpus `:goal-text` is one name). A fact's
`:source` is still interned as provenance, which is storage and no value. `ein_render::trace`'s `RefArg::Str` holds a
name's text and is named for it, not for a string.

**Nothing moved.** Every `.ein` under the five roots (515) through `bbab388a`'s release build and this commit's
(`build/s1f.5.2/xcheck/`, 150 s, 2026-11-09): `solve --json-summary -H --dump-answer`, `saturate --dump` and `test`,
timings masked, **515 of 515 the same on every command**; T12's 2 000 mutants through `saturate` the same (1 047
accepted, 767 refused with repairs, 186 without); and **the container both ways** (`einb.sh`): each file `ein kb
save`d by each build and each container read by both with `solve`, the four reads agreeing with each other and with
`solve` of the `.ein` on all 342 that load but `g14/derived-activator.ein`, whose four reads agree with each other and
place its compile refusal at `<string>`, as T1f.5.4.3 found; 12 of the 343 containers differ in bytes, each a program
with a `:goal-text`, whose text the symbol table no longer holds. No golden moved: the gate below reads every one.

**Taken by default on the user's instruction of 2026-11-09** (*"just continue with default decisions till the end of
the phase"*):
- **G-6's variable arm goes with the string's**: the same ruling, the same unreachability, and `Terms::value_var` has
  no other caller.
- **A text is no longer interned as a name**, which moves a container's bytes and no answer: a `.einb` is a private
  cache (AGENTS.md), and the reads agree.
- **`value_label` keeps labelling a text as a name prints**: quoting every text would move no corpus view but would
  change a one-word `:goal-text`'s label, which this commit may not; S1f.5.5's T7 or a later stage may.

**The gate.** On this commit, 15 min 41 s (`build/gate-s1f.5.2-T3.log`), green: 949 tests and every static check,
as T1f.5.2.2's, one `ein-core` unit test renamed; no golden file is in the commit.

### Task T1f.5.2.4 — the pages

After the swap, beside S1f.5.5's T7.

`00_ebnf.md` (fact 4 and the line that says where a non-ASCII character may appear),
`01_grammar.md` (the `STRING` row), `02_patterns.md`,
`defined_behaviour.md` (the lines at 407 and 547, and the refusal), `glossary.md`,
`examples/README.md`, and the docs that show a quoted value. Q-M1e2.20 and S1e2.2.18's
default each say what overrode them. `docs/history/` is a record, and does not change.

### Task T1f.5.2.5 — an escape that names no character (W38)

**Added 2026-11-09**, from S1f.5.5's [T4 record](s1f.5.5_the_grammar.md#t4-record-2026-11-09): *a `\u{…}` beyond
Unicode's range still reads as U+FFFD (`build_ast.rs`' `unescape`), which no file writes; refusing it is S1f.5.2's
after the swap*. The grammar's `TEXT` took one to six hex digits of any value, so `\u{110000}`, one past 10FFFF, and
`\u{D800}`, a surrogate, which is a code point and no character, loaded, and the parser read each as U+FFFD in
silence. [§ The escapes](#what-it-changes) gives `\u{…}` the meaning *that code point* and refuses any other escape;
a value no character has is neither. **Run first of the stage's remaining tasks**, so that the Unicode test
(T1f.5.2.2) holds the escapes as they end.

**A wrong answer, so a register row first.** A `:goal-text` writing one printed `�` in the read-out, a sentence the
register's schema /15 records (W37's precedent: the user filed a template that printed wrong as a row before it was
fixed). `W38/escape-past-unicode` is filed and fixed in the same commit, as W13 and W24's ruling rows were, its
`wrong` taken on `3de122a4`.

**The fix.** `ein.l`'s `TEXT` reads a `\u{…}` only where its digits name a Unicode scalar value, 0 to 10FFFF and not
D800 to DFFF, in one to six digits; any other is the `BADESCAPE` token, refused at the parse at the string's opening
quote, as an unknown escape is. A `BADESCAPE` gains a note per escape it does not read, naming the escape and its own
line and column, with the escapes a string reads. `unescape`'s U+FFFD fallback and its arms for an unknown escape
become an `expect` and an `unreachable!` naming `TEXT` (Q-M1f.9's convention).

**Named before it moves, 2026-11-09** (`cargo test --workspace --no-fail-fast` on the change, unblessed, 941 tests,
four failing, `build/s1f.5.2/t5-discover.log`):
- `ein-ir/tests/golden/grammar_decisions.txt`: the one row that writes an unknown escape, `(a "h\di")`, gains its
  note;
- `docs/kernel/ir/03-ein-lang/00_ebnf.md`'s lexical block, `TEXT`'s line, which `grammar_pages` writes from `ein.l`;
- `ein-cli/tests/golden/corpus_exits.txt`: the witness's two cells, `solve` and `saturate`, each exit 1;
- `ein-render/tests/golden/corpus_shapes.md5`: the witness's one row, `ir[parse]`;
- `corpus/wrong_answers.toml`: W38's row, `fixed`, with the null id.
No answer of any other program, and no other golden.

### T1f.5.2.5 record, 2026-11-09

**One commit, on `3de122a4`** (S1f.5.4 done). W38 is filed and fixed: a `\u{…}` that names no character is refused
at the parse.

**The token.** `ein.l`'s `TEXT` reads `\u{` then one of six alternatives then `}`: the values of one to six
significant hex digits, each after the leading zeros that keep it within six, the four-digit one leaving out D800 to
DFFF. So `\u{0}` to `\u{D7FF}`, `\u{E000}` to `\u{10FFFF}` and the same values zero-padded to six digits read as
their character, and `\u{110000}`, `\u{D800}` to `\u{DFFF}`, `\u{0000041}` and `\u{}` make the string a `BADESCAPE`,
refused at its opening quote, *unexpected `"…"`: a string with an unknown escape*. The error gains a note per escape
the string does not read (`grammar::notes`, through `build_ast::bad_escapes`), with the escape's own line and column:
*`\u{110000}, at 37:38, is no escape a string reads: a string's escapes are \n, \t, \r, \", \\ and \u{…}, one to six
hex digits naming a Unicode scalar value, 0 to 10FFFF and not D800 to DFFF`*; an unknown escape gets the same note
(`\d`, `\q`). `unescape`'s U+FFFD fallback is an `expect` and its arms for any other escape an `unreachable!`, each
naming `TEXT`; the lexer gives every other input the tokens it gave (`the_lexer_gives_the_tokens_t4s_did`, which
derives its reference from the shipped `ein.l`, passes over its 21 083 inputs).

**The test.** `generated_parser::a_strings_escapes_are_the_rulings`: the five escapes and raw Unicode read as
themselves, nine escapes the ruling does not give are refused with their note, and every boundary of the scalar
values (0, 7F, 7FF, D7FF, D800, DFFF, E000, FFFF, 10FFFF, 110000, FFFFFF and their neighbours) in every width from
its shortest to seven digits, both cases, with every 257th value up to 11FFFF in its shortest and its six-digit
spelling, each read as its character exactly where `char::from_u32` names one in six digits or fewer and refused with
its note otherwise: 17 432 read and 1 130 refused (`cargo test -p ein-ir --test generated_parser
a_strings_escapes_are_the_rulings`, 2026-11-09).

**The register.** `W38/escape-past-unicode`, class `read-out`, owned by this stage, `fixed`: its `wrong`, taken on
`3de122a4`'s release build, R {∅} and the sentence *“a � �”*, exit 0; its `right` and `now` the parse refusal. Its
`[case.fixed]` carries the null id, `0000000`, and the stage's next commit writes the hash. The witness,
`examples/ein-bugs/wrong-answers/escape-past-unicode.ein`, has a corpus entry (`regression`, `solve` and `saturate`)
and a clause in `examples/README.md`'s `ein-bugs/wrong-answers/` line, which counts 138 programs (`ls
examples/ein-bugs/wrong-answers/*.ein | wc -l`, 2026-11-09). The register reads **119 rows fixed, 21 open and 1
not-wrong** (`grep '^status' corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-09).

**What moved, as named above, and nothing else.** The four goldens (`EIN_BLESS=1 cargo test -p ein-ir --test
grammar_decisions --test grammar_pages`, `-p ein-cli --test corpus_cli every_cell_reproduces`, `-p ein-render --test
corpus_shapes`). Every `.ein` under the five roots (512, `find examples tests stdlib corpus plans -name '*.ein' | wc
-l`, 2026-11-09) through `3de122a4`'s release build and this commit's (`build/s1f.5.2/xcheck/`, T1f.5.4.3's scripts
pointed at the two, 91 s): `solve --json-summary -H --dump-answer`, `saturate --dump` and `test`, exit codes, stdout,
stderr, the summary and every file the dump wrote compared, timings masked, **511 the same on every command**, and
the witness, which loaded and is refused; and T12's 2 000 seeded mutants through `saturate`, the error text included,
the same (1 047 accepted, 767 refused with repairs, 186 without). No corpus program but the witness writes a `\u{…}`.

**Taken by default on the user's instruction of 2026-11-09** (*"just continue with default decisions till the end of
the phase"*):
- **Refused at the token**, in `ein.l`'s `TEXT`, where the stage's escapes table puts the escapes (*`\u{…}` or a
  refused escape in the token*), rather than by a check after the parse: the generated grammar page states exactly
  the strings that parse.
- **A register row, filed and fixed in one commit**, since the read-out printed a sentence the program did not write
  (W37's precedent), and no stage owned the fix but this one.
- **A note on every `BADESCAPE`**, naming each escape and where it is, which moves `grammar_decisions.txt`'s one
  unknown-escape row: the token's description says *unknown escape* and the note says which.

**The gate.** On this commit, 15 min 51 s (`build/gate-s1f.5.2-T5.log`), green: 941 tests and every static check,
T1f.5.4.5's 940 and `a_strings_escapes_are_the_rulings`.

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
