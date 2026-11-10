# P1f.5 — The close review

**Phase:** [P1f.5](README.md)
**Status:** written 2026-11-10 for the user's review, at `f454de3b`. Every stage is done, and the phase's
acceptance has been re-checked point by point and is met ([README § Acceptance re-taken](README.md#acceptance-re-taken-2026-11-10)).
**The user answered both questions the same day**, in this file, and each answer is recorded where it was given:
every default stands, accepted without review ([§ Decided by default](#decided-by-default)), and every finding
became a stage, eight of them, S1f.5.21 to S1f.5.28, so **the phase is re-opened** and closes when they are done
([§ What the close needs](#what-the-close-needs)). The `build/` cleanup, the third question, was approved on
2026-11-10 and is done ([§ `build/`](#build)).

## Where the phase stands

- **Stages:** all twenty, S1f.5.0 to S1f.5.19, are done, and so is the merge of the second lane. None is in
  progress and none is new.
- **Commits:** 106, from 2026-11-08 to 2026-11-10 (`git log --format=%s | grep -c '^T1f\.5\.'`).
  - Every code commit passed the gate, with its log named in the commit message.
  - The first nightly on the merged tree was green, in 81 min 54 s (`./run_tests.sh --nightly`, 2026-11-09,
    `build/nightly-s1f.5.19.log`).
- **Register:** it read 103 fixed, 10 open and 1 not wrong at M1e2's close. It now reads 161 fixed, 13 open and
  1 not wrong (`grep '^status' corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-10).
  - The phase filed 61 rows, W29 to W43 and W50 to W61.
  - 58 of them are fixed. 3 are still open, all owned by F22: `W36/alt-flag-refuted-by`,
    `W36/alt-flag-two-derivations` and `W39/or-one-key`.
  - The 13 open rows each have an owner: F22 ×8, F21 ×4 and S1f.10.6 ×1.

## Stages

Commits are counted with `git log --format=%s | grep -c '^T1f\.5\.<n>\.'`.

| Stage | Status | What it did | Commits | Register rows |
|---|---|---|---|---|
| [S1f.5.0](s1f.5.0_the_known_wrong_answers_filed.md) | done 2026-11-08 | Filed the known wrong answers before any stage moved them | 4 (`0525444e`..`e8ab1e17`) | W29–W35: 23 filed `open`, each owned by its stage |
| [S1f.5.17](s1f.5.17_ci_green_again.md) | done 2026-11-08 | CI green again. Ubuntu 26.04's x86-64-v3 Graphviz refused 20 corpus views; the overlay and Levi digraphs now set `ranksep=0.55`. Re-took the nightly's mutant baseline and added `utils/ci_container.sh` | 3 (`3e6e40f8`..`fd800fe0`) | — |
| [S1f.5.3](s1f.5.3_range_retired.md) | done 2026-11-08 | The `RANGE` token retired: Ein has no ranges | 3 (`b1ac2178`..`1cd74578`) | — |
| [S1f.5.15](s1f.5.15_rule_priority.md) | done 2026-11-08 | `:priority` removed. The agenda and the boundary are plain FIFO (Q-M1f.4 and .6, re-ruled (a)). Added a non-stratifiability diagnostic | 10 (`40272dbd`..`4c272aae`) | W36 ×2 filed, `open` (F22) |
| [S1f.5.5](s1f.5.5_the_grammar.md) | done 2026-11-09 | The grmtools-generated parser replaced the hand-written one (the swap, `8c3fe86e`). Also: the engine's names reserved, the grammar pages generated, `--diff-models` compares by value, and one read path for every command (T13) | 14 (`49e9fed6`..`dc4c0bdc`) | W35 fixed; W40 ×6 and W41 filed and fixed |
| [S1f.5.4](s1f.5.4_digits_are_names.md) | done 2026-11-09 | Digits are plain names; the int kind is gone from the engine; `{?n*}` templates are filled | 5 (`8b12ae07`..`3de122a4`) | W37 fixed |
| [S1f.5.2](s1f.5.2_strings_are_natural_language.md) | done 2026-11-09 | A string appears only after `:why`, `:source` and `:goal-text`; a `\u{…}` that names no character is refused | 4 (`90774a44`..`1e751131`) | W38 filed and fixed |
| [S1f.5.1](s1f.5.1_bare_underscore_refused.md) | done 2026-11-09 | A bare `_` is refused everywhere; `Slot::Any` and `Node::Wildcard` are gone | 4 (`728e716a`..`366877cc`) | W29 ×6 fixed; W39 filed, `open` (F22) |
| [S1f.5.11](s1f.5.11_closed_keywords.md) | done 2026-11-09 | Each form takes a fixed list of keywords; `:where` is gone | 2 (`bb87d53e`..`06433b60`) | W32 ×7 fixed |
| [S1f.5.16](s1f.5.16_the_read_out_reference.md) | done 2026-11-09 | The read-out reference: [`read_out.md`](../../../docs/kernel/inference/read_out.md) covers every row `ein solve` prints, and `read_out_reference.rs` holds it to the printer | 5 (`8b1bcabb`..`4aa9f3ef`) | — |
| [S1f.5.6](s1f.5.6_the_empty_state_decides_the_minimal_line.md) | done 2026-11-09 | Where ∅ is in R, every minimal-line question is decided at root, past `--region-limit` too | 6 (`eef535b2`..`114e8f26`) | W42 ×2 filed and fixed |
| [S1f.5.12](s1f.5.12_cores_for_n_only.md) | done 2026-11-09 | A core is taken only for the nodes an output prints; `not-kept` retired | 5 (`87b7cb5a`..`e0162f7c`) | W43 filed and fixed |
| [S1f.5.8](s1f.5.8_no_contradiction_no_core.md) | done 2026-11-09 | A node that is not dead has a `null` core on every JSON surface, not `[]` | 3 (`2b63223d`..`7bc6ea72`) | — |
| [S1f.5.13](s1f.5.13_large_dumps_to_the_slow_tier.md) | done 2026-11-09 | The round trip's four largest dumps moved to the nightly (`heavy_dump`); the per-commit run takes 8.6 s where it took 55 s | 3 (`63245a11`..`aa198e80`) | — |
| [S1f.5.7](s1f.5.7_eq_and_neq_are_relations.md) | done 2026-11-09, lane 2 | `eq` and `neq` are relations, in a goal, a premise and a conclusion; computed and never stored | 7 (`a081ae95`..`2eb483e0`) | W30 ×3 and W50–W56 (11 rows) fixed; W24 ×2 restated |
| [S1f.5.10](s1f.5.10_a_conclusion_variable_must_be_bound.md) | done 2026-11-09, lane 2 | A conclusion variable that nothing binds is refused at load | 4 (`6fbcf64b`..`8249e872`) | W57 ×4 filed and fixed; W58 filed |
| [S1f.5.18](s1f.5.18_an_activator_binds_a_nested_fact.md) | done 2026-11-09, lane 2 | An activator binds a parameter to a nested fact; a derived activator binding a relation parameter to a fact is refused at load | 5 (`e96a8d45`..`ee3da657`) | W33 and W58 fixed; W59 ×2 filed and fixed |
| [S1f.5.9](s1f.5.9_an_empty_match_fires.md) | done 2026-11-09, lane 2 | An empty `:match` fires; `:assert ()` asserts nothing; `eq` at another arity is refused at load | 5 (`b6852791`..`e5f49b03`) | W31 ×4 and W34 fixed; W60 ×4 and W61 filed and fixed |
| Merge of lane 2 ([§ The order](README.md#the-order), item 10) | done 2026-11-09 | `--no-ff` merge into master; gate green on the merged tree, 16 min 37 s (`build/gate-merge-lane2.log`) | 2 (`a90c452b`, `fd35efa6`) | — |
| [S1f.5.19](s1f.5.19_the_review_bundle_and_runs_that_are_not_a_solve.md) | done 2026-11-10 | The nightly's review bundle had been red since `fc53818f`; it now builds all 175 cases. On the way, every rule view the bundle draws was fixed: 654 had been refused by Graphviz since `4778af1f` | 3 (`919380c2`..`b7108e3a`) | — |
| [S1f.5.14](s1f.5.14_nl_required.md) | done 2026-11-10 | `:why` is required by default (`--no-require-why` loads without one); a fact reads as English in state files and `--trace` | 9 (`62f3f8bc`..`f454de3b`) | — |

**S1f.5.14's numbers** (`python3 utils/nl_census.py --slow`, 2026-11-10):
- loadable programs the requirement refuses: 238 of 341 before the migration, 0 after;
- answer facts that read as English: 44.9 % → 94.4 %;
- `zebra2`'s state: 88.7 % → 97.4 %.

## The user's rulings during the phase

[Q-M1f.1](../open_questions.md#q-m1f1--g-14-is-a-bad-conclusion-that-never-fires-refused), [Q-M1f.2](../open_questions.md#q-m1f2--the-55-files-the-grammar-refuses-that-todays-parser-accepts), [Q-M1f.3](../open_questions.md#q-m1f3--three-wrong-answers-the-census-found-with-no-row), [Q-M1f.4](../open_questions.md#q-m1f4--removing-priority-moves-the-facts-of-cores-on-seven-corpus-programs), [Q-M1f.5](../open_questions.md#q-m1f5--the-alternatives-flag-moves-refuted-by-beside-an-unmoved-core), [Q-M1f.6](../open_questions.md#q-m1f6--the-derived-agenda-holds-the-cores-the-fifo-boundary-does-not), [Q-M1f.7](../open_questions.md#q-m1f7--where-may-a-program-write-the-kernels-dunders); and [Q-M1f.9](../open_questions.md#q-m1f9--what-a-deleted-loader-line-leaves-behind), ruled (a) on 2026-11-09 after it had been defaulted.

## Decided by default

Each decision below was taken on the user's instruction of 2026-11-09: *"just continue with default decisions till the
end of the phase"*. That is 55 rows, Q-M1f.8, .10 to .48 and .50 to .64; Q-M1f.49 is unused. Each row in
[open_questions.md](../open_questions.md) gives its reason and what reopening it would mean. ★ marks the ones most
worth a look.

### The parser (S1f.5.5, .4, .2, .1, .11 and S1f.5.5's T13)

- ★ [Q-M1f.8](../open_questions.md#q-m1f8--the-parsers-remaining-cost-within-grmtools-or-past-it): the parser's speed was improved only within grmtools. `ein saturate examples/zebra.ein` takes 9.0 ms,
  against 4.7 ms on the hand-written parser and 11.9 ms at the swap (S1f.5.5 § T12 record).
  - Parked: (b) a patch to `lrlex`, the lexer half of grmtools.
  - Parked: (c) a hand-written lexer. It would breach "generated by grmtools", so a default could not take it.
  - Since the swap, each nightly fuzz seed takes about 25 min (1 533 s and 1 539 s,
    `build/nightly-s1f.5.19.log`, 2026-11-09).
- [Q-M1f.10](../open_questions.md#q-m1f10--the-einb-ints-section-once-the-int-kind-is-gone): the `.einb` `INTS` section stays, empty, with no format bump.
- [Q-M1f.11](../open_questions.md#q-m1f11--a-template-reference-to-a-variable-no-reference-can-name): a template that names a variable holding `}` is refused at load.
- [Q-M1f.12](../open_questions.md#q-m1f12--which-surfaces-carry-natural-language-text): natural-language text gets no new output surface.
- ★ [Q-M1f.13](../open_questions.md#q-m1f13--f3s-cause-reaches-a-disjunction-whose-row-is-it): W39 stays `open` as F22's. A top-level `(or …)` whose disjuncts bind the same registers loses a
  derivation.
- [Q-M1f.14](../open_questions.md#q-m1f14--which-keyword-pair-arms-go-once-no-term-holds-one): the code that dropped or labelled a keyword pair inside a term is gone; the walkers into a pair's value stay.
- [Q-M1f.15](../open_questions.md#q-m1f15--where-does-a-position-in-a-containers-stored-program-point): an error inside a container points into its stored program, named `<program stored in x.einb>`.
- [Q-M1f.16](../open_questions.md#q-m1f16--which-of-the-reads-defects-are-register-rows): each command that gave a wrong answer on a container is its own row (W40 ×6, W41).
- [Q-M1f.17](../open_questions.md#q-m1f17--what-a-view-that-never-loads-reads-from-a-container): a view that never loads the program draws the program the container stores.

### The read-out (S1f.5.16, .6)

- [Q-M1f.18](../open_questions.md#q-m1f18--which-object-the-word-model-names-on-the-read-out): *model* on the read-out keeps meaning the claim form's `(model …)`.
- [Q-M1f.19](../open_questions.md#q-m1f19--a-texts-line-break-on-the-read-out): a line break inside a text stays within its row.
- ★ [Q-M1f.20](../open_questions.md#q-m1f20--route-2s-licence-no-licence-on-one-row-l7-the-cube-on-another) and [Q-M1f.21](../open_questions.md#q-m1f21--regionstoppedlayer-on-two-routes): route 2's licence is named differently on two rows, and `region.stopped.layer` means
  different things on two routes. Both are documented, not fixed. This arguably goes against "one tool, consistent
  answers".
- [Q-M1f.22](../open_questions.md#q-m1f22--a-class-label-of-sixteen-characters-glues-to-its-value): the glued label was handed to S1f.5.6, which filed and fixed it as W42.
- [Q-M1f.23](../open_questions.md#q-m1f23--what-the-route-row-names-where-root-decides-a-flagged-programs-minimal-line): the `route` row is unchanged where root decides a flagged program's minimal line.
- [Q-M1f.24](../open_questions.md#q-m1f24--a-second-surface-that-glues-another-row-of-w42-or-a-case-of-its-own): the second output that glues its label (`-H`) became a second W42 row.

### Cores (S1f.5.12, .8)

- ★ [Q-M1f.25](../open_questions.md#q-m1f25--the-state-routes-members-of-n-it-did-not-saturate-whose-preparation-takes-their-cores): members of N that the state route never saturated take their cores from the walk's snapshot. 58 cores
  changed on the zebra2 family, all accepted as explanations; F22 makes cores canonical.
- [Q-M1f.26](../open_questions.md#q-m1f26--not-kept-retires-and-what-a-member-with-no-core-prints): `not-kept` retired.
- [Q-M1f.27](../open_questions.md#q-m1f27--what-program-a-einbs-answer-directory-writes): a `.einb`'s answer directory writes the stored program, so its state files keep unused imported
  declarations.
- [Q-M1f.28](../open_questions.md#q-m1f28--is-a-not-dead-nodes-core-written--a-wrong-answer): the change from `[]` to `null` is not a register row.
- [Q-M1f.29](../open_questions.md#q-m1f29--a-dead-roots-core-under---count): a dead root's core under `--count` stays `null`, on 39 programs.

### `eq` and `neq` (S1f.5.7)

- [Q-M1f.30](../open_questions.md#q-m1f30--a-form-that-holds-a-variable-as-a-guards-argument): a form that holds a variable is refused at load as a guard's argument.
- ★ [Q-M1f.31](../open_questions.md#q-m1f31--not-eq-x-y-a-stored-negation-or-neq-x-y): `(not (eq x y))` means `(neq x y)`.
- ★ [Q-M1f.32](../open_questions.md#q-m1f32--an-hrules-eq-or-neq-instance): an hrule never proposes an `eq` or `neq` fact.
- ★ [Q-M1f.33](../open_questions.md#q-m1f33--a-written-fact-of-eq): a written `(eq a b)` is a checked conclusion and is never stored; a false one is `(false)`.
  - Oddity: `(eq a b)` parses as a top-level fact and `(neq a b)` does not.
- [Q-M1f.34](../open_questions.md#q-m1f34--where-a-guards-free-variable-is-bound-beside-an-absent): a guard binds where it is written, relative to an `(absent …)`.
- [Q-M1f.35](../open_questions.md#q-m1f35--a-goals-absent--whose-row-is-it): a goal's `(absent …)` is this stage's own row, W56.

### Load refusals (S1f.5.10, .18, .9)

- [Q-M1f.36](../open_questions.md#q-m1f36--what-the-refusal-replaces-which-of-it-is-a-register-row) and [Q-M1f.44](../open_questions.md#q-m1f44--what-the-load-time-check-replaces-which-of-it-is-a-register-row): a run that stops and a run refused at load count as two different answers, so each change between
  them is a register row.
- [Q-M1f.37](../open_questions.md#q-m1f37--fireerrorunboundvars-last-path-w33s-cause): the last path to `FireError::UnboundVar` moved to S1f.5.18, which retired the variant.
- [Q-M1f.38](../open_questions.md#q-m1f38--the-refusals-line-its-separator-and-a-disjunction): the format of the refusal line.
- [Q-M1f.39](../open_questions.md#q-m1f39--the-one-corpus-program-the-check-refuses): the one corpus program the check refused was rewritten.
- [Q-M1f.40](../open_questions.md#q-m1f40--where-the-fuzz-property-lives-and-what-it-reads): the fuzz property lives in `fuzz_properties`; `ein-ir` takes `ein-infer` as a dev-dependency only.
- ★ [Q-M1f.41](../open_questions.md#q-m1f41--what-can-hold-a-fact-means-for-g-14s-derived-activator): "can hold a fact" is read by declared types.
  - That reading refuses the two `g14` programs; the literal one refused 32, `zebra` among them.
  - The cost: declaring a marker relation can now get a program refused, where the same program without the
    declaration loads.
- [Q-M1f.42](../open_questions.md#q-m1f42--a-conclusion-whose-relation-is-a-parameter): a conclusion whose relation is a parameter is allowed.
- [Q-M1f.43](../open_questions.md#q-m1f43--planactivator_args-once-a-nested-fact-binds): `Plan::activator_args` is every argument, as values.
- ★ [Q-M1f.45](../open_questions.md#q-m1f45--the-and-refusals-line-and-or): `(and)` and `(or)` stay parse errors, with a note that the empty conjunction is `()`.
- ★ [Q-M1f.46](../open_questions.md#q-m1f46--a-relation-parameter-bound-to-eq-at-another-arity) and [Q-M1f.47](../open_questions.md#q-m1f47--a-written-fact-of-eq-at-another-arity): `eq` at another arity is refused at load, both through a relation parameter and as a written
  fact.
- [Q-M1f.48](../open_questions.md#q-m1f48--how-a-text-surface-writes-the-empty-core): an empty core prints `∅` on text surfaces.

### Testing (S1f.5.13, .19)

- [Q-M1f.50](../open_questions.md#q-m1f50--where-the-heavy_dump-line-goes-and-which-entries-carry-the-flag): the `heavy_dump` threshold is 40 000 state files, which puts 5 entries in the nightly.
- ★ [Q-M1f.51](../open_questions.md#q-m1f51--three-more-tests-whose-slow-half-no-ci-tier-runs): the slow halves of three tests run in no CI tier: `route_invariance` (857 s), `region` (38 s) and
  `obligation_rung` (71 s). All three passed when last run (S1f.5.13 § T1 record, 2026-11-09).
- [Q-M1f.52](../open_questions.md#q-m1f52--the-nightlys-review-bundle-and-the-registers-runs-that-are-not-a-solve): the nightly's red was reported here; the coordinator put the fix in its own stage, S1f.5.19.
- [Q-M1f.53](../open_questions.md#q-m1f53--how-the-per-commit-gate-catches-a-register-row-the-bundle-cannot-read): the per-commit `--smoke` reads every register row and builds two command cases (0.82 s → 1.33 s,
  debug, 2026-11-09).
- [Q-M1f.54](../open_questions.md#q-m1f54--the-2026-11-06-inventorys-evidence-in-buildreview): the 2026-11-06 inventory's evidence moved to `build/inventory-2026-11-06/`.
- [Q-M1f.55](../open_questions.md#q-m1f55--a-container-on-a-solves-now): a solve on a container is shown as a solve on a container.

### `:why` and English (S1f.5.14)

- [Q-M1f.56](../open_questions.md#q-m1f56--what-the-nl-census-counts-which-fact-set-and-which-registers): which fact set and which registers the census counts.
- [Q-M1f.57](../open_questions.md#q-m1f57--which-fixtures-get-a-why): the 53 files refused at the parse get no `:why`. Where a refusal prints a position, the `:why` sits on the
  form's last line, so the position does not move.
- ★ [Q-M1f.58](../open_questions.md#q-m1f58--where-a-fact-is-rendered-in-english-and-what-it-reads-from): English goes to the state files and `--trace`, not to stdout's answer table.
- [Q-M1f.59](../open_questions.md#q-m1f59--what-a-macros-why-renders): a macro's `:why` is not used to render the fact; the rule's is.
- [Q-M1f.60](../open_questions.md#q-m1f60--a-relation-nobody-declares): a fact of an undeclared relation stays an s-expression.
- [Q-M1f.61](../open_questions.md#q-m1f61--a-template-per-goal-conjunct): no `:why` template per goal conjunct.
- [Q-M1f.62](../open_questions.md#q-m1f62--the-option-that-loads-a-program-without-a-why-once-the-default-is-true): `--no-require-why` replaces `--require-why` on the seven commands that load a program.
- ★ [Q-M1f.63](../open_questions.md#q-m1f63--what-each-program-generator-does-about-why) and [Q-M1f.64](../open_questions.md#q-m1f64--existing-whys-that-are-placeholders-or-name-nothing): the program generators write mechanical `:why`s, and 119 of the 785 corpus `:why`s are of the
  `"p holds of {?1}"` kind, on toy relations. They are true but say little. The 66 broken template references the
  new check found were all fixed; none was a register row.


**Ruled 2026-11-10, the overrides.** The user: *"I accept all decisions as is without review. Please remember that any decision may be changed later due to new findings, new research results, new requirements etc."* Every decision taken by default (Q-M1f.8, .10 to .48, .50 to .64) stands as taken, **accepted without review**, and none is a ruling: each may be reopened when a finding, a research result or a requirement bears on it ([open_questions.md](../open_questions.md), its header).


## Findings with no owner

Re-checked on 2026-11-10 with the release binary at `f454de3b`, unless marked otherwise. None is yet a register
row. The register rule applies: a finding that turns out to be a wrong answer is filed before it is fixed.

1. **A compile error's first line differs by command, and prints the IR's Python spelling.** On
   [`examples/broken/compile/nested_or.ein`](../../../examples/broken/compile/nested_or.ein):
   - `ein solve`, `ein oracle` and `ein render lattice` print the message bare;
   - `ein saturate` puts `ein.inference.compile.CompileError:` in front, a Python class name;
   - `ein render kb` puts the file name in front;
   - `ein render program` and `ein render rules` draw the rule and exit 0.

   Every one prints `SForm(head=Atom(name='or'), …)` and cites "S1.8.A13" and "the M1 compiler", and the
   fixture's `nested_or.expected` pins that text. This breaks the one-tool rule.

**Ruled 2026-11-10.** The user: *"new stage in p1f.5"* → [S1f.5.21](s1f.5.21_a_compile_error_in_eins_words.md).

2. **A non-UTF-8 imported module is reported as missing.** With `(import badmod :symbols (q))` and a
   `badmod.ein` holding a `0xff` byte, `ein solve` prints
   `kb load error: (import badmod) — module not found at …/badmod.ein (None)`, so a Rust `None` leaks into the
   message. `ein solve badmod.ein` on the same file prints the right line, `badmod.ein:3:4: byte 0xff is not UTF-8:
   …`. The cause is `imports.rs`'s `read_to_string(…).ok()`.

**Ruled 2026-11-10.** The user: *"new stage - Unicode is not allowed anywhere else than inside string literals, and the error must say it explicitely"* → [S1f.5.22](s1f.5.22_unicode_only_in_a_string.md). Its census: no corpus program writes a character that is not ASCII outside a string or a comment, and 429 of 454 write one in a comment, which is [Q-M1f.65](../open_questions.md#q-m1f65--may-a-comment-hold-unicode).

3. **`(p (open x))` loads and answers**: R has 2 states. Yet no rule may match `(open …)`, the verdict atom. It is
   open whether such a fact should be refused at load.

**Ruled 2026-11-10.** The user: *"new stage, Explain in more details, with program example, `open` is reserved word, here it appers as what? as relation name? - then it is an error, if not - then explain"* → [S1f.5.23](s1f.5.23_open_is_reserved.md), whose T1 record explains each place `open` can be written, with a program and today's answer for each. In `(p (open x))` it is a relation name, the head of a nested fact, so it is refused. As a plain value, `(p open)`, it is [Q-M1f.66](../open_questions.md#q-m1f66--may-open-be-written-as-a-value).

4. **A macro template `(eq ?a)` loads, then every command refuses it at compile, all in the same way** (S1f.5.9's
   record; not re-checked).

**Ruled 2026-11-10.** The user: *"new stage, explain"* → [S1f.5.24](s1f.5.24_a_macro_body_checked_where_a_rule_is.md), whose T1 record explains it. A macro's body is a `Template`, which takes any head at any arity because its variables stand for whole premises, so the grammar never reads the expanded rule and the compiler is the first to see `(eq ?x)`. `kb save` and the two views accept the program. Where it should be refused is [Q-M1f.67](../open_questions.md#q-m1f67--where-is-a-macro-body-that-no-rule-could-use-refused).

5. **New today: `ein saturate … | head` panics.** It prints `failed printing to stdout: Broken pipe (os error 32)`
   and exits 101. `ein solve … | head` exits 0.

**Ruled 2026-11-10.** The user: *"new stage, investigate and exaplain the root cause, propose fix, then I will decide"* → [S1f.5.25](s1f.5.25_a_closed_stdout.md), whose T1 record has the root cause. Rust ignores `SIGPIPE`, so a write to a closed pipe fails, and `print!` panics on any write error, which exits 101. `ein solve` exits 0 only because its read-out is one write that fits the pipe's buffer, and `ein solve --dump-config | head -1` exits 101. The fix is [Q-M1f.68](../open_questions.md#q-m1f68--what-ein-does-when-its-reader-closes-stdout).

6. **The tutorial is broken.**
   [`docs/guide/04_solving_the_whole_puzzle.md`](../../../docs/guide/04_solving_the_whole_puzzle.md) line 13 writes
   the retired `(config :enable-pre-branch-lookahead true)`, so a reader who copies it gets a refusal.

**Ruled 2026-11-10.** The user: *"fix the tutorial, new stage"* → [S1f.5.26](s1f.5.26_the_tutorial_runs.md).

7. **Stale slow counts.** There are 14 slow entries (`grep -c '^slow *= *true' corpus/corpus.toml`, 2026-11-10),
   pinned by `ein-corpus`'s `the_slow_set_is_exactly_these_fourteen`. But:
   - the `corpus/corpus.toml` header says *"Two entries are"* slow;
   - [`corpus/README.md`](../../../corpus/README.md) line 228 says *"Three"*;
   - that README and `ein-cli/tests/corpus_cli.rs` line 112 cite the test as `…_these_three`.

**Asked 2026-11-10.** The user: *"so what? what does mean slow here? slow for what? compared to what?"*

**The answer.** `slow = true` is a flag on a corpus entry in `corpus/corpus.toml`. It means *the entry's declared
runs cost 1 s or more of release wall time, together* ([`corpus/README.md` § `slow`](../../../corpus/README.md)).
The sum is measured by `utils/corpus_cost.py` and recorded in the entry's `cost_ms`.
- **Slow for what:** the per-commit gate. `cargo test`'s corpus sweep (`corpus_cli`) skips a slow entry unless
  `EIN_CORPUS_SLOW=1`, so the gate stays short. The nightly's `deep-corpus` job sets it and runs them.
- **Compared to what:** the fixed 1 s threshold, not other entries. Two tests hold the flag to it both ways:
  `slow_matches_the_recorded_cost`, exact against `cost_ms`, and `the_slow_flag_still_describes_the_sweep`, against
  the sweep's own wall clock at a 4× tolerance.

**So what:** no behaviour is wrong. 14 entries are flagged, the test pins the set, and CI runs them nightly.
What is wrong is prose. Two places state a count that drifted (*two*, *three*), and one of them names
`features/04_open`, which has not been slow since M1e2 S1e2.1a.5. Two cite a test by a name it no longer has
(`…_these_three`), so a reader who greps for it finds nothing. That is Q-M1e.4's rule broken: a count in prose
with no test or command behind it. The fix is to cite the test and drop the number, placed by default in
[S1f.5.28](s1f.5.28_the_unchecked_findings_triaged.md) as its item 11.


8. **[`run_tests.sh`](../../../run_tests.sh) line 81 says the fuzz and bench jobs are in the nightly "because they
   are cheap"**, but the fuzz job takes about 25 min per seed since the parser swap.

**Ruled 2026-11-10.** The user: *"reduce nightly job, remove long running tests, we will eventually run them on dev host, retain critical tests in CI"* → [S1f.5.27](s1f.5.27_the_nightly_reduced.md).

9. **Taken from the stage records and not re-checked today:**
   - [`region_dump.md`](../../../docs/kernel/inference/region_dump.md) never names seven fields: `reads.dynamic_*`,
     `absent_reads.dynamic_*`, `cycles[].negative`, `paths.dead_parents`, `paths.saturations`,
     `paths.unprinted.saturations` and `state.stale_nodes`.
     [`configuration.md`](../../../docs/kernel/configuration.md) never names `leftover.states_by_relation`
     (S1f.5.16).
   - `-t`'s `route` row and the cube's counts print without digit groups (S1f.5.16).
   - Route 1's "stopped before layer L" is reached by no CLI run (S1f.5.16).
   - `--trace` prints a `:why` or `:source` as markdown paragraphs, where a single line break is a soft break
     (S1f.5.16).
   - Stale comments: `ein-core/src/terms.rs` (the SYMBOL exclusion and `LEXER_KEYWORDS`), `FIFO_GAP_PRIORITY` in
     `ein-infer/tests/detector.rs`, the headers of `form-in-guard.ein` and `guard-before-binder.ein`, and the
     `corpus/fuzz_findings` notes.
   - `player` is declared `T T` but used with one argument in two Rust tests.
   - `utils/guard_split.py` reports `synth/trigger_hole` as "does not load".
   - `ein kb save` of a container writes 1 288 B, where the `.ein` gives 1 248 B for the same KB (S1f.5.19).
   - The fuzz property costs about 27 s per 20 000 inputs (S1f.5.10).

**Ruled 2026-11-10.** The user: *"new stage for investigation, confirmation, triage and fixes"* → [S1f.5.28](s1f.5.28_the_unchecked_findings_triaged.md), which also takes finding 7's prose.


## `build/`

Approved by the user on 2026-11-10 (*"Yes, build cleanup OK"*), and done the same day.
- **Deleted:** the large outputs of `build/s1f.5.3`, `s1f.5.4`, `s1f.5.5` and `s1f.5.15`, 2 431 244 files and
  159.4 GB. These were cargo targets, binaries, dumps, sweeps, snapshots and review bundles.
- **Kept:** every log, script and patch, and the small text evidence: `.tsv`, `.txt`, `.diff`, `.ein`, `.md` and
  `.json` files of up to 2 MB in a directory of at most 100 files, plus S1f.5.5 T9's two mutant tables.
- **What a commit or a page cites:** every `build/s1f.5.*` path a commit message or a committed page cites is still
  there, except four binaries a commit can rebuild: `s1f.5.4/protobin/ein`, `s1f.5.5/c_ein`,
  `s1f.5.5/t13/before/ein` and `s1f.5.5/t5/xcheck/bin/ein-t5`. A cited directory keeps only its small files.
- **Size:** `build/` is 62 GB, where it was 206 GB (`du -sh build`, 2026-11-10).
- **Left as is:** `ci-container` (38 GB, the image `utils/ci_container.sh` uses), `inventory-2026-11-06` (16 GB,
  the evidence the phase README cites) and `review` (3.1 GB, rebuilt by every nightly).
- **Since then:** when the coordinator came back to the tree later on 2026-11-10, `build/` and `ein.rs/target/`
  were both gone (`ls build`), so every `build/…` path a record cites, logs included, no longer resolves here. The
  stages S1f.5.21 to S1f.5.28 write their evidence under a new `build/`.

## What the close needs

1. **Overrides: none.** The user, 2026-11-10: every decision taken by default stands, accepted without review and
   revisable on a new finding.
2. **The findings: eight new stages in P1f.5**, the user's rulings of 2026-11-10 under each finding above:

   | finding | stage | stops for the user |
   |---|---|---|
   | 1, a compile error's first line | [S1f.5.21](s1f.5.21_a_compile_error_in_eins_words.md) | no |
   | 2, Unicode, and a non-UTF-8 import | [S1f.5.22](s1f.5.22_unicode_only_in_a_string.md) | [Q-M1f.65](../open_questions.md#q-m1f65--may-a-comment-hold-unicode), ruled (b): comments may |
   | 3, `open` | [S1f.5.23](s1f.5.23_open_is_reserved.md) | [Q-M1f.66](../open_questions.md#q-m1f66--may-open-be-written-as-a-value), ruled (a): refused |
   | 4, `(eq ?a)` in a macro | [S1f.5.24](s1f.5.24_a_macro_body_checked_where_a_rule_is.md) | [Q-M1f.67](../open_questions.md#q-m1f67--where-is-a-macro-body-that-no-rule-could-use-refused), ruled: allowed unused; never executed |
   | 5, `\| head` | [S1f.5.25](s1f.5.25_a_closed_stdout.md) | [Q-M1f.68](../open_questions.md#q-m1f68--what-ein-does-when-its-reader-closes-stdout), ruled (b): `SIGPIPE`'s default |
   | 6, the tutorial | [S1f.5.26](s1f.5.26_the_tutorial_runs.md) | no |
   | 7, the slow counts | answered above; the prose fix in S1f.5.28, by default | no |
   | 8, the nightly | [S1f.5.27](s1f.5.27_the_nightly_reduced.md) | no |
   | 9, the unchecked findings | [S1f.5.28](s1f.5.28_the_unchecked_findings_triaged.md) | only for what no criterion settles |

When they are done, the close record goes into the phase README. P1f.6, the calculator, is next, and its S1f.6.1
stops for the user.
