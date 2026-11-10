# S1f.5.23 — `open` is reserved: a program writes it only as a rule's verdict

**Phase:** [P1f.5](README.md) (The no-owner items)
**Status:** specified 2026-11-10 on the user's ruling on the close review's finding 3
([close_review.md § Findings with no owner](close_review.md#findings-with-no-owner)). T1f.5.23.1, the explanation the
ruling asks for, was taken by the coordinator the same day ([its record](#t1-record-2026-11-10)), and its one open
part, `open` as a value, was put to the user as
[Q-M1f.66](../open_questions.md#q-m1f66--may-open-be-written-as-a-value) and **ruled the same day, (a)**: refused
(the user chose *"Refuse it"*), so `open` is written only as a rule's `(open)` or `(open ?R)`. **Done 2026-11-10**, in
one working tree for the coordinator to commit: T1f.5.23.2 ([its record](#t2-record-2026-11-10)), fifty-two places a
program can write `open`, each on every command, thirty-eight of them loading and answered, W70 to W72, twenty-four
rows, filed `open` and held to the unchanged engine; T1f.5.23.3 ([its record](#t3-record-2026-11-10)), `open` one of the
words that are never a name, `ein.y`'s `Assert` the one production that takes it, so every other place is refused at
the parse, at the `open`, with a note, by every command, the views and `kb save` among them, `validate_open`'s five
placement lines deleted and its five projection lines placed, the twenty-four rows `fixed`, and the pin; T1f.5.23.4
([its record](#t4-record-2026-11-10)), the pages; the acceptance re-taken
([§ Acceptance, re-taken](#acceptance-re-taken-2026-11-10)). Four questions taken by default, Q-M1f.86 to Q-M1f.89.
The register reads 217 fixed, 13 open and 1 not wrong (`grep '^status' corpus/wrong_answers.toml | sort | uniq -c`,
2026-11-10). Four wrong answers found outside the stage are recorded in the T2 record, not fixed, for
[P1f.7](../p1f.7_findings_outside_scope/README.md), where they are S1f.7.6 to S1f.7.9. Committed as `0caf9dd2`, whose hash
W70 to W72's `[case.fixed]` rows carry.
**Depends on:** nothing since Q-M1f.66's ruling. It may edit `ein.y`, so it does not run beside
[S1f.5.22](s1f.5.22_unicode_only_in_a_string.md).

## The ruling

The close review's finding 3, as written on 2026-11-10:

> **`(p (open x))` loads and answers**: R has 2 states. Yet no rule may match `(open …)`, the verdict atom. It is
> open whether such a fact should be refused at load.

The user, 2026-11-10: *"new stage, Explain in more details, with program example, `open` is reserved word, here
it appers as what? as relation name? - then it is an error, if not - then explain"*.

## T1 record, 2026-11-10

Taken by the coordinator on the release binary at `21c24c07`.

**What `open` is.** A rule's `:assert` may be `(open)` or `(open ?R)`. That is the verdict atom: the rule
states an obligation, which is not stored, never fires a rule, and is read by one pass over the quiescent KB. The
state it is owed in reports *Open — owes n*. Four stdlib rules are written this way (`total-owed`,
`surjective-owed`, `slot-owed-room`, `slot-owed-fill`). The kernel interns `open` as its symbol 1
(`ein-core/src/entities.rs`).

**Where the grammar lets a program write it.** `open` is one of the grammar's *position words*: a token of its
own that is a keyword in one position and a name everywhere else. That is the design S1f.5.5's paper grammar took
for `model`, `region` and the rest (Q-M1e2.17). The productions that admit it are `Name`, `FactHead` and `ArgHead`
in [`00_ebnf.md` § 2](../../../docs/kernel/ir/03-ein-lang/00_ebnf.md#2-phrase-grammar). The loader then refuses one
case: an `(open …)` form under a rule's `:match` (`from_ir.rs`'s `validate_open`).

| program | `open` stands as | today | under the ruling |
|---|---|---|---|
| `(open x)`, a top-level fact | **a relation name**: the head of a written fact | loads, an auto-vivified relation `open` holding `(open x)`, R 1 state | refused |
| `(p (open x))`, finding 3's | **a relation name**: the head of a nested fact, the value `p` holds | loads, R 2 states: ∅ and `{(p x)}`. The second is from `x`, a name of the program, so H is `{(p x)}`, and has nothing to do with `open` | refused |
| `(not (open a))` | **a relation name**: the head of a stored negation | loads, R 1 state | refused |
| `(rule copy (?R) :match (?R ?x) …)` with `(copy open)` and `(open a)` | **a relation name** passed through a relation parameter | loads; the rule matches `(open a)` and derives `(q a)`, as it does with any other name in place of `open` | refused (the written `(open a)`; and an activator binding a relation parameter to `open`, read as S1f.5.18 and S1f.5.9 read relation names) |
| `:match (p (open ?x))` | **a relation name**: the head of a pattern inside a premise | refused at load: *`(open …)` is a verdict about the KB and is legal only in :assert … at None* | refused, in Ein's words (no `at None`, F23's D-2) |
| `(relation open T)` | a declared relation's name | refused at the parse, *unexpected `open`* (`DeclName` excludes it) | unchanged |
| `(p open)` | **a value**: a name `p` holds, not a relation | loads | Q-M1f.66 |

**The census.** No program under `examples/`, `tests/`, `stdlib/` or `corpus/` writes `open` anywhere but in a rule's
`:assert` and in the refusal fixtures (`examples/broken/load/reserved_open_*`, `examples/broken/open_*`). So the
refusals move no corpus answer. They move only the witnesses this stage writes.

**Why `open` as a value is a separate question.** A name is a value, and a value can reach a relation parameter.
With `(copy open)`, a rule's `(?R ?x)` reads relation `open`. If every written `(open …)` fact is refused, that
relation holds nothing, so the parameter reads an empty relation and nothing is wrong. Refusing `open` as a value
as well would make `open` a reserved word in the plain sense: written only as the verdict. That is what *`open` is
reserved word* says read strictly. Keeping it as a value would make `open` a position word like `model`.

## What it changes

- **Refused at load, every command, one line per place** (`<file>:<line>:<col>: \`open\` is the verdict a rule's
  :assert states, and is not a relation: …`): `open` as the head of a fact, a nested fact, a stored negation, a
  pattern anywhere in a rule, and as an activator's argument bound to a relation parameter. Where the grammar can
  refuse it, the grammar does (`FactHead`, the `NestedFact` head, `ArgHead` lose `open`), with a row per place in
  `00_ebnf.md`'s table. The loader takes what only the program can tell (a relation parameter).
- **By Q-M1f.66**: `open` as a value is refused too (the `Name` production loses it outside `:assert`), or stays.
- **Register rows** for each witness that loads today and is refused after, filed before the fix
  ([Q-M1f.36](../open_questions.md#q-m1f36--what-the-refusal-replaces-which-of-it-is-a-register-row)).
- **Not here**: the other position words a fact head admits (`eq`, `absent`, `false`, `true`, `model`, `step`).
  `eq` is S1f.5.7's checked conclusion, and the rest are names by Q-M1e2.17. T1f.5.23.2 lists how each behaves as
  a fact head, in its record. If one turns out to be the same case, it is noted for the phase's close and not
  fixed here.

## Defaults, as taken (2026-11-10)

Four choices the stage leaves open, each taken by default and recorded as a row of
[`open_questions.md`](../open_questions.md), marked *defaulted 2026-11-10*:

- **A macro's body writes no `open`**, called or not: refused at the parse, as `_` and a string are in a body; the
  verdict is written in the rule ([Q-M1f.86](../open_questions.md#q-m1f86--may-a-macros-body-write-open)).
- **`validate_open`'s lines**: the five the grammar covers since are deleted, as S1f.5.5's T5 deleted the lines the
  swap covered, each a row of the grammar's table; the five that stay, the projection's, start with where the
  `(open …)` is written and lose `at None`
  ([Q-M1f.87](../open_questions.md#q-m1f87--the-verdicts-loader-lines-once-the-grammar-places-open)).
- **The line**: the parse error's, ``<file>:<line>:<col>: unexpected `open` ``, with the stage's sentence as a note
  under it, as `_` and a string carry one
  ([Q-M1f.88](../open_questions.md#q-m1f88--the-line-open-is-refused-with)).
- **Which moves are register rows**: one per place with a reading of its own, twenty-four, the pin holding all
  fifty-two places ([Q-M1f.89](../open_questions.md#q-m1f89--which-of-what-the-refusal-replaces-is-a-register-row)).

## Tasks

### Task T1f.5.23.1 — the explanation (done, [above](#t1-record-2026-11-10))

### Task T1f.5.23.2 — every place, banked

One witness per row of the table and per place Q-M1f.66 adds, under `examples/ein-bugs/wrong-answers/`, each run on
every command that reads a program. The register rows, filed `open`. Every golden and fixture the refusal moves,
named.

### Task T1f.5.23.3 — the refusals

The grammar's productions and the loader's check; the rows flipped; the `at None` line rewritten in Ein's words.

### Task T1f.5.23.4 — the pages

[`01_grammar.md`](../../../docs/kernel/ir/03-ein-lang/01_grammar.md) (where `open` may be written),
[`defined_behaviour.md` § 4](../../../docs/kernel/defined_behaviour.md#4-errors-and-exit-codes), `00_ebnf.md` re-blessed, the stage's records.

## Acceptance

- `open` is accepted only as a rule's `(open)` or `(open ?R)` conclusion, and also as a value if Q-M1f.66 says so.
  Every other place is refused by every command with one line, held by the grammar's table and a test.
- Every witness that loaded before is a register row, `fixed`; no corpus answer moved.
- `./run_tests.sh` green.

## T2 record, 2026-11-10

Taken on the release binary at `f62e34e1` (`./build.sh --engine`), before any engine edit. The scripts are the
session's scratch (`census.py`, the places and every command; `sweep.sh`, the corpus; `witnesses.py` and
`witness_text.py`, the witnesses); every program a register row reads is banked under
`examples/ein-bugs/wrong-answers/`, and every place is the pin's (T3).

**Where a program can write `open`**: fifty-two places, each a program of five declarations and the place (and a
claim `ein test` checks, `(query :goal (is-a ?x Thing) :expect (model (is-a k Thing)))`, where the place is no query),
each run on the seventeen commands that read a program (`saturate`, `saturate --dump`, `solve` (describe),
`--minimal`, `--maximal`, `--single`, `--count`, `-n 1`, `oracle`, `test`, `render lattice`, `render kb`, `render
program`, `render rules`, `render rule --name r`, `render constraints`, `kb save`) and, where `kb save` wrote one, on
`saturate`, `solve`, `oracle` and `test` of the container:

| `open` stands as | places | at `f62e34e1` |
|---|---|---|
| a relation's name, in a fact | a top-level fact, `(open x)`; a nested fact, `(p (open x))` (finding 3); a stored negation, `(not (open k))`; a negation nested in a fact; a fact's `:using` premise | **loads**, every command answers (the witnesses: R 32, 32 and 4 states); `x`, a name of the program, is an object the blind enumerator proposes facts about |
| a relation's name, in a rule | a pattern inside a premise, under `not`, under `absent`, as a guard's value; inside a conclusion, an rule's or an hrule's | refused at load, *`(open …)` is a verdict about the KB and is legal only in :assert … at None* (four) and *`(open …)` is a whole conclusion, not a term inside one at None* (two): `validate_open`'s, at no position |
| a relation's name, in a query | a pattern inside a goal, `(query :goal (p (open ?x)))`; a nested fact in a claim, `(model (p (open k)))` | **loads**: the goal is answered (one class, *the goal has no row*), and the claim is read and refuted by `solve` and `oracle` (exit 1, *the claim lists 1 class, the query has 2*) while `saturate` and the views answer; `validate_open` read no query |
| | a claim's or a region claim's fact, `(model (open k))` | refused at load, the claim naming a relation nothing declares or writes, the line any undeclared relation takes |
| a relation parameter's binding | `(copy open)` beside `(open k)`; `(copy open)` alone; `(copy open)` concluded by a rule; an obligation's `(owe open)` | **loads**: `copy` reads the relation `open` and derives `(q k)` (the witness: R 2 states); reads nothing; the derived activator binds it (R 32); and `owe` owes, for every state, an `open` partner no state can hold, so R is empty, *every alive node owes* |
| the verdict's own argument | `:assert (open open)` | refused at load by the projection, *needs an `(absent …)` … holding a positive `open` premise … at None* |
| a value | a fact's argument, `(p open)`, and nested; an object, `(is-a open Thing)`; a type, `(relation t open)`, `(is-a j open)`; a premise's argument, under `absent` and `not`; a `neq`'s and an `eq`'s value; a conclusion's argument, a rule's and an hrule's; a macro call's argument, in a `:match` and an `:assert`; a goal's argument; a query claim's and a region claim's argument | **loads**, every command answers (a claim that does not hold is refuted, exit 1, the rest exit 0): `open` is a name of the program, so the blind enumerator proposes `(a open)`, `(p open)` and `(q open)` (the witnesses: R 2 to 32 states; `(p open)` and `(q open)` derived where a rule concludes them); an hrule proposes `(p open)` |
| | a claim's `(neq k open)`; a query's `:over open` | refused at load, by the claim's and `:over`'s own lines |
| a name in a list or a clause | a fact's `:rule open`; `(solve :hypothesis-relations (open))`, `:no-hypothesis`, `:hrules`; a trace step's name, its `:rule` and its `:derives` | **loads**, read as any name (H empty under `:hypothesis-relations (open)`, R {∅}) or not read at all |
| | `(import open)`; `(import std.macro :symbols (open))` | refused at load: no module `open`, and no symbol `open` the module provides |
| a macro's body | `(macro owe (?R) (open ?R))` called in an `:assert`; the same called in a `:match`; the same called by no rule; `(macro po (?v) (p open))` called | **loads**, an obligation stated through the macro (R empty, owing); refused at load after expansion (`validate_open`, *legal only in :assert*); **loads**; **loads** (R 22) |

So **thirty-eight places loaded** and each command answered them, where the ruling refuses every one (`kb save` wrote a
container of each, which `saturate`, `solve`, `oracle` and `test` answered as the file), and fourteen were refused at
load, eight by `validate_open`'s lines at no position and six by lines any name takes there. `(relation open T)`,
`(rule open …)`, `(macro open …)` and `:match (open ?R)` were refused at the parse already (the grammar's table).

**Register rows**, filed `open` by this task (Q-M1f.89, Q-M1f.36's reading), each `owner` S1f.5.23, `right` the
refusal at the parse, its `now` `ein saturate --dump`, which answered (`answered = true`), its `wrong` the record of
what every command answered at `f62e34e1`, witness under `examples/ein-bugs/wrong-answers/` with a corpus entry
(`regression`, `solve` and `saturate`), held to the unchanged engine by `wrong_answers.rs` (`cargo test -p ein-cli
--test wrong_answers`, 2026-11-10, green; the register then read 193 fixed, 37 open and 1 not wrong):

- **W70**, `open` written as a relation's name loads and is read as one, seven rows: `fact-head`, `nested-fact-head`
  (finding 3), `negation-head`, `goal-pattern-head`, `claim-fact-head`, `relation-parameter` and
  `obligation-parameter`;
- **W71**, `open` written as a value loads, a name of the program, fourteen rows: `fact-value`, `is-a-object`,
  `signature-type`, `premise-value`, `eq-value`, `conclusion-value`, `hrule-value`, `derived-activator`,
  `goal-value`, `claim-value`, `macro-argument`, `fact-clause`, `solve-list` and `trace-step`;
- **W72**, a macro's body writes `open`, three rows: `macro-verdict` (the obligation stated through a macro),
  `macro-value` and `macro-unused` (Q-M1f.86).

Twenty-four rows, the register's schema `/20` unchanged. The places that read as a row's do (a nested negation,
`:using`, a nested fact's argument, an `is-a`'s type, a value under `absent` or `not`, a `neq`'s value, a macro call
in an `:assert`, a region claim, the other two `(solve …)` lists and trace clauses, `(copy open)` alone) are no row,
and the pin holds them.

**The corpus**, `sweep.sh` (every `.ein` under `examples tests stdlib corpus plans docs`, 600 files, `ein kb save`,
exit and first line, with the binary before the fix and after): no program but the stage's own witnesses writes
`open` anywhere but a rule's whole `:assert`, so twenty-six move: the twenty-four witnesses, from exit 0 to the parse
error; `examples/broken/load/open_compound_witness.ein`, whose line starts with where its `(open ?R)` is written,
`{FILE}:10:12:`, where it ended `at None`; and `examples/broken/load/reserved_open_symbols.ein`, refused at its own
`:symbols (open)` where the module it imports was refused. No other program's load moves, so no other answer can.
A `--diff-models` file that lists `(p open)` was compared with the states (*lacks (p open)*); it is read with the
parser, so it is refused at its read since, *a model's facts do not parse*, and is no program and no row.

**The position words a fact's head admits** (*Not here*; listed, not fixed), each as a top-level fact on the same
seventeen commands, unmoved by the stage (the census re-taken after the fix):

| fact | at `f62e34e1` and since |
|---|---|
| `(eq k k)`, `(eq k j)` | checked, S1f.5.7's: the first holds and adds nothing; the second is `(false)`, root dead, R empty |
| `(false)` | a written contradiction: root dead, R empty |
| `(false k)` | **the same**: root dead, R empty, the argument read as nothing. S1f.5.24 refuses `(false ?x)` a macro's expansion concludes (*false takes no argument, not 1*) and the grammar a rule's written `(false k)`, so a written fact is the one place `false` with an argument is taken — the same case as `open`'s, noted for the phase's close |
| `(absent k)` | loads, a fact of a relation `absent` no premise can read (`PremiseHead` takes no `absent`, which is negation as failure), as `:assert (absent (p ?x))` stores one (S1f.5.24's note) — the same case, noted |
| `(true k)`, `(model k)`, `(step k)` | facts of relations a premise reads as any (`PremiseHead` takes each): names by Q-M1e2.17 |
| `(p (eq k))` | a value, the nested fact `(eq k)`, never stored or checked, as `from_ir`'s `eq_at_another_arity_is_refused_at_load` holds of `(box (eq a))` |

**Found outside the stage, recorded and not fixed** (each on the release binary at `f62e34e1` and the stage's,
programs in the session's scratch; for [P1f.7](../p1f.7_findings_outside_scope/README.md)):

1. **`(false k)` written as a fact kills root**, as above.
2. **`(absent k)` written as a fact, or concluded, stores a fact no premise can read**, as above.
3. **A `(solve …)` list naming nothing the program declares loads in silence.** `(solve :hrules (zzz))` beside a
   declared hrule `h` answers as the program without the form (two states), `(solve :hypothesis-relations (zzz))`
   leaves H empty and `:no-hypothesis (zzz)` restricts nothing, each with no word that the name matches no hrule or
   relation; a query's `:over zzz` and a claim's undeclared relation are refused at load.
4. **A fact's `:rule zzz` names no rule, in silence**: `(p k :rule zzz)` loads and `p k` is stored as derived by a
   rule the program does not declare (`ein saturate --dump`: `(p k :rule zzz)`).

**What will move**, listed before any golden was re-blessed, from the census, the sweep and the workspace's tests
run on the change unblessed (`cargo test --workspace --no-fail-fast`, 2026-11-10: nine tests failed, each on one of
these):

- `corpus/wrong_answers.toml`: the twenty-four rows `fixed`, each `now` the parse error's first line,
  ``<witness>:<line>:<col>: unexpected `open` ``, and `[case.fixed]` the null id; schema `/20` unchanged
  (`wrong_answers.rs`, and `ein-infer`'s `register_region`, which reads a refused row's witness as refused).
- *`ein.rs/crates/ein-cli/tests/golden/corpus_exits.txt`*: forty-eight cells in, each witness's `solve` and
  `saturate`, exit 1; no cell out (1 539 cells, `wc -l`, to 1 587).
- *`ein.rs/crates/ein-render/tests/golden/corpus_shapes.md5`*: the twenty-four witnesses' lines in;
  `open_compound_witness`'s refusal and its six KB views, which print it; and the four `reserved_open_*`
  programs' refusals and the renderings that print them, the note under the module's parse error and, for
  `reserved_open_symbols`, its own parse error. No other program's rendering moves, its read-out among them.
- *`ein.rs/crates/ein-ir/tests/golden/grammar_table.txt`*: the five rows whose program writes `open` where the
  grammar refuses it already (`(relation open A B)`, `(rule open …)`, `(macro open …)`, `:match … (open ?R)` and
  `(and (open ?R) …)`) gain the note, and lose a repair that reads `open` as a name where one did (`Insert `neq``,
  `Insert a variable`, which wrote `open` as a guard's or a pattern's argument); thirty-nine rows in, one per place
  (`grammar_pages`' `TABLE`, 296 rows to 335).
- *`ein.rs/crates/ein-ir/tests/golden/fuzz_seeds.txt`*: the two seeds that are `open_in_match` and
  `open_mixed_assert`, each the note in and those two repairs and the count out.
- `docs/kernel/ir/03-ein-lang/00_ebnf.md`'s phrase block: `Name`, `FactHead` and `ArgHead` lose `'open'`; the
  lexical block is unmoved (the rule stays where it was, so no token moves).
- The fixtures' `.expected`: `examples/broken/open_in_match` and `open_mixed_assert` gain the note and lose
  `repair: Insert `neq``, `repair: Insert a variable` and `and 1 more repair`, which read `open` as a name;
  `examples/broken/load/reserved_open_direct`, `_aliased` and `_qualified` gain it under the module's error;
  `reserved_open_symbols` is its own parse error, `{FILE}:7:40: unexpected `open``; `open_compound_witness` reads
  `{FILE}:10:12: rule 'bad': 2 positive `?R` premises … to branch on`, its `at None` gone.
- *Tests* that pin today's behaviour, restated: `ein-ir`'s `grammar_pages::the_lines_the_grammar_does_not_cover_are_still_reached`
  (the five `open` placements leave it for the table and `DELETED`, the parameter line joins it with its position),
  `imports_semantics::reserved_names_are_reserved_through_every_import_route` (the flat route's `:symbols (open)`
  refused at the program's own parse), `from_ir::tests::the_verdict_atom_refuses_every_shape_it_cannot_resolve`
  (`(not (open ?R))` leaves it, a parse error), and `ein-core`'s `text::tests` (`open` printed quoted as a name, and
  bare as the verdict's head).

No other golden: `help_shape.txt`, `grammar_decisions.txt`, every summary, read-out and plan-shape golden are unmoved.

## T3 record, 2026-11-10

**The grammar** (`ein-ir/src/ein.y`): `Name`, `FactHead` and `ArgHead` lose `'open'`, so `Assert`'s two productions,
`(open)` and `(open AnyHead)`, are the only ones that take the token, and `AnyHead` takes no `open` either, so
`(open open)` is refused at its second. A macro's `Template` heads a form with `AnyHead`, so a body writes no `open`
(Q-M1f.86). `ein.l`'s rule for `open` stays where it was, its comment restated, so that no token moves and the
lexical block of `00_ebnf.md` is unmoved. **The word lists** (`ein_ir::grammar`): `RESERVED_WORDS`, the words that
are never a name, gains `open`, thirteen; `POSITION_WORDS` loses it, ten; and a parse error found at an `open`
carries `OPEN_NOTE` (Q-M1f.88). The printer agrees (`ein_core::text`): `LEXER_KEYWORDS` gains `open`, so a name
`open`, which no program can write since, would print quoted, and `text::head` prints a form headed `open` bare, the
verdict's head.

**The loader** (`ein-ir/src/from_ir.rs`): `validate_open` loses the five lines the grammar covers since (`(open …)`
in a `:match`, inside a conclusion, beside one, with two arguments, with an argument neither a variable nor a name),
each named in `grammar_pages`' `DELETED` and a row of its table, and holds the rest as an invariant, an `assert!`
naming `ein.y`'s `Assert` (Q-M1f.9 (a)'s pattern): no `(open …)` stands but a rule's whole `:assert`, with one
argument at most. Its five projection lines stay, each starting with where the `(open …)` is written (a new `placed`,
the form's position, the `open` token's), their `at None` gone (Q-M1f.87): *`{FILE}:10:12: rule 'bad': 2 positive
`?R` premises each bind a witness variable — a compound witness has no single slot to branch on`*.
`refuse_malformed_primitives` loses its premise arm for `open`, which no premise is since.

**The census, re-taken on the build** (`census.py`, release build of the working tree, 2026-11-10): each of the
fifty-two places is refused on every one of the seventeen commands, exit 1 (`ein test` 2), with one first line, the
same on each, ``<file>:<line>:<col>: unexpected `open` `` at an `open`, and the note; `kb save` writes no container;
the position words of the *Not here* table answer as they did. **The corpus**, `sweep.sh` again: the twenty-six
programs T2 listed move, and no other.

**What moved**, T2's list, re-blessed with `EIN_BLESS=1` test by test, and one omission:

- `corpus/wrong_answers.toml`: the twenty-four rows `fixed`, each `now` its first line, `[case.fixed]` the null id,
  and the register's header; it reads **217 fixed, 13 open and 1 not wrong** (`grep '^status'
  corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-10).
- `corpus_exits.txt` (`cargo test -p ein-cli --test corpus_cli`): 48 lines in and none out (`git diff --numstat`,
  2026-11-10), 1 587 cells (`wc -l`), each a witness's `solve` or `saturate`, exit 1.
- `corpus_shapes.md5` (`cargo test -p ein-render --test corpus_shapes`): 55 lines in and 50 out (`git diff
  --numstat`, 2026-11-10), 15 490 lines: one line each for the twenty-four witnesses, refused at the parse; seven
  for `open_compound_witness` (its refusal and six KB views); ten, one and ten for `reserved_open_aliased`,
  `_direct` and `_qualified`; twenty out and one in for `reserved_open_symbols`, refused at its own parse since.
  **And one line each for `examples/broken/open_in_match.ein` and `open_mixed_assert.ein`**, which T2 named for
  their `.expected` and not here: the digest of their parse report, which gained the note and lost the two repairs,
  as their `.expected` did. No other program moved.
- `grammar_table.txt` (`cargo test -p ein-ir --test grammar_pages`): 264 lines in and 6 out, thirty-nine rows in,
  296 rows to 335 (`grep -c '^## '`), each refused at the parse at an `open` with the note; the five existing rows
  gained the note, two of them losing three repair lines each. `00_ebnf.md`'s phrase block, the same run: three
  productions. `fuzz_seeds.txt` (`cargo test -p ein-ir --test fuzz_properties`): 2 lines in and 6 out.
- The seven `.expected`, written from the build's output, as T2 said.

**The tests.** New: `ein-cli/tests/open_reserved.rs`, the pin: `open_is_refused_wherever_it_is_not_the_verdict`
(the fifty-two places, each on twenty commands, `--events` and `kb save --saturate` among them: one first line at an
`open`, the note under it, the same on each, no container, nothing fired),
`the_registers_witnesses_are_refused_on_every_command` (the twenty-four, the same), 
`the_verdict_loads_where_a_rule_concludes_it` (`(open)`, `(open likes)`, `(open ?R)` and an hrule's, every command
answering, and the container) and `a_container_that_stores_open_as_a_name_is_refused` (`(p open)` stored through
`ein_einb::save`, refused where it is read, at the stored program's `open`). Held both ways: with `ein.y` put back as
it was at `f62e34e1`, three of the four fail and the verdict's holds. Restated: `grammar_pages`' table (thirty-nine
rows), `DELETED` (five lines) and `the_lines_the_grammar_does_not_cover_are_still_reached` (the five placement cases
out, `(open ?S)` whose `?S` is no parameter in, with its position); `imports_semantics::reserved_names_are_reserved_through_every_import_route`;
`from_ir::tests::the_verdict_atom_refuses_every_shape_it_cannot_resolve`; `ein-core`'s `text::tests`. The two
`grammar` tests that hold the word lists to the printer's, `the_reserved_words_are_the_printers` and
`the_printers_symbol_is_the_lexers`, hold the move unchanged.

## T4 record, 2026-11-10

**The pages.**
- [`01_grammar.md`](../../../docs/kernel/ir/03-ein-lang/01_grammar.md)'s conclusion forms: `open` is written there and
  nowhere else, every place it was written until then, the note, and the projection refused at load, placed.
- [`00_ebnf.md`](../../../docs/kernel/ir/03-ein-lang/00_ebnf.md): the phrase block re-blessed; § 1's fact 1 (thirteen
  words never a name, ten position words, `open` the thirteenth and its note); § 4's *kernel's names* (where `open`
  stands is the grammar's, what the loader keeps; and `eq`'s and `neq`'s arity through a macro the loader's since
  S1f.5.24, which the bullet still gave the compiler).
- [`06_reserved_names.md`](../../../docs/kernel/ir/03-ein-lang/06_reserved_names.md): the thirteen reserved words, the
  two sets (five names in both, `relation` the one that decides it), the import routes' forty-eight cells, and § the
  verdict atom (written nowhere else; three of its refusals at the parse, two at load, placed).
- [`defined_behaviour.md`](../../../docs/kernel/defined_behaviour.md): § 1's notes, § 1.5's count (13 of 35 fixtures
  end `at None`, `grep -L 'at None' examples/broken/load/*.expected | wc -l` 22, 2026-11-10), § 4.2 (the first five at
  the parse, the last five placed, the fixtures, and `open` among the words never a name), and a new
  [§ 4.17](../../../docs/kernel/defined_behaviour.md#417-open-is-written-only-as-a-rules-verdict--m1f-s1f523): the
  places, the line and note, a container and a `--diff-models` file, what it replaced, what pins it and what is not
  refused.
- The fixture README (`examples/broken/load/`), `examples/README.md`'s catalogue (217 witnesses under
  `ein-bugs/wrong-answers/`, `ls examples/ein-bugs/wrong-answers | wc -l`, 2026-11-10, the `open-*` family
  described), the register's header and the twenty-four witnesses' headers; `corpus/README.md` holds no count that
  moved.

`utils/doc_audit.py --links --check` over the gate's scope: 405 pages, 0 findings (2026-11-10).

## Acceptance, re-taken (2026-11-10)

Each item, and what holds it:

- *`open` is accepted only as a rule's `(open)` or `(open ?R)` conclusion, and also as a value if Q-M1f.66 says so.
  Every other place is refused by every command with one line, held by the grammar's table and a test*: **met**,
  Q-M1f.66 having ruled the value refused too. `ein.y`'s `Assert` is the one production that takes `open`, a rule's
  or an hrule's whole `:assert`; `open_reserved.rs` holds the fifty-two places and the twenty-four witnesses on twenty
  commands each, one first line at an `open` and the note, and the verdict loading and answered where a rule states
  it; `grammar_pages`' table holds a row per place, refused at the parse; held both ways (T3).
- *Every witness that loaded before is a register row, `fixed`; no corpus answer moved*: **met**. W70's seven, W71's
  fourteen and W72's three, `fixed`, filed `open` first and held to the unchanged engine (T2); the places with no row
  of their own are the pin's (Q-M1f.89). No corpus program but the witnesses wrote `open` outside a rule's whole
  `:assert` (`sweep.sh`, 600 files), the two fixtures that moved are refusals whose line moved, and
  `corpus_shapes.md5` moved on no program that answers, so no read-out did. `wrong_answers.rs`, `register_region` and
  `ein-corpus`'s `the_register_is_well_formed` hold the rows.
- *`./run_tests.sh` green*: **met**, `build/gate-s1f.5.23.log` (green, 17 min 29 s, `./run_tests.sh`, 2026-11-10, exit 0: the six static checks, `cargo test --workspace`, the scratch check, the review bundle's smoke, 231 register rows read and 0 failed, and the bench smoke). The first run stopped at the link check, on this section's anchor, which T4 had cited before the section was written. The stage changes no DOT writer, bundle or CI install, so `utils/ci_container.sh` is not part of its done (Q-M1f.77).
