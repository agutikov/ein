# P1f.7 — The wrong answers P1f.5's stages found outside their scope, a stage each

**Milestone:** [M1f](../README.md)
**Status:** planned 2026-11-10, on the user's instruction of that day ([§ The instruction](#the-instruction)). No
stage has started. It runs after [P1f.5](../p1f.5_the_no_owner_items/README.md) closes. The coordinator stops at
that close (the user, 2026-11-10: *"stop in the end of the phase"*), so the user decides then when this phase
starts, and whether its questions stop the stage or take a default. **P1f.5 closed on 2026-11-11**
([its close](../p1f.5_the_no_owner_items/README.md#the-close-2026-11-11)): S1f.5.22, S1f.5.25, S1f.5.27 and S1f.5.28
found no wrong answer outside their scope, so this phase has the nine stages below. **Running since 2026-11-11**, on
the user's answer of that day, *"p1f.7 runs before p1f.6, it's questions take a default"*: it runs before P1f.6, and a question
a stage raises takes the stage's default, recorded as a Q-M1f row marked *defaulted* and shown at this phase's close
([§ How every stage runs](#how-every-stage-runs)). **S1f.7.8 is done** (2026-11-11, W76), and so are **S1f.7.5,
S1f.7.11 and S1f.7.1** (2026-11-11, W77 to W79), a query's `:goal` held to what a rule's `:match` is held to, and
**S1f.7.7, S1f.7.6, S1f.7.10 and S1f.7.3** (2026-11-11, W80 to W83), the value rule held in the grammar and the loader,
and **S1f.7.4 and S1f.7.2** (2026-11-11, W84 and W85), the two about `not`. **Re-planned 2026-11-11** on the user's rulings on the
[analysis of that day](#the-analysis-of-2026-11-11): S1f.7.8 runs first, S1f.7.9's fix moved to
[F24](../../followups/f24_written_provenance.md), and three findings of the analysis are S1f.7.10 to S1f.7.12.
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

## The value rule

The user, 2026-11-11, on what `(q (eq k j))` is for (*"(a), adopt the rule"*):

> **A value is a name, or a fact a state could hold: an atom of an ordinary relation, or its negation.**

A nested fact in Ein is a statement that some state could hold or deny: a stored negation, a hypothesis, what
someone says. The only ways back from it to a held fact are a pattern that matches it by shape and a conclusion
`(not ?s)`. So a head that no state can hold is never a nested head: `eq` and `neq` (computed), `absent` (a test),
`false` with an argument, `and` and `or` (connectives). `not` stays, since a negation is stored. It loses nothing a
program can say: a program that wants the shape names a relation of its own. Real quoting, of guards and
connectives too, would be a designed feature of [F5](../../followups/f5_rules_as_data.md) or
[F2](../../followups/f2_self_modifying_language.md), with its own syntax.

One sentence decides five stages: S1f.7.10 (`eq`), S1f.7.7 (`absent`), S1f.7.6 (`false k`), S1f.7.3 (a pattern's
argument is what a value can be) and S1f.7.4 (`not` of a name is the negation of no fact). The probe that settled
it, on the release binary at `4bf6da73`: with `(h (eq k j)) (h (eq k k))` and `:match (h ?s) :assert (not ?s)`,
`ein saturate --dump` stores `(not (eq k j))` and `(not (eq k k))` and no `(false)`, where S1f.5.7 has a concluded
`(not (eq …))` checked. S1f.7.10 files it as a row.

## Stages

| ID | title | finding | from | state |
|---|---|---|---|---|
| [S1f.7.1](s1f.7.1_a_goals_variable_relation_in_a_nested_pattern.md) | A goal's nested pattern whose relation is a variable: refused, as in a rule | `(query :goal (p (?R ?x)))` loads and matches nothing, in silence | S1f.5.28 item 12; S1f.5.21's T1 record | **done 2026-11-11**: W79, two rows filed and fixed; refused at load |
| [S1f.7.2](s1f.7.2_an_hrule_that_reads_not.md) | An hrule that reads `not`: one answer from both builds | a debug build stops (exit 101) where release answers | S1f.5.28 item 13; S1f.5.24's T2 record | **done 2026-11-11**: W85, three rows filed and fixed in one commit; the stale assertion removed, every hrule matched ahead where one reads `not` |
| [S1f.7.3](s1f.7.3_a_connective_as_a_patterns_argument.md) | A connective as a premise pattern's argument | `(q (and (p ?x)))` loads and never matches, in silence | S1f.5.28 item 14; S1f.5.24's T2 record | **done 2026-11-11**: W83, five rows filed and fixed; refused at load, after macro expansion |
| [S1f.7.4](s1f.7.4_the_negation_of_a_name_concluded.md) | A conclusion `(not ?x)` whose variable binds a name | stores `(not k)`, which no program can write | S1f.5.28 item 15; S1f.5.24's T2 record | **done 2026-11-11**: W84, seven rows filed and fixed; refused at load where the program shows a name, and the firing builds no negation of one |
| [S1f.7.5](s1f.7.5_a_macro_called_in_a_goal.md) | A macro called in a query's `:goal` | the call is never expanded, so the goal has no row, in silence | S1f.5.28 item 16; S1f.5.24's T2 record | **done 2026-11-11**: W77, four rows filed and fixed; expanded at load, the goal held to a rule's checks |
| [S1f.7.6](s1f.7.6_a_false_fact_with_an_argument.md) | A written `(false k)`: `false` with an argument | kills root, the argument stored (the finding said dropped), where a rule's is refused | S1f.5.23's T2 record, finding 1 | **done 2026-11-11**: W81, five rows filed and fixed; `false` takes no argument anywhere |
| [S1f.7.7](s1f.7.7_an_absent_fact.md) | A written or concluded `(absent k)` | stores a fact no premise can read | S1f.5.23's T2 record, finding 2 | **done 2026-11-11**: W80, five rows filed and fixed; refused at the parse, with a note pointing at `(not …)` |
| [S1f.7.8](s1f.7.8_a_solve_list_naming_nothing.md) | A `(solve …)` list that names nothing the program declares | `:hrules (zzz)` and the rest load in silence and move H | S1f.5.23's T2 record, finding 3 | **done 2026-11-11**: W76, five rows filed and fixed; refused at load |
| [S1f.7.9](s1f.7.9_a_facts_rule_that_names_no_rule.md) | A fact's `:rule` that names no rule | `(p k :rule zzz)` stored as derived by a rule the program lacks | S1f.5.23's T2 record, finding 4 | rows only since 2026-11-11: the fix is [F24](../../followups/f24_written_provenance.md)'s |
| [S1f.7.10](s1f.7.10_an_eq_fact_as_a_value.md) | `(eq k j)` as a value | `(q (eq k j))` is stored, and a pattern matches it by shape, where `(q (neq k j))` is a parse error | the analysis of 2026-11-11 | **done 2026-11-11**: W82, four rows filed and fixed; `(eq …)` is no value |
| [S1f.7.11](s1f.7.11_a_goal_over_a_relation_nothing_makes.md) | A goal over a relation nothing in the program makes | `(query :goal (zzz ?x))` loads and has no row, in silence | the analysis of 2026-11-11 | **done 2026-11-11**: W78, four rows filed and fixed; refused at load, and a claim over a derived relation loads |
| [S1f.7.12](s1f.7.12_a_macro_call_written_as_a_fact.md) | A macro call written as a fact | `(pp j)` is stored as a fact of `pp`, the macro not expanded | the analysis of 2026-11-11 | new 2026-11-11 |

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

**What a stage finds outside its scope** (the user, 2026-11-11: *"I don't want to review any new findings that
does not affect existing zebra problems - so if find anything - either fix in place if it is quick or record into
followups"*): if it moves no answer of an existing zebra program, the stage fixes it in place where that is quick,
and otherwise adds a row to [F25](../../followups/f25_roughness_found_outside_zebra.md). It is not a new stage and
not a question. Only a finding that moves a zebra program's answer is shown to the user. Stages that share enough
run in one subagent (*"may combine multiple stages together in a single subagent if they have enough in common"*):
S1f.7.8; then S1f.7.5, S1f.7.11 and S1f.7.1; then S1f.7.7, S1f.7.6, S1f.7.10 and S1f.7.3; then S1f.7.4 and
S1f.7.2; then S1f.7.12 and S1f.7.9.

A choice the finding leaves open, such as refusing a shape against giving it a meaning, is a Q-M1f row. **It takes
the stage's default** (the user, 2026-11-11, *"p1f.7 runs before p1f.6, it's questions take a default"*): the direction the stage
file names, which agrees with the rulings already made, recorded as a Q-M1f row marked *defaulted* and shown at the
phase's close. As in P1f.5, a default that would reverse one of the user's own rulings, or a step that is
irreversible or outward-facing, still stops for the user.

## The analysis of 2026-11-11

Before any stage started, the nine findings were re-taken on the release and debug binaries at `4bf6da73`, read-only,
and all nine reproduced. What each stage file did not say, and its stage must take in its T1:

| stage | what the analysis adds |
|---|---|
| S1f.7.8 | The largest: one misspelt character moves R in silence. On a probe with an hrule `guess`: `:hrules (guess (at Color House))` gives H of 3 candidates and R of 3 states; `gues` gives no candidate and R {∅}; `att` for `at` gives 4 candidates of an undeclared relation and 16 states, `functional at` no longer biting; `:no-hypothesis (like is-a)` for `likes` gives 58 candidates where 29. `:over` is not the analogue, the claim check is, and that check refuses a relation only a rule derives (S1f.7.11). An activator argument is a site too: a relation there can be checked, a value (`Colour`) cannot, and a row saying an `:hrules` entry proposes nothing at root would cover it. `examples/syntax/query-full.ein` names two hrules it does not declare |
| S1f.7.5 | The case an author writes is the stdlib's `unknown` and `forall` in a goal, and without the import it is silent where a rule is refused. `:expect (model)` on such a goal *holds*, and the right claim, `(model (p k))`, is refused at load. Whether the read-out prints the goal as written or expanded is the stage's question |
| S1f.7.1 | Under `absent` the unmatched pattern makes the goal *hold* in every state. The fix is `refuse_goal_relation_variables` descending into a pattern's arguments |
| S1f.7.3 | Under an outer `absent` the dead inner pattern is vacuously true and the rule **fires**, so R moves. The class is not one: `and`, `or`, `neq` as an argument can match nothing; `not` matches a stored negation and stays; `absent` and `eq` match stored nested facts today, so they follow S1f.7.7 and S1f.7.10. The same in an hrule (H empty) and a goal |
| S1f.7.7 | The case that matters: `(p a) (absent (p a))`, written where `(not (p a))` was meant, is alive with the constraint ignored. A written `(absent k)` adds `k` to the names, so H grows (R from 1 state to 3 on the probe), and an hrule concluding one splits R on a fact nothing reads |
| S1f.7.6 | The argument is **not** dropped: `(false k)` is stored and printed, and `k` is a name of the program. A premise or goal `(false ?s)`, `(not (false k))` and `(p (false k))` load too. `06_reserved_names.md` and `01-ein-graph/03_ein_model.md` describe `(false <witness>)` |
| S1f.7.4 | An **hrule** concluding `(not ?x)` puts a candidate `(not k)` in H: R has two identical states where one. A `--dump-answer` state file holding `(not k)` does not reload. Three corpus programs conclude `(not ?h)` with `?h` a fact (`examples/saturation/hypothesis-contradiction/*-disproved.ein`) and must keep loading. A static refusal cannot see every binding, so what the firing does with the rest is [F25](../../followups/f25_roughness_found_outside_zebra.md)'s R-1 since the user's instruction of 2026-11-11 |
| S1f.7.2 | Release is right: it agrees with the oracle on `(absent (not …))`, a positive `(not …)` and the stdlib's `unknown`. The assertion at `hrule.rs:130` is stale, since no command enumerates with the kill cache on. Remove it, or hold it only where the kill cache is on |
| S1f.7.9 | The larger defect is not the name: any written `:rule` or `:using` fact is a derivation with no premises and leaves every core (`unsat core ∅`). Moved to F24 |

Every finding moves an answer or what loads, so none is [F23](../../followups/f23_diagnostics_in_eins_words.md)'s,
and none is [F22](../../followups/f22_exact_core_past_the_cap.md)'s. The user's rulings on it, 2026-11-11:
*"yes S1f.7.9 moves to a new F24; yes the phase order changes to put S1f.7.8 first"*, and of the three findings with
no stage, *"yes - it become S1f.7.10 onward"*.

## The order

Each stage edits the loader (`ein-ir/src/from_ir.rs`) or the engine beside it, so the stages run one at a time on
the main tree. The order, since the user's ruling of 2026-11-11 put S1f.7.8 first:

1. [S1f.7.8](s1f.7.8_a_solve_list_naming_nothing.md), the `(solve …)` list that names nothing: the one a puzzle's
   author meets.
2. [S1f.7.5](s1f.7.5_a_macro_called_in_a_goal.md), the macro in a goal: once a goal's macros are expanded, the next
   stages' checks read the expanded goal.
3. [S1f.7.11](s1f.7.11_a_goal_over_a_relation_nothing_makes.md), then
   [S1f.7.1](s1f.7.1_a_goals_variable_relation_in_a_nested_pattern.md), whose census lists every check a rule's
   `:match` gets at load and a goal's premise does not.
4. [S1f.7.7](s1f.7.7_an_absent_fact.md), [S1f.7.6](s1f.7.6_a_false_fact_with_an_argument.md) and
   [S1f.7.10](s1f.7.10_an_eq_fact_as_a_value.md): the three edit `ein.y`'s fact and argument heads, and they settle
   which heads a nested fact may have.
5. [S1f.7.3](s1f.7.3_a_connective_as_a_patterns_argument.md), the connective as a pattern's argument, which
   extends S1f.5.24's walk and refuses what item 4 left no fact able to hold.
6. [S1f.7.4](s1f.7.4_the_negation_of_a_name_concluded.md), the negation of a name, then
   [S1f.7.2](s1f.7.2_an_hrule_that_reads_not.md), the hrule that reads `not`.
7. [S1f.7.12](s1f.7.12_a_macro_call_written_as_a_fact.md), the macro call written as a fact.
8. [S1f.7.9](s1f.7.9_a_facts_rule_that_names_no_rule.md), its rows only.

No stage is added later: a later finding is fixed in place or is F25's (§ How every stage runs).

This phase runs before [P1f.6](../p1f.6_calculator/README.md), the calculator, whose S1f.6.1 stops for the user (the
user, 2026-11-11, *"p1f.7 runs before p1f.6, it's questions take a default"*).

## Acceptance for the phase

- Every stage's finding is confirmed, not reproduced or already fixed, with its evidence in the stage's record.
- Every confirmed finding is a register row, filed `open` before its fix and `fixed` after it, or `open` and owned by
  a named later stage or followup with the user's ruling.
- Every refusal a stage adds reads the same on every command, in S1f.5.21's spelling.
- No corpus answer moves except a stage's own witnesses and the fixtures it names.
- `./run_tests.sh` green on each stage's commit.

## The ids

`S1f.7.<n>`, the stage numbers in the order the findings came: 1 to 5 for the batch of 2026-11-10, 6 to 9 from
S1f.5.23, and 10 to 12 from the analysis of 2026-11-11. Tasks are `T1f.7.<n>.<m>`. Questions are `Q-M1f.<n>` in
[M1f's open questions](../open_questions.md), continuing that file's numbering. Register rows continue the
register's `W` numbering.
