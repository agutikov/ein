# Open Questions — M1f

Questions **this milestone raises**, with sticky `Q-M1f.<n>` ids. Do not reuse
a closed id.

M1f was created on 2026-08-29 by moving two phases out of
M1e. The first questions it raised came from
S1f.5.5's paper grammar on 2026-11-08.

**Every row marked *defaulted* was accepted by the user on 2026-11-10, without review**, at P1f.5's close
(close_review.md): *"I accept all decisions as is
without review. Please remember that any decision may be changed later due to new findings, new research results,
new requirements etc."* So a defaulted row stands as taken and is not a ruling. Any one may be reopened, naming
what bears on it. That acceptance covers the rows defaulted by then, up to Q-M1f.64; from Q-M1f.69 on, a row
defaulted by a stage the close re-opened is that stage's, recorded in its file.

| id | question | state |
|---|---|---|
| [Q-M1f.1](#q-m1f1--g-14-is-a-bad-conclusion-that-never-fires-refused) | G-14: is a bad conclusion that never fires refused? | **ruled 2026-11-08, (b)**; S1f.5.5 and S1f.5.18; **built 2026-11-09** by S1f.5.18 (T1f.5.18.3, W59): a program in which a rule could derive an activator binding a relation parameter to a fact is refused at load, read off the program (Q-M1f.41) |
| [Q-M1f.2](#q-m1f2--the-55-files-the-grammar-refuses-that-todays-parser-accepts) | the 55 files the grammar refuses that today's parser accepts | **ruled 2026-11-08, (a)**; S1f.5.5 |
| [Q-M1f.3](#q-m1f3--three-wrong-answers-the-census-found-with-no-row) | three wrong answers the census found with no row | **ruled 2026-11-08, as recommended**; S1f.5.0 files W33–W35; W33 **fixed 2026-11-09** by S1f.5.18 (T1f.5.18.2), W34 by S1f.5.9 (T1f.5.9.2) and W35 by S1f.5.5's swap (`8c3fe86e`) |
| [Q-M1f.4](#q-m1f4--removing-priority-moves-the-facts-of-cores-on-seven-corpus-programs) | removing `:priority` moves the facts of cores on seven corpus programs | **ruled 2026-11-08, (b)**: derive the order from the rules first; S1f.5.15 |
| [Q-M1f.5](#q-m1f5--the-alternatives-flag-moves-refuted-by-beside-an-unmoved-core) | the alternatives flag moves `refuted by` beside an unmoved core | **ruled 2026-11-08, (a)**: W36, open, owned by F22; S1f.5.15 files it |
| [Q-M1f.6](#q-m1f6--the-derived-agenda-holds-the-cores-the-fifo-boundary-does-not) | the derived agenda holds the cores; the FIFO boundary does not | **ruled 2026-11-08, (a)** (re-ruled the same day from (b)): plain FIFO, a core is an explanation, F22 makes it canonical; S1f.5.15. **Its order superseded by [Q-M1f.313](#q-m1f313--may-the-agenda-inside-a-routes-node-fire-first-what-leads-to-false) (c), 2026-11-16**: the agenda fires first what leads to `(false)`, FIFO within each half; a core is still an explanation |
| [Q-M1f.7](#q-m1f7--where-may-a-program-write-the-kernels-dunders) | where may a program write the kernel's dunders? | **ruled 2026-11-08, (b)**: never bound; S1f.5.5's T11 |
| [Q-M1f.8](#q-m1f8--the-parsers-remaining-cost-within-grmtools-or-past-it) | the parser's remaining cost: within grmtools, or past it? | **defaulted 2026-11-09 (the user's instruction): within grmtools; alternatives parked for the close**; S1f.5.5's T12 |
| [Q-M1f.9](#q-m1f9--what-a-deleted-loader-line-leaves-behind) | what a deleted loader line leaves behind | **ruled 2026-11-09, (a)** (*"ok, keep (a), continue"*; defaulted the same day): an `expect`, `unreachable!` or `assert!` naming the grammar's production; a hand-built `Ast` the grammar would refuse panics, which no `ein` command reaches; S1f.5.5's T5 |
| [Q-M1f.10](#q-m1f10--the-einb-ints-section-once-the-int-kind-is-gone) | the `.einb` `INTS` section, once the int kind is gone | **re-ruled 2026-11-13, (b)** (*"yes, do it"*): dropped at format 2.0, whose major moved for `META`'s timestamp; defaulted (a) 2026-11-09; S1f.5.4's T1f.5.4.3 |
| [Q-M1f.11](#q-m1f11--a-template-reference-to-a-variable-no-reference-can-name) | a template reference to a variable no reference can name | **defaulted 2026-11-09 (the user's instruction): the loader refuses it, one line a reference**; S1f.5.4's T1f.5.4.5 |
| [Q-M1f.12](#q-m1f12--which-surfaces-carry-natural-language-text) | which surfaces carry natural-language text | **defaulted 2026-11-09 (the user's instruction): those that carried it before the swap, each held byte for byte; none added**; S1f.5.2's T1f.5.2.2 |
| [Q-M1f.13](#q-m1f13--f3s-cause-reaches-a-disjunction-whose-row-is-it) | F3's cause reaches a disjunction: whose row is it? | **defaulted 2026-11-09 (the user's instruction): W39, `open`, deferred to F22, as W25 is**; S1f.5.1's T1f.5.1.3 |
| [Q-M1f.14](#q-m1f14--which-keyword-pair-arms-go-once-no-term-holds-one) | which keyword-pair arms go, once no term holds one? | **defaulted 2026-11-09 (the user's instruction): the drops, the validator's skips and the renderers' arms; the walkers that descend into a pair's value stay**; S1f.5.11's T1f.5.11.2 |
| [Q-M1f.15](#q-m1f15--where-does-a-position-in-a-containers-stored-program-point) | where does a position in a container's stored program point? | **defaulted 2026-11-09 (the user's instruction): it names the container, `<program stored in x.einb>`, and the stored text's line; a source position needs a format change, not made**; S1f.5.5's T1f.5.5.13 |
| [Q-M1f.16](#q-m1f16--which-of-the-reads-defects-are-register-rows) | which of the read's defects are register rows? | **defaulted 2026-11-09 (the user's instruction): each refusal of a program another command answers (W40's six, W41); the lines and exits of a file every command refuses are held by a test**; S1f.5.5's T1f.5.5.13 |
| [Q-M1f.17](#q-m1f17--what-a-view-that-never-loads-reads-from-a-container) | what a view that never loads reads from a container | **defaulted 2026-11-09 (the user's instruction): the program it stores, resolved and import-flattened**; S1f.5.5's T1f.5.5.13 |
| [Q-M1f.18](#q-m1f18--which-object-the-word-model-names-on-the-read-out) | which object the word *model* names on the read-out (Q-M1e.7's read-out half) | **defaulted 2026-11-09 (the user's instruction): (a), document both objects; nothing printed moves**; S1f.5.16 |
| [Q-M1f.19](#q-m1f19--a-texts-line-break-on-the-read-out) | a text's line break on the read-out | **defaulted 2026-11-09 (the user's instruction): each line at the text's column, each counted against stdout's 100**; S1f.5.16's T1f.5.16.5 |
| [Q-M1f.20](#q-m1f20--route-2s-licence-no-licence-on-one-row-l7-the-cube-on-another) | route 2's licence: *no licence* on one row, `L7 the cube` on another | **defaulted 2026-11-09 (the user's instruction): documented, not moved**; S1f.5.16 |
| [Q-M1f.21](#q-m1f21--regionstoppedlayer-on-two-routes) | `region.stopped.layer` on two routes | **defaulted 2026-11-09 (the user's instruction): documented per route, not moved**; S1f.5.16 |
| [Q-M1f.22](#q-m1f22--a-class-label-of-sixteen-characters-glues-to-its-value) | a class label of sixteen characters glues to its value | **defaulted 2026-11-09 (the user's instruction): documented, not fixed here; the stage that fixes it files its row first**; S1f.5.16. **Fixed 2026-11-09** by S1f.5.6's T1f.5.6.6 (`31e9c7f7`), (b): W42 filed by T1f.5.6.5, `-H`'s form its second row ([Q-M1f.24](#q-m1f24--a-second-surface-that-glues-another-row-of-w42-or-a-case-of-its-own)) |
| [Q-M1f.23](#q-m1f23--what-the-route-row-names-where-root-decides-a-flagged-programs-minimal-line) | what the `route` row names where root decides a flagged program's minimal line | **defaulted 2026-11-09 (the user's instruction): (a), the cube and `L7 the cube`, as `-n 1` printed; no new licence**; S1f.5.6's T1f.5.6.2 |
| [Q-M1f.24](#q-m1f24--a-second-surface-that-glues-another-row-of-w42-or-a-case-of-its-own) | a second surface that glues: another row of W42, or a case of its own? | **defaulted 2026-11-09 (the user's instruction): (a), W42's second row, `W42/h-long-relation`**; S1f.5.6's T1f.5.6.5 |
| [Q-M1f.25](#q-m1f25--the-state-routes-members-of-n-it-did-not-saturate-whose-preparation-takes-their-cores) | the state route's members of N it did not saturate: whose preparation takes their cores? | **defaulted 2026-11-09 (the user's instruction): (a), the walk's snapshot, after the walk; 58 cores move on 3 programs**; S1f.5.12's T1f.5.12.1 |
| [Q-M1f.26](#q-m1f26--not-kept-retires-and-what-a-member-with-no-core-prints) | `not-kept` retires, and what a member with no core prints | **defaulted 2026-11-09 (the user's instruction): (a), the status goes, and a renderer refuses a member with no core**; S1f.5.12's T1f.5.12.1 |
| [Q-M1f.27](#q-m1f27--what-program-a-einbs-answer-directory-writes) | what program a `.einb`'s answer directory writes | **defaulted 2026-11-09 (the user's instruction): (a), the program the container stores, as every view of a container reads it**; S1f.5.12's T1f.5.12.5 |
| [Q-M1f.28](#q-m1f28--is-a-not-dead-nodes-core-written--a-wrong-answer) | is a not-dead node's core written `[]` a wrong answer? | **defaulted 2026-11-09 (the user's instruction): (a), no register row: a spelling, fixed with no answer moving**; S1f.5.8's T1f.5.8.1 |
| [Q-M1f.29](#q-m1f29--a-dead-roots-core-under---count) | a dead root's core under `--count` | **defaulted 2026-11-09 (the user's instruction): (a), `null`, as the question prints no core; documented**; S1f.5.8's T1f.5.8.1 |
| [Q-M1f.30](#q-m1f30--a-form-that-holds-a-variable-as-a-guards-argument) | a form that holds a variable, as a guard's argument | **defaulted 2026-11-09 (the user's instruction): refused at load, in a rule and in a goal; a fact that holds no variable is answered**; S1f.5.7's T1f.5.7.1 |
| [Q-M1f.31](#q-m1f31--not-eq-x-y-a-stored-negation-or-neq-x-y) | `(not (eq x y))`: a stored negation, or `(neq x y)`? | **defaulted 2026-11-09 (the user's instruction): `(neq x y)`, in every place**; S1f.5.7's T1f.5.7.1 |
| [Q-M1f.32](#q-m1f32--an-hrules-eq-or-neq-instance) | an hrule's `eq` or `neq` instance | **defaulted 2026-11-09 (the user's instruction): no hypothesis**; S1f.5.7's T1f.5.7.1 |
| [Q-M1f.33](#q-m1f33--a-written-fact-of-eq) | a written fact of `eq` | **defaulted 2026-11-09 (the user's instruction): a conclusion with no premise, checked, never stored**; S1f.5.7's T1f.5.7.1 |
| [Q-M1f.34](#q-m1f34--where-a-guards-free-variable-is-bound-beside-an-absent) | where a guard's free variable is bound, beside an `(absent …)` | **defaulted 2026-11-09 (the user's instruction): where it is written, as a relation premise binds**; S1f.5.7's T1f.5.7.1 |
| [Q-M1f.35](#q-m1f35--a-goals-absent--whose-row-is-it) | a goal's `(absent …)`: whose row is it? | **defaulted 2026-11-09 (the user's instruction): S1f.5.7's, filed (W56) and fixed in a task of its own**; S1f.5.7's T1f.5.7.5 |
| [Q-M1f.36](#q-m1f36--what-the-refusal-replaces-which-of-it-is-a-register-row) | what the refusal replaces: which of it is a register row? | **defaulted 2026-11-09 (the user's instruction): a row per way the engine answers today, W57's four**; S1f.5.10's T1f.5.10.1 |
| [Q-M1f.37](#q-m1f37--fireerrorunboundvars-last-path-w33s-cause) | `FireError::UnboundVar`'s last path: W33's cause | **defaulted 2026-11-09 (the user's instruction): it stays until S1f.5.18, which retires it; W58 filed, S1f.5.18's**; S1f.5.10's T1f.5.10.1 |
| [Q-M1f.38](#q-m1f38--the-refusals-line-its-separator-and-a-disjunction) | the refusal's line: its separator, and a disjunction | **defaulted 2026-11-09 (the user's instruction): ` — `, and the disjuncts that leave the variable unbound named**; S1f.5.10's T1f.5.10.1 |
| [Q-M1f.39](#q-m1f39--the-one-corpus-program-the-check-refuses) | the one corpus program the check refuses | **defaulted 2026-11-09 (the user's instruction): `syntax/rule-nary-and-or` rewritten, its answer unmoved**; S1f.5.10's T1f.5.10.1 |
| [Q-M1f.40](#q-m1f40--where-the-fuzz-property-lives-and-what-it-reads) | where the fuzz property lives, and what it reads | **defaulted 2026-11-09 (the user's instruction): `fuzz_properties`, over every plan an accepted program compiles**; S1f.5.10's T1f.5.10.1 |
| [Q-M1f.41](#q-m1f41--what-can-hold-a-fact-means-for-g-14s-derived-activator) | what *can hold a fact* means, for G-14's derived activator | **defaulted 2026-11-09 (the user's instruction): a declared relation's argument, one inside a fact pattern and one of a relation a parameter names; a marker's argument and a rule's parameter inferred from what the program passes on**; S1f.5.18's T1f.5.18.1 |
| [Q-M1f.42](#q-m1f42--a-conclusion-whose-relation-is-a-parameter) | a conclusion whose relation is a parameter | **defaulted 2026-11-09 (the user's instruction): followed into every rule of its arity whose name the program writes as an argument**; S1f.5.18's T1f.5.18.1 |
| [Q-M1f.43](#q-m1f43--planactivator_args-once-a-nested-fact-binds) | `Plan::activator_args`, once a nested fact binds | **defaulted 2026-11-09 (the user's instruction): every argument, as values, the compile key's list**; S1f.5.18's T1f.5.18.1 |
| [Q-M1f.44](#q-m1f44--what-the-load-time-check-replaces-which-of-it-is-a-register-row) | what the load-time check replaces: which of it is a register row? | **defaulted 2026-11-09 (the user's instruction): a row per way the engine answers today, W59's two**; S1f.5.18's T1f.5.18.1 |
| [Q-M1f.45](#q-m1f45--the-and-refusals-line-and-or) | the `(and)` refusal's line, and `(or)` | **defaulted 2026-11-09 (the user's instruction): a note on the parse error naming the rule and the clause, `(or)` given one too**; S1f.5.9's T1f.5.9.1 |
| [Q-M1f.46](#q-m1f46--a-relation-parameter-bound-to-eq-at-another-arity) | a relation parameter bound to `eq` at another arity | **defaulted 2026-11-09 (the user's instruction): refused at load wherever a parameter can hold `eq`, read as S1f.5.18 reads relation names, premises and conclusions alike; a row per way the engine answers, W60's four**; S1f.5.9's T1f.5.9.1 |
| [Q-M1f.47](#q-m1f47--a-written-fact-of-eq-at-another-arity) | a written fact of `eq` at another arity | **defaulted 2026-11-09 (the user's instruction): refused at load, beside Q-M1f.33's check; W61, this stage's**; S1f.5.9's T1f.5.9.1 |
| [Q-M1f.48](#q-m1f48--how-a-text-surface-writes-the-empty-core) | how a text surface writes the empty core | **defaulted 2026-11-09 (the user's instruction, on the coordinator's note): `∅` on every text surface, where a dead node's core was taken and holds no fact**; S1f.5.9's T1f.5.9.2 |
| [Q-M1f.50](#q-m1f50--where-the-heavy_dump-line-goes-and-which-entries-carry-the-flag) | where the `heavy_dump` line goes, and which entries carry the flag | **defaulted 2026-11-09 (the user's instruction): (a), 40 000 state files, in the gap above the six `slow` `square-*`; five entries flagged, `features/01_not_and_absent` among them**; S1f.5.13's T1f.5.13.1 |
| [Q-M1f.51](#q-m1f51--three-more-tests-whose-slow-half-no-ci-tier-runs) | three more tests whose slow half no CI tier runs | **defaulted 2026-11-09 (the user's instruction): (a), recorded, not scheduled; parked for the phase's close**; S1f.5.13's T1f.5.13.3. **Taken by S1f.5.27 on 2026-11-10, (b) by tier**: `region`'s and `obligation_rung`'s slow halves are CI nightly steps, `route_invariance`'s a dev-host step, and `nightly_steps.rs`' `every_slow_half_runs_on_a_tier` fails on a target that reads `EIN_CORPUS_SLOW` and runs on neither |
| [Q-M1f.52](#q-m1f52--the-nightlys-review-bundle-and-the-registers-runs-that-are-not-a-solve) | the nightly's review bundle and the register's runs that are not a solve | **defaulted 2026-11-09 (the user's instruction): (a), reported, not fixed here; the nightly is red at `utils/review_bundle.py` since `fc53818f`**; S1f.5.13's T1f.5.13.3. **Re-opened the same day as (b)**, taken by default on the user's instruction of 2026-11-09: placed as a stage of its own, S1f.5.19; **built 2026-11-09** (`00c4ed34`): the bundle builds all 175 cases and the nightly is green |
| [Q-M1f.53](#q-m1f53--how-the-per-commit-gate-catches-a-register-row-the-bundle-cannot-read) | how the per-commit gate catches a register row the bundle cannot read | **defaulted 2026-11-09 (the user's instruction): (b), the smoke builds a command case and reads every register row with no run**; S1f.5.19 |
| [Q-M1f.54](#q-m1f54--the-2026-11-06-inventorys-evidence-in-buildreview) | the 2026-11-06 inventory's evidence in `build/review/` | **defaulted 2026-11-09 (the user's instruction): (b), moved intact to `build/inventory-2026-11-06/`, and `build/review/` rebuilt**; S1f.5.19 |
| [Q-M1f.55](#q-m1f55--a-container-on-a-solves-now) | a `container` on a solve's `now` | **defaulted 2026-11-09 (the user's instruction): (a), the case page runs the container, as `wrong_answers.rs` does**; S1f.5.19 |
| [Q-M1f.56](#q-m1f56--what-the-nl-census-counts-which-fact-set-and-which-registers) | what the NL census counts: which fact set, and which registers | **defaulted 2026-11-10 (the user's instruction of 2026-11-09): (a), every fact of every state file, and four registers, puzzle, stdlib, kernel and undeclared**; S1f.5.14 |
| [Q-M1f.57](#q-m1f57--which-fixtures-get-a-why) | which fixtures get a `:why` | **defaulted 2026-11-10 (the user's instruction of 2026-11-09): (a), every file the loader reads, on the form's last line in a refused fixture; a file refused at the parse and the records under `plans/` and `docs/history/` are left**; S1f.5.14 |
| [Q-M1f.58](#q-m1f58--where-a-fact-is-rendered-in-english-and-what-it-reads-from) | where a fact is rendered in English, and what it reads from | **rewritten 2026-11-18** by S1f.23.5's rulings and build (`bd21f0bb`, merged by `79dd213b`): a `:why` reads the form it is written on, a declaration's `{?n}` its n-th arrow; a fact of a relation takes no text from the declaration and reads by its `:source`, its rule's `:why`, or as written; the decision file's title extended, slug kept (Q-M1f.480). Before: **defaulted 2026-11-10 (the user's instruction of 2026-11-09): (a), a state file's comments and `--trace`'s *Derives* line, from the fact's first recorded justification; stdout does not move**; S1f.5.14 |
| [Q-M1f.59](#q-m1f59--what-a-macros-why-renders) | what a macro's `:why` renders | **defaulted 2026-11-10 (the user's instruction of 2026-11-09): (a), nothing: it documents the pattern, and the rule that used it says what it meant there**; S1f.5.14 |
| [Q-M1f.60](#q-m1f60--a-relation-nobody-declares) | a relation nobody declares | **defaulted 2026-11-10 (the user's instruction of 2026-11-09): (a), no slot: a derived fact reads by its reason, a stated one falls back, counted under `stdlib` or `undeclared`**; S1f.5.14 |
| [Q-M1f.61](#q-m1f61--a-template-per-goal-conjunct) | a template per goal conjunct | **defaulted 2026-11-10 (the user's instruction of 2026-11-09): (a), no change**; S1f.5.14 |
| [Q-M1f.62](#q-m1f62--the-option-that-loads-a-program-without-a-why-once-the-default-is-true) | the option that loads a program without a `:why`, once the default is true | **defaulted 2026-11-10 (the user's instruction of 2026-11-09): (a), `--no-require-why` replaces `--require-why`, and `LoadOptions::default()` requires**; S1f.5.14 |
| [Q-M1f.63](#q-m1f63--what-each-program-generator-does-about-why) | what each program generator does about `:why` | **defaulted 2026-11-10 (the user's instruction of 2026-11-09): (a), the engine fuzzers and synthesisers write an honest mechanical `:why` on every form, keeping their random draws; `fuzz_properties` keeps its draws**; S1f.5.14 |
| [Q-M1f.64](#q-m1f64--existing-whys-that-are-placeholders-or-name-nothing) | existing `:why`s that are placeholders or name nothing | **defaulted 2026-11-10 (the user's instruction of 2026-11-09): (a), each fixed, the references by the check's commit and the placeholders by the migration; none is a register row**; S1f.5.14 |
| [Q-M1f.65](#q-m1f65--may-a-comment-hold-unicode) | may a comment hold Unicode? | **ruled 2026-11-10, (b)**: a comment may hold Unicode; S1f.5.22 |
| [Q-M1f.66](#q-m1f66--may-open-be-written-as-a-value) | may `open` be written as a value? | **ruled 2026-11-10, (a)**: refused; `open` is written only as a rule's verdict; S1f.5.23 |
| [Q-M1f.67](#q-m1f67--where-is-a-macro-body-that-no-rule-could-use-refused) | where is a macro body that no rule could use refused? | **ruled 2026-11-10**, in the user's words: an unused malformed macro is allowed, and what matters is that `(eq <arg>)` is never executed; S1f.5.24 |
| [Q-M1f.68](#q-m1f68--what-ein-does-when-its-reader-closes-stdout) | what Ein does when its reader closes stdout | **ruled 2026-11-10, (b)**: handle `SIGPIPE` properly, its default disposition restored on the binary; S1f.5.25; **built 2026-11-10** by S1f.5.25 (T1f.5.25.2, W75): `ein-cli/src/main.rs`'s `restore_sigpipe`, one audited `libc::signal(SIGPIPE, SIG_DFL)` as `main`'s first statement, Unix only |
| [Q-M1f.69](#q-m1f69--what-the-guides-pin-reads-in-a-program-block) | what the guide's pin reads in a program block | **defaulted 2026-11-10 (the stage's default, S1f.5.26): (a), the text decides: a block that writes `…` outside a string is an excerpt, a top-level form with one is elided; an excerpt's whole forms load together where they stand, and a whole program loads and `ein solve` answers it**; S1f.5.26 |
| [Q-M1f.70](#q-m1f70--what-the-stage-does-with-what-t1-found-false-beside-the-blocks) | what the stage does with what T1 found false beside the blocks | **defaulted 2026-11-10 (the stage's default, S1f.5.26): (a), each fixed in the page: a block that misquotes its file or answers otherwise than its page says, and four sentences of prose; the test does not hold them**; S1f.5.26 |
| [Q-M1f.71](#q-m1f71--does-a-view-draw-a-program-the-loader-refuses) | does a view draw a program the loader refuses? | **defaulted 2026-11-10 (the stage's default, S1f.5.21): (a), it refuses it: `render program`, `render rules`, `render rule` and `render constraints` read the program through the loader first, refuse what it refuses with its line, exit 1, and take `--no-require-why` as the other seven commands that load do**; S1f.5.21 |
| [Q-M1f.72](#q-m1f72--the-line-a-moved-refusal-prints) | the line a moved refusal prints | **defaulted 2026-11-10 (the stage's default, S1f.5.21): (a), the loader's lines as S1f.5.9's, .10's and .18's print, `rule '<name>': …` behind `kb load error: `, no position; the compiler's remaining line `<file>:<line>:<col>: rule '<name>': …`, its form as Ein text**; S1f.5.21 |
| [Q-M1f.73](#q-m1f73--how-the-loader-counts-the-register-ceiling) | how the loader counts the register ceiling | **defaulted 2026-11-10 (the stage's default, S1f.5.21): (a), one count, every distinct variable the `:match` writes and the rule's parameters, or the `:goal`'s, at most 256: a bound on each register file the compiler builds, so its arm is an invariant**; S1f.5.21 |
| [Q-M1f.74](#q-m1f74--a-nested-or--a-macros-expansion-writes) | a nested `(or …)` a macro's expansion writes | **defaulted 2026-11-10 (the stage's default, S1f.5.21): (a), refused at load as a written one is, the loader reading the expanded rule; within Q-M1f.67's ruling, which allows a refusal at instantiation, and an unused macro still loads**; S1f.5.21 |
| [Q-M1f.75](#q-m1f75--the-compilers-arms-no-program-reaches-and-d-1s-and-d-2s-other-lines) | the compiler's arms no program reaches, and D-1's and D-2's other lines | **defaulted 2026-11-10 (the stage's default, S1f.5.21): (a), the three a direct caller of `compile_rule` reaches stay its errors, in Ein's words, and the moved arms and the `_` head become invariants; `FileNotFoundError` stays F23's (the read path), and `loc_repr`'s `Loc(…)` is taken here (`<file>:<line>:<col>`), `at None` staying F23's**; S1f.5.21 |
| [Q-M1f.76](#q-m1f76--which-of-what-the-move-replaces-is-a-register-row) | which of what the move replaces is a register row | **defaulted 2026-11-10 (the stage's default, S1f.5.21): (a), a row per way the engine answers today (Q-M1f.36's reading), W62's six, W63's three, W64's two and W65's five, each fixed by T1f.5.21.2; `features/02`, the one `positive` program the move refuses, rewritten with no row (Q-M1f.39's reading)**; S1f.5.21 |
| [Q-M1f.77](#q-m1f77--when-is-utilsci_containersh-part-of-done) | when is `utils/ci_container.sh` part of done? | **defaulted 2026-11-10 (the stage's default, S1f.5.29): (a), a stage that changes a DOT writer, the review bundle or what CI installs is not done until `utils/ci_container.sh` is green on its commit; AGENTS.md's *Running the gate* says so**; S1f.5.29 |
| [Q-M1f.78](#q-m1f78--where-the-runners-missing-fdp-is-fixed) | where the runner's missing `fdp` is fixed | **defaulted 2026-11-10 (the coordinator's default, S1f.5.29): (a), the runner installs the plugin: `libgvplugin-neato-layout8` beside `graphviz` in the three workflows and `utils/ci_container.sh`'s image, and `run_tests.sh` exits 127 where `fdp` lays out no graph; no view, golden or drawing moves**; S1f.5.29 |
| [Q-M1f.79](#q-m1f79--does-a-refused-dot-fail-the-whole-bundle) | does a refused DOT fail the whole bundle? | **defaulted 2026-11-10 (the coordinator's default, S1f.5.29): (a), yes, as in the smoke: `utils/review_bundle.py` exits 1 on a DOT Graphviz refused, printing what Graphviz said under its path; a lattice past `DAG_NODES_DRAWN`, kept undrawn by design, says so beside its DOT and is not a refusal**; S1f.5.29 |
| [Q-M1f.80](#q-m1f80--where-a-malformed-primitive-a-macros-expansion-writes-is-refused) | where a malformed primitive a macro's expansion writes is refused | **defaulted 2026-11-10 (the stage's default, S1f.5.24): (a), at load, after expansion, the instantiation the ruling on Q-M1f.67 allows: every command refuses the program, the views and `kb save` among them, before any path compiles, fires, proposes or owes the rule; a macro no rule calls still loads**; S1f.5.24 |
| [Q-M1f.81](#q-m1f81--what-a-malformed-primitive-is) | what a malformed primitive is | **defaulted 2026-11-10 (the stage's default, S1f.5.24): (a), every form the grammar would refuse where it stands if the expanded rule were written: `eq` and `neq` at two values, `absent` at one premise, `not` of one pattern (or, concluded, a variable), `false` with nothing, no `(or …)` as a conclusion, no connective where a value goes, and a conclusion that is a form**; S1f.5.24 |
| [Q-M1f.82](#q-m1f82--the-line-a-malformed-primitive-is-refused-with) | the line a malformed primitive is refused with | **defaulted 2026-11-10 (the stage's default, S1f.5.24): (a), the compiler's, S1f.5.21's spelling, behind `kb load error: `: the position where the expanded form is written, the rule as the loader names it (`hrule 'h'`), the clause, the form as Ein text and what to write instead**; S1f.5.24 |
| [Q-M1f.83](#q-m1f83--which-of-what-the-refusal-replaces-is-a-register-row) | which of what the refusal replaces is a register row | **defaulted 2026-11-10 (the stage's default, S1f.5.24): (a), a row per way the engine answered where a refusal is right, W66's four, W67's four, W68's six and W69's two, each fixed by T1f.5.24.3; the compile stop and the firing stop at root, which the ruling allows, are no row, and the views' and `kb save`'s exits on such a program move under Q-M1f.71**; S1f.5.24 |
| [Q-M1f.84](#q-m1f84--the-compilers-arity-arms-and-the-firings-whole-conclusion-line) | the compiler's arity arms and the firing's whole-conclusion line | **defaulted 2026-11-10 (the stage's default, S1f.5.24): (a), invariants, as S1f.5.21's nested `(or …)` arm (Q-M1f.9 (a)'s pattern): the loader's check makes each unreachable, and `fuzz_properties` holds the compiler to refuse nothing the loader accepts**; S1f.5.24 |
| [Q-M1f.85](#q-m1f85--which-findings-p1f7-takes) | which findings P1f.7 takes | **defaulted 2026-11-10 (the coordinator, on the user's instruction of that day): S1f.5.28's items 12 to 16, the batch the coordinator had given it from S1f.5.21's and S1f.5.24's records, as S1f.7.1 to S1f.7.5; and every wrong answer P1f.5's remaining stages find outside their scope, a stage each from S1f.7.6. S1f.5.28 keeps items 1 to 11, the close review's findings 9 and 7, which the user ruled into it**; P1f.7 |
| [Q-M1f.86](#q-m1f86--may-a-macros-body-write-open) | may a macro's body write `open`? | **defaulted 2026-11-10 (the stage's default, S1f.5.23): (a), no: refused at the parse, called or not, as `_` and a string are in a body; the verdict is written in the rule**; S1f.5.23 |
| [Q-M1f.87](#q-m1f87--the-verdicts-loader-lines-once-the-grammar-places-open) | the verdict's loader lines once the grammar places `open` | **defaulted 2026-11-10 (the stage's default, S1f.5.23): (a), the five the grammar covers deleted, each a row of the grammar's table and named in `DELETED`, `validate_open` holding the rest as an invariant; the five that stay start with where the `(open …)` is written, `at None` gone**; S1f.5.23 |
| [Q-M1f.88](#q-m1f88--the-line-open-is-refused-with) | the line `open` is refused with | **defaulted 2026-11-10 (the stage's default, S1f.5.23): (a), the parse error's, ``unexpected `open` ``, the stage's sentence a note under it (`ein_ir::grammar::OPEN_NOTE`), as `_` and a string carry one**; S1f.5.23 |
| [Q-M1f.89](#q-m1f89--which-of-what-the-refusal-replaces-is-a-register-row) | which of what the refusal replaces is a register row | **defaulted 2026-11-10 (the stage's default, S1f.5.23): (a), a row per place with a reading of its own, W70's seven, W71's fourteen and W72's three; the pin holds all fifty-two places**; S1f.5.23 |
| [Q-M1f.90](#q-m1f90--which-of-the-import-reads-moves-are-register-rows-and-where-their-witnesses-live) | which of the import read's moves are register rows, and where their witnesses live | **defaulted 2026-11-10 (the stage's default, S1f.5.22): (a), W74's nine rows on the one witness git can keep, a module that is a directory, one per command run the register can run; a module that is not UTF-8, one that cannot be read and a `.einb` container are the pin's, `ein-cli/tests/unicode_only_in_a_string.rs`, which writes each at run time; and W73's five rows, one per Unicode space and one for a space inside a name**; S1f.5.22 |
| [Q-M1f.91](#q-m1f91--a-module-that-is-a-einb-container) | a module that is a `.einb` container | **defaulted 2026-11-10 (the stage's default, S1f.5.22): (a), refused with a line of its own, naming the module's path and saying an import reads Ein text (`ein_ir::read::container_module_line`)**; S1f.5.22 |
| [Q-M1f.92](#q-m1f92--how-the-line-quotes-a-character-a-reader-cannot-see) | how the line quotes a character a reader cannot see | **defaulted 2026-11-10 (the stage's default, S1f.5.22): (a), as the escape a string writes it with, `'\u{a0}' (U+00A0)`, for a space, a control, a zero-width or bidirectional format character, a combining mark or a variation selector (`ein_ir::grammar::shown_char`); any other character as itself**; S1f.5.22 |
| [Q-M1f.93](#q-m1f93--which-path-a-modules-read-line-names) | which path a module's read line names | **defaulted 2026-11-10 (the stage's default, S1f.5.22): (a), the path the import resolves to, the one `module not found at …` names, relative where the program was given relatively**; S1f.5.22 |
| [Q-M1f.94](#q-m1f94--a-directorys-and-an-unreadable-modules-line-and-a-missing-ones-none) | a directory's and an unreadable module's line, and a missing one's `(None)` | **defaulted 2026-11-10 (the stage's default, S1f.5.22): (a), the program's lines, `IsADirectoryError: …` and `PermissionError: …`, F23's D-1 for both; `module not found at … (None)` for a module that does not exist stays, its `(None)` F23's D-2 with every import line's**; S1f.5.22 |
| [Q-M1f.95](#q-m1f95--how-many-characters-the-refusal-names) | how many characters the refusal names | **defaulted 2026-11-10 (the stage's default, S1f.5.22): (a), the first, where the lexer stops: `lrpar` lexes a file before it parses it, so a lexing error is the file's one error**; S1f.5.22 |
| [Q-M1f.96](#q-m1f96--a-byte-order-mark) | a byte-order mark | **defaulted 2026-11-10 (the stage's default, S1f.5.22): (a), refused as any other character that is not ASCII, at 1:1, `'\u{feff}' (U+FEFF)`; it was a token that is not a name until then**; S1f.5.22 |
| [Q-M1f.97](#q-m1f97--how-a-test-closes-a-runs-stdout) | how a test closes a run's stdout | **defaulted 2026-11-10 (the stage's default, S1f.5.25): (a), the register's runs and the pin's second reader close it before the run starts, so every write meets no reader whatever the timing; the pin's third reader closes after the first line, as `head -1`, on a one-page pipe, and holds a run to `SIGPIPE` where its stdout is more than that line and the page, to `SIGPIPE` or exit 0 where it fits, never 101 and never a word on stderr**; S1f.5.25 |
| [Q-M1f.98](#q-m1f98--which-of-the-closed-stdouts-moves-are-register-rows) | which of the closed stdout's moves are register rows | **defaulted 2026-11-10 (the stage's default, S1f.5.25): (a), W75's eighteen rows, one per command and per `solve` question and `solve -c`, on two corpus programs, each a run that panicked, exit 101, its reader closed before the run; every other option, `--version`, `render region` and an artefact on `/dev/stdout` the pin's; and a move from exit 0 to the signal no row: `clap`'s help, which ignored its failed write, is no answer about a program, and a run whose writes landed before its reader closed exits 0 still**; S1f.5.25 |
| [Q-M1f.99](#q-m1f99--what-the-register-records-of-a-run-a-closed-stdout-ended) | what the register records of a run a closed stdout ended | **defaulted 2026-11-10 (the stage's default, S1f.5.25): (a), schema `/21`: a `now` with `stdout = "closed"` is a command's run whatever its subcommand, and answers by `signal` (13, nothing on stderr) or `panicked` (exit 101, the panic's message); a `right` can be `signal = 13`, and a recorded answer the panic**; S1f.5.25 |
| [Q-M1f.100](#q-m1f100--an-artefact-flag-that-writes-to-a-pipe-whose-reader-has-gone) | an artefact flag that writes to a pipe whose reader has gone | **defaulted 2026-11-10 (the stage's default, S1f.5.25): (a), it ends the process by `SIGPIPE` too, the disposition being process-wide; `defined_behaviour.md` § 4.4's arm, the message and the exit code unchanged, holds for every other failure; shown to the user at the close, since it narrows § 4.4**; S1f.5.25 |
| [Q-M1f.101](#q-m1f101--a-closed-stdout-on-windows) | a closed stdout on Windows | **defaulted 2026-11-10 (the stage's default, S1f.5.25): (a), left as it is, `restore_sigpipe` a no-op there: Windows has no `SIGPIPE`, so a write to a closed pipe is an error and a `print!` would still panic, exit 101, which no host here can run; shown to the user at the close**; S1f.5.25 |
| [Q-M1f.102](#q-m1f102--what-the-ci-nightly-keeps-and-what-the-dev-host-runs) | what the CI nightly keeps, and what the dev host runs | **defaulted 2026-11-10 (the stage's default, S1f.5.27): (a), the stage file's rule: a step that guards an answer or a contract no per-commit step guards stays in CI, at a budget that fits in 5 min on the runner; a step over that, or one that measures, runs on the dev host, which runs every nightly step at full budget too; the stage file's classification confirmed, the fuzzer's budget moved (Q-M1f.103)**; S1f.5.27 |
| [Q-M1f.103](#q-m1f103--the-fuzzers-budget-in-the-ci-nightly) | the fuzzer's budget in the CI nightly | **defaulted 2026-11-10 (the stage's default, S1f.5.27): (a), 20 000 inputs a seed, where the stage file proposed 100 000: on the runner's measured rate 100 000 is past the rule's 5 min, and 20 000 fits; the dev host runs 1 000 000**; S1f.5.27 |
| [Q-M1f.104](#q-m1f104--a-critical-step-the-runners-spread-puts-on-the-line) | a critical step the runner's spread puts on the line | **defaulted 2026-11-10 (the stage's default, S1f.5.27): (a), `corpus_cores`' slow half stays whole in the nightly: 256 s on the latest run, 428 s and 436 s on the two before, on a runner slower in every step, and its slow half has no budget to cut**; S1f.5.27 |
| [Q-M1f.105](#q-m1f105--how-the-dev-hosts-list-is-held) | how the dev host's list is held | **defaulted 2026-11-10 (the stage's default, S1f.5.27): (a), a third copy in `run_tests.sh`, held by `nightly_steps.rs` to the nightly's as a superset whose other steps `DEV_HOST_ONLY` names with why, with a floor, the two fuzz budgets pinned, and a census of the targets that read `EIN_CORPUS_SLOW`**; S1f.5.27 |
| [Q-M1f.106](#q-m1f106--the-whole-review-bundle-on-the-runners-graphviz) | the whole review bundle on the runner's Graphviz | **defaulted 2026-11-10 (the stage's default, S1f.5.27): (a), it runs on the dev host, here or in the runner's image (`utils/ci_container.sh --dev-host`); per commit the runner draws every view with `dot` (`dot_wellformed`) and the smoke's 31 DOTs with their own engines, and Q-M1f.77 is unchanged**; S1f.5.27 |
| [Q-M1f.107](#q-m1f107--where-the-close-reviews-finding-7-is-fixed) | where the close review's finding 7, the stale slow counts, is fixed | **defaulted 2026-11-10 (the coordinator's default, recorded by S1f.5.28): (a), S1f.5.28's item 11, three prose edits citing `the_slow_set_is_exactly_these_fourteen` in place of a count; shown to the user at the close**; S1f.5.28 |
| [Q-M1f.108](#q-m1f108--route-1s-stopped-before-layer-l-which-no-ein-run-prints) | route 1's *stopped before layer L*, which no `ein` run prints | **defaulted 2026-11-10 (the stage's default, S1f.5.28): (a), the row stays for `explore::describe_by`, and `read_out.md` says why no `ein` run prints it, pinned by `read_out_reference.rs`' `route_1_never_stops_on_an_ein_run`**; S1f.5.28 |
| [Q-M1f.109](#q-m1f109--a-texts-line-break-in-the-traces-markdown) | a text's line break in the trace's markdown | **defaulted 2026-11-10 (the stage's default, S1f.5.28): (a), a hard break, two spaces before it, the line after it in its block (`> ` in a `:why`'s quote), as Q-M1f.19 took the read-out's; shown to the user at the close, since it moves the bytes Q-M1f.12 holds the trace to**; S1f.5.28 |
| [Q-M1f.110](#q-m1f110--a-container-saved-again) | a container saved again | **defaulted 2026-11-10 (the stage's default, S1f.5.28): (a), the writer stops at the last provenance record the KB references, so a re-save is byte-identical and no id moves; shown to the user at the close, since S1f.5.19 recorded the growth and left it**; S1f.5.28 |
| [Q-M1f.111](#q-m1f111--the-fuzz-tests-shape) | the fuzz test's shape | **defaulted 2026-11-10 (the stage's default, S1f.5.28): (a), the stream parsed once, on every thread the process may use, each property keeping its test, its place-by-place reading and its own failure report; 4.3× less CPU at 20 000 inputs, 3.9× and 4.2× less wall on four P-core threads**; S1f.5.28 |
| [Q-M1f.114](#q-m1f114--what-a-relation-in-a-solve--list-must-be) | what a relation in a `(solve …)` list must be | **defaulted 2026-11-11 (the stage's default, S1f.7.8): (b), a name the program writes as a relation outside the lists, so a relation only a rule derives is accepted and a name written nowhere else is refused**; S1f.7.8 |
| [Q-M1f.115](#q-m1f115--what-else-of-an-hrules-list-is-refused) | what else of an `:hrules` list is refused | **defaulted 2026-11-11 (the stage's default, S1f.7.8): (a), every bare name is a declared hrule and an activator has as many arguments as its hrule has parameters**; S1f.7.8 |
| [Q-M1f.116](#q-m1f116--an-activators-argument-that-is-a-value) | an activator's argument that is a value | **defaulted 2026-11-11 (the stage's default, S1f.7.8): (b), not checked, and the missing word that an entry proposes nothing is F25's R-2**; S1f.7.8 |
| [Q-M1f.117](#q-m1f117--a-goal-with-a-macro-call-printed-as-written-or-as-expanded) | a goal with a macro call: printed as written, or as expanded | **defaulted 2026-11-11 (S1f.7.5): expanded at load as a rule's `:match` is, matched on the expansion, printed as written**; S1f.7.5 |
| [Q-M1f.118](#q-m1f118--a-macro-call-in-a-claim) | a macro call in a claim | **defaulted 2026-11-11 (S1f.7.5): not expanded; a claim lists facts, so it names no relation and is refused**; S1f.7.5 |
| [Q-M1f.119](#q-m1f119--which-relations-a-goal-may-ask-about) | which relations a goal may ask about | **defaulted 2026-11-11 (the stage's default (a), S1f.7.11): refused at load unless a relation of the program, S1f.7.8's notion without the goals, or one a `:hrules` activator has its hrule propose**; S1f.7.11 |
| [Q-M1f.120](#q-m1f120--a-rules-match-over-a-relation-nothing-makes) | a rule's `:match` over a relation nothing makes | **defaulted 2026-11-11 (the stage's default, S1f.7.11): reported in the stage's record and not refused; F25's R-6**; S1f.7.11 |
| [Q-M1f.121](#q-m1f121--the-line-a-goals-nested-relation-variable-is-refused-with) | the line a goal's nested relation variable is refused with | **defaulted 2026-11-11 (S1f.7.1): the goal's own line, the one its top-level relation variable has had since S1f.5.21, from the one walk a rule's clauses get**; S1f.7.1 |
| [Q-M1f.122](#q-m1f122--where-absent-still-heads-a-form-that-is-no-premise) | where `absent` still heads a form that is no premise | **defaulted 2026-11-11 (S1f.7.7): in a claim's class and nowhere else; the refusal is the grammar's, with a note pointing at `(not …)`**; S1f.7.7 |
| [Q-M1f.123](#q-m1f123--false-with-no-argument-as-a-value) | `(false)` with no argument as a value | **defaulted 2026-11-11 (S1f.7.6): it stays, `(q (false))` and `(not (false))` load; the rule refuses `false` with an argument**; S1f.7.6 |
| [Q-M1f.124](#q-m1f124--where-eq-a-b-is-still-written-once-it-is-no-value) | where `(eq a b)` is still written, once it is no value | **defaulted 2026-11-11 (S1f.7.10): a fact, a premise, a conclusion and a claim of its own, and under a `(not …)` that is one of those; never under a `(not …)` that is a value, in `:using` or in a trace**; S1f.7.10 |
| [Q-M1f.125](#q-m1f125--a-connective-as-a-patterns-argument-the-grammar-or-the-loader) | a connective as a pattern's argument: the grammar or the loader | **defaulted 2026-11-11 (S1f.7.3): the loader, after macro expansion, since `Arg` takes a connective for a macro's call; `and`, `or`, `neq`, `eq`, `absent` and `false` with an argument refused, `not` kept**; S1f.7.3 |
| [Q-M1f.126](#q-m1f126--a-relation-parameter-bound-to-absent-false-or-eq) | a relation parameter bound to `absent`, `false` or `eq` | **defaulted 2026-11-11 (S1f.7.7, S1f.7.6, S1f.7.10): refused at load where § 4.14's reading sees it, fixed in place**; S1f.7.7 |
| [Q-M1f.127](#q-m1f127--what-the-loader-reads-to-see-that-a-negated-variable-can-hold-a-name) | what the loader reads to see that a negated variable can hold a name | **defaulted 2026-11-11 (S1f.7.4): what the program's own text shows, a written fact, a declaration, a rule's or an hrule's conclusion, an activator, a declared relation the hypotheses are taken from and a guard alone; a position that holds a name and a fact is refused; a pattern whose relation is a parameter is read through its written activators, and not through a derived one**; S1f.7.4 |
| [Q-M1f.128](#q-m1f128--what-a-firing-does-with-the-negation-of-a-name-the-loader-did-not-foresee) | what a firing does with the negation of a name the loader did not foresee | **defaulted 2026-11-11 (S1f.7.4): it builds no fact, so the firing stores nothing for that conclusion and an hrule proposes nothing; not `(false)`, not a stop**; S1f.7.4, F25's R-1 |
| [Q-M1f.129](#q-m1f129--the-assertion-that-no-hrule-reads-not-removed-or-kept-where-the-kill-cache-is-on) | the assertion that no hrule reads `not`: removed, or kept where the kill cache is on | **defaulted 2026-11-11 (S1f.7.2): removed, and where an hrule reads `not` every hrule is matched before any candidate is emitted, so the cache's writes are not visible with it on; the rows filed and fixed in one commit**; S1f.7.2 |
| [Q-M1f.130](#q-m1f130--a-macros-call-as-a-conclusion-refused-or-expanded-as-it-is) | a macro's call as a conclusion: refused, or expanded as it is | **defaulted 2026-11-11 (S1f.7.12): expanded, as it is today; the stage's default refuses the call in a fact, and T1 found a conclusion's call expanded already**; S1f.7.12 |
| [Q-M1f.131](#q-m1f131--a-relation-declared-under-a-macros-name) | a relation declared under a macro's name | **defaulted 2026-11-11 (S1f.7.12): refused at load, one line at the declaration; no corpus program declares one**; S1f.7.12 |
| [Q-M1f.132](#q-m1f132--a-stdmacro-macro-the-program-does-not-import-written-as-a-fact) | a `std.macro` macro the program does not import, written as a fact | **defaulted 2026-11-11 (S1f.7.12): refused, as a rule's `:match` writing it has been since S1.8a.f20; a relation of that name is not refused where it is declared**; S1f.7.12 |
| [Q-M1f.133](#q-m1f133--a-macros-call-inside-a-fact-a-claim-lists-and-in-a-facts-using) | a macro's call inside a fact a claim lists, and in a fact's `:using` | **defaulted 2026-11-11 (S1f.7.12): refused as a written fact's is; a claim headed by the call keeps Q-M1f.118's line**; S1f.7.12 |
| [Q-M1f.134](#q-m1f134--what-a-register-row-records-as-right-while-f24s-question-is-not-ruled) | what a register row records as `right` while F24's question is not ruled | **defaulted 2026-11-11 (S1f.7.9): the answer under one named reading of F24's (a), (b), (c), each other reading an alternative, the review `deferred`, as W8's rows record Q-M1e2.2**; S1f.7.9, [F24](../followups/f24_written_provenance.md) |
| [Q-M1f.135](#q-m1f135--which-facts-a-flat-import-brings) | which facts a flat import brings | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.11): a name in the selection's closure selects the module's declaration of it and every top-level fact whose relation it is; a listed name may be a relation the module only states facts of (design.md § G's a3)**; S1f.6.11 |
| [Q-M1f.136](#q-m1f136--what-a-qualified-import-does-to-the-names-inside-a-fact) | what a qualified import does to the names inside a fact | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.11): unchanged, it renames declared names; a library of facts is imported flat**; S1f.6.11 |
| [Q-M1f.137](#q-m1f137--are-an-imported-facts-constants-names-of-the-program) | are an imported fact's constants names of the program? | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.11): yes, as an imported fact is a fact of the program**; S1f.6.11 |
| [Q-M1f.138](#q-m1f138--the-provenance-of-an-imported-fact) | the provenance of an imported fact | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.11): its own `:source`, or background; no module stamp, F24's**; S1f.6.11 |
| [Q-M1f.139](#q-m1f139--the-same-fact-from-two-places) | the same fact from two places | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.11): identical facts are one; two `:source`s keep the first statement, stated on the page and handed to F24**; S1f.6.11 |
| [Q-M1f.140](#q-m1f140--a-rule-of-the-module-the-selection-does-not-name) | a rule of the module the selection does not name | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.11): still dropped: a selection is a selection**; S1f.6.11 |
| [Q-M1f.141](#q-m1f141--does-a-selections-closure-follow-the-names-inside-a-kept-fact) | does a selection's closure follow the names inside a kept fact? | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.11): no: it follows the names a kept declaration writes, as before the stage**; S1f.6.11 |
| [Q-M1f.142](#q-m1f142--a-stored-negation-and-a-fact-of-a-kernel-relation-under-a-selection) | a stored negation, and a fact of a kernel relation, under a selection | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.11): a negation is a fact of the relation it negates; `(__closed__ r)` and a written `(eq a b)` come as any fact does, when the relation's name is in the closure**; S1f.6.11 |
| [Q-M1f.143](#q-m1f143--a-fact-that-arrives-twice-through-flat-imports) | a fact that arrives twice through flat imports | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.11): one statement, kept where it first arrived**; S1f.6.11 |
| [Q-M1f.144](#q-m1f144--what-w88s-rows-record) | what W88's rows record | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.11): the sentences a query's `:goal-text` prints, and a refusal; no schema change**; S1f.6.11 |
| [Q-M1f.145](#q-m1f145--how-the-table-generators-check-is-in-the-gate) | how the table generator's `--check` is in the gate | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.2): a cargo test runs it, as `gen_zebra2_variants.py --check` is run; no new gate step**; S1f.6.2 |
| [Q-M1f.146](#q-m1f146--how-stddigits-closes-its-tables-for-a-flat-importer) | how `std.digits` closes its tables for a flat importer | **Confirmed 2026-11-12** (reply 53 (a)). **defaulted 2026-11-11 (S1f.6.2): written `(__closed__ R)` facts, which a direct importer brings by listing `__closed__`; a theory that reads a table closes it in its own closing rule**; S1f.6.2 |
| [Q-M1f.147](#q-m1f147--the-dot-sweep-and-a-view-no-layout-finishes) | the DOT sweep and a view no layout finishes | **Overruled 2026-11-12** (reply 45 of the calculator's `questions.md`): *"don't test graphviz itself - that is the main point here. just skip big graphs, run tests only on small"*; S1f.6.21, **built 2026-11-12**: a view past 1 000 edges is skipped and counted. **defaulted 2026-11-11 (S1f.6.2): past 1 500 edges `dot_wellformed.rs` asks Graphviz under `fdp` stopped after one iteration; grammar and attributes held, `dot`'s layout of it not. For the user to confirm**; S1f.6.2 |
| [Q-M1f.148](#q-m1f148--one-module-for-the-seven-tables) | one module for the seven tables | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.2): one, as the stage file says; an importer parses all 28.8 KB, about 12 ms, whichever tables it lists**; S1f.6.2 |
| [Q-M1f.149](#q-m1f149--what-let-name-number-does-as-built) | what `(let NAME NUMBER)` does, as built | **Confirmed 2026-11-12** (reply 11 (a)). **defaulted 2026-11-11 (S1f.6.3): the rulings' default reading; every operand of a request is looked up, a number standing for itself and a name for the number its `let` states, four firings a request; a name no `let` states is never answered**; S1f.6.3 |
| [Q-M1f.150](#q-m1f150--the-counter-and-how-the-measured-test-is-in-the-gate) | the counter, and how the measured test is in the gate | **Overruled 2026-11-12** (reply 46 (b)): *"we don't need this proportional gate"*; S1f.6.21, **built 2026-11-12**: the step is out of the gate, the test run by hand. **defaulted 2026-11-11 (S1f.6.3): `candidates`, under `--features counters`, in a gate step of its own**; S1f.6.3 |
| [Q-M1f.151](#q-m1f151--the-names-of-the-circuits-own-relations) | the names of the circuit's own relations | **Reopened 2026-11-12** by reply 8: the clash is a gap of the import mechanism; S1f.6.30. **defaulted 2026-11-11 (S1f.6.3): words, `add-walk`, `strip`, `turn`, `operand`; what a clash does is S1f.6.12's**; S1f.6.3 |
| [Q-M1f.152](#q-m1f152--are-stdariths-rules-written-by-hand-or-generated) | are `std.arith`'s rules written by hand or generated? | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.3): by hand, while a rule is not a family of nine**; S1f.6.3 |
| [Q-M1f.153](#q-m1f153--the-dot-sweeps-bound-under-the-smallest-table) | the DOT sweep's bound, under the smallest table | **Overruled 2026-11-12** (reply 45 of the calculator's `questions.md`): *"don't test graphviz itself - that is the main point here. just skip big graphs, run tests only on small"*; S1f.6.21, **built 2026-11-12**: a view past 1 000 edges is skipped and counted. **defaulted 2026-11-11 (S1f.6.3): 1 000 edges, where S1f.6.2 took 1 500: a view that holds a table is past it. With Q-M1f.147, for the user to confirm**; S1f.6.3 |
| [Q-M1f.154](#q-m1f154--the-bounds-of-the-harness-against-the-integers) | the bounds of the harness against the integers | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.3): every pair to 120, every pair of lengths to 30 digits on a fixed-seed sample, `u128` the oracle**; S1f.6.3 |
| [Q-M1f.155](#q-m1f155--equal-operands-of-a-subtraction-two-walks-one-fact) | equal operands of a subtraction: two walks, one fact | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.4): both walks answer, and both derive the one `(diff-of X Y z z)`**; S1f.6.4 |
| [Q-M1f.156](#q-m1f156--comparison-called-with-a-tag) | comparison called with a tag | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.4): `(cmp-walk TAG X Y ORDER)` hands over `(order-is TAG O)`, a firing more than the prototype's, so a later theory can call it**; S1f.6.4 |
| [Q-M1f.157](#q-m1f157--each-request-looks-its-operands-up-by-rules-of-its-own) | each request looks its operands up by rules of its own | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.4): three rules and two tags a request, as `add` has them; addition's circuit unmoved**; S1f.6.4 |
| [Q-M1f.158](#q-m1f158--cls-column-order-and-the-index) | `cl`'s column order and the index | **Left as it is** in round 2 (`dc88520b`) (item 47, *"(a)"*). **defaulted 2026-11-11 (S1f.6.4): the table stays as S1f.6.2 shipped it; a comparison's column costs 100 candidates, a constant; E-12**; S1f.6.4 |
| [Q-M1f.159](#q-m1f159--multiplication-called-with-a-tag) | multiplication called with a tag | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.5): `(mul-call TAG X Y)` answered by `(done TAG P)`, a firing more than the prototype's, so a later theory can multiply**; S1f.6.5 |
| [Q-M1f.160](#q-m1f160--a-zero-digit-of-the-multiplier) | a zero digit of the multiplier | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.5): a row like any other, no rule tells it apart**; S1f.6.5 |
| [Q-M1f.161](#q-m1f161--the-dot-sweep-several-files-at-a-time) | the DOT sweep, several files at a time | **2026-11-12:** stands; not overruled by reply 45. **defaulted 2026-11-11 (S1f.6.5): the corpus files shared out among the cores, the report in the files' order; what the sweep holds unchanged. With Q-M1f.147 and .153, for the user to confirm**; S1f.6.5 |
| [Q-M1f.162](#q-m1f162--the-longest-sampled-operand-of-a-product) | the longest sampled operand of a product | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.5): 19 digits, so the product stays in `u128`**; S1f.6.5 |
| [Q-M1f.163](#q-m1f163--division-called-with-a-tag-and-whose-division-by-zero-is-flagged) | division called with a tag, and whose division by zero is flagged | **Overruled 2026-11-12** (reply 48: *"(div ?x 0) must emit division-by-zero no matter where it happens"*): a division by zero is flagged wherever it is called; **built 2026-11-13** by S1f.6.24, `div-call-by-zero`. **defaulted 2026-11-11 (S1f.6.6): `(div-call TAG X Y)` answered by `(div-done TAG Q R)`; the flag is derived for a public request, `(quotient X Y)`, and a later theory that calls division says what its own zero divisor is**; S1f.6.6 |
| [Q-M1f.164](#q-m1f164--divisions-families-of-nine-rules-by-hand-or-generated) | division's families of nine rules: by hand or generated? | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.6): by hand, and a test holds that the rules of a family differ by their digit alone**; S1f.6.6 |
| [Q-M1f.165](#q-m1f165--what-search-free-means-of-a-program-that-divides-by-zero) | what search-free means of a program that divides by zero | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.6): no candidate, one saturation, R empty and decided by root, the request in the core; the test holds such a program to that where it holds the others to R = {∅}**; S1f.6.6 |
| [Q-M1f.166](#q-m1f166--the-sizes-of-the-generated-arithmetic-examples) | the sizes of the generated arithmetic examples | **ruled 2026-11-12** (reply 49, *"make a generator in python that takes some size arguments and generate an example"*; round 2's item 49, `dc88520b`, *"pick the one that fits into memory, say ~16GB; do not call it in tests and in CI, just for manual solve call"*): the three shipped files stay at the sizes defaulted, 100-digit operands, 100 requests and 25!; `utils/gen_arith_examples.py --op … -o FILE` writes a program of any size on demand, under `build/`, never checked in and run by no test or tier, and `stdlib/README.md` § The biggest calculation has the sizes measured. Done by S1f.6.29, closed 2026-11-13 on the user's word. Before, **defaulted 2026-11-11 (S1f.6.15): (a), what every per-commit surface affords**; S1f.6.15, S1f.6.29 |
| [Q-M1f.167](#q-m1f167--the-names-of-stdnum9) | the names of `std.num9` | **2026-11-12** (reply 52): the relation names stand; the modules move under `std.math`, S1f.6.31. **defaulted 2026-11-11 (S1f.6.13): `std.arith`'s public names with a 9, `add9` and `sum9-of`, the entry `arith9`, the conversions `to9`/`as9` and `from9`/`of9`, and the flags `overflow` and `big`**; S1f.6.13 |
| [Q-M1f.168](#q-m1f168--overflow-and-division-by-zero-in-stdnum9) | overflow and division by zero in `std.num9` | **Confirmed 2026-11-12** (reply 51 (a)). **defaulted 2026-11-11 (S1f.6.13): overflow a fact naming the request, root alive, never a number; a division by zero `std.arith`'s own flag relation and its rule to `(false)`, imported**; S1f.6.13 |
| [Q-M1f.169](#q-m1f169--where-stdnum9s-multiples-and-rows-live) | where `std.num9`'s multiples and rows live | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.13): in the state and in the step rule, so no rule joins two facts on a number: one rule computes the nine multiples, ten step rules read them, and a product's step computes its own row**; S1f.6.13 |
| [Q-M1f.170](#q-m1f170--what-stdnum9-reads-as-an-operand) | what `std.num9` reads as an operand | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.13): a number written out; no named constants; a conversion reads a number as written, high zeros counted, a name not looked up**; S1f.6.13 |
| [Q-M1f.171](#q-m1f171--stdnum9s-step-counter-and-its-comparison) | `std.num9`'s step counter and its comparison | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.13): `std.digits`' `adj` and `cl`, no table of the module's own**; S1f.6.13 |
| [Q-M1f.172](#q-m1f172--the-harness-of-stdnum9-exhaustive-at-width-2) | the harness of `std.num9`: exhaustive at width 2 | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-11 (S1f.6.13): the generator's `--width 2 --stdout` in front of the test's requests, every pair under five operations; no second width is shipped**; S1f.6.13 |
| [Q-M1f.173](#q-m1f173--max_regs-and-the-wide-rules-of-stdnum9) | `MAX_REGS` and the wide rules of `std.num9` | **Confirmed 2026-11-12** (reply 50 (a)). **defaulted 2026-11-11 (S1f.6.13), the user's to overrule: the bound stays 256; `std.num9`'s generated rules are measured apart, the widest pinned at 162 registers, and every other corpus rule is held to an eighth of the bound as before**; S1f.6.13 |
| [Q-M1f.174](#q-m1f174--the-names-of-stdint) | the names of `std.int` | **2026-11-12** (reply 52): the relation names stand; the modules move under `std.math`, S1f.6.31. **defaulted 2026-11-12 (S1f.6.7), the user's to revise: a prefix, `int-add` answered by `int-sum-of` and so on, the entry `int-arith`**; S1f.6.7 |
| [Q-M1f.175](#q-m1f175--what-stdint-reads-as-an-operand) | what `std.int` reads as an operand | **Kept as built 2026-11-12** (reply 12); promotion is [M1g's P1g.8](../m1g_logic/p1g.8_a_theory_of_types/README.md), with a types theory. **defaulted 2026-11-12 (S1f.6.7), the user's to revise: an integer written out or a name through `std.arith`'s `let`; a natural number is not promoted**; S1f.6.7 |
| [Q-M1f.176](#q-m1f176--a-product-of-integers-without-a-rule-that-looks-at-a-sign) | a product of integers without a rule that looks at a sign | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-12 (S1f.6.7), the user's to revise: the formula on the parts, four products and no case**; S1f.6.7 |
| [Q-M1f.177](#q-m1f177--a-calls-tag-is-its-request-and-what-an-extension-writes-of-stdariths) | a call's tag is its request, and what an extension writes of `std.arith`'s | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-12 (S1f.6.7), the user's to revise: the request is the tag; the module starts `std.arith`'s walks and writes `operand` and `operand-is`**; S1f.6.7 |
| [Q-M1f.178](#q-m1f178--the-harness-of-stdint-a-sibling-file-exhaustive-on-one-digit-parts) | the harness of `std.int` | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-12 (S1f.6.7): a sibling file, `int_theory.rs`, exhaustive on parts of one digit**; S1f.6.7 |
| [Q-M1f.179](#q-m1f179--the-remainder-of-an-integer-division) | the remainder of an integer division: Euclidean, truncated or floored | **Ruled 2026-11-12** (replies 1 (d), 2 (a)): all three conventions, each its own request; the Euclidean remainder a natural number; **built 2026-11-12** by S1f.6.22. Before: **open, the user's; not defaulted.** Integer division is not built until it is ruled; the stage recommends Euclidean and shows the three as rules and computed examples; S1f.6.7 |
| [Q-M1f.180](#q-m1f180--the-names-of-stdrat) | the names of `std.rat` | **2026-11-12** (reply 52): the relation names stand; the modules move under `std.math`, S1f.6.31. **defaulted 2026-11-12 (S1f.6.8), the user's to revise: a prefix, `rat-add` answered by `rat-sum-of` and so on, `rat-div` by `rat-quot-of`, one rational; the entry `rat-arith`**; S1f.6.8 |
| [Q-M1f.181](#q-m1f181--what-stdrat-reads-as-an-operand) | what `std.rat` reads as an operand | **Kept as built 2026-11-12** (reply 12); promotion is [M1g's P1g.8](../m1g_logic/p1g.8_a_theory_of_types/README.md), with a types theory. **defaulted 2026-11-12 (S1f.6.8), the user's to revise: a rational written out, `(rat (int P N) D)`, or a name through `std.arith`'s `let`; an integer or a natural number is not promoted**; S1f.6.8 |
| [Q-M1f.182](#q-m1f182--a-zero-denominator-is-stdariths-division-by-zero) | a zero denominator is `std.arith`'s division by zero | **Confirmed** in round 2 (`dc88520b`) (item 48, point 2: *"OK, I agree"*); S1f.6.24 left `std.rat`'s four rules as built, none of them replaced by the flag at the call. **defaulted 2026-11-12 (S1f.6.8), the user's to revise: the same flag relation, the request as written, on any operand of any request, and on a `rat-div` by a rational that is zero**; S1f.6.8 |
| [Q-M1f.183](#q-m1f183--lowest-terms-by-stdariths-division-and-what-it-costs) | lowest terms by `std.arith`'s division, and what it costs | **Ruled** in round 2 (`dc88520b`) (item 54, *"(b)"*): reduce only on request; S1f.6.33. **defaulted 2026-11-12 (S1f.6.8), the user's to revise: Euclid's chain over `div-call`, one reduction at the end of each operation, none in a comparison; 492 firings for 1/2 + 1/3**; S1f.6.8 |
| [Q-M1f.184](#q-m1f184--the-harness-of-stdrat-and-the-sizes-memory-allows) | the harness of `std.rat`, and the sizes memory allows | **2026-11-12:** stands; section I of the calculator's `questions.md` was left empty. **defaulted 2026-11-12 (S1f.6.8): 28 operands exhaustively, 250 requests a saturation, batches of 100, 400 and 1 200**; S1f.6.8 |
| [Q-M1f.185](#q-m1f185--a-state-file-nested-past-the-parsers-limit) | a state file nested past the parser's limit (E-4): say it, flag it, or change the engine | **Ruled 2026-11-12** (reply 44 (a)): stated on the page and left. The stage recommended (a), which it had done; (b) and (c) are probed and not built; S1f.6.9 |
| [Q-M1f.186](#q-m1f186--where-the-probes-of-s1f610-are-banked) | where the probes of S1f.6.10 are banked | **Confirmed 2026-11-12** (reply 43 (a)). **defaulted 2026-11-12 (S1f.6.10), the user's to revise: `examples/beyond_arith/`, beside `examples/arith/` and not under it**; S1f.6.10 |
| [Q-M1f.187](#q-m1f187--which-probes-of-s1f610-are-banked-and-at-what-size) | which probes of S1f.6.10 are banked, and at what size | **Confirmed 2026-11-12** (reply 43 (a)). **defaulted 2026-11-12 (S1f.6.10): sixteen, each sized so that a bare `ein solve` exhausts R well inside the budget and its dump is a few megabytes; SEND + MORE = MONEY stays under `build/`**; S1f.6.10 |
| [Q-M1f.188](#q-m1f188--the-banked-probes-are-kept-by-hand) | the banked probes are kept by hand | **Confirmed 2026-11-12** (reply 43 (a)). **defaulted 2026-11-12 (S1f.6.10): written once by a script under `build/`, then hand-kept; no generator in `utils/` and no `--check` in the gate**; S1f.6.10 |
| [Q-M1f.189](#q-m1f189--a-users-program-calling-stdariths-tagged-calls) | a user's program calling `std.arith`'s tagged calls | **Answered** in round 2 (`dc88520b`) (item 41, *"(a)"*): the tagged calls are an interface; **built 2026-11-13** by S1f.6.30, T1, held by `arith_theory.rs`'s `the_tagged_calls_keep_their_shapes`. **defaulted 2026-11-12 (S1f.6.10) for the banked programs, and put to the user: eight of them call `add-walk`, `mul-call`, `div-call`, `cmp-walk` and read `done`, `div-done`, `order-is`**; S1f.6.10 |
| [Q-M1f.190](#q-m1f190--which-variants-of-the-binary-prototypes-are-banked-where-the-page-recommends-none) | which variants of the binary prototypes are banked where the page recommends none | **defaulted 2026-11-12 (the banking of S1f.6.16 and S1f.6.17), the user's to revise: the simplest its results hold under: the truth table's candidates from the obligations rung alone; the conversions a module of their own; Post's criterion on seven written sets over the generated facts; the simplifier reading the generated facts**; S1f.6.16, S1f.6.17 |
| [Q-M1f.191](#q-m1f191--the-generators-of-the-binary-examples-are-run-by-hand) | the generators of the binary examples are run by hand | **defaulted 2026-11-12, the user's to revise: `gen_binary.py` and `gen_gates.py` live in the example directories, each with a `--check`, neither in the gate; the properties' check is a corpus program's claims, which is in the gate**; **superseded 2026-11-15** at P1f.6's close (the user's ruling): `gen_binary.py` retired (`c2902aca`), and `utils/gen_gates.py` is in the gate (`bool_theory.rs`); S1f.6.16, S1f.6.17 |
| [Q-M1f.192](#q-m1f192--one-table-file-in-two-example-directories) | one table file in two example directories | **defaulted 2026-11-12, the user's to revise: `examples/binary_logic/bits.ein` is a copy of `examples/binary_arithmetic/bits.ein`, written by `gen_binary.py`, whose `--check` holds the two equal**; **superseded 2026-11-15** at P1f.6's close (the user's ruling): the copy retired, both directories importing `std.math.bits` (`c2902aca`); S1f.6.17 |
| [Q-M1f.193](#q-m1f193--the-sizes-of-the-banked-binary-programs) | the sizes of the banked binary programs | **defaulted 2026-11-12, the user's to revise: three inputs for every search, operands to seventeen bits, conversions to 255 and from 1000, none `slow`; the batches, the scale runs and the enumeration of all 65 535 sets stay under `build/`**; S1f.6.16, S1f.6.17 |
| [Q-M1f.194](#q-m1f194--which-of-the-word-problem-prototypes-programs-are-banked-and-under-what-names) | which of the word-problem prototype's programs are banked, and under what names | **defaulted 2026-11-12 (S1f.6.20), the user's to revise: the recommended variant of each choice; `readings_plain` banked as `readings.ein`, `gap_b` as `gap.ein`; the other forms, mechanisms and witnesses stay on the page**; S1f.6.20 |
| [Q-M1f.195](#q-m1f195--one-file-for-the-two-step-problem) | one file for the two-step problem | **defaulted 2026-11-12 (S1f.6.20), the user's to revise: `boxes12.ein` is the page's `boxes12` and `boxes12_plain` together, `readings` imported and the model's query kept**; S1f.6.20 |
| [Q-M1f.196](#q-m1f196--the-word-problem-modules-as-corpus-entries) | the word-problem modules as corpus entries | **defaulted 2026-11-12 (S1f.6.20), the user's to revise: each module is a `positive` entry with runs of its own, as the stdlib's modules are; `gaps.ein` declares no `solve`; `readings.ein` imports only what it reads (E-11)**; S1f.6.20 |
| [Q-M1f.197](#q-m1f197--what-holds-the-banked-word-problems) | what holds the banked word problems | **defaulted 2026-11-12 (S1f.6.20), the user's to revise: each file's `:expect` through its `test` cell, and the sweeps every corpus file is under; no test of their own until the rulings**; S1f.6.20 |
| [Q-M1f.198](#q-m1f198--the-remainder-variant-of-the-word-problems-and-the-gate) | the remainder variant of the word problems, and the gate | **answered 2026-11-13 by S1f.6.28** (round 2's item 31, the user's *"yes"*): E-33 resolved by its T1 with the test's meaning unchanged, and the 13-apple problem banked as `examples/word_problems/boxes13.ein`, with the readings always owed. Before: **2026-11-12:** owned by S1f.6.28's first task. **defaulted 2026-11-12 (S1f.6.20), the user's to revise: not banked. `region.rs`'s fact-grain sweep does not end on a program where a supposed reading leads to a `value` (E-33); the test and the engine are unmoved; three other ways are stated and one was run**; S1f.6.20 |
| [Q-M1f.199](#q-m1f199--how-the-skipped-views-of-the-dot-sweep-are-pinned) | how the skipped views of the DOT sweep are pinned | **defaulted 2026-11-12 (S1f.6.21), the user's to revise: the count printed, the directories that hold a skipped view named in the test**; S1f.6.21 |
| [Q-M1f.200](#q-m1f200--the-names-of-an-integer-divisions-results) | the names of an integer division's results | **defaulted 2026-11-12 (S1f.6.22), the user's to revise: `int-quot-euclid-of`, `int-quot-trunc-of` and `int-quot-floor-of`, each `(… X Y Q R)`**; S1f.6.22 |
| [Q-M1f.201](#q-m1f201--what-a-bare-relation-r-declares-of-its-count) | what a bare `(relation R)` declares of its count | **defaulted 2026-11-12 (S1f.6.23), the user's to revise: none, so `(R)` is its one form**; S1f.6.23 |
| [Q-M1f.202](#q-m1f202--a-relation-written-through-a-parameter-and-the-declared-count) | a relation written through a parameter, and the declared count | **defaulted 2026-11-12 (S1f.6.23), the user's to revise: not read**; S1f.6.23 |
| [Q-M1f.203](#q-m1f203--the-kernels-own-relations-and-an-activators-count) | the kernel's own relations, and an activator's count | **defaulted 2026-11-12 (S1f.6.23), the user's to revise: neither is held, each a finding (E-34, E-35)**; S1f.6.23 |
| [Q-M1f.204](#q-m1f204--which-written-places-the-declared-count-is-held-in) | which written places the declared count is held in | **defaulted 2026-11-12 (S1f.6.23), the user's to revise: a fact's `:using` and a claim's facts too; a macro's body where it is called; no `(solve …)` list**; S1f.6.23 |
| [Q-M1f.205](#q-m1f205--where-the-check-of-a-malformed-number-runs) | where the check of a malformed number runs | **defaulted 2026-11-13 (S1f.6.12's build), the user's to revise: a pass of its own at the operand, started by the two rules that find a number; the mixed placement measured and not built**; S1f.6.12 |
| [Q-M1f.206](#q-m1f206--how-a-fault-reaches-its-request) | how a fault reaches its request | **defaulted 2026-11-13 (S1f.6.12's build), the user's to revise: one relation at the operand's tag that carries the fault, `(bad-operand TAG FAULT)`, ten rules from a tag to its request, and one rule a flag**; S1f.6.12 |
| [Q-M1f.207](#q-m1f207--what-a-let-that-cannot-be-read-flags) | what a `let` that cannot be read flags | **defaulted 2026-11-13 (S1f.6.12's build), the user's to revise: the `let`, `(malformed (let NAME NUMBER))`, whether or not a request uses it; each of the two where a name is stated twice; and the digit 0 as a name, which the investigation's set left out**; S1f.6.12 |
| [Q-M1f.208](#q-m1f208--how-the-theory-says-that-an-operand-is-a-name) | how the theory says that an operand is a name | **defaulted 2026-11-13 (S1f.6.12's build), the user's to revise: two guards on a variable nothing else binds, `(neq ?v z) (eq ?v ?x)`, in that order; `z` and the digits left out so that a request has one flag; its cost, a comparison a name of the program, stated and pinned**; S1f.6.12 |
| [Q-M1f.209](#q-m1f209--the-binary-example-theory-and-the-check) | the binary example theory and the check | **defaulted 2026-11-13 (S1f.6.12's build), the user's to revise: `gen_binary.py` leaves the check out, so `bin.ein` is unchanged and answers a malformed request by silence**; **superseded 2026-11-15** at P1f.6's close (the user's ruling): binary's numeral is `std.math.binary`'s, checked since S1f.6.16's build; S1f.6.12 |
| [Q-M1f.210](#q-m1f210--stdint-and-stdrat-and-the-flags) | `std.int` and `std.rat`, and the flags | **ruled 2026-11-14, (c)**, with the principle that a malformed number is its representation's problem, detected as soon as it is written: S1f.6.38 built it 2026-11-14 (Q-M1f.272 to .277), **closed**; S1f.6.12 |
| [Q-M1f.211](#q-m1f211--which-fact-carries-the-flag-of-a-division-by-zero-at-the-call) | which fact carries the flag of a division by zero at the call | **defaulted 2026-11-13 (S1f.6.24), the user's to revise: (ii), the call, `(division-by-zero (div-call TAG X Y))` as it was asserted, and a public request's flag as it was, so a public division has two flags and `ein solve` reads as before; the three designs are shown on nine programs in the record**; S1f.6.24 |
| [Q-M1f.212](#q-m1f212--a-theorys-wrong-answer-and-the-register) | a theory's wrong answer and the register | **defaulted 2026-11-13 (S1f.6.24), the user's to revise: not a register row. The register holds the engine's wrong answers, and a tagged call by zero that derived nothing was a rule of a theory, answered by the engine as written; the witness is a test program**; S1f.6.24 |
| [Q-M1f.213](#q-m1f213--the-binary-example-theory-and-the-flag-at-the-call) | the binary example theory and the flag at the call | **defaulted 2026-11-13 (S1f.6.24), the user's to revise: `gen_binary.py` carries the rule over, so `bin.ein` has 68 rules and flags `(div-call2 TAG X Y)` by zero as decimal does**; **superseded 2026-11-15** at P1f.6's close (the user's ruling): the rule is `std.math.positional`'s, shared by both bases; S1f.6.24 |
| [Q-M1f.214](#q-m1f214--the-derives-line-of-a-firing-with-several-conclusions) | the *Derives* line of a firing with several conclusions | **defaulted 2026-11-13 (S1f.6.25), the user's to revise: one line, every conclusion in the rule's order, `; ` between two, each with its own statement**; S1f.6.25 |
| [Q-M1f.215](#q-m1f215--a-qualified-import-and-what-a-module-states-as-a-program) | a qualified import and what a module states as a program | **defaulted 2026-11-13 (S1f.6.25), the user's to revise: a wrong answer, W90; no import brings a module's `(query …)`, `(solve …)` or `(config …)`, nor its `(trace …)`**; S1f.6.25 |
| [Q-M1f.216](#q-m1f216--a-state-file-that-re-writes-less-than-the-program-as-loaded-and-the-register) | a state file that re-writes less than the program as loaded, and the register | **defaulted 2026-11-13 (S1f.6.25), the user's to revise: a register row, W91, under schema `/22`; the fix makes every name a top-level selection lists live, and no more**; S1f.6.25 |
| [Q-M1f.217](#q-m1f217--when-the-loader-says-that-a-selection-left-a-relations-rules-behind) | when the loader says that a selection left a relation's rules behind | **defaulted 2026-11-13 (S1f.6.25), the user's to revise: condition C of the census, a relation not listed, with a deriving rule in the module, none kept and none in the resolved program; one line a form, `note:` on stderr**; **superseded 2026-11-15** at P1f.6's close (the user's ruling): the line retired with E-20 by S1f.6.35; S1f.6.25 |
| [Q-M1f.218](#q-m1f218--what-a-count-past-the-runs-reach-says) | what a count past the run's reach says | **defaulted 2026-11-13 (S1f.6.25), the user's to revise: counted exactly and reported up to 2^63 − 1, which reads *this many or more* on every surface; no wider integer is printed**; S1f.6.25 |
| [Q-M1f.219](#q-m1f219--a-querys-false-on-a-program-that-only-owes) | a query's `(false)` on a program that only owes | **defaulted 2026-11-13 (S1f.6.26), the user's to revise: no change, the claim holds as it does; the row under the query now says which empty R is; the meaning is [M1g's P1g.7](../m1g_logic/p1g.7_unsatisfiable_against_owing/README.md)'s**; S1f.6.26 |
| [Q-M1f.220](#q-m1f220--the-form-of-the-rows-for-what-root-owes) | the form of the rows for what root owes | **defaulted 2026-11-13 (S1f.6.26), the user's to revise: an `owed` row an instance in the block under `∅`, at most four on stdout and one line for the rest, outside the listings' 100 lines, every one in `answer.txt`; `--count` prints none**; S1f.6.26 |
| [Q-M1f.221](#q-m1f221--one-question-a-file-and-the-fall-back) | one question a file, and the fall-back | **defaulted 2026-11-13 (S1f.6.27), the user's to revise: a file is asked the least question that can decide every claim it states, `--single` or `-n k+1` over the one part its queries' claims are over, and what that leaves undecided is asked the describing question; a solve claim, and claims over more than one part, ask describe at once; no mode a claim**; S1f.6.27 |
| [Q-M1f.222](#q-m1f222--a-claim-refuted-by-a-state-a-run-found) | a claim refuted by a state a run found | **defaulted 2026-11-13 (S1f.6.27), the user's to revise: under `ein test` a run that did not exhaust what a claim is about refutes the claim where a state it found gives a class the claim does not list, and `(solve :expect (false))` where root is alive; `ein solve` under a mode keeps its note, *not checked*, and its exit 0**; S1f.6.27 |
| [Q-M1f.223](#q-m1f223--what-the-surfaces-of-ein-test-say-of-the-question) | what the surfaces of `ein test` say of the question | **defaulted 2026-11-13 (S1f.6.27), the user's to revise: `-v` names the question before what the run found, *(--single: all of R, 1 class over 1 states)*, and *describe, after -n 3* on a fall-back; `--json-report` is `ein-test-report/5` with `route` and `fell_back_from` on every row; `--events` and `--json-summary` hold the last question asked**; S1f.6.27 |
| [Q-M1f.224](#q-m1f224--what-a-bare-description-pays-for-n) | what a bare description pays for N | **defaulted 2026-11-13 (S1f.6.27), the user's to revise: no change; a bare `ein solve` prints the `N` row with its count and takes the minimal transversals that count is, where a surface that prints neither `N` nor its count, `--count` and `ein test` without an artefact, takes none**; S1f.6.27 |
| [Q-M1f.225](#q-m1f225--what-the-fact-grain-test-does-where-the-closure-gives-up) | what the fact-grain test does where the closure gives up | **ruled 2026-11-13, (a)** (*"Ok"*): the closure runs only where a relation-grain site reads it; on a flagged program it gives up after 10 000 shapes on the safe side, every site one; and `region.rs`'s `fact_grain_never_calls_a_violated_cube_monotone` fails on a program where it gives up, naming it, so the bound never decides a verdict the test reads**; S1f.6.28 |
| [Q-M1f.226](#q-m1f226--an-obligations-name-in-a-closing-rule-and-alive_invariants-r0) | an obligation's name in a closing rule, and `alive_invariant`'s R0 | **ruled 2026-11-13, (b)** (*"I suppose yes, what it cost?"*, then *"continue next stages till the end of the phase"*): the names index holds the obligations' names; before, defaulted (a): `every_name_of_the_program_is_a_candidate_object` lists a seeded name the program categorises as an obligation's and does not hold it to the blind enumerator's objects; no engine source moves (E-48)**; S1f.6.28 |
| [Q-M1f.227](#q-m1f227--how-a-run-of-the-biggest-calculation-is-held-to-about-16-gb-and-how-long-it-may-take) | how a run of the biggest calculation is held to about 16 GB, and how long it may take | **defaulted 2026-11-13 (S1f.6.29), the user's to revise: a cgroup a run, `MemoryMax=16000000000` (16.0 GB, 14.9 GiB) and no swap, `oom_score_adj` 1000; a run past 600 s killed; the search narrowed to 3 %; since the resume, no run started with under 20 GB available. The first sweep was reaped by Claude Code at the cap with 1.4 GB left; resumed on the user's *"run again, I freed memory"*; stopped and the stage closed on *"runs way too long than I expected - just write down generated examples and close"***; S1f.6.29 |
| [Q-M1f.228](#q-m1f228--what-the-interface-of-the-tagged-calls-holds-beyond-the-seven-names-round-2-showed) | what the interface of the tagged calls holds beyond the seven names round 2 showed | **defaulted 2026-11-13 (S1f.6.30), the user's to revise: the interface is what the callers write: `sub-walk` and `short` beside the seven, since `std.int` calls the subtraction walk; the operand look-up, `operand` and `operand-is`, which `std.int` and `std.rat` call and extend; and an addition's carry in as the caller's value, since `examples/binary_arithmetic/conv.ein` writes a bit there**; S1f.6.30 |
| [Q-M1f.229](#q-m1f229--what-the-census-of-imported-names-reads-and-how) | what the census of imported names reads, and how | **defaulted 2026-11-13 (S1f.6.30), the user's to revise: the twelve stdlib modules and the fourteen example theories another file imports, the import mechanism's own fixtures left out; each imported by every name it declares itself; static, and held to the loader on every pair**; S1f.6.30 |
| [Q-M1f.230](#q-m1f230--how-the-proposals-are-prototyped-and-priced) | how the proposals are prototyped and priced | **defaulted 2026-11-13 (S1f.6.30), the user's to revise: `std.arith` alone, as module text under `$EIN_STDLIB`; exported, the requests, `let`, results, flags, `n`, the interface, the entry and the one rule another module selects; a hidden name spelt `std.arith.X`; the convention's prefix `arith-`; the price by two sweeps of the 133 programs that bring the calculator and a bound of `corpus_shapes.md5`, not by blessing**; S1f.6.30 |
| [Q-M1f.231](#q-m1f231--which-way-the-import-mechanism-closes-the-gap) | which way the import mechanism closes the gap | **ruled 2026-11-13: (h)**, the user's own model, full names and short names; `import_clashes.md` § 7; the fix is S1f.6.35 |
| [Q-M1f.232](#q-m1f232--how-a-module-says-what-it-exports-and-what-stdarith-exports) | how a module says what it exports, and what `std.arith` exports | **ruled 2026-11-13 by (h)**: no export list; a module's every name has a full name and the importer lists the short names it writes; `import_clashes.md` § 7 |
| [Q-M1f.233](#q-m1f233--how-a-name-a-module-does-not-export-is-printed) | how a name a module does not export is printed | **answered 2026-11-13 by (h)** (*"Ok, is that an issue?"*, answered: a one-time move of outputs, R unmoved): the rule is [Q-M1f.237](#q-m1f237--how-a-name-prints-under-h)'s default; `import_clashes.md` § 7 |
| [Q-M1f.234](#q-m1f234--two-flat-imports-that-export-one-name) | two flat imports that export one name | **ruled 2026-11-13**: one short name bound to two full names in one file is refused at load (*"it's an error, not correct ein code, just like ordinary namespaces"*); `import_clashes.md` § 7 |
| [Q-M1f.235](#q-m1f235--a-module-in-a-sibling-directory-e-32) | a module in a sibling directory (E-32) | **ruled 2026-11-13**: `..` stays refused, a module has one path; E-32 stays as stated, and the copy of `bits.ein` goes with S1f.6.16's build, which moves the binary tables into the stdlib; `import_clashes.md` § 7 |
| [Q-M1f.236](#q-m1f236--e-49-and-e-50) | E-49 and E-50 | **ruled 2026-11-13**: E-49 accepted as shadowing (*"silent re-definition, like variables shadowing, it's a mistake"*); E-50 dissolved by full names; `import_clashes.md` § 7 |
| [Q-M1f.237](#q-m1f237--how-a-name-prints-under-h) | how a name prints under (h) | **defaulted 2026-11-13, the user's to revise**: the main file's short name where it has one, the full name otherwise; S1f.6.35 |
| [Q-M1f.238](#q-m1f238--a-full-name-of-a-module-no-import-brings) | a full name of a module no import brings | **defaulted 2026-11-13, the user's to revise**: refused at load, naming the module; not an implicit import; S1f.6.35 |
| [Q-M1f.239](#q-m1f239--a-declaration-under-an-imported-modules-full-name) | a declaration under an imported module's full name | **defaulted 2026-11-13, the user's to revise**: refused at load; a `--dump-answer` state file, which imports nothing, loads as today; S1f.6.35 |
| [Q-M1f.240](#q-m1f240--entry-rules-and-__closed__-lists-under-h) | entry rules and `__closed__` lists under (h) | **defaulted 2026-11-13, the user's to revise**: they stay in the shipped modules; S1f.6.35 |
| [Q-M1f.241](#q-m1f241--a-declared-short-name-with-a-dot) | a declared short name with a dot | **defaulted 2026-11-13, the user's to revise**: refused at load, so a full name such as `word-theory.gaps.gaps` splits one way; S1f.6.35 |
| [Q-M1f.242](#q-m1f242--a-dead-or-redundant-hypothesis-declaration) | a dead or redundant hypothesis declaration | **defaulted 2026-11-14 (S1f.6.34), the user's to revise**: one `note:` line on stderr, on every command, no exit code moved; S1f.6.34 |
| [Q-M1f.243](#q-m1f243--an-obligation-read-as-declared-or-as-owed) | an obligation, read as declared or as owed | **defaulted 2026-11-14 (S1f.6.34), the user's to revise**: as declared, a plan root's activators compile, whether or not root owes it; S1f.6.34 |
| [Q-M1f.244](#q-m1f244--o--c-beside-an-hrule) | O + C beside an hrule | **defaulted 2026-11-14 (S1f.6.34), the user's to revise**: a conflict whatever generates; O + HR and O + NH only with no hrule; S1f.6.34 |
| [Q-M1f.245](#q-m1f245--where-the-conflict-check-runs) | where the conflict check runs | **defaulted 2026-11-14 (S1f.6.34), the user's to revise**: at load on every command, on root of a second load; `ein_infer::mechanisms::check` for a library caller, not in `region::prepare`; S1f.6.34 |
| [Q-M1f.246](#q-m1f246--the-corpuss-twenty-four-programs-in-conflict) | the corpus's twenty-four programs in conflict | **defaulted 2026-11-14 (S1f.6.34), the user's to revise**: each corrected to say what it meant, one kept as the refusal's fixture; S1f.6.34 |
| [Q-M1f.247](#q-m1f247--what-the-closure-ii-follows-a-relation-of-any-kind-or-a-declared-one) | what the closure (ii) follows: a relation of any kind, or a declared one | **ruled 2026-11-14: any relation, (a)**; **superseded the same day** by the user's *"Ok, just forget my intial ruling - in all cases we import everything, the difference only in names"*: every import brings its module whole and no closure exists; S1f.6.35 |
| [Q-M1f.248](#q-m1f248--a-closing-rule-under-the-closure-ii) | a closing rule under the closure (ii) | **ruled 2026-11-14: (a)**; **superseded the same day** by the user's *"Ok, just forget my intial ruling - in all cases we import everything, the difference only in names"*; S1f.6.35 |
| [Q-M1f.249](#q-m1f249--the-answers-h-moves-under-the-recommended-reading) | the answers (h) moves under the recommended reading | **answered 2026-11-14: (a), accepted**, because today's whole import gives each of the five (the record's *Import invariance*), and the stage's invariance test, selective against whole against `:as`, holds them; S1f.6.35 |
| [Q-M1f.250](#q-m1f250--a-listed-name-a-module-states-facts-of-and-does-not-declare) | a listed name a module states facts of and does not declare | **defaulted 2026-11-14 (S1f.6.35), the user's to revise**: a name only, bound to the relation the module's facts are of, as the module reads it, the facts coming with the whole module; S1f.6.35 |
| [Q-M1f.251](#q-m1f251--a-prefixed-name-that-names-nothing-the-module-declares) | a prefixed name that names nothing the module declares | **defaulted 2026-11-14 (S1f.6.35), the user's to revise**: refused at load; S1f.6.35 |
| [Q-M1f.252](#q-m1f252--an-empty-segment-or-a--in-a-modules-name) | an empty segment or a `/` in a module's name | **defaulted 2026-11-14 (S1f.6.35), the user's to revise**: refused at load; S1f.6.35 |
| [Q-M1f.253](#q-m1f253--how-a-name-prints-under-as) | how a name prints under `:as` | **defaulted 2026-11-14 (S1f.6.35), the user's to revise**: the main file's alias spelling, `a.sum-of`, as today; S1f.6.35 |
| [Q-M1f.254](#q-m1f254--s1f634s-conflicts-beside-solve-no-search) | S1f.6.34's conflicts beside `(solve :no-search)` | **defaulted 2026-11-14 (S1f.6.36), the user's to revise**: (a), every conflict stands, read as declared; a `note:` beside an hrule or a list; S1f.6.36 |
| [Q-M1f.255](#q-m1f255--what-ein-solve---no-search-prints) | what `ein solve --no-search` prints | **defaulted 2026-11-14 (S1f.6.36), the user's to revise**: (a), root, and an `R` row that says it was not searched; S1f.6.36 |
| [Q-M1f.256](#q-m1f256--which-claims-root-decides-under---no-search) | which claims root decides under `--no-search` | **defaulted 2026-11-14 (S1f.6.36), the user's to revise**: (a), what a description stopped after root decides, under `ein test`'s refute rule (E-57); S1f.6.36 |
| [Q-M1f.257](#q-m1f257--ein-test---no-searchs-exit-code) | `ein test --no-search`'s exit code | **defaulted 2026-11-14 (S1f.6.36), the user's to revise**: (a), a claim not checked moves no exit code; S1f.6.36 |
| [Q-M1f.258](#q-m1f258--which-commands-and-options-take---no-search) | which commands and options take `--no-search` | **defaulted 2026-11-14 (S1f.6.36), the user's to revise**: (a), `ein solve` and `ein test`; `ein oracle` refuses it; S1f.6.36 |
| [Q-M1f.259](#q-m1f259----json-summary-under---no-search) | `--json-summary` under `--no-search` | **defaulted 2026-11-14 (S1f.6.36), the user's to revise**: (a), `asked.ask = "no-search"`, `region` and `verdict` null, root's standing in the `root` block, the schema id unchanged; S1f.6.36 |
| [Q-M1f.260](#q-m1f260--which-of-s1f634s-twenty-four-say-no-search) | which of S1f.6.34's twenty-four say `:no-search` | **defaulted 2026-11-14 (S1f.6.36), the user's to revise**: (a), the 21 whose question was root alone; S1f.6.36 |
| [Q-M1f.261](#q-m1f261--the-decimal-theorys-module-name-after-the-user-wrote-stdmatharith-in-passing) | the decimal theory's module name, after the user wrote `std.math.arith` in passing | **defaulted 2026-11-14 (S1f.6.31), the user's to revise**: `std.math.decimal-arith`, entry `decimal-arith`, as replies 16 and 52 and round 2's confirmed layout name it; S1f.6.31 |
| [Q-M1f.262](#q-m1f262--a-records-link-to-a-file-the-layout-moved) | a record's link to a file the layout moved | **defaulted 2026-11-14 (S1f.6.31), the user's to revise**: the link's target re-pointed, its text and the record's prose as they were; S1f.6.31 |
| [Q-M1f.263](#q-m1f263--the-sixteen-layout-rules-of-stdboolgates-now-that-an-import-brings-every-rule) | the sixteen layout rules of `std.bool.gates`, now that an import brings every rule | **accepted 2026-11-14** (*"I accept all listed questions defaults"*; defaulted by S1f.6.17's build): (a), the sixteen rules in `std.bool.gates`, derived in every program that imports it; S1f.6.17 |
| [Q-M1f.264](#q-m1f264--a-functions-name-a-program-writes-and-does-not-list-e-62) | a function's name a program writes and does not list (E-62) | **accepted 2026-11-14** (*"I accept all listed questions defaults"*; defaulted by S1f.6.17's build): (a), the resolver as S1f.6.35 ruled, the rule said on every page a program reads first; S1f.6.17 |
| [Q-M1f.265](#q-m1f265--the-names-and-file-shapes-stdbools-build-chose) | the names and file shapes `std.bool`'s build chose | **accepted 2026-11-14** (*"I accept all listed questions defaults"*; defaulted by S1f.6.17's build): (a), as built; S1f.6.17 |
| [Q-M1f.266](#q-m1f266--how-a-base-is-connected-to-the-shared-rules) | how a base is connected to the shared rules of `std.math.positional` | **ruled 2026-11-14, (iii)**: one call of a macro, in the base's own module, which is the file that combines; S1f.6.16 |
| [Q-M1f.267](#q-m1f267--the-binary-theorys-names) | the binary theory's names | **defaulted 2026-11-14 (S1f.6.16's design), the user's to revise**: `b2` kept; every other name `bin-`, the tables `bin-fa`, `bin-fs`, `bin-cl`, `bin-nz` among them; the conversions `to-bin`, `bin-of`, `to-dec`, `dec-of`; S1f.6.16 |
| [Q-M1f.268](#q-m1f268--how-far-binary-flags-a-malformed-request) | how far binary flags a malformed request | **defaulted 2026-11-14 (S1f.6.16's design), the user's to revise**: as far as decimal: the shared check and two rules of binary's own over `0` and `1`; S1f.6.16 |
| [Q-M1f.269](#q-m1f269--division-by-zero-is-false-stays-decimals) | `division-by-zero-is-false` stays decimal's | **ruled 2026-11-14, neither (a) nor (b)**: the relation and the rule are arithmetic's own, in `std.math.arith` (with `z`), which both bases import; S1f.6.38 creates it; S1f.6.16 |
| [Q-M1f.270](#q-m1f270--the-connecting-macros-body-written-or-generated) | the connecting macro's body, written or generated | **defaulted 2026-11-14 (S1f.6.16's design), the user's to revise**: (a), written by hand in `std.math.positional` and held by a test; S1f.6.16 |
| [Q-M1f.271](#q-m1f271--what-an-hrules-and--conclusion-proposes) | what an hrule's `(and …)` conclusion proposes | **accepted 2026-11-14** (defaulted by S1f.6.37): (a), refused at load: an hrule's conclusion, read flat, is one conjunct; S1f.6.37, E-65, W95 |
| [Q-M1f.272](#q-m1f272--how-a-representation-module-sees-a-numeral-right-away) | how a representation module sees a numeral, right away | **ruled 2026-11-14, (a)**: E1, the loader states every nested fact a file writes as `(written T)`; S1f.6.38 |
| [Q-M1f.273](#q-m1f273--where-decimals-numeral-check-lives) | where decimal's numeral check lives | **ruled 2026-11-14, (b)**, against the proposed default: a separate `std.math.decimal` declares `n` and holds the check; binary's numeral `std.math.binary`'s by the same ruling, a default; S1f.6.38 |
| [Q-M1f.274](#q-m1f274--the-numerals-flag) | the numeral's flag | **accepted 2026-11-14**: (a), `(not-decimal NUMERAL THING)` on the smallest term at fault, binary's `(not-binary …)`; S1f.6.38 |
| [Q-M1f.275](#q-m1f275--what-the-loader-states-as-written) | what the loader states as written | **accepted 2026-11-14**: (a), the nested facts of every written fact and the ground ones of every rule's and hrule's conclusion; S1f.6.38 |
| [Q-M1f.276](#q-m1f276--an-integers-and-a-rationals-own-shape) | an integer's and a rational's own shape | **accepted 2026-11-14**: (a), flagged, `(not-integer …)` and `(not-rational …)`; S1f.6.38 |
| [Q-M1f.277](#q-m1f277--an-operand-of-another-type-in-int-and-rat) | an operand of another type in int and rat | **accepted 2026-11-14**: (b), silent and named on the pages; S1f.6.38 |
| [Q-M1f.278](#q-m1f278--how-a-module-below-let-flags-a-named-constant-inside-a-numeral) | how a module below `let` flags a named constant inside a numeral | **defaulted 2026-11-14 (S1f.6.38's build), the user's to revise**: (a), `named` is `std.math.decimal`'s, written by `let-named`, probed by `numeral-prefix-named`; S1f.6.38 |
| [Q-M1f.279](#q-m1f279--what-a-written-fact-is-derived-by-and-where-it-shows) | what a `written` fact is derived by, and where it shows | **defaulted 2026-11-14 (S1f.6.38's build), the user's to revise**: (a), the loader's step `#written` from the fact it was read in, shown where the mirrors show; S1f.6.38 |
| [Q-M1f.280](#q-m1f280--a-program-that-writes-written) | a program that writes `written` | **defaulted 2026-11-14 (S1f.6.38's build), the user's to revise**: (a), refused at load; S1f.6.38 |
| [Q-M1f.281](#q-m1f281--a-let-that-names-a-digit) | a `let` that names a digit | **defaulted 2026-11-14 (S1f.6.38's build), the user's to revise**: (a), the `let`'s fault, `let-to-digit`, `let-to-digit-zero`; S1f.6.38 |
| [Q-M1f.282](#q-m1f282--the-flag-of-an-integer-division-whose-operand-is-at-fault) | the flag of an integer division whose operand is at fault | **defaulted 2026-11-14 (S1f.6.38's build), the user's to revise**: (a), two flags, the tag's and the request's (`int-bad-div`); S1f.6.38 |
| [Q-M1f.283](#q-m1f283--the-two-new-modules-names-and-their-place) | the two new modules' names and their place | **defaulted 2026-11-14 (S1f.6.38's build), the user's to revise**: entries `arith`, `decimal`; `z` a relation of no argument; entries named in closing rules; num9's bare import of `std.math.decimal-arith`; S1f.6.38 |
| [Q-M1f.284](#q-m1f284--the-worker-pools-count-its-heavy-lane-and-its-memory-bound) | the worker pool's count, its heavy lane and its memory bound | **defaulted 2026-11-15 (S1f.6.39's T4), the user's to revise**: (a), 16 workers, a quarter of them the heavy lane, 24 GB; S1f.6.39 |
| [Q-M1f.285](#q-m1f285--the-nightlys-and-the-dev-hosts-test-steps-on-the-gate-profile) | the nightly's and the dev host's test steps on the gate profile | **defaulted 2026-11-15 (S1f.6.39's T5), the user's to revise**: (a), every test step of every tier on the gate profile; S1f.6.39 |
| [Q-M1f.286](#q-m1f286--id_order_invariance-keeps-its-plain-run) | `id_order_invariance` keeps its plain run | **defaulted 2026-11-15 (S1f.6.39's T3), the user's to revise**: (a), as built (E-72); S1f.6.39 |
| [Q-M1f.295](#q-m1f295--binarys-numeral-is-stdmathbinarys) | binary's numeral is `std.math.binary`'s | **defaulted 2026-11-15 (S1f.6.16's build), the user's to revise**: (a), a module `std.math.binary` on `std.math.decimal`'s pattern; S1f.6.16 |
| [Q-M1f.296](#q-m1f296--the-bit-tables-by-the-digit-tables-generator) | the bit tables, by the digit tables' generator | **defaulted 2026-11-15 (S1f.6.16's build), the user's to revise**: (a), `utils/gen_arith_tables.py` writes both, one `--check`, held by `digit_tables.rs`; S1f.6.16 |
| [Q-M1f.297](#q-m1f297--binarys-tagged-calls-an-interface-what-stdmathconvert-reads) | binary's tagged calls an interface; what `std.math.convert` reads | **defaulted 2026-11-15 (S1f.6.16's build), the user's to revise**: (a), (a), (a): an interface; binary's circuit as is; no `(bin-arith-rule …)` fact; S1f.6.16 |
| [Q-M1f.298](#q-m1f298--the-two-shared-sentences-that-wrote-decimals-numeral) | the two shared sentences that wrote decimal's numeral | **defaulted 2026-11-15 (S1f.6.16's build), the user's to revise**: (a), the base's constructor in the sentence; S1f.6.16 |
| [Q-M1f.299](#q-m1f299--gcd0-0) | gcd(0, 0) | **defaulted 2026-11-15 (S1f.6.32), the user's to revise**: (a), zero, and lcm(0, 0) zero; S1f.6.32 |
| [Q-M1f.300](#q-m1f300--stdmathgcds-tagged-call-an-interface) | `std.math.gcd`'s tagged call, an interface | **defaulted 2026-11-15 (S1f.6.32), the user's to revise**: (a), `(gcd-call TAG X Y)` answered by `(gcd-done TAG G)`, an interface; S1f.6.32 |
| [Q-M1f.301](#q-m1f301--what-gcd-call-does-before-euclids-chain) | what `gcd-call` does before Euclid's chain | **defaulted 2026-11-15 (S1f.6.32), the user's to revise**: (a), each number put into normal form first; S1f.6.32 |
| [Q-M1f.302](#q-m1f302--what-stdmathrats-unreduced-answer-fixes-and-rat-norm-under-b) | what `std.math.rat`'s unreduced answer fixes, and `rat-norm` under (b) | **defaulted 2026-11-15 (S1f.6.33), the user's to revise**: (a), the numerator in normal form, D with no high zero, nothing reduced; S1f.6.33 |
| [Q-M1f.303](#q-m1f303--rat-simplify-asks-stdmathgcd-the-numerator-first) | `rat-simplify` asks `std.math.gcd` the numerator first | **defaulted 2026-11-15 (S1f.6.33), the user's to revise**: (a), the numerator first, as before; S1f.6.33 |
| [Q-M1f.304](#q-m1f304--route_invariance-past-the-default-budget-past-any-shards-five-minutes) | `route_invariance` past the default budget, past any shard's five minutes | **ruled 2026-11-15: (d)**, no test runs past the default budget (the user: *"(d)"*); defaulted (a) the same day by T7; built at P1f.6's close; S1f.6.39 |
| [Q-M1f.306](#q-m1f306--what-the-blind-enumerator-takes-for-an-object) | what the blind enumerator takes for an object: a kernel dunder (E-1), a relation's name (E-7), what an import brings (E-54) | **ruled 2026-11-16**: the user, 2026-11-16, shown the four questions with their defaults: *"all defaults ok"*. **(b) for E-1**, a kernel dunder is never an object; **(a) for E-7**, left, and the read-out names such candidates; **(d) for E-54**, left. Whether (b) covers a dunder a program writes as a value was asked back and **ruled the same day**: *"dunder never an object, wide reading"*. **Built 2026-11-16** by S1f.10.9's task 4: the register's W96, four rows, filed and fixed; a `relation names` row for E-7 |
| [Q-M1f.307](#q-m1f307--a-debt-root-was-not-told-of-what-h-holds) | a debt root was not told of: what H holds where a node derives an obligation's activator (W26, Q-M1e.11) | **ruled 2026-11-16**: the user, 2026-11-16, shown the four questions with their defaults: *"all defaults ok"*. **(A)**, H holds what a node's debt would propose, by the static reading, which the census selected. **Built 2026-11-16** by S1f.10.6's task 4: W26 fixed, the witness alone moved; Q-M1e.11 is answered by it |
| [Q-M1f.308](#q-m1f308--the-rung-a-loaded-obligation-rule-chooses) | the rung a loaded obligation rule chooses: the second rung by a rule loaded, by an obligation activated at root, or by a debt owed at root (E-51) | **ruled 2026-11-16**: the user, asked back since the question had a recommendation and no default: *"308 (b) ok"*. **(b)**, the second rung where an obligation is activated at root. **Built 2026-11-16** by S1f.10.15's task 4: the register's W97, three rows, filed and fixed, the re-plan's probe refused at load; 41 corpus programs change rung, `features/09` changes H, no R moves |
| [Q-M1f.309](#q-m1f309--how-ein-shows-a-programs-structure) | how `ein` shows a program's structure: a view of `ein render`, rows of the read-out, both, or no surface | **ruled 2026-11-16**: the user, 2026-11-16, shown the four questions with their defaults: *"all defaults ok"*. **(a)**, a view of `ein render`, `ein render structure FILE`; `ein solve` is untouched. Built by S1f.10.12's third task, and the page that defines the structure by its fourth. **Renamed 2026-11-16** on the user's instruction: `ein render h-space FILE`, and `ein show hyps FILE` lists H as Ein facts |
| [Q-M1f.310](#q-m1f310--p1f10-part-1-the-defaults-taken) | P1f.10, part 1: what the stages of tracks S and R decided without a ruling, each where its record is | **accepted 2026-11-16**: the user, 2026-11-16, shown the four questions with their defaults: *"all defaults ok"*. Each stands until revised |
| [Q-M1f.311](#q-m1f311--a-describe-the-budget-would-stop-answer-what-the-tree-can-reach-instead) | a describe the budget would stop: answer what the tree can reach instead? | **ruled 2026-11-16: (e)**, then, on S1f.10.17's measured *no* for the whole border route, *"2 modes: state route (existing) and tree plus classes, second by default, cmdline option to force one or another, verbose warning about budget"*. **Built** by S1f.10.17's T3: `--describe-route tree|states`, the tree and the classes by default |
| [Q-M1f.312](#q-m1f312--what-the-shape-goldens-hyp-and-hypclosed-ops-digest) | what the shape golden's `hyp` and `hyp+closed` ops digest: the walk's generator, which no command runs, or H's own pass | **ruled 2026-11-16: (e)**, both ops removed and what only they read deleted; the user's choice at the phase's close; **built 2026-11-16** by S1f.10.16's record as ruled (`26d0bc56`, the pages `ff63da53`): the golden's 1 322 `hyp` and `hyp+closed` lines gone, nothing else moved, and the walk's generator pass deleted with K-29 and K-30 |
| [Q-M1f.313](#q-m1f313--may-the-agenda-inside-a-routes-node-fire-first-what-leads-to-false) | may the agenda inside a route's node fire first what leads to `(false)`? Reopens Q-M1f.6's FIFO | **ruled 2026-11-16: (c)**, fail-first everywhere, the `pairs` half; the user: *"… non-monotone programs still out of scope … don't claim about solution soundness, correctness or completeness or anything"*. Supersedes Q-M1f.6's plain FIFO. **Built** by S1f.10.19: `pairs` everywhere, the engine's order |
| [Q-M1f.314](#q-m1f314--what-the-release-voice-check-reads-and-what-fails-a-line) | what the release-voice check reads, and what fails a line: the readings S1f.20.1 took where the stage file left them open | **ruled 2026-11-17 as defaulted** (the user, at S1f.20.2's review, item 13, verbatim [below](#q-m1f314--what-the-release-voice-check-reads-and-what-fails-a-line)): every doc comment of a shipped crate, tests included; every `.ein` comment; every ISO date; the folder `plans/` named alone passes; a count that falls is reported, not failed; F-, E-, W-, K- and review ids and `:why` strings not read yet; **readings revised by S1f.20.9 part C2** (the check now reads every crate comment, every project `.md`, `corpus/regressions.toml`, plan finding, followup and review ids, register cases and process paths, has no baseline and does not read itself; the section says each) |
| [Q-M1f.390](#q-m1f390--what-an-obligation-emits-the-verdict-with-its-justification-or-as-a-fact) | what an obligation emits: the verdict, with its justification or as a fact. The user's *"maybe better would be to emit (open) facts as (open ?R <justification>)"* | **put 2026-11-17, not ruled**; [S1f.21.1](p1f.21_verdicts_as_rules/s1f.21.1_the_survey_and_the_design.md)'s [`design.md` § 5](p1f.21_verdicts_as_rules/design.md#5-the-question-and-a-recommendation). Recommended default: **(b)**, the verdict carries its justification, the instance, and stays a verdict; nothing is built on a default, and [S1f.21.2](p1f.21_verdicts_as_rules/README.md#stages) writes the pages for today's kernel either way |
| [Q-M1f.391](#q-m1f391--the-negation-of-a-negation-that-a-rule-builds) | the negation of a negation that a rule builds: a conclusion `(not ?x)` whose variable holds a stored negation stores `(not (not g))`, which the pages say is no fact and no file can write | **ruled 2026-11-17: (c)** with the elimination a rule (*"Q-M1f.391 OK"*, to the reading put to the user): `(not (not g))` is a fact a file may write and a rule may match, the kernel is not changed, and the rule that concludes `g` from it is a theory's, its home P1f.21's to say (its V-5); P1f.22's `contexts.md` § 6, copied to [P1f.21's `open_facts.md` § 10.5](p1f.21_verdicts_as_rules/open_facts.md#105-the-negation-of-a-negation-and-its-elimination-as-a-rule). **Built** by S1f.22.3 |
| [Q-M1f.392](#q-m1f392--a-state-file-that-reads-back-dead-through-written) | a state file that reads back dead, through `written`: the reload states `written` for a fact a rule assembled under an arrow, and a rule over `written` fires | **ruled 2026-11-17**: a bug; the requirement an invariance in both directions (*"save -> load -> save byte identical, load -> save -> load - have identical dumps; both save and load must not modify the program, must not add or remove any bit of information"*); **a saved solution is the program unchanged with the choices made**, the state derived again on load (*"(a)"*); **and it loads with no other file beside it** (*"yes"*). The format ruled as Q-M1f.400 to .403; **built** by S1f.22.5, on master since `88ba8cde`: a saved state is its program, its modules and `(commit …)`, and loads as the node of its program's lattice |
| [Q-M1f.393](#q-m1f393--a-fact-held-in-a-variable-as-a-conclusion-or-a-premise) | a fact held in a variable, as a conclusion or a premise: a rule denies whatever fact a variable holds, `(not ?f)`, and can neither hold it nor ask whether it is held | **ruled 2026-11-17: (a)** (*"Q-M1f.393 yes (a)"*): a rule unpacks by a fact's count, the relation a parameter where wanted; Q-M1f.351 stands |
| [Q-M1f.394](#q-m1f394--the-words-for-a-fact-under-an-arrow) | the words for a fact under an arrow: the three cases' names, whether a denied fact is in its negation's context, and *value* and *term* beside *fact* | **ruled 2026-11-17**, in the user's words: (1) *"held = in the global context; in a context = in a fact's (local) context"*; (2) a negation is a context, the one the kernel reads; (3) *object* for every node of the graph in the core theory, *value* only for what a variable of a rule is bound to, *graph node* fine, *relational node* not used, *term* a word of the syntax, `docs/ein/ein.rs/` free to name as software documentation needs; P1f.22's `q394_words.md` |
| [Q-M1f.395](#q-m1f395--what-ein-render-draws-of-a-fact-under-an-arrow) | what `ein render` draws of a fact under an arrow: `kb` draws a fact that is held and nested twice, `rule` a nested pattern as one box | **ruled 2026-11-17**: every object is drawn once, and *"3-arity fact can [be] an arrow only when nothing points on it"*: a fact that a fact points at is drawn as its node with its own arrows, held or not, in `ein render kb` and `ein render rule`; P1f.22's `contexts.md` § 7.3. **Built** by S1f.22.4; the mark re-ruled with Q-M1f.398 |
| [Q-M1f.396](#q-m1f396--where-the-explanation-stands-and-which-example-it-uses) | where the explanation of nested facts stands in `docs/ein/reasoning/`, and which example it is told on | **ruled 2026-11-17: (a)**, sections of the two pages of `01-static-ir/` (*"Q-M1f.396 sections"*); the example not named, so the default stands, the user's example beside the page's `says` |
| [Q-M1f.397](#q-m1f397--the-reserved-words-in-the-three-perspectives) | the reserved words in the three perspectives of the documentation | **ruled 2026-11-17** by the user's note: in `docs/ein/reasoning/` they are conceptual mathematical objects; in `docs/ein/user/` and `docs/ein/ein.rs/` they are the surface between the reasoning kernel and a theory in pure Ein; P1f.22's `contexts.md` § 8 |
| [Q-M1f.398](#q-m1f398--how-the-graph-is-drawn-to-show-the-global-context-and-a-facts-context) | how the graph is drawn to show that a fact is in the global context, in a fact's context, or both, every object drawn once | **ruled 2026-11-17**, in two looks at the trial's figures: *"I would like to see frame around facts in contexts rather than around global facts"*, then *"label contexts `<relation name> cts` instead of `in context`"*. So: a fact is **filled** where held, and **inside a dashed frame labelled with the context's relation**, `think ctx`, where it is in a context; a fact that facts of two relations point at is in one frame that names both; frames stand side by side; S1f.22.2 drew so. **Re-ruled 2026-11-18**: *"let's draw a frame around objects that are in context only"*, and *"drop the filling"*: a frame around what is in a context only, a held object in no frame, no octagon filled; **built** by S1f.22.7 (`58b91dc3`) |
| [Q-M1f.399](#q-m1f399--the-names-of-what-controls-the-search-false-and-open) | the names of what controls the search, `(false)` and `(open)`: the user's *"Maybe word `false` is too heavy loaded with classical binary logic and not expose its actual function in ein? Maybe (abort) or (dead) is more clear? Or some common search control interface accessible from ein? Like (ein.search.state.refute)?"* | **put 2026-11-17, not ruled; P1f.21's**, entered there as its V-5, with **the user's direction** of the same day: `(false)` stays a fact that rules infer, in a context or globally, and a state is dead only where a rule infers `(ein.contradiction)` from the global `(false)` or from whatever a theory chooses; *"both (false) renaming and (open) facts belongs to p1f.21"*. Nothing is renamed or built in P1f.22 |
| [Q-M1f.400](#q-m1f400--how-a-saved-file-states-the-choices-made) | how a saved file states the choices made: no form does today, and a choice typed as a fact reads as given and is `written` | **ruled 2026-11-18: (a)** (*"Q-M1f.400 (a), Q-M1f.401 (a), Q-M1f.402 (C), Q-M1f.403 (a)"*): a form of its own, `(commit F …)`, committed after root's fixpoint as a node commits it; its three points as defaulted. **Built** by S1f.22.5 |
| [Q-M1f.401](#q-m1f401--how-the-program-and-its-modules-are-in-one-file-unchanged) | how the program and its modules are in one file, unchanged: inlining rewrites the program, and the import lines alone need the modules | **ruled 2026-11-18: (a)**: `(program …)` and `(module NAME …)` around each file's bytes as read; a saved file reads no other file; its four points as defaulted. **Built** by S1f.22.5. Point 2 amended the same day by the user's ruling on F23's D-24: no note, nothing compared; built by S1f.23.8 |
| [Q-M1f.402](#q-m1f402--what-save-and-load-are-as-commands-the-dump-they-are-held-to-and-a-files-first-lines) | what *save* and *load* are as commands, the dump the second invariance compares, what a save of a loaded file is, and a file's first lines | **ruled 2026-11-18: (C)** for a save of a loaded file, a saved file a node of its program's lattice; the defaults for the rest: no new command, the dump `ein saturate --dump`'s KB section, one fixed first line. **Built** by S1f.22.5 |
| [Q-M1f.403](#q-m1f403--the-programs-claims-in-a-saved-file) | the program's claims in a saved file: 188 of 826 state files with a commitment exit 1 today, each a claim about the program's R checked against the file's own | **ruled 2026-11-18: (a)**: a file with a commitment checks no claim, names each NOT CHECKED, and exits 0. **Built** by S1f.22.5; `ein test` of one asks nothing since Q-M1f.416 (b), S1f.23.8 |
| [Q-M1f.404](#q-m1f404--written-is-erased-a-numeral-is-checked-by-the-operation-that-takes-it) | `written` is erased; a numeral is checked by the operation that takes it | **ruled 2026-11-18** by the user (*"revert this decision, completely erase this `written` crutch …"*): Q-M1f.272, .275, .279 and .280 reverted whole, and the half of .210 and .276 that flagged a numeral where it is written. **Built** by S1f.22.6 (`bf9693dc`); its readings are Q-M1f.405 to .411, ruled 2026-11-18 and built by [S1f.23.9](p1f.23_review_processing/s1f.23.9_numerals_answers.md) |
| [Q-M1f.405](#q-m1f405--which-numerals-are-taken-from-the-input-program-and-what-once-is) | which numerals are *taken from the input program*, and what *once* is | **ruled 2026-11-18: (a), with the user's distinction** (*"(a) the difference is between numerals produced by operation from valid numerals, and all other numerals from any source"*): a numeral an operation makes from valid numerals is valid as made and not walked again; every other one is checked, a request's, a rule's, and one handed straight to a tagged call, which is dead again. **Built** by [S1f.23.9](p1f.23_review_processing/s1f.23.9_numerals_answers.md): each tagged call checks what it is handed and hands itself on to a walk of the calculator's own, which the calculator calls itself |
| [Q-M1f.406](#q-m1f406--the-label-and-where-a-number-an-operation-makes-gets-it) | the label, and where a number an operation makes gets it | **ruled 2026-11-18: (a)**, the default as built: `(valid N)` and `(bin-valid N)`, a fact of the numeral, concluded where a walk ends and where the arithmetic makes a number |
| [Q-M1f.407](#q-m1f407--a-numeral-no-operation-takes-is-not-checked) | a numeral no operation takes is not checked | **ruled 2026-11-18: (a)**, the default as built: not checked; the three corpus programs claim R = {∅} |
| [Q-M1f.408](#q-m1f408--where-the-checks-rules-live) | where the check's rules live | **ruled 2026-11-18: (b)**, *the walk in `std.math.positional`, the representation keeping the flags*. **Built** by [S1f.23.9](p1f.23_review_processing/s1f.23.9_numerals_answers.md): `numeral-step`, `numeral-step-zero` and `numeral-valid` are `std.math.positional`'s, activated by each base's call with its representation's `check`, `valid`, numeral and `nz`; the relations, the label and the flags stay the representation's |
| [Q-M1f.409](#q-m1f409--an-integer-and-a-rational-their-check-and-their-label) | an integer and a rational: their check and their label | **ruled 2026-11-18: (a)**, and, asked how L-101's rule reads a rational whose part is a natural numeral, *"Verify the natural"*: the numerator is checked as a numeral, so `(rat (n z x) (n z 2))` is dead again (the (d) case). **Built** by [S1f.23.9](p1f.23_review_processing/s1f.23.9_numerals_answers.md), with L-101 |
| [Q-M1f.410](#q-m1f410--no-join-on-a-number-and-a-result-a-programs-rule-hands-on) | no join on a number, and a result a program's rule hands on | **ruled 2026-11-18: (a)**, the default as built: a made number a program's rule hands to a second request is walked again, and, under Q-M1f.405's ruling, one it hands to a tagged call |
| [Q-M1f.411](#q-m1f411--a-numeral-with-two-faults-has-one-flag) | a numeral with two faults has one flag | **ruled 2026-11-18: (a)**, the default as built: the first place at fault from the units |
| [Q-M1f.412](#q-m1f412--the-read-outs-is-a-row) | the read-out's `is-a` row | **ruled 2026-11-18** (*"V-6 (a), drop the is-a row"*), **built** `5a3f2979`; P1f.21's V-6. Its reading, no schema id moves, **defaulted**: (a); put to the user in the review bundle, who asked instead how deep `is-a` goes into the kernel (2026-11-19): nowhere, answered in the entry; **the default ruled 2026-11-19** with the phase's (*"accept all remain defaults in this phase"*) |
| [Q-M1f.413](#q-m1f413--ein-kb-save-of-a-saved-file) | `ein kb save` of a saved file | **ruled 2026-11-18: (c)** (*"(c)"*: give the container a section for the commitment, so a node's container opens as that node); **built** by P1f.23's S1f.23.8 (`86d46410`): the section `COMMIT`, kind 13, the format `einb/2.1`, the decision [Q-M1f.413](../../docs/decisions/engine/Q-M1f.413-a-nodes-container-opens-as-the-node.md). Before: defaulted (a), a node refused |
| [Q-M1f.414](#q-m1f414--a-facts-english-which-a-state-file-wrote-in-a-comment) | a fact's English, which a state file wrote in a comment | **resolved 2026-11-18** by S1f.23.5's rulings (a `:why` reads only the form it is written on; facts stay as they are): a fact has no English of its own, so no state file has a fact's English to write, and (a) stands; [the milestone's findings](findings.md) L-102 and L-105 follow, the writer gone and `utils/nl_census.py`'s answer half retired in S1f.23.5's build (`fbe2ba66`, `3047bbae`, merged by `79dd213b`). Before: **defaulted 2026-11-18 (S1f.22.5's build), the user's to revise**: (a), the trace is where a fact reads in English |
| [Q-M1f.415](#q-m1f415--ein-show-hyps-and-ein-render-h-space-of-a-saved-file) | `ein show hyps` and `ein render h-space` of a saved file | **ruled 2026-11-18: (a)** (*"(a)"*), the default as built: both read the program's H, the node aside |
| [Q-M1f.416](#q-m1f416--what-ein-test-asks-of-a-saved-files-node-and-how-its-claims-exit) | what `ein test` asks of a saved file's node, and how its claims exit | **ruled 2026-11-18: (b)** (*"(b)"*: no run at all, each claim NOT CHECKED, no run's data in `--json-report`); **built** by P1f.23's S1f.23.8 (`86d46410`), the report's id kept, `ein-test-report/6`. Before: defaulted (a), root alone |
| [Q-M1f.417](#q-m1f417--what-a-run-of-a-saved-file-says-of-its-node-and-the-directorys-schema) | what a run of a saved file says of its node, and the directory's schema | **ruled 2026-11-18: (a)** (*"(a)"*), the default as built: the read-out's `commit` row, `root.commit`, `index.json`'s `commit`, `ein-answer/3` |
| [Q-M1f.418](#q-m1f418--the-nesting-limit-and-a-saved-files-wrappers) | the nesting limit, and a saved file's wrappers | **ruled 2026-11-18: (a)** (*"(a)"*), the default as built: a saved file parsed with one level more, each text it carries under the limit; Q-M1f.185 rewritten |
| [Q-M1f.419](#q-m1f419--how-ein-render-draws-a-fact) | how `ein render` draws a fact | **ruled 2026-11-18 by the user**: the canonical form by default, a fact of three arrows folded into a labelled arrow on request, no labelled octagon and no text node produced or drawn on a page; built by P1f.23's S1f.23.1 |
| [Q-M1f.420](#q-m1f420--what-ein-render-program-draws) | what `ein render program` draws | **ruled 2026-11-18 by the user**: the program's knowledge base as one graph, as the KB view draws one, its rules, its query; no split into ontology and facts; built by S1f.23.1 |
| [Q-M1f.421](#q-m1f421--is-a-and-relation-in-a-drawing) | `is-a` and `relation` in a drawing | **ruled 2026-11-18 by the user**: ordinary relations, nothing special in any drawing; `--no-types` and `--no-instances` go; built by S1f.23.1 |
| [Q-M1f.422](#q-m1f422--the-traces-drawings-folded) | the trace's drawings folded | **ruled 2026-11-18 by the user**: `ein solve --fold` folds the trace's drawings; built by S1f.23.1 |
| [Q-M1f.423](#q-m1f423--how-the-arrows-of-a-fact-are-numbered) | how the arrows of a fact are numbered | **ruled 2026-11-18 by the user**: (a), `1` at the relation, `2`, `3`, … at the arguments, digits alone; carried by the decision Q-M1f.419 |
| [Q-M1f.424](#q-m1f424--where-what-a-fact-rests-on-is-drawn) | where what a fact rests on is drawn | **ruled 2026-11-18 by the user**: (a), an `xlabel` beside the octagon; carried by Q-M1f.419 |
| [Q-M1f.425](#q-m1f425--the-folds-option-and-what-it-folds) | the fold's option, and what it folds | **ruled 2026-11-18 by the user**: (a) + (b), `--fold` on every command that draws facts, a fact of three arrows an arrow and a fact of two a loop on its argument; **built** by S1f.23.6; the decision Q-M1f.425 |
| [Q-M1f.426](#q-m1f426--what-the-constraint-view-draws) | what the constraint view draws | **ruled 2026-11-18 by the user**: (a), each fact as a fact; the user's addition, a slice of the KB defined by a query, is S1f.23.7 (planned, not started); the decision Q-M1f.426 |
| [Q-M1f.427](#q-m1f427--how-a-rules-pattern-is-drawn) | how a rule's pattern is drawn | **ruled 2026-11-18 by the user**: (a), as a fact; `(not P)` a red fact of `not`; `forall` a pattern as called (R-15 (i)); the decision Q-M1f.427 |
| [Q-M1f.428](#q-m1f428--how-a-derivation-is-drawn) | how a derivation is drawn | **ruled 2026-11-18 by the user**: (a), and *"more contrast colors and 2x thicker lines"*: a derivation's arrows and steps `penwidth=2`, in a dark palette by the rule's name; **built** by S1f.23.6; the decision Q-M1f.428 |
| [Q-M1f.429](#q-m1f429--a-folded-fact-in-a-drawing-of-a-derivation) | a folded fact in a drawing of a derivation | **ruled 2026-11-18 by the user**: no arrow to or from an arrow; a fact a derivation's arrow touches never folded, every other fact of two or three arrows folded, no point; **built** by S1f.23.6; the decision Q-M1f.429 |
| [Q-M1f.430](#q-m1f430--the-region-lattice-and-h-space-views) | the region, lattice and h-space views | **ruled 2026-11-18 by the user**: (a), left as they are; the canonical form for a KB alone (Q-M1f.470); the decision Q-M1f.430 |
| [Q-M1f.431](#q-m1f431--the-program-views-three-parts) | the program view's three parts | **ruled 2026-11-18 by the user**: (a), as built; a claim's text in a query's note stays; carried by Q-M1f.420, amended |
| [Q-M1f.432](#q-m1f432--what-the-kb-view-leaves-out) | what the KB view leaves out | **ruled 2026-11-18 by the user**: (a), nothing left out; carried by Q-M1f.420, amended |
| [Q-M1f.433](#q-m1f433--the-shape-of-an-atom) | the shape of an atom | **ruled 2026-11-18 by the user**: (a): an object a labelled box, a fact an unlabelled octagon, a variable a labelled diamond; the decision Q-M1f.433 |
| [Q-M1f.434](#q-m1f434--the-standard-librarys-rule-drawings-past-the-bound) | the standard library's rule drawings past the bound | **ruled 2026-11-18 by the user**: (a), canonical, else folded, else Ein only; the bound kept (L-97); the decision Q-M1f.434 |
| [Q-M1f.435](#q-m1f435--where-an-arrows-number-is-written) | where an arrow's number is written | **ruled 2026-11-18 by the user**: (a), its `xlabel`, and a folded arrow's relation in a knowledge base's drawing too; the decision Q-M1f.435 |
| [Q-M1f.436](#q-m1f436--the-kernel-reads-no-theorys-name-and-the-built-in-symmetry-goes) | the kernel reads no theory's name, and the built-in symmetry goes | **ruled 2026-11-18 by the user**: the kernel gives a fixed meaning to its own names alone; `is-a`, `symmetric` and every other name a theory's or a program's; the native mirror dropped, `__symmetric__` an ordinary name; **built** by S1f.23.4, with S1f.23.1's and S1f.23.2's parts, held by the renaming test; the decision Q-M1f.436 |
| [Q-M1f.437](#q-m1f437--the-flagged-spelling-of-symmetric-derived-node) | the flagged spelling of `symmetric-derived-node` | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.4): (a), not the stop condition, the answer the same; the program retired |
| [Q-M1f.438](#q-m1f438--the-programs-that-showed-only-the-mirror-retired-or-rewritten) | the programs that showed only the mirror: retired or rewritten | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.4): (a), all six retired with their corpus entries and four regression rows |
| [Q-M1f.439](#q-m1f439--examplesfeatures-numbering-with-06-gone) | `examples/features/`' numbering with `06` gone | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.4): (a), kept, a gap at `06` |
| [Q-M1f.440](#q-m1f440--the-retired-flag-enable-symmetric-mirror) | the retired flag `enable-symmetric-mirror` | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.4): (a), stays retired, refused with a new reason, its field kept |
| [Q-M1f.441](#q-m1f441--the---events-protocols-version-the-mirror-kind-gone) | the `--events` protocol's version, the `mirror` kind gone | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.4): (a), stays `ein-events/1` |
| [Q-M1f.442](#q-m1f442--q-m1f7s-file-name-its-title-changed) | Q-M1f.7's file name, its title changed | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.4): (a), renamed to the new title's slug on the merge, every link with it |
| [Q-M1f.443](#q-m1f443--a-decision-for-the-two-rulings) | a decision for the two rulings | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.4): (a), a decision of its own, Q-M1f.436, taken on the merge; Q-M1f.7 cites it |
| [Q-M1f.444](#q-m1f444--what-the-renaming-test-keeps) | what the renaming test keeps | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.4): (a), as built: the kernel's names, `:over`'s values, the grammar's words, module names and aliases; the read-out's text not compared |
| [Q-M1f.445](#q-m1f445--the-renaming-tests-tier) | the renaming test's tier | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.4): (a), per commit outside `tests/`, every program in the nightly |
| [Q-M1f.446](#q-m1f446--the-forall-drawing-the-renaming-test-found) | the `forall` drawing the renaming test found | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.4): (a), S1f.23.1's, which dropped the read (Q-M1f.427); `READS` emptied on the merge |
| [Q-M1f.447](#q-m1f447--the-loaders-argument-count-check-once-relation-is-a-convention) | the loader's argument-count check, once `relation` is a convention: kept at load, a rule's verdict at run time, or dropped | **open, the user's, not ruled** (*"will think about it in p1f.21"*): no default taken; P1f.21's S1f.21.4 shows each option and does not choose |
| [Q-M1f.448](#q-m1f448----models-with-key-and-grid-gone) | `--models`, with `key` and `grid` gone | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.2): (a), removed whole, from `ein solve` and `ein oracle` |
| [Q-M1f.449](#q-m1f449----models-removed-or-retired) | `--models`: removed, or retired | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.2): (a), removed, not retired: clap refuses it, exit 2 |
| [Q-M1f.450](#q-m1f450----models-grids-reading-of-is-a-and-the-grids-names) | `--models grid`'s reading of `is-a`, and the grid's names | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.2): (a), ended with the code: `grid.rs` deleted, P1f.23's R-1 and R-3 with it |
| [Q-M1f.451](#q-m1f451--the-determining-key-on-the-reasoning-page) | the determining key on the reasoning page | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.2): (a), taken out with the term and *what models differ in* |
| [Q-M1f.452](#q-m1f452--what-the-walkthrough-shows-where-it-showed-the-grids) | what the walkthrough shows where it showed the grids | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.2): (a), `-f`'s output, Ein facts, no table |
| [Q-M1f.453](#q-m1f453--modelsrs-name-once-it-holds-the-diff-alone) | `models.rs`' name, once it holds the diff alone | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.2): (a), kept; superseded on the merge by S1f.23.3's `claim_diff.rs` (Q-M1f.465) |
| [Q-M1f.454](#q-m1f454--ein-test---diffs-spelling) | `ein test --diff`'s spelling | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.3): (a), `--diff stats|full`, a bare `--diff` the stats where no word follows; before a path refused, exit 2 |
| [Q-M1f.455](#q-m1f455--how-many-facts-off-a-near-model-is) | how many facts off a near model is | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.3): (a), an edit distance: a fact replaced counts once |
| [Q-M1f.456](#q-m1f456--what-the-diff-counts-as-a-match) | what the diff counts as a match | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.3): (a), what the claim's check pairs, a maximum matching: a held claim's diff is all matches |
| [Q-M1f.457](#q-m1f457--which-claims-have-a-diff) | which claims have a diff | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.3): (a), every claim a run decided, held or FAILED, but the region's `(false)` |
| [Q-M1f.458](#q-m1f458--a-missing-model-and-an-extra-found-item) | a missing model and an extra found item | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.3): (a), a missing model against its nearest found item with its facts; an extra found item names its nearest model, without facts |
| [Q-M1f.459](#q-m1f459--an-owing--claims-diff) | an `(owing …)` claim's diff | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.3): (a), against the owing frontier, *owing node i* |
| [Q-M1f.460](#q-m1f460--merged--and-the-diff) | `(merged …)` and the diff | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.3): (a), not diffed: the claim's own lines say why |
| [Q-M1f.461](#q-m1f461--where-the-diff-prints) | where the diff prints | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.3): (a), after the claim's lines, every claim with a diff at the default volume; `-q` diffs only what it prints |
| [Q-M1f.462](#q-m1f462--the-reports-diff-and-its-id) | the report's `diff`, and its id | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.3): (a), `diff` on every row, `null` where none; `ein-test-report/6` |
| [Q-M1f.463](#q-m1f463--the-review-bundles-right-answer-written-as-claims) | the review bundle's right answer, written as claims | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.3): (a), query claims, one per relation of `closes`, over the row's line |
| [Q-M1f.464](#q-m1f464--the-review-run-pages-cross-check-1) | the review run page's cross-check 1 | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.3): (a), the `ein test` run's claims against the listed states, by count |
| [Q-M1f.465](#q-m1f465--the-diffs-module) | the diff's module | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.3): (a), `claim_diff.rs` |
| [Q-M1f.466](#q-m1f466----diff-models-removed-or-retired) | `--diff-models`: removed, or retired | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.3): (a), removed, not retired: clap refuses it, exit 2 |
| [Q-M1f.467](#q-m1f467--a-commands-output-on-a-page-canonical-or-folded) | A command's output on a page: canonical or folded | **ruled 2026-11-18 by the user**: (a); the decision Q-M1f.467 |
| [Q-M1f.468](#q-m1f468--a-pages-kb-drawing-the-whole-view-or-a-cut) | A page's KB drawing: the whole view or a cut | **ruled 2026-11-18 by the user**: (a); the decision Q-M1f.468 |
| [Q-M1f.469](#q-m1f469--how-a-derivations-drawing-marks-a-terminal-and-the-core) | How a derivation's drawing marks a terminal and the core | **ruled 2026-11-18 by the user**: (a); the decision Q-M1f.469 |
| [Q-M1f.470](#q-m1f470--which-hand-drawings-the-canonical-form-leaves-as-they-are) | Which hand drawings the canonical form leaves as they are | **ruled 2026-11-18 by the user**: (a), *"canonical drawing is ONLY for Ein KB (objects and facts)"*; a small lattice with its states' facts drawn an exception, filed to M20; the decision Q-M1f.470 |
| [Q-M1f.471](#q-m1f471--01-object-orders-drawing) | `01-object-order`'s drawing | **ruled 2026-11-18 by the user**: (a), *"their own arrows have nothing related to this ordering example"* |
| [Q-M1f.472](#q-m1f472--names-of-the-drawings-that-changed-form) | Names of the drawings that changed form | **ruled 2026-11-18 by the user**: (a) |
| [Q-M1f.473](#q-m1f473--programs-a-page-draws-that-no-file-holds) | Programs a page draws that no file holds | **ruled 2026-11-18 by the user**: (a); the decision Q-M1f.473 |
| [Q-M1f.474](#q-m1f474--an-arrow-in-a-hand-schematic-that-is-no-arrow-of-the-graph) | An arrow in a hand schematic that is no arrow of the graph | **ruled 2026-11-18 by the user**: (a) for a hand drawing, *"not render output"*; the decision Q-M1f.474 |
| [Q-M1f.475](#q-m1f475--where-a-declarations-statement-prints-on-the-premises-line) | Where a declaration's statement prints on the premises line | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.5, task 2): (a), right after the declaration, ` — <statement>`, the premises' sources still once after the last premise, so a line can carry two dashes (the examples page's open point 3 (a)) |
| [Q-M1f.476](#q-m1f476--a-declaration-a-rule-concludes) | A declaration a rule concludes | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.5, task 2): (a), it reads by its `:why` on the *Derives* line, however a run reaches it (open point 10); no corpus program derives one |
| [Q-M1f.477](#q-m1f477--the-bound-on-a-declarations-references) | The bound on a declaration's references | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.5, task 2): (a), `{?2}` to `{?n+2}`, `{?1}` refused by `template_wellformedness.rs`, an untyped declaration bounded to `{?2}`; the engine prints a `{?1}` as `relation` and a reference past the arrows as written (open point 7 (a)) |
| [Q-M1f.478](#q-m1f478--l-102s-reach) | L-102's reach | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.5, task 2): (a), `sentence.rs`' `fact_sentence`, `FactSentence`, `Origin` and `as_comment` go with the six functions L-102 names, which alone read them; the shape golden's `states` op digests each fact and its first origin, no English |
| [Q-M1f.479](#q-m1f479--l-105s-reach) | L-105's reach | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.5, task 2): (a), `nl_census.py`'s `KERNEL` copy goes with its answer half, and `check_tool_mirrors.py`'s census mirror with it; § 4's registers stay held to `ein_core::RESERVED` by `natural_language_page.rs` |
| [Q-M1f.480](#q-m1f480--the-decisions-the-ruling-rewrites) | The decisions the ruling rewrites | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.5, task 2): (a), Q-M1f.58 rewritten as the ruling (title extended, slug kept); Q-M1f.61's title extended (slug kept); Q-M1f.60, Q-M1f.214, Q-M1f.12 and Q31 restated; Q27's example renumbered; Q-M1f.62, Q-M1f.57 and Q-M1f.63 stand |
| [Q-M1f.481](#q-m1f481--the-pages-the-ruling-rewrites) | The pages the ruling rewrites | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.5, task 2): (a), `14-natural-language.md` rewritten, its § 3 block from `zebra2.ein`; six pages' trace blocks re-taken from their programs; the prose of nine more pages and AGENTS.md's rule on `:why` |
| [Q-M1f.482](#q-m1f482--the-words-of-a-renumbered-why) | The words of a renumbered `:why` | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.5, task 2): (a), not rewritten (open point 9): only the page examples that show the rule use the user's pattern, `a {?3} {?2} a {?4}`; the shipped texts read stiffly as their declaration, and the pages say so |
| [Q-M1f.483](#q-m1f483--the-region-views-words-on-an-arrow) | The region view's words on an arrow | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.1, task 4): (a), `sat` and `✗ alive`, words on arrows with `constraint=false`, counted as R-31's pattern and written as `xlabel`s, though Graphviz refused none |
| [Q-M1f.484](#q-m1f484--a-traces-slice-that-graphviz-refuses) | A trace's slice that Graphviz refuses | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.1, task 4): (a), left as it is and its refusals filed (the phase's R-33), since what routes them, `rankdir=TB`, changes how a slice is laid out (Q-M1f.428, Q-M1f.429) |
| [Q-M1f.485](#q-m1f485--what-09-core-in-derivation-draws-heavy) | What `09-core-in-derivation` draws heavy | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.1, task 4): (a), the facts the derivations added, *heavy* being *a derivation added it* as on the page and in `09-proof-graph`; the core red and double-bordered, not heavy |
| [Q-M1f.486](#q-m1f486--long_numbersein-and-the-pools-long-lane) | `long_numbers.ein` and the pool's long lane | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.1, task 4): (a), not on `ein_corpus::pool::LONG`: at four workers the order units start in cannot shorten a sweep the work bounds |
| [Q-M1f.487](#q-m1f487--how-the-layers-of-a-knowledge-bases-drawing-are-written) | How the layers of a knowledge base's drawing are written | **ruled 2026-11-19** by the user (item 56): (a) as built; the concentric layout the user asks about is put as examples by [S1f.23.11](p1f.23_review_processing/s1f.23.11_drawings_second_answers.md), task 2, ruled (a), nothing built ([Q-M1f.547](#q-m1f547--a-concentric-layout-the-layers-drawn-as-rings)); before, defaulted (S1f.23.6): (a), from the arrows alone, no name read: `rankdir=TB`, a first arrow drawn back, and in a view of a knowledge base `rank=same` groups, objects and facts in turn, a band as deep as its facts nest, ordered by invisible anchors; none where the arrows go round, nor in a panel, a slice or the derivation graph |
| [Q-M1f.488](#q-m1f488--a-folded-arrow-within-a-layer-or-ranked) | A folded arrow: within a layer, or ranked | **ruled 2026-11-19** by the user (item 57): (a) as built; the concentric layout, [S1f.23.11](p1f.23_review_processing/s1f.23.11_drawings_second_answers.md), task 2, ruled (a), nothing built ([Q-M1f.547](#q-m1f547--a-concentric-layout-the-layers-drawn-as-rings)); before, defaulted (S1f.23.6): (a), within a layer, `constraint=false`, its fact placing its names as its octagon would, so a folded drawing keeps the canonical one's layers of objects; the first build's ranking reversed |
| [Q-M1f.489](#q-m1f489--a-folded-arrow-in-a-rules-panel-and-in-a-slice) | A folded arrow in a rule's panel and in a slice | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.6): (a), its `label` and its ranking kept (Q-M1f.435 (a)), since `constraint=false` beside a `label` is what Graphviz could not route (R-31) |
| [Q-M1f.490](#q-m1f490--a-stored-negation-under---fold) | A stored negation under `--fold` | **ruled 2026-11-19** by the user (item 58): (a) as built; before, defaulted (S1f.23.6): (a), `(not g)` folds into a loop on `g`'s octagon, as any fact of two arrows; no name read |
| [Q-M1f.491](#q-m1f491--the-closing-rules-drawings-under---fold) | The closing rules' drawings under `--fold` | **ruled 2026-11-19** by the user (item 58): (a) as built; before, defaulted (S1f.23.6): (a), the five closing rules drawn folded, within `MAX_NODES` (L-97), where they were Ein only |
| [Q-M1f.492](#q-m1f492--the-derivations-palette) | The derivation's palette | **ruled 2026-11-19** by the user (item 59): (a) amended: no colour near black; each rule's hue in two shades, the brighter the derived fact's; built by [S1f.23.11](p1f.23_review_processing/s1f.23.11_drawings_second_answers.md), task 1 (`21728654`, merged by `9f5c0487`; [Q-M1f.548](#q-m1f548--a-derivations-three-levels-as-built)); before, defaulted (S1f.23.6): (a), eight dark colours by the rule's name, the derivation graph in the first, a step's border `penwidth=2`, a redundant firing dashed at that width |
| [Q-M1f.493](#q-m1f493--how-a-derived-fact-is-marked-beside-a-derivation) | How a derived fact is marked beside a derivation | **ruled 2026-11-19** by the user (item 59): amended: a derived fact is drawn in its derivation's hue, the brightest shade, and thick; built by [S1f.23.11](p1f.23_review_processing/s1f.23.11_drawings_second_answers.md), task 1 (`21728654`, merged by `9f5c0487`; [Q-M1f.548](#q-m1f548--a-derivations-three-levels-as-built)); before, defaulted (S1f.23.6): (a), bold, `penwidth=2`, the derivation's width, told from its arrows by colour |
| [Q-M1f.494](#q-m1f494--a-steps-slice-under---fold) | A step's slice under `--fold` | **ruled 2026-11-19** by the user (item 59): (a) as built: a step's slice the same under `--fold`; before, defaulted (S1f.23.6): (a), the canonical one, every fact of it touched by the step's arrows (Q-M1f.429); `--fold` folds a trace's `-F` snapshots |
| [Q-M1f.495](#q-m1f495--how-the-pages-copies-of-commands-outputs-were-drawn-again) | How the pages' copies of commands' outputs were drawn again | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.6): (a), by a transformer validated against the binary, so that a cut stays its cut (Q-M1f.468); two hand derivations restyled by hand, `reasoning/01-static-ir/`'s hand graphs left |
| [Q-M1f.496](#q-m1f496--the-fdp-provisioning-now-that-no-view-suggests-fdp) | The `fdp` provisioning, now that no view suggests `fdp` | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.6): (a), left: the CI's layout plugin, `run_tests.sh`'s exit 127 and the comments that say every KB view suggests `fdp`, wrong from the stage on; the main session's call |
| [Q-M1f.497](#q-m1f497--where-r-33s-check-lives-and-what-it-reads) | Where R-33's check lives, and what it reads | **ruled 2026-11-19** by the user (item 60): (a) as built; before, defaulted (S1f.23.6): (a), `dot_wellformed`, per commit, every slice of two witnesses' traces (611 slices) |
| [Q-M1f.498](#q-m1f498--whether-a-page-pins-the-layers) | Whether a page pins the layers | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.6): (a), no page pins a `rank`: the layers a preference no test holds (Q21) |
| [Q-M1f.499](#q-m1f499--what-a-reader-of-einb20-makes-of-a-nodes-container) | What a reader of `einb/2.0` makes of a node's container | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.8): (a), the minor moves, `einb/2.1`, as the format page's rule says, so a 2.0 reader opens a node's container as its program's root |
| [Q-M1f.500](#q-m1f500--what-a-roots-container-is-unchanged-means) | What *a root's container is unchanged* means | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.8): (a), the same sections byte for byte and no `COMMIT`, its header's minor 1 |
| [Q-M1f.501](#q-m1f501--the-words-a-nodes-container-prints) | The words a node's container prints | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.8): (a), the saved file's, not the mock's *container*: the container's output is its file's but the path (Q-M1a.22) |
| [Q-M1f.502](#q-m1f502--a-load-notes-position-in-a-nodes-container) | A load note's position in a node's container | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.8): (a), the stored program's (Q-M1f.15), compared as `<position>` by `by_its_container`, and the pages say so |
| [Q-M1f.503](#q-m1f503--a-nodes-container-and-the-standard-librarys-digest) | A node's container and the standard library's digest | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.8): (a), as is: the container records the digest and warns `StaleStdlib` under another library where the saved file says nothing; the answer the same |
| [Q-M1f.504](#q-m1f504--ein-kb-save-of-a-node-whose-choice-is-no-candidate) | `ein kb save` of a node whose choice is no candidate | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.8): (a), as is: stored, and refused by every command that reads the node |
| [Q-M1f.505](#q-m1f505--ein-kb-save---saturate-of-a-node) | `ein kb save --saturate` of a node | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.8): (a), as is: root's fixpoint and the commitment banked, not the node's state |
| [Q-M1f.506](#q-m1f506--a-bad-commitment-under-ein-test) | A bad commitment under `ein test` | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.8): (a), NOT CHECKED, exit 0, since no run builds H; the other commands still refuse it |
| [Q-M1f.507](#q-m1f507----json-summary-and---events-under-ein-test-of-a-node) | `--json-summary` and `--events` under `ein test` of a node | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.8): (a), nothing written, as for a file that states no claim |
| [Q-M1f.508](#q-m1f508--the--v-header-of-a-saved-nodes-claims) | The `-v` header of a saved node's claims | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.8): (a), no question named, the mock's *(a saved file's node: not run)* not taken |
| [Q-M1f.509](#q-m1f509--the-reports-row-of-a-claim-no-run-checked) | The report's row of a claim no run checked | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.8): (a), `outside` `null` and no `region` key, not the mock's `[]` and `region: null`; the id stays `ein-test-report/6` |
| [Q-M1f.510](#q-m1f510--claimchecks-saved-file-branch) | `claim::check`'s saved-file branch | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.8): (a), unchanged |
| [Q-M1f.511](#q-m1f511--decision-files-for-q-m1f415-to-418) | Decision files for Q-M1f.415 to .418 | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.8): (a), none: each answer is a file it already has (Q-M1f.403's, .402's, .185's) |
| [Q-M1f.512](#q-m1f512--where-a-nodes-container-is-held-to-its-saved-file-and-its-cost) | Where a node's container is held to its saved file, and its cost | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-18 (S1f.23.8): (a), `answer_round_trip`'s `by_its_container`, beside the dump it reads; its cost against the nightly's five minutes left to the merged tree's nightly |
| [Q-M1f.513](#q-m1f513--what-tells-a-numeral-to-check-from-one-an-operation-made) | What tells a numeral to check from one an operation made | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-19 (S1f.23.9): (a), the relation it is written in: a tagged call, which asks the check and hands itself on to a walk of its own, which the calculator alone writes and which checks nothing |
| [Q-M1f.514](#q-m1f514--the-names-of-the-calls-and-of-the-walks) | The names of the calls and of the walks | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-19 (S1f.23.9): (a), the calls keep theirs (Q-M1f.189, Q-M1f.228), the walks are new, `add-run` … `gcd-run` |
| [Q-M1f.515](#q-m1f515--the-public-division-a-call-or-a-walk) | The public division: a call or a walk | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-19 (S1f.23.9): (a), a call, so that its division-by-zero flag names a fact the state holds (Q-M1f.211); +3 firings a division |
| [Q-M1f.516](#q-m1f516--what-the-division-by-zero-flag-names) | What the division-by-zero flag names | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-19 (S1f.23.9): (a), the call, `(div-call …)`, whatever made the division; every flag's text as before |
| [Q-M1f.517](#q-m1f517--what-an-integers-or-a-rationals-call-is-checked-as) | What an integer's or a rational's call is checked as | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-19 (S1f.23.9): (a), an operand of its kind, `int-check` and `rat-check`, so a digit handed as a part is flagged `not-integer` or `not-rational`, where it was silent |
| [Q-M1f.518](#q-m1f518--the-calculators-own-constants) | The calculator's own constants | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-19 (S1f.23.9): (a), not checked: `(n z 1)` and `(n z 2)`, written by its own rules into its own walks |
| [Q-M1f.519](#q-m1f519--the-decisions-the-numerals-rulings-live-in) | The decisions the numerals' rulings live in | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-19 (S1f.23.9): (a), Q-M1f.404's file, no new decision id |
| [Q-M1f.520](#q-m1f520--the-walks-words-in-binary) | The walk's words in binary | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-19 (S1f.23.9): (a), the base's: `numeral-step`'s `:why` says *digit* in binary too |
| [Q-M1f.521](#q-m1f521--what-the-tagged-call-test-holds) | What the tagged-call test holds | **ruled 2026-11-19: the default** (the user, *"accept all remain defaults in this phase"*); before, defaulted 2026-11-19 (S1f.23.9): (a), the walks as well as the calls, by `arith_theory.rs`' table `RUNS` |
| [Q-M1f.522](#q-m1f522--a-relation-is-any-atom-first-in-a-tuple) | a relation of the program: which atoms are, and what a declaration is | **ruled 2026-11-19** by the user (P1f.21's V-7, in the review bundle): *"Its enough to be the first item in the tuple to became a relation"*, at any depth; a declaration an optional fact, a convention. Built by [S1f.23.10](p1f.23_review_processing/s1f.23.10_language_answers.md), task 2 (`fdbb71ae`) |
| [Q-M1f.523](#q-m1f523--the-slice-by-a-query-its-definition) | The slice by a query: its definition | **ruled 2026-11-19: the default** (the user, *"S1f.23.7 - I accept"*): every fact a knowledge base holds with a relation of the program directly under an argument arrow (U (c)); not the constraint view's selection as it is, nor readings (a) or (b), nor at any depth. To be built by [S1f.23.7](p1f.23_review_processing/s1f.23.7_a_slice_by_a_query.md), task 2 |
| [Q-M1f.524](#q-m1f524--the-slice-by-a-query-what-a-relation-is) | The slice by a query: what *a relation* is | **ruled 2026-11-19: the default** (the user, *"S1f.23.7 - I accept"*): a relation of the program, one notion with the goal's and the claim's (Q-M1f.119, and Q-M1f.522 since the user's V-7); not a state's relations, nor the declared ones only. To be built by [S1f.23.7](p1f.23_review_processing/s1f.23.7_a_slice_by_a_query.md), task 2 |
| [Q-M1f.525](#q-m1f525--the-slice-by-a-query-an-atom-that-is-a-relation-and-an-object) | The slice by a query: an atom that is a relation and an object | **ruled 2026-11-19: the default** (the user, *"S1f.23.7 - I accept"*): selected as the graph has it, `z` in a numeral and `eq` in an order among them; `eq` and `neq` not left out. To be built by [S1f.23.7](p1f.23_review_processing/s1f.23.7_a_slice_by_a_query.md), task 2 |
| [Q-M1f.526](#q-m1f526--the-slice-by-a-query-depth) | The slice by a query: depth | **ruled 2026-11-19: the default** (the user, *"S1f.23.7 - I accept"*): directly, the fact's arrows 2, 3, …; not at any depth. To be built by [S1f.23.7](p1f.23_review_processing/s1f.23.7_a_slice_by_a_query.md), task 2 |
| [Q-M1f.527](#q-m1f527--the-slice-by-a-query-what-is-drawn-with-it) | The slice by a query: what is drawn with it | **ruled 2026-11-19: the default** (the user, *"S1f.23.7 - I accept"*): what its facts point at, at any depth, a fact held nowhere in its frame; not the selected facts alone. To be built by [S1f.23.7](p1f.23_review_processing/s1f.23.7_a_slice_by_a_query.md), task 2 |
| [Q-M1f.528](#q-m1f528--the-slice-by-a-query-a-fact-under-an-arrow-held-nowhere) | The slice by a query: a fact under an arrow, held nowhere | **ruled 2026-11-19: the default** (the user, *"S1f.23.7 - I accept"*): selected only through a held fact that points at it. To be built by [S1f.23.7](p1f.23_review_processing/s1f.23.7_a_slice_by_a_query.md), task 2 |
| [Q-M1f.529](#q-m1f529--the-slice-by-a-query-how-a-goal-is-given) | The slice by a query: how a goal is given | **ruled 2026-11-19: the default** (the user, *"S1f.23.7 - I accept"*): `ein render kb FILE --goal '(…)'`, of root or a saved state, with `--query N` beside it; not the form only, nor a saved state only. To be built by [S1f.23.7](p1f.23_review_processing/s1f.23.7_a_slice_by_a_query.md), task 2 |
| [Q-M1f.530](#q-m1f530--what-ein-render-constraints-becomes) | What `ein render constraints` becomes | **ruled 2026-11-19: the default** (the user, *"S1f.23.7 - I accept"*): the slice of Q-M1f.523 on root's saturated knowledge base, defined in the engine and drawn by the KB view's code, with no language change; not two computed relations, nor the view kept, nor removed for `--goal`. To be built by [S1f.23.7](p1f.23_review_processing/s1f.23.7_a_slice_by_a_query.md), task 2 |
| [Q-M1f.531](#q-m1f531--a-written-check-in-the-constraint-view) | A written check in the constraint view | **ruled 2026-11-19: the default** (the user, *"S1f.23.7 - I accept"*): `(eq a w)` left out, as the program view leaves it (R-40); not drawn as a fact. To be built by [S1f.23.7](p1f.23_review_processing/s1f.23.7_a_slice_by_a_query.md), task 2 |
| [Q-M1f.532](#q-m1f532--the-class-of-an-or--goal) | The class of an `(or …)` goal | **ruled 2026-11-19: the default** (the user, *"S1f.23.7 - I accept"*): a class lists the facts of the disjuncts that hold, and its `as :expect` parses back (R-39). To be built by [S1f.23.7](p1f.23_review_processing/s1f.23.7_a_slice_by_a_query.md), task 2 |
| [Q-M1f.533](#q-m1f533--the-kernels-words-no-relation-by-heading-a-tuple) | The kernel's words: no relation by heading a tuple | **ruled 2026-11-19: the default** (the user, *"Go with all defaults to the end of this phase p1f.23 and stop"*): `ein_core::terms::RESERVED` and the loader's `#` names, read by one function, `heads_a_relation`; `=` a relation; not `=` among the kernel's words. Built by [S1f.23.10](p1f.23_review_processing/s1f.23.10_language_answers.md), task 2 (`fdbb71ae`, merged by `cbb9635b`) |
| [Q-M1f.534](#q-m1f534--the-goals-and-claims-list-and-the-kernels-words) | The goal's and claim's list and the kernel's words | **ruled 2026-11-19: the default** (the user, *"Go with all defaults to the end of this phase p1f.23 and stop"*): the list is the written set and `eq`, `neq`, as before, the kernel's words a program writes among it; the registry leaves them out. Built by [S1f.23.10](p1f.23_review_processing/s1f.23.10_language_answers.md), task 2 (`fdbb71ae`, merged by `cbb9635b`) |
| [Q-M1f.535](#q-m1f535--a-solve-hrules--activators-relation) | A `(solve :hrules …)` activator's relation | **ruled 2026-11-19: the default** (the user, *"Go with all defaults to the end of this phase p1f.23 and stop"*): as before, out of the registry and the solve lists' set (Q-M1f.114), a relation a goal and a claim may name. Built by [S1f.23.10](p1f.23_review_processing/s1f.23.10_language_answers.md), task 2 (`fdbb71ae`, merged by `cbb9635b`) |
| [Q-M1f.536](#q-m1f536--a-declared-relation-that-no-tuple-heads) | A declared relation that no tuple heads | **ruled 2026-11-19: the default** (the user, *"Go with all defaults to the end of this phase p1f.23 and stop"*): as before, a relation of the program, the blind rung's eligible relations the declared ones; whether a declaration counts is P1f.21's, with Q-M1f.447. Built by [S1f.23.10](p1f.23_review_processing/s1f.23.10_language_answers.md), task 2 (`fdbb71ae`, merged by `cbb9635b`) |
| [Q-M1f.537](#q-m1f537--a-relation-only-roots-fixpoint-shows) | A relation only root's fixpoint shows | **ruled 2026-11-19: the default** (the user, *"Go with all defaults to the end of this phase p1f.23 and stop"*): no object, left out by `candidate_objects` (`Kb::name_as_head`); the registry, fixed at load, does not take it. Built by [S1f.23.10](p1f.23_review_processing/s1f.23.10_language_answers.md), task 2 (`fdbb71ae`, merged by `cbb9635b`) |
| [Q-M1f.538](#q-m1f538--the-relation-names-row-its-keys-and-its-module) | The `relation names` row, its keys and its module | **ruled 2026-11-19: the default** (the user, *"Go with all defaults to the end of this phase p1f.23 and stop"*): removed, not kept for a case that cannot arise; no schema id moves (Q-M1f.412's (a)). Built by [S1f.23.10](p1f.23_review_processing/s1f.23.10_language_answers.md), task 2 (`fdbb71ae`, merged by `cbb9635b`) |
| [Q-M1f.539](#q-m1f539--a-premises-own-node-in-a-derivations-drawing) | A premise's own node in a derivation's drawing | **ruled 2026-11-19: the default** (the user, *"Go with all defaults to the end of this phase p1f.23 and stop"*): the rest, black at the default width, its arrow into the step the explanation; a premise another step added keeps that step's bright shade. Built by [S1f.23.11](p1f.23_review_processing/s1f.23.11_drawings_second_answers.md), task 1 (`21728654`, merged by `9f5c0487`) |
| [Q-M1f.540](#q-m1f540--a-commitments-red-and-a-negations-grey-beside-the-three-levels) | A commitment's red and a negation's grey beside the three levels | **ruled 2026-11-19: the default** (the user, *"Go with all defaults to the end of this phase p1f.23 and stop"*): kept; a derived negation takes its step's bright shade. Built by [S1f.23.11](p1f.23_review_processing/s1f.23.11_drawings_second_answers.md), task 1 (`21728654`, merged by `9f5c0487`) |
| [Q-M1f.541](#q-m1f541--the-widths-of-a-derivations-two-emphasised-levels) | The widths of a derivation's two emphasised levels | **ruled 2026-11-19: the default** (the user, *"Go with all defaults to the end of this phase p1f.23 and stop"*): the added fact `penwidth=3`, the explanation `penwidth=2`. Built by [S1f.23.11](p1f.23_review_processing/s1f.23.11_drawings_second_answers.md), task 1 (`21728654`, merged by `9f5c0487`) |
| [Q-M1f.542](#q-m1f542--a-redundant-firings-conclusion) | A redundant firing's conclusion | **ruled 2026-11-19: the default** (the user, *"Go with all defaults to the end of this phase p1f.23 and stop"*): added by nothing, so the rest; its step dashed in the deeper shade. Built by [S1f.23.11](p1f.23_review_processing/s1f.23.11_drawings_second_answers.md), task 1 (`21728654`, merged by `9f5c0487`) |
| [Q-M1f.543](#q-m1f543--the-refutations-arrows) | The refutation's arrows | **ruled 2026-11-19: the default** (the user, *"Go with all defaults to the end of this phase p1f.23 and stop"*): red, the learned no-good's as the core's (library only). Built by [S1f.23.11](p1f.23_review_processing/s1f.23.11_drawings_second_answers.md), task 1 (`21728654`, merged by `9f5c0487`) |
| [Q-M1f.544](#q-m1f544--the-derivation-graphs-added-fact) | The derivation graph's added fact | **ruled 2026-11-19: the default** (the user, *"Go with all defaults to the end of this phase p1f.23 and stop"*): one a drawn derivation concludes whose first record is a rule's; its provenance note black. Built by [S1f.23.11](p1f.23_review_processing/s1f.23.11_drawings_second_answers.md), task 1 (`21728654`, merged by `9f5c0487`) |
| [Q-M1f.545](#q-m1f545--what-near-black-means-and-how-many-hues) | What *near black* means, and how many hues | **ruled 2026-11-19: the default** (the user, *"Go with all defaults to the end of this phase p1f.23 and stop"*): a contrast under 3.5 against black, held by a test; six hues of two shades, not eight. Built by [S1f.23.11](p1f.23_review_processing/s1f.23.11_drawings_second_answers.md), task 1 (`21728654`, merged by `9f5c0487`) |
| [Q-M1f.546](#q-m1f546--the-reasoning-pages-hand-drawings-of-a-derivation) | The reasoning pages' hand drawings of a derivation | **ruled 2026-11-19: the default** (the user, *"Go with all defaults to the end of this phase p1f.23 and stop"*): restyled by the same rule, a given fact a derivation also concludes plain. Built by [S1f.23.11](p1f.23_review_processing/s1f.23.11_drawings_second_answers.md), task 1 (`21728654`, merged by `9f5c0487`) |
| [Q-M1f.547](#q-m1f547--a-concentric-layout-the-layers-drawn-as-rings) | A concentric layout: the layers drawn as rings | **ruled 2026-11-19: the default** (the user, *"Go with all defaults to the end of this phase p1f.23 and stop"*): (a), no ring layout, nothing built; the review bundle's item 61, examples by [S1f.23.11](p1f.23_review_processing/s1f.23.11_drawings_second_answers.md), task 2, its page and programs in `git log` (`7c39c671`); not `--layout radial`, `--layout rings` or a documented recipe |
| [Q-M1f.548](#q-m1f548--a-derivations-three-levels-as-built) | A derivation's three levels, as built | **ruled 2026-11-19: the default** (the user, *"Go with all defaults to the end of this phase p1f.23 and stop"*): (a), as built, with Q-M1f.539 to .544; the review bundle's item 62; not one width, the premises as explanation, or a redundant firing's conclusion bright. Built by [S1f.23.11](p1f.23_review_processing/s1f.23.11_drawings_second_answers.md), task 1 (`21728654`, merged by `9f5c0487`) |

**The questions those two phases already own are M1e's and keep their ids.**
They are not copied here and they are not renumbered — the stage files cite
them across the milestone boundary, which is what a sticky id is for:

| id | what it is | who owns it here |
|---|---|---|
| [Q-M1e.5](../../docs/decisions/answer/Q-M1e.5-no-surface-disagrees-with-the-oracle.md) | is *experimental* a licence to ship a surface whose read-out is false? | closed in M1e2 (the user's (a), 2026-11-03: no). S1f.10.7, whose `--traversal` flag waited on it, was dropped at P1f.10's re-plan of 2026-11-15; its learning half is a source S1f.10.13 prices |
| [Q-M1e.6](../../docs/decisions/answer/Q-M1e.6-a-model-is-what-a-solutions-state-adds.md) | what is a **solution**, and what is a **model**? | decided 2026-08-28; P1f.10 is its first consumer |
| Q-M1e.9 | is `dead` really upward-closed under `absent`? | the **language** half was S1f.10.8's: ruled *forbid* on 2026-11-02 and built in M1e2; the stage is closed |
| [Q-M1e.10](../../docs/decisions/language/Q-M1e.10-two-inert-config-flags-are-refused.md) | two `(config …)` flags are **inert** | raised by the shipped `S1e.5.1`; owner unassigned |
| [Q-M1e.11](../../docs/decisions/hypotheses/Q-M1e.11-h-holds-what-a-debt-only-a-commitment-brings.md) | what happens to an obligation **derived under a hypothesis**? | **owned** by S1f.10.6, which since the re-plan of 2026-11-15 asks it over R, with register row W26 as its witness |

A question raised by an M1f stage from now on gets a `Q-M1f.<n>` here.

---

### Q-M1f.1 — G-14: is a bad conclusion that never fires refused?

**Decision file:** [`docs/decisions/language/Q-M1f.1-a-rule-that-could-derive-an-activator-binding.md`](../../docs/decisions/language/Q-M1f.1-a-rule-that-could-derive-an-activator-binding.md)

**Raised** 2026-11-08 by S1f.5.5's step 2
(paper grammar, Q1).
With `:assert` checked by the grammar, a conclusion no fact can hold (`(q ?x :k v)`, `(q _)`, `?x`) is a
parse error whether or not its rule fires. One shape is left that no grammar sees: an activator a rule
*derives* binding a relation parameter to a fact (`g14/derived-activator.ein`), which today stops the run
where its plan is compiled, and answers (64 states) where no node derives it
(`g14/derived-activator-never.ein`). The options were (a) refused at the parse, the derived shape a refusal
where compiled; (b) as (a), and the derived shape refused at load by a static check, which also refuses
`derived-activator-never.ein`; (c) checked only when a rule fires. The agent recommended (a).

**Ruled 2026-11-08: (b).** The user: *"Q1 - b"*. The grammar's half is S1f.5.5's; the load-time check is
S1f.5.18's, which stops and shows the
user any corpus program it would refuse.

**Built 2026-11-09** by S1f.5.18's
T1f.5.18.3: both `g14` programs are refused at load, one line each, and no corpus program that answers is (W59's two
rows).

### Q-M1f.2 — The 55 files the grammar refuses that today's parser accepts

**Raised** 2026-11-08 by S1f.5.5's step 2
(paper grammar, Q2),
re-taking the stage's 34. None is a puzzle, a stdlib module or a `tests/` program. The agent proposed (a):
the 26 refusal fixtures whose refusal the grammar takes over stay refusal fixtures and move to
`examples/broken/`, except `reserved_open_direct`, which three fixtures import by path;
`derivation_cycle` and five `examples/syntax/` fixtures are rewritten; 17 register witnesses and 6
regression fixtures are the owning stages' (S1f.5.1, S1f.5.2, S1f.5.4, S1f.5.11). The alternatives were (b)
the 26 left in place with only their `.expected` moved, and (c) the 26 rewritten.

**Ruled 2026-11-08: (a).** The user: *"Q2 - a"*. Since the user's re-plan the same day, the 23 the owning stages
had land in S1f.5.5's swap, each credited to its stage's ruling, and of the rest what moves no answer lands just
before it (S1f.5.5 § The order).

### Q-M1f.3 — Three wrong answers the census found with no row

**Raised** 2026-11-08 by S1f.5.5's step 1
(census § Findings outside the grammar):
an activator binding a value parameter to a nested fact leaves it unbound; `:assert ()` stores a fact
named `@empty`, which joins H; and `(absent ())` holds. The agent recommended filing all three now as
`open` rows, W33–W35, as a fourth task of S1f.5.0: W33 owned by a new stage, S1f.5.18, where the parameter
binds as any value does; W34 by S1f.5.9, whose
case table it corrects; W35 by S1f.5.5, whose grammar refuses `(absent ())`.

**Ruled 2026-11-08, as recommended.** The user: *"Q3 - as recommended"*.

W33 **fixed 2026-11-09** by S1f.5.18's T1f.5.18.2: an activator binds a parameter to a nested fact as to a name. W34
the same day by S1f.5.9's T1f.5.9.2, `:assert ()` asserting
nothing, and W35 by S1f.5.5's swap (`8c3fe86e`), `(absent ())` refused at the parse.

### Q-M1f.4 — Removing `:priority` moves the facts of cores on seven corpus programs

**Raised** 2026-11-08 by S1f.5.15's T1
(§ T1 record), which stopped on it
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

**Decision file:** [`docs/decisions/answer/Q-M1f.6-a-core-is-an-explanation.md`](../../docs/decisions/answer/Q-M1f.6-a-core-is-an-explanation.md)

**Raised** 2026-11-08 by S1f.5.15's T3
(§ T3 record), on Q-M1f.4 (b)'s
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

**Decision file:** [`docs/decisions/language/Q-M1f.7-the-dunder-is-never-bound-and-the-writeback.md`](../../docs/decisions/language/Q-M1f.7-the-dunder-is-never-bound-and-the-writeback.md)

**Raised** 2026-11-08 by S1f.5.5's T11
(§ T11 record), before any edit. The user
ruled at the T9 review stop that the engine's five names *"are also reserved names"*, and the coordinator read it,
for the user to correct: the three writeback names `<forced-positive>`, `<monotonic-unconditional>` and
`<lookahead-dies-immediately>` are refused wherever a program writes them, and the dunders `__closed__` and
`__symmetric__` keep the positions the corpus writes them in, the relation of a fact, of a pattern and of an
`:assert`, and are refused as a declared relation, a rule's name, or **an argument value**. The writeback half
meets nothing. The dunder half meets two things the reading did not see:

- **The stdlib documents a dunder as an argument value, and a test holds it.** `stdlib/algebra.ein`'s `imply1`,
  *"Doubles as property→property: (imply1 functional __closed__) turns a functional relation closed"*;
  [`04-stdlib/`](../../docs/ein/user/04-reference/04-stdlib/README.md)'s `imply1` row, *e.g. `(imply1
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

**Decision file:** [`docs/decisions/engine/Q-M1f.8-the-lexer-is-lrlexs-unpatched.md`](../../docs/decisions/engine/Q-M1f.8-the-lexer-is-lrlexs-unpatched.md)

**Raised** 2026-11-09 by S1f.5.5's T12
(§ T12 record), which recovered what it
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
are parked for the phase's close with the numbers above (§ T12 record, Parked for the phase
close).

### Q-M1f.9 — What a deleted loader line leaves behind

**Decision file:** [`docs/decisions/engine/Q-M1f.9-a-shape-the-grammar-refuses-is-a-loader.md`](../../docs/decisions/engine/Q-M1f.9-a-shape-the-grammar-refuses-is-a-loader.md)

**Raised** 2026-11-09 by S1f.5.5's T5
(§ T5 record), which deleted the 55 refusal
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

**Decision file:** [`docs/decisions/engine/Q-M1f.10-a-einb-file-holds-no-integer-table.md`](../../docs/decisions/engine/Q-M1f.10-a-einb-file-holds-no-integer-table.md)

**Raised** 2026-11-09 by S1f.5.4's T1f.5.4.3
(§ T1f.5.4.3 record), which retired
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

**Re-ruled 2026-11-13: (b), at format 2.0.** The per-commit CI went red on `einb_cli`'s
`a_container_saved_again_is_the_container` (runs 43396694686 and 43433241494): `META`'s `created_unix`, the second a
file was saved in, under the header's digest, made two saves of one program differ wherever they crossed a second.
On the user's *"remove"* the field went, a layout change and so a major bump, 1.2 to 2.0; the coordinator then put
this question again, since the bump (a) avoided was being made anyway, and the user said *"yes, do it"*. The `INTS`
section is not written, not read and not required; its kind number, 3, is retired and not reused, so a section of
kind 3 is an unknown one and skipped; a 1.x file is refused with the version line, and a value tagged `0b01` still
with `EinbError::Int`'s sentence. `a_file_that_holds_an_integer_is_refused` holds the three: no kind 3 in what
the writer writes, a 1.2 header refused, a tagged value refused.

### Q-M1f.11 — A template reference to a variable no reference can name

**Decision file:** [`docs/decisions/language/Q-M1f.11-a-template-may-not-refer-to-a-variable-whose.md`](../../docs/decisions/language/Q-M1f.11-a-template-may-not-refer-to-a-variable-whose.md)

**Raised** 2026-11-08 by S1f.5.5's T9 review (§ 11) and carried
by S1f.5.4's T1f.5.4.5, the fix of W37
(§ T1f.5.4.5 record). A template
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

**Decision file:** [`docs/decisions/read-out/Q-M1f.12-natural-language-text-reaches-a-fixed-set.md`](../../docs/decisions/read-out/Q-M1f.12-natural-language-text-reaches-a-fixed-set.md)

**Raised** 2026-11-09 by S1f.5.2's T1f.5.2.2,
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
`ein-cli/tests/natural_language_text.rs` holds it. Re-opening it means (b), surface by surface; S1f.5.14,
which makes `:why` required, is where a reader of the text is most likely to ask for one.

### Q-M1f.13 — F3's cause reaches a disjunction: whose row is it?

**Raised** 2026-11-09 by S1f.5.1's T1f.5.1.3, step 1
(its record), which traced
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

**Decision file:** [`docs/decisions/engine/Q-M1f.14-a-keyword-pair-is-only-ever-a-forms-clause.md`](../../docs/decisions/engine/Q-M1f.14-a-keyword-pair-is-only-ever-a-forms-clause.md)

**Raised** 2026-11-09 by S1f.5.11's T1f.5.11.2
(its record). Since S1f.5.5's swap
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

**Decision file:** [`docs/decisions/diagnostics/Q-M1f.15-a-position-in-a-container-names-its-stored.md`](../../docs/decisions/diagnostics/Q-M1f.15-a-position-in-a-container-names-its-stored.md)

**Raised** 2026-11-09 by S1f.5.5's T1f.5.5.13, the coordinator's
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

**Raised** 2026-11-09 by S1f.5.5's T1f.5.5.13. The brief files the
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

**Decision file:** [`docs/decisions/cli/Q-M1f.17-a-view-of-a-container-draws-the-program-it.md`](../../docs/decisions/cli/Q-M1f.17-a-view-of-a-container-draws-the-program-it.md)

**Raised** 2026-11-09 by S1f.5.5's T1f.5.5.13. `ein render rules`,
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

**Decision file:** [`docs/decisions/read-out/Q-M1f.18-the-read-outs-model-is-the-claim-form-and-p.md`](../../docs/decisions/read-out/Q-M1f.18-the-read-outs-model-is-the-claim-form-and-p.md)

**Raised** 2026-11-09 by S1f.5.16's T1f.5.16.1, as its Q1: M1e's
[Q-M1e.7](../../docs/decisions/answer/Q-M1e.7-a-state-is-what-the-engine-judges-and-a-model-is.md)
ruled that a state's model, [`06-solution-semantics.md` § 3](../../docs/ein/reasoning/02-kb-evolution/06-solution-semantics.md#3-model)'s
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

**Decision file:** [`docs/decisions/read-out/Q-M1f.19-a-texts-line-break-on-the-read-out-continues-at.md`](../../docs/decisions/read-out/Q-M1f.19-a-texts-line-break-on-the-read-out-continues-at.md)

**Raised** 2026-11-09 by S1f.5.16's T1f.5.16.1, from an item S1f.5.2 handed it. A
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

**Decision file:** [`docs/decisions/read-out/Q-M1f.20-on-route-2-describe-says-no-licence-and-a-mode.md`](../../docs/decisions/read-out/Q-M1f.20-on-route-2-describe-says-no-licence-and-a-mode.md)

**Raised** 2026-11-09 by S1f.5.16's T1f.5.16.1, its census. On route 2, describe's and
`ein oracle`'s `route` row says *no licence* (`2, the cube — no licence: the static check flags the program`, `…:
asked for by name`), where `--json-summary`'s `region.licences` is `["L7 the cube"]` and a mode's `route` row on the
same route prints `2, the cube — L7 the cube` (313 runs of 3 221, `build/s1f.5.16/census.py`, 2026-11-09).
`solve_interface.md`'s licence table lists L7 as
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

**Decision file:** [`docs/decisions/read-out/Q-M1f.21-region-stopped-layer-is-each-routes-own-unit.md`](../../docs/decisions/read-out/Q-M1f.21-region-stopped-layer-is-each-routes-own-unit.md)

**Raised** 2026-11-09 by S1f.5.16's T1f.5.16.1, its census. When a budget stops a
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

**Raised** 2026-11-09 by S1f.5.16's T1f.5.16.4, from T3's test, whose row parser had to
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

**Fixed 2026-11-09, (b)**, by S1f.5.6,
to which the coordinator handed it the same day: T1f.5.6.5 filed it as W42 under the register's schema `/17` (a line a
run prints, read off stdout or `answer.txt`), with a second row for `-H`'s form, which glues a relation's name of
sixteen characters or more to its count the same way; T1f.5.6.6 put one space at least between a label and its value in
both printers, at every width, and flipped both rows (`31e9c7f7`). No golden held either line, so none moved.

### Q-M1f.23 — What the `route` row names where root decides a flagged program's minimal line

**Decision file:** [`docs/decisions/read-out/Q-M1f.23-where-root-decides-a-flagged-programs-minimal.md`](../../docs/decisions/read-out/Q-M1f.23-where-root-decides-a-flagged-programs-minimal.md)

**Raised** 2026-11-09 by S1f.5.6's
T1f.5.6.2. On a program the static check flags, a minimal question where ∅ is in R is decided by root's node, one
saturation (the user's ruling of 2026-11-07 on S-16, (b)). The rule is the minimal line's definition, ∅ being below
every commitment, and no theorem about the program, so it fits none of
`solve_interface.md` § 3.1's licences, which each
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
row on the live page says why the line is decided ([`08-read-out.md`](../../docs/ein/user/04-reference/08-read-out.md#3-a-mode---minimal---maximal--n-n---single) § 3).
Re-opening it means (b), which is [Q-M1f.20](#q-m1f20--route-2s-licence-no-licence-on-one-row-l7-the-cube-on-another)'s
neighbour: both ask what a route row's licence list is for.

### Q-M1f.24 — A second surface that glues: another row of W42, or a case of its own?

**Raised** 2026-11-09 by S1f.5.6's
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

**Decision file:** [`docs/decisions/answer/Q-M1f.25-every-core-a-run-prints-is-taken-against-its-one.md`](../../docs/decisions/answer/Q-M1f.25-every-core-a-run-prints-is-taken-against-its-one.md)

**Raised** 2026-11-09 by S1f.5.12's T1f.5.12.1. The user's
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

**Decision file:** [`docs/decisions/read-out/Q-M1f.26-every-member-of-n-carries-its-core.md`](../../docs/decisions/read-out/Q-M1f.26-every-member-of-n-carries-its-core.md)

**Raised** 2026-11-09 by S1f.5.12's T1f.5.12.1. With every
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

**Decision file:** [`docs/decisions/cli/Q-M1f.27-a-containers-answer-directory-saves-the-program.md`](../../docs/decisions/cli/Q-M1f.27-a-containers-answer-directory-saves-the-program.md)

**Raised** 2026-11-09 by S1f.5.12's T1f.5.12.5, which fixes
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

**Raised** 2026-11-09 by S1f.5.8's T1f.5.8.1, on the
coordinator's brief: a wrong answer, or a printer line that says something false about one, is a register row before
it is fixed. Under the ruling the stage builds
([Q-M1e2.27](../../docs/decisions/read-out/Q-M1e2.27-in-eins-json-is-the-empty-set-and-null-what-does.md)), `[]` is the empty
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

**Decision file:** [`docs/decisions/read-out/Q-M1f.29-a-question-that-prints-no-core-writes-a-dead.md`](../../docs/decisions/read-out/Q-M1f.29-a-question-that-prints-no-core-writes-a-dead.md)

**Raised** 2026-11-09 by S1f.5.8's T1f.5.8.1. On a
program whose root is dead, `ein solve FILE --count` writes `region.root: "dead"` beside `region.core: null` and
`region.refuted_by: null`: 39 programs of the stage's sweep (2026-11-09). A bare `ein solve` writes root's core and
rules there, and `--count`, which reads no state, prints neither on stdout; a mode writes `region.root: null` and no
core either. Root is dead, so its core exists.

- **(a) `null`, documented**: the summary is stdout's facts, and `null` is where a key does not apply; the ruling reads
  `null` as *an object that does not exist or is not known*, and a count does not state root's core.
  [`08-read-out.md` § 11](../../docs/ein/user/04-reference/08-read-out.md#11---json-summary-the-answer-blocks) says so, from
  T1f.5.8.2 on.
- **(b) Root's core under every question** whose summary says root is dead: `--count`'s summary, and its stdout's
  `unsat core` and `refuted by` rows, where a count prints no listing today.

**Defaulted 2026-11-09 (the user's instruction): (a).** It moves nothing, and a count's question is *how many*.
Re-opening it means (b), which moves `--count`'s stdout on those 39 programs.

### Q-M1f.30 — A form that holds a variable, as a guard's argument

**Decision file:** [`docs/decisions/language/Q-M1f.30-a-guard-argument-holds-no-variable-inside-a-fact.md`](../../docs/decisions/language/Q-M1f.30-a-guard-argument-holds-no-variable-inside-a-fact.md)

**Raised** 2026-11-09 by S1f.5.7's T1f.5.7.1. The user
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

**Decision file:** [`docs/decisions/language/Q-M1f.31-not-eq-x-y-is-neq-x-y-in-every-place.md`](../../docs/decisions/language/Q-M1f.31-not-eq-x-y-is-neq-x-y-in-every-place.md)

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

**Decision file:** [`docs/decisions/hypotheses/Q-M1f.32-an-hrules-eq-or-neq-instance-is-no-hypothesis.md`](../../docs/decisions/hypotheses/Q-M1f.32-an-hrules-eq-or-neq-instance-is-no-hypothesis.md)

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

**Decision file:** [`docs/decisions/language/Q-M1f.33-a-written-fact-of-eq-is-a-conclusion-with-no.md`](../../docs/decisions/language/Q-M1f.33-a-written-fact-of-eq-is-a-conclusion-with-no.md)

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

**Decision file:** [`docs/decisions/language/Q-M1f.34-a-guard-that-binds-its-variable-binds-it-where.md`](../../docs/decisions/language/Q-M1f.34-a-guard-that-binds-its-variable-binds-it-where.md)

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

**Raised** 2026-11-09 by S1f.5.7's T1f.5.7.5,
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

**Raised** 2026-11-09 by S1f.5.10's
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
S1f.5.18's, the next stage on lane 2. The stage
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

**Decision file:** [`docs/decisions/diagnostics/Q-M1f.38-a-refusals-line-names-the-fault-then-what.md`](../../docs/decisions/diagnostics/Q-M1f.38-a-refusals-line-names-the-fault-then-what.md)

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

**Decision file:** [`docs/decisions/language/Q-M1f.41-a-position-holds-a-fact-unless-the-program.md`](../../docs/decisions/language/Q-M1f.41-a-position-holds-a-fact-unless-the-program.md)

**Raised** 2026-11-09 by S1f.5.18's T1f.5.18.1. The user's ruling on
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

**Decision file:** [`docs/decisions/language/Q-M1f.42-a-conclusion-whose-relation-is-a-parameter.md`](../../docs/decisions/language/Q-M1f.42-a-conclusion-whose-relation-is-a-parameter.md)

**Raised** 2026-11-09 by S1f.5.18's T1f.5.18.1. The check of Q-M1f.41 reads a conclusion's relation as written. `(?T ?x)`
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

**Decision file:** [`docs/decisions/engine/Q-M1f.43-a-plans-activator-is-every-argument.md`](../../docs/decisions/engine/Q-M1f.43-a-plans-activator-is-every-argument.md)

**Raised** 2026-11-09 by S1f.5.18's T1f.5.18.1. With W33's fix a value parameter binds to a nested fact, so `(note edge (src
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

**Raised** 2026-11-09 by S1f.5.18's T1f.5.18.1. Q-M1f.1 (b)'s check refuses at load two ways the engine answers today: a
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

**Decision file:** [`docs/decisions/diagnostics/Q-M1f.45-the-parse-error-at-and-or-or-carries-a-note.md`](../../docs/decisions/diagnostics/Q-M1f.45-the-parse-error-at-and-or-or-carries-a-note.md)

**Raised** 2026-11-09 by S1f.5.9's T1f.5.9.1. The user ruled on F12 that `(and)` stays refused, `and` taking
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

**Decision file:** [`docs/decisions/language/Q-M1f.46-a-relation-parameter-that-can-hold-eq-heads-only.md`](../../docs/decisions/language/Q-M1f.46-a-relation-parameter-that-can-hold-eq-heads-only.md)

**Raised** 2026-11-09 by S1f.5.9's T1f.5.9.1, from the shape S1f.5.18 found and left unfiled: `(rule r (?R)
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

**Decision file:** [`docs/decisions/language/Q-M1f.47-a-written-fact-of-eq-at-another-arity-than-two.md`](../../docs/decisions/language/Q-M1f.47-a-written-fact-of-eq-at-another-arity-than-two.md)

**Raised** 2026-11-09 by S1f.5.9's T1f.5.9.1, beside Q-M1f.46. A top-level fact may be headed `eq` (`ein.y`'s
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

**Decision file:** [`docs/decisions/read-out/Q-M1f.48-a-text-surface-writes-the-empty-core.md`](../../docs/decisions/read-out/Q-M1f.48-a-text-surface-writes-the-empty-core.md)

**Raised** 2026-11-09 by S1f.5.9's T1f.5.9.2, on the
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

**Raised** 2026-11-09 by S1f.5.13's
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

**Raised** 2026-11-09 by S1f.5.13's
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

**Taken by S1f.5.27 on 2026-11-10**, when the user had accepted
(a) as parked and the close review's finding 8 re-opened the nightly: (b), by tier, under the stage's rule
([Q-M1f.102](#q-m1f102--what-the-ci-nightly-keeps-and-what-the-dev-host-runs)). `region`'s and `obligation_rung`'s slow
halves are CI nightly steps, each a second route held to an answer on the `zebra2-minus-15` family that no per-commit
test reaches; `route_invariance`'s is a dev-host step, over 5 min, its programs' answers digested per commit by
`corpus_shapes`. The guard (b) asked for is `nightly_steps.rs`' `every_slow_half_runs_on_a_tier`: a target that reads
`EIN_CORPUS_SLOW` and that neither the nightly nor `--dev-host` runs with it fails, but `read_out_examples`, whose slow
branch reads only the page's blocks at `--region-budget 33554432`, and the page has none, which the test checks. Each
slow half's time is in the stage's records.

### Q-M1f.52 — The nightly's review bundle and the register's runs that are not a solve

**Raised** 2026-11-09 by S1f.5.13's
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
as a stage of its own, S1f.5.19,
after S1f.5.13 and the merge of the second lane (T1f.5.M, `a90c452b`) and before S1f.5.14: the index guarded, and a
register case whose run is not a solve given a case page of its own, which shows the command as run, its exit, its
stdout and stderr in full, the register's `wrong`, `now` and `right` beside it, and every artefact the run writes,
linked and drawn where the engine draws one. The merge brought two more rows the index cannot read,
`W55/goal-neq-free` and `W56/goal-absent`, whose `[case.wrong]` is `ein saturate`'s refusal beside a solve's `now`.

**Built 2026-11-09** by S1f.5.19's T1f.5.19.2 (`00c4ed34`,
its record):
`python3 utils/review_bundle.py` builds all 175 cases, 0 failed, the seven command runs on `utils/review/command.py`'s
page, and the nightly on that commit is green in every job and step, 81 min 54 s
(its T3 record).

### Q-M1f.53 — How the per-commit gate catches a register row the bundle cannot read

**Raised** 2026-11-09 by S1f.5.19,
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

**Raised** 2026-11-09 by S1f.5.19.
The coordinator's brief allows `build/review/`, which is gitignored, to be cleared and rebuilt, so that the four dead
links S1f.5.13 found there go. Those four are in pages the
no-owner inventory wrote there on 2026-11-06 for F1's two
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
The inventory is a record and is not edited; P1f.5's README names the new place
where it names the evidence. The two scripts name `build/review` in their defaults, so re-running one writes there
again.

### Q-M1f.55 — A `container` on a solve's `now`

**Raised** 2026-11-09 by S1f.5.19.
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

**Raised** 2026-11-10 by S1f.5.14's T1f.5.14.1. The stage file
names `--json-summary`'s `verdict.solutions[i].facts`, which M1e2's switch removed, and three registers (puzzle,
stdlib-derived, kernel), where its acceptance asks for "the four registers".

- **(a) The whole state**: every fact of every state file `ein solve FILE --dump-answer DIR` writes, root's facts
  included, the set [Q-M1e.7](../../docs/decisions/answer/Q-M1e.7-a-state-is-what-the-engine-judges-and-a-model-is.md)
  is about; and four registers, the stage's three and **undeclared** (a relation written or derived and never
  declared, and no rule's), the small set T1f.5.14.3 says the census prints.
- **(b) The listed projection**: `region.states[i].facts`, the facts over H's relations the read-out prints (25 on
  `zebra2`), every one of them a puzzle relation's.

**Defaulted 2026-11-10 (the user's instruction of 2026-11-09): (a).** It is the set the stage's 444-fact number was
about, and (b) has no register to split. `utils/nl_census.py`'s docstring states it.

### Q-M1f.57 — Which fixtures get a `:why`

**Decision file:** [`docs/decisions/corpus/Q-M1f.57-every-fixture-the-loader-reads-carries-a-why.md`](../../docs/decisions/corpus/Q-M1f.57-every-fixture-the-loader-reads-carries-a-why.md)

**Raised** 2026-11-10 by S1f.5.14's T1f.5.14.7.

- **(a) Every file the loader reads**, a fixture refused at load among them, each `:why` on its form's last line so no
  line or column a refusal prints moves; a fixture refused at the parse is left as it is, since the parse refusal
  stops before the loader reads a `:why`; and the `.ein` files under `plans/` and `docs/history/`, records no test
  loads, are left too.
- **(b) Every `.ein` file**, those refused at the parse and the records included.

**Defaulted 2026-11-10 (the user's instruction of 2026-11-09): (a).** A `:why` in a file refused at the parse is
read by nothing and moves the positions its refusal prints; a record is not edited.

### Q-M1f.58 — Where a fact is rendered in English, and what it reads from

**Decision file:** [`docs/decisions/read-out/Q-M1f.58-a-fact-reads-in-english-in-the-trace.md`](../../docs/decisions/read-out/Q-M1f.58-a-fact-reads-in-english-in-the-trace.md)

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

**Rewritten 2026-11-18** by S1f.23.5 ([its Record](p1f.23_review_processing/s1f.23.5_a_why_reads_its_own_form.md#record);
the user: *"Declaration only"*, *"{?n}, renumbered"*), built by `bd21f0bb`, merged by `79dd213b`: a `:why` reads
the form it is written on, a declaration's `{?n}` its n-th arrow, `{?1}` the atom `relation`, `{?2}` the
relation's name, `{?k+2}` its k-th label; a fact of a relation takes no text from the declaration and carries
none of its own, reading by its `:source`, its rule's `:why`, or as written; a saved state holds no fact to
render (Q-M1f.392). The decision file keeps its slug, its title extended
([Q-M1f.480](#q-m1f480--the-decisions-the-ruling-rewrites)). The writer of (a)'s state-file comments went with
L-102 ([Q-M1f.478](#q-m1f478--l-102s-reach)).

### Q-M1f.59 — What a macro's `:why` renders

**Decision file:** [`docs/decisions/read-out/Q-M1f.59-a-macros-why-documents-the-pattern-and-is.md`](../../docs/decisions/read-out/Q-M1f.59-a-macros-why-documents-the-pattern-and-is.md)

**Raised** 2026-11-10 by S1f.5.14's T1f.5.14.3. S1f.5.5's T3 gave a macro an optional `:why`, and since the swap
`--require-why` asks for it. A macro is expanded at load, and no fact's justification names one.

- **(a) Nothing**: it documents the pattern for a reader of the program, and a state file re-writes it with the program.
- **(b) The rule's reason**: fill the macro's `:why` into the reason of a fact whose rule's `:match` used it.

**Defaulted 2026-11-10 (the user's instruction of 2026-11-09): (a).** The rule that used the macro has a `:why` of its
own that says what the pattern meant there; (b) is a second reason for one fact.

### Q-M1f.60 — A relation nobody declares

**Decision file:** [`docs/decisions/read-out/Q-M1f.60-a-fact-of-a-relation-nobody-declares-has-no.md`](../../docs/decisions/read-out/Q-M1f.60-a-fact-of-a-relation-nobody-declares-has-no.md)

**Raised** 2026-11-10 by S1f.5.14's T1f.5.14.3: an auto-vivified relation has no form to carry a `:why`.

- **(a) No slot**: a fact of one that a rule derived reads by its reason; one the program states falls back to its
  s-expression, and the census counts it under `stdlib` or `undeclared`.
- **(b) A declaration-less template**, a new form that templates a relation without declaring it.

**Defaulted 2026-11-10 (the user's instruction of 2026-11-09): (a).** (b) is a language change; a program that wants
the fact read declares the relation.

### Q-M1f.61 — A template per goal conjunct

**Decision file:** [`docs/decisions/read-out/Q-M1f.61-a-query-has-one-goal-text-and-each-conjunct.md`](../../docs/decisions/read-out/Q-M1f.61-a-query-has-one-goal-text-and-each-conjunct.md)

**Raised** 2026-11-10 by S1f.5.14's T1f.5.14.3: `:goal-text` is one string for the whole query.

- **(a) No change**: each conjunct's fact reads by its relation's `:why` where a surface prints it.
- **(b) A per-conjunct template**, a language change.

**Defaulted 2026-11-10 (the user's instruction of 2026-11-09): (a)**, as the stage file says a yes would be a
question and not a quiet addition.

### Q-M1f.62 — The option that loads a program without a `:why`, once the default is true

**Decision file:** [`docs/decisions/cli/Q-M1f.62-no-require-why-loads-a-program-without-a-why.md`](../../docs/decisions/cli/Q-M1f.62-no-require-why-loads-a-program-without-a-why.md)

**Raised** 2026-11-10 by S1f.5.14's T1f.5.14.7. The stage file asks that a program without `:why` still load once the
default is true.

- **(a) `--no-require-why`** on the seven commands, a set-true flag as `--no-types` and `--no-instances` are, replacing
  `--require-why`; `LoadOptions::default()` requires.
- **(b) `--require-why=false`**, a valued option.
- **(c) Both `--require-why` and `--no-require-why`.**

**Defaulted 2026-11-10 (the user's instruction of 2026-11-09): (a).** Every boolean option the CLI has is a set-true
flag, and a negative one is spelled `--no-…`; `--require-why` would be inert with the default true, and an inert
option is what [Q-M1e.10](../../docs/decisions/language/Q-M1e.10-two-inert-config-flags-are-refused.md)
retired. The option count stays 78.

### Q-M1f.63 — What each program generator does about `:why`

**Decision file:** [`docs/decisions/corpus/Q-M1f.63-a-program-a-test-writes-carries-a-why-on-every.md`](../../docs/decisions/corpus/Q-M1f.63-a-program-a-test-writes-carries-a-why-on-every.md)

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

**Decision file:** [`docs/decisions/language/Q-M1f.65-a-comment-may-hold-any-character.md`](../../docs/decisions/language/Q-M1f.65-a-comment-may-hold-any-character.md)

**Raised** 2026-11-10 by S1f.5.22's census, on the
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

**Decision file:** [`docs/decisions/language/Q-M1f.66-open-is-written-only-as-a-rules-verdict.md`](../../docs/decisions/language/Q-M1f.66-open-is-written-only-as-a-rules-verdict.md)

**Raised** 2026-11-10 by S1f.5.23's T1, on the user's ruling
of that day: *"`open` is reserved word, here it appers as what? as relation name? - then it is an error, if not -
then explain"*. As a relation name (a fact's head, a nested fact's head, a pattern's head, a relation parameter's
binding) it is refused by the ruling. As a value, `(p open)`, it is a name `p` holds, which loads today, as the
other position words do. No corpus program writes it there.

- **(a) Refuse it**: `open` is written only as a rule's `(open)` or `(open ?R)` conclusion.
- **(b) Keep it as a value**, a position word like `model`; refused only as a relation name.

**Ruled 2026-11-10, (a)** (the user chose *"Refuse it"*). `open` is reserved in the plain sense.

### Q-M1f.67 — Where is a macro body that no rule could use refused?

**Decision file:** [`docs/decisions/language/Q-M1f.67-a-macros-body-is-checked-where-it-is-called.md`](../../docs/decisions/language/Q-M1f.67-a-macros-body-is-checked-where-it-is-called.md)

**Raised** 2026-11-10 by S1f.5.24's T1.
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

**Decision file:** [`docs/decisions/cli/Q-M1f.68-a-closed-stdout-ends-ein-by-sigpipe.md`](../../docs/decisions/cli/Q-M1f.68-a-closed-stdout-ends-ein-by-sigpipe.md)

**Raised** 2026-11-10 by S1f.5.25's T1. Rust ignores
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

**Decision file:** [`docs/decisions/docs/Q-M1f.69-a-guide-block-that-writes-an-ellipsis-is.md`](../../docs/decisions/docs/Q-M1f.69-a-guide-block-that-writes-an-ellipsis-is.md)

**Raised** 2026-11-10 by S1f.5.26's T2. The stage says a
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

**Raised** 2026-11-10 by S1f.5.26's T1. Besides the three
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

**Decision file:** [`docs/decisions/cli/Q-M1f.71-a-view-refuses-a-program-the-loader-refuses.md`](../../docs/decisions/cli/Q-M1f.71-a-view-refuses-a-program-the-loader-refuses.md)

**Raised** 2026-11-10 by S1f.5.21's T1. `ein render program`, `render rules`, `render rule` and `render constraints` parse a
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

**Decision file:** [`docs/decisions/diagnostics/Q-M1f.72-a-nested-or-a-wide-match-and-a-goals-relation.md`](../../docs/decisions/diagnostics/Q-M1f.72-a-nested-or-a-wide-match-and-a-goals-relation.md)

**Raised** 2026-11-10 by S1f.5.21's T1. The stage file sketches the line as `<file>:<line>:<col>: rule '<name>': <what>`,
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

**Decision file:** [`docs/decisions/engine/Q-M1f.73-a-rule-binds-at-most-256-variables-counted.md`](../../docs/decisions/engine/Q-M1f.73-a-rule-binds-at-most-256-variables-counted.md)

**Raised** 2026-11-10 by S1f.5.21's T1. The compiler numbers a plan's variables into a register file of 256 (`MAX_REGS`):
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

**Decision file:** [`docs/decisions/language/Q-M1f.74-a-nested-or-a-macros-expansion-writes-is-refused.md`](../../docs/decisions/language/Q-M1f.74-a-nested-or-a-macros-expansion-writes-is-refused.md)

**Raised** 2026-11-10 by S1f.5.21's T1. `(forall ?x (or (p ?x) (q ?x)) (a ?x))` and a program's `(macro either (?x) (or
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

**Decision file:** [`docs/decisions/engine/Q-M1f.75-the-compilers-errors-are-a-library-callers.md`](../../docs/decisions/engine/Q-M1f.75-the-compilers-errors-are-a-library-callers.md)

**Raised** 2026-11-10 by S1f.5.21's T1. Three of the compiler's refusals are reached by no program and by a direct caller
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

**Raised** 2026-11-10 by S1f.5.21's T1. The move replaces, for each refusal, a stop on every command that compiles, a
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

**Raised** 2026-11-10 by S1f.5.29. Twice a Graphviz refusal reached the per-commit CI behind
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

**Raised** 2026-11-10 by S1f.5.29's T1. The runner's Graphviz 14.1.2 refuses every KB view the review bundle
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

**Raised** 2026-11-10 by S1f.5.29's T1. Since `00c4ed34` the smoke fails on a DOT with no SVG beside it, but
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

**Decision file:** [`docs/decisions/language/Q-M1f.80-a-malformed-primitive-a-macros-expansion-writes.md`](../../docs/decisions/language/Q-M1f.80-a-malformed-primitive-a-macros-expansion-writes.md)

**Raised** 2026-11-10 by S1f.5.24's T2. The user's ruling on Q-M1f.67 allows the refusal *"during either compilation of
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

**Decision file:** [`docs/decisions/language/Q-M1f.81-a-malformed-primitive-is-a-form-the-grammar.md`](../../docs/decisions/language/Q-M1f.81-a-malformed-primitive-is-a-form-the-grammar.md)

**Raised** 2026-11-10 by S1f.5.24's T2. The ruling names `(eq <arg>)`; the stage file names a literal head, a head
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

**Decision file:** [`docs/decisions/diagnostics/Q-M1f.82-a-malformed-primitive-a-macro-writes-is-refused.md`](../../docs/decisions/diagnostics/Q-M1f.82-a-malformed-primitive-a-macro-writes-is-refused.md)

**Raised** 2026-11-10 by S1f.5.24's T3. S1f.5.21 gave the compiler's line a position, the macro body's, since the form it
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

**Raised** 2026-11-10 by S1f.5.24's T2: eighteen shapes at four or five sites, eighty programs, and for each some or every
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

**Decision file:** [`docs/decisions/engine/Q-M1f.84-a-compiler-arm-the-loader-covers-is-an-invariant.md`](../../docs/decisions/engine/Q-M1f.84-a-compiler-arm-the-loader-covers-is-an-invariant.md)

**Raised** 2026-11-10 by S1f.5.24's T3. Once the loader refuses each shape, three lines no program reaches remain: the
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

**Decision file:** [`docs/decisions/language/Q-M1f.86-a-macros-body-never-writes-open.md`](../../docs/decisions/language/Q-M1f.86-a-macros-body-never-writes-open.md)

**Raised** 2026-11-10 by S1f.5.23's T2, on the user's
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

**Decision file:** [`docs/decisions/diagnostics/Q-M1f.87-the-loaders-refusals-of-the-verdict-start-where.md`](../../docs/decisions/diagnostics/Q-M1f.87-the-loaders-refusals-of-the-verdict-start-where.md)

**Raised** 2026-11-10 by S1f.5.23's T3. With `open` no
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

**Decision file:** [`docs/decisions/diagnostics/Q-M1f.88-a-misplaced-open-is-a-parse-error-with-a-note.md`](../../docs/decisions/diagnostics/Q-M1f.88-a-misplaced-open-is-a-parse-error-with-a-note.md)

**Raised** 2026-11-10 by S1f.5.23's T3. The stage file
sketches one line, *``<file>:<line>:<col>: `open` is the verdict a rule's :assert states, and is not a relation: …``*.
Where the grammar refuses, the line is a parse error's ([`07-diagnostics.md`
§ 1](../../docs/ein/user/04-reference/07-diagnostics.md#1-parse-and-load-diagnostics)): ``<file>:<line>:<col>: unexpected `open` ``,
then the error's notes and repairs.

- **(a) The parse error's line, and the stage's sentence as a note under it**, as S1f.5.1's `_` and S1f.5.2's string
  carry one: *`note: open is reserved: it is the verdict a rule's :assert states, written as the whole :assert, (open)
  or (open ?R), and nowhere else, so it names no relation and no value`* (`ein_ir::grammar::OPEN_NOTE`).
- **(b) A first line of its own**: the parse error's `what` replaced by the sentence wherever the token found is `open`.

**Defaulted 2026-11-10 (the stage's default): (a).** One spelling for a parse error, the precedent of `_` and a
string; the first line names the token and its place, and the note says what it is. Re-opening it means (b).

### Q-M1f.89 — Which of what the refusal replaces is a register row

**Raised** 2026-11-10 by S1f.5.23's T2: fifty-two places a
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

**Raised** 2026-11-10 by S1f.5.22's T2. An imported module that exists and cannot be read as text, four ways, a module that
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

**Decision file:** [`docs/decisions/imports/Q-M1f.91-an-imported-module-is-ein-text-and-a-einb.md`](../../docs/decisions/imports/Q-M1f.91-an-imported-module-is-ein-text-and-a-einb.md)

**Raised** 2026-11-10 by S1f.5.22's T3. Once a module is read through the program's read, a container is told from text by
its first bytes; the program a command is given may be one, and is opened. `ein-ir`, where the resolver is, cannot open
one: `ein-einb` stands above it.

- **(a) Refused, with a line of its own**: *`<path>: a .einb container, and an imported module is Ein text: import the
  .ein it was saved from`* (`ein_ir::read::container_module_line`).
- **(b) Opened as the program it stores**, the resolver taking a callback from `ein-cli` that opens a container.

**Defaulted 2026-11-10 (the stage's default): (a).** AGENTS.md: *`.einb` is a private cache format, never an interchange
one*; a module is a program's text. Re-opening it means (b).

### Q-M1f.92 — How the line quotes a character a reader cannot see

**Decision file:** [`docs/decisions/diagnostics/Q-M1f.92-a-character-a-reader-cannot-see-is-quoted-as.md`](../../docs/decisions/diagnostics/Q-M1f.92-a-character-a-reader-cannot-see-is-quoted-as.md)

**Raised** 2026-11-10 by S1f.5.22's T3. The user's line names the character, `'<char>' (U+XXXX)`. A no-break space, a
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

**Decision file:** [`docs/decisions/diagnostics/Q-M1f.93-a-modules-read-line-names-the-path-its-import.md`](../../docs/decisions/diagnostics/Q-M1f.93-a-modules-read-line-names-the-path-its-import.md)

**Raised** 2026-11-10 by S1f.5.22's T3. The read's line names the path it was handed. A module has two: the path the import
resolves to, `<the importing file's directory>/<module>.ein`, which `module not found at …` names, and its canonical
path, which its parse errors and its forms' positions name.

- **(a) The path the import resolves to**: relative where the program was given relatively, as `ein solve` run from the
  repository names `examples/…`.
- **(b) The canonical path**, as the module's parse errors.

**Defaulted 2026-11-10 (the stage's default): (a).** The read is of the path the import names, and the line is the one
that path, given to a command, prints. Re-opening it means (b).

### Q-M1f.94 — A directory's and an unreadable module's line, and a missing one's `(None)`

**Decision file:** [`docs/decisions/diagnostics/Q-M1f.94-a-module-is-refused-with-the-line-its-path-gives.md`](../../docs/decisions/diagnostics/Q-M1f.94-a-module-is-refused-with-the-line-its-path-gives.md)

**Raised** 2026-11-10 by S1f.5.22's T3. The program's read names a directory and an unreadable file in Python's spelling,
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

**Decision file:** [`docs/decisions/diagnostics/Q-M1f.95-a-file-is-refused-at-its-first-character-that-is.md`](../../docs/decisions/diagnostics/Q-M1f.95-a-file-is-refused-at-its-first-character-that-is.md)

**Raised** 2026-11-10 by S1f.5.22's T3. `lrpar` lexes the whole file before it parses, and stops at the first lexing error
(`parse_actions`), so a file is refused at its first character that is not ASCII outside a string and a comment, one
error, where a parse error is followed by every error recovery reaches.

- **(a) The first**, where the lexer stops.
- **(b) Every one**, a scan over the file before the parse, each a line.

**Defaulted 2026-11-10 (the stage's default): (a).** It is where the file stops being Ein, as a character no token
starts with was; the scan would read a string's and a comment's extent a second time. Re-opening it means (b).

### Q-M1f.96 — A byte-order mark

**Decision file:** [`docs/decisions/language/Q-M1f.96-a-byte-order-mark-is-refused-as-any-character.md`](../../docs/decisions/language/Q-M1f.96-a-byte-order-mark-is-refused-as-any-character.md)

**Raised** 2026-11-10 by S1f.5.22's T2. A file an editor saved with a byte-order mark starts with U+FEFF, which was a token
that is not a name, `` unexpected `\u{feff}…` ``; no corpus file starts with one.

- **(a) Refused as any other character that is not ASCII**, at 1:1, `'\u{feff}' (U+FEFF) is not ASCII: …`.
- **(b) Skipped where it is the file's first character.**

**Defaulted 2026-11-10 (the stage's default): (a).** The ruling is *anywhere else than inside string literals*, with
comments added by Q-M1f.65 (b). Re-opening it means (b).

### Q-M1f.97 — How a test closes a run's stdout

**Decision file:** [`docs/decisions/corpus/Q-M1f.97-a-test-closes-a-runs-stdout-before-the-run.md`](../../docs/decisions/corpus/Q-M1f.97-a-test-closes-a-runs-stdout-before-the-run.md)

**Raised** 2026-11-10 by S1f.5.25's T2. The acceptance reads a run's stdout with *a reader that closes after
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

**Raised** 2026-11-10 by S1f.5.25's T2. Its census ran seventy-two runs, every subcommand and every option that
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

**Raised** 2026-11-10 by S1f.5.25's T2. The register's schema `/20` records an exit code and a line of stderr. A
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

**Decision file:** [`docs/decisions/cli/Q-M1f.100-an-artefact-written-to-a-closed-pipe-ends.md`](../../docs/decisions/cli/Q-M1f.100-an-artefact-written-to-a-closed-pipe-ends.md)

**Raised** 2026-11-10 by S1f.5.25's T2. [`07-diagnostics.md` § 2.4](../../docs/ein/user/04-reference/07-diagnostics.md#24-an-artefact-flags-failure)
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

**Decision file:** [`docs/decisions/cli/Q-M1f.101-off-unix-a-closed-stdout-is-left-to-rusts.md`](../../docs/decisions/cli/Q-M1f.101-off-unix-a-closed-stdout-is-left-to-rusts.md)

**Raised** 2026-11-10 by S1f.5.25's T2. Windows, a release target, has no `SIGPIPE`: a write to a closed pipe there
is an error, which a `print!` turns into the same panic, exit 101. No host here runs Windows, so no probe shows it,
and the standard of proof refutes a behaviour only with one.

- **(a) Leave it**: `restore_sigpipe` is a no-op off Unix, and the case is shown to the user at the close.
- **(b) A quiet stop on Windows**, Q-M1f.68's (a), every stdout write through one writer there.

**Defaulted 2026-11-10 (the stage's default): (a).** The ruling is about `SIGPIPE`, which Windows does not have, and
(b) is the writer the ruling chose not to build. Re-opening it means (b).

### Q-M1f.102 — What the CI nightly keeps, and what the dev host runs

**Raised** 2026-11-10 by S1f.5.27, on the user's ruling on the close
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

**Raised** 2026-11-10 by S1f.5.27's T1. The stage file proposed 100 000
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

**Raised** 2026-11-10 by S1f.5.27's T1. `corpus_cores`' slow half, the
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

**Raised** 2026-11-10 by S1f.5.27's T2. The stage asks that a test pin the
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

**Raised** 2026-11-10 by S1f.5.27's T1. Since
S1f.5.29 a DOT Graphviz refuses fails the
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

**Raised** 2026-11-10 by the coordinator, writing S1f.5.28. The close review's finding 7, the stale slow
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

**Decision file:** [`docs/decisions/read-out/Q-M1f.108-route-1s-stop-at-a-layer-is-the-librarys-and-no.md`](../../docs/decisions/read-out/Q-M1f.108-route-1s-stop-at-a-layer-is-the-librarys-and-no.md)

**Raised** 2026-11-10 by S1f.5.28's T1 (its item 4, from S1f.5.16). Describe takes route 1 only on a flagged
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

**Decision file:** [`docs/decisions/read-out/Q-M1f.109-a-texts-line-break-in-the-trace-is-a-markdown.md`](../../docs/decisions/read-out/Q-M1f.109-a-texts-line-break-in-the-trace-is-a-markdown.md)

**Raised** 2026-11-10 by S1f.5.28's T1 (its item 5, from S1f.5.16). `--trace` writes markdown, and wrote a
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

**Decision file:** [`docs/decisions/engine/Q-M1f.110-a-container-saved-again-is-the-same-container.md`](../../docs/decisions/engine/Q-M1f.110-a-container-saved-again-is-the-same-container.md)

**Raised** 2026-11-10 by S1f.5.28's T1 (its item 9, from S1f.5.19's record, which recorded it and did not file
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

**Raised** 2026-11-10 by S1f.5.28's item 10, on S1f.5.27's T1: the parse is 99 % of an input's cost, and five
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
analysis of P1f.7's nine findings, taken before
any stage started. Three things it left to the user:

- whether S1f.7.9's fix stays in the phase or moves to a followup, since refusing a `:rule` that names no rule
  leaves the larger defect, a written provenance that leaves every core;
- whether S1f.7.8 runs first, as the one finding a puzzle's author meets by a misspelt name;
- whether the three findings with no stage (`(q (eq k j))` stored, a goal over a relation nothing makes, a macro
  call written as a fact) become stages.

**Ruled 2026-11-11 (the user):** *"yes S1f.7.9 moves to a new F24; yes the phase order changes to put S1f.7.8
first"*, and *"yes - it become S1f.7.10 onward"*. [F24](../followups/f24_written_provenance.md) is filed, S1f.7.9
files its rows `open` and owned by it, the order is restated,
and S1f.7.10 to S1f.7.12 are planned.

### Q-M1f.113 — What is a nested `(eq k j)` for: a quoted statement, or nothing a state can hold?

**Decision file:** [`docs/decisions/language/Q-M1f.113-an-object-is-an-atom-or-a-fact-a-state-could.md`](../../docs/decisions/language/Q-M1f.113-an-object-is-an-atom-or-a-fact-a-state-could.md)

**Raised** 2026-11-11 by the coordinator, on S1f.7.10's
finding and the user's question of that day (what such a fact would be used for, from Ein's point of view).

- **(a) The value rule**: a value is a name, or a fact a state could hold, an atom of an ordinary relation or its
  negation. `eq`, `neq`, `absent`, `false` with an argument, `and` and `or` are never a nested head; `not` stays.
- **(b) Quoting kept**: `eq` and `neq` both allowed as values matched by shape, and the `(not ?s)` conclusion made
  to check a bound `eq`.

**Ruled 2026-11-11 (the user): (a)**, *"(a), adopt the rule"*. The reasons are in
P1f.7 § The value rule. With it the user ruled how P1f.7
treats what it finds outside its scope: fixed in place if quick, or a row of
[F25](../followups/f25_roughness_found_outside_zebra.md), and not shown for review unless it moves an existing
zebra program's answer.

### Q-M1f.114 — What a relation in a `(solve …)` list must be

**Decision file:** [`docs/decisions/language/Q-M1f.114-a-relation-a-solve-list-names-is-one-the-program.md`](../../docs/decisions/language/Q-M1f.114-a-relation-a-solve-list-names-is-one-the-program.md)

**Raised** 2026-11-11 by S1f.7.8's T1. The stage refuses a name in `:hypothesis-relations`, in
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

**Decision file:** [`docs/decisions/language/Q-M1f.115-an-hrules-list-names-declared-hrules.md`](../../docs/decisions/language/Q-M1f.115-an-hrules-list-names-declared-hrules.md)

**Raised** 2026-11-11 by S1f.7.8's T1. Beside the list's first name, an `:hrules` list holds bare names
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

**Decision file:** [`docs/decisions/language/Q-M1f.116-an-activators-argument-at-a-parameter-that-names.md`](../../docs/decisions/language/Q-M1f.116-an-activators-argument-at-a-parameter-that-names.md)

**Raised** 2026-11-11 by the analysis of that day
and S1f.7.8's T1: `(guess (at Colour House))`, a misspelt type at a parameter that names no relation,
gives the hrule no match and H is empty.

- **(a) Refuse a value the program names nowhere else.** A value is any name, and an activator may name one first:
  `examples/features/11_expect_ambiguity.ein`'s `(guess (Ann S1) …)` gives objects.
- **(b) Do not check it**, and say in the read-out that an `:hrules` entry proposes nothing at root.

**Defaulted 2026-11-11 (the stage's default): (b)**, as the stage's brief has it. The read-out row is not built
here: it is a new `Row` and a line of `read_out.md`, and is [F25](../followups/f25_roughness_found_outside_zebra.md)'s
R-2.

### Q-M1f.117 — A goal with a macro call: printed as written, or as expanded

**Decision file:** [`docs/decisions/language/Q-M1f.117-a-macro-call-in-a-goal-is-expanded-and-the-goal.md`](../../docs/decisions/language/Q-M1f.117-a-macro-call-in-a-goal-is-expanded-and-the-goal.md)

**Raised** 2026-11-11 by the analysis of that day
and S1f.7.5's T1. The stage file names two directions, expanding a goal's macro calls as a rule's `:match`'s are, or
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

**Decision file:** [`docs/decisions/language/Q-M1f.118-a-claim-is-not-expanded-so-a-claim-headed.md`](../../docs/decisions/language/Q-M1f.118-a-claim-is-not-expanded-so-a-claim-headed.md)

**Raised** 2026-11-11 by S1f.7.5's T1, which the stage file asks to state for each clause of a query that holds a
fact: `:expect (model (settled Ann))` beside a macro `settled`, and the same in a `(solve … :expect (region …))`.

- **(a) Not expanded**: a claim lists facts, a macro is a premise's shorthand, and a fact headed by a macro's name
  is a fact of no relation, refused at load as any claim naming no relation of the program is.
- **(b) Expanded**, so that a claim may be written in the goal's own words. An expansion is a premise, `(and
  (absent …) …)`, which is no fact.

**Defaulted 2026-11-11: (a).** Before the stage the claim was refused with the same line, so nothing moved. A macro
call written as a fact is S1f.7.12's.

### Q-M1f.119 — Which relations a goal may ask about

**Decision file:** [`docs/decisions/language/Q-M1f.119-a-goal-and-a-claim-ask-only-about-relations.md`](../../docs/decisions/language/Q-M1f.119-a-goal-and-a-claim-ask-only-about-relations.md)

**Raised** 2026-11-11 by S1f.7.11. Its default is (a), refused at load, against (b), a warning row in the read-out.
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

**Decision file:** [`docs/decisions/language/Q-M1f.120-a-rule-may-read-a-relation-nothing-makes.md`](../../docs/decisions/language/Q-M1f.120-a-rule-may-read-a-relation-nothing-makes.md)

**Raised** 2026-11-11 by S1f.7.11's stage file: a rule whose `:match` reads a relation no declaration, fact or
conclusion makes can never fire.

- **(a) Refused at load**, as the goal is.
- **(b) Reported and not refused**: a library rule may read a relation its importer supplies, and the stdlib's
  rules read marker relations, `functional`, `total`, whose absence means the rule does not fire.

**Defaulted 2026-11-11 (the stage's default): (b).** It is reported in the stage's T1 record and is
[F25](../followups/f25_roughness_found_outside_zebra.md)'s R-6; no surface says that a rule can never fire.

### Q-M1f.121 — The line a goal's nested relation variable is refused with

**Decision file:** [`docs/decisions/diagnostics/Q-M1f.121-a-goals-relation-variable-is-refused-with.md`](../../docs/decisions/diagnostics/Q-M1f.121-a-goals-relation-variable-is-refused-with.md)

**Raised** 2026-11-11 by S1f.7.1, whose acceptance says *refused on every command with the rule's line*. A rule's
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

**Decision file:** [`docs/decisions/language/Q-M1f.122-absent-heads-a-premise-and-a-member-of-a-claims.md`](../../docs/decisions/language/Q-M1f.122-absent-heads-a-premise-and-a-member-of-a-claims.md)

**Raised** 2026-11-11 by S1f.7.7's T1. The stage's direction is that `FactHead`, `ArgHead` and `ConclusionHead` lose
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

**Decision file:** [`docs/decisions/language/Q-M1f.123-false-with-no-argument-is-an-object.md`](../../docs/decisions/language/Q-M1f.123-false-with-no-argument-is-an-object.md)

**Raised** 2026-11-11 by S1f.7.6's T1. `(q (false))`, `(not (false))` and a premise `(false)` load today. The value
rule names *`false` with an argument*.

- **(a) `(false)` stays** wherever it is written today, and only an argument is refused.
- **(b) `false` heads no value at all**, with or without an argument.

**Defaulted 2026-11-11: (a).** The rule's own words, and `(false)` is a fact a state could hold: a dead state holds
it. What `(not (false))` and `(q (false))` mean is [F25](../followups/f25_roughness_found_outside_zebra.md)'s R-7.

### Q-M1f.124 — Where `(eq a b)` is still written, once it is no value

**Decision file:** [`docs/decisions/language/Q-M1f.124-eq-a-b-is-written-only-as-a-form-of-its-own.md`](../../docs/decisions/language/Q-M1f.124-eq-a-b-is-written-only-as-a-form-of-its-own.md)

**Raised** 2026-11-11 by S1f.7.10's T1. S1f.5.7 has a written `(eq a b)` and `(not (eq a b))`, a premise and a
conclusion of either, and a claim's, each checked. The ruling (a) refuses `eq` as a nested head. Three places sit
between the two: `(q (not (eq a b)))`, a fact's `:using ((eq a b))`, and a trace's `:derives (eq a b)`.

- **(a) Refused in all three**: `(not (eq a b))` is the negation of no fact a state could hold, so it is no value;
  `:using` lists facts a derivation rested on and a trace's step derives a fact, and no state stores an equality.
- **(b) Kept in `:using` and a trace**, which are provenance and not values.

**Defaulted 2026-11-11: (a).** No corpus program writes any of the three (the sweep of T3). `(eq a b)` and `(not
(eq a b))` stay where S1f.5.7 checks them (`ein.y`'s `EqFact` and `Negated`).

### Q-M1f.125 — A connective as a pattern's argument: the grammar or the loader

**Decision file:** [`docs/decisions/language/Q-M1f.125-a-connective-as-a-patterns-argument-is-refused.md`](../../docs/decisions/language/Q-M1f.125-a-connective-as-a-patterns-argument-is-refused.md)

**Raised** 2026-11-11 by S1f.7.3's stage file: *if no macro call needs it, the grammar refuses the written shape at
the parse*. T1 read why `ein.y`'s `Arg` takes a `Connective`: a macro is called as a pattern, and `std.macro`'s
`forall` takes premises, `(forall ?b (and …) (B))`. A context-free grammar cannot tell a macro's name from a
relation's.

- **(a) The loader**, after macro expansion, by the walk S1f.5.24 gave a rule's clauses.
- **(b) The grammar**, with a macro's call given a syntax of its own.

**Defaulted 2026-11-11: (a).** (b) is a language change. The refused heads are what no value has: `and`, `or`,
`neq`, with S1f.5.24's line, and `eq`, `absent` and `false` with an argument, which S1f.7.10, S1f.7.7 and S1f.7.6
left no fact able to hold. `(not <pattern>)` stays: a stored negation is a value and is matched.

### Q-M1f.126 — A relation parameter bound to `absent`, `false` or `eq`

**Decision file:** [`docs/decisions/language/Q-M1f.126-a-relation-parameter-bound-to-absent-false-or-eq.md`](../../docs/decisions/language/Q-M1f.126-a-relation-parameter-bound-to-absent-false-or-eq.md)

**Raised** 2026-11-11 by the three stages' T1: with `(mk absent)` and `(rule mk (?R) :match (p ?x) :assert (?R
?x))` the rule stored `(absent a)`; with `(mk false)`, `(false a)`; and with `(mk eq)` and `:assert (q (?R ?x
?x))`, `(q (eq a a))`. The grammar reads a variable where the relation is written.

- **(a) Refused at load where the loader can see the binding**, as S1f.5.9 refuses a parameter that can hold `eq`
  heading a form of another arity (`defined_behaviour.md` § 4.14).
- **(b) A row of F25**, the firing storing what it is given.

**Defaulted 2026-11-11: (a), fixed in place.** The check of § 4.14 runs once per word, and reads a parameter that
heads a form where a value goes too.

### Q-M1f.127 — What the loader reads to see that a negated variable can hold a name

**Decision file:** [`docs/decisions/language/Q-M1f.127-a-conclusion-not-x-is-refused-where-the-program.md`](../../docs/decisions/language/Q-M1f.127-a-conclusion-not-x-is-refused-where-the-program.md)

**Raised** 2026-11-11 by S1f.7.4's T1. The stage's default is a refusal at
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

**Decision file:** [`docs/decisions/language/Q-M1f.128-a-firing-that-meets-the-negation-of-a-name.md`](../../docs/decisions/language/Q-M1f.128-a-firing-that-meets-the-negation-of-a-name.md)

**Raised** 2026-11-11 by S1f.7.4's T1, and
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

**Decision file:** [`docs/decisions/engine/Q-M1f.129-an-hrule-that-reads-not-reads-the-state-it-was.md`](../../docs/decisions/engine/Q-M1f.129-an-hrule-that-reads-not-reads-the-state-it-was.md)

**Raised** 2026-11-11 by S1f.7.2's T1. The premise of `Hrules::candidates`' `debug_assert!` is
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

**Decision file:** [`docs/decisions/language/Q-M1f.130-a-macros-call-in-a-conclusion-is-expanded.md`](../../docs/decisions/language/Q-M1f.130-a-macros-call-in-a-conclusion-is-expanded.md)

**Raised** 2026-11-11 by S1f.7.12's T1. The stage file lists a rule's and an hrule's `:assert` among the sites where a
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

**Decision file:** [`docs/decisions/language/Q-M1f.131-no-relation-is-declared-under-a-macros-name.md`](../../docs/decisions/language/Q-M1f.131-no-relation-is-declared-under-a-macros-name.md)

**Raised** 2026-11-11 by S1f.7.12's T1. `(relation pp T)` beside `(macro pp (?a) (p ?a))` loads. A premise `(pp ?x)` is
the macro's call, so no premise reads the relation, and since the stage no fact is of it. No corpus program declares
a relation and a macro of one name (`git ls-files '*.ein'`, 651 files, the stage's census, 2026-11-11).

- **(a) Refused at load**, one line at the declaration.
- **(b) Left**: the declaration loads, and each fact of the relation is refused.

**Defaulted 2026-11-11: (a).** A relation nothing can hold or read is a mistake in the name, and the declaration is
where the author reads about it. A rule and a macro of one name still load: the rule's activator fact is refused as
the macro's call ([F25](../followups/f25_roughness_found_outside_zebra.md)'s R-8).

### Q-M1f.132 — A `std.macro` macro the program does not import, written as a fact

**Decision file:** [`docs/decisions/language/Q-M1f.132-a-std-macro-macro-the-program-does-not-import.md`](../../docs/decisions/language/Q-M1f.132-a-std-macro-macro-the-program-does-not-import.md)

**Raised** 2026-11-11 by S1f.7.12's T1. `(unknown (p j))` in a program that does not import `std.macro` is stored as a
fact of a relation `unknown`. A rule's `:match` that writes `(unknown …)` without the import has been refused since
S1.8a.f20, and a goal's since S1f.7.5, so no premise of the program can read that fact.

- **(a) Refused**, the line saying the macro is `std.macro`'s and not imported.
- **(b) Stored**: without the import `unknown` is an ordinary name.

**Defaulted 2026-11-11: (a).** The guard already makes `unknown` and `forall` names a premise cannot use as a
relation, so a fact of either is read by nothing. No corpus program writes one. A `(relation unknown …)` declaration
in such a program is not refused where it stands (F25's R-8).

### Q-M1f.133 — A macro's call inside a fact a claim lists, and in a fact's `:using`

**Decision file:** [`docs/decisions/language/Q-M1f.133-a-macros-call-inside-a-claims-fact-or-a-using-is.md`](../../docs/decisions/language/Q-M1f.133-a-macros-call-inside-a-claims-fact-or-a-using-is.md)

**Raised** 2026-11-11 by S1f.7.12's T1. Q-M1f.118 has a claim headed by a macro's call refused: it names no relation.
A call inside a listed fact, `(model (q (pp j)))`, loaded, and the claim failed against a state holding `(q (p j))`.
A fact's `:using ((pp j))` loaded too.

- **(a) Refused as a written fact's is**: a claim lists facts, and a `:using` names facts.
- **(b) Left to the claim's check**, which fails it at the run.

**Defaulted 2026-11-11: (a).** A claim that can never hold is refused at load elsewhere (`eq` in a region claim,
S1f.5.7). The claim's line is added only where nothing else refused the program, so Q-M1f.118's line stays the one
a claim headed by the call prints.

### Q-M1f.134 — What a register row records as `right` while F24's question is not ruled

**Raised** 2026-11-11 by S1f.7.9's T2. The stage files F24's three table rows `open`, owned by F24, and the right
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

**Raised** 2026-11-11 by [04-stdlib/README.md § G](../../docs/ein/user/04-reference/04-stdlib/README.md#importing-a-module), point (a), and taken by S1f.6.11. The user ruled that day that *"importing facts is
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

**Raised** 2026-11-11 by [04-stdlib/README.md § G](../../docs/ein/user/04-reference/04-stdlib/README.md#importing-a-module), point (b). A qualified import renames every name the module declares, the relation
heading a fact among them, so a library's `is-a` facts land in `o.is-a` and do not join the program's `is-a`.

- **(b1) Keep it, and say nothing.**
- **(b2) Never rename a relation that heads a fact**: the module's own rules, renamed, would stop reading them.
- **(b3) Keep it, and say on the page that a library of facts is imported flat.**

**Defaulted 2026-11-11: (b3).** Renaming is what a qualified import is. `01_grammar.md` § Import says it.

### Q-M1f.137 — Are an imported fact's constants names of the program?

**Decision file:** [`docs/decisions/imports/Q-M1f.137-an-imported-facts-constants-are-names.md`](../../docs/decisions/imports/Q-M1f.137-an-imported-facts-constants-are-names.md)

**Raised** 2026-11-11 by [04-stdlib/README.md § G](../../docs/ein/user/04-reference/04-stdlib/README.md#importing-a-module), point (c). They were under a qualified import, and they enter the blind
enumerator's objects: importing an ontology of n constants widens the universe of every open relation by n.

- **(a) Yes**: an imported fact is a fact of the program, and `solution_semantics.md` § 1.2 defines the names as
  the constants the program's facts and rules write.
- **(b) Mark library constants**, so the enumerator skips them.

**Defaulted 2026-11-11: (a).** A program that wants no guesses about a library's relation closes it, or the
library closes its own.

### Q-M1f.138 — The provenance of an imported fact

**Decision file:** [`docs/decisions/imports/Q-M1f.138-an-imported-fact-keeps-its-own-provenance.md`](../../docs/decisions/imports/Q-M1f.138-an-imported-fact-keeps-its-own-provenance.md)

**Raised** 2026-11-11 by [04-stdlib/README.md § G](../../docs/ein/user/04-reference/04-stdlib/README.md#importing-a-module), point (d).

- **(a) Its own**: a `:source` where the module wrote one, otherwise background, as if written in the program.
- **(b) Stamped with the module's name** where it has no `:source`.

**Defaulted 2026-11-11: (a).** What a written provenance may claim is
[F24](../followups/f24_written_provenance.md)'s, and a stamp is the same question.

### Q-M1f.139 — The same fact from two places

**Decision file:** [`docs/decisions/imports/Q-M1f.139-a-fact-stated-twice-keeps-its-first-statement.md`](../../docs/decisions/imports/Q-M1f.139-a-fact-stated-twice-keeps-its-first-statement.md)

**Raised** 2026-11-11 by [04-stdlib/README.md § G](../../docs/ein/user/04-reference/04-stdlib/README.md#importing-a-module), point (e); E-6 of `findings.md`. A fact stated by
a module and by the program, or by two modules, with different `:source`s keeps its first statement, and the
later `:source` is gone with no word, as it is for a fact written twice in one file.

- **(a) State the behaviour, and hand the question to F24.**
- **(b) Refuse, or warn, here.**

**Defaulted 2026-11-11: (a).** The stage does not make it worse or better: `defined_behaviour.md` § 4.25 and
`01_grammar.md` § Import state it, and E-6 names F24. Identical facts are one fact (Q-M1f.143).

### Q-M1f.140 — A rule of the module the selection does not name

**Raised** 2026-11-11 by [04-stdlib/README.md § G](../../docs/ein/user/04-reference/04-stdlib/README.md#importing-a-module), point (f). `(import onto :symbols (is-a))` brings the `is-a` facts and neither of
the module's rules over `is-a`, though the facts would fire them.

- **(a) Still dropped.**
- **(b) Brought by the facts it reads.**

**Defaulted 2026-11-11: (a).** A selection is a selection; one name that brings a theory is a rule that names
its rules, as the calculator's closing rule does.

### Q-M1f.141 — Does a selection's closure follow the names inside a kept fact?

**Raised** 2026-11-11 by S1f.6.11's T3. The closure followed the names a kept *declaration* writes. Once facts are
kept, a kept fact writes names too: `(owns Ann cat)`, where `cat` is a relation the module declares and states
facts of.

- **(a) No**: declarations only, as before the stage. `(owns Ann cat)` brings neither `cat`'s declaration nor
  its facts.
- **(b) Yes**: every name a kept form writes is followed, so a constant that happens to be a declared name
  brings a declaration nobody asked for, and a fact's `:rule` brings the rule.

**Defaulted 2026-11-11: (a).** Point (a)'s default says *the closure follows names as today*, and nothing comes
unasked. Held by `a_selection_follows_declarations_and_not_the_names_inside_a_fact`.

### Q-M1f.142 — A stored negation, and a fact of a kernel relation, under a selection

**Raised** 2026-11-11 by S1f.6.11's T3. `(not (likes Milk Tea))` is headed `not`, and `(__closed__ is-a)` and a
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

**Decision file:** [`docs/decisions/imports/Q-M1f.143-a-modules-fact-arrives-once-however-many-imports.md`](../../docs/decisions/imports/Q-M1f.143-a-modules-fact-arrives-once-however-many-imports.md)

**Raised** 2026-11-11 by S1f.6.11's T3 and T4. A flat diamond over declarations collapses, identical ones being one.
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

**Raised** 2026-11-11 by S1f.6.11's T2. The witnesses have no candidate, so R is {∅} before and after, and a
state's `(model …)` names the candidates' relations: what the import drops is root's facts.

- **(a) The sentences the query's `:goal-text` prints**, one per row of the goal (schema /15, W37's), and for
  the name that could not be listed, the refusal.
- **(b) A new field for root's facts**: a schema change.
- **(c) A claim**: a witness's `:expect` states today's answer until the fix, so the row would read *held*
  before and after.

**Defaulted 2026-11-11: (a).** No schema change, as the stage file expected.

### Q-M1f.145 — How the table generator's check is in the gate

**Raised** 2026-11-11 by S1f.6.2's T1. The user ruled the tables generated, with a `--check` in the gate
(`design.md`'s question 7, (a)). *In the gate* has two shapes in this repository.

- **(a) A cargo test runs the script's `--check`**, as `ein-cli`'s `cli_semantics.rs` runs
  `examples/gen_zebra2_variants.py --check`. `cargo test --workspace` is a gate step, and the per-commit job
  installs a `python3` for that test already.
- **(b) A step of its own** in `run_tests.sh`, `per-commit.yml` and `gate_steps.rs`, beside
  `utils/stdlib_manifest.py`: a seventh static check, and the header's *six* in three places.

**Defaulted 2026-11-11: (a).** The precedent, and the same test file holds every row to Rust's integers, so the
generator and its oracle fail together. A machine without `python3` fails the test and says why.

### Q-M1f.146 — How `std.digits` closes its tables for a flat importer

**Decision file:** [`docs/decisions/stdlib/math/Q-M1f.146-each-digit-table-is-closed-by-a-written-fact.md`](../../docs/decisions/stdlib/math/Q-M1f.146-each-digit-table-is-closed-by-a-written-fact.md)

**Raised** 2026-11-11 by S1f.6.2's T2. The stage file says the module closes its relations with written
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

**Raised** 2026-11-11 by S1f.6.2's T4. `ein-render/tests/dot_wellformed.rs` puts every view of every corpus file
through `dot -Tsvg`, per commit. Every view of `stdlib/digits.ein` has some 7 900 edges onto eleven shared
names: `dot` 16.0.0 had not drawn one after five minutes, `fdp` not after 100 s, `sfdp` dumps core
(2026-11-11). The gate did not finish. E-10 of `findings.md` is the finding.

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

**Overruled 2026-11-12, and built** (S1f.6.21). The user,
on reply 45 of the calculator's `questions.md`: *"don't test graphviz itself - that is the main point here. just
skip big graphs, run tests only on small"*. It is (c), by size: a view of more than 1 000 edges is counted,
named in the test's output and put to Graphviz under no layout. Nothing checks the DOT text of such a view.

### Q-M1f.148 — One module for the seven tables

**Decision file:** [`docs/decisions/stdlib/math/Q-M1f.148-the-seven-digit-tables-are-one-module.md`](../../docs/decisions/stdlib/math/Q-M1f.148-the-seven-digit-tables-are-one-module.md)

**Raised** 2026-11-11 by S1f.6.2's T4. A flat import of one table parses the whole module: 28.8 KB, about 12 ms on
the generated parser: `ein test` of a program that imports `fa` alone takes 17.9 ms, and of a `std.macro` test
5.8 ms (best of ten, release, 2026-11-11).

- **(a) One module,** `stdlib/digits.ein`, as the stage file says.
- **(b) A module per table,** `std.digits.fa` and so on: a program that adds parses 200 rows, not 1 664.

**Defaulted 2026-11-11: (a).** The stage file's, and one name for the ontology. The cost is per program, not
per request: a program that does not import the module pays nothing. (b) stays open if a suite of many small
arithmetic programs makes the 12 ms matter.

### Q-M1f.149 — What `(let NAME NUMBER)` does, as built

**Decision file:** [`docs/decisions/stdlib/math/decimal/Q-M1f.149-a-request-may-name-an-operand-by-a-let.md`](../../docs/decisions/stdlib/math/decimal/Q-M1f.149-a-request-may-name-an-operand-by-a-let.md)

**Raised** 2026-11-11 by S1f.6.3's T2. The user ruled *"(let NAME NUMBER) is ok"*; what a request may do with a name
is item 2 of [`04-stdlib/07-calculator.md` § what is open](../../docs/ein/user/04-reference/04-stdlib/07-calculator.md), the
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

**Raised** 2026-11-11 by S1f.6.3's T4, which asks for *a deterministic count of the matcher's work per request (a
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

**Overruled 2026-11-12, and built** (S1f.6.21). The user,
on reply 46, option (b): *"(b) - we don't need this proportional gate"*. The step left `run_tests.sh` and
`per-commit.yml`; the test, the feature and the bounds stay, run by hand, in no tier.

### Q-M1f.151 — The names of the circuit's own relations

**Decision file:** [`docs/decisions/stdlib/math/decimal/Q-M1f.151-the-circuits-own-relations-are-named-with-words.md`](../../docs/decisions/stdlib/math/decimal/Q-M1f.151-the-circuits-own-relations-are-named-with-words.md)

**Raised** 2026-11-11 by S1f.6.3's T1. The stage file names `n`, `z`, `b`, `add`, `sum-of`, `let`, `done` and the tag
`sum`. The prototype's other relations are `ak`, `sb`, `bk`.

- **(a) Words:** `add-walk`, `strip`, `turn`, `operand`, `operand-is`, `named`, `add-first`, `add-second`,
  `arith-rule`. A `--trace` and a state file print them.
- **(b) The prototype's two letters:** unlikely to clash with a program's names, and unreadable.
- **(c) A prefix,** `arith.walk`: a dot is what a qualified import writes, and would read as one.

**Defaulted 2026-11-11: (a).** Imported flat, every one is a name the program may not use otherwise, `done`,
`sum`, `strip` and `turn` among them; the module's header lists them. What a clash does and what the loader
should say is S1f.6.12's.

### Q-M1f.152 — Are `std.arith`'s rules written by hand or generated?

**Decision file:** [`docs/decisions/stdlib/math/decimal/Q-M1f.152-the-decimal-arithmetics-rules-are-written.md`](../../docs/decisions/stdlib/math/decimal/Q-M1f.152-the-decimal-arithmetics-rules-are-written.md)

**Raised** 2026-11-11 by S1f.6.3's T1. The tables are generated (the user's ruling). The prototype generated its
rules too, since division has a rule for each of nine multiples.

- **(a) By hand,** in `stdlib/arith.ein`, with comments beside the rules.
- **(b) Generated,** with the same `--check`.

**Defaulted 2026-11-11: (a), for addition.** 19 rules, none a family. The stage that writes division's
`div-multiple-2` to `-9` and `div-try-1` to `-9` meets the question again; a generated block inside a
hand-written module would need its own check.

### Q-M1f.153 — The DOT sweep's bound, under the smallest table

**Raised** 2026-11-11 by S1f.6.3's T6, on Q-M1f.147's bound of 1 500 edges. The KB view of a program that imports
`std.arith` holds `fa`'s 200 rows, some 1 200 edges; `dot` draws it in 0.55 s, and each program has six KB
views.

- **(a) 1 000 edges:** a view that holds a table is past the bound; the bounded layout takes 0.1 s on it.
- **(b) 1 500:** `dot` still lays out the addition programs, 3.3 s each, 26 s for the stage's eight, and more
  with every program a later stage adds until it imports a second table.

**Defaulted 2026-11-11: (a).** The largest view the corpus had before the tables has 361 edges, so what `dot`
lays out is what it laid out then. With Q-M1f.147, for the user to confirm.

**Overruled 2026-11-12 with Q-M1f.147, and built** (S1f.6.21):
the bound stays 1 000 edges, and a view past it is skipped where it went to the bounded layout.

### Q-M1f.154 — The bounds of the harness against the integers

**Decision file:** [`docs/decisions/corpus/Q-M1f.154-the-calculators-harness-is-exhaustive-to-120.md`](../../docs/decisions/corpus/Q-M1f.154-the-calculators-harness-is-exhaustive-to-120.md)

**Raised** 2026-11-11 by S1f.6.3's T3, which asks for *random operands of every length up to a stated bound*.

- **(a) Exhaustive to 120, sampled to 30 digits:** every pair from 0 to 120; every pair of lengths from 0 to
  30 digits, twice; a carry through every length to 30; 1 500 operands padded with high zeros. The oracle is
  `u128`, which holds 38 digits. A second of test time.
- **(b) Longer operands,** against a decimal adder written in the test: an oracle that is itself a column
  adder.

**Defaulted 2026-11-11: (a).** The theory has no width, so a length is not a boundary of it: the walk at the
thirtieth column is the walk at the third. S1f.6.9, on how long a number may be, is where the nesting limit of
1 000 is met.

### Q-M1f.155 — Equal operands of a subtraction: two walks, one fact

**Decision file:** [`docs/decisions/stdlib/math/decimal/Q-M1f.155-equal-operands-of-a-subtraction-are-answered.md`](../../docs/decisions/stdlib/math/decimal/Q-M1f.155-equal-operands-of-a-subtraction-are-answered.md)

**Raised** 2026-11-11 by S1f.6.4's T1. The circuit subtracts both ways. When the operands are equal neither
walk borrows, so both hand over zero.

- **(a) Both answer:** `sub-answer` derives `(diff-of X Y z z)` from the one walk and `sub-answer-short` the
  same fact from the other. One answer, with two derivations; two firings more than an unequal pair's one.
- **(b) One walk is told to keep quiet on zero:** a rule more, or a shape test on the shortfall, `(n ?p ?a)`,
  so that only a shortfall that is not zero answers.

**Defaulted 2026-11-11: (a).** The answer is the same fact either way, the harness holds *exactly one answer a
request*, and (b) buys two firings on equal pairs with a rule that reads less plainly.

### Q-M1f.156 — Comparison called with a tag

**Decision file:** [`docs/decisions/stdlib/math/decimal/Q-M1f.156-comparison-is-called-with-a-tag.md`](../../docs/decisions/stdlib/math/decimal/Q-M1f.156-comparison-is-called-with-a-tag.md)

**Raised** 2026-11-11 by S1f.6.4's T1. The prototype's comparison walk carried the request's operands and
wrote `cmp-of` when it ran out of columns.

- **(a) A tag, like every other walk:** `(cmp-walk TAG X Y ORDER)` ends in `(order-is TAG ORDER)`, and the
  public request's rule reads `(order-is (order X Y) O)`. One firing more a request, 13 for 123078 against 4567.
- **(b) As the prototype:** a firing fewer, and no way to call a comparison from inside the theory.

**Defaulted 2026-11-11: (a).** S1f.6.7 and S1f.6.8 compare numbers inside their own circuits, and
S1f.6.3's rule is that a call inside the theory starts a walk with a tag of its own.

### Q-M1f.157 — Each request looks its operands up by rules of its own

**Decision file:** [`docs/decisions/stdlib/math/decimal/Q-M1f.157-each-request-looks-its-operands-up-by-rules.md`](../../docs/decisions/stdlib/math/decimal/Q-M1f.157-each-request-looks-its-operands-up-by-rules.md)

**Raised** 2026-11-11 by S1f.6.4's T1. `add` has three rules and two tags, `add-first` and `add-second`, to look
its two operands up. Every further request needs the same.

- **(a) Repeat them:** three rules and two tag relations a request, 15 rules for the five requests.
- **(b) Share them:** one pair lookup, `(operands TAG X Y)` answered by `(operands-are TAG U V)`, three rules
  for all, and two a request to call it and go on. Every request costs two firings more, and addition's pinned
  circuit, 21 firings, moves to 23.

**Defaulted 2026-11-11: (a),** which is what S1f.6.3's last section tells a later stage to do. (b) is a
refactoring any later stage can make with no answer moving.

### Q-M1f.158 — `cl`'s column order and the index

**Decision file:** [`docs/decisions/stdlib/math/decimal/Q-M1f.158-the-comparators-table-keeps-the-order-below-as.md`](../../docs/decisions/stdlib/math/decimal/Q-M1f.158-the-comparators-table-keeps-the-order-below-as.md)

**Raised** 2026-11-11 by S1f.6.4's T2, on E-12 of the phase's `findings.md`. The matcher keys a premise on its
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

**Decision file:** [`docs/decisions/stdlib/math/decimal/Q-M1f.159-multiplication-is-called-with-a-tag.md`](../../docs/decisions/stdlib/math/decimal/Q-M1f.159-multiplication-is-called-with-a-tag.md)

**Raised** 2026-11-11 by S1f.6.5's T1. The prototype's multiplication carried the request's operands in
every step and wrote `product-of` when the multiplier ran out.

- **(a) A tag:** `(mul-call TAG U V)` is answered by `(done TAG P)`, and the public request calls it with
  `(product X Y)`. One firing more a request, 72 for 123 × 456.
- **(b) As the prototype.**

**Defaulted 2026-11-11: (a),** as for comparison (Q-M1f.156): S1f.6.8's rationals multiply inside their own
circuits.

### Q-M1f.160 — A zero digit of the multiplier

**Decision file:** [`docs/decisions/stdlib/math/decimal/Q-M1f.160-a-zero-digit-of-the-multiplier-is-a-row-like-any.md`](../../docs/decisions/stdlib/math/decimal/Q-M1f.160-a-zero-digit-of-the-multiplier-is-a-row-like-any.md)

**Raised** 2026-11-11 by S1f.6.5's T1.

- **(a) A row like any other:** the multiplicand times 0 is a walk of its length that strips to `z`, and `z` is
  added to ten times the sum. About four firings a digit of the multiplicand, for nothing.
- **(b) Two rules that skip it:** a zero digit multiplies the sum so far by ten and goes on. `mul-row` then
  needs `(nz ?a)`.

**Defaulted 2026-11-11: (a).** The answer is the same and the theory is two rules shorter; (b) is an
optimisation any later stage can make with no answer moving, and the circuit's pinned sizes would say what it
saves.

### Q-M1f.161 — The DOT sweep, several files at a time

**Raised** 2026-11-11 by S1f.6.5's T4, on Q-M1f.147 and Q-M1f.153. Every program that imports `std.arith`
holds the tables the theory reads, `md`'s 900 rows among them since this stage, so each of its six KB views is
some 9 000 edges and takes the bounded layout 2.2 s. The suite has eighteen such programs and S1f.6.6 adds more:
441 s of `dot` in one thread.

- **(a) Several files at a time:** `dot_wellformed.rs` shares the corpus files among the cores. 28 s on sixteen
  cores; about a quarter of the single thread's time on the runner's four. What is held is unchanged.
- **(b) Leave it:** four minutes more a gate now, and more with every arithmetic program.
- **(c) Draw a table as one node** (E-10's open question): a DOT writer's change, and the user's.

**Defaulted 2026-11-11: (a).** It changes no claim. (c) stays E-10's.

### Q-M1f.162 — The longest sampled operand of a product

**Decision file:** [`docs/decisions/corpus/Q-M1f.162-a-sampled-products-operands-have-at-most-19.md`](../../docs/decisions/corpus/Q-M1f.162-a-sampled-products-operands-have-at-most-19.md)

**Raised** 2026-11-11 by S1f.6.5's T2, on Q-M1f.154's oracle, `u128`, which holds 38 digits.

- **(a) 19 digits:** a product of two has at most 38.
- **(b) 30 digits, as the other operations,** against a multiplier written in the test.

**Defaulted 2026-11-11: (a),** for Q-M1f.154's reason: a length is not a boundary of the theory.

### Q-M1f.163 — Division called with a tag, and whose division by zero is flagged

**Decision file:** [`docs/decisions/stdlib/math/decimal/Q-M1f.163-a-division-by-zero-is-flagged-wherever-division.md`](../../docs/decisions/stdlib/math/decimal/Q-M1f.163-a-division-by-zero-is-flagged-wherever-division.md)

**Raised** 2026-11-11 by S1f.6.6's T1. Division is tagged as multiplication is (Q-M1f.159): `(div-call TAG U V)`
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

**Decision file:** [`docs/decisions/stdlib/math/decimal/Q-M1f.164-divisions-families-of-nine-rules-are-written.md`](../../docs/decisions/stdlib/math/decimal/Q-M1f.164-divisions-families-of-nine-rules-are-written.md)

**Raised** 2026-11-11 by S1f.6.6's T1, on Q-M1f.152, defaulted *by hand, while a rule is not a family of nine*.
Division has two: nine trial rules, one a multiple, and eight rules of the multiples' chain.

- **(a) By hand, held by a test:** the seventeen rules are in `arith.ein` as text, and `arith_theory.rs` holds
  that the rules of a family differ by their digit alone.
- **(b) Generated:** a script writes the two families, or the module, with a `--check` in the gate, as
  `std.digits` is.
- **(c) One rule:** not without a premise that joins on a number, or a change to the language.

**Defaulted 2026-11-11: (a).** The module stays one hand-written file a reader can follow, and what a
generator would guarantee the test holds.

### Q-M1f.165 — What search-free means of a program that divides by zero

**Decision file:** [`docs/decisions/stdlib/math/decimal/Q-M1f.165-a-program-that-divides-by-zero-is-decided.md`](../../docs/decisions/stdlib/math/decimal/Q-M1f.165-a-program-that-divides-by-zero-is-decided.md)

**Raised** 2026-11-11 by S1f.6.6's T2. The phase's claim is *H empty, k_s 1, R {∅}* on every arithmetic program, and
the user ruled a division by zero `(false)`: such a program's R is empty, k_s 0.

- **(a) Held to what is true of it:** no candidate, one saturation and no fork, monotone, R empty and decided
  by root alone, the request in the core, the second rule in `refuted by`. `arith_search_free.rs` takes this
  branch for a program that claims `(solve :expect (false))`.
- **(b) Kept out of the search-free roots:** the programs live elsewhere and the test's claim stays uniform.

**Defaulted 2026-11-11: (a).** Nothing is searched in either case, which is the claim; the count of states is
what the ruling changed.

### Q-M1f.166 — The sizes of the generated arithmetic examples

**Decision file:** [`docs/decisions/corpus/Q-M1f.166-the-generated-arithmetic-examples-are-sized-for.md`](../../docs/decisions/corpus/Q-M1f.166-the-generated-arithmetic-examples-are-sized-for.md)

**Raised** 2026-11-11 by S1f.6.15's T1. The plan asked for operands of some hundreds of
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

**Ruled 2026-11-12.** The user, reply 49: *"Let's have biggest solvable arithmetic calculation in let's say 100+-50
seconds on this host. make a generator in python that takes some size arguments and generate an example"*; and,
told in round 2's item 49 that memory ends a calculation before time does: *"Ok, then pick the one that fits into
memory, say ~16GB; do not call it in tests and in CI, just for manual solve call"* (`dc88520b`). So (a) stands for
the three shipped files, and the larger sizes are a generator's arguments and a manual run, never the corpus's.
S1f.6.29 did it, and closed on the user's word of 2026-11-13, *"runs way too long than I expected - just
write down generated examples and close"*: `utils/gen_arith_examples.py --op OP … -o FILE` writes a program of a
stated size, and `stdlib/README.md` § The biggest calculation has what was measured within 16 GB: under `ein
saturate` the longest written product, 999 digits by 999, 14.9 s and 6.13 GB, the parser's limit, and the cap
reached by 43 430 requests in 60.4 s and by 1 081! in 47.0 s; under `ein solve` a product of 547 digits by 547 and
a quotient of 626 by 313 at the 600 s limit, and 30 379 requests in 277.2 s at the cap, its search stopped there.

### Q-M1f.167 — The names of `std.num9`

**Decision file:** [`docs/decisions/stdlib/math/num9/Q-M1f.167-the-names-of-std-math-num9-are-decimals-with-a-9.md`](../../docs/decisions/stdlib/math/num9/Q-M1f.167-the-names-of-std-math-num9-are-decimals-with-a-9.md)

**Raised** 2026-11-11 by S1f.6.13's T2. The plan: *its relations are its own, `add9`, `sum9-of` and so on, so that a
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

**Decision file:** [`docs/decisions/stdlib/math/num9/Q-M1f.168-overflow-is-a-fact-and-no-number-and-a-division.md`](../../docs/decisions/stdlib/math/num9/Q-M1f.168-overflow-is-a-fact-and-no-number-and-a-division.md)

**Raised** 2026-11-11 by S1f.6.13's T1. The plan: overflow is a fact and no number, the design's recommendation, which
the user did not rule; a division by zero is as in `std.arith`.

- **(a) Overflow `(overflow REQUEST)`, root alive; a division by zero by `std.arith`'s relation and rule,** which
  the module imports: one flag relation and one rule that concludes `(false)` across both modules.
- **(b) A flag relation and a `(false)` rule of the module's own,** `division-by-zero9`: nothing imported from
  `std.arith` for it, and two rules of the stdlib that conclude `(false)`.
- **(c) Overflow as `(false)` too.** A program that overflows once then loses every other answer.

**Defaulted 2026-11-11: (a).** An overflow is an answer about a width, which a program may want to read and go on;
a division by zero asked for what does not exist, as ruled.

### Q-M1f.169 — Where `std.num9`'s multiples and rows live

**Decision file:** [`docs/decisions/stdlib/math/num9/Q-M1f.169-a-divisions-multiples-are-computed-into.md`](../../docs/decisions/stdlib/math/num9/Q-M1f.169-a-divisions-multiples-are-computed-into.md)

**Raised** 2026-11-11 by S1f.6.13's T3, which the plan left to it: *the multiples moved into the state if that removes
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

**Decision file:** [`docs/decisions/stdlib/math/num9/Q-M1f.170-a-num9-operand-is-a-number-written-out.md`](../../docs/decisions/stdlib/math/num9/Q-M1f.170-a-num9-operand-is-a-number-written-out.md)

**Raised** 2026-11-11 by S1f.6.13's T4. `std.arith` looks an operand up, so a request may name it; round 1's `num9` did
not.

- **(a) A number written out, and nothing else:** no `let` in the module. A conversion reads a nested number as
  written: high zeros count, and nine digits behind a zero are `big`.
- **(b) Named operands,** through `std.arith`'s `operand`: four firings a request where one, and the whole of
  `std.arith`'s lookup imported.
- **(c) A conversion by value:** `to9` strips high zeros first, a firing a zero.

**Defaulted 2026-11-11: (a).** The module's point is one firing. A program that wants a name writes a rule of its
own from the name to the request.

### Q-M1f.171 — `std.num9`'s step counter and its comparison

**Decision file:** [`docs/decisions/stdlib/math/num9/Q-M1f.171-std-math-num9-counts-and-compares-with-the-digit.md`](../../docs/decisions/stdlib/math/num9/Q-M1f.171-std-math-num9-counts-and-compares-with-the-digit.md)

**Raised** 2026-11-11 by S1f.6.13's T1. The plan said the module adds its own step counter; round 2's `num9` compared
by two subtractors.

- **(a) `std.digits`' tables:** `adj`, the pairs k and k + 1, counts the steps, and `cl`, the comparator, compares
  in one rule.
- **(b) A table of the module's own** for the steps, and two subtractors and a rule for equal numbers.

**Defaulted 2026-11-11: (a).** No fact is added to what every program loads. A comparison's lookup walks the
hundred rows an order keys (E-12), 901 candidates where two subtractors would walk some 360; it is one firing
either way.

### Q-M1f.172 — The harness of `std.num9`: exhaustive at width 2

**Raised** 2026-11-11 by S1f.6.13's T5. The plan: *exhaustive at width 2 on the generator's output and sampled at
width 9*. A width-2 theory is not a module of the stdlib.

- **(a) The generator's `--width 2 --stdout`, put in front of the test's requests** as the program's own
  declarations: 49 900 requests in one saturation.
- **(b) A second shipped module,** `std.num2`, which the coverage gate would then ask tests for.

**Defaulted 2026-11-11: (a).** The plan's *one width ships*.

### Q-M1f.173 — `MAX_REGS` and the wide rules of `std.num9`

**Decision file:** [`docs/decisions/engine/Q-M1f.173-the-register-file-stays-256-wide.md`](../../docs/decisions/engine/Q-M1f.173-the-register-file-stays-256-wide.md)

**Raised** 2026-11-11 by S1f.6.13's gate. A rule binds its variables to a register file of 256
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

**Decision file:** [`docs/decisions/stdlib/math/int/Q-M1f.174-the-names-of-std-math-int-carry-the-prefix-int.md`](../../docs/decisions/stdlib/math/int/Q-M1f.174-the-names-of-std-math-int-carry-the-prefix-int.md)

**Raised** 2026-11-12 by S1f.6.7's T1. The stage file fixes the
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

**Decision file:** [`docs/decisions/stdlib/math/int/Q-M1f.175-an-integer-operand-is-an-integer-written-out.md`](../../docs/decisions/stdlib/math/int/Q-M1f.175-an-integer-operand-is-an-integer-written-out.md)

**Raised** 2026-11-12 by S1f.6.7's T1.

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

**Decision file:** [`docs/decisions/stdlib/math/int/Q-M1f.176-a-product-of-integers-is-computed-on-the-parts.md`](../../docs/decisions/stdlib/math/int/Q-M1f.176-a-product-of-integers-is-computed-on-the-parts.md)

**Raised** 2026-11-12 by S1f.6.7's T1. The stage file gives both:
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

**Decision file:** [`docs/decisions/stdlib/math/Q-M1f.177-a-module-over-the-decimal-arithmetic-tags-each.md`](../../docs/decisions/stdlib/math/Q-M1f.177-a-module-over-the-decimal-arithmetic-tags-each.md)

**Raised** 2026-11-12 by S1f.6.7's T1. Two choices about how a
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

**Decision file:** [`docs/decisions/corpus/Q-M1f.178-std-math-int-s-harness-is-a-file-of-its-own.md`](../../docs/decisions/corpus/Q-M1f.178-std-math-int-s-harness-is-a-file-of-its-own.md)

**Raised** 2026-11-12 by S1f.6.7's T3. The stage file: *every sign
combination at the exhausting width*, written when the numbers under it had a width.

- **(a) A sibling of `arith_theory.rs`,** `int_theory.rs`, with its own `OPS` against `i128`; the exhausting set
  is every integer written with parts from 0 to 9, 100 spellings of −9 to 9, against every other.
- **(b) Rows of `arith_theory.rs`'s `OPS`,** whose harness reads operands as `u128` and a module's text as
  `stdlib/arith.ein`.

**Defaulted 2026-11-12: (a).** The three static tests read one module's text each, and an integer operand is
not a `u128`.

### Q-M1f.179 — The remainder of an integer division

**Decision file:** [`docs/decisions/stdlib/math/int/Q-M1f.179-integer-division-names-its-convention.md`](../../docs/decisions/stdlib/math/int/Q-M1f.179-integer-division-names-its-convention.md)

**Raised** 2026-11-12 by S1f.6.7, whose stage file put it to the
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
the stage's T2 record
(`python3 build/s1f.6.7/intdiv_proto.py`, 2026-11-12). With (a) there is a second question: whether R is
answered as an integer, `(int R z)`, or as a number of `std.arith`. S1f.6.8 does not wait for this: the rationals
never divide integers.

**Ruled 2026-11-12, and built** (S1f.6.22). The user, on
questions 1 and 2 of the calculator's `questions.md`: *"(d)"*, and *"(a)"*, a natural number. There is no bare
`int-div`: `(int-div-euclid X Y)`, `(int-div-trunc X Y)` and `(int-div-floor X Y)`, answered by
`int-quot-euclid-of`, `int-quot-trunc-of` and `int-quot-floor-of`
([Q-M1f.200](#q-m1f200--the-names-of-an-integer-divisions-results)).

### Q-M1f.180 — The names of `std.rat`

**Decision file:** [`docs/decisions/stdlib/math/rat/Q-M1f.180-the-names-of-std-math-rat-carry-the-prefix-rat.md`](../../docs/decisions/stdlib/math/rat/Q-M1f.180-the-names-of-std-math-rat-carry-the-prefix-rat.md)

**Raised** 2026-11-12 by S1f.6.8's T1. The stage file fixes the pair, `(rat Z D)`, and no other name.

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

**Decision file:** [`docs/decisions/stdlib/math/rat/Q-M1f.181-a-rational-operand-is-a-rational-written-out.md`](../../docs/decisions/stdlib/math/rat/Q-M1f.181-a-rational-operand-is-a-rational-written-out.md)

**Raised** 2026-11-12 by S1f.6.8's T1.

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

**Decision file:** [`docs/decisions/stdlib/math/rat/Q-M1f.182-a-zero-denominator-is-a-division-by-zero.md`](../../docs/decisions/stdlib/math/rat/Q-M1f.182-a-zero-denominator-is-a-division-by-zero.md)

**Raised** 2026-11-12 by S1f.6.8's T1. The stage file: *a zero denominator, written by a program or reached by
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

**Decision file:** [`docs/decisions/stdlib/math/rat/Q-M1f.183-a-rational-is-reduced-only-on-request.md`](../../docs/decisions/stdlib/math/rat/Q-M1f.183-a-rational-is-reduced-only-on-request.md)

**Raised** 2026-11-12 by S1f.6.8's T1 and T3.

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

**Ruled 2026-11-12, round 2's item 54: *"(b)"*** — not one of the three options above but *reduce only on request*
(`questions_round2.md` § 54; the user's question there, *"I think we
don't need to reduce every rational, can ask explicitely for it?"*). **Built by
S1f.6.33, 2026-11-15:** an operation answers
unreduced, `rat-simplify` alone reduces, by `std.math.gcd`'s tagged call, and `std.math.rat` holds no chain of its
own; 1/2 + 1/3 is 101 firings, 2/3 × 3/4 81, and 2/4 simplified 345.

### Q-M1f.184 — The harness of `std.rat`, and the sizes memory allows

**Decision file:** [`docs/decisions/corpus/Q-M1f.184-std-math-rat-s-harness-exhausts-28-fractions.md`](../../docs/decisions/corpus/Q-M1f.184-std-math-rat-s-harness-exhausts-28-fractions.md)

**Raised** 2026-11-12 by S1f.6.8's T3. The stage file: *every operand pair at the exhausting width*. A request
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

**Decision file:** [`docs/decisions/engine/Q-M1f.185-a-state-deeper-than-the-parsers-limit.md`](../../docs/decisions/engine/Q-M1f.185-a-state-deeper-than-the-parsers-limit.md)

**Raised** 2026-11-12 by S1f.6.9, whose stage file put the choice to the user
from the start. **Open: the user's, and not defaulted.** It is E-4 of the phase's
`findings.md`, re-taken on the shipped theory.

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
the stage's T1 record.

### Q-M1f.186 — Where the probes of S1f.6.10 are banked

**Decision file:** [`docs/decisions/corpus/Q-M1f.186-what-the-calculators-tables-do-besides.md`](../../docs/decisions/corpus/Q-M1f.186-what-the-calculators-tables-do-besides.md)

**Raised** 2026-11-12 by S1f.6.10. Its stage file says *banked
under `examples/arith/`*. `ein-cli/tests/arith_search_free.rs` holds every `.ein` under that directory, at any
depth, to the search-free claims: it imports a theory of the calculator, the universe has no candidate, R is
{∅}, one saturation, and a result fact of the calculator is in the state. Four of the banked probes search, and
four import the tables alone.

- **(a) A directory beside it,** `examples/beyond_arith/`, which that test does not walk.
- **(b) Under `examples/arith/`,** with the test told which files search, by a list or a marker in the file.

**Defaulted 2026-11-12: (a).** It moves no test, and the directory's name is the page's. It is question 8 of
[`08-word-problems.md`](../../docs/ein/user/02-guide/08-word-problems.md).

### Q-M1f.187 — Which probes of S1f.6.10 are banked, and at what size

**Decision file:** [`docs/decisions/corpus/Q-M1f.187-each-program-beyond-arithmetic-is-sized-for.md`](../../docs/decisions/corpus/Q-M1f.187-each-program-beyond-arithmetic-is-sized-for.md)

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

**Decision file:** [`docs/decisions/stdlib/math/Q-M1f.189-the-decimal-arithmetics-tagged-calls-are.md`](../../docs/decisions/stdlib/math/Q-M1f.189-the-decimal-arithmetics-tagged-calls-are.md)

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
[`08-word-problems.md`](../../docs/ein/user/02-guide/08-word-problems.md), with the recommendation that the module's page
name the tagged calls as what a theory built on it calls. Eight programs under `examples/beyond_arith/` would
move if a tagged call's shape did; `arith_theory.rs` does not hold those shapes for outside callers.

**Answered 2026-11-12: the tagged calls are an interface.** Question 41 of
`questions.md` got *"I don't understand the question"*; round 2's item
41 explained it, and the user's reply there, committed as `dc88520b`, is
*"(a)"*. **Built 2026-11-13** by S1f.6.30's
T1: `std.arith`'s header and [`stdlib/README.md`](../../stdlib/README.md) name the five calls and their four
answers as an interface, `std.int`'s and `std.rat`'s calls likewise, and `ein-infer/tests/arith_theory.rs`'s
`the_tagged_calls_keep_their_shapes` holds the declarations, the pages and every caller in the stdlib,
`examples/` and `tests/` to their shapes. What it holds beyond the seven names round 2 showed is
[Q-M1f.228](#q-m1f228--what-the-interface-of-the-tagged-calls-holds-beyond-the-seven-names-round-2-showed).

### Q-M1f.190 — Which variants of the binary prototypes are banked where the page recommends none

**Decision file:** [`docs/decisions/corpus/Q-M1f.190-each-circuit-example-takes-the-simplest-form.md`](../../docs/decisions/corpus/Q-M1f.190-each-circuit-example-takes-the-simplest-form.md)

**Raised** 2026-11-12 by the banking of S1f.6.16's and
S1f.6.17's prototypes as examples (the user, that day: *"bank binary
arithmetic and logic prototypes into examples too"*). Where a page recommends a variant, that one is banked: way
(b) and its generation from `std.arith` for arithmetic; the one table, and the properties as generated facts with
the deriving rules as their check, for logic. On four points the pages show variants and recommend none:

- **The truth table's candidates.** [`04-stdlib/18-boolean-functions.md`](../../docs/ein/user/04-reference/04-stdlib/18-boolean-functions.md#using-stdboolinputs--inputs-as-hypotheses) shows three ways,
  the obligations rung alone, an hrule, and `:hypothesis-relations`, with one answer and one cost.
- **Where the conversions live,** in the binary module or a module of their own: question 5 of
  [`04-stdlib/14-binary.md`](../../docs/ein/user/04-reference/04-stdlib/14-binary.md).
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

**Superseded 2026-11-15** at P1f.6's close, on the user's ruling of that day (*"all as you recommended"*, `close.md` § The user's rulings, 3): `gen_binary.py` retired (`c2902aca`), and `utils/gen_gates.py` is in the gate (`bool_theory.rs`). The question has no subject since; the default stands as the record of what was built before.

### Q-M1f.192 — One table file in two example directories

**Decision file:** [`docs/decisions/stdlib/math/Q-M1f.192-a-program-that-reads-the-binary-tables-imports.md`](../../docs/decisions/stdlib/math/Q-M1f.192-a-program-that-reads-the-binary-tables-imports.md)

**Raised** 2026-11-12 by the same banking. Two programs of `examples/binary_logic/` set a circuit against the
binary full adder's table, `fa2`, which is `examples/binary_arithmetic/bits.ein`'s. A file-relative import
resolves against the importing file's directory and below it, so the one cannot import the other's module (the
finding is in the phase's `findings.md`).

- **(a) A copy,** `examples/binary_logic/bits.ein`, written by `gen_binary.py` with the original, whose `--check`
  compares both.
- **(b) One directory** for both stages' examples.
- **(c) The eight rows written again** in each program that reads them.

**Defaulted 2026-11-12, the user's to revise: (a).** The user named the two directories, and (c) would set the
gates against rows typed beside them and not against the calculator's table.

**Superseded 2026-11-15** at P1f.6's close, on the user's ruling of that day (*"all as you recommended"*, `close.md` § The user's rulings, 3): the copy retired, both directories importing `std.math.bits` (`c2902aca`). The question has no subject since; the default stands as the record of what was built before.

### Q-M1f.193 — The sizes of the banked binary programs

**Decision file:** [`docs/decisions/corpus/Q-M1f.193-the-binary-examples-are-sized-for-every-command.md`](../../docs/decisions/corpus/Q-M1f.193-the-binary-examples-are-sized-for-every-command.md)

**Raised** 2026-11-12 by the same banking. Every corpus program is solved, dumped and reloaded per commit by a
bare `ein solve`, which describes all of R and writes each state as text (E-24); describing R over n inputs as
hypotheses is 3ⁿ + n saturations and stops at the default budget near twelve inputs
([`04-stdlib/18-boolean-functions.md`](../../docs/ein/user/04-reference/04-stdlib/18-boolean-functions.md#using-stdboolinputs--inputs-as-hypotheses)).

- **(a) Sizes every surface affords:** three inputs for every search, 30 saturations at most; binary operands
  to seventeen bits; five numbers to binary, the largest 255, and seven to decimal, the largest 1000.
- **(b) The pages' headline sizes,** marked `slow`.

**Defaulted 2026-11-12, the user's to revise: (a).** No banked program is `slow`. The batches of 250 requests,
the conversion of 2^40 + 1, the scale runs to 24 inputs, Post's criterion over all 65 535 sets and the
three-input classes stay under `build/s1f.6.16/` and `build/s1f.6.17/`, with their numbers on the pages.

### Q-M1f.194 — Which of the word-problem prototype's programs are banked, and under what names

**Decision file:** [`docs/decisions/corpus/Q-M1f.194-each-word-problem-is-one-file-written-in-one.md`](../../docs/decisions/corpus/Q-M1f.194-each-word-problem-is-one-file-written-in-one.md)

**Raised** 2026-11-12 by S1f.6.20, on the user's instruction of that day: *"bank word problems into
examples/word_problems with recommended variants"*. The prototype is 29 programs and nine modules, most of them
one problem written under several variants ([`08-word-problems.md`](../../docs/ein/user/02-guide/08-word-problems.md#the-problems) § 0).

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

**Decision file:** [`docs/decisions/corpus/Q-M1f.195-a-problem-that-asks-what-is-needed-imports.md`](../../docs/decisions/corpus/Q-M1f.195-a-problem-that-asks-what-is-needed-imports.md)

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

**Decision file:** [`docs/decisions/corpus/Q-M1f.196-each-module-of-the-word-theory-is-a-corpus-entry.md`](../../docs/decisions/corpus/Q-M1f.196-each-module-of-the-word-theory-is-a-corpus-entry.md)

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

**Decision file:** [`docs/decisions/corpus/Q-M1f.197-the-corpus-holds-the-word-problems-answers.md`](../../docs/decisions/corpus/Q-M1f.197-the-corpus-holds-the-word-problems-answers.md)

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

**Decision file:** [`docs/decisions/corpus/Q-M1f.198-the-thirteen-apple-problem-is-a-corpus-program.md`](../../docs/decisions/corpus/Q-M1f.198-the-thirteen-apple-problem-is-a-corpus-program.md)

**Raised** 2026-11-12 by S1f.6.20. The page's recommended form of a missing reading is `readings_plain`:
readings as hypotheses, nothing owed, and under a supposed reading the answer is a `value`, so that a later
quantity follows from it by the same rules. Its two problems, 13 apples in boxes of 3 and the same with the
reading stated, answer on today's engine as the page reports. `ein-infer/tests/region.rs`'s
`fact_grain_never_calls_a_violated_cube_monotone` does not end with either in the corpus: it ran 13 263 s on one
thread before it was stopped, where it takes 20 s without them
(E-33, `cargo test -p ein-infer --test region fact_grain_never`, 2026-11-12).
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

**Answered 2026-11-13** by S1f.6.28,
on the user's reply to round 2's item 31 (*"yes"*: the examples rebuilt to *always owed*, the 13-apple variant
back once the test ends on it, its meaning unchanged). Its T1 made the fact-grain closure run only where a
relation-grain site reads it, which no unflagged program has, and give up on the safe side on a flagged one, where
the test fails naming the program ([Q-M1f.225](#q-m1f225--what-the-fact-grain-test-does-where-the-closure-gives-up)):
every verdict of the corpus and the 437 cubes it read were the same after. Its T5 banked
`examples/word_problems/boxes13.ein`, two states and two answers, 4 and 5, the readings owed; the test ends on it,
`cargo test -p ein-infer --test region` 40.60 s, and reads 441 cubes. None of (b) to (d) was taken: (d)'s *list
of files it skips* was not acceptable, and (d)'s *cap* is taken only as a failure, never as a narrower test.

### Q-M1f.199 — How the skipped views of the DOT sweep are pinned

**Raised** 2026-11-12 by S1f.6.21's T2, which asks that the
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

**Decision file:** [`docs/decisions/stdlib/math/int/Q-M1f.200-an-integer-divisions-result-is-named-after.md`](../../docs/decisions/stdlib/math/int/Q-M1f.200-an-integer-divisions-result-is-named-after.md)

**Raised** 2026-11-12 by S1f.6.22, whose stage file names it.
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
S1f.6.31 moves the modules and renames no relation.

### Q-M1f.201 — What a bare `(relation R)` declares of its count

**Decision file:** [`docs/decisions/language/Q-M1f.201-a-bare-relation-r-declares-no-argument.md`](../../docs/decisions/language/Q-M1f.201-a-bare-relation-r-declares-no-argument.md)

**Raised** 2026-11-12 by S1f.6.23, whose T1 names it: the ruling holds a form to *the count declared*, and a
declaration with no type atom, `(relation flag)`, is legal
([`02-syntax.md` § Relation](../../docs/ein/user/04-reference/02-syntax.md#relation-declarator)): *a relation
node with no declared arg types*.

- **(a) It declares none,** so `(flag)` is its one form and `(flag up)` is refused. The count is the count of
  types, with no special case.
- **(b) It declares no count,** so a bare relation's forms are not held, as an undeclared relation's are not.

**Defaulted 2026-11-12: (a), the user's to revise.** Every bare declaration of the tracked `.ein` files whose
program loads is used with no argument, `clash`, `marker`, `z` (`git ls-files '*.ein' | xargs grep -nE
'^\(relation [^ )]+( +:why|\))'`, 2026-11-12), and so is every one in the Rust suites but `oops` of
`rule_semantics.rs`, which was declared `(relation oops T)` and written `(oops)`. Under (b) a flag a theory declares bare would be the one kind of declared
relation a misspelt count passes.

### Q-M1f.202 — A relation written through a parameter, and the declared count

**Decision file:** [`docs/decisions/language/Q-M1f.202-a-relation-written-as-a-variable-is-held-to-no.md`](../../docs/decisions/language/Q-M1f.202-a-relation-written-as-a-variable-is-held-to-no.md)

**Raised** 2026-11-12 by S1f.6.23, whose T1 names it. `(rule mirror (?R) :match (?R ?a ?b) :assert (?R ?b ?a))`
writes its relation as a variable, and `(mirror likes)` binds it.

- **(a) Not read.** A rule over a parameter is generic over counts: matching is by count, so `(?R ?a)` under
  `(mirror likes)` matches nothing, which is what a generic rule means by it, and the stdlib's fan-outs pass
  relations of several counts to one rule.
- **(b) Read where the program shows what the parameter holds,** as S1f.5.18's check reads relation names off
  the written activators, a rule's conclusions to a fixpoint and the `(solve …)` lists: a premise `(?R ?a)`
  whose `?R` can hold `likes`, declared with two, is refused.

**Defaulted 2026-11-12: (a), the user's to revise.** Under (b) a generic rule is refused for one activator of
many, where today it matches nothing for that one; the probe `build/s1f.6.23/p/param.ein` loads and derives
nothing, old binary and new. What (a) leaves is a *conclusion* through a parameter, which derives a fact of a
declared relation at another count, `build/s1f.6.23/p/param3.ein`: E-35 of the phase's
`findings.md`.

### Q-M1f.203 — The kernel's own relations, and an activator's count

**Decision file:** [`docs/decisions/language/Q-M1f.203-a-kernel-marker-and-an-activator-are-held-to-no.md`](../../docs/decisions/language/Q-M1f.203-a-kernel-marker-and-an-activator-are-held-to-no.md)

**Raised** 2026-11-12 by S1f.6.23, whose T1 names both. No program declares `__closed__`, `__symmetric__` or
`relation`, each reserved, and an activator `(functional r)` is a fact headed by a rule's name, held to the
rule's parameters and to no declaration.

- **(a) Neither is held by this stage.** The ruling says *declared relation*. `(relation R ?A ?B)` stays a
  pattern at any count, which is how a rule reads a signature. `eq` is held to two by `defined_behaviour.md`
  § 4.14.
- **(b) Both are held:** the two dunders to one argument, and an activator to its rule's count of parameters,
  as a `(solve :hrules …)` activator is since S1f.7.8.

**Defaulted 2026-11-12: (a), the user's to revise**, as the stage file has it for the activator. What (a)
leaves is E-34 and E-35 of the phase's `findings.md`, each with its probe:
`(sym likes extra)` activates nothing in silence, and `(__closed__ likes extra)` is stored and closes nothing.

### Q-M1f.204 — Which written places the declared count is held in

**Decision file:** [`docs/decisions/language/Q-M1f.204-the-declared-count-is-held-in-a-using-a-claim.md`](../../docs/decisions/language/Q-M1f.204-the-declared-count-is-held-in-a-using-a-claim.md)

**Raised** 2026-11-12 by S1f.6.23. The ruling's *in a fact, a rule or a query* leaves four places to decide.

- **A fact's `:using`,** `(seen Bob :rule r :using ((likes Bob)))`: **held.** Its premises are facts, written
  as a nested fact is. Whether a `:using` may name a fact no state holds is
  [F24](../followups/f24_written_provenance.md)'s, W87, and is not touched.
- **A claim's listed fact,** a query's `:expect` and `(solve … :expect …)`'s: **held.** A claim's own words,
  `model`, `region`, `merged`, `owing` and `or`, are its shape and no relation's form, so a program that
  declares a relation `model` is not refused for its claim.
- **A macro's body: held where a rule or a goal calls it,** on the expansion, and not read where nothing
  calls it, as S1f.5.24 has it for a malformed primitive.
- **A `(solve …)` list: not read.** `:hrules (guess (at Color House))` and `:hypothesis-relations (likes
  fan)` are lists of names the parser builds as forms; S1f.7.8 holds an activator there to its hrule.

**Defaulted 2026-11-12, each as written above, the user's to revise.** Each is a case of
`ein-cli/tests/declared_counts.rs`.

### Q-M1f.205 — Where the check of a malformed number runs

**Raised** 2026-11-13 by S1f.6.12's
build, T1f.6.12.4. The user's reply 4 (a) of 2026-11-12 was to build the investigation's set *as a checking
pass of its own at the operand, the cheaper placement measured by the build before it is chosen*.

Measured on two prototypes of the whole set (`utils/bench_env.sh python3 build/s1f.6.12/build/cost2.py`,
2026-11-13; the record has the table): **the pass** costs a firing a digit of each operand, whatever the
operation; **the mixed placement**, guards on the walks of a sum, a difference and a comparison and the pass
for a product and a quotient, costs no firing for the first three and is dearer than the pass for the last
two, 4 913 ms where 4 406 for 4 000 products, since every internal walk of a product then carries the
guards. Neither is cheaper for every operation. The mixed one is 58 rules where the pass is 26, flags an
internal tag and not the request where a walk is a module's own (`std.int`'s, `std.rat`'s), and leaves an
operand that no walk reads unchecked, `(mul X z)`.

**Defaulted 2026-11-13, the user's to revise: the pass.** One mechanism for five operations; the stage
file's cost table and acceptance are the pass's. `operand-number` and `operand-name` conclude the `check`
fact beside `operand-is`, which is a firing an operand less than a rule of its own and leaves the operands
of `std.int` and `std.rat`, which those modules find themselves, without an inert fact.

### Q-M1f.206 — How a fault reaches its request

**Decision file:** [`docs/decisions/stdlib/math/decimal/Q-M1f.206-an-operands-fault-travels-with-its-tag.md`](../../docs/decisions/stdlib/math/decimal/Q-M1f.206-an-operands-fault-travels-with-its-tag.md)

**Raised** 2026-11-13 by S1f.6.12's build. A fault is found at an operand's tag, `(add-first X Y)`, and the
flag names the request, `(add X Y)`. With two kinds of flag, `malformed` and `undeclared`, the
investigation's ten rules from a tag to a request would be twenty.

**Defaulted 2026-11-13, the user's to revise:** the fault travels with the tag,
`(bad-operand TAG (not-a-number THING))`, THING what stands where a digit or a number goes, or `(bad-operand TAG (no-let NAME))`; ten rules, alike but for the
operation, conclude `(bad-request REQUEST FAULT)`; `bad-request-malformed` and `bad-request-undeclared`
conclude the two flags. Twelve rules where twenty, at one firing more on a flagged request and none on a
well-formed one. It adds three relations of the circuit's own, `bad-request`, `not-a-number` and `no-let`.
The fault is a fact and not a bare name, so no rule concludes a constant: a name a rule asserts is a name
of every program that imports the theory, and `alive_invariant.rs` lists each.

### Q-M1f.207 — What a `let` that cannot be read flags

**Decision file:** [`docs/decisions/stdlib/math/decimal/Q-M1f.207-a-let-that-cannot-be-read-is-flagged-whether.md`](../../docs/decisions/stdlib/math/decimal/Q-M1f.207-a-let-that-cannot-be-read-is-flagged-whether.md)

**Raised** 2026-11-13 by S1f.6.12's build. The stage file says a detected *request* derives
`(malformed REQUEST)`; a name stated twice and `z` or a digit as a name are faults of a `let`.

**Defaulted 2026-11-13, the user's to revise: the flag names the `let`,** `(malformed (let NAME NUMBER))`,
as the investigation's prototype did, and is derived whether or not a request uses the name, since the
rules read `let` and no operand. A name stated twice flags each of its two `let`s. A request that uses such
a name is still answered in root's state, twice for a name stated twice, and root is dead, so no query reads
either. **`(let 0 N)` is flagged too** (`let-digit-zero`): the investigation's `let-digit` probes `nz`,
which has no row for 0. One consequence, stated in `35_let_zero.ein`: with `(let z N)` in a program `z` is a
named constant, so every number of every request ends in one and is flagged as well.

### Q-M1f.208 — How the theory says that an operand is a name

**Decision file:** [`docs/decisions/stdlib/math/decimal/Q-M1f.208-two-guards-say-that-an-operand-is-a-name.md`](../../docs/decisions/stdlib/math/decimal/Q-M1f.208-two-guards-say-that-an-operand-is-a-name.md)

**Raised** 2026-11-13 by S1f.6.12's build, on round 2's item 5 (b). The premise `(absent (let ?x ?w))`
holds of a number too, which is a fact no `let` names, so the rule must also say that `?x` is a *name*, and
no pattern says *not of the shape `(n P D)`*.

**Defaulted 2026-11-13, the user's to revise:** `(neq ?v z) (eq ?v ?x)`, `?v` bound by nothing else: the
first guard ranges `?v` over the program's names but `z`, the second keeps the one that is the operand
(`01_grammar.md`: a variable nothing binds *is bound by the guard, where it is written*). The two guards
mean something else in the other order, which is E-37 of the phase's `findings.md`, shown. Ten more guards
leave the digits out, so a digit as an operand has one flag, `malformed`. **The cost is a comparison a name
of the program, twice, for each operand of each request**, a number too, since the guards run before the
`absent` whatever order they are written in: 1 000 sums take 69.7 ms beside no named constant and 140.7 ms
beside 4 000, where the theory before the stage took 36.6 and 41.3 (E-36). It is flat in the batch and grows
with the program's names; `arith_batch.rs` pins it. The alternative found, one fact a name of the program
derived once, makes the test one probe and puts some three hundred facts into every arithmetic state.

### Q-M1f.209 — The binary example theory and the check

**Raised** 2026-11-13 by S1f.6.12's build. `examples/binary_arithmetic/bin.ein` is generated from
`stdlib/arith.ein`, and the generator carried the new rules over by name: ten `neq` guards on the decimal
digits and probes of `nz`, a table the binary theory does not have.

**Defaulted 2026-11-13, the user's to revise: `gen_binary.py` leaves the check out,** rules and relations,
and takes the two rules of the operands as they were, so `bin.ein` is byte for byte what it was and the
binary theory answers a malformed request by silence. A check of base two is two guards where ten and is
[`04-stdlib/14-binary.md`](../../docs/ein/user/04-reference/04-stdlib/14-binary.md)'s to ask, with the page's other questions.

**Superseded 2026-11-15** at P1f.6's close, on the user's ruling of that day (*"all as you recommended"*, `close.md` § The user's rulings, 3): binary's numeral is `std.math.binary`'s, checked since S1f.6.16's build. The question has no subject since; the default stands as the record of what was built before.

### Q-M1f.210 — `std.int` and `std.rat`, and the flags

**Decision file:** [`docs/decisions/stdlib/math/Q-M1f.210-a-malformed-number-is-its-representations-fault.md`](../../docs/decisions/stdlib/math/Q-M1f.210-a-malformed-number-is-its-representations-fault.md)

**Raised** 2026-11-13 by S1f.6.12's build, T1f.6.12.9, whose task was to check the two modules and not to
change them. **Open, shown; nothing built.**

What fires there, on 26 probes (`build/s1f.6.12/build/t9/`, the record has the table): a `let` that cannot
be read is flagged and is `(false)`, as in `std.arith`, since those rules read `let` alone. An undeclared
name and a digit as an operand are *seen*: both modules look a named operand up through `std.arith`'s
`operand`, so its rules derive `(bad-operand (int-first REQUEST Y) (no-let foo))`; but no rule turns a tag
of theirs into a request, so there is no flag and no `(false)`. A malformed number inside an integer or a
rational is not seen at all: the parts go to `std.arith`'s walks and never through its operand.

- **(a) As it is:** silent, named on the pages. *The stage file's default.*
- **(b) Three rules in each module,** from its tags to `(bad-request REQUEST FAULT)`: an undeclared name
  and a digit as an operand are then flagged, and `(false)`. Cheap, and it leaves a malformed part silent.
- **(c) (b), and each part of an integer and a rational handed to `std.arith`'s check,** a firing a digit
  of each part, as a natural number's operand costs.

**Ruled 2026-11-14: (c), with a principle that places it.** The user: *"yes (c), but have to notice that
malformed decimal numbers is a problem of decimal module, malformed binary numbers - of binary module, not
arithmetic, integer or rationals or whatever else. If there is a malformed number, I expect it has to be detected
right away, without need to start calculations with it."* So the (b) part, an undeclared name and a digit as an
operand, is int's and rat's; a malformed numeral, wherever it is written, is its representation's, seen from what
was written and not handed along by a calculation. S1f.6.38 designed it (T1, `6cca4406`) and builds it,
on the user's rulings of the same day, Q-M1f.272 to .277.

**Closed 2026-11-14 by S1f.6.38's build** (record):
the loader states every fact nested in what a file writes as `(written T)`; `std.math.decimal`, a module of its own,
declares `n` and flags a malformed numeral, `(not-decimal NUMERAL THING)` and `(false)`, wherever a file writes it,
with no request reading it; `std.math.int` and `std.math.rat` flag an undeclared name and a digit as an operand on
the request, and a digit or a named constant in their own terms; no well-formed program's answer moved.

**Reverted 2026-11-18** by [Q-M1f.404](#q-m1f404--written-is-erased-a-numeral-is-checked-by-the-operation-that-takes-it): `written` is erased, and a numeral is checked by the operation that takes it.

### Q-M1f.211 — Which fact carries the flag of a division by zero at the call

**Decision file:** [`docs/decisions/stdlib/math/decimal/Q-M1f.211-a-public-division-by-zero-holds-two-flags.md`](../../docs/decisions/stdlib/math/decimal/Q-M1f.211-a-public-division-by-zero-holds-two-flags.md)

**Raised** 2026-11-13 by S1f.6.24's T1, as the stage file's design question. The user's
ruling of 2026-11-12, *"(div ?x 0) must emit division-by-zero no matter where it happens"*, moves the
detection to `(div-call TAG X Y)`; it does not say what the flag names.

- **(i) The call alone,** `(division-by-zero (div-call TAG X Y))`. One rule. A public division's flag then
  reads `(division-by-zero (div-call (quotient X Y) U V))` where it read `(division-by-zero (div X Y))`:
  `ein solve` is unmoved (the core is the written request, `refuted by` the same rule), and the flag in
  `--trace` and in `ein saturate --dump` is respelt.
- **(ii) The call, and the public request as it was.** Two flags for one public division; `ein solve` is
  unmoved; `--trace` shows two steps more and `ein saturate --dump` one fact more.
- **(iii) The caller's request, through the tag:** the theory derives a marker that is no flag,
  `(div-zero TAG X Y)`, and each caller writes a rule from its tag to its request's flag. A caller that
  writes none stays alive and silent, which is what the ruling overruled.

**Defaulted 2026-11-13: (ii), the user's to revise.** It is the stage file's default: no text a program
read before moves. Two details the stage settled inside it. **The request's flag is derived first and
from the walk,** `div-by-zero` written before `div-call-by-zero` and both matching the divisor's normal
form, so that a trace reads the request's flag, then the call's, then the two contradictions; deriving the
request's flag *from* the call's reverses that order and reads no better. **The call's flag repeats the
call as it was asserted,** the divisor as the caller wrote it, `(n (n z 0) 0)` and not `z`, so the flag
names a fact that is in the state: for that the first multiple's tag holds the divisor beside the
dividend, `(div-m1 TAG X Y)`, which respells that walk's facts in every division's state and adds none.

### Q-M1f.212 — A theory's wrong answer and the register

**Raised** 2026-11-13 by S1f.6.24's T2. Until the stage a rule of a program's own that
called `(div-call TAG X z)` derived nothing and the program was alive, which the user ruled wrong.

- **(a) Not a register row.** `corpus/wrong_answers.toml` holds the *engine's* wrong answers; here the
  engine answered the program as the stdlib's rules said, and what was wrong was a rule of a theory. The
  witness is a test program, `tests/stdlib/arith/39_div_call_by_zero.ein`, and the record's run of it on
  the stdlib before the change.
- **(b) A row,** filed `open` before the fix, with a schema that can say *a theory's*.

**Defaulted 2026-11-13: (a), the user's to revise,** as the stage file has it. S1f.6.12 did the same for
the malformed requests answered by silence.

### Q-M1f.213 — The binary example theory and the flag at the call

**Raised** 2026-11-13 by S1f.6.24. `examples/binary_arithmetic/bin.ein` is generated from
`stdlib/arith.ein`, keeps `div-by-zero` and `division-by-zero-is-false`, and has a tagged division of its
own, `div-call2`, which `conv.ein` calls.

- **(a) The generator carries the new rule over,** as it carries the two beside it: `div-call-by-zero2`,
  68 rules where 67, and the first step's tag holding the divisor as written.
- **(b) The generator leaves it out,** as it leaves S1f.6.12's check out (Q-M1f.209), and `bin.ein` stays
  byte for byte what it was.

**Defaulted 2026-11-13: (a), the user's to revise.** The ruling is about division by zero wherever it
happens, and the rule knows nothing of the base, which is the generator's test for what it keeps; the
check was left out because its rules name the ten digits. No binary program's `ein solve` moved.

**Superseded 2026-11-15** at P1f.6's close, on the user's ruling of that day (*"all as you recommended"*, `close.md` § The user's rulings, 3): the rule is `std.math.positional`'s, shared by both bases. The question has no subject since; the default stands as the record of what was built before.

### Q-M1f.214 — The *Derives* line of a firing with several conclusions

**Decision file:** [`docs/decisions/read-out/Q-M1f.214-a-firings-derives-line-names-every-conclusion.md`](../../docs/decisions/read-out/Q-M1f.214-a-firings-derives-line-names-every-conclusion.md)

**Raised** 2026-11-13 by S1f.6.25's T1 (E-5). The stage file says the line *lists every conclusion the
firing stored, in the order the rule writes them*, and not how a line of several reads.

- **(a) One line,** `; ` between two conclusions, each followed by its own statement where its relation
  has a `:why`: *Derives `q(a)` — a is q; `flag(a)`; `r(a, a)` — a is related to a*. A rule of one
  conclusion prints the line it always did, so no golden of such a rule moves.
- **(b) A line a conclusion,** each a *Derives* of its own. Every step of a multi-conclusion rule grows,
  and a reader of the file who counts *Derives* lines no longer counts steps.
- **(c) A list under the word.**

**Defaulted 2026-11-13: (a), the user's to revise.** It is the stage file's word, *the line*, and
`--reorder`'s arrow lists the same conclusions with commas, having no statements to carry.

### Q-M1f.215 — A qualified import and what a module states as a program

**Decision file:** [`docs/decisions/imports/Q-M1f.215-no-import-brings-a-modules-query-solve-config.md`](../../docs/decisions/imports/Q-M1f.215-no-import-brings-a-modules-query-solve-config.md)

**Raised** 2026-11-13 by S1f.6.25's T2 (E-8). Row 59 of the phase's `questions.md` asked whether a qualified
import bringing a module's `(query …)`, `(solve …)` and `(config …)` is a wrong answer, and was left empty.
Two things the stage file does not settle:

1. **Is it a register row?** (a) Yes: the importer's R is computed under a lattice it did not state and it
   answers a query it did not ask, both false of the program as written. (b) No: the import did what its
   page said, *brings every form*.
2. **A module's `(trace …)`.** The stage file names three forms. `ein_ir::imports` has four top-level heads
   that are neither a declaration nor a fact, `trace` the fourth, and a flat import brings none of the
   four. (a) A qualified import drops the fourth too, so the two kinds of import bring the same kinds of
   form. (b) It keeps a module's trace, renamed, as before.

**Defaulted 2026-11-13: 1 (a), as the stage file has it, and 2 (a), the user's to revise.** W90, two rows.
No tracked program imports qualified a module that holds any of the four (`python3
build/s1f.6.25/e8_census.py`, 2026-11-13).

### Q-M1f.216 — A state file that re-writes less than the program as loaded, and the register

**Raised** 2026-11-13 by S1f.6.25's T3 (E-11). The stage file leaves two things to the task.

1. **Is it a register row?** The stage file's test is *a program whose reloaded dump answers differently
   from the run that wrote it*. T3 wrote one: `dump-selection.ein`, whose state file holds the state's two
   facts and neither the rule that derived them nor the relation's declaration, so that `ein saturate
   --dump` on the file lists no declared relation and no rule, and `answer_round_trip.rs`, which holds
   every corpus program's state files to their states by value, fails on it. R itself is right.
   (a) A row, under a schema that can read a line of a state file. (b) No row: the answer is R, and the
   file is a rendering of it.
2. **How far the fix goes.** (a) A name a **top-level** import's selection lists is live, the stage
   file's words; with it the tree-shake keeps everything a flat import brings, since the selection's
   closure is what is reachable from the listed names. (b) Also every imported rule with an empty
   `:match` and no parameter, which fires in any program that holds it, so that a qualified import of a
   library with such an entry is written whole too.

**Defaulted 2026-11-13: 1 (a) and 2 (a), the user's to revise.** 1 (a) since the file says of itself
*the program as loaded* and a per-commit test holds that sentence for every corpus program; the schema
moved to `/22` for it. 2 (a) since it is what the stage file specifies; 2 (b)'s case, a qualified import
of a library whose entry is an empty-`:match` rule, is E-41 of the phase's `findings.md`, found and not
fixed.

### Q-M1f.217 — When the loader says that a selection left a relation's rules behind

**Raised** 2026-11-13 by S1f.6.25's T4 (E-20). The stage file asks for the condition to be chosen on a census, and to
stop if none is quiet on the corpus and loud on the three witnesses. Three were taken (`python3
build/s1f.6.25/e20_census.py`, 2026-11-13, 842 tracked files and the witnesses):

- **(A) Any kept relation declaration,** listed or not, where the module has a rule that concludes it and
  the selection keeps none: 24 tracked files print a line, every `std.num9` program among them, whose
  module lists `division-by-zero` on purpose.
- **(B) A, and the resolved program holds no rule that concludes it:** 2 tracked files,
  `examples/word_problems/gaps.ein` and `readings.ein`, each a module that lists the word theory's
  relations on purpose and is a corpus program alone.
- **(C) B, and the relation is not listed,** so it came through the closure: no tracked file, and each of
  the three witnesses.

**Defaulted 2026-11-13: (C), the user's to revise.** A name in the list is asked for; a relation the
closure dragged in is the one nobody chose. The stage file's second candidate, *the module has an entry
rule and it is not kept*, was not taken: the language has no notion of an entry, and a rule with an empty
`:match` is only how the stdlib writes one.

Three smaller things, also defaults: the line is `note: `, the prefix the read-out's own stderr lines
carry; relations past the second are counted and not listed, so the line stays one line; and a
conclusion is the head of a fact the `:assert` concludes, not a fact written inside one, so the
calculator's `n`, which every number is built of, is not taken for a relation rules derive.

**Superseded 2026-11-15** at P1f.6's close, on the user's ruling of that day (*"all as you recommended"*, `close.md` § The user's rulings, 3): the line retired with E-20 by S1f.6.35. The question has no subject since; the default stands as the record of what was built before.

### Q-M1f.218 — What a count past the run's reach says

**Decision file:** [`docs/decisions/answer/Q-M1f.218-a-count-is-exact-and-reported-up-to-2-63-1.md`](../../docs/decisions/answer/Q-M1f.218-a-count-is-exact-and-reported-up-to-2-63-1.md)

**Raised** 2026-11-13 by S1f.6.25's T5 (E-22). The stage file asks: *a value that says "more than 2¹²⁸", or the count in a wider
integer*, the first the default, *since nothing reads a count that large as a number*. T5 found that the
bound in the error's sentence was not the bound: a total was summed into `u64`, so a program of 64 free
candidates met it at 2^64, and `--json-summary` wrote a count from 2^63 as a negative number, its writer
holding a signed 64-bit integer.

- **(a) Counted exactly, reported up to one bound, 2^63 − 1.** The engine counts in an exact integer of
  any size, so nothing fails for being large; a surface carries a count no greater than 2^63 − 1,
  9 223 372 036 854 775 807, and **a count equal to it reads *this many or more***: the read-out and
  `--count` print *or more* and why, a state file's header and `ein test -v` the same words, and
  `--json-summary`, `answer.json` and `ein test --json-report` write the number, which their page says
  is a floor at that value. One bound on every surface, which JSON and the register's TOML both hold.
- **(b) The stage file's words, *more than 2¹²⁸*.** Two bounds would stay, 2^64 for a total and 2^128 for
  a state, and JSON would need a second form for a number between 2^63 and 2^128.
- **(c) The exact count,** in every surface: 2^792 is 239 digits in a read-out row, and a JSON number no
  reader's integer holds.

**Defaulted 2026-11-13: (a), the user's to revise.** What it costs: a count from 2^63 to 2^64 − 1, which
the read-out printed exactly, now reads *9 223 372 036 854 775 807 or more*.

### Q-M1f.219 — A query's `(false)` on a program that only owes

**Decision file:** [`docs/decisions/answer/Q-M1f.219-a-querys-false-holds-over-either-kind-of-empty-r.md`](../../docs/decisions/answer/Q-M1f.219-a-querys-false-holds-over-either-kind-of-empty-r.md)

**Raised** 2026-11-13 by S1f.6.26's T4, the claim half of E-18 of the calculator's
`findings.md`. A query's `:expect (false)` says that the query has no class,
and a query has no class over an empty R, whichever kind of empty it is. So the claim **holds** on a
program whose root is alive and only owes, where `ein solve --single` says `Open` and not `Contradiction`,
and where the region claim tells the two apart: `(solve :expect (region (owing …)))` holds and `(solve
:expect (false))` fails, *(false) says every node is dead; R is empty, but alive nodes owe*.

The witnesses. S1f.6.20's two, in scratch: `build/s1f.6.20/witness/gap_a2x.ein`, a word problem whose root
owes a quantity, with a query claiming `(false)`, which holds, and `gap_a2y.ein`, the same program claiming
`(solve :expect (false))`, which fails. Three tracked programs whose query claims `(false)` and holds on an
R that is empty by owing: `examples/binary_logic/unsatisfiable.ein`, `examples/binary_logic/equivalent.ein`
and `examples/beyond_arith/equation_no_solution.ein` (`ein solve FILE`, 2026-11-13).

- **(a) No change.** A query's `(false)` is *no class*, on either empty. Since S1f.6.26 the row above it
  says which empty: *R is empty, root is dead* or *R is empty, root is alive and owes n instances*. A
  program that means *contradiction* states it over the region, `(solve :expect (false))`.
- **(b) A query's `(false)` fails where root is alive.** It would then mean what the region's `(false)`
  means, *every node is dead*, and the three tracked programs above would each need another claim: what a
  query claims over an R that is empty by owing would have to be written first.
- **(c) A second claim form for a query,** one that says *no class, because R is empty by owing*.

**Defaulted 2026-11-13: (a), the user's to revise.** It is a change to what a claim means, which is the
subject of question 24 of the calculator's `questions.md`, not ruled
(round 2, items 24, 28 and 37), and of
[M1g's P1g.7](../m1g_logic/p1g.7_unsatisfiable_against_owing/README.md), which decides it with what an
unpayable debt may be called. No claim's outcome moved in S1f.6.26.

### Q-M1f.220 — The form of the rows for what root owes

**Decision file:** [`docs/decisions/read-out/Q-M1f.220-what-root-owes-is-a-block-of-at-most-four-owed.md`](../../docs/decisions/read-out/Q-M1f.220-what-root-owes-is-a-block-of-at-most-four-owed.md)

**Raised** 2026-11-13 by S1f.6.26's T1, which shows the rows on real programs before the printer moves
(E-19; the user's reply 30 (a) of 2026-11-12, *a read-out row for what root owes when R is not empty*). The
stage file has *the same rows the empty case prints today, under the `∅` row*, and has the listing share
the read-out's 100 lines. Three forms were looked at, the first two built and run on the corpus:

- **(a) A block under `∅`, cut as the warning's block is.** One `owed` row an instance, its value the
  instance through its rule's `:why`; at most four on stdout, then one line, *… n more instances:
  `--dump-answer DIR` writes every one, to DIR/answer.txt*; outside the listings' 100 lines; every one in
  `answer.txt`. `--count` prints the `∅` row and no `owed` row.
- **(b) The stage file's words: the rows charged to the listings' 100 lines.** Every instance is printed
  while the lines last. Measured (`python3 build/s1f.6.26/sweep.py --diff`, 849 files, 2026-11-13): the five
  zebra programs print 36 or 46 rows between `∅` and `minimal line`; `zebra2-minus-15.ein` and its twin then
  list four of their 32 certified states where they listed seven; and a root that owes a hundred instances
  leaves R's own listing, whatever its size, cut and outside its `(region …)`.
- **(c) Every instance, never cut,** as an empty R's `owing` rows are today: 46 lines on the two programs,
  and as many as a program's root owes.

**Defaulted 2026-11-13: (a), the user's to revise.** The debts of a root are not R's states, and (b)
charges them before the answer. What (a) costs: a fifth instance is not on stdout, and
`lattice/06_friends_tables_everyone_sits.ein`, whose root owes five, prints four and *… 1 more instance*.
The label `owed`, and the two wordings of a query's row over an empty R, *R is empty, root is dead* and
*R is empty, root is alive and owes n instances*, are defaults of the same task.

### Q-M1f.221 — One question a file, and the fall-back

**Decision file:** [`docs/decisions/answer/Q-M1f.221-ein-test-asks-a-file-the-least-question-that.md`](../../docs/decisions/answer/Q-M1f.221-ein-test-asks-a-file-the-least-question-that.md)

**Raised** 2026-11-13 by S1f.6.27's T1 (E-30; the user's reply (a) of 2026-11-12 on question 39, *`ein test`
reads the shape of each claim and runs the cheapest mode that decides it*). The reply settles that the
runner asks a cheaper question than describe. It does not say how many runs a file with several claims
gets, nor what happens where the cheaper question does not decide.

- **(a) One question a file, then the describing one.** The file is asked the least question that can
  decide every claim it states: for queries' claims all over one part of R, the most classes one lists
  being k, that part is asked for k + 1 states, and `--single` where k is at most one; a `(solve … :expect
  …)`, and claims over more than one part, ask describe at once. What the question leaves undecided is asked
  the describing question, of the program loaded again. Two runs a file at most.
- **(b) A mode a claim.** Each claim is asked its own question in its own run.
- **(c) One question a file and no fall-back:** a claim the question leaves undecided is NOT CHECKED.

Measured over the 69 corpus files whose claims are queries' alone (`python3 build/s1f.6.27/t1_price.py`,
2026-11-13): describe saturates 2 357 nodes, (a) 1 494 in 69 runs and one fall-back, (b) 1 590 in 156 runs,
each of which loads the program. (c) would leave `tests/region/08_query_classes.ein`, held today, NOT
CHECKED: two classes over seven states, where three states found decide nothing.

**Defaulted 2026-11-13: (a), the user's to revise.** It is the stage file's own preference (*one run where
one run decides them all*), it is the cheapest of the three on the corpus, and it is the only one under
which no claim describe decides today is left unchecked. What it costs: a file whose least question does
not decide is run twice, and the first run's work is not reused.

### Q-M1f.222 — A claim refuted by a state a run found

**Decision file:** [`docs/decisions/answer/Q-M1f.222-under-ein-test-a-state-found-refutes-a-claim.md`](../../docs/decisions/answer/Q-M1f.222-under-ein-test-a-state-found-refutes-a-claim.md)

**Raised** 2026-11-13 by the same task. Round 2's item 39 has *find up to k + 1: a (k + 1)-th refutes the
claim*. A claim is about projection classes and a mode finds states, and several states can fall in one
class, so the rule as built is: **a class of a state found that the claim does not list refutes the
claim**, whatever the run left unvisited, since every state a run lists is a certified member of the part
it was asked. Until this stage a run that did not exhaust what a claim is about decided nothing, under
`ein test` and under `ein solve` alike. The same reading refutes `(solve … :expect (false))` at an alive
root where describe stops at its budget.

The rule could be given to `ein solve` too: `ein solve --single FILE`, on a file whose claim lists one
class where R has two states in two classes, prints `:expect NOT CHECKED` and a `note:` and exits 0 today,
and would print `FAILED` and exit 1.

- **(a) `ein test` alone.** The runner decides claims; `ein solve` under a mode answers the question it was
  asked, and a claim that question did not exhaust stays a note, *not checked*, exit 0, as the user's
  ruling of 2026-10-16 has it (*a claim the run could not check is a `note:` under `ein solve`*).
- **(b) Both.** A run that holds a state whose class a claim does not list has refuted the claim, and by
  the same ruling a refuted claim exits 1 under `ein solve`. Whether a `-n N` run then says FAILED or *not
  checked* of a false claim would depend on which states it met first.
- **(c) Neither:** a claim is decided by an exhausted answer alone, and a false claim on a search too large
  to describe stays NOT CHECKED under `ein test`.

**Defaulted 2026-11-13: (a), the user's to revise.** The stage is the runner's, and under (a) no line and
no exit code of `ein solve` moves, which the stage's sweep holds. `ein_infer::explore::Options::refute` is
the one switch, set by `ein test` and by nothing else; (b) is setting it under `ein solve`.

### Q-M1f.223 — What the surfaces of `ein test` say of the question

**Decision file:** [`docs/decisions/read-out/Q-M1f.223-ein-test-says-which-question-decided-each-claim.md`](../../docs/decisions/read-out/Q-M1f.223-ein-test-says-which-question-decided-each-claim.md)

**Raised** 2026-11-13 by the same task, for T3 and T4. The stage file has *`--json-report` gains the route
each row was decided by* and *`ein test -v` says the route in a word*, and leaves the words.

- **`-v`.** A query claim's line names the question before what the run found: *holds (--single: all of
  R, 1 class over 1 states)*, *holds (-n 4: all of R, 3 classes over 3 states)*, *holds (describe: all of
  R, 2 classes over 7 states)*, and on a fall-back *holds (describe, after -n 3: …)*. The question is
  spelled as `ein solve`'s flags, and `describe` for the bare run. A solve claim's line is unchanged: it is
  asked describe alone and names the engine's route, *state route*, as before.
- **`--json-report`.** `ein-test-report/5`, additively: every row carries `route`, the question its
  claim's outcome was read off, in the same spelling, and `fell_back_from`, the question asked first where
  it left the claim undecided; both `null` on a row no run checked.
- **A claim still NOT CHECKED** keeps describe's line and gains one: *asked first: --single, which stopped
  at --region-budget*, or *…, which found 3 states, each in a class the claim lists, and did not exhaust
  what the claim is about*.
- **`--events` and `--json-summary`**, which name one run: where a file was asked two questions they hold
  the last, the describing run. The first run's stream is not kept.

**Defaulted 2026-11-13: as above, the user's to revise.** The alternative for the artefacts is both runs in
one stream, which the event protocol's one header a file does not have a place for.

### Q-M1f.224 — What a bare description pays for N

**Decision file:** [`docs/decisions/read-out/Q-M1f.224-a-run-takes-n-only-where-it-prints-n.md`](../../docs/decisions/read-out/Q-M1f.224-a-run-takes-n-only-where-it-prints-n.md)

**Raised** 2026-11-13 by S1f.6.27's
T5 (E-31). On the state route `N` is the one part of an exhausted description that is not read off the states
the walk met: it is searched for after the walk, the minimal transversals of the maximal alive states'
complements (`ein_infer::transversal`, MMCS). Its count is how many transversals there are, so a surface that
prints the count needs the search as much as one that prints the sets. The stage made the runs that print
neither take none, `ein solve --count` and `ein test` without an artefact, and every surface stayed byte for
byte. A bare `ein solve` prints the `N` row and its count, so it still pays: on x × 7 at five digits
(`build/s1f.6.27/p/mul7_5.ein`, 16 331 saturations) `ein solve` takes 5.05 s, 87 % and 98.7 % of the samples on
the machine's two kinds of core `transversal::Mmcs::rec`, where `--count` takes 0.21 s (`perf record`, the
release binary of S1f.6.27's T5, 2026-11-13).

- **(a) No change.** A bare `ein solve` prints `N` and its count, and pays for them; a question that wants
  less is asked by `--count`, a mode, or `ein test`.
- **(b) The `N` row reads *not taken* on stdout**, and its count and sets are taken where an artefact that
  carries them is written (`--json-summary`, `--events`, `--dump-answer`, `--trace`). Every describe's
  stdout moves by one row, and the read-out's examples page with it.
- **(c) A cheaper search for N**: an engine stage on `ein_infer::transversal`, the output kept byte for byte.
  How much it gains is not measured.

**Defaulted 2026-11-13: (a), the user's to revise.** The stage's rule for E-31 was *remove the work only if
every surface is byte-identical after*, and (b) moves a row; (c) is a stage of its own. Neither is this
phase's to start without a reply.

### Q-M1f.225 — What the fact-grain test does where the closure gives up

**Raised** 2026-11-13 by S1f.6.28's
T1 (E-33). `ein-infer/examples/shared/fact_grain.rs` closes a program's candidates under its rules over shapes,
and on a program where a supposed fact leads to a request to `std.arith` with an operand the commitment does not
bind the closure follows every shape of the calculator's circuits and does not end in any time a gate has. The
stage made it run only where a relation-grain site reads it, which is the whole of the fix for every program the
stage banks, and none of these: what remains is a **flagged** program of that kind, the page's `boxes13` with the
reading owed only where something is left over, whose obligation reads `left-over`. The closure has to give an
answer there or not end.

- **(a) The closure gives up after 10 000 shapes, on the safe side, and the test fails on such a program, naming
  it.** The bound never decides a verdict the test reads: the test reads the exact closure's verdict or fails.
  Banking such a program needs a ruling first, this one's (b).
- **(b) The closure gives up, and the test reads the safe verdict**, *not monotone*, as it reads every program
  fact grain does not call monotone: no cube of it is checked here. The test then checks less than it did, on a
  program chosen by the size of its closure, which is what the stage file names as not acceptable without a
  ruling.
- **(c) No bound.** The test does not end on such a program, as before the stage.

**Ruled 2026-11-13: (a).** The user: *"Q-M1f.225: Ok"*. Defaulted the same day by the stage: it is the only
one of the three that keeps what the test means on every program and ends. No program of the corpus is one; the largest closure a flagged corpus program
computes is 100 shapes.

### Q-M1f.226 — An obligation's name in a closing rule, and `alive_invariant`'s R0

**Decision file:** [`docs/decisions/hypotheses/Q-M1f.226-an-obligations-name-is-no-guessable-atom.md`](../../docs/decisions/hypotheses/Q-M1f.226-an-obligations-name-is-no-guessable-atom.md)

**Raised** 2026-11-13 by S1f.6.28's
T5 (E-48). The rebuilt `examples/word_problems/readings.ein` owes a reading by an
obligation, `reading-owed`, and its closing rule names every rule of the module, `(readings-rule reading-owed)`
among them, since a selective import brings what the selected rule names: without it a problem that imports
`readings` has no obligation (`boxes12.ein`, 3 states and root owing nothing, 2026-11-13). Named there, the
obligation's name is *seeded*, a name of the program the KB's names index does not hold, since the index unions
the rules' names and not the obligations'; and `Program::categorise` calls it a rule's name (M1d S1d.2.4), which
the blind enumerator drops by design. `ein-infer/tests/alive_invariant.rs`'s
`every_name_of_the_program_is_a_candidate_object` asserted every seeded name to be one of the enumerator's
objects, and failed on the three files.

- **(a) The test lists a seeded obligation's name and does not hold it to the objects**, with a comment that says
  why. No engine source moves; R0 holds of every other seeded name as before.
- **(b) The names index holds the obligations' names as it holds the rules'** (`ein-core`'s `Kb`, the two places
  that union the registry's names in). The name is then not seeded, R0 is unchanged, and `candidate_objects`
  drops it as it drops every rule's. An engine change, in an examples stage.
- **(c) The closing rule leaves the obligation out.** Not viable: the import then leaves the obligation behind.

**Defaulted 2026-11-13: (a), the user's to revise.** It moves no engine source in a stage whose subject is the
examples; (b) is the change that removes the asymmetry the finding names, and is a stage of its own.

**(b) measured, 2026-11-13, on the user's question** (*"Q-M1f.226: I suppose yes, what it cost?"*), on a throwaway
worktree of `ba407d83`, nothing committed. The change is four lines of `ein-core`'s `kb.rs` (`rebuild_layer` and
`check_layering` union `program.obligations`' names in beside the rules') and the removal of (a)'s twelve-line
carve-out in `alive_invariant.rs`. **No answer moves:** `ein solve` prints the same bytes on all 747 tracked
programs, the one difference the worktree's path in a module-not-found line (a sweep of both release binaries,
`saturate` and `solve`, timings masked); `corpus_cli`, `answer_round_trip`, `lever_invariance`, `golden_trace`,
`dot_wellformed` and `alive_invariant` pass, R0 with no carve-out, and its banked list of seeded names loses
exactly `reading-owed`'s three rows (`boxes12`, `boxes13`, `readings`). **What moves** is the loaded KB's names
index on every program with an obligation: `ein saturate`'s snapshot counts one name more, and one more `category
= rule`, per obligation, on 35 programs (the stdlib's `bijection` and `slots`, their `tests/stdlib/` programs,
`zebra.ein`, `zebra2.ein` and its variants, the `oblig-*` witnesses, `lattice/`, the word problems); and
`corpus_shapes.md5` moves by 40 digests, all of kind `::load`, none of any other kind (blessed on the worktree and
diffed). No page quotes those counts.

**Ruled 2026-11-13: (b).** The user, shown the measure: *"continue next stages till the end of the phase"*,
after *"I suppose yes"*. Applied as measured: `kb.rs`'s two unions, `alive_invariant.rs`'s carve-out gone and
its seeded list less `reading-owed`'s three rows; `corpus_shapes.md5` re-banked, 40 `::load` digests moved and
no other (`EIN_BLESS=1 cargo test -p ein-render --test corpus_shapes`, then the diff counted). E-48 closed.

### Q-M1f.227 — How a run of the biggest calculation is held to about 16 GB, and how long it may take

**Raised** 2026-11-13 by S1f.6.29's
T2. The user's reply to round 2's item 49 (`dc88520b`): *"Ok, then pick the one that fits into memory, say ~16GB;
do not call it in tests and in CI, just for manual solve call"*; the first reply: *"biggest solvable arithmetic
calculation in let's say 100+-50 seconds on this host"*. The host has 31 GiB, and before the sweep 16.2 GiB of it
was available (`MemAvailable` 16 994 396 kB, 2026-11-13 21:45), the rest held by the desktop, a browser and an
editor. Under `ein solve` a product's or a chain's cost is the state's text (E-24), so time can end a run long
before memory does, and *the largest that fits in 16 GB* may be a run of hours.

- **(a) 16.0 GB, a cgroup a run, and 600 s.** `systemd-run --user --scope -p MemoryMax=16000000000 -p
  MemorySwapMax=0`, the run's `oom_score_adj` 1000 so that it is the kernel's first choice if the host runs out
  first; a run past 600 s, four times the window's top, killed and its size reported as bound by time; the search
  narrowed to 3 %.
- **(b) 16 GiB, 17.2 GB.** Past what the host had available, so a run near the cap meets the host's limit before
  its own.
- **(c) No time limit.** Memory alone, as the reply reads; under `ein solve` a run of hours.
- **(d) The window's top, 150 s, as the limit.** The biggest that fits in memory *and* in the first reply's
  window: under `ein saturate` the same rows, since none that memory bound passed 61 s; under `ein solve` smaller
  programs.

**Defaulted 2026-11-13: (a), the user's to revise.** It reads *about 16 GB* as written, stays inside what the host
had, and lets memory bind wherever it can within ten minutes.

**What the first run met.** A run at the cap leaves the host about 1.4 GB of the 17.4 GB it had, and at 21:51,
during `ein saturate`'s chain, Claude Code stopped the background sweep because the host was critically low on
memory, with the note *do not start it again on your own … start it again only when asked*. So `ein saturate`'s
product, quotient and requests are measured, its chain to 1 075! with the next size unknown, and `ein solve` not at
all (the record).
How to finish it is the user's: the same sweep started again by the user's word, the rest of it alone (`--commands
saturate --shapes chain`, then `--commands solve`); with the harness's reaping turned off
(`CLAUDE_CODE_DISABLE_BG_SHELL_PRESSURE_REAP=1` in Claude Code's environment, which only the user can set); or
with a lower cap that leaves the host more than the harness's threshold.

**Resumed, and closed.** The user, 2026-11-13: *"run again, I freed memory"*. The sweep resumed at 22:37 with
28.2 GB available, the same cap, and a guard: no run starts with under the cap and 4 GB available. No run met the
guard. `ein saturate`'s chain ran to the cap, 1 081!; `ein solve`'s product and quotient ran to the 600 s limit,
547 digits by 547 and 626 by 313, and its requests to 30 379 at the cap. At 23:48 the user: *"runs way too long than
I expected - just write down generated examples and close"*, so the sweep was stopped there, `ein solve`'s chain
not run and its requests not narrowed to the end. The default (a) is what every figure was taken under.

### Q-M1f.228 — What the interface of the tagged calls holds beyond the seven names round 2 showed

**Decision file:** [`docs/decisions/stdlib/math/Q-M1f.228-the-interface-of-the-tagged-calls-is-what-their.md`](../../docs/decisions/stdlib/math/Q-M1f.228-the-interface-of-the-tagged-calls-is-what-their.md)

**Raised** 2026-11-13 by S1f.6.30's
T1. Round 2's item 41 showed the user seven relations, `add-walk`,
`mul-call`, `div-call` and `cmp-walk`, answered by `done`, `div-done` and `order-is`, and the starting shape
`(add-walk TAG X Y 0 z)`; the user's *"(a)"* made them an interface. Two things the callers write are not in that
list:

- **`sub-walk`, answered by `done` or `short`.** `std.int` calls the subtraction walk three times, in its
  simplification (`stdlib/int.ein`, rule `int-norm-walks`, both ways at once) and in a division's remainder (`int-div-rest`), and
  `tests/stdlib/arith/08_sub.ein` reads `short`. The question's list came from the eight banked examples, which
  never subtract by the walk.
- **The operand look-up, `operand`, answered by `operand-is`.** `std.int` and `std.rat` call it for every
  operand of every request, so that a name goes through `std.arith`'s `let`, and extend it: their rules
  `int-operand` and `rat-operand` answer `(operand TAG (int P N))` and `(operand TAG (rat I D))` themselves. Found
  by T3's census, which read the modules over `std.arith` for the names they use of it, after T1's first commit.
- **An addition's carry in.** `examples/binary_arithmetic/conv.ein`'s `to-dec-double` writes
  `(add-walk (doubling ?b ?rest) ?acc ?acc ?bit z)`: the number so far, plus itself, plus the bit as the carry, in
  one walk. The rules take a carry of `0` or `1` there; the test, written to round 2's `0`, failed on that file
  first.

- **(a) The interface is what the callers write:** eleven relations, `sub-walk`, `short`, `operand` and
  `operand-is` among them, and
  `(add-walk TAG X Y C z)` with C a carry in of `0` or `1`; a subtraction starts at `0`, a comparison at `eq`, a
  walk at `z`.
- **(b) The seven names and the `0`:** `sub-walk`, `short`, `operand` and `operand-is` stay the circuit's own and
  `std.int` and `std.rat` call what is not promised; `conv.ein` is rewritten to double with a carry of `0` and add the bit as a second call, one walk
  more a bit.

**Defaulted 2026-11-13: (a), the user's to revise.** It promises what two shipped callers already depend on, and
moves no program; (b) would move `conv.ein`'s states and leave `std.int` outside its own rule.

### Q-M1f.229 — What the census of imported names reads, and how

**Raised** 2026-11-13 by S1f.6.30's
T3: *over every stdlib module and every example theory, the relations it declares … and every pair of modules
that cannot be imported flat together today*. Three choices the task leaves open.

- **Which modules.** (a) The twelve stdlib modules and the fourteen example files another file imports as a
  theory (`binary_arithmetic/` three, `binary_logic/` eight, `word_problems/` three); the fixtures of the import
  mechanism, `tests/imports/` and the `import-*` witnesses, left out, since they exist to break it. (b) Every
  file another file imports.
- **Imported how.** (a) By every name the module declares itself, the most a flat import brings, which for a
  module whose entry names everything is the entry's import. (b) By the entry a program writes, which differs
  only for the libraries of rules imported by lists.
- **Taken how.** (a) Statically, by a reader that copies `ein_ir::imports`' selection and its check, and held
  to the loader on every pair. (b) By the loader alone, which names only the first clash of a pair.

**Defaulted 2026-11-13: (a) on all three, the user's to revise.** The census then names every clash of a pair,
and the loader confirms each pair's verdict: 325 of 325 agree (`import_clashes.md` §
3).

### Q-M1f.230 — How the proposals are prototyped and priced

**Raised** 2026-11-13 by S1f.6.30's
T4: *each proposal is shown as the probes of T2 rewritten under it … computed on a prototype where the proposal
can be prototyped outside the engine (by rewriting module text)*. The choices the rewriting leaves open:

- **Which modules.** (a) `std.arith` alone, the module question 8 is about, with `std.int` and `std.rat` named as
  taking the same treatment. (b) The three.
- **What `std.arith` exports.** (a) The requests and `let`, the results, the flags, `n`, T1's eleven, the entry
  `arith`, and `division-by-zero-is-false`, which `std.num9` selects by name: 26 relations, 2 rules. (b) Fewer:
  the interface left out, which fails `std.int` and `std.rat`.
- **How a hidden name is spelt.** (a) `std.arith.strip`, the module's name, as a bare qualified import already
  writes it. (b) A name the loader makes up, `#…`, which no program can write: not writable as module text, so
  not computed.
- **The convention's prefix.** (a) `arith-`, as `arith-rule` already is. (b) A dotted one, `arith.strip`, which
  reads as a qualified import's (Q-M1f.151's (c)).
- **The price.** (a) Two sweeps of the 133 corpus programs that bring `std.arith`, `ein solve` bare and `ein
  saturate --dump`, with the checkout's stdlib and the prototype's, and a bound of `corpus_shapes.md5`'s digests
  (5 520 of 24 479), since that test refuses `$EIN_STDLIB`. (b) A worktree with the prototype in place, a build
  of the workspace in it, and the test blessed there, for an exact count.

**Defaulted 2026-11-13: (a) on all five, the user's to revise.** The exact count of (b) is what T6's change,
whichever it is, takes before blessing.

### Q-M1f.231 — Which way the import mechanism closes the gap

**Decision file:** [`docs/decisions/imports/Q-M1f.231-every-name-a-module-declares-has-a-full-name.md`](../../docs/decisions/imports/Q-M1f.231-every-name-a-module-declares-has-a-full-name.md)

**Raised** 2026-11-13 by S1f.6.30's T5, on the user's reply to P1f.6's question 8 (2026-11-12): *"potential
names clash is a gap of import mechanism, plan a stage for investigation, proposals and fix"*. Today a flat
import shares by name, which is what connects theories and what clashes; `:as` shares nothing
(`import_clashes.md` § 2). Each option is shown on the probes, computed where module text
can prototype it (§ 4):

- **(a) A module says what it exports**, and what it does not is named under the module. P4e, P4f, P11 load;
  P2a and P4a do not (`done` is exported). `ein solve` moves on 17 of 133 programs, R on none.
- **(b) A module is one copy, whoever imports it,** and `:as` prefixes only the module's own names. The tables
  once, P10 connected. No corpus program moves. Whole only with (a).
- **(c) A rename on import,** `:rename ((done chore-done))`. P2a and P4a load; a renamed theory beside a theory
  built on it is refused, computed.
- **(d1) A convention:** the calculator's own 47 relations prefixed `arith-`. P4f loads; `ein solve` moves on 3.
  **(d2)** and the interface's 11: P2a and P4a load; 18 files rewritten.
- **(e) Nothing,** and the page's six lines on how to mix theories today.
- **(f) (a), (b) and (c) as one mechanism,** an exported name two imports share refused only where the program
  writes it.

**Recommended: (f), in two steps, (a) and (b) first** (§ 5). **Open, the user's.**

The user, 2026-11-13, in conversation, quoted in full in `import_clashes.md` § 7: *"flat import has to import listed symbols unrenamed, but remain (private) symbols standard way - with std.module... path"*, then the numbered answers and *"1 - yes (ii)"*. **Ruled: (h)**, which the page had not proposed: a full name per declaration, global and the same through every path, so one copy of a module; a short name an alias local to the file that lists it; a selection brings what the listed names require (the closure (ii)); a whole-module import brings everything. The fix is S1f.6.35.

### Q-M1f.232 — How a module says what it exports, and what `std.arith` exports

**Decision file:** [`docs/decisions/imports/Q-M1f.232-a-module-has-no-export-list.md`](../../docs/decisions/imports/Q-M1f.232-a-module-has-no-export-list.md)

**Raised** 2026-11-13 by S1f.6.30's T5; it matters under (a) and (f) of Q-M1f.231.

- **How.** (i) A top-level form `(export NAME+)`, one row of the grammar's table. (ii) The entry rule's names
  are the exports: no new form, but `arith` names every rule and closes every relation, so it would need a
  second list anyway. (iii) A mark on a name, a convention the loader reads.
- **What.** (i) The prototype's: the requests and `let`, the results, the flags, `n`, T1's eleven (`done` among
  them), the entry `arith` and `division-by-zero-is-false`, which `std.num9` selects by name: 26 relations, 2
  rules. (ii) The same with the interface's eleven exported under new names, `arith-done`, so that `done` is no
  longer a calculator's name a program can meet.

**Recommended: (i) and (i).** **Open, the user's.**

**Ruled 2026-11-13 by (h):** no `(export …)`, no entry list: every name a module declares has a full name, and the importer lists the short names it writes.

### Q-M1f.233 — How a name a module does not export is printed

**Raised** 2026-11-13 by S1f.6.30's T5; under (a) and (f). A state, `--trace`, a core and `ein saturate
--dump` print facts and rules the program did not write.

- **(i) `std.arith.strip`,** the module's name and a dot, as a bare qualified import prints it today:
  `refuted by std.arith.malformed-is-false` (P4g under the prototype).
- **(ii) Bare,** `strip`, with the module said apart; two modules' hidden `strip`s then read alike.
- **(iii) A name the loader makes up,** `#…`, which no program can write and no reader recognises.

**Recommended: (i).** **Open, the user's.**

**Answered 2026-11-13 by (h).** The user asked *"Ok, is that an issue?"*; the coordinator answered that it is a one-time move of printed outputs, R unmoved, and that the rule left open is which name a state prints: [Q-M1f.237](#q-m1f237--how-a-name-prints-under-h), defaulted.

### Q-M1f.234 — Two flat imports that export one name

**Decision file:** [`docs/decisions/imports/Q-M1f.234-one-short-name-bound-to-two-full-names-in-one.md`](../../docs/decisions/imports/Q-M1f.234-one-short-name-bound-to-two-full-names-in-one.md)

**Raised** 2026-11-13 by S1f.6.30's T5; under (f). P2a, `std.arith` and `chores`, both exporting `done`.

- **(i) Refused at load,** as every clash is today.
- **(ii) Refused only where the program writes the name,** with a line that names the two modules and the two
  remedies, `:as` and `:rename`; where it does not, each theory keeps its own.
- **(iii) The later import wins.** One theory's rules then read the other's facts: a wrong answer waiting.

**Recommended: (ii).** **Open, the user's.**

**Ruled 2026-11-13:** *"2. if it happens in the same file - it's an error, not correct ein code, just like ordinary namespaces"*. One short name bound to two full names in one file, by two imports or by an import and the file's own declaration, is refused at load; one short name bound to one full name twice is not an error.

### Q-M1f.235 — A module in a sibling directory (E-32)

**Decision file:** [`docs/decisions/imports/Q-M1f.235-a-module-has-one-path-from-the-program-files.md`](../../docs/decisions/imports/Q-M1f.235-a-module-has-one-path-from-the-program-files.md)

**Raised** 2026-11-13 by S1f.6.30's T5 (§ 4 (g)); E-32 is named in the
proposals and ruled with them.

- **(g1) `..` in a module name,** resolved by `locate` and refused outside the checkout; no grammar, since
  `NAME` takes `.` and `/` already.
- **(g2) A root,** dotted names under the nearest directory above that holds a marker.
- **(g3) A search path,** `$EIN_PATH`; a program's meaning then depends on the environment.
- **(g4) Nothing:** `examples/binary_logic/bits.ein` stays a generated copy.

**Recommended: (g1), after (b).** **Open, the user's.**

**Ruled 2026-11-13:** *"module canonical identity should work fine, std has one defined root, other imports are relative to the program file, `..` going outside the folder is restricted so there is always one single path; ignore potential problems with symlinks and hardlinks - it is out of the scope"*. `..` stays refused; E-32 stays as stated, and `examples/binary_logic/bits.ein`'s copy goes with S1f.6.16's build, which moves the binary tables into the stdlib. Made precise by the user the same day: *"imports are \"relative to the program file\" - means examples/word_problems may contain subdirectory word-theory with files there that would be imported with (import word-theory.gaps :as gaps) having global names like word-theory.gaps.gaps"*: every local module name is a path relative to the **program file's** directory, for the imports a module writes too, and a full name is the module's name and the short name.

### Q-M1f.236 — E-49 and E-50

**Decision file:** [`docs/decisions/imports/Q-M1f.236-a-short-name-a-file-writes-and-lists-from-no.md`](../../docs/decisions/imports/Q-M1f.236-a-short-name-a-file-writes-and-lists-from-no.md)

**Raised** 2026-11-13 by S1f.6.30's T5; the two findings its probes and census filed.

- **E-49,** a qualified theory's name written bare answered by silence. (i) A `note:` line on every command, as
  E-20's, where a program writes bare a name only an alias exports. (ii) Nothing; the page's line 2 of § 4 (e).
- **E-50,** two pairs that cannot be imported flat together and `difference`. (i) `std.elim`'s and
  `std.bijection`'s headers say they are alternatives; the logic's generator renames `complement` and
  `identity`; `difference` cured by (a) or (d) of Q-M1f.231 (P11 loads under both). (ii) `std.elim` merged into
  `std.bijection`.

**Recommended: (i) and (i).** **Open, the user's.**

**Ruled 2026-11-13:** E-49, *"3. Yes, we can accept it now, silent re-definition, like variables shadowing, it's a mistake"*: no `note:` line. E-50 is dissolved by full names under (h): the two pairs and `difference` coexist, and a program that lists one clashing short name from both modules is refused (Q-M1f.234).

### Q-M1f.237 — How a name prints under (h)

**Decision file:** [`docs/decisions/imports/Q-M1f.237-a-name-prints-as-the-main-file-writes-it-else.md`](../../docs/decisions/imports/Q-M1f.237-a-name-prints-as-the-main-file-writes-it-else.md)

**Raised** 2026-11-13 by the user's ruling of Q-M1f.231 (`import_clashes.md` § 7). Under (h) a name a module declares
is its full name, and a file may write it short. A state, a trace, a core and a read-out row print one of them.

- **(a) The main file's short name where it has one, the full name otherwise.** A program that lists `add`
  reads `(add …)`; the circuit's own steps read `std.arith.strip`.
- **(b) Always the full name.** One spelling per fact, long.
- **(c) Always the short name,** the module said apart. Ambiguous where two modules declare one short name.

**Defaulted 2026-11-13: (a), the user's to revise.** It prints a program's facts as the program writes them.

### Q-M1f.238 — A full name of a module no import brings

**Decision file:** [`docs/decisions/imports/Q-M1f.238-a-full-name-of-a-module-no-import-brings-is.md`](../../docs/decisions/imports/Q-M1f.238-a-full-name-of-a-module-no-import-brings-is.md)

**Raised** 2026-11-13 with Q-M1f.237. Full names are global (*"full names can call global"*); a file may write
`std.arith.add` where no import of the program brings `std.arith`.

- **(a) Refused at load,** naming the module: *write an import of it*.
- **(b) An implicit import** of the whole module.

**Defaulted 2026-11-13: (a), the user's to revise.** An import stays the one way a module enters a program.

### Q-M1f.239 — A declaration under an imported module's full name

**Decision file:** [`docs/decisions/imports/Q-M1f.239-a-program-declares-no-name-under-the-full-name.md`](../../docs/decisions/imports/Q-M1f.239-a-program-declares-no-name-under-the-full-name.md)

**Raised** 2026-11-13 with Q-M1f.237. Today a name with dots is a name, and a program may declare
`(relation std.arith.mine …)` (the coordinator's probe `v10`, 2026-11-13).

- **(a) Refused at load where the program imports that module;** a `--dump-answer` state file, which imports
  nothing and declares the full names it was loaded with, loads as today.
- **(b) Allowed,** a module's name space open to any file.

**Defaulted 2026-11-13: (a), the user's to revise.**

### Q-M1f.240 — Entry rules and `__closed__` lists under (h)

**Decision file:** [`docs/decisions/imports/Q-M1f.240-a-module-keeps-its-entry-rule-which-a-program.md`](../../docs/decisions/imports/Q-M1f.240-a-module-keeps-its-entry-rule-which-a-program.md)

**Raised** 2026-11-13 with Q-M1f.237. Under the closure (ii) a selection follows a relation to the rules that
read or conclude it, so an entry rule that names every rule of its module (`arith`, `int-arith`, …) is no
longer needed to bring them.

- **(a) They stay,** as they are; a program may still list the entry.
- **(b) They go,** each module's `*-rule` relation and facts with them.

**Defaulted 2026-11-13: (a), the user's to revise.** Keeping them moves nothing a program reads; removing them
moves every state of every program that imports a module.

### Q-M1f.241 — A declared short name with a dot

**Decision file:** [`docs/decisions/imports/Q-M1f.241-a-modules-declared-name-holds-no-dot.md`](../../docs/decisions/imports/Q-M1f.241-a-modules-declared-name-holds-no-dot.md)

**Raised** 2026-11-13 by the user's precision of Q-M1f.235: a full name is a module's name, dotted by
directory, and the short name, `word-theory.gaps.gaps`. A module that declared `gaps.x` in `word-theory` would
make `word-theory.gaps.x` the full name of two declarations, its own and module `word-theory.gaps`'s `x`.

- **(a) A declared short name holds no dot,** refused at load. No tracked program declares one (a scan of every
  `(relation|rule|hrule|macro NAME` of the tracked `.ein` files, 2026-11-13).
- **(b) Allowed,** a clash of two full names refused where it happens.

**Defaulted 2026-11-13: (a), the user's to revise.**

### Q-M1f.242 — A dead or redundant hypothesis declaration

**Decision file:** [`docs/decisions/hypotheses/Q-M1f.242-a-hypothesis-declaration-that-filters-nothing-is.md`](../../docs/decisions/hypotheses/Q-M1f.242-a-hypothesis-declaration-that-filters-nothing-is.md)

**Raised** 2026-11-14 by S1f.6.34, the user's simplification of 2026-11-13 on the pairs of mechanisms:
*"no-hypothesis make no sense if there are hrules - if the conflict then it's conflict, if not - then it filters
nothing"*. A list beside an hrule that contradicts none of its proposals filters no candidate (E-52), and a name of
`:no-hypothesis` that `:hypothesis-relations` already leaves out, or that a closure already closes, says nothing new.

- **(a) One `note:` line on stderr**, placed at the list or the name, on every command that loads the program, as the
  loader's own notes are (S1f.6.25); no exit code, answer or stdout moves.
- **(b) Silence.**
- **(c) Refused**, as a conflict is.

**Defaulted 2026-11-14: (a), the user's to revise.** Two corpus programs print one:
`examples/features/13_mixed_solution_and_open.ein` (a dead `:no-hypothesis` beside its hrule `pick`) and
`examples/word_problems/kinds_listed.ein` (`has` named and closed by `words`).

### Q-M1f.243 — An obligation, read as declared or as owed

**Decision file:** [`docs/decisions/hypotheses/Q-M1f.243-an-obligation-is-read-as-activated-owed-or-not.md`](../../docs/decisions/hypotheses/Q-M1f.243-an-obligation-is-read-as-activated-owed-or-not.md)

**Raised** 2026-11-14 by S1f.6.34's census. The verdicts of O + HR, O + NH and O + C name *an owed relation*. Seven
corpus programs list a relation their obligation can owe and root does not owe (`tests/stdlib/algebra/24`, `26`,
`tests/stdlib/slots/01`, `08`, `10`, `12`, `examples/ein-bugs/minimal-line-stated-obligation.ein`), and two more
the relation of an obligation root discharges (`tests/stdlib/bijection/06`, `tests/stdlib/closure/02`).

- **(a) As declared**: the relation of an obligation plan root's activators compile, owed at root or not, as an hrule
  is read as active whether or not its `:match` has a row. The user: *"filtered obligations are nonsense by
  definition"*.
- **(b) As owed at root**: only an instance root owes; the nine above load as they did.

**Defaulted 2026-11-14: (a), the user's to revise.** The nine are corrected (Q-M1f.246); under (b) they would
not have needed to be. An activator only a commitment derives is not seen by either reading, since the check reads
root.

### Q-M1f.244 — O + C beside an hrule

**Decision file:** [`docs/decisions/hypotheses/Q-M1f.244-an-obligated-relations-held-extent-is-not-closed.md`](../../docs/decisions/hypotheses/Q-M1f.244-an-obligated-relations-held-extent-is-not-closed.md)

**Raised** 2026-11-14 by S1f.6.34's witnesses. The table qualifies O + HR and O + NH with *with no hrule*, the two
lists filtering only rungs 2 and 3, and writes O + C without it.

- **(a) A conflict whatever generates**: a closure says no commitment extends the relation, which the obligation
  needs one to; beside an hrule that does not propose it, a state that owes it can never be paid (`o_c_h.ein`: R
  empty, root owing).
- **(b) Only with no hrule**, as the lists.

**Defaulted 2026-11-14: (a), the user's to revise** (*"obligated relation can't be __closed__ also"*). No corpus
program is in O + C beside an hrule.

### Q-M1f.245 — Where the conflict check runs

**Decision file:** [`docs/decisions/hypotheses/Q-M1f.245-the-hypothesis-mechanisms-are-judged-at-load-on.md`](../../docs/decisions/hypotheses/Q-M1f.245-the-hypothesis-mechanisms-are-judged-at-load-on.md)

**Raised** 2026-11-14 by S1f.6.34's T2. A derived closure and a derived activator exist only at root's fixpoint,
and four commands never saturate root (`ein render program`, `rules`, `rule`, `constraints`, and `ein kb save`
without `--saturate`).

- **(a) At load, on every command**: `ein-cli`'s one load path loads the program a second time, saturates that
  root, and judges, where the program could declare two mechanisms at all (`mechanisms::may_conflict`; 78 corpus
  programs, root at most 36 ms each); the command's own tables are untouched. A library caller calls
  `ein_infer::mechanisms::check`; `region::prepare` does not, so the library's routes answer a conflicting
  program as before.
- **(b) In `region::prepare` beside *forbid***, on the commands that take a universe only, as § 4.7.
- **(c) Both.**

**Defaulted 2026-11-14: (a), the user's to revise**, as § 4.20 refuses a list on every command.

### Q-M1f.246 — The corpus's twenty-four programs in conflict

**Raised** 2026-11-14 by S1f.6.34's T1, which showed each with its answer. Every one is a deliberate fixture that
switched an obligation's generating role off, by `:no-hypothesis` on what it owes or by a closure of it, so that
its question was root's saturation or root's verdict alone. None is an accident of spelling.

- **(a) Each corrected to say what it meant, one kept as the refusal's fixture**: where root discharges every debt,
  the owed relation out of the list, nothing moving (seven); where the question is the obligation's own report, the
  obligation proposes and the claim is R's states (`algebra/23`, `25`; `slots/09`, `11`, with a negative per
  unwanted candidate so that R is one state), or, for the region forms' examples, the debt made unpayable by the
  program's own negatives and R left empty (`tests/region/04_owing`, `05_empty`); where the question is root's
  saturation, `slot-partition`'s fan-out written out without its two obligations (`slots/02`, `03`, `04`, `07`,
  `13`), or, where the fan-out is the subject, `bijective-setup`'s, an hrule that proposes only the `wears` root
  holds, so nothing new is guessed and the obligations judge root alone (`bijection/01` to `03`: letting the
  obligations propose instead solved the bijection, and six mutants of the negative-completion and elimination
  rules survived it); `bijection/06`'s own obligation a bare `(open)`, declining for that reason; `closure/02`
  without its obligation; `closure/03` moved to `examples/ein-bugs/closed-and-owing.ein`, the fixture of O + C.
- **(b) Each kept as a fixture of the refusal**: the conformance suite then activates none of the four obligation
  rules where they owe, and `stdlib_coverage.rs` fails.
- **(c) An hrule that proposes nothing new beside each**, which keeps every claim; H + O is silent. Taken for the
  three bijection fixtures within (a).

**Defaulted 2026-11-14: (a), the user's to revise.** Fourteen of the twenty-four answer as before, twelve of them
printing as before too and the bijection fixtures `02` and `03` one `outside` line more; nine answer a different R,
by their own edit and not the engine's, each claim restated and held; one is refused.

**Restated 2026-11-14 by S1f.6.36**
([Q-M1f.260](#q-m1f260--which-of-s1f634s-twenty-four-say-no-search)): `(solve :no-search)` is the sanctioned way to say
what the twenty-four said with a list, so the 21 whose question was root alone are back to their program and claim as
they were before S1f.6.34, `:no-search` in their list's place, and (a)'s workarounds are gone (the `as-stated` hrules,
the fan-outs written out, the added negatives, `algebra/23` and `25`'s second question). `bijection/06`, `closure/02`
and the refusal's fixture stay as (a) left them.

### Q-M1f.247 — What the closure (ii) follows: a relation of any kind, or a declared one

**Decision file:** [`docs/decisions/imports/Q-M1f.247-every-import-brings-its-module-whole.md`](../../docs/decisions/imports/Q-M1f.247-every-import-brings-its-module-whole.md)

**Raised** 2026-11-14 by S1f.6.35's T1, on its prototype of (h). The ruling: *"a listed or kept relation to
the rules that read or conclude it"*. A relation in Ein is what `(relation …)` declares, and also the head of
any fact, declared or not: a parametrised rule's activators, `(converse owns owned-by)`, are facts of a relation
named after the rule, which `std.algebra` and the other zebra modules never declare.

- **(a) Any kept name of the module's own** that its rules read or conclude: a rule's name read as its
  activators' relation too. `std.algebra`'s nine lemmas come with any activator (`symmetric` brings 13
  declarations where today it brings 4), and `tests/stdlib/typing/03_type_hierarchy_welltyped.ein` is refuted,
  `converse-pair-symmetric` checking the well-typed pair reversed (E-53).
- **(b) A relation the module declares.** The calculator's selections bring the circuit; a zebra module's
  bring what their names reach, as today, and its own imports' lists. No claim moves.
- **(c) A listed relation only**, not a kept one: a selection of `add` brings what reads `add`, and nothing
  follows from what that brings; the circuit's walks, read by no listed name, stay behind.

**Recommended: (b).** Open; the stage stopped for it (the coordinator's instruction: a stop where (h) as
ruled moves R).

**Ruled 2026-11-14: (a), any relation**, against the recommendation. The user: *"the behaviour must not change - when import std.math.arith and use std.math.arith.sum must work identical to sum from import std.math.arith :symbols (sum); So I guess that any touched relation must be pulled in, otherwise semantic of imported symbold could be incomplete"*. **The rule the closure
must satisfy**, read from those words: a short name a selection lists means exactly what its full name means
under a whole-module import of the module, so the selection brings every rule of the module that touches, by
reading or concluding, a relation it keeps, that relation a declared one or a rule's activators', and
transitively; whatever would make the selected name's meaning incomplete is pulled in. The coordinator's relay
of the ruling lettered it (b); the reading is this page's (a), with Q-M1f.248 (a) the prototype's V2. E-53
moves from hypothetical to the shipped library's: S1f.6.35's record.

**Superseded 2026-11-14**, after the user saw the two stop probes traced (the forms' root saturations equal;
what moved was the generator's rung, chosen by a loaded obligation rule, and the blind enumerator's universe):
*"Ok, just forget my intial ruling - in all cases we import everything, the difference only in names"*. **Every import brings its module whole**, with its own imports, all the way down, one copy per program,
under full names; a selection lists only short names. No closure exists, so neither this question nor
Q-M1f.248 has a subject; § 7 point 5 of `import_clashes.md`, *"no background side effects"*, goes with them.
The rest of (h) stands. Built by S1f.6.35.

### Q-M1f.248 — A closing rule under the closure (ii)

**Raised** 2026-11-14 by S1f.6.35's T1, with Q-M1f.247. `std.arith` closes every relation it declares and names
every rule it has in one rule with an empty `:match`, `arith`, which writes `(__closed__ let)` and `(arith-rule
add-ask)`: the relations are values there, so (ii)'s *conclude* does not reach the rule. A selection that keeps
the circuit and not its closure leaves `let` open, and *forbid* refuses the program: the acceptance's own
`(import std.arith :symbols (add sum-of n))`, and the sixteen `std.num9` programs, whose module lists
`division-by-zero`.

- **(a) A rule whose `:assert` writes `(__closed__ R)` of a kept `R` concludes it**, so the selection brings it,
  and the closing rule's own names bring the rest: `:symbols (add sum-of n)` computes, `(sum-of (n z 2) (n z 3)
  (n z 5))`; every `std.num9` program loads, with the calculator whole (580 more facts and rules, R unmoved).
- **(b) No**: a selection that wants a computed theory lists its entry, as today. The acceptance's program is
  refused; `std.num9` lists `arith`.
- **(c) The closures written as top-level facts** in each module, `(__closed__ let)`, which a kept relation's
  facts bring: every module's text moves, against Q-M1f.240's default.

**Recommended: (a).** Open; the stage stopped for it.

**Ruled 2026-11-14: (a).** The user: *"Yes, closing counts"*. With Q-M1f.247 (a) it is the prototype's V2.

**Superseded 2026-11-14** by the user's *"Ok, just forget my intial ruling - in all cases we import everything, the difference only in names"*: no closure exists (Q-M1f.247).

### Q-M1f.249 — The answers (h) moves under the recommended reading

**Decision file:** [`docs/decisions/imports/Q-M1f.249-the-answer-does-not-depend-on-the-import-form.md`](../../docs/decisions/imports/Q-M1f.249-the-answer-does-not-depend-on-the-import-form.md)

**Raised** 2026-11-14 by S1f.6.35's T1. Under Q-M1f.247 (b) and Q-M1f.248 (a), of the 514 tracked programs
that load, R is the same on 510; the read-out byte-equal on 478 and equal once module prefixes are off on 500.
What moves, each by the ruling and not by a defect:

- `tests/imports/02_nested_import_left_behind.ein`, E-20's witness: its module's own import of `lower` is an
  edge of the program, so `(got a aa)` is derived and its claim `(model)` refuted; R is {∅} either way;
- `examples/word_problems/gaps.ein` and `listed_is_all.ein`, modules read alone: `ein solve` describes them,
  R {∅} where it was E-1's four states of `(asked __closed__)`; their corpus entries run no solve;
- `examples/features/09_adjacent_via_same_house.ein`: R unmoved, the universe row 82 candidates to none, an
  obligation rule loaded by `std.bijection`'s own import (E-51);
- and one program's text beyond its imports: `examples/binary_logic/adder_against_table.ein`'s own copy of
  `circuit`'s `gate` replaced by `(import circuit :symbols (gate))`, R and read-out unmoved.

- **(a) Accepted as the ruling's**: the witness's claim restated, `(model (got a aa))`, and its file named for
  what it shows; the two modules' notes say a solve now describes them; no register row.
- **(b) Each a register row**, as a wrong answer would be.

**Recommended: (a).** Open; the stage stopped for it.

**2026-11-14: the user, *"Show me first"*.** Nothing is restated or accepted yet. Under the reading ruled (Q-M1f.247 (a),
.248 (a)), the moves are re-taken and shown, each with its imports and its run before and after, in
S1f.6.35's record; the list above,
taken under the recommended reading, is superseded by it. **Open, the user's.**

**Answered 2026-11-14 (a), accepted**, on the user's criterion of that day (*"the answer MUST be invarian of import
type - write 2 same programs with different imports and check that both have equivalent solution (up to names
prefixes)"*): each of the five is answered by today's whole import as (h) answers it, so the move is today's
selective import having been incomplete (E-20's family), not (h); `typing/03` is E-53, a bug of `std.algebra`
that exists today wherever the module is whole (W93, fixed by F1). The E-20 witness's claim restated
`(model (got a aa))`; the two modules' corpus notes say a solve now describes them; `features/09`'s universe row
moves with E-51. The stage's invariance test, every corpus program selective against whole against `:as`,
is what holds them (`ein-cli/tests/import_invariance.rs`).

### Q-M1f.250 — A listed name a module states facts of and does not declare

**Decision file:** [`docs/decisions/imports/Q-M1f.250-a-listed-name-a-module-states-facts-of-names.md`](../../docs/decisions/imports/Q-M1f.250-a-listed-name-a-module-states-facts-of-names.md)

**Raised** 2026-11-14 by S1f.6.35's T1. Today a selection may list a name the module only states facts of (*a
relation needs no declaration*); `examples/binary_logic/equivalent.ein` lists `gate` from `full_adder`, a library
of `gate` facts, and `require` lists `total-owed` from `inputs`, which states `(total-owed in is-a)`. Under (h)
those facts are of another module's relation (`circuit.gate`, `std.algebra.total-owed`).

- **(a) The listed name is bound to the relation the module's facts are of, as the module's own text reads it**,
  and the selection brings those facts. The importer writes `gate` and means `circuit`'s.
- **(b) Refused as a re-export**: the importer brings the library whole and lists the relation from its
  declaring module.

**Defaulted 2026-11-14 (S1f.6.35): (a)**, restated under the whole-import ruling: the module comes whole
whatever is listed, so a listed name is a name only, and one the module states facts of and does not declare is
bound to the relation those facts are of, as the module's own text reads it (`examples/binary_logic/equivalent.ein`'s
`gate` is `circuit.gate`). The user's to revise.

### Q-M1f.251 — A prefixed name that names nothing the module declares

**Decision file:** [`docs/decisions/imports/Q-M1f.251-a-prefixed-name-that-names-nothing-its-module.md`](../../docs/decisions/imports/Q-M1f.251-a-prefixed-name-that-names-nothing-its-module.md)

**Raised** 2026-11-14 by S1f.6.35's T1. Under `(import std.int :as i)`, `i.n`: `std.int` declares no `n`, which is
`std.arith`'s. Likewise `std.arith.nope` beside an import of `std.arith`.

- **(a) Refused at load**, naming the module and the name, as a `:symbols` name it does not provide is.
- **(b) A name of its own**, as today, answered by silence.

**Defaulted 2026-11-14 (S1f.6.35): (a)**, Q-M1f.238's reading for a module the program does bring. The user's to
revise.

### Q-M1f.252 — An empty segment or a `/` in a module's name

**Decision file:** [`docs/decisions/imports/Q-M1f.252-a-modules-name-has-no-empty-segment-and-no-slash.md`](../../docs/decisions/imports/Q-M1f.252-a-modules-name-has-no-empty-segment-and-no-slash.md)

**Raised** 2026-11-14 by S1f.6.35's T1. Today `(import ..succ)` loads `succ.ein` beside the importer, the empty
segments dropped, and `(import ../x/m)` names `/x/m.ein`, an absolute path (E-32). Under (h) a module's name is its
identity, so `..succ` and `succ` would be one file under two names.

- **(a) Refused at load**: a module's name is segments joined by dots, none empty, none holding a `/`.
- **(b) Normalised**, the empty segments dropped, as today.

**Defaulted 2026-11-14 (S1f.6.35): (a)**, the user's *"`..` going outside the folder is restricted so there is always
one single path"*. The user's to revise.

### Q-M1f.253 — How a name prints under `:as`

**Decision file:** [`docs/decisions/imports/Q-M1f.253-under-an-alias-a-name-prints-as-the-main-file.md`](../../docs/decisions/imports/Q-M1f.253-under-an-alias-a-name-prints-as-the-main-file.md)

**Raised** 2026-11-14 by S1f.6.35's T1, on Q-M1f.237's default (the main file's short name where it has one,
the full name otherwise). Under `(import std.arith :as a)` the program writes `a.sum-of`.

- **(a) The main file's alias spelling, `a.sum-of`**, read as its name for the declaration: every qualified
  program prints as today. Where the program lists the name too, the listed short name.
- **(b) The full name, `std.arith.sum-of`**: an alias is no short name.

**Defaulted 2026-11-14 (S1f.6.35): (a)**: Q-M1f.237's *"it prints a program's facts as the program writes them"*.
The user's to revise.

### Q-M1f.254 — S1f.6.34's conflicts beside `(solve :no-search)`

**Decision file:** [`docs/decisions/hypotheses/Q-M1f.254-solve-no-search-relaxes-no-conflict.md`](../../docs/decisions/hypotheses/Q-M1f.254-solve-no-search-relaxes-no-conflict.md)

**Raised** 2026-11-14 by S1f.6.36's T1. Under `:no-search` no rung runs,
so an hrule proposes nothing and a list filters nothing. The stage keeps O + NH refused (*"`:no-hypothesis` over an
owed relation stays refused"*); the H conflicts (an active hrule proposing a relation a list or a closure takes
out) are about what an hrule proposes, which under `:no-search` is nothing.

- **(a) Read as declared, every conflict stands**: H + HR, H + NH, H + C, O + HR, O + NH, O + C and HR + NH refuse
  as S1f.6.34 refuses them, the hrule read as active and the obligation as its plans whatever the search does (as
  Q-M1f.243 reads an obligation); `:no-search` adds a `note:` beside an hrule (*proposes no candidate*) and beside each
  list (*filters no candidate*), and none beside a closure or an obligation. E-47's witness with `:no-search` in its
  list's place stays refused for H + C (`has` closed by `words`).
- **(b) The H conflicts dissolve under `:no-search`**, each a note that the hrule proposes nothing; the O conflicts
  and HR + NH stand. E-47's witness with `:no-search` in its list's place loads, its hrule noted.

**Defaulted 2026-11-14: (a), the user's to revise** — the stage's *"Not relax S1f.6.34's conflicts"*, and one reading
of a declaration for every pair. No tracked program is in either case.

### Q-M1f.255 — What `ein solve --no-search` prints

**Decision file:** [`docs/decisions/read-out/Q-M1f.255-ein-solve-no-search-prints-root-and-an-r-row.md`](../../docs/decisions/read-out/Q-M1f.255-ein-solve-no-search-prints-root-and-an-r-row.md)

**Raised** 2026-11-14 by S1f.6.36's T1, on the user's words of that day, *"flag that disables search doesn't change R,
it disables discovering of R. R is not an output"*.

- **(a) Root, and an `R` row that says it was not searched**: `asked` (*root alone*), `∅` and its `owed` block as
  describe prints them, `R` (*not searched — --no-search builds no node past root*), `lattice` and `strata` (the static
  check), `universe` (H built, *none visited*), `route` (*none*), `work` (one saturation); a dead root's core; the
  queries read at ∅ where ∅ is in R; the claims. The `R` row reads the same on a dead root, where L0 decides R. Root's
  facts are drawn by the forms (`-p`, `-P`, `-f`, `--models`, `--diff-models`), whatever root is, and written as
  `root.ein` by `--dump-answer`; stdout does not list them.
- **(b) The same, and root's facts beyond the program as loaded listed on stdout by default**, within its 100 lines.
- **(c) On a dead root the `R` row says what L0 decides**, *every node is dead*.

**Defaulted 2026-11-14: (a), the user's to revise.**

### Q-M1f.256 — Which claims root decides under `--no-search`

**Decision file:** [`docs/decisions/answer/Q-M1f.256-under-no-search-a-claim-is-decided-as-far-as.md`](../../docs/decisions/answer/Q-M1f.256-under-no-search-a-claim-is-decided-as-far-as.md)

**Raised** 2026-11-14 by S1f.6.36's T1. A claim is about R, or a part of it, and `--no-search` builds no node past
root.

- **(a) What a description stopped after root decides**, under `ein test`'s refute rule (M1f S1f.6.27): on a dead
  root every claim, as the full run decides it (it too reads root alone there, L0); on an alive root a `(solve :expect
  (false))` refuted, and where ∅ is in R a query whose class at ∅ the claim does not list refuted; every other claim
  NOT CHECKED, the line saying whether ∅ is in R. The corpus: 57 claims held (every one a `(false)` on a dead root),
  one FAILED, 418 not checked, none other than the full run's (S1f.6.36's sweep).
- **(b) (a), and the refute rule extended to the region claims ∅ in R refutes**: `(region)`, `(region (owing …))`,
  and a `(region (model …))` none of whose models is root's state (E-57; it would hold of every partial run).
- **(c) Nothing decided**: every claim NOT CHECKED, a dead root's `(false)` included.

**Defaulted 2026-11-14: (a), the user's to revise.**

### Q-M1f.257 — `ein test --no-search`'s exit code

**Decision file:** [`docs/decisions/cli/Q-M1f.257-under-no-search-a-claim-not-checked-moves-no.md`](../../docs/decisions/cli/Q-M1f.257-under-no-search-a-claim-not-checked-moves-no.md)

**Raised** 2026-11-14 by S1f.6.36's T1. `ein test` exits 1 where a claim is FAILED or NOT CHECKED; under
`--no-search` most claims are not checked because the run was asked not to search.

- **(a) A claim not checked moves no exit code under `--no-search`**: 0 unless a claim FAILED (1) or a file is
  refused, aborted or states nothing (2). On the corpus no exit code moves from the run without the flag.
- **(b) As without the flag**: 1 wherever a claim is not checked.

**Defaulted 2026-11-14: (a), the user's to revise** (*"exit non-zero only on error"*, 2026-10-16).

### Q-M1f.258 — Which commands and options take `--no-search`

**Decision file:** [`docs/decisions/cli/Q-M1f.258-ein-solve-and-ein-test-take-no-search-and-ein.md`](../../docs/decisions/cli/Q-M1f.258-ein-solve-and-ein-test-take-no-search-and-ein.md)

**Raised** 2026-11-14 by S1f.6.36's T1.

- **(a) `ein solve` and `ein test`**; `ein oracle --no-search` refused, exit 2, with a sentence naming `ein solve FILE
  --no-search` (the oracle is R by brute force, the reference every mode is held to); `ein saturate`, `ein render` and
  `ein kb` never search and take no flag. Beside `-n`, `--single`, `--minimal`, `--maximal`, `--count` and
  `--monotone`, each a question about R or about the program, refused by the parser; beside every artefact and form
  flag (`--json-summary`, `--dump-answer`, `--dump-lattice`, `--trace`, `--events`, `-p`, `-P`, `-f`, `--models`,
  `--diff-models`, `-H`, `-y`), each over root.
- **(b) `ein oracle --no-search` reports root**, as `ein solve --no-search` does.
- **(c) A mode beside it answers what root decides**: `--minimal --no-search` lists ∅ where ∅ is in R.

**Defaulted 2026-11-14: (a), the user's to revise.**

### Q-M1f.259 — `--json-summary` under `--no-search`

**Decision file:** [`docs/decisions/read-out/Q-M1f.259-under-no-search-the-summary-states-no-region.md`](../../docs/decisions/read-out/Q-M1f.259-under-no-search-the-summary-states-no-region.md)

**Raised** 2026-11-14 by S1f.6.36's T1. `ein-summary/2` has one key set on every arm, and a run that states no R has
no `region` block to hold root's standing (`region.root`, `root_owes`, `root_owed`).

- **(a) `asked.ask = "no-search"`, `region` and `verdict` `null`** (as `--monotone`'s), the queries and the claim as
  checked, and **root's standing as three keys of the `root` block on every arm**, `dead`, `owes` and `owed`, read off
  root's fixpoint where every route starts; additive, the schema id unchanged.
- **(b) A `region` block holding root's three fields alone**, every other `null`.

**Defaulted 2026-11-14: (a), the user's to revise.** On the corpus the full run's `region.root`, `root_owes` and
`root_owed` equal the new keys on every program that writes a summary (517).

### Q-M1f.260 — Which of S1f.6.34's twenty-four say `:no-search`

**Raised** 2026-11-14 by S1f.6.36's T1, revisiting Q-M1f.246.

- **(a) The 21 whose question was root alone**, S1f.6.34's groups A and B, each back to its program and claim as
  they were before S1f.6.34 with `:no-search` in place of its `:no-hypothesis (…)`: no `as-stated` hrule
  (`tests/stdlib/bijection/01` to `03`), no fan-out written out (`tests/stdlib/slots/02`, `03`, `04`, `07`, `13`), no
  negatives added (`tests/region/04`, `05`; `tests/stdlib/slots/09`, `11`), and `tests/stdlib/algebra/23`, `25` back
  to their owing claim. `tests/stdlib/bijection/06` (the blind enumerator's), `tests/stdlib/closure/02` (O + C, which
  `:no-search` does not relax) and `examples/ein-bugs/closed-and-owing.ein` (the refusal's fixture) stay.
- **(b) (a), but `tests/stdlib/algebra/23` and `25` keep S1f.6.34's question**, both roles, the obligation proposing
  and R the three states that pay: their text holds no workaround, and they are the suite's programs in which an
  obligation's proposals are R.

**Defaulted 2026-11-14: (a), the user's to revise.** Every claim of the 21 holds under `ein solve`, `ein test` and
`ein oracle`, and `utils/stdlib_mutants.py` over `tests/` with the 20 of them it holds leaves the banked survivor set
exactly, 71 (the prototype, 2026-11-14).

### Q-M1f.261 — The decimal theory's module name, after the user wrote `std.math.arith` in passing

**Decision file:** [`docs/decisions/stdlib/math/decimal/Q-M1f.261-the-decimal-arithmetic-is-std-math-decimal-arith.md`](../../docs/decisions/stdlib/math/decimal/Q-M1f.261-the-decimal-arithmetic-is-std-math-decimal-arith.md)

**Raised** 2026-11-14 by S1f.6.31, re-reading
the replies before the move. Replies 16 and 52 of 2026-11-12 say *"rename arith to decimal-arith"*, and round 2's
layout, `std.math.decimal-arith` with the entry `decimal-arith`, was confirmed (*"Confirm others"*). Later, writing
about imports, the user named the module otherwise twice: *"let's call symbols full names
`std.math.arith.decimal...`"* (2026-11-13, `import_clashes.md` § 7)
and *"when import std.math.arith and use std.math.arith.sum must work identical to sum from import std.math.arith
:symbols (sum)"* (2026-11-14, Q-M1f.247), beside `sum`, which is no name of the module.

- **(a) `std.math.decimal-arith`, `stdlib/math/decimal-arith.ein`, entry `decimal-arith`**: the layout as replied
  and confirmed; the later mentions are examples about imports.
- **(b) `std.math.arith`, `stdlib/math/arith.ein`**, as the user wrote it on 2026-11-14; the entry `arith` or
  `decimal-arith`.
- **(c) `std.math.arith.decimal`, `stdlib/math/arith/decimal.ein`**, as written on 2026-11-13: round 2's option (b),
  one level deeper, which round 2's reply did not take.

**Defaulted 2026-11-14: (a), the user's to revise.** The move is mechanical and held by a sweep (S1f.6.31's T5), so
(b) or (c) is the same move again, with the same sweep, before S1f.6.16's build adds `std.math.binary-arith` beside
it.

### Q-M1f.262 — A record's link to a file the layout moved

**Raised** 2026-11-14 by S1f.6.31.
P1f.6's stage files are records and are not rewritten (the stage file's T6), and the link check, which the gate runs
over `plans/`, fails on a link to a path that moved: ten links in six of the phase's records point at a module or a
test directory the stage moved (`s1f.6.7`, `.8`, `.13`, `.16`, `.24`, `word_problems.md`), one of them at an anchor of
`stdlib/README.md` § `std.num9`, which the section's new name moved.

- **(a) Re-point the link's target**, `../../../stdlib/arith.ein` to `../../../stdlib/math/decimal-arith.ein`, and leave
  its text (`` [`stdlib/arith.ein`] ``) and the record's prose as they were: the record still says what it said, and
  the link reaches the file it meant.
- **(b) Leave the records alone** and exempt them from the link check.
- **(c) Rewrite the records to the new names.**

**Defaulted 2026-11-14: (a), the user's to revise.** The phase README's note maps every old name to the new.

### Q-M1f.263 — The sixteen layout rules of `std.bool.gates`, now that an import brings every rule

**Decision file:** [`docs/decisions/stdlib/bool/Q-M1f.263-every-program-that-imports-std-bool-gates.md`](../../docs/decisions/stdlib/bool/Q-M1f.263-every-program-that-imports-std-bool-gates.md)

**Raised** 2026-11-14 by S1f.6.17's build.
The plan's table says the relation-per-function layout `(And 1 1 1)` is *"derived only in a program that selects
them"*: written before S1f.6.35, when a selection brought only what it named. Since S1f.6.35 every import brings
its module whole, so no import can select the sixteen rules apart from the table.

- **(a) The sixteen rules in `std.bool.gates`, derived in every program that imports it**: 64 firings and 64 facts
  at root, beside the 64 rows, in each of the six applications and in every program over `std.bool.props`. The
  ruled module table as it stands; no answer moves (every application's R equal before and after, but for the
  names).
- **(b) Each rule gated by a fact a program writes**, `Xor-rows` with a premise `(layout-of Xor)`, say: the plan's
  *"selects"* in S1f.6.35's terms, nothing paid where nothing is written, one fact a function to write.
- **(c) A fifth module for the second layout.** Not as such: the relation `Xor` declared in another module would be
  that module's `Xor`, a name the table's rows do not hold, so the rules would match nothing unless the table's
  names were declared where the table is, which is (a).

**Defaulted 2026-11-14: (a); accepted by the user the same day.** The user, 2026-11-14: *"I accept all listed questions defaults: Q-M1f.263, Q-M1f.264 (E-62), Q-M1f.265, E-64"*. The cost is measured in `stdlib/README.md` § `std.bool`; the
change to (b) is sixteen premises and a relation, with the tests' claims unmoved.

### Q-M1f.264 — A function's name a program writes and does not list (E-62)

**Decision file:** [`docs/decisions/imports/Q-M1f.264-a-gate-functions-name-a-program-writes-and-does.md`](../../docs/decisions/imports/Q-M1f.264-a-gate-functions-name-a-program-writes-and-does.md)

**Raised** 2026-11-14 by S1f.6.17's build,
its check of a name that is both a relation and a value (T1f.6.17.4). Each of the sixteen names is declared by
`std.bool.gates` as the relation of the second layout (reply 19 (c)), so it is the module's in the table's rows too,
`std.bool.gates.Xor`. A program that writes `(gate g1 Xor a b c)` without listing `Xor` writes its own `Xor`, a name
no row has: the gate computes nothing, and nothing says so (S1f.6.35: *a short name no import lists is the file's
own, shadowing in silence*). `knights.ein` with `Nfst` left out of its list answers four states where two, and its
claims fail.

- **(a) Keep it**, the resolver as S1f.6.35 ruled; `std.bool.gates`' header, `stdlib/README.md` and the examples'
  README say a program lists every function's name it writes.
- **(b) A guard in `std.bool.circuit`**: a gate whose function has no `column-name` is a contradiction,
  `(absent (column-name ?f ?c))` over a closed, written table, so such a program is dead and its core names the
  gate. One rule, one `absent`, the module no longer free of negation.
- **(c) A loader note** where a file writes, unlisted, a short name a module it imports declares: an engine change,
  for every module, against S1f.6.35's *in silence*.

**Defaulted 2026-11-14: (a); accepted by the user the same day.** The user, 2026-11-14: *"I accept all listed questions defaults: Q-M1f.263, Q-M1f.264 (E-62), Q-M1f.265, E-64"*. (b) is the cheapest remedy and changes no answer of the corpus;
(c) reopens a ruling.

### Q-M1f.265 — The names and file shapes `std.bool`'s build chose

**Decision file:** [`docs/decisions/stdlib/bool/Q-M1f.265-each-std-bool-module-is-entered-by-its-last-name.md`](../../docs/decisions/stdlib/bool/Q-M1f.265-each-std-bool-module-is-entered-by-its-last-name.md)

**Raised** 2026-11-14 by S1f.6.17's build.
The rulings name the four modules and what each holds; these were the build's to choose:

- **the entries** are the modules' last names, `gates`, `props`, `circuit`, `inputs`, as `std.math.decimal-arith`'s is
  `decimal-arith`; the banked `props-derived` is `props`, and `require.ein`'s `requirements` is gone, the
  requirement theory inside `std.bool.inputs` (the ruled table's *"with `require`"*);
- **the sixteen layout rules** are `Zero-rows` to `One-rows`;
- **`stdlib/bool/gates.ein` is written by hand but for its rows**, which `utils/gen_gates.py` writes between two
  marked lines, so that the sixteen rules are hand-written as the ruled table says; every relation of the module,
  the table's among them, is closed by its closing rule, where `std.math.digits` closes its tables by written facts;
- **the expected property facts** are the claims of seven programs under `tests/stdlib/bool/props/`, a family of
  relations each, and the second layout's of one under `tests/stdlib/bool/gates/`, all written by the generator,
  rather than one program of thirty claims or a module of facts.

- **(a) As built.**
- **(b) Another name or shape**, named in the reply.

**Defaulted 2026-11-14: (a); accepted by the user the same day.** The user, 2026-11-14: *"I accept all listed questions defaults: Q-M1f.263, Q-M1f.264 (E-62), Q-M1f.265, E-64"*. A rename is a sweep of `tests/stdlib/bool/` and the six
applications, held by their claims and by `ein-cli/tests/import_invariance.rs`.

### Q-M1f.266 — How a base is connected to the shared rules

**Decision file:** [`docs/decisions/stdlib/math/Q-M1f.266-a-base-is-connected-to-the-shared-rules-by-one.md`](../../docs/decisions/stdlib/math/Q-M1f.266-a-base-is-connected-to-the-shared-rules-by-one.md)

**Raised** 2026-11-14 by S1f.6.16's design, on the user's ruling of that day: *"(a), show me the design
first, if they can't be connected with single hand written fact, if it require ~100 facts for that - then you need
a separate file that combines theories"*. A rule with parameters fires for a fact of its own name, so the 78 shared
rules take 78 activators; one hand-written form can derive them at root. Measured on the prototype (the stage's
record): the same work by every count but one firing; saturation over the shipped theory +3.3 to +3.8 % for (i),
+7.9 to +9.3 % for (ii), +3.9 to +4.0 % for (iii).

- **(i) 78 activator facts a base,** the trial's.
- **(ii) one fact a base,** `(positional operand operand-is n …)`, 63 names in the order of a fan-out rule's
  parameters, which derives the 78.
- **(iii) one call a base,** `(rule decimal-arith-positional () :match () :assert (positional operand …))`, a macro
  of `std.math.positional` whose body is the 78. A rule of its own until E-63 is
  fixed, and then the call can stand in the closing rule, with no firing.

And where the connection is written: **in the base's own module**, which imports the shared rules and the tables
and is what a program imports, so the base's module is the file that combines; or **in a separate combining file**
over a module of the base's relations and rules, which would list the base's 63 names in its import (S1f.6.35).

**Defaulted 2026-11-14: (iii), in the base's own module, the user's to revise.** One hand-written form, at the cost
the user took with (a); (ii) is the literal one fact at twice the saturation cost; a separate file buys nothing,
since nothing uses the base's relations without the shared rules.

**Ruled 2026-11-14: (iii).** The user: *"yes (iii); division by zero is false in binary arithmetic too, so it
belongs to arithmetic itself, not even to positional one right?"* (the second half is Q-M1f.269's ruling).

**Built 2026-11-15** by S1f.6.16's build: the call stands in each base's closing rule's `(and …)` (E-63 fixed), no rule of its
own and no firing; 77 shared rules and 61 parameters once S1f.6.38 had taken the numeral's check out.

### Q-M1f.267 — The binary theory's names

**Decision file:** [`docs/decisions/stdlib/math/binary/Q-M1f.267-binarys-names-carry-the-prefix-bin.md`](../../docs/decisions/stdlib/math/binary/Q-M1f.267-binarys-names-carry-the-prefix-bin.md)

**Raised** 2026-11-14 by S1f.6.16's design. Reply 14 took a word prefix, `bin-add` answered by
`bin-sum-of`; the plan kept the shape `b2` by default and filed the question for the build.

- **(a) `b2` kept; every other name `bin-`**: the requests and results, the internal relations and tags, binary's own
  rules and its connecting rule, its entry `binary-arith`, and the tables of `std.math.bits`, `bin-fa`, `bin-fs`,
  `bin-cl` and `bin-nz`; the conversions `to-bin`, `bin-of`, `to-dec` and `dec-of`, as banked.
- **(b) The shape `bin` too**, so that no name ends in a digit.
- **(c) The tables unprefixed**, `fa`, `fs`, `cl`, `nz` in `std.math.bits`: their full names already differ from
  `std.math.digits`' (S1f.6.35), and a file lists one or the other.

**Defaulted 2026-11-14: (a), the user's to revise.** As `std.math.int` prefixes every name with `int-`.
**Built so** 2026-11-15 by S1f.6.16's build, with binary's representation module's names on the same rule (Q-M1f.295).

### Q-M1f.268 — How far binary flags a malformed request

**Decision file:** [`docs/decisions/stdlib/math/binary/Q-M1f.268-binary-flags-a-malformed-request-as-far-as.md`](../../docs/decisions/stdlib/math/binary/Q-M1f.268-binary-flags-a-malformed-request-as-far-as.md)

**Raised** 2026-11-14 by S1f.6.16's design. The plan's T1f.6.16.7 says *"the detection of malformed
numbers as far as the shared rules bring it, and the page saying what stays silent"*. The shared rules bring all of
S1f.6.12's check but the two rules that list the digits, `check-digit` and `operand-undeclared`.

- **(a) As far as decimal**: binary writes those two over `0` and `1`, so a bit's place holding anything else and an
  undeclared name are flagged, and binary is silent where decimal is.
- **(b) The shared rules alone**: `(b2 z 7)` and a name no `bin-let` states are answered by silence, and the page
  says so.

**Defaulted 2026-11-14: (a), the user's to revise.** Two rules, measured on the prototype: 234 sampled requests
right, each answered once. **Built so** 2026-11-15 by S1f.6.16's build, on S1f.6.38's split: the digit's place is
`std.math.binary`'s check where a numeral is written, the undeclared name `bin-operand-undeclared`.

### Q-M1f.269 — `division-by-zero-is-false` stays decimal's

**Decision file:** [`docs/decisions/stdlib/math/Q-M1f.269-division-by-zero-is-a-contradiction.md`](../../docs/decisions/stdlib/math/Q-M1f.269-division-by-zero-is-a-contradiction.md)

**Raised** 2026-11-14 by S1f.6.16's trial.
`std.math.num9`'s closing rule names the rule, `(num9-rule division-by-zero-is-false)`, and a rule's name written as
an argument makes every parameterised conclusion of its arity a possible activator (S1f.5.18, E-55), so a shared,
parameterised one refuses `std.math.num9`.

- **(a) It stays `std.math.decimal-arith`'s**, and binary has `bin-division-by-zero-is-false`.
- **(b) It is shared**, and `std.math.num9`'s closing rule stops naming it: its states lose one fact.

**Defaulted 2026-11-14: (a), the user's to revise.** One rule of one premise, and `std.math.num9` untouched.

**Ruled 2026-11-14: neither; arithmetic's own.** The user, with Q-M1f.266: *"division by zero is false in binary
arithmetic too, so it belongs to arithmetic itself, not even to positional one right?"*, and on S1f.6.38: *"z is
shared by both bases, put it in std.math.arith"*. So the relation `division-by-zero` and the rule
`division-by-zero-is-false` leave `std.math.decimal-arith` for a new module, `std.math.arith`, arithmetic itself,
beside `z`, unparameterised; both bases import it, and `std.math.positional` does not hold it. The concern that
raised the question does not arise: the rule has no parameter, so `std.math.num9`'s closing rule may go on naming
it, by its new full name. S1f.6.38's build creates the module, so that every
program's imports are rewritten once. **Built** 2026-11-15 by S1f.6.16's build: the shared `div-by-zero` and
`div-call-by-zero` write `std.math.arith`'s flag, and binary has no rule of its own for it.

### Q-M1f.270 — The connecting macro's body, written or generated

**Decision file:** [`docs/decisions/stdlib/math/Q-M1f.270-the-connecting-macros-body-is-written-by-hand.md`](../../docs/decisions/stdlib/math/Q-M1f.270-the-connecting-macros-body-is-written-by-hand.md)

**Raised** 2026-11-14 by S1f.6.16's design. The macro `positional` activates each of the module's 78 rules
once with that rule's parameters, so its body repeats every rule's parameter list.

- **(a) Written by hand**, beside the rules it activates, and held by a test that every rule is activated once, with
  its parameters in their order.
- **(b) Generated** from the module's rules by a script with a `--check` in the gate.

**Defaulted 2026-11-14: (a), the user's to revise.** The module stays a file a reader can follow, as the plan's
reply 16 wanted of `std.math.decimal-arith`; the test fails on the drift a generator would prevent. **Built so**
2026-11-15 by S1f.6.16's build: `arith_theory.rs`' `one_call_of_the_macro_connects_each_base`.

### Q-M1f.271 — What an hrule's `(and …)` conclusion proposes

**Decision file:** [`docs/decisions/language/Q-M1f.271-an-hrules-conclusion-is-one-fact.md`](../../docs/decisions/language/Q-M1f.271-an-hrules-conclusion-is-one-fact.md)

**Raised** 2026-11-14 by S1f.6.37's
inventory of what reads a conclusion (E-65, W95). An hrule proposes facts, each a commitment of its own, and the
hypothesis layer read the first template of its plan alone: `(hrule h () :match () :assert (and (p a) (q a)))`
proposed `(p a)` and dropped `(q a)` in silence, and the mechanisms check and the universe's scope took `p` alone
for the relation it proposes. No tracked hrule writes an `(and …)`, as written or after a macro's expansion.

- **(a) Refused at load**: an hrule proposes one fact per match, so its conclusion, read flat as every conclusion is
  since S1f.6.37, is one conjunct; one of more is refused, one line, *write one hrule per conclusion*.
- **(b) Each conjunct proposed**: `(and (p a) (q a))` proposes `(p a)` and `(q a)`, two candidates, as two hrules
  propose them; the hypothesis layer, the mechanisms check and the scope read every conjunct.
- **(c) As it was**, the first conjunct, and a `note:` naming the rest.

**Defaulted 2026-11-14: (a); accepted by the user the same day** (*"Q-M1f.271 (a) is fine, E-66 and E-68 - to followups"*). It adds no meaning: what (b) would mean is written as two
hrules, which the refusal's line says, so a reading taken later moves no program written meanwhile; (b) reads a
conjunction as *each of these is a guess on its own*, where a reader may take it for *these together*, which no
commitment states. (c) keeps the silent drop's answer.

### Q-M1f.272 — How a representation module sees a numeral, right away

**Decision file:** [`docs/decisions/language/Q-M1f.272-a-representation-module-reads-what-a-file-writes.md`](../../docs/decisions/stdlib/math/Q-M1f.404-a-numeral-is-checked-where-an-operation-takes-it.md)

**Raised** 2026-11-14 by S1f.6.38's T1, the design
(`6cca4406`), on Q-M1f.210's ruling: a malformed numeral is the representation's problem, detected as soon as it is
written. No rule-level way is *right away* (the stage's § 2).

- **(a) E1, an engine stage first:** the loader states every nested fact a program writes as `(written T)`, and
  the representation's rules read it, so a malformed numeral is flagged as root's first or second firing, wherever
  it is written.
- **(b) E2:** a constructor's argument domain checked at load, a refusal; a malformed program a load error.
- **(c) W1:** a rule-level walk of every fact by rules whose head is a parameter: two to three times the firings,
  three to seven times the time, a trace of 2 081 steps, and a load error beside `std.algebra` (E-69).
- **(d) None:** S1f.6.12's pass at a request's operand, which the ruling rules out.

**Defaulted 2026-11-14: (a). Ruled the same day: (a).** The user: *"E1 yes, separate std.math.decimal module, other
defaults fine"*.

**Reverted 2026-11-18** by [Q-M1f.404](#q-m1f404--written-is-erased-a-numeral-is-checked-by-the-operation-that-takes-it): `written` is erased, and a numeral is checked by the operation that takes it.

### Q-M1f.273 — Where decimal's numeral check lives

**Decision file:** [`docs/decisions/stdlib/math/decimal/Q-M1f.273-the-decimal-numeral-is-std-math-decimal-s-which.md`](../../docs/decisions/stdlib/math/decimal/Q-M1f.273-the-decimal-numeral-is-std-math-decimal-s-which.md)

**Raised** 2026-11-14 by S1f.6.38's T1.

- **(a) `std.math.decimal-arith`'s own rules,** beside `n` and `let`: no program's imports change.
- **(b) A new `std.math.decimal`** that declares `n` and holds the check, `decimal-arith` importing it: the
  representation apart from its arithmetic, and every program that writes a numeral lists `n` from it (S1f.6.35, no
  re-export).
- **(c) `std.math.digits`:** no, a library of generated facts.

**Defaulted 2026-11-14: (a). Ruled the same day: (b),** against the default (the user, as Q-M1f.272 quotes). By
the same ruling's symmetry binary's numeral is `std.math.binary`'s, built by S1f.6.16's build on this pattern: a
default, the user's to revise. Open in the build, a default it files: how a prefix that is a named constant,
`(n price 1)`, is flagged from the module below `let`.

### Q-M1f.274 — The numeral's flag

**Decision file:** [`docs/decisions/stdlib/math/decimal/Q-M1f.274-a-malformed-numeral-is-flagged-on-its-smallest.md`](../../docs/decisions/stdlib/math/decimal/Q-M1f.274-a-malformed-numeral-is-flagged-on-its-smallest.md)

**Raised** 2026-11-14 by S1f.6.38's T1.

- **(a) `(not-decimal NUMERAL THING)`** on the smallest `n` term at fault, `(false)` from it by
  `not-decimal-is-false`; binary's `(not-binary NUMERAL THING)`.
- **(b) On the numeral as written,** the outermost `n`, which takes a walk down it again, a firing a place.
- **(c) `(malformed NUMERAL)`,** the request flag's relation reused.

**Defaulted 2026-11-14: (a); accepted by the user the same day** (*"other defaults fine"*).

### Q-M1f.275 — What the loader states as written

**Decision file:** [`docs/decisions/language/Q-M1f.275-written-states-every-fact-a-file-nests.md`](../../docs/decisions/stdlib/math/Q-M1f.404-a-numeral-is-checked-where-an-operation-takes-it.md)

**Raised** 2026-11-14 by S1f.6.38's T1.

- **(a) The nested facts of every written fact, and the ground nested facts of every rule's and hrule's
  conclusion:** a malformed numeral written whole in a rule's text is caught, and one a rule assembles from a
  variable and a non-digit is silent and named so.
- **(b) Written facts only:** both of those silent, two detections today's pass has, lost.
- **(c) (a), and E2's static check of rule texts** for the assembled case.

**Defaulted 2026-11-14: (a); accepted by the user the same day** (*"other defaults fine"*).

**Reverted 2026-11-18** by [Q-M1f.404](#q-m1f404--written-is-erased-a-numeral-is-checked-by-the-operation-that-takes-it): `written` is erased, and a numeral is checked by the operation that takes it.

### Q-M1f.276 — An integer's and a rational's own shape

**Decision file:** [`docs/decisions/stdlib/math/Q-M1f.276-an-integers-and-a-rationals-own-shape-is-checked.md`](../../docs/decisions/stdlib/math/Q-M1f.276-an-integers-and-a-rationals-own-shape-is-checked.md)

**Raised** 2026-11-14 by S1f.6.38's T1.

- **(a) Flagged:** int and rat flag a digit or a named constant where their own term's part goes, `(int 7 z)`,
  `(int price z)`, `(rat (int …) 7)`, read off `written`: `(not-integer …)`, `(not-rational …)`, six rules and a
  flag each.
- **(b) Silent,** named on the page.

**Defaulted 2026-11-14: (a); accepted by the user the same day** (*"other defaults fine"*).

**Reverted 2026-11-18** by [Q-M1f.404](#q-m1f404--written-is-erased-a-numeral-is-checked-by-the-operation-that-takes-it): `written` is erased, and a numeral is checked by the operation that takes it.

### Q-M1f.277 — An operand of another type in int and rat

**Decision file:** [`docs/decisions/stdlib/math/Q-M1f.277-an-operand-of-another-kind-of-number-is-answered.md`](../../docs/decisions/stdlib/math/Q-M1f.277-an-operand-of-another-kind-of-number-is-answered.md)

**Raised** 2026-11-14 by S1f.6.38's T1.

- **(a) Flagged:** int and rat flag a natural where an integer goes, and an integer or a natural where a rational
  goes, at their operand.
- **(b) Silent,** as S1f.6.12 left it, and named on the pages: it is not in Q-M1f.210's ruling's list.

**Defaulted 2026-11-14: (b); accepted by the user the same day** (*"other defaults fine"*).

### Q-M1f.278 — How a module below `let` flags a named constant inside a numeral

**Decision file:** [`docs/decisions/stdlib/math/decimal/Q-M1f.278-a-named-constant-is-told-by-named-the-numeral.md`](../../docs/decisions/stdlib/math/decimal/Q-M1f.278-a-named-constant-is-told-by-named-the-numeral.md)

**Raised** 2026-11-14 by S1f.6.38's
build, the item [Q-M1f.273](#q-m1f273--where-decimals-numeral-check-lives)'s ruling left open: `std.math.decimal` is
below `std.math.decimal-arith`, which declares `let`, so it cannot read `let` to say that `price` in `(n price 1)` is a
named constant.

- **(a) `named` is `std.math.decimal`'s**, a name that stands for a number: `std.math.decimal-arith`'s `let-named`
  writes it, as it did, and decimal's `numeral-prefix-named` probes it, `(and (written (n ?m ?a)) (neq ?m z) (named
  ?m))`. One probe a place, a well-formed numeral costing it nothing; `(n end 1)` stays silent, as it was.
- **(b) E-36's walk of the program's names** in `std.math.decimal`, *the prefix is a name, and no digit and not `z`*:
  `(neq ?v z) (eq ?v ?m)` and ten `neq`. It flags `(n end 1)` too, which no rule flagged, and costs every written
  numeral two comparisons a name of the program (measured in the stage's record).
- **(c) The rule in `std.math.decimal-arith`**, deriving decimal's `not-decimal`: a module other than the
  representation's names a numeral's fault, which the acceptance rules out.

**Defaulted 2026-11-14: (a), the user's to revise.** What makes a name a named constant stays the module's that
declares `let`, the module below reads it by a relation of its own, and nothing is paid that the check at the
operand did not pay.

### Q-M1f.279 — What a `written` fact is derived by, and where it shows

**Decision file:** [`docs/decisions/language/Q-M1f.279-a-written-fact-is-derived-by-the-loaders-step.md`](../../docs/decisions/stdlib/math/Q-M1f.404-a-numeral-is-checked-where-an-operation-takes-it.md)

**Raised** 2026-11-14 by S1f.6.38's
T1f.6.38.2. The ruled design: a core that rests on a stated fact names the fact as written, and `written` shows where
the declarations' mirrors show.

- **(a) Derived by the loader's step `#written`** (`ein_core::terms::WRITTEN_STEP`, one of the names the loader makes
  up), from the fact it was read in, its one premise, or from no fact where a rule's text writes it: every walk
  follows it to the written fact, so the core needs no case of its own. `ein saturate --dump` lists it under
  `DERIVED`, `:rule "#written"`, and counts it so; a KB view draws it; a state file, `ein render`'s KB text, `-P`'s
  blocks and a model's comparison leave it out, the loader stating it again.
- **(b) A background fact, as a mirror is**, with no provenance, and the explanation mapping it to the fact it was
  read in by a table of its own: the dump lists it with the mirrors, and every walk learns one case.

**Defaulted 2026-11-14: (a), the user's to revise.**

**Reverted 2026-11-18** by [Q-M1f.404](#q-m1f404--written-is-erased-a-numeral-is-checked-by-the-operation-that-takes-it): `written` is erased, and a numeral is checked by the operation that takes it.

### Q-M1f.280 — A program that writes `written`

**Decision file:** [`docs/decisions/language/Q-M1f.280-a-program-does-not-write-written.md`](../../docs/decisions/stdlib/math/Q-M1f.404-a-numeral-is-checked-where-an-operation-takes-it.md)

**Raised** 2026-11-14 by S1f.6.38's
T1f.6.38.2.

- **(a) Refused at load**, a fact of it, a stored negation of one, and a rule's or an hrule's conclusion of one, one
  line each, on every command (`defined_behaviour.md` § 4.30); no declarator binds the name (`ein_core::RESERVED`).
  So `written` is decided at root, and holds what the files wrote.
- **(b) A fact like any other**: written, it is one more statement; concluded, a commitment could extend it, and a
  check that reads it would no longer be decided at root.

**Defaulted 2026-11-14: (a), the user's to revise.**

**Reverted 2026-11-18** by [Q-M1f.404](#q-m1f404--written-is-erased-a-numeral-is-checked-by-the-operation-that-takes-it): `written` is erased, and a numeral is checked by the operation that takes it.

### Q-M1f.281 — A `let` that names a digit

**Decision file:** [`docs/decisions/stdlib/math/decimal/Q-M1f.281-a-let-that-names-a-digit-is-flagged.md`](../../docs/decisions/stdlib/math/decimal/Q-M1f.281-a-let-that-names-a-digit-is-flagged.md)

**Raised** 2026-11-14 by S1f.6.38's
build. Until the stage a request that named `seven`, beside `(let seven 7)`, was flagged by the check its value was
handed to, `check-prefix-digit`; with the numeral's check moved to `std.math.decimal`, nothing read the value.

- **(a) The `let`'s fault**, `let-to-digit` and `let-to-digit-zero` beside `let-to-name`, `(malformed (let seven 7))`,
  whether or not a request names it.
- **(b) Silent**, a detection the stage would lose.

**Defaulted 2026-11-14: (a), the user's to revise.** A `let`'s value is the `let`'s to read, as `let-to-name`'s is.

### Q-M1f.282 — The flag of an integer division whose operand is at fault

**Decision file:** [`docs/decisions/stdlib/math/int/Q-M1f.282-an-integer-division-with-an-operand-at-fault.md`](../../docs/decisions/stdlib/math/int/Q-M1f.282-an-integer-division-with-an-operand-at-fault.md)

**Raised** 2026-11-14 by S1f.6.38's
T1f.6.38.4. `std.math.int`'s three divisions share their rules under the tag `(int-first (int-div-of REQUEST) Y)`,
so `int-bad-first` takes an operand's fault to `(bad-request (int-div-of REQUEST) FAULT)`, a tag and no request.

- **(a) Two flags, the tag's and the request's:** a fourth rule, `int-bad-div`, takes `(int-div-of REQUEST)` to the
  request as written, as a public division by zero holds its request's flag and its call's (Q-M1f.211 (ii)).
- **(b) The tag's alone**, the three rules of the design: the core still names the request.
- **(c) The divisions' tags restructured**, so that their operand's tag holds the request: every division's state
  respelt.

**Defaulted 2026-11-14: (a), the user's to revise.**

### Q-M1f.283 — The two new modules' names and their place

**Decision file:** [`docs/decisions/stdlib/math/Q-M1f.283-zero-is-a-relation-of-no-argument-and-each.md`](../../docs/decisions/stdlib/math/Q-M1f.283-zero-is-a-relation-of-no-argument-and-each.md)

**Raised** 2026-11-14 by S1f.6.38's
T1f.6.38.3. Defaulted as built, each the user's to revise: the entries `arith` and `decimal`, and the relations their
closing rules name rules by, `arith-module-rule` (`arith-rule` being `std.math.decimal-arith`'s) and `decimal-rule`;
`z` declared in `std.math.arith` as a relation of no argument that no fact is of, the one way Ein declares a name,
so that it has one full name, and no hypothesis target, its signature being empty; every closing rule of the
calculator naming the entries of the calculator's modules it imports, `std.math.decimal-arith` naming `arith` and
`decimal`; and `std.math.num9` keeping a bare import of `std.math.decimal-arith`, from which it lists nothing now,
so that a program of nine digits holds the modules it held, beside its imports of `std.math.decimal` and
`std.math.arith` for `n`, `z` and the division by zero.

### Q-M1f.284 — The worker pool's count, its heavy lane and its memory bound

**Raised** 2026-11-15 by S1f.6.39's T4. The stage file set the
count, `EIN_TEST_JOBS`, at 16 by default and at most the machine's threads, and left the lane and the bound to T1's
measurement: no corpus program's process went past 529 MB through the CLI or 272 MB in-process, and no pooled binary
past 1.80 GiB at sixteen workers (the stage's record).

- **(a) 16, a quarter of them for the heavy lane, the bound 24 GB**: the P-cores' sixteen threads' worth here, the
  runner's four by the cap; a program on `ein_corpus::pool::HEAVY` (T1's 256 MB or more: `send_more_money`,
  `features/04_open`) runs at most four at once.
- **(b) Every thread, 32**: the E-cores beside the P-cores, each E-core 1.7 times slower a thread (§ Today).
- **(c) The P-cores alone, by `taskset`**: no unit on an E-core.

**Defaulted 2026-11-15: (a), the user's to revise.**

### Q-M1f.285 — The nightly's and the dev host's test steps on the gate profile

**Raised** 2026-11-15 by S1f.6.39's T5. The nightly ran its test
steps `--release` (release codegen, no debug checks), the per-commit tier the dev profile (`opt-level = 1`, the checks
on); the gate profile is release codegen with the checks.

- **(a) Every test step of every tier on the gate profile**: the debug assertions and the overflow checks run on the
  slow cells too, which no tier checked them on; `corpus_cli`'s cost band reads release codegen as before; the
  mutation census runs the gate profile's `ein`, which the nightly's steps built.
- **(b) The per-commit tier on the gate profile, the nightly and the dev host `--release`**, as they were.

**Defaulted 2026-11-15: (a), the user's to revise.** `release.yml`, the release tier, is unchanged: it tests the
binary that ships.

### Q-M1f.286 — `id_order_invariance` keeps its plain run

**Raised** 2026-11-15 by S1f.6.39's T3,
E-72. The stage file had the shuffled run held to `corpus_shapes`' golden and the
plain run computed only on a mismatch; the shuffled id space is a permutation of the plain run's own table, so the
plain run cannot be skipped.

- **(a) As built**: the plain run, then the shuffled one, on the pool; the sweep checks what it checked.
- **(b) A permutation of a smaller table** (the load's), the plain run on a mismatch only: the facts a run derives keep
  their derivation order under it, the perturbation the file's own note calls none, where M1e2 S1e2.2.12's leak was.

**Defaulted 2026-11-15: (a), the user's to revise.** Removing a pass is T6's
[Q-M1f.287](#q-m1f287--one-pass-for-corpus_shapes-and-id_order_invariance).

### Q-M1f.287 — One pass for `corpus_shapes` and `id_order_invariance`

**Raised** 2026-11-15 by S1f.6.39's
T6, from E-72. `id_order_invariance`'s plain run of every (file, output) is the run
`corpus_shapes` digests: the same bytes, computed twice a commit, 30.6 s of `corpus_shapes` here (`b8c931f3`).

- **(a) Merge**: `id_order_invariance` digests its plain run against the golden, its floors and `EIN_BLESS` with it,
  and `corpus_shapes`' own pass goes; the re-bless command and the pages that name `corpus_shapes` follow. Leaves the
  two at about 65 s here, where they are 96 s.
- **(b) Keep both.**

**Ruled 2026-11-15: (a)** (the user, 2026-11-15: *"T6 rulings: all as you recommended"*). Built by S1f.6.39's T7: `id_order_invariance`
digests its plain run against `corpus_shapes.md5`, its floors, the checkout's stdlib and `EIN_BLESS` with it, and
`corpus_shapes.rs` is gone; every check it made is made by the merged pass. Probe (T6, before the ruling): `id-leak`
moved 273 renderings of the golden and 483 pairs of the permuted sweep; a merged test asserts both.

### Q-M1f.288 — `jobs_invariance` over the programs that search

**Raised** 2026-11-15 by S1f.6.39's T6. A job count reaches only a region route's fan-out, which has more than one node
to fan out only in a program that searches: the 188 corpus programs whose describe saturates more than root
(`probes/sets.py`).

- **(a) The 188**, the list computed by the test from the program (a describe that saturates past root) rather than
  banked. `jobs-leak` moved 983 cells over them, of 987 over all (`t6.tsv.sets`). Leaves 7.5 s here, where it is
  20.2 s.
- **(b) Every program**, as now.

**Ruled 2026-11-15: (a)** (the user, 2026-11-15: *"T6 rulings: all as you recommended"*). Built by S1f.6.39's T7, with Q-M1f.290 (a): per commit
the programs outside `tests/` that search, `corpus_ops::searches` (describe at the sweep's budget saturates more than
root), asked of each program; under `EIN_CORPUS_SLOW=1` every corpus program, the nightly's. Five other binaries catch
the same leak on fixed programs at four and eight jobs (`explore`, `region`, `register_region`, `region_modes`,
`line_modes_past_the_limit`).

### Q-M1f.289 — `id_order_invariance` over the programs that search

**Raised** 2026-11-15 by S1f.6.39's T6.

- **(a) The 188 programs that search**: `id-leak` moved 144 pairs over them (483 over all); the 33 frontend and shape
  outputs of the 629 others are no longer permuted, the load's shape and refusals still being `kb_semantics`' on every
  file. Leaves 14.6 s here, where it is 65.1 s.
- **(b) Every program, the region outputs over the 188 only.**
- **(c) Every program**, as now.

**Ruled 2026-11-15: (a), the full-corpus sweep moved to the nightly** (the user, 2026-11-15: *"T6 rulings: all as you recommended"*). Built by S1f.6.39's T7, with
Q-M1f.290 (a): per commit the permutation runs over the programs outside `tests/` that search, the golden over every
program outside `tests/`; the nightly runs both over every corpus program, in shards (Q-M1f.293).

### Q-M1f.290 — `tests/` out of the render and determinism sweeps

**Raised** 2026-11-15 by S1f.6.39's T6, § Today's candidate. The 220 programs under `tests/` are each written for one
stdlib rule's claim, and are 48 % of `corpus_shapes`' work and 52 % of `id_order_invariance`'s; their claims are
checked by `ein test` (`corpus_cli`'s and `test_cli`'s cells), the coverage census and the mutants.

- **(a) `corpus_shapes`, `id_order_invariance`, `jobs_invariance` and `dot_wellformed` over the 597 programs outside
  `tests/`**, the golden and `dot_wellformed`'s skip list re-banked for them. Every probe each caught over the corpus
  it still caught (`id-leak` 269 pairs, `jobs-leak` 942 cells, `neq-nested` and `states-maximal` renderings moved
  outside `tests/`); `neq-nested` is caught on `tests/` itself by `corpus_cli`, `test_cli` and `arith_search_free`.
  Leaves the four at 66.5 s here, where they are 129.5 s.
- **(b) With Q-M1f.288 and .289**: the determinism sweeps over the 179 programs that search outside `tests/`.
- **(c) As now.**

**Ruled 2026-11-15: (a), the same way** (the user, 2026-11-15: *"T6 rulings: all as you recommended"*): `tests/` leaves `corpus_shapes`' golden (now
`id_order_invariance`'s), the two determinism sweeps and `dot_wellformed` per commit; the nightly runs them over the
full corpus. Built by S1f.6.39's T7: the golden is one file, held per commit over the lines of the programs a run reads and
in the nightly over all of them, so nothing was re-banked; `dot_wellformed`'s skip list per commit is its entries
outside `tests/`. A corpus program with no golden line fails per commit.

### Q-M1f.291 — The five sweeps that set modes and routes against one another

**Raised** 2026-11-15 by S1f.6.39's T6, § Today's candidate: `explore`, `lever_invariance`, `route_invariance`,
`region_modes`, `claim_routes`. The probes: a state-route bug (`states-maximal`) was caught by `explore`,
`route_invariance`, `region_modes` and `claim_routes`, and by `region`'s, `register_region`'s and the golden's own
checks; a route-1 bug (`route1-maximal`) only by `region::route_1_is_the_oracle…` (within the oracle's limit) and by
`route_invariance`, on programs past the limit where nothing else holds route 1; `lever_invariance` caught none of the
six, its claim, that no config moves an answer, being held by no other sweep.

- **(a) Keep the five**; `route_invariance` per commit without the `slow` entries (the zebra family and
  `send_more_money`, 127.8 of its 165.0 s at one worker, `units-gate-j1.tsv`), which its slow half on the dev host
  runs at both budgets. Leaves it at about 8 s here, where it is 26.8 s; the zebra family's route equality leaves CI
  unless the nightly takes the slow half (Q-M1f.293).
- **(b) `route_invariance` over the programs past the oracle's limit only**, the rest being `region`'s two oracle
  sweeps' claim.
- **(c) As now.**

**Ruled 2026-11-15: (a)** (the user, 2026-11-15: *"T6 rulings: all as you recommended"*): keep the five; `route_invariance`'s `slow` entries go to its slow half,
the nightly's. Built by S1f.6.39's T7 (where its 2²⁵ half runs: the record). No probe here argues for removing
`explore`, `region_modes` or `claim_routes`: each was one of four to catch the state-route bug, so the overlap is real,
but no single probe shows which one a fault would reach alone.

### Q-M1f.292 — One load a program, shared by the outputs that do not change it

**Raised** 2026-11-15 by S1f.6.39's T6, § Today's candidate. Of a `corpus_shapes` pass at the gate profile, 6.2 s is
`ir[resolve]` alone, reading and parsing the program's stdlib modules, and about 30 of the 41 outputs resolve again:
about 61 % of each of the three A sweeps' work.

- **(a) Load once a program and clone the loaded state for each output**: `Ast`, `Terms` and `Kb` would need
  `Clone`, which `Terms` and `Ast` do not derive — a change under `ein.rs/crates/*/src` beyond a test helper, so the
  user's to allow. Proven by invariance: `corpus_shapes`' golden reproduced byte for byte, and `id_order_invariance`'s
  permuted runs from the clone. Would leave the A sweeps at roughly half their 116 s here.
- **(b) Cache the resolved stdlib modules in the test helper** without a clone: not possible without the resolver's
  cooperation, which is the engine's.
- **(c) As now.**

**Ruled 2026-11-15: (a)** (the user, 2026-11-15: *"T6 rulings: all as you recommended"*): one load a program, cloned for each output, `Clone` on `Ast` and `Terms` (and
`Kb`), an engine source change under which no output may move. Built by S1f.6.39's T7 (`corpus_ops::Loads`), and
verified by invariance: `corpus_shapes.md5` reproduced byte for byte from the copies, every other golden unchanged, and
the gate green (the run at one worker stopped after 99 binaries, every one `ok`, as redundant: T4 holds the count).

### Q-M1f.293 — The nightly's steps against five minutes

**Raised** 2026-11-15 by S1f.6.39's
T6, against [Q-M1f.102](#q-m1f102--what-the-ci-nightly-keeps-and-what-the-dev-host-runs)'s rule that a nightly step fits
in five minutes on the runner. On four P-core threads with four workers, the runner's count (`taskset -c 0-3 env
EIN_TEST_JOBS=4 ./run_tests.sh --nightly`, 2026-11-15): the mutation census 1 382.9 s, `corpus_cores`' slow entries
376.3 s, `answer_round_trip`'s slow and `heavy_dump` entries 353.0 s, and `region`'s slow half 271.7 s here, over five
minutes at the runner's 1.13 to 1.34 times (Q-M1f.103). The deep-corpus job is 47.7 min at that count.

- **(a) The four to the dev host**, each named in `DEV_HOST_ONLY` with why; the nightly keeps the slow cells, the
  levers', the imports' and `obligation_rung`'s slow halves, the fuzzer and the bench, about 8 min; the census and
  the four slow halves run where nothing schedules them.
- **(b) The census by shards**: a seventh of the mutants a night, the shard named by the weekday, `--check` on the
  shard's banked survivors, the week covering all 572; about 3.3 min a night. The three slow halves split by program
  the same way, or (a) for them.
- **(c) The rule relaxed for the census** (it guards the suite's sensitivity, which no per-commit step guards), the
  three slow halves to (a) or (b).
- **(d) As now**, past the rule.

**Ruled 2026-11-15: split into parallel nightly jobs (shards), staying in CI** (the user, 2026-11-15: *"T6 rulings: all as you recommended"*). The nightly steps over five
minutes at the runner's four threads (the mutation census, `corpus_cores`' and `answer_round_trip`'s slow halves,
`region`'s slow half) and the full sweeps Q-M1f.289 and .290 add are sharded into parallel nightly jobs, each shard
at most five minutes measured as T6 did (`taskset -c 0-3`, `EIN_TEST_JOBS=4`, on AC). Built by S1f.6.39's T7:
`EIN_TEST_SHARD=k/n`, `ein_corpus::shard` and `stdlib_mutants.py --shard`; the shards and their times are the record's.

### Q-M1f.294 — `answer_dump`'s `heavy_dump` witness, and the feature tests the pool does not reach

**Raised** 2026-11-15 by S1f.6.39's T6. Two per-commit costs no sweep's program set explains:

- `answer_dump` dumps W8's witness `square-unique/corner-house.ein`, a `heavy_dump` entry, 33.2 s of its 33.8 s here;
  `answer_round_trip` left the `heavy_dump` entries to the nightly on the user's ruling of 2026-11-08 (P-16). **(a)**
  the same for `answer_dump` (leaves about 5 s); **(b)** as now.
- `int_theory`, `arith_batch`, `read_out_reference`, `read_out_examples` and `arith_theory`, feature tests, are
  65.5, 36.0, 23.3, 18.7 and 16.4 s here, 160 s of the gate's 887: **(a)** their loops on the pool (no check moves);
  **(b)** their long halves to the nightly; **(c)** as now.

**Ruled 2026-11-15: (a) and (a)** (the user, 2026-11-15: *"T6 rulings: all as you recommended"*): the `heavy_dump` witness to the nightly, and the five feature tests
on the pool where their loops allow it. Built by S1f.6.39's T7: `answer_dump` per commit leaves the `heavy_dump`
entries' runs, `EIN_CORPUS_SLOW=1` reads them; `int_theory`'s, `arith_theory`'s and `arith_batch`'s loops over
operations, `read_out_reference`'s example runs and `read_out_examples`' blocks are units of the pool. Neither is a
corpus sweep of § Today's 31.

### Q-M1f.295 — Binary's numeral is `std.math.binary`'s

**Decision file:** [`docs/decisions/stdlib/math/binary/Q-M1f.295-the-binary-numeral-is-std-math-binary-s-which.md`](../../docs/decisions/stdlib/math/binary/Q-M1f.295-the-binary-numeral-is-std-math-binary-s-which.md)

**Raised** 2026-11-15 by S1f.6.16's build. Q-M1f.210's ruling, *"malformed decimal numbers is a problem of decimal module,
malformed binary numbers - of binary module"*, and S1f.6.38's split (a numeral's module apart from its arithmetic,
Q-M1f.273 (b)) leave the shape of binary's representation to the build.

- **(a) A module `std.math.binary` on `std.math.decimal`'s pattern**: it declares `b2` and `bin-named`, and four
  rules over the loader's `(written (b2 P A))` flag `(not-binary NUMERAL THING)`, `not-binary-is-false` deriving
  `(false)`; its rules named with `bin-` (`bin-numeral-bit`, `bin-numeral-prefix-bit`, `-zero`, `-named`); entry
  `binary`. A decimal digit as a prefix, `(b2 7 1)`, is a name in binary's eyes and silent, as `(n end 1)` is.
- **(b) The check inside `std.math.binary-arith`**, one module a base, as decimal's was until S1f.6.38.

**Defaulted 2026-11-15: (a), the user's to revise.** The coordinator's brief and the ruling's symmetry.

### Q-M1f.296 — The bit tables, by the digit tables' generator

**Raised** 2026-11-15 by S1f.6.16's build (T1f.6.16.6, *"generator in `utils/`"*).

- **(a) `utils/gen_arith_tables.py` writes both** `stdlib/math/digits.ein` and `stdlib/math/bits.ein`, one
  `--check` over the two, held in the gate by `ein-ir/tests/digit_tables.rs`, which holds every row of both to Rust's
  integers; `digits.ein` byte for byte as before.
- **(b) A generator of its own**, `utils/gen_bits.py`, and a test of its own.

**Defaulted 2026-11-15: (a), the user's to revise.** The tables are one family at two bases, computed by one function
each in the script.

### Q-M1f.297 — Binary's tagged calls an interface; what `std.math.convert` reads

**Decision file:** [`docs/decisions/stdlib/math/binary/Q-M1f.297-binarys-tagged-calls-are-an-interface.md`](../../docs/decisions/stdlib/math/binary/Q-M1f.297-binarys-tagged-calls-are-an-interface.md)

**Raised** 2026-11-15 by S1f.6.16's build.

- **Binary's tagged calls**, `(bin-add-walk TAG X Y C z)` and its ten twins of decimal's interface (S1f.6.30's T1,
  round 2's item 41 (a)): **(a)** an interface as decimal's, eleven rows of `arith_theory.rs`' `TAGGED`, written in
  the module's header and `stdlib/README.md`; **(b)** the circuit's own until a caller outside the stdlib needs them.
- **What `std.math.convert` reads of binary**: `bin-strip`, `bin-done`, `bin-top-first`, `bin-top-first-is` and
  `bin-b`, binary's circuit, as `conv.ein` read the example theory's; **(a)** as is, a module of the stdlib reading
  another's circuit, held by `binary_theory.rs`' conversions; **(b)** binary's interface grows a strip-and-turn
  call.
- **`(arith-rule decimal-arith)`'s twin**: decimal's module writes its closing rule's name as a fact, kept since
  removing it would move every state (E-11); **(a)** binary's writes none; **(b)** `(bin-arith-rule binary-arith)`
  for symmetry.

**Defaulted 2026-11-15: (a), (a), (a), the user's to revise.**

### Q-M1f.298 — The two shared sentences that wrote decimal's numeral

**Decision file:** [`docs/decisions/stdlib/math/Q-M1f.298-a-shared-rules-text-writes-the-bases-own-numeral.md`](../../docs/decisions/stdlib/math/Q-M1f.298-a-shared-rules-text-writes-the-bases-own-numeral.md)

**Raised** 2026-11-15 by S1f.6.16's build. `operand-digit`'s and `let-to-digit`'s `:why` wrote *"the number is (n z {?x})"*,
decimal's numeral in the text.

- **(a) The base's constructor, `({?N} z {?x})`**: the parameter in the sentence, so binary's reads `(b2 z 1)`; a
  program that does not list `n` reads it by its full name.
- **(b) A sentence of no base**, *"the number of that one digit"*.
- **(c) Decimal's text kept**, wrong in binary.

**Defaulted 2026-11-15: (a), the user's to revise.** Each rule gained the parameter `?N`, and reads only where a
digit or a bit stands for a number, a dead program's trace.

### Q-M1f.299 — gcd(0, 0)

**Decision file:** [`docs/decisions/stdlib/math/gcd/Q-M1f.299-the-greatest-common-divisor-of-zero-and-zero-is.md`](../../docs/decisions/stdlib/math/gcd/Q-M1f.299-the-greatest-common-divisor-of-zero-and-zero-is.md)

**Raised** 2026-11-15 by S1f.6.32 (the stage file: *gcd(0, 0) is zero by the usual convention,
which the stage states and the harness holds*). Every number divides zero, so zero and zero have no greatest common
divisor in the order of size.

- **(a) Zero**, the usual convention: zero is the greatest element of divisibility, gcd(X, 0) = X holds at X = 0 too,
  and Euclid's chain gives it with no rule of its own (the chain is at its end at once). lcm(0, 0) is zero, by a rule
  that divides nothing, `lcm-zero`.
- **(b) No answer**, a request left unanswered, as a malformed one in silence.
- **(c) A flag and `(false)`**, as a division by zero.

**Defaulted 2026-11-15: (a), the user's to revise.** `gcd_theory.rs`' `zero_in_each_place` holds it, and
`tests/stdlib/math/gcd/02_gcd_zero.ein` and `06_lcm_zero.ein` claim it.

### Q-M1f.300 — `std.math.gcd`'s tagged call, an interface

**Decision file:** [`docs/decisions/stdlib/math/gcd/Q-M1f.300-the-greatest-common-divisors-tagged-call-is.md`](../../docs/decisions/stdlib/math/gcd/Q-M1f.300-the-greatest-common-divisors-tagged-call-is.md)

**Raised** 2026-11-15 by S1f.6.32. S1f.6.33 has `rat-simplify` *call* `std.math.gcd`, and a
rule of `std.math.rat` cannot read `(gcd-of X Y G)` without joining it to its own state on a number, which the rule
of the theory forbids. So the module has a call under the caller's tag, as `std.math.decimal-arith`'s division has.

- **(a) `(gcd-call TAG X Y)` answered by `(gcd-done TAG G)`, an interface** as decimal's tagged calls are (round 2's
  item 41, *"(a)"*): two rows of `arith_theory.rs`' `TAGGED`, written in the module's header and
  `stdlib/README.md`. X and Y are numbers, high zeros allowed, never a name; G in normal form. The public requests
  are built on it.
- **(b) The circuit's own**, `std.math.rat` reading `std.math.gcd`'s chain as `std.math.convert` reads binary's
  circuit (Q-M1f.297's second part).

**Defaulted 2026-11-15: (a), the user's to revise.** `tests/stdlib/math/gcd/07_gcd_call.ein` calls it from a rule
of a program's own.

### Q-M1f.301 — What `gcd-call` does before Euclid's chain

**Decision file:** [`docs/decisions/stdlib/math/gcd/Q-M1f.301-the-greatest-common-divisor-puts-each-number.md`](../../docs/decisions/stdlib/math/gcd/Q-M1f.301-the-greatest-common-divisor-puts-each-number.md)

**Raised** 2026-11-15 by S1f.6.32. The banked example's chain and `std.math.rat`'s divide the
operands as written: an answer that is an operand's value comes back as that operand was written, `006`, and a zero
written with digits, `(n z 0)`, is a divisor of the shape `(n P D)`, a division by zero.

- **(a) Each number put into normal form first, by an addition walk to zero**, `(add-walk TAG X z 0 z)`: zero is
  `z` and every divisor `(n P D)` is not zero, and the answer is in normal form; some two firings a digit, 13 on
  two one-digit numbers, 23 on 1071 and 462 (the record's comparison). The chain divides the larger first where
  it is so written; where X is the smaller, its first step's quotient is zero and turns the two round, one
  division more (gcd(462, 1071) 742 firings, gcd(1071, 462) 553).
- **(b) (a), and the two compared first** (a comparison walk, a digit a firing), so that the chain never turns the
  two round: one division fewer where X is the smaller, a walk more everywhere.
- **(c) No normal form:** a zero compared with zero first (as `std.math.rat` compares a denominator), and an answer
  that is an operand's value put into normal form at the end.

**Defaulted 2026-11-15: (a), the user's to revise.** The simplest that is right; (b) is the cheaper on a proper
fraction's terms, which `rat-simplify` hands over smaller first.

### Q-M1f.302 — What `std.math.rat`'s unreduced answer fixes, and `rat-norm` under (b)

**Decision file:** [`docs/decisions/stdlib/math/rat/Q-M1f.302-an-unreduced-answer-has-its-numerator-in-normal.md`](../../docs/decisions/stdlib/math/rat/Q-M1f.302-an-unreduced-answer-has-its-numerator-in-normal.md)

**Raised** 2026-11-15 by S1f.6.33. The stage file: *the operations answer unreduced: the cross products over the
product of the denominators, as computed, with the sign in the numerator's integer in normal form and the
denominator a natural number that is not zero; … only the common factor of numerator and denominator stays*. A
sum's and a product's denominator is a product, which has no high zero; a negation's is the operand's, as written,
`(n (n z 0) 4)` perhaps. And `(rat-norm TAG P N D)`, an interface (round 2's item 41), was *P less N over D in
lowest terms*.

- **(a) Every answer with no high zero:** `rat-norm` simplifies the numerator and walks D to normal form by an
  addition walk to zero, and reduces nothing; its meaning in the interface becomes *P less N over D, the numerator
  in normal form and D with no high zero, not reduced*, the shape unchanged. A sum pays the walk on a denominator
  that has no high zero, some two firings a digit.
- **(b) D as handed over:** `rat-norm` simplifies the numerator only; a negation walks its denominator before it
  calls, and a caller of the interface that hands D with high zeros gets them back.
- **(c) D as written in a negation's answer too**: `-(1/02)` is `-1/02`.

**Defaulted 2026-11-15: (a), the user's to revise.** One sentence says what every answer is, and a caller of the
interface reads it whatever it hands over.

### Q-M1f.303 — `rat-simplify` asks `std.math.gcd` the numerator first

**Decision file:** [`docs/decisions/stdlib/math/rat/Q-M1f.303-lowest-terms-hands-the-numerator-to-the-greatest.md`](../../docs/decisions/stdlib/math/rat/Q-M1f.303-lowest-terms-hands-the-numerator-to-the-greatest.md)

**Raised** 2026-11-15 by S1f.6.33. `rat-simplify` calls `(gcd-call TAG A D)`, A the numerator's size and D the
denominator, as `std.math.rat`'s own chain took them until this stage. Where A is the smaller, a proper fraction,
the chain's first division turns the two round ([Q-M1f.301](#q-m1f301--what-gcd-call-does-before-euclids-chain)):
one division more.

- **(a) A first**, as before: the before-and-after measures of this stage compare step for step.
- **(b) D first**, the cheaper on a proper fraction and the dearer on an improper one.

**Defaulted 2026-11-15: (a), the user's to revise.**

### Q-M1f.304 — `route_invariance` past the default budget, past any shard's five minutes

**Raised** 2026-11-15 by S1f.6.39's
T7, applying [Q-M1f.291](#q-m1f291--the-five-sweeps-that-set-modes-and-routes-against-one-another) (a) and
[.293](#q-m1f293--the-nightlys-steps-against-five-minutes): `route_invariance`'s `slow` entries go to its slow half,
the nightly's, each nightly step in shards of at most five minutes on the runner. Its slow half was two things: the
`slow` entries at the default budget, and every program route 1 finishes only past it (the zebra family, the six
`square-*` cubes) at `--region-budget` 2²⁵. Measured as T6 measured (`taskset -c 0-3`, `EIN_TEST_JOBS=4`, AC,
2026-11-15, `build/s1f.6.39/t7/slow-4.log`), the slow half ran 29 min unfinished, three workers busy throughout its
last 20, and was stopped there; and route 1 alone took 495 s at `-j 16` to reach a 2²² ceiling on `zebra2-minus-15`
(`alive_set_reach.md` § 2.3, 2026-11-03), where the
test asks 2²⁵ of one unit, one program. No sharding by program fits it.

- **(a) The default-budget half in the nightly, the 2²⁵ half on the dev host**: the test split in two,
  `the_state_route_and_route_1_print_one_read_out` (per commit the programs but the `slow` entries, under
  `EIN_CORPUS_SLOW=1` every one), which the nightly runs by name, and
  `route_1_and_the_state_route_agree_past_the_default_budget`, which `--dev-host` runs with the target whole
  (`DEV_HOST_ONLY`), as the whole slow half ran there since S1f.5.27.
- **(b) The 2²⁵ half in the nightly past the rule**, a step of its own over five minutes.
- **(c) A larger budget that fits**, between 2²⁰ and 2²⁵: the zebra family's route equality at a budget route 1
  does not exhaust is no comparison, so it holds of fewer programs.
- **(d) No test past the default budget.** Describe on a monotone program always takes the state route, and no
  option routes one to route 1, so the 2²⁵ half ran an internal route, `explore::describe_by`, at 32 times the
  default `--region-budget` of 2²⁰ the user ruled on 2026-11-03 after 2²⁴ ran the gate out of memory, on inputs no
  command sends it. The 2²⁵ test goes. In the default-budget test, a program the state route finishes and route 1
  does not is held by the cut, the state route's saturations and facts no more than route 1 spent before it
  stopped, and its read-outs are not compared; each is named on stderr. One neither route finishes is skipped, as
  before.

**Defaulted 2026-11-15: (a), the user's to revise.**

**Ruled 2026-11-15: (d).** The user: *"(d)"*, after the main session set out the default budget's ruling and what
the 2²⁵ half ran. Built at P1f.6's close: `route_1_and_the_state_route_agree_past_the_default_budget` deleted, and
with it the per-budget fold; `the_state_route_and_route_1_print_one_read_out` holds the cut's premise and names the
programs it holds by the cut and those it skips, and reads `EIN_CORPUS_SLOW` itself, so the nightly's step that
names it runs the target's whole slow half. Under `EIN_CORPUS_SLOW=1` (2026-11-15, the gate profile): 582 programs
compared; 5 held by the cut, `examples/zebra.ein`, `zebra2.ein`, `zebra2-obligations.ein` and the register's
`count-past-2-64.ein` and `count-past-2-128.ein`; 9 skipped, neither route finishing, the six `square-bwd` and
`square-fwd` programs, `send_more_money.ein` and `zebra2-minus-15.ein` with its `-obligations` twin. The dev
host's step and its `DEV_HOST_ONLY` entry are gone (`nightly_steps.rs`, `run_tests.sh`), and the pages that named
the half (`corpus/README.md`'s table, the workflow's comments).

### Q-M1f.305 — P1f.10 re-planned: the defaults taken

**Raised** 2026-11-15 by the re-plan of P1f.10, which the user asked for
that day (*"do re-plan according you findings and those 2 ideas"*) after an analysis of its stage files against
the engine as it stands and their statement of the two ideas behind the phase
(§ The two ideas). The instruction fixes the two tracks
and that they are independent. What the re-plan decided beyond it, each without a ruling:

1. **The ids.** No stage had run. The seven stages that keep a subject keep their id and their file name, so
   that the register (`owner = "S1f.10.6"`), the kernel pages and `docs/history/` still point at something:
   S1f.10.1 to .6 and .9, re-scoped in place. S1f.10.7 is dropped and S1f.10.8 closed, each left as a stub, the
   ids used and vacated. New stages are S1f.10.10 to S1f.10.16. The alternative, renumbering all of them by
   track, was not taken.
2. **A third group, track H**, for the three findings the phase owns about what H is (W26 and Q-M1e.11; E-1, E-7
   and E-54; E-51), which are neither idea and each need the user.
3. **One stop.** Part 1 reads and measures and changes nothing; then the user is shown the structure on real
   programs, the price list and four questions at once (the surface, and track H's three). The alternative, a
   stop per H stage before any structure exists, was not taken: the bundle is itself the evidence for those
   questions.
4. **The pair layer is read by the engine's own node**, an example in the crate beside `state_reach`, and not by
   a script over `ein saturate` on a rewritten program as the plan of 2026-08-28 had it. The small-KB check of
   the user's instruction of 2026-08-28 is kept as its own stage, S1f.10.10, with the support of a death as a
   second reading that checks it.
5. **Coupling is a stage**, S1f.10.11, added because the re-plan's analysis infers that E-29's witness cannot be
   ordered from pairs alone. The inference is not a measurement, and S1f.10.5's first task checks it.
6. **The rule for *go*** in S1f.10.13: a source of structure is spent on a question only where the corpus's
   total work for that question falls with the source's cost counted **and no program's rises by more than one
   tenth**. The tenth is the re-plan's number.
7. **The surface's default is a view of `ein render`**, not rows of `ein solve`'s read-out, put to the user at
   the stop and not built before.
8. **S1f.10.15 recommends reversing a reading of M1d's** (the second rung by an obligation activated at root, not
   by a rule loaded). Because it restates a ruling it is a recommendation only, and is not taken as a default.
9. **Whether describe should answer from the tree where the budget would stop it** is recorded with numbers and
   filed as a question at the phase's close (S1f.10.14's T4). Nothing is built for it in this phase.
10. **Estimates are removed** from the phase's files (the user's ruling of 2026-11-08), and the stage records no
    longer name `docs/history/` as their destination (the user's rulings of 2026-11-15 on the docs).

**Defaulted 2026-11-15: all ten as written, the user's to revise.**

**Revised the same day by the user**, while part 1 ran: *"wait a second, both goals - Track S and Track R are both
required, but R obviously depends on the results of S. Search improvements depends on hypothesis structure"*. That
corrects the re-plan's reading of *independent* behind defaults 3 and 6: track R is owed and is not closed by a
table of *no*, S1f.10.13 runs after track S's stages and prices every object of the structure, and the rule for
*go* judges one way of spending for one question
(P1f.10 § The two ideas).

**Defaults the stages of part 1 took after it**, each in its stage file's § Defaults taken and shown at the
phase's stop: what a group is, after S1f.10.2's control failed
(S1f.10.2); the rule for the small KB and seven
more (S1f.10.10).

### Q-M1f.306 — What the blind enumerator takes for an object

**Decision file:** [`docs/decisions/hypotheses/Q-M1f.306-the-guessable-atoms-are-the-programs-names-never.md`](../../docs/decisions/hypotheses/Q-M1f.306-the-guessable-atoms-are-the-programs-names-never.md)

**Raised** 2026-11-15 by
S1f.10.9's
tasks 1 to 3, for E-1, E-7 and E-54 of the calculator's `findings.md`, which
P1f.10 owns. **It is put at P1f.10's one stop**
(S1f.10.12), with the phase's other three
questions. Removing a candidate changes H, and H is part of the answer, so this is one of the three places in
the phase where an answer would move.

**What the engine does** (the record's § What the code does). The blind enumerator's objects are the names
index at root's fixpoint, which holds every fact's head beside its arguments, and the program's names
(`invariant::program_names`: every constant in an argument position of a fact or a rule, Q-M1e2.9 (a)); less
the names a signature declares, seven reserved words, and every name the registry calls a relation or a rule.
The registry holds a relation a `(relation …)` declares and the head of every fact **a file writes**. So a head
only a rule derives is an object, and that is E-1 where the head is a kernel dunder and E-7 where it is an
undeclared relation: one mechanism. Every relation with a signature that nothing scopes out is proposed about,
over every object, wherever either was loaded from: E-54.

**The witnesses**, each by `ein oracle` on the release binary of `d7050ccf`, R without the marked candidates
counted in the oracle's cube and, where the language has a means, again by `ein oracle` on a copy of the
program in which they do not arise (`build/s1f.10.9/run.sh`, 2026-11-15; every program is in the record):

| | witness | H | marked | R | R without them |
|---|---|---:|---|---:|---:|
| E-1 | ten digits closed by a rule, `total` open | 11 | `(total __closed__)` | 2 048 | 1 024, and 1 024 on the copy that writes `(__closed__ digit)` as a fact |
| E-1 | `(import std.math.arith)`, `total` open | 1 | `(total __closed__)` | 2 | 1 |
| E-1 | `(import std.math.decimal-arith)`, `total` open | 13 | none: `std.math.digits` writes its closures as facts | 8 192 | 8 192 |
| E-1 | a dunder written as a value, as `(imply1 functional __closed__)` | 2 | `(total __closed__)` | 4 | 2 |
| E-7 | the import probe, `tasty` alone open | 5 | `(tasty liquid)`, `(tasty thing)` | 32 | 8, and 8 on the copy that declares both |
| E-7 | the same in one file | 3 | `(tasty liquid)` | 8 | 4, and 4 on the copy that declares it |
| E-54 | a library of `colour` and `shape`, imported for `colour`, `near` declared | 4 | all 4: each names `red` or `square`, the library's | 16 | 1; without the one of `lib.shape` alone, 8 |

- **E-1 as worded does not reproduce on the shipped calculator**: a written `(__closed__ R)` fact registers
  the dunder as a relation, so it is an object only where every closure is derived by a rule (the prototype's
  shape, `std.math.arith` alone), or where it is written as a value. **One closure stated two ways gives two
  answers today**, 2 048 states and 1 024.
- **E-7's names are heads, never values.** No rule *asserts them as values*, and `program_names`, which built
  Q-M1e2.9 (a), does not list them. A relation's name that a program does write as a value is **not** an
  object where a file declares the relation or writes a fact of it (the record's control).

**The census** (`cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example object_census`,
2026-11-15, the oracle's default limit of 16; rows in `build/s1f.10.9/census/rows.tsv`). Of 867 corpus files
644 load. The blind enumerator built H on 470; 115 of them have a candidate, 2 190 candidates in all.

| the candidate names | programs | candidates | R differs without them |
|---|---:|---:|---|
| **E-1**, a kernel dunder | 0 | 0 | — |
| **E-7**, a relation's name | 1, `examples/features/01_not_and_absent.ein` | 16 of its 35 | **not reached by the oracle**, 2³⁵; a default-budget describe computes 64 655 nodes, all in R, and 58 966 of them hold one, so R would differ |
| **E-54**, a name only an imported module writes | 2, `examples/ein-bugs/wrong-answers/import-directives-as.ein` and `-twice.ein` | 2 of 2 each | **yes, both**, by the oracle's cube: 4 states to 1 |
| E-54, a candidate of a relation the main file does not spell | 0 | 0 | — |

On the 174 programs another rung builds H for, the blind enumerator is not asked. Were it, it would name
`__closed__` on 8 of them and a derived relation's name on 4 (the record's last table): the corpus's answers
are clear of E-1 and E-7 by the rung those programs take.

**The options**, as the stage states them, each with what it would change:

- **(a) Leave E-1 and E-7, and the read-out names such candidates** as it names types bound as objects
  (Q-M1e.12 (a)). No answer moves. The `universe` row and `root.hypgen` gain a count, which today would print
  on one corpus program (`features/01`, 16 candidates naming `explicitly-dislikes`) and on the witnesses.
- **(b) A kernel dunder is never an object**, which is a wrong answer today and a register row before its
  fix. The witnesses: 2 048 → 1 024, which is what the same program answers with its closure written as a
  fact; 2 → 1; and for `__symmetric__` 8 → 4. **No corpus program's H holds a dunder, so no corpus
  answer, golden or expectation moves.** The user would say whether *never* covers a dunder a program writes
  as a value (4 → 2 on that witness), which Q-M1f.7 (b) keeps legal and Q-M1e2.9 (a) reads as a name.
- **(c) (b), and a relation's name is not an object either.** The witnesses: 32 → 8 and 8 → 4, each equal to
  its declaring copy; the original probe, whose R `ein solve` reads as the whole cube of 39 candidates, would be
  the whole cube of 17. **One corpus program moves**, `features/01`: H 35 → 19, its R, and with it the goldens
  that hold its renderings and its measured cost; it states no `:expect`. The stage says
  this *narrows Q-M1e2.9 and needs the user to say so*. On the evidence it narrows nothing of that ruling as
  built: it would treat a head only a rule derives as every head a file declares or writes is treated already.
- **(d) Leave E-54**: the universe is the program's names wherever they were written.
- **(e) A name or a relation only an import brings is not guessed about unless the program writes it**, which
  makes what is guessed depend on what the main file spells and needs the user to say so. The witness: 16 → 1
  under both import forms for the names; for the relation alone 16 → 8 where the main file lists `colour` in
  `:symbols` and 16 → 4 where it imports the module whole, **if a listing counts as writing**, which is an
  answer that depends on the import form (`ein-cli/tests/import_invariance.rs` holds that none does). **Two
  corpus programs move**, 4 states → 1, and both are register witnesses of W90 whose `fixed` rows state the
  four states as the right answer, by hand, over the module's names: those two rows' `right` would be
  restated. A program that imports the calculator beside an open relation of its own is left with no candidate
  (the third E-1 witness, 8 192 → 1).

**The default, as the stage names it: (a) for E-7**, *since it is the ruling as written*; **(b) for E-1**,
*since no program means a dunder as an object*; **(d) for E-54**, *since every import brings its module whole
and the answer does not depend on the import form*.

**What the census says of those three reasons**, for the user to weigh; it changes no default:

- **E-7's reason does not hold of its witness.** The ruling as written makes names of the constants in
  argument positions, and the witness's names are in none. What does speak for (a) is the cost of (c): one
  corpus program's answer, by a register row. What speaks for (c) is that E-1 and E-7 are one mechanism, so (b)
  alone is a rule about two names inside a behaviour that stays, and that declaring a derived relation already
  removes its name today.
- **E-1's reason holds**, and the census adds that (b) moves nothing in the corpus and makes the two ways of
  stating a closure agree.
- **E-54's reason holds**, and the census adds that (e) would restate two answers the register holds as right
  and, in its relation half, break the import invariance on the witness pair.

**Not defaulted. The question is put at the phase's one stop, and no default is acted on before the user
replies**: no candidate is removed, no register row filed, no read-out row added. As ruled, it is
T1f.10.9.4.

**Ruled 2026-11-16.** The user, shown the phase's four questions with their defaults at the stop: *"all defaults ok"*. So: **(b) for E-1**, a kernel dunder is never an object; **(a) for E-7**, the candidates stay and the read-out names them; **(d) for E-54**, the universe is the program's names wherever they were written. One thing the entry left to the user is not covered by a default and is asked back: whether *never* covers a dunder a program writes as a value (the fourth E-1 witness, 4 states to 2). Until the reply that witness is not moved: (b) is built for a dunder that is an object only because no file wrote a fact of it.

**The sub-question, ruled 2026-11-16.** The user, asked whether *never* covers a dunder a program writes as a
value: *"dunder never an object, wide reading"*. So it does: a kernel dunder is no object of the blind
enumerator where a rule derives the fact it heads, and none where a program writes it in an argument position.
Writing it so stays legal (Q-M1f.7 (b)), and it stays a name of the program as the alive-set invariant lists
them (Q-M1e2.9 (a)); it is the enumerator's objects that leave it out. Closed.

**Built 2026-11-16** by
S1f.10.9's
task 4.

- **E-1, (b), wide.** `hypgen::candidate_objects` leaves `__closed__` and `__symmetric__` out by name;
  `invariant::program_names` is unchanged. Filed as the register's **W96**, four rows, `open` in `200df470` and
  fixed in `4ee131e3`: ten digits closed by a rule, 2 048 states → 1 024, its closed-by-fact control's;
  `(import std.math.arith)`, 2 → 1; `__symmetric__` derived by a rule, 8 → 4; `__closed__` written as a value,
  4 → 2. One closure stated two ways has one H and one R, held as an invariance by
  `ein-infer/tests/dunder_objects.rs`.
- **What the census did not count.** No corpus program's H held a dunder, and none of their H, R, read-outs or
  exit codes moved. But on 22 programs the dunder was an object that was walked and proposed nothing, and the
  summary's `root.hypgen.pre_candidate.closed_relation` and the shape golden's `hyp` line dropped; and the
  golden's `hyp+closed` op, which runs a pass no command calls, moved on 185. The lines were re-banked on the
  main session's decision, each group named in the fixing commit; the record has the lists and the check that
  no other line moved.
- **E-7, (a).** The candidates stay. The read-out prints a `relation names` row under `universe` and the
  summary carries `root.hypgen.relation_names_as_objects`, written only where there is one: on one corpus
  program, `examples/features/01_not_and_absent.ein`, 16 of its 35 candidates.
- **E-54, (d).** Nothing built.
- **Found on the way, open, in the phase's `findings.md`**: H9-4, a `(__symmetric__ R)` a rule derives is not
  mirrored by the saturation that derives it; H9-5, the `hyp+closed` op. H9-1, the reverse of E-1 for a
  reserved word written as a value, is not in the ruling and stays open.

### Q-M1f.307 — A debt root was not told of: what H holds

**Raised** 2026-11-15 by
S1f.10.6's
tasks 1 to 3, for the register's `W26/oblig-derived-activator`, `open` and owned by that stage, and for
[Q-M1e.11](../../docs/decisions/hypotheses/Q-M1e.11-h-holds-what-a-debt-only-a-commitment-brings.md),
*what happens to an obligation derived under a hypothesis*, which P1f.10 owns. **Q-M1e.11 stays open**: this
entry restates it over R, with what was measured, and the ruling given here is recorded there by the stage's
task 4. **It is put at P1f.10's one stop**
(S1f.10.12), with the phase's other three
questions. Three of its four answers change H or refuse a program, so this is one of the three places in the
phase where an answer would move.

**What the engine does** (the record's § What the engine does). H is the generator's list at root's fixpoint
and is taken nowhere else. On the obligations rung it is what would pay the debts **root** owes. A node's
debts are read over the activator facts the **node** holds, so a rule that derives an obligation's activator
from a guess gives a node a debt whose payment was never a candidate. The static check flags such a program
(an activator site), so it takes the cube; `check_owed`, which refuses a run where a node owes what root did
not, runs only on a program the check calls monotone and so never on this one.

**The witness and its control, and the same pair on the other two rungs** (`ein solve` and `ein oracle` on
the release binary of `14063474`, 2026-11-15, `build/s1f.10.6/run.sh`; every program is in the record):

| rung | the activator derived by a rule | the activator also stated |
|---|---|---|
| obligations: the witness and the control | H 1, **R empty**, every alive node owing | H 2, R 1 state, `{(drinks Ann Tea) (likes Ann Soup)}` |
| hrule, each hrule proposing a relation where its activator holds | H 1, **R empty** | H 2, R 1 state, the control's |
| hrule, the hrules reading `is-a` alone | H 2, R 1 state, the control's | the same |
| blind, the obligations rung declined by a bare `(open)` | H 12, R 1 024 states, the minimal line the control's state | the same |

**The question is not the obligations rung's alone, and it is not the blind rung's at all.** It is the
question of a generator that reads the state. And it does not need a derived activator: a program with both
debts stated and an hrule that proposes a drink for whoever likes a food has H 1 and **R empty**, where its
twin that states the liking has a model; the static check calls it monotone and nothing flags it
(H6-1).

**The census** (`cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example owed_census --
--jobs 12`, 2026-11-15, the oracle's default limit of 16; rows in `build/s1f.10.6/census/rows.tsv`). Of 867
corpus files 644 load, and 127 of them declare an obligation rule.

| | programs | which |
|---|---:|---|
| statically: an obligation rule's activator relation is one a commitment can change (`D`) | 3 | the witness, the control, `examples/lattice/09_activator_hole.ein` |
| statically: … only through a rule root has no plan of (`D⁺`) | 1 | `examples/lattice/12_activator_chain_hole.ein` |
| by the cube: an alive node owes an instance of a plan root never compiled | 3 of the 582 the oracle reaches | the witness, `lattice/09`, `lattice/12` |
| by the cube: an alive node owes an instance root did not, of a plan root has (a trigger site; on the obligations rung Q-M1e2.10 (a) declines for one) | 4 | `lattice/04`, `lattice/synth/{local_not_global,maximal_not_top}`, `tests/region/07_cube` |
| by the cube: any of that on a program the static check calls monotone | 0 of 560 | |
| **by the cube: the generator would propose at an alive node a fact that is not in H** | **1** | **the witness** |

On `lattice/09` and `/12` an hrule proposes every meal from `is-a` alone, so the new debt's payment is in H:
their R is 3 states with a hole, and what R is on them is F21's. **Not reached**: the 43 programs past the
oracle's limit, named in the record; none has an obligation rule's activator relation in `D` or `D⁺`, and the
one that has an hrule reading `D` was read at `--limit 17` and proposes nothing outside H. 19 more are refused
by *forbid* and have no R.

**The four answers**, each on the four programs the census finds:

| | the witness | the control | `lattice/09` | `lattice/12` |
|---|---|---|---|---|
| today | H 1, R empty | H 2, R 1 state | H 2, R 3 states | H 2, R 3 states |
| **(A)** H holds what a node's debt would propose | H 2, **R 1 state, the control's** | unmoved | unmoved | unmoved |
| **(B)** decline: such a program's H is the blind enumerator's, as Q-M1e2.10 (a) took for W14 | H 12, R 1 024 states; the minimal line is the control's state | **moves**, H 12, R 1 024 states: the decline is at relation grain | unmoved, an hrule builds H | unmoved |
| **(C)** refuse at load: every obligation activator must hold at root | refused | **refused** by the static check's site, which is at relation grain; accepted only by a test that reads the fact | **refused** | refused only by a test that follows `D⁺`, as *forbid* does |
| **(D)** the answer stands, and the read-out says why | R empty; the row `not-wrong`; a new line, since no row of the read-out says today that nothing in H pays the debt | unmoved | unmoved, no line | unmoved |

- **(A)** changes one corpus answer, the witness's, by its register row.
- **(B)** changes two, and restates the row's control, as Q-M1e2.10 (a) restated two of W14's. It is the move
  the user ruled there for a *domain* a commitment can change. `object_census` marks none of the twelve
  candidates, so Q-M1f.306 moves nothing on them
  (H6-2).
- **(C)** turns three or four corpus programs into refusals. Two of them, `lattice/09` and `/12`, are not
  wrong in H today: they are the corpus's two exercised activator sites, which
  [`08-monotonicity.md`](../../docs/ein/reasoning/02-kb-evolution/08-monotonicity.md) and `ein-infer`'s detector tests read by
  name, and F21's examples of the third cause of a non-monotone program.
- **(D)** changes no answer and keeps an empty R where a model exists.

**(A) priced, under its two readings.** Neither was built; each universe was stated by hrules on a copy of
the program, which changes no node, and R read by `ein oracle`.

- **Reading 1, static**: the ground activators of an obligation rule that a rule reading `D⁺` writes in its
  `:assert`, with the rung's own scan run for them at root, repeated until it adds nothing. No saturation.
- **Reading 2, over nodes**: the least set that holds H and what the generator proposes at every alive node
  over it. `owed_census --iterate` computes it, a round a cube.

On the corpus they agree: the witness gains `(drinks Ann Tea)` and answers the control's R; the control,
`lattice/09` and `lattice/12` gain nothing; no other program can gain under reading 1, which adds only where
an activator relation is in `D⁺` (computed on all 644), and none of the 582 the cube read gains under
reading 2. Off the corpus, on six programs written for it, each the witness with one thing changed:

| probe | today | reading 1 | reading 2 |
|---|---|---|---|
| `arm` fires on one food of two | R 1 state | R 4 states | R 4 states. The control's is 3: **(A) is not *state the activator*** |
| no alive node derives the activator | R 2 states | **over**: R 4 states, two holding a tea no alive node owes | R 2 states, today's |
| the rule writes the owed relation as a variable | R empty | **under**: no ground activator, R stays empty | R 1 state |
| the tea that pays switches on a second debt | R empty | R 1 state, in two passes | R 1 state, in two rounds |
| `lattice/12`'s chain on the obligations rung | R empty | R 1 state over `D⁺`; nothing over `D` | R 1 state |
| the new debt's domain is one a commitment can change | R empty | the rung's existing decline: **(B)** | the node's rung declines: **(B)** |
| H6-1's program, no activator derived | R empty | **nothing** | R 1 state |

Reading 1 reads rules and root alone, costs a closure and a scan, and is inexact both ways on shapes no
corpus program has. Reading 2 is exact by its definition, costs a generator pass on every alive node of every
round's cube, which no route but the cube can take today, and is a function of nodes: **what a node is on a
flagged program is F21's**, so its universe would move with that ruling. Under either, every program (A)
touches stays flagged, and a region with a hole stays one.

**The default, as the stage states it: (A) where T2 shows a static reading that gives the control's R on the
witness and moves no other corpus program's R; otherwise (C).** *Both name a wrong answer today; (B) trades
it for a different H, and (D) keeps an empty R where a model exists.*

**The census selects the first branch, (A)**, by reading 1: it gives the control's R on the witness and moves
no other corpus program's R.

**What the census says beside that**, for the user to weigh; it changes no default:

- **The branch is selected by the corpus, and reading 1 is exact only there.** The two probes on which it is
  over and under are each the witness with one rule changed.
- **(A) by reading 1 leaves H6-1 as it is**, and so do (B), (C) and (D) as worded: that program derives no
  activator and is monotone. Only reading 2 answers it.
- **The row's `right`, *the control's R*, is (A)'s answer on the witness and not (A)'s definition**: where a
  member of R need not derive the activator, (A) and the control differ.
- **(A) is a *widen*, as Q-M1e2.10 (b) was for a domain**, and there the user ruled (a), *decline*, which is
  (B) here. The two cases differ in one respect the census can state: under (B) the control moves too, to
  1 024 states of free facts, and under (A) nothing but the witness moves.
- **The default's fallback, (C), costs more of the corpus than any other answer**, and two of the programs it
  refuses are right in H today.
- **A static reading has to follow `D⁺`**, which today only *forbid* reads; the static check's `D` and the
  rung's own decline do not see `lattice/12`'s shape.

**Not defaulted. The question is put at the phase's one stop, and no default is acted on before the user
replies**: H is unchanged, `check_owed` untouched, no register row filed or flipped, no read-out row added,
no program refused. As ruled, it is
T1f.10.6.4.

**Ruled 2026-11-16.** The user, shown the phase's four questions with their defaults at the stop: *"all defaults ok"*. So: **(A), by the static reading** (reading 1), the branch the census selected: the ground activators of an obligation rule that a rule reading `D⁺` writes, with the rung's own scan run for them at root, repeated until it adds nothing. The caveats above stand as recorded: it is exact on the corpus and inexact both ways on two written probes, and it leaves H6-1 as it is.

**Built 2026-11-16** by S1f.10.6's
task 4. The obligations rung reads the activators through *forbid*'s one reading of `D⁺`, scans each one's plan at
root and repeats until nothing is added; `W26/oblig-derived-activator` is `fixed`, the witness alone moved, and
every row of the probes' table above is what the engine now answers, the over and the under case held as such by
`ein-infer/tests/later_debts.rs`. The read-out names what H holds for such a debt, a `later debt` row. The
record lists the defaults taken where the ruling left a choice (an activator is read as its rule writes it; `D⁺`
is seeded with the candidates' relations; a later plan is held to the rung's declines) and two findings it did
not fix, H6-3
and H6-4.

### Q-M1f.308 — The rung a loaded obligation rule chooses

**Decision file:** [`docs/decisions/hypotheses/Q-M1f.308-the-ladders-second-rung-is-taken-where-root.md`](../../docs/decisions/hypotheses/Q-M1f.308-the-ladders-second-rung-is-taken-where-root.md)

**Raised** 2026-11-15 by
S1f.10.15's
tasks 1 to 3, for E-51 of the calculator's `findings.md`, which P1f.10 owns.
**It is put at P1f.10's one stop**
(S1f.10.12), with the phase's other three
questions. Two of its three options change which rung builds H, and H is part of the answer, so this is one
of the three places in the phase where an answer would move.

**What the engine does** (the record's § What the engine does). Hypothesis generation is a ladder: an
hrule's proposals if the program declares one, else what would pay the debts root owes, else the blind
enumerator. With no hrule, the second rung is taken wherever the program **loads** an obligation rule
(`hypgen::generate_with`, `!program.obligations.is_empty()`), activated or not, and a second rung that owes
nothing proposes nothing: H is empty and the blind enumerator does not run. Since every import brings its
module whole, a program loads `std.algebra`'s two obligation rules by importing `std.algebra`,
`std.bijection`, `std.closure`, `std.typing` or `std.bool.inputs` for anything at all, and `std.slots` brings
its own two. `mechanisms.rs` reads the same declaration the other way: an obligation is a plan root's
activators compile, and a rule nothing activates is not among a program's mechanisms.

**What M1d ruled.** Its decision table has *obligations supersede `:hrules` as the generator, as a ladder*
(user, 2026-08-24) and *the read-out is scoped — discharge where stated, exhaustion where not* (2026-08-25).
The second was a rule about the verdict, and under R it is gone:
[`06-solution-semantics.md`](../../docs/ein/reasoning/02-kb-evolution/06-solution-semantics.md) has *one definition for every
program*. What is left of *where stated* is this switch, and its reason is `oblgen.rs`'s: *falling back to the
blind enumerator whenever nothing was owed would put exhaustion back in charge of completeness*. That is said
of a discharged state under a stated obligation. In M1d a program loaded an obligation rule only by naming
it, so *loaded* and *stated* were one thing.

**The witnesses** (`ein solve` and `ein oracle` on the release binary of `c6251db3`, 2026-11-15,
`build/s1f.10.15/run.sh`; every program is in the record). E-51's three reproduce, the third to the state.

| program | at root | (a) today | (b) | (c) |
|---|---|---|---|---|
| 1: `(total-owed at is-a)` stated and paid, beside an open `near` | discharged | H 0, R {∅} | as today | H 22, R the whole cube, at least 110 057 states at the budget |
| 2: the same with the activator deleted, the import kept | unactivated | H 0, R {∅} | H 22, the whole cube: program 3's read-out, row for row | as (b) |
| 3: the import deleted too | no rule loaded | H 22, the whole cube, at least 110 057 states at the budget | as today | as today |
| the re-plan's: `std.algebra` for `connex`, over `lt` | unactivated | H 0; `outside lt — no candidate: no obligation owes it` | **refused at load** by *forbid*: the blind enumerator proposes `lt`, and `connex` rests on an `absent` over it | as (b) |
| the fourth: `std.algebra` for `symmetric`, beside an open `likes` | unactivated | H 0, R {∅}; the read-out names `near` as outside and says nothing of `likes` | H 4, **R 16 commitments, 8 states**, what the program answers with the import deleted and `symmetric` written out | as (b) |

Twins of 1 to 3 over fewer names put the same rows within the oracle's limit: H 11, R 2 048 states.

**The census** (`cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example rung_census --
--jobs 8 --check`, 2026-11-15, the oracle's default limit of 16; rows in `build/s1f.10.15/census/rows.tsv`).
An option's H is the blind enumerator's own list at the program's root, and its R the cube over that list by
`region`'s node function, held node for node to `region::compute` on the 437 programs where the blind list is
H today; past the limit, `ein solve` on a copy with one obligation rule added that nothing owes and that
declines the rung. Of 867 corpus files 644 load, and 127 load an obligation rule.

| at root | programs | on the second rung today | where the blind enumerator would propose a candidate |
|---|---:|---:|---|
| a rule loaded, none activated | 45 | 41; the other 4 declare an hrule | 1 of the 41, `examples/features/09_adjacent_via_same_house.ein`, 82 candidates |
| activated, root owes nothing | 16 | 1; of the other 15, 7 say `(solve :no-search)`, 7 declare an hrule, 1 declines on purpose | that 1, `stdlib/bool/inputs.ein`, 342 candidates |
| root owes | 66 | 23 | not moved by any option |

**Each option, on the witnesses and on each corpus program it moves:**

- **(a) as it is**: the rung is chosen by a rule loaded. The read-out says so where it bites: a `universe`
  row that names the rule which took the program off the blind enumerator, and `(solve :no-search)` remains
  the way to say *judge root alone* on purpose.
  *No answer moves.* The row would print on the witnesses 2, the re-plan's and the fourth, and on one corpus
  program, `features/09`; printed wherever an unactivated rule chose the rung, on 41. Kept, the ladder has no
  declared way from the second rung to the third: one corpus program gets there by an obligation nothing
  owes (H15-5).
- **(b) by an obligation activated at root**: the second rung where root's activators compile a plan of an
  obligation rule, the reading `mechanisms.rs` has. A program that imports a module for something else is on
  the third rung again. A discharged state under an activated obligation still proposes nothing and is
  complete, so *complete means discharged* is untouched.
  *Witnesses*: 1 and 3 unmoved; 2 answers as 3; the fourth gains its 4 candidates and 8 states; the re-plan's
  is refused at load. *Corpus*: **41 programs change rung, 1 changes H, none changes R.** On 40 the blind
  enumerator's list is empty, 35 of them by a `:no-hypothesis` list that names every relation it would fill,
  and the read-out is the same row for row. On `features/09` H goes from 0 to 82, all dead with a dead root,
  and R is empty before and after; two rows of its read-out change.
- **(c) by a debt owed at root**: the second rung only where root owes. Rejected by M1d for a reason that
  stands, that a discharged root would fall to the blind enumerator and exhaustion would be back in charge of
  completeness; listed so that the reply can say so.
  *Witnesses*: as (b), and witness 1, whose one obligation is paid, gains 22 untyped candidates. *Corpus*:
  (b)'s 41 and one more, **`stdlib/bool/inputs.ein`: R from {∅} to at least 55 747 states over 342
  candidates of `in`**, describe not exhausted at the default budget, on a module that states its obligation
  and has no input to owe for. That is M1d's reason on a corpus program.

**Recommended, as the stage states it: (b).** *It makes the two readers agree and makes an import of one
name not change what is guessed.* **It restates a ruling of M1d's, so it is the user's to take: it is not a
default and is not acted on.**

**The census does not speak against (b).** It moves no corpus program's R, changes one program's H, lands no
corpus program on a refusal, and leaves every discharged root where it is. **What it puts beside the
recommendation**, for the user to weigh; it changes no wording above:

- **No tracked program's answer shows the change.** The finding moves answers on probes only. Under (b) task
  4 would file no register row, and the switch would land with nothing in the corpus that fails if it is
  undone, unless a witness is added: the fourth, or `features/09` with its guard put back, which answers one
  state today and nine on the third rung
  (H15-3).
- **(b) turns a program that loads today into a load error**, where a refutation reads an `absent` over a
  relation the blind enumerator comes back to. Two probes, no corpus program; the refusal names its remedy,
  `(solve :no-hypothesis (lt))`
  (H15-2).
- **What a program comes back to is the blind enumerator as it is**: untyped, so witness 2's R is 4 194 304
  commitments that hold facts such as `(at Blue Red)`. And
  [Q-M1f.306](#q-m1f306--what-the-blind-enumerator-takes-for-an-object) then bears on it: 44 of
  `features/09`'s 82 new candidates name `adjacent-via-fwd` or `adjacent-via-bwd`, a derived relation's name
  (E-7); under (c), 340 of `stdlib/bool/inputs.ein`'s 342 are marked, 36 naming `__closed__`
  (H15-1).
- **The corpus already reads as (b).** `features/09`'s header describes the hypothesis space the blind
  enumerator gives it, and 35 programs keep the lists that only the third rung reads (H15-3).
- **The switch is written twice** (`hypgen::generate_with`, `hypgen::eligible_relations`), and the ladder's
  sentence is in five places, three of which read as (c)
  (H15-4).
- **Not measured**: under (b) the 41 programs' narration changes, the `rung` event and the generator's
  counters, and with it their shape digests, on 40 of them with H, R and the read-out as they are.

**Not defaulted. The question is put at the phase's one stop, and nothing is acted on before the user
replies**: the ladder is unchanged, no register row filed, no read-out row added, no page restated, and E-51
is open. As ruled, it is
T1f.10.15.4.

**Not ruled by the first reply of 2026-11-16.** The user's *"all defaults ok"* at the stop covers the questions that have a default; this one has a recommendation and no default, by its own wording. Asked back the same day.

**Ruled 2026-11-16**, on being asked back, the user: *"308 (b) ok"*. So: **(b)**, the second rung only where an obligation is activated at root. What the entry puts beside the recommendation stands as recorded: no tracked program's answer shows the change, so task 4 adds a witness, and one probe that loads today is refused at load.

**Built 2026-11-16** by S1f.10.15's
task 4. The ladder's switch is one function, `hypgen::rung`, which the generator and `hypgen::eligible_relations`
both read: an hrule first, then the second rung where root's activators compile an obligation plan, by
`mechanisms::obligation_plans`, the reading the mechanisms check and *forbid*'s remedy share, else the blind
enumerator. A discharged root under an activated obligation still proposes nothing. What moved is what the
stage's inventory, taken before the switch was committed, listed: **41 corpus programs change rung, one changes
H** (`features/09_adjacent_via_same_house.ein`, 0 to 82 candidates, all dead with a dead root, three rows of its
read-out), **no R, exit code or `:expect` moves**, and 94 lines of the shape golden. The witnesses are the
register's **W97**, filed `open` and fixed: witness 2 over fewer names answers its control's 2 048 states, the
fourth answers 8 states, and **the re-plan's probe, which loaded, is refused at load by *forbid*** (`W97/connex-import`),
a change of behaviour named as such. `ein-cli/tests/import_invariance.rs` holds that a program importing
`std.algebra` for one rule and the same program with the rule written out take one rung, one H and one R. The
ladder is restated in the five places H15-4 names and three more. K-11 is unchanged by (b): its probe's rule is
activated and root owes it nothing, so it stays on the second rung. The record lists the defaults taken and one
finding met, H15-6.

### Q-M1f.309 — How `ein` shows a program's structure

**Decision file:** [`docs/decisions/cli/Q-M1f.309-ein-render-h-space-shows-the-hypothesis-space.md`](../../docs/decisions/cli/Q-M1f.309-ein-render-h-space-shows-the-hypothesis-space.md)

**Raised** 2026-11-16 at P1f.10's one stop, by S1f.10.12's second task,
for the user's first idea of the phase: *the structure is itself a valuable piece of information for
analysis*. Part 1 computed the structure in one owner, `ein_infer::structure`, and no command of `ein` shows
it: it is seen today through the examples `pair_layer` and `price_list` and the bundle
`utils/structure_bundle.py` writes under `build/structure/`.

**The options**, as the stage file has them:

- **(a) a view of `ein render`**, `ein render structure FILE`, writing what a page of the bundle holds: the
  graphs and a `structure.json`. `ein solve` is untouched and computes no structure it was not asked for.
- **(b) rows of `ein solve`'s read-out**, a `structure` row with the counts, the lists with `--dump-answer`.
- **(c) both.**
- **(d) no surface**: the bundle's script stays the way to see it.

**What part 1 measured that bears on it** (S1f.10.13): the
structure is not free. Over the 188 corpus programs that search, root's pair layer is 49 015 saturations and the
small KB's 82 589, where every `--single` of the corpus together is 19 677; on `zebra.ein` the layer whole takes
a describe from 445 saturations to 740. So under (b) a read-out row would cost most programs more than their
answer, which is what (a) avoids.

**The default named: (a).** It keeps each question's work the least it needs and puts no count in the read-out
that a reader cannot act on. **Not acted on**: no view is designed or built before the user's reply; as ruled
it is T1f.10.12.3, and the page that
defines the structure, T1f.10.12.4, follows it.

**Ruled 2026-11-16.** The user, shown the phase's four questions with their defaults at the stop: *"all defaults ok"*. So: **(a)**, `ein render structure FILE`.

**Renamed 2026-11-16.** The user, of the view as built: *"does `ein render structure` shows any structure? if only
hypothesis then rename it to something hypothesis-relate, like `ein render h-structure` or `ein render h-space`;
also can we get plain dump of hypothesis? we should be able to"*. The view shows the structure of the hypothesis
set H alone, so it is renamed: **`ein render h-space FILE`**, the main session's choice between the user's two
examples and a default the user may change, with no alias; its document is `h-space.json`, schema
`ein-h-space/1`, and its page
[`docs/ein/reasoning/02-kb-evolution/05-hypothesis-space.md`](../../docs/ein/reasoning/02-kb-evolution/05-hypothesis-space.md). And of the dump,
the same day: *"ein render writes dot, by listing hypothesis I meant listing them as ein facts, so add one more
top level command `show` and one sub-command `show hyps` that lists them"*. So: **`ein show hyps FILE`**, a new
top-level command with one subcommand, prints H as Ein facts, one candidate a line and nothing else on stdout,
at the cost of root's saturation and the generator's pass, held to `ein solve -H`'s candidates and to the
document's; the view gained no listing of its own. (a) stands: the structure is a view of `ein render`, and
`ein solve` computes none of it. Built by
S1f.10.12's follow-up.

### Q-M1f.310 — P1f.10, part 1: the defaults taken

**Raised** 2026-11-16 at P1f.10's one stop. Part 1 read and measured and changed no answer, route or surface;
what its stages decided without a ruling is listed in each record's *Defaults taken*, whole. The ones a later
stage builds on, each the user's to revise:

1. **Track R is required and reads track S's results.** Not a default: the user's correction of 2026-11-15
   (§ The two ideas), which re-ordered part 1 and re-scoped the price list. A way
   of spending the structure that does not pay closes that way and not the track.
2. **A group is a maximal set of two or more candidates that pairwise exclude by the theory alone**, over all of
   H, a candidate dead alone at root a member and marked
   (S1f.10.2, entry 3, and *Defaults taken here*). Taken by the main
   session after the stage's control failed under the other reading: the maximal cliques of root's own exclusion
   graph are 902 on `zebra2.ein` and none is a row or a column. Under this one the 36 groups are the 36 rows and
   columns of the five declared bijections. It is a cover and not a partition.
3. **Which statements are the puzzle's and which the theory's**
   (S1f.10.10, entry 1): the planned rule and four more were
   falsified by the corpus; the rule in force has two branches, by the rung that built H, and counts the row of
   houses as the puzzle's. It does not fit the programs the blind enumerator serves
   (K-5, K-9).
4. **An owed group is a group whose members are exactly a licensed instance's alternatives; a grid is strict,
   every member in exactly two groups; a bijection has equal families**
   (S1f.10.4, entries 1 to 5). Nothing is synthesised: where the
   *must* is missing the record says what a program would have to state.
5. **A node of the hypergraph is a question, a group or a candidate in no group, and every debt root owes is an
   edge of its own kind, licensed by L5 or not**
   (S1f.10.11, entries 1 to 5). The second was taken by the main
   session at the stage's stop: on six programs the conflicts' components are finer than R, each by a debt, and
   with the debts as edges R is the product over the components on every program compared. The other two ways
   put, *listed beside the edges* and *the question is the debt*, are in the record.
6. **How the price list's rule was read where it does not say**
   (S1f.10.13): a *go* needs both units, saturations and facts;
   a stopped cell counts with what it had spent; the rule is applied to a way of spending and not to a bare
   source; and ways the plan did not name were priced, `denials` among them, which is the one *go* for every
   question. **The tenth** in the rule, *no program higher by more than one tenth*, is the stage's number and
   the user's.
7. **Three test halves joined the nightly** (`pair_layer`, `structure_invariance`, `coupling`, each by name
   under `EIN_CORPUS_SLOW=1`), each measured within the five minutes on four P-core threads
   (K-2).

What part 1 found outside its stages' scope is `findings.md`, K-1 to K-13 and track H's
H9-n, H6-n and H15-n. **Stands until revised.**

**Accepted 2026-11-16.** The user, shown the stop: *"all defaults ok"*.

### Q-M1f.311 — A describe the budget would stop: answer what the tree can reach instead?

**Decision file:** [`docs/decisions/answer/Q-M1f.311-describe-takes-the-tree-and-the-classes.md`](../../docs/decisions/answer/Q-M1f.311-describe-takes-the-tree-and-the-classes.md)

**Raised** 2026-11-16 by S1f.10.14's
T4, the third place the stage file names where the engine has an *exactly one* it does not spend: describe takes
the state route and never the obligation tree. **Put, not ruled, to be shown at the phase's close**; nothing is
built for it, and describe's route, M1e2's decision, is as it was.

**What describe reads, and from what.** The state route visits every alive state once and saturates its one-up
extensions, the owing states among them. `k_c`, the merged states, the minimal line (`E`'s alive members, as the
minimal generators of R's states), `N` (the minimal transversals of the maximal alive states' complements) and
the shape are read off every alive state ([`states.rs`](../../ein.rs/crates/ein-infer/src/states.rs) § Where N
and E come from). `k_s`, the states listed, the maximal line and a query's classes need R's states alone, and the
obligation tree reaches those without the owing states its branches skip (L5 and L9): `-n N` lists every state of
R once its leaves run out, `--maximal` the line, `--single` whether R is one state.

**The numbers** (re-taken 2026-11-16 on S1f.10.14's engine, the default budget; the stage's table has eleven
programs). Four monotone programs have a describe the budget stops, and on each nearly all of it is alive owing
states while the tree reaches every state of R:

| program | describe | of it, alive owing | what the tree reaches |
|---|---:|---:|---|
| `beyond_arith/send_more_money.ein` | 336 101 saturations, stopped, no state met | 330 329 (98.3 %) | `--single`: *Solution*, R is one state, in 18 030; `--maximal` the same |
| `beyond_arith/send_more_money_written.ein` | the same | the same | the same, 18 030 |
| `zebra2-minus-15.ein` | 7 261, stopped, `k_s` ≥ 32, the maximal line not reached | 5 530 (76.2 %) | `-n 1000000`: all of R, 32 states, exhausted, in 104; `--maximal` the 32 in 108 |
| `zebra2-minus-15-obligations.ein` | the same | the same | the same |

Over the corpus, the price list's census (S1f.10.13, 188 programs, before L8 and the order): describe's alive
owing share is 343 323 of 1 350 675 saturations, 25.4 %, and 341 389 of it is the three of these four it had. Six
more describes the budget stops (`saturation/square-*/`) meet no owing state and decide R as the whole cube
already; seven are flagged programs past the cube's limit, where no tree runs. Where describe finishes, its
owing share runs from a fifth to four fifths of its work, and that is what `k_c`, `N`, the minimal line and the
merged states are read from.

**The options.**

- **(a) As it is.** Describe takes the state route; a run the budget stops says what it met, every count a lower
  bound, and a reader who wants R's states asks `-n N`, `--maximal` or `--single`, which take the tree. No surface
  moves.
- **(b) Where the budget would stop describe, answer R's states by the tree.** `k_s`, the states listed, the
  maximal line and a query's classes exact; `k_c`, the merged states, `N`, the minimal line and the shape said
  not computed. Describe projects a layer's cost before it saturates the layer, so it knows where it would stop;
  this changes describe's route, a decision of M1e2's, and a describe that stopped late says less of `N` than it
  does today.
- **(c) Both.** Describe as it is, and where the budget stopped it, the tree's states of R beside the lower
  bounds, in one run: R's states exact at the tree's cost on top (104 saturations on `zebra2-minus-15.ein`, 18 030
  on SEND), `k_c` and `N` still lower bounds.
- **(d) Say so, and nothing more.** Where describe is stopped on a program the tree can branch, the read-out names
  the question that would list R's states, and spends nothing.

**The default named: (a)**, which the plan's framing gives: describe's route is M1e2's decision, which the phase
reports on and does not reopen by itself. **Not acted on.**

**A fifth option, put by the main session on 2026-11-16 in answer to the user's questions** (*"how describe could
work the same way as --single but when there are potentially R contains many nodes?"*, then *"what is more
generic approach, independent from k_c = k_s?"*):

- **(e) A describe that visits each predicate's borders instead of every alive state.** Every quantity describe
  reports is a border of a monotone predicate over the cube, or a count over one:
  - R's **states** are the exhausted tree's leaves and the alive states above them (L5, L2, L3), as `-n N` lists
    them;
  - **`k_c`** is `Σ |class(s)|` over R's states, where `class(s) = { C ⊆ s : cl(C) = s }` is an up-set inside
    `2^s`: its count follows from its border, the maximal closed proper subsets of `s`
    (`|class(s)| = 2^|s| − |⋃_j 2^{m_j}|`), found by saturating subsets of `s` only, by dualize and advance with
    `cl` as the oracle;
  - the **minimal line** is the alive minimal discharged sets, the border of *discharged* against the maximal
    owing states;
  - **N** is the minimal dead sets, the border of *alive*.

  Each border is found at a cost polynomial in its size and its dual's (quasi-polynomial at worst), not in the
  number of alive states. On `send_more_money.ein`, where no rule writes `val`, every class has one member, found
  in eight saturations. The global borders' sizes, the maximal owing states and N, were not measured.

**Ruled 2026-11-16: (e).** The user: *"YES! a describe that visits each predicate's borders instead of every alive
state - lets pick this variant"*. Built by S1f.10.17,
which measures the borders against the state route before it builds, by the price list's rule.

**Measured 2026-11-16, and ruled again.** S1f.10.17's T2: the border route is identical to the state route on every
program both finish and to the oracle within its limit, but *no* for describe on the corpus (1 234 630 · 10 286 321
saturations · facts by the state route, 817 794 · 11 058 806 by borders, 20 programs higher by more than a tenth):
N, the global border, is larger than the alive states it replaces where root owes, and a class with many minimal
generators pays one saturation each. The cheap part pays: the tree and the classes give R, `k_c` and the minimal line
exactly, `send_more_money.ein --count` in 18 127 saturations where the state route stops. The user, shown the table:
*"S1f.10.17 - 2 modes: state route (existing) and tree plus classes, second by default, cmdline option to force one
or another, verbose warning about budget so user knows about forecast and options."* So describe and `--count` take
**the tree and the classes by default**, and **the state route on request**, by a command-line option; where a
route's forecast passes the budget, the run says so, with the forecast and the options. Built by S1f.10.17's T3.

**Built 2026-11-16** (S1f.10.17 § T3 as ruled).
`ein solve --describe-route tree|states`, `tree` the default: describe and `--count` take the tree and the classes
where root owes an instance L5 or L9 licenses, read off root's fixpoint, and the state route elsewhere, where the
tree's walk would be the state route's, or when asked. The tree and the classes compute neither `N` nor an empty R's
owing frontier, and the read-out says so with the members met and the option; `ein test` asks the state route for a
region claim over the frontier. A stopped describe's `note:` carries the forecast, what was established, what not,
and the options. Every field both routes compute is the same on every corpus program, each route is the oracle's
within its limit, and the option moves no part of R (`ein-infer/tests/tree_classes.rs`, `explore.rs`,
`claim_routes.rs`, `ein-cli/tests/lever_invariance.rs`). Over the 173 monotone programs that search: 605 068 · 8 828 073
saturations · facts against 1 234 652 · 10 286 374, 37 programs on the tree, six higher by more than a tenth (the two
`zebra2-minus-15` files, stopped either way, and four small ones); `send_more_money.ein` described, one state, in
18 127 saturations where the state route stops after 336 101.

**Two findings of P1f.10 left with this entry** when the phase's folder was deleted (S1f.20.9), as its close
disposed of them and the user accepted (*"Accept as proposed"*, 2026-11-16): K-21 *with Q-M1f.311*, describe's
next work, and K-13's stdlib half, *a stdlib change of its own, later*, which has no other owner. Verbatim from
the phase's `findings.md`:

| id | what | witness, and the command | found by | today | state |
|---|---|---|---|---|---|
| K-13 | **No route reads what a state denies.** An alive node's fixpoint may store `(not b)` for a candidate `b` outside its closure. Both routes then extend the state by `b`, saturate `S ∪ {b}` and watch it die, though on a monotone program that node holds `b` and `(not b)` whatever else it derives. The node function reports a node's closure and not its denials (`region::NodeOut`), and neither `tree.rs`'s `extend` nor `states.rs`'s step asks. On the zebras every dead node that holds a pair of the theory is one of these, since their rules store the negation: on `zebra2.ein` 38 of `--single`'s 45 saturations and 111 of describe's 374, on `zebra2-minus-15.ein --maximal` 2 246 of 2 346. On `send_more_money.ein` none is: `std.algebra`'s `functional` and `injective` conclude `(false)` from the two facts and store nothing, so the same exclusion is announced by one program's rules and not by another's. Reading the denials costs no saturation | `ein.rs/target/release/examples/price_list examples/zebra2.ein`, then `jq '.cells.single.ways.denials' build/structure/examples/zebra2/prices.json`; the corpus's table is S1f.10.13's record | S1f.10.13, 2026-11-16 | the routes as they are | open, and S1f.10.3's to take first: it is *at most one* spent with no pair layer, on the programs whose rules store negations, and the record's `denials` way prices it on every corpus program. Whether `std.algebra`'s `functional` and `injective` should also store the negation is the same question from the stdlib's side and is nobody's yet |
| K-21 | **A describe its budget stops saturates ⊤ though its own memory says ⊤ is dead.** Where the budget stops the state route it saturates the commitment of every candidate once, as route 1 does, to say whether ⊤ is in R (`states.rs`, *⊤ once*). It does so whatever it knows: on `send_more_money.ein` the stopped describe holds thousands of dead sets, each inside ⊤, and still pays the node of 80 candidates; on a monotone program L1 decides it. What the node gives beyond its death is its debts and its facts, which `Top` reports | `ein solve examples/beyond_arith/send_more_money.ein`, whose last saturation is ⊤'s; `grep -n 'when the budget stopped the route and' ein.rs/crates/ein-infer/src/states.rs` | S1f.10.3, 2026-11-16 | as it was: one saturation of a stopped cell, and S1f.10.13's `singles` way was *go* under describe by this node alone | open. Asking the memory first is one line; it changes what a stopped read-out says of ⊤ where ⊤ owes, so it wants its own look |

### Q-M1f.312 — What the shape golden's `hyp` and `hyp+closed` ops digest

**Raised** 2026-11-16 by S1f.10.16's
T1 and T2, from `findings.md`'s H9-5. **Put, not ruled, to be shown at
the phase's close.** The stage was told not to remove an op from the per-commit shape check on its own judgment,
so both ops are as they were, and so is every piece they call.

**What each op digests.** `ein-render/tests/golden/corpus_shapes.md5` holds a line of each for every corpus
program that loads, 659 of each (391 outside `tests/`, which the per-commit sweep reads; the nightly reads all).
Both are `ein_infer::shape::hyp_shape_with`: root saturated with events off, then **the walk's generator**,
`hypgen::generate`, narrated at verbose (its `compile`, `rung`, `hyp` and `hypskip` events), its counters
(`--hyp-stats`' old report, `BALANCE`), and a trailer, `COMPLETE` and `OPEN`, from two more calls of it. `generate`
is the generation ladder with the walk's one-step lookahead on and its kill cache writing a `(not h)` for each
candidate the lookahead drops, both read from two retired config fields at their defaults. `hyp+closed` is the
same after `closed::emit_closed` has written a `(__closed__ R)` for every declared relation no compiled rule
concludes, with a `CLOSED […]` line.

**Who reads them.** `id_order_invariance` digests both against the golden and holds them under a permuted id
space; nothing else reads the golden lines. `hypgen_coverage.rs` reads the `hyp` text, not the line, for its
eight counters. **No command runs either computation**: H is `hypgen::generate_config_free`'s list at root
(`region::prepare_with`), the same ladder with neither the lookahead nor the kill cache, sorted by raw text;
no command calls `emit_closed`.

**How far they are from H.** Measured by a scratch probe on `060576ec` over the 659 programs (kept as
`build/s1f.10.16/probe.rs` and its output, not committed): **H is exactly `generate`'s emitted candidates with
its lookahead-killed ones, on all 659**; the `hyp` op's event stream is the narration of H's pass, candidate for
candidate, once four things are set aside, and identical outright on 91. The four, each something no route runs:
`Lookahead::new`'s `compile` events (568 programs), the verdict `lookahead_killed` (59), the kill cache's
`negated_fact` for a repeat H's pass calls `seen_in_call` (29), and the ordinals they shift. `emit_closed` closes
a relation on 612 of the 659. So `hyp` is the golden's only digest of the generator's narration and counters, and
it digests them under two filters no command applies; `hyp+closed` digests them after a pass no command calls.

**What each keeps alive.** `hypgen::generate`'s reading of the two retired lookahead fields, `Lookahead`, the
kill cache (`write_negated`), `complete` and `open_hypotheses` have no other reader but tests and two examples
(`hypgen_calls.rs`, `hypgen_cost.rs`), and `emit_closed` none but tests; S1f.10.16 kept each for the ops
(its T2).

**The options.**

- **(a) Both as they are.** Nothing moves. The walk's generator and `emit_closed` stay for the golden, and a
  change to the lookahead re-banks lines of programs whose answers do not move, as the fix of W96 re-banked 185
  `hyp+closed` lines.
- **(b) `hyp` re-pointed at H's pass, `hyp+closed` removed.** `hyp` digests `generate_config_free` at root as
  `ein solve --events` narrates it, its counters and H's size: what every command builds, which no line of the
  golden digests today. Moves: the `hyp` line of at least the 568 programs with a `compile` event, the exact set
  by the re-bank; all 659 `hyp+closed` lines go. After it the lookahead, its kill cache, `generate`'s config
  reading, `complete`, `open_hypotheses` and `emit_closed` have tests and examples as their only readers, and
  S1f.10.16's rule deletes them in a later stage, with the two retired fields' last code reader.
- **(c) `hyp` re-pointed, `hyp+closed` kept**: `hyp+closed` alone keeps the walk's generator and `emit_closed`.
- **(d) `hyp+closed` removed, `hyp` kept**: its 659 lines go and `emit_closed` has tests alone; the walk's
  generator stays for `hyp`.
- **(e) Both removed**: the generator's narration and counters are then in no golden. H itself is still held by
  the read-out's `universe` rows on every program, `ein show hyps`' tests and the routes' invariances.

**The default named: (b).** The golden then pins what a command produces and nothing only the golden runs,
which is the rule S1f.10.16 settled every other piece by. **Not acted on**: both ops, and every piece they
call, are as they were.

**Ruled 2026-11-16: (e), remove both.** The user, asked at the phase's close with the options above, chose *"(e)
Remove both"*. So the shape golden digests no hypothesis generation, H being held by `ein show hyps`' tests and the
routes' sweeps; and what only the two ops read (the walk's generator `hypgen::generate`, its lookahead and kill
cache, `closed::emit_closed`) has no reader left and is deleted by S1f.10.16's rule.

**Built 2026-11-16** by S1f.10.16's record as ruled at the close, inventory first (`c7c1e4fd`),
then the code (`26d0bc56`). `corpus_shapes.md5` lost its 661 `::hyp` and 661 `::hyp+closed` lines and nothing
else (blessed with `EIN_CORPUS_SLOW=1`, every program permuted), and no other golden moved. Deleted with their
tests: `hypgen::generate` with its config reading, `complete`, `complete_counted`, `open_hypotheses`, `consistent`
and `is_solution_node` (K-30); the lookahead (`lookahead.rs`, the kill cache, `Drop::LookaheadKilled`, the two
matcher entries only it called); `closed.rs` (`emit_closed`, `producible_relations`); `shape::hyp_shape`; the stats
report; `hypgen_coverage.rs`; the examples `hypgen_calls` and `hypgen_cost`; two counters; and `Kb`'s no-good store
with the `.einb` `NOGOODS` section (K-29), with no layout change. Three test helpers that asked what the generator
proposes ask H's pass, `generate_config_free`. Kept, with their readers: the writeback name
`<lookahead-dies-immediately>` (the grammar refuses it) and the fifteen retired config fields (the load refusal, the
`.einb` `META` layout). No answer, read-out line or event moved: no command ran any of it.

### Q-M1f.313 — May the agenda inside a route's node fire first what leads to `(false)`?

**Decision file:** [`docs/decisions/engine/Q-M1f.313-the-agenda-fires-first-what-leads-to-false.md`](../../docs/decisions/engine/Q-M1f.313-the-agenda-fires-first-what-leads-to-false.md)

**Raised** 2026-11-16 by S1f.10.19's
T3, on the user's *"Maybe we can also optimize saturation in general by firing first rules that emit owed
relations"* and *"Yes, both"* of that day, to the main session's answer: stop a dead node at its first contradiction
on the routes, and fire first what leads to `(false)`. **Put, not ruled, to be shown at the phase's close.** The stop
under FIFO needs no ruling: it cuts the same derivation shorter, and S1f.10.19's T4 builds it after S1f.10.17. The
order is put here and **not acted on**: the agenda is FIFO as ruled, and a fail-first agenda exists only as a
measurement's switch, `ein_infer::saturator::fail_first`, which no route, option, `(config …)` field or `EIN_*`
name reaches.

**What it reopens.** [Q-M1f.6](#q-m1f6--the-derived-agenda-holds-the-cores-the-fifo-boundary-does-not), re-ruled
(a) on 2026-11-08: *"plain FIFO, core is an explanation, F22 makes it canonical"*, so the agenda and the boundary
are FIFO in written order, no band orders either, and Q-M1f.4 (b)'s derived agenda was withdrawn with it. A
fail-first agenda is such a band, derived from the rules' conclusions: the rules whose conclusion a contradiction can
be made of first, the rest after, FIFO within each half, and the boundary walking its parked candidates in the same
order. The user's reasoning at that ruling, that an order of the rules is in effect an optimisation and needs only
the saturation results to agree, bears on it; so does what moved then, cores and narration.

**The two agendas measured**, read off the compiled plans by `monotone::writes_of`: **`pairs`**, a conclusion that is
`(false)`, a stored negation, or a fact of a relation whose negation some plan concludes or the KB holds (either side
of a pair, the stage's reading); **`negations`**, `(false)` or a stored negation alone.

**The numbers** (S1f.10.19's T2, `ein-infer/examples/early_stop.rs`, 2026-11-16: 200 programs that search, 171
monotone, every question at the default budget, every distinct node a route visits saturated again under each order;
the table). On the monotone
programs, the cells' firings with the stop under FIFO, then under each agenda:

| question | FIFO, the stop | `pairs` | `negations` | facts: FIFO, the stop | `pairs` | `negations` |
|---|---:|---:|---:|---:|---:|---:|
| describe, `--count` | 12 294 482 | 12 284 015 (−0.1 %) | 12 200 271 (−0.8 %) | 9 993 816 | 9 983 351 | 9 971 043 |
| `--single` | 194 729 | 169 570 (−12.9 %) | 169 010 (−13.2 %) | 380 624 | 355 465 | 355 391 |
| `-n 1` | 155 717 | 133 038 (−14.6 %) | 132 866 (−14.7 %) | 340 827 | 318 148 | 318 140 |
| `--minimal` | 5 291 083 | 5 225 817 (−1.2 %) | 5 088 188 (−3.8 %) | 3 774 714 | 3 709 448 | 3 690 506 |
| `--maximal` | 287 853 | 262 449 (−8.8 %) | 256 254 (−11.0 %) | 447 012 | 421 590 | 421 086 |

Today, with no stop, the same cells are 17 245 101, 392 264, 283 525, 13 308 267 and 892 608 firings: the stop
under FIFO is most of the saving, and the agenda adds the rest, most of it on the two SEND programs
(`send_more_money.ein --single` 65 442 → 52 882 firings under either). By S1f.10.13's rule, each agenda with the
stop is ***go* against FIFO with the stop on all six questions, with no program higher by a tenth**. `pairs` is higher
than FIFO on no cell; `negations` is higher on `zebra2.ein` and `zebra2-obligations.ein`, by under one percent.

**What it would change.**

- **R: nothing measured.** Under both agendas the fixpoint of every one of the 1 872 755 distinct nodes the routes
  visit (251 733 dead, 1 621 022 alive, the 29 flagged programs' cube nodes among them) is FIFO's, and so is its
  count of firings to quiescence: the agenda changes when a fact is derived, not what. Argued, not met: on a program
  whose negative cycle has a rule in each half, the boundary would admit another candidate first, and a fixpoint can
  move, which is F21's D2 ground and why Q-M1f.6's ruling kept the boundary FIFO; the corpus's cycle
  (`features/07_unstratifiable.ein`) has both its rules in one half.
- **Work**: the table above, in the routes' units; root's saturation and the oracle's are not measured under an
  agenda.
- **Cores, a core being an explanation that may vary with rule order (Q-M1f.6).** With the stop, of the 121 105 dead
  nodes of the monotone programs: FIFO's stopped core differs from the fixpoint's on 6 268, whatever the agenda;
  `pairs`' stopped core differs from FIFO's stopped core on **none**, `negations`' on 3 012. To quiescence, `pairs`
  moves no core, `negations` 538. On the flagged programs neither agenda moves a core.
- **What a dead node reports when the stop is built**: the node as far as its first contradiction, under the order.
  Against FIFO's stop, `pairs` moves the facts of 89 990 dead nodes, the closure of 2 and the debts of 1;
  `negations` the facts of 92 848, the closure of 2 761, the denials of 2 879 and the debts of 2 709.
- **Narration**: every node's `--events` lines are in the agenda's order, an alive node's too.

**The options.**

- **(a) FIFO as ruled.** The routes' dead nodes stop at their first contradiction under FIFO (S1f.10.19's T4); the
  agenda stays the written order everywhere. Nothing of Q-M1f.6 moves.
- **(b) A fail-first agenda in the routes' nodes, on a monotone program.** Every node a route computes, alive or
  dead (a route cannot know which before it saturates it), with the stop on the dead; root's saturation, the
  oracle's node, a flagged program's cube, the whole node and `region::Nodes`' readers (a state's trace, a member of
  N's refutation, the detector's re-fixpoints) stay FIFO. The oracle keeps the definition; R, N and every count are
  held to it with and without, as the stop is. Q-M1f.6's ruling then stands for the definition and gives way inside
  the routes.
- **(c) A fail-first agenda everywhere**: root, every route, the oracle, every reader. Q-M1f.6 reversed for the
  agenda; the boundary's order with it. The oracle's cores move as the routes' do (`negations`: 538 at quiescence on
  the corpus; `pairs`: none), and every narration.

Under (b) and (c) the reading is a second choice: `pairs` (no core moves on the corpus, no cell higher than FIFO) or
`negations` (more saved under `--minimal` and `--maximal`, cores move).

**The default named: (b), with `pairs`.** By the stage's rule it is *go* against FIFO with the stop on every question,
it loses on no cell, it moved no fixpoint and no core of the corpus, and the oracle keeps the definition, so the
ruling's ground, that an order is speed and a core an explanation, holds of it. (a) is the ruling's own consequence
if the written order is part of the definition the user wants every node to keep. **Not acted on**: no route takes
the switch, and S1f.10.19's T4 builds the stop under FIFO alone unless this is ruled otherwise first.

**Ruled 2026-11-16: (c)**, a fail-first agenda everywhere: root, every route, the oracle and every reader, with the
`pairs` half first (rules concluding `(false)`, a stored negation, or a fact of a relation whose negation is concluded
or stored). The user: *"Q-M1f.313 (c) non-monotone programs still out of scope, we know about them, can detect them,
can solve them somehow, but don't claim about solution soundness, correctness or completeness or anything."* It
supersedes Q-M1f.6's *plain FIFO* for the agenda's order; a core stays an explanation. On a monotone program the
fixpoint does not depend on the order; on a program the static check flags, the engine claims nothing of its answer
under any order. Built by S1f.10.19 after its T4.

**The user's reason, the same day**, on being asked what the earlier position had been: *"yes, we don't claim
anything because even first "non-monotonic case R is completely defined only by full set of nodes" is not true,
because for non-monotone programs `sat(root + C1 + C2) = sat(sat(root + C1) + C2)` is not always true"*. So the
2026-10-16 position quoted in `solution_semantics.md` § 2.1 holds of monotone programs only, and the page says so.

**Built** 2026-11-16 by S1f.10.19,
after the stop under FIFO: the agenda with the `pairs` half first is the engine's order, `saturator::AGENDA`, taken by
every saturation, root's, every route's node, the oracle's, the whole node and every reader's, and by a resumed one
from its snapshot; the boundary walks its parked candidates in the same order. `saturator::fifo` is the switch for
the written order alone, which no lever reaches, and `ein-infer/tests/agenda.rs` holds every question and every claim
of every monotone corpus program alike under both. Of a flagged program nothing is held; the one gate test that held
a flagged program's cube to what the written order shows, `region.rs`' `fact_grain_on_programs_written_to_test_it`,
computes that cube under the switch, named in the test with why.

### Q-M1f.314 — What the release-voice check reads, and what fails a line

**Raised** 2026-11-16 by S1f.20.1's
T1f.20.1.3. **Defaulted, to be shown at S1f.20.2's
review**, with the sweep's defaults. The stage file names the check's scope (`docs/ein/`, the shipped crates' doc
comments, `.ein` header comments, `corpus.toml`, `AGENTS.md` but § Where the work is, `README.md`) and what fails
(a milestone, phase, stage or task id, a path into `plans/`, a date that dates a development event; a decision id
passes). Seven readings it left open, each taken as below and built that way (`utils/check_release_voice.py`); the
baseline, 6 743 lines in 1 041 files, is of these readings.

1. **A shipped crate's doc comments** are every `///` and `//!` line of a `.rs` file under the six shipped crates,
   their `tests/`, `examples/` and `benches/` included: 3 271 baseline lines. *Alternative:* `src/` only. Plain `//`
   comments are not read; S1f.20.9 widens the check to code,
   comments and tests.
2. **A `.ein` file's comments** are all of them, not only the leading block: 2 501 baseline lines. One reader serves
   both checks (`doc_audit.py`'s, the text after a `;` outside a string), and a body comment speaks the same voice
   as the header. *Alternative:* the header block only.
3. **A date** is any ISO date, `YYYY-MM-DD`, in scope: no rule tells a development event from another date, and
   the escape, `release-voice-ok: <reason>`, covers the exception. *Alternative:* a date after *since*, *on*,
   *ruled*, *measured* and the like.
4. **A path into `plans/`** is `plans/` followed by a name, bare or relative; the folder named alone (*the stage
   records under `plans/`*) passes. *Alternative:* the folder's name fails too.
5. **What it does not read yet**: followup ids (`F21`), finding and register ids (`E-33`, `W93`, `K-82`), the M1e
   review's ids (`CD-H1`, `TE-L3`), and the strings of a `.ein` file (`:why`, `:source`, `:goal-text`), of which at
   least `stdlib/elim.ein`'s `:why "M1-blocking stdlib"` and the dates in `examples/lattice/synth/*.ein`'s carry
   the development. S1f.20.9 counts `E-n` and `D-n` as plan ids, so it widens the patterns with the scope.
6. **A count that falls is reported, not failed**: the check exits 0 and names the file to re-bank with `--bless`,
   as the stage file states the failure (*"where a count rises or a new file appears"*). *Alternative:* fail both
   ways, the baseline held exact as `tests/mutation_survivors.txt` is, so that a count lowered without a re-bank
   cannot later rise back to its banked number unnoticed.
7. **The check reads itself**, so its own docstring is held to the voice it enforces; its two lines of examples
   carry the escape, which were its first findings.

**Ruled 2026-11-17 as defaulted.** The user, at S1f.20.2's review, item 13, verbatim: *"Q-M1f.314: the release-voice
check reads every doc comment and every .ein comment, and every ISO date fails a line. A falling count is reported,
not failed. F/E/W/K ids are not read yet."*

**The readings S1f.20.9 changed**, when it widened the check and deleted its baseline: (1) every comment of every
crate, plain, trailing and block, the dev-only crates and the grammar's sources too, not a shipped crate's doc
comments; (4) `plans/` named alone fails, as `utils/` does: the project names no process; (5) followup, finding,
register and review ids are read now, and what is still not read is a `.ein` string (`:why`, `:source`,
`:goal-text`), a Rust string, a plan's `D<n>`, a `C<n>` case and a commit hash (L-2, L-49, L-54 and L-85 of
[the milestone's findings page](findings.md)); (6) moot: with no baseline nothing is banked, so nothing falls; (7)
the check does not read itself. The scope also widened from `docs/ein/`, `docs/decisions/` and `README.md` to
every `.md` of the project, and gained `corpus/regressions.toml`; `AGENTS.md` left it, being the process's.
Readings (2) and (3) stand. P1f.20's folder, with both stages' records, is deleted
(`git log --diff-filter=D -- 'plans/m1f_hypothesis_and_documentation/p1f.20_*'`).


### Q-M1f.390 — What an obligation emits: the verdict, with its justification or as a fact

**Raised** 2026-11-17 by the user's instruction that opened [P1f.21](p1f.21_verdicts_as_rules/README.md): *"… can
say that if treat (open ?R) as facts then they are only facts that (1) duplicates (2) can be removed from KB; but
maybe better would be to emit (open) facts as (open ?R <justification>) ?"*. **Put, not ruled.**
[S1f.21.1](p1f.21_verdicts_as_rules/s1f.21.1_the_survey_and_the_design.md) surveyed the kernel and designed the
form; [`design.md`](p1f.21_verdicts_as_rules/design.md) has every run, and its
[§ 5](p1f.21_verdicts_as_rules/design.md#5-the-question-and-a-recommendation) is the page to rule from. The ids
between Q-M1f.314 and this one are in use: each that is decided has its file under `docs/decisions/`.

**What the survey found** (each a run in `design.md`):

- **The two properties are real.** On `examples/lattice/05_owed_seats_bounded.ein` root owes 4 instances and they
  are one tuple, `open(seats)`; `{(seats Ann C1)}` owes 2 and holds every other fact root holds. On
  `examples/zebra2-obligations.ein` root's 36 instances are 5 tuples.
- **(1) goes with one justification, the instance**: the rule's name, its activator's arguments and its
  trigger's bindings. Under it a tuple is a debt at every one of the 120 nodes `ein solve
  examples/zebra2-obligations.ein` enters; under the activator alone at 70, under the trigger's matched facts
  not where two obligations over one relation share a trigger, and a payment is a tuple per alternative, not per
  debt. The engine already computes the instance, keys the tree's table by it, and prints it on every `owe`
  event.
- **(2) goes with no justification.** A fact that means *owed now* is in a state and not in a larger one, and a
  fork inherits what its parent holds. Written as an ordinary rule, the debt outlives its payment in 15 of that
  program's 16 nodes, the engine reports each as a *stale fact*, the static check flags the program and the
  route is the cube.
- **What can be facts** are the requirement (from the trigger), its payment (from the trigger with its witness)
  and *discharged* (every requirement is paid): three ordinary rules today's Ein runs, with the static check
  *monotone*, the count of unpaid requirements equal to the kernel's `owes` at all 16 nodes of the first program
  and all 120 of the second, and every field of the answer the same.
- **No choice changes $R$**: $R$ reads only whether anything is owed.

**The options**, in each the justification being the instance, made by the kernel:

- **(a) As today.** The rule concludes `open(r)`, the pass counts matches, and the reasoning pages state the
  three verdicts as rules and say of `(open R)` what the instruction says. Nothing is built.
- **(b) The verdict carries its justification and stays a verdict.** The pass's result is the set of tuples
  `open(r, j)`: facts of the extent of `open` that are true of the state and that no knowledge base holds. The
  pages write it so. Nothing need be built; the engine may print the instance as one term on `owe` and in the
  answer dump, which is additive.
- **(c) The requirement and the payment are facts, and *owed* is their difference.** The kernel emits both in
  the agenda; nothing is removed; `(discharged)` can be a fact, and $R$ the states that hold it and not
  `(false)`. To build: two more plans an obligation, two reserved relations, the listings, every golden of a
  program with an obligation (78 of the 567 corpus programs `ein solve` answers). On
  `examples/zebra2-obligations.ein` the run's facts beyond root go from 5 409 to 8 623 in the emulation.
- **(d) `(open R j)` is a fact of the state that owes it**, written by the pass after the fixpoint into a part of
  the node no fork inherits, matched by no rule. A state's facts then no longer grow with its commitment. To
  build: that part of a node, the fact in the grammar and the loader, the dumps.

*The agenda derives `(open R j)` as any conclusion* is not offered: its run is the stale debt above.

**Recommended default: (b).** It is all of the instruction's *maybe* that can be had: the duplicate is gone, and
the removal is what a verdict is, as *alive* is. It moves no licence, check, golden or state, and it does not
close (c), which a later phase can build or a module can state. **The strongest argument against it**: the debt
stays outside the knowledge base, so no rule and no claim reads one and the pass remains a second mechanism;
(c) is the option that makes owed and discharged ordinary rewriting, with runs behind it, at the price of its
build and of a requirement the knowledge base states beside the rule's guard, which
[Q-M1d.2](../../docs/decisions/hypotheses/Q-M1d.2-an-obligation-is-a-rule-that-concludes-open-r.md) chose not to
have.

**What waits on it.** S1f.21.3, planned only on (c) or (d), or on (b) with the term printed. S1f.21.2 does not
wait: it states today's kernel as rules under every ruling, and the ruling decides how it writes the verdict and
the remark ([`design.md` § What S1f.21.2 writes under each ruling](p1f.21_verdicts_as_rules/design.md#what-s1f212-writes-under-each-ruling)).

**The user's direction, 2026-11-18**, verbatim in [P1f.21 § The second instruction](p1f.21_verdicts_as_rules/README.md#the-second-instruction) (*"instead of emitting (open ?R) emit (open (?R ?a
?b)) - the actual hypothesis … Can it be flexible or transactional move from open into global true context or into
not context?"*). Not a ruling among the four options: it takes (d) further. The open facts are the hypotheses
themselves, rules read them, and they leave the open context as a state decides them. Worked out in
[`open_facts.md`](p1f.21_verdicts_as_rules/open_facts.md):
- the open facts are $H$, and a debt is a disjunction of them, so `(open F J)` keeps the count;
- the move is per node, after the fixpoint;
- rules over open facts run in a stratum of their own and conclude no held fact.

S1f.21.4 puts it again on programs.

### Q-M1f.391 — The negation of a negation that a rule builds

**Raised** 2026-11-17 by S1f.22.1, finding N-1 of
P1f.22's findings. **Not ruled.** The programs, the commands and the options'
costs are `analysis.md` § 4.1
and § 1.8.

A conclusion `(not ?x)` whose variable holds a stored negation, bound under an arrow of a fact such as
`(think Alice (not (= 5 (+ 2 2))))`, stores `(not (not (= 5 (+ 2 2))))`. The knowledge-base page, the user's
objects page and Q-M1f.113 say no such fact exists, the grammar refuses it where a file writes it, and the state
file of a state that holds one does not load.

- **(a)** nothing is concluded where `?x` holds a negation, as where it holds a name (Q-M1f.128), and the loader
  notes where the text shows that it can. **Default.**
- **(b)** the denial of a denial holds the fact: `(not ?x)` with `?x` the negation of `g` concludes `g`.
- **(c)** it is a fact: the grammar, Q-M1f.113 and the knowledge-base page admit `(not (not g))`.
- **(d)** as it is, documented.

**The user's direction, 2026-11-17, to confirm.** *"Rule for (not (not g)) -> g is a simple rule, essential negation
elimination, already trivial in ein. That's why I also raised a question about (and g (not g)) -> (false) could be
a rule in p1f.21."* Read as **(c)** with the elimination a rule: `(not (not g))` is a fact a file may write and a
rule may match, the kernel is not changed, and the rule that concludes `g` from it is a theory's, its home
P1f.21's to say. The rule is not writable today (the parse refuses `(not (not …))`), though a state can hold the
fact. What it would take, and what stays out, is P1f.22's `contexts.md` § 6, copied to
[P1f.21's `open_facts.md` § 10.5](p1f.21_verdicts_as_rules/open_facts.md#105-the-negation-of-a-negation-and-its-elimination-as-a-rule).

**Ruled 2026-11-17: (c), the elimination a rule.** The user, to the reading above: *"Q-M1f.391 OK"*.

### Q-M1f.392 — A state file that reads back dead, through `written`

**Raised** 2026-11-17 by S1f.22.1, finding N-2. **Not ruled.**
`analysis.md` § 4.2.

A state file writes a derived fact as a fact, so its reload states `written` for a fact a rule assembled under an
arrow, which the run that wrote the file did not; where a rule reads `written`, the reload can be dead. The
answer-dump page says a reload derives no fact and no `(false)`.

- **(a)** the state file marks what the state derived or committed, and the loader states no `written` for a
  marked fact.
- **(b)** `written` follows every held fact, derived and committed ones too.
- **(c)** the answer-dump page, the rendering page and the file's header say it, as Q-M1f.185 says of a state
  nested past the parser's limit; the engine is not changed. **Default.**

**The user, 2026-11-17:** *"I don't understand - write with details into separate file in plans."* Written:
`q392_state_file_and_written.md`. Still not ruled. (The
user's *"Q-M1f.392 (a) for now, I don't yet know why we would need to change it"* is read as Q-M1f.393's (a),
below: this question's (a) is a change, and the sentence before it says the question is not understood.)

**The user, 2026-11-17, again:** *"I still do not understand, show an issue, how to reproduce it, explain for
dummies."* The page is written again from the start, with the program
`q392_demo.ein` and three commands. Still not ruled.

**Ruled 2026-11-17: it is a bug.** The user, shown it as *"`ein` can save a solution to a file. For some
programs, loading that saved file back says there is no solution"*: *"it is a bug and has to be verified with
simple unit test. Where from comes (written ...)???? Why? What for?"* So option (c) is out: a saved state loads
back as that state, and a unit test holds it. Where `written` comes from and what reads it, and the fix proposed
with its test, are the last sections of
`q392_state_file_and_written.md`. The fix is S1f.22.3's
first task and is built on the user's word on it.

**The requirement, 2026-11-17.** Asked *"should a saved solution always load back as that solution?"*, the user:
*"not correct question; save and load must in both directions must have identical results: save -> load -> save
byte identical, load -> save -> load - have identical dumps; both save and load must not modify the program, must
not add or remove any bit of information"*, and then *"what is the reason to modify during saving?"*. The answer
given: nothing forces it. The state file was designed as a new program whose given facts are the state (the
answer-dump page: *"not a second kind of program that re-states where each came from"*), so the save writes a
derived fact as a typed one with its rule in a comment, leaves out every `written` fact for the loader to state
again, and writes the program as loaded with its imports inlined; the existing round-trip test compares by value
and takes `written` as a superset. Measured on a program of one rule and no `written` (the gate build): a second
save differs from the first and each round appends a copy of the state's facts to the program's part; the
original holds `(note a :rule make)` and the reloaded file `(note a)`, as given. So the fix proposed above, a
clause on a saved line, is not enough, and the design is open on two points that are the user's: **what a saved
solution is** (the program unchanged with the choices made, the state derived again on load; or every fact written
out with where it came from), and **whether it must load with no other file beside it** (which is what inlining
the imports was for, and is a change to the program).

**Ruled 2026-11-17, the two points of design.** The user: *"What a saved solution is - (a); Must a saved file
load with no other file beside it? - yes"*. So a saved solution is the program unchanged with the choices made,
the state derived again on load, and it is one file that loads alone. The two pull apart where a program imports,
and what keeps both is the modules' texts carried in the file as they are;
S1f.22.5 designs that, the form for the choices and what *save*
and *load* are as commands, and shows it on real programs before anything is built.

### Q-M1f.393 — A fact held in a variable, as a conclusion or a premise

**Raised** 2026-11-17 by S1f.22.1. **Not ruled.**
`analysis.md` § 4.3.

A rule unpacks a fact from under an arrow by spelling its relation and its count. It cannot conclude `?f` for a
variable bound to a whole fact (Q-M1f.351), nor write `?f`, `(not ?f)` or `(absent ?f)` as a premise; it can
conclude `(not ?f)`.

- **(a)** as it is. **Default.**
- **(b)** a variable is a literal of a conclusion and a premise.
- **(c)** a variable is a premise and no conclusion.

**Ruled 2026-11-17: (a), on a reading to confirm.** The user wrote *"Q-M1f.392 (a) for now, I don't yet know why
we would need to change it. The only strictly required for now is to know fact's arity, for all other you can use
variables."* It names Q-M1f.392 and describes this question's (a): a rule unpacks by spelling the fact's count, the
relation a parameter where wanted. One thing (a) leaves out is noted for the user in
P1f.22's `contexts.md` § 6, copied to [P1f.21's `open_facts.md` § 10.5](p1f.21_verdicts_as_rules/open_facts.md#105-the-negation-of-a-negation-and-its-elimination-as-a-rule):
a variable for a whole fact may be denied at the top of a conclusion and may not stand under `not` in an argument,
so the rule *deny whatever a liar thinks* has no twin inside a context.

**Confirmed 2026-11-17.** The user: *"Q-M1f.393 yes (a)"*.

### Q-M1f.394 — The words for a fact under an arrow

**Raised** 2026-11-17 by S1f.22.1. **Not ruled.**
`analysis.md` § 4.4, with the
pages' words in § 2.5.

1. The three cases: **(a)** *held* and *in a context*, the context of a fact being the held facts that point at
   it at any depth (**default**); **(b)** the knowledge-base page's *held* and *only pointed at*, the third case
   unnamed.
2. The fact a stored negation denies: **(a)** is in the negation's context, the nesting function of the
   knowledge-base page staying as it is (**default**); **(b)** is not.
3. *Value* and *term*: **(a)** the reasoning's and the user's pages say *fact* wherever it stands; *value* stays
   in Q-M1f.113's title and the diagnostics; *term* stays in the standard library's pages, defined once in the
   glossary (**default**); **(b)** *value* is the user's word and *object* the reasoning's, each page saying once
   that they are one.

**The user, 2026-11-17:** *"explain with examples in separate file."* Written:
`q394_words.md`. Still not ruled. The user's own words in the same note,
*global context* and *local/fact context*, are taken into option (a) of the first point as a second name for the
knowledge base.

**Ruled 2026-11-17**, all three points, in the user's words. (1) *"held = in the global context; in a context =
in a fact's (local) context"*. (2) *"`not` obviously holds something like logically mirrored world where all true
facts are false and all false are true, right? it looks quite consistent with context idea here"*: a negation is a
context; what the mirror is and is not is
P1f.22's `contexts.md` § 11, copied to [P1f.21's `open_facts.md` § 10.6](p1f.21_verdicts_as_rules/open_facts.md#106-not-as-a-context). (3) *"object - in core ein theory
explained on graphs, we have to say what are all those nodes - they are objects; value - … Use word `value` only
when talk about variables in rules, … the object is a value only when binded to variable; `relational node` -
don't use it, it is too SQL-ish; graph node - ok; In docs/ein/ein.rs - use whatever naming convention you want,
just make software documentation good; term - is mostly about syntax"*. The user also found that the survey's
quotations of the pages were behind the pages (N-16 of the phase's findings).

### Q-M1f.395 — What `ein render` draws of a fact under an arrow

**Raised** 2026-11-17 by S1f.22.1, findings N-3 and N-4. **Not
ruled.** `analysis.md` § 4.5.

- **(a)** the rendering page says what the drawing leaves out; no writer changes. **Default.**
- **(b)** the node of a nested fact is filled where the knowledge base holds the fact.
- **(c)** a view that draws the graph as the reasoning defines it.

**Ruled 2026-11-17: not (a).** The user: *"The implementation (ein.rs) renders nested facts as atoms with names
made from entire tuple. It is not valid because if fact present in multiple contexts it will be drawn multiple
times, while canonical (semi-Levi-bipartite) graph has every object drawn once."* So the text node goes, from the
writer and from the knowledge-base page's table of the labelled-arrow form. The shape to build, to confirm, is
`contexts.md` § 7.3: a fact that a fact points
at is drawn as its node with its own arrows, held or not, in `ein render kb` and `ein render rule`; how *held* is
marked is Q-M1f.398.

**Confirmed 2026-11-17.** The user: *"yes, 3-arity fact can an arrow only when nothing points on it"*.

### Q-M1f.396 — Where the explanation stands, and which example it uses

**Raised** 2026-11-17 by S1f.22.1. **Not ruled.**
`analysis.md` § 4.6.

- Where: **(a)** sections of `01-knowledge-base.md` and `02-rewriting-rules.md`, a sentence each on the pages of
  `02-kb-evolution/` (**default**); **(b)** a third page of `01-static-ir/`.
- The example: **(a)** the user's, `(think Alice (= 5 (+ 2 2)))`; **(b)** the three-house example's `says`;
  **(c)** both (**default**).

**The user, 2026-11-17: (a) for where**, *"Q-M1f.396 sections"*. The example was not named, and the default stands.

### Q-M1f.397 — The reserved words in the three perspectives

**Ruled 2026-11-17** by the user's note at the start of P1f.22, verbatim: *"In
the docs/ein/reasoning all those reserved words are conceptual mathematical objects. But in docs/ein/user and
docs/ein/ein.rs they are surface between ein reasoning kernel and pure ein theory."* It had no id and takes this
one. How it is read for the pages is `contexts.md` § 8:
the reasoning defines the negation, falsity, absence, equality, `written` and `relation` as mathematics; the
user's pages and the engine's present the same names as the fixed list of relations whose facts the kernel reads
or states, everything else being a rule of Ein.

### Q-M1f.398 — How the graph is drawn to show the global context and a fact's context

**Raised** 2026-11-17 by the user's note: *"docs/ein/reasoning/ - canonical graph representation has to somehow
show that nexted fact is in global context or/and is in local/fact context. Now it is not explicitely presented.
Maybe consider approach similar to `absent` drawing in rules."* **Not ruled.** The three ways, each drawn on one
knowledge base in which every case occurs, are
`contexts.md` § 7.2.

- **(a)** a frame: the knowledge base is a frame, and a fact is inside it exactly when it is held; the filling is
  kept. **Default.**
- **(b)** a node for the global context, with an arrow to each held fact.
- **(c)** two marks on the node: filled for held, a second outline for in a context.

A box for each context, the nested fact drawn inside it, is ruled out by the same note: a fact in two contexts
would be drawn twice.

**Ruled 2026-11-17: (a), to try.** The user: *"lets try (a) and see how it looks"*. Tried on six of the figures
the explanation needs, `contexts.md` § 14: it reads well;
the pinning of rows in the pages' sources has to go; the extent's dashed frame cannot stand beside it, and the
trial draws the extent as its first arrows; frames nest where a figure already frames a rule and a knowledge base.

**Re-ruled 2026-11-17.** The user, on the first trial: *"frame-*.svg - I would like to see frame around facts in
contexts rather than around global facts."* So the frame is around the facts that are in a context, and *held*
keeps its filling: two marks for two properties. Tried again,
`contexts.md` § 14: the three cases are told apart at
a glance, and a figure with no fact under an arrow does not change.

**The label, 2026-11-17.** The user, on the second trial: *"label contexts `<relation name> cts` instead of `in
context`"*. A frame is labelled with the relation of the facts that point at what is in it, written `think ctx`.
Two readings taken with it, both forced by *every object drawn once*: a fact pointed at by facts of two relations
is in one frame that names both, and frames are never one inside another.

**Re-ruled 2026-11-18.** The user, verbatim:

> draw nested facts that were wrapped and still present also in global context
>
> structurally we see clearly nexted facts, and see what context they belong to, even without frames
>
> but the only thing we don't see on rendering is whether this fact is also in global context
>
> let's draw a frame around objects that are in context only

So **a frame is around the objects that are in a context only**: pointed at, and not held. A held object is in no
frame, whatever points at it. Which contexts it is in is read off the arrows that enter it, as before.

**Tried on the three multi-context figures** of P1f.22's `diagrams/`, as
`frame-ctx-only-*.{dot,svg}`. Each is `ein render kb`'s output with every held octagon moved out of its frame and a
frame left empty dropped:
- *held-in-two-contexts*: `(owns Bob Dog)`, held and under `think` and `says`, stands in no frame, and the figure
  has none.
- *two-contexts-of-one-relation*: `(owns Bob Dog)`, held and thought by Alice and Dan, is outside; `(owns Bob Cat)`,
  only thought, is alone in `think ctx`.
- *two-contexts*: `(= 5 (+ 2 2))`, held and under `think` and `says`, is outside; `(+ 2 2)`, never held, is in
  `= ctx`.

**The filling, ruled 2026-11-18.** For a fact that is pointed at, *in no frame* and *held* are now one property, so
the filling would be a second mark of it. The user: *"drop the filling"*. No octagon is filled; *held* is *outside
every frame*. The trial figures are drawn so.

**To build.** It is its own change on master, after the `is-a` row's change now on master. That one shares the
main tree, and a gate builds the whole tree. It touches:
- the KB view's and the program view's frames and filling, and the rule view's, whose pattern that is *also a
  premise* (or a literal of the conclusion) leaves its frame and loses its filling;
- `12-rendering.md`'s frame rule and its two context drawings;
- Q-M1f.398's decision file;
- the standard library's rule drawings, through their generator, and the shape golden.

It writes no answer. Because the DOT writers change, `utils/ci_container.sh` is part of done.

**Built by S1f.22.7**, `58b91dc3`.

### Q-M1f.399 — The names of what controls the search: `false` and `open`

**Raised** 2026-11-17 by the user, ruling P1f.22's questions: *"This also leads
me to the surface between ein theory and reasoning kernel - things like (false) and (open) are effectively
controlling the search. Maybe word `false` is too heavey loaded with classical binary logic and not expose it's
actual function in ein? Maybe (abort) or (dead) is more clear? Or some common search control interface accassible
from ein? Like (ein.search.state.refute)?"* **Not ruled, and no default taken.** It is
[P1f.21](p1f.21_verdicts_as_rules/README.md)'s question, the kernel's verdicts and how a program sees them, and is
entered there as its [V-5](p1f.21_verdicts_as_rules/findings.md#v-5--the-users-notes-from-p1f22-questions-as-facts-the-two-negation-rules-and-the-names).
The options, with what each reads like held and under an arrow, are
P1f.22's `contexts.md` § 12, copied to [P1f.21's `open_facts.md` § 10.7](p1f.21_verdicts_as_rules/open_facts.md#107-the-kernels-surface-false-and-open-control-the-search):

- **(a)** keep `false`; the pages say once that it is no truth value and what the kernel does with it.
- **(b)** a word for the proposition: `(contradiction)`, `(impossible)` or `(absurd)`.
- **(c)** a word for the search's act: `(dead)`, `(abort)`, `(refute)`.
- **(d)** the kernel's surface as a named interface, every relation the kernel reads or states under one prefix.

`(false)` also stands under an arrow, `(think Ann (false))`, where it is a proposition and controls nothing.

**The user's direction, 2026-11-17**, verbatim: *"Renaming (false): rules as before can infer (false) in contexts,
also can infer (false) in global context, but the state considered dead only if there is a rule that infer
(ein.contradiction) from global (false), or from global (whatever); both (false) renaming and (open) facts belongs
to p1f.21"*. Read: `false` is not renamed and stops being the kernel's; the kernel's verdict is a fact of its own,
under the kernel's prefix, and what makes a state dead is a rule a theory states. Not built, and not this phase's.

**The user's second direction, 2026-11-18**, verbatim in [P1f.21 § The second instruction](p1f.21_verdicts_as_rules/README.md#the-second-instruction): *"(false) controls not search but
saturation, right? if we have dead node then somebody has to kill it, maybe (ein.kill) or (ein.die) or
(ein.saturation.kill) ?"*. Read: the kernel's fact is a verb a rule concludes from a global `(false)`, and
`(false)` is a proposition; `(ein.kill)` is the leaning
([`open_facts.md` § 2 and § 3](p1f.21_verdicts_as_rules/open_facts.md#3-who-kills-a-dead-node-einkill)). S1f.21.4
puts it on programs, with the reserved names that are left (§ 8 there).

### Q-M1f.400 — How a saved file states the choices made

**Raised** 2026-11-18 by S1f.22.5's first task, on
[Q-M1f.392](#q-m1f392--a-state-file-that-reads-back-dead-through-written)'s ruling (*a saved solution is the program
unchanged with the choices made, the state derived again on load*). **Put, not ruled.** The files and the runs are
`s1f.22.5_the_format.md`; the measurement is its
§ 2.

No form states a choice today: a state file writes each committed candidate as a typed fact, `; hypothesis 0` in a
comment (the `0` is the same in all 157 907 such lines, N-29; L-104 of [the milestone's findings](findings.md) now). Measured over 1 267 state files of the corpus, the
choices typed and saturated with the program read each as given (2 303 facts, 826 states); where a choice has a
fact inside it, it is `written`, and the program of
§ 3.2
loads dead; saturated at once with the program, the choices move facts in 6 programs (each with an `absent`) and a
derived fact's first rule in 8. So a choice is not text of the program: it is committed after root's fixpoint, as
a node of the search commits it.

- **(a)** **a form of its own, `(commit F …)`**, after the program and its modules: each `F` a candidate of H,
  written in the program's spelling, committed after root's fixpoint as a node commits it, its origin *hypothesis*,
  no `written` stated for what it has inside it. A root's file has none. **Default.**
- **(b)** each chosen fact written with a mark, `(note (bad a) :hypothesis)`, after the program; the loader holds a
  marked fact apart from the program's and commits it after root's fixpoint.
- **(c)** the choices by their places in H, `(commit 0 3)`: short, unreadable, and moved by any change to H's
  order.

And three points that go with any of them, each with a default:

1. **A choice that is no candidate of H on load** (a file edited by hand, or a binary whose generator builds
   another H): **refused**, naming the fact, since the file is then no node of its program (default); or committed
   with a `note:`.
2. **An owing node's file and a dead node's**: the same form; the load computes the state and the read-out says
   what it is; nothing in the file marks it (default). `--dump-answer` writes no dead node's file, as today.
3. **Which commitment of a state reached by several**: the one the state was read from, today's `from:` (default),
   which costs no work: then one state can have two files, as zebra's has under describe and under `--maximal`
   (48 of 588 `--maximal` files read from another commitment than describe's). Or the class's first minimal
   generator in H's order, one file a state, at the cost of finding it where a question does not.

**Ruled 2026-11-18: (a).** The user: *"Q-M1f.400 (a), Q-M1f.401 (a), Q-M1f.402 (C), Q-M1f.403 (a)"*. A saved file states the choices made as one `(commit F …)` form after
the program and its modules, each `F` a candidate of H, committed after root's fixpoint as a node commits it, with no
`written` for what it has inside it. The three points under it take their defaults, which the user did not revise: a
choice that is no candidate of H on load is refused; an owing node's file and a dead node's take the same form; the
commitment written is the one the state was read from. Built by
S1f.22.5's tasks 2 to 4.

### Q-M1f.401 — How the program and its modules are in one file, unchanged

**Raised** 2026-11-18 by S1f.22.5's first task, on
[Q-M1f.392](#q-m1f392--a-state-file-that-reads-back-dead-through-written)'s rulings that the program is unchanged
and that the file *"loads with no other file beside it"* (*"yes"*). **Put, not ruled.**
`s1f.22.5_the_format.md` § 3.3, § 3.5 and § 3.6 show it on a program
that imports the standard library and on one with a local module.

Today the program is inlined and printed again: its comments and layout are gone, and its imports are replaced by
the modules' forms. Measured, on 480 programs, the inlined program loads to the same root, fact for fact and origin
for origin; it loses the text and nothing of the knowledge base. The two rulings pull apart where a program imports:
the import lines as written need the modules beside the file.

- **(a)** **each text in a wrapper of its own**: `(program …)` around the program file's bytes, `(module NAME …)`
  around each module's, as the run read them, the choices after them. The loader reads each text as the file it
  was, its positions its own, and resolves every import against the modules in the file and never the disk: **a
  saved file reads no other file.** **Default.**
- **(b)** the program's bytes first, unwrapped, then the modules and the choices: the file's lines are the
  program's lines, and where the program ends has to be marked.
- **(c)** a bundle: each text after a comment line that names it and gives its length in bytes. No form is added to
  the language, and the loader reads comments.
- **(d)** the local modules carried, the standard library named by its manifest's hash and read from the binary's
  own copy, the load refused where the hash differs: smaller, and a file saved by one binary does not load in a
  binary whose standard library moved.

A module's text as a string is out: a string is natural-language text, only after `:why`, `:source` or
`:goal-text`. Inlining is out: it is today's, and changes the program.

And, under any of them, each with a default:

1. **Which bytes**: the program file's, and each module's as the run read it, from `$EIN_STDLIB`, the checkout or
   the binary's embedded copy, whichever the resolution chose. A state saved from a `.einb` carries the program the
   container stores, import-flattened, and no module, since the container keeps no source (default).
2. **A carried standard-library module that differs from the one the binary would read**: the carried text is
   loaded and a `note:` names the module (default); or loaded with nothing said; or refused.
3. **Which modules**: every module the load reads, each once, under the name the resolver gives it (`std.x.y`, or
   a path from the program's directory, a dot a directory), a module's own imports among them; in the file sorted by
   name, and read on load in the order the imports reach them, as from the disk, since the order facts are loaded
   in is the agenda's order (default).
4. **The size**, measured over the 1 267 files: 123.1 MB today, 32.2 MB proposed; the proposed file is smaller for
   527 and larger for 740, those a small state of a program that imports a long module
   (`tests/region/04_owing.ein`: 9 684 B today, 29 164 B proposed).

**Ruled 2026-11-18: (a).** The user: *"Q-M1f.400 (a), Q-M1f.401 (a), Q-M1f.402 (C), Q-M1f.403 (a)"*. `(program …)` around the program file's bytes and `(module NAME …)`
around each module's, as the run read them; imports resolved against the modules in the file and never the disk, so
a saved file reads no other file. The four points under it take their defaults: the bytes the resolution chose (a
`.einb`'s save carries the flattened program it stores); a carried standard-library module that differs from the
binary's is loaded with a `note:`; every module the load reads, once, sorted by name in the file and read in the
order the imports reach them. Built by S1f.22.5's tasks 2 to 4.

**Point 2 amended 2026-11-18** by the user's ruling on [F23](../followups/f23_diagnostics_in_eins_words.md)'s D-24,
in the review bundle's `01-save-and-load/ANSWERS.md`, as written (`35719115`): *"Drop the note entirely, since the
file is self-contained and comparing libraries isn't the loader's business"*. A carried module is loaded as carried
and nothing is compared or said; the note, its code (`ein_ir::saved`'s comparison, `Resolver::stdlib_text`), its test
and its pages went in P1f.23's S1f.23.8 (`86d46410`), and the decision
[Q-M1f.401](../../docs/decisions/read-out/Q-M1f.401-a-saved-file-carries-its-program-and-modules.md) says so.

### Q-M1f.402 — What save and load are as commands, the dump they are held to, and a file's first lines

**Raised** 2026-11-18 by S1f.22.5's first task, on
[Q-M1f.392](#q-m1f392--a-state-file-that-reads-back-dead-through-written)'s requirement, verbatim: *"save and load
must in both directions must have identical results: save -> load -> save byte identical, load -> save -> load -
have identical dumps; both save and load must not modify the program, must not add or remove any bit of
information"*. **Put, not ruled.** The commands, run today on every program of
`s1f.22.5_the_format.md`, are in each folder's `commands.txt`.

Measured today, over 1 267 state files: a second save is byte-identical for **0**, and 2.9 times as long; the two
loads' dumps are identical for 1 265, so today's file passes the second invariance and fails at the step before
both, **the first load is not the state saved**. The writer's state is a function of the program and the
commitment, byte for byte, in all 28 311 files a second run and `-j 4` write; the program and the choices
saturated together are not (6 programs move facts, 8 a first rule). So the load is the node's computation, and the
dump can promise every fact, `written` among them, its origin, its first rule, and the order.

- **The commands.** **(a)** **no new command** (default): *save* is `ein solve FILE --dump-answer DIR`, each state
  of the part of R asked as `models/NN.ein` and the owing frontier as `owing/NN.ein`, and `--no-search` saves root
  as `DIR/root.ein`; *load* is any command given a saved file. The first invariance is `ein solve F --no-search
  --dump-answer D` then `cmp F D/root.ein`. **(b)** a new `ein show state FILE`, the saved file of FILE's root on
  stdout, as `ein show` prints Ein text a file could hold.
- **The dump the second invariance compares.** **(a)** **`ein saturate --dump`'s KB section** (default): every fact
  held, `written` included, by origin, with its first rule or `:hypothesis`, in the order held, byte for byte.
  **(b)** the facts by value, origins aside, as `ein-cli/tests/answer_round_trip.rs` compares today: it passes the
  two corpus programs that load dead (N-30, N-31). **(c)** `answer.json` and the file: they say R, not the facts'
  origins.
- **A save of a loaded file.** **(A)** **a loaded file is a program whose root is its state** (default): every
  command reads it as today, `ein solve F` describes the answer above that state, and a state found above it is
  saved as F with a second `(commit …)`, a node of a node, so the file keeps the order the choices were made in;
  `--no-search --dump-answer` writes F again. **(C)** a saved file is a node of its program's lattice: `ein solve
  F` describes the part of the program's R above the node, and every file is the program with one commitment of H.
- **A file's first lines.** **(a)** **one fixed line** naming the format and saying the file reads no other
  (default); what today's header says of the run (the state's number, its lines, how many commitments reach it, its
  minimal generators, the path read from) is not reproduced by a second save, and moves to `index.json`, which has
  `state`, `lines`, `listed` and `source` now. **(b)** none. **(c)** today's, which no second save reproduces.

**Ruled 2026-11-18: (C) for a save of a loaded file; the defaults for the rest.** The user: *"Q-M1f.400 (a), Q-M1f.401 (a), Q-M1f.402 (C), Q-M1f.403 (a)"*. **A saved file
is a node of its program's lattice**: `ein solve F` describes the part of the program's R above the node, H is the
program's, and every saved file is the program with one commitment of H, so a state found above a saved file is saved
as the program with that state's commitment, in one `(commit …)`. The default (A), a loaded file read as a program
whose root is its state, is out. The other points take their defaults, which the user did not revise: no new command
(save is `--dump-answer`; `ein solve F --no-search --dump-answer D` then `cmp F D/root.ein` checks the first
invariance); the dump the second compares is `ein saturate --dump`'s KB section, byte for byte; one fixed first line,
and what today's header says of the run moves to `index.json`. Built by
S1f.22.5's tasks 2 to 4.

### Q-M1f.403 — The program's claims in a saved file

**Raised** 2026-11-18 by S1f.22.5's first task. **Put, not
ruled.** `s1f.22.5_the_format.md` § 4.

A saved file holds the program unchanged, its `(query … :expect …)` and `(solve … :expect …)` among it, and those
are claims about the program's R. Measured today: of 826 corpus state files with a commitment, **188, in 40
programs, exit 1** when solved, each a claim of the program checked against the file's own answer
(`examples/binary_logic/knights.ein`'s `models/01.ein`: *the claim lists 2 classes, the query has 1*); of 441 with
none, 2 exit 1 where their program exits 0, both N-30's, and 1 exits 1 as its program does.

- **(a)** **a file with a commitment checks no claim**: its read-out names them as the program's, NOT CHECKED, and
  it exits 0; a root's file checks them as the program does, since it loads to the program's root. **Default.**
- **(b)** as today: checked against the loaded file's answer, exit 1 where a claim is about the program's R.
- **(c)** checked against the program's R, by solving the program again from the file's text.

**Ruled 2026-11-18: (a).** The user: *"Q-M1f.400 (a), Q-M1f.401 (a), Q-M1f.402 (C), Q-M1f.403 (a)"*. A saved file with a commitment checks none of the program's claims: its
read-out names them as the program's, NOT CHECKED, and it exits 0; a root's file checks them as the program does.
Built by S1f.22.5's tasks 2 to 4.

### Q-M1f.404 — `written` is erased; a numeral is checked by the operation that takes it

**Ruled 2026-11-18** by the user, verbatim, after asking of `written` *"WTF is written ctx???"* and *"When it was
added and what for?"*, and being told it was Q-M1f.272's loader step (E1), made for Q-M1f.210: *"revert this
decision, completely erase this `written` crutch. insert malformed numbers detection into decimal and binary
arithmetic operations. all numbers taken from input program will be checked once and labeled valid, and numbres
produced by operations has to be labelled valid already. put current running stage into branch and pause its work,
first remove `written` and then again draw multi-context examples and continue p1f.22.5"*.

Reverted: Q-M1f.272, Q-M1f.275, Q-M1f.279 and Q-M1f.280 whole. From Q-M1f.210 and Q-M1f.276, the half that says a
malformed number is flagged where it is written, with no calculation reading it. The loader states no `written`,
and `written` is no reserved name. A numeral an operation takes from the program is checked once and labelled
valid, or flagged. A numeral an operation produces is labelled valid as it is produced. Built by
S1f.22.6, whose readings of the ruling are put as Q-M1f.405
onward, each with a default; the user ruled them 2026-11-18, and S1f.23.9 built the rulings (Q-M1f.405, .408,
.409 and the milestone's L-101), the decision's file stating them.

### Q-M1f.405 — Which numerals are "taken from the input program", and what "once" is

**Raised** 2026-11-18 by S1f.22.6, reading 1 of
[Q-M1f.404](#q-m1f404--written-is-erased-a-numeral-is-checked-by-the-operation-that-takes-it). **Ruled 2026-11-18.**
The stage's Record shows it on real programs.

- **(a)** **a numeral an operation takes**: the numeral an operand stands for, written as the operand or as a
  `let`'s value (`std.math.positional`'s `operand-check`), a part of an integer or a rational an operation takes
  (`int-check`, `rat-check`), and the number `to-bin`, `to-dec` and `to9` take. Whoever wrote the request, the file
  or a rule of the program's own. **Once** is a fact of the numeral: the ask `(check N N)` and the label
  `(valid N)`, so a numeral is walked once whatever reads it, however many requests name it. **Default.**
- **(b)** only the numerals of requests the file writes, not of one a rule derives: needs a statement of what the
  file wrote, the context the ruling erased.
- **(c)** every numeral the knowledge base holds, a `let` nobody asks about and a fact of the program's own among
  them: needs a walk of every fact for its nested terms, the loader's step again, or a rule over every relation.
- **(d)** once a request: the check keyed by the operand's tag, walked again for every request that names the
  numeral.
- **(e)** as (a), and every tagged call asking the check of the numbers it is handed, `(add-walk TAG X Y 0 z)` and
  the rest: a program's own theory that hands one a malformed numeral is flagged. Under (a) it is not: a call looks
  nothing up, the walk stalls, no answer, root alive, where master flagged the numeral where it was written
  (`(add-walk (mine a) (n z x) (n z 1) 0 z)`: `states (k_s) 1`, master `0`). Under (e) every number the calculator
  hands itself, every intermediate of a product and a quotient, is walked again, a firing a place.

**Ruled 2026-11-18: (a), with the user's distinction.** The user, verbatim: *"(a) the difference is between
numerals produced by operation from valid numerals, and all other numerals from any source"*. A numeral an
operation makes from valid numerals is valid as made and is not walked again; every other numeral is checked,
whatever its source: one a request takes, written or made up by a rule of the program's own, and one handed
straight to a tagged call, so the (e) case, `(add-walk (mine a) (n z x) (n z 1) 0 z)`, is dead again. *Once* stays
a fact of the numeral. Built by [S1f.23.9](p1f.23_review_processing/s1f.23.9_numerals_answers.md), within the calculator's laws: each tagged call a program or a module over the
calculator writes, the five of each base, `std.math.int`'s four, `std.math.rat`'s three and `std.math.gcd`'s, asks
the check of what it is handed and hands itself on to a walk of the calculator's own (`add-run` and the rest),
which checks nothing and which the calculator calls itself with numbers it checked or made; what is checked is told
by the relation it is written in, and no rule joins two facts on a number. The decision Q-M1f.404 states it.

### Q-M1f.406 — The label, and where a number an operation makes gets it

**Raised** 2026-11-18 by S1f.22.6, reading 2. **Ruled 2026-11-18.**

- **(a)** **`(valid N)`, `std.math.decimal`'s, and `(bin-valid N)`, `std.math.binary`'s, a fact of the numeral**:
  the walk's end for a numeral an operation takes; for a number the arithmetic makes, a conclusion of the firing
  that makes it, `std.math.positional`'s `turn-end` (every walk's hand-over, a tagged call's included) and
  `div-end` (a quotient and its remainder); `z` by the representation's closing rule. Every number an operation
  answers with, in every module over the bases, is so valid already. **Default.**
- **(b)** the public answers alone, `sum-of`'s S and the like, each module's answer rules labelling what they hold:
  a tagged call's hand-over unlabelled, and fourteen answer rules of `std.math.int` and `std.math.rat` to write.
- **(c)** as (a), with `z` labelled only where a walk makes it or a check reaches it.

**Ruled 2026-11-18: (a).** The user, verbatim: *"(a)"*. As built by S1f.22.6; the decision Q-M1f.404 states it.

### Q-M1f.407 — A numeral no operation takes is not checked

**Raised** 2026-11-18 by S1f.22.6, reading 3. **Ruled 2026-11-18.**
Three corpus programs answer otherwise, each dead before and R = {∅} after:
`tests/stdlib/math/decimal/02_in_a_let_nobody_asks_about.ein`, `03_in_a_fact_of_the_programs_own.ein` and
`04_in_a_rules_conclusion.ein`. Binary has no such program.

- **(a)** **not checked**: a numeral in a `let` nobody asks about, in a fact of the program's own relation, in a
  rule's conclusion that never fires. The three programs are rewritten to claim R = {∅}, no flag and no label,
  with a sum answered beside each, and `07_in_a_rules_conclusion_that_fires.ein` added, the rule that fires and
  hands its numeral to a request, which is flagged. **Default.**
- **(b)** the same, the three programs deleted.
- **(c)** a `let`'s value checked as the `let` is read, `let` being the calculator's relation: `02` dead again, and
  `03` and `04` alive.

**Ruled 2026-11-18: (a).** The user, verbatim: *"(a)"*. As built by S1f.22.6; the decision Q-M1f.404 states it.

### Q-M1f.408 — Where the check's rules live

**Raised** 2026-11-18 by S1f.22.6, reading 4. **Ruled 2026-11-18.**

- **(a)** **the representation's**: `std.math.decimal` declares `check` and `valid` and holds the walk
  (`numeral-step`, `numeral-step-zero`, `numeral-valid`) and its four flags; `std.math.binary` the same under
  `bin-`. The operations ask: `std.math.positional`'s `operand-check`, one rule activated for each base with its
  `?CHECK`, and `std.math.convert`'s and `std.math.num9`'s requests. The label at a walk's end is written by
  `std.math.positional` with the base's `?VALID`. **Default.**
- **(b)** the walk in `std.math.positional`, parameterised by the base's digits, the representation keeping only
  the flags.
- **(c)** each operation checking its own operands, the representation keeping nothing.

**Ruled 2026-11-18: (b).** The user, verbatim: *"(b)"*: the walk in `std.math.positional`, the representation
keeping the flags. Built by [S1f.23.9](p1f.23_review_processing/s1f.23.9_numerals_answers.md): `numeral-step`, `numeral-step-zero` and `numeral-valid` moved into
`std.math.positional`, `(?CHECK ?N ?NZ)`, `(?CHECK ?N)` and `(?CHECK ?VALID)`, activated by each base's call of the
macro; `std.math.decimal` and `std.math.binary` keep `check`, `valid`, the flags and `not-decimal-is-false`, whose
rules read the walk. The decision Q-M1f.404 states it.

### Q-M1f.409 — An integer and a rational: their check and their label

**Raised** 2026-11-18 by S1f.22.6, reading 5. **Ruled 2026-11-18.**

- **(a)** **`(int-check I)` and `(rat-check Q)`, asked where an operand's look-up finds one** (`int-operand-check`,
  `rat-check-ask`), read by the six shape rules of each, which flag a digit or a named constant where a part goes as
  before; `int-check-parts` asks `std.math.decimal`'s check of each part, `rat-check-parts` `std.math.int`'s of the
  numerator and `std.math.decimal`'s of the denominator. **No label of their own**: an integer's and a rational's
  numerals carry `valid`, the parts of one an operation takes by their walks and those of one an operation makes as
  the walks that make them end. **Default.**
- **(b)** a label of their own, `(int-valid I)`, by a walk that carries the integer through both parts in turn,
  since joining its parts' labels would join two facts on a number: a second walk of each part, once an integer
  rather than once a numeral.
- **(c)** the shapes left unchecked, only the numerals: `(int 7 z)` answered by silence.
- **(d)** as (a), and `rat-check-parts` asking `std.math.decimal`'s check of a numerator that is a natural number.
  Under (a) a rational whose numerator is a malformed natural, `(rat-add (rat (n z x) (n z 2)) Q)`, is answered
  by silence and not flagged (`states (k_s) 1`; master `0`), since the numerator is no integer and the operation
  computes nothing with it ([Q-M1f.277](../../docs/decisions/stdlib/math/Q-M1f.277-an-operand-of-another-kind-of-number-is-answered.md));
  where an integer goes, a natural number is flagged, `(int-add (n z x) I)` dead as on master, since
  `std.math.decimal-arith`'s look-up finds it and checks it. No corpus program writes either.

**Ruled 2026-11-18: (a)**, and the natural part. The user, verbatim: *"(a)"*; and, asked how L-101's rule
reads a rational whose part is a natural numeral, *"Verify the natural"*: a rational built from a natural numeral
checks it as a numeral, as an operation checks its operands, so `(rat (n z x) (n z 2))` is dead again (the (d)
case). Built by [S1f.23.9](p1f.23_review_processing/s1f.23.9_numerals_answers.md) with [L-101](findings.md#the-standard-librarys-rules): `rat-check-z-natural`; no label of
their own. The decision Q-M1f.404 states it.

### Q-M1f.410 — No join on a number, and a result a program's rule hands on

**Raised** 2026-11-18 by S1f.22.6, reading 6. **Ruled 2026-11-18.**
The rule of the calculator ([Q-M1f.362](../../docs/decisions/stdlib/math/Q-M1f.362-no-rule-of-the-calculator-joins-two-facts-on.md))
is kept: the walk carries the numeral it checks, so its end labels it with no second premise, and no operation
waits for the label. One consequence: a number an operation made, which a rule of the program's own hands to a
second request, is that request's operand and is walked again, deriving the label it has
(`tests/stdlib/math/decimal-arith/19_div_named_and_chained.ein`, 1500 walked in five firings). Nothing positive
tells it from a numeral the file wrote, and only a negation or a join could.

- **(a)** **walked again**, a firing a place, its label already held. **Default.**
- **(b)** the operations wait for the label, a probe `(valid ?v)` beside the operand: allowed by the static rule,
  and it walks the made number again all the same.
- **(c)** every prefix of a number labelled as the turn makes it, so that the ask of a made number finds its walk's
  facts already held: no second walk, and a fact a digit more for every number any walk makes, the intermediate
  ones included.

**Ruled 2026-11-18: (a).** The user, verbatim: *"(a)"*. As built by S1f.22.6; under Q-M1f.405's ruling a made number a
program's rule hands to a tagged call is walked again too, as one it hands to a request. The decision Q-M1f.404
states it.

### Q-M1f.411 — A numeral with two faults has one flag

**Raised** 2026-11-18 by S1f.22.6, found building the walk.
**Ruled 2026-11-18.** The loader's statement read every place of a numeral at once, so `(n (n (n z x) y) 5)` was
flagged twice, at `(n z x)` and at `(n (n z x) y)`. The walk stops at the first fault it meets from the units.

- **(a)** **one flag, the first place at fault from the units**
  ([Q-M1f.274](../../docs/decisions/stdlib/math/decimal/Q-M1f.274-a-malformed-numeral-is-flagged-on-its-smallest.md)
  re-worded so): the program is dead either way, and no corpus program writes such a numeral. **Default.**
- **(b)** every place at fault flagged: the walk carries on past a fault, which needs a second walk relation or a
  fault bit carried, so that the label is not derived for a malformed numeral.

**Ruled 2026-11-18: (a).** The user, verbatim: *"(a)"*. As built by S1f.22.6; the decision Q-M1f.274 states it.

### Q-M1f.412 — The read-out's `is-a` row

**Raised** 2026-11-18 by the user, P1f.21's
[V-6](p1f.21_verdicts_as_rules/findings.md#v-6--the-read-out-reads-is-a-by-its-name): *"is-a is just some random
relation, or maybe not even a relation; what for is it read by kernel read-out?"* The read-out's `types` row named
the atoms on the right of an `is-a` that the blind enumerator's candidates have as arguments, reading the relation
by its spelling, and decided nothing.

**Ruled 2026-11-18**, the user verbatim: *"V-6 (a), drop the is-a row"*. Built in `5a3f2979`: the row on every
question, `region.md`'s line, the key `types_as_objects` in `region.json`, `--json-summary`'s `root.hypgen` and
`ein test --json-report`'s rows, `ein test`'s line beside a claim, and `ein_infer::types_as_objects`. Q-M1e.12's
second half goes with them: its file states the first half alone, *the blind enumerator is untyped*, and carries the
ruling's reason in its **Why**, so this id has no decision file of its own.

One reading taken with it, **put, not ruled**:

- **(a)** **no schema id moves**: `ein-summary/2`, `ein-region/1` and `ein-test-report/5` lose the key and keep
  their ids, as [Q-M1f.259](../../docs/decisions/read-out/Q-M1f.259-under-no-search-the-summary-states-no-region.md)
  keeps an id where keys are added and no other key moves. **Default.**
- **(b)** `ein-test-report/6`, as the report's id moved on each change of its row before; the summary and the
  region dump keep theirs.

**Put to the user in the review bundle** (`04-language-and-decisions/16-Q-M1f.412/`), who answered with a
question, 2026-11-19, verbatim (`b0e9c412`): *"How deep does this `is-a` go into the kernel? I repeat again and again
THERE IS NO TYPES in ein. It's substrate, field, data plane is a pure labelled graph. Does graph nodes have types?
NO! Graphs are about structure. If we want - we add types-like behaviour to the program on relations and rules
level, usually (Usually! not always) following stdlib convention. If we don't need - no types."*

**Answered** (the main session, 2026-11-19): nowhere. No code of the six shipping crates reads `is-a` by its
spelling: of the 45 lines of their `src/` that write it, every one outside a comment is the text of a unit test's own
program, which declares `is-a` as an ordinary relation (`scope.rs`, `relation_names.rs`, `oblgen.rs`, `from_ir.rs`
under `#[cfg(test)]`), and the comments say it is not read (`hypgen.rs`: *"reaches for `is-a` here would be a type
system in the kernel"*). `ein-render/tests/renaming_invariance.rs` holds every corpus program to its answer with
every name but the kernel's renamed, `is-a` among them, and its `READS` is empty
([Q-M1f.436](../../docs/decisions/engine/Q-M1f.436-the-kernel-reads-no-theorys-name.md)). `is-a` is a convention of
five stdlib modules (`std.typing`, `std.algebra`, `std.bijection`, `std.elim`, `std.bool.inputs`) and of the corpus's
programs. One stale comment says otherwise, `from_ir.rs`' above `program_relations` (*"one it declares or states a
fact of, `is-a`, …"*), and is re-worded by [S1f.23.10](p1f.23_review_processing/s1f.23.10_language_answers.md), task 1. The reading's default, (a), stands: the user picked
none.

### Q-M1f.413 — `ein kb save` of a saved file

**Raised** 2026-11-18 by S1f.22.5's build. **Put, not ruled.** A
`.einb` stores a program, resolved and import-flattened, and its root's sections; a saved file is a program with one
commitment of H (Q-M1f.402), and a container has no place for the commitment. Resolving the saved file's imports
from the disk, as the container's writer did, would read other files than the saved one.

- **(a)** **a saved file of root, with no `(commit …)`, is stored as the program it carries, its imports read from
  the modules it carries; one with a `(commit …)` is refused**, `kb save error: F is a saved file with a (commit …),
  a node of its program's lattice, and a .einb holds a program's root — save the program, or the saved file of its
  root`, exit 1. **Default.**
- **(b)** both refused: a container is made from a program file.
- **(c)** the container carries the commitment, a section of its own, and a container of a node opens as that node.

**Default taken: (a).** Built: `ein-cli/src/kb.rs`, `ein_einb::sections::Carried`; held by
`ein-cli/tests/saved_files.rs`' `what_a_saved_file_is_refused_for`; the page `10-answer-dump.md` and
`07-diagnostics.md` § 2.30.

**Ruled 2026-11-18: (c).** The user, in the review bundle's `01-save-and-load/ANSWERS.md`, as written (`35719115`): *"(c)"*, the option
*give the container a section for the commitment, so a node's container opens as that node*. **Built** by P1f.23's
[S1f.23.8](p1f.23_review_processing/s1f.23.8_save_and_load_answers.md) (`86d46410`): `ein kb save` of a saved file
stores the program it carries, its imports from the modules it carries, and a node's `(commit …)` in the section
`COMMIT`, kind 13, which an open sets as the stored program's commitment; `ein solve x.einb` of a node's container
prints what `ein solve` of the saved file prints, byte for byte but the path, held by `einb_cli.rs`'
`a_nodes_container_opens_as_the_node` and, over every corpus program's last saved node, `answer_round_trip.rs`'
`by_its_container`. A root's container holds no `COMMIT`. **The format is 2.1**: a section kind added moves the
minor (`09-binary-format.md` § 4.1, Q-M1f.10's *why*, `corruption.rs`' `an_unknown_section_kind_is_ignored_rather_than_refused`);
the phase's R-30, a container 24 bytes shorter at `einb/2.0`, is what `SYMBOLS` holds and not its layout, and moves
nothing. The decision [Q-M1f.413](../../docs/decisions/engine/Q-M1f.413-a-nodes-container-opens-as-the-node.md);
the readings, each with its default, are the stage's Record.

### Q-M1f.414 — A fact's English, which a state file wrote in a comment

**Raised** 2026-11-18 by S1f.22.5's build. **Put, not ruled.** A
state file wrote every fact of the state with its reason and its statement in a comment
([Q-M1f.58](../../docs/decisions/read-out/Q-M1f.58-a-fact-reads-in-english-in-the-trace.md) as it stood). The ruled
format, Q-M1f.400 and Q-M1f.401, holds no fact of the state, so no file holds a state's facts in English: `--trace`
writes a firing's reason and each fact it derives with its statement, and `ein render kb` labels a fact's arrow with
its source or its rule.

- **(a)** **the English goes with the facts**: the trace is where a fact reads in English, Q-M1f.58 says so, and
  `14-natural-language.md` § 3 shows a trace's lines instead of a state file's. **Default.**
- **(b)** `ein saturate --dump` writes each fact's English in a comment after it, as the state file did.
- **(c)** a listing of a state, `-p`'s, writes each fact's English.

**Default taken: (a).** Built: the page, the decision, `ein-cli/tests/natural_language_page.rs`'
`the_pages_trace_lines_are_a_runs`. The code that wrote the comments (`ein_render::dump::serialise`'s
`facts_to_ein_text`, `kb_to_ein_text`) is read by no surface of the binary now (N-36; L-102 of [the milestone's findings](findings.md) now).

**Resolved 2026-11-18** by S1f.23.5's rulings
([its stage file's Record](p1f.23_review_processing/s1f.23.5_a_why_reads_its_own_form.md#record)): a `:why` reads only
the form it is written on, and facts stay as they are (*"leave facts as they are"*), so a fact has no English of its
own and no state file has a fact's English to write; (a) stands. L-102 and L-105 follow from it: the writer goes and
`utils/nl_census.py`'s answer half is retired, both in S1f.23.5's build.

### Q-M1f.415 — `ein show hyps` and `ein render h-space` of a saved file

**Raised** 2026-11-18 by S1f.22.5's build. **Put, not ruled.**
Q-M1f.402 (C): *H is the program's*; a run of a saved file is over H less the node's choices.

- **(a)** **both read the program's H, the node aside**: `ein show hyps F` lists every candidate of H, the node's
  choices among them, and `ein render h-space F` draws the program's structure. **Default.**
- **(b)** both read the run's universe, H less the choices, as `ein solve -H` lists it.

**Default taken: (a).** Built: `ein-cli/src/common.rs`' `program_of_saved`; held by `saved_files.rs`'
`a_choice_is_saved_as_a_commitment_and_loaded_as_the_node`.

**Ruled 2026-11-18: (a).** The user, in the review bundle's `01-save-and-load/ANSWERS.md`, as written (`35719115`): *"(a)"*, the default
as built; [Q-M1f.402](../../docs/decisions/answer/Q-M1f.402-a-saved-file-is-a-node-of-its-programs-lattice.md)
states it. A node's container reads the same, its commitment set aside by the same function (S1f.23.8).

### Q-M1f.416 — What `ein test` asks of a saved file's node, and how its claims exit

**Raised** 2026-11-18 by S1f.22.5's build, on Q-M1f.403 (a)
(*a file with a commitment checks no claim … and it exits 0*). `ein test` asks a file the least question that
decides its claims, and a claim not checked exits 1 there but under `--no-search`.

- **(a)** **root alone, the node**: no question about the node decides a claim of its program, so the least is
  root's; each claim is NOT CHECKED with Q-M1f.403's line, and none moves the exit code, as under `--no-search`; the
  verbose header says *(a saved file's node: root alone)*. **Default.**
- **(b)** no run at all: each claim NOT CHECKED, no row of a run in `--json-report`.

**Default taken: (a).** Built: `ein-cli/src/test.rs`; held by `saved_files.rs` and `test_cli.rs`'
`the_verbose_header_and_the_report_row_agree_field_for_field` on `tests/saved/04_a_saved_node.ein`.

**Ruled 2026-11-18: (b).** The user, in the review bundle's `01-save-and-load/ANSWERS.md`, as written (`35719115`): *"(b)"*, the option
*no run at all: each claim NOT CHECKED, no run's data in `--json-report`*. **Built** by P1f.23's
[S1f.23.8](p1f.23_review_processing/s1f.23.8_save_and_load_answers.md) (`86d46410`): `ein test` of a file with a
`(commit …)`, or of a node's container, runs nothing, not root's saturation either (`ein-cli/src/test.rs`' `NotRun`);
each claim is NOT CHECKED with Q-M1f.403's line and counted not checked with no exit code; the `-v` header names no
question; the report's row has `answer`, `outside`, `route`, `fell_back_from` and `diff` `null` and a solve row no
`region`, as on any row no run checked, so no key moves and the id stays `ein-test-report/6`; `--events` and
`--json-summary` write nothing for the file, as for a file that states no claim. Held by `saved_files.rs`'
`a_choice_is_saved_as_a_commitment_and_loaded_as_the_node` and `test_cli.rs`' header and scope tests. The decision
[Q-M1f.403](../../docs/decisions/answer/Q-M1f.403-a-saved-file-with-a-commitment-checks-none.md) says it; the
readings are the stage's Record.

### Q-M1f.417 — What a run of a saved file says of its node, and the directory's schema

**Raised** 2026-11-18 by S1f.22.5's build. **Put, not ruled.**
Q-M1f.402's *what today's header says of the run moves to `index.json`*; and a run of a saved file, the part of R
above a node, has to say which node.

- **(a)** **as built**: the read-out's `commit` row under `universe`, the candidates the `(commit …)` names, with `∅`
  the node; the summary's `root.commit`, and `root.dead`, `root.owes` and `root.owed` the node's standing while
  `root.facts` stays the program root's fixpoint and `root.hypgen.emitted` the program's H; `index.json`'s per-file
  `commit` beside `region.states`' `from`, `commitments` and `minimal_generators`; the directory's schema
  `ein-answer/3`, whose writer clears an earlier schema's directory as its own. **Default.**
- **(b)** no row: the read-out of a saved file is the read-out of a program.

**Default taken: (a).** Built: `ein-render/src/read_out.rs`, `ein-cli/src/summary.rs`,
`ein-render/src/dump/region_answer.rs`; the pages `08-read-out.md` § 5.3 and its example X21, and
`10-answer-dump.md`.

**Ruled 2026-11-18: (a).** The user, in the review bundle's `01-save-and-load/ANSWERS.md`, as written (`35719115`): *"(a)"*, the default
as built. A node's container prints the same row, its read-out the saved file's (S1f.23.8).

### Q-M1f.418 — The nesting limit, and a saved file's wrappers

**Raised** 2026-11-18 by S1f.22.5's build. **Put, not ruled.**
`(program …)` and `(module …)` stand one level above the texts they carry, so a program at the parser's limit,
1 000 parentheses, is one level past it inside its saved file, which would then not load.

- **(a)** **a text that opens as a saved file is parsed with one level more**, its wrappers' own; each text it carries
  is parsed again alone under the limit, as the file it was, so the limit is the texts'. **Default.**
- **(b)** the limit stays, and a program at it has a state that cannot be saved.

**Default taken: (a).** Built: `ein_ir::grammar`'s parse; held by `ein-ir/tests/generated_parser.rs`'
`a_saved_file_carries_a_program_at_the_limit`; the decision
[Q-M1f.185](../../docs/decisions/engine/Q-M1f.185-a-state-deeper-than-the-parsers-limit.md), rewritten: a state
deeper than the limit is answered, saved and loaded again.

**Ruled 2026-11-18: (a).** The user, in the review bundle's `01-save-and-load/ANSWERS.md`, as written (`35719115`): *"(a)"*, the default
as built; Q-M1f.185's file states it.

### Q-M1f.419 — How `ein render` draws a fact

**Raised and ruled** 2026-11-18 by the user, on reading the review bundle's drawings
(`review/03-drawings/`) and the main session's account of the four ways the tree draws a fact: (1) **canonical**, an
octagon with no name in it, its first arrow at the relation's atom, its next ones at the arguments, numbered; (2)
**labelled octagon**, the relation's name in the octagon and no node for the relation; (3) **labelled arrow**, a fact
of a relation and two arguments as one arrow labelled with the relation's name; (4) **text node**, a node labelled
with a fact's text, in the drawings of a derivation. The user, verbatim:

> Then important: I want `ein render` now draw first form by default and optionally fold 3-facts into labelled
> arrows, and not produce labelled octagons anymore. Forms 2 and 4 must not be produces or used in docs. […] Both KB
> and program view must follow the same label="owns", and number the arrows 1/2

**Ruling.** Every view draws a fact in form (1) by default; on request, a fact of three arrows, a relation and two
arguments, is folded into form (3); forms (2) and (4) are produced by no command and drawn on no page under `docs/`.
The KB and the program views write a relation's name as it is, `owns`, and number arrows with digits alone. It
answers L-99 (no text node in a derivation's drawing) and rewrites Q1, Q-M1f.382 and Q-M1f.395 where they say
otherwise. **Built by** P1f.23's
[S1f.23.1](p1f.23_review_processing/s1f.23.1_drawing.md), whose readings where the ruling is silent are put from
Q-M1f.421 on.

### Q-M1f.420 — What `ein render program` draws

**Raised and ruled** 2026-11-18 by the user, with Q-M1f.419, verbatim:

> `ein render kb` renders whole KB, `ein render program` renders 3 parts: the same as KB + rules + query; DO NOT
> split KB into ontology and facts. This split is relevant during program sythesis, not during it's
> review/analysis.

**Ruling.** `ein render kb` draws the whole knowledge base, one graph. `ein render program` draws three parts: the
program's knowledge base, drawn as the KB view draws one; its rules; its query. The digraphs `ontology` and `facts`
go. It answers L-98 (the program view is several digraphs). **Built by**
[S1f.23.1](p1f.23_review_processing/s1f.23.1_drawing.md).

### Q-M1f.421 — `is-a` and `relation` in a drawing

**Raised and ruled** 2026-11-18 by the user, during S1f.23.1, replacing the stage's reading that left `is-a`'s
shapes as they were. The user, verbatim:

> `is-a` - is just ordinary relation, no special dashed arrows, nothing special, completely ordinary relations. even
> `relation` relation is ordinary relation

**Ruling.** No drawing gives `is-a`, `relation` or any relation a look of its own: no dashed arrow with an empty
head, no box for an atom an `is-a` fact points at and no oval for one it starts from; every atom one shape. A
declaration is drawn as the fact it is, an octagon whose first arrow is at the atom `relation`, or folded where it has
three arrows, in the program view and the KB view alike. `ein render kb`'s `--no-types` and `--no-instances`, which
existed for that convention alone, go. **Built by** S1f.23.1: the decision
[Q-M1f.421](../../docs/decisions/cli/Q-M1f.421-no-view-draws-a-relation-otherwise-than-another.md); the program
view's equality node for a fact of `=` goes with it, a fact of `=` drawn as any fact. The reads of `is-a` outside the
drawings are [P1f.23's findings](p1f.23_review_processing/findings.md) R-1 to R-3, the user's to rule.

### Q-M1f.422 — The trace's drawings folded

**Raised and ruled** 2026-11-18 by the user, during S1f.23.1, verbatim:

> trace can also draw folded relations if --fold flag provided;

**Ruling.** `ein solve` takes `--fold` beside `--trace`'s other modifiers: each step's drawing and each `-F`
snapshot are drawn in the canonical form by default and folded under it, as the views of `ein render` are. **Built
by** S1f.23.1, under the decision
[Q-M1f.419](../../docs/decisions/cli/Q-M1f.419-ein-render-draws-a-fact-as-the-graph-has-it.md); how a fact a step
reads or concludes is folded is [Q-M1f.429](#q-m1f429--a-folded-fact-in-a-drawing-of-a-derivation).

### Q-M1f.423 — How the arrows of a fact are numbered

**Raised** 2026-11-18 by S1f.23.1's build, on Q-M1f.419 (*"number the arrows 1/2"*). **Ruled 2026-11-18.**

- **(a)** **as the reasoning pages number them**: the first arrow, at the relation, is `1`, and the arguments' are
  `2`, `3`, … in order (`01-canonical-graph.dot`, *counted from the first*); digits alone, in every view; a folded
  arrow carries the relation's name and no number. **Default.**
- **(b)** the arrows to the arguments `1`, `2`, … and the first arrow unnumbered, or `0`.

**Ruled 2026-11-18** by the user, on the review bundle's `03-drawings/` (item 34; the answers are `e4c0eaf6`),
verbatim: *"(a)"* **(a)**, the default as built. Recorded by S1f.23.6; carried by the decision
[Q-M1f.419](../../docs/decisions/cli/Q-M1f.419-ein-render-draws-a-fact-as-the-graph-has-it.md), which states the
numbering.

The default had been (a). Built: `ein-render/src/fact_graph.rs`, `rules.rs`, `slice.rs`, `derivation.rs`, `ir_dot.rs`.

### Q-M1f.424 — Where what a fact rests on is drawn

**Raised** 2026-11-18 by S1f.23.1's build. **Ruled 2026-11-18.** The KB view wrote a fact's source's short id or
`by <rule>` in the octagon's label, after the relation's name and a literal `\n` (L-96), or after the name on its
arrow.

- **(a)** **beside the octagon, an `xlabel`**: `(10)`, `by symmetric`; on a folded arrow after the relation's name, as
  before. A derived fact stays dashed, its octagon and its arrows; a fact a state holds beyond `--since`'s stays at
  `penwidth=3`, its octagon and its arrows; the colour stays by relation. The literal `\n` goes with the label, so
  L-96 is gone from every view this stage rewrote. **Default.**
- **(b)** a node of its own for the source, pointed at by the fact.

**Ruled 2026-11-18** by the user, on the review bundle's `03-drawings/` (item 35; the answers are `e4c0eaf6`),
verbatim: *"(a)"* **(a)**, the default as built. Recorded by S1f.23.6; carried by
[Q-M1f.419](../../docs/decisions/cli/Q-M1f.419-ein-render-draws-a-fact-as-the-graph-has-it.md), which says what stands
beside an octagon.

The default had been (a). Built: `ein-render/src/fact_graph.rs` (`Look`), `kb_dot.rs`, `ir_dot.rs`' `stated_look`;
the derivation graph writes its source, rule or hypothesis there too.

### Q-M1f.425 — The fold's option, and what it folds

**Raised** 2026-11-18 by S1f.23.1's build, on Q-M1f.419 (*"optionally fold 3-facts into labelled arrows"*). **Ruled 2026-11-18.**

- **(a)** **`--fold`, one name on every subcommand that draws facts** (`ein render program`, `kb`, `rules`, `rule`,
  `constraints`, and `ein solve` for the trace, Q-M1f.422); `--levi` goes, since the octagon is the default. It
  folds a fact of exactly three arrows that no drawn fact points at; a fact of two arrows is no loop, and a fact a
  drawn fact points at is a node. **Default.**
- **(b)** a fact of two arrows folded into a loop on its argument, as the views drew one before.

**Ruled 2026-11-18** by the user, on the review bundle's `03-drawings/` (item 36; the answers are `e4c0eaf6`),
verbatim: *"(a) + (b)"* **(a) and (b)**: `--fold` on every command that draws facts, folding a fact of three arrows
into an arrow and a fact of two into a loop on its argument, where nothing drawn points at it. **Built** by
[S1f.23.6](p1f.23_review_processing/s1f.23.6_the_drawings_answers.md): `ein_render::fact_graph::folds`, `rules.rs`,
`slice.rs`; `every_object_once.rs` fails on a fact of two or three arrows that nothing points at and is not folded.
Recorded by S1f.23.6; the decision
[Q-M1f.425](../../docs/decisions/cli/Q-M1f.425-fold-folds-a-fact-of-three-arrows-into-an-arrow.md); Q-M1f.419 and
Q-M1f.382 amended.

The default had been (a). Built: `ein-cli/src/cmdline.rs`' `fold`; held by `every_object_once.rs` and
`render_doors.rs`' `every_door_draws_a_fact_in_both_forms`; the pages `12-rendering.md`, `05-cli.md`,
`06-config.md`.

### Q-M1f.426 — What the constraint view draws

**Raised** 2026-11-18 by S1f.23.1's build. **Ruled 2026-11-18.** The view wrote a relation's properties into its
node, `next-to\n«symmetric»` (a fact's text in a node), and chose its facts by leaving out three heads, `relation`,
`type` and `instance`; the user's ruling of 2026-11-18 on theory names takes `type` and `instance` out (S1f.23.1's
Record).

- **(a)** **each fact drawn as a fact**, by the KB view's code: `(symmetric next-to)` an octagon whose second arrow
  is at `next-to`, folded where it has three arrows. The facts drawn are every fact the program states with no
  `:source`, `:rule` or `:using`, of a relation it does not declare; no name is left out by what it means, so a
  declaration, a fact of `relation`, which no program declares, is drawn among them. **Default.**
- **(b)** as (a), declarations left out by the name `relation`.

**Ruled 2026-11-18** by the user, on the review bundle's `03-drawings/` (item 37; the answers are `e4c0eaf6`),
verbatim: *"So, it turns out that we have commands that render part of KB. I'm not against it - (a). Small addition -
would be nice to formalize this KB slice with something, e.g. ein query or rule that selects all nodes where relation
is argument (is it correct definition?), and together with `constraints` allow to render arbitrary query."* **(a)**;
the addition, a slice of the knowledge base defined by a query and drawn, is
[S1f.23.7](p1f.23_review_processing/s1f.23.7_a_slice_by_a_query.md), planned and not started. Recorded by S1f.23.6;
the decision [Q-M1f.426](../../docs/decisions/cli/Q-M1f.426-the-constraint-view-draws-each-fact-as-a-fact.md).

The default had been (a). Built: `ein-render/src/constraints.rs`; held by `every_object_once.rs`'
`the_constraint_view_draws_a_property_as_a_fact`.

### Q-M1f.427 — How a rule's pattern is drawn

**Raised** 2026-11-18 by S1f.23.1's build. **Ruled 2026-11-18.**

- **(a)** **a pattern as a fact**: an octagon with no name in it, its arrow `1` at its relation, a box per panel,
  or the variable's diamond where the relation is a variable; folded under `--fold` where it has three arrows and
  nothing points at it. Guards keep their dotted `≠`/`=` links and `absent` its cluster. A `(not P)` is a pattern
  of `not`, red, its arrow `2` at `P`'s node, which stands in `not ctx` where `P` is no premise, as the reasoning
  pages draw a negation (`01-context-negation.dot`). No macro's name is read: `std.macro`'s `forall`, which the view
  drew as a `∀` cluster, and `unknown` are patterns as called. An empty `()` draws nothing. **Default.**
- **(b)** `forall` kept as a cluster of its own.

**Ruled 2026-11-18** by the user, on the review bundle's `03-drawings/` (item 38; the answers are `e4c0eaf6`),
verbatim: *"(a)"* **(a)**, written in the item's notes; R-15's answer with it, (i). Recorded by S1f.23.6; the decision
[Q-M1f.427](../../docs/decisions/cli/Q-M1f.427-a-rules-pattern-is-drawn-as-a-fact-and-no-macro.md).

The default had been (a). Built: `ein-render/src/rules.rs`; held by `every_object_once.rs`'
`the_rule_view_draws_a_pattern_as_a_fact` and `presentation_semantics.rs`.

### Q-M1f.428 — How a derivation is drawn

**Raised** 2026-11-18 by S1f.23.1's build, on Q-M1f.419 (L-99). **Ruled 2026-11-18.**

- **(a)** **each fact as its node, each step a node of its own**: in a trace's slice, the firing's node from its
  premises to what it derives, a commitment's facts red and a stored negation grey, no node filled; a fact a fact of
  the cone points at is its node too, in its frame where the state does not hold it. The `⊥` of a refutation stays
  a mark of the step, which the core's facts point at, and is no fact's node; the learned no-good is a note,
  `learned no-good`, that points at each of its facts. The derivation graph draws a diamond for every derivation,
  one where a fact has one, and what a fact rests on beside it. A written trace's `dag` view draws each step a node,
  `s1` and its rule, from its premises to the fact it derives. **Default.**
- **(b)** the derivation graph's diamond only where a fact has two derivations, as before.

**Ruled 2026-11-18** by the user, on the review bundle's `03-drawings/` (item 39; the answers are `e4c0eaf6`),
verbatim: *"(a) I like what we already have, just use more contrast colors and 2x thicker lines then default."*
**(a)**, and a derivation's arrows and steps in colours of more contrast and twice Graphviz's default width. **Built**
by [S1f.23.6](p1f.23_review_processing/s1f.23.6_the_drawings_answers.md): `slice.rs`' `DERIVATION_PALETTE` (eight dark
colours, none red nor grey, picked by the rule's name) and `DERIVATION_WIDTH` (`penwidth=2`), on the slices, the
derivation graph and a written trace's `dag` view. Recorded by S1f.23.6; the decision
[Q-M1f.428](../../docs/decisions/cli/Q-M1f.428-a-derivation-is-drawn-with-each-fact-its-node.md).

The default had been (a). Built: `ein-render/src/slice.rs`, `derivation.rs`, `ir_dot.rs`' `render_trace_dag`; the
goldens `slice_render_slice.dot`, `kb_provenance_dag.dot`, `ir_render_trace_dag.dot`, the trace goldens.

### Q-M1f.429 — A folded fact in a drawing of a derivation

**Raised** 2026-11-18 by S1f.23.1's build, on Q-M1f.422. **Ruled 2026-11-18.** In a slice every fact is the end of a
firing's arrow, and an arrow cannot end at an arrow.

- **(a)** **a labelled arrow through a point**: a fact of three arrows that no fact of the cone points at is an
  arrow from its first argument to a point, labelled with its relation, and on from the point to its second
  argument; the firings' arrows enter and leave the point. **Default.**
- **(b)** under `--fold` a slice folds nothing, and only the `-F` snapshots fold.

**Ruled 2026-11-18** by the user, on the review bundle's `03-drawings/` (item 40; the answers are `e4c0eaf6`),
verbatim: *"do not fold, when we show derivation with additional arrow (of different color); general rule for
rendering - no arrow to/from arrows, arrows go only to/from shapes; So other 3-facts and 2-facts can be folded, but
not those that hold derivation arrows."* **neither option**: no arrow goes to or from an arrow, in every drawing; in a
slice a fact a derivation's arrow touches is never folded, every other fact of two or three arrows folds, and the
point goes. **Built** by [S1f.23.6](p1f.23_review_processing/s1f.23.6_the_drawings_answers.md): `slice.rs`;
`render_doors.rs` holds a step's slices under `--fold` to the canonical ones, with no point. Recorded by S1f.23.6; the
decision [Q-M1f.429](../../docs/decisions/cli/Q-M1f.429-no-arrow-goes-to-or-from-an-arrow.md).

The default had been (a). Built: `ein-render/src/slice.rs`; held by `render_doors.rs`'
`every_door_draws_a_fact_in_both_forms`.

### Q-M1f.430 — The region, lattice and h-space views

**Raised** 2026-11-18 by S1f.23.1's build. **Ruled 2026-11-18.** These views draw states, nodes and candidates,
labelled with their facts' text, and the region view draws a reported state as an octagon with its facts in it.

- **(a)** **left as they are**: a node is a state or a candidate, and its label a list of what it holds, not a
  drawing of a fact. **Default.**
- **(b)** they are form 4 too, and each state's or candidate's facts are drawn as nodes.

**Ruled 2026-11-18** by the user, on the review bundle's `03-drawings/` (item 41; the answers are `e4c0eaf6`),
verbatim: *"(a)"* **(a)**, the default as built. Recorded by S1f.23.6; the decision
[Q-M1f.430](../../docs/decisions/cli/Q-M1f.430-the-canonical-form-draws-a-knowledge-base.md), with Q-M1f.470's words.

The default had been (a). Nothing built; `region_dot.rs`, `dump/region_lattice.rs` and `structure.rs` are unchanged.

### Q-M1f.431 — The program view's three parts

**Raised** 2026-11-18 by S1f.23.1's build, on Q-M1f.420. **Ruled 2026-11-18.**

- **(a)** **as built**: the digraph `kb` holds every fact and declaration the program states, in the order the
  loader reads them, declarations first, a fact written twice drawn once as written first; a written check,
  `(eq …)`, `(neq …)`, `(not (eq …))`, is no fact and not drawn; an `(import …)` and a `(config …)` draw nothing; a
  program that states no fact has no `kb`. A digraph per rule and hrule as `ein render rule` draws one, and per
  macro its body as a match, `macro_<name>`. The digraph `query`: each query a cluster, its `:goal` drawn as a rule's
  match is and its other clauses in a note; the `(solve …)` clause a note, a flag written alone, `:no-search` (it
  printed `:no-search (#empty)`). A written `(trace …)` is drawn after them. **Default.**
- **(b)** the query's `:goal` in its note as text, as before.

**What stays a question:** a note writes a claim as it is written, `:expect (model (owns Bob Dog))`, so a fact's
text stands in a note's label, as a state's does in the region view (Q-M1f.430).

**Ruled 2026-11-18** by the user, on the review bundle's `03-drawings/` (item 42; the answers are `e4c0eaf6`),
verbatim: *"(a)"* **(a)**, the default as built. Recorded by S1f.23.6; carried by
[Q-M1f.420](../../docs/decisions/cli/Q-M1f.420-ein-render-program-draws-the-programs-knowledge.md), amended with the
parts' details.

The default had been (a). Built: `ein-render/src/ir_dot.rs`; held by `every_object_once.rs`'
`the_program_views_kb_is_the_kb_views_drawing` and its corpus sweep.

### Q-M1f.432 — What the KB view leaves out

**Raised** 2026-11-18 by S1f.23.1's build, on Q-M1f.420 (*"`ein render kb` renders whole KB"*). **Ruled 2026-11-18.**
The view left out a fact that applies a rule, `(symmetric co-located)`, and every stored negation.

- **(a)** **nothing**: every fact the knowledge base holds is drawn, a stored negation a fact of `not` whose second
  arrow is at the fact it denies, a fact that applies a rule an octagon whose first arrow is at the rule's name, so
  the program view's `kb` and the KB view of a program with no rule are one drawing. **Default.**
- **(b)** both left out, in the KB view and the program view's `kb`.

**Ruled 2026-11-18** by the user, on the review bundle's `03-drawings/` (item 43; the answers are `e4c0eaf6`),
verbatim: *"(a)"* **(a)**, the default as built. Recorded by S1f.23.6; carried by
[Q-M1f.420](../../docs/decisions/cli/Q-M1f.420-ein-render-program-draws-the-programs-knowledge.md), amended: the KB
view leaves nothing out.

The default had been (a). Built: `ein-render/src/kb_dot.rs`; the page `12-rendering.md` § The KB.

### Q-M1f.433 — The shape of an atom

**Raised** 2026-11-18 by S1f.23.1's build, on Q-M1f.421 (*"every atom the same shape"*). **Ruled 2026-11-18.**

- **(a)** **a box, as the reasoning pages draw one**: the node default of the KB, program and constraint views, so
  no atom has a line of its own, and `box` in a rule's panel, where a variable is a diamond. **Default.**
- **(b)** Graphviz's default ellipse, as the KB view drew an atom that was neither a type nor an instance.

**Ruled 2026-11-18** by the user, on the review bundle's `03-drawings/` (item 44; the answers are `e4c0eaf6`),
verbatim: *"(a) main: object - labelled box, fact - non-labelled octagon, variable - labelled diamond;"* **(a)**, the
user's rule for every drawing of facts: an object a labelled box, a fact an unlabelled octagon, a variable a labelled
diamond, built so already. Recorded by S1f.23.6; the decision
[Q-M1f.433](../../docs/decisions/cli/Q-M1f.433-an-object-is-a-labelled-box-a-fact-an-unlabelled.md).

The default had been (a). Built: `ein-render/src/kb_dot.rs`' `kb_digraph_open`, `dot_util.rs`' `GROUND_SHAPE`.

### Q-M1f.434 — The standard library's rule drawings past the bound

**Raised** 2026-11-18 by S1f.23.1's build; L-97 is still open. **Ruled 2026-11-18.**

- **(a)** **the canonical drawing where it is within `MAX_NODES`, else the folded one where that is, else Ein
  only**, the image's text saying a drawing is folded. **Default.**
- **(b)** the canonical drawing or Ein only.

**Ruled 2026-11-18** by the user, on the review bundle's `03-drawings/` (item 45; the answers are `e4c0eaf6`),
verbatim: *"(a)"* **(a)**, the default as built. Recorded by S1f.23.6; the decision
[Q-M1f.434](../../docs/decisions/docs/Q-M1f.434-a-standard-library-rule-is-drawn-canonical-else.md); with Q-M1f.425's
loop the generator counts 493 canonical, 6 folded, 12 as Ein only (five closing rules move from Ein only to folded).

The default had been (a). Built: `utils/gen_stdlib_rule_pages.py`' `chosen`; the counts are S1f.23.1's Record.

### Q-M1f.435 — Where an arrow's number is written

**Raised** 2026-11-18 by S1f.23.1's build, on Q-M1f.423. **Ruled 2026-11-18.** Written as the arrow's `label`, as the
reasoning pages' hand drawings write it, each number is a node of `dot`'s layered layout, and a knowledge base of
some hundreds of facts with frames is laid out in minutes: the program view's `kb` of
`examples/arith/long_numbers.ein`, 750 arrows, took 360 s of `dot` with labels, 55 s with `taillabel` and 38 s with
`xlabel`, on this machine under the load of two other test runs (the program view before the stage, 493 arrows,
142 s). `dot_wellformed.rs` lays out each view of every corpus program outside `tests/` per commit.

- **(a)** **the arrow's `xlabel`**, which Graphviz places beside the arrow after the layout, near its middle, in
  every drawing of facts: the KB, program and constraint views, a rule's patterns, a trace's slices, the derivation
  graph; and in a drawing of a knowledge base (the KB view, the program view's `kb`, the constraint view) a folded
  arrow's relation too, `xlabel="owns"`: one `label` in a digraph makes `dot` lay every arrow over two ranks, and
  the folded `kb` of `long_numbers.ein`, ten labelled arrows among 752, ran past a quarter of an hour of `dot`, where
  with `xlabel`s it took 51 s. The user's *label="owns"* is read as the word written on the arrow, `owns` and not
  `(owns)`. A rule's panel and a trace's slice, small, keep a folded arrow's `label`. **Default.**
- **(b)** the `label`, as the pages draw it, with the gate's DOT sweep slower by that view alone.

**Ruled 2026-11-18** by the user, on the review bundle's `03-drawings/` (item 46; the answers are `e4c0eaf6`),
verbatim: *"(a)"* **(a)**, the default as built. Recorded by S1f.23.6; the decision
[Q-M1f.435](../../docs/decisions/cli/Q-M1f.435-an-arrows-number-is-written-beside-it.md).

The default had been (a). Built: `ein-render/src/fact_graph.rs`, `rules.rs`, `slice.rs`, `derivation.rs`.

### Q-M1f.436 — The kernel reads no theory's name, and the built-in symmetry goes

**Raised and ruled** 2026-11-18 by the user, on P1f.23's review, verbatim:

> In general: ein.rs must expect the minimal shape of the program: rules, relations, ... Not special names like
> `is-a` `type` `instance` `symmetric` `bijection` `slot`; All those names belong to some specific theory and must
> not pollute the kernel.

and, on the main session's census of what `ein.rs` reads by a theory's name and its proposal of a stage for it, with
the question whether the built-in `__symmetric__` was that stage's or P1f.21's:

> Yes, let's drop also built-in symmetry.

**Ruling.** The kernel gives a fixed meaning to its own names alone, `ein_core::terms::Kernel`'s, the grammar's words
and `:over`'s three values; `is-a`, `type`, `instance`, `symmetric`, `bijective`, `slot-partition` and every other
name are a theory's or a program's, and mean what the rules that read them make them mean. The native mirror goes:
`__symmetric__` is an ordinary name, and a symmetric relation is `std.algebra`'s rule. `program_relations` names `eq`
and `neq` alone, so a goal over `is-a` in a program that writes none is refused. **Built by** P1f.23's
[S1f.23.4](p1f.23_review_processing/s1f.23.4_the_kernel_reads_no_theorys_name.md) (the mirror, `program_relations`,
six programs retired with four regression rows, the renaming test), with
[S1f.23.1](p1f.23_review_processing/s1f.23.1_drawing.md)'s part (no drawing reads `is-a`, `type`, `instance`,
`relation` or `std.macro`'s `forall`: Q-M1f.421, Q-M1f.426, Q-M1f.427) and
[S1f.23.2](p1f.23_review_processing/s1f.23.2_no_key_and_no_grid.md)'s (`--models key` and `grid`, whose `grid.rs`
and `models.rs` read `is-a`, `T`, `slot-partition`, `functional`, `injective` and `bijective`, removed in its
worktree; R-1 and R-3 until it is merged). **Held by** `ein-render/tests/renaming_invariance.rs`: every corpus
program, with every name that is not the kernel's renamed, answers and is drawn alike, its `READS` empty on the
merged tree. The decision
[Q-M1f.436](../../docs/decisions/engine/Q-M1f.436-the-kernel-reads-no-theorys-name.md);
[Q-M1f.7](../../docs/decisions/language/Q-M1f.7-the-dunder-is-never-bound-and-the-writeback.md) keeps the dunder
and cites it.

### Q-M1f.437 — The flagged spelling of `symmetric-derived-node`

**Raised** 2026-11-18 by S1f.23.4's build, step 1. **Put, not ruled.** With `std.algebra`'s `(symmetric near)` in
place of `(__symmetric__ near)`, `examples/ein-bugs/wrong-answers/symmetric-derived-node.ein` answered the same R, in
every mode and the oracle, but the static check flagged it, *monotone no — 6 sites* (the module's lemmas read
activator relations a commitment can change), so its route was the cube where the native spelling's was the state
route. The native mark derived at a node was invisible to the static check.

- **(a)** **not the stop condition**: the invariance the stage holds is of the answer, which is the same; shown in
  its record, and the program retired with the mirror (Q-M1f.438). **Default.**
- **(b)** the stop condition: the program shown to the user before the mirror goes.

**Default taken: (a).** Built: S1f.23.4's Record, step 1; no corpus program carries the difference.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.438 — The programs that showed only the mirror: retired or rewritten

**Raised** 2026-11-18 by S1f.23.4's build, step 2. **Put, not ruled.** Six programs wrote `(__symmetric__ R)` or
existed as the control of one that did: `examples/features/06_symmetric_native.ein`, and under
`examples/ein-bugs/wrong-answers/` `symmetric-derived-root`, `symmetric-written-root`, `symmetric-derived-node`,
`symmetric-rule-node` and `dunder-object-symmetric-by-rule`.

- **(a)** **retired, all six**, each showing only the mirror, with their corpus entries and their four rows of
  `corpus/regressions.toml` (`symmetric-mirror-off`, `symmetric-by-rule`, `derived-at-root`, `derived-at-node`): a
  fixed wrong answer of a mechanism that is gone goes with it, its history in git; a symmetric relation by
  `std.algebra`'s rule is `examples/saturation/symmetric/`'s three. **Default.**
- **(b)** rewritten to `std.algebra`'s rule and kept, with their rows.

**Default taken: (a).** Built: `corpus/corpus.toml`, `corpus/regressions.toml` (333 rows, 329 after).

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.439 — `examples/features/`' numbering with `06` gone

**Raised** 2026-11-18 by S1f.23.4's build. **Put, not ruled.**

- **(a)** **kept, a gap at `06`**: the numbering rule is the documentation's (Q-M1f.367), not the examples'.
  **Default.**
- **(b)** renumbered, the programs after `06` moved down one, with every reference.

**Default taken: (a).**

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.440 — The retired flag `enable-symmetric-mirror`

**Raised** 2026-11-18 by S1f.23.4's build. **Put, not ruled.**

- **(a)** **stays retired**: refused with a new reason, *it switched the kernel's native mirror, and the kernel mirrors
  no relation …*, its field kept for the `.einb` layout. **Default.**
- **(b)** taken off the retired list, so that it is refused as any unknown field is.

**Default taken: (a).** Built: `ein-core/src/config.rs`' `RETIRED`; the grammar table's golden line;
`06-config.md` § 2.4.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.441 — The `--events` protocol's version, the `mirror` kind gone

**Raised** 2026-11-18 by S1f.23.4's build. **Put, not ruled.**

- **(a)** **stays `ein-events/1`**: the `mirror` kind is gone and no other field moved, so a reader of `/1` reads
  every stream the engine writes. **Default.**
- **(b)** `ein-events/2`.

**Default taken: (a).** Built: `05-events.md`; `ein-cli/tests/events_reference.rs` (eighteen kinds).

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.442 — Q-M1f.7's file name, its title changed

**Raised** 2026-11-18 by S1f.23.4's build. **Put, not ruled.** Q-M1f.7's title became *The dunder is never bound,
and the writeback names never written*; its file kept the old title's slug, since the stage did not edit this file,
which links it.

- **(a)** **renamed on the merge** to the new title's slug,
  `Q-M1f.7-the-dunder-is-never-bound-and-the-writeback.md` (Q-M1f.344), every link with it. **Default.**
- **(b)** the old slug kept.

**Default taken: (a).** Built on the merge.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.443 — A decision for the two rulings

**Raised** 2026-11-18 by S1f.23.4's build. **Put, not ruled.** The stage gave the rulings no decision of their own:
Q-M1f.7 and the pages carried *the kernel reads no theory's name*, and the stage left a decision file to the merge.

- **(a)** **a decision of its own**, [Q-M1f.436](../../docs/decisions/engine/Q-M1f.436-the-kernel-reads-no-theorys-name.md),
  under `engine/`, carrying both rulings; Q-M1f.7 keeps the dunder and cites it, as do `AGENTS.md` and
  `07-reserved-engine-strings.md`. **Default**, taken on the merge.
- **(b)** none: Q-M1f.7 and the pages carry the rulings, as the stage left them.

**Default taken: (a).**

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.444 — What the renaming test keeps

**Raised** 2026-11-18 by S1f.23.4's build, step 4. **Put, not ruled.**

- **(a)** **as built**: it keeps the kernel's names (what `Terms::new` interns), `:over`'s three values
  (`ein_ir::expect::Over::NAMES`, which the loader reads as a keyword's), the grammar's words (no `NAME` token),
  module names and aliases (where a file is found); it does not compare the read-out's text, which wraps at a width a
  longer name moves and whose content is the summary's, which it compares. **Default.**
- **(b)** the read-out's text compared too, under a normaliser of its layout.

**Default taken: (a).** Built: `ein-render/tests/renaming_invariance.rs`.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.445 — The renaming test's tier

**Raised** 2026-11-18 by S1f.23.4's build, step 4. **Put, not ruled.**

- **(a)** **per commit the programs outside `tests/`**, as `id_order_invariance` and `dot_wellformed` read them, and
  **under `EIN_CORPUS_SLOW=1` every program**, a step of the nightly's `deep-corpus` job (Q-M1f.290's split): at the
  stage 19.9 s per commit, and the whole corpus 86 s on four P-core threads. **Default.**
- **(b)** the nightly alone.

**Default taken: (a).** Built: `run_tests.sh`, `.github/workflows/nightly.yml`.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.446 — The `forall` drawing the renaming test found

**Raised** 2026-11-18 by S1f.23.4's build, step 4. **Put, not ruled.** The renaming test found `ein-render/src/rules.rs`
reading `std.macro`'s `forall` by its name (`head_name`, `is_guard`), a theory's name in the renderer and not in the
census at the ruling (P1f.23's R-4). S1f.23.4 listed `dot[rules]` and `dot[rules-overlay]` in the test's `READS` and
left `rules.rs` to S1f.23.1, which was rewriting it.

- **(a)** **S1f.23.1's**: its rewrite of `rules.rs` drops the read (Q-M1f.427), and the merge takes the two views out
  of `READS`. **Default.**
- **(b)** a stage of its own after S1f.23.1.

**Default taken: (a).** Built: S1f.23.1 drew a call of `forall` as the form it is; on the merge `READS` is empty and
every view is compared.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.447 — The loader's argument-count check, once `relation` is a convention

**Raised** 2026-11-18 by the user, with [P1f.21's fourth instruction](p1f.21_verdicts_as_rules/README.md#the-fourth-instruction),
verbatim:

> p1f.21: `false` also becomes stdlib convention; argument-count check - will think about it in p1f.21, record as
> open question;

**Open: not ruled, and no default taken. The user will think about it in P1f.21.** Today the loader holds a form
headed by a declared relation to as many arguments as the declaration has labels, and refuses another count. The
fourth instruction makes `relation`, the declaration that check reads, a convention of the standard library and not
the kernel's, so the check would be the loader reading a theory's form.

- **(a)** kept at load, as the loader's reading of a declaration;
- **(b)** a rule's verdict at run time, a rule of the module that states the convention over `relation`'s facts;
- **(c)** dropped: Ein has no types, and a label means what a rule makes it mean (Q-M1f.372), so a relation may be
  written at any count.

P1f.21's S1f.21.4 shows each on the corpus's programs that the check refuses or that write a relation at two counts,
and does not choose ([`open_facts.md` § 9](p1f.21_verdicts_as_rules/open_facts.md#9-what-s1f214-answers), item 6), with
[V-7](p1f.21_verdicts_as_rules/findings.md#v-7--a-rule-can-conclude-a-fact-of-a-relation-the-program-does-not-count),
a relation that only an activator names.

### Q-M1f.448 — `--models`, with `key` and `grid` gone

**Raised** 2026-11-18 by S1f.23.2's build, reading 1. **Put, not ruled.** The user ruled `--models key` and
`--models grid` away (*"remove key and grid"*); the flag's one value left, `list`, the default, added nothing, and
nothing read it: no script (`utils/review` ran only `key` and `grid`), no corpus run, no config row (`06-config.md`
named it in its presentation list alone), neither `ein test` nor any artefact.

- **(a)** **removed whole**, from both parsers that had it (`ein solve`, `ein oracle`), with its help, its id in
  `--monotone`'s conflicts, its `--count` refusal and its name in `--dump-answer`'s help. **Default.**
- **(b)** kept with `list` alone, a flag of nothing, for a reader that still passes it.

**Default taken: (a).** Built: `ein-cli/src/cmdline.rs`, `modes.rs`, `oracle.rs`; `help_surface.rs` and the help
golden; `05-cli.md`, `06-config.md`.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.449 — `--models`: removed, or retired

**Raised** 2026-11-18 by S1f.23.2's build, reading 2. **Put, not ruled.** `ein solve` refuses a retired option with
what answers instead (`cmdline.rs`' `retired(…)`, `solve::RETIRED`); that table holds what the switch to R replaced.

- **(a)** **removed, not retired**: clap refuses `--models` as an unknown argument, exit 2. **Default.**
- **(b)** a retired entry, refused with what answers instead, `--maximal` and `-f`.

**Default taken: (a).** `--diff-models` takes the same reading, Q-M1f.466.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.450 — `--models grid`'s reading of `is-a`, and the grid's names

**Raised** 2026-11-18 by S1f.23.2's build, reading 3. **Put, not ruled.** Q-M1f.412's review item found
`--models grid` reading `is-a` (P1f.23's [R-1](p1f.23_review_processing/findings.md)), and its help speaking of a
type (R-3); both waited on the user's ruling on theory names, Q-M1f.436.

- **(a)** **ended with the code**: `ein-render/src/grid.rs` is deleted whole, and R-1 and R-3 with it. **Default.**
- **(b)** kept open until the user rules on them by name.

**Default taken: (a).** R-1 and R-3 are marked ended in P1f.23's findings.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.451 — The determining key on the reasoning page

**Raised** 2026-11-18 by S1f.23.2's build, reading 4. **Put, not ruled.** `reasoning/02-kb-evolution/10-satisfiability.md`
§ 6 defined the *determining key*, what `--models key` computed, and *what models differ in*, the variables only the
key read; no other page uses either.

- **(a)** **taken out**, with the term, *what models differ in*, the drawing `diagrams/10-key.dot`, the intro's
  *compact exact form*, and `key` from the notation table of `reasoning/README.md`; the model and the count's
  guarantee stay. **Default.**
- **(b)** kept as mathematics, a notion of the page with no command behind it.

**Default taken: (a).**

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.452 — What the walkthrough shows where it showed the grids

**Raised** 2026-11-18 by S1f.23.2's build, reading 5. **Put, not ruled.** The guide's `06-zebra-walkthrough.md`
showed `--models grid`'s tables at root after step 1 and at the final state.

- **(a)** **`-f`'s output**: Ein facts of the five relations a guess is made of, as printed, and no table; the
  minus-15 bullet says `--maximal` lists the states. **Default.**
- **(b)** a hand-drawn table, marked as drawn by hand.

**Default taken: (a).**

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.453 — `models.rs`' name, once it holds the diff alone

**Raised** 2026-11-18 by S1f.23.2's build, reading 6. **Put, not ruled.** With the key and the grid gone,
`ein-render/src/models.rs` held `--diff-models`' code alone.

- **(a)** **kept**, `models.rs`. **Default.**
- **(b)** `diff.rs`.

**Default taken: (a)**, and superseded on the merge by S1f.23.3, which removed `--diff-models` and renamed the module
for what it holds, `claim_diff.rs` (Q-M1f.465).

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.454 — `ein test --diff`'s spelling

**Raised** 2026-11-18 by S1f.23.3's build, reading 1. **Put, not ruled.** The user asked for the near-miss as an
option of `ein test` (*"when ein test see expected models and solution result it can diff them (under option) and
print stats and/or diffs"*).

- **(a)** **`--diff stats|full`**, clap's optional value: a bare `--diff` is `stats` where no word follows it; before
  a path it takes the path for its level and is refused, exit 2, naming the two levels (pinned by a test).
  **Default.**
- **(b)** `--diff[=LEVEL]` (`require_equals`): a bare `--diff` anywhere, but `--diff full` reads `full` as a path.

**Default taken: (a).** Built: `ein-cli/src/cmdline.rs`, `test.rs`; `test_cli.rs`'
`a_bare_diff_before_a_path_is_refused_with_its_levels`.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.455 — How many facts off a near model is

**Raised** 2026-11-18 by S1f.23.3's build, reading 2. **Put, not ruled.**

- **(a)** **an edit distance**: a fact replaced counts once, `max(lacks, has)`, so the user's *one fact off*
  includes a wrong value, `(in C 1)` for `(in C 0)`; pairs are taken fewest facts off first, then fewest facts
  differing, then the found items' order and the models'; near only while at most half the larger side off, and at
  least one. **Default.**
- **(b)** the symmetric difference: a wrong value is two facts off, and a model of one fact never pairs near.

**Default taken: (a).** Built: `ein-render/src/claim_diff.rs`; `a_claim_one_fact_off_reads_as_one_fact_off`.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.456 — What the diff counts as a match

**Raised** 2026-11-18 by S1f.23.3's build, reading 3. **Put, not ruled.**

- **(a)** **what the claim's check pairs**: a maximum matching, a listed negative read for its presence, so a held
  claim's diff is all matches, and a region claim that lists a model twice against one state shows one match and one
  missing. **Default.**
- **(b)** every model equal to a found item a match, so a model listed twice is two matches of one state.

**Default taken: (a).** Built: `claim::compared`, `claim_diff::diff_sets`; `the_diff_moves_no_verdict_and_no_exit_code`.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.457 — Which claims have a diff

**Raised** 2026-11-18 by S1f.23.3's build, reading 4. **Put, not ruled.**

- **(a)** **every claim a run decided, held or FAILED, but the region's `(false)`**, which lists no model and is
  about root: `(region)` has one over no relation (every state extra, each `(model)`), a query's `(false)` one whose
  every class is extra; a NOT CHECKED claim has none, what its run found not being what it is about; a query claim
  refuted from a part has one, marked *a part of R*. **Default.**
- **(b)** only a FAILED claim has a diff.

**Default taken: (a).**

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.458 — A missing model and an extra found item

**Raised** 2026-11-18 by S1f.23.3's build, reading 5. **Put, not ruled.**

- **(a)** **a missing model is shown against the found item nearest it**, among all found, matched ones too, with
  its facts; **an extra found item names the model nearest it and how far, without facts** (the user named the
  missing and near models' facts; an extra state's facts are the read-out's). Past the pair table's bound no
  nearest is sought. **Default.**
- **(b)** an extra found item with its facts too.

**Default taken: (a).** Built: `a_region_claim_names_the_missing_and_the_extra`.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.459 — An `(owing …)` claim's diff

**Raised** 2026-11-18 by S1f.23.3's build, reading 6. **Put, not ruled.**

- **(a)** **compared with the owing frontier**, its nodes numbered in the read-out's order, *owing node i*; where R
  is not empty the frontier is empty and every owing model missing. **Default.**
- **(b)** compared with R's states, as a region claim's models are.

**Default taken: (a).**

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.460 — `(merged …)` and the diff

**Raised** 2026-11-18 by S1f.23.3's build, reading 7. **Put, not ruled.**

- **(a)** **not diffed**: the diff is the models', so a claim FAILED by its `(merged …)` alone has an all-match
  diff, and the claim's own lines say why. **Default.**
- **(b)** the merged classes diffed too.

**Default taken: (a).**

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.461 — Where the diff prints

**Raised** 2026-11-18 by S1f.23.3's build, reading 8. **Put, not ruled.**

- **(a)** **under `--diff` a claim with a diff prints its line at the default volume**, held ones too; the diff
  follows the claim's report lines and precedes the warning and *outside the universe* lines; `-q` diffs only the
  claims it prints. **Default.**
- **(b)** `--diff` overrides `-q`.

**Default taken: (a).** Built: `quiet_diffs_only_the_claims_it_prints`.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.462 — The report's `diff`, and its id

**Raised** 2026-11-18 by S1f.23.3's build, reading 9. **Put, not ruled.**

- **(a)** **`diff` on every row**, `null` without the option, on a claim no run decided and on the region's
  `(false)`; one shape on every arm, indices 1-based as printed, `items` `null` under `stats`; the id moves to
  `ein-test-report/6`. **Default.**
- **(b)** the key only under `--diff`, and the id kept.

**Default taken: (a).** Built: `05-cli.md` § 4.4, `06-config.md`. On the merge master's report was still
`ein-test-report/5`, [Q-M1f.412](#q-m1f412--the-read-outs-is-a-row)'s default (a) having moved no id, so `/6` is the
diff's alone (P1f.23's R-10).

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.463 — The review bundle's right answer, written as claims

**Raised** 2026-11-18 by S1f.23.3's build, reading 10. **Put, not ruled.** `utils/review/case.py` writes a register
row's right answer into a copy of the witness and runs `ein test --diff full` on it, where `--diff-models` read a
file of its own format.

- **(a)** **query claims, one per relation of `closes`**, over the line the row's run asks, not a region claim: a
  region claim closes only the relations its models name (fifo-gap-root's `p`, named by no right model, is the row's
  wrong fact) and is about all of R, where two of the three rows that state models ask `--maximal`. Where `right`
  states more than one model the claims state each relation's facts across them and not which go together; the three
  rows that state models state one each. A witness that imports a module of its own is not copied, and its page says
  so (no open row does). **Default.**
- **(b)** a region claim, `(solve … :expect (region …))`, of the right models.

**Default taken: (a).** Built: `utils/review/case.py`, `run.py`.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.464 — The review run page's cross-check 1

**Raised** 2026-11-18 by S1f.23.3's build, reading 11. **Put, not ruled.** Cross-check 1 compared the old second
solve's summary, under `--diff-models`, with the main run's.

- **(a)** **the `ein test` run's claims, where whole, against the run's listed states, by count**. **Default.**
- **(b)** dropped, with the second solve.

**Default taken: (a).** Built: `utils/review/run.py`.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.465 — The diff's module

**Raised** 2026-11-18 by S1f.23.3's build, reading 12. **Put, not ruled.** S1f.23.2's reading kept `models.rs`
(Q-M1f.453), and offered `diff.rs`.

- **(a)** **`claim_diff.rs`**: it holds a claim's diff and no models' form. **Default.**
- **(b)** `diff.rs`.
- **(c)** `models.rs`, as S1f.23.2 left it.

**Default taken: (a).** Built: `ein-render/src/claim_diff.rs`; `01-architecture.md` and its module diagram.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.466 — `--diff-models`: removed, or retired

**Raised** 2026-11-18 by S1f.23.3's build, reading 13. **Put, not ruled.** As Q-M1f.449 for `--models`.

- **(a)** **removed, not retired**: clap refuses it as an unknown argument, exit 2. **Default.**
- **(b)** a retired entry, refused with what answers instead, `ein test --diff`.

**Default taken: (a).**

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.467 — A command's output on a page: canonical or folded

**Raised** 2026-11-18 by S1f.23.1's task 2, the hand-written drawings and the pages. **Ruled 2026-11-18.**

- **(a)** on a page, the binary's output for the command as the page writes it, canonical by default; folded where the page's point is the compact view, and the page then writes `--fold` (in `reasoning/`, says that it folds). **Default.**
- **(b)** the page shows the canonical form everywhere.

**Ruled 2026-11-18** by the user, on the review bundle's `03-drawings/` (item 47; the answers are `e4c0eaf6`),
verbatim: *"(a)"* **(a)**, the default as built. Recorded by S1f.23.6; the decision
[Q-M1f.467](../../docs/decisions/docs/Q-M1f.467-a-commands-drawing-on-a-page-is-its-output.md).

The default had been (a). Built: `231486f0` (the merge of task 2, `1d858bb1`).

### Q-M1f.468 — A page's KB drawing: the whole view or a cut

**Raised** 2026-11-18 by S1f.23.1's task 2, the hand-written drawings and the pages. **Ruled 2026-11-18.**

- **(a)** a KB drawing may be a cut of the view: the facts of the relations the section is about and every fact they point at, a framed fact kept only where a kept fact points at it, the page saying what is left out; Q-M1f.368's *kept as the file the command wrote* read of a whole view. **Default.**
- **(b)** only whole views, however large.

**Ruled 2026-11-18** by the user, on the review bundle's `03-drawings/` (item 48; the answers are `e4c0eaf6`),
verbatim: *"(a)"* **(a)**, the default as built. Recorded by S1f.23.6; the decision
[Q-M1f.468](../../docs/decisions/docs/Q-M1f.468-a-pages-drawing-of-a-knowledge-base-may-be-a-cut.md).

The default had been (a). Built: `231486f0` (the merge of task 2, `1d858bb1`).

### Q-M1f.469 — How a derivation's drawing marks a terminal and the core

**Raised** 2026-11-18 by S1f.23.1's task 2, the hand-written drawings and the pages. **Ruled 2026-11-18.**

- **(a)** a terminal double-bordered, the core red, where both were filled before (no octagon is filled, Q-M1f.398). **Default.**
- **(b)** another mark, or none.

**Ruled 2026-11-18** by the user, on the review bundle's `03-drawings/` (item 49; the answers are `e4c0eaf6`),
verbatim: *"(a)"* **(a)**, the default as built. Recorded by S1f.23.6; the decision
[Q-M1f.469](../../docs/decisions/docs/Q-M1f.469-a-derivations-drawing-on-a-page-marks-its.md).

The default had been (a). Built: `231486f0` (the merge of task 2, `1d858bb1`).

### Q-M1f.470 — Which hand drawings the canonical form leaves as they are

**Raised** 2026-11-18 by S1f.23.1's task 2, the hand-written drawings and the pages. **Ruled 2026-11-18.**

- **(a)** as Q-M1f.430 for the views, extended to the pages: region, lattice, cube and h-space drawings, states and constructions, dependency graphs, schematics and flowcharts, the design diagrams: none of them a drawing of facts. **Default.**
- **(b)** each drawn with its facts in the canonical form.

**Ruled 2026-11-18** by the user, on the review bundle's `03-drawings/` (item 50; the answers are `e4c0eaf6`),
verbatim: *"(a) canonical drawing is ONLY for Ein KB (objects and facts), everything else can be drawn in any other
best way.
plans/m1f_hypothesis_and_documentation/review/03-drawings/50-Q-M1f.470/renders/06-class-of-two.option-b-facts-drawn.MOCK.svg
- this one looks good, it could be an option for small lattice and small KB, but it is an exception.
plans/m1f_hypothesis_and_documentation/review/03-drawings/50-Q-M1f.470/renders/08-constructions.svg - this also looks
good."* **(a)**, the canonical form for a knowledge base's objects and facts only; the mock of a small lattice with
its states' facts drawn an option for a small lattice over a small knowledge base, an exception (filed to M20 by the
main session, `23539de5`). Recorded by S1f.23.6; the decision
[Q-M1f.470](../../docs/decisions/docs/Q-M1f.470-only-a-knowledge-base-is-drawn-in-the-canonical.md).

The default had been (a). Built: `231486f0` (the merge of task 2, `1d858bb1`).

### Q-M1f.471 — `01-object-order`'s drawing

**Raised** 2026-11-18 by S1f.23.1's task 2, the hand-written drawings and the pages. **Ruled 2026-11-18.**

- **(a)** six unlabelled octagons f1 to f6 in the order's chain, their arrows left out, the page giving each fact in TeX. **Default.**
- **(b)** each fact with its arrows.

**Ruled 2026-11-18** by the user, on the review bundle's `03-drawings/` (item 51; the answers are `e4c0eaf6`),
verbatim: *"(a), their own arrows have nothing related to this ordering example"* **(a)**, the default as built.
Recorded by S1f.23.6; no decision file: one page's drawing.

The default had been (a). Built: `231486f0` (the merge of task 2, `1d858bb1`).

### Q-M1f.472 — Names of the drawings that changed form

**Raised** 2026-11-18 by S1f.23.1's task 2, the hand-written drawings and the pages. **Ruled 2026-11-18.**

- **(a)** `levi` → `canonical`, `compact` → `folded`; `01-is-a-convention` dropped, its paragraph rewritten; the two `12-*-form` drawings merged into `12-query-view`. **Default.**
- **(b)** the old names kept.

**Ruled 2026-11-18** by the user, on the review bundle's `03-drawings/` (item 52; the answers are `e4c0eaf6`),
verbatim: *"(a)"* **(a)**, the default as built. Recorded by S1f.23.6; no decision file: the names of files.

The default had been (a). Built: `231486f0` (the merge of task 2, `1d858bb1`).

### Q-M1f.473 — Programs a page draws that no file holds

**Raised** 2026-11-18 by S1f.23.1's task 2, the hand-written drawings and the pages. **Ruled 2026-11-18.**

- **(a)** reconstructed for the drawing from the page's text: the hrule `guess-drink`, the written-out `bag-is-ready`. **Default.**
- **(b)** a program file added for each, or the drawing dropped.

**Ruled 2026-11-18** by the user, on the review bundle's `03-drawings/` (item 53; the answers are `e4c0eaf6`),
verbatim: *"(a)"* **(a)**, the default as built. Recorded by S1f.23.6; the decision
[Q-M1f.473](../../docs/decisions/docs/Q-M1f.473-a-program-a-page-draws-and-no-file-holds.md).

The default had been (a). Built: `231486f0` (the merge of task 2, `1d858bb1`).

### Q-M1f.474 — An arrow in a hand schematic that is no arrow of the graph

**Raised** 2026-11-18 by S1f.23.1's task 2, the hand-written drawings and the pages. **Ruled 2026-11-18.**

- **(a)** allowed where its page says so (`03-bijective-family`). **Default.**
- **(b)** never.

**Ruled 2026-11-18** by the user, on the review bundle's `03-drawings/` (item 54; the answers are `e4c0eaf6`),
verbatim: *"Allowed as hand-written, for explanations, not render output. For rendering layout see L-100"* **(a)** for
a hand drawing, and never in a command's output; the layout of a command's drawing is L-100's. Recorded by S1f.23.6;
the decision [Q-M1f.474](../../docs/decisions/docs/Q-M1f.474-a-hand-schematic-may-draw-an-arrow-the-graph.md).

The default had been (a). Built: `231486f0` (the merge of task 2, `1d858bb1`).

### Q-M1f.475 — Where a declaration's statement prints on the premises line

**Raised** 2026-11-18 by S1f.23.5's build (task 2), reading 1. **Put, not ruled.** The examples page's open point 3,
ruled as its default (a) by the user's *"defaults for the rest"*, put a declaration's statement on the trace's
premises line; where on the line was left to the build.

- **(a)** right after the declaration, `` `relation(owns, Person, Pet)` — a Person owns a Pet ``, the premises'
  sources still once after the last premise, ` — from <source>`: a line can carry two dashes, the second the
  sources' of the line, and `14-natural-language.md` § 3 says so. **Default.**
- **(b)** the statements gathered after the sources, one dash a line.

**Default taken: (a).** Built: `bd21f0bb` (`TraceStep`'s `premise_statements`), merged by `79dd213b`.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.476 — A declaration a rule concludes

**Raised** 2026-11-18 by S1f.23.5's build (task 2), reading 2. **Put, not ruled.** A rule may conclude a fact of
`relation` whose tuple is a declaration the program wrote.

- **(a)** it reads by the declaration's `:why` on the *Derives* line, as the examples page's open point 10 has it
  (*however a run reaches it*: one object, one text); no corpus program derives one, and the stage's new test does not
  either. **Default.**
- **(b)** it reads as written there, as every other derived fact does, its statement on a premises line alone.

**Default taken: (a).** Built: `bd21f0bb` (`ein_render::sentence::statement`).

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.477 — The bound on a declaration's references

**Raised** 2026-11-18 by S1f.23.5's build (task 2), reading 3. **Put, not ruled.** `ein-ir/tests/template_wellformedness.rs`
held a relation's `:why` to `{?1}` … `{?n}`, n its labels, and admitted any `{?n}` in an untyped declaration's.

- **(a)** `{?2}` to `{?n+2}`, `{?1}` refused there (it always prints `relation`, so a `{?1}` is a text the
  renumbering missed), an untyped declaration bounded to `{?2}`; the engine itself prints a `{?1}` as `relation`
  and a reference past the arrows as written (the examples page's open point 7 (a)); no corpus text is past the
  bound. **Default.**
- **(b)** `{?1}` to `{?n+2}`, every arrow of the form.

**Default taken: (a).** Built: `bd21f0bb`.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.478 — L-102's reach

**Raised** 2026-11-18 by S1f.23.5's build (task 2), reading 4. **Put, not ruled.** [The milestone's findings](findings.md)
L-102 names the functions of `ein_render::dump::serialise` that no command calls.

- **(a)** the six go (`facts_to_ein_text`, `kb_to_ein_text`, `fact_line`, `provenance_comment`, `firing_to_json`,
  `fact_summary`), and with them `sentence.rs`' `fact_sentence`, `FactSentence`, `Origin` and `as_comment`, which
  only that writer read; the shape golden's `states` op prints each fact of a state and its first recorded origin
  (`source "…"`, `rule r`, `hypothesis N`), with no English, the trace's ops holding the rule's filled `:why`.
  **Default.**
- **(b)** the six alone, the `states` op digesting the rule's filled `:why`.

**Default taken: (a).** Built: `fbe2ba66` (454 `states` lines of the shape golden moved).

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.479 — L-105's reach

**Raised** 2026-11-18 by S1f.23.5's build (task 2), reading 5. **Put, not ruled.** [The milestone's findings](findings.md)
L-105: `utils/nl_census.py`'s answer half counts nothing on the saved format.

- **(a)** the answer half retired, and with it the census's copy of the kernel's names, `KERNEL`, so
  `check_tool_mirrors.py` drops its census mirror; the flag half stays (which programs the loader refuses for a
  missing `:why`, `--check`), and `14-natural-language.md` § 4's registers stay held to `ein_core::RESERVED` by
  `natural_language_page.rs`. **Default.**
- **(b)** `KERNEL` and its mirror kept with the flag half.

**Default taken: (a).** Built: `3047bbae`.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.480 — The decisions the ruling rewrites

**Raised** 2026-11-18 by S1f.23.5's build (task 2), reading 6. **Put, not ruled.**

- **(a)** [Q-M1f.58](../../docs/decisions/read-out/Q-M1f.58-a-fact-reads-in-english-in-the-trace.md) rewritten as
  the ruling, its title extended and its slug kept; Q-M1f.61's title now *…each conjunct reads as written*, slug
  kept; Q-M1f.60, Q-M1f.214, Q-M1f.12 and Q31 restated to the rule; Q27's example renumbered; Q-M1f.62, Q-M1f.57
  and Q-M1f.63 stand (the examples page's open points 4 and 5). **Default.**
- **(b)** a decision of its own for the ruling, Q-M1f.58 superseded and its citations rewritten to the new id.

**Default taken: (a).** Built: `bd21f0bb`.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.481 — The pages the ruling rewrites

**Raised** 2026-11-18 by S1f.23.5's build (task 2), reading 7. **Put, not ruled.**

- **(a)** `14-natural-language.md` rewritten: its intro, § 1's row, § 2's house style restated as the sentence a
  declaration reads as (the standard library's and the examples' texts said to read stiffly), § 3's ladder without
  the relation's rung and its block moved from `zebra.ein`, whose trace reads no declaration, to `zebra2.ein`, § 4's
  column *who can give its declaration a text*, §§ 5 to 7; the trace blocks of `04-ontology.md`, `05-rules.md`,
  `07-properties-and-operators.md`, `10-theory.md`, `11-macros.md` and the guide's `05-jack-drinks-coffee.md`
  re-taken from their programs; the prose of `01-objects.md`, `02-relations.md`, `03-facts.md`, `09-hypotheses.md`,
  `02-syntax.md`, `07-diagnostics.md`, `12-rendering.md`, the guide's `01-objects-and-relations.md` and the
  section's README; AGENTS.md's standing rule on `:why`. **Default.**
- **(b)** `14-natural-language.md` and the trace blocks alone, the other pages' prose left to a stage of its own.

**Default taken: (a).** Built: `bd21f0bb`.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.482 — The words of a renumbered `:why`

**Raised** 2026-11-18 by S1f.23.5's build (task 2), reading 8: the examples page's open point 9, ruled as its
default by the user's *"defaults for the rest"*, as built.

- **(a)** the words are not rewritten: the renumbering is mechanical, `{?k}` → `{?k+2}`, and only the page examples
  that show the rule use the user's pattern, `a {?3} {?2} a {?4}`; Q-M1f.58 and `14-natural-language.md` § 2 say the
  shipped texts read stiffly as their declaration. **Default.**
- **(b)** every declaration's text rewritten as a sentence about its declaration, in the user's pattern.

**Default taken: (a).** Built: `bd21f0bb`.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.483 — The region view's words on an arrow

**Raised** 2026-11-18 by S1f.23.1's task 4, reading (i). **Put, not ruled.** The phase's R-31 was a word written as
an arrow's `label` on a link with `constraint=false`; the region view's `sat` and `✗ alive` (`region_dot.rs`) are
words on such arrows.

- **(a)** counted as R-31's pattern and written as `xlabel`s, though Graphviz refused none (7 728 region views of 644
  programs probed); the two drawings under `docs/` that copy the view, `06-class-of-two` and
  `08-region-merged-states`, drawn again. **Default.**
- **(b)** left as `label`s until one is refused.

**Default taken: (a).** Built: `18bc66ac`.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.484 — A trace's slice that Graphviz refuses

**Raised** 2026-11-18 by S1f.23.1's task 4, reading (ii). **Put, not ruled.** The phase's
[R-33](p1f.23_review_processing/findings.md): 169 canonical and 487 folded blocks of the calculator's 36 programs'
traces refused; `rankdir=TB` routes every refused block sampled.

- **(a)** the slice is left as it is and its refusals filed (R-33): what routes them changes how a slice is laid
  out, which [Q-M1f.428](#q-m1f428--how-a-derivation-is-drawn) and
  [Q-M1f.429](#q-m1f429--a-folded-fact-in-a-drawing-of-a-derivation) rule. **Default.**
- **(b)** a slice laid out top to bottom, `rankdir=TB`.

**Default taken: (a).** Nothing built; R-33 stays open, the main session's and the user's.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.485 — What `09-core-in-derivation` draws heavy

**Raised** 2026-11-18 by S1f.23.1's task 4, reading (iii). **Put, not ruled.** The phase's R-28.

- **(a)** the two facts the derivations added, `penwidth=2`, *heavy* meaning *a derivation added it*, as in
  `09-proof-graph` and on the page; the core's three terminals red and double-bordered
  ([Q-M1f.469](#q-m1f469--how-a-derivations-drawing-marks-a-terminal-and-the-core)), not heavy. **Default.**
- **(b)** the core heavy too.

**Default taken: (a).** Built: `d5fceff9`.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.486 — `long_numbers.ein` and the pool's long lane

**Raised** 2026-11-18 by S1f.23.1's task 4, reading (iv). **Put, not ruled.** The phase's R-32: `dot_wellformed` on
the pool, 261.2 s on the runner's four threads, the work bounding it, not one file.

- **(a)** `examples/arith/long_numbers.ein` is not added to `ein_corpus::pool::LONG`: at four workers the order
  units start in cannot shorten a sweep the work bounds. **Default.**
- **(b)** added, so its seven views start first.

**Default taken: (a).** Built: `82e8097b`, which leaves `LONG` as it was.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.487 — How the layers of a knowledge base's drawing are written

**Raised** 2026-11-18 by S1f.23.6's build, reading 1. **Put, not ruled.** L-100 and R-18 were ruled the layered
layout, objects → facts → objects, and the user's note during the stage (verbatim in
[S1f.23.6](p1f.23_review_processing/s1f.23.6_the_drawings_answers.md#the-answers-item-by-item)) asked for a band of
facts several ranks deep where facts nest; how the layers are had was left to the build.

- **(a)** written, from the arrows alone: a fact's first arrow drawn back from its relation, `rankdir=TB`, and in a
  view of a knowledge base `rank=same` groups, objects and facts in turn, a band as deep as its facts nest (the
  note's *"global-only facts"* read as the held facts nothing points at, on a band's top rank, and *"nested"* as a
  fact under another's arrow, held or not, a rank below it); no name read; the groups ordered by a chain of
  invisible anchors; no groups where the arrows go round, nor in a rule's panel, a slice or the derivation graph; a
  relation under no argument arrow stands just above its highest fact, and a flat fact of it just above its names.
  **Default.**
- **(b)** no groups, `dot` ranking by the arrows alone, which breaks the sandwich where facts nest.

**Default taken: (a).** Built: `ad20a5e0` (`ein_render::fact_graph::layers`), merged by `547afea5`; the review's
`03-drawings/`, item 56, shows it beside the drawing by the arrows alone.

**Ruled 2026-11-19** by the user, in the review bundle (item 56 of `03-drawings/`, the answers file at `d124e63b`), verbatim: *"Perfect! (a) But large diagrams still look messy. Does Graphviz have circular/concentric layout? Objects will be outside, then facts, then relations and types opbejcts, then relations definitions and properties facts and in the center `relation` `symmetric` and other, is it possible?"* So: (a) as built; the concentric layout the user asks about is put as examples by [S1f.23.11](p1f.23_review_processing/s1f.23.11_drawings_second_answers.md), task 2, ruled 2026-11-19 as its default, (a), nothing built ([Q-M1f.547](#q-m1f547--a-concentric-layout-the-layers-drawn-as-rings)).

### Q-M1f.488 — A folded arrow: within a layer, or ranked

**Raised** 2026-11-18 by S1f.23.6's build, reading 2. **Put, not ruled.** The stage's continuing brief stated the
target, *folded arrows within a layer*; the first build had ranked them.

- **(a)** within a layer: a folded arrow ranks nothing (`constraint=false`) and its fact places its names as its
  octagon would, so the folded drawing keeps the canonical one's layers of objects; a drawing every fact of which
  folds is one row (`05-row-state`). It reverses the first build's reading. **Default.**
- **(b)** ranked as any arrow, which keeps chains as chains (the first build: zebra2's folded program `kb` six
  layers, the houses a chain by `right-of`).

**Default taken: (a).** Built: `29792a38`, merged by `547afea5`; item 57 puts the two side by side.

**Ruled 2026-11-19** by the user, in the review bundle (item 57 of `03-drawings/`, the answers file at `d124e63b`), verbatim: *"(a) All good, but still, for large diagrams concentric layout maybe can improve visibility."* So: (a) as built; the concentric layout, [S1f.23.11](p1f.23_review_processing/s1f.23.11_drawings_second_answers.md), task 2, ruled 2026-11-19 as its default, (a), nothing built ([Q-M1f.547](#q-m1f547--a-concentric-layout-the-layers-drawn-as-rings)).

### Q-M1f.489 — A folded arrow in a rule's panel and in a slice

**Raised** 2026-11-18 by S1f.23.6's build, reading 3. **Put, not ruled.** R-31: `constraint=false` beside a `label`
is what Graphviz could not route.

- **(a)** a rule's panel and a slice keep a folded arrow's `label` and its ranking (Q-M1f.435 (a)); only a drawing of
  a knowledge base writes a folded arrow within a layer ([Q-M1f.488](#q-m1f488--a-folded-arrow-within-a-layer-or-ranked)).
  **Default.**
- **(b)** within a layer there too, `constraint=false`, its relation an `xlabel`.

**Default taken: (a).** Built: `29792a38`, which leaves the panels and the slices as `ad20a5e0` draws them.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.490 — A stored negation under `--fold`

**Raised** 2026-11-18 by S1f.23.6's build, reading 4. **Put, not ruled.** Q-M1f.425 (b) folds a fact of two arrows
into a loop on its argument.

- **(a)** it folds: `(not g)` is a fact of two arrows and folds into a loop on `g`'s octagon, in the views of a
  knowledge base and, `(not P)`, in a rule's panel; no name is read. **Default.**
- **(b)** a negation never folded, which reads the name `not`.

**Default taken: (a).** Built: `ad20a5e0`; item 58.

**Ruled 2026-11-19** by the user, in the review bundle (item 58 of `03-drawings/`, the answers file at `d124e63b`), verbatim: *"(a)"* So: (a) as built.

### Q-M1f.491 — The closing rules' drawings under `--fold`

**Raised** 2026-11-18 by S1f.23.6's build, reading 5. **Put, not ruled.** With the loop, five closing rules
(`decimal-arith`, `rat-arith`, `binary-arith`, `arith9`, `props`) fall within the drawing bound, `MAX_NODES`,
unchanged (L-97).

- **(a)** drawn folded on the standard library's rule pages, where they were Ein only (493 canonical, 6 folded, 12
  Ein only). **Default.**
- **(b)** a closing rule left Ein only.

**Default taken: (a).** Built: `ad20a5e0`; item 58.

**Ruled 2026-11-19** by the user, in the review bundle (item 58 of `03-drawings/`, the answers file at `d124e63b`), verbatim: *"(a)"* So: (a) as built.

### Q-M1f.492 — The derivation's palette

**Raised** 2026-11-18 by S1f.23.6's build, reading 6. **Put, not ruled.** Q-M1f.428 was ruled with *"more contrast
colors and 2x thicker lines"*; which colours, and how many, was left to the build.

- **(a)** eight dark colours by the rule's name (`slice.rs`' `DERIVATION_PALETTE`); the derivation graph, which
  records no rule, in the first; a step's border `penwidth=2` in place of `style=bold`; a redundant firing dashed at
  the same width. **Default.**
- **(b)** one colour for every derivation.

**Default taken: (a).** Built: `ad20a5e0`; item 59.

**Ruled 2026-11-19** by the user, in the review bundle (item 59 of `03-drawings/`, the answers file at `d124e63b`), verbatim: *"Yes, but don't use black color for highlighting. use both color and thickness, derived fact also colored. same color as derivation but more bright. The added fact is the brightest, then derivation explanation, and then all the rest with default black and default thickness."* So: (a) amended: no colour near black; each rule's hue in two shades, the brighter the derived fact's; built by [S1f.23.11](p1f.23_review_processing/s1f.23.11_drawings_second_answers.md), task 1 (`21728654`, merged by `9f5c0487`); the build ruled 2026-11-19 as its default, (a) as built ([Q-M1f.548](#q-m1f548--a-derivations-three-levels-as-built)).

### Q-M1f.493 — How a derived fact is marked beside a derivation

**Raised** 2026-11-18 by S1f.23.6's build, reading 7. **Put, not ruled.** A derived fact was drawn bold; the
derivation's arrows are now as wide.

- **(a)** it stays bold, `penwidth=2`, its octagon and its arrows, the derivation's width, told from the
  derivation's arrows by colour. **Default.**
- **(b)** the width the derivation's alone.

**Default taken: (a).** Built: `ad20a5e0`; item 59.

**Ruled 2026-11-19** by the user, in the review bundle (item 59 of `03-drawings/`, the answers file at `d124e63b`), verbatim: *"Yes, but don't use black color for highlighting. use both color and thickness, derived fact also colored. same color as derivation but more bright. The added fact is the brightest, then derivation explanation, and then all the rest with default black and default thickness."* So: amended: a derived fact is drawn in its derivation's hue, the brightest shade, and thick; built by [S1f.23.11](p1f.23_review_processing/s1f.23.11_drawings_second_answers.md), task 1 (`21728654`, merged by `9f5c0487`); the build ruled 2026-11-19 as its default, (a) as built ([Q-M1f.548](#q-m1f548--a-derivations-three-levels-as-built)).

### Q-M1f.494 — A step's slice under `--fold`

**Raised** 2026-11-18 by S1f.23.6's build, reading 8. **Put, not ruled.** Q-M1f.429: a fact a derivation's arrow
touches is never folded.

- **(a)** a step's slice under `--fold` is the canonical one: a step's slice has one firing and no commitment, so
  every fact in it is touched by the step's arrows; `--fold` folds a trace's `-F` snapshots. No page says otherwise,
  and across the corpus every trace's folded blocks are its canonical ones. **Default.**
- **(b)** none named by the stage: under Q-M1f.429 a step's slice could fold a fact only by drawing more than its
  step.

**Default taken: (a).** Built: `ad20a5e0`, held by `render_doors.rs` on `02_packing.ein`; item 59.

**Ruled 2026-11-19** by the user, in the review bundle (item 59 of `03-drawings/`, the answers file at `d124e63b`), verbatim: *"Yes, but don't use black color for highlighting. use both color and thickness, derived fact also colored. same color as derivation but more bright. The added fact is the brightest, then derivation explanation, and then all the rest with default black and default thickness."* So: (a) as built: a step's slice the same under `--fold`.

### Q-M1f.495 — How the pages' copies of commands' outputs were drawn again

**Raised** 2026-11-18 by S1f.23.6's build, reading 9. **Put, not ruled.** Q-M1f.468: a page's drawing of a knowledge
base may be a cut.

- **(a)** by a transformer validated against the binary over every corpus program (`review/03-drawings/tools/`'s
  `relayer.py`, then `refold.py`), not by re-running each page's command, so that a cut stays the cut it was; the
  four folded rule drawings with a pattern of two arrows, which it refuses, by the binary from their pages'
  programs; the hand derivations `09-cyclic` and `09-proof-graph` restyled by hand to the derivation's palette and
  width; `reasoning/01-static-ir/`'s hand graphs left. **Default.**
- **(b)** each page's command run again, and a cut cut again from its output.

**Default taken: (a).** Built: `ad20a5e0` and `29792a38`, merged by `547afea5`.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.496 — The `fdp` provisioning, now that no view suggests `fdp`

**Raised** 2026-11-18 by S1f.23.6's build, reading 10. **Put, not ruled.** The layered layout dropped the KB view's
`// suggested layout: fdp`, so the layout plugin guards no drawing the gate makes; `render_examples.sh` alone still
picks `fdp`, for files named `*ontology*` or `*constraints*`.

- **(a)** left as it is: the CI's `libgvplugin-neato-layout8`, `run_tests.sh`'s exit 127 where `fdp` lays out
  nothing, and the comments in `AGENTS.md`, the two workflows, `run_tests.sh`, `utils/ci_container.sh`,
  `utils/review_bundle.py` and `utils/review/util.py` that say every KB view suggests `fdp`, which are wrong from the
  stage on. **Default.**
- **(b)** the comments brought to the layered layout, and what CI installs and `run_tests.sh`'s check taken again.

**Default taken: (a).** Nothing built: what CI installs, and the comments, the stage left to the main session.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.497 — Where R-33's check lives, and what it reads

**Raised** 2026-11-18 by S1f.23.6's build, reading 11. **Put, not ruled.** The phase's
[R-33](p1f.23_review_processing/findings.md): no test put a trace's drawing to Graphviz, and the probe over every
trace laid out 154 062 distinct blocks, past any per-commit budget.

- **(a)** in `dot_wellformed`, per commit and bounded: `graphviz_accepts_every_slice_of_the_witnesses_traces` puts
  every slice of two witnesses' traces to `dot`, `examples/arith/div.ein` and
  `examples/binary_arithmetic/first.ein`, the smallest programs a canonical and a folded slice of which were
  refused, 611 slices; the test is the authority the DOT views have on being well-formed, and the user's rule leaves
  out the layout only. **Default.**
- **(b)** no check, the probe a script run by hand.
- **(c)** every trace's blocks.

**Default taken: (a).** Built: `ad20a5e0`; item 60.

**Ruled 2026-11-19** by the user, in the review bundle (item 60 of `03-drawings/`, the answers file at `d124e63b`), verbatim: *"(a)"* So: (a) as built.

### Q-M1f.498 — Whether a page pins the layers

**Raised** 2026-11-18 by S1f.23.6's build, reading 12. **Put, not ruled.** L-100 was ruled with *"No tests required
here!"*, the layered layout the user's preference.

- **(a)** no page pins a `rank`: the rendering page says the layers are a preference no test holds, and the decision
  Q21 carries the user's rule that a test holds structure and never layout. **Default.**
- **(b)** the rendering page states the layers as what a drawing is held to.

**Default taken: (a).** Built: `ad20a5e0` (`12-rendering.md` § Layers, Q21 amended).

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.499 — What a reader of `einb/2.0` makes of a node's container

**Raised** 2026-11-18 by S1f.23.8's build, reading 1. **Put, not ruled.** Q-M1f.413 (c) adds the section `COMMIT`,
kind 13; by `09-binary-format.md` § 4.1 and Q-M1f.10's *why*, a section kind added moves the minor, and a reader of
the same major skips a kind it does not know.

- **(a)** the minor, `einb/2.1`, as the page's rule says: a 2.0 reader, a binary built before the stage, skips
  `COMMIT` and opens a node's container as its program's root, answering the program's R; no 2.0 writer wrote a
  node's container (`kb save` refused one), so a root's container is read right both ways. **Default.**
- **(b)** the major, 3.0: every 2.x container refused with the version line, a cache to save again, and no older
  binary ever opens a node.

**Default taken: (a).** Built: `86d46410` (`FORMAT_MINOR = 1`), merged by `aad59286`. The stage leaves it to the user
if the gap matters.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.500 — What *a root's container is unchanged* means

**Raised** 2026-11-18 by S1f.23.8's build, reading 2. **Put, not ruled.** The stage's brief: *a root's container
stays as it is*.

- **(a)** the same sections, byte for byte, and no `COMMIT`; its header's minor reads 1. **Default.**
- **(b)** its header too: a root's container written at 2.0, the minor moved only where a `COMMIT` is written.

**Default taken: (a).** Built: `86d46410`, held by `einb_cli.rs`' `a_nodes_container_opens_as_the_node`.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.501 — The words a node's container prints

**Raised** 2026-11-18 by S1f.23.8's build, reading 3. **Put, not ruled.** The review's mock of Q-M1f.413 (c) wrote
*container* for *saved file* in the `commit` row and the NOT CHECKED lines.

- **(a)** the saved file's words: the container's contract is its file's output byte for byte but the path
  (Q-M1a.22). **Default.**
- **(b)** the mock's, *container*.

**Default taken: (a).** Built: `86d46410`.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.502 — A load note's position in a node's container

**Raised** 2026-11-18 by S1f.23.8's build, reading 4. **Put, not ruled.** A position in a container names its stored
program (Q-M1f.15; F23's D-18).

- **(a)** the stored program's, as for every container: `answer_round_trip`'s `by_its_container` compares a
  position as `<position>`, and the pages say so beside the byte-for-byte promise; one corpus node printed one,
  `features/13_mixed_solution_and_open.ein`'s mechanism note. **Default.**
- **(b)** the saved file's own position, which a container does not store (Q-M1f.15: a format change).

**Default taken: (a).** Built: `86d46410`.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.503 — A node's container and the standard library's digest

**Raised** 2026-11-18 by S1f.23.8's build, reading 5. **Put, not ruled.** Found by the review's item 17-Q-M1f.413,
§ 7: a saved file's container records the environment's stdlib digest, as every container does, so a later run under
another library warns `… was written under different inputs (StaleStdlib)`, which the saved file, comparing nothing
since D-24, does not; the answer is the same (a loaded container drops nothing).

- **(a)** as is: the container's freshness is the cache's own, and a root's container is left unchanged.
  **Default.**
- **(b)** a saved file's container records no digest, and is compared with nothing.

**Default taken: (a).** Nothing built. The stage marks it the user's.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.504 — `ein kb save` of a node whose choice is no candidate

**Raised** 2026-11-18 by S1f.23.8's build, reading 6. **Put, not ruled.**

- **(a)** as is: `ein kb save` saturates nothing and checks no commitment against H, so a node whose choice is no
  candidate is stored, and refused by every command that reads the node, as the saved file is. **Default.**
- **(b)** `ein kb save` builds H and refuses such a node itself.

**Default taken: (a).** Built: `86d46410`.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.505 — `ein kb save --saturate` of a node

**Raised** 2026-11-18 by S1f.23.8's build, reading 7. **Put, not ruled.**

- **(a)** as is: it banks root's fixpoint and the commitment, not the node's state; the run commits the node on
  open, as for the saved file. **Default.**
- **(b)** it banks the node's state.

**Default taken: (a).** Built: `86d46410`.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.506 — A bad commitment under `ein test`

**Raised** 2026-11-18 by S1f.23.8's build, reading 8. **Put, not ruled.** Q-M1f.416 (b): no run at all.

- **(a)** as built: with no run H is never built, so a `(commit …)` naming no candidate, or one twice, is NOT CHECKED
  with exit 0 under `ein test`, where it was an ERROR, exit 2, under (a); `solve`, `oracle`, `saturate` and `render
  kb` still refuse it, and `07-diagnostics.md` § 2.30 says `ein test` refuses neither. **Default.**
- **(b)** `ein test` builds H to check the commitment, refuses a bad one, exit 2, and otherwise runs nothing.

**Default taken: (a).** Built: `86d46410` (`test.rs`' `NotRun`).

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.507 — `--json-summary` and `--events` under `ein test` of a node

**Raised** 2026-11-18 by S1f.23.8's build, reading 9. **Put, not ruled.** The ruling's (b) left them unsaid.

- **(a)** they write nothing, as for a file that states no claim. **Default.**
- **(b)** each writes a record that names the node and no run.

**Default taken: (a).** Built: `86d46410`.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.508 — The `-v` header of a saved node's claims

**Raised** 2026-11-18 by S1f.23.8's build, reading 10. **Put, not ruled.**

- **(a)** it names no question: the parenthesis names the question an outcome was read off, and the report row the
  header is rebuilt from has no field to tell a node's row from a goal-less query's. **Default.**
- **(b)** the review's mock, *(a saved file's node: not run)*.

**Default taken: (a).** Built: `86d46410`.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.509 — The report's row of a claim no run checked

**Raised** 2026-11-18 by S1f.23.8's build, reading 11. **Put, not ruled.** The review's mock wrote `outside` `[]` and
`region: null`.

- **(a)** `outside` `null`, and a solve row with no `region` key: both the shape of a row no run checked today (`[]`
  would say H leaves nothing out, which only a run knows), so no key moves and the id stays `ein-test-report/6`, as
  Q-M1f.259 keeps an id; `06-config.md` unchanged. **Default.**
- **(b)** the mock's, `outside` `[]` and `region: null`.

**Default taken: (a).** Built: `86d46410`.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.510 — `claim::check`'s saved-file branch

**Raised** 2026-11-18 by S1f.23.8's build, reading 12. **Put, not ruled.**

- **(a)** unchanged: `ein test` does not call it, and a library caller checking a node's claim gets NOT CHECKED as
  `ein solve` does. **Default.**
- **(b)** taken out, no command calling it for a saved file.

**Default taken: (a).** Nothing built.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.511 — Decision files for Q-M1f.415 to .418

**Raised** 2026-11-18 by S1f.23.8's build, reading 13. **Put, not ruled.**

- **(a)** none of their own: .416's answer is Q-M1f.403's file, .415's and .417's Q-M1f.402's, .418's Q-M1f.185's;
  .413 has its own (`engine/`). **Default.**
- **(b)** a file each.

**Default taken: (a).** Built: `86d46410` and the ledger `d07eb5bb`.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.512 — Where a node's container is held to its saved file, and its cost

**Raised** 2026-11-18 by S1f.23.8's build, reading 14. **Put, not ruled.** `answer_round_trip`'s `by_its_container`
costs one `kb save` and two `ein solve`s a program, on its last node, 164 nodes per commit; it was not measured
against the nightly's five minutes, L-106's (a) being today's shards.

- **(a)** kept in `answer_round_trip`, beside the dump it reads; the merged tree's nightly re-takes the shards.
  **Default.**
- **(b)** a test of its own.

**Default taken: (a).** Built: `86d46410`.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.513 — What tells a numeral to check from one an operation made

**Raised** 2026-11-19 by S1f.23.9's build, reading 1. **Put, not ruled.** Q-M1f.405 was ruled with the user's
distinction: a numeral an operation makes from valid numerals is valid as made, every other one is checked. The
calculator cannot tell the two by reading a number: only its label does, and a rule that read the label beside the
number would join two facts on a number.

- **(a)** the relation it is written in: each tagged call has a walk of its own behind it; the calculator writes the
  walks, anything outside it the calls; a call asks the check of what it is handed and hands itself on to its walk,
  which checks nothing. **Default.**
- **(b)** one relation, every start asking the check (Q-M1f.405's (e)), which walks again every number the
  calculator hands itself, as the ruling excludes.

**Default taken: (a).** Built: `90e88122` (`std.math.positional`'s fifteen rules, three a call; `std.math.int`'s,
`std.math.rat`'s and `std.math.gcd`'s calls), merged by `04433406`.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.514 — The names of the calls and of the walks

**Raised** 2026-11-19 by S1f.23.9's build, reading 2. **Put, not ruled.**

- **(a)** the interface keeps its names: the calls stay `add-walk` … `gcd-call` (Q-M1f.189, Q-M1f.228), a program's
  text unchanged, and the walks take the new names, `add-run` … `gcd-run`. **Default.**
- **(b)** the walks keep the old names, and every call a program writes is renamed.

**Default taken: (a).** Built: `90e88122`.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.515 — The public division: a call or a walk

**Raised** 2026-11-19 by S1f.23.9's build, reading 3. **Put, not ruled.** Every other request's start writes its walk.

- **(a)** a call: `div-start` writes `(?DIV-CALL (?QUOTIENT X Y) U V)`, which is handed on, so the call's
  division-by-zero flag, `(division-by-zero (div-call (quotient X Y) U V))`, names a fact the state holds, as
  Q-M1f.211's *why* says; its two asks find their facts held; the cost is the division's +3 firings a request.
  **Default.**
- **(b)** `div-start` writes `div-run`, and the call's flag names a `(div-call …)` no state holds.

**Default taken: (a).** Built: `90e88122`.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.516 — What the division-by-zero flag names

**Raised** 2026-11-19 by S1f.23.9's build, reading 4. **Put, not ruled.**

- **(a)** the call, `(division-by-zero (div-call …))`, whatever made the division; the calculator's own divisions
  divide by nothing that is zero; every flag's text as before. **Default.**
- **(b)** the walk the division runs in, `(div-run …)`.

**Default taken: (a).** Built: `90e88122`.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.517 — What an integer's or a rational's call is checked as

**Raised** 2026-11-19 by S1f.23.9's build, reading 5. **Put, not ruled.**

- **(a)** as an operand of its kind, `(int-check (int P N))` and `(rat-check (rat (int P N) D))`, so a digit or a name
  a call is handed as a part is the shape rules' flag (`not-integer`, `not-rational`), where it was silent.
  **Default.**
- **(b)** a rule a part asking `std.math.decimal`'s check only: fourteen rules for `std.math.int` and fifteen for
  `std.math.rat`, and a digit left silent.

**Default taken: (a).** Built: `90e88122` (`int-norm-runs` and its siblings).

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.518 — The calculator's own constants

**Raised** 2026-11-19 by S1f.23.9's build, reading 6. **Put, not ruled.**

- **(a)** not checked: `(n z 1)` in `std.math.int`'s adjustments and `(n z 2)` in `std.math.convert`'s halving are
  written by the calculator's own rules into its own walks. **Default.**
- **(b)** checked as any numeral no operation made.

**Default taken: (a).** Built: `90e88122`.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.519 — The decisions the numerals' rulings live in

**Raised** 2026-11-19 by S1f.23.9's build, reading 7. **Put, not ruled.**

- **(a)** Q-M1f.404's file states the rulings on Q-M1f.405 to .411 and L-101, and Q-M1f.163, .210, .228, .276 and
  .277 are brought to them; no decision file of their own, the code citing Q-M1f.404. **Default.**
- **(b)** a file each for Q-M1f.405, .408 and .409.

**Default taken: (a).** Built: `90e88122`.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.520 — The walk's words in binary

**Raised** 2026-11-19 by S1f.23.9's build, reading 8. **Put, not ruled.** The walk moved into
`std.math.positional` (Q-M1f.408 (b)).

- **(a)** the base's words: `numeral-step`'s `:why` says *the digit {?a} is one* in binary too, as
  `std.math.positional`'s other rules do. **Default.**
- **(b)** a word each base passes, binary's *bit*.

**Default taken: (a).** Built: `90e88122`.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.521 — What the tagged-call test holds

**Raised** 2026-11-19 by S1f.23.9's build, reading 9. **Put, not ruled.** `arith_theory.rs`'
`the_tagged_calls_keep_their_shapes` held the tagged calls (Q-M1f.189).

- **(a)** the walks the calls are handed on to as well, by a table `RUNS`, a call and its walk a row: each walk
  declared with its call's types, and a caller that starts one writing the call's start values; what is started
  somewhere is now `add-run`, `sub-run`, `cmp-run`, `bin-add-run` and `bin-sub-run`, and the calls `add-walk` and
  `cmp-walk` from `examples/beyond_arith/`. **Default.**
- **(b)** the calls alone, as before the walks were split off.

**Default taken: (a).** Built: `90e88122`.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

### Q-M1f.522 — A relation is any atom first in a tuple

**Raised** as P1f.21's [V-7](p1f.21_verdicts_as_rules/findings.md#v-7--a-rule-can-conclude-a-fact-of-a-relation-the-program-does-not-count)
(from P1f.22's N-22): a rule that takes a relation as a parameter, activated with a name that no declaration and no
written fact heads, concludes facts of it, and the engine's four lists of the program's relations disagree on it
(the registry, the alive-set invariant's baseline, the goal's list, the saturation page's $\operatorname{rels}(P)$).
Put to the user in the review bundle (`04-language-and-decisions/08-V-7/`): (a) the registry takes it, (b) the
loader refuses the activator, (c) a page says such a relation is declared.

**Ruled 2026-11-19** by the user, verbatim (`b0e9c412`): *"What does mean "program does not count relation" ?
relation is almost any atom if it is 1st in the tuple. Also for nested facts. Its enough to be the first item in the
tuple to became a relation. (XXX) - is a relation fact with relation 'XXX'. Relation declaration `(relation =
Formula Formula)` is optional fact that can be used to handle some random labels associated with the relation, even
maybe not arguments types, not arguments count. It is just by convention is "relation declaration", that is used to
reason about arguments domains. You easily can use `(rel-arg-types <relation_name> Type Type)`, `(rel-arg-types
<relation_name> (list Type Type))`, `(defrel <relation_name> Type Type)` WHATEVER for the same purpose. But
`relation` is already used by SOME OF stdlib modules. That is it."*

**Read as (a), whole**: an atom is a relation of the program where it is the first item of a tuple the program
writes, at any depth, a stated fact's, a nested fact's, a pattern's of a rule, an hrule or an obligation, a constant
head or one an activator binds; a declaration is an optional fact, a convention some stdlib modules use, and no
relation of the program depends on one. *"Almost"*: the forms the kernel reads as forms are not made relations by
heading a tuple; the build lists them. *"not arguments count"* decides nothing of the argument-count check, which
stays open ([Q-M1f.447](#q-m1f447--the-loaders-argument-count-check-once-relation-is-a-convention)).

**Built** by [S1f.23.10](p1f.23_review_processing/s1f.23.10_language_answers.md), task 2 (`fdbb71ae`), filed as
[`docs/decisions/language/Q-M1f.522-…`](../../docs/decisions/language/Q-M1f.522-a-relation-of-the-program-is-any-atom-it-writes.md):
`ein_ir::from_ir::relations_written` is the one computation, every atom the program writes first in a tuple outside
its goals and `(solve …)` lists (a stated fact's, a nested fact's, a rule's, an hrule's or an obligation's pattern's
at any depth, a declared relation, a name an activator gives a parameter that heads a form); the registry takes it
less the kernel's words (`ein_core::terms::heads_a_relation`: `RESERVED` and the `#` names; `=` is a relation), the
goal's and claim's list is it with `eq` and `neq`, as before, and the alive-set invariant's baseline is the registry
with the kernel's words. The blind rung takes no registered relation and no relation of a fact root's fixpoint holds;
the read-out's `relation names` row, its JSON keys and `ein_infer::relation_names` are gone, no schema id moving.
**What moved**, by the stage's census of every corpus program's R and H: `examples/features/01_not_and_absent.ein`
(H 35 → 19, its 16 candidates over `explicitly-dislikes` gone; flagged and past the limit, 64 655 states met → 65 299)
and `examples/features/09_adjacent_via_same_house.ein` (H 82 → 38, R the same); nothing else, no refusal, and no
`:expect` stated the old answer. The V-7 witness `v7_blind.ein`: 8 states → 1. The readings the stage took, each with
its default, are in its Record.

### Q-M1f.523 — The slice by a query: its definition

**Raised** 2026-11-18 by S1f.23.7's examples, [open point 1](p1f.23_review_processing/s1f.23.7_examples.md#6-open-points-for-the-user). **Ruled 2026-11-19: the default**, the user verbatim: *"S1f.23.7 - I accept"*: every fact a knowledge base holds with a relation of the program directly under an argument arrow (U (c)); not the constraint view's selection as it is, nor readings (a) or (b), nor at any depth. To be built by [S1f.23.7](p1f.23_review_processing/s1f.23.7_a_slice_by_a_query.md), task 2.

### Q-M1f.524 — The slice by a query: what *a relation* is

**Raised** 2026-11-18 by S1f.23.7's examples, [open point 2](p1f.23_review_processing/s1f.23.7_examples.md#6-open-points-for-the-user). **Ruled 2026-11-19: the default**, the user verbatim: *"S1f.23.7 - I accept"*: a relation of the program, one notion with the goal's and the claim's (Q-M1f.119, and Q-M1f.522 since the user's V-7); not a state's relations, nor the declared ones only. To be built by [S1f.23.7](p1f.23_review_processing/s1f.23.7_a_slice_by_a_query.md), task 2.

### Q-M1f.525 — The slice by a query: an atom that is a relation and an object

**Raised** 2026-11-18 by S1f.23.7's examples, [open point 3](p1f.23_review_processing/s1f.23.7_examples.md#6-open-points-for-the-user). **Ruled 2026-11-19: the default**, the user verbatim: *"S1f.23.7 - I accept"*: selected as the graph has it, `z` in a numeral and `eq` in an order among them; `eq` and `neq` not left out. To be built by [S1f.23.7](p1f.23_review_processing/s1f.23.7_a_slice_by_a_query.md), task 2.

### Q-M1f.526 — The slice by a query: depth

**Raised** 2026-11-18 by S1f.23.7's examples, [open point 4](p1f.23_review_processing/s1f.23.7_examples.md#6-open-points-for-the-user). **Ruled 2026-11-19: the default**, the user verbatim: *"S1f.23.7 - I accept"*: directly, the fact's arrows 2, 3, …; not at any depth. To be built by [S1f.23.7](p1f.23_review_processing/s1f.23.7_a_slice_by_a_query.md), task 2.

### Q-M1f.527 — The slice by a query: what is drawn with it

**Raised** 2026-11-18 by S1f.23.7's examples, [open point 5](p1f.23_review_processing/s1f.23.7_examples.md#6-open-points-for-the-user). **Ruled 2026-11-19: the default**, the user verbatim: *"S1f.23.7 - I accept"*: what its facts point at, at any depth, a fact held nowhere in its frame; not the selected facts alone. To be built by [S1f.23.7](p1f.23_review_processing/s1f.23.7_a_slice_by_a_query.md), task 2.

### Q-M1f.528 — The slice by a query: a fact under an arrow, held nowhere

**Raised** 2026-11-18 by S1f.23.7's examples, [open point 6](p1f.23_review_processing/s1f.23.7_examples.md#6-open-points-for-the-user). **Ruled 2026-11-19: the default**, the user verbatim: *"S1f.23.7 - I accept"*: selected only through a held fact that points at it. To be built by [S1f.23.7](p1f.23_review_processing/s1f.23.7_a_slice_by_a_query.md), task 2.

### Q-M1f.529 — The slice by a query: how a goal is given

**Raised** 2026-11-18 by S1f.23.7's examples, [open point 7](p1f.23_review_processing/s1f.23.7_examples.md#6-open-points-for-the-user). **Ruled 2026-11-19: the default**, the user verbatim: *"S1f.23.7 - I accept"*: `ein render kb FILE --goal '(…)'`, of root or a saved state, with `--query N` beside it; not the form only, nor a saved state only. To be built by [S1f.23.7](p1f.23_review_processing/s1f.23.7_a_slice_by_a_query.md), task 2.

### Q-M1f.530 — What `ein render constraints` becomes

**Raised** 2026-11-18 by S1f.23.7's examples, [open point 8](p1f.23_review_processing/s1f.23.7_examples.md#6-open-points-for-the-user). **Ruled 2026-11-19: the default**, the user verbatim: *"S1f.23.7 - I accept"*: the slice of Q-M1f.523 on root's saturated knowledge base, defined in the engine and drawn by the KB view's code, with no language change; not two computed relations, nor the view kept, nor removed for `--goal`. To be built by [S1f.23.7](p1f.23_review_processing/s1f.23.7_a_slice_by_a_query.md), task 2.

### Q-M1f.531 — A written check in the constraint view

**Raised** 2026-11-18 by S1f.23.7's examples, [open point 9](p1f.23_review_processing/s1f.23.7_examples.md#6-open-points-for-the-user). **Ruled 2026-11-19: the default**, the user verbatim: *"S1f.23.7 - I accept"*: `(eq a w)` left out, as the program view leaves it (R-40); not drawn as a fact. To be built by [S1f.23.7](p1f.23_review_processing/s1f.23.7_a_slice_by_a_query.md), task 2.

### Q-M1f.532 — The class of an `(or …)` goal

**Raised** 2026-11-18 by S1f.23.7's examples, [open point 10](p1f.23_review_processing/s1f.23.7_examples.md#6-open-points-for-the-user). **Ruled 2026-11-19: the default**, the user verbatim: *"S1f.23.7 - I accept"*: a class lists the facts of the disjuncts that hold, and its `as :expect` parses back (R-39). To be built by [S1f.23.7](p1f.23_review_processing/s1f.23.7_a_slice_by_a_query.md), task 2.

### Q-M1f.533 — The kernel's words: no relation by heading a tuple

**Raised** 2026-11-19 by S1f.23.10's build, task 2, [reading 1](p1f.23_review_processing/s1f.23.10_language_answers.md#readings-taken-each-with-its-default). Q-M1f.522 makes a relation of *"almost any atom"* first in a tuple; which atoms stay the kernel's words, and head a tuple without making a relation of the program, was left to the build. **Ruled 2026-11-19: the default**, the user verbatim: *"Go with all defaults to the end of this phase p1f.23 and stop"*: `ein_core::terms::RESERVED` (`__closed__`, `absent`, `and`, `eq`, `false`, `neq`, `not`, `open`, `or`, `relation`) and the loader's `#` names, read by one function, `heads_a_relation`; `=`, which `Kernel` interns for the grammar's sake (`Kernel::equals`), is a relation, since the kernel reads no form of it, the user's answer writes `(relation = Formula Formula)` and `06_double_negation.ein` declares it; the other declarator words (`rule`, `query`, …) never head a tuple, the grammar reading them as no name. Not `=` among the kernel's words. Built: `fdbb71ae`, merged by `cbb9635b`.

### Q-M1f.534 — The goal's and claim's list and the kernel's words

**Raised** 2026-11-19 by S1f.23.10's build, task 2, [reading 2](p1f.23_review_processing/s1f.23.10_language_answers.md#readings-taken-each-with-its-default). The registry and the goal's and claim's list (Q-M1f.114, Q-M1f.119) are read off one computation, `relations_written`; whether the list keeps the kernel's words a program writes, which the registry leaves out, was left to the build. **Ruled 2026-11-19: the default**, the user verbatim: *"Go with all defaults to the end of this phase p1f.23 and stop"*: the list is the written set and `eq`, `neq`, unchanged from before, so a region claim listing a state's `(__closed__ r)` loads as it did and no goal, claim or `(solve …)` list is refused or let in that was not (the stage's census: no refusal moved); the registry, which decides what is an object, leaves the kernel's words out. Not the registry's set. Built: `fdbb71ae`, merged by `cbb9635b`.

### Q-M1f.535 — A `(solve :hrules …)` activator's relation

**Raised** 2026-11-19 by S1f.23.10's build, task 2, [reading 3](p1f.23_review_processing/s1f.23.10_language_answers.md#readings-taken-each-with-its-default). Q-M1f.522 makes a relation of every atom an activator gives a parameter that heads a form; an activator a `(solve :hrules …)` list writes is no fact of the program. **Ruled 2026-11-19: the default**, the user verbatim: *"Go with all defaults to the end of this phase p1f.23 and stop"*: as before, `att` of `(guess (att Color House))` stays out of the registry and of the solve lists' set, as Q-M1f.114 says (*outside the three lists*), and stays a relation a goal and a claim may name. Built: `fdbb71ae`, merged by `cbb9635b`.

### Q-M1f.536 — A declared relation that no tuple heads

**Raised** 2026-11-19 by S1f.23.10's build, task 2, [reading 4](p1f.23_review_processing/s1f.23.10_language_answers.md#readings-taken-each-with-its-default). Under Q-M1f.522 a declaration is an optional fact, a convention; whether it alone still makes a relation of the program was left to the build. **Ruled 2026-11-19: the default**, the user verbatim: *"Go with all defaults to the end of this phase p1f.23 and stop"*: as before, a declared relation stays a relation of the program with no tuple written, the blind rung's eligible relations being the declared ones; whether a declaration should count once `relation` is a convention is P1f.21's, with the argument-count check ([Q-M1f.447](#q-m1f447--the-loaders-argument-count-check-once-relation-is-a-convention)). Built: `fdbb71ae`, merged by `cbb9635b`.

### Q-M1f.537 — A relation only root's fixpoint shows

**Raised** 2026-11-19 by S1f.23.10's build, task 2, [reading 5](p1f.23_review_processing/s1f.23.10_language_answers.md#readings-taken-each-with-its-default). The registry is fixed at load, from the text; a relation no text writes first in a tuple, a parameter a guard binds (`(rule pick () :match (eq ?y glad) :assert (tag ?y))`), heads facts root's fixpoint holds. **Ruled 2026-11-19: the default**, the user verbatim: *"Go with all defaults to the end of this phase p1f.23 and stop"*: no object: `hypgen::candidate_objects` takes no registered relation and no atom that heads a fact root's fixpoint holds (`Kb::name_as_head`), held by the unit test `candidate_objects_hold_no_relation`; the registry does not take it, so `alive_invariant.rs` would report it as a new relation where a describe meets one (no corpus program has such a rule: the census reads the same with and without the line). Built: `fdbb71ae`, merged by `cbb9635b`.

### Q-M1f.538 — The `relation names` row, its keys and its module

**Raised** 2026-11-19 by S1f.23.10's build, task 2, [reading 6](p1f.23_review_processing/s1f.23.10_language_answers.md#readings-taken-each-with-its-default). Once no candidate has a relation as an argument (Q-M1f.522, Q-M1f.537), the read-out's `relation names` row can print nothing. **Ruled 2026-11-19: the default**, the user verbatim: *"Go with all defaults to the end of this phase p1f.23 and stop"*: removed, not kept for a case that cannot arise: the row, `ein_infer::relation_names`, `RootStats::relation_names`, `--json-summary`'s `root.hypgen.relation_names_as_objects.*`, `region.json`'s `relation_names_as_objects`, `region.md`'s *Relation names as objects* line and `ein-cli/tests/relation_names_row.rs`, with `08-read-out.md`'s row, `10-answer-dump.md`'s three rows and `11-region-dump.md`'s two mentions; the three keys were written only where non-empty, so no schema id moves (Q-M1f.412's (a)). Not kept. Built: `fdbb71ae`, merged by `cbb9635b`.

### Q-M1f.539 — A premise's own node in a derivation's drawing

**Raised** 2026-11-19 by S1f.23.11's build, task 1, [reading 1](p1f.23_review_processing/s1f.23.11_drawings_second_answers.md#readings-each-with-its-default). The user's answer on item 59 (Q-M1f.492, .493) draws a derivation at three levels, the fact it adds, its explanation and the rest; whether a premise's own node is explanation or rest was left to the build. **Ruled 2026-11-19: the default**, the user verbatim: *"Go with all defaults to the end of this phase p1f.23 and stop"*: the rest, black at the default width; its arrow into the step is the explanation; a premise another step of the drawing added keeps that step's bright shade. Not the premises as explanation (item 62's (c), [Q-M1f.548](#q-m1f548--a-derivations-three-levels-as-built)). Built: `21728654`, merged by `9f5c0487`.

### Q-M1f.540 — A commitment's red and a negation's grey beside the three levels

**Raised** 2026-11-19 by S1f.23.11's build, task 1, [reading 2](p1f.23_review_processing/s1f.23.11_drawings_second_answers.md#readings-each-with-its-default). **Ruled 2026-11-19: the default**, the user verbatim: *"Go with all defaults to the end of this phase p1f.23 and stop"*: kept beside the three levels, since they say what a state assumed and what is denied, not how a fact was derived; a derived negation takes its step's bright shade (it was grey and bold), the added fact being the brightest whatever its relation. Built: `21728654`, merged by `9f5c0487`.

### Q-M1f.541 — The widths of a derivation's two emphasised levels

**Raised** 2026-11-19 by S1f.23.11's build, task 1, [reading 3](p1f.23_review_processing/s1f.23.11_drawings_second_answers.md#readings-each-with-its-default). **Ruled 2026-11-19: the default**, the user verbatim: *"Go with all defaults to the end of this phase p1f.23 and stop"*: the fact a step added at `penwidth=3` (`DERIVED_WIDTH`), the explanation at `penwidth=2`, so the order is told by brightness and width both. Not one width (item 62's (b), [Q-M1f.548](#q-m1f548--a-derivations-three-levels-as-built)). Built: `21728654`, merged by `9f5c0487`.

### Q-M1f.542 — A redundant firing's conclusion

**Raised** 2026-11-19 by S1f.23.11's build, task 1, [reading 4](p1f.23_review_processing/s1f.23.11_drawings_second_answers.md#readings-each-with-its-default). **Ruled 2026-11-19: the default**, the user verbatim: *"Go with all defaults to the end of this phase p1f.23 and stop"*: a redundant firing adds nothing (`Firing::derived` holds facts already there), so its conclusion is the rest, where it was bold; its step stays dashed in the deeper shade, and its arrows are the explanation. Not bright as any derived fact (item 62's (d), [Q-M1f.548](#q-m1f548--a-derivations-three-levels-as-built)). Built: `21728654`, merged by `9f5c0487`.

### Q-M1f.543 — The refutation's arrows

**Raised** 2026-11-19 by S1f.23.11's build, task 1, [reading 5](p1f.23_review_processing/s1f.23.11_drawings_second_answers.md#readings-each-with-its-default). **Ruled 2026-11-19: the default**, the user verbatim: *"Go with all defaults to the end of this phase p1f.23 and stop"*: red: the core's arrows to `⊥` were, and the learned no-good's, black at width 2 before, are red now, so nothing emphasised is black (library only: no CLI path draws a `⊥` slice). Built: `21728654`, merged by `9f5c0487`.

### Q-M1f.544 — The derivation graph's added fact

**Raised** 2026-11-19 by S1f.23.11's build, task 1, [reading 6](p1f.23_review_processing/s1f.23.11_drawings_second_answers.md#readings-each-with-its-default). The derivation graph (`derivation_dag_to_dot`, in the first hue, since it records no rule) has no step that adds. **Ruled 2026-11-19: the default**, the user verbatim: *"Go with all defaults to the end of this phase p1f.23 and stop"*: a fact is added where a drawn derivation concludes it and its first record is a rule's, a given fact a derivation also concludes having been there before it; its provenance note stays black, since a bright shade is for lines and reads poorly as text. Built: `21728654`, merged by `9f5c0487`.

### Q-M1f.545 — What *near black* means, and how many hues

**Raised** 2026-11-19 by S1f.23.11's build, task 1, [reading 7](p1f.23_review_processing/s1f.23.11_drawings_second_answers.md#readings-each-with-its-default). The user: *"don't use black color for highlighting"*; the stage's brief: no shade near black. **Ruled 2026-11-19: the default**, the user verbatim: *"Go with all defaults to the end of this phase p1f.23 and stop"*: *near black* is a contrast under 3.5 against black, and a shade is readable at 5.5 against white where a step's label is written in it, at 3 where it is a line only; held by `no_shade_of_the_derivation_palette_is_near_black`. Six hues of two shades, not eight: fewer hues that stay distinct at two shades each, clear of red and grey (blue, teal, green, orange, purple, magenta; the deeper shade at 5.8 to 5.9 against white and 3.6 against black, the bright one at 3.3 and 6.4). Built: `21728654`, merged by `9f5c0487`.

### Q-M1f.546 — The reasoning pages' hand drawings of a derivation

**Raised** 2026-11-19 by S1f.23.11's build, task 1, [reading 8](p1f.23_review_processing/s1f.23.11_drawings_second_answers.md#readings-each-with-its-default). **Ruled 2026-11-19: the default**, the user verbatim: *"Go with all defaults to the end of this phase p1f.23 and stop"*: restyled by the same rule (`02-derivation`, `09-core-in-derivation`, `09-cyclic`, `09-proof-graph`), the steps' hues the binary's for `neighbour` (blue) and `fox-keeps-away` (magenta), the hand ones' primary blue and alternative magenta; `09-cyclic`'s given `r(a, b)`, which the alternative derivation also concludes, stays plain (given, not added); `09-proof-graph`'s `F` in the primary's hue. Built: `21728654`, merged by `9f5c0487`.

### Q-M1f.547 — A concentric layout: the layers drawn as rings

**Raised** 2026-11-19 by the user's question on items 56 and 57 ([Q-M1f.487](#q-m1f487--how-the-layers-of-a-knowledge-bases-drawing-are-written), [Q-M1f.488](#q-m1f488--a-folded-arrow-within-a-layer-or-ranked)), verbatim: *"Does Graphviz have circular/concentric layout? Objects will be outside, then facts, then relations and types opbejcts, then relations definitions and properties facts and in the center `relation` `symmetric` and other, is it possible?"*; answered with examples by [S1f.23.11](p1f.23_review_processing/s1f.23.11_drawings_second_answers.md), task 2, and put as item 61 of the review bundle's `03-drawings/` (`7c39c671`), with the stage's [reading 11](p1f.23_review_processing/s1f.23.11_drawings_second_answers.md#readings-each-with-its-default). **Put, not ruled.** It is possible, two ways, neither built: `twopi`, Graphviz's radial layout, from a hidden centre joined to the top layer, whose rings are by graph distance and miss the layers on a nested fact, a fact with a relation as argument, objects split by their nearest fact, an undeclared relation and a type that is also an object (99 of zebra2's saturated 532 nodes moved), and which draws no frame; and the layers bent into rings, every position pinned and the arrows drawn by `neato -n2` or a plain `dot -Tsvg`, the user's rings exactly, a relation's arrows fanned across the centre and a frame's box stretched. The rings turn a strip into a disc and are larger (zebra2's saturated KB 210 × 4 in layered, 52 × 51 bent); below a few dozen facts the layers read best. The examples drew straight arrows in the radial ways (with the router, zebra2's bent KB took 240 s) and the bent ways' frames from computed boxes; 116 drawings, none refused by this host's Graphviz 16.0.0 or the CI image's 14.1.2.

- **(a)** no ring layout: the layers stay the only layout of a knowledge base's views; nothing is built. **Default.**
- **(b)** `--layout radial`: the binary writes a hidden centre, `layout=twopi`, `root` and fitted ring radii; Graphviz
  does the rest. Rings by graph distance, not exactly the user's; no frames.
- **(c)** `--layout rings`: the binary writes every position, its layers as rings, an order of its own on each ring,
  drawn by `dot -Tsvg` as today. The user's rings exactly; the binary computes a layout.
- **(d)** a documented recipe: `ein render kb FILE | dot -Tjson0 | <bend> | neato -n2 -Tsvg`, the binary unchanged.

**Default taken: (a).** Nothing built. The item's page and programs are in `git log` (`7c39c671`); its scripts and
drawings were the bundle's ignored outputs and went with the folder.

**Ruled 2026-11-19: the default**, by the user's *"Go with all defaults to the end of this phase p1f.23 and stop"*
(P1f.23); the item left the bundle with its topic.

**What is kept**: the item's page and programs, in git history at `7c39c671`. Its scripts (`tools/concentric.py`, `sheets.py`, `rerun.sh`) and pictures were the bundle's ignored outputs and went with the folder; the page states both methods (a hidden centre under `twopi`; `dot`'s layers bent into rings, positions pinned, drawn with `neato -n2` or one DOT file with `layout=neato`, `inputscale=72`), from which they can be written again if a ring layout is ever asked for.

### Q-M1f.548 — A derivation's three levels, as built

**Raised** 2026-11-19 by [S1f.23.11](p1f.23_review_processing/s1f.23.11_drawings_second_answers.md), task 1: the
user's answer on item 59 ([Q-M1f.492](#q-m1f492--the-derivations-palette),
[Q-M1f.493](#q-m1f493--how-a-derived-fact-is-marked-beside-a-derivation)) built, and put as item 62 of the review
bundle's `03-drawings/` (`7c39c671`): a derivation drawn before (`d124e63b`'s binary) and after, a step of four
premises, a redundant step, a nested fact, two steps of the reasoning pages and the library's three goldens; 140
drawings, none refused by either Graphviz build. **Put, not ruled.**

- **(a)** as built: the fact a step added the brightest, in its rule's hue at `penwidth=3`; the step and its arrows
  the same hue, deeper, at `penwidth=2`; everything else default black at the default width; six hues of two shades,
  none near black; and the six readings, [Q-M1f.539](#q-m1f539--a-premises-own-node-in-a-derivations-drawing) to
  [.544](#q-m1f544--the-derivation-graphs-added-fact). **Default.**
- **(b)** one width: the added fact at `penwidth=2`, as the explanation, told apart by brightness alone.
- **(c)** the premises as explanation: each premise's octagon and arrows in the step's deeper shade.
- **(d)** a redundant firing's conclusion bright, as any derived fact.

**Default taken: (a).** Built: `21728654` (`ein_render::slice::DERIVATION_PALETTE` a hue a rule in two shades,
`derivation_hue`, the trace's slices, the derivation graph and a written trace's `dag`; `presentation_semantics.rs`'
three tests; Q-M1f.428's decision amended; every page's drawing of a derivation drawn again), merged by `9f5c0487`.

**Ruled 2026-11-19: the default**, by the user's *"Go with all defaults to the end of this phase p1f.23 and stop"*
(P1f.23); the item left the bundle with its topic.
