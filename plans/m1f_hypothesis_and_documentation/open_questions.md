# Open Questions — M1f

Questions **this milestone raises**, with sticky `Q-M1f.<n>` ids. Do not reuse
a closed id.

M1f was created on 2026-08-29 by moving two phases out of
[M1e](../../docs/history/m1e_review_processing/README.md). The first questions it raised came from
[S1f.5.5](p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md)'s paper grammar on 2026-11-08.

**Every row marked *defaulted* was accepted by the user on 2026-11-10, without review**, at P1f.5's close
([close_review.md](p1f.5_the_no_owner_items/close_review.md#decided-by-default)): *"I accept all decisions as is
without review. Please remember that any decision may be changed later due to new findings, new research results,
new requirements etc."* So a defaulted row stands as taken and is not a ruling. Any one may be reopened, naming
what bears on it. That acceptance covers the rows defaulted by then, up to Q-M1f.64; from Q-M1f.69 on, a row
defaulted by a stage the close re-opened is that stage's, recorded in its file.

| id | question | state |
|---|---|---|
| [Q-M1f.1](#q-m1f1--g-14-is-a-bad-conclusion-that-never-fires-refused) | G-14: is a bad conclusion that never fires refused? | **ruled 2026-11-08, (b)**; S1f.5.5 and [S1f.5.18](p1f.5_the_no_owner_items/s1f.5.18_an_activator_binds_a_nested_fact.md); **built 2026-11-09** by S1f.5.18 (T1f.5.18.3, W59): a program in which a rule could derive an activator binding a relation parameter to a fact is refused at load, read off the program (Q-M1f.41) |
| [Q-M1f.2](#q-m1f2--the-55-files-the-grammar-refuses-that-todays-parser-accepts) | the 55 files the grammar refuses that today's parser accepts | **ruled 2026-11-08, (a)**; S1f.5.5 |
| [Q-M1f.3](#q-m1f3--three-wrong-answers-the-census-found-with-no-row) | three wrong answers the census found with no row | **ruled 2026-11-08, as recommended**; [S1f.5.0](p1f.5_the_no_owner_items/s1f.5.0_the_known_wrong_answers_filed.md) files W33–W35; W33 **fixed 2026-11-09** by S1f.5.18 (T1f.5.18.2), W34 by S1f.5.9 (T1f.5.9.2) and W35 by S1f.5.5's swap (`8c3fe86e`) |
| [Q-M1f.4](#q-m1f4--removing-priority-moves-the-facts-of-cores-on-seven-corpus-programs) | removing `:priority` moves the facts of cores on seven corpus programs | **ruled 2026-11-08, (b)**: derive the order from the rules first; [S1f.5.15](p1f.5_the_no_owner_items/s1f.5.15_rule_priority.md) |
| [Q-M1f.5](#q-m1f5--the-alternatives-flag-moves-refuted-by-beside-an-unmoved-core) | the alternatives flag moves `refuted by` beside an unmoved core | **ruled 2026-11-08, (a)**: W36, open, owned by F22; S1f.5.15 files it |
| [Q-M1f.6](#q-m1f6--the-derived-agenda-holds-the-cores-the-fifo-boundary-does-not) | the derived agenda holds the cores; the FIFO boundary does not | **ruled 2026-11-08, (a)** (re-ruled the same day from (b)): plain FIFO, a core is an explanation, F22 makes it canonical; [S1f.5.15](p1f.5_the_no_owner_items/s1f.5.15_rule_priority.md) |
| [Q-M1f.7](#q-m1f7--where-may-a-program-write-the-kernels-dunders) | where may a program write the kernel's dunders? | **ruled 2026-11-08, (b)**: never bound; [S1f.5.5](p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md)'s T11 |
| [Q-M1f.8](#q-m1f8--the-parsers-remaining-cost-within-grmtools-or-past-it) | the parser's remaining cost: within grmtools, or past it? | **defaulted 2026-11-09 (the user's instruction): within grmtools; alternatives parked for the close**; [S1f.5.5](p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md)'s T12 |
| [Q-M1f.9](#q-m1f9--what-a-deleted-loader-line-leaves-behind) | what a deleted loader line leaves behind | **ruled 2026-11-09, (a)** (*"ok, keep (a), continue"*; defaulted the same day): an `expect`, `unreachable!` or `assert!` naming the grammar's production; a hand-built `Ast` the grammar would refuse panics, which no `ein` command reaches; [S1f.5.5](p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md)'s T5 |
| [Q-M1f.10](#q-m1f10--the-einb-ints-section-once-the-int-kind-is-gone) | the `.einb` `INTS` section, once the int kind is gone | **defaulted 2026-11-09 (the user's instruction): the section stays, written empty, and a file that holds an int is refused; no format bump**; [S1f.5.4](p1f.5_the_no_owner_items/s1f.5.4_digits_are_names.md)'s T1f.5.4.3 |
| [Q-M1f.11](#q-m1f11--a-template-reference-to-a-variable-no-reference-can-name) | a template reference to a variable no reference can name | **defaulted 2026-11-09 (the user's instruction): the loader refuses it, one line a reference**; [S1f.5.4](p1f.5_the_no_owner_items/s1f.5.4_digits_are_names.md)'s T1f.5.4.5 |
| [Q-M1f.12](#q-m1f12--which-surfaces-carry-natural-language-text) | which surfaces carry natural-language text | **defaulted 2026-11-09 (the user's instruction): those that carried it before the swap, each held byte for byte; none added**; [S1f.5.2](p1f.5_the_no_owner_items/s1f.5.2_strings_are_natural_language.md)'s T1f.5.2.2 |
| [Q-M1f.13](#q-m1f13--f3s-cause-reaches-a-disjunction-whose-row-is-it) | F3's cause reaches a disjunction: whose row is it? | **defaulted 2026-11-09 (the user's instruction): W39, `open`, deferred to F22, as W25 is**; [S1f.5.1](p1f.5_the_no_owner_items/s1f.5.1_bare_underscore_refused.md)'s T1f.5.1.3 |
| [Q-M1f.14](#q-m1f14--which-keyword-pair-arms-go-once-no-term-holds-one) | which keyword-pair arms go, once no term holds one? | **defaulted 2026-11-09 (the user's instruction): the drops, the validator's skips and the renderers' arms; the walkers that descend into a pair's value stay**; [S1f.5.11](p1f.5_the_no_owner_items/s1f.5.11_closed_keywords.md)'s T1f.5.11.2 |
| [Q-M1f.15](#q-m1f15--where-does-a-position-in-a-containers-stored-program-point) | where does a position in a container's stored program point? | **defaulted 2026-11-09 (the user's instruction): it names the container, `<program stored in x.einb>`, and the stored text's line; a source position needs a format change, not made**; [S1f.5.5](p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md)'s T1f.5.5.13 |
| [Q-M1f.16](#q-m1f16--which-of-the-reads-defects-are-register-rows) | which of the read's defects are register rows? | **defaulted 2026-11-09 (the user's instruction): each refusal of a program another command answers (W40's six, W41); the lines and exits of a file every command refuses are held by a test**; [S1f.5.5](p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md)'s T1f.5.5.13 |
| [Q-M1f.17](#q-m1f17--what-a-view-that-never-loads-reads-from-a-container) | what a view that never loads reads from a container | **defaulted 2026-11-09 (the user's instruction): the program it stores, resolved and import-flattened**; [S1f.5.5](p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md)'s T1f.5.5.13 |
| [Q-M1f.18](#q-m1f18--which-object-the-word-model-names-on-the-read-out) | which object the word *model* names on the read-out (Q-M1e.7's read-out half) | **defaulted 2026-11-09 (the user's instruction): (a), document both objects; nothing printed moves**; [S1f.5.16](p1f.5_the_no_owner_items/s1f.5.16_the_read_out_reference.md) |
| [Q-M1f.19](#q-m1f19--a-texts-line-break-on-the-read-out) | a text's line break on the read-out | **defaulted 2026-11-09 (the user's instruction): each line at the text's column, each counted against stdout's 100**; [S1f.5.16](p1f.5_the_no_owner_items/s1f.5.16_the_read_out_reference.md)'s T1f.5.16.5 |
| [Q-M1f.20](#q-m1f20--route-2s-licence-no-licence-on-one-row-l7-the-cube-on-another) | route 2's licence: *no licence* on one row, `L7 the cube` on another | **defaulted 2026-11-09 (the user's instruction): documented, not moved**; [S1f.5.16](p1f.5_the_no_owner_items/s1f.5.16_the_read_out_reference.md) |
| [Q-M1f.21](#q-m1f21--regionstoppedlayer-on-two-routes) | `region.stopped.layer` on two routes | **defaulted 2026-11-09 (the user's instruction): documented per route, not moved**; [S1f.5.16](p1f.5_the_no_owner_items/s1f.5.16_the_read_out_reference.md) |
| [Q-M1f.22](#q-m1f22--a-class-label-of-sixteen-characters-glues-to-its-value) | a class label of sixteen characters glues to its value | **defaulted 2026-11-09 (the user's instruction): documented, not fixed here; the stage that fixes it files its row first**; [S1f.5.16](p1f.5_the_no_owner_items/s1f.5.16_the_read_out_reference.md). **Fixed 2026-11-09** by [S1f.5.6](p1f.5_the_no_owner_items/s1f.5.6_the_empty_state_decides_the_minimal_line.md)'s T1f.5.6.6 (`31e9c7f7`), (b): W42 filed by T1f.5.6.5, `-H`'s form its second row ([Q-M1f.24](#q-m1f24--a-second-surface-that-glues-another-row-of-w42-or-a-case-of-its-own)) |
| [Q-M1f.23](#q-m1f23--what-the-route-row-names-where-root-decides-a-flagged-programs-minimal-line) | what the `route` row names where root decides a flagged program's minimal line | **defaulted 2026-11-09 (the user's instruction): (a), the cube and `L7 the cube`, as `-n 1` printed; no new licence**; [S1f.5.6](p1f.5_the_no_owner_items/s1f.5.6_the_empty_state_decides_the_minimal_line.md)'s T1f.5.6.2 |
| [Q-M1f.24](#q-m1f24--a-second-surface-that-glues-another-row-of-w42-or-a-case-of-its-own) | a second surface that glues: another row of W42, or a case of its own? | **defaulted 2026-11-09 (the user's instruction): (a), W42's second row, `W42/h-long-relation`**; [S1f.5.6](p1f.5_the_no_owner_items/s1f.5.6_the_empty_state_decides_the_minimal_line.md)'s T1f.5.6.5 |
| [Q-M1f.25](#q-m1f25--the-state-routes-members-of-n-it-did-not-saturate-whose-preparation-takes-their-cores) | the state route's members of N it did not saturate: whose preparation takes their cores? | **defaulted 2026-11-09 (the user's instruction): (a), the walk's snapshot, after the walk; 58 cores move on 3 programs**; [S1f.5.12](p1f.5_the_no_owner_items/s1f.5.12_cores_for_n_only.md)'s T1f.5.12.1 |
| [Q-M1f.26](#q-m1f26--not-kept-retires-and-what-a-member-with-no-core-prints) | `not-kept` retires, and what a member with no core prints | **defaulted 2026-11-09 (the user's instruction): (a), the status goes, and a renderer refuses a member with no core**; [S1f.5.12](p1f.5_the_no_owner_items/s1f.5.12_cores_for_n_only.md)'s T1f.5.12.1 |
| [Q-M1f.27](#q-m1f27--what-program-a-einbs-answer-directory-writes) | what program a `.einb`'s answer directory writes | **defaulted 2026-11-09 (the user's instruction): (a), the program the container stores, as every view of a container reads it**; [S1f.5.12](p1f.5_the_no_owner_items/s1f.5.12_cores_for_n_only.md)'s T1f.5.12.5 |
| [Q-M1f.28](#q-m1f28--is-a-not-dead-nodes-core-written--a-wrong-answer) | is a not-dead node's core written `[]` a wrong answer? | **defaulted 2026-11-09 (the user's instruction): (a), no register row: a spelling, fixed with no answer moving**; [S1f.5.8](p1f.5_the_no_owner_items/s1f.5.8_no_contradiction_no_core.md)'s T1f.5.8.1 |
| [Q-M1f.29](#q-m1f29--a-dead-roots-core-under---count) | a dead root's core under `--count` | **defaulted 2026-11-09 (the user's instruction): (a), `null`, as the question prints no core; documented**; [S1f.5.8](p1f.5_the_no_owner_items/s1f.5.8_no_contradiction_no_core.md)'s T1f.5.8.1 |
| [Q-M1f.30](#q-m1f30--a-form-that-holds-a-variable-as-a-guards-argument) | a form that holds a variable, as a guard's argument | **defaulted 2026-11-09 (the user's instruction): refused at load, in a rule and in a goal; a fact that holds no variable is answered**; [S1f.5.7](p1f.5_the_no_owner_items/s1f.5.7_eq_and_neq_are_relations.md)'s T1f.5.7.1 |
| [Q-M1f.31](#q-m1f31--not-eq-x-y-a-stored-negation-or-neq-x-y) | `(not (eq x y))`: a stored negation, or `(neq x y)`? | **defaulted 2026-11-09 (the user's instruction): `(neq x y)`, in every place**; [S1f.5.7](p1f.5_the_no_owner_items/s1f.5.7_eq_and_neq_are_relations.md)'s T1f.5.7.1 |
| [Q-M1f.32](#q-m1f32--an-hrules-eq-or-neq-instance) | an hrule's `eq` or `neq` instance | **defaulted 2026-11-09 (the user's instruction): no hypothesis**; [S1f.5.7](p1f.5_the_no_owner_items/s1f.5.7_eq_and_neq_are_relations.md)'s T1f.5.7.1 |
| [Q-M1f.33](#q-m1f33--a-written-fact-of-eq) | a written fact of `eq` | **defaulted 2026-11-09 (the user's instruction): a conclusion with no premise, checked, never stored**; [S1f.5.7](p1f.5_the_no_owner_items/s1f.5.7_eq_and_neq_are_relations.md)'s T1f.5.7.1 |
| [Q-M1f.34](#q-m1f34--where-a-guards-free-variable-is-bound-beside-an-absent) | where a guard's free variable is bound, beside an `(absent …)` | **defaulted 2026-11-09 (the user's instruction): where it is written, as a relation premise binds**; [S1f.5.7](p1f.5_the_no_owner_items/s1f.5.7_eq_and_neq_are_relations.md)'s T1f.5.7.1 |
| [Q-M1f.35](#q-m1f35--a-goals-absent--whose-row-is-it) | a goal's `(absent …)`: whose row is it? | **defaulted 2026-11-09 (the user's instruction): S1f.5.7's, filed (W56) and fixed in a task of its own**; [S1f.5.7](p1f.5_the_no_owner_items/s1f.5.7_eq_and_neq_are_relations.md)'s T1f.5.7.5 |
| [Q-M1f.36](#q-m1f36--what-the-refusal-replaces-which-of-it-is-a-register-row) | what the refusal replaces: which of it is a register row? | **defaulted 2026-11-09 (the user's instruction): a row per way the engine answers today, W57's four**; [S1f.5.10](p1f.5_the_no_owner_items/s1f.5.10_a_conclusion_variable_must_be_bound.md)'s T1f.5.10.1 |
| [Q-M1f.37](#q-m1f37--fireerrorunboundvars-last-path-w33s-cause) | `FireError::UnboundVar`'s last path: W33's cause | **defaulted 2026-11-09 (the user's instruction): it stays until S1f.5.18, which retires it; W58 filed, S1f.5.18's**; [S1f.5.10](p1f.5_the_no_owner_items/s1f.5.10_a_conclusion_variable_must_be_bound.md)'s T1f.5.10.1 |
| [Q-M1f.38](#q-m1f38--the-refusals-line-its-separator-and-a-disjunction) | the refusal's line: its separator, and a disjunction | **defaulted 2026-11-09 (the user's instruction): ` — `, and the disjuncts that leave the variable unbound named**; [S1f.5.10](p1f.5_the_no_owner_items/s1f.5.10_a_conclusion_variable_must_be_bound.md)'s T1f.5.10.1 |
| [Q-M1f.39](#q-m1f39--the-one-corpus-program-the-check-refuses) | the one corpus program the check refuses | **defaulted 2026-11-09 (the user's instruction): `syntax/rule-nary-and-or` rewritten, its answer unmoved**; [S1f.5.10](p1f.5_the_no_owner_items/s1f.5.10_a_conclusion_variable_must_be_bound.md)'s T1f.5.10.1 |
| [Q-M1f.40](#q-m1f40--where-the-fuzz-property-lives-and-what-it-reads) | where the fuzz property lives, and what it reads | **defaulted 2026-11-09 (the user's instruction): `fuzz_properties`, over every plan an accepted program compiles**; [S1f.5.10](p1f.5_the_no_owner_items/s1f.5.10_a_conclusion_variable_must_be_bound.md)'s T1f.5.10.1 |
| [Q-M1f.41](#q-m1f41--what-can-hold-a-fact-means-for-g-14s-derived-activator) | what *can hold a fact* means, for G-14's derived activator | **defaulted 2026-11-09 (the user's instruction): a declared relation's argument, one inside a fact pattern and one of a relation a parameter names; a marker's argument and a rule's parameter inferred from what the program passes on**; [S1f.5.18](p1f.5_the_no_owner_items/s1f.5.18_an_activator_binds_a_nested_fact.md)'s T1f.5.18.1 |
| [Q-M1f.42](#q-m1f42--a-conclusion-whose-relation-is-a-parameter) | a conclusion whose relation is a parameter | **defaulted 2026-11-09 (the user's instruction): followed into every rule of its arity whose name the program writes as an argument**; [S1f.5.18](p1f.5_the_no_owner_items/s1f.5.18_an_activator_binds_a_nested_fact.md)'s T1f.5.18.1 |
| [Q-M1f.43](#q-m1f43--planactivator_args-once-a-nested-fact-binds) | `Plan::activator_args`, once a nested fact binds | **defaulted 2026-11-09 (the user's instruction): every argument, as values, the compile key's list**; [S1f.5.18](p1f.5_the_no_owner_items/s1f.5.18_an_activator_binds_a_nested_fact.md)'s T1f.5.18.1 |
| [Q-M1f.44](#q-m1f44--what-the-load-time-check-replaces-which-of-it-is-a-register-row) | what the load-time check replaces: which of it is a register row? | **defaulted 2026-11-09 (the user's instruction): a row per way the engine answers today, W59's two**; [S1f.5.18](p1f.5_the_no_owner_items/s1f.5.18_an_activator_binds_a_nested_fact.md)'s T1f.5.18.1 |
| [Q-M1f.45](#q-m1f45--the-and-refusals-line-and-or) | the `(and)` refusal's line, and `(or)` | **defaulted 2026-11-09 (the user's instruction): a note on the parse error naming the rule and the clause, `(or)` given one too**; [S1f.5.9](p1f.5_the_no_owner_items/s1f.5.9_an_empty_match_fires.md)'s T1f.5.9.1 |
| [Q-M1f.46](#q-m1f46--a-relation-parameter-bound-to-eq-at-another-arity) | a relation parameter bound to `eq` at another arity | **defaulted 2026-11-09 (the user's instruction): refused at load wherever a parameter can hold `eq`, read as S1f.5.18 reads relation names, premises and conclusions alike; a row per way the engine answers, W60's four**; [S1f.5.9](p1f.5_the_no_owner_items/s1f.5.9_an_empty_match_fires.md)'s T1f.5.9.1 |
| [Q-M1f.47](#q-m1f47--a-written-fact-of-eq-at-another-arity) | a written fact of `eq` at another arity | **defaulted 2026-11-09 (the user's instruction): refused at load, beside Q-M1f.33's check; W61, this stage's**; [S1f.5.9](p1f.5_the_no_owner_items/s1f.5.9_an_empty_match_fires.md)'s T1f.5.9.1 |
| [Q-M1f.48](#q-m1f48--how-a-text-surface-writes-the-empty-core) | how a text surface writes the empty core | **defaulted 2026-11-09 (the user's instruction, on the coordinator's note): `∅` on every text surface, where a dead node's core was taken and holds no fact**; [S1f.5.9](p1f.5_the_no_owner_items/s1f.5.9_an_empty_match_fires.md)'s T1f.5.9.2 |
| [Q-M1f.50](#q-m1f50--where-the-heavy_dump-line-goes-and-which-entries-carry-the-flag) | where the `heavy_dump` line goes, and which entries carry the flag | **defaulted 2026-11-09 (the user's instruction): (a), 40 000 state files, in the gap above the six `slow` `square-*`; five entries flagged, `features/01_not_and_absent` among them**; [S1f.5.13](p1f.5_the_no_owner_items/s1f.5.13_large_dumps_to_the_slow_tier.md)'s T1f.5.13.1 |
| [Q-M1f.51](#q-m1f51--three-more-tests-whose-slow-half-no-ci-tier-runs) | three more tests whose slow half no CI tier runs | **defaulted 2026-11-09 (the user's instruction): (a), recorded, not scheduled; parked for the phase's close**; [S1f.5.13](p1f.5_the_no_owner_items/s1f.5.13_large_dumps_to_the_slow_tier.md)'s T1f.5.13.3. **Taken by [S1f.5.27](p1f.5_the_no_owner_items/s1f.5.27_the_nightly_reduced.md) on 2026-11-10, (b) by tier**: `region`'s and `obligation_rung`'s slow halves are CI nightly steps, `route_invariance`'s a dev-host step, and `nightly_steps.rs`' `every_slow_half_runs_on_a_tier` fails on a target that reads `EIN_CORPUS_SLOW` and runs on neither |
| [Q-M1f.52](#q-m1f52--the-nightlys-review-bundle-and-the-registers-runs-that-are-not-a-solve) | the nightly's review bundle and the register's runs that are not a solve | **defaulted 2026-11-09 (the user's instruction): (a), reported, not fixed here; the nightly is red at `utils/review_bundle.py` since `fc53818f`**; [S1f.5.13](p1f.5_the_no_owner_items/s1f.5.13_large_dumps_to_the_slow_tier.md)'s T1f.5.13.3. **Re-opened the same day as (b)**, taken by default on the user's instruction of 2026-11-09: placed as a stage of its own, [S1f.5.19](p1f.5_the_no_owner_items/s1f.5.19_the_review_bundle_and_runs_that_are_not_a_solve.md); **built 2026-11-09** (`00c4ed34`): the bundle builds all 175 cases and the nightly is green |
| [Q-M1f.53](#q-m1f53--how-the-per-commit-gate-catches-a-register-row-the-bundle-cannot-read) | how the per-commit gate catches a register row the bundle cannot read | **defaulted 2026-11-09 (the user's instruction): (b), the smoke builds a command case and reads every register row with no run**; [S1f.5.19](p1f.5_the_no_owner_items/s1f.5.19_the_review_bundle_and_runs_that_are_not_a_solve.md) |
| [Q-M1f.54](#q-m1f54--the-2026-11-06-inventorys-evidence-in-buildreview) | the 2026-11-06 inventory's evidence in `build/review/` | **defaulted 2026-11-09 (the user's instruction): (b), moved intact to `build/inventory-2026-11-06/`, and `build/review/` rebuilt**; [S1f.5.19](p1f.5_the_no_owner_items/s1f.5.19_the_review_bundle_and_runs_that_are_not_a_solve.md) |
| [Q-M1f.55](#q-m1f55--a-container-on-a-solves-now) | a `container` on a solve's `now` | **defaulted 2026-11-09 (the user's instruction): (a), the case page runs the container, as `wrong_answers.rs` does**; [S1f.5.19](p1f.5_the_no_owner_items/s1f.5.19_the_review_bundle_and_runs_that_are_not_a_solve.md) |
| [Q-M1f.56](#q-m1f56--what-the-nl-census-counts-which-fact-set-and-which-registers) | what the NL census counts: which fact set, and which registers | **defaulted 2026-11-10 (the user's instruction of 2026-11-09): (a), every fact of every state file, and four registers, puzzle, stdlib, kernel and undeclared**; [S1f.5.14](p1f.5_the_no_owner_items/s1f.5.14_nl_required.md) |
| [Q-M1f.57](#q-m1f57--which-fixtures-get-a-why) | which fixtures get a `:why` | **defaulted 2026-11-10 (the user's instruction of 2026-11-09): (a), every file the loader reads, on the form's last line in a refused fixture; a file refused at the parse and the records under `plans/` and `docs/history/` are left**; [S1f.5.14](p1f.5_the_no_owner_items/s1f.5.14_nl_required.md) |
| [Q-M1f.58](#q-m1f58--where-a-fact-is-rendered-in-english-and-what-it-reads-from) | where a fact is rendered in English, and what it reads from | **defaulted 2026-11-10 (the user's instruction of 2026-11-09): (a), a state file's comments and `--trace`'s *Derives* line, from the fact's first recorded justification; stdout does not move**; [S1f.5.14](p1f.5_the_no_owner_items/s1f.5.14_nl_required.md) |
| [Q-M1f.59](#q-m1f59--what-a-macros-why-renders) | what a macro's `:why` renders | **defaulted 2026-11-10 (the user's instruction of 2026-11-09): (a), nothing: it documents the pattern, and the rule that used it says what it meant there**; [S1f.5.14](p1f.5_the_no_owner_items/s1f.5.14_nl_required.md) |
| [Q-M1f.60](#q-m1f60--a-relation-nobody-declares) | a relation nobody declares | **defaulted 2026-11-10 (the user's instruction of 2026-11-09): (a), no slot: a derived fact reads by its reason, a stated one falls back, counted under `stdlib` or `undeclared`**; [S1f.5.14](p1f.5_the_no_owner_items/s1f.5.14_nl_required.md) |
| [Q-M1f.61](#q-m1f61--a-template-per-goal-conjunct) | a template per goal conjunct | **defaulted 2026-11-10 (the user's instruction of 2026-11-09): (a), no change**; [S1f.5.14](p1f.5_the_no_owner_items/s1f.5.14_nl_required.md) |
| [Q-M1f.62](#q-m1f62--the-option-that-loads-a-program-without-a-why-once-the-default-is-true) | the option that loads a program without a `:why`, once the default is true | **defaulted 2026-11-10 (the user's instruction of 2026-11-09): (a), `--no-require-why` replaces `--require-why`, and `LoadOptions::default()` requires**; [S1f.5.14](p1f.5_the_no_owner_items/s1f.5.14_nl_required.md) |
| [Q-M1f.63](#q-m1f63--what-each-program-generator-does-about-why) | what each program generator does about `:why` | **defaulted 2026-11-10 (the user's instruction of 2026-11-09): (a), the engine fuzzers and synthesisers write an honest mechanical `:why` on every form, keeping their random draws; `fuzz_properties` keeps its draws**; [S1f.5.14](p1f.5_the_no_owner_items/s1f.5.14_nl_required.md) |
| [Q-M1f.64](#q-m1f64--existing-whys-that-are-placeholders-or-name-nothing) | existing `:why`s that are placeholders or name nothing | **defaulted 2026-11-10 (the user's instruction of 2026-11-09): (a), each fixed, the references by the check's commit and the placeholders by the migration; none is a register row**; [S1f.5.14](p1f.5_the_no_owner_items/s1f.5.14_nl_required.md) |
| [Q-M1f.65](#q-m1f65--may-a-comment-hold-unicode) | may a comment hold Unicode? | **ruled 2026-11-10, (b)**: a comment may hold Unicode; [S1f.5.22](p1f.5_the_no_owner_items/s1f.5.22_unicode_only_in_a_string.md) |
| [Q-M1f.66](#q-m1f66--may-open-be-written-as-a-value) | may `open` be written as a value? | **ruled 2026-11-10, (a)**: refused; `open` is written only as a rule's verdict; [S1f.5.23](p1f.5_the_no_owner_items/s1f.5.23_open_is_reserved.md) |
| [Q-M1f.67](#q-m1f67--where-is-a-macro-body-that-no-rule-could-use-refused) | where is a macro body that no rule could use refused? | **ruled 2026-11-10**, in the user's words: an unused malformed macro is allowed, and what matters is that `(eq <arg>)` is never executed; [S1f.5.24](p1f.5_the_no_owner_items/s1f.5.24_a_macro_body_checked_where_a_rule_is.md) |
| [Q-M1f.68](#q-m1f68--what-ein-does-when-its-reader-closes-stdout) | what Ein does when its reader closes stdout | **ruled 2026-11-10, (b)**: handle `SIGPIPE` properly, its default disposition restored on the binary; [S1f.5.25](p1f.5_the_no_owner_items/s1f.5.25_a_closed_stdout.md); **built 2026-11-10** by S1f.5.25 (T1f.5.25.2, W75): `ein-cli/src/main.rs`'s `restore_sigpipe`, one audited `libc::signal(SIGPIPE, SIG_DFL)` as `main`'s first statement, Unix only |
| [Q-M1f.69](#q-m1f69--what-the-guides-pin-reads-in-a-program-block) | what the guide's pin reads in a program block | **defaulted 2026-11-10 (the stage's default, S1f.5.26): (a), the text decides: a block that writes `…` outside a string is an excerpt, a top-level form with one is elided; an excerpt's whole forms load together where they stand, and a whole program loads and `ein solve` answers it**; [S1f.5.26](p1f.5_the_no_owner_items/s1f.5.26_the_tutorial_runs.md) |
| [Q-M1f.70](#q-m1f70--what-the-stage-does-with-what-t1-found-false-beside-the-blocks) | what the stage does with what T1 found false beside the blocks | **defaulted 2026-11-10 (the stage's default, S1f.5.26): (a), each fixed in the page: a block that misquotes its file or answers otherwise than its page says, and four sentences of prose; the test does not hold them**; [S1f.5.26](p1f.5_the_no_owner_items/s1f.5.26_the_tutorial_runs.md) |
| [Q-M1f.71](#q-m1f71--does-a-view-draw-a-program-the-loader-refuses) | does a view draw a program the loader refuses? | **defaulted 2026-11-10 (the stage's default, S1f.5.21): (a), it refuses it: `render program`, `render rules`, `render rule` and `render constraints` read the program through the loader first, refuse what it refuses with its line, exit 1, and take `--no-require-why` as the other seven commands that load do**; [S1f.5.21](p1f.5_the_no_owner_items/s1f.5.21_a_compile_error_in_eins_words.md) |
| [Q-M1f.72](#q-m1f72--the-line-a-moved-refusal-prints) | the line a moved refusal prints | **defaulted 2026-11-10 (the stage's default, S1f.5.21): (a), the loader's lines as S1f.5.9's, .10's and .18's print, `rule '<name>': …` behind `kb load error: `, no position; the compiler's remaining line `<file>:<line>:<col>: rule '<name>': …`, its form as Ein text**; [S1f.5.21](p1f.5_the_no_owner_items/s1f.5.21_a_compile_error_in_eins_words.md) |
| [Q-M1f.73](#q-m1f73--how-the-loader-counts-the-register-ceiling) | how the loader counts the register ceiling | **defaulted 2026-11-10 (the stage's default, S1f.5.21): (a), one count, every distinct variable the `:match` writes and the rule's parameters, or the `:goal`'s, at most 256: a bound on each register file the compiler builds, so its arm is an invariant**; [S1f.5.21](p1f.5_the_no_owner_items/s1f.5.21_a_compile_error_in_eins_words.md) |
| [Q-M1f.74](#q-m1f74--a-nested-or--a-macros-expansion-writes) | a nested `(or …)` a macro's expansion writes | **defaulted 2026-11-10 (the stage's default, S1f.5.21): (a), refused at load as a written one is, the loader reading the expanded rule; within Q-M1f.67's ruling, which allows a refusal at instantiation, and an unused macro still loads**; [S1f.5.21](p1f.5_the_no_owner_items/s1f.5.21_a_compile_error_in_eins_words.md) |
| [Q-M1f.75](#q-m1f75--the-compilers-arms-no-program-reaches-and-d-1s-and-d-2s-other-lines) | the compiler's arms no program reaches, and D-1's and D-2's other lines | **defaulted 2026-11-10 (the stage's default, S1f.5.21): (a), the three a direct caller of `compile_rule` reaches stay its errors, in Ein's words, and the moved arms and the `_` head become invariants; `FileNotFoundError` stays F23's (the read path), and `loc_repr`'s `Loc(…)` is taken here (`<file>:<line>:<col>`), `at None` staying F23's**; [S1f.5.21](p1f.5_the_no_owner_items/s1f.5.21_a_compile_error_in_eins_words.md) |
| [Q-M1f.76](#q-m1f76--which-of-what-the-move-replaces-is-a-register-row) | which of what the move replaces is a register row | **defaulted 2026-11-10 (the stage's default, S1f.5.21): (a), a row per way the engine answers today (Q-M1f.36's reading), W62's six, W63's three, W64's two and W65's five, each fixed by T1f.5.21.2; `features/02`, the one `positive` program the move refuses, rewritten with no row (Q-M1f.39's reading)**; [S1f.5.21](p1f.5_the_no_owner_items/s1f.5.21_a_compile_error_in_eins_words.md) |
| [Q-M1f.77](#q-m1f77--when-is-utilsci_containersh-part-of-done) | when is `utils/ci_container.sh` part of done? | **defaulted 2026-11-10 (the stage's default, S1f.5.29): (a), a stage that changes a DOT writer, the review bundle or what CI installs is not done until `utils/ci_container.sh` is green on its commit; AGENTS.md's *Running the gate* says so**; [S1f.5.29](p1f.5_the_no_owner_items/s1f.5.29_the_review_bundle_smoke_on_the_runner.md) |
| [Q-M1f.78](#q-m1f78--where-the-runners-missing-fdp-is-fixed) | where the runner's missing `fdp` is fixed | **defaulted 2026-11-10 (the coordinator's default, S1f.5.29): (a), the runner installs the plugin: `libgvplugin-neato-layout8` beside `graphviz` in the three workflows and `utils/ci_container.sh`'s image, and `run_tests.sh` exits 127 where `fdp` lays out no graph; no view, golden or drawing moves**; [S1f.5.29](p1f.5_the_no_owner_items/s1f.5.29_the_review_bundle_smoke_on_the_runner.md) |
| [Q-M1f.79](#q-m1f79--does-a-refused-dot-fail-the-whole-bundle) | does a refused DOT fail the whole bundle? | **defaulted 2026-11-10 (the coordinator's default, S1f.5.29): (a), yes, as in the smoke: `utils/review_bundle.py` exits 1 on a DOT Graphviz refused, printing what Graphviz said under its path; a lattice past `DAG_NODES_DRAWN`, kept undrawn by design, says so beside its DOT and is not a refusal**; [S1f.5.29](p1f.5_the_no_owner_items/s1f.5.29_the_review_bundle_smoke_on_the_runner.md) |
| [Q-M1f.80](#q-m1f80--where-a-malformed-primitive-a-macros-expansion-writes-is-refused) | where a malformed primitive a macro's expansion writes is refused | **defaulted 2026-11-10 (the stage's default, S1f.5.24): (a), at load, after expansion, the instantiation the ruling on Q-M1f.67 allows: every command refuses the program, the views and `kb save` among them, before any path compiles, fires, proposes or owes the rule; a macro no rule calls still loads**; [S1f.5.24](p1f.5_the_no_owner_items/s1f.5.24_a_macro_body_checked_where_a_rule_is.md) |
| [Q-M1f.81](#q-m1f81--what-a-malformed-primitive-is) | what a malformed primitive is | **defaulted 2026-11-10 (the stage's default, S1f.5.24): (a), every form the grammar would refuse where it stands if the expanded rule were written: `eq` and `neq` at two values, `absent` at one premise, `not` of one pattern (or, concluded, a variable), `false` with nothing, no `(or …)` as a conclusion, no connective where a value goes, and a conclusion that is a form**; [S1f.5.24](p1f.5_the_no_owner_items/s1f.5.24_a_macro_body_checked_where_a_rule_is.md) |
| [Q-M1f.82](#q-m1f82--the-line-a-malformed-primitive-is-refused-with) | the line a malformed primitive is refused with | **defaulted 2026-11-10 (the stage's default, S1f.5.24): (a), the compiler's, S1f.5.21's spelling, behind `kb load error: `: the position where the expanded form is written, the rule as the loader names it (`hrule 'h'`), the clause, the form as Ein text and what to write instead**; [S1f.5.24](p1f.5_the_no_owner_items/s1f.5.24_a_macro_body_checked_where_a_rule_is.md) |
| [Q-M1f.83](#q-m1f83--which-of-what-the-refusal-replaces-is-a-register-row) | which of what the refusal replaces is a register row | **defaulted 2026-11-10 (the stage's default, S1f.5.24): (a), a row per way the engine answered where a refusal is right, W66's four, W67's four, W68's six and W69's two, each fixed by T1f.5.24.3; the compile stop and the firing stop at root, which the ruling allows, are no row, and the views' and `kb save`'s exits on such a program move under Q-M1f.71**; [S1f.5.24](p1f.5_the_no_owner_items/s1f.5.24_a_macro_body_checked_where_a_rule_is.md) |
| [Q-M1f.84](#q-m1f84--the-compilers-arity-arms-and-the-firings-whole-conclusion-line) | the compiler's arity arms and the firing's whole-conclusion line | **defaulted 2026-11-10 (the stage's default, S1f.5.24): (a), invariants, as S1f.5.21's nested `(or …)` arm (Q-M1f.9 (a)'s pattern): the loader's check makes each unreachable, and `fuzz_properties` holds the compiler to refuse nothing the loader accepts**; [S1f.5.24](p1f.5_the_no_owner_items/s1f.5.24_a_macro_body_checked_where_a_rule_is.md) |
| [Q-M1f.85](#q-m1f85--which-findings-p1f7-takes) | which findings P1f.7 takes | **defaulted 2026-11-10 (the coordinator, on the user's instruction of that day): S1f.5.28's items 12 to 16, the batch the coordinator had given it from S1f.5.21's and S1f.5.24's records, as S1f.7.1 to S1f.7.5; and every wrong answer P1f.5's remaining stages find outside their scope, a stage each from S1f.7.6. S1f.5.28 keeps items 1 to 11, the close review's findings 9 and 7, which the user ruled into it**; [P1f.7](p1f.7_findings_outside_scope/README.md) |
| [Q-M1f.86](#q-m1f86--may-a-macros-body-write-open) | may a macro's body write `open`? | **defaulted 2026-11-10 (the stage's default, S1f.5.23): (a), no: refused at the parse, called or not, as `_` and a string are in a body; the verdict is written in the rule**; [S1f.5.23](p1f.5_the_no_owner_items/s1f.5.23_open_is_reserved.md) |
| [Q-M1f.87](#q-m1f87--the-verdicts-loader-lines-once-the-grammar-places-open) | the verdict's loader lines once the grammar places `open` | **defaulted 2026-11-10 (the stage's default, S1f.5.23): (a), the five the grammar covers deleted, each a row of the grammar's table and named in `DELETED`, `validate_open` holding the rest as an invariant; the five that stay start with where the `(open …)` is written, `at None` gone**; [S1f.5.23](p1f.5_the_no_owner_items/s1f.5.23_open_is_reserved.md) |
| [Q-M1f.88](#q-m1f88--the-line-open-is-refused-with) | the line `open` is refused with | **defaulted 2026-11-10 (the stage's default, S1f.5.23): (a), the parse error's, ``unexpected `open` ``, the stage's sentence a note under it (`ein_ir::grammar::OPEN_NOTE`), as `_` and a string carry one**; [S1f.5.23](p1f.5_the_no_owner_items/s1f.5.23_open_is_reserved.md) |
| [Q-M1f.89](#q-m1f89--which-of-what-the-refusal-replaces-is-a-register-row) | which of what the refusal replaces is a register row | **defaulted 2026-11-10 (the stage's default, S1f.5.23): (a), a row per place with a reading of its own, W70's seven, W71's fourteen and W72's three; the pin holds all fifty-two places**; [S1f.5.23](p1f.5_the_no_owner_items/s1f.5.23_open_is_reserved.md) |
| [Q-M1f.90](#q-m1f90--which-of-the-import-reads-moves-are-register-rows-and-where-their-witnesses-live) | which of the import read's moves are register rows, and where their witnesses live | **defaulted 2026-11-10 (the stage's default, S1f.5.22): (a), W74's nine rows on the one witness git can keep, a module that is a directory, one per command run the register can run; a module that is not UTF-8, one that cannot be read and a `.einb` container are the pin's, `ein-cli/tests/unicode_only_in_a_string.rs`, which writes each at run time; and W73's five rows, one per Unicode space and one for a space inside a name**; [S1f.5.22](p1f.5_the_no_owner_items/s1f.5.22_unicode_only_in_a_string.md) |
| [Q-M1f.91](#q-m1f91--a-module-that-is-a-einb-container) | a module that is a `.einb` container | **defaulted 2026-11-10 (the stage's default, S1f.5.22): (a), refused with a line of its own, naming the module's path and saying an import reads Ein text (`ein_ir::read::container_module_line`)**; [S1f.5.22](p1f.5_the_no_owner_items/s1f.5.22_unicode_only_in_a_string.md) |
| [Q-M1f.92](#q-m1f92--how-the-line-quotes-a-character-a-reader-cannot-see) | how the line quotes a character a reader cannot see | **defaulted 2026-11-10 (the stage's default, S1f.5.22): (a), as the escape a string writes it with, `'\u{a0}' (U+00A0)`, for a space, a control, a zero-width or bidirectional format character, a combining mark or a variation selector (`ein_ir::grammar::shown_char`); any other character as itself**; [S1f.5.22](p1f.5_the_no_owner_items/s1f.5.22_unicode_only_in_a_string.md) |
| [Q-M1f.93](#q-m1f93--which-path-a-modules-read-line-names) | which path a module's read line names | **defaulted 2026-11-10 (the stage's default, S1f.5.22): (a), the path the import resolves to, the one `module not found at …` names, relative where the program was given relatively**; [S1f.5.22](p1f.5_the_no_owner_items/s1f.5.22_unicode_only_in_a_string.md) |
| [Q-M1f.94](#q-m1f94--a-directorys-and-an-unreadable-modules-line-and-a-missing-ones-none) | a directory's and an unreadable module's line, and a missing one's `(None)` | **defaulted 2026-11-10 (the stage's default, S1f.5.22): (a), the program's lines, `IsADirectoryError: …` and `PermissionError: …`, F23's D-1 for both; `module not found at … (None)` for a module that does not exist stays, its `(None)` F23's D-2 with every import line's**; [S1f.5.22](p1f.5_the_no_owner_items/s1f.5.22_unicode_only_in_a_string.md) |
| [Q-M1f.95](#q-m1f95--how-many-characters-the-refusal-names) | how many characters the refusal names | **defaulted 2026-11-10 (the stage's default, S1f.5.22): (a), the first, where the lexer stops: `lrpar` lexes a file before it parses it, so a lexing error is the file's one error**; [S1f.5.22](p1f.5_the_no_owner_items/s1f.5.22_unicode_only_in_a_string.md) |
| [Q-M1f.96](#q-m1f96--a-byte-order-mark) | a byte-order mark | **defaulted 2026-11-10 (the stage's default, S1f.5.22): (a), refused as any other character that is not ASCII, at 1:1, `'\u{feff}' (U+FEFF)`; it was a token that is not a name until then**; [S1f.5.22](p1f.5_the_no_owner_items/s1f.5.22_unicode_only_in_a_string.md) |
| [Q-M1f.97](#q-m1f97--how-a-test-closes-a-runs-stdout) | how a test closes a run's stdout | **defaulted 2026-11-10 (the stage's default, S1f.5.25): (a), the register's runs and the pin's second reader close it before the run starts, so every write meets no reader whatever the timing; the pin's third reader closes after the first line, as `head -1`, on a one-page pipe, and holds a run to `SIGPIPE` where its stdout is more than that line and the page, to `SIGPIPE` or exit 0 where it fits, never 101 and never a word on stderr**; [S1f.5.25](p1f.5_the_no_owner_items/s1f.5.25_a_closed_stdout.md) |
| [Q-M1f.98](#q-m1f98--which-of-the-closed-stdouts-moves-are-register-rows) | which of the closed stdout's moves are register rows | **defaulted 2026-11-10 (the stage's default, S1f.5.25): (a), W75's eighteen rows, one per command and per `solve` question and `solve -c`, on two corpus programs, each a run that panicked, exit 101, its reader closed before the run; every other option, `--version`, `render region` and an artefact on `/dev/stdout` the pin's; and a move from exit 0 to the signal no row: `clap`'s help, which ignored its failed write, is no answer about a program, and a run whose writes landed before its reader closed exits 0 still**; [S1f.5.25](p1f.5_the_no_owner_items/s1f.5.25_a_closed_stdout.md) |
| [Q-M1f.99](#q-m1f99--what-the-register-records-of-a-run-a-closed-stdout-ended) | what the register records of a run a closed stdout ended | **defaulted 2026-11-10 (the stage's default, S1f.5.25): (a), schema `/21`: a `now` with `stdout = "closed"` is a command's run whatever its subcommand, and answers by `signal` (13, nothing on stderr) or `panicked` (exit 101, the panic's message); a `right` can be `signal = 13`, and a recorded answer the panic**; [S1f.5.25](p1f.5_the_no_owner_items/s1f.5.25_a_closed_stdout.md) |
| [Q-M1f.100](#q-m1f100--an-artefact-flag-that-writes-to-a-pipe-whose-reader-has-gone) | an artefact flag that writes to a pipe whose reader has gone | **defaulted 2026-11-10 (the stage's default, S1f.5.25): (a), it ends the process by `SIGPIPE` too, the disposition being process-wide; `defined_behaviour.md` § 4.4's arm, the message and the exit code unchanged, holds for every other failure; shown to the user at the close, since it narrows § 4.4**; [S1f.5.25](p1f.5_the_no_owner_items/s1f.5.25_a_closed_stdout.md) |
| [Q-M1f.101](#q-m1f101--a-closed-stdout-on-windows) | a closed stdout on Windows | **defaulted 2026-11-10 (the stage's default, S1f.5.25): (a), left as it is, `restore_sigpipe` a no-op there: Windows has no `SIGPIPE`, so a write to a closed pipe is an error and a `print!` would still panic, exit 101, which no host here can run; shown to the user at the close**; [S1f.5.25](p1f.5_the_no_owner_items/s1f.5.25_a_closed_stdout.md) |
| [Q-M1f.102](#q-m1f102--what-the-ci-nightly-keeps-and-what-the-dev-host-runs) | what the CI nightly keeps, and what the dev host runs | **defaulted 2026-11-10 (the stage's default, S1f.5.27): (a), the stage file's rule: a step that guards an answer or a contract no per-commit step guards stays in CI, at a budget that fits in 5 min on the runner; a step over that, or one that measures, runs on the dev host, which runs every nightly step at full budget too; the stage file's classification confirmed, the fuzzer's budget moved (Q-M1f.103)**; [S1f.5.27](p1f.5_the_no_owner_items/s1f.5.27_the_nightly_reduced.md) |
| [Q-M1f.103](#q-m1f103--the-fuzzers-budget-in-the-ci-nightly) | the fuzzer's budget in the CI nightly | **defaulted 2026-11-10 (the stage's default, S1f.5.27): (a), 20 000 inputs a seed, where the stage file proposed 100 000: on the runner's measured rate 100 000 is past the rule's 5 min, and 20 000 fits; the dev host runs 1 000 000**; [S1f.5.27](p1f.5_the_no_owner_items/s1f.5.27_the_nightly_reduced.md) |
| [Q-M1f.104](#q-m1f104--a-critical-step-the-runners-spread-puts-on-the-line) | a critical step the runner's spread puts on the line | **defaulted 2026-11-10 (the stage's default, S1f.5.27): (a), `corpus_cores`' slow half stays whole in the nightly: 256 s on the latest run, 428 s and 436 s on the two before, on a runner slower in every step, and its slow half has no budget to cut**; [S1f.5.27](p1f.5_the_no_owner_items/s1f.5.27_the_nightly_reduced.md) |
| [Q-M1f.105](#q-m1f105--how-the-dev-hosts-list-is-held) | how the dev host's list is held | **defaulted 2026-11-10 (the stage's default, S1f.5.27): (a), a third copy in `run_tests.sh`, held by `nightly_steps.rs` to the nightly's as a superset whose other steps `DEV_HOST_ONLY` names with why, with a floor, the two fuzz budgets pinned, and a census of the targets that read `EIN_CORPUS_SLOW`**; [S1f.5.27](p1f.5_the_no_owner_items/s1f.5.27_the_nightly_reduced.md) |
| [Q-M1f.106](#q-m1f106--the-whole-review-bundle-on-the-runners-graphviz) | the whole review bundle on the runner's Graphviz | **defaulted 2026-11-10 (the stage's default, S1f.5.27): (a), it runs on the dev host, here or in the runner's image (`utils/ci_container.sh --dev-host`); per commit the runner draws every view with `dot` (`dot_wellformed`) and the smoke's 31 DOTs with their own engines, and Q-M1f.77 is unchanged**; [S1f.5.27](p1f.5_the_no_owner_items/s1f.5.27_the_nightly_reduced.md) |
| [Q-M1f.107](#q-m1f107--where-the-close-reviews-finding-7-is-fixed) | where the close review's finding 7, the stale slow counts, is fixed | **defaulted 2026-11-10 (the coordinator's default, recorded by S1f.5.28): (a), S1f.5.28's item 11, three prose edits citing `the_slow_set_is_exactly_these_fourteen` in place of a count; shown to the user at the close**; [S1f.5.28](p1f.5_the_no_owner_items/s1f.5.28_the_unchecked_findings_triaged.md) |
| [Q-M1f.108](#q-m1f108--route-1s-stopped-before-layer-l-which-no-ein-run-prints) | route 1's *stopped before layer L*, which no `ein` run prints | **defaulted 2026-11-10 (the stage's default, S1f.5.28): (a), the row stays for `explore::describe_by`, and `read_out.md` says why no `ein` run prints it, pinned by `read_out_reference.rs`' `route_1_never_stops_on_an_ein_run`**; [S1f.5.28](p1f.5_the_no_owner_items/s1f.5.28_the_unchecked_findings_triaged.md) |
| [Q-M1f.109](#q-m1f109--a-texts-line-break-in-the-traces-markdown) | a text's line break in the trace's markdown | **defaulted 2026-11-10 (the stage's default, S1f.5.28): (a), a hard break, two spaces before it, the line after it in its block (`> ` in a `:why`'s quote), as Q-M1f.19 took the read-out's; shown to the user at the close, since it moves the bytes Q-M1f.12 holds the trace to**; [S1f.5.28](p1f.5_the_no_owner_items/s1f.5.28_the_unchecked_findings_triaged.md) |
| [Q-M1f.110](#q-m1f110--a-container-saved-again) | a container saved again | **defaulted 2026-11-10 (the stage's default, S1f.5.28): (a), the writer stops at the last provenance record the KB references, so a re-save is byte-identical and no id moves; shown to the user at the close, since S1f.5.19 recorded the growth and left it**; [S1f.5.28](p1f.5_the_no_owner_items/s1f.5.28_the_unchecked_findings_triaged.md) |
| [Q-M1f.111](#q-m1f111--the-fuzz-tests-shape) | the fuzz test's shape | **defaulted 2026-11-10 (the stage's default, S1f.5.28): (a), the stream parsed once, on every thread the process may use, each property keeping its test, its place-by-place reading and its own failure report; 4.3× less CPU at 20 000 inputs, 3.9× and 4.2× less wall on four P-core threads**; [S1f.5.28](p1f.5_the_no_owner_items/s1f.5.28_the_unchecked_findings_triaged.md) |
| [Q-M1f.114](#q-m1f114--what-a-relation-in-a-solve--list-must-be) | what a relation in a `(solve …)` list must be | **defaulted 2026-11-11 (the stage's default, S1f.7.8): (b), a name the program writes as a relation outside the lists, so a relation only a rule derives is accepted and a name written nowhere else is refused**; [S1f.7.8](p1f.7_findings_outside_scope/s1f.7.8_a_solve_list_naming_nothing.md) |
| [Q-M1f.115](#q-m1f115--what-else-of-an-hrules-list-is-refused) | what else of an `:hrules` list is refused | **defaulted 2026-11-11 (the stage's default, S1f.7.8): (a), every bare name is a declared hrule and an activator has as many arguments as its hrule has parameters**; [S1f.7.8](p1f.7_findings_outside_scope/s1f.7.8_a_solve_list_naming_nothing.md) |
| [Q-M1f.116](#q-m1f116--an-activators-argument-that-is-a-value) | an activator's argument that is a value | **defaulted 2026-11-11 (the stage's default, S1f.7.8): (b), not checked, and the missing word that an entry proposes nothing is F25's R-2**; [S1f.7.8](p1f.7_findings_outside_scope/s1f.7.8_a_solve_list_naming_nothing.md) |
| [Q-M1f.117](#q-m1f117--a-goal-with-a-macro-call-printed-as-written-or-as-expanded) | a goal with a macro call: printed as written, or as expanded | **defaulted 2026-11-11 (S1f.7.5): expanded at load as a rule's `:match` is, matched on the expansion, printed as written**; [S1f.7.5](p1f.7_findings_outside_scope/s1f.7.5_a_macro_called_in_a_goal.md) |
| [Q-M1f.118](#q-m1f118--a-macro-call-in-a-claim) | a macro call in a claim | **defaulted 2026-11-11 (S1f.7.5): not expanded; a claim lists facts, so it names no relation and is refused**; [S1f.7.5](p1f.7_findings_outside_scope/s1f.7.5_a_macro_called_in_a_goal.md) |
| [Q-M1f.119](#q-m1f119--which-relations-a-goal-may-ask-about) | which relations a goal may ask about | **defaulted 2026-11-11 (the stage's default (a), S1f.7.11): refused at load unless a relation of the program, S1f.7.8's notion without the goals, or one a `:hrules` activator has its hrule propose**; [S1f.7.11](p1f.7_findings_outside_scope/s1f.7.11_a_goal_over_a_relation_nothing_makes.md) |
| [Q-M1f.120](#q-m1f120--a-rules-match-over-a-relation-nothing-makes) | a rule's `:match` over a relation nothing makes | **defaulted 2026-11-11 (the stage's default, S1f.7.11): reported in the stage's record and not refused; F25's R-6**; [S1f.7.11](p1f.7_findings_outside_scope/s1f.7.11_a_goal_over_a_relation_nothing_makes.md) |
| [Q-M1f.121](#q-m1f121--the-line-a-goals-nested-relation-variable-is-refused-with) | the line a goal's nested relation variable is refused with | **defaulted 2026-11-11 (S1f.7.1): the goal's own line, the one its top-level relation variable has had since S1f.5.21, from the one walk a rule's clauses get**; [S1f.7.1](p1f.7_findings_outside_scope/s1f.7.1_a_goals_variable_relation_in_a_nested_pattern.md) |
| [Q-M1f.122](#q-m1f122--where-absent-still-heads-a-form-that-is-no-premise) | where `absent` still heads a form that is no premise | **defaulted 2026-11-11 (S1f.7.7): in a claim's class and nowhere else; the refusal is the grammar's, with a note pointing at `(not …)`**; [S1f.7.7](p1f.7_findings_outside_scope/s1f.7.7_an_absent_fact.md) |
| [Q-M1f.123](#q-m1f123--false-with-no-argument-as-a-value) | `(false)` with no argument as a value | **defaulted 2026-11-11 (S1f.7.6): it stays, `(q (false))` and `(not (false))` load; the rule refuses `false` with an argument**; [S1f.7.6](p1f.7_findings_outside_scope/s1f.7.6_a_false_fact_with_an_argument.md) |
| [Q-M1f.124](#q-m1f124--where-eq-a-b-is-still-written-once-it-is-no-value) | where `(eq a b)` is still written, once it is no value | **defaulted 2026-11-11 (S1f.7.10): a fact, a premise, a conclusion and a claim of its own, and under a `(not …)` that is one of those; never under a `(not …)` that is a value, in `:using` or in a trace**; [S1f.7.10](p1f.7_findings_outside_scope/s1f.7.10_an_eq_fact_as_a_value.md) |
| [Q-M1f.125](#q-m1f125--a-connective-as-a-patterns-argument-the-grammar-or-the-loader) | a connective as a pattern's argument: the grammar or the loader | **defaulted 2026-11-11 (S1f.7.3): the loader, after macro expansion, since `Arg` takes a connective for a macro's call; `and`, `or`, `neq`, `eq`, `absent` and `false` with an argument refused, `not` kept**; [S1f.7.3](p1f.7_findings_outside_scope/s1f.7.3_a_connective_as_a_patterns_argument.md) |
| [Q-M1f.126](#q-m1f126--a-relation-parameter-bound-to-absent-false-or-eq) | a relation parameter bound to `absent`, `false` or `eq` | **defaulted 2026-11-11 (S1f.7.7, S1f.7.6, S1f.7.10): refused at load where § 4.14's reading sees it, fixed in place**; [S1f.7.7](p1f.7_findings_outside_scope/s1f.7.7_an_absent_fact.md) |
| [Q-M1f.127](#q-m1f127--what-the-loader-reads-to-see-that-a-negated-variable-can-hold-a-name) | what the loader reads to see that a negated variable can hold a name | **defaulted 2026-11-11 (S1f.7.4): what the program's own text shows, a written fact, a declaration, a rule's or an hrule's conclusion, an activator, a declared relation the hypotheses are taken from and a guard alone; a position that holds a name and a fact is refused; a pattern whose relation is a parameter is read through its written activators, and not through a derived one**; [S1f.7.4](p1f.7_findings_outside_scope/s1f.7.4_the_negation_of_a_name_concluded.md) |
| [Q-M1f.128](#q-m1f128--what-a-firing-does-with-the-negation-of-a-name-the-loader-did-not-foresee) | what a firing does with the negation of a name the loader did not foresee | **defaulted 2026-11-11 (S1f.7.4): it builds no fact, so the firing stores nothing for that conclusion and an hrule proposes nothing; not `(false)`, not a stop**; [S1f.7.4](p1f.7_findings_outside_scope/s1f.7.4_the_negation_of_a_name_concluded.md), F25's R-1 |
| [Q-M1f.129](#q-m1f129--the-assertion-that-no-hrule-reads-not-removed-or-kept-where-the-kill-cache-is-on) | the assertion that no hrule reads `not`: removed, or kept where the kill cache is on | **defaulted 2026-11-11 (S1f.7.2): removed, and where an hrule reads `not` every hrule is matched before any candidate is emitted, so the cache's writes are not visible with it on; the rows filed and fixed in one commit**; [S1f.7.2](p1f.7_findings_outside_scope/s1f.7.2_an_hrule_that_reads_not.md) |
| [Q-M1f.130](#q-m1f130--a-macros-call-as-a-conclusion-refused-or-expanded-as-it-is) | a macro's call as a conclusion: refused, or expanded as it is | **defaulted 2026-11-11 (S1f.7.12): expanded, as it is today; the stage's default refuses the call in a fact, and T1 found a conclusion's call expanded already**; [S1f.7.12](p1f.7_findings_outside_scope/s1f.7.12_a_macro_call_written_as_a_fact.md) |
| [Q-M1f.131](#q-m1f131--a-relation-declared-under-a-macros-name) | a relation declared under a macro's name | **defaulted 2026-11-11 (S1f.7.12): refused at load, one line at the declaration; no corpus program declares one**; [S1f.7.12](p1f.7_findings_outside_scope/s1f.7.12_a_macro_call_written_as_a_fact.md) |
| [Q-M1f.132](#q-m1f132--a-stdmacro-macro-the-program-does-not-import-written-as-a-fact) | a `std.macro` macro the program does not import, written as a fact | **defaulted 2026-11-11 (S1f.7.12): refused, as a rule's `:match` writing it has been since S1.8a.f20; a relation of that name is not refused where it is declared**; [S1f.7.12](p1f.7_findings_outside_scope/s1f.7.12_a_macro_call_written_as_a_fact.md) |
| [Q-M1f.133](#q-m1f133--a-macros-call-inside-a-fact-a-claim-lists-and-in-a-facts-using) | a macro's call inside a fact a claim lists, and in a fact's `:using` | **defaulted 2026-11-11 (S1f.7.12): refused as a written fact's is; a claim headed by the call keeps Q-M1f.118's line**; [S1f.7.12](p1f.7_findings_outside_scope/s1f.7.12_a_macro_call_written_as_a_fact.md) |
| [Q-M1f.134](#q-m1f134--what-a-register-row-records-as-right-while-f24s-question-is-not-ruled) | what a register row records as `right` while F24's question is not ruled | **defaulted 2026-11-11 (S1f.7.9): the answer under one named reading of F24's (a), (b), (c), each other reading an alternative, the review `deferred`, as W8's rows record Q-M1e2.2**; [S1f.7.9](p1f.7_findings_outside_scope/s1f.7.9_a_facts_rule_that_names_no_rule.md), [F24](../followups/f24_written_provenance.md) |
| [Q-M1f.135](#q-m1f135--which-facts-a-flat-import-brings) | which facts a flat import brings | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.11): a name in the selection's closure selects the module's declaration of it and every top-level fact whose relation it is; a listed name may be a relation the module only states facts of (design.md § G's a3)**; [S1f.6.11](p1f.6_calculator/s1f.6.11_importing_facts.md) |
| [Q-M1f.136](#q-m1f136--what-a-qualified-import-does-to-the-names-inside-a-fact) | what a qualified import does to the names inside a fact | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.11): unchanged, it renames declared names; a library of facts is imported flat**; [S1f.6.11](p1f.6_calculator/s1f.6.11_importing_facts.md) |
| [Q-M1f.137](#q-m1f137--are-an-imported-facts-constants-names-of-the-program) | are an imported fact's constants names of the program? | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.11): yes, as an imported fact is a fact of the program**; [S1f.6.11](p1f.6_calculator/s1f.6.11_importing_facts.md) |
| [Q-M1f.138](#q-m1f138--the-provenance-of-an-imported-fact) | the provenance of an imported fact | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.11): its own `:source`, or background; no module stamp, F24's**; [S1f.6.11](p1f.6_calculator/s1f.6.11_importing_facts.md) |
| [Q-M1f.139](#q-m1f139--the-same-fact-from-two-places) | the same fact from two places | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.11): identical facts are one; two `:source`s keep the first statement, stated on the page and handed to F24**; [S1f.6.11](p1f.6_calculator/s1f.6.11_importing_facts.md) |
| [Q-M1f.140](#q-m1f140--a-rule-of-the-module-the-selection-does-not-name) | a rule of the module the selection does not name | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.11): still dropped: a selection is a selection**; [S1f.6.11](p1f.6_calculator/s1f.6.11_importing_facts.md) |
| [Q-M1f.141](#q-m1f141--does-a-selections-closure-follow-the-names-inside-a-kept-fact) | does a selection's closure follow the names inside a kept fact? | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.11): no: it follows the names a kept declaration writes, as before the stage**; [S1f.6.11](p1f.6_calculator/s1f.6.11_importing_facts.md) |
| [Q-M1f.142](#q-m1f142--a-stored-negation-and-a-fact-of-a-kernel-relation-under-a-selection) | a stored negation, and a fact of a kernel relation, under a selection | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.11): a negation is a fact of the relation it negates; `(__closed__ r)` and a written `(eq a b)` come as any fact does, when the relation's name is in the closure**; [S1f.6.11](p1f.6_calculator/s1f.6.11_importing_facts.md) |
| [Q-M1f.143](#q-m1f143--a-fact-that-arrives-twice-through-flat-imports) | a fact that arrives twice through flat imports | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.11): one statement, kept where it first arrived**; [S1f.6.11](p1f.6_calculator/s1f.6.11_importing_facts.md) |
| [Q-M1f.144](#q-m1f144--what-w88s-rows-record) | what W88's rows record | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.11): the sentences a query's `:goal-text` prints, and a refusal; no schema change**; [S1f.6.11](p1f.6_calculator/s1f.6.11_importing_facts.md) |
| [Q-M1f.145](#q-m1f145--how-the-table-generators-check-is-in-the-gate) | how the table generator's `--check` is in the gate | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.2): a cargo test runs it, as `gen_zebra2_variants.py --check` is run; no new gate step**; [S1f.6.2](p1f.6_calculator/s1f.6.2_digits_and_tables.md) |
| [Q-M1f.146](#q-m1f146--how-stddigits-closes-its-tables-for-a-flat-importer) | how `std.digits` closes its tables for a flat importer | **Confirmed 2026-11-12** (reply 53 (a)). **defaulted 2026-11-11 (S1f.6.2): written `(__closed__ R)` facts, which a direct importer brings by listing `__closed__`; a theory that reads a table closes it in its own closing rule**; [S1f.6.2](p1f.6_calculator/s1f.6.2_digits_and_tables.md) |
| [Q-M1f.147](#q-m1f147--the-dot-sweep-and-a-view-no-layout-finishes) | the DOT sweep and a view no layout finishes | **Overruled 2026-11-12** (reply 45 of the calculator's [`questions.md`](p1f.6_calculator/questions.md#q45)): *"don't test graphviz itself - that is the main point here. just skip big graphs, run tests only on small"*; [S1f.6.21](p1f.6_calculator/s1f.6.21_the_gate_reduced.md), **built 2026-11-12**: a view past 1 000 edges is skipped and counted. **defaulted 2026-11-11 (S1f.6.2): past 1 500 edges `dot_wellformed.rs` asks Graphviz under `fdp` stopped after one iteration; grammar and attributes held, `dot`'s layout of it not. For the user to confirm**; [S1f.6.2](p1f.6_calculator/s1f.6.2_digits_and_tables.md) |
| [Q-M1f.148](#q-m1f148--one-module-for-the-seven-tables) | one module for the seven tables | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.2): one, as the stage file says; an importer parses all 28.8 KB, about 12 ms, whichever tables it lists**; [S1f.6.2](p1f.6_calculator/s1f.6.2_digits_and_tables.md) |
| [Q-M1f.149](#q-m1f149--what-let-name-number-does-as-built) | what `(let NAME NUMBER)` does, as built | **Confirmed 2026-11-12** (reply 11 (a)). **defaulted 2026-11-11 (S1f.6.3): the rulings' default reading; every operand of a request is looked up, a number standing for itself and a name for the number its `let` states, four firings a request; a name no `let` states is never answered**; [S1f.6.3](p1f.6_calculator/s1f.6.3_addition.md) |
| [Q-M1f.150](#q-m1f150--the-counter-and-how-the-measured-test-is-in-the-gate) | the counter, and how the measured test is in the gate | **Overruled 2026-11-12** (reply 46 (b)): *"we don't need this proportional gate"*; [S1f.6.21](p1f.6_calculator/s1f.6.21_the_gate_reduced.md), **built 2026-11-12**: the step is out of the gate, the test run by hand. **defaulted 2026-11-11 (S1f.6.3): `candidates`, under `--features counters`, in a gate step of its own**; [S1f.6.3](p1f.6_calculator/s1f.6.3_addition.md) |
| [Q-M1f.151](#q-m1f151--the-names-of-the-circuits-own-relations) | the names of the circuit's own relations | **Reopened 2026-11-12** by reply 8: the clash is a gap of the import mechanism; [S1f.6.30](p1f.6_calculator/s1f.6.30_name_clashes_between_imported_theories.md). **defaulted 2026-11-11 (S1f.6.3): words, `add-walk`, `strip`, `turn`, `operand`; what a clash does is S1f.6.12's**; [S1f.6.3](p1f.6_calculator/s1f.6.3_addition.md) |
| [Q-M1f.152](#q-m1f152--are-stdariths-rules-written-by-hand-or-generated) | are `std.arith`'s rules written by hand or generated? | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.3): by hand, while a rule is not a family of nine**; [S1f.6.3](p1f.6_calculator/s1f.6.3_addition.md) |
| [Q-M1f.153](#q-m1f153--the-dot-sweeps-bound-under-the-smallest-table) | the DOT sweep's bound, under the smallest table | **Overruled 2026-11-12** (reply 45 of the calculator's [`questions.md`](p1f.6_calculator/questions.md#q45)): *"don't test graphviz itself - that is the main point here. just skip big graphs, run tests only on small"*; [S1f.6.21](p1f.6_calculator/s1f.6.21_the_gate_reduced.md), **built 2026-11-12**: a view past 1 000 edges is skipped and counted. **defaulted 2026-11-11 (S1f.6.3): 1 000 edges, where S1f.6.2 took 1 500: a view that holds a table is past it. With Q-M1f.147, for the user to confirm**; [S1f.6.3](p1f.6_calculator/s1f.6.3_addition.md) |
| [Q-M1f.154](#q-m1f154--the-bounds-of-the-harness-against-the-integers) | the bounds of the harness against the integers | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.3): every pair to 120, every pair of lengths to 30 digits on a fixed-seed sample, `u128` the oracle**; [S1f.6.3](p1f.6_calculator/s1f.6.3_addition.md) |
| [Q-M1f.155](#q-m1f155--equal-operands-of-a-subtraction-two-walks-one-fact) | equal operands of a subtraction: two walks, one fact | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.4): both walks answer, and both derive the one `(diff-of X Y z z)`**; [S1f.6.4](p1f.6_calculator/s1f.6.4_subtraction.md) |
| [Q-M1f.156](#q-m1f156--comparison-called-with-a-tag) | comparison called with a tag | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.4): `(cmp-walk TAG X Y ORDER)` hands over `(order-is TAG O)`, a firing more than the prototype's, so a later theory can call it**; [S1f.6.4](p1f.6_calculator/s1f.6.4_subtraction.md) |
| [Q-M1f.157](#q-m1f157--each-request-looks-its-operands-up-by-rules-of-its-own) | each request looks its operands up by rules of its own | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.4): three rules and two tags a request, as `add` has them; addition's circuit unmoved**; [S1f.6.4](p1f.6_calculator/s1f.6.4_subtraction.md) |
| [Q-M1f.158](#q-m1f158--cls-column-order-and-the-index) | `cl`'s column order and the index | **Left as it is** in round 2 (`dc88520b`) (item 47, *"(a)"*). **defaulted 2026-11-11 (S1f.6.4): the table stays as S1f.6.2 shipped it; a comparison's column costs 100 candidates, a constant; E-12**; [S1f.6.4](p1f.6_calculator/s1f.6.4_subtraction.md) |
| [Q-M1f.159](#q-m1f159--multiplication-called-with-a-tag) | multiplication called with a tag | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.5): `(mul-call TAG X Y)` answered by `(done TAG P)`, a firing more than the prototype's, so a later theory can multiply**; [S1f.6.5](p1f.6_calculator/s1f.6.5_multiplication.md) |
| [Q-M1f.160](#q-m1f160--a-zero-digit-of-the-multiplier) | a zero digit of the multiplier | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.5): a row like any other, no rule tells it apart**; [S1f.6.5](p1f.6_calculator/s1f.6.5_multiplication.md) |
| [Q-M1f.161](#q-m1f161--the-dot-sweep-several-files-at-a-time) | the DOT sweep, several files at a time | **2026-11-12:** stands; not overruled by reply 45. **defaulted 2026-11-11 (S1f.6.5): the corpus files shared out among the cores, the report in the files' order; what the sweep holds unchanged. With Q-M1f.147 and .153, for the user to confirm**; [S1f.6.5](p1f.6_calculator/s1f.6.5_multiplication.md) |
| [Q-M1f.162](#q-m1f162--the-longest-sampled-operand-of-a-product) | the longest sampled operand of a product | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.5): 19 digits, so the product stays in `u128`**; [S1f.6.5](p1f.6_calculator/s1f.6.5_multiplication.md) |
| [Q-M1f.163](#q-m1f163--division-called-with-a-tag-and-whose-division-by-zero-is-flagged) | division called with a tag, and whose division by zero is flagged | **Overruled 2026-11-12** (reply 48): a division by zero is flagged wherever it is called; [S1f.6.24](p1f.6_calculator/s1f.6.24_division_by_zero_at_the_call.md). **defaulted 2026-11-11 (S1f.6.6): `(div-call TAG X Y)` answered by `(div-done TAG Q R)`; the flag is derived for a public request, `(quotient X Y)`, and a later theory that calls division says what its own zero divisor is**; [S1f.6.6](p1f.6_calculator/s1f.6.6_division.md) |
| [Q-M1f.164](#q-m1f164--divisions-families-of-nine-rules-by-hand-or-generated) | division's families of nine rules: by hand or generated? | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.6): by hand, and a test holds that the rules of a family differ by their digit alone**; [S1f.6.6](p1f.6_calculator/s1f.6.6_division.md) |
| [Q-M1f.165](#q-m1f165--what-search-free-means-of-a-program-that-divides-by-zero) | what search-free means of a program that divides by zero | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.6): no candidate, one saturation, R empty and decided by root, the request in the core; the test holds such a program to that where it holds the others to R = {∅}**; [S1f.6.6](p1f.6_calculator/s1f.6.6_division.md) |
| [Q-M1f.166](#q-m1f166--the-sizes-of-the-generated-arithmetic-examples) | the sizes of the generated arithmetic examples | **Ruled 2026-11-12** (reply 49): a generator with size arguments; [S1f.6.29](p1f.6_calculator/s1f.6.29_the_biggest_calculation.md). **defaulted 2026-11-11 (S1f.6.15): what every per-commit surface affords, 100-digit operands, 100 requests and 25!; the plan's sizes are the script's sweep and the generator's arguments**; [S1f.6.15](p1f.6_calculator/s1f.6.15_load_against_calculation.md) Round 2's item 49 (`dc88520b`): about 16 GB of memory is the bound, and no test or CI step runs it. |
| [Q-M1f.167](#q-m1f167--the-names-of-stdnum9) | the names of `std.num9` | **2026-11-12** (reply 52): the relation names stand; the modules move under `std.math`, [S1f.6.31](p1f.6_calculator/s1f.6.31_the_stdlib_layout.md). **defaulted 2026-11-11 (S1f.6.13): `std.arith`'s public names with a 9, `add9` and `sum9-of`, the entry `arith9`, the conversions `to9`/`as9` and `from9`/`of9`, and the flags `overflow` and `big`**; [S1f.6.13](p1f.6_calculator/s1f.6.13_num9.md) |
| [Q-M1f.168](#q-m1f168--overflow-and-division-by-zero-in-stdnum9) | overflow and division by zero in `std.num9` | **Confirmed 2026-11-12** (reply 51 (a)). **defaulted 2026-11-11 (S1f.6.13): overflow a fact naming the request, root alive, never a number; a division by zero `std.arith`'s own flag relation and its rule to `(false)`, imported**; [S1f.6.13](p1f.6_calculator/s1f.6.13_num9.md) |
| [Q-M1f.169](#q-m1f169--where-stdnum9s-multiples-and-rows-live) | where `std.num9`'s multiples and rows live | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.13): in the state and in the step rule, so no rule joins two facts on a number: one rule computes the nine multiples, ten step rules read them, and a product's step computes its own row**; [S1f.6.13](p1f.6_calculator/s1f.6.13_num9.md) |
| [Q-M1f.170](#q-m1f170--what-stdnum9-reads-as-an-operand) | what `std.num9` reads as an operand | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.13): a number written out; no named constants; a conversion reads a number as written, high zeros counted, a name not looked up**; [S1f.6.13](p1f.6_calculator/s1f.6.13_num9.md) |
| [Q-M1f.171](#q-m1f171--stdnum9s-step-counter-and-its-comparison) | `std.num9`'s step counter and its comparison | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.13): `std.digits`' `adj` and `cl`, no table of the module's own**; [S1f.6.13](p1f.6_calculator/s1f.6.13_num9.md) |
| [Q-M1f.172](#q-m1f172--the-harness-of-stdnum9-exhaustive-at-width-2) | the harness of `std.num9`: exhaustive at width 2 | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.13): the generator's `--width 2 --stdout` in front of the test's requests, every pair under five operations; no second width is shipped**; [S1f.6.13](p1f.6_calculator/s1f.6.13_num9.md) |
| [Q-M1f.173](#q-m1f173--max_regs-and-the-wide-rules-of-stdnum9) | `MAX_REGS` and the wide rules of `std.num9` | **Confirmed 2026-11-12** (reply 50 (a)). **defaulted 2026-11-11 (S1f.6.13), the user's to overrule: the bound stays 256; `std.num9`'s generated rules are measured apart, the widest pinned at 162 registers, and every other corpus rule is held to an eighth of the bound as before**; [S1f.6.13](p1f.6_calculator/s1f.6.13_num9.md) |
| [Q-M1f.174](#q-m1f174--the-names-of-stdint) | the names of `std.int` | **2026-11-12** (reply 52): the relation names stand; the modules move under `std.math`, [S1f.6.31](p1f.6_calculator/s1f.6.31_the_stdlib_layout.md). **defaulted 2026-11-12 (S1f.6.7), the user's to revise: a prefix, `int-add` answered by `int-sum-of` and so on, the entry `int-arith`**; [S1f.6.7](p1f.6_calculator/s1f.6.7_negative_numbers.md) |
| [Q-M1f.175](#q-m1f175--what-stdint-reads-as-an-operand) | what `std.int` reads as an operand | **Kept as built 2026-11-12** (reply 12); promotion is [M1g's P1g.8](../m1g_logic/p1g.8_a_theory_of_types/README.md), with a types theory. **defaulted 2026-11-12 (S1f.6.7), the user's to revise: an integer written out or a name through `std.arith`'s `let`; a natural number is not promoted**; [S1f.6.7](p1f.6_calculator/s1f.6.7_negative_numbers.md) |
| [Q-M1f.176](#q-m1f176--a-product-of-integers-without-a-rule-that-looks-at-a-sign) | a product of integers without a rule that looks at a sign | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-12 (S1f.6.7), the user's to revise: the formula on the parts, four products and no case**; [S1f.6.7](p1f.6_calculator/s1f.6.7_negative_numbers.md) |
| [Q-M1f.177](#q-m1f177--a-calls-tag-is-its-request-and-what-an-extension-writes-of-stdariths) | a call's tag is its request, and what an extension writes of `std.arith`'s | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-12 (S1f.6.7), the user's to revise: the request is the tag; the module starts `std.arith`'s walks and writes `operand` and `operand-is`**; [S1f.6.7](p1f.6_calculator/s1f.6.7_negative_numbers.md) |
| [Q-M1f.178](#q-m1f178--the-harness-of-stdint-a-sibling-file-exhaustive-on-one-digit-parts) | the harness of `std.int` | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-12 (S1f.6.7): a sibling file, `int_theory.rs`, exhaustive on parts of one digit**; [S1f.6.7](p1f.6_calculator/s1f.6.7_negative_numbers.md) |
| [Q-M1f.179](#q-m1f179--the-remainder-of-an-integer-division) | the remainder of an integer division: Euclidean, truncated or floored | **Ruled 2026-11-12** (replies 1 (d), 2 (a)): all three conventions, each its own request; the Euclidean remainder a natural number; **built 2026-11-12** by [S1f.6.22](p1f.6_calculator/s1f.6.22_integer_division.md). Before: **open, the user's; not defaulted.** Integer division is not built until it is ruled; the stage recommends Euclidean and shows the three as rules and computed examples; [S1f.6.7](p1f.6_calculator/s1f.6.7_negative_numbers.md) |
| [Q-M1f.180](#q-m1f180--the-names-of-stdrat) | the names of `std.rat` | **2026-11-12** (reply 52): the relation names stand; the modules move under `std.math`, [S1f.6.31](p1f.6_calculator/s1f.6.31_the_stdlib_layout.md). **defaulted 2026-11-12 (S1f.6.8), the user's to revise: a prefix, `rat-add` answered by `rat-sum-of` and so on, `rat-div` by `rat-quot-of`, one rational; the entry `rat-arith`**; [S1f.6.8](p1f.6_calculator/s1f.6.8_rational_numbers.md) |
| [Q-M1f.181](#q-m1f181--what-stdrat-reads-as-an-operand) | what `std.rat` reads as an operand | **Kept as built 2026-11-12** (reply 12); promotion is [M1g's P1g.8](../m1g_logic/p1g.8_a_theory_of_types/README.md), with a types theory. **defaulted 2026-11-12 (S1f.6.8), the user's to revise: a rational written out, `(rat (int P N) D)`, or a name through `std.arith`'s `let`; an integer or a natural number is not promoted**; [S1f.6.8](p1f.6_calculator/s1f.6.8_rational_numbers.md) |
| [Q-M1f.182](#q-m1f182--a-zero-denominator-is-stdariths-division-by-zero) | a zero denominator is `std.arith`'s division by zero | **Confirmed** in round 2 (`dc88520b`) (item 48, point 2: *"OK, I agree"*). **defaulted 2026-11-12 (S1f.6.8), the user's to revise: the same flag relation, the request as written, on any operand of any request, and on a `rat-div` by a rational that is zero**; [S1f.6.8](p1f.6_calculator/s1f.6.8_rational_numbers.md) |
| [Q-M1f.183](#q-m1f183--lowest-terms-by-stdariths-division-and-what-it-costs) | lowest terms by `std.arith`'s division, and what it costs | **Ruled** in round 2 (`dc88520b`) (item 54, *"(b)"*): reduce only on request; [S1f.6.33](p1f.6_calculator/s1f.6.33_lowest_terms_on_request.md). **defaulted 2026-11-12 (S1f.6.8), the user's to revise: Euclid's chain over `div-call`, one reduction at the end of each operation, none in a comparison; 492 firings for 1/2 + 1/3**; [S1f.6.8](p1f.6_calculator/s1f.6.8_rational_numbers.md) |
| [Q-M1f.184](#q-m1f184--the-harness-of-stdrat-and-the-sizes-memory-allows) | the harness of `std.rat`, and the sizes memory allows | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-12 (S1f.6.8): 28 operands exhaustively, 250 requests a saturation, batches of 100, 400 and 1 200**; [S1f.6.8](p1f.6_calculator/s1f.6.8_rational_numbers.md) |
| [Q-M1f.185](#q-m1f185--a-state-file-nested-past-the-parsers-limit) | a state file nested past the parser's limit (E-4): say it, flag it, or change the engine | **Ruled 2026-11-12** (reply 44 (a)): stated on the page and left. **open, the user's; not defaulted.** The stage recommends (a), say it on the page, which it has done; (b) and (c) are probed and not built; [S1f.6.9](p1f.6_calculator/s1f.6.9_width.md) |
| [Q-M1f.186](#q-m1f186--where-the-probes-of-s1f610-are-banked) | where the probes of S1f.6.10 are banked | **Confirmed 2026-11-12** (reply 43 (a)). **defaulted 2026-11-12 (S1f.6.10), the user's to revise: `examples/beyond_arith/`, beside `examples/arith/` and not under it**; [S1f.6.10](p1f.6_calculator/s1f.6.10_beyond_arithmetic.md) |
| [Q-M1f.187](#q-m1f187--which-probes-of-s1f610-are-banked-and-at-what-size) | which probes of S1f.6.10 are banked, and at what size | **Confirmed 2026-11-12** (reply 43 (a)). **defaulted 2026-11-12 (S1f.6.10): sixteen, each sized so that a bare `ein solve` exhausts R well inside the budget and its dump is a few megabytes; SEND + MORE = MONEY stays under `build/`**; [S1f.6.10](p1f.6_calculator/s1f.6.10_beyond_arithmetic.md) |
| [Q-M1f.188](#q-m1f188--the-banked-probes-are-kept-by-hand) | the banked probes are kept by hand | **Confirmed 2026-11-12** (reply 43 (a)). **defaulted 2026-11-12 (S1f.6.10): written once by a script under `build/`, then hand-kept; no generator in `utils/` and no `--check` in the gate**; [S1f.6.10](p1f.6_calculator/s1f.6.10_beyond_arithmetic.md) |
| [Q-M1f.189](#q-m1f189--a-users-program-calling-stdariths-tagged-calls) | a user's program calling `std.arith`'s tagged calls | **Answered** in round 2 (`dc88520b`) (item 41, *"(a)"*): the tagged calls are an interface; [S1f.6.30](p1f.6_calculator/s1f.6.30_name_clashes_between_imported_theories.md), T1. **defaulted 2026-11-12 (S1f.6.10) for the banked programs, and put to the user: eight of them call `add-walk`, `mul-call`, `div-call`, `cmp-walk` and read `done`, `div-done`, `order-is`**; [S1f.6.10](p1f.6_calculator/s1f.6.10_beyond_arithmetic.md) |
| [Q-M1f.190](#q-m1f190--which-variants-of-the-binary-prototypes-are-banked-where-the-page-recommends-none) | which variants of the binary prototypes are banked where the page recommends none | **defaulted 2026-11-12 (the banking of S1f.6.16 and S1f.6.17), the user's to revise: the simplest its results hold under: the truth table's candidates from the obligations rung alone; the conversions a module of their own; Post's criterion on seven written sets over the generated facts; the simplifier reading the generated facts**; [S1f.6.16](p1f.6_calculator/s1f.6.16_binary_arithmetic.md#record-2026-11-12-way-b-banked-as-examples), [S1f.6.17](p1f.6_calculator/s1f.6.17_binary_logic.md#record-2026-11-12-the-recommended-variants-banked-as-examples) |
| [Q-M1f.191](#q-m1f191--the-generators-of-the-binary-examples-are-run-by-hand) | the generators of the binary examples are run by hand | **defaulted 2026-11-12, the user's to revise: `gen_binary.py` and `gen_gates.py` live in the example directories, each with a `--check`, neither in the gate; the properties' check is a corpus program's claims, which is in the gate**; [S1f.6.16](p1f.6_calculator/s1f.6.16_binary_arithmetic.md#record-2026-11-12-way-b-banked-as-examples), [S1f.6.17](p1f.6_calculator/s1f.6.17_binary_logic.md#record-2026-11-12-the-recommended-variants-banked-as-examples) |
| [Q-M1f.192](#q-m1f192--one-table-file-in-two-example-directories) | one table file in two example directories | **defaulted 2026-11-12, the user's to revise: `examples/binary_logic/bits.ein` is a copy of `examples/binary_arithmetic/bits.ein`, written by `gen_binary.py`, whose `--check` holds the two equal**; [S1f.6.17](p1f.6_calculator/s1f.6.17_binary_logic.md#record-2026-11-12-the-recommended-variants-banked-as-examples) |
| [Q-M1f.193](#q-m1f193--the-sizes-of-the-banked-binary-programs) | the sizes of the banked binary programs | **defaulted 2026-11-12, the user's to revise: three inputs for every search, operands to seventeen bits, conversions to 255 and from 1000, none `slow`; the batches, the scale runs and the enumeration of all 65 535 sets stay under `build/`**; [S1f.6.16](p1f.6_calculator/s1f.6.16_binary_arithmetic.md#record-2026-11-12-way-b-banked-as-examples), [S1f.6.17](p1f.6_calculator/s1f.6.17_binary_logic.md#record-2026-11-12-the-recommended-variants-banked-as-examples) |
| [Q-M1f.194](#q-m1f194--which-of-the-word-problem-prototypes-programs-are-banked-and-under-what-names) | which of the word-problem prototype's programs are banked, and under what names | **defaulted 2026-11-12 (S1f.6.20), the user's to revise: the recommended variant of each choice; `readings_plain` banked as `readings.ein`, `gap_b` as `gap.ein`; the other forms, mechanisms and witnesses stay on the page**; [S1f.6.20](p1f.6_calculator/s1f.6.20_word_problems.md) |
| [Q-M1f.195](#q-m1f195--one-file-for-the-two-step-problem) | one file for the two-step problem | **defaulted 2026-11-12 (S1f.6.20), the user's to revise: `boxes12.ein` is the page's `boxes12` and `boxes12_plain` together, `readings` imported and the model's query kept**; [S1f.6.20](p1f.6_calculator/s1f.6.20_word_problems.md) |
| [Q-M1f.196](#q-m1f196--the-word-problem-modules-as-corpus-entries) | the word-problem modules as corpus entries | **defaulted 2026-11-12 (S1f.6.20), the user's to revise: each module is a `positive` entry with runs of its own, as the stdlib's modules are; `gaps.ein` declares no `solve`; `readings.ein` imports only what it reads (E-11)**; [S1f.6.20](p1f.6_calculator/s1f.6.20_word_problems.md) |
| [Q-M1f.197](#q-m1f197--what-holds-the-banked-word-problems) | what holds the banked word problems | **defaulted 2026-11-12 (S1f.6.20), the user's to revise: each file's `:expect` through its `test` cell, and the sweeps every corpus file is under; no test of their own until the rulings**; [S1f.6.20](p1f.6_calculator/s1f.6.20_word_problems.md) |
| [Q-M1f.198](#q-m1f198--the-remainder-variant-of-the-word-problems-and-the-gate) | the remainder variant of the word problems, and the gate | **2026-11-12:** owned by [S1f.6.28](p1f.6_calculator/s1f.6.28_word_problems_second_pass.md)'s first task. **defaulted 2026-11-12 (S1f.6.20), the user's to revise: not banked. `region.rs`'s fact-grain sweep does not end on a program where a supposed reading leads to a `value` (E-33); the test and the engine are unmoved; three other ways are stated and one was run**; [S1f.6.20](p1f.6_calculator/s1f.6.20_word_problems.md) |
| [Q-M1f.199](#q-m1f199--how-the-skipped-views-of-the-dot-sweep-are-pinned) | how the skipped views of the DOT sweep are pinned | **defaulted 2026-11-12 (S1f.6.21), the user's to revise: the count printed, the directories that hold a skipped view named in the test**; [S1f.6.21](p1f.6_calculator/s1f.6.21_the_gate_reduced.md) |
| [Q-M1f.200](#q-m1f200--the-names-of-an-integer-divisions-results) | the names of an integer division's results | **defaulted 2026-11-12 (S1f.6.22), the user's to revise: `int-quot-euclid-of`, `int-quot-trunc-of` and `int-quot-floor-of`, each `(… X Y Q R)`**; [S1f.6.22](p1f.6_calculator/s1f.6.22_integer_division.md) |

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

**Built 2026-11-09** by [S1f.5.18](p1f.5_the_no_owner_items/s1f.5.18_an_activator_binds_a_nested_fact.md)'s
T1f.5.18.3: both `g14` programs are refused at load, one line each, and no corpus program that answers is (W59's two
rows).

### Q-M1f.2 — The 55 files the grammar refuses that today's parser accepts

**Raised** 2026-11-08 by S1f.5.5's step 2
([paper grammar, Q2](p1f.5_the_no_owner_items/s1f.5.5_paper_grammar.md#q2--the-55-files-the-grammar-refuses-that-todays-parser-accepts)),
re-taking the stage's 34. None is a puzzle, a stdlib module or a `tests/` program. The agent proposed (a):
the 26 refusal fixtures whose refusal the grammar takes over stay refusal fixtures and move to
`examples/broken/`, except `reserved_open_direct`, which three fixtures import by path;
`derivation_cycle` and five `examples/syntax/` fixtures are rewritten; 17 register witnesses and 6
regression fixtures are the owning stages' (S1f.5.1, S1f.5.2, S1f.5.4, S1f.5.11). The alternatives were (b)
the 26 left in place with only their `.expected` moved, and (c) the 26 rewritten.

**Ruled 2026-11-08: (a).** The user: *"Q2 - a"*. Since the user's re-plan the same day, the 23 the owning stages
had land in S1f.5.5's swap, each credited to its stage's ruling, and of the rest what moves no answer lands just
before it ([S1f.5.5 § The order](p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md#the-order-re-planned-2026-11-08)).

### Q-M1f.3 — Three wrong answers the census found with no row

**Raised** 2026-11-08 by S1f.5.5's step 1
([census § Findings outside the grammar](p1f.5_the_no_owner_items/s1f.5.5_census.md#findings-outside-the-grammar)):
an activator binding a value parameter to a nested fact leaves it unbound; `:assert ()` stores a fact
named `@empty`, which joins H; and `(absent ())` holds. The agent recommended filing all three now as
`open` rows, W33–W35, as a fourth task of S1f.5.0: W33 owned by a new stage, S1f.5.18, where the parameter
binds as any value does; W34 by [S1f.5.9](p1f.5_the_no_owner_items/s1f.5.9_an_empty_match_fires.md), whose
case table it corrects; W35 by S1f.5.5, whose grammar refuses `(absent ())`.

**Ruled 2026-11-08, as recommended.** The user: *"Q3 - as recommended"*.

W33 **fixed 2026-11-09** by S1f.5.18's T1f.5.18.2: an activator binds a parameter to a nested fact as to a name. W34
the same day by [S1f.5.9](p1f.5_the_no_owner_items/s1f.5.9_an_empty_match_fires.md)'s T1f.5.9.2, `:assert ()` asserting
nothing, and W35 by S1f.5.5's swap (`8c3fe86e`), `(absent ())` refused at the parse.

### Q-M1f.4 — Removing `:priority` moves the facts of cores on seven corpus programs

**Raised** 2026-11-08 by [S1f.5.15](p1f.5_the_no_owner_items/s1f.5.15_rule_priority.md)'s T1
([§ T1 record](p1f.5_the_no_owner_items/s1f.5.15_rule_priority.md#t1-record-2026-11-08)), which stopped on it
before any removal. Taking every `:priority` out of the corpus and the stdlib (`without_priority`, 475 clauses in
136 files) moves **no** R, N, H, claim, exit code or stored fact on any of the 511 programs under the five roots,
nor on any of the corpus's 1 378 declared runs. It moves **the facts of cores**: on `zebra.ein` 292 of N's 458
(13 193 → 14 697 facts over N), on the zebra2 family 24 to 215 a program (+0.4 % to +3.7 % of N's facts), and on
W25's control, `guard-only-first.ein`, whose core `(p a)`, the row's right answer, becomes the witness's wrong
`(b a) (p a) (seed a)`. Each move goes through an order-dependency the store already has, each owned: the
alternatives cap (the zebra2 family; at 10⁶ the two arms agree; Q-M1e.15, F22), `:record-alternative-
justifications false`, which `zebra.ein` sets (the config principle's one exemption, M1e2 S1e2.2.11), and W25
(F22). The typed number decides the order those three read; without it the written order does. The same
stripped corpus also takes away the `alt-cap-core` pair's difference (they are one `:priority` apart) and shows
[Q-M1f.5](#q-m1f5--the-alternatives-flag-moves-refuted-by-beside-an-unmoved-core)'s breach on a corpus program.

- **(a) Remove it now, and accept the cores' moves.** The agenda is FIFO within one priority, the written order.
  W25's control is restated as a program whose `w` derivation arrives first by its rules, not by a number;
  the `alt-cap-core` pair's tests and the flag's exemption take a witness that differs by its written order;
  and every moved core is named before the commit that moves it, as F22's to make order-free.
- **(b) Derive the order first, then remove.** T3 builds the rule graph and a band per rule from its shape; the
  closure's agenda is ordered by that band where the number orders it today, and the boundary stays FIFO, since
  admitting by stratum is F21's D2. The sweep is re-taken with it before any removal: if the cores hold, the
  removal moves none, and what still moves comes back to the user as in (a). It takes the user's note of
  2026-08-28 literally, *"remove `:priority` completely / analyze rules structure instead"*.
- **(c) Keep `:priority` until F22 makes a core a function of the program**, the written refusal the stage's
  acceptance allows: the production comes back to the generated grammar before S1f.5.5's T9, S1f.5.4's
  `:priority` guard with it, and the graph, the strata and the diagnostic (T3, T4) ship either way.

**The agent recommends (b)**, and (a) for whatever (b) leaves. No order is semantic, but the bands the authors
typed buy smaller cores on the puzzles, and a band derived from the rules is the replacement this stage was
written to find. (b) moves no answer of its own, since no order moved R, N, H or a claim on the corpus, and its
measurement says whether (a)'s cost remains.

**Ruled 2026-11-08, (b).** The user chose *"Derive order first"*: T3's band, derived from each rule's shape,
orders the agenda where the number does today, the boundary FIFO; the sweep is re-taken before any removal, and
what still moves comes back to the user.

### Q-M1f.5 — The alternatives flag moves `refuted by` beside an unmoved core

**Raised** 2026-11-08 by the same T1, whose snapshot of the corpus without `:priority` failed `lever_invariance`.
`examples/features/12_expect_false.ein` written without its `:priority` prints `refuted by no-room` and, with
`(config :record-alternative-justifications false)`, `refuted by forced, occupied`, beside the same five-fact
core (`ein solve`, today's binary, 2026-11-08). The flag's exemption is the core (the user on Q-M1e2.11 (ii),
*"(b) Exempt the core"*, which `configuration.md` § 2.5 words *"and nothing else"*), and `refuted by` may follow
it only where the core moved (Q-M1e2.21 (a), argued from *"where the core is the same the primaries are a
derivation from it"*). Here one
core derives the contradiction through two rules, and the flag decides which one the store keeps. **It is a
wrong answer today**, on a program anyone can write; the typed order is what keeps the corpus from showing it,
and if `:priority` goes (Q-M1f.4 (a) or (b)) `features/12` shows it and the gate fails.

- **(a) File it now as an `open` register row, W36**, its witness `features/12` without `:priority` under
  `examples/ein-bugs/wrong-answers/`, owned by F22, whose search re-derives what the store does not keep, so
  that `refuted by` no longer depends on it. `lever_invariance` exempts the row's program until then.
- **(b) Widen the exemption to the core and the rules `refuted by` names**, beside or without a moved core:
  `refuted by` is the core's explanation, and the flag decides what the store keeps for explanations.
  `configuration.md` § 2.5 and `lever_invariance` say so; no row.

**The agent recommends (a)**: the user exempted the core, not its explanation, and a wrong answer is a row
before it is fixed.

**Ruled 2026-11-08, (a).** The user chose *"Register row W36"*: `open`, owned by F22, its witness
`features/12` without `:priority` under `examples/ein-bugs/wrong-answers/`, and `lever_invariance` exempting that
program until F22 fixes it. S1f.5.15 files it before the removal: filed the same day by T1f.5.15.7 as
`W36/alt-flag-refuted-by`, the witness `examples/ein-bugs/wrong-answers/alt-flag-refuted-by.ein` with the flag set
in the file, under the register's schema `/14`.

### Q-M1f.6 — The derived agenda holds the cores; the FIFO boundary does not

**Raised** 2026-11-08 by [S1f.5.15](p1f.5_the_no_owner_items/s1f.5.15_rule_priority.md)'s T3
([§ T3 record](p1f.5_the_no_owner_items/s1f.5.15_rule_priority.md#t3-record-2026-11-08)), on Q-M1f.4 (b)'s
instruction that whatever still moves comes back to the user. T3 derives a band per rule from its shape and, in a
`derived-order` build, orders the closure's agenda by it with the boundary FIFO, as ruled. T1's sweep re-taken
with it (that build on the corpus without `:priority`, against today's binary on the corpus as written) moves no
R, N, H, claim, exit code or stored fact on the 512 programs or the 1 380 declared runs, and moves **the facts of
cores** on the same programs T1 found: `zebra.ein` 292 of N's 458 (13 193 → 14 699 facts), the zebra2 family 24 to
212 a program, and W25's control; plus `W29/core-two-ways`' witness, 4 facts → 2, its twin's, and three records;
and `refuted by` beside an unmoved core on the zebra family and `zebra2-bad`.

Measured one part at a time on builds of T3's commit: **the derived agenda with the declared boundary moves no core
of `zebra.ein`, the zebra2 family or `zebra2-bad`**; the declared agenda with a FIFO boundary moves exactly what the
derived arm moves. So the typed numbers' work that the shape does not reproduce is at the **boundary**, where the
candidates of `absent`-reading rules (`domain-elimination`, the `slot-*` exclusions and prunes) are admitted one a
round. On the zebra2 family that order chooses which 32 derivations the alternatives cap keeps (uncapped, no core
moves, Q-M1e.15 and F22's); on `zebra.ein` it is the first derivation, `:record-alternative-justifications false`
(the flag's exemption). The agenda moves the rest, W25's control (`early` fires before `w`, all three rules
`propagate`) and W29's witness (`s` before `m1`), whichever the boundary.

- **(a) Remove it as ruled, the boundary FIFO, and accept the moves**: every move above is named in the removal
  commit as F22's to make order-free; W25's control and W29's `now` are restated; the `alt-cap-core` tests and the
  flag's exemption take a witness that differs by its written order; `features/12` joins W36's exemption.
- **(b) Order the boundary by the derived band as well, then remove.** The band is the agenda's key, not a stratum,
  and the boundary is ordered by a key today (`:priority`). Measured over the whole corpus: the zebra2 family and
  `zebra2-bad` hold every core and `refuted by`; 35 of `zebra.ein`'s 458 cores move (through the flag's first
  derivation), every one smaller or the same size (13 193 → 13 136 facts over N); W36's witness prints its right
  answer, `refuted by no-room`; and the agenda's moves of (a) remain. What is left is restated as in (a). On a program with a negative cycle the band picks the
  admission order where `:priority` does today, which is F21's D2's ground.
- **(c) Keep `:priority` until F22 makes a core a function of the program**, the written refusal the stage allows:
  the production comes back to the generated grammar before S1f.5.5's T9, S1f.5.4's guard with it, and the graph,
  the strata, the band read-out and T4's diagnostic ship.

**The agent recommends (b)**, then (a) for the 35 and the agenda's moves. It keeps the discipline the typed numbers
gave the boundary with a key derived from the rules, holds every core the cap decides, and leaves only the moves the
order-dependencies the store already has decide (`zebra.ein`'s flag, W25, W29), each owned. It does take an order at
the boundary, which the ruling of Q-M1f.4 left FIFO on purpose: if that is D2's to decide, (a) is the ruling's own
consequence and (c) the refusal.

**Ruled 2026-11-08, (b).** The user chose *"Order boundary by band"*: the boundary admits its candidates in
derived-band order, as the agenda does, and `:priority` is removed; what still moves (`zebra.ein`'s cores through
the flag, W25's control, W29's witness, the records) is named in the removal commit as F22's, each witness restated.

**Re-ruled 2026-11-08, (a).** Asked what `:priority` is for, the user reasoned that it orders saturation and is in
effect an optimisation, so that a removal needs only the saturation results to agree, not the numbers reproduced
(*"it is not about particular values of priorities, it is about order of rules during saturation. What if
completely remove priority and don't make ordering of rules at all, what changes?"*). Told that T1's sweep is that
experiment (no stored fact, R, N, H, claim or exit code moved; cores, `refuted by`, narration and cost did), that
no order and no band makes a core a function of the program, and that the choice is whether a core is part of the
answer, the user ruled: *"plain FIFO, core is an explanation, F22 makes it canonical"*. So the agenda and the
boundary are FIFO, the written order, and no band orders either; Q-M1f.4 (b)'s derived agenda is withdrawn with
this (b). Every core and `refuted by` the removal moves is named in its commit as F22's; each stays a valid core
(`ein-infer/tests/corpus_cores.rs`). T3's rule graph and strata stay for T4's diagnostic; nothing orders by them.

### Q-M1f.7 — Where may a program write the kernel's dunders?

**Raised** 2026-11-08 by [S1f.5.5](p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md)'s T11
([§ T11 record](p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md#t11-record-2026-11-08)), before any edit. The user
ruled at the T9 review stop that the engine's five names *"are also reserved names"*, and the coordinator read it,
for the user to correct: the three writeback names `<forced-positive>`, `<monotonic-unconditional>` and
`<lookahead-dies-immediately>` are refused wherever a program writes them, and the dunders `__closed__` and
`__symmetric__` keep the positions the corpus writes them in, the relation of a fact, of a pattern and of an
`:assert`, and are refused as a declared relation, a rule's name, or **an argument value**. The writeback half
meets nothing. The dunder half meets two things the reading did not see:

- **The stdlib documents a dunder as an argument value, and a test holds it.** `stdlib/algebra.ein`'s `imply1`,
  *"Doubles as property→property: (imply1 functional __closed__) turns a functional relation closed"*;
  [`07_stdlib_api.md`](../../docs/kernel/ir/03-ein-lang/07_stdlib_api.md)'s `imply1` row, *e.g. `(imply1
  functional __closed__)`*; and `ein-infer/tests/algebra_semantics.rs`'
  `imply1_lifts_a_property_marker_onto_a_kernel_trigger`, which holds that the program derives `(__closed__
  foo)`, *"userspace reaching a kernel switch"*. Today's binary and T9's generated build both derive `(__closed__
  foo :rule imply1)` from it (`ein saturate --dump`, `build/s1f.5.5/t11/imply1.ein`, 2026-11-08). No corpus
  program writes it outside that comment (`grep` over `examples/`, `tests/`, `stdlib/`, `corpus/`, 2026-11-08), so
  no corpus answer moves either way.
- **The stage file's test cannot hold under the reading.** T11 asks for T9's probe refused at each of its five
  names; the probe writes its dunders as `(__symmetric__ s)` and `(__closed__ hit)`, the relation of a fact, which
  the reading keeps legal. Under the reading the probe is refused at its three writeback names only.

Where each position stands (`grep` of every `.ein` under the five roots, the Rust suites' inline programs, and
two probes on today's binary and T9's generated build, 2026-11-08):

| position | written by | today, both builds | the reading | (b) |
|---|---|---|---|---|
| the relation of a fact, `(__closed__ is-a)` | `branching/12`, `features/06` (three) | loads | legal | legal |
| the relation of a premise, `:match (__closed__ ?R)` | `tests/stdlib/closure/02`, `/03` | loads | legal | legal |
| the relation of a conclusion, `:assert (__closed__ ?R)` | `stdlib/closure.ein` | loads | legal | legal |
| an argument value, `(imply1 functional __closed__)` | `stdlib/algebra.ein`'s comment, `07_stdlib_api.md`, `algebra_semantics`; `(a __closed__)` in `ir_semantics`' and `grammar_decisions`' parse tables | loads, derives `(__closed__ foo)` | refused | legal |
| a declared relation, `(relation __closed__ T)` | none | loads, and declares it | refused | refused |
| a rule's or hrule's name, `(rule __symmetric__ () …)` | none | loads, and its firings print `:rule __symmetric__`, the native mirror's provenance | refused | refused |
| a macro's name, an import's `:as` | none | loads | not stated | refused |
| a query's goal, a claim's fact, a nested fact's relation | none | loads | not stated (a pattern's or a fact's relation?) | legal |
| a `:hypothesis-relations` or `:no-hypothesis` list; a fact's `:rule __symmetric__` | none | loads | not stated (an argument?) | legal |

- **(a) The reading as written.** A dunder is legal only as a relation: of a fact, a premise, a conclusion, and,
  read the same way, of a goal, a claim's fact and a nested fact; refused everywhere else, an argument value and a
  name list included. `imply1`'s documented use is refused at load: the comment and the API row go, and
  `algebra_semantics`' test is restated to the refusal (a program that wants its functional relations closed
  writes the rule, `:match (functional ?R) :assert (__closed__ ?R)`). The refusal of an argument is a loader line
  (the grammar's `Name` is every name), which both parsers share.
- **(b) Never bound.** A dunder is refused where a program **binds** a name, the positions where the loader already
  refuses the kernel's `RESERVED` names (*shadows a reserved kernel name*): a declared relation, a rule's, an
  hrule's or a macro's name, and an import's `:as`. It is legal wherever a name is **used**, as a relation or as a
  value, which keeps `(imply1 functional __closed__)`. One loader check, `is_reserved`'s, shared by both parsers.
  T11's test is T9's probe refused at its three writeback names, and a probe binding each dunder in each of those
  positions refused at each.
- **(c) Never bound, and a value only where it names a relation**: (b), and a dunder as an argument refused unless
  the argument binds a rule's relation parameter, as `imply1`'s `?R2` does. A check on activators, not on
  positions, for a case no program writes.

**The agent recommends (b).** *Reserved* is written *"only with its fixed meaning, never redefined"* (§ The review
stop ruled), and only a binding position can give a dunder another meaning; a value names the kernel's trigger,
which is how `imply1` hands it to a rule that asserts it, a stdlib idiom that works on both builds today. Under
every option the three writeback names are refused wherever a program writes them: in the generated grammar as a
token of their own that no production takes, with its own text, and today at the parse already, `<` being no name
character in the hand-written lexer.

**Ruled 2026-11-08: (b).** The user chose *"Never bound (b)"*. A dunder is refused where a program binds a
name: a declared relation, a rule's, an hrule's or a macro's name, and an import's `:as`. It is legal wherever a
name is used, as a relation or as a value, so `(imply1 functional __closed__)`, its comment, its API row and
`algebra_semantics`' test stay as they are. The three writeback names are refused wherever a program writes
them. T11 builds it.

### Q-M1f.8 — The parser's remaining cost: within grmtools, or past it?

**Raised** 2026-11-09 by [S1f.5.5](p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md)'s T12
([§ T12 record](p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md#t12-record-2026-11-09)), which recovered what it
could of the swap's cost inside grmtools: one rule for whitespace and comments and a start condition in `ein.l`,
and a lexer per thread in `grammar.rs`, which halve the lexing and move no token. What remains of the cost is the
shape of lrlex itself: it runs the regex of every rule its start condition allows at every position (55 rules
where a token starts), and it compiles every rule's regex once a process. Recovering more needs one of two things
the user's rulings rule out today, so the brief would have stopped here:

- **(a) Within grmtools**, as T12 leaves it: the lexer generated from `ein.l` by lrlex as published, `lrpar`
  patched only for F11.
- **(b) A second vendored patch, to `lrlex`**: its loop tries only the rules whose regex can start with the byte
  at the cursor, each rule's first bytes read off its DFA when the lexer is built. Measured by a probe of that loop
  over T4's rules, the 414 corpus files lex in 11.1 ms a pass, against lrlex's 108.3 ms on T4's `ein.l` and
  57.1 ms on T12's, the tokens the same on every file (`lexbench dispatch`, pinned to one P-core, 2026-11-09); it
  recovers lexing, not the 55 regexes a process compiles.
- **(c) A hand-written lexer feeding `lrpar`**, which the user's *"if speed will ever become an issue - will add
  super-optimized hand-written parser"* (2026-11-07) anticipated for the whole parser. The old hand-written parser
  is the measure of what it buys: `saturate` of `01_copiers.ein` 1.86 ms against T12's 4.77, of `zebra.ein` 4.67
  against 9.02 (pinned to one P-core, 50 runs each, 2026-11-09).

**Defaulted 2026-11-09 (the user's instruction): within grmtools; alternatives parked for the close.** The user,
2026-11-09: *"just continue with default decisions till the end of the phase"*. So (a) stands, and (b) and (c)
are parked for the phase's close with the numbers above ([§ T12 record, Parked for the phase
close](p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md#parked-for-the-phase-close)).

### Q-M1f.9 — What a deleted loader line leaves behind

**Raised** 2026-11-09 by [S1f.5.5](p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md)'s T5
([§ T5 record](p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md#t5-record-2026-11-09)), which deleted the 55 refusal
lines the grammar covers since the swap, each shown unreachable from any program the parser accepts. Where a line
was an arm of a match on the tree's shape, something has to stand in its place, because `ein_ir::load` is public
API (`docs/api/rust.md`) and an `Ast` can be built by hand, past the grammar:

- **(a) An `expect`, `unreachable!` or `assert!` naming the production of `ein.y` that states the shape**: a
  hand-built `Ast` the grammar would refuse panics with the grammar's name for what was wanted, and the readers that
  can no longer fail say so in their types (`expect::parse`, `expect::fact`, `config_from_kw_pairs`).
- **(b) A single internal load error** in place of each, *a shape the grammar refuses*, keeping every reader's
  `Result`.
- **(c) Keep the lines**, as `examples/broken/load/README.md` argued for six of them (*defence in depth for a
  caller that hand-builds an AST*), which the stage's instruction rules out: *each loader line the grammar now covers
  goes*.

**Defaulted 2026-11-09 (the user's instruction): (a).** The user, 2026-11-09: *"just continue with default
decisions till the end of the phase"*. (a) is the agent's recommendation: no program reaches any of them (940 tests
and 240 000 mutants with a panic at each, the record), a panic is louder than a load error for the one caller that
can, and (b) would keep 55 error arms for a shape nothing builds. Re-opening it means (b), at any later stage.

**Ruled 2026-11-09: (a).** The user, after the coordinator's demo that a hand-built `Ast` panics through the library
while no `ein` command reaches the panic: *"ok, keep (a), continue"*. The binary cannot reach these panics: every path
into the loader goes through the parser, a `.einb`'s stored text included, and the lines a macro reaches stayed loader
errors (the coordinator's 13 macro and top-level probes, 2026-11-09, each a parse error, a load error or a clean
load). It is a contract of the library's API alone.

### Q-M1f.10 — The `.einb` `INTS` section, once the int kind is gone

**Raised** 2026-11-09 by [S1f.5.4](p1f.5_the_no_owner_items/s1f.5.4_digits_are_names.md)'s T1f.5.4.3
([§ T1f.5.4.3 record](p1f.5_the_no_owner_items/s1f.5.4_digits_are_names.md#t1f543-record-2026-11-09)), which retired
`Value`'s int tag, the int pool and its id space. A `.einb` 1.2 file carries an `INTS` section (kind 3, a string
table, required by every 1.x reader) and stores a value as its 32 bits, the int's tag `0b01`. The stage file planned
the section retired *with its format version bumped*, and the brief asked that the format's rule be followed if the
layout changes and that an old file holding an int be refused cleanly, never misread. design/10 §4: the major bumps
on any layout change, and a minor bump only adds a section.

- **(a) Keep the section in the layout, written empty, and refuse a file that holds an int**: an `INTS` entry or a
  value tagged `0b01`, with its own sentence, `EinbError::Int`. Nothing about the layout changes, so no version
  moves: a 1.x reader opens what this writer writes (the empty table is the bytes the empty pool wrote), this reader
  opens what a 1.x writer wrote for a program with no int, and a file from a program with one is refused.
- **(b) Drop the section and bump the major to 2.0**: every 1.x file is refused with the version line, a file with
  no int among them; `ein --version`'s `container einb/1.2`, `docs/install.md`'s transcript and the reader's two
  `head.minor` checks move with it.
- **(c) Drop the section under a minor bump, 1.3**: a 1.2 reader refuses a 1.3 file for its missing `INTS`, which
  breaks the rule that a minor bump is additive.

**Defaulted 2026-11-09 (the user's instruction): (a).** The user, 2026-11-09: *"just continue with default
decisions till the end of the phase"*. (a) is the agent's recommendation: the plan's bump assumed the section had to
go, and (a) keeps its bytes so that nothing needs one; it moves no golden and no output (`ein-einb/tests/invalidation.rs`'
`a_file_that_holds_an_integer_is_refused` builds both shapes of an old file and holds the refusal), and the next
major bump, whenever one is made, can drop the section. Re-opening it means (b).

### Q-M1f.11 — A template reference to a variable no reference can name

**Raised** 2026-11-08 by [S1f.5.5](p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md)'s T9 review (§ 11) and carried
by [S1f.5.4](p1f.5_the_no_owner_items/s1f.5.4_digits_are_names.md)'s T1f.5.4.5, the fix of W37
([§ T1f.5.4.5 record](p1f.5_the_no_owner_items/s1f.5.4_digits_are_names.md#t1f545-record-2026-11-09)). A template
reference reads a variable's name up to the first `}`, and `}` is one of the name rule's characters (the user,
2026-11-07, *for now*), so a variable holding one, `?a}b`, cannot be named: `{?a}b}` is the reference `{?a}` and the
text `b}`. No corpus program writes such a variable.

- **(a) The loader refuses a template that writes a reference to a variable in its scope holding `}`**: a rule's,
  an hrule's or a macro's `:why` and a query's `:goal-text`, one line a reference naming the form and the clause.
- **(b) Leave it**: the reference reads `{?a}`, unbound or bound to another variable, and the sentence prints with
  `b}` after it, which its author did not write.
- **(c) Take `}` out of the name rule**, which re-opens the user's ruling on what a name is.

**Defaulted 2026-11-09 (the user's instruction): (a).** The user, 2026-11-09: *"just continue with default decisions
till the end of the phase"*. (a) is what S1f.5.5's T9 review recommended and S1f.5.4's task wrote down: a refusal at
load says so where (b) prints a wrong sentence in silence, and (c) is the user's to rule. It moves no corpus answer.
Re-opening it means (b), or (c) if the name rule changes.

### Q-M1f.12 — Which surfaces carry natural-language text

**Raised** 2026-11-09 by [S1f.5.2](p1f.5_the_no_owner_items/s1f.5.2_strings_are_natural_language.md)'s T1f.5.2.2,
from S1f.5.5's T9 review (its § *Unicode on every surface that carries it*), which said *which surfaces should carry
the text, and the test that holds each byte for byte, is S1f.5.2's T1f.5.2.2*. A string is natural-language text,
written only after `:why`, `:source` and `:goal-text` (Q-M1e2.23). Before and since the swap, on the T9 probe:
the read-out and `answer.txt` print a query's `:goal-text` filled; `--trace` a rule's `:why` filled at each firing
and a premise's `:source`; `ein saturate --dump` each `:source` and each `:goal-text`; a `--dump-answer` state file
all three, the program re-written; `ein render program` a `:goal-text`; and a `.einb` all three. `ein render rules`,
`kb`, `constraints` and `lattice`, `--events`, `summary.json` and `answer.json` carry none.

- **(a) The surfaces that carry it today, each held byte for byte, and none added.**
- **(b) Add the text where a reader might want it**: a relation's `:why` in the read-out, a rule's `:why` in `ein
  render rules`, a `:goal-text` sentence in `summary.json`, the event stream. Each is a new surface and a schema or
  golden move.

**Defaulted 2026-11-09 (the user's instruction): (a).** The user, 2026-11-09: *"just continue with default decisions
till the end of the phase"*. (a) moves nothing a user sees, which the stage's acceptance asks of it, and the ruling
asks that a program *can* emit Unicode text, which (a) holds on every surface that emits any;
`ein-cli/tests/natural_language_text.rs` holds it. Re-opening it means (b), surface by surface; [S1f.5.14](p1f.5_the_no_owner_items/s1f.5.14_nl_required.md),
which makes `:why` required, is where a reader of the text is most likely to ask for one.

### Q-M1f.13 — F3's cause reaches a disjunction: whose row is it?

**Raised** 2026-11-09 by [S1f.5.1](p1f.5_the_no_owner_items/s1f.5.1_bare_underscore_refused.md)'s T1f.5.1.3, step 1
([its record](p1f.5_the_no_owner_items/s1f.5.1_bare_underscore_refused.md#t1f513-record-2026-11-09)), which traced
P1f.5's F3: `_` in a rule's pattern reported root's core as four facts where a variable used once reported two. The
cause is a rule application's identity, its rule, activator and register file (`firing::BindingKey`): the saturator
drops a match whose key it has seen or fired, so a second match that binds every register as the first did is never
fired redundantly and never recorded as an alternative, whatever facts it matched. A premise's fact is a function of
the register file except through `Slot::Any`, which bound nothing, and across the disjuncts of one top-level `(or …)`,
which share their registers by name. With `_` refused since S1f.5.5's swap, the disjunction is left:
`examples/ein-bugs/wrong-answers/or-one-key.ein`, one rule `(or (p ?y) (q ?y))`, reports four facts where the same
rule written as two rules reports two. The brief: *a reach is a wrong answer, filed as a register row before anything
moves, and fixed if it is in scope*.

- **(a) File it `open`, deferred to F22**, as the user put W25 there on 2026-11-05 (Q-M1e2.16 (2) and (5): a
  derivation the saturator never offers to the store, from an activator a node derived). F22 decides which
  derivations a core is the smallest over, and how it gets them.
- **(b) Fix it in S1f.5.1**: the disjunct joins the application's identity (the `seen` and `fired` keys), so the
  second disjunct's match fires redundantly and records its alternative. It moves the cores of programs that write
  such a disjunction and the redundant firings `--events` narrates, and it changes the identity every fork inherits.
- **(c) Not a wrong answer**: the promise is *the smallest frontier over every derivation the program records*, and
  this derivation is never recorded. The register's W25 reads the promise over every derivation, recorded or not.

**Defaulted 2026-11-09 (the user's instruction): (a).** The user, 2026-11-09: *"just continue with default decisions
till the end of the phase"*. (a) is the agent's recommendation: it follows the user's ruling on W25, the same class of
defect, and keeps S1f.5.1, whose subject is `_`, from moving the saturator's dedup, which every match crosses. In the corpus at `1e751131` one program reaches the cause, `wild-in-head-twin.ein`, whose run
reports no core, so no printed answer moves today (the census, T1f.5.1.3's record). `W39/or-one-key` is the row.
Re-opening it means (b), in F22 or before it.

### Q-M1f.14 — Which keyword-pair arms go, once no term holds one?

**Raised** 2026-11-09 by [S1f.5.11](p1f.5_the_no_owner_items/s1f.5.11_closed_keywords.md)'s T1f.5.11.2
([its record](p1f.5_the_no_owner_items/s1f.5.11_closed_keywords.md#t1f5112-record-2026-11-09)). Since S1f.5.5's swap
the grammar builds a keyword pair only as a form's own clause, and no term or nested fact holds one (S1f.5.11's
ruling). The stage's brief names what goes: the compiler's in-`:match` drop and its filters, the macro expander's
arity filter, `from_ir.rs`' *not refused* note, and any arm in `value_label` or another renderer that only a pair
inside a term reaches. The census put a panic at 23 arms that are only ever handed a term, and none was reached by
any program (the record). Unlike `_`, whose node S1f.5.1 retired with every arm that read it, the pair stays a
node: every form's clauses are pairs.

- **(a) The drops, the validator's skips and the renderers' arms go; the walkers stay.** What drops, skips or
  labels a pair inside a term goes, an `unreachable!` naming `ein.y`'s production where the match needs an arm
  (Q-M1f.9 (a)). What descends into a pair's value as into a form's arguments (the expander's `expand` and
  `substitute`, the loader's term walkers, the invariant's constant walk, the guard key's code, `expect::render`,
  `node_repr`) stays, total over `Node` as the dumper and the import resolver are.
- **(b) Every arm the census found unreachable goes**, each walker's pair arm an `unreachable!` too.
- **(c) Only what the brief names**: the compiler's drop and filters, the arity filter, the note and
  `value_label`'s arm; the validator's skips and the rules view's filter, which skip a pair as the drop did, stay.

**Defaulted 2026-11-09 (the user's instruction): (a).** The user, 2026-11-09: *"just continue with default decisions
till the end of the phase"*. (a) is the agent's recommendation: it retires everything that treated a pair inside a
term as something to drop or label, which is what the acceptance names (*nothing in the compiler or the macro
expander drops a keyword pair*), and leaves the walks correct for a library caller that hands one a whole form,
where (b) would make each a panic and retire nothing the ruling names. No answer moves either way. Re-opening it
means (b).

### Q-M1f.15 — Where does a position in a container's stored program point?

**Raised** 2026-11-09 by [S1f.5.5](p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md)'s T1f.5.5.13, the coordinator's
item 4: a `.einb` whose stored program fails to compile at its first firing, S1f.5.5's parser probe
`g14/derived-activator.ein` saved by `ein kb save`, reports the rule at `Loc(file='<string>', line=9, col=27)`, where
the `.ein` reports `line=5`. A container stores its program as canonical text, the resolved and import-flattened form
list `dump_canonical` writes, and re-parses it on open; every position of the rebuilt `Ast` is the stored text's. The
text was parsed under the first file name the provenance table holds, which is empty on that program and most others,
so `<string>`, or, where it is not, under the source's name with the stored text's line. design/10 chose text over the
AST's arenas on a measurement: `zebra2`'s 3 024 nodes and as many `Loc`s are past 60 KB against a 64 KB budget for the
whole file, the canonical text 11 KB (`ein_einb::sections::program_section`).

- **(a) Name the container.** The stored text is parsed under `<program stored in x.einb>`, the path the command was
  handed, so a position says which text it is in, in brackets as `<string>` is since no file holds it, and its line is
  that text's. No format change.
- **(b) Store each node's source position**, a new optional section and a minor bump (einb 1.3), so a compile error in
  a container names the source's line. A second encoding beside the text, which design/10 measured against, and the
  `--version` line and the container's size goldens move.
- **(c) Leave it**: `<string>` and the stored line.

**Defaulted 2026-11-09 (the user's instruction): (a).** The user, 2026-11-09: *"just continue with default decisions
till the end of the phase"*. (a) is the agent's recommendation: it makes the position true, a line of the program the
container stores, under a name that says so, at no cost to the format, and only an error reaches it (a run that
answers prints no position, which is why `ein solve x.einb` is byte-identical to `ein solve x.ein`). The source's line
is (b), which re-opens design/10's size measurement. `program_read::a_containers_positions_name_its_stored_program`
holds (a).

### Q-M1f.16 — Which of the read's defects are register rows?

**Raised** 2026-11-09 by [S1f.5.5](p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md)'s T1f.5.5.13. The brief files the
coordinator's items 1 and 2 as rows, on the user's rule that a command refusing a program another answers is a wrong
answer. The task's survey (`06433b60`'s release build, every command on a `.ein`, its `.einb`, a file-relative import
and a file that is not UTF-8) found item 1's defect on five more commands, `render rules`, `rule`, `constraints` and
`program`, which parse a program and never load it, and `kb save`; and three defects that are no refusal of an answered
program: the non-UTF-8 line was Python's on every command, its `process::exit(1)` stopped `ein test`'s whole selection,
exit 1, a refuted claim's code, where a load error there is counted and exits 2, and an end-of-input error was placed at
the last token's end.

- **(a) A row per refusal of a program another command answers**: W40, one row per command that refused a `.einb`
  (`saturate`, the four `render` views, `kb save`), and W41, `saturate`'s import; the rest held by tests
  (`program_read`, `generated_parser::the_end_of_input_is_where_the_text_ends`), since every command refuses those
  files before and after, and a non-UTF-8 witness cannot be a corpus entry, whose readers take every `.ein` as text.
- **(b) Also a row for `ein test`'s stopped selection**, its witness a non-UTF-8 file under `corpus/fuzz_findings/`,
  which every reader of that directory would have to skip.
- **(c) Only the brief's two rows**, `saturate`'s, the other five commands fixed with no row.

**Defaulted 2026-11-09 (the user's instruction): (a).** The user, 2026-11-09: *"just continue with default decisions
till the end of the phase"*. (a) is the agent's recommendation: it files exactly what the user's rule calls a wrong
answer, each command a row, and holds the rest per commit. Re-opening it means (b).

### Q-M1f.17 — What a view that never loads reads from a container

**Raised** 2026-11-09 by [S1f.5.5](p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md)'s T1f.5.5.13. `ein render rules`,
`rule`, `constraints` and `program` parse a program and never load it, so `examples/broken/` draws. A container holds no
source: its program is the resolved, import-flattened text `ein kb save` wrote, and on a program that imports, it holds
the imported forms in place of each `(import …)`.

- **(a) The program it stores**, parsed under its name (Q-M1f.15): on a program that imports nothing each view draws
  what it draws for the `.ein`, and on one that imports, the imports' forms too. Measured over the corpus (T1f.5.5.13's
  cross-check, 2026-11-09): the three views differ on the 106 programs that import and load, and on 51 of them `render
  rules` finds the imports' rules where the `.ein`'s own forms have none, exit 0 for 1; every other command answers a
  container as it answers the `.ein`.
- **(b) Refuse it**, saying the source is wanted, as `solve --dump-answer` refuses a container, exit 2: a command
  refusing what another answers, which the user's rule makes a wrong answer.
- **(c) The recorded source**, re-read where it still exists and hashes the same: another file than the one the
  command was handed.

**Defaulted 2026-11-09 (the user's instruction): (a).** The user, 2026-11-09: *"just continue with default decisions
till the end of the phase"*. (a) is the agent's recommendation: it answers, and what it draws is the program the file
holds. `program_read` holds each view's drawing of a container to the `.ein`'s on a program with no import.

### Q-M1f.18 — Which object the word *model* names on the read-out

**Raised** 2026-11-09 by [S1f.5.16](p1f.5_the_no_owner_items/s1f.5.16_the_read_out_reference.md)'s T1f.5.16.1, as its Q1: M1e's
[Q-M1e.7](../../docs/history/m1e_review_processing/open_questions.md#q-m1e7--the-read-out-prints-the-solution-kb-and-calls-it-a-model)
ruled that a state's model, [`solution_semantics.md` § 3](../../docs/kernel/inference/solution_semantics.md#3-model)'s
*positive part minus the positive initial KB*, is printed by `ein solve` and `--json-summary` *"under that name"*, and
gave that half to this stage. Today a listing's `(model …)` is the region language's claim form, a state's facts over the
candidates' relations (`region.states[].facts` in the summary); § 3's model is what `-P` prints, under *final-state
facts (beyond the program as loaded, (not …) omitted …)*; and `-t`'s `read-out` row says *each listed state's model* of
the first.

- **(a) Document it**: the page names the two objects apart and says which surface prints which; nothing printed moves.
- **(b)** (a), and the heads move: `-P`'s head names § 3's model, and `-t`'s row says *each listed state*.
- **(c)** (b), and each state's model is a summary field, printed under that name.

**Defaulted 2026-11-09 (the user's instruction): (a).** The user, 2026-11-09: *"just continue with default decisions
till the end of the phase"*. (a) is the stage's default and the agent's recommendation: the claim form's name belongs to
the language, whose grammar is final since S1f.5.5's T3, and printing *model* over § 3's object on the screen that holds
`(model …)` blocks of another set would make SE-M1's collision worse. Re-opening it means (b) or (c), each moving `-P`'s
head and the goldens that hold it.

### Q-M1f.19 — A text's line break on the read-out

**Raised** 2026-11-09 by [S1f.5.16](p1f.5_the_no_owner_items/s1f.5.16_the_read_out_reference.md)'s T1f.5.16.1, from an item S1f.5.2 handed it. A
`:goal-text` filled for a class's row, and an obligation's `:why` under an `owing` row, print their characters raw:
a `\n` or `\r` in one starts the text's next line at column 0, outside every row, and a class block counts each
sentence as one of stdout's 100 lines however many it prints (`examples/features/15_unicode_escapes.ein`,
`tests/text/01_unicode.ein`). `ein saturate --dump`, a state file and the `source` row quote every text
(`ein_core::text::quote`), and `--trace` prints no `:goal-text`.

- **(a) Each line at the text's column**: every line break the text holds (`\r\n`, `\n`, `\r`, and Unicode's other
  mandatory breaks) starts a new line at the column the text's first character is printed at, and each is a line of
  the 100. The example's sentence still prints over two lines, as its header says.
- **(b) Escape it**: print a line break as the two characters `\n`, as the quoting surfaces do, so each sentence is one
  line; the example's header, and S1f.5.2's record, would no longer hold.
- **(c) Leave it.**

**Defaulted 2026-11-09 (the user's instruction): (a).** The user, 2026-11-09: *"just continue with default decisions
till the end of the phase"*. (a) is the agent's recommendation: the text is natural language the program wrote for a
reader (Q-M1e2.23), its line break is the writer's, and the read-out's job is to keep it inside its row. Fixed at
T1f.5.16.5; re-opening it means (b).

### Q-M1f.20 — Route 2's licence: *no licence* on one row, `L7 the cube` on another

**Raised** 2026-11-09 by [S1f.5.16](p1f.5_the_no_owner_items/s1f.5.16_the_read_out_reference.md)'s T1f.5.16.1, its census. On route 2, describe's and
`ein oracle`'s `route` row says *no licence* (`2, the cube — no licence: the static check flags the program`, `…:
asked for by name`), where `--json-summary`'s `region.licences` is `["L7 the cube"]` and a mode's `route` row on the
same route prints `2, the cube — L7 the cube` (313 runs of 3 221, `build/s1f.5.16/census.py`, 2026-11-09).
[`solve_interface.md`](../m1e2_correctness/p1e2.1a_the_solution_region/solve_interface.md)'s licence table lists L7 as
the licence that holds always. It misstates no answer.

- **(a) Document it, move nothing**: the page says describe's *no licence* means no licence that lets the route skip a
  node, and that the summary and a mode name the cube's own, L7.
- **(b) Describe's row names L7** as a mode's does, keeping its reason: `2, the cube — L7 the cube: the static check
  flags the program`. It moves the `route` row of every describe and `ein oracle` read-out on route 2, and their goldens.
- **(c) The summary drops L7**, so a licence is only one that skips; a mode's row then names none.

**Defaulted 2026-11-09 (the user's instruction): (a).** The user, 2026-11-09: *"just continue with default decisions
till the end of the phase"*. (a) is the stage's rule: a line that misreports is shown and not fixed by this stage, which
moves no byte of output. Re-opening it means (b) or (c), a P1e2.3-class fix.

### Q-M1f.21 — `region.stopped.layer` on two routes

**Raised** 2026-11-09 by [S1f.5.16](p1f.5_the_no_owner_items/s1f.5.16_the_read_out_reference.md)'s T1f.5.16.1, its census. When a budget stops a
description, `region.stopped.layer` is, on route 1, the commitment size the route stopped before (the row's *route 1
stopped before layer L*); on the state route it is the route's own layer, a state's size plus one
(`examples/saturation/square-bwd/floors.ein`: `layer` 10, where the row says *the states of 9 candidates*). And the
state route's *complete up to states of k candidates*, on `states (k_s)` and `minimal line`, is held by no field:
`region.complete_up_to` is route 1's and `null` there. It misstates no answer.

- **(a) Document it per route, move nothing**: the page defines `stopped.layer` on each route and says the state route's
  *k* is `stopped.layer − 2`.
- **(b) One unit**: the state route writes the size it stopped before, and `complete_up_to` its *k*, moving the summary
  of every partial state-route run (16 corpus runs, the six `square-*` cubes and the `zebra2-minus-15` pair).
- **(c) A field per route.**

**Defaulted 2026-11-09 (the user's instruction): (a).** The user, 2026-11-09: *"just continue with default decisions
till the end of the phase"*. (a) is the stage's rule, as Q-M1f.20's is: no summary field moves here. Re-opening it means
(b), a summary change.

### Q-M1f.22 — A class label of sixteen characters glues to its value

**Raised** 2026-11-09 by [S1f.5.16](p1f.5_the_no_owner_items/s1f.5.16_the_read_out_reference.md)'s T1f.5.16.4, from T3's test, whose row parser had to
read a numbered label that fills its column. The read-out pads a block's label to sixteen characters and prints the
value straight after it, so `class 100 of 277`, sixteen characters, is followed by no space: the uncut listing of
`examples/features/04_open.ein` (`ein solve examples/features/04_open.ein --dump-answer DIR`, `DIR/answer.txt`,
2026-11-09) prints `class 100 of 27728 states` for class 100 of 277, 28 states. Stdout's 100 lines never reach a
hundredth class on the corpus, so only `answer.txt` prints it; no golden holds that file's text. A row label of
eighteen characters or more would glue the same way at the margin; none is that long today (`commitments (k_c)` is
seventeen).

- **(a) Document it, fix it in a later stage**, which files a register row first: the line garbles a class's count of
  states, which the user's rule makes a row before its fix.
- **(b) Fix it here**: one space at least between a label and its value. It moves `answer.txt` on one corpus program
  and no golden, and it needs a register row and a schema field the test can hold first.

**Defaulted 2026-11-09 (the user's instruction): (a).** The user, 2026-11-09: *"just continue with default decisions
till the end of the phase"*. (a) is the stage's rule: a line that misreports is shown and not fixed by this stage,
which moves no byte of output but the item handed to it. Re-opening it means (b).

**Fixed 2026-11-09, (b)**, by [S1f.5.6](p1f.5_the_no_owner_items/s1f.5.6_the_empty_state_decides_the_minimal_line.md),
to which the coordinator handed it the same day: T1f.5.6.5 filed it as W42 under the register's schema `/17` (a line a
run prints, read off stdout or `answer.txt`), with a second row for `-H`'s form, which glues a relation's name of
sixteen characters or more to its count the same way; T1f.5.6.6 put one space at least between a label and its value in
both printers, at every width, and flipped both rows (`31e9c7f7`). No golden held either line, so none moved.

### Q-M1f.23 — What the `route` row names where root decides a flagged program's minimal line

**Raised** 2026-11-09 by [S1f.5.6](p1f.5_the_no_owner_items/s1f.5.6_the_empty_state_decides_the_minimal_line.md)'s
T1f.5.6.2. On a program the static check flags, a minimal question where ∅ is in R is decided by root's node, one
saturation (the user's ruling of 2026-11-07 on S-16, (b)). The rule is the minimal line's definition, ∅ being below
every commitment, and no theorem about the program, so it fits none of
[`solve_interface.md`](../m1e2_correctness/p1e2.1a_the_solution_region/solve_interface.md) § 3.1's licences, which each
hold for a class of program. On a monotone program the same stop has always printed
`the state route, each state at most once — L1 dead is upward-closed · L2 discharged is upward-closed`
(`ein_infer::tree`'s `minimal`), and on a flagged one `--minimal -n 1` stopped at root before the rule, printing
`2, the cube — L7 the cube`. It misstates no answer.

- **(a) The cube and `L7 the cube`**, as `-n 1` printed: the route is the cube in layer order, and its first layer,
  root, decides the line. The read-out's `exhausted` or `decided` row says the line is all of it. Nothing new in the
  vocabulary, and a reader of `L7` alone is not told why one node sufficed.
- **(b) A new licence**, `L8 ∅ is below every commitment` say, on every route, monotone or flagged, so the row names
  the reason: a label added to the vocabulary, `region.licences` on every minimal run where ∅ is in R (the 211
  corpus programs of T1's census), and every golden that prints a minimal run's `route` row.
- **(c) No licence named**: the row says `2, the cube — decided at root`, which a monotone program's run would not say.

**Defaulted 2026-11-09 (the user's instruction): (a).** The user, 2026-11-09: *"just continue with default decisions
till the end of the phase"*. (a) moves no `route` row the rule does not already move, and the read-out's `minimal line`
row on the live page says why the line is decided ([`read_out.md`](../../docs/kernel/inference/read_out.md) § 3).
Re-opening it means (b), which is [Q-M1f.20](#q-m1f20--route-2s-licence-no-licence-on-one-row-l7-the-cube-on-another)'s
neighbour: both ask what a route row's licence list is for.

### Q-M1f.24 — A second surface that glues: another row of W42, or a case of its own?

**Raised** 2026-11-09 by [S1f.5.6](p1f.5_the_no_owner_items/s1f.5.6_the_empty_state_decides_the_minimal_line.md)'s
T1f.5.6.5, which files [Q-M1f.22](#q-m1f22--a-class-label-of-sixteen-characters-glues-to-its-value)'s glue as W42 and
checks every other surface that pads a label to a width. `-H`'s form glues the same way: it pads a relation's name to
sixteen characters and prints its candidate count straight after, so `lives-next-door-to`, eighteen, prints
`lives-next-door-to2 candidates`. It is another printer (`ein-cli`'s `modes.rs`, not `ein_render::read_out`), and no
corpus program declares a name that long.

- **(a) W42's second row**, `W42/h-long-relation`, with a witness written for it: one defect, a padded column with no
  separator, and one rule fixes both printers, so one case closes when both do.
- **(b) A case of its own**, W43: the register's cases are one mechanism each, and these are two printers.

**Defaulted 2026-11-09 (the user's instruction): (a).** The user, 2026-11-09: *"just continue with default decisions
till the end of the phase"*, and the coordinator's brief for the task: *"If another one glues, it is the same row
(another witness) or another, in the same task"*. (a) keeps the fix and its witnesses together; re-opening it moves one
row's id.

### Q-M1f.25 — The state route's members of N it did not saturate: whose preparation takes their cores?

**Raised** 2026-11-09 by [S1f.5.12](p1f.5_the_no_owner_items/s1f.5.12_cores_for_n_only.md)'s T1f.5.12.1. The user's
ruling on P-15, (c), takes N's cores after the walk, *each through the node function against the same snapshot*. The
spec names the members a run met. The state route reads N off its maximal alive states and saturates few of its
members, and `--dump-answer` and `--trace` took every other member's core through `region::Nodes::new`, a second
preparation of the already-saturated root: `W10/second-preparation`'s mechanism, which on `zebra2.ein` gives 19 of
366 members another core, status or rules than the run's own preparation (the stage's probe, 2026-11-09).

- **(a) The walk's snapshot**, after the walk, as every other member of N is taken: one preparation for every core a
  run prints, and each the first preparation's (366 of 366 on `zebra2.ein`, 73 of 73 on `zebra2-hints.ein`). 58
  members' cores, statuses or rules move on three programs, `zebra2`, `zebra2-obligations` and `zebra2-hints`, each an
  explanation, as the user's ruling of 2026-11-08 on Q-M1f.6 has every core (*"core is an explanation, F22 makes it
  canonical"*).
- **(b) The second preparation, as today**: no core moves, the renderers keep their own preparation, and a member of
  one N is taken against two snapshots by whether the walk visited it.

**Defaulted 2026-11-09 (the user's instruction): (a).** The user, 2026-11-09: *"just continue with default decisions
till the end of the phase"*. (a) is the ruling's own clause applied to every member, and it removes the one path that
prepared root twice; re-opening it means (b), and the 58 cores move back.

### Q-M1f.26 — `not-kept` retires, and what a member with no core prints

**Raised** 2026-11-09 by [S1f.5.12](p1f.5_the_no_owner_items/s1f.5.12_cores_for_n_only.md)'s T1f.5.12.1. With every
member of N given its core after the walk, `explore.rs`'s `partial_cube`, the one writer of `CoreStatus::NotKept`, has
nothing to write it for. A member can still carry no core where a library caller describes R without asking for N's
cores (`region::Cores::nogoods` off), which no `ein` command does before it writes `nogoods.json` or `--trace`.

- **(a) The status goes**, its word, its sentence and every reader, and `nogoods.json` and `--trace` refuse a member
  with no core, naming the option the run was not asked: an engine defect, never a line.
- **(b) The status stays** for the library caller, and the renderers print it there.

**Defaulted 2026-11-09 (the user's instruction): (a)**, the spec's own default (*"`not-kept` retires (default)…
If one is left, the status stays for it alone"*), step 1 having found no other writer. Re-opening it means (b), one
enum variant and its two sentences.

### Q-M1f.27 — What program a `.einb`'s answer directory writes

**Raised** 2026-11-09 by [S1f.5.12](p1f.5_the_no_owner_items/s1f.5.12_cores_for_n_only.md)'s T1f.5.12.5, which fixes
W43: `ein solve X.einb --dump-answer DIR` writes the directory where it was refused, and every state file starts with
the program. On a `.ein` that program is the source with its imports inlined and tree-shaken
(`ein_ir::resolve_and_minimize`), every imported declaration nothing reaches dropped. A container stores its program
resolved and import-flattened but not shaken (`ein_einb`'s program section), and keeps no mark of which forms an
import brought, so the shake cannot be re-run on it. On `zebra2-hints.ein` the two differ by one form, the stdlib's
`unknown` macro, which the container's state files keep; on a program that imports nothing they are byte for byte
the same (`einb_cli::the_answer_directory_of_a_container_is_its_sources`, 2026-11-09).

- **(a) The program the container stores**, as `ein render program`, `rules` and `constraints` draw it
  ([Q-M1f.17](#q-m1f17--what-a-view-that-never-loads-reads-from-a-container)): the program the run answered, so each
  state file loads to its state; on a program that imports, a state file's program keeps the unused declarations.
- **(b) Shake the stored program again**, every form treated as the program's own: nothing is dropped, so (b) is (a).
- **(c) Store the shaken program in the container** (`ein kb save`): the files would match the `.ein`'s, at the cost
  of a container whose views no longer draw what Q-M1f.17 ruled they draw, and of a format change.

**Defaulted 2026-11-09 (the user's instruction): (a).** The user, 2026-11-09: *"just continue with default decisions
till the end of the phase"*. (a) moves no container and no view, and the state and every other file are the `.ein`'s.
Re-opening it means (c).

### Q-M1f.28 — Is a not-dead node's core written `[]` a wrong answer?

**Raised** 2026-11-09 by [S1f.5.8](p1f.5_the_no_owner_items/s1f.5.8_no_contradiction_no_core.md)'s T1f.5.8.1, on the
coordinator's brief: a wrong answer, or a printer line that says something false about one, is a register row before
it is fixed. Under the ruling the stage builds
([Q-M1e2.27](../m1e2_correctness/open_questions.md#q-m1e227--what-do--and-null-mean-in-eins-json)), `[]` is the empty
set, *the empty set of facts suffices*, so a node that is not dead written `"core": []` names a core where none
exists. The stage's re-take found it on every JSON surface that writes a node's core: the summary's owing holes,
`region.json`, `nodes.json` and `--events`' `enter` lines.

- **(a) No register row**: a spelling, fixed by the stage with no answer moving. The register holds answers that are
  false under the definitions the user has ruled on: R, N, a dead node's core and rules, claims, exit codes,
  refusals, and a printed line that reads as another answer (W42's count). A node that is not dead has no core to be
  wrong about, so no row could state a `right` for it but its absence, and every object that carries the `[]`
  carries its deadness beside it (`dead: false`, `status`, `kind`), which says there is no contradiction. The
  inventory took D-14 as a spelling (*a surface*, *no answer moves*) and the user ruled it as one (*what do `[]` and
  `null` mean*), and the nearest precedent, M1e2 S1e2.3.9's empty core on a member of N a run had not taken (N6,
  *reads as refuted by nothing*), moved no row.
- **(b) A register row**, W44, filed `open` before the fix and fixed by it, its `now` an owing hole's `[]` read off
  the summary; the register would need a field for a hole's core, schema `/20`.

**Defaulted 2026-11-09 (the user's instruction): (a).** The user, 2026-11-09: *"just continue with default decisions
till the end of the phase"*. (a) is the stage's own statement (*"No answer moves, and no register row"*), which the
user approved with the plan. Re-opening it means (b), one row filed `fixed` with the stage's commit.

### Q-M1f.29 — A dead root's core under `--count`

**Raised** 2026-11-09 by [S1f.5.8](p1f.5_the_no_owner_items/s1f.5.8_no_contradiction_no_core.md)'s T1f.5.8.1. On a
program whose root is dead, `ein solve FILE --count` writes `region.root: "dead"` beside `region.core: null` and
`region.refuted_by: null`: 39 programs of the stage's sweep (2026-11-09). A bare `ein solve` writes root's core and
rules there, and `--count`, which reads no state, prints neither on stdout; a mode writes `region.root: null` and no
core either. Root is dead, so its core exists.

- **(a) `null`, documented**: the summary is stdout's facts, and `null` is where a key does not apply; the ruling reads
  `null` as *an object that does not exist or is not known*, and a count does not state root's core.
  [`read_out.md` § 11](../../docs/kernel/inference/read_out.md#11---json-summary-the-answer-blocks) says so, from
  T1f.5.8.2 on.
- **(b) Root's core under every question** whose summary says root is dead: `--count`'s summary, and its stdout's
  `unsat core` and `refuted by` rows, where a count prints no listing today.

**Defaulted 2026-11-09 (the user's instruction): (a).** It moves nothing, and a count's question is *how many*.
Re-opening it means (b), which moves `--count`'s stdout on those 39 programs.

### Q-M1f.30 — A form that holds a variable, as a guard's argument

**Raised** 2026-11-09 by [S1f.5.7](p1f.5_the_no_owner_items/s1f.5.7_eq_and_neq_are_relations.md)'s T1f.5.7.1. The user
ruled a nested fact answered as a guard's argument (`W24/form-in-guard`, *"yes, keep that default"*): `(eq ?x (f a))`
compares `?x` with the value `(f a)`, as `(p (f a))` holds it. A form with a variable in it, `(eq ?x (f ?y))`, is a
pattern, and the ruling's witness writes none. `eq` binds a variable nothing else binds to the other side's value, so
with `?x` free and `?y` bound it would bind `?x` to `(f v)`, a value a match builds and no store may hold: the matcher
reads the fact store and cannot add to it, and a register holds only a value the store holds.

- **(a) Refused at load**, in a rule and in a query's goal, one line per form in Ein's spelling: *… and the form (f ?y)
  holds a variable — a guard compares two values, each a variable, a name, or a fact that holds no variable*. The
  same question written as a premise, `(p (f ?y))`, is answered.
- **(b) Answered where the store holds the value**: a form whose variables are bound is looked up, and `eq` binds a
  free side to it only when found. A row would go missing where it is not, in silence.
- **(c) Answered in full**, the matcher allowed to intern a fact it builds: a change to the fact store's lending rule
  (`Terms`, P1a.7's *nothing interns during a search*).

**Defaulted 2026-11-09 (the user's instruction): (a).** The user, 2026-11-09: *"just continue with default decisions
till the end of the phase"*. (a) keeps Q-M1e2.18 (i)(a)'s refusal for the one shape the 2026-11-07 ruling did not
answer, and moves no program: no corpus file writes a form in a guard. (b) answers wrongly where it cannot answer, and
(c) is a change to the interner's contract outside the stage. `cli_semantics` and `from_ir`'s unit tests hold (a).

### Q-M1f.31 — `(not (eq x y))`: a stored negation, or `(neq x y)`?

**Raised** 2026-11-09 by S1f.5.7's T1f.5.7.1. The grammar takes `(not (eq a b))` wherever a stored negation goes: a
top-level fact, a premise, a conclusion, a goal and a claim. Until S1f.5.7 each was the relation `not` over the fact
`(eq a b)`: a premise matched a stored `(not (eq …))` and nothing else, so it never fired, and a fact or a conclusion
was stored, `(not (eq a a))` too, and killed nothing (W51). `(not (neq …))` is a parse error, `neq` being no name.

- **(a) It is `(neq x y)`**, in every place: `eq`'s facts are computed, and so is the negation of one. A premise and a
  goal are the `neq` guard, a conclusion is checked as `neq`'s (G-21), a written one is a fact of `neq` (Q-M1f.33),
  and an hrule's is no hypothesis (Q-M1f.32).
- **(b) A stored negation**, as until S1f.5.7, with the pair check extended so a stored `(not (eq a a))` is a
  contradiction; a premise still matches only stored ones.
- **(c) Refused at load**, with a line naming `(neq …)`.

**Defaulted 2026-11-09 (the user's instruction): (a).** It is the ruling read where it reads: `eq` and `neq` are
relations of equality and non-equality wherever a relation can be written, and `(not P)` over a relation whose truth
is computed is computed. (b) keeps one relation with two readings, the defect G-21 named, and (c) is a restriction
with no logical reason, which the user asked for none of (2026-11-07, G-21). W51's three rows hold (a).

### Q-M1f.32 — An hrule's `eq` or `neq` instance

**Raised** 2026-11-09 by S1f.5.7's T1f.5.7.1. An hrule's `:assert` takes an `eq` or `neq` conclusion, and the hrule
rung proposed every instance as a hypothesis, which a commitment stored, `(eq a b)` among them (W53).

- **(a) No hypothesis**: a commitment states facts a state stores, and a state stores neither relation, so the rung
  proposes no instance of either.
- **(b) A hypothesis checked as a conclusion**: a true one a commitment that adds nothing, a false one a candidate dead
  alone, which joins N.
- **(c) Refused at load.**

**Defaulted 2026-11-09 (the user's instruction): (a).** A commitment that adds nothing, or only `(false)`, is no
hypothesis about the program's facts; (b) would put a state in R twice over and a member in N that no fact explains,
and (c) is a restriction the user asked for none of. `W53/hrule-eq` holds (a).

### Q-M1f.33 — A written fact of `eq`

**Raised** 2026-11-09 by S1f.5.7's T1f.5.7.1. A top-level fact's relation may be `eq` (`ein.y`'s `FactHead`), so `(eq
a w)` loaded, and was stored as an ordinary fact, which no guard reads (W50). `(neq …)` is no fact the grammar takes.

- **(a) A conclusion with no premise**: a true one is not stored, and a false one is `(false)`, written where it
  stands, so root is a contradiction, `refuted by` `(given)`, as a rule's false conclusion is (G-21).
- **(b) Refused at load.**
- **(c) Stored, as until S1f.5.7.**

**Defaulted 2026-11-09 (the user's instruction): (a).** A fact is a rule with no premise, and G-21 says what a rule's
false conclusion of `eq` is; (b) is a restriction the user asked for none of, and (c) is the defect, a fact changing a
relation whose facts are fixed. The grammar's asymmetry, `(eq …)` a fact and `(neq …)` none, stays the grammar's: no
program writes either, and S1f.5.5's grammar is the phase's final one. `W50/fact-eq-false` and `W51/not-eq-fact` hold
(a).

### Q-M1f.34 — Where a guard's free variable is bound, beside an `(absent …)`

**Raised** 2026-11-09 by S1f.5.7's T1f.5.7.1. A premise means the same in any order of its conjuncts (the user's ruling
of 2026-11-07), and an `(absent …)` reads the variables the premises before it bind (S1.21.8): `(and (absent (q ?x))
(p ?x))` asks whether there is no `q` at all. A guard whose variable nothing in its conjunction binds, `(neq ?x a)`
with no `(p ?x)`, binds it, ranging over the program's names, so where it binds decides what an `(absent …)` beside it
reads.

- **(a) Where it is written**, as a relation premise binds: `(and (neq ?x a) (absent (q ?x)))` is *the names other than
  `a` with no `q`*, and `(and (absent (q ?x)) (neq ?x a))` *the names other than `a`, if there is no `q` at all*. A
  guard whose variable a relation premise of the conjunction binds waits for it, whatever the order.
- **(b) Before every `(absent …)`**, so the two read alike: the guard moved, as (a) moves it after a binder.

**Defaulted 2026-11-09 (the user's instruction): (a).** It reads a binding guard as the relation it is, a premise that
binds, so the order rule an `(absent …)` already has is the only one; (b) would make a guard bind past an `(absent …)`
that a relation premise in its place would not. `ein-infer/tests/rule_semantics.rs`'
`a_binding_guard_binds_where_it_is_written` (T1f.5.7.3) holds both orders.

### Q-M1f.35 — A goal's `(absent …)`: whose row is it?

**Raised** 2026-11-09 by [S1f.5.7](p1f.5_the_no_owner_items/s1f.5.7_eq_and_neq_are_relations.md)'s T1f.5.7.5,
outside its case. The stage's test of a goal's empty row wrote `(query :goal (absent (q ?x)))` and found the goal
answered with a `q` in the state. A goal is compiled as a rule's `:match` is, its top-level `(absent …)` lifted out of
the plan for the closure/world boundary to judge, and `verdict::goal_bindings`, which reads the goal's rows, never
judges it: `(and (p ?y) (absent (q ?y)))` over `(p a) (p b) (q b)` answers `?y = b`. And a claim cannot list the
class, `(absent (q a))` being no relation's fact to the loader, as `(neq w a)` was not until T1f.5.7.2. No corpus goal
writes an `(absent …)`, `forall` or `unknown`. The register's `W56/goal-absent` holds it.

- **(a) S1f.5.7 fixes it**, in a task of its own after the pages (T1f.5.7.6): `goal_bindings` judges each lifted guard
  against the state, as the boundary does a rule's, and a query's claim may list a goal's `(absent …)` instance, as it
  lists a guard's. The stage rewrote the goal's evaluation, and the fix moves no corpus program.
- **(b) The row is filed `open` and deferred** to a stage that owns goals, which P1f.5 has none of.

**Defaulted 2026-11-09 (the user's instruction): (a).** It is the goal evaluation the stage changed, the class the
stage's own guards made claimable, and a deferred row would have no owner to wait on. `W56/goal-absent` is filed `open`
by T1f.5.7.5 and fixed by T1f.5.7.6.

### Q-M1f.36 — What the refusal replaces: which of it is a register row?

**Raised** 2026-11-09 by [S1f.5.10](p1f.5_the_no_owner_items/s1f.5.10_a_conclusion_variable_must_be_bound.md)'s
T1f.5.10.1. The stage refuses at load a rule or an hrule whose conclusion writes a variable nothing binds (the user's
ruling of 2026-11-07 on F13). Its step 1 (`build/s1f.5.10/shapes`, `2eb483e0`'s release and debug builds) found four
ways the engine answers such a program today: root fires the rule, and every command stops at the firing, exit 1,
*unbound var ?y in :assert — bindings: {'x': 'a'}* (`ein test`: 2, an `ERROR`); only a commitment fires it, and `ein
saturate` and `ein solve --minimal` answer, exit 0, where describe and `ein oracle` stop; an hrule's template builds no
fact, and proposes nothing, in silence; and a rule that never fires, an empty `:match` (F12) among them, loads and
answers. The stage file says its fixture, `examples/ein-bugs/unbound-assert-var.ein`, *needs no register row, since it is
a regression fixture, not a wrong answer*.

- **(a) A row per way, each `right` the refusal**: W57's four, `assert-var-stops`, `assert-var-under-commitment`,
  `hrule-assert-var` and `assert-var-never-fires`, `open` and S1f.5.10's, fixed by its T1f.5.10.2. The fixture stays no
  row: its program is the first row's shape, reached through an `(or …)` arm.
- **(b) Only the three that answer**: no exit code moves on a stop, 1 → 1 and `ein test`'s 2 → 2, only its line.
- **(c) No row**, as the stage file says, the fixture and the stage's tests holding the refusal.

**Defaulted 2026-11-09 (the user's instruction): (a).** The register compares a run that stops (`now.error`) and one
refused at load (`now.refused`) as two answers (`ein_corpus::register::Now::against`: *an error is no answer at all, so
it is never the right one*), and W18 filed its stops as rows; the second way is the user's rule that no two commands may
disagree; and S1f.5.7 filed every move it made. Re-opening it means (b).

### Q-M1f.37 — `FireError::UnboundVar`'s last path: W33's cause

**Raised** 2026-11-09 by S1f.5.10's T1f.5.10.1, outside its case. Once the loader checks a conclusion's variables, one
shape still reaches `ein_infer::firing::FireError::UnboundVar`: a value parameter its activator binds to a nested fact,
read only by the `:assert`. The compiler seeds a parameter from its activator and skips a nested fact (*a nested Fact
does not bind*), W33's cause, so the parameter reaches the conclusion unbound and the run stops, exit 1. The loader
cannot refuse it, since a parameter is bound by its activator, and the user ruled on 2026-11-08 that a nested fact binds
a value parameter as any other value (S1f.5.5's census, *"Q3 - as recommended"*), which is
[S1f.5.18](p1f.5_the_no_owner_items/s1f.5.18_an_activator_binds_a_nested_fact.md)'s, the next stage on lane 2. The stage
file's acceptance says *`FireError::UnboundVar` is gone*.

- **(a) It stays until S1f.5.18**, its doc naming the one path, and S1f.5.18 retires it with W33's fix. The shape is a
  wrong answer, a program the ruling answers that stops, so it is a row, `W58/parameter-nested-assert`, `open` and
  owned by S1f.5.18, and that acceptance item moves to S1f.5.18.
- **(b) S1f.5.10 binds the parameter** itself, which is W33's fix and S1f.5.18's step 1, sweep and review.
- **(c) A loader refusal** of a parameter read only by a conclusion, which would refuse a program the ruling answers.

**Defaulted 2026-11-09 (the user's instruction): (a).** It keeps each fix with the stage that owns its cause, in the order
the coordinator set (S1f.5.10, then S1f.5.18), and loses nothing: the stop's line is unchanged until S1f.5.18, and W58
holds it. Re-opening it means (b).

### Q-M1f.38 — The refusal's line: its separator, and a disjunction

**Raised** 2026-11-09 by S1f.5.10's T1f.5.10.1. The stage file's line reads *rule 'r': its :assert writes ?y, which no
premise or parameter binds; to conclude it of every object of a type, bind it with a premise such as (is-a ?y T), or of
every name of the program with (eq ?y ?y)*. The loader joins several lines with `; ` on one stderr line
(`KbLoadError`), and its other lines separate the finding from the advice with ` — `. And where a top-level `(or …)` is
one way per disjunct, some disjuncts may bind the variable and others not (`unbound-assert-var.ein`).

- **(a) ` — ` for the stage file's `;`**, and where some disjuncts bind the variable, the line names those that do not:
  *rule 'fire-3': its :assert writes ?v0, which no premise or parameter binds where its :match holds through (r1 ?v1
  ?v3) — to conclude it …*.
- **(b) The stage file's text verbatim**, its `;` read as a line's end by anyone splitting the loader's report.

**Defaulted 2026-11-09 (the user's instruction): (a).** The line is the stage file's in every word but the separator, as
every loader line is one finding, ` — `, and what to write; and a reader of `unbound-assert-var.ein`'s line can see which
arm to fix.

### Q-M1f.39 — The one corpus program the check refuses

**Raised** 2026-11-09 by S1f.5.10's T1f.5.10.1. Its cross-check of every program of the five roots through `2eb483e0`'s
build and the prototype's (`build/s1f.5.10/xcheck`, 533 files) moved two: the stage's fixture, and
`examples/syntax/rule-nary-and-or.ein`, one of the eighteen node-kind fixtures, a parametrised rule with a top-level
`(or …)` over disjuncts of two arities whose `:assert`, `(quad ?d ?c ?b ?a)`, writes `?c` and `?d`, which the second
disjunct, `(?rel ?a ?b)`, leaves unbound. No activator is written, so the rule never fires, and the program answers R
{∅} on every command; the check refuses it.

- **(a) Rewritten**: its `:assert` concludes `(quad ?b ?a ?b ?a)`, which both disjuncts bind. It keeps its node kinds and
  its answer, R {∅}, and its digests move.
- **(b) Moved to `examples/broken/load/`**, a load-negative fixture whose `.expected` is the refusal.

**Defaulted 2026-11-09 (the user's instruction): (a).** A node-kind fixture exists for the shapes it draws, which a
rewrite keeps, as S1f.5.5's swap rewrote four `syntax/` fixtures (T1f.5.5.4); the refusal's shapes are W57's witnesses
and the stage's tests. Re-opening it means (b).

### Q-M1f.40 — Where the fuzz property lives, and what it reads

**Raised** 2026-11-09 by S1f.5.10's T1f.5.10.1. The stage file says *`fuzz_properties` gains the property that no firing
meets an unbound variable*. `fuzz_properties` is `ein-ir`'s, the frontend fuzzer the nightly runs a million times a seed,
and a firing is `ein-infer`'s.

- **(a) In `fuzz_properties`, over the plans**: `ein-ir` takes `ein-infer` as a dev-dependency, as `ein-infer` takes
  `ein-render`, and for every generated program the loader accepts, every plan the compiler builds for its rules and
  hrules reads, in each conclusion, only registers each disjunct binds, a parameter excepted until S1f.5.18 (W58). It
  is every firing at once, an hrule's and a rule's that never fires among them, and it holds the loader's reading of
  what binds to the compiler's.
- **(b) In `fuzz_properties`, by firing**: each accepted program saturated at root, and no stop; it reaches no hrule and
  no rule that root does not fire.
- **(c) In `utils/fuzz_engine.py`**, the engine fuzzer, as a line no run prints; not in the gate or the nightly.

**Defaulted 2026-11-09 (the user's instruction): (a).** It is the stage file's place and the strongest reading of its
property; the dev-dependency adds no edge to any shipped crate.

### Q-M1f.41 — What *can hold a fact* means, for G-14's derived activator

**Raised** 2026-11-09 by [S1f.5.18](p1f.5_the_no_owner_items/s1f.5.18_an_activator_binds_a_nested_fact.md)'s T1f.5.18.1. The user's ruling on
[Q-M1f.1](#q-m1f1--g-14-is-a-bad-conclusion-that-never-fires-refused) (b) refuses at load a program in which some rule
could derive an activator binding a relation parameter to a fact, `g14/derived-activator-never.ein` among them, although
no run derives one. The stage file sketches the check: *a position whose values are relation names is one that the
program uses as a relation (a relation parameter, a `(?R …)` head, or a relation signature's name), and everything else
can hold a fact*, and it stops if the check refuses a corpus program that answers today, or if no reading both refuses
the two `g14` programs and keeps the corpus. The census (`build/s1f.5.18/census/`, a prototype loader over the 538
programs of the five roots, `find examples tests stdlib corpus plans -name '*.ein'`, 2026-11-09) read three ways:

- **(a) The sketch, as written.** Refuses the two `g14` programs and **32 corpus programs that answer today**:
  `zebra.ein`, the six `zebra2` programs, `domain_elim`'s three, `features/09`, `stdlib/algebra`, `bijection`, `slots`
  and `typing` loaded alone, and sixteen of `tests/stdlib/`. Each reaches a relation parameter through a fan-out rule
  that reads a marker relation no rule and no signature names, `(bijective ?R)`, `(bijection-hierarchy ?isa)`,
  `(slot-partition ?R ?isa ?sub …)`, `(adjacent-via ?S ?R1 ?V1 …)`, and passes the name on, as
  `bijective-properties` passes `?R` to `functional`.
- **(b) Typed by declaration, the rest inferred.** An argument of a **declared** relation holds any value of its type, a
  fact among them (`(relation item Node)`, and `(box (a b))` under `(relation box Node)`), and so do an argument inside a
  fact pattern, `(p (g ?x))`, and an argument of a relation a parameter names, `(?S ?x)`; every argument of `(relation
  …)` holds a name. An argument of an **undeclared** relation, a marker such as `(bijective color-loc)`, and a rule's
  parameter hold what the program does with them: where a variable bound there reaches a relation parameter, the
  position holds relation names too, inferred to a fixpoint, and a fact or a conclusion that writes a fact there is
  refused, as W24 refuses one written as a rule's activator. Refuses the two `g14` programs and no corpus program; it
  infers 20 marker positions over 32 programs, every fan-out the sketch refuses, and checks 851 derived activators'
  arguments at a relation position.
- **(c) The flow from the facts**: a position holds a fact only where a fact or a conclusion puts one. Accepts
  `derived-activator-never.ein`, against the ruling.

**Defaulted 2026-11-09 (the user's instruction): (b).** It is the one reading of the three that refuses both `g14`
programs and keeps the corpus, the stage file's condition for going on, and it is the sketch with a marker's argument
read by what the program does with it rather than as a value. `item`'s declaration is what refuses
`derived-activator-never.ein`: written without `(relation item Node)`, that program would load, and
`derived-activator.ein` would be refused at its fact `(item (a b))`. A program that declares its marker, `(relation
mark T)`, is refused for it: one test program in Rust did, and dropped the declaration (S1f.5.18's T1f.5.18.3).
Re-opening it means (a), with the 32 programs rewritten, or (c), against Q-M1f.1 (b).

### Q-M1f.42 — A conclusion whose relation is a parameter

**Raised** 2026-11-09 by [S1f.5.18](p1f.5_the_no_owner_items/s1f.5.18_an_activator_binds_a_nested_fact.md)'s T1f.5.18.1. The check of Q-M1f.41 reads a conclusion's relation as written. `(?T ?x)`
writes into whichever relation its activator names, which no load reads off the conclusion: `(rule copy (?S ?T) :match
(?S ?x) :assert (?T ?x))` with `(copy src mirror)` and `(src (q A B))` derives `(mirror (q A B))`, and the run stops where
`mirror`'s plan is compiled (`build/s1f.5.18/shapes/varhead-copy.ein`, `8249e872`).

- **(a) Followed into every rule of the conclusion's arity whose name the program writes as an argument**, in a fact at
  any depth, a rule's clause or the `(solve …)` form: a rule's name reaches a parameter only from a place the program
  writes it, and H proposes no rule's name (`varhead-copy-names.ein`: H is ten candidates over the objects `A`, `B` and
  `a`). Refuses no corpus program: no conclusion written `(?T …)` reaches a rule whose name the program writes as an
  argument.
- **(b) Not followed**: the compiler's refusal stays where such a plan is compiled, a stop at run where Q-M1f.1 (b) says
  a refusal at load, which is a register row with no owner.
- **(c) Followed through the relation names each parameter can hold**, a flow over the program's names; the precise
  form of (a).

**Defaulted 2026-11-09 (the user's instruction): (a).** It is sound for every name a program writes, and moves nothing
(c) would keep.

### Q-M1f.43 — `Plan::activator_args`, once a nested fact binds

**Raised** 2026-11-09 by [S1f.5.18](p1f.5_the_no_owner_items/s1f.5.18_an_activator_binds_a_nested_fact.md)'s T1f.5.18.1. With W33's fix a value parameter binds to a nested fact, so `(note edge (src
X))` and `(note edge (src Y))` compile two plans that bind two values; the compile key and the binding key already tell
them apart (M1e2 S1e2.2.7, S1e2.2.16). `Plan::activator_args` keeps an activator's symbol arguments only (Q-M1a.8's
asymmetry), and it is more than a label: the obligations rung's `InstanceKey` (`oblgen`), the region's owed-instance
lookup (`region.rs`), `detector`'s `RootPlans` and its activator match, and `naf_deps`' key each read it as a plan's
identity, beside a match's bindings, which hold no seeded register. Two plans the seed tells apart would share it.

- **(a) Every argument, as values, the compile key's list**: each identity follows its plan, and the `enqueue`, `fire`
  and `owe` events and the static checks' `[activator …]` name the nested fact, `"activator": ["(f a)"]` where they
  printed `[]`.
- **(b) Symbols only, and each identity keyed on the seed beside it.**
- **(c) Symbols only, unchanged.**

**Defaulted 2026-11-09 (the user's instruction): (a).** One list for a plan's activator everywhere, which closes the
asymmetry Q-M1a.8 named, and moves the narration and the labels of a program whose activator holds a nested fact only;
(c) leaves five identities that collapse two plans the stage makes different.

### Q-M1f.44 — What the load-time check replaces: which of it is a register row?

**Raised** 2026-11-09 by [S1f.5.18](p1f.5_the_no_owner_items/s1f.5.18_an_activator_binds_a_nested_fact.md)'s T1f.5.18.1. Q-M1f.1 (b)'s check refuses at load two ways the engine answers today: a
derived activator binds a relation parameter to a fact, and the run stops where its plan is compiled
(`g14/derived-activator.ein`); or no run derives it, and the program answers (`g14/derived-activator-never.ein`, 64
states).

- **(a) A row per way, each `right` the refusal**: W59's two, `derived-activator` and `derived-activator-never`, `open`
  and S1f.5.18's, witnesses the two `g14` probes with `:why`s added, fixed by T1f.5.18.3; the stage file's refusal
  fixtures under `examples/broken/load/` banked by T1f.5.18.3 as well.
- **(b) No row**, the fixtures and the stage's tests holding the refusal.

**Defaulted 2026-11-09 (the user's instruction): (a).** It is [Q-M1f.36](#q-m1f36--what-the-refusal-replaces-which-of-it-is-a-register-row)'s
reading: a stop and a refusal are two answers, and so are an answer and a refusal.

### Q-M1f.45 — The `(and)` refusal's line, and `(or)`

**Raised** 2026-11-09 by [S1f.5.9](p1f.5_the_no_owner_items/s1f.5.9_an_empty_match_fires.md)'s T1f.5.9.1. The user ruled on F12 that `(and)` stays refused, `and` taking
arguments and `()` being the one spelling of the empty conjunction, and the stage file asks for a line that says so,
naming the rule and the clause. Since S1f.5.5's swap the grammar refuses `(and)` (`ein.y`'s `Premises` and
`Conclusions` take one or more), before the loader reads the rule, with *unexpected `)`* and its repairs; `(or)` is
refused the same way, and the ruling says nothing of it.

- **(a) A note on the parse error** (`ein_ir::grammar`'s notes, as S1f.5.1's `_` and S1f.5.2's string carry one),
  naming the rule or the form and the clause, `rule 'r''s :match writes (and), the empty conjunction, whose one
  spelling is () — and takes one premise or more`; `(or)` gets one too, `or takes one disjunct or more`, which reads
  no meaning into the empty disjunction.
- **(b) The same for `(and)` alone**, `(or)` keeping the bare parse error.
- **(c) A loader line**, the grammar accepting `(and)` again so the loader can name the rule.

**Defaulted 2026-11-09 (the user's instruction): (a).** The grammar is the phase's final one and refuses the shape
already, so the line is the parse error's, and a note is how that error says what a ruling says (S1f.5.1, S1f.5.2);
(b) leaves the twin shape unexplained; (c) undoes a refusal the grammar holds.

### Q-M1f.46 — A relation parameter bound to `eq` at another arity

**Raised** 2026-11-09 by [S1f.5.9](p1f.5_the_no_owner_items/s1f.5.9_an_empty_match_fires.md)'s T1f.5.9.1, from the shape S1f.5.18 found and left unfiled: `(rule r (?R)
:match (?R ?x) …)` with `(r eq)` loads, and the run stops where the activator's plan is compiled, M1e S1e.2.1's arity
check (`eq` is the relation of equality between two values, S1f.5.7). The stage's probes (`build/s1f.5.9/case/`,
`ee3da657`) find four ways the engine answers it: a written activator stops every command at root; an activator a
rule derives only under a commitment stops `ein solve` where `ein saturate` answers; an hrule's `(solve :hrules (h
(eq)))` activator stops `ein solve` where `ein saturate` answers; and a conclusion `(?R ?x)` stores `(eq a)`. The same
pattern through a declared relation of another arity, `(r q)` with `q` binary, matches nothing and loads, as `(q ?x)`
written directly does, matching being arity-coupled.

- **(a) Refused at load wherever a parameter can hold `eq`**, read as S1f.5.18's check reads relation names
  (`refuse_derived_relation_activators`): a position that holds relation names can hold `eq` where a fact, a
  conclusion or a `(solve :hrules …)` activator writes it there, where a conclusion writes a variable its `:match`
  binds only at positions that can, or only with an `eq` or `neq` guard while `eq` is a name the program writes, to a
  fixpoint; each premise or conclusion such a parameter heads whose arity is not 2 is one line. A row per way the
  engine answers today, W60's four (Q-M1f.36's reading); the declared relation of another arity no row.
- **(b) Refused at load where the activator is written down** (`(r eq)`, `(solve :hrules (h (eq)))`), and a derived
  activator left `open`, owned by the P1f.5 close, a stop at run.
- **(c) Answered**: `(eq ?x)` matches nothing and concludes nothing.

**Defaulted 2026-11-09 (the user's instruction): (a).** S1f.5.18's ruling (Q-M1f.1 (b)) refuses a derived activator
a static check can see, whether or not a run derives it, and this check is that one's, over the same flow; (b) keeps a
pair of commands disagreeing on the commitment and hrule witnesses, which the user's *one tool, consistent answers*
forbids; and (c) gives `eq` a reading at one argument that the grammar refuses where `(eq ?x)` is written.

### Q-M1f.47 — A written fact of `eq` at another arity

**Raised** 2026-11-09 by [S1f.5.9](p1f.5_the_no_owner_items/s1f.5.9_an_empty_match_fires.md)'s T1f.5.9.1, beside Q-M1f.46. A top-level fact may be headed `eq` (`ein.y`'s
`FactHead`), and Q-M1f.33 reads a written fact of `eq`, or of `(not (eq …))`, as a conclusion with no premise, checked
and never stored; but the check reads two arguments only, so `(eq a)`, `(eq)`, `(eq a a a)` and `(not (eq a))` load and
are stored as ordinary facts (`build/s1f.5.9/case/eqfact/`, `ee3da657`).

- **(a) Refused at load**, one line per fact, beside Q-M1f.33's check (`from_ir::ingest_fact`), as the grammar refuses
  `(eq ?x)` as a premise and a conclusion; a nested `(box (eq a))` stays a value, as `(box (eq a b))` is.
- **(b) Refused at the parse**, `FactHead` taking `eq` with two arguments only.
- **(c) Stored, as today**, left `open` and owned by the P1f.5 close.

**Defaulted 2026-11-09 (the user's instruction): (a).** It is W60's conclusion row written as a fact, and Q-M1f.33 made
the loader the place that reads a written `eq` fact; (b) changes the grammar's fact production and its negation's for a
shape no program writes, which the loader's line reaches as well; (c) leaves a fact of a relation whose facts are
computed stored. One row, `W61/fact-eq-arity`, this stage's.

### Q-M1f.48 — How a text surface writes the empty core

**Raised** 2026-11-09 by [S1f.5.9](p1f.5_the_no_owner_items/s1f.5.9_an_empty_match_fires.md)'s T1f.5.9.2, on the
coordinator's note of that day (S1f.5.8 landed on `master` as `c4708638`). Once `:match ()` fires, `(rule r () :match ()
:assert (false))` makes root dead with the core `[]`, the empty set, the first in Ein (Q-M1e2.27: `[]` is a core that
exists and holds no fact). Every JSON surface writes it `[]` beside `core_status: "within-budget"` and `refuted_by:
["r"]`. The text surfaces were written before a core could be empty, and print a core only where it holds a fact:
`ein solve`'s read-out prints one `unsat core` row per fact, so none; `--trace`'s N prints *Core: not kept by this
route.*, which is false, the core having been taken; and `region.md`'s and `nodes.md`'s *why* columns and `ein render
region`'s tooltips and table print nothing, which reads as *no core*.

- **(a) `∅` on every text surface, where a dead node's core was taken and holds no fact**, the read-out's spelling of
  the empty set already (*∅, root itself*): the read-out's `unsat core ∅, the empty set: the refutation rests on no
  fact`, and `--trace`'s `Core: ∅, …` with the same words; `core ∅` in `nodes.md`, `core ∅ by r` in `region.md` and
  `ein render region`'s table, `core: ∅` in its tooltips, and a minimal hole's `dead: ∅` (`explain::core_text`,
  `EMPTY_CORE`). A core not taken prints as it did.
- **(b) The word `none`**, or *no fact*, in the same places.
- **(c) The read-out and the trace alone**, the dumps and the views left to print nothing.

**Defaulted 2026-11-09 (the user's instruction): (a).** `∅` is the read-out's word for the empty set, so the text and
the JSON's `[]` say one thing; `none` reads as *no core*, the very confusion Q-M1e2.27 separated `[]` from `null` to
avoid; and (c) leaves views that read as *no core* where a core was taken. The trace's false sentence goes in the
commit that makes the empty core reachable, so no tree prints it.

### Q-M1f.50 — Where the `heavy_dump` line goes, and which entries carry the flag

**Raised** 2026-11-09 by [S1f.5.13](p1f.5_the_no_owner_items/s1f.5.13_large_dumps_to_the_slow_tier.md)'s
T1f.5.13.1. The spec sets the line *in the gap of the distribution*, and the distribution of that day
(`EIN_ROUND_TRIP_LIST=1 EIN_CORPUS_SLOW=1 cargo test -p ein-cli --release --test answer_round_trip -- --nocapture`,
`7bc6ea72`) has two gaps: 23 390 to 64 514 state files, and 169 to 4 096. Above the first are the four entries P-16
named and `features/01_not_and_absent`, which is `slow`. Between the two are the six `slow` `square-fwd` and
`square-bwd` at 23 390, `type-exclusivity/pets` at 11 339 and two `syntax/` programs at 4 096. A probe of the per-commit
run (debug, under the second lane's load) timed the four at 49 to 58 s each, `pets` at 8.1 s and the two `syntax/`
programs at 1.3 and 1.7 s.

- **(a) 40 000, in the upper gap.** Exactly the four leave the per-commit tier, as the ruling named them, and the
  test's slowest shard falls from 62.2 s to 9.3 s, `pets` the longest program left. The flag is held on every entry
  the round trip dumps, the slow tier's included, so `features/01_not_and_absent`, `slow` and over the line, carries it
  too: five entries.
- **(b) A line in the lower gap**, 1 000 say. `pets` and the two `syntax/` programs leave too, and the six `square-*`
  carry the flag beside `slow`: 14 entries, the slowest shard 5.5 s, and three more programs round-tripped only nightly.
- **(c) The flag held per commit alone**, the slow tier not checking an unflagged entry. `features/01` carries no
  flag, and a `slow` entry that lost `slow` would rejoin the per-commit round trip with a 64 655-file dump that only
  that run would notice.

**Defaulted 2026-11-09 (the user's instruction): (a).** The user, 2026-11-09: *"just continue with default decisions
till the end of the phase"*. (a) moves what the ruling named and nothing else, and every program under 40 000 state
files is still round-tripped per commit. Re-opening it means (b): one constant in `ein-corpus`'s `manifest.rs`, nine
more flags in `corpus/corpus.toml`, and the set `the_heavy_dump_set_is_exactly_these_five` pins restated.

### Q-M1f.51 — Three more tests whose slow half no CI tier runs

**Raised** 2026-11-09 by [S1f.5.13](p1f.5_the_no_owner_items/s1f.5.13_large_dumps_to_the_slow_tier.md)'s
T1f.5.13.3, outside its brief. The stage's case was that `answer_round_trip`'s slow half ran in no CI tier, and the
same is true of others. Eight test targets read `EIN_CORPUS_SLOW` (`grep -rln EIN_CORPUS_SLOW ein.rs/crates`,
2026-11-09, less the three files that only name it): `corpus_cli`, `lever_invariance` and `corpus_cores` were nightly
steps before the stage, and `answer_round_trip` is one since. Of the other four, `read_out_examples`' slow branch adds
the page's blocks at `--region-budget 33554432`, and today's page has none. The other three have a slow half that only
`./run_tests.sh --slow`, run by hand, reaches:

| target | what `EIN_CORPUS_SLOW` adds | release, without · with the flag |
|---|---|---:|
| `ein-cli`'s `route_invariance` | the zebra family and the six `square-*`, at `--region-budget` 2²⁵ | 24.8 s · 857.3 s |
| `ein-infer`'s `region` | `the_state_route_is_route_1_on_zebra2_minus_15_through_layer_6` | 13.0 s · 37.6 s |
| `ein-infer`'s `obligation_rung` | `the_theory_finds_what_the_hrule_finds` on `zebra2-minus-15`'s 32 states | 3.4 s · 71.0 s |

Each passes at its slow depth (`build/s1f.5.13/t3_other_slow_halves.log`, 2026-11-09; run beside this stage's gate and
the second lane's tests, so the times are under load). No wrong answer is hiding there today, but if one of the three
started failing, nothing would notice.

- **(a) Recorded, not scheduled.** The stage carries P-16's ruling, which is about one test; the three stay under
  `--slow` until the user places them.
- **(b) A nightly step each**, beside the round trip's in `deep-corpus`, `route_invariance`'s the largest, and a test
  in `nightly_steps.rs` that fails on a target reading `EIN_CORPUS_SLOW` with no nightly step, so that the next one
  cannot slip in the same way.

**Defaulted 2026-11-09 (the user's instruction): (a)**, parked for the phase's close beside
[Q-M1f.8](#q-m1f8--the-parsers-remaining-cost-within-grmtools-or-past-it): the stage stays in its scope, and nothing it
measured fails. Re-opening it means (b).

**Taken by [S1f.5.27](p1f.5_the_no_owner_items/s1f.5.27_the_nightly_reduced.md) on 2026-11-10**, when the user had accepted
(a) as parked and the close review's finding 8 re-opened the nightly: (b), by tier, under the stage's rule
([Q-M1f.102](#q-m1f102--what-the-ci-nightly-keeps-and-what-the-dev-host-runs)). `region`'s and `obligation_rung`'s slow
halves are CI nightly steps, each a second route held to an answer on the `zebra2-minus-15` family that no per-commit
test reaches; `route_invariance`'s is a dev-host step, over 5 min, its programs' answers digested per commit by
`corpus_shapes`. The guard (b) asked for is `nightly_steps.rs`' `every_slow_half_runs_on_a_tier`: a target that reads
`EIN_CORPUS_SLOW` and that neither the nightly nor `--dev-host` runs with it fails, but `read_out_examples`, whose slow
branch reads only the page's blocks at `--region-budget 33554432`, and the page has none, which the test checks. Each
slow half's time is in the stage's records.

### Q-M1f.52 — The nightly's review bundle and the register's runs that are not a solve

**Raised** 2026-11-09 by [S1f.5.13](p1f.5_the_no_owner_items/s1f.5.13_large_dumps_to_the_slow_tier.md)'s
T1f.5.13.3, outside its brief, running `./run_tests.sh --nightly` on `87d18cf6` (`build/nightly-s1f.5.13.log`). Since
`fc53818f`, where S1f.5.5's T13 filed W40 and W41 under the register's schema `/16` (a `now` that is a command's run and
not a solve), `utils/review_bundle.py`, a step of the nightly's `deep-corpus`, exits 1, for two reasons:

- its index reads `[case.wrong] verdict` unguarded (`utils/review/case.py`'s `index_page`), and a refusal has none:
  `W41/saturate-import`, and `W43/einb-dump-answer` since `96f66c40`, so the bundle stops with a `KeyError`;
- with that line guarded as the `right` column beside it already is (tried, and reverted), the bundle finishes and
  seven cases fail to build, W40's six and W41's, on `case.py`'s assertion that *a register run is a solve*
  (`build/nightly-s1f.5.13-rest.log`, 145 cases, 375 s).

The gate's `review_bundle.py --smoke` builds one case and no index, so the gate stayed green. No answer is wrong: one
nightly step is red, and on the runner every step after it in `deep-corpus` is skipped.

- **(a) Reported, not fixed here**: the coordinator places it, in P1f.5, whose S1f.5.5 filed the rows, or in a stage
  of its own.
- **(b) Fixed here**: the index guarded, and a case page for a run that is not a solve, the command, its exit, stdout
  and stderr against the register's; a page design of its own, under the user's rule that a review artefact carries
  every engine artefact.

**Defaulted 2026-11-09 (the user's instruction): (a).** The stage's brief is the round trip, and (b)'s page is a design
this stage does not own. Re-opening it means (b).

**Re-opened the same day as (b), taken by default on the user's instruction of 2026-11-09.** The coordinator placed it
as a stage of its own, [S1f.5.19](p1f.5_the_no_owner_items/s1f.5.19_the_review_bundle_and_runs_that_are_not_a_solve.md),
after S1f.5.13 and the merge of the second lane (T1f.5.M, `a90c452b`) and before S1f.5.14: the index guarded, and a
register case whose run is not a solve given a case page of its own, which shows the command as run, its exit, its
stdout and stderr in full, the register's `wrong`, `now` and `right` beside it, and every artefact the run writes,
linked and drawn where the engine draws one. The merge brought two more rows the index cannot read,
`W55/goal-neq-free` and `W56/goal-absent`, whose `[case.wrong]` is `ein saturate`'s refusal beside a solve's `now`.

**Built 2026-11-09** by S1f.5.19's T1f.5.19.2 (`00c4ed34`,
[its record](p1f.5_the_no_owner_items/s1f.5.19_the_review_bundle_and_runs_that_are_not_a_solve.md#t2-record-2026-11-09)):
`python3 utils/review_bundle.py` builds all 175 cases, 0 failed, the seven command runs on `utils/review/command.py`'s
page, and the nightly on that commit is green in every job and step, 81 min 54 s
([its T3 record](p1f.5_the_no_owner_items/s1f.5.19_the_review_bundle_and_runs_that_are_not_a_solve.md#t3-record-2026-11-09)).

### Q-M1f.53 — How the per-commit gate catches a register row the bundle cannot read

**Raised** 2026-11-09 by [S1f.5.19](p1f.5_the_no_owner_items/s1f.5.19_the_review_bundle_and_runs_that_are_not_a_solve.md),
on the coordinator's brief: *make the per-commit gate catch this class of break*. The gate's `review_bundle.py
--smoke` builds `W2/forced-2` and one program, so its index read one row shape, a walk's verdict; the whole bundle,
the nightly's, is the only run that read W40's to W56's.

- **(a) The smoke builds a command case** beside its solve case, `W40/kb-save` or `W41/saturate-import`, so the index
  reads a refusal and a command's run. It catches today's shapes, and a shape the next stage files waits for the
  nightly again.
- **(b) (a), and every register row read with no run**: the index's register cells, each case page's § 1 table, its
  fixed callout and its checklist, written from `corpus/wrong_answers.toml` alone for all of its rows, each row named
  where one fails. A row shape the bundle cannot read fails the commit that files it. It runs no engine, so it costs
  milliseconds.

**Defaulted 2026-11-09 (the user's instruction): (b).** It is (a) and the class besides, at no measurable cost.
Re-opening it means dropping the register pass from `cmd_smoke`.

### Q-M1f.54 — The 2026-11-06 inventory's evidence in `build/review/`

**Raised** 2026-11-09 by [S1f.5.19](p1f.5_the_no_owner_items/s1f.5.19_the_review_bundle_and_runs_that_are_not_a_solve.md).
The coordinator's brief allows `build/review/`, which is gitignored, to be cleared and rebuilt, so that the four dead
links S1f.5.13 found there go. Those four are in pages the
[no-owner inventory](../m1e2_correctness/inventory_2026-11-06_no_owner.md) wrote there on 2026-11-06 for F1's two
files, which `ccd1007f` deleted. The same directory holds the rest of that evidence, which the inventory's text names
(`build/review/items/<id>.md`): `items/`, `commands/`, `programs/`, `.state/` and its two driver scripts,
`build_open_items.py` and `gen_no_owner.py`, which are in no commit.

- **(a) Clear it**, as the brief allows: the evidence and the two scripts are gone.
- **(b) Move the evidence aside intact**, to `build/inventory-2026-11-06/`, at the same depth, so its relative links
  resolve as before, then clear `build/review/` and rebuild it. The nightly's local link check then reads what a fresh
  checkout's does, the bundle alone.
- **(c) Delete F1's three pages** and leave the rest where it is: the link check keeps reading the inventory's pages
  with the bundle's.

**Defaulted 2026-11-09 (the user's instruction): (b).** Nothing is lost, and the link check reads the bundle alone.
The inventory is a record and is not edited; [P1f.5's README](p1f.5_the_no_owner_items/README.md) names the new place
where it names the evidence. The two scripts name `build/review` in their defaults, so re-running one writes there
again.

### Q-M1f.55 — A `container` on a solve's `now`

**Raised** 2026-11-09 by [S1f.5.19](p1f.5_the_no_owner_items/s1f.5.19_the_review_bundle_and_runs_that_are_not_a_solve.md).
`W43/einb-dump-answer`'s `now` is a solve with `container = true` (schema `/16`'s field, on a solve since `/19`):
`wrong_answers.rs` saves the witness with `ein kb save` and runs `ein solve` on the `.einb`. The case page ran it on
the `.ein`, and said *this build re-ran it*.

- **(a) Run the container**, as `wrong_answers.rs` does: the case page saves the witness first, shows that step, and
  every invocation of the run reads the `.einb`, the program page showing the witness's source as the program the
  container stores.
- **(b) Run the `.ein`**, and say so on the page.

**Defaulted 2026-11-09 (the user's instruction): (a).** The page then shows the run the register holds, and every
command the run drives takes a container since S1f.5.5's T13 and S1f.5.12's W43.

### Q-M1f.56 — What the NL census counts: which fact set, and which registers

**Raised** 2026-11-10 by [S1f.5.14](p1f.5_the_no_owner_items/s1f.5.14_nl_required.md)'s T1f.5.14.1. The stage file
names `--json-summary`'s `verdict.solutions[i].facts`, which M1e2's switch removed, and three registers (puzzle,
stdlib-derived, kernel), where its acceptance asks for "the four registers".

- **(a) The whole state**: every fact of every state file `ein solve FILE --dump-answer DIR` writes, root's facts
  included, the set [Q-M1e.7](../../docs/history/m1e_review_processing/open_questions.md#q-m1e7--the-read-out-prints-the-solution-kb-and-calls-it-a-model)
  is about; and four registers, the stage's three and **undeclared** (a relation written or derived and never
  declared, and no rule's), the small set T1f.5.14.3 says the census prints.
- **(b) The listed projection**: `region.states[i].facts`, the facts over H's relations the read-out prints (25 on
  `zebra2`), every one of them a puzzle relation's.

**Defaulted 2026-11-10 (the user's instruction of 2026-11-09): (a).** It is the set the stage's 444-fact number was
about, and (b) has no register to split. `utils/nl_census.py`'s docstring states it.

### Q-M1f.57 — Which fixtures get a `:why`

**Raised** 2026-11-10 by S1f.5.14's T1f.5.14.7.

- **(a) Every file the loader reads**, a fixture refused at load among them, each `:why` on its form's last line so no
  line or column a refusal prints moves; a fixture refused at the parse is left as it is, since the parse refusal
  stops before the loader reads a `:why`; and the `.ein` files under `plans/` and `docs/history/`, records no test
  loads, are left too.
- **(b) Every `.ein` file**, those refused at the parse and the records included.

**Defaulted 2026-11-10 (the user's instruction of 2026-11-09): (a).** A `:why` in a file refused at the parse is
read by nothing and moves the positions its refusal prints; a record is not edited.

### Q-M1f.58 — Where a fact is rendered in English, and what it reads from

**Raised** 2026-11-10 by S1f.5.14's T1f.5.14.2. The stage file's surface for a fact's sentence, the read-out's
*rendered* answer table, went with the walk at M1e2 S1e2.1c.5; since then a relation's `:why` reaches no surface,
though `01_grammar.md` still said the solve table renders it. The read-out's `(model …)` blocks are Ein text a claim
is pasted from.

- **(a) The state files and the trace**: a `--dump-answer` state file's comment after each fact carries its rule's
  `:why` filled at its first recorded justification and its relation's `:why` filled with its arguments; `--trace`'s
  *Derives* line carries the statement. stdout and `answer.txt` do not move.
- **(b) (a), and a new presentation form on stdout**, `--models text`, a sentence per listed fact.
- **(c) The read-out's `(model …)` blocks in English**, which would stop them being claims.

**Defaulted 2026-11-10 (the user's instruction of 2026-11-09): (a).** It costs nothing on a bare `ein solve` and moves
no read-out byte; (b) is a form nobody asked for and the stage's minimum-work rule leaves it out; (c) breaks what a
claim is. *First recorded* is the stage file's own pick, and the user's ruling of 2026-11-08 on cores (*"core is an
explanation, F22 makes it canonical"*) covers it.

### Q-M1f.59 — What a macro's `:why` renders

**Raised** 2026-11-10 by S1f.5.14's T1f.5.14.3. S1f.5.5's T3 gave a macro an optional `:why`, and since the swap
`--require-why` asks for it. A macro is expanded at load, and no fact's justification names one.

- **(a) Nothing**: it documents the pattern for a reader of the program, and a state file re-writes it with the program.
- **(b) The rule's reason**: fill the macro's `:why` into the reason of a fact whose rule's `:match` used it.

**Defaulted 2026-11-10 (the user's instruction of 2026-11-09): (a).** The rule that used the macro has a `:why` of its
own that says what the pattern meant there; (b) is a second reason for one fact.

### Q-M1f.60 — A relation nobody declares

**Raised** 2026-11-10 by S1f.5.14's T1f.5.14.3: an auto-vivified relation has no form to carry a `:why`.

- **(a) No slot**: a fact of one that a rule derived reads by its reason; one the program states falls back to its
  s-expression, and the census counts it under `stdlib` or `undeclared`.
- **(b) A declaration-less template**, a new form that templates a relation without declaring it.

**Defaulted 2026-11-10 (the user's instruction of 2026-11-09): (a).** (b) is a language change; a program that wants
the fact read declares the relation.

### Q-M1f.61 — A template per goal conjunct

**Raised** 2026-11-10 by S1f.5.14's T1f.5.14.3: `:goal-text` is one string for the whole query.

- **(a) No change**: each conjunct's fact reads by its relation's `:why` where a surface prints it.
- **(b) A per-conjunct template**, a language change.

**Defaulted 2026-11-10 (the user's instruction of 2026-11-09): (a)**, as the stage file says a yes would be a
question and not a quiet addition.

### Q-M1f.62 — The option that loads a program without a `:why`, once the default is true

**Raised** 2026-11-10 by S1f.5.14's T1f.5.14.7. The stage file asks that a program without `:why` still load once the
default is true.

- **(a) `--no-require-why`** on the seven commands, a set-true flag as `--no-types` and `--no-instances` are, replacing
  `--require-why`; `LoadOptions::default()` requires.
- **(b) `--require-why=false`**, a valued option.
- **(c) Both `--require-why` and `--no-require-why`.**

**Defaulted 2026-11-10 (the user's instruction of 2026-11-09): (a).** Every boolean option the CLI has is a set-true
flag, and a negative one is spelled `--no-…`; `--require-why` would be inert with the default true, and an inert
option is what [Q-M1e.10](../../docs/history/m1e_review_processing/open_questions.md#q-m1e10--two-config--flags-are-inert)
retired. The option count stays 78.

### Q-M1f.63 — What each program generator does about `:why`

**Raised** 2026-11-10 by S1f.5.14's T1f.5.14.7: once the default requires a `:why`, a generator whose programs carry
none generates programs the loader refuses.

- **(a) Each emits an honest, mechanical `:why`** on every form it writes: `utils/fuzz_engine.py` (whose rules wrote
  *"fire-N fired"*, the placeholder the stage forbids, on 40 %), `utils/region_synth.py`, `utils/guard_split.py`, the
  Rust suites' generators (`detector.rs`, `compile_limits.rs`, `match_alloc.rs`, `imports_semantics.rs`,
  `region_dump.rs`), and `examples/gen_zebra2_variants.py` by copying `zebra2.ein`'s; each keeps its random draws, so a
  seed generates the program it did. `fuzz_properties`, which mutates the corpus's own programs, loads under the
  default: its seeds carry their `:why`s, and 419 of the 453 programs its sweep accepts load (2026-11-10).
- **(b) Every generator loads with the requirement off.**

**Defaulted 2026-11-10 (the user's instruction of 2026-11-09): (a).** It keeps each generator's question: the engine
fuzzers' programs load under the default, and a mechanical `:why` says what the form says, its clauses in words.

### Q-M1f.64 — Existing `:why`s that are placeholders or name nothing

**Raised** 2026-11-10 by S1f.5.14's T1f.5.14.5 and T1f.5.14.7. The check found 45 relation `:why`s over 13 files
writing `{1}` for `{?1}` (58 literals) and one rule's writing a relation's `{?1}` and `{?2}` (`uncovered.ein` and its
twin), each printing as written; and the migration's helpers found `:why`s that are placeholders: `"a"`, `"b"`,
`"w"`, `"u"`, `"forall"`, `"guarded"`.

- **(a) Fix each**, in the commit of the check that finds it (the references) or the migration (the placeholders); the
  check flags a literal `{x}` that names a reference its form could fill.
- **(b) Leave them**: the mandate asks for a `:why`, not a good one.

**Defaulted 2026-11-10 (the user's instruction of 2026-11-09): (a).** The user overrode the refusal of rung (d) on the
understanding that each `:why` is honest. None is a wrong answer: a `:why` moves no R, N, H, claim or exit code, so
none is a register row. Terse but true ones (`"{?rel} sym"`) are left.

### Q-M1f.65 — May a comment hold Unicode?

**Raised** 2026-11-10 by [S1f.5.22](p1f.5_the_no_owner_items/s1f.5.22_unicode_only_in_a_string.md)'s census, on the
user's ruling of that day: *"Unicode is not allowed anywhere else than inside string literals, and the error must
say it explicitely"*. No corpus program writes a character that is not ASCII outside a string or a comment. 429 of
the 454 `.ein` files write 35 590 of them in comments, 88 % the box-drawing `─`, and the doc pages' Ein blocks carry
more.

- **(a) Refuse it in a comment too**, the ruling read literally. Every comment is rewritten in ASCII first by a
  mapping written before the edit (`─` → `-`, `→` → `->`, `⟹` → `=>`, …), mechanically, moving no answer and no
  position a refusal prints.
- **(b) A comment may hold Unicode.** A comment is free text, as a string is, and the refusal says *inside a
  string literal or a comment*.

**Ruled 2026-11-10, (b)** (the user chose *"Comments may hold Unicode"*). Only a token may not hold a character
that is not ASCII; no file changes.

### Q-M1f.66 — May `open` be written as a value?

**Raised** 2026-11-10 by [S1f.5.23](p1f.5_the_no_owner_items/s1f.5.23_open_is_reserved.md)'s T1, on the user's ruling
of that day: *"`open` is reserved word, here it appers as what? as relation name? - then it is an error, if not -
then explain"*. As a relation name (a fact's head, a nested fact's head, a pattern's head, a relation parameter's
binding) it is refused by the ruling. As a value, `(p open)`, it is a name `p` holds, which loads today, as the
other position words do. No corpus program writes it there.

- **(a) Refuse it**: `open` is written only as a rule's `(open)` or `(open ?R)` conclusion.
- **(b) Keep it as a value**, a position word like `model`; refused only as a relation name.

**Ruled 2026-11-10, (a)** (the user chose *"Refuse it"*). `open` is reserved in the plain sense.

### Q-M1f.67 — Where is a macro body that no rule could use refused?

**Raised** 2026-11-10 by [S1f.5.24](p1f.5_the_no_owner_items/s1f.5.24_a_macro_body_checked_where_a_rule_is.md)'s T1.
`(macro one (?a) (eq ?a))` parses and loads, and is refused only when a rule using it is compiled. `kb save`,
`render program` and `render rules` accept it.

- **(a) At its definition where its literal head decides, and at each use after expansion otherwise.**
- **(b) At each use, after expansion**; a macro no rule uses loads whatever its body.
- **(c) At compile, as today**, in S1f.5.21's words.

**Ruled 2026-11-10.** The user: *"So the question is "do we allow invalid macro if it is not instantiated - the
answer is yes, we allow, error will raise later during either compilation of instantiation or even later, the
ultimate goal in this case is not allow execution of (eq <arg>)"*. So a macro body is not refused at its
definition. The refusal may come at expansion, at compile or later, and the requirement is that no path executes a
malformed primitive. Today's compile-time refusal meets it wherever every executing path compiles first, which
S1f.5.24 shows and pins.

### Q-M1f.68 — What Ein does when its reader closes stdout

**Raised** 2026-11-10 by [S1f.5.25](p1f.5_the_no_owner_items/s1f.5.25_a_closed_stdout.md)'s T1. Rust ignores
`SIGPIPE`, `print!` panics on the failed write, and the run exits 101. That happens on every command that writes
more than once after its reader has gone.

- **(a) Stop quietly and exit 0**, a `BrokenPipe` on stdout ending the command; any other write error is exit 1
  with one line.
- **(b) Die by `SIGPIPE`**, as `cat` does: the shell sees 141; needs `unsafe` or a dependency in `ein-cli`.
- **(c) Exit 1 with one line** saying stdout was closed.

**Ruled 2026-11-10, (b).** The user: *"why not handle sigpipe properly?"* The coordinator had weighed (b) down
for its `unsafe` and its 141. Neither holds up. The disposition is a process-wide choice that belongs on the
binary, as the global allocator does (`ein-cli/src/main.rs`), not in the library an embedder links, and
`main.rs` is not under `lib.rs`'s `#![forbid(unsafe_code)]`. And a death by `SIGPIPE` is the Unix filter's
convention (`cat`, `grep`), not an exit Ein chooses. Rust 1.97.1 has no stable option for it
(`rustc -C help`), so it is one audited `libc::signal(SIGPIPE, SIG_DFL)` call on the binary, the second
audited `unsafe` site beside `ein-einb`'s `cast.rs` (design/12 § 2).

### Q-M1f.69 — What the guide's pin reads in a program block

**Raised** 2026-11-10 by [S1f.5.26](p1f.5_the_no_owner_items/s1f.5.26_the_tutorial_runs.md)'s T2. The stage says a
`lisp` block of `docs/guide/` is a whole program or an excerpt (it writes `…`), that a whole program is loaded and
answered and an excerpt's complete forms parsed and loaded, and leaves open how the kind and the forms are told
apart, how an excerpt's forms load, and what *answered* asks.

- **(a) The text decides, read as the lexer reads strings and comments.** A block that writes `…` outside a string
  is an excerpt; a top-level form with a `…` in it outside a string and a comment is elided, and blanked, as is a
  `…` between forms, character for character, so a position the loader prints is the page's. An excerpt's whole
  forms load together, as one program, through `ein_ir::load_file` under the default options; a whole program loads
  the same way and `ein solve FILE`, the default question, must exit 0, so a refuted `:expect` fails it. The prose,
  the `sh` commands outside a marked transcript, and whether an excerpt quotes its file are not read.
- **(b) Each whole form of an excerpt loaded alone.** A refusal names the form rather than the block, and two forms
  that clash, two relations of one name, pass.
- **(c) A marker per block**, `<!-- program -->` or `<!-- excerpt -->`, as a transcript has, rather than the `…`.

**Defaulted 2026-11-10 (the stage's default): (a).** A `…` is what tells a reader a block is cut, so the test reads
the block as the reader does and needs no marker kept in step with it; the forms loading together is what a reader
who copies them gets. `ein-cli/tests/guide_programs.rs` holds it.

### Q-M1f.70 — What the stage does with what T1 found false beside the blocks

**Raised** 2026-11-10 by [S1f.5.26](p1f.5_the_no_owner_items/s1f.5.26_the_tutorial_runs.md)'s T1. Besides the three
blocks today's engine refuses, the read found two blocks it loads that are false: chapter 2's transitive block
answers one state where the page's run of the file it quotes says four, lacking the file's `(relation is-a T T …)`,
and chapter 2's co-located block quotes a rule `05_mini_zebra.ein` does not have; and four sentences of prose:
chapter 1's *`is-a` is a built-in relation*, chapter 4's bare `ein solve` that *stops at its default budget … and
says so on stderr*, `-s`'s *wall clock per phase*, and `--trace`'s *"it follows that…" narrative*.

- **(a) Fix each in the page**, as the stage fixes the refusals; the test does not hold them, and the guide's README
  says so.
- **(b) Fix the refusals only**, the stage's acceptance read narrowly, and leave the rest to a later reading.

**Defaulted 2026-11-10 (the stage's default): (a).** The user's ruling is *"fix the tutorial"*; each is the page's,
none moves an engine answer or a register row, and a reader who copies chapter 2's block or trusts chapter 4's
sentence is misled as one who copied the retired flag was.

### Q-M1f.71 — Does a view draw a program the loader refuses?

**Raised** 2026-11-10 by [S1f.5.21](p1f.5_the_no_owner_items/s1f.5.21_a_compile_error_in_eins_words.md)'s T1. `ein render program`, `render rules`, `render rule` and `render constraints` parse a
program and never load it (*"Parsed, never loaded, so a program the loader refuses still draws"*, M1e2 S1e2.1.3, the
agent's design answering Q-M1e.20), so each draws a program every other command refuses, exit 0: the close review's
finding 1 names `render program` and `render rules` doing so on `nested_or.ein`, and once the stage moves that refusal
to the loader they would draw it still, as they draw the twelve load-negatives that have a `render rules` run today.

- **(a) The views refuse it**: each reads the program through the loader first, as `solve`, `saturate`, `oracle`,
  `test`, `render lattice`, `render kb` and `kb save` do, refuses what it refuses with the same line and exit 1, and
  then draws the program as written, its imports unresolved, as before; each takes `--no-require-why`, so a program
  with no `:why` loads where the other seven load it. The one-tool rule is *no two commands may disagree*, and a view
  that exits 0 on a refused program disagrees about whether it is one. The cost: a refused program is not drawn, so the
  review bundle's program page shows the refusal where it drew the rules (`utils/review/run.py`), and four more
  commands take the option (78 → 82 CLI options, `configuration.md`).
- **(b) The views draw it and print the loader's line as a `note:`**, exit 0: nothing a reviewer reads is lost, and
  the views keep drawing what the loader refuses; but they still answer a program every other command refuses.

**Defaulted 2026-11-10 (the stage's default): (a).** It is the stage file's default, written from the user's ruling on
finding 1, whose text counts the views' exit 0 as a break of the one-tool rule. W65's rows hold it (`render-program`,
`render-rules`, `render-rule`, `render-constraints`, beside `kb-save`), and `render_doors.rs`' test of a drawn refusal is
turned around. Re-opening it means (b).

### Q-M1f.72 — The line a moved refusal prints

**Raised** 2026-11-10 by [S1f.5.21](p1f.5_the_no_owner_items/s1f.5.21_a_compile_error_in_eins_words.md)'s T1. The stage file sketches the line as `<file>:<line>:<col>: rule '<name>': <what>`,
*as the loader's refusals since S1f.5.9, .10 and .18 print*; those print `rule '<name>': <what>`, behind the CLI's
`kb load error: `, with no position (`examples/broken/load/derived_activator.expected`, `W57`'s `now`).

- **(a) The precedent's shape**: each moved refusal is one loader line, `rule 'r': its :match writes (or (p ?x) (q ?x))
  as a premise of and — …`, `; `-joined with the loader's others behind `kb load error: `, the form as Ein text
  (`ein_ir::dump_compact`); the compiler's remaining line, reached through a macro's body, keeps a position, since the
  form it names is the macro's and not the rule's: `<file>:<line>:<col>: rule 'r': its :match writes (eq ?x), and eq
  takes exactly 2 arguments — …`. The compiler knows a rule's name and not its form, so an hrule's reads `rule 'h'`.
- **(b) A position on every line**, the loader's too, as the sketch writes it: the moved lines and S1f.5.9's, .10's and
  .18's would differ, or the earlier ones move with no answer behind it.

**Defaulted 2026-11-10 (the stage's default): (a).** The sketch's stated reason is the precedent, and the precedent has
no position; a position on the loader's lines is D-2's question (F23), whose `Loc(…)` half this stage takes
(Q-M1f.75). Re-opening it means (b).

### Q-M1f.73 — How the loader counts the register ceiling

**Raised** 2026-11-10 by [S1f.5.21](p1f.5_the_no_owner_items/s1f.5.21_a_compile_error_in_eins_words.md)'s T1. The compiler numbers a plan's variables into a register file of 256 (`MAX_REGS`):
one for the plan, the rule's parameters and the variables its positive premises and guards write, and one per lifted
`(absent …)`, the variables its body writes. Moving the refusal to the loader needs a count the loader can make.

- **(a) One count**: every distinct variable a rule's `:match` writes, anywhere in it, and its parameters, or every one a
  `:goal` writes, at most 256. It bounds each register file the compiler builds, so the compiler's arm is an invariant,
  and it says the bound in the program's words, *a rule binds at most 256 variables*. It refuses a program the compiler
  would compile, one whose `:match` writes more than 256 variables split between its positive premises and an
  `(absent …)`'s body; no corpus program is within 249 of it (the widest binds 7).
- **(b) The compiler's count**, register file by register file, in the loader: exact, and a second statement of the
  compiler's scoping kept in step with it.

**Defaulted 2026-11-10 (the stage's default): (a).** `compile_limits` holds a rule at 256 to load and compile, and one
at 257 to be refused at load. Re-opening it means (b).

### Q-M1f.74 — A nested `(or …)` a macro's expansion writes

**Raised** 2026-11-10 by [S1f.5.21](p1f.5_the_no_owner_items/s1f.5.21_a_compile_error_in_eins_words.md)'s T1. `(forall ?x (or (p ?x) (q ?x)) (a ?x))` and a program's `(macro either (?x) (or
(p ?x) (q ?x)))` called inside an `(and …)` expand to a nested `(or …)`, which the compiler refuses as it refuses a
written one. The user's ruling on Q-M1f.67 leaves a macro's malformed primitive to S1f.5.24 and allows its refusal at
expansion, at compile or later.

- **(a) Refused at load, as a written one is**: the loader reads the rule after expansion (S1f.5.10's check does), so
  one walk refuses both; the macro's body is no malformed primitive (`(or …)` is a whole `:match` wherever the macro is
  so called), and a macro no rule uses still loads.
- **(b) Left at compile**, S1f.5.24's with the arity lines.

**Defaulted 2026-11-10 (the stage's default): (a).** It is a refusal at instantiation, which Q-M1f.67's ruling allows,
and it leaves the compiler no nested `(or …)` to meet. Re-opening it means (b).

### Q-M1f.75 — The compiler's arms no program reaches, and D-1's and D-2's other lines

**Raised** 2026-11-10 by [S1f.5.21](p1f.5_the_no_owner_items/s1f.5.21_a_compile_error_in_eins_words.md)'s T1. Three of the compiler's refusals are reached by no program and by a direct caller
of `compile_rule`: an activator of another arity (`activators_for` drops it), a relation head its activator binds to a
fact (S1f.5.18's checks refuse each activator a program can write or derive), and a rule's relation variable compiled
with no activator. D-1 names `FileNotFoundError` beside `CompileError`, and D-2 the loader's `at None` and `at
Loc(file=…, line=…, col=…)`.

- **(a)** The three stay the compiler's errors for that caller, each line in Ein's words; the arms the loader now
  covers (the nested `(or …)`, the register ceiling, a goal's relation variable) and the `_` head (the grammar's
  `PremiseHead`) become invariants (Q-M1f.9 (a)'s pattern). `FileNotFoundError` is the read path's
  (`common::read_program`), not the compiler's: left to F23, with a line in D-1's row. `ein_ir::loc_repr`, which the
  compiler's arity line and the loader's macro lines print, is one function: its `Loc(…)` is taken here,
  `<file>:<line>:<col>`, and `at None`, a top-level form with no position, stays F23's.
- **(b)** Every arm no program reaches an invariant, the three included; and D-1's and D-2's rest left to F23 whole.

**Defaulted 2026-11-10 (the stage's default): (a).** The three are a library caller's diagnostics, which an invariant
would turn into a panic for a call the API allows; and the `Loc(` the acceptance names is printed by the one function
both use. Re-opening it means (b).

### Q-M1f.76 — Which of what the move replaces is a register row

**Raised** 2026-11-10 by [S1f.5.21](p1f.5_the_no_owner_items/s1f.5.21_a_compile_error_in_eins_words.md)'s T1. The move replaces, for each refusal, a stop on every command that compiles, a
disagreement where only a commitment, an hrule or a listed state compiles it, and an answer where nothing compiles it;
and four views and `kb save` that answer a program every other command refuses. `examples/features/02_star_in_identifiers.ein`,
a `positive` corpus program, writes `(query :goal (?R Rex Animal))`, which `solve` and `oracle` refuse and its declared
runs never ask.

- **(a) A row per way the engine answers today**, Q-M1f.36's reading: W62's six (the nested `(or …)` at root, under a
  commitment, in an hrule, never compiled, in a goal, in a goal with no state), W63's three, W64's two and W65's five,
  each `open` before T1f.5.21.2 and fixed by it; `features/02`'s goal rewritten, `(is-a* Rex Animal)`, its declared runs
  answering as before, with no row (Q-M1f.39's reading of the one corpus program a check refuses).
- **(b) One row per refusal**, the ways in the note.

**Defaulted 2026-11-10 (the stage's default): (a).** A stop and a refusal are two answers, as are an answer and a
refusal (Q-M1f.36, Q-M1f.44), and the precedent filed every way. Re-opening it means (b).

### Q-M1f.77 — When is `utils/ci_container.sh` part of done?

**Raised** 2026-11-10 by [S1f.5.29](p1f.5_the_no_owner_items/s1f.5.29_the_review_bundle_smoke_on_the_runner.md). Twice a Graphviz refusal reached the per-commit CI behind
a green `./run_tests.sh` here: S1f.5.17's, the runner's x86-64-v3 `libpathplan` failing to route 20 views, and
S1f.5.29's, the runner's `fdp` with no layout plugin behind it, which refused every KB view the review bundle's smoke
draws, red on GitHub from `00c4ed34` on. In both, `utils/ci_container.sh`'s image, built like the runner, showed what
the runner showed. It runs HEAD as committed, so it is a check on a commit, not on a working tree.

- **(a) A stage that changes a DOT writer, the review bundle or what CI installs is not done until
  `utils/ci_container.sh` is green on its commit**, the gate's run in the image beside the local one, and the stage's
  commit or record says both. Cost: one more gate run per such stage, in the image, which needs an x86-64-v3 CPU and
  `docker`.
- **(b) Install the baseline Graphviz build in CI**, `runs-on` pinned or the amd64 variant forced, so the runner meets
  the build the local gate meets. It answers S1f.5.17's case only: S1f.5.29's plugin is missing from both 26.04
  builds, and the local Graphviz is neither.
- **(c) Run the smoke's drawings in the container from `run_tests.sh` itself**, so every local gate draws them on the
  runner's build. Every developer's gate then needs `docker`, a built image and the CPU, which the gate needs nowhere
  else.

**Defaulted 2026-11-10 (the stage's default): (a).** It is the stage file's default: it costs only the stages that
can break the drawing, and it would have caught both cases. Re-opening it means (b) or (c).

### Q-M1f.78 — Where the runner's missing `fdp` is fixed

**Raised** 2026-11-10 by [S1f.5.29](p1f.5_the_no_owner_items/s1f.5.29_the_review_bundle_smoke_on_the_runner.md)'s T1. The runner's Graphviz 14.1.2 refuses every KB view the review bundle
draws, 612 of the whole bundle's 3 885 laid-out DOTs, with *There is no layout engine support for "fdp"*: every KB
view suggests `fdp` (`kb_dot::to_dot`), and Ubuntu 26.04's `graphviz` links `fdp` to `dot` without
`libgvplugin-neato-layout8`, the plugin behind it, which it neither depends on nor recommends. The stage file had
presumed S1f.5.17's cause and asked for a view-side fix; no DOT is at fault. Each fix below draws all 3 885 on the
runner's build (the stage's T1 record).

- **(a) The runner installs the plugin**: `libgvplugin-neato-layout8` beside `graphviz` on the apt line of
  `per-commit.yml`, `nightly.yml` and `release.yml` (its two, for parity), in `utils/ci_container.sh`'s image (only
  where the archive has the package: 24.04's `libgvc6` carries the plugin), and in the provisioning lines
  `gate_steps.rs` and `nightly_steps.rs` name; `run_tests.sh` exits 127 where `fdp` lays out no graph, as it does
  where there is no `dot`. No view, golden, answer or drawing moves, and the CI lays out what the user's bundle lays
  out. Cost: a 26.04 package name on the apt line, which a later image renames loudly (the install fails).
- **(b) The bundle falls back to `dot`** where the suggested engine does not load (one probe a run), and says so on
  the page. No golden moves, but on the runner the smoke checks a drawing the user's bundle does not make.
- **(c) The KB view stops suggesting `fdp`** (`kb_dot.rs`), so every KB view is laid out by `dot` everywhere. It moves
  the four DOT goldens that carry the line (`kb_render_to_dot.dot`, `slice_render_solution.dot`,
  `slice_render_state.dot`, `kb_zebra_unified.dot`), at least the 2 346 `dot[kb…]` lines of `corpus_shapes.md5`, the
  worked fragment in `04_dot_rendering.md`, and every KB drawing the user reviews, from force-directed to ranked.

**Defaulted 2026-11-10 (the coordinator's default): (a).** It fixes the cause, a package, and moves nothing. No user
ruling is reversed: the user's ruling on S1f.5.17 was not to report the Graphviz bug upstream, and *view-side* was
the stage file's presumption, not a ruling. Re-opening it means (b) or (c).

### Q-M1f.79 — Does a refused DOT fail the whole bundle?

**Raised** 2026-11-10 by [S1f.5.29](p1f.5_the_no_owner_items/s1f.5.29_the_review_bundle_smoke_on_the_runner.md)'s T1. Since `00c4ed34` the smoke fails on a DOT with no SVG beside it, but
the whole bundle, the nightly's step, wrote a refused drawing on its page as *Graphviz refused this graph* and exited
0, a case failing only on an exception. So the GitHub nightly's bundle step drew no KB view from 2026-11-06, when the
runner moved to 26.04, and passed (the nightly was red then on later steps, S1f.5.17's mutants and S1f.5.19's row
shape), as S1f.5.17's in-container nightly at `76ede241` (*137 cases, 0 failed*) did in the same
image. Five DOTs in the whole bundle are kept undrawn on purpose: lattices past `DAG_NODES_DRAWN` (1 000 nodes),
whose page says so.

- **(a) Yes, as in the smoke**: `cmd_bundle` exits 1 on a DOT Graphviz refused, printing its path and what Graphviz
  said (kept beside the DOT as `.refused.txt`), over the cases it built. A DOT kept undrawn by design says so beside it
  (`.undrawn.txt`, `review.util.keep_undrawn`) and is not a refusal, told apart by what the bundle wrote, not by its
  size or path. The summary line counts the refused DOTs.
- **(b) No**: the nightly's bundle stays a review artefact, a refusal a line on its page, and only the smoke gates.
  A drawing the user reviews can then go missing with the nightly green, as every KB view did.

**Defaulted 2026-11-10 (the coordinator's default): (a).** A review artefact missing its drawings is what the user's
rule against incomplete review artefacts forbids, and the nightly is the only run that builds the whole bundle.
Re-opening it means (b).

### Q-M1f.80 — Where a malformed primitive a macro's expansion writes is refused

**Raised** 2026-11-10 by [S1f.5.24](p1f.5_the_no_owner_items/s1f.5.24_a_macro_body_checked_where_a_rule_is.md)'s T2. The user's ruling on Q-M1f.67 allows the refusal *"during either compilation of
instantiation or even later"* and requires that `(eq <arg>)` is never executed. T2 found that the compile-time
refusal met that only at root: under a commitment, in an hrule, in an obligation and in a rule no activator reaches,
some commands answered a program others refused (W66, W69), and seven shapes the compiler never checked were
matched, concluded or proposed, `(eq k)` stored among them (W67, W68).

- **(a) At load, after expansion**: the loader holds a rule's clauses, expanded, to the shapes the grammar gives a
  written rule (`ein_ir::from_ir`'s `refuse_malformed_primitives`), so every command refuses the program, one line,
  before any path runs the rule, as Q-M1f.74 (a) refuses a nested `(or …)` a macro writes. A macro no rule calls
  still loads. The views and `kb save` refuse such a program too, a view refusing what the loader refuses
  (Q-M1f.71), so `kb save` writes no container every other command refuses.
- **(b) An eager check before execution**: every command that compiles or fires runs the same walk over every rule
  before it starts, and the views and `kb save` keep accepting the program, as the stage's section on the ruling
  expected while the compile-time refusal was thought to suffice. A second walk to wire into six commands and the
  container's, and `kb save` writes a container every executing command refuses, which W65 is about.
- **(c) Where compiled or fired, as today**, with checks added to the compiler for conclusions: the root sites only;
  the commitment, hrule, obligation and no-activator disagreements stay.

**Defaulted 2026-11-10 (the stage's default): (a).** It is a refusal at instantiation, which the ruling allows, the
one place before every path, and the precedent's (Q-M1f.74). Re-opening it means (b).

### Q-M1f.81 — What a malformed primitive is

**Raised** 2026-11-10 by [S1f.5.24](p1f.5_the_no_owner_items/s1f.5.24_a_macro_body_checked_where_a_rule_is.md)'s T2. The ruling names `(eq <arg>)`; the stage file names a literal head, a head
through a parameter, and `absent` or `not` given a premise list. T2 found more shapes a macro's expansion writes
that the grammar refuses where a rule writes them, each matched or concluded today.

- **(a) Every form the grammar would refuse where it stands**: `eq` and `neq` at two values (`Guard`), `absent` at
  one premise, `not` of one pattern, or as a conclusion of a variable too, `false` with nothing, no `(or …)` as a
  conclusion, no connective where a value goes (`Term`), and a conclusion that is a form (`Conclusion`). One walk,
  the grammar's productions by position, so a written rule never fails it.
- **(b) `eq`'s and `absent`'s arity only**, the compiler's two lines; the rest left matched or concluded.

**Defaulted 2026-11-10 (the stage's default): (a).** Each of (b)'s leftovers is a wrong answer T2 found (W67, W68,
W69), and the ruling's property is about every malformed primitive, `(eq k)` concluded among them. Re-opening it
means (b).

### Q-M1f.82 — The line a malformed primitive is refused with

**Raised** 2026-11-10 by [S1f.5.24](p1f.5_the_no_owner_items/s1f.5.24_a_macro_body_checked_where_a_rule_is.md)'s T3. S1f.5.21 gave the compiler's line a position, the macro body's, since the form it
names is the macro's (Q-M1f.72 (a)), and the loader's lines none.

- **(a) The compiler's line, behind `kb load error: `**: `<file>:<line>:<col>: rule 'r': its :match writes (eq ?x),
  and eq takes exactly 2 arguments, not 1 — it compares two values, so write one guard per pair`. The position is
  the first one at or under the expanded form: the macro body's for a form the body writes as `(eq ?a)`, and, for
  one the grammar builds for a keyword (`(not …)`, `(neq …)`, `(or …)`, which keep none), its first part's, the
  call's argument, so a form `std.macro`'s `unknown` writes is placed in the program's file. The rule is named as
  the loader names it, `hrule 'h'` for an hrule.
- **(b) The loader's precedent, no position**: one spelling for every loader line, and the macro body's line loses
  the position S1f.5.21 gave it.

**Defaulted 2026-11-10 (the stage's default): (a).** It keeps S1f.5.21's line, as the brief asks, and the form is
the macro's, which a position finds. Re-opening it means (b).

### Q-M1f.83 — Which of what the refusal replaces is a register row

**Raised** 2026-11-10 by [S1f.5.24](p1f.5_the_no_owner_items/s1f.5.24_a_macro_body_checked_where_a_rule_is.md)'s T2: eighteen shapes at four or five sites, eighty programs, and for each some or every
command answered, stopped, or executed the primitive.

- **(a) A row per way the engine answered where a refusal is right** (Q-M1f.36's reading, as Q-M1f.76): W66's four
  (`eq` or `absent` under a commitment, in an hrule, in an obligation, in a rule no activator reaches, the last
  standing for that site in W67 to W69), W67's four (matched at root twice, by the hypothesis layer, by the
  obligation pass), W68's six (concluded at root five times, proposed by an hrule once) and W69's two (a commitment,
  an hrule). The compile stop and the firing stop at root are no row: the ruling allows an error at compile or
  later, and nothing executes; their views' and `kb save`'s exits move with the loader's refusal (Q-M1f.71), as
  S1f.5.21's twelve load-negatives' `render rules` did.
- **(b) A row per shape and site**, eighty.

**Defaulted 2026-11-10 (the stage's default): (a).** The precedent filed every way, not every program; the stage's
pin holds all eighty. Re-opening it means (b).

### Q-M1f.84 — The compiler's arity arms and the firing's whole-conclusion line

**Raised** 2026-11-10 by [S1f.5.24](p1f.5_the_no_owner_items/s1f.5.24_a_macro_body_checked_where_a_rule_is.md)'s T3. Once the loader refuses each shape, three lines no program reaches remain: the
compiler's `absent` and `eq` arities and the firing's *its :assert concludes ?y, which is not a fact*.

- **(a) Invariants**, an `assert!` naming the loader's check, as S1f.5.21's nested `(or …)` arm (Q-M1f.9 (a)'s
  pattern); `fuzz_properties::the_compiler_refuses_nothing_the_loader_accepts` loses its exemption for a macro
  body's arity.
- **(b) Library errors** for a direct caller of `compile_rule` with a hand-built tree, as Q-M1f.75 (a) kept the three
  the loader does not cover.

**Defaulted 2026-11-10 (the stage's default): (a).** The loader covers each, which is (a)'s precedent; (b)'s three
are the ones it does not. Re-opening it means (b).

### Q-M1f.85 — Which findings P1f.7 takes

**Raised** 2026-11-10 by the user's instruction: *"new batch of findings
plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.28_the_unchecked_findings_triaged.md creates
new phase p1f.7 with stage per finding to allow to close current phase"*. S1f.5.28 then listed sixteen items. Items 1
to 11 were the close review's findings 9 and 7, which the user had ruled into that stage. Items 12 to 16 were wrong
answers that S1f.5.21's T1 and S1f.5.24's T2 found outside their scope, which the coordinator had added the same day.

- **(a) Items 12 to 16, and every later finding of the same kind**: the batch becomes S1f.7.1 to S1f.7.5, and a
  wrong answer that P1f.5's remaining stages (S1f.5.22, .23, .25, .27, .28) find outside their scope becomes a
  stage from S1f.7.6. S1f.5.28 keeps items 1 to 11.
- **(b) Items 12 to 16 only**: later findings go back to S1f.5.28, which keeps growing until the phase closes.
- **(c) All sixteen items**: S1f.5.28 is emptied, and the close review's findings 9 and 7 leave the phase the user
  ruled them into.

**Defaulted 2026-11-10 (the coordinator): (a).** It is the reading of *"to allow to close current phase"* that
lets P1f.5 stop growing. (c) would undo the user's ruling on finding 9. Re-opening it means (b) or (c).

### Q-M1f.86 — May a macro's body write `open`?

**Raised** 2026-11-10 by [S1f.5.23](p1f.5_the_no_owner_items/s1f.5.23_open_is_reserved.md)'s T2, on the user's
ruling on [Q-M1f.66](#q-m1f66--may-open-be-written-as-a-value): `open` is written only as a rule's `(open)` or
`(open ?R)` conclusion. A macro's body is a template (`ein.y`'s `Template`), whose head is any name, so until S1f.5.23
a body could write `(open ?R)`: a rule's `:assert` calling it was an obligation (`W72/macro-verdict`), and one called
in a `:match` or beside a conclusion was `validate_open`'s refusal after expansion.

- **(a) Refused at the parse, called or not**: `open` is no name, so no template writes it, as none writes `_` or a
  string. The verdict is written in the rule.
- **(b) A template takes the verdict**: `(open)` and `(open AnyHead)` as productions of `Template`, and the loader keeps
  `validate_open`'s placement lines for what an expansion puts in a `:match`, inside a conclusion or beside one, as
  S1f.5.24 reads every other primitive a macro's expansion writes.

**Defaulted 2026-11-10 (the stage's default): (a).** The ruling is *written only as a rule's conclusion*, and a
macro's body is none; no corpus program writes `open` in a macro (T2's sweep). An unused macro that writes it is
refused too. The ruling on [Q-M1f.67](#q-m1f67--where-is-a-macro-body-that-no-rule-could-use-refused) does not reach
it: that ruling is about a primitive's shape, which only a use decides, and `open` is a word the grammar refuses where
it stands, as it refuses `_` in a body. Re-opening it means (b).

### Q-M1f.87 — The verdict's loader lines once the grammar places `open`

**Raised** 2026-11-10 by [S1f.5.23](p1f.5_the_no_owner_items/s1f.5.23_open_is_reserved.md)'s T3. With `open` no
name, five of `validate_open`'s ten lines are reached by no program: `(open …)` in a `:match`, inside a conclusion,
beside one, with two arguments, and with an argument that is neither a variable nor a name. Five stay: `(open ?R)` whose
`?R` is no parameter, and the projection's four. Each of the ten ended `at None` (F23's D-2), a rule being a form the
parser keeps no position for.

- **(a) Delete the five, and place the five that stay**: S1f.5.5's T5 precedent, each deleted line named in
  `grammar_pages`' `DELETED` and a row of the grammar's table, the loader holding as an invariant that no `(open …)`
  stands but a rule's whole `:assert`; each line that stays starts with where the `(open …)` is written,
  `<file>:<line>:<col>: rule 'r': …`, S1f.5.21's spelling, and drops `at None`.
- **(b) Keep the five as lines no program reaches, and leave `at None` to F23.**

**Defaulted 2026-11-10 (the stage's default): (a).** The stage file asks for the verdict's lines in Ein's words, with
no `at None`; the deleted lines are what the grammar refuses since, and one fixture moves, `open_compound_witness`.
Re-opening it means (b).

### Q-M1f.88 — The line `open` is refused with

**Raised** 2026-11-10 by [S1f.5.23](p1f.5_the_no_owner_items/s1f.5.23_open_is_reserved.md)'s T3. The stage file
sketches one line, *``<file>:<line>:<col>: `open` is the verdict a rule's :assert states, and is not a relation: …``*.
Where the grammar refuses, the line is a parse error's ([`defined_behaviour.md`
§ 1](../../docs/kernel/defined_behaviour.md#1-parse-and-load-diagnostics)): ``<file>:<line>:<col>: unexpected `open` ``,
then the error's notes and repairs.

- **(a) The parse error's line, and the stage's sentence as a note under it**, as S1f.5.1's `_` and S1f.5.2's string
  carry one: *`note: open is reserved: it is the verdict a rule's :assert states, written as the whole :assert, (open)
  or (open ?R), and nowhere else, so it names no relation and no value`* (`ein_ir::grammar::OPEN_NOTE`).
- **(b) A first line of its own**: the parse error's `what` replaced by the sentence wherever the token found is `open`.

**Defaulted 2026-11-10 (the stage's default): (a).** One spelling for a parse error, the precedent of `_` and a
string; the first line names the token and its place, and the note says what it is. Re-opening it means (b).

### Q-M1f.89 — Which of what the refusal replaces is a register row

**Raised** 2026-11-10 by [S1f.5.23](p1f.5_the_no_owner_items/s1f.5.23_open_is_reserved.md)'s T2: fifty-two places a
program can write `open`, of which thirty-eight loaded and were answered.

- **(a) A row per place with a reading of its own** (Q-M1f.36's reading, as Q-M1f.83): W70's seven, `open` as a
  relation (a fact's, a nested fact's, a stored negation's, a goal's pattern's, a claim's fact's, a relation
  parameter's beside a fact of `open`, an obligation's parameter's); W71's fourteen, `open` as a value (a fact's
  argument, an object, a type, a premise's argument, a guard's value, a conclusion's, an hrule's, a derived
  activator's, a goal's, a claim's, a macro call's, a fact's `:rule`, a `(solve …)` list's, a trace step's name); W72's
  three, a macro's body (the verdict called in an `:assert`, a value, a body no rule calls). The places that read as
  one of those do (a nested negation, `:using`, the other claim, solve-list and trace shapes, an `is-a`'s type, a
  value under `absent` or `not`) are the pin's, `ein-cli/tests/open_reserved.rs`, which holds all fifty-two; a
  `--diff-models` file that lists `open` is a run's input and no program, refused at its read since, and no row.
- **(b) A row per place that loaded**, thirty-eight.

**Defaulted 2026-11-10 (the stage's default): (a).** The precedent filed every way, not every program, and the pin
holds every place on every command. Re-opening it means (b).

### Q-M1f.90 — Which of the import read's moves are register rows, and where their witnesses live

**Raised** 2026-11-10 by [S1f.5.22](p1f.5_the_no_owner_items/s1f.5.22_unicode_only_in_a_string.md)'s T2. An imported module that exists and cannot be read as text, four ways, a module that
is not UTF-8 (the close review's finding 2), one that cannot be read, a directory and a `.einb` container, was reported
on every command as `module not found at … (None)`; the coordinator's instruction makes each command run whose line
moves a register row (Q-M1f.36). A register row's witness is a corpus file, or one under `corpus/fuzz_findings/`, and
only one of the four can be one: a `.ein` under `examples/`, `stdlib/` or `tests/` that is not UTF-8 breaks the eleven
test files that read every corpus file as UTF-8 text (`read_to_string(…).expect`), git keeps no mode bit but the
executable one, and a container is binary and moves with the format. A directory named `x.ein` is no corpus file
(`ein_corpus::corpus_files` walks into it). And a register `now` can hold a command's refusal, exit 1, but not `ein
test`'s, exit 2, and a `wrong` cannot record a solve's refusal at load (a solve's recorded refusal is a usage one, exit 2,
schema `/19`).

- **(a) W74's rows on the directory**: `import-module-directory.ein` and its module, a directory holding a README, one row
  per command run the register can run, nine (`saturate --dump`, `oracle`, the six views, `kb save`); `ein test`, the
  `solve` modes and the other three ways, written at run time, are the pin's
  (`ein-cli/tests/unicode_only_in_a_string.rs`). W73, a Unicode space that loaded, one row per space the census found
  (U+00A0, U+3000, U+2028, U+0085) and one for a no-break space inside what is written as one name, by Q-M1f.89's
  reading: the places where a space loads read it as a space in every one.
- **(b) The module that is not UTF-8 banked under `corpus/fuzz_findings/`**, the register's one place outside the corpus,
  with `not_in_corpus`: a row for the finding's own way, at the cost of that directory's steady state, *empty*, a
  finding there being promoted to `examples/` when fixed, which this one never can be.
- **(c) A schema that lets a row write its module at run time** into its scratch, the witness copied beside it.

**Defaulted 2026-11-10 (the stage's default): (a).** It files a row for every way the register can hold, and holds every
way on every command in the pin. Re-opening it means (b) or (c).

### Q-M1f.91 — A module that is a `.einb` container

**Raised** 2026-11-10 by [S1f.5.22](p1f.5_the_no_owner_items/s1f.5.22_unicode_only_in_a_string.md)'s T3. Once a module is read through the program's read, a container is told from text by
its first bytes; the program a command is given may be one, and is opened. `ein-ir`, where the resolver is, cannot open
one: `ein-einb` stands above it.

- **(a) Refused, with a line of its own**: *`<path>: a .einb container, and an imported module is Ein text: import the
  .ein it was saved from`* (`ein_ir::read::container_module_line`).
- **(b) Opened as the program it stores**, the resolver taking a callback from `ein-cli` that opens a container.

**Defaulted 2026-11-10 (the stage's default): (a).** AGENTS.md: *`.einb` is a private cache format, never an interchange
one*; a module is a program's text. Re-opening it means (b).

### Q-M1f.92 — How the line quotes a character a reader cannot see

**Raised** 2026-11-10 by [S1f.5.22](p1f.5_the_no_owner_items/s1f.5.22_unicode_only_in_a_string.md)'s T3. The user's line names the character, `'<char>' (U+XXXX)`. A no-break space, a
zero-width space, a right-to-left override or a byte-order mark written between the quotes shows nothing, or reorders
the line.

- **(a) As the escape a string writes it with**: `'\u{a0}' (U+00A0)`, for a space, a control, a zero-width or
  bidirectional format character, a combining mark or a variation selector (`ein_ir::grammar::shown_char`), and any
  other character as itself, `'é' (U+00E9)`.
- **(b) Always the character itself.**
- **(c) The code point alone**, with no quote.

**Defaulted 2026-11-10 (the stage's default): (a).** The line shows what it names, and one error stays one line, as
`cut_at_40` escapes a control in a token. Re-opening it means (b) or (c).

### Q-M1f.93 — Which path a module's read line names

**Raised** 2026-11-10 by [S1f.5.22](p1f.5_the_no_owner_items/s1f.5.22_unicode_only_in_a_string.md)'s T3. The read's line names the path it was handed. A module has two: the path the import
resolves to, `<the importing file's directory>/<module>.ein`, which `module not found at …` names, and its canonical
path, which its parse errors and its forms' positions name.

- **(a) The path the import resolves to**: relative where the program was given relatively, as `ein solve` run from the
  repository names `examples/…`.
- **(b) The canonical path**, as the module's parse errors.

**Defaulted 2026-11-10 (the stage's default): (a).** The read is of the path the import names, and the line is the one
that path, given to a command, prints. Re-opening it means (b).

### Q-M1f.94 — A directory's and an unreadable module's line, and a missing one's `(None)`

**Raised** 2026-11-10 by [S1f.5.22](p1f.5_the_no_owner_items/s1f.5.22_unicode_only_in_a_string.md)'s T3. The program's read names a directory and an unreadable file in Python's spelling,
`IsADirectoryError: [Errno 21] Is a directory: '<path>'` and `PermissionError: [Errno 13] Permission denied:
'<path>'`, which F23's D-1 owns; the stage file asked for *the OS's reason in Ein's words*. And a module that does not
exist keeps `(import m) — module not found at … (None)`, whose `(None)` is the import's position, a top-level form
carrying none, F23's D-2 with every import line's `at None`.

- **(a) The program's lines, for a module too**, and the missing module's line kept: one read, so F23's change of the
  spelling moves both; and D-2 placed with every other `at None`.
- **(b) Ein's words for both now**, the program's line too, taking that half of D-1; and the import's position placed,
  taking D-2's import lines.

**Defaulted 2026-11-10 (the stage's default): (a).** The user's instruction is *the main file's line*, and D-1 and D-2
are F23's (Q-M1f.75 left both there). No line of the four ways carries `None` or *not found*. Re-opening it means (b).

### Q-M1f.95 — How many characters the refusal names

**Raised** 2026-11-10 by [S1f.5.22](p1f.5_the_no_owner_items/s1f.5.22_unicode_only_in_a_string.md)'s T3. `lrpar` lexes the whole file before it parses, and stops at the first lexing error
(`parse_actions`), so a file is refused at its first character that is not ASCII outside a string and a comment, one
error, where a parse error is followed by every error recovery reaches.

- **(a) The first**, where the lexer stops.
- **(b) Every one**, a scan over the file before the parse, each a line.

**Defaulted 2026-11-10 (the stage's default): (a).** It is where the file stops being Ein, as a character no token
starts with was; the scan would read a string's and a comment's extent a second time. Re-opening it means (b).

### Q-M1f.96 — A byte-order mark

**Raised** 2026-11-10 by [S1f.5.22](p1f.5_the_no_owner_items/s1f.5.22_unicode_only_in_a_string.md)'s T2. A file an editor saved with a byte-order mark starts with U+FEFF, which was a token
that is not a name, `` unexpected `\u{feff}…` ``; no corpus file starts with one.

- **(a) Refused as any other character that is not ASCII**, at 1:1, `'\u{feff}' (U+FEFF) is not ASCII: …`.
- **(b) Skipped where it is the file's first character.**

**Defaulted 2026-11-10 (the stage's default): (a).** The ruling is *anywhere else than inside string literals*, with
comments added by Q-M1f.65 (b). Re-opening it means (b).

### Q-M1f.97 — How a test closes a run's stdout

**Raised** 2026-11-10 by [S1f.5.25](p1f.5_the_no_owner_items/s1f.5.25_a_closed_stdout.md)'s T2. The acceptance reads a run's stdout with *a reader that closes after
its first line*. Whether such a reader meets a write is a race: a run whose writes all land in the pipe before the
reader closes has met no closed pipe, and exits 0 before and after the fix, as `cat` does (T1's `ein solve … | head`,
its read-out one `print!`). A test that took `head -1`'s reader alone would pass or fail by timing.

- **(a) Close it before the run starts** in the register and in the pin's second reader, a pipe whose read end is
  dropped before the spawn, so every write the run makes is one after its reader has gone and each run ends at its
  first write; and keep `head -1`'s reader as the pin's third, on a pipe shrunk to one page (`F_SETPIPE_SZ`, Linux),
  strict where a run's stdout, read in full, is more than the first line and what the pipe holds (it must write after
  the close), and either way where it fits, `SIGPIPE` or exit 0.
- **(b) `head -1`'s reader alone, run until it meets a write.**
- **(c) A run slowed down** so that it writes after the reader closes.

**Defaulted 2026-11-10 (the stage's default): (a).** It decides every run by construction: twenty-one of the
sixty-eight runs fall in the strict arm and forty-seven either way (`cargo test -p ein-cli --test closed_stdout`,
2026-11-10). (b) is a race, and (c) would need a lever in the binary. Re-opening it means (b) or (c).

### Q-M1f.98 — Which of the closed stdout's moves are register rows

**Raised** 2026-11-10 by [S1f.5.25](p1f.5_the_no_owner_items/s1f.5.25_a_closed_stdout.md)'s T2. Its census ran seventy-two runs, every subcommand and every option that
writes to stdout and `kb save`, with the reader closed before each: fifty-five panicked, exit 101; sixteen, `clap`'s
help, exited 0, `clap` ignoring its write's failure; and `kb save`, which writes nothing to stdout, exited 0. Under
`head -1` the 101s were the runs that wrote after `head` exited, and the rest exited 0 (the stage's T2 record).

- **(a) One row per command and per `solve` question, and `solve -c`**, whose write comes before the read-out: eighteen
  rows, W75, on two corpus programs, `examples/branching/05_mini_zebra.ein` (no claim, so no question prints a note)
  and `examples/features/11_expect_ambiguity.ein` for `ein test` (a claim that holds); every other option dies at the
  read-out's write or at one of its own, `--version` has no program, `render region` reads an oracle's directory, and
  an artefact naming `/dev/stdout` writes another file: the pin's. A move from exit 0 to the signal is no row: the
  help is no answer about a program, and a run whose writes all landed before its reader closed exits 0 still.
- **(b) A row per run of the census that panicked**, fifty-five.
- **(c) As (a), and a row for each help** too.

**Defaulted 2026-11-10 (the stage's default): (a).** Every option of a command meets the same `print!` the
command's row does, and the pin holds each; a row per option would repeat one assertion fifty-five times. Re-opening
it means (b) or (c).

### Q-M1f.99 — What the register records of a run a closed stdout ended

**Raised** 2026-11-10 by [S1f.5.25](p1f.5_the_no_owner_items/s1f.5.25_a_closed_stdout.md)'s T2. The register's schema `/20` records an exit code and a line of stderr. A
process `SIGPIPE` ends has no exit code (`ExitStatus::code()` is `None`, `signal()` 13), and a panic's first line
carries a thread id and a source position in the toolchain's `std`, `thread 'main' (3134551) panicked at
library/std/src/io/stdio.rs:1166:9:`.

- **(a) Schema `/21`**: `[case.now]` gains `stdout = "closed"`, the run is a command's whatever its subcommand
  (`Now::is_command`), and its answer is `signal = 13` (nothing on stderr) or `panicked`, the panic's message, the line
  after `panicked at`, exit 101; `[case.right]` gains `signal`, and a recorded answer `stdout` and `panicked`.
- **(b) `exit = 141`**, the shell's report, and `error` with the first line masked.

**Defaulted 2026-11-10 (the stage's default): (a).** 141 is what a shell makes of a signal, not what the process
ended with, and the panic's first line is the toolchain's. `ein-corpus`'s `register` checks the fields
(`a_closed_stdout_ends_a_run_by_a_signal_or_a_panic`), `wrong_answers.rs` runs them, and the review bundle reads and
builds them, its smoke a `W75/solve` case per commit. Re-opening it means (b).

### Q-M1f.100 — An artefact flag that writes to a pipe whose reader has gone

**Raised** 2026-11-10 by [S1f.5.25](p1f.5_the_no_owner_items/s1f.5.25_a_closed_stdout.md)'s T2. [`defined_behaviour.md` § 4.4](../../docs/kernel/defined_behaviour.md#44-an-artefact-flags-failure--m1e-s1e35)
rules that an artefact flag's failed write prints `error: --<flag> <path>: <os error>` and leaves the exit code
unchanged. Its census ran `--events`, `--json-summary`, `--trace` and `--json-report` on `/dev/stdout`: under `head
-1`, `solve --trace /dev/stdout` and `--json-summary /dev/stdout` printed that line and exited 0, the read-out's one
write having landed first. A process-wide `SIGPIPE` reaches every pipe the process writes, the artefact's too, a FIFO
`utils/layer_census.py` reads included.

- **(a) It ends the process by `SIGPIPE` too**; § 4.4's arm holds for every other failure, a directory, a full disk,
  a permission, and § 4.4 and § 4.19 say so.
- **(b) Ignore `SIGPIPE` around an artefact's writes**, the disposition flipped per write.

**Defaulted 2026-11-10 (the stage's default): (a).** It is the ruling's own consequence: the disposition is
process-wide, and flipping it per write is not thread-safe and is more `unsafe` for a case no corpus run reaches.
It narrows a rule § 4.4 stated, so it is shown to the user at the close. Re-opening it means (b).

### Q-M1f.101 — A closed stdout on Windows

**Raised** 2026-11-10 by [S1f.5.25](p1f.5_the_no_owner_items/s1f.5.25_a_closed_stdout.md)'s T2. Windows, a release target, has no `SIGPIPE`: a write to a closed pipe there
is an error, which a `print!` turns into the same panic, exit 101. No host here runs Windows, so no probe shows it,
and the standard of proof refutes a behaviour only with one.

- **(a) Leave it**: `restore_sigpipe` is a no-op off Unix, and the case is shown to the user at the close.
- **(b) A quiet stop on Windows**, Q-M1f.68's (a), every stdout write through one writer there.

**Defaulted 2026-11-10 (the stage's default): (a).** The ruling is about `SIGPIPE`, which Windows does not have, and
(b) is the writer the ruling chose not to build. Re-opening it means (b).

### Q-M1f.102 — What the CI nightly keeps, and what the dev host runs

**Raised** 2026-11-10 by [S1f.5.27](p1f.5_the_no_owner_items/s1f.5.27_the_nightly_reduced.md), on the user's ruling on the close
review's finding 8: *"reduce nightly job, remove long running tests, we will eventually run them on dev host, retain
critical tests in CI"*. The ruling names two kinds of step and leaves their line to be drawn. The nightly took 81 min
54 s here on 2026-11-09, two fuzz seeds 51 min of it, and the latest run on GitHub 74 min 44 s, its fuzz jobs 75 and
59 min, one a seed beside the others (run 42802597417, 2026-11-10).

- **(a) The stage file's rule.** *Critical*: a step that guards an answer or a contract no per-commit step guards (the
  slow entries' answers, the config principle on the slow cells, the cores and dumps of the entries the per-commit tier
  leaves out, the suite's sensitivity). *Long-running*: over 5 min on the runner, or measuring rather than checking. A
  critical step stays in CI at a budget that fits and runs whole on the dev host; the rest run on the dev host only.
  `./run_tests.sh --dev-host` runs every nightly step at full budget and the steps that left, and nothing schedules it.
- **(b) A wall-clock budget for the nightly as a whole**, filled by priority. It needs a priority order the ruling does
  not give, and a step's place would move with the runner's speed, which varies up to 2× between runs (the stage's T1).
- **(c) Keep every step in CI, spread over more parallel jobs.** It keeps the long-running tests in CI, which the
  ruling removes.

**Defaulted 2026-11-10 (the stage's default): (a).** T1 re-took every step here and read three runs on the runner, and
confirmed the stage file's classification row by row, but the fuzzer's budget
([Q-M1f.103](#q-m1f103--the-fuzzers-budget-in-the-ci-nightly)): kept, the slow corpus entries' cells, `answer_dump`,
the round trip's, the levers' and the cores' slow halves, the mutation census and the bench smoke; added, `region`'s and
`obligation_rung`'s slow halves
([Q-M1f.51](#q-m1f51--three-more-tests-whose-slow-half-no-ci-tier-runs)); to the dev host, the id space permuted eight
ways, `stdlib_census.py --check`, the whole review bundle with its link check, and `route_invariance`'s slow half.
Re-opening it means (b), or a line other than 5 min.

### Q-M1f.103 — The fuzzer's budget in the CI nightly

**Raised** 2026-11-10 by [S1f.5.27](p1f.5_the_no_owner_items/s1f.5.27_the_nightly_reduced.md)'s T1. The stage file proposed 100 000
inputs a seed, *about 2.5 min*, from this machine's rate, a million in 1 532.7 s (S1f.5.19's T3 record). The runner's
rate is what the rule reads: a million took 4 410 s and 3 494 s (run 42802597417, 2026-11-10, six properties on four
vCPUs; seven since S1f.5.21), and 2 572 s and 3 020 s the day before (run 42605782048). Here, on four P-core threads,
the runner's count, 20 000 take 106.3 s and 100.1 s, and the properties that parse every input ran 1.13× and 1.34×
slower an input on the runner than here, so on the runner 20 000 come to about 2.7 and 3.3 min with the step's build
of about 50 s, and 100 000 to about 10 and 13 min.

- **(a) 20 000 a seed** in the nightly, ten times the per-commit tier's 2 000 and a fiftieth of the dev host's million.
- **(b) 100 000 a seed**, the stage file's number, past the rule's 5 min on the runner.
- **(c) Make an input cheaper**, so a larger budget fits: S1f.5.28's item 10 and
  [Q-M1f.8](#q-m1f8--the-parsers-remaining-cost-within-grmtools-or-past-it), not this stage's.

**Defaulted 2026-11-10 (the stage's default): (a).** It is the rule's own budget, *a budget that fits*, read on the
runner's rate rather than this machine's. `nightly_steps.rs`' `the_fuzzer_runs_at_each_tiers_budget` holds it between
the per-commit 2 000 and the dev host's million. The first nightly after the push measures it on the runner.
Re-opening it means (b), or (c) first and a larger budget after.

### Q-M1f.104 — A critical step the runner's spread puts on the line

**Raised** 2026-11-10 by [S1f.5.27](p1f.5_the_no_owner_items/s1f.5.27_the_nightly_reduced.md)'s T1. `corpus_cores`' slow half, the
class-4 claim on the 14 `slow` entries, which no per-commit test reaches, took 256 s on the runner on 2026-11-10 (run
42802597417) and 428 s and 436 s on 11-08 and 11-09 (runs 42469293951, 42605782048), whose `deep-corpus` jobs were
slower in their long steps than the latest's (`stdlib_census.py` 642 s against 380 s). The rule keeps a critical step in CI at
a budget that fits; this one's slow half is all or nothing (`corpus_cores.rs`' `slow_too`).

- **(a) Keep it whole in the nightly**, under the line on the latest run, and re-read it when a nightly shows it over.
- **(b) Give the test a budget**, a subset of the slow entries in CI and all of them on the dev host: a new selection
  and a new pin, for a step the latest run puts under the line.
- **(c) Move it to the dev host**, which leaves the claim on the zebra family to no CI tier.

**Defaulted 2026-11-10 (the stage's default): (a).** It is critical and fits on the latest run, (b) is engineering
for a spread, and (c) drops a critical step from CI, which the ruling keeps. Re-opening it means (b).

### Q-M1f.105 — How the dev host's list is held

**Raised** 2026-11-10 by [S1f.5.27](p1f.5_the_no_owner_items/s1f.5.27_the_nightly_reduced.md)'s T2. The stage asks that a test pin the
dev-host list as `nightly_steps.rs` pins the nightly's, and that a step leaving the nightly appear in it.

- **(a) A third copy, held to the nightly's.** `run_tests.sh --dev-host` lists its steps under `dev_host_step "…"`
  banners; `nightly_steps.rs` holds every nightly step to be one of them, in order, at the dev host's fuzz budget, every
  other one to be named in `DEV_HOST_ONLY` with why, and the list to a floor of 17, so a step taken out of the nightly
  and left there fails until named, and one taken out of all three copies fails the floor. It pins the two fuzz
  budgets, which the diff reads past, the checked `TMPDIR`, and every target that reads `EIN_CORPUS_SLOW` to a step on
  one of the two lists (Q-M1f.51's (b)).
- **(b) One copy**, `--dev-host` calling `--nightly`'s steps with a budget variable: the nightly's steps are then not
  the workflow's text, which is what the diff compares, and the moved steps sit outside the shared block.
- **(c) A file of its own**, `dev_host_steps.rs`, with the workflow parse moved to `steps/mod.rs`: the same checks, and
  a restructure of `nightly_steps.rs` for no new property.

**Defaulted 2026-11-10 (the stage's default): (a).** Each step is read from the file that runs it, as `gate_steps.rs`
and `nightly_steps.rs` already do, and every way the copies can drift fails a test (the stage's T2 record checks each
by mutation). Re-opening it means (b) or (c).

### Q-M1f.106 — The whole review bundle on the runner's Graphviz

**Raised** 2026-11-10 by [S1f.5.27](p1f.5_the_no_owner_items/s1f.5.27_the_nightly_reduced.md)'s T1. Since
[S1f.5.29](p1f.5_the_no_owner_items/s1f.5.29_the_review_bundle_smoke_on_the_runner.md) a DOT Graphviz refuses fails the
whole bundle, and the nightly's bundle step was the one CI run that drew all its DOTs on the runner's Graphviz with the
engine each suggests. The rule moves the bundle to the dev host: 372 s on the latest run, 740 s and 752 s on the two
before, and what it guards is the review's pages, not an answer.

- **(a) The dev host draws it**, here or in the runner's image (`utils/ci_container.sh --dev-host`). Per commit the
  runner still lays every view `ein-render` writes out with `dot` (`dot_wellformed`) and the smoke's 31 DOTs with the
  engine each suggests, `fdp`'s KB views and the trace blocks among them, and
  [Q-M1f.77](#q-m1f77--when-is-utilsci_containersh-part-of-done) is unchanged.
- **(b) Widen Q-M1f.77**: a stage that changes a DOT writer or the bundle also builds the whole bundle in the runner's
  image, `utils/ci_container.sh --dev-host` or the bundle alone in its `--shell`.
- **(c) Keep the bundle in the nightly**, past the rule's 5 min.

**Defaulted 2026-11-10 (the stage's default): (a).** The 612 refusals S1f.5.29 found were every `fdp` view, which the
smoke draws, and no view kind the bundle draws is outside `dot_wellformed`'s and the smoke's. Re-opening it means (b).

### Q-M1f.107 — Where the close review's finding 7 is fixed

**Raised** 2026-11-10 by the coordinator, writing [S1f.5.28](p1f.5_the_no_owner_items/s1f.5.28_the_unchecked_findings_triaged.md). The close review's finding 7, the stale slow
counts, drew a question from the user and no ruling (*"so what? what does mean slow here? slow for what? compared to
what?"*); the review answered it: no behaviour is wrong, and two places state a count that drifted and two cite a test
by a name it no longer has, Q-M1e.4's rule broken. The stage file says the placement is recorded as a Q-M1f row, and
none was until S1f.5.28 wrote this one.

- **(a) S1f.5.28's item 11**: `corpus/README.md` § `slow`, the `corpus/corpus.toml` header and `corpus_cli.rs` cite
  `the_slow_set_is_exactly_these_fourteen`, which names the entries per commit, and state no count of their own.
- **(b) A stage of its own**, for three prose edits.
- **(c) Left**, the counts drifting until a docs pass.

**Defaulted 2026-11-10 (the coordinator's default): (a).** S1f.5.28's T3 record has the edits. The user may move it.

### Q-M1f.108 — Route 1's *stopped before layer L*, which no `ein` run prints

**Raised** 2026-11-10 by [S1f.5.28](p1f.5_the_no_owner_items/s1f.5.28_the_unchecked_findings_triaged.md)'s T1 (its item 4, from S1f.5.16). Describe takes route 1 only on a flagged
program whose root is dead: route 1 reads root at layer 0, projected at one fact, which the smallest `--region-budget`
the command takes, 1, admits, and a dead root leaves layer 1 empty, so no `ein` run stops it at the budget. The
read-out's arm for it is reached by `explore::describe_by`, which `route_invariance.rs` calls to hold route 1 and the
state route to one read-out, and which stops route 1 at the budget on a monotone program.

- **(a) Keep the arm**, and say on `read_out.md` why no `ein` run prints it, with a test that the one shape describe
  takes route 1 on exhausts at `--region-budget 1`.
- **(b) Delete the arm**, making it an invariant: `describe_by` would need a text of its own for the stop.
- **(c) Make an `ein` run reach it**, an option that picks route 1, which the module's rule (*no option chooses it*)
  does not have.

**Defaulted 2026-11-10 (the stage's default): (a).** The arm is the library's, and a page that says when a row is
printed is what S1f.5.16's reference is for. Re-opening it means (b) or (c).

### Q-M1f.109 — A text's line break in the trace's markdown

**Raised** 2026-11-10 by [S1f.5.28](p1f.5_the_no_owner_items/s1f.5.28_the_unchecked_findings_triaged.md)'s T1 (its item 5, from S1f.5.16). `--trace` writes markdown, and wrote a
text's `\n` or `\r` as it stands, which markdown reads as a soft break: a rule's `:why` in a quote, a premise's
`:source`, a relation's statement and an owed instance each rendered as one line. Q-M1f.12 holds the trace to *the
characters*, which a soft break keeps and a renderer joins.

- **(a) A hard break**: each line break two spaces and a newline, the line after it led by its block's marker (`> `
  in the quote, the item's indent in a list), as Q-M1f.19 (a) took the read-out's lines to their column.
- **(b) The characters as written**, the soft break documented.
- **(c) A text as a code span or block**, which keeps its lines and stops reading as prose.

**Defaulted 2026-11-10 (the stage's default): (a).** The characters stay, each line of the text a line of the
rendered page; `ein-cli/tests/natural_language_text.rs` holds it. Two `corpus_shapes` digests moved, the region traces
of `tests/text/01_unicode.ein` and `examples/features/15_unicode_escapes.ein`. Shown to the user, since it moves the
bytes Q-M1f.12 holds. Re-opening it means (b) or (c).

### Q-M1f.110 — A container saved again

**Raised** 2026-11-10 by [S1f.5.28](p1f.5_the_no_owner_items/s1f.5.28_the_unchecked_findings_triaged.md)'s T1 (its item 9, from S1f.5.19's record, which recorded it and did not file
it). `ein kb save` of a container answered as the program did, and wrote 40 bytes more a given fact each time: opening
a container pushes the file's provenance records and then the loader's, and where the derived state is kept the
loader's are referenced by nothing, and the writer wrote the whole arena. A saturated `zebra2.einb` saved again passed
design/10 §6's 64 KB.

- **(a) Write up to the last record the KB references**, a believed fact's primary or alternative or a closure's
  arrival: a re-save is byte-identical, and no id moves, so a first save keeps its bytes but where its arena ends on a
  record nothing references (a fact stated twice).
- **(b) Write only the referenced records, ids remapped**: every unreferenced record goes, and a fact stated twice
  moves the ids of the records after its second statement, which `ein-einb`'s round trip compares (`Kb::diff`).
- **(c) Leave it**, a private cache format.

**Defaulted 2026-11-10 (the stage's default): (a).** design/10 §6's size bound is the criterion, and (a) meets it with
no id moved; (b) failed the round trip's id identity on `examples/syntax/unary-and-nary.ein`. Pinned by
`einb_cli.rs`' `a_container_saved_again_is_the_container`. Shown to the user, since S1f.5.19 left it. Re-opening it
means (b).

### Q-M1f.111 — The fuzz test's shape

**Raised** 2026-11-10 by [S1f.5.28](p1f.5_the_no_owner_items/s1f.5.28_the_unchecked_findings_triaged.md)'s item 10, on S1f.5.27's T1: the parse is 99 % of an input's cost, and five
properties of `ein-ir/tests/fuzz_properties.rs` each generated and parsed the whole stream. The recovery search's cost
at its budget is [Q-M1f.8](#q-m1f8--the-parsers-remaining-cost-within-grmtools-or-past-it)'s, parked for the close,
and not this.

- **(a) One parse, shared**: a process-wide cache parses the stream once, on every thread the process may use, and
  keeps each input's verdict by its place, and property 4's sample's reports; each property keeps its test, reads every
  input it read by its place, re-parses only the accepted ones, and reports and minimises its own failure.
- **(b) One test**, every property in one loop: one parse, and one name for seven properties, a failure in one hiding
  the others.
- **(c) Leave it**, five parses.

**Defaulted 2026-11-10 (the stage's default): (a).** At 20 000 inputs, seed 1, one P-core: 197.3 s → 46.1 s; on four
P-core threads, the runner's count: 92.3 s → 23.6 s (seed 1) and 87.4 s → 21.0 s (seed 987654321), every property's
counters the same (`utils/bench_env.sh`, release, 2026-11-10, S1f.5.28's T2 record). Each property still fails on a
mutation of its check, by its iteration and seed. Re-opening it means (b), or the budgets raised.

### Q-M1f.112 — P1f.7 after its analysis: the order, S1f.7.9, and three findings with no stage

**Raised** 2026-11-11 by the coordinator, on the read-only
[analysis of P1f.7's nine findings](p1f.7_findings_outside_scope/README.md#the-analysis-of-2026-11-11), taken before
any stage started. Three things it left to the user:

- whether S1f.7.9's fix stays in the phase or moves to a followup, since refusing a `:rule` that names no rule
  leaves the larger defect, a written provenance that leaves every core;
- whether S1f.7.8 runs first, as the one finding a puzzle's author meets by a misspelt name;
- whether the three findings with no stage (`(q (eq k j))` stored, a goal over a relation nothing makes, a macro
  call written as a fact) become stages.

**Ruled 2026-11-11 (the user):** *"yes S1f.7.9 moves to a new F24; yes the phase order changes to put S1f.7.8
first"*, and *"yes - it become S1f.7.10 onward"*. [F24](../followups/f24_written_provenance.md) is filed, S1f.7.9
files its rows `open` and owned by it, the [order](p1f.7_findings_outside_scope/README.md#the-order) is restated,
and S1f.7.10 to S1f.7.12 are planned.

### Q-M1f.113 — What is a nested `(eq k j)` for: a quoted statement, or nothing a state can hold?

**Raised** 2026-11-11 by the coordinator, on [S1f.7.10](p1f.7_findings_outside_scope/s1f.7.10_an_eq_fact_as_a_value.md)'s
finding and the user's question of that day (what such a fact would be used for, from Ein's point of view).

- **(a) The value rule**: a value is a name, or a fact a state could hold, an atom of an ordinary relation or its
  negation. `eq`, `neq`, `absent`, `false` with an argument, `and` and `or` are never a nested head; `not` stays.
- **(b) Quoting kept**: `eq` and `neq` both allowed as values matched by shape, and the `(not ?s)` conclusion made
  to check a bound `eq`.

**Ruled 2026-11-11 (the user): (a)**, *"(a), adopt the rule"*. The reasons are in
[P1f.7 § The value rule](p1f.7_findings_outside_scope/README.md#the-value-rule). With it the user ruled how P1f.7
treats what it finds outside its scope: fixed in place if quick, or a row of
[F25](../followups/f25_roughness_found_outside_zebra.md), and not shown for review unless it moves an existing
zebra program's answer.

### Q-M1f.114 — What a relation in a `(solve …)` list must be

**Raised** 2026-11-11 by [S1f.7.8](p1f.7_findings_outside_scope/s1f.7.8_a_solve_list_naming_nothing.md)'s T1. The stage refuses a name in `:hypothesis-relations`, in
`:no-hypothesis` or at an activator's relation parameter that names no relation of the program, and has to say
what a relation of the program is.

- **(a) Declared or a fact's**, as a claim's relation is checked (`validate_solve`): it refuses a relation only a
  rule's `:assert` derives, which the phase's analysis names as that check's over-refusal (S1f.7.11).
- **(b) Named elsewhere**: a name the program writes as a relation outside the three lists: declared, a fact's,
  `is-a`, `eq`, `neq`, the relation of a form in a rule's, an hrule's or an obligation's `:match` or `:assert`, in a
  goal or in a fact's argument, or a name passed where a relation's name is held (`(bijective color-loc)`,
  S1f.5.18's positions).
- **(c) Made by something**: declared, a fact's or concluded. It cannot tell `att` from `at` at an activator, since
  the activator's hrule concludes whatever relation it is given, and it refuses a list that names a relation the
  program only reads.

**Defaulted 2026-11-11 (the stage's default): (b).** It refuses every misspelt name the stage's witnesses hold and
no program of the corpus but `examples/syntax/query-full.ein`, whose two hrules were not declared (a sweep of 612
tracked `.ein` files, the binary at `63f1e569` against the stage's, `ein saturate`, 2026-11-11). It can be
narrowed to (c) for the two relation lists once S1f.7.11 has said what *makes* a relation.

### Q-M1f.115 — What else of an `:hrules` list is refused

**Raised** 2026-11-11 by [S1f.7.8](p1f.7_findings_outside_scope/s1f.7.8_a_solve_list_naming_nothing.md)'s T1. Beside the list's first name, an `:hrules` list holds bare names
after it, which the engine reads as nothing (`tests/region/01_states.ein` writes `guess` three times), and
activators, of which one with another number of arguments than its hrule has parameters is skipped in silence, as
is any activator of an hrule without parameters.

- **(a) Refuse both**: every bare name is an hrule the program declares, and an activator has as many arguments as
  its hrule has parameters.
- **(b) Refuse the first name only**, and leave the rest read as nothing.

**Defaulted 2026-11-11 (the stage's default): (a)**, the stage's rule, *every name a list holds names something
the program declares*, and an activator that binds nothing is the same silence (W76's third row). No corpus program
moved. That a bare name selects nothing is F25's R-3.

### Q-M1f.116 — An activator's argument that is a value

**Raised** 2026-11-11 by the [analysis of that day](p1f.7_findings_outside_scope/README.md#the-analysis-of-2026-11-11)
and [S1f.7.8](p1f.7_findings_outside_scope/s1f.7.8_a_solve_list_naming_nothing.md)'s T1: `(guess (at Colour House))`, a misspelt type at a parameter that names no relation,
gives the hrule no match and H is empty.

- **(a) Refuse a value the program names nowhere else.** A value is any name, and an activator may name one first:
  `examples/features/11_expect_ambiguity.ein`'s `(guess (Ann S1) …)` gives objects.
- **(b) Do not check it**, and say in the read-out that an `:hrules` entry proposes nothing at root.

**Defaulted 2026-11-11 (the stage's default): (b)**, as the stage's brief has it. The read-out row is not built
here: it is a new `Row` and a line of `read_out.md`, and is [F25](../followups/f25_roughness_found_outside_zebra.md)'s
R-2.

### Q-M1f.117 — A goal with a macro call: printed as written, or as expanded

**Raised** 2026-11-11 by the [analysis of that day](p1f.7_findings_outside_scope/README.md#the-analysis-of-2026-11-11)
and [S1f.7.5](p1f.7_findings_outside_scope/s1f.7.5_a_macro_called_in_a_goal.md)'s T1. The stage file names two directions, expanding a goal's macro calls as a rule's `:match`'s are, or
refusing a call in a goal, the second a question for the user only if expansion in a goal cannot mean what it means
in a rule. T1 found that it can: a goal is compiled as a rule's `:match` is (`ein_infer::goal_bindings`'s
synthetic rule). What is left open is what a surface prints.

- **(a) As written**: the read-out's `query` row, the summary's `goal` and `ein test`'s report print `(unknown
  (paid ?g))`; the goal is matched on its expansion, and a class and a claim list the expansion's instances.
- **(b) As expanded**: every surface prints `(and (absent (paid ?g)) (absent (not (paid ?g))))`.

**Defaulted 2026-11-11: expansion, and (a).** The author reads back the question as asked, and the row below it,
`as :expect`, shows what the macro means. The cost is one field, `ein_core::Query::goal_written`, set where the
loader expanded a goal. With it the read-out's classes read a goal's atoms as the loader reads them for a claim,
an `(and …)` inside an `(and …)` flattened, since `unknown`'s expansion writes one and a class listing `(and …)` is
one no claim can write.

### Q-M1f.118 — A macro call in a claim

**Raised** 2026-11-11 by [S1f.7.5](p1f.7_findings_outside_scope/s1f.7.5_a_macro_called_in_a_goal.md)'s T1, which the stage file asks to state for each clause of a query that holds a
fact: `:expect (model (settled Ann))` beside a macro `settled`, and the same in a `(solve … :expect (region …))`.

- **(a) Not expanded**: a claim lists facts, a macro is a premise's shorthand, and a fact headed by a macro's name
  is a fact of no relation, refused at load as any claim naming no relation of the program is.
- **(b) Expanded**, so that a claim may be written in the goal's own words. An expansion is a premise, `(and
  (absent …) …)`, which is no fact.

**Defaulted 2026-11-11: (a).** Before the stage the claim was refused with the same line, so nothing moved. A macro
call written as a fact is [S1f.7.12](p1f.7_findings_outside_scope/s1f.7.12_a_macro_call_written_as_a_fact.md)'s.

### Q-M1f.119 — Which relations a goal may ask about

**Raised** 2026-11-11 by [S1f.7.11](p1f.7_findings_outside_scope/s1f.7.11_a_goal_over_a_relation_nothing_makes.md). Its default is (a), refused at load, against (b), a warning row in the read-out.
The stage file words (a) as *no declaration, no fact, and no rule's, hrule's or obligation's conclusion names it*;
the phase's coordinator asked for S1f.7.8's notion of a relation of the program
([Q-M1f.114](#q-m1f114--what-a-relation-in-a-solve--list-must-be) (b)) rather than a second one.

- **(a1) Made by something**: declared, a fact's or concluded.
- **(a2) S1f.7.8's notion without the goals**: a name the program writes as a relation in a declaration, a fact, or
  a rule's, an hrule's or an obligation's `:match` or `:assert`, at any depth, or passes where a relation's name is
  held; and a relation a `(solve :hrules …)` activator gives its hrule at a parameter that names one, which that
  hrule proposes facts of.

**Defaulted 2026-11-11: (a), as (a2).** One notion serves the lists, a goal and a claim, and the claim check's
over-refusal of a relation only a rule's `:assert` derives is fixed by reading the same list. A goal's own relations
do not count for a goal or a claim, and still count for the lists, where a relation named only by a goal and by an
activator is what the activator makes. Under (a2) a goal over a relation only a rule's `:match` reads loads and
has no row; (a1) would refuse it, and would refuse an importer's goal over a relation a library rule reads. The
relation of a pattern inside a pattern, and of a stored negation, is held to the same list. No corpus program moved
(a sweep of 612 tracked `.ein` files, the binary at `73a0c17d` against the stage's, `ein saturate` and `ein solve`,
2026-11-11).

### Q-M1f.120 — A rule's `:match` over a relation nothing makes

**Raised** 2026-11-11 by [S1f.7.11](p1f.7_findings_outside_scope/s1f.7.11_a_goal_over_a_relation_nothing_makes.md)'s stage file: a rule whose `:match` reads a relation no declaration, fact or
conclusion makes can never fire.

- **(a) Refused at load**, as the goal is.
- **(b) Reported and not refused**: a library rule may read a relation its importer supplies, and the stdlib's
  rules read marker relations, `functional`, `total`, whose absence means the rule does not fire.

**Defaulted 2026-11-11 (the stage's default): (b).** It is reported in the stage's T1 record and is
[F25](../followups/f25_roughness_found_outside_zebra.md)'s R-6; no surface says that a rule can never fire.

### Q-M1f.121 — The line a goal's nested relation variable is refused with

**Raised** 2026-11-11 by [S1f.7.1](p1f.7_findings_outside_scope/s1f.7.1_a_goals_variable_relation_in_a_nested_pattern.md), whose acceptance says *refused on every command with the rule's line*. A rule's
line, `rule 'r': its :match writes (?R ?x) with ?R as its relation, which no parameter binds — a relation is named
by a name or by a parameter its activator binds`, speaks of a parameter, and a query has none. A goal's top-level
relation variable has had its own line since S1f.5.21 (W64): *which nothing binds — a relation is named by a name,
or by a rule's parameter its activator binds, and a query has no activator*.

- **(a) The goal's line**, at every depth.
- **(b) The rule's line**, word for word.

**Defaulted 2026-11-11: (a).** One shape in one clause reads one way at every depth. The walk is the rule's
(`refuse_no_fact_holds`), which reads a goal since the stage, so a goal has no walk of its own; the giving of a
meaning to the shape, a nested pattern whose relation the match binds, would reverse the W24 ruling and is not
taken.

### Q-M1f.122 — Where `absent` still heads a form that is no premise

**Raised** 2026-11-11 by [S1f.7.7](p1f.7_findings_outside_scope/s1f.7.7_an_absent_fact.md)'s T1. The stage's direction is that `FactHead`, `ArgHead` and `ConclusionHead` lose
`absent`. Its sweep found one more place the word heads a form: a claim. A query's claim lists its projection
classes, and the class of a goal's `(absent (q ?y))` is written `(absent (q a))`, as
`examples/ein-bugs/wrong-answers/goal-absent.ein` and `goal-macro-unknown.ein` write it and as the read-out's
`as :expect` row prints it.

- **(a) A claim keeps it**: `ClaimFact` takes `(absent <fact>)`, a class atom, which is no fact and no value.
- **(b) A claim loses it too**, and a class over an `absent` cannot be claimed.

**Defaulted 2026-11-11: (a).** (b) would take away what M1e2's classes and S1f.7.5's read-out give, and move two
register witnesses. The refusal everywhere else is the grammar's, and its error carries a note for the author who
meant negation: *the stored negation of a fact is (not …)*.

### Q-M1f.123 — `(false)` with no argument as a value

**Raised** 2026-11-11 by [S1f.7.6](p1f.7_findings_outside_scope/s1f.7.6_a_false_fact_with_an_argument.md)'s T1. `(q (false))`, `(not (false))` and a premise `(false)` load today. The value
rule names *`false` with an argument*.

- **(a) `(false)` stays** wherever it is written today, and only an argument is refused.
- **(b) `false` heads no value at all**, with or without an argument.

**Defaulted 2026-11-11: (a).** The rule's own words, and `(false)` is a fact a state could hold: a dead state holds
it. What `(not (false))` and `(q (false))` mean is [F25](../followups/f25_roughness_found_outside_zebra.md)'s R-7.

### Q-M1f.124 — Where `(eq a b)` is still written, once it is no value

**Raised** 2026-11-11 by [S1f.7.10](p1f.7_findings_outside_scope/s1f.7.10_an_eq_fact_as_a_value.md)'s T1. S1f.5.7 has a written `(eq a b)` and `(not (eq a b))`, a premise and a
conclusion of either, and a claim's, each checked. The ruling (a) refuses `eq` as a nested head. Three places sit
between the two: `(q (not (eq a b)))`, a fact's `:using ((eq a b))`, and a trace's `:derives (eq a b)`.

- **(a) Refused in all three**: `(not (eq a b))` is the negation of no fact a state could hold, so it is no value;
  `:using` lists facts a derivation rested on and a trace's step derives a fact, and no state stores an equality.
- **(b) Kept in `:using` and a trace**, which are provenance and not values.

**Defaulted 2026-11-11: (a).** No corpus program writes any of the three (the sweep of T3). `(eq a b)` and `(not
(eq a b))` stay where S1f.5.7 checks them (`ein.y`'s `EqFact` and `Negated`).

### Q-M1f.125 — A connective as a pattern's argument: the grammar or the loader

**Raised** 2026-11-11 by [S1f.7.3](p1f.7_findings_outside_scope/s1f.7.3_a_connective_as_a_patterns_argument.md)'s stage file: *if no macro call needs it, the grammar refuses the written shape at
the parse*. T1 read why `ein.y`'s `Arg` takes a `Connective`: a macro is called as a pattern, and `std.macro`'s
`forall` takes premises, `(forall ?b (and …) (B))`. A context-free grammar cannot tell a macro's name from a
relation's.

- **(a) The loader**, after macro expansion, by the walk S1f.5.24 gave a rule's clauses.
- **(b) The grammar**, with a macro's call given a syntax of its own.

**Defaulted 2026-11-11: (a).** (b) is a language change. The refused heads are what no value has: `and`, `or`,
`neq`, with S1f.5.24's line, and `eq`, `absent` and `false` with an argument, which S1f.7.10, S1f.7.7 and S1f.7.6
left no fact able to hold. `(not <pattern>)` stays: a stored negation is a value and is matched.

### Q-M1f.126 — A relation parameter bound to `absent`, `false` or `eq`

**Raised** 2026-11-11 by the three stages' T1: with `(mk absent)` and `(rule mk (?R) :match (p ?x) :assert (?R
?x))` the rule stored `(absent a)`; with `(mk false)`, `(false a)`; and with `(mk eq)` and `:assert (q (?R ?x
?x))`, `(q (eq a a))`. The grammar reads a variable where the relation is written.

- **(a) Refused at load where the loader can see the binding**, as S1f.5.9 refuses a parameter that can hold `eq`
  heading a form of another arity (`defined_behaviour.md` § 4.14).
- **(b) A row of F25**, the firing storing what it is given.

**Defaulted 2026-11-11: (a), fixed in place.** The check of § 4.14 runs once per word, and reads a parameter that
heads a form where a value goes too.

### Q-M1f.127 — What the loader reads to see that a negated variable can hold a name

**Raised** 2026-11-11 by [S1f.7.4](p1f.7_findings_outside_scope/s1f.7.4_the_negation_of_a_name_concluded.md)'s T1. The stage's default is a refusal at
load *wherever the program shows that `?x` can bind a name*, and its condition was that the census find no program
in which a variable binds both a name and a fact. The census found none: the corpus concludes `(not ?h)` in three
programs, `examples/saturation/hypothesis-contradiction/`, each binding `?h` to facts through the undeclared
`hypothesis`. What counts as *the program shows* is open.

- **(a) The program's own text**: a written fact at any depth, a declaration, a rule's or an hrule's conclusion to
  a fixpoint, an activator (a fact, or `(solve :hrules …)`), a declared relation the hypotheses are taken from, and
  a guard that alone binds the variable. A variable can hold a name where one disjunct binds it only at such
  positions. A position that holds a name and a fact is refused. A pattern or a conclusion whose relation is a
  parameter, `(?R ?x)`, is read as one of each relation a written activator names there, and is not read where the
  activator is derived through a variable.
- **(b) Written facts and activators only**, and everything a commitment or a rule adds left to the firing.
- **(c) Every declared relation's argument**, whatever the `(solve …)` lists say.

**Defaulted 2026-11-11: (a).** (b) loads a program whose every state past root would negate a name, and a state
file of such a state, which writes the commitment as a fact, would then be refused on reload: `(relation seen
Guest)` with a rule `(seen ?g)` to `(not ?g)` and no fact of `seen` (T1's probe). (c) refuses a rule over a declared
relation that holds facts and is held out of the hypotheses, which no commitment can give a name. Under (a) a
declared relation is read where no hrule is declared, the lists leave it in and nothing writes the closure marker,
which is where `ein_infer::hypgen::eligible_relations` has the blind enumerator propose its facts. One test's
program moved: `value_rule.rs`'s `the-negation-of-a-bound-fact` holds its declared `h` out of the hypotheses.

### Q-M1f.128 — What a firing does with the negation of a name the loader did not foresee

**Raised** 2026-11-11 by [S1f.7.4](p1f.7_findings_outside_scope/s1f.7.4_the_negation_of_a_name_concluded.md)'s T1, and
[F25](../followups/f25_roughness_found_outside_zebra.md)'s R-1 since the user's instruction of that day. Under
Q-M1f.127 (a) a pattern whose relation is a parameter with a derived activator is not read, so `(rule deny (?R)
:match (?R ?g) :assert (not ?g))` beside `(marks paid)` and a rule concluding `(deny ?R)` of what is marked loads,
and its firing meets `?g = Ann`.

- **(a) No fact**: the firing stores nothing for that conclusion and stores its others; an hrule proposes nothing.
- **(b) `(false)`**, as a false `(eq …)` conclusion is (S1f.5.7).
- **(c) A stop** with Ein's line.

**Defaulted 2026-11-11: (a).** The value rule says `not` of a name is the negation of no fact, so there is nothing
to conclude; a true `eq` conclusion adds nothing in the same way. (b) kills a state for a conclusion that states
nothing, and the coordinator's brief for the stage ruled it out; (c) breaks *exit non-zero only on error* for a
binding only some state meets. One place builds every conclusion, `ein_infer::firing::build_fact`, so (a) holds on
every command and in both builds, and the sweep of T3 moved no corpus answer. R-1 records what is left.

### Q-M1f.129 — The assertion that no hrule reads `not`: removed, or kept where the kill cache is on

**Raised** 2026-11-11 by [S1f.7.2](p1f.7_findings_outside_scope/s1f.7.2_an_hrule_that_reads_not.md)'s T1. The premise of `Hrules::candidates`' `debug_assert!` is
stale for every command: H is taken config-free since M1e2 S1e2.3.1, the lookahead's kill cache off. The
config-reading `hypgen::generate` can have it on, and the shape digest's `hyp` op and one test call it.

- **(a) Removed**, and the enumeration made right with the cache on: where an hrule reads `not`, every hrule is
  matched before any candidate is emitted.
- **(b) Kept**, scoped to a call with the kill cache on.
- **(c) A refusal at load** of an hrule that reads `not`.

**Defaulted 2026-11-11: (a).** (b) keeps a debug build stopping on the `hyp` op of a corpus program, and the
witnesses are corpus programs whose shape digest `cargo test` takes. (c) refuses what release answers right, as
`ein oracle` does. A second question the stage met: `wrong_answers.rs` runs with debug assertions, where the
unchanged engine's run ends by a panic whose first line carries a thread id, so no `open` row can hold it, and the
register reads `debug_panic` in no test since M1e2 S1e2.2.16. W85's three rows are filed `fixed` in the fixing
commit, their `wrong` recording the release answer and the debug build's stop, and the stage's T2 record says so.

### Q-M1f.130 — A macro's call as a conclusion: refused, or expanded as it is

**Raised** 2026-11-11 by [S1f.7.12](p1f.7_findings_outside_scope/s1f.7.12_a_macro_call_written_as_a_fact.md)'s T1. The stage file lists a rule's and an hrule's `:assert` among the sites where a
macro's name heads a form that is not expanded, and its default (a) refuses a conclusion headed by one. T1 found the
opposite: the loader runs the expander over `:assert` as over `:match` (`from_ir`'s `expand_pair`), so `:assert (pp
?x)` beside `(macro pp (?a) (p ?a))` stores `(p k :rule r)`, and `:assert (s (pp ?x))` stores `(s (p k))`. Ten corpus
witnesses call a macro in an `:assert` (eight `macro-*.ein`, `not-name-macro.ein` and `open-macro-verdict.ein` under
`examples/ein-bugs/wrong-answers/`), and each one's recorded refusal reads the expansion.

- **(a) Expanded, as today.** The stage refuses the call where nothing expands it: a fact and a claim.
- **(b) Refused**: a macro's call is a premise only. It moves the ten witnesses' lines and S1f.5.24's reading of a
  conclusion's expansion.

**Defaulted 2026-11-11: (a).** The finding is a call that is not expanded, and a conclusion's is. (b) would undo
what S1f.5.24 and S1f.7.4 hold a conclusion's expansion to, on the user's ruling that a macro's body is checked where
a rule is. The refusal's line says where a macro is called: *in a :match, an :assert or a :goal*.

### Q-M1f.131 — A relation declared under a macro's name

**Raised** 2026-11-11 by [S1f.7.12](p1f.7_findings_outside_scope/s1f.7.12_a_macro_call_written_as_a_fact.md)'s T1. `(relation pp T)` beside `(macro pp (?a) (p ?a))` loads. A premise `(pp ?x)` is
the macro's call, so no premise reads the relation, and since the stage no fact is of it. No corpus program declares
a relation and a macro of one name (`git ls-files '*.ein'`, 651 files, the stage's census, 2026-11-11).

- **(a) Refused at load**, one line at the declaration.
- **(b) Left**: the declaration loads, and each fact of the relation is refused.

**Defaulted 2026-11-11: (a).** A relation nothing can hold or read is a mistake in the name, and the declaration is
where the author reads about it. A rule and a macro of one name still load: the rule's activator fact is refused as
the macro's call ([F25](../followups/f25_roughness_found_outside_zebra.md)'s R-8).

### Q-M1f.132 — A `std.macro` macro the program does not import, written as a fact

**Raised** 2026-11-11 by [S1f.7.12](p1f.7_findings_outside_scope/s1f.7.12_a_macro_call_written_as_a_fact.md)'s T1. `(unknown (p j))` in a program that does not import `std.macro` is stored as a
fact of a relation `unknown`. A rule's `:match` that writes `(unknown …)` without the import has been refused since
S1.8a.f20, and a goal's since S1f.7.5, so no premise of the program can read that fact.

- **(a) Refused**, the line saying the macro is `std.macro`'s and not imported.
- **(b) Stored**: without the import `unknown` is an ordinary name.

**Defaulted 2026-11-11: (a).** The guard already makes `unknown` and `forall` names a premise cannot use as a
relation, so a fact of either is read by nothing. No corpus program writes one. A `(relation unknown …)` declaration
in such a program is not refused where it stands (F25's R-8).

### Q-M1f.133 — A macro's call inside a fact a claim lists, and in a fact's `:using`

**Raised** 2026-11-11 by [S1f.7.12](p1f.7_findings_outside_scope/s1f.7.12_a_macro_call_written_as_a_fact.md)'s T1. Q-M1f.118 has a claim headed by a macro's call refused: it names no relation.
A call inside a listed fact, `(model (q (pp j)))`, loaded, and the claim failed against a state holding `(q (p j))`.
A fact's `:using ((pp j))` loaded too.

- **(a) Refused as a written fact's is**: a claim lists facts, and a `:using` names facts.
- **(b) Left to the claim's check**, which fails it at the run.

**Defaulted 2026-11-11: (a).** A claim that can never hold is refused at load elsewhere (`eq` in a region claim,
S1f.5.7). The claim's line is added only where nothing else refused the program, so Q-M1f.118's line stays the one
a claim headed by the call prints.

### Q-M1f.134 — What a register row records as `right` while F24's question is not ruled

**Raised** 2026-11-11 by [S1f.7.9](p1f.7_findings_outside_scope/s1f.7.9_a_facts_rule_that_names_no_rule.md)'s T2. The stage files F24's three table rows `open`, owned by F24, and the right
answer of each depends on F24's question, (a) refused, (b) checked, (c) as given and marked, which is the user's
and is not ruled.

- **(a) One named reading, the others as alternatives**, the review `deferred`: how W8's rows record Q-M1e2.2 for
  F21 (`[case.right]` with a `reading`, `[[case.right.alternative]]` per other reading, class *contested*).
- **(b) No `right`**: the schema requires one.

**Defaulted 2026-11-11: (a).** `W87/written-rule-no-such-rule` states (b)'s refusal, which was S1f.7.9's own first
direction, with (a) and (c) as alternatives. The two core rows state (c)'s core, `(paid Ann)`, with (a)'s and (b)'s
refusals as alternatives: a row whose `right` is a refusal may not hold today's core beside it while a live row can
show it (`ein-corpus`'s `beside_a_refusal_now_records_what_today_reports`), and the core is what F24's table says
is wrong. The choice of reading rules nothing: F24's question stays the user's.

### Q-M1f.135 — Which facts a flat import brings

**Raised** 2026-11-11 by [design.md § G](p1f.6_calculator/design.md#g-the-flat-import-finding-a-specification-for-importing-facts), point (a), and taken by [S1f.6.11](p1f.6_calculator/s1f.6.11_importing_facts.md). The user ruled that day that *"importing facts is
mandatory"*; a fact has no name to list, so which facts `(import M :symbols (…))` brings is a choice.

- **(a1) Every fact of the module, always.**
- **(a2) The facts whose relation is in the selection's closure.**
- **(a3) a2, and a listed name may be a relation the module only states facts of.**
- **(a4) A new form for the whole module, flat**, or the bare form redefined: a grammar change, or six corpus
  imports moved.

**Defaulted 2026-11-11: (a3)**, the design's recommendation. A selection stays a selection, nothing comes
unasked, and everything is reachable. `(import std.arith :symbols (arith))` keeps working, the closing rule
naming every relation. Held by `imports_semantics.rs`, W88's four rows and `defined_behaviour.md` § 4.25.

### Q-M1f.136 — What a qualified import does to the names inside a fact

**Raised** 2026-11-11 by [design.md § G](p1f.6_calculator/design.md#g-the-flat-import-finding-a-specification-for-importing-facts), point (b). A qualified import renames every name the module declares, the relation
heading a fact among them, so a library's `is-a` facts land in `o.is-a` and do not join the program's `is-a`.

- **(b1) Keep it, and say nothing.**
- **(b2) Never rename a relation that heads a fact**: the module's own rules, renamed, would stop reading them.
- **(b3) Keep it, and say on the page that a library of facts is imported flat.**

**Defaulted 2026-11-11: (b3).** Renaming is what a qualified import is. `01_grammar.md` § Import says it.

### Q-M1f.137 — Are an imported fact's constants names of the program?

**Raised** 2026-11-11 by [design.md § G](p1f.6_calculator/design.md#g-the-flat-import-finding-a-specification-for-importing-facts), point (c). They were under a qualified import, and they enter the blind
enumerator's objects: importing an ontology of n constants widens the universe of every open relation by n.

- **(a) Yes**: an imported fact is a fact of the program, and `solution_semantics.md` § 1.2 defines the names as
  the constants the program's facts and rules write.
- **(b) Mark library constants**, so the enumerator skips them.

**Defaulted 2026-11-11: (a).** A program that wants no guesses about a library's relation closes it, or the
library closes its own.

### Q-M1f.138 — The provenance of an imported fact

**Raised** 2026-11-11 by [design.md § G](p1f.6_calculator/design.md#g-the-flat-import-finding-a-specification-for-importing-facts), point (d).

- **(a) Its own**: a `:source` where the module wrote one, otherwise background, as if written in the program.
- **(b) Stamped with the module's name** where it has no `:source`.

**Defaulted 2026-11-11: (a).** What a written provenance may claim is
[F24](../followups/f24_written_provenance.md)'s, and a stamp is the same question.

### Q-M1f.139 — The same fact from two places

**Raised** 2026-11-11 by [design.md § G](p1f.6_calculator/design.md#g-the-flat-import-finding-a-specification-for-importing-facts), point (e); E-6 of [`findings.md`](p1f.6_calculator/findings.md). A fact stated by
a module and by the program, or by two modules, with different `:source`s keeps its first statement, and the
later `:source` is gone with no word, as it is for a fact written twice in one file.

- **(a) State the behaviour, and hand the question to F24.**
- **(b) Refuse, or warn, here.**

**Defaulted 2026-11-11: (a).** The stage does not make it worse or better: `defined_behaviour.md` § 4.25 and
`01_grammar.md` § Import state it, and E-6 names F24. Identical facts are one fact (Q-M1f.143).

### Q-M1f.140 — A rule of the module the selection does not name

**Raised** 2026-11-11 by [design.md § G](p1f.6_calculator/design.md#g-the-flat-import-finding-a-specification-for-importing-facts), point (f). `(import onto :symbols (is-a))` brings the `is-a` facts and neither of
the module's rules over `is-a`, though the facts would fire them.

- **(a) Still dropped.**
- **(b) Brought by the facts it reads.**

**Defaulted 2026-11-11: (a).** A selection is a selection; one name that brings a theory is a rule that names
its rules, as the calculator's closing rule does.

### Q-M1f.141 — Does a selection's closure follow the names inside a kept fact?

**Raised** 2026-11-11 by [S1f.6.11](p1f.6_calculator/s1f.6.11_importing_facts.md)'s T3. The closure followed the names a kept *declaration* writes. Once facts are
kept, a kept fact writes names too: `(owns Ann cat)`, where `cat` is a relation the module declares and states
facts of.

- **(a) No**: declarations only, as before the stage. `(owns Ann cat)` brings neither `cat`'s declaration nor
  its facts.
- **(b) Yes**: every name a kept form writes is followed, so a constant that happens to be a declared name
  brings a declaration nobody asked for, and a fact's `:rule` brings the rule.

**Defaulted 2026-11-11: (a).** Point (a)'s default says *the closure follows names as today*, and nothing comes
unasked. Held by `a_selection_follows_declarations_and_not_the_names_inside_a_fact`.

### Q-M1f.142 — A stored negation, and a fact of a kernel relation, under a selection

**Raised** 2026-11-11 by [S1f.6.11](p1f.6_calculator/s1f.6.11_importing_facts.md)'s T3. `(not (likes Milk Tea))` is headed `not`, and `(__closed__ is-a)` and a
written `(eq a b)` are headed by reserved names.

- **(a) A negation is a fact of the relation it negates, and a kernel relation's fact comes as any fact does**:
  when its relation's name is in the closure, listed (`:symbols (is-a __closed__)`) or written by a kept
  declaration (a rule that asserts `(__closed__ …)`).
- **(b) A kernel relation's fact is never brought**, or `(__closed__ r)` comes with `r`.

**Defaulted 2026-11-11: (a).** One rule and no special case. Its cost: listing `__closed__` brings every
`(__closed__ …)` fact of the module, the ones about relations the selection left behind too, and listing `is-a`
alone leaves `(__closed__ is-a)` behind. A library that closes its own relations for a flat importer has the
marker listed or writes it in a rule the selection names, as the calculator's closing rule does.

### Q-M1f.143 — A fact that arrives twice through flat imports

**Raised** 2026-11-11 by [S1f.6.11](p1f.6_calculator/s1f.6.11_importing_facts.md)'s T3 and T4. A flat diamond over declarations collapses, identical ones being one.
A fact has no name to collapse on, so a library reached by two flat paths put each of its facts in the form
list twice. The KB is a set, so no answer differs.

- **(a) One statement, kept where it first arrived**: a module is parsed once per resolution, so the second
  arrival is the same node, and `dedup_declarations` drops it.
- **(b) Leave both for the loader.**

**Defaulted 2026-11-11: (a).** The calculator's tables are over a thousand rows, and every module that imports
them would hand them to the loader again. It also drops the second copy of an undeclared relation's fact under
two qualified imports of one module, which the loader folded already. A fact the program writes itself beside
the library's is another statement and is left to the loader (Q-M1f.139).

### Q-M1f.144 — What W88's rows record

**Raised** 2026-11-11 by [S1f.6.11](p1f.6_calculator/s1f.6.11_importing_facts.md)'s T2. The witnesses have no candidate, so R is {∅} before and after, and a
state's `(model …)` names the candidates' relations: what the import drops is root's facts.

- **(a) The sentences the query's `:goal-text` prints**, one per row of the goal (schema /15, W37's), and for
  the name that could not be listed, the refusal.
- **(b) A new field for root's facts**: a schema change.
- **(c) A claim**: a witness's `:expect` states today's answer until the fix, so the row would read *held*
  before and after.

**Defaulted 2026-11-11: (a).** No schema change, as the stage file expected.

### Q-M1f.145 — How the table generator's check is in the gate

**Raised** 2026-11-11 by [S1f.6.2](p1f.6_calculator/s1f.6.2_digits_and_tables.md)'s T1. The user ruled the tables generated, with a `--check` in the gate
(`design.md`'s question 7, (a)). *In the gate* has two shapes in this repository.

- **(a) A cargo test runs the script's `--check`**, as `ein-cli`'s `cli_semantics.rs` runs
  `examples/gen_zebra2_variants.py --check`. `cargo test --workspace` is a gate step, and the per-commit job
  installs a `python3` for that test already.
- **(b) A step of its own** in `run_tests.sh`, `per-commit.yml` and `gate_steps.rs`, beside
  `utils/stdlib_manifest.py`: a seventh static check, and the header's *six* in three places.

**Defaulted 2026-11-11: (a).** The precedent, and the same test file holds every row to Rust's integers, so the
generator and its oracle fail together. A machine without `python3` fails the test and says why.

### Q-M1f.146 — How `std.digits` closes its tables for a flat importer

**Raised** 2026-11-11 by [S1f.6.2](p1f.6_calculator/s1f.6.2_digits_and_tables.md)'s T2. The stage file says the module closes its relations with written
`(__closed__ R)` facts. By Q-M1f.142 such a fact comes with a selection only when `__closed__` is in the
closure, so `(import std.digits :symbols (fa nz adj))` leaves `nz` and `adj` open, and `ein solve` stops at
*a state's commitments pass 2^128*.

- **(a) Written facts, and the importer lists `__closed__`**: `(import std.digits :symbols (fa nz
  __closed__))`. All seven closing facts come, the ones about tables left behind harmless. A theory that reads
  the tables closes the ones it reads in its own closing rule, so its importer lists one name.
- **(b) A closing rule in the module**, `(rule digits () :match () :assert (and (__closed__ fa) …))`, listed by
  the importer: one name brings every table and its rows, wanted or not, and the module is no longer facts
  alone.
- **(c) The engine brings `(__closed__ r)` with `r`**: Q-M1f.142's (b), an engine change.

**Defaulted 2026-11-11: (a).** The stage file's wording, the module stays a library of facts, and an importer
loads only the tables it lists. Its cost is one more name in a direct import, stated in the module's header,
on `stdlib/README.md` and in `tests/stdlib/digits/01_tables.ein`, and a forgotten `__closed__` is not refused:
it is searched. (c) would remove the cost and is the user's.

### Q-M1f.147 — The DOT sweep and a view no layout finishes

**Raised** 2026-11-11 by [S1f.6.2](p1f.6_calculator/s1f.6.2_digits_and_tables.md)'s T4. `ein-render/tests/dot_wellformed.rs` puts every view of every corpus file
through `dot -Tsvg`, per commit. Every view of `stdlib/digits.ein` has some 7 900 edges onto eleven shared
names: `dot` 16.0.0 had not drawn one after five minutes, `fdp` not after 100 s, `sfdp` dumps core
(2026-11-11). The gate did not finish. E-10 of [`findings.md`](p1f.6_calculator/findings.md) is the finding.

- **(a) Past a bound, another layout:** a view of more than 1 500 edges goes to Graphviz under `fdp`, the
  layout a KB view suggests, stopped after one iteration. The same parser, renderer and warnings; about two
  seconds on the largest view. Not held: `dot`'s own layout of such a view.
- **(b) `dot` with its effort cut** (`nslimit`, `mclimit`, no splines): 6.5 s a view on the largest, 3.4 s on
  `md` alone, and every later program that imports `md` pays it six times.
- **(c) Leave such a view out of the sweep,** by size or by name: the test's own header refuses a silent skip.
- **(d) Keep a table out of the corpus's files:** not possible, the module is one.
- **(e) Change what a KB view draws for a library of facts:** a DOT writer's change, an engine stage.

**Defaulted 2026-11-11: (a), for the user to confirm.** No view the corpus had before is past the bound (the
largest has 361 edges), so nothing that was checked by `dot` stopped being. The sweep takes 171.9 s where it
took 135.9 s. Graphviz's version is part of what that test checks (S1f.5.17, S1f.5.29), so the bounded layout
was tried on the runner image's Graphviz too, 14.1.2 in its x86-64-v3 build: the module's KB view and its
program view in 2.1 s each and `md`'s KB view in 0.8 s, none with a warning (`dot -Kfdp -Gmaxiter=1
-Goverlap=true -Gsplines=false -Tsvg` in `ein-ci:ubuntu-26.04-amd64v3`, 2026-11-11). (e) is the real answer to
*a picture of a table* and is not this stage's.

**Overruled 2026-11-12, and built** ([S1f.6.21](p1f.6_calculator/s1f.6.21_the_gate_reduced.md)). The user,
on reply 45 of the calculator's `questions.md`: *"don't test graphviz itself - that is the main point here. just
skip big graphs, run tests only on small"*. It is (c), by size: a view of more than 1 000 edges is counted,
named in the test's output and put to Graphviz under no layout. Nothing checks the DOT text of such a view.

### Q-M1f.148 — One module for the seven tables

**Raised** 2026-11-11 by [S1f.6.2](p1f.6_calculator/s1f.6.2_digits_and_tables.md)'s T4. A flat import of one table parses the whole module: 28.8 KB, about 12 ms on
the generated parser: `ein test` of a program that imports `fa` alone takes 17.9 ms, and of a `std.macro` test
5.8 ms (best of ten, release, 2026-11-11).

- **(a) One module,** `stdlib/digits.ein`, as the stage file says.
- **(b) A module per table,** `std.digits.fa` and so on: a program that adds parses 200 rows, not 1 664.

**Defaulted 2026-11-11: (a).** The stage file's, and one name for the ontology. The cost is per program, not
per request: a program that does not import the module pays nothing. (b) stays open if a suite of many small
arithmetic programs makes the 12 ms matter.

### Q-M1f.149 — What `(let NAME NUMBER)` does, as built

**Raised** 2026-11-11 by [S1f.6.3](p1f.6_calculator/s1f.6.3_addition.md)'s T2. The user ruled *"(let NAME NUMBER) is ok"*; what a request may do with a name
is item 2 of [`design.md` § what is open](p1f.6_calculator/design.md#what-is-open-after-the-rulings), the
coordinator's reading, the user's to overrule.

- **(a) A request may name an operand,** and its answer repeats the names: `(add price qty)` is answered by
  `(sum-of price qty S)`. The same sum by numbers is another request with its own answer.
- **(b) A name is only a label,** and requests take numbers alone.

**Defaulted 2026-11-11: (a), built as an operand lookup.** Every operand of a request goes through
`(operand TAG X)` and comes back as `(operand-is TAG NUMBER)`: a number stands for itself, a name for the number
its `let` states. What that settles and costs:

- four firings a request, numbers too: 21 for 123078 + 4567;
- a name no `let` states is never answered, with no word; a name stands for a number, not for another name;
  both are S1f.6.12's to flag or not;
- the lookup is the one rule of the theory with a second premise over a program's facts, and it asks
  `(named ?x)` first, a premise with every slot bound, so a number operand costs one lookup however many `let`s
  there are. The stage's T2 record has both measurements.

### Q-M1f.150 — The counter, and how the measured test is in the gate

**Raised** 2026-11-11 by [S1f.6.3](p1f.6_calculator/s1f.6.3_addition.md)'s T4, which asks for *a deterministic count of the matcher's work per request (a
counter of `ein_core::counters`, which the stage picks and names, not the wall clock)*. The counters are behind
the `counters` feature and compiled out of every build the gate made: an increment on the engine's hottest
path.

- **(a) `candidates`, in a gate step of its own:** `cargo test -p ein-infer --features counters --test
  arith_batch`, in `run_tests.sh` and `per-commit.yml`. `ein-infer` is built a second time with the feature,
  for one test; 15 s the first time here.
- **(b) The static check alone,** and the measured test run by hand: the check reads the module's text, and a
  cost that is not a second premise is not in the text.
- **(c) The feature on in the workspace's test build,** by a dev-dependency: every test binary, the `ein` the
  corpus sweep times among them, would count.

**Defaulted 2026-11-11: (a).** `candidates` is the facts the matcher offered to a premise and unified against,
which is what a scan inflates and nothing else does: 181.38 a request at 250, 1 000 and 4 000 requests. Without
the feature the same file holds the firings a request, in `cargo test --workspace`.

**Overruled 2026-11-12, and built** ([S1f.6.21](p1f.6_calculator/s1f.6.21_the_gate_reduced.md)). The user,
on reply 46, option (b): *"(b) - we don't need this proportional gate"*. The step left `run_tests.sh` and
`per-commit.yml`; the test, the feature and the bounds stay, run by hand, in no tier.

### Q-M1f.151 — The names of the circuit's own relations

**Raised** 2026-11-11 by [S1f.6.3](p1f.6_calculator/s1f.6.3_addition.md)'s T1. The stage file names `n`, `z`, `b`, `add`, `sum-of`, `let`, `done` and the tag
`sum`. The prototype's other relations are `ak`, `sb`, `bk`.

- **(a) Words:** `add-walk`, `strip`, `turn`, `operand`, `operand-is`, `named`, `add-first`, `add-second`,
  `arith-rule`. A `--trace` and a state file print them.
- **(b) The prototype's two letters:** unlikely to clash with a program's names, and unreadable.
- **(c) A prefix,** `arith.walk`: a dot is what a qualified import writes, and would read as one.

**Defaulted 2026-11-11: (a).** Imported flat, every one is a name the program may not use otherwise, `done`,
`sum`, `strip` and `turn` among them; the module's header lists them. What a clash does and what the loader
should say is [S1f.6.12](p1f.6_calculator/s1f.6.12_malformed_numbers.md)'s.

### Q-M1f.152 — Are `std.arith`'s rules written by hand or generated?

**Raised** 2026-11-11 by [S1f.6.3](p1f.6_calculator/s1f.6.3_addition.md)'s T1. The tables are generated (the user's ruling). The prototype generated its
rules too, since division has a rule for each of nine multiples.

- **(a) By hand,** in `stdlib/arith.ein`, with comments beside the rules.
- **(b) Generated,** with the same `--check`.

**Defaulted 2026-11-11: (a), for addition.** 19 rules, none a family. The stage that writes division's
`div-multiple-2` to `-9` and `div-try-1` to `-9` meets the question again; a generated block inside a
hand-written module would need its own check.

### Q-M1f.153 — The DOT sweep's bound, under the smallest table

**Raised** 2026-11-11 by [S1f.6.3](p1f.6_calculator/s1f.6.3_addition.md)'s T6, on Q-M1f.147's bound of 1 500 edges. The KB view of a program that imports
`std.arith` holds `fa`'s 200 rows, some 1 200 edges; `dot` draws it in 0.55 s, and each program has six KB
views.

- **(a) 1 000 edges:** a view that holds a table is past the bound; the bounded layout takes 0.1 s on it.
- **(b) 1 500:** `dot` still lays out the addition programs, 3.3 s each, 26 s for the stage's eight, and more
  with every program a later stage adds until it imports a second table.

**Defaulted 2026-11-11: (a).** The largest view the corpus had before the tables has 361 edges, so what `dot`
lays out is what it laid out then. With Q-M1f.147, for the user to confirm.

**Overruled 2026-11-12 with Q-M1f.147, and built** ([S1f.6.21](p1f.6_calculator/s1f.6.21_the_gate_reduced.md)):
the bound stays 1 000 edges, and a view past it is skipped where it went to the bounded layout.

### Q-M1f.154 — The bounds of the harness against the integers

**Raised** 2026-11-11 by [S1f.6.3](p1f.6_calculator/s1f.6.3_addition.md)'s T3, which asks for *random operands of every length up to a stated bound*.

- **(a) Exhaustive to 120, sampled to 30 digits:** every pair from 0 to 120; every pair of lengths from 0 to
  30 digits, twice; a carry through every length to 30; 1 500 operands padded with high zeros. The oracle is
  `u128`, which holds 38 digits. A second of test time.
- **(b) Longer operands,** against a decimal adder written in the test: an oracle that is itself a column
  adder.

**Defaulted 2026-11-11: (a).** The theory has no width, so a length is not a boundary of it: the walk at the
thirtieth column is the walk at the third. S1f.6.9, on how long a number may be, is where the nesting limit of
1 000 is met.

### Q-M1f.155 — Equal operands of a subtraction: two walks, one fact

**Raised** 2026-11-11 by [S1f.6.4](p1f.6_calculator/s1f.6.4_subtraction.md)'s T1. The circuit subtracts both ways. When the operands are equal neither
walk borrows, so both hand over zero.

- **(a) Both answer:** `sub-answer` derives `(diff-of X Y z z)` from the one walk and `sub-answer-short` the
  same fact from the other. One answer, with two derivations; two firings more than an unequal pair's one.
- **(b) One walk is told to keep quiet on zero:** a rule more, or a shape test on the shortfall, `(n ?p ?a)`,
  so that only a shortfall that is not zero answers.

**Defaulted 2026-11-11: (a).** The answer is the same fact either way, the harness holds *exactly one answer a
request*, and (b) buys two firings on equal pairs with a rule that reads less plainly.

### Q-M1f.156 — Comparison called with a tag

**Raised** 2026-11-11 by [S1f.6.4](p1f.6_calculator/s1f.6.4_subtraction.md)'s T1. The prototype's comparison walk carried the request's operands and
wrote `cmp-of` when it ran out of columns.

- **(a) A tag, like every other walk:** `(cmp-walk TAG X Y ORDER)` ends in `(order-is TAG ORDER)`, and the
  public request's rule reads `(order-is (order X Y) O)`. One firing more a request, 13 for 123078 against 4567.
- **(b) As the prototype:** a firing fewer, and no way to call a comparison from inside the theory.

**Defaulted 2026-11-11: (a).** S1f.6.7 and S1f.6.8 compare numbers inside their own circuits, and
[S1f.6.3](p1f.6_calculator/s1f.6.3_addition.md#for-the-stages-that-follow)'s rule is that a call inside the theory starts a walk with a tag of its own.

### Q-M1f.157 — Each request looks its operands up by rules of its own

**Raised** 2026-11-11 by [S1f.6.4](p1f.6_calculator/s1f.6.4_subtraction.md)'s T1. `add` has three rules and two tags, `add-first` and `add-second`, to look
its two operands up. Every further request needs the same.

- **(a) Repeat them:** three rules and two tag relations a request, 15 rules for the five requests.
- **(b) Share them:** one pair lookup, `(operands TAG X Y)` answered by `(operands-are TAG U V)`, three rules
  for all, and two a request to call it and go on. Every request costs two firings more, and addition's pinned
  circuit, 21 firings, moves to 23.

**Defaulted 2026-11-11: (a),** which is what S1f.6.3's last section tells a later stage to do. (b) is a
refactoring any later stage can make with no answer moving.

### Q-M1f.158 — `cl`'s column order and the index

**Raised** 2026-11-11 by [S1f.6.4](p1f.6_calculator/s1f.6.4_subtraction.md)'s T2, on E-12 of the phase's `findings.md`. The matcher keys a premise on its
first bound argument. `cl`'s first argument is the order below, shared by 100 of its 300 rows, so a column of a
comparison offers the matcher 100 candidates where a column of a sum offers 20: 901 candidates a nine-digit
comparison.

- **(a) Leave the table:** the cost is a constant a column, the proportion holds, and 4 000 comparisons
  saturate in 74 ms.
- **(b) Put a digit first,** `(cl A B BELOW ORDER)`: 30 candidates a column. The generator, the table's test,
  its `:why`, `tests/stdlib/digits/01_tables.ein` and three pages move.
- **(c) The engine picks the most selective bound slot:** an engine change, outside P1f.6's theory stages.

**Defaulted 2026-11-11: (a).** The row is kept as E-12 for whoever owns the index.

### Q-M1f.159 — Multiplication called with a tag

**Raised** 2026-11-11 by [S1f.6.5](p1f.6_calculator/s1f.6.5_multiplication.md)'s T1. The prototype's multiplication carried the request's operands in
every step and wrote `product-of` when the multiplier ran out.

- **(a) A tag:** `(mul-call TAG U V)` is answered by `(done TAG P)`, and the public request calls it with
  `(product X Y)`. One firing more a request, 72 for 123 × 456.
- **(b) As the prototype.**

**Defaulted 2026-11-11: (a),** as for comparison (Q-M1f.156): S1f.6.8's rationals multiply inside their own
circuits.

### Q-M1f.160 — A zero digit of the multiplier

**Raised** 2026-11-11 by [S1f.6.5](p1f.6_calculator/s1f.6.5_multiplication.md)'s T1.

- **(a) A row like any other:** the multiplicand times 0 is a walk of its length that strips to `z`, and `z` is
  added to ten times the sum. About four firings a digit of the multiplicand, for nothing.
- **(b) Two rules that skip it:** a zero digit multiplies the sum so far by ten and goes on. `mul-row` then
  needs `(nz ?a)`.

**Defaulted 2026-11-11: (a).** The answer is the same and the theory is two rules shorter; (b) is an
optimisation any later stage can make with no answer moving, and the circuit's pinned sizes would say what it
saves.

### Q-M1f.161 — The DOT sweep, several files at a time

**Raised** 2026-11-11 by [S1f.6.5](p1f.6_calculator/s1f.6.5_multiplication.md)'s T4, on Q-M1f.147 and Q-M1f.153. Every program that imports `std.arith`
holds the tables the theory reads, `md`'s 900 rows among them since this stage, so each of its six KB views is
some 9 000 edges and takes the bounded layout 2.2 s. The suite has eighteen such programs and S1f.6.6 adds more:
441 s of `dot` in one thread.

- **(a) Several files at a time:** `dot_wellformed.rs` shares the corpus files among the cores. 28 s on sixteen
  cores; about a quarter of the single thread's time on the runner's four. What is held is unchanged.
- **(b) Leave it:** four minutes more a gate now, and more with every arithmetic program.
- **(c) Draw a table as one node** (E-10's open question): a DOT writer's change, and the user's.

**Defaulted 2026-11-11: (a).** It changes no claim. (c) stays E-10's.

### Q-M1f.162 — The longest sampled operand of a product

**Raised** 2026-11-11 by [S1f.6.5](p1f.6_calculator/s1f.6.5_multiplication.md)'s T2, on Q-M1f.154's oracle, `u128`, which holds 38 digits.

- **(a) 19 digits:** a product of two has at most 38.
- **(b) 30 digits, as the other operations,** against a multiplier written in the test.

**Defaulted 2026-11-11: (a),** for Q-M1f.154's reason: a length is not a boundary of the theory.

### Q-M1f.163 — Division called with a tag, and whose division by zero is flagged

**Raised** 2026-11-11 by [S1f.6.6](p1f.6_calculator/s1f.6.6_division.md)'s T1. Division is tagged as multiplication is (Q-M1f.159): `(div-call TAG U V)`
is answered by `(div-done TAG Q R)`. The flag the user ruled names a request, `(division-by-zero (div X Y))`,
and a call from inside a later theory has no request to name.

- **(a) The flag is the public request's:** `div-by-zero` matches the tag `(quotient X Y)`. A call under
  another tag whose divisor is zero derives nothing, and the theory that makes the call writes a rule of its
  own for it, with a flag that names its own request.
- **(b) A flag that names the tag:** `(division-by-zero TAG)` for any call. The flag of a public request then
  reads `(division-by-zero (quotient X Y))`, which is not the ruled spelling.

**Defaulted 2026-11-11: (a).** The ruling's spelling holds, and S1f.6.8's rationals, the first caller, decide
what a zero denominator is where they are built.

### Q-M1f.164 — Division's families of nine rules: by hand or generated?

**Raised** 2026-11-11 by [S1f.6.6](p1f.6_calculator/s1f.6.6_division.md)'s T1, on Q-M1f.152, defaulted *by hand, while a rule is not a family of nine*.
Division has two: nine trial rules, one a multiple, and eight rules of the multiples' chain.

- **(a) By hand, held by a test:** the seventeen rules are in `arith.ein` as text, and `arith_theory.rs` holds
  that the rules of a family differ by their digit alone.
- **(b) Generated:** a script writes the two families, or the module, with a `--check` in the gate, as
  `std.digits` is.
- **(c) One rule:** not without a premise that joins on a number, or a change to the language.

**Defaulted 2026-11-11: (a).** The module stays one hand-written file a reader can follow, and what a
generator would guarantee the test holds.

### Q-M1f.165 — What search-free means of a program that divides by zero

**Raised** 2026-11-11 by [S1f.6.6](p1f.6_calculator/s1f.6.6_division.md)'s T2. The phase's claim is *H empty, k_s 1, R {∅}* on every arithmetic program, and
the user ruled a division by zero `(false)`: such a program's R is empty, k_s 0.

- **(a) Held to what is true of it:** no candidate, one saturation and no fork, monotone, R empty and decided
  by root alone, the request in the core, the second rule in `refuted by`. `arith_search_free.rs` takes this
  branch for a program that claims `(solve :expect (false))`.
- **(b) Kept out of the search-free roots:** the programs live elsewhere and the test's claim stays uniform.

**Defaulted 2026-11-11: (a).** Nothing is searched in either case, which is the claim; the count of states is
what the ruling changed.

### Q-M1f.166 — The sizes of the generated arithmetic examples

**Raised** 2026-11-11 by [S1f.6.15](p1f.6_calculator/s1f.6.15_load_against_calculation.md)'s T1. The plan asked for operands of some hundreds of
digits, some thousands of requests an operation, and a chain whose answer has hundreds of digits. `ein solve`
writes a state as text, and a circuit's state as text grows as its facts times its digits (E-24 of the phase's
`findings.md`): a 100-digit product is 81 ms under `ein saturate` and 4.0 s under `ein solve`, and its state file
reloads at 0.3 s a megabyte. Every corpus program is solved, dumped and reloaded per commit.

- **(a) Under the line:** the shipped files are what every per-commit surface affords, 100-digit operands for a
  sum, 10-digit for a product, 100 requests, 25!; `utils/arith_cost.py` runs the plan's sizes and larger as a
  sweep of programs it writes itself, and the generator takes the sizes as arguments.
- **(b) The plan's sizes, flagged `slow`:** the three leave the per-commit sweep and round trip for the nightly,
  whose round trip then reloads state files of hundreds of megabytes.
- **(c) The plan's sizes with `saturate` as the one declared run:** the round trip and the search-free test take
  every entry whatever it declares, so they would need an exemption each.

**Defaulted 2026-11-11: (a).** An example a reader can run under every command, and the sweep where the sizes
are. What E-24 argues for is the user's to weigh; with it lifted the files can grow by an argument.

### Q-M1f.167 — The names of `std.num9`

**Raised** 2026-11-11 by [S1f.6.13](p1f.6_calculator/s1f.6.13_num9.md)'s T2. The plan: *its relations are its own, `add9`, `sum9-of` and so on, so that a
program may import both modules flat; the names are the stage's to propose with examples before the module is
written.*

- **(a) `std.arith`'s public names with a 9:** `add9`/`sum9-of`, `sub9`/`diff9-of`, `cmp9`/`cmp9-of`,
  `mul9`/`product9-of`, `div9`/`quot9-of`; the entry `arith9`; `to9`/`as9` and `from9`/`of9`; the flags `overflow`
  and `big`, with no digit, since `std.arith` has neither.
- **(b) A prefix:** `n9-add`, `n9-sum-of`.
- **(c) `std.arith`'s names,** the two modules then importable flat only one at a time.

**Defaulted 2026-11-11: (a).** A reader who knows one module reads the other. The stage's record shows the eight
target programs under the names; the user may rename before anything is built on them.

### Q-M1f.168 — Overflow and division by zero in `std.num9`

**Raised** 2026-11-11 by [S1f.6.13](p1f.6_calculator/s1f.6.13_num9.md)'s T1. The plan: overflow is a fact and no number, the design's recommendation, which
the user did not rule; a division by zero is as in `std.arith`.

- **(a) Overflow `(overflow REQUEST)`, root alive; a division by zero by `std.arith`'s relation and rule,** which
  the module imports: one flag relation and one rule that concludes `(false)` across both modules.
- **(b) A flag relation and a `(false)` rule of the module's own,** `division-by-zero9`: nothing imported from
  `std.arith` for it, and two rules of the stdlib that conclude `(false)`.
- **(c) Overflow as `(false)` too.** A program that overflows once then loses every other answer.

**Defaulted 2026-11-11: (a).** An overflow is an answer about a width, which a program may want to read and go on;
a division by zero asked for what does not exist, as ruled.

### Q-M1f.169 — Where `std.num9`'s multiples and rows live

**Raised** 2026-11-11 by [S1f.6.13](p1f.6_calculator/s1f.6.13_num9.md)'s T3, which the plan left to it: *the multiples moved into the state if that removes
the join*. The prototype kept a divisor's multiples and a multiplier's rows as facts and joined its state with
them on a number: 4 000 divisions in 8.1 s.

- **(a) In the state, computed by one rule:** `div9-start` reads 72 rows of `md` and states the nine multiples;
  ten step rules, one a digit of the quotient, each read their own. A product's step computes its row itself.
- **(b) In the state, computed by a chain of eight rules,** as `std.arith` does: eight firings more a division,
  and nine relations of growing arity.
- **(c) No multiples: ten step rules that each compute their multiple,** nine rows of `md` a rule a step.
- **(d) As the prototype:** quadratic.

**Defaulted 2026-11-11: (a).** No rule joins two facts on a number, a division is eleven firings, and 4 000 take
1.24 s. A rule of 73 premises is what a generator is for.

### Q-M1f.170 — What `std.num9` reads as an operand

**Raised** 2026-11-11 by [S1f.6.13](p1f.6_calculator/s1f.6.13_num9.md)'s T4. `std.arith` looks an operand up, so a request may name it; round 1's `num9` did
not.

- **(a) A number written out, and nothing else:** no `let` in the module. A conversion reads a nested number as
  written: high zeros count, and nine digits behind a zero are `big`.
- **(b) Named operands,** through `std.arith`'s `operand`: four firings a request where one, and the whole of
  `std.arith`'s lookup imported.
- **(c) A conversion by value:** `to9` strips high zeros first, a firing a zero.

**Defaulted 2026-11-11: (a).** The module's point is one firing. A program that wants a name writes a rule of its
own from the name to the request.

### Q-M1f.171 — `std.num9`'s step counter and its comparison

**Raised** 2026-11-11 by [S1f.6.13](p1f.6_calculator/s1f.6.13_num9.md)'s T1. The plan said the module adds its own step counter; round 2's `num9` compared
by two subtractors.

- **(a) `std.digits`' tables:** `adj`, the pairs k and k + 1, counts the steps, and `cl`, the comparator, compares
  in one rule.
- **(b) A table of the module's own** for the steps, and two subtractors and a rule for equal numbers.

**Defaulted 2026-11-11: (a).** No fact is added to what every program loads. A comparison's lookup walks the
hundred rows an order keys (E-12), 901 candidates where two subtractors would walk some 360; it is one firing
either way.

### Q-M1f.172 — The harness of `std.num9`: exhaustive at width 2

**Raised** 2026-11-11 by [S1f.6.13](p1f.6_calculator/s1f.6.13_num9.md)'s T5. The plan: *exhaustive at width 2 on the generator's output and sampled at
width 9*. A width-2 theory is not a module of the stdlib.

- **(a) The generator's `--width 2 --stdout`, put in front of the test's requests** as the program's own
  declarations: 49 900 requests in one saturation.
- **(b) A second shipped module,** `std.num2`, which the coverage gate would then ask tests for.

**Defaulted 2026-11-11: (a).** The plan's *one width ships*.

### Q-M1f.173 — `MAX_REGS` and the wide rules of `std.num9`

**Raised** 2026-11-11 by [S1f.6.13](p1f.6_calculator/s1f.6.13_num9.md)'s gate. A rule binds its variables to a register file of 256
(`ein_infer::MAX_REGS`, the engine's one compile-time bound, S1a.3.1; refused at load past it since S1f.5.21).
`ein-infer/tests/compile_limits.rs` holds a tripwire on it: the widest rule of the corpus stays within an eighth of
the bound, 32 registers, *and if a shipping rule ever approaches it the bound stops being acceptable: it needs
re-deciding*. Until this stage the widest was 16. A wide rule of `std.num9` binds a multiple of nine variables by
what it is: 35 for a sum, about 60 for a product's step, about 90 for a division's, and **162 for `div9-start`**,
which computes eight multiples of nine columns in one firing (Q-M1f.169 (a)). The module the user ruled in cannot
be written under 32 at width nine, whatever Q-M1f.169 chose: a sum alone names three numbers and eight carries.

- **(a) The bound stays, and the module is measured apart:** the tripwire holds every other rule of the corpus to
  an eighth as before, and holds `std.num9`'s widest to its number, 162, and under the bound. The module is
  generated and its width is fixed, so the number moves only when the generator does, and then the test fails.
- **(b) A narrower `div9-start`:** Q-M1f.169 (b), a chain of eight rules for the multiples, takes the widest to
  the division's step, about 92, at eight firings more a division. Still three times the tripwire.
- **(c) Raise the bound,** to 1 024 or to a file sized by the plan: an engine change, and the register file's
  fixed size is what keeps the matcher's inner loop free of allocation (design/05 § 3).
- **(d) Narrower rules:** a sum in two halves, two firings. It is no longer round 1's theory, and a product's and a
  division's steps would be split three and four ways.

**Defaulted 2026-11-11: (a), for the user to confirm.** No engine source moved: 162 is under 256 and nothing hits
the bound. What the tripwire asked for is a decision, and this is the smallest one that keeps it watching
everything else. It also says where the wide style ends: `div9-start` takes eighteen registers a digit, so a `num15` by
this generator would not load.

### Q-M1f.174 — The names of `std.int`

**Raised** 2026-11-12 by [S1f.6.7](p1f.6_calculator/s1f.6.7_negative_numbers.md)'s T1. The stage file fixes the
pair, `(int P N)`, and no other name. `std.num9` took `std.arith`'s names with a `9`.

- **(a) A prefix, `int-`:** `int-add` answered by `int-sum-of`, `int-sub` by `int-diff-of`, `int-mul` by
  `int-product-of`, `int-cmp` by `int-cmp-of`, `int-neg` by `int-neg-of`, `int-simplify` by `int-simple-of`; the
  module `std.int`, its entry `int-arith`.
- **(b) A letter,** `addz`/`sumz-of` or `iadd`/`isum-of`, as `num9`'s suffix.
- **(c) `std.arith`'s own names,** told apart by the operand's shape: `(add (int …) (int …))`.

**Defaulted 2026-11-12: (a), the user's to revise.** The names read as what they are and none is a name of
`std.arith` or `std.num9`, so the three import flat into one program. (c) would have this module's rules match
`std.arith`'s request relations, and one request relation answered by two result relations of different
arities.

### Q-M1f.175 — What `std.int` reads as an operand

**Raised** 2026-11-12 by [S1f.6.7](p1f.6_calculator/s1f.6.7_negative_numbers.md)'s T1.

- **(a) An integer written out, `(int P N)` with P and N numbers, or a name** that `std.arith`'s `(let NAME (int P
  N))` states. One rule of the module says an integer stands for itself, and `std.arith`'s `operand-name` reads
  the `let`.
- **(b) A natural number too,** promoted: `(int-add (n z 5) X)` read as `(int-add (int (n z 5) z) X)`, four rules
  more and two firings an operand.
- **(c) Names inside the pair,** `(int price z)`.

**Defaulted 2026-11-12: (a), the user's to revise.** A natural number in an integer's place is answered by
silence, as every malformed operand of the calculator is today; whether it should be flagged is S1f.6.12's
question, and whether it should be promoted is one the expression layer (S1f.6.14) will meet again. (b) can be
added later with no answer of today's moving. `int_theory.rs`'s `a_natural_number_is_not_an_integer_operand`
pins the silence.

### Q-M1f.176 — A product of integers without a rule that looks at a sign

**Raised** 2026-11-12 by [S1f.6.7](p1f.6_calculator/s1f.6.7_negative_numbers.md)'s T1. The stage file gives both:
*(P1·P2 + N1·N2, P1·N2 + N1·P2). On simplified operands, only the sign rules remain.*

- **(a) The formula on the parts as written:** four products of `std.arith`, two sums, one simplification; seven
  rules in a chain and no case.
- **(b) Simplify both operands first, then one product and the sign by rule:** four rules for the combinations of
  signs and two for the answer, and one product.

**Defaulted 2026-11-12: (a), the user's to revise.** Measured (`int_theory.rs`'s `the_circuit_of_each_operation`):
123 × −456 is 142 firings where `std.arith`'s 123 × 456 is 72, three of the four products being by zero; an
operand that is not simplified pays four real products, 413 firings for (200 less 77) × (1000 less 1456). (b)
would be about 100 and 150. (a) has no case to get wrong and none for a mutant to hide in; (b) is what integer
division will need anyway (Q-M1f.179), and the product can move to it then if the cost matters.

### Q-M1f.177 — A call's tag is its request, and what an extension writes of `std.arith`'s

**Raised** 2026-11-12 by [S1f.6.7](p1f.6_calculator/s1f.6.7_negative_numbers.md)'s T1. Two choices about how a
module over `std.arith` is wired, which S1f.6.8 follows.

- **The tag.** `std.arith` declares a tag relation for each result, `(sum X Y)` for `(add X Y)`. `std.int` uses
  the request itself: `(int-done (int-add ?x ?y) ?s)`. A request is a fact a state holds, so it is a value by the
  value rule, and the module declares 15 tag relations where it would declare 21.
- **What it writes of `std.arith`'s relations.** It starts `std.arith`'s walks and calls with tags of its own
  (`add-walk`, `sub-walk`, `cmp-walk`, `mul-call`) and reads their hand-overs (`done`, `order-is`), which is how
  `std.arith` says a later theory calls it. It also writes `operand` and `operand-is`, with a value that is no
  number, so that a request may name its operand through the one `let`.

**Defaulted 2026-11-12: both as built, the user's to revise.** The alternative to the second is a `let` of the
module's own, `(int-let NAME INTEGER)`, with its own lookup rules, and two kinds of named constant in one
program.

### Q-M1f.178 — The harness of `std.int`: a sibling file, exhaustive on one-digit parts

**Raised** 2026-11-12 by [S1f.6.7](p1f.6_calculator/s1f.6.7_negative_numbers.md)'s T3. The stage file: *every sign
combination at the exhausting width*, written when the numbers under it had a width.

- **(a) A sibling of `arith_theory.rs`,** `int_theory.rs`, with its own `OPS` against `i128`; the exhausting set
  is every integer written with parts from 0 to 9, 100 spellings of −9 to 9, against every other.
- **(b) Rows of `arith_theory.rs`'s `OPS`,** whose harness reads operands as `u128` and a module's text as
  `stdlib/arith.ein`.

**Defaulted 2026-11-12: (a).** The three static tests read one module's text each, and an integer operand is
not a `u128`.

### Q-M1f.179 — The remainder of an integer division

**Raised** 2026-11-12 by [S1f.6.7](p1f.6_calculator/s1f.6.7_negative_numbers.md), whose stage file put it to the
user from the start. **Open: the user's, and not defaulted.** Integer division is not built until it is ruled.

For `(int-div X Y)` answered by `(int-quot-of X Y Q R)`, X = Q·Y + R:

- **(a) Euclidean:** 0 ≤ R < |Y|. −7 ÷ 2 is −4 remainder 1; 7 ÷ −2 is −3 remainder 1; −7 ÷ −2 is 4 remainder 1.
  The one convention whose remainder could be answered as a natural number.
- **(b) Truncated:** Q toward zero, R with the dividend's sign. −7 ÷ 2 is −3 remainder −1. Four rules where the
  others take nine.
- **(c) Floored:** Q rounded down, R with the divisor's sign. 7 ÷ −2 is −4 remainder −1.
- **(d) More than one,** each under a request of its own.

**The stage's recommendation is (a).** Each of the three is written as rules over the shipped modules and right
on 924 requests against Python, with the table of answers and the cost of each, in
[the stage's T2 record](p1f.6_calculator/s1f.6.7_negative_numbers.md#t2-record-2026-11-12-the-remainder-convention-three-ways-for-the-user)
(`python3 build/s1f.6.7/intdiv_proto.py`, 2026-11-12). With (a) there is a second question: whether R is
answered as an integer, `(int R z)`, or as a number of `std.arith`. S1f.6.8 does not wait for this: the rationals
never divide integers.

**Ruled 2026-11-12, and built** ([S1f.6.22](p1f.6_calculator/s1f.6.22_integer_division.md)). The user, on
questions 1 and 2 of the calculator's `questions.md`: *"(d)"*, and *"(a)"*, a natural number. There is no bare
`int-div`: `(int-div-euclid X Y)`, `(int-div-trunc X Y)` and `(int-div-floor X Y)`, answered by
`int-quot-euclid-of`, `int-quot-trunc-of` and `int-quot-floor-of`
([Q-M1f.200](#q-m1f200--the-names-of-an-integer-divisions-results)).

### Q-M1f.180 — The names of `std.rat`

**Raised** 2026-11-12 by [S1f.6.8](p1f.6_calculator/s1f.6.8_rational_numbers.md)'s T1. The stage file fixes the pair, `(rat Z D)`, and no other name.

- **(a) `std.int`'s scheme, with `rat-`:** `rat-add` answered by `rat-sum-of`, `rat-sub` by `rat-diff-of`,
  `rat-mul` by `rat-product-of`, `rat-div` by `rat-quot-of`, `rat-cmp` by `rat-cmp-of`, `rat-neg` by `rat-neg-of`,
  `rat-simplify` by `rat-simple-of`; the module `std.rat`, its entry `rat-arith`; the two signs a tag carries,
  `pos` and `neg`.
- **(b)** `frac-`, or `q-`.

**Defaulted 2026-11-12: (a), the user's to revise.** `rat-quot-of` has **one** answer after its operands, where
`std.arith`'s `quot-of` has two: a quotient of rationals is exact and has no remainder, so the same word names a
relation of another arity under another prefix. `pos` and `neg` are two more names the module occupies; the
orders `gt` and `lt` could have served and would have read worse in a trace.

### Q-M1f.181 — What `std.rat` reads as an operand

**Raised** 2026-11-12 by [S1f.6.8](p1f.6_calculator/s1f.6.8_rational_numbers.md)'s T1.

- **(a) A rational written out, `(rat (int P N) D)`, or a name** that `std.arith`'s `(let NAME (rat (int P N) D))`
  states. The numerator is an integer written out, simplified or not; the denominator a number, with or without
  high zeros.
- **(b) An integer or a natural number too,** promoted to a denominator of 1.
- **(c) A name inside the pair,** `(rat debt (n z 4))`.

**Defaulted 2026-11-12: (a), the user's to revise**, as Q-M1f.175 for the integers and for its reasons. 5 is
written `(rat (int (n z 5) z) (n z 1))`. `rat_theory.rs`'s `an_integer_is_not_a_rational_operand` pins the
silence. A tower in which each kind of number is an operand of the next is the expression layer's question
(S1f.6.14).

### Q-M1f.182 — A zero denominator is `std.arith`'s division by zero

**Raised** 2026-11-12 by [S1f.6.8](p1f.6_calculator/s1f.6.8_rational_numbers.md)'s T1. The stage file: *a zero denominator, written by a program or reached by
dividing by a zero rational, is reported, as S1f.6.1 ruled for division by zero*: a flag, then a rule that derives
`(false)`.

- **(a) The same flag,** `(division-by-zero REQUEST)`, the request as written, and `std.arith`'s own rule from it
  to `(false)`: for a request any operand of which has a denominator of zero, whichever the operation, and for a
  `rat-div` by a rational that is zero.
- **(b) A flag of the module's own,** `(zero-denominator REQUEST)`, with a second rule to `(false)`.
- **(c) The flag only where the denominator is used:** a comparison or a product by 3/0 has an arithmetic
  reading; a sum's cross products do too.

**Defaulted 2026-11-12: (a), the user's to revise.** A rational over zero is a division by zero written down, and
one flag relation is one thing for a program's rule to match and one rule that concludes `(false)` in the whole
calculator, which `arith_theory.rs` holds of `std.arith` and `rat_theory.rs` of this module. Each operand's
denominator is compared with zero as it is looked up, by `std.arith`'s comparison walk, so a zero written with
digits is zero: a firing a digit of each denominator, on every request. The flag is derived only for an operand
of a **request**: a `(let NAME (rat Z z))` nobody asks about is not flagged, as nothing in the calculator is
computed that was not asked for. The cost is as ruled for `std.arith`: root is dead and every other query of the
program loses its answer.

### Q-M1f.183 — Lowest terms by `std.arith`'s division, and what it costs

**Raised** 2026-11-12 by [S1f.6.8](p1f.6_calculator/s1f.6.8_rational_numbers.md)'s T1 and T3.

- **(a) Euclid's chain over `div-call`,** as the stage file says, and two more divisions by what it finds; one
  reduction at the end of each operation, on the parts as written, and none in a comparison.
- **(b) A greatest common divisor by subtraction and halving** (Stein's), which needs no division: walks of
  `std.arith` only, more steps and cheaper ones.
- **(c) Operands reduced first,** so that the products are of smaller numbers.

**Defaulted 2026-11-12: (a), the user's to revise.** Measured (`rat_theory.rs`'s `the_circuit_of_each_operation`):
1/2 + 1/3 is 492 firings, 2/3 × 3/4 is 459, 2/4 simplified 334, and 1/2 against 1/3 is 76. Most of a request is
lowest terms, and most of that is `std.arith`'s division computing its divisor's nine multiples before its first
digit, some 70 firings for a divisor of one digit, on each of the chain's steps and on the two divisions after
it: a rational over a small denominator pays for a long division's preparation five or six times. (b) is the
alternative if rationals are used at volume; it was not prototyped. What a program can do today is compare
before it computes: a comparison reduces nothing.

### Q-M1f.184 — The harness of `std.rat`, and the sizes memory allows

**Raised** 2026-11-12 by [S1f.6.8](p1f.6_calculator/s1f.6.8_rational_numbers.md)'s T3. The stage file: *every operand pair at the exhausting width*. A request
of two one-digit fractions is some 450 to 650 firings and a stored fact about a kilobyte (E-26), so a program of
4 000 such requests is two gigabytes.

- **(a) A small exhausting set, in programs of bounded size:** the 28 fractions with a numerator from −3 to 3
  over a denominator from 1 to 4, every pair under each of the five operations, 250 requests a saturation; a
  sample past it; and batches of 100, 400 and 1 200 in `arith_batch.rs`, with a tolerance of 15 % where the
  others take 5 %, since the length of Euclid's chain varies from request to request.
- **(b) The other modules' sizes,** 4 000 requests a batch.

**Defaulted 2026-11-12: (a).** The claim a batch holds is a proportion, and a join on a number multiplies the
work by the batches' ratio, 4 and 12, which a tolerance of 15 % still catches.

### Q-M1f.185 — A state file nested past the parser's limit

**Raised** 2026-11-12 by [S1f.6.9](p1f.6_calculator/s1f.6.9_width.md), whose stage file put the choice to the user
from the start. **Open: the user's, and not defaulted.** It is E-4 of the phase's
[`findings.md`](p1f.6_calculator/findings.md), re-taken on the shipped theory.

The parser refuses a program nested past 1 000 parentheses; a number nests a level a digit; and a step of a
circuit holds its operands inside its tag. So `ein solve --dump-answer` writes a state file that is refused when
it is loaded again, from operands of 999 digits for a sum, a difference and a comparison, 998 for a product and a
quotient, 997 for a sum of integers and 994 for a rational's numerator, each a request that loads and is answered
rightly. The engine itself holds a fact of any depth.

- **(a) Say it on the page and leave it.** Done by the stage, since it is true whatever is ruled:
  `stdlib/README.md` § How long a number may be, and the header of `stdlib/arith.ein`.
- **(b) The theory flags a number longer than a stated bound,** as `std.num9` flags overflow. Probed: five rules
  and a ruler of 990 links flag an addition, at a firing a digit of each operand and of the sum, which doubles
  every sum, 50 firings where 28 at nine digits.
- **(c) An engine change,** so that what a rule can derive the parser can read: the three recursive walks the
  limit protects made iterative or given a stack, or the state file read by a path without them. Not built: the
  stage may not change the engine.

**The stage's recommendation is (a), until a program needs more.** A state past the limit is 40 MB of text at the
least and gigabytes for a product, so the file that fails to reload is one nobody would wait for; (b) taxes every
request for it; (c) belongs with E-24, whose fix would make such a state small enough to be worth reloading. The
probes, with their tables, are in
[the stage's T1 record](p1f.6_calculator/s1f.6.9_width.md#t1-record-2026-11-12-the-limit-re-taken).

### Q-M1f.186 — Where the probes of S1f.6.10 are banked

**Raised** 2026-11-12 by [S1f.6.10](p1f.6_calculator/s1f.6.10_beyond_arithmetic.md). Its stage file says *banked
under `examples/arith/`*. `ein-cli/tests/arith_search_free.rs` holds every `.ein` under that directory, at any
depth, to the search-free claims: it imports a theory of the calculator, the universe has no candidate, R is
{∅}, one saturation, and a result fact of the calculator is in the state. Four of the banked probes search, and
four import the tables alone.

- **(a) A directory beside it,** `examples/beyond_arith/`, which that test does not walk.
- **(b) Under `examples/arith/`,** with the test told which files search, by a list or a marker in the file.

**Defaulted 2026-11-12: (a).** It moves no test, and the directory's name is the page's. It is question 8 of
[`beyond_arithmetic.md`](p1f.6_calculator/beyond_arithmetic.md).

### Q-M1f.187 — Which probes of S1f.6.10 are banked, and at what size

**Raised** 2026-11-12 by S1f.6.10. Every corpus program is solved, dumped and reloaded per commit, by a bare
`ein solve` whatever runs it declares (`answer_round_trip.rs`, `corpus_shapes.rs`), and `ein solve` writes a
state as text (E-24).

- **(a) Bank what a bare `ein solve` exhausts cheaply,** at a size whose dump is a few megabytes: the laws over
  0 to 3, primes to 30, casting out nines to 40, perfect numbers to 12, a Collatz trajectory of 16 steps and one
  stopped at 20, the equations at two and three digits, I + BB = ILL.
- **(b) Bank the headline sizes** and mark them `slow`, which moves `the_slow_set_is_exactly_these_fourteen`.

**Defaulted 2026-11-12: (a).** SEND + MORE = MONEY, whose description stops at the budget and whose claim `ein
test` cannot check (E-30), stays under `build/s1f.6.10/p/` with its numbers on the page; so do the runs to 5 000
and the scale tables.

### Q-M1f.188 — The banked probes are kept by hand

**Raised** 2026-11-12 by S1f.6.10. The probes were written by generators under `build/s1f.6.10/`, which is not
tracked.

- **(a) Written once, then hand-kept.** `build/s1f.6.10/bank.py` wrote each with the claim the engine printed,
  after a Python check of that claim; the files say nothing of a generator.
- **(b) A generator in `utils/`** with a `--check` in the gate, as `gen_arith_examples.py` has.

**Defaulted 2026-11-12: (a).** They are examples to read, of some sixty lines each, and no size of theirs is a
measurement another page quotes.

### Q-M1f.189 — A user's program calling `std.arith`'s tagged calls

**Raised** 2026-11-12 by S1f.6.10's probe 4. `std.arith`'s public protocol is a request fact and a result fact
that repeats its operands. A rule that uses an answer *for a particular purpose* must then find it by its
operands beside its own facts, a join on a number, which the engine answers by a scan (E-3): the laws of
arithmetic over the digits took 16.8 s so written. Written with the theory's tagged calls, `(add-walk TAG X Y 0
z)`, `(mul-call TAG X Y)`, `(div-call TAG X Y)` and `(cmp-walk TAG X Y eq)`, answered by `(done TAG A)`,
`(div-done TAG Q R)` and `(order-is TAG O)`, they took 0.42 s. The module's header lists those relations as *the
circuit's own*; `std.int` and `std.rat` call them from inside the stdlib.

- **(a) The banked programs call them,** and the question of whether they are an interface goes to the user.
- **(b) The banked programs use the public protocol only,** at the cost above, or at domains small enough to
  hide it.

**Defaulted 2026-11-12: (a), for the banked programs; the interface question is the user's**, question 4 of
[`beyond_arithmetic.md`](p1f.6_calculator/beyond_arithmetic.md), with the recommendation that the module's page
name the tagged calls as what a theory built on it calls. Eight programs under `examples/beyond_arith/` would
move if a tagged call's shape did; `arith_theory.rs` does not hold those shapes for outside callers.

### Q-M1f.190 — Which variants of the binary prototypes are banked where the page recommends none

**Raised** 2026-11-12 by the banking of [S1f.6.16](p1f.6_calculator/s1f.6.16_binary_arithmetic.md)'s and
[S1f.6.17](p1f.6_calculator/s1f.6.17_binary_logic.md)'s prototypes as examples (the user, that day: *"bank binary
arithmetic and logic prototypes into examples too"*). Where a page recommends a variant, that one is banked: way
(b) and its generation from `std.arith` for arithmetic; the one table, and the properties as generated facts with
the deriving rules as their check, for logic. On four points the pages show variants and recommend none:

- **The truth table's candidates.** [`binary_logic.md`](p1f.6_calculator/binary_logic.md#2-the-truth-table-of-a-circuit) shows three ways,
  the obligations rung alone, an hrule, and `:hypothesis-relations`, with one answer and one cost.
- **Where the conversions live,** in the binary module or a module of their own: question 5 of
  [`binary_arithmetic.md`](p1f.6_calculator/binary_arithmetic.md).
- **Post's criterion as a program:** the page's has 140 written sets and reads eight `-u` relations restated by
  `forall` and `absent`.
- **What the simplifier reads:** the prototype's imports the deriving rules; the recommendation makes the
  properties facts.

**Defaulted 2026-11-12, the user's to revise: the simplest variant the page's results hold under.** The
obligations rung alone, so the program has no hrule and no `(solve …)` form; `conv.ein`, a module of its own, as
on the prototype; `post.ein` with seven written sets, `complete` by the page's monotone rule and `inside` by
`forall` over the members, reading the generated class facts; `simplify.ein` importing three relations of
`props.ein`. None is a ruling on the pages' questions, which stay open.

### Q-M1f.191 — The generators of the binary examples are run by hand

**Raised** 2026-11-12 by the same banking. `binary_arithmetic.md` recommends that the binary rules be generated
from `std.arith` *and held there by a test*, and asks it as its fourth question; `binary_logic.md` recommends the
properties as generated facts *with a test that holds them to the rows*, its third.

- **(a) Generators in the example directories, run by hand,** each with a `--check`:
  `examples/binary_arithmetic/gen_binary.py` and `examples/binary_logic/gen_gates.py`. No test runs them.
- **(b) Generators in `utils/` with `--check` in a test,** as `gen_arith_tables.py` and `gen_arith_examples.py`
  have.

**Defaulted 2026-11-12, the user's to revise: (a).** These are examples, and (b) is what the pages' open
questions would decide: a `--check` in the gate would make every edit of `stdlib/arith.ein` fail a test until an
example is regenerated, which is the coupling question 4 asks about. What it leaves unheld: after an edit to
`std.arith`, `bin.ein` is stale until someone runs the script, and nothing says so but `--check`. **The
properties' check is in the gate all the same**, without the generator: `props_check.ein` is a corpus program
whose thirty claims hold exactly when the rules derive the generated facts, and its `test` cell is banked.

### Q-M1f.192 — One table file in two example directories

**Raised** 2026-11-12 by the same banking. Two programs of `examples/binary_logic/` set a circuit against the
binary full adder's table, `fa2`, which is `examples/binary_arithmetic/bits.ein`'s. A file-relative import
resolves against the importing file's directory and below it, so the one cannot import the other's module (the
finding is in the phase's [`findings.md`](p1f.6_calculator/findings.md)).

- **(a) A copy,** `examples/binary_logic/bits.ein`, written by `gen_binary.py` with the original, whose `--check`
  compares both.
- **(b) One directory** for both stages' examples.
- **(c) The eight rows written again** in each program that reads them.

**Defaulted 2026-11-12, the user's to revise: (a).** The user named the two directories, and (c) would set the
gates against rows typed beside them and not against the calculator's table.

### Q-M1f.193 — The sizes of the banked binary programs

**Raised** 2026-11-12 by the same banking. Every corpus program is solved, dumped and reloaded per commit by a
bare `ein solve`, which describes all of R and writes each state as text (E-24); describing R over n inputs as
hypotheses is 3ⁿ + n saturations and stops at the default budget near twelve inputs
([`binary_logic.md`](p1f.6_calculator/binary_logic.md#scale-inputs-4-to-24)).

- **(a) Sizes every surface affords:** three inputs for every search, 30 saturations at most; binary operands
  to seventeen bits; five numbers to binary, the largest 255, and seven to decimal, the largest 1000.
- **(b) The pages' headline sizes,** marked `slow`.

**Defaulted 2026-11-12, the user's to revise: (a).** No banked program is `slow`. The batches of 250 requests,
the conversion of 2^40 + 1, the scale runs to 24 inputs, Post's criterion over all 65 535 sets and the
three-input classes stay under `build/s1f.6.16/` and `build/s1f.6.17/`, with their numbers on the pages.

### Q-M1f.194 — Which of the word-problem prototype's programs are banked, and under what names

**Raised** 2026-11-12 by [S1f.6.20](p1f.6_calculator/s1f.6.20_word_problems.md), on the user's instruction of that day: *"bank word problems into
examples/word_problems with recommended variants"*. The prototype is 29 programs and nine modules, most of them
one problem written under several variants ([`word_problems.md`](p1f.6_calculator/word_problems.md) § 0).

- **(a) One file per distinct problem, under the variant the page recommends for each choice**, and the modules
  under the names the page proposes (§ 8): `words.ein`, `gaps.ein`, and `readings.ein`, which is the page's
  `readings_plain`; `gap.ein` is its `gap_b`. Not banked: forms 2 and 3 of § 1; the gap as an obligation and as a
  candidate from a list; the assumption as a hypothesis, alone and with either gap mechanism; `readings` (owed),
  `readings_always`, `readings_both`, `readings_gap` and their problems; the witnesses of § 9.
- **(b) Every program of the page**, the rejected variants in a directory of their own.

**Defaulted 2026-11-12: (a)**, which is what the instruction says. The page keeps every variant with its run,
and `build/s1f.6.20/` the files. **This is not a ruling on the page's twelve questions**, which stay open: a
ruling that differs from a recommendation moves the banked files. Two recommended programs are not banked,
[Q-M1f.198](#q-m1f198--the-remainder-variant-of-the-word-problems-and-the-gate).

### Q-M1f.195 — One file for the two-step problem

**Raised** 2026-11-12 by S1f.6.20. The page runs *Bob has 5 apples, Alice has 7, a box holds 3* twice under the
recommended variants: `boxes12`, with `words` alone and the model's query, and `boxes12_plain`, with
`readings_plain` imported, to show that nothing is proposed where nothing is left over.

- **(a) One file**, `boxes12.ein`: `readings` imported, `(solve :hrules (read-need))`, and three queries, the
  answer, `(reads-as need ?m)` with no row, and the model's. It fires 140 rules at root, the page's
  `boxes12_plain`; the page's `boxes12` fires 139, without the entry rule of `readings` (`ein saturate
  examples/word_problems/boxes12.ein`, 2026-11-12).
- **(b) Two files**, as on the page.

**Defaulted 2026-11-12: (a)**: one file a problem, and a *need* problem imports the readings of *need* whether or
not its division leaves a remainder, since the author does not know that before the run.

### Q-M1f.196 — The word-problem modules as corpus entries

**Raised** 2026-11-12 by S1f.6.20. The completeness check wants an entry for every `.ein` under `examples/`, and
the three modules are imported and are not problems.

- **(a) Each a `positive` entry with runs of its own**, as the stdlib's modules and W88's `import-facts-library`
  are: `words.ein` and `readings.ein` under `solve`, `saturate` and `render rules`, each R {∅} alone; `gaps.ein`
  under `saturate` and `render rules` only, since it imports the declarations of `words` and not its closing
  rule, so alone `asked` and `wanted` are open and a solve describes `(asked __closed__)` (E-1).
- **(b) The modules in a sub-directory the corpus does not walk.** There is none: every `.ein` under the three
  roots is walked.

**Defaulted 2026-11-12: (a).** One line of a module moved for it: the prototype's `readings_plain` imported
`wanted` from `words` and read it nowhere, and the state file `ein solve readings.ein --dump-answer` writes
dropped the declaration, which `answer_round_trip.rs` refuses, *2 facts lost* (E-11's cause, a relation in place
of a rule). The banked `readings.ein` imports what it reads, `value`, `left-over`, `needed` and `over`.

### Q-M1f.197 — What holds the banked word problems

**Raised** 2026-11-12 by S1f.6.20. The stage's T2 lists tests of its own, the answers held to the harness's
integers, the two-step problem's trace steps, each gap problem held to its named gap; they are written *after
the rulings*.

- **(a) The corpus holds them**: every problem declares `test`, so its `:expect` is a banked exit code;
  `boxes12.ein` declares a `--trace` cell; and every file is under the sweeps all corpus files are under, the
  shapes, the dump's round trip, the cores and the levers.
- **(b) A test of their own now**, `ein-cli/tests/word_problems.rs`.

**Defaulted 2026-11-12: (a).** The claims state every answer the page reports for the banked problems, the gap
problems' `unanswered` and `rests-on` facts among them. What no claim can state is the order of the two requests
and the steps of the trace, which is T2's test and waits for the rulings with it.

### Q-M1f.198 — The remainder variant of the word problems, and the gate

**Raised** 2026-11-12 by S1f.6.20. The page's recommended form of a missing reading is `readings_plain`:
readings as hypotheses, nothing owed, and under a supposed reading the answer is a `value`, so that a later
quantity follows from it by the same rules. Its two problems, 13 apples in boxes of 3 and the same with the
reading stated, answer on today's engine as the page reports. `ein-infer/tests/region.rs`'s
`fact_grain_never_calls_a_violated_cube_monotone` does not end with either in the corpus: it ran 13 263 s on one
thread before it was stopped, where it takes 20 s without them
([E-33](p1f.6_calculator/findings.md), `cargo test -p ein-infer --test region fact_grain_never`, 2026-11-12).
The corpus has no field that keeps a file out of that sweep, which walks the files and not the manifest.

- **(a) Not banked.** The module `readings.ein` is, and `boxes12.ein` imports it; the 13-apple program is shown
  in the directory's README and runs beside the banked files. The test and the engine are unmoved.
- **(b) The answer under a reading as a fact of its own, not a `value`.** Run: with the two reading rules
  concluding `(answer-under Q N OVER)`, R is the same three states and the test passes in 20 s. It is a cross of
  the page's `readings_plain` and `readings_both`, and it gives up what the page chose hypotheses for: nothing
  later is computed from an answer that is not a `value`.
- **(c) `readings_both` or `readings_gap`**, the page's two search-free forms, which it does not recommend.
- **(d) The prototype analysis bounded**, by a cap on its shapes or a list of files it skips. That changes what
  a test of M1e2 claims, and is not this stage's to do.
- Asking the calculator for the fifth box at root, whatever the reading, was tried and does not help: the
  `value` a reading concludes is what the word rules turn into a request.

**Defaulted 2026-11-12: (a), the user's to revise**, on the coordinator's instruction for this case: bank what
the gate can hold, and report what was left out. Every program in which a hypothesis leads to a `value` of the
word theory meets it, so it bears on question 6 of the page, and on the hypothesis form of an assumption
(question 4), whichever is ruled.

### Q-M1f.199 — How the skipped views of the DOT sweep are pinned

**Raised** 2026-11-12 by [S1f.6.21](p1f.6_calculator/s1f.6.21_the_gate_reduced.md)'s T2, which asks that the
count of skipped views be *printed and pinned*, so that *a bound that starts skipping a small view is noticed*.
The count grows with every program that imports the calculator, six KB views a program, so an exact number in
the test would be re-banked by every later stage of the phase.

- **(a) The exact count in the test.** Fails on every new arithmetic program.
- **(b) The count printed, and the directories that hold a skipped view named in the test** (`SKIPPING`, eleven
  of them): a view skipped in a directory that had none fails until the directory is named. Every skipped view
  is printed with its edges.
- **(c) Printed only.**

**Defaulted 2026-11-12: (b), the user's to revise.** The count on the stage's commit, 663 views of 111 files,
is in the stage's record with its command.

### Q-M1f.200 — The names of an integer division's results

**Raised** 2026-11-12 by [S1f.6.22](p1f.6_calculator/s1f.6.22_integer_division.md), whose stage file names it.
The user ruled the three requests, `int-div-euclid`, `int-div-trunc` and `int-div-floor` (reply 1 (d), the
option's own words), and not the relations that answer them.

- **(a) `int-quot-euclid-of`, `int-quot-trunc-of`, `int-quot-floor-of`,** each `(… X Y Q R)` with the operands
  repeated as written, after `std.arith`'s `quot-of` and `std.rat`'s `rat-quot-of`.
- **(b) One result relation with the convention as an argument,** `(int-quot-of CONVENTION X Y Q R)`. The
  Euclidean remainder is a natural number and the other two are integers, so one relation would hold two shapes
  in one position.
- **(c) The request's own name with `-of`,** `int-div-euclid-of`: no other result of the calculator is named
  after its request.

**Defaulted 2026-11-12: (a), the user's to revise**, as the stage file has it.
[S1f.6.31](p1f.6_calculator/s1f.6.31_the_stdlib_layout.md) moves the modules and renames no relation.
