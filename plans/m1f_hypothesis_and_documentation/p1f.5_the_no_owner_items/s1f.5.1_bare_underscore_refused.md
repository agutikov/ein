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
**Not started** until the swap, which moved its refusal, fixtures and rows (below). **Running since 2026-11-09**
([§ T1f.5.1.3 record](#t1f513-record-2026-11-09)): T1f.5.1.3's step 1 done, F3's cause traced to a rule application's
identity, which reaches a disjunction too, filed as `W39/or-one-key`, `open` and deferred to F22 (`728e716a`); step 2,
`_` and `?_` held refused in every position (66 rows of the grammar's table) and on every command (`307021db`);
step 3, `Slot::Any` and `Node::Wildcard` retired, no answer and no golden moving.
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

### T1f.5.1.3 record, 2026-11-09

**Step 1, F3 traced: one commit, on `1e751131`** (S1f.5.2's T1f.5.2.4). F3's cause is the identity of a rule
application, and it reaches one shape `_` does not: a disjunction whose disjuncts bind the same registers. That is a
wrong answer, filed as `W39/or-one-key`, `open` and deferred to F22. No answer of any program moved.

**The trace**, read on `1e751131` while `Slot::Any` still exists:
- **The drop.** `Saturator::enqueue_binding` keys each match by `(BindingKey, GuardSetId)`, where `BindingKey` is
  the rule, the activator and the register file (`firing.rs`). A match whose key `seen` holds, or whose `BindingKey`
  `engine.fired` holds, returns before it is enqueued. Only `apply` fires a match redundantly, and only
  `record_alternative`, which `apply` calls, offers its premises to the store as an alternative justification. So a
  dropped match's derivation is never recorded, and the core search, which reads the recorded justifications, never
  sees it. The comment above `encode_guards` says the collapse is meant: *two `(or …)` disjuncts asking the same
  negative question collapse in `seen`*.
- **Within one disjunct, nothing is lost but through `Slot::Any`.** A positive step has a fixed relation (a symbol;
  a relation variable is a constant its activator binds, or refused at load since W24), and each slot is a `Const`, a
  `Reg` (in the key), a `Nested` pattern of the same, an `Opaque` slot that never matches, or `Slot::Any`, which
  `Matcher::unify` matches against any value, binding nothing. So two matches of one disjunct under one key matched
  the same facts unless a premise holds `Slot::Any`. F3's `(holds _)` compiled to a plan with no register at all:
  every match keyed alike, `(holds s0)`'s was dropped as seen after `(holds b0)`'s, and `(hit w)`'s one recorded
  derivation rests on `big`, `big2` and `big3`. That is the case's *likely cause*, confirmed in the code.
- **Across disjuncts, the reach.** A top-level `(or …)` is one plan whose disjuncts share their registers by name
  (`compile.rs`' `run`), and neither key names the disjunct: two disjuncts with no guards share a guard set, and
  `fired` has none. So a match through the second disjunct that binds every register as one through the first did is
  dropped the same way. `examples/ein-bugs/wrong-answers/or-one-key.ein`, one rule `:match (or (p ?y) (q ?y))`,
  reports root's core as `(big b0) (big2 b0) (big3 b0) (c x)`; its twin, the rule written as two rules, `(c x)
  (small s0)` (`ein solve FILE`, release and debug, 2026-11-09). The same holds at a dead node: with `(c x)` taken
  out and `k` reading `(c b0)`, N {(c b0)}'s core is four facts against two (`ein solve --dump-answer`, 2026-11-09).

**The census.** A copy of `1e751131` (`git archive`, `build/s1f.5.1/t3/f3/`, patched by `f3_patch.py`) logs each
match dropped at `seen` or `fired` whose premises, as a set, differ from those of the match kept under the same key,
with both disjuncts and whether the dropped one's has a `Slot::Any`; and with `F3_WILD=name` its builder makes `?name`
a `Node::Wildcard`, which is what the parser made of `_` in the same places until the swap.
- **F3 itself**, the twin of `W29/core-two-ways` with `F3_WILD=w0`: root's core is the four facts again, and the log
  names `cause=slot-any`, `(holds s0)` dropped beside `(holds b0)`; without the variable, the two facts and no line
  (`f3-demo.log`). The witness above logs `cause=other-disjunct`, `(q w)` beside `(p w)`.
- **Every corpus program** (515, `find examples tests stdlib corpus plans -name '*.ein' | wc -l` at `1e751131`)
  under `ein solve` and `ein saturate`: one program reaches it, `examples/ein-bugs/wrong-answers/wild-in-head-twin.ein`,
  whose `(or (holds a) (hit a))` binds no register, at the node that commits `(hit a)`; its R is two states, it
  reports no core, and no answer it prints depends on the drop (`f3-corpus.log`).
- **T12's 2 000 mutants** under `ein solve`: three, `m0044`, `m1489` and `m1738`, each that program unchanged in its
  first lines, the same drop (`f3-mutants2000.log`).
- **40 000 seeded mutants (seed 0) and 200 000 (seed 987654321)**, parsed, loaded and saturated in process
  (`f3_sweep.rs`, T1f.5.2.3's sweep with the input named in the log): no line, the outcomes T1f.5.2.3's on the same
  inputs (`f3-sweep.txt`, `f3-sweep2.txt`); `ein saturate` of the census copy, the same path, logs both demo causes.
- **No line names a third cause**, a drop within one disjunct with no `Slot::Any`, and none a key `fired` holds that
  no kept match had (409 s, 2026-11-09).

So F3's cause reaches nothing `_` does not but the disjunction, and with `_` refused since the swap the disjunction is
what is left.

**W39, filed** (`corpus/wrong_answers.toml`, the header's paragraph and one row): `W39/or-one-key`, class `core`,
`open`, owner F22, reviewed `deferred`, its `wrong` and `now` root's four facts, its twin's and its `right` the two;
the witness and its twin each a corpus entry (`solve`, `saturate`) with a header deriving the answer by hand and a
`:why` on every relation and rule, and a clause in `examples/README.md`'s `wrong-answers/` line. F22's page lists it.
The register reads **119 rows fixed, 22 open and 1 not-wrong** (`grep '^status' corpus/wrong_answers.toml | sort |
uniq -c`, 2026-11-09).

**Not fixed here.** Taken by default on the user's instruction of 2026-11-09 (*"just continue with default decisions
till the end of the phase"*): the row is F22's, as W25's never-offered derivation is by the user's ruling of
2026-11-05 on Q-M1e2.16 (2) and (5), and this stage, whose subject is `_`, does not move the saturator's dedup
([Q-M1f.13](../open_questions.md#q-m1f13--f3s-cause-reaches-a-disjunction-whose-row-is-it) (a), defaulted). The fix
the question prices as (b) is the disjunct in both keys, so the second disjunct's match fires redundantly and records
its alternative.

**What moved**, the two new programs' and nothing else: `corpus_exits.txt` (four cells, each exit 0) and
`corpus_shapes.md5` (82 rows, 41 a program), blessed with `EIN_BLESS=1 cargo test -p ein-cli --test corpus_cli
every_cell_reproduces` and `-p ein-render --test corpus_shapes`; and `alive_invariant`'s `SEEDED`, 27 → 28 rows,
the twin's `w`, which a rule asserts and no fact names (in the witness a rule named `w` makes it a name of the program).
`wrong_answers` and `register_region` hold the row.

**Step 2, `_` held in every position: one commit, on step 1's (`728e716a`).** A bare `_` and `?_` are refused at the
parse in every place of [the table](#what-it-changes) and wherever a name or a variable goes, on every command that
reads a program. No answer moved, and no register row.
- **The grammar's table** (`ein-ir/tests/grammar_pages.rs`), 66 rows at its end (190 → 256; `cargo test -p ein-ir
  --test grammar_pages`, 2026-11-09), each refused at the parse at its token, with S1f.5.1's note: `_` and `?_` as a
  fact's, a nested fact's, an `is-a` fact's, a stored negation's and an activator's argument (W-22's places), a
  nested fact's relation, a top-level form's head, a relation's name and signature, a rule's name and parameters, a
  premise's, a pattern's (under an `absent` too) and an hrule premise's argument and relation, a guard's argument, a
  `:match`, a conjunct, a disjunct and an `absent`'s premise, a conclusion, its relation and argument (a stored
  negation's in one too), an `(open …)`'s relation and an hrule's conclusion, a query's goal and `:over`, a claim's
  fact and `neq`, a region claim's fact, a solve's relation lists and `:hrules`, a config flag's value, a macro's
  name, parameters and body, an import's module, alias and symbols, a fact's `:rule` and `:using`, and a trace
  step's name and `:derives`. Each place writes the token the earlier rows did not: T5's sixteen `_` rows (`awk` over
  `grammar_table.txt`'s first 190, 2026-11-09) (a pattern's
  relation and argument, a guard's argument, a premise, a conclusion's argument, `:expect`) gain their `?_`, and the
  places they did not cover gain both.
- **Every command** (`ein-cli/tests/cli_semantics.rs`' `a_bare_underscore_names_nothing_in_every_place`): fifteen
  programs, one per place of the stage's table, each written with `_` and with `?_`, through `ein saturate`, `solve`,
  `oracle`, `render program`, `render rules`, `render constraints`, `render kb` and `kb save` (exit 1) and `ein
  test` (exit 2), each with the line at the token and the note; and a module whose macro body writes `_`, refused
  where it is imported, with the module's line, by `ein solve`, `oracle`, `test`, `render kb` and `kb save`, the
  commands that resolve a file-relative import.
- **Found outside the brief:** `ein saturate` resolves no file-relative import at all, where `solve`, `oracle` and
  `test` do: it loads with no base directory (`saturate.rs`, whose comment says it resolves against the working
  directory), so `(import mod)` is refused with *file-relative import needs a base directory (load from a file path)
  at None* from any directory, and `examples/broken/load/README.md` calls that line *not expressible as a file*.
  Not this stage's; reported to the coordinator.

**What moved**, the table's golden and nothing else: `grammar_table.txt` gains the 66 rows' reports, 460 lines
(`EIN_BLESS=1 cargo test -p ein-ir --test grammar_pages`), each the token's line, the note and the repair *Delete*.

**Step 3, `Slot::Any` and `Node::Wildcard` retired: one commit, on step 2's (`307021db`).** Nothing the parser builds
holds a wildcard since S1f.5.5's swap, and the node, the slot and every arm that read either are gone, each shown
unreachable first. **No answer moved, and no golden**: the gate below reads every golden as it was.

**How unreachable was shown** (T1f.5.2.3's harness). A copy of step 2's tree (`build/s1f.5.1/t3/probe/`) with a panic,
`S1-PROBE <id>`, at each of 30 sites (`probe_patch.py`, by text): the two roots, `Ast::push` of a `Node::Wildcard`
(the only way a node enters an `Ast`, whose arena is private) and the compiler's `Node::Wildcard => Slot::Any` (the
only place a `Slot::Any` is made); every other arm that reads either, `eq_nodes`, `node_repr`, the dumper's compact
form, the loader's name walk and `wildcard_under`, the claim reader's match and render, the compiler's variable walk,
guard key (slot and node) and probes (two), the matcher's unify, ground arguments and candidate scan, the shape dump,
the obligations rung's `bears_local`, the firing's `build_fact`, `not_a_fact` and `resolve`, the detector's and the
saturator's NAF grounding, and `ein-render`'s `value_label`, `atom_id`, `is_atom_like`, `atom_arg_attrs`,
`atom_id_for_value` and the `_` label of `rules.rs`' `shape_attrs`. Each probe logs its id, the thread and argv, so one
reached inside a child `ein` would be named.
- `cargo test --workspace --no-fail-fast` on it (`probe-tests.log`, 16 min 14 s): **950 passed, 0 failed, and no
  probe**, over every corpus sweep, golden, fuzz property and CLI run the gate runs; no test builds a `Wildcard` by hand.
- Every corpus file under 6 000 bytes (404) and 40 000 seeded mutants of them (`ein_corpus::mutants`, seed 0), then
  200 000 more (seed 987654321), each parsed, drawn (`ein render program`'s, `rules`' and `constraints`' functions),
  dumped, loaded, saturated and printed (`s1_sweep.rs`, T1f.5.2.3's `s2_sweep`): 16 317 refused at the parse, 2 646 at
  load, 64 at compile, 21 294 saturated and 83 stopped saturating, and 81 974, 12 729, 340, 104 851 and 510, **no
  probe and no other panic** (`sweep.txt`, `sweep2.txt`).

And by construction: the generated parser makes no `Wildcard`, its `_` and `?_` being tokens no production of `ein.y`
takes, and the macro expander and the import resolver copy only what a program wrote.

**Removed.**
- `ein-ir`: `Node::Wildcard`, and its arms in `eq_nodes`, `node_repr`, the dumper's `compact_into`, the loader's
  `intern_program_names`, and the claim reader's match and render (`expect.rs`); `from_ir::wildcard_under`, which
  `validate_open` read to count a `_` as a slot an `(open …)`'s `absent` introduces.
- `ein-infer`: `Slot::Any` and its arms in the compiler (`slot`, `collect_vars`, the guard key's slot code 4 and node
  code 3, the probe list twice), the matcher (`unify`, `ground_args`, `candidates_scan`), `shape::slot_text`,
  `oblgen::bears_local`, the firing (`build_fact`, `not_a_fact`, `resolve`), and the detector's and the saturator's
  NAF grounding; and in the three examples that match on a slot (`fact_grain`, `detector_draft`, `node_census`).
- `ein-render`: `Node::Wildcard`'s arms in `value_label` and `ir_dot`'s four atom-like helpers, the public
  `dot_util::WILDCARD_ATTRS`, the dashed diamond the shape was drawn with, and `rules.rs`' `_` label branch.
- Every comment that described either as live says what it was and when it went. No line of code names either since
  (`grep -rn 'Slot::Any\|Node::Wildcard' --include='*.rs' ein.rs/crates | grep -v /tests/`: 37 lines on `307021db`, 6
  on this commit, each a comment saying what it was; 2026-11-09). The stage's 42 were taken on 2026-11-06, before
  S1f.5.5's T5 deleted the loader's lines for a head, a guard and a premise.

**Kept, and why.**
- **`dump.rs`' `atom_text` catch-all**, which prints `_` for a head that is neither a name nor a variable: it was the
  wildcard's arm, and by construction a macro can reach it: a body that puts a parameter in head position, called
  with a form, makes a head that is neither (`macros.rs`' `substitute`), which a dump of the expanded program prints
  before the loader refuses it. Not only `_`'s, so not this step's to remove.
- **`ir_dot.rs`' `atom_id` panic** for a node that is not atom-like: the arm every other node took.
- **The tests named for the wildcard** (`a_wildcard_in_a_rule_matches_what_a_fresh_variable_does` and its four
  siblings): each holds the refusal and the variable written in the wildcard's place, and each doc says so since the
  swap; only the two in `compile.rs` said *`Slot::Any` goes with T1f.5.1.3*, now *went*.

**Nothing moved.** Every `.ein` under the five roots (517, `find examples tests stdlib corpus plans -name '*.ein' |
wc -l`, 2026-11-09) through step 2's release build and this commit's (`build/s1f.5.1/t3/xcheck/`, S1f.5.2's scripts
pointed at the two, 145 s): `solve --json-summary -H --dump-answer`, `saturate --dump` and `test`, timings masked,
**517 of 517 the same on every command**; T12's 2 000 mutants through `saturate`, the error text included, the same
(1 047 accepted, 767 refused with repairs, 186 without); and **the container both ways** (`einb.sh`): on the 344 that
load, the four reads agree with each other and with `solve` of the `.ein` but `g14/derived-activator.ein`, whose four
reads agree with each other and place its compile refusal at `<string>`, as T1f.5.4.3 and T1f.5.2.3 found. Seven
containers' bytes differed between the two builds, each by the second it was saved in (`meta.rs`' `created_unix`, which
the header's digest covers); saved again, all seven are the same bytes from both builds.

**The gates**, each on the tree it committed: step 1, 15 min 35 s (`build/gate-s1f.5.1-T3-1.log`), 949 tests, after a
first run stopped on `alive_invariant`'s `SEEDED`, which the commit then carried; step 2, 15 min 38 s
(`build/gate-s1f.5.1-T3-2.log`), 950 tests, step 1's and `a_bare_underscore_names_nothing_in_every_place`; step 3,
16 min 13 s (`build/gate-s1f.5.1-T3-3.log`), 950 tests, every golden read as it was. All green, with every static check.

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
