# S1f.5.3 — No ranges: the `RANGE` token retired

**Phase:** [P1f.5](README.md) (The no-owner items)
**Was S1e2.5.3**, in M1e2's P1e2.5, until 2026-11-08 ([§ The ids](README.md#the-ids)).
**Estimate:** 1 day.
- A quarter of a day to list every move and show the programs that change.
- Half a day to retire the token and its arms.
- A quarter of a day for the pages.

**Status:** specified 2026-11-06 on the user's ruling of that day, and revised 2026-11-07 on the
user's rule for a name
([Q-M1e2.24](../../m1e2_correctness/open_questions.md#q-m1e224--are-there-numbers-in-ein)): `1..5` stops being a range and
becomes a plain name, where this page had it refused.
**Done 2026-11-08**, on the user's go-ahead to run the phase stage by stage (*"run
plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items, each stage in separate subagent, …, ask if
decision required"*): T1f.5.3.1 `b1ac2178` (every move listed, [§ Step 1](#step-1--every-move-listed-2026-11-08)),
T1f.5.3.2 `65d1ad85` (the token retired) and T1f.5.3.3, the commit after it (the pages, and this record).
Step 1 stopped before any engine edit on its own rule, for a fifth program, and put three questions to the
coordinator, who settled them the same day as recommended, none of them the user's: the fifth program is
accepted; between this stage and S1f.5.4 the two corpus files that write a range keep their programs and are
named in one list, held both ways ([§ Between this stage and S1f.5.4](#between-this-stage-and-s1f54)); and a
`cli_semantics` test holds `:card 1..*` in place of a parse-negative fixture
([§ The parse-negative fixture](#the-parse-negative-fixture)). [§ What changed](#what-changed) is the result.
**Depends on:** nothing. It touches what [S1f.5.1](s1f.5.1_bare_underscore_refused.md)
and [S1f.5.2](s1f.5.2_strings_are_natural_language.md) touch, so the three land one at a
time. [S1f.5.4](s1f.5.4_digits_are_names.md) follows it, and [S1f.5.5](s1f.5.5_the_grammar.md) frames it.

## The case

Taken on 2026-11-06 at `916d97ac`, on the release binary.

`RANGE` (`1..5`, `0..1`, `1..*`) entered the first grammar as *UML-style cardinality*, for
`(relation has-color (House Color) :cardinality 1..1)` (`c44ccfb5`, 2026-05-18). **It was
never built.** No `.ein` has written `:cardinality` in the repository's history
(`git log -G':cardinality' -- '*.ein'` is empty), no code reads it, and `01_grammar.md` still
advertises it (lines 22 and 91).

| written | today |
|---|---|
| `(r 1..5 a)` in a fact | loads, and stores a name printed `"1..5"` |
| `(r a b :card 1..*)`, `(relation r T T :card 1..*)` | loads, and the range is dropped |
| `(relation r T T 1..* 1..1)` | refused: *malformed (relation)* |
| `(n 1..3)` in a rule's `:match` | refused at load since W20, on Q-M1e2.13 (a) |

Outside comments, four programs write one: the fixture `syntax/arg-kinds` (`(r 1..5 1..*)`,
2026-08-20), W20's witness `range-in-match`, and two probes under M1e2's plans,
`inventory_2026-11-05/probes/kwpair_in_pattern.ein` and
`p1e2.3_…/s1e2.3.8_probes/values.ein`. **A fifth** was banked after this page was written, which is why it is
not among the four: S1f.5.5's rewrite of `arg-kinds` for the final grammar,
`s1f.5.5_parser_probe/programs/arg-kinds.ein` (`49e9fed6`, 2026-11-08), which writes `(r 1..5 1..*)` and is
refused today for its `$x`. Step 1 found it, and the coordinator accepted it the same day: it moves by this
stage's mechanism alone, and it is a record under `plans/` that stays, as M1e2's two probes do.

## The ruling

The user, 2026-11-06, on being shown F6: *"we don't have native numbers in Ein and therefore
we don't have number ranges either, and never had and don't have plant to have numbers or
ranges"*.

This supersedes Q-M1e2.13's ruling on a range, which refused one in a rule only.

## What it changes

**The token goes.** Until [S1f.5.4](s1f.5.4_digits_are_names.md), `1..5` is a parse error wherever
it is written, since `1` is followed by `..5`, which is no token. From that stage on it is a plain name,
under [the user's rule](s1f.5.5_the_grammar.md#the-name-rule), and means nothing beyond its spelling:
`(p 1..5)` is a fact about the name `1..5`, as `(p a..b)` is about `a..b` today. No operation reads it as
a range, and none can.

**What only a range used.** `Node::Range` and its arms in `lex.rs` (`match_range`, `Term::Range`),
`parse.rs`, `ast.rs`, `from_ir.rs` (`Found::Range` and its load line), `compile.rs`,
`expect.rs`, `dump.rs`, `ir_dot.rs` and `dot_util.rs`, with `Terms::value_range`.

**The register.** `W20/range-in-match` is refused by this stage, with its `refused` line moved, and
then loads under S1f.5.4, a pattern over the name `1..3`; its `right` is restated there. No new case: no
program relies on a range. A dropped `:card 1..*` is not an answer, and closed keywords refuse it
([S1f.5.11](s1f.5.11_closed_keywords.md), ruled 2026-11-07).

**The fixtures.** `syntax/arg-kinds` keeps `(r 1..5 1..*)` as a fact over two names once S1f.5.4 lands,
and the node-kind probes lose that kind (`examples/README.md`). `range-in-match` stays as the refusal's witness. One
parse-negative fixture holds `:card 1..*`. The two probes under M1e2's plans are records and
stay, each exiting as its new line says. *(Step 1 changed the fixture to a test: a parse-negative fixture
would sit in the wrong group from S1f.5.4 to S1f.5.11, so `cli_semantics`' `a_range_is_no_token` holds
`:card 1..*` instead, [§ The parse-negative fixture](#the-parse-negative-fixture).)*

**The goldens and tests, named before they move** (step 1 completes this list, in
[§ The tests and goldens that move](#the-tests-and-goldens-that-move)):
`corpus_shapes.md5`'s rows for `arg-kinds` and `range-in-match`; `grammar_decisions.txt` and
its test; `lex.rs`' unit tests (lines 560 to 562); `cli_semantics.rs`'s refusal test.

**What it closes.** F6, and the range half of G-6.

## Defaults

1. **Withdrawn 2026-11-07.** This page had a digit-first name refuse `..`, so that `1..5` was never a
   name. The user's rule has no such exception, and an exception would be a range reading kept alive
   only to be refused.
2. **Between the two stages the refusal is the plain parse error.** `1..5` is two bad tokens and no longer
   a concept, so it needs no line of its own.

## The method

### Task T1f.5.3.1 — every move listed

Prototype on a `git archive` snapshot under `build/`. Sweep every `.ein` under `examples/`,
`tests/`, `stdlib/`, `corpus/` and `plans/` through `solve` and `saturate`, against today's
binary, run `cargo test --workspace` on the snapshot, and list every move. **Stops if** any
program outside the four above moves. Show the user `arg-kinds` as written and after.

### Task T1f.5.3.2 — retired

The token, its arms, the fixtures and the register row.

### Task T1f.5.3.3 — the pages

`00_ebnf.md` (`RANGE`, fact 3, the terminal census), `01_grammar.md` (the `RANGE` row, and
`:cardinality` at lines 22 and 91), `02_patterns.md`, `04_dot_rendering.md`,
`defined_behaviour.md` § 4.8's range half, `examples/README.md`. In `predicates.rs`, line 5's
*numeric / set / cardinality / aggregation primitives are deferred to followups* becomes what
the ruling says. Q-M1e2.13's entry says what superseded it. `docs/history/` is a record, and
does not change.

## Acceptance

- `1..5` and `1..*` have no reading as a range anywhere: refused by this stage, and plain names after
  S1f.5.4, and a test holds each.
- `Node::Range` and `Terms::value_range` are gone, and no page describes a range or
  `:cardinality` as part of the language.
- No corpus program outside step 1's list moved.
- `./run_tests.sh` is green.

## Step 1 — every move listed (2026-11-08)

T1f.5.3.1, at `fd800fe0`. The prototype is the parser with no range reading and nothing else: `parse.rs`'
`leaf_value` no longer asks for `Term::Range`, on a `git archive` snapshot under `build/s1f.5.3/snapshot`
(gitignored), release build. The sweep, `build/s1f.5.3/sweep.sh`: every `.ein` under `examples/`, `tests/`,
`stdlib/`, `corpus/` and `plans/`, 511 files (`find examples tests stdlib corpus plans -name '*.ein' | wc
-l`, 2026-11-08), each through `ein solve FILE` and `ein saturate FILE --dump` on today's binary and the
prototype, stdout and stderr compared with the timing lines removed, 37 s. `1..5` is the `INT` `1` and then
`..5`, which no terminal matches, so the parse stops at the first `.`, as Default 2 says.

### Five programs move, and 506 answer as they did

| program | today | the prototype |
|---|---|---|
| `examples/syntax/arg-kinds.ein` | loads: `solve` exit 0, R decided as the whole cube of 41 candidates, k_s a lower bound (112 793 states) past `--region-budget`; `saturate` exit 0, six facts | refused at the parse, `14:5`; both exit 1 |
| `examples/ein-bugs/wrong-answers/range-in-match.ein` | refused at load, W20's line (*kb load error: rule 'r': its :match writes 1..3 as an argument of n, …*), exit 1 | refused at the parse, `31:23`, exit 1 |
| `plans/m1e2_correctness/inventory_2026-11-05/probes/kwpair_in_pattern.ein` | loads, R {∅}, exit 0 | refused at the parse, `4:33` (`:where 1..3`), exit 1 |
| `plans/m1e2_correctness/p1e2.3_…/s1e2.3.8_probes/values.ein` | loads, R 8 states, exit 0 | refused at the parse, `30:18` (`(given 1 "1" ?v 1..5 …)`), exit 1 |
| **`s1f.5.5_parser_probe/programs/arg-kinds.ein`** (this phase's directory) | refused at the parse at `$x`, `12:4`, exit 1 | refused at the parse at `1..5`, `11:5`, exit 1 |

The first four are [§ The case](#the-case)'s. **The fifth is not**, which is the stop: it is S1f.5.5's
rewrite of `arg-kinds` for the final grammar ([`s1f.5.5_paper_grammar.md`](s1f.5.5_paper_grammar.md),
`49e9fed6`, 2026-11-08), banked after this page listed four. It writes `(r 1..5 1..*)` on line 11, so it
moves by this stage's mechanism and no other: refused both ways, exit 1 both ways, only the position of the
first refusal moves, from `$x` (which today's parser cannot read either) to `1..5`. It is a probe of a
grammar not yet built, a record under `plans/` as M1e2's two probes are, and like them it stays, exiting as
its new line says. **Recommended:** accept it as the fifth of the list.

### `arg-kinds`, as written and after

```lisp
(relation r T T)
(r 7 "a string")
(r 1..5 1..*)
(r _ ?x)
(r (nested a b) c)
```

| run | today (`fd800fe0`) | after this stage |
|---|---|---|
| `ein saturate FILE --dump` | exit 0; stores `(r 7 "a string")`, `(r "1..5" "1..*")`, `(r "?x")`, `(r (nested a b) c)` and the declaration's two facts | exit 1: `examples/syntax/arg-kinds.ein:14:5: unexpected input`, under it `(r 1..5 1..*)` and a caret at the first `.` |
| `ein solve FILE` | exit 0; `states (k_s) 112 793 (a lower bound: complete up to states of 3 candidates)`, `commitments (k_c) 2 199 023 255 552`, ∅ and ⊤ both in R, so R is the whole cube of 41 candidates; `note: … partial — the state route stopped before the states of 4 candidates at --region-budget 1048576` | exit 1, the same line |
| `ein render rules FILE` | exit 1, *no rule forms* | exit 1, the same line |
| `ein test FILE` | exit 2, *nothing to check* | exit 2, the same line |

From [S1f.5.4](s1f.5.4_digits_are_names.md) it loads again as written, `(r 1..5 1..*)` a fact over two names.

### The tests and goldens that move

`cargo test --workspace --no-fail-fast` on the snapshot (`build/s1f.5.3/proto-test.log`, 2026-11-08):
sixteen tests in thirteen targets move, each by the stage's mechanism, and no other.

| test | what moves |
|---|---|
| `ein-cli` `corpus_cli::every_cell_reproduces_its_exit_code` | `corpus_exits.txt`, 1 of 1 302 cells: `arg-kinds :: saturate` 0 → 1. `range-in-match`'s two cells stay 1 |
| `ein-render` `corpus_shapes` | `corpus_shapes.md5`, 61 lines out and 2 in: `arg-kinds`' 41 rows and `range-in-match`'s 20 become one `ir[parse]` row each, the parse error (`diff` against the golden blessed on the snapshot, run with its target inside it) |
| `ein-ir` `grammar_decisions` | `grammar_decisions.txt`: `(a 1..5)` and `(a 1..*)`, `ok (1 form(s))` → `<string>:1:5: unexpected input`; `(a 1..)` was refused and stays |
| `ein-ir` `fuzz_properties` | `fuzz_seeds.txt`: the seed `(a 1..5 :k 1..* …)`, `ok` → refused at `1:5` |
| `ein-ir` `load_semantics::the_config_coercion_table_is_stable` | `config_coercion.txt`: `(config :lattice-sanity-check 1..5)`, the loader's *got Range(low=1, high=5)* → `<IRParseError> <string>:1:32` |
| `ein-ir` `ir_semantics`, two | the round-trip table's `(a 1..5)` and `(a 1..*)` stop parsing; the dump-then-parse shapes lose `:cardinality 0..0`, `0..*` and `9999..*` |
| `ein-ir` `from_ir` unit tests, two | `a_range_or_a_wildcard_relation_in_a_rule_is_refused_at_load`'s eight range cases and its joined case, and `a_rule_shape_that_reads_nothing_is_refused_at_load`'s `(p ?x :where 1..3)` |
| `ein-cli` `cli_semantics::a_range_or_a_wildcard_relation_in_a_rule_is_refused_at_load` | its two range cases: the loader's line → the parser's |
| `ein-cli` `wrong_answers` | `W20/range-in-match`'s `refused`: the loader's line → `…/range-in-match.ein:31:23: unexpected input` |
| `ein-infer` `register_region` | it reads a refused row's witness as a load error, and a parse error panics |
| `ein-cli` `corpus_cli::every_positive_entry_answers_under_at_least_one_run` | `arg-kinds` answers under no run |
| `ein-cli` `answer_round_trip::the_witnesses_reload_to_their_states` | `arg-kinds` writes no state |
| `ein-infer` `alive_invariant::every_name_of_the_program_is_a_candidate_object` | `SEEDED`'s two `arg-kinds` lines, `a` and `b` in `(r (nested a b) c)` |
| `ein-ir` `dump_goldens::dump_then_parse_is_a_fixed_point` | every corpus file outside `examples/broken/` parses, and these two would be the first that do not |

Two more failed on the snapshot and are the snapshot's, not the prototype's: `ein-ir`'s `imports_semantics`
(two tests) and `corpus_shapes` resolve the stdlib by walking up from the test binary, and with the target
under `build/` that found the checkout's `stdlib/`; with the target inside the snapshot the first two pass
and the third moves as the table says. Retiring the rest (`Term::Range`, `match_range`, `Node::Range`,
`Terms::value_range` and their arms) moves no answer, only the unit tests that name them: `lex.rs`'
`range_is_digit_anchored_and_beats_int` and `terms.rs`' value test. `dot_wellformed` passes: a file that does
not parse draws no view. `grep -rn 'Term::Range\|Node::Range\|value_range' ein.rs/crates` names every
site [§ What it changes](#what-it-changes) lists and no other.

### Between this stage and S1f.5.4

This page has `arg-kinds` keep its program and be refused, by the plain parse error, until S1f.5.4 makes
`1..5` a name; `range-in-match` likewise, until S1f.5.4 loads it as a pattern over the name `1..3`. Its
golden list foresaw `corpus_shapes`' rows. It did not foresee the last five rows of the table above: tests
that read the two files as files that parse, load or answer. `arg-kinds` is `positive` because a group is a
directory (`ein-corpus`'s `negatives_are_grouped_by_where_they_fail`), its one answering cell is
`saturate` (`render rules` exits 1, *no rule forms*), and it is 112 793 of the round trip's 112 836 state
files (`ein solve FILE --dump-answer DIR` on each witness, 2026-11-08); the other four witnesses keep the
test's floor of 40. Three ways through:

- **(a) Recommended: both programs as written, one named list, held both ways.** `ein_corpus::
  REFUSED_UNTIL_S1F_5_4` names the two files; `every_positive_entry_answers…`, the round trip, `SEEDED` and
  the dump fixed point read it by name, never by a silent skip, the round trip asserting that a listed
  witness writes no state; and one new `corpus_cli` test holds the list both ways: every run of each file is
  the parser's line, exit 1 (`ein test` 2), so the commit of S1f.5.4 that makes them load fails it and
  empties the list. That is S1f.5.0's `KNOWN_R1` pattern (`alive_invariant.rs`, W34, which S1f.5.9 empties).
  `register_region` reads a parse refusal as the binary prints it, which is permanent and not the
  interim's. S1f.5.4's page gains one line: empty the list. Nothing here edits a program. **Prototyped**
  with the whole retirement on a second snapshot, `build/s1f.5.3/full`: with the five goldens above
  re-banked and the register's line moved, `cargo test --workspace` passes, 921 tests and none failed
  (`build/s1f.5.3/full-test.log`, 2026-11-08), and `cargo fmt --check`, `clippy -D warnings` and `cargo doc
  -D warnings` are clean. The gate runs at T2.
- **(b) Edit `arg-kinds` for the interim**, dropping `(r 1..5 1..*)` until S1f.5.4 puts it back. Three of
  the five tests stay as they are, but it edits a fixture's program, which this page does not license, and
  moves `arg-kinds`' H and R twice; `range-in-match` still needs (a)'s dump-point and `register_region`
  changes.
- **(c) No interim**: land S1f.5.4's reading of `1..5` as one name here, or S1f.5.3 and S1f.5.4 as one
  commit. S1f.5.4 stops at its step 1 to show the user real programs, and the phase's order lands each so
  that every move is attributed to its ruling, so this is the user's.

**Settled 2026-11-08 by the coordinator: (a).** The page licenses it: both programs stay as written, its
Default 2 makes the refusal the plain parse error between the stages, and the name `1..5` stays writable as
the string `"1..5"` meanwhile, so the phase's *no name is ever left unwritable* holds. Built by T1f.5.3.2,
with [S1f.5.4](s1f.5.4_digits_are_names.md)'s T2 told to empty the list and delete the interim test.

### The parse-negative fixture

[§ What it changes](#what-it-changes) has *one parse-negative fixture hold `:card 1..*`*. A fixture's group is
its directory, and `(relation r T T :card 1..*)` is a parse error only until S1f.5.4: from S1f.5.4 `1..*` is a
name and the form loads with `:card` dropped in silence, from [S1f.5.11](s1f.5.11_closed_keywords.md) the
loader refuses `:card`, and with [S1f.5.5](s1f.5.5_the_grammar.md)'s grammar the parser does. In
`examples/broken/` it would be wrong-grouped from S1f.5.4 to S1f.5.11, and `grammar_decisions`'
`the_parse_negative_fixtures_reproduce_their_message` counts four fixtures. **Recommended:** hold `:card 1..*`,
as a fact's keyword value and as a declaration's, in the `cli_semantics` test the acceptance asks for, with a
range in a fact, in a rule's `:match` and `:assert`, and in a `(config …)`, each the parser's line from
`saturate`, `solve`, `oracle` and `test`; S1f.5.4 and S1f.5.11 each restate their rows. The alternative is the
fixture, which S1f.5.4's step 1 then has to move.

**Settled 2026-11-08 by the coordinator: the test.** The `examples/broken/` fixture was not banked, since it
would sit in the wrong group from S1f.5.4 to S1f.5.11; `a_range_is_no_token` holds `:card 1..*` with the other
places, and S1f.5.4's T2 turns it into its plain-name counterpart, the acceptance's *a test holds each*.

## What changed

**The token** (T1f.5.3.2, `65d1ad85`). No terminal reads a range: `Term::Range` and `match_range` in
`lex.rs`, the `RANGE` branch of `parse.rs`' `leaf_value`, `Node::Range` and its arms (`ast.rs`; `from_ir.rs`'
interning walk, `atomic_value` and `refuse_no_fact_holds`' `Found::Range` with its line; `compile.rs`' guard
key, whose tag 6 stays unused; `expect.rs`, `dump.rs`, `ir_dot.rs`, `dot_util.rs`) and `Terms::value_range`
are gone (`grep -rn 'Term::Range\|Node::Range\|value_range' ein.rs/crates` is empty, 2026-11-08). `1..5` is
the `INT` `1` and then `..5`, and the parse stops at the `.`, from every command that reads the program, exit
1 (`ein test` 2). The editor grammar lost its `RANGE` rule (`utils/vscode-ein/`).

**The tests.** `lex.rs`' `no_terminal_reads_a_range`; `from_ir`'s and `cli_semantics`'
`a_wildcard_relation_in_a_rule_is_refused_at_load`, the range cases gone; `cli_semantics`' new
`a_range_is_no_token`, a range in a fact, a fact's and a declaration's `:card`, a rule's `:match` and
`:assert` and a `(config …)`, each the parser's line from `saturate`, `solve`, `oracle` and `test`;
`ir_semantics`' two tables. The interim: `ein_corpus::REFUSED_UNTIL_S1F_5_4` and `corpus_cli`'s
`the_files_refused_until_digits_are_names_are_refused`, which holds it both ways, read by
`every_positive_entry_answers…`, the round trip's witnesses, `alive_invariant`'s `SEEDED` and the dump fixed
point. `register_region` reads a refusal at the parse as the binary prints it.

**The register.** `W20/range-in-match` stays fixed, its `now` the parser's line at `32:23` (its header is one
line longer), its note and the header's paragraph saying why; 103 fixed, 33 open and 1 not-wrong (`grep
'^status' corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-08). No other row moved.

**The goldens**, each named at step 1 before it moved: `corpus_exits.txt` (1 of 1 302 cells), `corpus_shapes.md5`
(61 lines out and 2 in), `grammar_decisions.txt`, `fuzz_seeds.txt` and `config_coercion.txt`. No answer
outside step 1's five programs moved. `./run_tests.sh` green at T1f.5.3.2, 13 min 24 s
(`build/gate-s1f.5.3-T2.log`), and at T1f.5.3.3, which touches `predicates.rs`' module line, 13 min 24 s
(`build/gate-s1f.5.3-T3.log`); and `ein-render`'s `dot_wellformed` green at `65d1ad85` in a container built
like GitHub's runner (`utils/ci_container.sh --tests-only -p ein-render --test dot_wellformed`: Ubuntu
26.04.1, Graphviz 14.1.2 with `libpathplan4` amd64v3, 3 tests in 77 s, 2026-11-08), since `ir_dot.rs` and
`dot_util.rs` changed.

**The pages** (T1f.5.3.3). `00_ebnf.md` (`RANGE` out of § 1 and the `Value` production; fact 3 says there
is none, numbered as before so that fact 4's citations hold; the terminal census), `01_grammar.md` (the
`RANGE` row replaced by a paragraph after the table; `:cardinality` out of the `KEYWORD` row's examples and
the relation declarator), `02_patterns.md` (a guard's refusals; the registry's sentence on numbers),
`04_dot_rendering.md` (the ontology sketch's `(1..1)`), `defined_behaviour.md` (§ 4.8 is the wildcard
relation, with the range's history; § 4's *conclusion no fact can hold* bullet), `examples/README.md`,
`predicates.rs`' module line (what the ruling says: no numeric, cardinality or aggregation primitive, and
none planned), [Q-M1e2.13](../../m1e2_correctness/open_questions.md#q-m1e213--what-does-a-wildcard-or-a-range-in-a-rules-pattern-mean)
and [Q-M1e2.24](../../m1e2_correctness/open_questions.md#q-m1e224--are-there-numbers-in-ein) (built),
[S1f.5.11](s1f.5.11_closed_keywords.md)'s rows this stage touched, the phase README's rows, and `AGENTS.md`.
`docs/history/` is a record and did not change.
