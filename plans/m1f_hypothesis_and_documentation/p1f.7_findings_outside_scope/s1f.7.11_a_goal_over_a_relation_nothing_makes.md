# S1f.7.11 — A goal over a relation nothing in the program makes

**Phase:** [P1f.7](README.md) (The wrong answers P1f.5's stages found outside their scope)
**Status:** new 2026-11-11, from the [analysis of 2026-11-11](README.md#the-analysis-of-2026-11-11), on the user's
ruling of that day (*"yes - it become S1f.7.10 onward"*). **Done 2026-11-11**, with S1f.7.5 and S1f.7.1 in one
subagent: the records are below. W78, four rows, filed and fixed; Q-M1f.119 and .120 defaulted; one row of F25.
**Depends on:** [S1f.7.5](s1f.7.5_a_macro_called_in_a_goal.md), so that a goal's macro call is expanded before its
relations are read, and [S1f.7.8](s1f.7.8_a_solve_list_naming_nothing.md), whose check of a name against the
program this stage reuses.

## The finding

Taken on the release binary at `4bf6da73`, 2026-11-11: `(p k)` with `(query :goal (zzz ?x))` loads, and every state
reports *the goal has no row*, exit 0. No declaration, fact, rule or hrule names `zzz`, so no state can hold a
`zzz` fact. A relation misspelt in a goal reads the same as a goal that is undetermined.

A claim is already checked: `:expect` naming such a relation is refused, *names att, which no declaration or fact
makes a relation*. That check is too strict the other way: it refuses a relation that only a rule's `:assert`
derives (the analysis's `d1.ein`: the claim lists `(won a)`, which rule `r` derives, and the load is refused).

## The direction

- **(a) Refused at load** (the default): a goal's premise over a relation that no declaration, no fact, and no
  rule's, hrule's or obligation's conclusion names. A relation a conclusion can make is accepted.
- **(b) A warning row** in the read-out, the goal loaded.

The claim check's over-refusal is the same list read short. T1 takes it, and the stage fixes it with (a)'s list if
T1 confirms it, as a row of its own.

## Tasks

- **T1f.7.11.1, confirmation and the census.** Re-take both. Every site a relation is read and never made: a goal,
  a goal under `absent` (where it holds vacuously), a claim, a rule's `:match` (a rule that can never fire).
  Whether the rule's site is this stage's is a Q-M1f row; the default is that it is reported and not refused,
  because a library rule may read a relation its importer supplies.
- **T1f.7.11.2, the rows**, `open`, held to today's engine.
- **T1f.7.11.3, the fix**, the rows flipped, a test on every command.
- **T1f.7.11.4, the pages**: `01_grammar.md` § Query, `defined_behaviour.md`, the records.

## Acceptance

- A goal over a relation nothing makes is refused on every command, in S1f.5.21's spelling; a claim over a relation
  a rule derives loads.
- Every witness is a register row. No corpus answer moves but the witnesses' and the fixtures the stage names.
- `./run_tests.sh` green.

## T1 record, 2026-11-11

**Both confirmed**, on the release binary at `73a0c17d`. `(p k)` with `(query :goal (zzz ?x))` loads and reports
*the goal has no row: undetermined here*, exit 0. A program whose rule `r` derives `(won ?x)` from `(p ?x)`, with
`(query :goal (won ?x) :expect (model (won k)))`, is refused at load, *:expect names won, which no declaration or
fact makes a relation*, and so is `(solve :expect (region (model (p k) (won k))))`; without the claim the goal
answers `?x = k`.

**Every site a relation is read and never made:**

| site | before the stage | since |
|---|---|---|
| a goal's pattern, `(zzz ?x)` | loads, no row | refused at load |
| a goal's pattern under `(absent …)` | loads, and holds whatever the state holds: on the witness Ann, who has paid, is answered as one who has not | refused |
| a goal's stored negation, `(not (zzz ?x))`, and a pattern inside a pattern, `(p (zzz ?x))` | loads, no row | refused |
| a goal's disjunct, and a second query's goal | loads, no row for it | refused |
| a claim's fact, a query's and a region's | refused, where a rule's `:assert` derives the relation too | refused only where the program names no such relation |
| a rule's, an hrule's or an obligation's `:match` | loads; the rule never fires | unmoved: reported here and not refused ([Q-M1f.120](../open_questions.md#q-m1f120--a-rules-match-over-a-relation-nothing-makes), F25's R-6) |
| a `(solve …)` list | refused since S1f.7.8 | unmoved |

On every command the goal loaded: `ein saturate`, the views and `kb save` do not answer a query, and `ein solve`,
`ein test` and `ein oracle` answered it with no row.

**What a relation of the program is** is S1f.7.8's notion without the goals
([Q-M1f.119](../open_questions.md#q-m1f119--which-relations-a-goal-may-ask-about)): the stage file's wording, *no
conclusion names it*, would refuse a goal over a relation only a rule's `:match` reads, and the phase has one
notion. Two things T1 added to it. A goal's own relations cannot count for a goal, or every goal would pass. And
an activator of `(solve :hrules …)` makes the relation it is given, `(guess (at Color House))` proposing facts of
`at`, so a goal over `at` beside that list asks about something a state can hold, though the program writes `at`
nowhere else; S1f.7.8 accepts the list because the goal names `at`, and this stage accepts the goal because the
list's activator proposes it. A misspelling in either one is still refused.

## T2 record, 2026-11-11

Four witnesses under `examples/ein-bugs/wrong-answers/`, each a corpus entry (`solve`, `saturate`) and a register
row, W78:

| row | witness | answered at `73a0c17d` | right |
|---|---|---|---|
| `W78/goal-no-such-relation` | `goal-no-such-relation.ein` | loads, no row | refused at load |
| `W78/goal-absent-no-such-relation` | `goal-absent-no-such-relation.ein` | loads, two rows, `?g = Ann` and `?g = Bob` | refused at load |
| `W78/claim-derived-relation` | `claim-derived-relation.ein` | refused at load | loads; the claim holds |
| `W78/solve-claim-derived-relation` | `solve-claim-derived-relation.ein` | refused at load | loads; the region claim holds |

Filed `open` with the other two stages' rows and held to the unchanged engine
([S1f.7.5's T2 record](s1f.7.5_a_macro_called_in_a_goal.md#t2-record-2026-11-11), which also names the goldens).

## T3 record, 2026-11-11

**The list** is `ein_ir::from_ir`'s `program_relations`, taken out of `refuse_solve_names`: the declared and
fact-made relations, `is-a`, `eq` and `neq`, the relation of every form at any depth in a rule's, an hrule's or an
obligation's `:match` and `:assert` and in a fact, and every name passed where a relation's name is held.
`refuse_solve_names` adds the goals' relations for its lists, as before. A goal and a claim are held to the list
and to `activator_relations`, what a `:hrules` activator has its hrule propose.

**The refusal** is `refuse_goal_relations`, in `validate_queries`, after the goal's other checks: one line per
relation, placed where the pattern writes it,

```text
kb load error: t.ein:28:15: query: its :goal writes (payed ?g), and the program names no relation payed — a relation of the program is one it declares, states a fact of, or writes in a rule or an hrule, so no state holds a fact of payed
```

It reads the relation of every form of the goal at any depth but the connectives, `absent`, `not`, `eq`, `neq`
and the kernel's reserved names, so `(relation ?R ?T)` loads. A `std.macro` name the program did not import has
S1f.7.5's line and not this one. A program already refused gets no line from it, as from `refuse_solve_names`.

**The claim check** reads the same list, in `validate_solve` and `validate_queries`, so a claim over a relation a
rule's `:assert` derives loads. Its line is reworded, since *no declaration or fact makes a relation* no longer
says what is checked:

```text
kb load error: :expect names nosuch, and the program names no relation nosuch — a relation of the program is one it declares, states a fact of, or writes in a rule or an hrule
kb load error: (solve … :expect …) names payed, and the program names no relation payed — a relation of the program is one it declares, states a fact of, or writes in a rule or an hrule
```

**One test of S1f.7.8's moved**: `solve_lists.rs`'s accepted program whose list names a relation only a goal
reads, `(solve :no-hypothesis (wanted))`, is refused at its goal since, and is now `(solve :hrules (guess (wanted
Color House)))` beside the goal, the case that still loads.

The sweep, the rows' count and the pin are in
[S1f.7.5's T3 record](s1f.7.5_a_macro_called_in_a_goal.md#t3-record-2026-11-11): no corpus goal asks about a
relation the program does not name, and one broken fixture's refusal is reworded.

## T4 record, 2026-11-11

`defined_behaviour.md` § 4.21 and its § 4.1 table's two claim rows, `01_grammar.md` § Query, and the records;
[S1f.7.5's T4 record](s1f.7.5_a_macro_called_in_a_goal.md#t4-record-2026-11-11) lists the pages.

## Acceptance re-taken, 2026-11-11

- *A goal over a relation nothing makes is refused on every command, in S1f.5.21's spelling; a claim over a
  relation a rule derives loads*: `goal_premises.rs`, with what *nothing makes* means in Q-M1f.119.
- *Every witness is a register row. No corpus answer moves but the witnesses' and the fixtures the stage names*:
  the T2 and T3 records; the one fixture named is `examples/broken/load/expect_unknown_relation.ein`, refused as
  before with the reworded line, its `.expected` restated.
- *`./run_tests.sh` green*: exit 0 on the stages' tree (`build/gate-s1f.7.5.log`, 2026-11-11).
