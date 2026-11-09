# S1f.5.7 — `eq` and `neq` are relations: in a goal, a premise and a conclusion

**Phase:** [P1f.5](README.md) (The no-owner items)
**Was S1e2.5.7**, in M1e2's P1e2.5, until 2026-11-08 ([§ The ids](README.md#the-ids)).
**Estimate:** 3 days.
- Half a day to list every move and show the user real programs, answers computed.
- Half a day for the goal's evaluation, the loader and the wording.
- One day for the premises: the matcher, and W24's rows restated.
- Half a day for the conclusions (G-21).
- Half a day for the tests, the fixtures and the pages.

**Status:** specified 2026-11-07 on the user's rulings of that day on S-18, its two questions and G-21
([Q-M1e2.26](../../m1e2_correctness/open_questions.md#q-m1e226--are-eq-and-neq-relations)), and on what stays refused in a guard
([§ Questions](#questions-it-has-to-settle)). Nothing is open.
**Done 2026-11-09**, in lane 2's worktree (`p1f5-lane2`), on the user's instruction of that day (*"just continue with
default decisions till the end of the phase"*), in seven commits: T1f.5.7.1 ([its record](#t1f571-record-2026-11-09)),
every move listed, ten rows filed (W50 to W55) and W24's two restated, five questions taken by default (Q-M1f.30 to
Q-M1f.34) (`a081ae95`); T1f.5.7.2 ([its record](#t1f572-record-2026-11-09)), the relations in a goal (`e5ae6946`); T1f.5.7.3
([its record](#t1f573-record-2026-11-09)), premises (`6841faff`); T1f.5.7.4 ([its
record](#t1f574-record-2026-11-09)), conclusions (`31a68fdf`); T1f.5.7.5 ([its
record](#t1f575-record-2026-11-09)), the pages, and W56 filed (`c35135c9`); T1f.5.7.6 ([its
record](#t1f576-record-2026-11-09)), a goal's `(absent …)` judged (`607c5d88`); T1f.5.7.7, the acceptance re-taken
([§ Acceptance re-taken](#acceptance-re-taken-2026-11-09)) and what the merge writes ([§ For the merge](#for-the-merge)).
Six questions taken by default in all, Q-M1f.30 to Q-M1f.35; the register on the lane reads 140 fixed, 19 open and 1 not
wrong (`grep '^status' corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-09).
**Depends on:** nothing. It lands on the parser of the day. [S1f.5.5](s1f.5.5_the_grammar.md) frames the
goal's position in the grammar.

## The ruling

The user, 2026-11-07, on S-18: *"the question is much deeper here; At some point I said that some forms
that are not relations facts (not implemented as relation facts) for example rules instantiations actually
look like a relation facts and actually have meaning as relation facts, for example in zebra2 co-located is
a rule, but (co-located nation-loc Englishman color-loc Red) it is naturally looks like a fact of relation
`co-located` and means a fact; I think the same here eq and neq are reserved words, tests, guargs, but we
have to admit that they work as a relations of equality and non-equality."*

And, on the agent's three readings of a free variable: *"logically, result for (query :goal (neq w ?y)) are
all objects from the program except w - (model (neq w a)) (model (neq w b) ..., and for (query :goal (eq w
?y)) - only w - single model (eq w w); It is trivial, but all the more reason to solve it correctly instead
of reject"*.

So **`eq` and `neq` are relations**, of equality and of non-equality. Their facts are fixed by name
identity: `(eq x x)` for every name, and `(neq x y)` for every two different names. They are computed, never
stored, and no fact, rule or commitment changes them. A query over them is answered, not refused.

**The same day, on the agent's two questions and on G-21:** *"1 - default is true; 2 - we have to follow
logical correctness at first, so premises should follow the same rule; G-21 - yes, rule can conclude wrong
(eq x y) or (neq x x), what is the problem? I expect that correct meaningfull ein program will not, but I
don't want add logically unnecessary restrictions without a reason; Is there any?"* The agent found none
([§ Conclusions](#conclusions-g-21)). So the reading holds in every place a relation can be written: a
query's goal, a rule's premise and a rule's conclusion, over the program's names.

`co-located` already works this way: `zebra2.ein` writes `(co-located nation-loc Englishman color-loc Red)`
as a fact, the KB stores it, and `co-located-fanout` derives its swapped twin from it. `eq` and `neq` were
the place the language did not yet admit it.

## The case

Taken on 2026-11-07 at `916d97ac`, on the release binary, a program of `(relation p Node)` and `(p a)` with
one query (the inventory's [probe](../../m1e2_correctness/inventory_2026-11-06/probes/goal_guard.ein) and variations):

| `:goal` | `ein solve` today | its suggested `:expect` | after |
|---|---|---|---|
| `(neq w ?y)` | `the goal holds: it has no variable` | `(model (neq w ?y))`, refused: *an expectation is an answer, not a pattern* | a row per name of the program but `w` |
| `(eq w ?y)` | no row: `the goal has no row: undetermined here` | `(model)` | `?y = w`: `(model (eq w w))` |
| `(neq a b)` | `the goal holds: it has no variable` | `(model (neq a b))`, refused: *names neq, which no declaration or fact makes a relation* | holds: `(model (neq a b))`, which loads and holds |
| `(neq a a)` | no row | `(model)` | no row, unchanged |
| `(and (p ?y) (neq w ?y))` | `?y = a` | `(model (neq w a) (p a))`, refused as above | the same row, and the claim loads and holds |

The last row shows the defect at its worst. `ein test` on that goal, today:

```text
:expect (model (neq w a) (p a))  →  kb load error: :expect names neq, which no declaration or fact makes a relation   [exit 2]
:expect (model (p a))            →  FAILED: (model (neq w a) (p a)) is a class the claim does not list                [exit 1]
```

So a query whose goal holds a guard can carry no claim that holds. No corpus program writes a guard in a
goal (`grep` over `examples/`, `tests/`, `stdlib/` and `corpus/`, 2026-11-07), so no answer moves.

## What it changes

**The domain.** *"All objects from the program"* is read as the program's names, the list the user ruled
on 2026-11-04 (Q-M1e2.9 (a), `invariant::program_names`): every constant a fact names at any depth, and
every constant of a rule's `:match` or `:assert`. A name only a query names, such as `w` above, is not one.
H's candidates are built from the same list, and no state adds a name to it (the M1 alive-set invariant),
so `neq`'s facts are the same in every state. **Ruled: the list as it is**, relation and type names
included. On the probe it holds `p`, `Node` and `a`. In Ein a relation name is a value an argument holds, as
`nation-loc` is in a `co-located` activator, so `(neq w nation-loc)` is a true fact.

**The goal's evaluation** (`ein_infer::verdict::goal_bindings`):
- A guard whose arguments are bound is a test, as today.
- `eq` with one side free binds it to the other side. With both free, it gives every name paired with itself.
- `neq` with one side free gives every name of the program but the other side. With both free, it gives
  every ordered pair of two different names.
- A conjunction gives the same rows whatever the order of its conjuncts. Evaluating the guards after the
  stored relations that bind their variables is a matter of cost only.

**The classes.** A state's class lists the goal's instances, a guard's among them, as today. Within one state,
every row is in the one class: `(query :goal (p ?y))` over `(p a) (p b)` reads `(model (p a) (p b))`, so
`(neq w ?y)` reads `(model (neq w Node) (neq w a) (neq w p))`, one model, not one per row.

**The claim.** The loader accepts `eq` and `neq` in a query's `:expect`: the refusal *names neq, which no
declaration or fact makes a relation* goes for those two. *Naming a relation closes it* reads over the
class, which holds the goal's instances only, so a claim stays finite. A claim about a whole state,
`(solve … :expect (region …))`, compares stored facts, and a state stores no `eq` or `neq`. **Default:**
one listed there is refused at load, with a line saying a state stores none.

**The wording.** *the goal holds: it has no variable* is printed only for a goal with no variable, such as
`(neq a b)`.

**What else.** `scope.rs`'s `outside` reads the relations a goal asks about, and `eq` and `neq` are never
outside H, since no commitment changes their facts. `(neq ?x ?y)` with both free costs n(n−1) rows per state
over n names. The read-out's 100 lines and `--dump-answer` already bound what is printed.

## Premises

**Ruled: a premise follows the same reading.** A rule's `:match` is a conjunction, and its meaning does not
depend on the order of its conjuncts: `(and (neq ?x a) (p ?x))` means what `(and (p ?x) (neq ?x a))` means.
A guard whose variable no premise binds at all, as in `:match (neq ?x a)`, ranges over the program's names.

**This supersedes Q-M1e2.18 (ii)(a)**, the user's ruling of 2026-11-05 that refused at load a guard on a
variable no premise before it binds, and whose option (b), moving the guard after its binders, the reading
contains. Its register row, `W24/guard-before-binder`, holds that refusal as `right`. The witness:

```lisp
(p a) (p b) (p 1) (p (f a)) (q a)
(rule e () :match (and (eq ?x a) (p ?x)) :assert (hit-eq ?x))
(rule n () :match (and (neq ?x a) (p ?x)) :assert (hit-neq ?x))
```

| | today, since S1e2.2.24 | right, under this ruling |
|---|---|---|
| `W24/guard-before-binder` | refused: *rule 'e': its :match writes (eq ?x a), and no premise before it binds ?x* | loads: `(hit-eq a)`, and `(hit-neq b)`, `(hit-neq 1)`, `(hit-neq (f a))` |

Before S1e2.2.24 it loaded and was wrong the other way: each guard compared the unbound `?x` as a null, so
`e` never fired and `n` fired for `a` too. Step 1 restates the row's `right` in the register and the row
goes `open`, owned by this stage. Step 3 fixes it in the same commit as the matcher.

**The matcher.** A guard's variables are bound by the premises around it, whatever their order, and a variable
nothing else binds ranges over the program's names (`compile.rs`, where a built-in predicate is read in a
`:match`, line 627, and the plan's `GuardArgKind`). A rule whose guard has such a variable reads every name
per firing, bounded by the list.

## Conclusions (G-21)

**Ruled: a rule may conclude `eq` or `neq`**, and no load-time restriction is added. The agent looked for a
reason to refuse one and found none:

- **It is logically sound.** `eq` and `neq` have fixed facts. A conclusion that holds, `(neq a b)`, adds
  nothing. One that does not, `(neq a a)` or `(eq a w)`, cannot hold, so the state that derives it is a
  contradiction, as a state that derives `(false)` is. The stdlib already concludes `(false)` on a violation
  (`bijection.ein` lines 137 and 143, `elim.ein` lines 44 and 51), and `:assert (neq ?x ?y)` is the same
  constraint written as the fact it demands: *these two must differ*.
- **Names stay distinct.** Ein reads two different names as two different values, so `(eq a w)` cannot make
  them one. It is a contradiction, not a merge. The ruling's *"wrong (eq x y)"* says so.
- **It adds no non-monotone site.** A conclusion is positive: a state that derives a false one is dead, and
  so is every state above it, as with `(false)`.

**What is wrong today.** The engine stores the conclusion as an ordinary fact, so a false one sits in the KB
while a guard computes the opposite. One relation, two readings, in one state. Taken on 2026-11-07 at
`916d97ac`, on `(relation p Node) (p a)` and one rule `(rule r () :match (p ?x) :assert …)`:

| `:assert` | today | right |
|---|---|---|
| `(neq w ?x)` | stores `(neq w a :rule r)`, R 2 states | at root it holds and adds nothing; `w` is a name of the program, so `(p w)` is a candidate, and at `{(p w)}` the conclusion `(neq w w)` is false: that node is dead, R {∅} |
| `(neq ?x ?x)` | stores `(neq a a :rule r)`, R 1 state, ∅ in R | root is dead: R is empty |
| `(eq ?x w)` | stores `(eq a w :rule r)`, R 2 states, ∅ in R | root is dead: R is empty |

**The fix.** A concluded `eq` or `neq` is checked against its definition, never stored. A true one adds
nothing. A false one is a contradiction, explained as `(false)` is: its core is the premises the firing
matched, and `refuted by` names the rule. No corpus program concludes either (the inventory's census,
2026-11-06), so no corpus answer moves.

## Questions it has to settle

**What stays refused in a guard.** The user, 2026-11-07, correcting the agent, who had listed `_` in a guard
as answerable: *"Seems we already decided not to use `_`? So (eq _ _), (neq ?x _) is refused during load;
(eq ?x :k a) is wrong for 2 reasons - there is no defined `:k` kvarg and `eq` is 2-ary relation"*.

- **`_` in a guard stays refused.** A bare `_` is reserved and refused wherever it is written
  ([Q-M1e2.22](../../m1e2_correctness/open_questions.md#q-m1e222--does-ein-need-a-wildcard),
  [S1f.5.1](s1f.5.1_bare_underscore_refused.md)), and `W24/wild-in-guard` keeps its `right`.
- **A keyword pair as a guard's argument stays refused** (Q-M1e2.18 (iii)(a)): `(eq ?x :k a)` names a keyword
  `eq` does not take, and gives a relation of two places three arguments. Its line becomes
  [S1f.5.11](s1f.5.11_closed_keywords.md)'s, since a term takes no keyword.

**A nested fact as a guard's argument: answered, ruled the same day.** `W24/form-in-guard` (Q-M1e2.18 (i)(a)) is a
fixed row whose `right` is a refusal:

| the witness's rules | today | under the reading |
|---|---|---|
| over `(p a) (p b) (p 1) (p (f a))`: `(and (p ?x) (eq ?x (f a)))`, `(neq ?x (f a))` | refused: *the form (f a) is no value* | a nested fact is a value, as it is in `(p (f a))`: `(hit-eq (f a))`, and `(hit-neq a)`, `(hit-neq b)`, `(hit-neq 1)` |

The agent's default was *answered*, by the ruling's *"no logically unnecessary restrictions"*, and the user
kept it: *"yes, keep that default"*. The row's `right` is restated, and this stage fixes it as it does
`W24/guard-before-binder`.

## The method

### Task T1f.5.7.1 — every move listed, real programs shown

It starts from W30's three rows, which [S1f.5.0](s1f.5.0_the_known_wrong_answers_filed.md) filed `open`
on 2026-11-08, each witness under `examples/ein-bugs/wrong-answers/` and a corpus entry, each `now` a
`solve` taken at `ec004a48` (the engine `916d97ac`'s), release and debug alike:

| row | witness | `now`, today | `right` |
|---|---|---|---|
| `W30/assert-neq-w-x` | `assert-neq-w-x.ein`, the inventory's `assert_neq.ein` | R 2 states; `(neq w a)` stored at root, and the false `(neq w w)` at `{(p w)}` | R {∅}: `{(p w)}` dead |
| `W30/assert-neq-x-x` | `assert-neq-x-x.ein` | R {∅}; `(neq a a)` stored | root dead, R empty |
| `W30/assert-eq-x-w` | `assert-eq-x-w.ein` | R 2 states; `(eq a w)` stored | root dead, R empty |

**The first row's `right` is not [§ Conclusions](#conclusions-g-21)' table's**, which reads *R 2 states*:
`w` is a name of the program, so `(p w)` is a candidate, and at `{(p w)}` the rule concludes `(neq w w)`,
which the ruling makes a contradiction. S1f.5.0 filed R {∅}, and the coordinator settled the difference on
the ruling itself (2026-11-08): G-21 decides it, so the table above now reads R {∅} too. Two more things move with the fix. `neq` stored as a fact makes a
`--dump-answer` state file that does not load, a fact line `(neq …)` being a parse error, so
`ein-cli/tests/answer_round_trip.rs` and `answer_dump.rs` name the two `neq` witnesses (`DO_NOT_LOAD`), and
each list fails once its files load: T1f.5.7.4 empties both. And `ein-infer/tests/alive_invariant.rs`'
`SEEDED` names `w` on `assert-neq-w-x` and `assert-eq-x-w`, which stays a name of the program unless the fix
changes what `invariant::program_names` reads.

Then a prototype on a `git archive` snapshot under `build/`: the five goals above, `(eq ?x ?y)`,
`(neq ?x ?y)`, and a claim over each, each answer computed; the corpus swept, and `cargo test --workspace`
run. Every move must be one of the rows, W24's two restated rows, or the case's goals. **Stop and show the
user real programs before any edit** (the standing rule on syntax).

### Task T1f.5.7.2 — the relations

`goal_bindings`, the loader's check in `expect.rs`, the classes, and the wording. A fixture,
`examples/features/NN_eq_neq_goal.ein`, carries a query per row of the case, each with an `:expect`, so
`ein test` holds them. A unit test in `ein-infer` holds `eq` and `neq` to their definition over a program's
names, each argument free and bound.

### Task T1f.5.7.3 — premises

The matcher, as above, in the same commit as the register: `W24/guard-before-binder` fixed to its restated
`right`, and `W24/form-in-guard` fixed to its restated `right`. `ein-infer/tests/explore.rs` and the oracle
hold every mode on the witnesses, and a fixture shows a premise guard with a free variable.

### Task T1f.5.7.4 — conclusions (G-21)

`compile.rs`' `:assert` side: a concluded `eq` or `neq` is checked, never stored. The three rows of
[§ Conclusions](#conclusions-g-21) are W30's rows, filed `open` by [S1f.5.0](s1f.5.0_the_known_wrong_answers_filed.md) on 2026-11-08 and fixed here, `DO_NOT_LOAD` emptied in the same commit, with the oracle
agreeing on each.

### Task T1f.5.7.5 — the pages

`01_grammar.md` § Query (a goal may name `eq` and `neq`, and what their facts are) and its rule section (a
premise in any order, a conclusion checked), the glossary's `eq` and `neq` (relations of equality and
non-equality, computed, never stored), `02_patterns.md` where it calls them guards, `defined_behaviour.md`
where it states a guard's refusals, and `read_out.md`'s wording row. The inventory's S-18 and G-21 say what
fixed them. Q-M1e2.18 says what superseded its (ii)(a).

## Acceptance

- `(query :goal (neq w ?y))` answers a row for each name of the program but `w`, and `(eq w ?y)` answers
  `?y = w`.
- `(neq a b)` holds and `(eq a b)` has no row, each with a claim that loads and holds.
- `(and (p ?y) (neq w ?y))`'s claim `(model (neq w a) (p a))` loads and holds under `ein test`.
- *the goal holds: it has no variable* is printed only for a goal with no variable.
- A rule's premise means the same in any order of its conjuncts, and a guard's variable nothing else binds
  ranges over the program's names: `W24/guard-before-binder` answers its restated `right`.
- A rule that concludes a true `eq` or `neq` stores nothing, and one that concludes a false one makes its
  state a contradiction, with a core and a rule.
- No corpus answer moves.
- `./run_tests.sh` is green.

## T1f.5.7.1 record (2026-11-09)

**Run on the user's instruction of 2026-11-09**, *"just continue with default decisions till the end of the phase"*:
the step's stop to show the user real programs is covered by this record, and every decision a reviewer would call one
is taken by default and is a row of [`open_questions.md`](../open_questions.md), Q-M1f.30 to Q-M1f.34. The stage ran in
lane 2's worktree, on `p1f5-lane2` from `dc4c0bdc`.

**The prototype** was the whole stage, goals, premises and conclusions, built in the worktree before any commit, its
release binary kept as `build/s1f.5.7/xcheck/bin/ein-new` beside the base's, `ein-base` (`dc4c0bdc`). Every program of
the five roots, 520 files (`find examples tests stdlib corpus plans -name '*.ein'`), went through both:
`solve --json-summary -H --dump-answer`, `saturate --dump` and `test`, the exit, stdout and stderr of each, the summary
and every file the dump wrote compared, timings masked (`build/s1f.5.7/xcheck/run_all.sh`, 90 s). **513 are the same**
and seven move, each one the case's:

| program | what moves |
|---|---|
| `wrong-answers/assert-neq-w-x.ein` (`W30/assert-neq-w-x`) | R two states → {∅}: `{(p w)}` dead, `(neq w w)` false; root stores no `neq` |
| `wrong-answers/assert-neq-x-x.ein` (`W30/assert-neq-x-x`) | R {∅} → empty: root dead, core `(p a)`, `refuted by r` |
| `wrong-answers/assert-eq-x-w.ein` (`W30/assert-eq-x-w`) | R two states → empty: root dead, core `(p a)`, `refuted by r` |
| `wrong-answers/guard-before-binder.ein` (`W24/guard-before-binder`) | refused at load → loads: root `(hit-eq a)`, `(hit-neq b)`, `(hit-neq 1)`, `(hit-neq (f a))` |
| `wrong-answers/form-in-guard.ein` (`W24/form-in-guard`) | refused at load → loads: root `(hit-eq (f a))`, `(hit-neq a)`, `(hit-neq b)`, `(hit-neq 1)` |
| `plans/…/inventory_2026-11-06/probes/assert_neq.ein` | `W30/assert-neq-w-x`'s program, the inventory's probe |
| `plans/…/inventory_2026-11-06/probes/goal_guard.ein` | the case's first goal: three rows, `?y = Node`, `a`, `p` |

Every one is a row or the case, as the step asks. `goal_guard_claim.ein`, the inventory's probe writing the suggested
`(model (neq w ?y))`, is a parse error before and after, a variable in a claim. W24's two restated witnesses answer R
of 256 states each, the blind enumerator's eight candidates over `f`, `hit-eq`, `hit-neq` and `q`, which describe,
`--minimal`, `--maximal` and `ein oracle` agree on; their rows are asked under `--minimal`, root's state.

**The case's goals**, each on `(relation p Node) (p a)`, the prototype's answer; the program's names are `Node`, `a`
and `p` (`(relation p Node)` is a stored fact), and `w` is only the query's:

| `:goal` | today (`dc4c0bdc`) | the prototype | its `as :expect`, which loads and holds |
|---|---|---|---|
| `(neq w ?y)` | *the goal holds: it has no variable* | `?y = Node`, `?y = a`, `?y = p` | `(model (neq w Node) (neq w a) (neq w p))` |
| `(eq w ?y)` | no row | `?y = w` | `(model (eq w w))` |
| `(neq a b)` | holds; its claim refused at load | *the goal holds: it has no variable* | `(model (neq a b))` |
| `(neq a a)` | no row | no row | `(model)` |
| `(and (p ?y) (neq w ?y))` | `?y = a`; its claim refused at load | `?y = a` | `(model (neq w a) (p a))` |
| `(and (neq a ?y) (p ?y))` | `?y = a`, the guard judged on a null | no row | `(model)` |
| `(eq ?x ?y)` | *it has no variable* | three rows, each name with itself | `(model (eq Node Node) (eq a a) (eq p p))` |
| `(neq ?x ?y)` | no row | six rows, every ordered pair of two names | six facts |
| `(eq ?x ?x)` / `(neq ?x ?x)` | *no variable* / no row | three rows / no row | — |

A claim about a whole state, `(solve … :expect (region (model (p a) (neq a b))))`, is refused at load, with *names neq,
which a state never stores — eq and neq are computed from their definition, so a region claim, which lists a state's
facts, holds neither* (the stage file's default, taken). A goal over a nested fact binds to it: `(and (p ?y) (eq ?x
?y))` over `(p (f a))` answers `?x = (f a)`, the one value `eq` holds of.

**Premises, beyond W24's two**: `(rule r (?P) :match (and (neq ?x ?P) (p ?x)) …)` under `(r a)` fires for `b`, the
guard waiting for `(p ?x)`; `(and (neq ?x a) (absent (q ?x)))` over `(p a) (p b) (q b)` derives `hit` of every name but
`a` and `b`, `Node`, `hit`, `p` and `q` among them, a binding guard ranging over the program's names, relation names
included, as the stage file rules; `(and (absent (q ?x)) (neq ?x a))` derives none, the `(absent …)` reading no `?x`
since nothing before it binds one (Q-M1f.34).

**Conclusions**: W30's three answer their rights, and the oracle agrees on each.

**What the step found that the stage file does not list as a row**, each a wrong answer today under the ruling, filed `open`,
owned by this stage, with a witness under `examples/ein-bugs/wrong-answers/` and a corpus entry, each `now` taken at
`dc4c0bdc` on the release build, describe and `ein oracle` agreeing:

| row | witness | `now`, today | `right` | fixed by |
|---|---|---|---|---|
| `W50/fact-eq-false` | `fact-eq-false.ein`: `(p a) (eq a w)` | R two states, `(eq a w)` stored | root dead (Q-M1f.33) | T1f.5.7.4 |
| `W51/not-eq-premise` | `not-eq-premise.ein`: a premise `(not (eq ?x ?y))` | the rule never fires, R four states | root holds `(hit a b)`, `(hit b a)`, R {∅} (Q-M1f.31) | T1f.5.7.3 |
| `W51/not-eq-fact` | `not-eq-fact.ein`: `(not (eq a a))` | R {∅}, the negation stored | root dead (Q-M1f.31, Q-M1f.33) | T1f.5.7.4 |
| `W51/not-eq-assert` | `not-eq-assert.ein`: `:assert (not (eq ?x ?x))` | R {∅}, the negation stored | root dead, core `(p a)`, `refuted by r` | T1f.5.7.4 |
| `W52/relvar-eq-premise` | `relvar-eq-premise.ein`: `(?R ?x ?y)` under `(r eq)` | the rule never fires | root holds `(hit a a)`, `(hit b b)` | T1f.5.7.3 |
| `W52/relvar-eq-assert` | `relvar-eq-assert.ein`: `:assert (?R ?x w)` under `(r eq)` | R two states, `(eq a w)` stored | root dead | T1f.5.7.4 |
| `W53/hrule-eq` | `hrule-eq.ein`: an hrule concluding `(eq ?x ?y)` | H four `eq` facts, R sixteen states | H empty, R {∅} (Q-M1f.32) | T1f.5.7.4 |
| `W54/goal-nested-var` | `goal-nested-var.ein`: `(query :goal (not (p ?x)) …)` | one class `(not (p ?x))`, the claim FAILED | the class is the two instances, the claim holds | T1f.5.7.2 |
| `W55/goal-neq-free` | `goal-neq-free.ein`: `goal_guard.ein` with the claim its answer states | refused at load, the claim naming `neq` | three rows, the claim holds | T1f.5.7.2 |
| `W55/goal-guard-before-binder` | `goal-guard-before-binder.ein`: `(and (neq a ?y) (p ?y))`, `:expect (model)` | `?y = a`, the claim FAILED | no row, the claim holds | T1f.5.7.2 |

W53's witness concludes `eq`, not `neq`: an hrule concluding `neq` proposes the four `neq` facts, whose state files do
not load, the line `(neq …)` being a parse error, which `answer_round_trip.rs` and `answer_dump.rs` would have to name;
the fix covers both. W54 is outside the case: a query's class grounds a conjunct's arguments one level deep, so a pattern inside a pattern
printed with its variables in it. A `(not (eq ?x ?y))` goal reached it; no corpus goal does. W55 is the case's own: its
goals had no row, S1f.5.0 having filed G-21's three and none of S-18's, so the two that show it in a status code, the
case's probe with the claim its answer states and the goal whose guard comes first, are rows.

**A form that holds a variable** as a guard's argument, `(eq ?x (f ?y))`, stays refused at load (Q-M1f.30), now in a
query's goal too, where it was compared as an IR node and had no row. Its line is new: *… and the form (f ?y) holds a
variable — a guard compares two values, each a variable, a name, or a fact that holds no variable*.

**W24's two rows are restated**, as the stage file says: `W24/guard-before-binder`'s and `W24/form-in-guard`'s `right`
is the loaded answer above, each row `open` and owned by S1f.5.7 until T1f.5.7.3 fixes it. The register then reads
**124 fixed, 34 open and 1 not wrong** (`grep '^status' corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-09).

**What moves by banking**: `corpus_exits.txt` gains the ten witnesses' twenty cells and `corpus_shapes.md5` 389
digests, none removed; `alive_invariant.rs`' `SEEDED` names `goal-nested-var`'s `a` and `b` and `relvar-eq-assert`'s
`w`, 28 → 30 programs; `claim_round_trip.rs`' `DO_NOT_READ_BACK` names `hrule-eq`, whose printed region lists `eq`
facts, which no region claim may name, until T1f.5.7.4; the register's well-formedness test takes a live row's classes
(`register::tests::a_claim_and_a_querys_classes_are_answers`), W54's, where it took W28's as they stood; and
`examples/README.md` names the witnesses.

**`cargo test --workspace` on the prototype** failed where the step expected and nowhere else: the register's rows
(W24's, W30's), `answer_round_trip.rs` and `answer_dump.rs`, whose `DO_NOT_LOAD` names W30's two `neq` witnesses, the
W24 refusal's tests (`cli_semantics`, `from_ir`'s unit test), the matcher's null-guard test, and the banked exits.
Each is a task's below.

## T1f.5.7.2 record (2026-11-09)

**`eq` and `neq` in a goal.** The goal is compiled as a rule's `:match` is, so what changed is the compiler and the
matcher, which a premise reaches too: since this task a rule the loader accepts reaches them as well, and none until
T1f.5.7.3 lets the loader take a guard before its binder.

- **The matcher** (`match_::Matcher::guard`): a guard whose sides are bound is a test; a side that is a variable nothing
  bound is bound by the guard, `eq` to the other side's value (a name only the query writes, or a nested fact, among
  them) and with both sides free to every name paired with itself, `neq` to every name but the other side and with both
  free to every ordered pair of two different names, `(neq ?x ?x)` to none. Until then an unbound variable was a null,
  equal to another unbound one and to nothing else.
- **The domain** (`predicates::domain`): `invariant::program_names`, sorted by text, taken once per program and kept on
  it (`Program::domain`, cleared by `Kb::program_mut`), the same in every state by the M1 alive-set invariant.
- **Where a guard goes** (`compile`'s `Seq`): a guard whose variable a relation premise of its conjunction binds waits
  for it, and is a test after it; one whose variable nothing in the conjunction binds binds it where it is written, as
  a relation premise does, an `(absent …)` after it reading it (Q-M1f.34). Every guard of every program the loader took
  until now has its variables bound where written, so no plan the corpus compiles moved (the cross-check below).
- **A guard's argument** (`GuardArgKind`): a variable, a name, or a fact that holds no variable, which the compiler
  reads as the nested fact it spells, interned at load (`from_ir::intern_guard_facts`); `GuardArgKind::Node`, an IR node
  compared by node equality, is gone. A form that holds a variable is refused in a query's goal at load, *query: its
  :goal writes (neq ?x (f ?y)), and the form (f ?y) holds a variable — …* (Q-M1f.30), and by the compiler otherwise.
- **The claim** (`from_ir::validate_queries`): a query's `:expect` may name `eq` and `neq`; a region claim that names
  either is refused, *(solve … :expect …) names neq, which a state never stores — eq and neq are computed from their
  definition, so a region claim, which lists a state's facts, holds neither* (the stage file's default, taken).
- **The classes** (`read_out::ground`): a class substitutes its rows at every depth (W54).
- **The wording** (`ein-render`'s query block): *the goal holds: it has no variable* is printed only for a goal with no
  variable; a goal whose every variable is an `(absent …)`'s own, which binds none, prints *the goal holds: it binds no
  variable*.

**Moved**, every one a row or the case's, and nothing else, by the cross-check of every program against `dc4c0bdc`'s
build (`build/s1f.5.7/xcheck`, 531 files, 526 the same, 2026-11-09): W54's row and W55's two fixed, their witnesses'
exits 1 → 0; `goal_guard.ein`, the inventory's probe, answers three rows; and the new fixture. `examples/features/16_eq_neq_goal.ein` carries a query per row of the case, nine
claims, which `ein test` holds (`test_cli`'s count of held claims under `examples/features/`, 6 → 15).
Banked: `corpus_exits.txt` four cells 1 → 0 (W54's and W55's witnesses) and the fixture's four; `corpus_shapes.md5` 15
digests of those three witnesses moved, 23 added for `goal-neq-free`, which loads, and 41 for the fixture, none of any
other program. `predicates::tests::eq_and_neq_are_their_definition_over_the_programs_names` holds the two to their definition over a
program's names, each of 32 goals, every argument a name of the program, a name only the query writes, or a variable,
free or shared; `match_::tests::two_free_guard_arguments_range_over_the_programs_names` replaces the null's test. The
register reads **127 fixed, 31 open and 1 not wrong** (`grep '^status' corpus/wrong_answers.toml | sort | uniq -c`,
2026-11-09).

## T1f.5.7.3 record (2026-11-09)

**Premises.** A rule's `:match` follows the reading a goal does, on the user's ruling of 2026-11-07 (*"premises should
follow the same rule"*), which supersedes Q-M1e2.18 (i)(a) and (ii)(a):

- **The loader** (`from_ir::refuse_no_fact_holds`) takes a guard on a variable no premise before it binds, and a fact
  that holds no variable as a guard's argument; it refuses a form that holds a variable, one line per form, *rule 'e':
  its :match writes (eq ?x (f ?y)), and the form (f ?y) holds a variable — a guard compares two values, each a
  variable, a name, or a fact that holds no variable* (Q-M1f.30). Its other refusals are unmoved.
- **The compiler** reads `eq` wherever it is written (`compile`'s `guard_form`): a pattern whose relation a parameter
  binds to `eq`, `(?R ?x ?y)` under `(r eq)`, is the `eq` guard, and `(not (eq ?x ?y))` is `(neq ?x ?y)` (Q-M1f.31), in
  a premise and in a goal. T1f.5.7.2's guard placement and matcher answer the rest.

**Moved**, every one a row: `W24/guard-before-binder` and `W24/form-in-guard` load, and answer their restated `right`s
(root holds `(hit-eq a)`, `(hit-neq b)`, `(hit-neq 1)`, `(hit-neq (f a))`, and `(hit-eq (f a))`, `(hit-neq a)`,
`(hit-neq b)`, `(hit-neq 1)`), describe, `--minimal`, `--maximal` and `ein oracle` agreeing on R's 256 states
(`ein-infer/tests/explore.rs` holds every mode to the oracle on every register witness); `W51/not-eq-premise` and
`W52/relvar-eq-premise` fire. `cli_semantics::a_rule_shape_that_reads_nothing_is_refused_at_load` holds the two
witnesses loading on `saturate`, `solve` and `oracle`, and the pattern's refusal in a rule and in a goal on every
command; `from_ir`'s unit test of the same name holds the shapes that load and the line. Four tests in
`ein-infer/tests/rule_semantics.rs` hold the reading: a premise in either order, a free variable over the program's
names, a binding guard where it is written beside an `(absent …)` (Q-M1f.34), and `(not (eq …))` and a relation bound
to `eq`. `examples/features/17_eq_neq_premise.ein` shows a premise guard before its binder, a fact as a guard's
argument, and a guard whose variable no premise binds, four claims `ein test` holds (`test_cli`'s count, 15 → 19).
Against `dc4c0bdc`'s build every program but these and T1f.5.7.2's is the same (`build/s1f.5.7/xcheck`, 532 files, 522
the same, 2026-11-09). Banked: `corpus_exits.txt` W24's four cells 1 → 0 and the fixture's four; `corpus_shapes.md5` 54
digests of the four witnesses moved, 42 added for W24's two, which load, and 41 for the fixture, none of any other
program; the register's region blocks for the four, `register_region.rs`'s re-bank. The register reads **131 fixed, 27
open and 1 not wrong** (`grep '^status' corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-09).

## T1f.5.7.4 record (2026-11-09)

**Conclusions** (G-21): a conclusion of `eq` or `neq`, or of `(not (eq …))`, is checked against its definition, never
stored (`firing::build_conclusion`, `firing::computed`): one that holds adds nothing, and one that does not is
`(false)`, derived by the rule from the premises its firing matched, so its state is dead, explained as a rule
concluding `(false)` is, its core those premises and `refuted by` the rule. An application whose every conclusion holds
derives nothing and is no firing (`Saturator::apply`), as a rule with no `:assert`; the lookahead probes the same
conclusions (`lookahead::dies_immediately`), and the static readers count such a conclusion as one that may write
`(false)` (`monotone::writes_of`, `naf_deps`), which moves no corpus program's flag. **A written fact** of `eq` or `(not
(eq …))` is a conclusion with no premise (Q-M1f.33): a true one is not stored, a false one is `(false)`, written where
it stands. **An hrule's** `eq` or `neq` instance is no hypothesis (Q-M1f.32).

**Moved**, every one a row: W30's three, `W30/assert-neq-w-x` R {∅} with `{(p w)}` dead, `W30/assert-neq-x-x` and
`W30/assert-eq-x-w` root dead, core `(p a)`, `refuted by r`; `W50/fact-eq-false` and `W51/not-eq-fact` root dead,
`refuted by (given)`; `W51/not-eq-assert` and `W52/relvar-eq-assert` root dead, core `(p a)`, `refuted by r`;
`W53/hrule-eq` H empty, R {∅}. The oracle agrees on each (`explore.rs`). The state files of W30's two `neq` witnesses
load, so `answer_round_trip.rs`' and `answer_dump.rs`' `DO_NOT_LOAD` name W34's alone, and `claim_round_trip.rs`'
`DO_NOT_READ_BACK` lets `hrule-eq` go. `alive_invariant.rs`' `SEEDED` is unmoved: `w` stays a name of the program on
W30's two and `relvar-eq-assert`, a constant a rule's `:assert` writes, and `fact-eq-false`'s `w`, written in a fact
that is no longer stored, was never a seed (`(eq a w)` is a top-level fact). Two tests in `rule_semantics.rs` hold
it: a conclusion checked, never stored, with the false one's `(false)` the rule's from its premises, and a written fact.
Against `dc4c0bdc`'s build every program but these and T1f.5.7.2's and .3's is the same (`build/s1f.5.7/xcheck`, 532
files, 513 the same, 2026-11-09), the inventory's `assert_neq.ein` among the moved, `W30/assert-neq-w-x`'s program. Banked:
`corpus_shapes.md5` 147 digests of the eight witnesses, none of any other program, and no exit code; the eight rows'
region blocks. The register reads **139 fixed, 19 open and 1 not wrong** (`grep '^status' corpus/wrong_answers.toml |
sort | uniq -c`, 2026-11-09).

## T1f.5.7.5 record (2026-11-09)

**The pages.** No current kernel page describes `eq` and `neq` as predicates that bind nothing, a guard on a variable
no premise before it binds or a form as a guard's argument as refused, or a conclusion of either as stored: each
sentence that did says what it was and what it is since this stage.

- [`01_grammar.md`](../../../docs/kernel/ir/03-ein-lang/01_grammar.md): a fact's relation (a written `(eq a b)` checked,
  `(neq …)` no fact), what a premise may be, a conclusion of `eq` or `neq` (G-21), *Premise order around a guard*
  rewritten (a premise in any order, a free variable over the program's names, Q-M1f.34), and § Query (a goal over
  `eq` and `neq`, a claim naming either, the empty row's two sentences, a region claim refused).
- [`02_patterns.md`](../../../docs/kernel/ir/03-ein-lang/02_patterns.md): the closure line, the computed-equality row,
  and the registry's table and its paragraph.
- [`06_reserved_names.md`](../../../docs/kernel/ir/03-ein-lang/06_reserved_names.md) § Computed predicates, and
  [`implementation.md`](../../../docs/kernel/inference/implementation.md)'s `predicates.rs` row.
- [`glossary.md`](../../../docs/kernel/glossary.md): an entry for `eq` and `neq`.
- [`defined_behaviour.md`](../../../docs/kernel/defined_behaviour.md) § 4.1's claim rows (a region claim naming
  either, a goal's form that holds a variable) and § 4.9's guard rows and its paragraph.
- The inventory's [S-18 and G-21](../../m1e2_correctness/inventory_2026-11-06_no_owner.md) say what fixed them, and
  [Q-M1e2.18](../../m1e2_correctness/open_questions.md#q-m1e218--what-else-in-a-rule-reads-nothing-in-silence) and
  [Q-M1e2.26](../../m1e2_correctness/open_questions.md#q-m1e226--are-eq-and-neq-relations) that they are built.

M1e2's `read_out.md` is the record and is not edited; the live read-out page is S1f.5.16's, on `master`, and what this
stage changes on it is the coordinator's to write at the merge ([§ For the merge](#for-the-merge)).

**Found outside the case, filed, and fixed by the next task.** The stage's test of a goal's empty row
(`cli_semantics::a_goals_empty_row_says_whether_the_goal_has_a_variable`, added here) wrote `(query :goal (absent (q
?x)))`, and the goal answered over a state holding a `q`: a goal's top-level `(absent …)` is lifted out of its plan
and `verdict::goal_bindings` never judges it, and a claim listing the class's `(absent (q a))` is refused. Filed as
`W56/goal-absent`, `open` and owned by S1f.5.7
([Q-M1f.35](../open_questions.md#q-m1f35--a-goals-absent--whose-row-is-it), defaulted), its witness
`examples/ein-bugs/wrong-answers/goal-absent.ein`, `now` the claim's refusal at `31a68fdf`. No corpus goal writes an
`(absent …)`, `forall` or `unknown` (a scan of every `:goal` in the five roots, 2026-11-09). The register reads **139
fixed, 20 open and 1 not wrong** (`grep '^status' corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-09).

## T1f.5.7.6 record (2026-11-09)

**A goal's `(absent …)` is judged** (W56, Q-M1f.35): `verdict::goal_bindings` judges each guard lifted out of the
goal's plan against the state, as the boundary judges a rule's, so a row the state refutes is none; and a query's claim
may list a goal's `(absent …)` instance, as it lists a guard's (`from_ir::validate_queries`). `W56/goal-absent` answers
its right: one row, `?y = a`, and the claim holds, exit 0. `verdict::tests::a_goals_absent_is_judged` holds the guard
at the top level, alone, and in a `forall`'s nested shape. Against `dc4c0bdc`'s build only the stage's programs move,
the new witness among them (`build/s1f.5.7/xcheck`, 533 files, 513 the same, 2026-11-09). Banked: `corpus_exits.txt`
the witness's two cells 1 → 0, `corpus_shapes.md5` its 7 digests moved and 21 added, and its region block. The register reads **140 fixed, 19 open and 1 not
wrong** (`grep '^status' corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-09).

## Acceptance re-taken (2026-11-09)

Each item, and what holds it:

- *`(query :goal (neq w ?y))` answers a row for each name of the program but `w`, and `(eq w ?y)` answers `?y = w`*:
  `examples/features/16_eq_neq_goal.ein`'s first two claims under `ein test` (`corpus_cli`, `test_cli`), the register's
  `W55/goal-neq-free`, and `predicates::tests::eq_and_neq_are_their_definition_over_the_programs_names`, every
  combination of a name, a query's own name and a variable on each side.
- *`(neq a b)` holds and `(eq a b)` has no row, each with a claim that loads and holds*: the fixture's third and fourth.
- *`(and (p ?y) (neq w ?y))`'s claim `(model (neq w a) (p a))` loads and holds under `ein test`*: the fixture's eighth.
- *the goal holds: it has no variable is printed only for a goal with no variable*:
  `cli_semantics::a_goals_empty_row_says_whether_the_goal_has_a_variable`.
- *A rule's premise means the same in any order of its conjuncts, and a guard's variable nothing else binds ranges
  over the program's names: `W24/guard-before-binder` answers its restated right*: the register's two W24 rows, and
  `rule_semantics`' `a_premise_means_the_same_in_any_order_of_its_conjuncts`,
  `a_guards_free_variable_ranges_over_the_programs_names` and `a_binding_guard_binds_where_it_is_written`;
  `examples/features/17_eq_neq_premise.ein`.
- *A rule that concludes a true `eq` or `neq` stores nothing, and one that concludes a false one makes its state a
  contradiction, with a core and a rule*: W30's three rows, W51's and W52's conclusion rows, and
  `rule_semantics::an_eq_or_neq_conclusion_is_checked_never_stored`.
- *No corpus answer moves*: the cross-check of every program of the five roots against `dc4c0bdc`'s build moved only
  the register's witnesses the stage owns, its two fixtures and the two inventory probes of its case
  (`build/s1f.5.7/xcheck/result.txt`).
- *`./run_tests.sh` is green*: at every commit, each body naming its log.

**S1f.5.7 is done, 2026-11-09.**

## For the merge

S1f.5.7 ran in lane 2 (`p1f5-lane2`, from `dc4c0bdc`), which does not edit the prose both lanes share. What those
places should gain when the coordinator merges the branch:

1. **P1f.5's README, the stage table's S1f.5.7 row**, its state: *"**done 2026-11-09**, on lane 2 in seven commits
   ([the records](s1f.5.7_eq_and_neq_are_relations.md#t1f571-record-2026-11-09)): T1f.5.7.1, every move listed, W50 to
   W55 filed and W24's two restated (`a081ae95`); T1f.5.7.2, the relations in a goal (`e5ae6946`); T1f.5.7.3,
   premises (`6841faff`); T1f.5.7.4, conclusions (`31a68fdf`); T1f.5.7.5, the pages, and W56 filed (`c35135c9`);
   T1f.5.7.6, a goal's `(absent …)` judged (`607c5d88`); T1f.5.7.7, the acceptance re-taken. [Q-M1f.30](../open_questions.md#q-m1f30--a-form-that-holds-a-variable-as-a-guards-argument)
   to Q-M1f.35 taken by default; on the lane the register reads 140 fixed, 19 open and 1 not wrong"*. The same
   README's items table: S-18's and G-21's last cell gains *"**done 2026-11-09** (W55, W30)"*.
2. **The register's counts**: lane 2 files eleven rows (W50 to W56) and fixes them and W30's three and W24's two, all
   in `corpus/wrong_answers.toml`, which the merge carries; on the lane `grep '^status' corpus/wrong_answers.toml | sort
   | uniq -c` reads 140 fixed, 19 open, 1 not wrong (160 rows). The merged count is master's plus these: +14 fixed, −3
   open, +11 rows.
3. **AGENTS.md**, after the sentence on S1f.5.5's close: *"**S1f.5.7 ran to its end the same day** (lane 2): `eq` and
   `neq` are the relations of equality and non-equality, computed and never stored (the user's ruling of 2026-11-07,
   Q-M1e2.26, and G-21). In a goal and a premise a guard binds a variable nothing else binds, `eq` to the other side
   and `neq` over the program's names (`ein_infer::predicates::domain`), and waits for the relation premise that
   binds it, so a premise means the same in any order of its conjuncts; a conclusion of either, a written `(eq …)`
   fact and `(not (eq …))` are checked, a false one `(false)`; a query's claim may name either. W30's three, W24's two
   restated rows and W50 to W56's eleven fixed, Q-M1f.30 to Q-M1f.35 defaulted"*, linking this file as
   `plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.7_eq_and_neq_are_relations.md`, with the
   merged count.
4. **The milestone README**: nothing; it names no stage's state.
5. **The live read-out page, `docs/kernel/inference/read_out.md`, and its generated golden page** (S1f.5.16's, on
   `master`), what `ein solve` prints differently since this stage:
   - a query's class: a guard's variable is bound in every row it has, `?y = Node`, `?y = a`, … where the row read
     *the goal holds: it has no variable*; that sentence is printed only for a goal with no variable, and a new one,
     *the goal holds: it binds no variable*, for a goal whose every variable is an `(absent …)`'s own
     (`ein-render`'s query block);
   - a class's facts and its `as :expect` line: the goal's instances under the rows at every depth (W54), an `eq`,
     `neq` or `(absent …)` instance among them, so the suggested claim loads; a goal's `(absent …)` is judged (W56),
     so a row the state refutes is gone;
   - a state dead by a false `eq` or `neq` conclusion: `empty  every node is dead — root's own fixpoint derives
     (false)` at root, `unsat core` the premises the firing matched, `refuted by` the rule; by a written false `(eq a
     w)`: `unsat core (false)`, `refuted by (given)`;
   - no summary field's name or schema moves (`ein-summary/2`).
6. **The register's schema**: no bump. Every row lane 2 files uses schema /16's fields; /18, which the coordinator kept
   for lane 2, is unused.
7. **The id blocks**: rows W50 to W56 used, W57 unused; questions Q-M1f.30 to Q-M1f.35 used, Q-M1f.36 to Q-M1f.41
   unused.
8. **Files both lanes may have touched**, which the merge should re-take rather than splice: `corpus_exits.txt` and
   `corpus_shapes.md5` (`EIN_BLESS=1`; lane 2's cells are its eleven witnesses and two fixtures), the register's
   header paragraphs (lane 2 appends six after W40/W41's), `corpus/corpus.toml`'s entries, `examples/README.md`'s
   witness count (154 on the lane) and its `features/` line, `open_questions.md`'s index table (lane 2's rows follow
   Q-M1f.17's), `test_cli::a_query_with_no_expectation_is_never_solved`'s count of held claims under
   `examples/features/` (19 on the lane), and the kernel pages lane 2 edited: `01_grammar.md`, `02_patterns.md`,
   `06_reserved_names.md`, `glossary.md`, `defined_behaviour.md` (§ 4.1's claim rows, § 4.9) and
   `inference/implementation.md`.
