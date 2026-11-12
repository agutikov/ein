# P1g.6 — The engine requirements the logic turns out to have

**Milestone:** [M1g](../README.md)
**Status:** planned 2026-11-11, not started. **Its stage list is provisional on purpose:** it is settled by
[P1g.1](../p1g.1_survey_and_design/README.md)'s survey
([S1g.1.4](../p1g.1_survey_and_design/s1g.1.4_engine_requirements_and_the_design_page.md)) and the user's
rulings at that stop. What is below is the seed: the candidates known on the day of planning, each with
where it was found.
**Depends on:** P1g.1's stop. Each stage then runs when the phase that needs it reaches the need
([M1g § The order](../README.md#the-order)).

## Why it is a phase of this milestone

The user, 2026-11-11, on whether the engine work should be a milestone of its own: *"no M1h need, M1g already
contains everything I request"*. So the engine work the logic needs is here, beside the programs that show
why it is needed. (An M1h exists since 2026-11-12, [*beyond word problems*](../../m1h_beyond_word_problems/README.md);
it is another subject and not an engine milestone for the logic.) A requirement is built **because a stage of P1g.2 to P1g.5 cannot be done, or can only be
done by a detour, without it**, and each stage file names those stages. A requirement no stage needs is
ruled or given an owner at the milestone's close and is not built for its own sake.

## The discipline

Every change this phase makes to the engine or the language follows the repo's, restated because this is the
one phase of M1g that touches them:

- **A wrong answer is a register row before it is fixed** (`corpus/wrong_answers.toml`,
  `ein-cli/tests/wrong_answers.rs`): today's answer and the right one as data, the fix flipping the row.
  Most of this phase's candidates are *missing* behaviour, not wrong answers; the two kinds are told apart
  in the table.
- **A change to the grammar needs the user's ruling,** on real programs shown first. The grammar is
  generated from `ein.l` and `ein.y`; its pages are generated from it; a refusal the grammar takes on gets a
  row in the grammar's table.
- **Config never changes the answer** (the user's ruling of 2026-10-15). A bound, a cache or an index is a
  lever only if R, N, H, every claim and every exit code are the same with it and without, held by
  `lever_invariance`.
- **No native numbers.** [Q-M1e2.24](../../m1e2_correctness/open_questions.md#q-m1e224--are-there-numbers-in-ein)
  stands. Nothing here adds engine arithmetic, a number token or a range: a bound a program or a flag states
  is the *harness's* number, as a `-n N` is, and never a value in a program.
- **Every answer exits 0; only an error does not** (2026-10-16). A new outcome word has to be placed on one
  side of that line by the user.
- **One tool, consistent answers:** a new behaviour is the same under every command that reads a program,
  and `ein oracle` is held to it.
- **Minimum work per question:** nothing here makes a run do more than its question needs.
- Every sweep names the goldens it expects to move **before** blessing.

## The candidates, as seeded 2026-11-11

| id | what the logic wants and the engine lacks | found by | kind | stage |
|---|---|---|---|---|
| C-1 | **A word for a saturation that does not stop.** A rule that builds a term from a term can run for ever; the engine has no bound and no outcome for it. Nested facts make consistency itself undecidable | [F26](../../followups/f26_consistency_and_completeness.md), promotion 3; M1g's probe 3: `(nat ?n)` gives `(nat (s ?n))`, 8 388 505 facts when a 20 s timeout ended `ein saturate` (2026-11-11) | new behaviour; a read-out word; **the user's** ([Q-M1g.6](../open_questions.md#q-m1g6--the-word-for-a-saturation-that-does-not-stop)) | [S1g.6.1](s1g.6.1_a_saturation_that_does_not_stop.md) |
| C-2 | **The fact index does not key on a nested value,** and keys a premise on its first bound argument whatever it selects. A theory about formulas, proofs or terms joins on nested values by nature | E-3 and E-12 of the calculator's [`findings.md`](../../m1f_hypothesis_and_documentation/p1f.6_calculator/findings.md): right answers, quadratic time | performance; no answer may move | [S1g.6.2](s1g.6.2_nested_facts_at_scale.md) |
| C-3 | **A per-library cache.** `.einb` caches a whole program; a program that imports a large library of facts and rules re-reads it every run | the coordinator, 2026-11-11; [S1f.6.15](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.15_load_against_calculation.md)'s first measurement, load against calculation | performance; a private format | [S1g.6.2](s1g.6.2_nested_facts_at_scale.md) |
| C-4 | **The nesting limit.** The parser nests to 1 000; a rule derives deeper, and such a state file does not reload. Unary numbers and long proofs nest by nature | E-4; S1f.6.9 puts the calculator's side to the user | a limit to state or move; follows S1f.6.9's ruling | [S1g.6.2](s1g.6.2_nested_facts_at_scale.md) |
| C-5 | **Nested facts unfolded in the graph views.** `ein render kb` and `render rule` draw a nested fact as one opaque labelled node, so a formula, a proof term or a number has no structure in a drawing | the coordinator's finding of 2026-11-11; E-10 (no view of a library of facts can be drawn) is its neighbour | a view; no answer | [S1g.6.3](s1g.6.3_seeing_nested_facts.md) |
| C-6 | **Numbers and formulas rendered readably** in traces and `:why` text. A `:why` prints `(n (n z 1) 7)` for 17 and would print `(Imp (And p q) p)` for a formula | the coordinator's finding of 2026-11-11 | presentation; the natural-language pages' subject | [S1g.6.3](s1g.6.3_seeing_nested_facts.md) |
| C-7 | **Rules exported as mathematical formulas,** in the logical, relational and algebraic forms | [F15](../../followups/f15_math_formulae/ideas.md); [S1g.4.2](../p1g.4_first_order_logic/s1g.4.2_relation_algebra.md) writes the target by hand | a new view | [S1g.6.3](s1g.6.3_seeing_nested_facts.md) |
| C-8 | **Fresh names.** Ein never makes a name at run time; a witness or an arbitrary individual has to be a Skolem term, a nested fact over names in scope | [S1g.4.4](../p1g.4_first_order_logic/s1g.4.4_quantifier_rules.md); M1g's probe 5 (a Skolem term works, one level) | possibly nothing to build; **the user's** if anything ([Q-M1g.8](../open_questions.md#q-m1g8--fresh-names)) | [S1g.6.4](s1g.6.4_witnesses_and_fresh_names.md) |
| C-9 | **A hypothesis about a nested fact, named in `(solve :hrules …)`.** An activator's argument in the list is a name; `(suppose ((Not p)))` is a parse error, though an activator *fact* binds a nested fact since S1f.5.18 | M1g's probe 4b (2026-11-11); `ein.y`'s `HruleItem` | grammar; **the user's** ([Q-M1g.7](../open_questions.md#q-m1g7--a-hypothesis-about-a-nested-fact)); a marker fact is the detour | [S1g.6.4](s1g.6.4_witnesses_and_fresh_names.md) |
| C-10 | **Rules as data.** A rule cannot name, read or conclude a rule. A proof system that talks about Ein's own rules, a scoped induction hypothesis used as a rule, and any self-reference need it | [F5](../../followups/f5_rules_as_data.md); [`08_self_describing.md`](../../../docs/kernel/ir/03-ein-lang/08_self_describing.md) § 5 | new behaviour, large; to be priced, not assumed built | [S1g.6.5](s1g.6.5_rules_as_data_and_refutations.md) |
| C-11 | **A refutation is reported and never concluded; no rule reads another state.** A dead assumption is in N with its core, and root holds nothing of it; `Imp` introduction, reductio and proof by cases each conclude from what holds in *other* states | M1g's probe 4 (2026-11-11); [S1g.3.3](../p1g.3_proof_systems/s1g.3.3_natural_deduction.md) | new behaviour; **the user's** ([Q-M1g.5](../open_questions.md#q-m1g5--is-a-refuted-assumption-a-fact)) | [S1g.6.5](s1g.6.5_rules_as_data_and_refutations.md) |
| C-12 | **A search that knows *exactly one value per object*.** With each of n objects a hypothesis that owes one of k values, describing R visits the partial assignments too: 3ⁿ saturations for n two-valued inputs where the complete assignments are 2ⁿ, and no answer in 150 s at 16 inputs | question 25 of the calculator's [`questions.md`](../../m1f_hypothesis_and_documentation/p1f.6_calculator/questions.md#q25) and its table (2026-11-11); the user's reply of 2026-11-12 and round 2's item 25 | performance; no answer may move | [S1g.6.6](s1g.6.6_exactly_one_value_per_object.md) |

Not candidates, though they will be met: the read-out's wording for an unsatisfiable requirement (probe 1b;
it is right, and S1g.2.2 teaches it); a schema's unbound variable refused at load (§ 4.13; rightly); W39 on
`or` (F22's); cores made canonical (F22's); non-monotone programs decided (F21's). Each stays with its owner.

## The stages

| ID | title | candidates | needed by | status |
|---|---|---|---|---|
| [S1g.6.1](s1g.6.1_a_saturation_that_does_not_stop.md) | A saturation that does not stop: a bound, a word, and what a run then says | C-1 | every theory past P1g.2; S1g.4.5, S1g.5.4 | planned; **stops for the user** |
| [S1g.6.2](s1g.6.2_nested_facts_at_scale.md) | Nested facts at scale: the index, a library's cache, and the nesting limit | C-2, C-3, C-4 | S1g.2.1, S1g.2.3, S1g.3.x, S1g.4.3, S1g.5.1 | planned |
| [S1g.6.3](s1g.6.3_seeing_nested_facts.md) | Seeing nested facts: the graph views, readable numbers and formulas, and rules as formulas | C-5, C-6, C-7 | every page of the milestone that shows a run; S1g.4.2 | planned; its first task **stops for the user** |
| [S1g.6.4](s1g.6.4_witnesses_and_fresh_names.md) | Witnesses and fresh names: Skolem terms, hypotheses about nested facts, and what H holds of them | C-8, C-9 | S1g.3.3, S1g.3.5, S1g.4.4 | planned; two questions **for the user** |
| [S1g.6.5](s1g.6.5_rules_as_data_and_refutations.md) | Rules as data, and a refutation a rule can read: priced first | C-10, C-11 | S1g.3.3, S1g.5.3, S1g.5.4 | planned; a design and a price, **for the user**; a build only on their word |
| [S1g.6.6](s1g.6.6_exactly_one_value_per_object.md) | A cheaper search where each object takes exactly one value (added 2026-11-12) | C-12 | S1g.2.2, S1g.2.4 | planned |

The numbers S1g.6.7 onward are free for what the survey adds.

## Acceptance

- Every candidate of the list **as S1g.1.4 restated it** is in one of four states, visible from this table:
  **built** (with its tests, and its sweep showing which answers moved, expected none unless a register row
  says otherwise); **ruled not to be built**, with the user's words; **detoured**, with the detour named and
  the stages that took it; or **owned** by a followup that lists it.
- No answer of an existing corpus program moved without a register row or a ruling that says so; every
  golden re-banked was named before blessing.
- The kernel pages that describe a changed behaviour change in the same commit as the behaviour
  (`defined_behaviour.md`, `read_out.md`, `configuration.md`, the grammar pages), and their tests hold.
- The gate and the nightly are green on each stage's commit; a stage that changes a DOT writer is green
  under `utils/ci_container.sh` too.
