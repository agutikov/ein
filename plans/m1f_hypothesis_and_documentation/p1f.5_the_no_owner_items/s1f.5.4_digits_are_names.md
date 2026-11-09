# S1f.5.4 — Digits are plain names

**Phase:** [P1f.5](README.md) (The no-owner items)
**Was S1e2.5.4**, in M1e2's P1e2.5, until 2026-11-08 ([§ The ids](README.md#the-ids)).
**Estimate:** 1.25 days, re-taken on 2026-11-08 on the user's re-plan ([P1f.5 § The order](README.md#the-order));
it was 1.5.
- Half a day to re-take the moves on a prototype and show real programs. Done (T1f.5.4.1).
- Half a day to retire the int kind, with the `:priority` guard it needs, after the swap (T1f.5.4.3).
- A quarter of a day for the pages, after the swap (T1f.5.4.4).
- The lexer, and the register and the fixtures, a quarter of a day each in the old estimate, are
  [S1f.5.5](s1f.5.5_the_grammar.md)'s swap since the re-plan (T1f.5.5.4), and the review of this stage's moves is
  its stop (T1f.5.5.9). The int kind's retirement grows by the quarter its `:priority` guard takes.

**Status:** specified 2026-11-06 on the user's ruling of that day, and extended 2026-11-07 on the
user's rulings on F8 and on what a name is
([Q-M1e2.24](../../m1e2_correctness/open_questions.md#q-m1e224--are-there-numbers-in-ein)).
**Started 2026-11-08**, on the user's go-ahead to run the phase stage by stage (*"run
plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items, each stage in separate subagent, …, ask if
decision required"*). **Step 1 done** (T1f.5.4.1, [§ Step 1](#step-1--every-move-listed-2026-11-08)), and the
stage **stopped there, as its method says, to show the user real programs before any edit**: one finding (two
synthetic names become spellable) and four questions. No engine code, test, golden or fixture has moved.
**The four questions were decided on 2026-11-08 by the coordinator**, each as this step recommended, and the user
may still overrule any of them at S1f.5.5's review stop (T1f.5.5.9): [Q1](#q1--the-synthetic-names) (a),
[Q2](#q2--relation_parameter_int) (a), [Q3](#q3--arg-kinds-and-the-round-trip) (a) and
[Q4](#q4--a-name-of-digits-in-summaryjson) (a). C1 and C2 are the attribution's rules, and C3 is parked
([§ Notes](#notes)).
**Re-planned on 2026-11-08, on the user's ruling** ([P1f.5 § The order](README.md#the-order)): *"It is the same
work done twice, why not switch to grmtools first and remove hand-written parser and then verify requirement met
(strings, names, etc...) ?"* The name rule is built once, in S1f.5.5's generated parser, and lands at its swap
(T1f.5.5.4), credited to this stage's ruling; this step's 46 programs are the first cross-check of the
attribution that precedes the swap (T1f.5.5.9). T1f.5.4.2 moved there. What is left, T1f.5.4.3 and T1f.5.4.4,
runs after the swap, on the generated parser. Nothing edits the hand-written lexer or parser for this stage.
**A dated note, 2026-11-08, on the user's ruling *"S1f.5.15 before swap"*** ([P1f.5 § The order](README.md#the-order)):
S1f.5.5's T3 built the grammar with no `:priority` and no `b::count` (`54715b5f`), and
[S1f.5.15](s1f.5.15_rule_priority.md) removes the keyword from every program before the swap. So the `:priority`
guard this page plans (§ What it changes, step 1's tables, T1f.5.4.3 and the acceptance's second item) has nothing
to guard: at the swap no program writes `:priority`, and retiring the int kind moves no rule order. If S1f.5.15
ends in a written refusal, the production comes back, and the guard with it. The sentences below are left as the
plan stood.
**Run after the swap on 2026-11-09**, on the coordinator's brief of that day and the user's instruction *"just
continue with default decisions till the end of the phase"*, in the order T1f.5.4.3, T1f.5.4.5, T1f.5.4.4, one
commit each. **T1f.5.4.3 done 2026-11-09** (`410a3388`, [its record](#t1f543-record-2026-11-09)): the int kind is gone, each
path shown unreachable first (30 probes, the whole suite and 240 000 mutants, none reached by a program), no answer
or register row moved, and `.einb`'s `INTS` section kept empty with no format bump
([Q-M1f.10](../open_questions.md#q-m1f10--the-einb-ints-section-once-the-int-kind-is-gone), defaulted).
**T1f.5.4.5 done 2026-11-09** (`eccf4186`, [its record](#t1f545-record-2026-11-09)): W37 fixed, a template reference reading the
name rule's characters up to the first `}`, and a template naming a variable no reference can reach refused at load
([Q-M1f.11](../open_questions.md#q-m1f11--a-template-reference-to-a-variable-no-reference-can-name), defaulted); the
register reads 118 fixed, 21 open and 1 not wrong. **T1f.5.4.4 done 2026-11-09** ([its
record](#t1f544-record-2026-11-09)): no page describes an `INT` token or an int value as live, and § 4.11 states the
new refusal. **S1f.5.4 is done**: every item of [§ Acceptance](#acceptance) is met (that section).
**Depends on:** [S1f.5.3](s1f.5.3_range_retired.md), done, so that `1..5` was gone first. Its remaining tasks
run after [S1f.5.5](s1f.5.5_the_grammar.md)'s swap.

## The ruling

The user, 2026-11-06, on F7: *"YES, digits are not numbers, digits are plain names, and refuse
?x in facts; (p 1) and (p 01) are VERY DIFFERENT facts, (p "1") has to be refused in
plans/m1e2_correctness/p1e2.5_the_no_owner_items/s1e2.5.2_strings_are_natural_language.md"*.

The user, 2026-11-07, on F8 and on what a name is: *"yes, `0`, `1` - any alphanumeric plus connectors
signs (`_`,`-`) plus decorators signs (`$`, `%`, `^`, `*`, ...) string without space considered a NAME,
except variables that marked with `?` that sits in front of the name"*. The rule itself, and what the
agent read it to mean, are in [S1f.5.5](s1f.5.5_the_grammar.md#the-name-rule).

## The case

Taken on 2026-11-06 at `916d97ac`, on the release binary.

No operation reads a value as a quantity: the built-in predicates are `eq` and `neq`. But the
language has `INT ::= '-'? [0-9]+`, a value kind (`Tag::Int`, `IntId`, a pool and a `.einb`
section), and a numeric reading: `Terms::cmp_value` sorts by value, and `(p 1) (p 01)` is one
fact. Programs write a digit as `:priority 110` (475, in 136 files) and as a plain value (217, in
34 files; the agent's scan), a label in every case.

| written | today | after |
|---|---|---|
| `(p 1) (p 01)` | **one fact**, `(p 1)` | **two facts** (the ruling) |
| `(p 1st)` | `(p 1 st)`, split in silence (F2) | the name `1st` |
| `1_a`, `1-2`, `1.5` | refused | a name |
| `(1 a)` | refused | a fact of a relation named `1`, as any name |
| `-7` | an int | **a name**, so `(p -7)` and `(p 7)` are two facts; only `ein-bugs/int-goal-binding.ein` writes one |
| `1..5` | a range | a name, with no range reading ([S1f.5.3](s1f.5.3_range_retired.md)) |
| `(functional R 0 1)` | ints | names, and `std.elim`'s text does not change |
| `:priority 110` | a keyword's value | unchanged, and **not a name** (below) |

## What moves

**Measured without a prototype** (2026-11-06): every program that writes a digit or imports
`std.elim`, with each plain digit `1` renamed `d1` in a throwaway copy, including the stdlib's,
and `ein solve --json-summary` compared on today's binary. A digit is no candidate today and a
name is one (Q-M1e2.9 (a)), so the blind enumerator's universe grows. 36 programs: 17 answer as
they do, and 19 move.

| programs | today → with digits as names |
|---|---|
| `features/05_stdlib_domain_elim` | k_s 32, k_c 32 unmoved; **N 21 → 47** |
| `branching/12_typed_blind_solve` | k_s 3, k_c 7 unmoved; **N 25 → 51** |
| the `(edge 1 2)` family: `binding-key-*`, `activator-text-*` and four `corpus/fuzz_findings/` | k_s 1 or 2 → past 56 000, stopped at the budget; one exits 1 |
| `str-in-assert`, `str-in-match`, `classes-int-name` | k_s doubles or more; these go with [S1f.5.2](s1f.5.2_strings_are_natural_language.md) |
| `syntax/arg-kinds` | stopped either way, with larger counts |
| `broken/load/relation_parameter_int` | refused → loads, so the fixture's point is gone |

The two real programs keep R, and the extra members of N are dead sets over the new candidates.
**A label is a candidate like any other name: the user ruled F8 on 2026-11-07, as recommended**, so these
moves are accepted. A signature that names its types keeps a label out of H.
The `(edge 1 2)` family are W7's and W16's witnesses, whose point is a value kind that no longer
exists. Step 1 says which retire and which are kept, scoped with `:no-hypothesis`.

## What it changes

**The lexer.** `SYMBOL` becomes [the name rule](s1f.5.5_the_grammar.md#the-name-rule): a run of name
characters up to a separator, any of them first. `INT` goes as a value, and `EQ` with it, since `=` is a
name. This was to land on the hand-written parser; since the re-plan of 2026-11-08 the rule is the generated
parser's, and lands with most of F2 at S1f.5.5's swap.

**`:priority` is the trap.** This page said until step 1 that the trap was `invariant.rs`' name walk (line 501).
Step 1 found it elsewhere ([§ `:priority`](#priority-h-unmoved-and-where-the-trap-is)): the walk reads a rule's
`:match` and `:assert` alone, and **`from_ir.rs`' `priority` match** reads `Node::Int` only and drops any other
value in silence, so a `:priority 110` lexed as a name would lose every rule's order, moving 163 files' rule
lists and no answer. At the swap the generated parser's action builds today's `Node::Int` for `:priority
DIGITS`, so nothing moves; when T1f.5.4.3 retires the int kind, the loader reads the digits as a count and stores
no atom, which is the guard. **Step 1 stopped if H moved on any program that writes no plain digit**: it moved on
one, for the loader's internal names and not for `:priority`
([§ H moved](#h-moved-on-a-program-that-writes-no-plain-digit-the-finding)). `:priority` itself is [S1f.5.15](s1f.5.15_rule_priority.md)'s to delete (the user,
2026-11-07), so this stage keeps it working and gives it nothing new: its value is a run of digits, as every
corpus use is. `ein_core::parse_int` is for CLI options, and stays.

**What only an int used.** `Tag::Int`, `IntId`, the pool and `cmp_value` in `value.rs` and
`terms.rs`; `Node::Int` in `parse.rs`, `ast.rs`, `from_ir.rs`, `expect.rs`, `compile.rs`, `dump.rs`,
`ir_dot.rs`, `dot_util.rs`, `read_out.rs` and `trace/ast.rs`; the arms in `saturator.rs`,
`obligations.rs`, `region.rs`, `invariant.rs`, `detector.rs` and `saturate.rs`; and `.einb`'s int
section, with its format version bumped (`einb/1.2`).

**Order.** Names sort by interned rank, so `(edge 10 …)` may print before `(edge 2 …)`. Goldens on
digit programs move, and step 1 lists them.

**The register.** W7's, W16's and W28's rows are about an int beside another value, and each `right`
is restated or retired on step 1's list. The fixes behind W16 and W28 compare by value, which is
still right. Since the re-plan the rows move in S1f.5.5's swap, where S1f.5.2's ruling lands too, so a witness
that also writes a string is refused for it ([S1f.5.5 § T4](s1f.5.5_the_grammar.md#task-t1f554--the-swap)).

**What it closes.** F7, F8, and F2: `a?x`, `?x?y` and `"a"b` are refused once a token runs to a
separator. S-15 shrinks: `(p 01)` against a stored `(p 1)` is
now a true difference, and a whitespace difference is what is left of it (`(p  a)` gets *0 matched*
today), which [S1f.5.5](s1f.5.5_the_grammar.md#task-t1f556----diff-models-compares-by-value-s-15) fixes.

## The method

### Task T1f.5.4.1 — every move listed, real programs shown

Prototype on a `git archive` snapshot under `build/`, on top of S1f.5.3. Sweep every `.ein` under
`examples/`, `tests/`, `stdlib/`, `corpus/` and `plans/`, run `cargo test --workspace`, and list
every move against the table above. **Stop and show the user real programs before any edit** (the
standing rule on syntax): `features/05`, `std.elim`, `zebra2.ein` unmoved, and `(p 1) (p 01)`.

### Task T1f.5.4.2 — digits are names

**Moved to S1f.5.5 on 2026-11-08**, on the user's re-plan: the lexer is the generated parser's name rule, and the
fixtures, the register, the printer, the JSON writer and the interim below land in its swap
([T1f.5.5.4](s1f.5.5_the_grammar.md#task-t1f554--the-swap)), the loader's internal names before it
([T1f.5.5.8](s1f.5.5_the_grammar.md#task-t1f558--the-swaps-prerequisite-the-loaders-names-out-of-the-name-space)).
The `:priority` guard is T1f.5.4.3's. What follows is the task as it stood, and what the swap now carries.

The lexer, the `:priority` guard, the fixtures and the register.

**What [S1f.5.3](s1f.5.3_range_retired.md) left for this task** (2026-11-08, [§ Between this stage and
S1f.5.4](s1f.5.3_range_retired.md#between-this-stage-and-s1f54)):

- **Empty `ein_corpus::REFUSED_UNTIL_S1F_5_4`** and delete `corpus_cli`'s
  `the_files_refused_until_digits_are_names_are_refused`, which holds the list both ways and fails in this
  commit, once `syntax/arg-kinds` and `wrong-answers/range-in-match` parse. With the list empty, the round
  trip's witnesses, `alive_invariant`'s `SEEDED`, the dump fixed point and `every_positive_entry_answers…`
  read both files again with no other edit; `W20/range-in-match` loads, a pattern over the name `1..3`, and
  its `right` is restated.
- **`cli_semantics`' `a_range_is_no_token` becomes its plain-name counterpart**: `1..5`, `1..*` and `2..*`
  in a fact, a keyword's value, a declaration, a rule's `:match` and `:assert` and a `(config …)` each read
  as the name it spells, which is S1f.5.3's acceptance, *a test holds each*. `:card 1..*` then loads with
  `:card` dropped until [S1f.5.11](s1f.5.11_closed_keywords.md) refuses the keyword, and the row says so.

### Task T1f.5.4.3 — what only an int used, after the swap

On the generated parser, after S1f.5.5's T5. `Tag::Int`, `IntId`, the pool, `cmp_value`, `Node::Int` and
`.einb`'s int section ([§ What it changes](#what-it-changes)), retired as a separate commit, so that no answer
moves in it: after the swap no program yields an int but `:priority`'s count. **The `:priority` guard lands
here**: the grammar's `b::count` and `from_ir.rs`' `priority` match read the digits as a count with no int value
behind it (`Rule::priority` needs a count that is not a value), so `:priority 110` still orders the rule and puts
no name in the program, and every rule list `saturate --dump` prints is unmoved.

**Named before it moves, 2026-11-09** (the brief's rule: a golden moves only where this page names it first).
Two test files move with the kind, and nothing a program answers:
- `ein-core/tests/golden/repr_values.txt`, the `repr` format's table of every value shape reachable from a fact
  argument: its eight rows that held a `ReprValue::Int` (`0`, `7`, `-7`, the two wide literals, `(-7,)`, `('a', 2,
  "it's")` and `Fact(… args=(1, …))`) print the same digits as names, quoted (`'7'`, `('-7',)`), since
  `ReprValue::Int` goes and a run of digits is a `str`. The row count, 37, is unmoved.
- `ein-ir/tests/golden/grammar_table.txt` grows by this stage's rows, appended after the 123 T5 banked so that no
  row renumbers: a token that is no name (`a?x`, `?x?y`, `a:b`, `a,b`, `a#b`, `a|b`, `a\b`, `"a"b`, `Жук`) in each
  place a name goes, refused at the parse (§ The method's rows, P1f.5's order item 8).

### T1f.5.4.3 record, 2026-11-09

**One commit, on `edb096af`** (S1f.5.5's T6). The int kind is gone: `Value` has two shapes, a name and a nested
fact, and nothing in the engine reads a run of digits as a number. **No answer moved**, and no register row; the two
test goldens named above moved as named (below, *Nothing moved*).

**How unreachable was shown.** A copy of `edb096af` (`git archive`, `build/s1f.5.4/t3/probe/`) with a panic,
`T3-PROBE <id>`, at each of 30 sites, every path only an int used (`build/s1f.5.4/t3/probe_patch.py` places them by
their text): the roots every int came from (`Value::int`, `Value::tag`'s int arm, `IntPool::intern` and `get`,
`Terms::value_int`, `intern_int` and `int_text`, `.einb`'s `Maps::value` int arm and its `INTS` entries) and every
arm that read one (`cmp_semantic`, `repr_value`, `display`, `tag_order`; `Node::Int` in `eq_nodes`, `node_repr`,
`dump`, `expect::render`, `intern_program_names`, `atomic_value`, the relation-parameter line, the compiler's slot,
guard, guard key and integer line, `read_out`, `dot_util`, `ir_dot`; `Tag::Int` in `invariant`, `region`, the trace's
`RefArg`). Each probe also logs its id, the thread (libtest names it after the test) and the process's argv, so one
reached inside a child `ein` would be named (`build/s1f.5.4/t3/hits.log`).
- `cargo test --workspace --no-fail-fast` on it (`build/s1f.5.4/t3/probe-tests.log`, 16 min 21 s): 919 passed and
  19 failed, every failure a probe, and **every probe in a test binary, none in an `ein` process**: 17 Rust tests
  that build an int by hand (`ein-core`'s nine unit tests of the pool, the tag, `Terms`, the fact store and the
  participation index; `fork_cost`'s two; `values_semantics`' two; `ein-einb`'s `tables` unit test; `roundtrip`'s
  two, whose crowded interner held 37 ints; `search_semantics`' `(not 5)`), and `corruption`'s two fuzzers, whose
  damaged bytes the reader decoded as a value tagged `0b01`, an int. No corpus sweep, golden, fuzz property or CLI
  run reached one.
- Every corpus file under 6 000 bytes (398) and 40 000 seeded mutants of them (`ein_corpus::mutants`, seed 0), then
  200 000 more (seed 987654321), each parsed, loaded, saturated and printed, every fact through the one printer and
  the program through `dump_canonical` (`t3_sweep`, an example in the copy, `build/s1f.5.4/t3/t3_sweep.rs`): 16 308
  refused at the parse, 2 671 at load, 82 at compile, 21 238 saturated and 99 stopped saturating, and 81 452,
  12 999, 352, 105 126 and 469, **no probe and no other panic** (`sweep.txt`, `sweep2.txt`), T5's counts on the same
  inputs.

And by construction: the generated parser builds no `Int` (S1f.5.5's T4 and T5), so the loader never interned one,
and the only other way in was a `.einb`'s bytes.

**Removed.**
- `ein-core`: `Tag::Int`, `Value::int` and `as_int`, `IntId`, `IntPool` with `cmp_value`; `Terms::ints`,
  `value_int`, `intern_int` and `int_text`, and the int arms of `cmp_semantic`, `repr_value`, `display` and
  `tag_order` (`Sym < Fact`, design/02 §5 H2's order without its first member); `Overflow::Ints`; `ReprValue::Int`
  and `canonical_int`, with `ein-ir`'s re-export.
- `ein-ir`: `Node::Int` and its arms in `eq_nodes`, `node_repr`, `dump`, `expect::render`, `intern_program_names`
  and `atomic_value`, and the loader's line *its parameter ?p names a relation, and … binds it to the integer …*
  (`defined_behaviour.md` § 4.10's row, T1f.5.4.4's).
- `ein-infer`: the compiler's two `Node::Int` arms (a slot, a guard's argument), the guard key's code 5, and
  `head_not_a_name`'s integer line, *the relation head … is bound to the integer …* (§ 4.10's second row), so a value
  that is not a name is a fact there; the `Tag::Int` arms of `invariant::walk_value` and `region`'s order key, and
  `read_out`'s goal term.
- `ein-render`: `RefArg::Int` and the `Node::Int` arms of `dot_util::value_label` and `ir_dot`.
- `ein-einb`: `Maps::int` and its id space. `INTS` is written empty and an entry in it is refused, as is a value
  tagged `0b01`, with `EinbError::Int`: *.einb holds an integer, which Ein has not had since M1f S1f.5.4 (a run of
  digits is a name): save it again from its .ein*.
- Every Rust test that built an int by hand builds the name of the same digits, and the comments that described an
  int as a live value say what it was (a record where they name the stage that changed it).

**Kept, and why.**
- **`.einb`'s `INTS` section**, written empty and required, so no layout changed and no version moved: a 1.x reader
  opens what this writer writes, and this reader opens what a 1.x writer wrote for a program with no int.
  [Q-M1f.10](../open_questions.md#q-m1f10--the-einb-ints-section-once-the-int-kind-is-gone) (a), taken by default on
  the user's instruction of 2026-11-09; the stage's *with its format version bumped* was (b).
- **`Tag`'s numbers**, `Sym = 0` and `Fact = 2`, so every stored value keeps its bits; `0b01` is no value,
  `Value::from_bits` refuses it, and `Value::tag` asserts it in a debug build.
- **What only looks like an int**: `ein_core::parse_int` (CLI options), `Json::Int` and `BigInt` (the JSON writer's
  numbers, counts and ids), and `config::FieldKind::Int` (a `(config …)` field's kind, the retired seeds'). None is a
  value a program writes.
- **The guard refusal's sentence**, *a guard compares two values, each a variable a premise before it binds, a
  name, a string or an integer* (`from_ir`'s `GUARD`). `W24/form-in-guard` and `W24/guard-before-binder` record it as
  their `refused` line, so restating it moves two register rows, which this commit may not. It names no int a
  program can write; S1f.5.2's string arms leave the same sentence, and it is restated there.
- **The `:priority` guard** this page planned has nothing to guard: S1f.5.15 removed the keyword from every program
  and the grammar refuses it (the dated note above), so no program yields a count, and every rule list `saturate
  --dump` prints is unmoved (*Nothing moved*).

**The ruling, held in every place** (P1f.5's order item 8):
- `cli_semantics`' `a_run_of_digits_is_a_name_in_every_place`: one program writes a run of digits in a fact's
  argument and relation, a nested fact, a stored negation, a declaration's name and signature, a rule's name, a
  premise's relation and argument, a guard, an `absent`, a conclusion and a claim; `ein saturate --dump` prints each
  as written and keeps `1` and `01`, `7`, `-7` and `07` apart, the rule `1` fires on `(p 1)` and not `(p 01)`, and a
  claim naming `(q a 1)` where the state holds `(q a 01)` is refuted (`ein test` exit 1).
- `grammar_pages`' table, 18 rows at its end (123 → 141): a token that is no name in each place one goes.
- `values_semantics`' `a_run_of_digits_is_a_name_as_written`, where the int pool's canonicalisation test was: every
  literal that test generated, 361 distinct texts of 391 (the test asserts it), is a name of its own text that
  prints as written.
- `ein-core`'s `value` tests: `an_ints_tag_bits_are_no_value`; `ein-einb`'s `tables` test and `invalidation`'s
  `a_file_that_holds_an_integer_is_refused`, which builds both shapes of an old file (an `INTS` entry; a value
  tagged `0b01`) on one this writer wrote, its digest good, and holds the refusal and its sentence; the file as
  written opens with `(p 7)` and `(p 07)` two facts.

**Nothing moved.** Every `.ein` under the five roots (511, `find examples tests stdlib corpus plans -name '*.ein' |
wc -l`, 2026-11-09) through `edb096af`'s release build and this commit's (`build/s1f.5.4/t3/xcheck/`, T5's
scripts pointed at the two, 140 s): `solve --json-summary -H --dump-answer`, `saturate --dump` and `test`, exit
codes, stdout, stderr, the summary and every file the dump wrote compared, timings masked, **511 of 511 the same on
every command**, so R, N, H, every claim and every rule list `saturate --dump` prints are unmoved; and T12's 2 000
seeded mutants through `saturate`, the error text included, the same (1 047 accepted, 767 refused with repairs,
186 without). **The container in both directions** (`einb.sh`): each file `ein kb save`d by each binary and each
container read by both with `solve`: on the 339 that load, the four reads agree with each other and with `solve` of
the `.ein`, but one, S1f.5.5's probe `g14/derived-activator.ein`, whose compile refusal names its place as
`<string>` and the stored text's line, in all four reads alike, which is how a container's re-parsed program has
always placed one; the other 171 do not load, and neither build saves them. The goldens: `repr_values.txt` and
`grammar_table.txt`, as named above, and no other.

**Taken by default on the user's instruction of 2026-11-09** (*"just continue with default decisions till the end
of the phase"*), each the agent's recommendation:
- **The `INTS` section kept, empty, with no format bump** (Q-M1f.10 (a), above).
- **The guard sentence left** for S1f.5.2 (above), where restating it would move two register rows.
- **`repr_values.txt`'s int rows kept as names**, not deleted: each shape stays in the table, a run of digits a
  `str` of any width, which is the ruling in the `repr` format.
- **The tests that held an int's case keep their names** (`activators_differing_only_by_an_int_argument_both_fire`
  and its two siblings in `rule_semantics`, cited by name in `firing.rs` and in M1e's record): their programs are
  unchanged and now hold the case for names, which a dated paragraph above them says.

**The gate.** On this commit, 15 min 48 s (`build/gate-s1f.5.4-T3.log`), green: 937 tests and every static check,
where T6's ran 938 in 15 min 46 s: four went (`value.rs`' three of the int pool, `repr.rs`' `canonical_int`) and
three came (`an_ints_tag_bits_are_no_value`, `a_run_of_digits_is_a_name_in_every_place`,
`a_file_that_holds_an_integer_is_refused`); `values_semantics`' pool test became the name test, and the 18 table rows
are rows of one test.

### Task T1f.5.4.4 — the pages

After the swap, beside S1f.5.5's T7. `00_ebnf.md`, `01_grammar.md` (the `INT` row), `02_patterns.md`,
`defined_behaviour.md`, `glossary.md`, `examples/README.md`. Q-M1e2.20 says what overrode it.

### T1f.5.4.4 record, 2026-11-09

**One commit, on `eccf4186`** (T1f.5.4.5), pages only, and the register's one id. Run last of the three, after W37's
fix, so that the pages describe both. Each sentence that described an `INT` token or an int value as live says what
it was and what it is since this stage; a record, and a quotation of an engine line, is left as written. S1f.5.5's
T7 keeps the grammar pages' prose: what this commit restates there is only the digits and the int.

**Restated.**
- `01_grammar.md`: the `INT` row, *none since the swap*, a run of digits a `NAME` and the int kind gone; a premise's
  and a guard's values, where *an integer* was one of them; and the `:goal-text` paragraph, what a reference names
  and that a template naming a variable no reference can reach is refused.
- `02_patterns.md`: the same two lists. `00_ebnf.md`: § 1's fact 1, where `neq-1` *starts an `INT`*, and § 5's census
  line, whose `INT` 84 are names; the rest of § 1's facts and § 3 stay T7's.
- `defined_behaviour.md`: § 2.1, the order `Sym < Fact` (its heading's old anchor kept by an `<a id>`, since
  `docs/history/m1a_rust/design/02` cites it); § 2.2, the `repr` and the printer of three shapes; § 3.2, the bindings
  half and the register layout, a name where an integer binds; § 3.3, the one shape outside the induction a nested
  fact; § 4.5, two id spaces and *too many distinct {symbols,facts}*; § 4.6, its two lines retired by T1f.5.4.3; § 4's
  firing line, which T5 deleted, its *int* no value; § 4.9's premise row; and **a new § 4.11**, the loader's refusal of
  a template reference no reference can reach (T1f.5.4.5).
- `glossary.md`'s **Atom**: a run of digits is an atom like any other. `examples/README.md`: the `mixed-type-hypothesis`
  clause and W7's family, their int a name since.
- Beyond the stage's list, each a sentence that described the int as live: `02-data-model/03_implementation.md`'s
  `value.rs` and `terms.rs` rows, `01_entities.md`'s `Fact.args`, `architecture_and_algorithms.md`'s `FactId` row,
  `inference/README.md`'s two, `answer_dump.md`'s printer, and `docs/api/rust.md`'s `Terms`.
- [Q-M1e2.20](../../m1e2_correctness/open_questions.md#q-m1e220--does-a-claim-compare-values-or-their-printed-text):
  overtaken, its case unwritable since the swap and the int kind gone, the printer's comparison by value standing.

**The register's id.** `W37/template-var-star`'s `[case.fixed]` names `eccf4186`, where T1f.5.4.5 left the null id,
and the header's paragraph says so; `ein-corpus`' schema tests pass (`cargo test -p ein-corpus`, 2026-11-09).

**What a page still says about an int**, each on purpose (`grep -rn -i "\bint\b\|integer\|\bINT\b" docs/kernel
docs/api docs/guide`, 2026-11-09): a dated record (*until M1f S1f.5.4*, § 3.2's W7 and W16 history, `answer_dump.md`'s
W28), a quotation of an engine line (§ 4.9's two guard rows, whose sentence still says *or an integer*, kept by
T1f.5.4.3 for the two W24 rows that record it; § 4.6's two retired lines), the three pages under a superseded banner,
and an integer that is no value (a `u32` id, `priority: int` in `01_entities.md`'s `Rule`, S1f.5.15's).

### Task T1f.5.4.5 — a template names every variable (W37)

**Added 2026-11-08**, on the user's ruling on S1f.5.5's T9 review stop, item 7 (*"Accept all as recommended"*):
S1f.5.5's T10 filed [`W37/template-var-star`](../../../corpus/wrong_answers.toml), `open` and owned by this
stage, since the name rule is what gives a variable its characters. `ein_core::why::render_why` reads a template
reference as `{?`, one letter or digit, then letters, digits, `_` and `-`, then `}`, so a reference to a variable
holding any other character is printed as written: the stdlib's three `{?isR*}` `:why`s on `--trace` today, the
witness's `:goal-text` on the read-out, and after the swap every variable the name rule adds (`?_x_`, `?a.b`,
`?x'`; [S1f.5.5's T9 review § 11](s1f.5.5_the_grammar.md#t9-record-2026-11-08)). **The fix, as ruled:** a
reference reads the name rule's characters up to the first `}`. After the swap, on the generated parser, with
W37's row flipped in the same commit and the stdlib's three traces printing the hierarchy relation
(`tests/stdlib/typing/02_type_hierarchy_converse.ein`, `03_type_hierarchy_welltyped.ein`,
`tests/stdlib/algebra/17_converse_illtyped_ran.ein`). One question rides on it, which T9's review recommended and
this stage settles: `{` and `}` are name characters for now (the user, 2026-11-07), so a variable holding `}`
cannot be referenced; recommended, the loader refuses a template that names a variable it cannot reach, which no
corpus program writes.

**Named before it moves, 2026-11-09.** A reference to a variable outside `[A-Za-z0-9_-]` is written, among the corpus
programs, by the stdlib's three `{?isR*}` templates and by the witness (`grep -rnoE '\{\?[^} ]*[^A-Za-z0-9_} -][^} ]*\}'
--include='*.ein' examples tests stdlib corpus`, 2026-11-09), and `--trace` prints `{?isR*}` on four programs, not three:
`tests/stdlib/algebra/16_converse_illtyped_dom.ein` too, whose `converse-illtyped-dom` fires (`ein solve FILE --trace
T` over every program naming `type-hierarchy` or `converse`, 2026-11-09). So what moves is:
- `ein-render/tests/golden/corpus_shapes.md5`, the rows of those five programs, `typing/02` and `/03`, `algebra/16`
  and `/17` and `template-var-star.ein`, wherever a row renders a template: the region trace, and any other op that
  prints a `:why` or a `:goal-text` filled;
- `corpus/wrong_answers.toml`'s `W37/template-var-star`, flipped to `fixed`, its `now` the right sentence, *a is a q*.
No other golden, and no answer.

### T1f.5.4.5 record, 2026-11-09

**One commit, on `410a3388`** (T1f.5.4.3). `W37/template-var-star` is fixed: a template reference names every
variable the name rule spells, and the question that rides on it is settled by the loader.

**The fix** (`ein_core::why::render_why`). A reference is `{?`, one or more of the name rule's characters
(`ein_core::text::is_name_char`) other than `}`, then `}`: the variable's name as the program writes it after its
`?`, up to the first `}`. Until now it read one letter or digit and then letters, digits, `_` and `-`, so `{?isR*}`,
`{?n*}` and every variable the name rule adds past those (`?_x_`, `?a.b`, `?x'`, `?$x`) printed as written. A
character no name has (a space, `?`, `:`, `,`, `#`, `|`, anything outside ASCII) still ends no reference, and the
text stays. Held by `why`'s `a_reference_reads_the_name_rules_characters_up_to_the_first_brace` (23 names, one per
decorator and connector, and the characters that end none).

**The question, settled** (taken by default on the user's instruction of 2026-11-09, as S1f.5.5's T9 review
recommended and § Task T1f.5.4.5 above wrote it): `}` is a name character, so `?a}b` is a variable no reference can
name, `{?a}b}` reading as `{?a}` and the text `b}`. **The loader refuses a template that writes a reference to a
variable in its scope holding `}`**, one line a reference: a rule's or an hrule's `:why` (its parameters and both
clauses' variables), a macro's `:why` (its parameters) and a query's `:goal-text` (its goal's variables), *rule 'r':
its :why writes {?a}b}, a reference to ?a}b, which no reference can name — a reference reads a variable's name up to
its first }*. A relation's `:why` names its slots by position and is not read. No corpus program writes one, so no
answer moves (*Nothing else moved*). Held by `load_semantics`'
`a_template_naming_a_variable_no_reference_reaches_is_refused` (each of the four places, and each loading with a
reference it can reach, with no reference, and with `{?a}`, a reference to no variable, kept as text) and
`ein_core::why::unreachable_references`' unit test.
[Q-M1f.11](../open_questions.md#q-m1f11--a-template-reference-to-a-variable-no-reference-can-name) records it.

**What moved, as named above.**
- The four stdlib programs' traces name the hierarchy relation, `is-a*`, where they printed `{?isR*}`, two lines
  each and four on `typing/02` (`ein solve FILE --trace T`, release, 2026-11-09: no reference left unfilled on any
  of the four), and the witness prints *“a is a q”* and, on `--trace`, *a is a p, so it is a q*.
- `corpus_shapes.md5`: ten rows, the four programs' `region-trace` and the witness's `region-trace` and five
  `read-out[…]` (`EIN_BLESS=1 cargo test -p ein-render --test corpus_shapes`, 2026-11-09), no other.
- The register: `W37/template-var-star` flipped to `fixed`, its `now` *a is a q*, its `[case.fixed]` the null id
  (the stage's next commit, T1f.5.4.4, wrote `eccf4186`), and the header's paragraph; **118 rows fixed, 21 open and 1
  not-wrong** (`grep '^status' corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-09). `ein-corpus`'
  `what_a_goal_text_prints_is_an_answer`, which took W37's live row as its open example, reads the row as it stood at
  `410a3388`, embedded, as the swap's three schema tests read theirs, and holds that the live row answers.
- The witness's header, its corpus note and its `examples/README.md` line say it is fixed.
- And, comments only, T1f.5.4.3's leftovers: `mixed-type-hypothesis.ein`'s and `nested-fact-hypothesis.ein`'s
  headers, which described the int as live, and `fact_grain.rs`' `Arg::Val`.

**Nothing else moved.** Every `.ein` under the five roots (511) through `410a3388`'s release build and this
commit's (`build/s1f.5.4/t5/xcheck/`, T1f.5.4.3's scripts with `solve --trace`'s page added, 157 s, 2026-11-09):
**506 the same on every command**, and five move, each only where named: the witness's read-out sentence (`solve`'s
stdout and the dump's `answer.txt`, *“a is a q”*) and trace, and the four stdlib programs' traces, the `{?isR*}`
lines alone; no exit code, summary, state file, claim or `saturate --dump` moved. T12's 2 000 mutants through
`saturate` the same (1 047 accepted, 767 refused with repairs, 186 without).

**The gate.** On this commit, 15 min 50 s (`build/gate-s1f.5.4-T5.log`), green: 940 tests and every static check,
T1f.5.4.3's 937 and three more, `why`'s two and `load_semantics`' one.

## Acceptance

**Since the re-plan of 2026-11-08 the items the grammar settles are
[S1f.5.5's acceptance](s1f.5.5_the_grammar.md#acceptance)**, held at its swap: `(p 1) (p 01)` two facts, the
names that load and print as written, the characters refused, `:priority` digits only and no name, and the five
places [the paper grammar says more](#the-acceptance-list-against-the-paper-grammar). This stage's own, after the
swap:

- `Tag::Int` and `Node::Int` are gone, and that commit moved no golden.
- With the int kind gone, `:priority 110` still orders its rule and puts no name in the program: H, and every
  rule list `saturate --dump` prints, are unmoved on every program.
- No page describes an `INT` token or an int value.
- `W37/template-var-star` is fixed: a template reference names every variable the name rule spells (T1f.5.4.5).
- `./run_tests.sh` is green.

**Met, 2026-11-09.** `Tag::Int` and `Node::Int` are gone (T1f.5.4.3), and that commit moved two test goldens it
named first, `repr_values.txt`'s int rows and `grammar_table.txt`'s new rows, and no answer golden; no program writes
`:priority`, so H and every rule list are unmoved on the 511 (its cross-check); no page describes an `INT` token or an
int value as live (T1f.5.4.4); W37 is fixed (T1f.5.4.5); and the gate was green on each code commit (15 min 48 s, 15
min 50 s).

## Step 1 — every move listed (2026-11-08)

T1f.5.4.1, at `67da56a7`. **The prototype** is a `git archive` snapshot under `build/s1f.5.4/proto` (gitignored)
with four edits, each marked `S1f.5.4 prototype` and nothing else: the lexer's name rule (`lex.rs`: `SYMBOL` a run
of [name characters](s1f.5.5_the_grammar.md#the-name-rule) up to a separator, any of them first, not a bare `_` and
not a whole reserved word; `VAR` `?` and such a run, `?_` refused; a string, a keyword and a reserved word end at a
separator too, so nothing splits), no `INT` and no `EQ` (`parse.rs`: `leaf_value`, `list_head`, `alt_eq_fact`), the
printer's `is_symbol` as the same rule (`ein-core/src/text.rs`), and the `:priority` guard (`from_ir.rs`: an atom of
digits is the count). Release build, target inside the snapshot. Today's binary is the checkout's at `67da56a7`.
Every answer below is one of the two, `EIN_STDLIB` the checkout's. The prototype is a measuring device on the
hand-written parser, not a plan for it: since the user's re-plan of 2026-11-08 the name rule is built in
[S1f.5.5](s1f.5.5_the_grammar.md)'s generated parser and lands at the swap, and this list is the digits part of the
attribution that precedes it ([§ Which moves are the grammar's](#which-moves-are-the-grammars)).

**The review material is `build/s1f.5.4/review/index.md`** (gitignored; `python3 build/s1f.5.4/review_build.py &&
python3 build/s1f.5.4/index_build.py` rebuilds it, 54 programs, 3.2 GB): for each program the program as written,
its plain digits marked, today against the prototype (exit codes, k_s, k_c, N, H, the `universe` rows, the
read-out, `saturate --dump`), and every artefact both binaries write, raw: `--json-summary`, `--dump-answer` with
every state file, N's `nogoods.json` and H's `hyp-stats.txt`, `--dump-lattice`, `--trace`, `ein test -v`, `ein oracle
--dump --states` where H has 16 candidates or fewer, and every `ein render` view as DOT, drawn as SVG where it moved.

### The sweep: 46 programs move, and 465 answer as they did

`build/s1f.5.4/run_sweep.sh`: every `.ein` under `examples/`, `tests/`, `stdlib/`, `corpus/` and `plans/`, 511 files
(`find examples tests stdlib corpus plans -name '*.ein' | wc -l`, 2026-11-08), each through `ein solve FILE
--json-summary J -H --dump-answer D` and `ein saturate FILE --dump` on both binaries, timing lines removed, 1 min 14 s;
`build/s1f.5.4/analyze.py` compares exit codes, k_s, k_c, the states, N's sets, H's candidates and the stored facts
(`build/s1f.5.4/moves.tsv`, one row a file). 44 of the 46 write a plain digit or import `std.elim`
(`build/s1f.5.4/digits.py`: a token today's lexer reads as an `INT`, outside a `:priority` value, comments and strings);
two write none, `assert-empty` ([the finding](#the-finding-two-synthetic-names-become-spellable)) and S1f.5.5's
`refused/names.ein`, refused both ways, at `a?x` (3:4) where today's parser stopped at `a:b` (5:7).

| § What moves | measured on 2026-11-06 without a prototype | the prototype |
|---|---|---|
| `features/05_stdlib_domain_elim` | k_s 32, k_c 32; N 21 → 47 | the same: k_s 32, k_c 32, **N 21 → 47, H 26 → 52**, each new candidate (`(color-of 0 1)`, `(color-of 1 House-2)`, …) dead alone; the stored facts the same |
| `branching/12_typed_blind_solve` | k_s 3, k_c 7; N 25 → 51 | the same: **N 25 → 51, H 25 → 51** |
| the `(edge 1 2)` family | k_s 1 or 2 → past 56 000, stopped; one exits 1 | `binding-key-both`, `-swapped`, `-nested-only` and the two `binding-key-layout` fuzz seeds: k_s 2 → **≥ 56 401, stopped at the budget**, k_c 2²¹, H 1 → 21, each `:expect` NOT CHECKED (exit 0); `binding-key-int-only`: k_s 1 → 640, H 0 → 10; the four `activator-text-*` and the two `activator-text-key` seeds: k_s 1 → **1 280, exhausted**, k_c 2 048, H 0 → 11. **Five exit 1**, not one, each its own `:expect` refuted: the measurement renamed `1` to `d1` and kept the string `"1"` apart, and the prototype makes `1` and `"1"` one name |
| `str-in-assert`, `str-in-match`, `classes-int-name` | k_s doubles or more (S1f.5.2's) | `str-in-assert` k_s 4 → 32, H 2 → 5; `str-in-match` and `classes-int-name` keep k_s 8 and H 3, the candidate printed `(hit 1)` and `(pick 1)` where it was `(hit "1")` and `(pick "1")`, the same value; `classes-int-name`'s query **4 classes → 2**, the int and the name being one name (as above) |
| `syntax/arg-kinds` | stopped either way | refused (S1f.5.3's interim) → **loads**: k_s ≥ 342 542, stopped at the budget, k_c 2⁵⁴, H 54; `saturate` stores `(r 7 "a string")`, `(r 1..5 1..*)`, `(r "?x")`, `(r (nested a b) c)` |
| `broken/load/relation_parameter_int` | refused → loads | refused → **loads**: `(mirror 3)` names the relation `3`; k_s 32, H 5 |
| *not in the table:* `wrong-answers/range-in-match` | (S1f.5.3's interim) | refused → **loads**, the rule a pattern over the name `1..3`: H `(hit 1..3)`, `(hit 2)`, `(n 1..3)`, R the whole cube, 8 states, `(hit r)` exactly where `(n 1..3)` holds; the oracle the same |
| `wrong-answers/claim-int-name`, `-solve` | — | exit 1 → **0**: the claim `(model (holds 1))` holds, the int and the state's name `"1"` being one name |
| `ein-bugs/activator-all-int` | — | refused both ways; the line gains `[activator 1]`, as its symbol control's has `[activator a]` |
| `ein-bugs/int-goal-binding`, `mixed-type-hypothesis`, `str-in-hrule`, `syntax/is-a-shapes`, `tests/stdlib/elim/01` and `/02`, S1f.5.5's `programs/is-a-shapes` | — | `saturate`'s name table alone, a digit now counting as a name; R, N, H and the stored facts the same |
| 16 more under `plans/` (M1e2's probes and S1f.5.5's), records that stay | — | listed in the review's § 5; `values.ein` and `kwpair_in_pattern.ein` load again, and S1f.5.5's `programs/arg-kinds.ein` now stops `solve` with the state route's *a state's commitments pass 2^128* (exit 1), an engine limit three M1e2 probes reach today |

`zebra2.ein` is unmoved: k_s 1, k_c 261 982, N 366, H 56, byte for byte. `std.elim`'s text does not change; read
differently, its `0` and `1` move `features/05` and `branching/12` as above and nothing on `tests/stdlib/elim/`, whose
four programs close their relations, so the two names join no H. `(p 1) (p 01)` stores one fact today and two in the
prototype, each printing as written and reloading to itself. **Order:** candidates sort as text, so `(p 10)` lists
before `(p 2)` (`build/s1f.5.4/scratch/order2.ein`); no corpus program writes a two-digit name.

### Which moves are the grammar's

For the swap's attribution (the user's re-plan of 2026-11-08). The generated parser's actions
(`s1f.5.5_parser_probe/grammar/ein.y` and `build_ast.rs`) build a run of digits as a name, `Atom("01")`, everywhere
but after `:priority`, where `b::count` builds today's `Node::Int`; the prototype built the same atoms by hand. So:

**The grammar's, S1f.5.4's ruling (the name rule), arriving with the swap:**

- **Every token that reads differently:** a run of digits is a name (`(p 1) (p 01)` two facts, `-7`, `1st`, `1_a`,
  `1..5`, `07`); the other name characters (`$x`, `'name'`, `{}`, `[a]`, `<=`, …); a reserved word only as a whole
  token (`rule-x`, `rule.x`, `neq-1`, `not-a`: `grammar_decisions`' 435 `keyword-1` parses); `=` a name, so `(= a b c)`
  is a fact; and a token ending at a separator, so `a?x`, `?x?y`, `"a"b`, `a:b`, `Жук` are refused, and `:priority`
  takes digits and nothing else. `_x`, `__` and `?_x` are attributed by [C1](#for-the-coordinator).
- **Every answer move above**, since each follows from a digit or a range being a name in the tree with no other code
  changed: `features/05` and `branching/12` (N, H), the `(edge 1 2)` family, `str-in-*`, W28's (`1` and `"1"` one
  name), `relation_parameter_int`, `arg-kinds`, `range-in-match`, `values.ein` and `kwpair_in_pattern.ein` loading,
  `activator-all-int`'s line, the 2^128 probe, and the two refusals whose position moves (`four_mistakes`,
  `refused/names`).
- **The test and golden moves those carry:** `corpus_exits.txt`, `corpus_shapes.md5`, `grammar_decisions.txt`,
  `fuzz_seeds.txt`, `config_coercion.txt`, the two `branching/12` traces, `ir_semantics`' table, the interim list's
  four readers and `a_range_is_no_token`, and the int-against-string cases of `rule_semantics`,
  `compile_semantics` and `compile.rs`, which parse `1` to get an int.

**Not the grammar's** (outside the parser; each says when it has to land):

| what | where | when |
|---|---|---|
| the printer: a name bare where the lexer reads it back as one name, so `(p 1)`, `(hit 1)`, `(q @empty)`, `it's` print bare | `ein_core::text::is_symbol` | **with the swap**: `the_printers_symbol_is_the_lexers` holds the two to one answer. Without it every digit name prints quoted, `(p "1")`, the same value (S1f.5.5's prototype did so). The moves that are the printer's alone: the spelling in `str-in-match`'s and `classes-int-name`'s states, W28's and W34's `now`, `KNOWN_R1`'s line, `values_semantics` and the `terms`/`text` unit tests |
| the `:priority` guard | `from_ir.rs`' `priority` match | **nothing at the swap**, since the grammar's action keeps today's `Node::Int`; it becomes work when the int kind goes, as `Rule::priority: Option<IntId>` needs a count that is not a value. Without any guard every value is dropped and 163 files' rule lists move, no answer |
| the int kind's retirement: `Tag::Int`, `IntId`, the pool, `cmp_value`, `Node::Int`, `.einb`'s int section | `ein-core`, `ein-ir`, `ein-infer`, `ein-render`, `ein-einb` | after the swap, moving no answer: no program then yields an int but `:priority`'s count |
| a name of digits in `summary.json` ([Q4](#q4--a-name-of-digits-in-summaryjson)) | `region_answer.rs`' `binding` | **with the swap**, or `(p 007)` and `(p 7)` render as one JSON `7` |
| the synthetic names ([Q1](#q1--the-synthetic-names)) | `@empty` and `@params`, which today's `parse.rs` and the generated parser's `build_ast.rs` both build; `text.rs`' `EMPTY_HEAD` and `NESTED_HEAD`; `terms.rs`' `QUERY_RULE` | **no later than the swap**, or the swap brings the collision |
| the register's 12 rows and their `[case.region]` blocks ([§ The register](#the-register-12-rows-move)) | `corpus/wrong_answers.toml` | in the commit where each moves, the swap's |
| the fixtures: the witnesses' scoping and `:expect`, `relation_parameter_int` ([Q2](#q2--relation_parameter_int)), `arg-kinds` ([Q3](#q3--arg-kinds-and-the-round-trip)), `int-goal-binding`'s note, the slow flags | `examples/`, `corpus/` | with the swap, so that the corpus stays green across it |

### H moved on a program that writes no plain digit: the finding

The stop this page sets. One program: `examples/ein-bugs/wrong-answers/assert-empty.ein` (W34, open, S1f.5.9's),
whose one candidate printed `(q "@empty")` today and `(q @empty)` in the prototype. **The value is the same**: the
printer prints a name bare where it lexes back as itself, and under the name rule `@empty` does.

#### The finding: two synthetic names become spellable

The loader names two things no program could spell until now, `@empty` (the relation of the empty form `()`) and
`<nested>` (the relation of a nested form whose head is not a name, `(_ a)`), and `@`, `<` and `>` are name
characters. In the prototype a program can write either and gets the loader's: on `(relation p T) (p ()) (p (@empty))
(p (_ a)) (p (<nested> a))`, `ein saturate --dump` is a parse error today (3:5) and **two** stored facts in the
prototype, `(p ())` and `(p (_ a))`, for the four written (`build/s1f.5.4/scratch/synthetic-heads.ein`). That is a
wrong answer this stage would introduce. `@params` (the parameter list's head, which the loader's pattern walk skips
as a relation name) and `<query>` (the query's rule name, which a firing's `:rule` prints) are spellable too; a
relation `@params` and a rule `<query>` answer as their renamed twins on the probes tried. [Q1](#q1--the-synthetic-names).

### `:priority`: H unmoved, and where the trap is

179 files write `:priority`, 577 values; 175 write no plain digit, and H is the same as printed on all 175
(`build/s1f.5.4/priority.txt`, from the sweep). One value is not digits, `:priority high` in S1f.5.5's
`refused/rules.ein`, refused both ways for other reasons. **The guard holds the rule order, not a name:** every file
that contains `:priority`, 180 (`grep -l`, one of them in a comment alone), through the prototype and through the
prototype without the guard (`build/s1f.5.4/noguard/`, every value dropped by the loader's `_ => None`): 17 are
identical, and 163 move in `saturate --dump`'s rule list alone (`:priority 200 (derive)` → `:priority
None (unbanded)`), none in an exit code, R, N or H (`build/s1f.5.4/noguard_sweep.txt`). So the trap is `from_ir.rs`'
`priority` match, which reads `Node::Int` only and drops anything else in silence; `invariant.rs`' name walk reads a
rule's `:match` and `:assert` alone (`rule_constants`), never `:priority`. Today `:priority high` and `:priority
"110"` load with the value dropped, `-5` loads as an int, and `1.5` is a parse error; the prototype drops all four.
The acceptance's *a run of digits and nothing else* is the paper grammar's `:priority DIGITS`, refused at the parse.

### The acceptance list against the paper grammar

`build/s1f.5.4/acceptance/probe.sh`, each as `(p NAME)` under `(relation p T)`, `saturate --dump` and the printed fact
reloaded: in the prototype `1st`, `1_a`, `7`, `-7`, `1..5`, `$x`, `%`, `a^b`, `a/b`, `<=`, `!x`, `@x`, `~`, `'name'`,
`f'`, `{}` and `[a]` each print as written and reload to themselves, `'name'` and `name` are two facts, `(p 1) (p 01)
(p -7) (p 7)` four, `?a.b` binds as a variable, `a?x`, `?x?y`, `??x`, `a:b`, `a,b`, `a#b`, `a|b`, `a\b`, `Жук`, `"a"b`
and `?_` are parse errors, and `Жук` in a `:why` loads. Today `a?x`, `?x?y` and `"a"b` load split, `01` stores `1`,
`1st` stores `(p 1 st)`, and the rest of the names are refused.

**No item of the list disagrees with [the paper grammar](s1f.5.5_paper_grammar.md)** (`ein.l`'s `NAME`, `VAR`,
`DIGITS` and its catch-alls). It says more in five places, which the prototype follows and the acceptance should
name: `_x`, `__`, `_-1` and `?_x` are a name and a variable (any name character first; [C1](#for-the-coordinator));
`rule-x`, `rule.x`, `neq-1` and `not-a` are names, a reserved word being a whole token, which overrides
[Q-M1e2.17](../../m1e2_correctness/open_questions.md#q-m1e217--where-does-a-keyword-end) (i)(a) as
[the name rule](s1f.5.5_the_grammar.md#the-name-rule) says; `=` is a name, so `(= a b c)` is a fact; `(1 a)` and
`(relation 1 T)` are a fact of and a declaration of the relation `1`; and a keyword is `:` and a run of name
characters, any unknown one refused ([C2](#for-the-coordinator)). The finding above is the one place it says nothing.

### `cargo test --workspace` on the snapshot

`build/s1f.5.4/proto-test.log`, 2026-11-08, 14 min 41 s: 888 passed and 33 failed, in 21 targets, each by this stage's
mechanism:

| target | test | why it moves |
|---|---|---|
| `ein-cli` `answer_dump` | `every_recorded_state_is_a_fixpoint_of_its_own_program` | `claim-int-name-solve` exits 0: its claim holds (W28) |
| `ein-cli` `answer_round_trip` | `the_witnesses_reload_to_their_states` | `arg-kinds` answers: `REFUSED_UNTIL_S1F_5_4`, emptied where the name rule lands |
| `ein-cli` `cli_semantics` | `a_range_is_no_token` | `(r 1..5 a)` loads: S1f.5.3's test, to become its plain-name counterpart where the name rule lands |
| `ein-cli` `corpus_cli` | `the_files_refused_until_digits_are_names_are_refused` | the interim list, deleted where the name rule lands |
| | `every_cell_reproduces_its_exit_code` | `corpus_exits.txt`, 20 of 1 302 cells (below) |
| | `every_refusal_carries_a_diagnostic` | the eight `ein-bugs/` witnesses' `test` cells exit 1 (each `:expect` refuted) with an empty stderr |
| | `the_slow_flag_still_describes_the_sweep` | `binding-key-both` 5 185 ms, `-swapped` 5 203 ms, `-nested-only` 4 504 ms of cells, past the 1 s mark |
| `ein-cli` `wrong_answers` | `every_row_answers_what_the_register_records` | 12 of 137 rows |
| `ein-core` lib | `terms::tests::display_of_each_of_the_three_shapes`, `text::tests::a_name_is_bare_only_where_it_lexes_as_itself` | the name `1` prints bare |
| `ein-core` `values_semantics` | `display_renders_every_value_shape` | `it's` prints bare (`'` is a name character) |
| `ein-infer` lib | `compile::tests::plan_key_is_the_activators_values` | the int and the name with one text are one value: 3 plans, not 4 |
| `ein-infer` `alive_invariant` | `every_name_of_the_program_is_a_candidate_object` | `SEEDED` moves: digits are names, and `arg-kinds` loads |
| | `no_run_reaches_a_name_the_program_does_not_have` | `KNOWN_R1`'s W34 line prints `@empty` bare |
| `ein-infer` `compile_semantics` | `a_relation_parameter_a_rule_binds_to_a_number_is_refused` | `(mirror 3)` names the relation `3` |
| | `a_string_in_a_rule_is_the_symbol_it_names` | `(tag other 1)` and `(tag other "1")` are one fact |
| `ein-infer` `refutation_under_absent` | `the_refused_set_is_exactly_these` | `activator-all-int`'s line gains `[activator 1]` |
| `ein-infer` `register_region` | `every_row_states_its_witness_s_region` | 10 rows' `[case.region]`: W7, W15 ×2, W16 ×2, W18 ×2, W20, W28/classes-int-name, W34 |
| `ein-infer` `rule_semantics` | `an_int_and_a_string_with_one_text_are_two_activators`, `a_string_in_a_rule_means_what_it_means_in_a_fact` | W16's and W18's int-against-string cases: one value now |
| `ein-ir` lib | `lex::tests::a_literal_is_a_whole_word` | `neq-1` is a name |
| | `lex::tests::a_single_underscore_is_the_wildcard_and_a_dunder_is_not` | `_x` is a name |
| | `lex::tests::no_terminal_reads_a_range` | `1..5` is one name |
| | `lex::tests::symbol_rejects_reserved_words_at_a_word_boundary` | `rule-x`, `rule.x` are names |
| `ein-ir` `dump_goldens` | `dump_then_parse_is_a_fixed_point` | `range-in-match` parses: the interim list |
| `ein-ir` `fuzz_properties` | `every_seed_and_finding_parses_the_way_it_was_recorded` | `fuzz_seeds.txt` |
| `ein-ir` `grammar_decisions` | `a_keyword_is_a_whole_word_in_every_place` | its 435 `keyword-1` parses read the name `keyword-1`, overriding Q-M1e2.17 (i)(a) |
| | `the_documented_ambiguities_keep_their_decisions` | `grammar_decisions.txt` |
| `ein-ir` `ir_semantics` | `the_documented_ambiguities_resolve_as_recorded` | 9 of 78: `(rule-x A)`, `(rule.x A)`, `(= a b c)`, `(a _x)`, `(a 1..5)`, `(a 1..*)` and `(a 1..)` parse; `(a 007)` and `(a -0)` keep their spelling |
| `ein-ir` `load_semantics` | `the_config_coercion_table_is_stable` | `config_coercion.txt`: `got Int(value=1)` → `got Atom(name='1')` |
| | `the_load_negative_fixtures_are_byte_identical` | `relation_parameter_int` loads |
| `ein-render` `corpus_shapes` | `every_corpus_rendering_reproduces_its_digest` | `corpus_shapes.md5` (below) |
| `ein-render` `golden_trace` | `every_real_solve_reproduces_its_golden` | `trace_one-hypothesis.md`, `trace_ambiguous.md`: both `branching/12`, its universe and N 25 → 51 |

`dot_wellformed`, `id_order_invariance`, `jobs_invariance`, `lever_invariance`, `route_invariance`,
`corpus_cores`, `stdlib_coverage` and every other target pass.

**The goldens, named before they move.** Blessed on the snapshot (`EIN_BLESS=1 cargo test --workspace
--no-fail-fast`, `build/s1f.5.4/proto-bless.log`, 2026-11-08, 16 min 57 s; diffed against an untouched
`build/s1f.5.4/base`), seven goldens move, and the register's `[case.region]` blocks (45 lines). This is the
prototype's list; the proposed scoping ([§ The register](#the-register-12-rows-move), [§ The fixtures](#the-fixtures)) changes the `ein-bugs/` witnesses' rows again.

| golden | what moves |
|---|---|
| `ein-cli` `corpus_exits.txt` | 20 of 1 302 cells: `relation_parameter_int`'s `saturate` and `solve`, `range-in-match`'s two, `arg-kinds`' `saturate` and the two `claim-int-name` `solve` cells 1 → 0; the eight `ein-bugs/` witnesses' `test` cells and five of their `solve` cells 0 → 1 |
| `ein-render` `corpus_shapes.md5` | 537 lines out and 638 in, over 30 files (`build/s1f.5.4/shapes_by_file.txt`): the 22 corpus files above that move; `arg-kinds` and `range-in-match`, one `ir[parse]` row each → 41 rows; and nine whose digest moves and whose answer does not, a digit now an atom in the tree: `syntax/relation-decls` (`(relation R4 7 T)`), `syntax/is-a-shapes`, `int-goal-binding`, `mixed-type-hypothesis`, `str-in-hrule`, W24's `form-in-guard`, `guard-before-binder` and `kwpair-in-guard`, and `tests/stdlib/elim/01` and `/02` |
| `ein-ir` `grammar_decisions.txt` | seven rows: `(rule-x A)`, `(rule.x A)`, `(= a b c)`, `(a _x)`, `(a 1..5)`, `(a 1..*)` and `(a 1..)` parse |
| `ein-ir` `fuzz_seeds.txt` | the seed `(a 1..5 :k 1..* :s "x\ny" :w _ :v ?v)` parses |
| `ein-ir` `config_coercion.txt` | `(config :lattice-sanity-check 1)`: `got Int(value=1)` → `got Atom(name='1')`; `… 1..5`: the parser's line → the loader's, `got Atom(name='1..5')` |
| `ein-render` `trace_one-hypothesis.md`, `trace_ambiguous.md` | `branching/12`: the universe and N, 25 → 51 (238 lines in the second) |

The 27 tests the bless leaves failing are assertions restated by hand where the name rule lands: the lexer's and
printer's unit tests (the hand-written lexer's go with it), the int-against-string cases of `rule_semantics`, `compile_semantics` and `compile.rs`, `ir_semantics`' table, the
interim list's four readers, `alive_invariant`'s two lists, the refused set, the register's rows, the load-negative
fixture, the slow flags and the diagnostics check.

### The register: 12 rows move

`wrong_answers::every_row_answers_what_the_register_records`, and `register_region`'s `[case.region]` blocks for ten:

| row | what moves | proposed |
|---|---|---|
| `W7/binding-key-layout`, `W15/binding-key-layout`, `W15/binding-key-layout-swapped` | `binding-key-both` and `-swapped`: H 1 → 21, R past the budget, `--maximal` lists every candidate | **keep, scoped A** (`(solve :no-hypothesis (edge noted) …)`): H `(holds 2)`, `(holds Y)`, R four states, the oracle the same; `now`, `right`, `[case.region]` and both files' `:expect` restated to them. W15's point stays visible (`Y`, written only inside `(src Y)`, is a candidate), and W7's (root's four `noted` facts in either order) |
| `W16/int-and-string`, `W16/string-then-int` | `1` and `"1"` are one name, so each file's two activators are one fact: the case cannot be written | **keep, scoped C** (`(solve :no-hypothesis (edge holds noted) …)`): R {∅} and `now` exactly as recorded today, and each file's `:expect` holds unchanged (its six facts are root's three); `right` and `[case.region]` restated (one value: `(holds 1)`, `(noted 1 1)`, `(noted 2 1)`), the note saying the case is unwritable since this stage; [S1f.5.2](s1f.5.2_strings_are_natural_language.md) then refuses `"1"` as a value |
| `W16/activator-all-int` | the refusal's line gains `[activator 1]` | **keep**, `now.refused` restated; `right`, a refusal, unchanged. The witness is now its symbol control with `1` for `a` |
| `W20/range-in-match` | refused at the parse → loads | **`right` restated**: `1..3` is a name and the rule a pattern over it, which no fact at root holds; H `(hit 1..3)`, `(hit 2)`, `(n 1..3)`, R the whole cube of 8, `(hit r)` exactly where `(n 1..3)` holds, the oracle the same; `now` that `solve`; fixed; the witness's header restated |
| `W21/typed-blind-solve` | N's cores, 25 → 51 (`branching/12`) | **`now` re-taken**, F8's accepted move |
| `W28/claim-int-name`, `-solve` | the claim holds, exit 1 → 0 | **`right` restated**: one name, the claim held, exit 0; the note saying the case is unwritable since this stage, and S1f.5.2 refusing `"1"` |
| `W28/classes-int-name` | 4 classes → 2; the candidate prints `(pick 1)` | **`right` and `now` restated**: 2 classes |
| `W34/assert-empty` (open, S1f.5.9's) | the candidate prints `(q @empty)` | **`now` restated** to whatever Q1 decides the name is |
| `W18/str-in-match`, `W18/str-in-assert` (S1f.5.2's) | `[case.region]` only (`str-in-assert` 4 → 32 states) | **re-banked** |

Alternatively a status `retired` (schema /14) for rows whose case cannot be written, keeping their record blocks and
running nothing. Not recommended: the register kept W1's lever rows as rows, restated, when their flags retired.

### The fixtures

| fixture | proposed |
|---|---|
| `binding-key-both`, `-swapped` (W7, W15), `-nested-only`, `-int-only` (W7's controls) | keep, **scoped A**, each `:expect` restated to its R (four states; `-int-only` two). Unscoped, every one of the eight `ein-bugs/` witnesses' `:expect` is refuted, and its `test` cell exits 1 with an empty stderr ([C3](#for-the-coordinator)) |
| `activator-text-int-and-string`, `-string-then-int` (W16), `-int-only`, `-string-only` (W16's controls) | keep, **scoped C**, each `:expect` unchanged and holding; S1f.5.2 refuses the string ones |
| the four `corpus/fuzz_findings/` seeds | keep as seeds, scoped as their promoted witnesses (A, C), so that the cost stays where it is: unscoped, three `ein-bugs/` entries cross the corpus's 1 s slow mark (`binding-key-both` 5.2 s of cells, `-swapped` 5.2 s, `-nested-only` 4.5 s: `the_slow_flag_still_describes_the_sweep`), and the two `binding-key-layout` seeds stop at the budget too |
| `activator-all-int` (W16) | keep; its line restated |
| `claim-int-name`, `-solve`, `classes-int-name` (W28) | keep; headers say what they answer now; S1f.5.2 refuses `"1"` |
| `range-in-match` (W20), `syntax/arg-kinds` | parse again; `REFUSED_UNTIL_S1F_5_4` emptied and its test deleted, as S1f.5.3 left it; `arg-kinds`' 342 542 states are the round trip's cost ([Q3](#q3--arg-kinds-and-the-round-trip)) |
| `broken/load/relation_parameter_int` | [Q2](#q2--relation_parameter_int) |
| `ein-bugs/int-goal-binding` | [Q4](#q4--a-name-of-digits-in-summaryjson) |

### The questions for the user

#### Q1 — the synthetic names

`@empty`, `<nested>`, `@params` and `<query>` are names a program can now write
([the finding](#the-finding-two-synthetic-names-become-spellable)).

- **(a) Recommended: rename the four out of the name space no later than the name rule lands** (the swap), to
  text the list cannot spell (a character it
  excludes, such as `#`: `#empty`, `#nested`, `#params`, `#query`). Nothing moves but the spelling of W34's leaked
  candidate, `(q "#empty")`, which S1f.5.9 removes.
- (b) Leave them until their producers go (S1f.5.9 for `:assert ()`, S1f.5.1 for `(_ a)`, S1f.5.5 for `()` as a
  value), the collision filed as a register row meanwhile.
- (c) Refuse the four spellings at load.

**Decided 2026-11-08 by the coordinator, on this step's recommendation: (a)**, open to the user's review at
S1f.5.5's stop. The four are renamed out of the name space, in characters the name rule refuses, no later than the
swap: a prerequisite of it, held by a test, in its own commit
([T1f.5.5.8](s1f.5.5_the_grammar.md#task-t1f558--the-swaps-prerequisite-the-loaders-names-out-of-the-name-space)).
W34's `now` is restated there, since its candidate's printing moves. **Done 2026-11-08**: `#empty`, `#nested`,
`#params` and `#query`, with the generated parser's `@list` as `#list`
([S1f.5.5 § T8 record](s1f.5.5_the_grammar.md#t8-record-2026-11-08)).

#### Q2 — `relation_parameter_int`

The load-negative fixture of an int binding a relation parameter loads now: `(mirror 3)` names the relation `3`.

- **(a) Recommended: retire it**, its `.expected` and its corpus entry. The refusal it pinned is int-only code T3
  retires; the nested-fact shape has its own fixtures (W24, [S1f.5.18](s1f.5.18_an_activator_binds_a_nested_fact.md)).
- (b) Move it to a positive group, as *a relation named `3`*.

**Decided 2026-11-08 by the coordinator, on this step's recommendation: (a)**, open to the user's review at
S1f.5.5's stop. Retired in the swap's commit
([T1f.5.5.4](s1f.5.5_the_grammar.md#task-t1f554--the-swap)).

#### Q3 — `arg-kinds` and the round trip

`arg-kinds` writes 342 542 state files in the prototype (S1f.5.3 measured 112 793 before the interim), each reloaded
by `answer_round_trip`. **Recommended: (a)** leave the program as written and let the round trip read it (it took
18.5 s for `solve` with `--dump-answer` here), as S1f.5.3's interim said; (b) scope it with `(solve :no-hypothesis
(r))`, which the fixture's point (the node kinds as written) does not need: R is then {∅}, one state, no candidate
(`build/s1f.5.4/scoped/arg-kinds.scoped.ein`).

**Decided 2026-11-08 by the coordinator, on this step's recommendation: (a)**, open to the user's review at
S1f.5.5's stop: the program is kept as written, the swap measures the gate's cost, and the fixture is scoped only
if the gate visibly slows. **What the re-plan adds** (2026-11-08): the program this question measured is the one
written today, which the paper grammar refuses for its `"a string"`, `_` and `?x`. After the swap the corpus
carries [Q-M1f.2](../open_questions.md#q-m1f2--the-55-files-the-grammar-refuses-that-todays-parser-accepts) (a)'s
rewrite, and on this step's prototype its `ein solve` stops with *a state's commitments pass 2^128*, exit 1,
writing no state ([§ Notes](#notes)). S1f.5.5's stop puts it to the user, recommending (b) for the rewrite
([T1f.5.5.9](s1f.5.5_the_grammar.md#task-t1f559--every-difference-attributed-and-one-review-stop)).

#### Q4 — a name of digits in `summary.json`

`summary.json`'s query rows render a binding whose text is digits as a JSON number (`region_answer.rs`, `binding`;
M1a S1a.6.6's regression, `ein-bugs/int-goal-binding`). With digits as names, `(p 007)` and `(p 7)`, two names, both
render `{"x": 7}` (`build/s1f.5.4/scratch/json-binding.ein`).

- **(a) Recommended: every name is a JSON string**, landing with the name rule, and `int-goal-binding`'s note restated: a name of digits,
  and one wider than an i64, is a string. Its summary moves (`8`, `-7`, `99999999999999999999999` become strings).
- (b) Keep the number for a canonical run of digits only (`7`, not `007` or `-0`); two spellings would still not
  collide, but a name would be a number on one surface.

**Decided 2026-11-08 by the coordinator, on this step's recommendation: (a)**, open to the user's review at
S1f.5.5's stop. Every name is a JSON string, with the swap; if the summary's schema id has to move, the swap's
commit names the move, and `int-goal-binding`'s note is restated
([T1f.5.5.4](s1f.5.5_the_grammar.md#task-t1f554--the-swap)).

#### For the coordinator

Taken by the coordinator on 2026-11-08: C1 and C2 are the rules S1f.5.5's attribution reads by
([T1f.5.5.9](s1f.5.5_the_grammar.md#task-t1f559--every-difference-attributed-and-one-review-stop)), and C3's two
findings are parked in [§ Notes](#notes).

- **C1.** In the swap's attribution, `_x`, `__`, `_-1` and `?_x` (a name and a variable) are the name rule's, S1f.5.4's
  (*any of them first*), and the bare `_` and `?_` [S1f.5.1](s1f.5.1_bare_underscore_refused.md)'s, whose table lists
  the four as its own. No corpus program writes one, so no move of this list depends on it. Recommended.
- **C2.** No move in this list is a keyword's: the prototype kept today's keyword characters
  (`:[a-z][A-Za-z0-9_-]*`, ended at a separator), so at the swap every keyword difference is
  [S1f.5.11](s1f.5.11_closed_keywords.md)'s or the paper grammar's own.
- **C3.** Two things found here are not this stage's: `ein test` exits 1 with an empty stderr on a refuted claim (every
  `test` cell; `ein solve` writes one line), which `corpus_cli::every_refusal_carries_a_diagnostic` would catch on a
  corpus cell and which no corpus cell reaches today or with the witnesses scoped as proposed; and the state route's *pass 2^128* limit,
  reached by three M1e2 probes today and by S1f.5.5's `programs/arg-kinds.ein` in the prototype. To be parked where
  the coordinator files out-of-phase findings.

## Notes

Parked on 2026-11-08 by the coordinator, from [C3](#for-the-coordinator), as findings outside this stage and
outside P1f.5's rulings. Neither is a task here.

- **`ein test` exits 1 with an empty stderr on a refuted claim**, on every `test` cell, where `ein solve` writes one
  line. `corpus_cli::every_refusal_carries_a_diagnostic` would catch it on a corpus cell, and no corpus cell
  reaches it today, nor with the witnesses scoped as [§ The register](#the-register-12-rows-move) proposes. The
  prototype's unscoped `ein-bugs/` witnesses reached it eight times.
- **The state route's *a state's commitments pass 2^128: the state route cannot count them*** stops `ein solve`,
  exit 1, where k_c needs more than 128 bits. Three M1e2 probes reach it today, and on this step's prototype so
  does S1f.5.5's [`programs/arg-kinds.ein`](s1f.5.5_parser_probe/programs/arg-kinds.ein), which is
  [Q-M1f.2](../open_questions.md#q-m1f2--the-55-files-the-grammar-refuses-that-todays-parser-accepts) (a)'s rewrite
  of `syntax/arg-kinds` (`build/s1f.5.4/protobin/ein solve` on it: exit 1; `saturate`: exit 0; 2026-11-08). So the
  limit is on the swap's path, through that one fixture: S1f.5.5's stop shows it with [Q3](#q3--arg-kinds-and-the-round-trip),
  and the limit stays parked.
