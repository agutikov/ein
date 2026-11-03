# P1e2.2 — The wrong answers, one stage each

**Estimate:** ~2.5 weeks: 13 stages, eight live and five dropped, **12.5
days**, since the revisit, [S1e2.1.7](../p1e2.1_seeing_the_answer/s1e2.1.7_the_review.md#revisited-2026-11-03), dropped S1e2.2.13 and the user's ruling on C1
restated S1e2.2.10 (2 → 1.5), both 2026-11-03 (§ Stages' S1e2.1.7 column). It
was 13.5 after [S1e2.1c.5](../p1e2.1c_the_region_search/s1e2.1c.5_wrong_answers_re_scoped.md)
re-scoped it the same day (its outcome column), and 27.5 before.
**Depends on:** [P1e2.1](../p1e2.1_seeing_the_answer/README.md), all of it,
and [P1e2.1a](../p1e2.1a_the_solution_region/README.md), which redefines the
answer as a region R
([Q-M1e2.7](../open_questions.md#q-m1e27--the-answer-is-a-region-of-the-lattice))
and restates every right answer under it before the review resumes. Since
2026-10-16 it also depends on [P1e2.1c](../p1e2.1c_the_region_search/README.md),
whose last stage re-scoped this one on 2026-11-03: W1, W2, W5 and W9 were closed
by S1e2.1a.5's switch, and their four stages are dropped, because the code
each fix was to change is the walk's, which no command reaches.
Since 2026-11-02 it also comes after P1e2.3 and P1e2.4, the user's order of
the milestone. No stage here starts until [S1e2.1.7](../p1e2.1_seeing_the_answer/s1e2.1.7_the_review.md)
has confirmed its case. That review may correct a right answer, withdraw a
case or split one, and this table is edited to match before anything is
fixed. **Since 2026-11-03 every case has its verdict**, the last three on the
user's rulings of that day ([S1e2.1.7 § The user's rulings](../p1e2.1_seeing_the_answer/s1e2.1.7_the_review.md#the-users-rulings-2026-11-03)), and the stages run in the order below.
**Blocks:** nothing outside the milestone. Two stages here touch ground M1f
also plans on, and the boundary is drawn per stage, in
[§ Where this phase meets M1f](#where-this-phase-meets-m1f).
**Source:** [`inventory.md`](../inventory.md), the survey of 2026-10-14. Every
case was re-run on the release binary at `71dd237`, and the case ids (W1–W12,
C1) are its own.

---

## What counts as a wrong answer here

A **well-formed** program, a run the CLI accepts, and an answer that is false
under the definitions the user has ruled on:
[Q-M1e.6](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e6--what-is-a-solution-and-what-is-a-model)
(what a solution and a model are),
[Q-M1e.7](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e7--the-read-out-prints-the-solution-kb-and-calls-it-a-model)
ruling A (the recorded object is the state, and engine-written negatives
count as entailed), and the 2026-08-25 ruling that a state that owes what it
cannot pay is `Open`, not `Contradiction`. The answer is one of:

| class | what is false | stages |
|---|---|---|
| **1 — verdict / `k`** | the verdict word, or the count of models | S1e2.2.1, .2, .3, .9 |
| **2 — model set** | a model missing, one that is not a model, a derivation lost | S1e2.2.4, .5, .6, .7, .10 |
| **3 — certification** | `exhausted = true` (or the unhedged wording) over a claim it does not license | S1e2.2.1, .2, .3, .8 |
| **4 — the core** | an unsat core other than the one the docs promise | S1e2.2.11, .12, .13 |

Surfaces that misstate a run whose answer is right (the inventory's S1–S9),
and the one reachable panic (P1), are not stages here. They are
[P1e2.3](../p1e2.3_surfaces_that_misreport_the_answer/README.md), a phase
since 2026-11-02, whose S1e2.3.1–.4 left none of them live (2026-11-03).

**Contested** cases are wrong under one reading of a ruling and right under
another. Each contested stage starts with the reading, taken by the user in
the review, and the fix follows from it. A contested case may close as
*not wrong* with no code change, and that is a result.

## Stages

Ordered by what a user meets first: wrong verdicts at **default** settings,
then wrong model sets, then certifications, then levers, then cores.

| ID | case | class | settings | carried | est. (was) | S1e2.1c.5 | S1e2.1.7 | ends with |
|---|---|---|---|---|---:|---|---|---|
| [S1e2.2.1](s1e2.2.1_maximal_not_recorded.md) | **W1** a maximal alive commitment is never recorded | 1 + 3 | **default** (and `-L`) | [Q-M1e.8](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e8--exhausted-certifies-the-lattice-not-the-model-set), review Q5 | 0 (4) d | **drop** | confirmed, but `alive-empty-interlayer`: re-opened, a refusal, S1e2.2.5's | a commitment whose every child died is recorded as a solution, at layer 0 too; `branching/15` answers k = 2; the 9 `-L` cells stop certifying a false `Contradiction` |
| [S1e2.2.2](s1e2.2.2_forced_positive_refutes.md) | **W2** the forced-positive cascade refutes the program | 1 + 3 + 4 | **default** | new | 0 (2) d | **drop** | confirmed | a singleton that contradicts root is concluded false, not promoted; `forced-2.ein` answers `Solution`, not a 0-fact `Contradiction` |
| [S1e2.2.3](s1e2.2.3_obligations_join_superset.md) | **W5** the obligations rung records a superset of a model, chosen by name order | 1 + 2 + 3 | **default** (any obligation program) | new | 0 (2.5) d | **drop** | confirmed | the join re-checks its subsets; a model set that is invariant under renaming a constant; lattice and tree agree |
| [S1e2.2.4](s1e2.2.4_fresh_name_unreachable.md) | **W4** a rule's fresh constant never becomes a hypothesis | 2 | **default** | [Q-M1e.21](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e21--a-rule-may-name-an-object-the-search-can-never-hypothesise-about) | 2.5 (3) d | keep | confirmed | Q-M1e.21's candidate taken; the fresh-name fixtures answer the model `{(q A Z),(q B Z)}` |
| [S1e2.2.5](s1e2.2.5_naf_upward_closure.md) | **W3** `dead` is not upward-closed under `absent` | 2 | **default** | [Q-M1e.9](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e9--is-dead-really-upward-closed-under-absent), F18, S1f.10.8 | 2.5 (3) d | shrink | confirmed; gains `W1/alive-empty-interlayer` | the language ruling taken first; then the program is refused, or the three mechanisms stop learning from a refutation that rests on an `absent` |
| [S1e2.2.6](s1e2.2.6_stale_naf_positive.md) | **W8** a positive derived under `absent` survives the commitment that falsifies its guard | 2 (contested) | **default** | new | 0.5 (2.5) d | restate | **deferred** (the user): both rows open, owned by F21; re-checks after S1e2.2.5's F2 | a reading; then no recorded state contains a fact whose `absent` premise the state contradicts, or the program is refused as unstratifiable |
| [S1e2.2.7](s1e2.2.7_binding_key_layout.md) | **W7** two activator layouts share a binding key, and a derivation is lost | 2 | **default** (release) | [Q-M1e.16](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e16--the-binding-key-compares-two-register-layouts-as-one) | 1.5 d | keep | confirmed | Q-M1e.16's fix taken; the §3.2 program derives four `noted` facts in either activator order, in both profiles |
| [S1e2.2.8](s1e2.2.8_uncovered_certified.md) | **W6** `exhausted = true` over relations the rung never hypothesises | 3 (contested) | **default** | M1d's eight measurements, #4 | 2 d | keep | confirmed: (a) *scoped*, ruled by the user | a reading; then the certification names its scope, or the rung covers what it certifies |
| [S1e2.2.9](s1e2.2.9_kill_cache_off_entailment.md) | **W9** with the kill cache off, a state is recorded as consistent because an entailment went unnoticed | 1 | lever `-K` | Q-M1e.7 ruling A | 0 (1) d | **drop** | confirmed | `-K` changes cost and never an answer, or it is refused for certified runs |
| [S1e2.2.10](s1e2.2.10_types_as_objects.md) | **C1** the untyped blind rung binds a type as an object | 2 (contested) | **default** | [Q-M1e.12](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e12--the-blind-rung-is-untyped-and-a-model-binds-a-type-as-an-object) | 1.5 (2) d | keep | **not wrong**: (a), ruled by the user; restated to the fixture and the warning | one of Q-M1e.12's three readings ruled, and applied |
| [S1e2.2.11](s1e2.2.11_alt_cap_core.md) | **W10** the alternatives cap decides which core is reported | 4 | default | [Q-M1e.15](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e15--the-alternatives-cap-decides-which-unsat-core-is-reported) | 1.5 d | keep | confirmed | Q-M1e.15's fix taken; the `alt-cap-core` pair report the same 2-fact core |
| [S1e2.2.12](s1e2.2.12_core_id_order.md) | **W11** which core is reported depends on interner id order | 4 | default | `corpus/fuzz_findings/` | 0.5 (1.5) d | shrink | confirmed: the user's first visit | a canonical choice among equal-size cores; the id-order sweep green on both fuzz findings at every seed |
| [S1e2.2.13](s1e2.2.13_empty_core.md) | **W12** an exhausted `Contradiction` reports a 0-fact core | 4 (contested) | default | S1e.3.1 § did not do | 0 (1) d | shrink | confirmed, fixed: **drop** | a reading; then the core names the facts the refutation rests on, or the read-out says why it cannot |

**The outcome column is [S1e2.1c.5](../p1e2.1c_the_region_search/s1e2.1c.5_wrong_answers_re_scoped.md#task-t1e21c52--the-stages-and-the-record)'s**
(2026-11-03), each with its reason there and a status line in the stage's
file. A **drop** is a stage whose rows are all `fixed`, by S1e2.1a.5's switch,
and whose fix was to code no command reaches, the walk's, which that stage's
step 5 deletes. Its rows keep `fixed` until the revisit reads them, with
S1e2.1a.5 as their `owner`, and its file stays until the milestone closes. A
**shrink** lost the part of its scope that named the walk, and a **restate**
had its question moved by a ruling (S1e2.2.6's, to F21's D2). The *est.*
column is the re-scoped estimate, with the old one in brackets. The *ends
with* column is as filed, and each stage file says what of it is left.
**The S1e2.1.7 column is the review's** (2026-11-03): the revisit's verdict
where a stated criterion decides the case, and the user's where none did
([S1e2.1.7 § The user's rulings](../p1e2.1_seeing_the_answer/s1e2.1.7_the_review.md#the-users-rulings-2026-11-03)). One more **drop**, S1e2.2.13, whose one row closed by
`solve_interface.md` § 0; S1e2.2.10 restated by the user's ruling on C1; and
`W1/alive-empty-interlayer` re-opened under *forbid*, its refusal S1e2.2.5's.

**W13 has no stage here.** *A config flag changes R*:
`enable-symmetric-mirror` off took the mirror's facts out of root's fixpoint
on `features/06_symmetric_native.ein`. P1e2.1a's
[S1e2.1a.6](../p1e2.1a_the_solution_region/s1e2.1a.6_engine_against_oracle.md)
found it under every mode, filed it, and fixed it in the same commit by
retiring the flag on the user's ruling of 2026-11-01, so this phase has
nothing left to do for it.

### Dependencies inside the phase

Re-taken by S1e2.1c.5 on 2026-11-03. Until then S1e2.2.1 came first, because W1
confounded three other cases. Since S1e2.1a.5's switch none of them is confounded:
`forced-2.ein` answers right, `alive-set-fresh-name.ein` answers W4's {∅}, and W9's
`-K` witnesses answer R. So the dependencies on S1e2.2.1, S1e2.2.2 and S1e2.2.3
went with those stages.

- **S1e2.2.5 before S1e2.2.6.** Both are about what a fact derived under
  `(absent …)` licenses. S1e2.2.5 applies *forbid*, ruled 2026-11-02, and its F2
  (whether a death that rests on an `absent` only through a derived fact counts,
  `via`) decides whether W8's two witnesses still load. S1e2.2.6's reading is
  F21's D2, deferred, and what is left of it is the revisit's.
- **S1e2.2.4 and S1e2.2.10 no longer share a sitting**: C1's reading was ruled on 2026-11-03, (a), so each runs alone. Until then: **S1e2.2.4 and S1e2.2.10 in one sitting.** Both ask what the candidate list's
  universe is, and answering one alone risks answering the other by accident.
- **S1e2.2.12 after S1e2.2.11**, if the second visit to W11 asks for a canonical
  tie-break among equal-size cores: it is written once.
- **S1e2.2.7, S1e2.2.8 and S1e2.2.11 are independent** (S1e2.2.13 dropped at the revisit) of everything
  above, and can run in parallel with any of it.

## Acceptance

**Per stage.** Every stage's acceptance includes these, and they are not
repeated in the stage files:

- **The register flips.** The case's rows in `corpus/wrong_answers.toml` go
  `open` → `fixed` (or `not-wrong`, with the review's reason). Any `:expect`
  that states today's wrong answer moves in the **same commit**.
- **The corpus sweep is diffed before and after.** Every declared run of every
  corpus entry, verdict, `k`, and the model set as facts. Every change is
  listed in the stage's outcome and explained. **An unexplained change is a
  stop, not a re-bless.** Every golden that moves is named in the stage file
  before it moves.
- **The case's bundle page is re-rendered** and shows the engine's answer
  equal to the right one.
- **Every sentence that stated the false premise is corrected**, and the
  inventory's § 8 lists the ones known at the start. A comment that argued
  for the defective behaviour goes with it.
- `./run_tests.sh` green.

**For the phase.** No register row is `open`, but a row whose reading the
user deferred to a followup: W8's two, `deferred` to F21 on 2026-11-03 (the
milestone's acceptance, amended that day). No wrong answer at default
settings remains. A lever may still change *cost*, and no lever changes an
answer the engine certifies.

## Where this phase meets M1f

| here | there | the line |
|---|---|---|
| S1e2.2.5 (W3), S1e2.2.6 (W8) | [S1f.10.8](../../m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.8_refutation_under_absent.md): *may a refutation rest on an `absent`?* | **the ruling moved to M1e2 on 2026-11-01**, as [P1e2.4](../p1e2.4_what_the_switch_left/README.md)'s S1e2.4.1, the user's call. S1f.10.8's T1 (narrow the 60 syntactic sites to the exposed set) and its ruling are what S1e2.2.5 needs before it can choose a fix; S1e2.2.5 applies the ruling, and S1f.10.8 keeps the corollary it writes into `absent_semantics.md` |
| S1e2.2.1 (W1) | [S1f.10.3](../../m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.3_the_restricted_join.md): the join, restricted | **dropped 2026-11-03 (S1e2.1c.5)**, and with it this row's *this first*. S1f.10.3 restricts `apriori.rs`'s join, which no route uses (T1e2.1c.4.4) and S1e2.1c.5's step 5 deletes. Whether S1f.10.3 survives is M1f's to re-scope, **parked in its header, not decided here** |
| S1e2.2.3 (W5) | S1f.10.3, again | **dropped 2026-11-03 (S1e2.1c.5)**: W5's row is fixed under the *literal* ruling, and the join it would have fixed is the walk's. As the row above |
| S1e2.2.10 (C1) | S1f.10.4: bounded groups | C1's ruling on what the blind rung's universe *is* is a premise of S1f.10.4. It is taken here because it changes answers |
