# S1f.5.1 — W29: a bare `_` is reserved and refused, and `_` is valid anywhere in a name

**Phase:** [P1f.5](README.md) (The no-owner items)
**Was S1e2.5.1**, in M1e2's P1e2.5, until 2026-11-08 ([§ The ids](README.md#the-ids)).
**Estimate:** 1.25 days, re-taken on 2026-11-08 on the user's re-plan ([P1f.5 § The order](README.md#the-order));
it was 2.
- A quarter of a day to trace F3's cause in the matcher, before `Slot::Any` goes (T1f.5.1.3).
- A quarter of a day for `_` held in every position, in the grammar's table (T1f.5.1.3).
- A quarter of a day to retire what only `_` used (T1f.5.1.3).
- Half a day for the pages (T1f.5.1.4).
- Re-taking W29's rows, the prototype, every move and the real programs (T1f.5.1.1, half a day), and the
  refusal, the printer, the fixtures and the register (T1f.5.1.2, half a day) are
  [S1f.5.5](s1f.5.5_the_grammar.md)'s since the re-plan: its attribution sweep and review stop
  (T1f.5.5.9) and its swap (T1f.5.5.4).

**Status:** specified 2026-11-06 on the user's ruling of that day
([Q-M1e2.22](../../m1e2_correctness/open_questions.md#q-m1e222--does-ein-need-a-wildcard)), and revised 2026-11-07 on the
user's rule for a name ([S1f.5.5](s1f.5.5_the_grammar.md#the-name-rule)): a run of underscores is a
name, and only a bare `_` and `?_` stay refused.
**Not started.** No engine code, test, golden or fixture has moved.
**Re-planned on 2026-11-08, on the user's ruling** ([P1f.5 § The order](README.md#the-order)): *"It is the same
work done twice, why not switch to grmtools first and remove hand-written parser and then verify requirement met
(strings, names, etc...) ?"* The refusal is the generated parser's (bare `_` is no `NAME` and `?_` no `VAR` in
[the paper grammar](s1f.5.5_paper_grammar.md#the-grammar-by-position)), and nothing edits the hand-written lexer or
parser for it. This stage's step 1 is done inside S1f.5.5's attribution sweep and its one review stop
([T1f.5.5.9](s1f.5.5_the_grammar.md#task-t1f559--every-difference-attributed-and-one-review-stop)), and its
refusal, printer, fixtures and register rows land in S1f.5.5's swap
([T1f.5.5.4](s1f.5.5_the_grammar.md#task-t1f554--the-swap)), credited to this stage's ruling. **W29's six rows
are fixed in the swap's commit**: each `right` is the refusal, which the grammar makes at the parse, so none waits
for a task here. What is left, T1f.5.1.3 and T1f.5.1.4, runs after the swap, on the generated parser.
**Depends on:** S1f.5.5's swap, which lands the refusal.

## The case

Everything here was taken on 2026-11-06 at `916d97ac`, on the release binary,
with `ein saturate FILE --dump` and `ein solve FILE`, on throwaway probes that
step 1 banks.

**`_` has never had a use of its own.** The grammar has named it since 2026-05-18
(`c44ccfb5`, *head / arg wildcard in patterns*). No engine matched it as
documented until W20 (`eddea251`, 2026-11-05). No rule under `examples/`,
`tests/` or `stdlib/` wrote it (711 rule forms, Q-M1e2.13, 2026-11-04). The only
programs that use it outside W20's and W24's witnesses and the probes are two
node-kind fixtures added on 2026-08-20 (`a756d5d3`), which write it in a fact,
where it is dropped.

**What it does today:**

| position | today | settled by |
|---|---|---|
| argument in a rule's or hrule's `:match`, at any depth, under `absent` too | matches any value, binds nothing | W20, on [Q-M1e2.13](../../m1e2_correctness/open_questions.md#q-m1e213--what-does-a-wildcard-or-a-range-in-a-rules-pattern-mean) (a) |
| as a relation, `(_ a)`, in a rule | refused at load | [Q-M1e2.14](../../m1e2_correctness/open_questions.md#q-m1e214--how-does-a-head-wildcard-match-every-relation) (c) |
| in a guard, or as a whole premise | refused at load | W24 |
| in `:expect` | refused at load | S1c.1.2's load error |
| argument in a rule's `:assert` | stops the run at the first firing; a rule that never fires loads (G-14) | S1e2.2.18 |
| argument in an hrule's `:assert` | proposes nothing, in silence (D-8) | none |
| in a query's goal, `(holds _)` | accepted, and suggests `:expect (model (holds _))`, which the loader refuses | none |
| **fact argument: `(r _ a)`, `(r (f _ a) c)`** | **dropped in silence: `(r a)`, `(r (f a) c)`** | **W-22** |
| **activator argument: `(q _)`** | **dropped in silence: `(q)`** | **W-22** |
| relation of a nested form in a fact: `(h (_ b))` | kept, as if `_` were a name | none |
| top-level `(_ a)`, `(relation r _ T)` | refused, exit 1 | parser, loader |

**Two corpus programs write it in a fact**, where the inventory of 2026-11-06
names one:

| program | written | stored |
|---|---|---|
| [`syntax/arg-kinds.ein`](../../../examples/syntax/arg-kinds.ein) | `(r _ ?x)` | `(r "?x")` |
| [`syntax/is-a-shapes.ein`](../../../examples/syntax/is-a-shapes.ein) | `(is-a _ T)` | `(is-a T)` |

**F3: `_` can report a larger core than a variable used once.** With the slot
written `_` and then `?w0`:

```lisp
(relation holds T) (relation hit T) (relation big T) (relation big2 T) (relation big3 T) (relation small T) (relation c T)
(big b0) (big2 b0) (big3 b0)
(rule m1 () :match (and (big ?x) (big2 ?x) (big3 ?x)) :assert (holds ?x))
(rule w () :match (holds _) :assert (hit w))
(rule k () :match (and (hit w) (c x)) :assert (false))
(rule s () :match (small ?x) :assert (holds ?x))
(c x) (small s0)
```

| the slot | root's core |
|---|---|
| `_` | `(big b0) (big2 b0) (big3 b0) (c x)` |
| `?w0` | `(c x) (small s0)` |

The same holds at a dead node, where `(c b0)` is a candidate: four facts against
two. The likely cause, inferred from these runs and not traced in the code: `_`
binds nothing, so a second match looks identical to the first and its derivation
is not recorded. Facts, candidates and R agreed between the two on every other
probe: seven shapes of `saturate`, an hrule, W20's three witnesses, and a
non-monotone program the oracle agrees on (8 states).

## The ruling

The user, 2026-11-06, on Q-M1e2.22: *"I agree, simple `_` reserved and refused
because of potential ambiguity, but remain valid in any place in atom and variable
names `_a_in_b_` `_______01` `A________B`"*.

This reverses [Q-M1e2.13](../../m1e2_correctness/open_questions.md#q-m1e213--what-does-a-wildcard-or-a-range-in-a-rules-pattern-mean)
(a) for `_` (its ruling on a range stands) and absorbs
[Q-M1e2.14](../../m1e2_correctness/open_questions.md#q-m1e214--how-does-a-head-wildcard-match-every-relation)
(c), and W20's `_` rows move with them.

## What it changes

*Since the re-plan of 2026-11-08 the grammar, the refusal, the printer, the register and the fixtures are S1f.5.5's
swap, credited to this ruling, and what only `_` used is this stage's, after it.*

**The grammar.** `_` stays a token only so the refusal can say why.

```text
NAME ::= NameChar+, except a reserved word or a bare _
VAR  ::= '?' NameChar+, except ?_
```

| written | today | after |
|---|---|---|
| `_a_in_b_`, `_______01` | refused | **a name** (the ruling) |
| `?_a_in_b_`, `?_______01` | refused | **a variable** (the ruling) |
| `A________B`, `?A________B` | a name, a variable | unchanged (the ruling) |
| `Old_Gold`, `x_`, `a-_`, `?x_y` | a name, a variable | unchanged |
| `__closed__`, `__d` | a name | unchanged, and a `__dunder__` keeps its kernel meaning |
| `_x`, `_1`, `_-a`, `?_x` | refused | a name, a variable |
| **`_`, in every position above** | as in the table | **refused** |
| `__`, `___`, `_-`, `_*`, `?__` | refused | a name, a variable (default 1, revised) |
| `?_` | refused | refused, as `_` is (default 1, revised) |
| `1_a`, `1..5_a` | refused | refused here (default 2); `1_a` is a name after S1f.5.4 |
| `1_`, `-1_`, `_-1` | `(r 1)`, `(r -1)`, `(r -1)`, in silence | the name each spells, once S1f.5.4 has landed |
| `"_"` | the name `_`, printed `"_"` | unchanged here (default 3), and refused by S1f.5.2 |
| `:_k` | refused | unchanged (default 4) |

**The refusal.** The parser refuses the token, so every command that reads a
program refuses it, `ein render program` and `ein saturate` included, and macro
bodies and imports with them. One line, naming the place. Proposed wording, for
the user to see in step 1:

```text
FILE:L:C: _ is reserved and names nothing: where any value will do, write a variable used once
```

**The printer.** `ein_core::text::is_symbol` is the second statement of `SYMBOL`,
and moves with it, so `_a` prints bare and still reloads to the same value
(`ein-cli/tests/answer_round_trip.rs`).

**What only `_` used.** `Slot::Any` and `Node::Wildcard` go, with their arms in
`compile.rs`, `match_.rs`, `firing.rs`, `oblgen.rs`, `saturator.rs`, `detector.rs`,
`shape.rs`, `macros.rs`, `from_ir.rs` (`Found::WildHead`, and the load lines for a
head, a guard and a premise), `expect.rs`, `dump.rs`, `ir_dot.rs` and `dot_util.rs`.
42 lines name one of the two (`grep -rn 'Slot::Any\|Node::Wildcard' --include='*.rs'
ein.rs/crates | grep -v /tests/ | wc -l`, 2026-11-06).

**The register.** Schema `/13` if a field needs it.
- **W29**, new, each row's `right` the refusal: `W29/fact-arg`, `W29/is-a-arg`,
  `W29/nested-fact-arg`, `W29/activator-arg`, `W29/goal-arg`, and `W29/core-two-ways`
  for F3. Filed `open` by [S1f.5.0](s1f.5.0_the_known_wrong_answers_filed.md), then `fixed` by the refusal.
- **W20**: `wild-in-match`, `wild-in-absent` and `wild-in-hrule` matched any value,
  and their `right` moves to the refusal. Their `-twin`s, already written with
  `?any`, are what the refusal says to write, and keep their R.
- **W20/wild-in-head, W24/wild-in-guard, W24/bare-premise**: still refused, with the
  new line. `W20/range-in-match` does not move.

**The fixtures.** `arg-kinds` and `is-a-shapes` lose their `_` line, and one new
parse-negative fixture holds the refusal. W20's three matching witnesses and their
corpus entries move to refused.

**The goldens and tests, named before they move** (step 1 completes this list):
`corpus_shapes.md5`'s `ir[parse]` rows; `ein-ir/tests/golden/grammar_decisions.txt`
and its test; `ir_semantics.rs`'s `a_wildcard_or_var_head_survives_lowering_and_a_round_trip`
and its grammar cases; `cli_semantics.rs`'s refusal test; `fuzz_properties.rs`;
`lex.rs`'s unit tests.

**What it closes in the inventory.** W-22, and F3. The `_` half of G-14 and of
D-8, and the goal row above, since `_` no longer reaches them. G-6 stays: a
variable or a range written in a fact is still a name, as `(r ?x)` shows, and
that is its own question.

## Defaults

Each holds unless the user says otherwise, and step 1 shows it working.

1. **Revised 2026-11-07: only `_` and `?_` are refused.** This page had a name hold a letter or a digit,
   so that `__` and `___` stayed refused. The user's rule makes any run of name characters a name.
   `?_` stays refused because two uses of it in one rule would bind the same value, the opposite of what
   the spelling suggests.
2. **A number does not end where a name could begin.** Once `_a` is a name, `1_a`
   would otherwise split into `1` and `_a` where it is refused today
   ([F2](README.md#f2-a-token-needs-no-separator)). This stage keeps `1_a` refused until
   [S1f.5.4](s1f.5.4_digits_are_names.md) makes digits names, and then `1_a` is one. The rest of
   F2 is [S1f.5.5](s1f.5.5_the_grammar.md)'s.
3. **A string still names what it spells, in this stage.** `(r "_")` is the name `_`,
   as S1e2.2.18 holds, and prints quoted because the bare spelling does not lex.
   [S1f.5.2](s1f.5.2_strings_are_natural_language.md), which runs after this one,
   refuses a string as a name altogether.
4. **A keyword is unchanged.** The ruling names atoms and variables.

*Since the re-plan of 2026-11-08 defaults 2 and 3 are moot.* The name rule (S1f.5.4's ruling) and S1f.5.2's
refusal land at the same swap as this stage's, so `1_a` is one name from the first commit that refuses `_`, and
`"_"` is refused there too.

## The method

### Task T1f.5.1.1 — W29 re-taken, every move listed, real programs shown

**Done inside S1f.5.5 since the re-plan of 2026-11-08**: its attribution sweep credits every `_` difference to
this ruling, and its one review stop shows what step 5 below lists
([T1f.5.5.9](s1f.5.5_the_grammar.md#task-t1f559--every-difference-attributed-and-one-review-stop)). Step 3, F3's
trace, moved to T1f.5.1.3. The task as it stood is that sweep's brief for this ruling:

1. Re-take the case on a fresh build, release and debug.
2. Start from W29's six rows, which [S1f.5.0](s1f.5.0_the_known_wrong_answers_filed.md) filed `open` on
   2026-11-08, each witness under `examples/ein-bugs/wrong-answers/` and a corpus entry, each `now`
   taken at `ec004a48` (the engine `916d97ac`'s), release and debug alike, every one the case's answer
   above:

   | row | witness | `now`, today |
   |---|---|---|
   | `W29/fact-arg` | `underscore-fact-arg.ein`, `(r _ a)` | `solve --minimal`: root's state `(model (r a b) (r a))` |
   | `W29/is-a-arg` | `underscore-is-a-arg.ein`, `(is-a _ T)` | `solve --minimal`: R {∅}, no candidate; root holds `(is-a T)` |
   | `W29/nested-fact-arg` | `underscore-nested-fact-arg.ein`, `(r (f _ a) c)` | `solve --minimal`: `(model (r (f a) c))` |
   | `W29/activator-arg` | `underscore-activator-arg.ein`, `(q _)` | `solve --minimal`: root holds `(q)` and no `hit` |
   | `W29/goal-arg` | `underscore-goal-arg.ein`, `(query :goal (holds _))` | `solve`: one class, no claim |
   | `W29/core-two-ways` | `underscore-core-two-ways.ein`, F3 verbatim, and its `-twin`, `?w0` | `solve`: R empty, root's core four facts; the twin's two |

   Each row's `right` is the refusal. Two rows record what the refusal replaces beside it, the
   register's schema `/13`: `core-two-ways` its core, and `goal-arg` its query's class. The fix moves
   each `now` to `refused = "kb load error: …"` and drops those two fields, which a refused `now`
   may not record. `ein-infer/tests/alive_invariant.rs`' `SEEDED` names `underscore-nested-fact-arg`'s
   `a`, which the refusal takes out of the corpus's loaded programs, so its line goes in the same
   commit. Every move the fix makes must be one of these rows or step 4's list.
3. Trace F3's cause in the matcher, and say whether it reaches anything `_` does
   not (a rule with no variable in that slot cannot, by construction).
4. Prototype the grammar, the refusal and the printer on a `git archive` snapshot
   under `build/`. Sweep every `.ein` under `examples/`, `tests/`, `stdlib/`,
   `corpus/` and `plans/` through `solve` and `saturate`, against today's binary,
   and run `cargo test --workspace` on the snapshot. List every move.
5. **Stop and show the user real programs before any edit** (the standing rule on
   syntax): the two fixtures and W20's three witnesses as written and rewritten,
   `zebra2.ein` with names such as `_Old_Gold_` and `?_h_water_`, and the table
   above with each row computed. Its R must not move under the renaming.

**Stops if** any corpus program outside W20's, W24's and W29's witnesses and the two
fixtures moves, or if a rename moves `zebra2`'s R.

### Task T1f.5.1.2 — refused, and valid in a name

**Moved to S1f.5.5's swap on 2026-11-08** ([T1f.5.5.4](s1f.5.5_the_grammar.md#task-t1f554--the-swap)): the
grammar's refusal, the printer, the fixtures and the register, W29's six rows flipped to `fixed`, and W20's and
W24's `_` rows restated, as listed. `arg-kinds` and `is-a-shapes` are Q-M1f.2 (a)'s rewrites, without the
dropped fact.

### Task T1f.5.1.3 — F3 traced, `_` held in every position, and what only `_` used

After the swap, on the generated parser, after S1f.5.5's T5. Three steps:

1. **F3's cause traced** in the matcher (T1f.5.1.1's step 3, moved here), while `Slot::Any` still exists: say
   whether it reaches anything `_` does not (a rule with no variable in that slot cannot, by construction). A
   reach is a wrong answer, filed as a register row before anything moves.
2. **A test per position**: `_` and `?_` refused in every position of [the table](#what-it-changes), on every
   command that reads a program, as rows of the grammar's table S1f.5.5's T5 builds.
3. **`Slot::Any` and `Node::Wildcard` retired**, as a separate commit, so that no answer moves in it. Every
   golden unmoved by this step, byte for byte. The loader's refusal lines for `_` (`Found::WildHead` and the
   lines for a head, a guard and a premise) went in S1f.5.5's T5; this step retires the rest of the 42 lines.

### Task T1f.5.1.4 — the pages

After the swap, beside S1f.5.5's T7.

`00_ebnf.md` (the tokens and fact 2), `01_grammar.md` (the `WILDCARD` row and
§ Rules), `02_patterns.md` (the construct table's two rows, and its premise and
guard sentences), `04_dot_rendering.md`, `06_reserved_names.md`,
`defined_behaviour.md` (§ 4, § 4.8, § 4.9), `README.md`, `examples/README.md`. The
two Q entries the ruling reverses say so. W-22 and F3 marked in
[the no-owner inventory](../../m1e2_correctness/inventory_2026-11-06_no_owner.md).
`docs/history/` is a record, and does not change.

## Acceptance

**Since the re-plan of 2026-11-08 the items the grammar settles are
[S1f.5.5's acceptance](s1f.5.5_the_grammar.md#acceptance)**, held at its swap: `_` and `?_` refused in every
position, `_a_in_b_`, `_______01` and `A________B` names and variables, `1_a` one name, W29's rows `fixed` and
W20's and W24's answering their new `right`, and `zebra2`'s R unmoved under the renaming. This stage's own, after
the swap:

- A test holds each position of the table, `_` and `?_` refused in it, on every command that reads a program.
- F3's cause is named, and anything it reaches beyond `_` is a register row.
- `Slot::Any` and `Node::Wildcard` are gone, and that commit moved no golden.
- `./run_tests.sh` is green.

## Notes

- A prototype of the lexer, refusal and printer was built on a snapshot on
  2026-11-06, and removed unrun when the user asked for plans first. It shows the
  change fits in about 60 lines before the retirement, and shows nothing else.
