# P1f.7 — The wrong answers P1f.5's stages found outside their scope, a stage each

**Milestone:** [M1f](../README.md)
**Status:** planned 2026-11-10, on the user's instruction of that day ([§ The instruction](#the-instruction)). No
stage has started. It runs after [P1f.5](../p1f.5_the_no_owner_items/README.md) closes. The coordinator stops at
that close (the user, 2026-11-10: *"stop in the end of the phase"*), so the user decides then when this phase
starts, and whether its questions stop the stage or take a default.
**Depends on:** P1f.5. Its stages use [S1f.5.21](../p1f.5_the_no_owner_items/s1f.5.21_a_compile_error_in_eins_words.md)'s
refusal line and [S1f.5.24](../p1f.5_the_no_owner_items/s1f.5.24_a_macro_body_checked_where_a_rule_is.md)'s walk of
a rule's clauses after macro expansion (`ein_ir::from_ir::refuse_malformed_primitives`).
**Blocks:** nothing. It exists so that P1f.5 can close. Without it, P1f.5 would keep growing: each re-opened stage
finds wrong answers outside its scope, and until this phase they were added to S1f.5.28.

## The instruction

The user, 2026-11-10, on S1f.5.28's items 12 to 16:

> new batch of findings plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.28_the_unchecked_findings_triaged.md
> creates new phase p1f.7 with stage per finding to allow to close current phase

and, the same day: *"stop in the end of the phase"*.

## What it takes

