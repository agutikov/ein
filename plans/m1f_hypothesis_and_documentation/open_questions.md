# Open Questions — M1f

Questions **this milestone raises**, with sticky `Q-M1f.<n>` ids. Do not reuse
a closed id.

M1f was created on 2026-08-29 by moving two phases out of
[M1e](../../docs/history/m1e_review_processing/README.md). The first questions it raised came from
[S1f.5.5](p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md)'s paper grammar on 2026-11-08.

| id | question | state |
|---|---|---|
| [Q-M1f.1](#q-m1f1--g-14-is-a-bad-conclusion-that-never-fires-refused) | G-14: is a bad conclusion that never fires refused? | **ruled 2026-11-08, (b)**; S1f.5.5 and [S1f.5.18](p1f.5_the_no_owner_items/s1f.5.18_an_activator_binds_a_nested_fact.md) |
| [Q-M1f.2](#q-m1f2--the-55-files-the-grammar-refuses-that-todays-parser-accepts) | the 55 files the grammar refuses that today's parser accepts | **ruled 2026-11-08, (a)**; S1f.5.5 |
| [Q-M1f.3](#q-m1f3--three-wrong-answers-the-census-found-with-no-row) | three wrong answers the census found with no row | **ruled 2026-11-08, as recommended**; [S1f.5.0](p1f.5_the_no_owner_items/s1f.5.0_the_known_wrong_answers_filed.md) files W33–W35 |

**The questions those two phases already own are M1e's and keep their ids.**
They are not copied here and they are not renumbered — the stage files cite
them across the milestone boundary, which is what a sticky id is for:

| id | what it is | who owns it here |
|---|---|---|
| [Q-M1e.5](../../docs/history/m1e_review_processing/open_questions.md#q-m1e5--is-experimental-a-licence-to-ship-a-lying-surface) | is *experimental* a licence to ship a surface whose read-out is false? | [S1f.10.7](p1f.10_hypothesis_structure/s1f.10.7_tree_calibration_and_flag.md)'s `--traversal` flag waits on it |
| [Q-M1e.6](../../docs/history/m1e_review_processing/open_questions.md#q-m1e6--what-is-a-solution-and-what-is-a-model) | what is a **solution**, and what is a **model**? | decided 2026-08-28; [P1f.10](p1f.10_hypothesis_structure/README.md) is its first consumer |
| [Q-M1e.9](../../docs/history/m1e_review_processing/open_questions.md#q-m1e9--is-dead-really-upward-closed-under-absent) | is `dead` really upward-closed under `absent`? | the **language** half is [S1f.10.8](p1f.10_hypothesis_structure/s1f.10.8_refutation_under_absent.md)'s |
| [Q-M1e.10](../../docs/history/m1e_review_processing/open_questions.md#q-m1e10--two-config--flags-are-inert) | two `(config …)` flags are **inert** | raised by the shipped `S1e.5.1`; owner unassigned |
| [Q-M1e.11](../../docs/history/m1e_review_processing/open_questions.md#q-m1e11--what-happens-to-an-obligation-derived-under-a-hypothesis) | what happens to an obligation **derived under a hypothesis**? | **owned** by [S1f.10.6](p1f.10_hypothesis_structure/s1f.10.6_obligations_under_hypothesis.md) |

A question raised by an M1f stage from now on gets a `Q-M1f.<n>` here.

---

### Q-M1f.1 — G-14: is a bad conclusion that never fires refused?

**Raised** 2026-11-08 by S1f.5.5's step 2
([paper grammar, Q1](p1f.5_the_no_owner_items/s1f.5.5_paper_grammar.md#q1--g-14-with-assert-checked-by-the-grammar-is-a-bad-conclusion-that-never-fires-refused)).
With `:assert` checked by the grammar, a conclusion no fact can hold (`(q ?x :k v)`, `(q _)`, `?x`) is a
parse error whether or not its rule fires. One shape is left that no grammar sees: an activator a rule
*derives* binding a relation parameter to a fact (`g14/derived-activator.ein`), which today stops the run
where its plan is compiled, and answers (64 states) where no node derives it
(`g14/derived-activator-never.ein`). The options were (a) refused at the parse, the derived shape a refusal
where compiled; (b) as (a), and the derived shape refused at load by a static check, which also refuses
`derived-activator-never.ein`; (c) checked only when a rule fires. The agent recommended (a).

**Ruled 2026-11-08: (b).** The user: *"Q1 - b"*. The grammar's half is S1f.5.5's; the load-time check is
[S1f.5.18](p1f.5_the_no_owner_items/s1f.5.18_an_activator_binds_a_nested_fact.md)'s, which stops and shows the
user any corpus program it would refuse.

### Q-M1f.2 — The 55 files the grammar refuses that today's parser accepts

**Raised** 2026-11-08 by S1f.5.5's step 2
([paper grammar, Q2](p1f.5_the_no_owner_items/s1f.5.5_paper_grammar.md#q2--the-55-files-the-grammar-refuses-that-todays-parser-accepts)),
re-taking the stage's 34. None is a puzzle, a stdlib module or a `tests/` program. The agent proposed (a):
the 26 refusal fixtures whose refusal the grammar takes over stay refusal fixtures and move to
`examples/broken/`, except `reserved_open_direct`, which three fixtures import by path;
`derivation_cycle` and five `examples/syntax/` fixtures are rewritten; 17 register witnesses and 6
regression fixtures are the owning stages' (S1f.5.1, S1f.5.2, S1f.5.4, S1f.5.11). The alternatives were (b)
the 26 left in place with only their `.expected` moved, and (c) the 26 rewritten.

**Ruled 2026-11-08: (a).** The user: *"Q2 - a"*.

### Q-M1f.3 — Three wrong answers the census found with no row

**Raised** 2026-11-08 by S1f.5.5's step 1
([census § Findings outside the grammar](p1f.5_the_no_owner_items/s1f.5.5_census.md#findings-outside-the-grammar)):
an activator binding a value parameter to a nested fact leaves it unbound; `:assert ()` stores a fact
named `@empty`, which joins H; and `(absent ())` holds. The agent recommended filing all three now as
`open` rows, W33–W35, as a fourth task of S1f.5.0: W33 owned by a new stage, S1f.5.18, where the parameter
binds as any value does; W34 by [S1f.5.9](p1f.5_the_no_owner_items/s1f.5.9_an_empty_match_fires.md), whose
case table it corrects; W35 by S1f.5.5, whose grammar refuses `(absent ())`.

**Ruled 2026-11-08, as recommended.** The user: *"Q3 - as recommended"*.
