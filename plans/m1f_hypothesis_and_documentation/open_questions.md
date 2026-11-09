# Open Questions — M1f

Questions **this milestone raises**, with sticky `Q-M1f.<n>` ids. Do not reuse
a closed id.

M1f was created on 2026-08-29 by moving two phases out of
[M1e](../../docs/history/m1e_review_processing/README.md). The first questions it raised came from
[S1f.5.5](p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md)'s paper grammar on 2026-11-08.

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
| [Q-M1f.51](#q-m1f51--three-more-tests-whose-slow-half-no-ci-tier-runs) | three more tests whose slow half no CI tier runs | **defaulted 2026-11-09 (the user's instruction): (a), recorded, not scheduled; parked for the phase's close**; [S1f.5.13](p1f.5_the_no_owner_items/s1f.5.13_large_dumps_to_the_slow_tier.md)'s T1f.5.13.3 |
| [Q-M1f.52](#q-m1f52--the-nightlys-review-bundle-and-the-registers-runs-that-are-not-a-solve) | the nightly's review bundle and the register's runs that are not a solve | **defaulted 2026-11-09 (the user's instruction): (a), reported, not fixed here; the nightly is red at `utils/review_bundle.py` since `fc53818f`**; [S1f.5.13](p1f.5_the_no_owner_items/s1f.5.13_large_dumps_to_the_slow_tier.md)'s T1f.5.13.3. **Re-opened the same day as (b)**, taken by default on the user's instruction of 2026-11-09: placed as a stage of its own, [S1f.5.19](p1f.5_the_no_owner_items/s1f.5.19_the_review_bundle_and_runs_that_are_not_a_solve.md); **built 2026-11-09** (`00c4ed34`): the bundle builds all 175 cases and the nightly is green |
| [Q-M1f.53](#q-m1f53--how-the-per-commit-gate-catches-a-register-row-the-bundle-cannot-read) | how the per-commit gate catches a register row the bundle cannot read | **defaulted 2026-11-09 (the user's instruction): (b), the smoke builds a command case and reads every register row with no run**; [S1f.5.19](p1f.5_the_no_owner_items/s1f.5.19_the_review_bundle_and_runs_that_are_not_a_solve.md) |
| [Q-M1f.54](#q-m1f54--the-2026-11-06-inventorys-evidence-in-buildreview) | the 2026-11-06 inventory's evidence in `build/review/` | **defaulted 2026-11-09 (the user's instruction): (b), moved intact to `build/inventory-2026-11-06/`, and `build/review/` rebuilt**; [S1f.5.19](p1f.5_the_no_owner_items/s1f.5.19_the_review_bundle_and_runs_that_are_not_a_solve.md) |
| [Q-M1f.55](#q-m1f55--a-container-on-a-solves-now) | a `container` on a solve's `now` | **defaulted 2026-11-09 (the user's instruction): (a), the case page runs the container, as `wrong_answers.rs` does**; [S1f.5.19](p1f.5_the_no_owner_items/s1f.5.19_the_review_bundle_and_runs_that_are_not_a_solve.md) |
| [Q-M1f.56](#q-m1f56--what-the-nl-census-counts-which-fact-set-and-which-registers) | what the NL census counts: which fact set, and which registers | **defaulted 2026-11-10 (the user's instruction of 2026-11-09): (a), every fact of every state file, and four registers, puzzle, stdlib, kernel and undeclared**; [S1f.5.14](p1f.5_the_no_owner_items/s1f.5.14_nl_required.md) |
| [Q-M1f.57](#q-m1f57--which-fixtures-get-a-why) | which fixtures get a `:why` | **defaulted 2026-11-10 (the user's instruction of 2026-11-09): (a), every file the loader reads, on the form's last line in a refused fixture; a file refused at the parse and the records under `plans/` and `docs/history/` are left**; [S1f.5.14](p1f.5_the_no_owner_items/s1f.5.14_nl_required.md) |

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