Two kinds of finding come here
([Q-M1f.85](../open_questions.md#q-m1f85--which-findings-p1f7-takes), defaulted):

- **The new batch**: S1f.5.28's items 12 to 16, as they stood on 2026-11-10. Each is a wrong answer that a
  re-opened P1f.5 stage found outside its scope and recorded without fixing. The coordinator had given each to
  S1f.5.28 the same day. Item 12 is from S1f.5.21's T1 record; items 13 to 16 are from S1f.5.24's T2 record.
- **Later findings**: every wrong answer that P1f.5's remaining stages (S1f.5.22, S1f.5.23, S1f.5.25, S1f.5.27 and
  S1f.5.28) find outside their scope. Each becomes a stage here, numbered from S1f.7.6 in the order found:
  S1f.5.23 gave four on 2026-11-10, S1f.7.6 to S1f.7.9.

S1f.5.28 keeps its items 1 to 11. They are the close review's findings 9 and 7, which the user ruled into that
stage, and they are confirmed, triaged and fixed there.

No finding here has a register row yet. Each stage files its own rows in its T2, before its fix, as S1f.5.21 to
S1f.5.24 did. P1f.5's close record names them as this phase's.

## Stages

| ID | title | finding | from | state |
|---|---|---|---|---|
| [S1f.7.1](s1f.7.1_a_goals_variable_relation_in_a_nested_pattern.md) | A goal's nested pattern whose relation is a variable: refused, as in a rule | `(query :goal (p (?R ?x)))` loads and matches nothing, in silence | S1f.5.28 item 12; S1f.5.21's T1 record | new 2026-11-10 |
| [S1f.7.2](s1f.7.2_an_hrule_that_reads_not.md) | An hrule that reads `not`: one answer from both builds | a debug build stops (exit 101) where release answers | S1f.5.28 item 13; S1f.5.24's T2 record | new 2026-11-10 |
| [S1f.7.3](s1f.7.3_a_connective_as_a_patterns_argument.md) | A connective as a premise pattern's argument | `(q (and (p ?x)))` loads and never matches, in silence | S1f.5.28 item 14; S1f.5.24's T2 record | new 2026-11-10 |
| [S1f.7.4](s1f.7.4_the_negation_of_a_name_concluded.md) | A conclusion `(not ?x)` whose variable binds a name | stores `(not k)`, which no program can write | S1f.5.28 item 15; S1f.5.24's T2 record | new 2026-11-10 |
| [S1f.7.5](s1f.7.5_a_macro_called_in_a_goal.md) | A macro called in a query's `:goal` | the call is never expanded, so the goal has no row, in silence | S1f.5.28 item 16; S1f.5.24's T2 record | new 2026-11-10 |
| [S1f.7.6](s1f.7.6_a_false_fact_with_an_argument.md) | A written `(false k)`: `false` with an argument | kills root, the argument dropped, where a rule's is refused | S1f.5.23's T2 record, finding 1 | new 2026-11-10 |
| [S1f.7.7](s1f.7.7_an_absent_fact.md) | A written or concluded `(absent k)` | stores a fact no premise can read | S1f.5.23's T2 record, finding 2 | new 2026-11-10 |
| [S1f.7.8](s1f.7.8_a_solve_list_naming_nothing.md) | A `(solve …)` list that names nothing the program declares | `:hrules (zzz)` and the rest load in silence and move H | S1f.5.23's T2 record, finding 3 | new 2026-11-10 |
| [S1f.7.9](s1f.7.9_a_facts_rule_that_names_no_rule.md) | A fact's `:rule` that names no rule | `(p k :rule zzz)` stored as derived by a rule the program lacks | S1f.5.23's T2 record, finding 4 | new 2026-11-10 |

## How every stage runs

Every stage follows the method of P1f.5's re-opened stages, S1f.5.21 to S1f.5.24:

1. **T1, the confirmation and the census.** The finding is re-taken on the release binary at the stage's start
   and marked *confirmed*, *not reproduced* (with what changed) or *already fixed* (with the commit). Then the
   whole class is taken: every site the shape can stand in (a rule, an hrule, an obligation, a goal, a claim, a
   macro's body or argument) and every command that reads a program, the four `render` views, `kb save` and a
   saved `.einb` included.
2. **T2, the rows.** One witness per way the engine answers wrongly, under `examples/ein-bugs/wrong-answers/`,
   each with a corpus entry. Each is filed `open` in [`corpus/wrong_answers.toml`](../../../corpus/wrong_answers.toml)
   and held to the unchanged engine. Every golden the fix will move is named before it is re-blessed.
3. **T3, the fix**, then the rows flipped to `fixed`, and a test that holds the class on every command and fails
   with the fix taken out.
4. **T4, the pages**: the kernel pages the class touches, `defined_behaviour.md`, `00_ebnf.md`'s table where the
   grammar takes a refusal, and the stage's records.

A choice the finding leaves open, such as refusing a shape against giving it a meaning, is a Q-M1f row. How it
is taken is the user's call when the phase starts ([§ Status](#p1f7--the-wrong-answers-p1f5s-stages-found-outside-their-scope-a-stage-each)).
Each stage file names the direction that agrees with the rulings already made, so that a default, if the user
allows defaults, has a precedent.

## The order

Each stage edits the loader (`ein-ir/src/from_ir.rs`) or the engine beside it, so the stages run one at a time on
the main tree. The default order:

1. [S1f.7.5](s1f.7.5_a_macro_called_in_a_goal.md), the macro in a goal, first: once a goal's macros are expanded,
   the next stage's checks read the expanded goal.
2. [S1f.7.1](s1f.7.1_a_goals_variable_relation_in_a_nested_pattern.md), the goal's nested variable relation. Its
   census also lists every check a rule's `:match` gets at load and a goal's premise does not.
3. [S1f.7.3](s1f.7.3_a_connective_as_a_patterns_argument.md), the connective as a pattern's argument, which extends
   S1f.5.24's walk.
4. [S1f.7.4](s1f.7.4_the_negation_of_a_name_concluded.md), the negation of a name.
5. [S1f.7.2](s1f.7.2_an_hrule_that_reads_not.md), the hrule that reads `not`.

6. [S1f.7.6](s1f.7.6_a_false_fact_with_an_argument.md), then [S1f.7.7](s1f.7.7_an_absent_fact.md): both edit
   `ein.y`'s fact heads.
7. [S1f.7.8](s1f.7.8_a_solve_list_naming_nothing.md), then [S1f.7.9](s1f.7.9_a_facts_rule_that_names_no_rule.md).

Stages added later (S1f.7.10 onward) run after these unless one depends on another.

Where this phase falls against [P1f.6](../p1f.6_calculator/README.md) (the calculator, whose S1f.6.1 stops for the
user) is the user's call at P1f.5's close.

## Acceptance for the phase

- Every stage's finding is confirmed, not reproduced or already fixed, with its evidence in the stage's record.
- Every confirmed finding is a register row, filed `open` before its fix and `fixed` after it, or `open` and owned by
  a named later stage or followup with the user's ruling.
- Every refusal a stage adds reads the same on every command, in S1f.5.21's spelling.
- No corpus answer moves except a stage's own witnesses and the fixtures it names.
- `./run_tests.sh` green on each stage's commit.

## The ids

`S1f.7.<n>`, the stage numbers in the order the findings came: 1 to 5 for the batch of 2026-11-10, and 6 onward
for later findings (6 to 9 from S1f.5.23). Tasks are `T1f.7.<n>.<m>`. Questions are `Q-M1f.<n>` in
[M1f's open questions](../open_questions.md), continuing that file's numbering. Register rows continue the
register's `W` numbering.
