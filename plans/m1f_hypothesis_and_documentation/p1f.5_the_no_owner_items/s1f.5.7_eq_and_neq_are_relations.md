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
**Not started, and not to start until the user says so.** No engine code, test, golden or fixture has
moved.
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
| `(neq w ?x)` | stores `(neq w a :rule r)`, R 2 states | holds: adds nothing, R 2 states |
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

A prototype on a `git archive` snapshot under `build/`: the five goals above, `(eq ?x ?y)`, `(neq ?x ?y)`,
and a claim over each, each answer computed; the corpus swept, and `cargo test --workspace` run. **Stop and
show the user real programs before any edit** (the standing rule on syntax).

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
[§ Conclusions](#conclusions-g-21) are W30's rows, filed `open` by [S1f.5.0](s1f.5.0_the_known_wrong_answers_filed.md) and fixed here, with the oracle
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
