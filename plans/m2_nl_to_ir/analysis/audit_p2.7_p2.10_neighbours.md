# M2 against the project today: P2.7 to P2.10, the milestone's top level, and its neighbours

Audit of 2026-11-25, on HEAD `458bf90b`, read-only. Binary: `ein.rs/target/release/ein` (`ein 0.1.0`,
`ein-events/1`, `einb/2.2`). Scratch runs are under
`(scratch, not kept: re-run the commands) p27_runs/`.

Files audited: `plans/m2_nl_to_ir/README.md` (313 lines), `plans/m2_nl_to_ir/open_questions.md` (329),
`p2.7_failure_scaling_generalization/README.md` (125), `p2.8_representations/README.md` (95),
`p2.9_formal_account/README.md` (92), `p2.10_result_artifact_demo/README.md` (118), with Stages I, J, K, L, M, O,
P, Q of `EinAf.md`. Not audited: the stage files of P2.1 to P2.6 (other auditors); where the README or the ledger
leans on one, only the leaning sentence was checked.

**One fact first, because it shapes how to read section A.** `python3 utils/doc_audit.py --links --check
plans/m2_nl_to_ir` prints `── links ── 35 files (35 md), 0 finding(s)`. Every link and anchor in M2's folder
resolves: the folder was swept mechanically each time a page moved (`git log -- plans/m2_nl_to_ir`: 44 commits, 21
of them on 2026-11-17). What is stale is **what the sentences say**, not where they point. A link that resolves to
`Q-M1d.6-contradiction-is-said-only-where-no-node-is.md` sits in a sentence that says the engine still says
*Contradiction* where it should not.

Abbreviations: **README** = `plans/m2_nl_to_ir/README.md`; **ledger** = `plans/m2_nl_to_ir/open_questions.md`;
**P2.n** = that phase's `README.md`; **root ledger** = `plans/open_questions.md`.

---

## A. Stale claims

### A.1 The milestone README

| # | where | the plan says | what is true today | evidence |
|---|---|---|---|---|
| 1 | README:10–16 | *"M1d for one word: Q-M1d.6 is the engine saying Contradiction where the research plan's Stage A forbids it to … P2.1 says what M2 does until that lands."* | M1d shipped 2026-08-27. Q-M1d.6 is a decision in force: `Contradiction` is said only where no node is alive, `Open` where every alive node owes, and a stopped run says *not decided — the run stopped first*. The depth cap that produced the wrong word is retired. | `plans/README.md:74`, `:360`; `docs/decisions/answer/Q-M1d.6-…md`; `ein-infer/src/explore.rs:458` (`enum Verdict { Solution, Ambiguity, Contradiction, Open }`); `ein solve examples/zebra2.ein -m 3` → `error: -m/--max-set-size is retired: a depth cap changed answers…`, exit 2; `06-config.md:203–214` (*Eleven options of `ein solve` are retired*) |
| 2 | README:27–30 | *"The root README keeps two things apart: Ein, the symbolic kernel …; and EinAf, the framework around it"* | The root README does not contain the word EinAf. It says the loop is not in the repository. | `git grep -n -i einaf -- ':!plans/m2_nl_to_ir'` lists `nl2smt/README.md`, `plans/README.md`, `plans/followups/f4…`, `f7…`, `plans/m1g_logic/p1g.40…`, `plans/m5_presentation/README.md`, and no line of `README.md`; `README.md:9–12`: *"That loop is not in this repository; the reasoner is"* |
| 3 | README:281–283 | *"`README.md` § EinAf — the thirteen components with their built / scheduled / open columns; the two scheduled cells that pointed at the old S2.4.2 / S2.4.3 now point at P2.4."* | No such section, table or cell exists. `README.md`'s sections are: What it does, The running example, How it reasons, The standard library, Build and run, ein-lang at a glance, Documentation, Layout, License. Under Q-M1f.345 the project README may not name a plan at all. | `README.md` (231 lines, read whole); `docs/decisions/docs/Q-M1f.345-…md` |
| 4 | README:83, :103 | Level A's gap: *"the six outcomes distinguished — with `unknown` distinct from the other five"*; *"A1's sixth outcome, `unknown`, is M1d Q-M1d.6's"* | The kernel distinguishes them today, and has outcomes the plan's six do not list: a load refusal; under `--single` four words (`Solution`, `Ambiguity`, `Contradiction`, `Open`) each with a `decided` row; under describe `exhausted yes/no` with the reason; `-T` abort (exit 2, nothing on stdout, no summary); and **`WARNING … NOT MONOTONE: NOTHING IS CLAIMED OF THIS ANSWER`** on a flagged program. | runs in § B.0 below; `08-read-out.md:181`, `:185`, `:539`, `:115` |
| 5 | README:165 | *"`docs/decisions/` is dropped — it never existed, and a verdict lands beside its question in `open_questions.md`, as M1a's did."* | `docs/decisions/` exists since 2026-11-17 and holds 459 decision files in 19 subject folders; a decision about Ein is a file there under the id of its question. | `git log --diff-filter=A -- docs/decisions/README.md` → `31c15e28 2026-11-17`; `find docs/decisions -name '*.md' ! -name README.md \| wc -l` → 459; `AGENTS.md:110–121`; `docs/decisions/README.md` |
| 6 | README:192 | *"`einaf from-text examples/zebra.txt`"* | `examples/zebra.txt` does not exist (`ls examples/*.txt`: no match). The puzzle's English sentences exist as `nl2smt/puzzles/zebra.txt`, a limited-vocabulary list outside the project (with nine variants beside it: `zebra-minus-15`, `-contradiction`, `-shuffled`, `-reversed`, `-swapped`, `new-sentences`, three `refused-*`). | `ls nl2smt/puzzles`; `AGENTS.md:359–370` (nl2smt is wired into nothing) |
| 7 | README:200–203; ledger:74 | *"The three verdict classes are distinguished: … gold verdict Ambiguity or Contradiction"*; *"the kernel reports `k > 1` / `k = 0`"* | Four words since M1d, and a word exists only when `--single` is asked; a bare `ein solve` describes R and its summary has `"verdict": null`. There are two counts, `k_s` (states) and `k_c` (commitments), and the word depends on which part of R is asked. | § B.0 runs: `zebra2.ein` describe → `verdict None`, `k_s 1`, `k_c 261982`; `examples/lattice/02_genuine_3set_death.ein` → `Ambiguity` over all of R and on the maximal line, `Solution` on the minimal line; `06-solution-semantics.md:515–544`, `:826–838` |
| 8 | README:253–261, :310–311 | *"S1f.6.20 solves elementary-school word problems … Its results, when it has run, are this milestone's to take up"*; *"candidates for the benchmark once it has run"* | It has run, twice (S1f.6.20 and the second pass S1f.6.28), and M1f is closed. `examples/word_problems/` holds 16 problems and four theory modules; each fact carries the sentence it encodes as `:source`; `gap.ein`, `kinds.ein`, `before.ein` are the *not enough information* instances, `contra.ein` the contradictory one, `boxes13.ein` two readings with two answers. | `examples/word_problems/README.md`; `git log -- examples/word_problems` (`f3fe6ff0 2026-11-12`, `d30931e8 2026-11-13`); `docs/decisions/corpus/Q-M1f.334-…`, `Q-M1f.197-…`; guide chapter `docs/ein/user/02-guide/08-word-problems.md` |
| 9 | README:266–268; ledger:19 | *"Q7 leaves the research question (the NL explanation side is M5's demo, not the loop's)"* | A milestone for Ein to natural language exists since 2026-11-20, M1p, and says of M2 *"this milestone is the other direction, and the two meet in a round trip"*. M2's plan names M1p nowhere. | `plans/m1p_ein_to_natural_language/README.md:65–67`; `grep -rn 'M1p' plans/m2_nl_to_ir` → nothing |
| 10 | README:306–309 | *"`--json-summary` (`ein-summary/1`)"* | `ein-summary/2`. (`ein-events/1` is still right.) | `ein solve --help`: *"write the read-out as ein-summary/2"*; `zebra2.json`: `"schema": "ein-summary/2"`; `08-read-out.md:590` |
| 11 | README:115 | Stage M *"with F1 / F1b"* | F1b was promoted to M1g on 2026-11-11; its four parts are M1g stages (S1g.4.1, S1g.3.4, S1g.4.2, S1g.4.5). The file is kept as the statement of the question. | `plans/followups/f1b_logical_formulation.md:3–9`; `plans/followups/README.md` (F1b row) |
| 12 | README:114, :303–305 | *"seeded by M10's hand-written encodings"*; *"M10 — the hand-written encodings P2.8 starts from"* | M10 has not started. The only non-Ein encodings in the tree are three SMT-LIB files of 2021. See A.4 #36. | `plans/README.md:372` (*queued behind M1a*); `ls nl2smt/smt` |
| 13 | README:6–7; `plans/README.md:370` | *"nothing started"*; status row *"after M1e2"* | M1e2 closed 2026-11-08 and M1f, which came after it, closed 2026-11-25. `AGENTS.md:379–385` says *"No milestone is current"* and *"The work stops after M1f … nothing below starts without the user's word"*; its *After M1f* list (`:414–430`) names M1g, M1h, M12, M15, M1k, M1n, M1p and **not M2** (nor M5, M10, M20). The coordinator's brief says the user has now started M2; no file in the tree records that yet. | as cited |

### A.2 The milestone ledger

| # | where | the plan says | what is true today | evidence |
|---|---|---|---|---|
| 14 | ledger:11–13 | *"A verdict lands here, beside its question — the old plan's `docs/decisions/M2-*.md` never existed and is not coming."* | Same as #5. The root ledger's own decided questions now carry a `**Decision file:**` line (Q1, Q2, Q3, Q20, Q23, Q25). | root ledger:83, :100, :121, :335, :438, :495 |
| 15 | ledger:6–8 | *"Q7–Q11 and Q23–Q25 are sticky ids from the cross-milestone pool"* | True of Q7–Q11 (root ledger:22–26 point at M2). **False of Q23–Q25**: in the root ledger those ids are three other questions, M1's (*What carries an explicit type slot*, *`:where` in `sibling-exclusive`*, *Cardinality + ordinality rules with vars*), and two of them are decision files the project cites: `docs/decisions/language/Q23-a-premise-says-what-a-variable-ranges-over.md` (cited by six pages of `docs/ein/` and one `.ein` comment, `tests/stdlib/bijection/06_blind_enumeration.ein:45`) and `docs/decisions/stdlib/Q25-ein-has-no-ordinal-form-an-order-is-a-relation.md`. M2's Q23 is *local model choice*, Q24 *one GBNF or many*, Q25 *frontend language*. | root ledger:31–33, :436–438, :473, :493–495; `git grep -n -w 'Q23\|Q25' -- docs ein.rs/crates tests` |
| 16 | ledger:49–58 | Q7: *"parked here … until one of them [M5, M20] asks for it. The engine's `:why` templates already render every fact and the headline (rendering A, shipped); what is open is only B."* | Since Q-M1f.58 **a fact of a relation takes no text from its declaration**: it reads by its `:source`, by the deriving rule's `:why`, or *as written*, an s-expression. The headline is the query's `:goal-text`, not a `:why`. M1p was created because not every fact reads as English (*"no line a reader is shown falls back to an s-expression for want of words"* is its aim, not the state). | `docs/decisions/read-out/Q-M1f.58-…md`; `README.md:81`; `plans/m1p_ein_to_natural_language/README.md:15–32`; `14-natural-language.md` § 5 *Where a template has nowhere to live* |
| 17 | ledger:65–68 | Q8: *"a `(hypothesis-parse ?id …)` wrapper that the M1 engine treats like any other hypothesis. Hypothesis level 0 = parse choices; level 1+ = puzzle hypotheses."* | There are no hypothesis levels. H is one set, read off root's fixpoint: *the union of the hypotheses root holds*, `(open F)` facts. | `AGENTS.md:184–193` (Q-M1f.570); `04-hypothesis-generation.md` § 2 |
| 18 | ledger:79–83 | *"the engine's existing `hrule` is the form that already makes a choice point out of alternatives. S2.4.5 decides whether a new form is needed or `hrule` is it."* | **There is no `hrule` form** and no `:hrules` keyword. A hypothesis rule is a `(rule …)` whose `:assert` concludes `(open F)`; a program may also write `(open F)` facts itself, held at root. | probe `p27_runs/hrule.ein` → `hrule.ein:2:15: unexpected ')'` with repairs, a parse refusal; `docs/decisions/hypotheses/Q-M1f.567-…md`; `07-diagnostics.md:1114` (§ 2.31) |
| 19 | ledger:121–123 | Q10: *"with the SMT milestone dropped (2026-08-18) there is no in-repo solver path to compare against either"* | `nl2smt/` (M1j, done 2026-11-13) takes the Zebra sentences through NLTK or link grammar and a Racket converter to SMT code and runs a solver, with tests; it is outside the project and *"decides nothing for M2"* by its own page. `z3`, `cvc5`, `cvc4`, `souffle`, `racket`, `link-parser` are on this machine's PATH; `swipl`, `clingo`, `lean` are not. | `nl2smt/README.md:566–568`; `plans/README.md:178–184`; `command -v …` |
| 20 | ledger:207 | *"`07-diagnostics.md` §1/§4 pins them as such"* | The page has § 1 and § 2 and no § 4. | `grep -n '^## ' docs/ein/user/04-reference/07-diagnostics.md` → `## 1. Parse and load diagnostics`, `## 2. Errors and exit codes`, `## See also` |
| 21 | ledger:239–246 | Q-M2.1: *"two kernel milestones are queued: M1d changes what the engine says at a depth cap (Q-M1d.6) and may change the traversal (Q-M1d.4), and M1c may find a stdlib rule wrong. … an instrument that cannot change for six months blocks two milestones."* | M1c, M1d, M1e, M1e2 and M1f all shipped or closed. Q-M1d.4 was closed 2026-08-25, has no decision file, and its only surviving mention in the tree is this line; the traversal it concerned (the walk, `EIN_TRAVERSAL`) is gone. The kernel-side milestones planned today are M1g (P1g.6 engine requirements), M1h (P1h.2 slicing, *an engine phase*), M1k, M1n, M1p, M12, each *only on the user's word*. | `plans/README.md:67–132`; `git grep 'Q-M1d.4'` → only `ledger:242`; `git grep -n 'Q-M1d.4' d1e59532 -- docs` (*closed 2026-08-25*); `AGENTS.md:854` (*The walk, the search R replaced, is gone with its flags and words*) |
| 22 | ledger:255–259 | *"until it lands, the loop computes `unknown` itself from `exhausted = false` and `verdict.type`"* | It landed. The keys are `region.exhausted`, `region.stopped.{why, spent, projected, …}` and `verdict.word` (with `verdict.which`); there is no `verdict.type`. | `zebra2_single.json`: `verdict = {"word": "Solution", "which": "all"}`; `m15.json`: `'exhausted': False, 'stopped': {'why': 'budget', 'spent': 1048507, 'projected': 93, …}`; `08-read-out.md:594` |
| 23 | ledger:318–322 | Q-M2.4: *"§ 1.5 … pins that canonical state identity is the sorted fact list itself, which makes the saturated state's digest — or the `.einb` of a saturated KB — the semantic one, and the summary's sorted `solutions` the model-set one."* | § 1.5 today: *"The identity of a state is its set of facts, compared as a set … A digest d of a state may label it for a reader and **never decides that two states are one**"*. The summary has no `solutions` key: states are `region.states[*]` with keys `facts, closure, from, minimal_generators, commitments, lines, listed`. `ein kb save -s` does bank a saturated KB, but `.einb` is *a private cache format, never an interchange one*. `ein_ir::dump_canonical` exists as the plan says. | `06-solution-semantics.md:364–373`; `zebra2.json` key listing; `ein kb save --help`; `AGENTS.md:331–336`; `ein-ir/src/dump.rs:170` |

### A.3 P2.7

| # | where | the plan says | what is true today | evidence |
|---|---|---|---|---|
| 24 | P2.7:42–44 | *"representation (ein-lang cannot say it — the finding M1d and F1b want)"* | M1d shipped and its plan is deleted; F1b is promoted to M1g. Who takes *ein-lang cannot say it* today is not stated anywhere for M2: the candidates in the tree are M1g's P1g.6, M1h, and the followups M1f left (F28). | `plans/README.md:74–83`; followups README, F1b and F28 rows |
| 25 | P2.7:57–60 | *"solver degradation (the canonical program under `ein solve -e` …; enterings, wall, `exhausted`)"* | `-e` is retired and refused; *enterings* is no counter any more (`-E/--max-enterings` retired). A bare `ein solve` describes all of R; the effort is `work: N saturations · M facts beyond root` (`work.saturations`, `work.facts`), the clock `-t/--timing` or `-s/--stats`, and completion `region.exhausted` with `region.stopped`. | `ein solve examples/zebra2.ein -e` → `error: -e/--exhaustive is retired…`, exit 2; `06-config.md:208–210`; `zebra2.json` `work = {saturations: 120, facts: 5411, layers: null}` |
| 26 | P2.7:64–67 | *"The solver curve has a known shape already on one family: M1d P1d.10's table, where `zebra2-minus-15` finds all 32 models by depth 3 and cannot finish proving there are no more."* | The table is deleted (M1d's plan 2026-08-27, its record with `docs/history/` 2026-11-17). Today the default run establishes the state count and stops in the commitment classes: `states (k_s) 32 (every state of R: the run stopped before it counted their commitments)`, `commitments (k_c) 138 621 243 (a lower bound …)`, `exhausted no — … past --region-budget 1048576`, 11 222 saturations; `--single` answers `Ambiguity`, `decided yes`. *Depth 3* has no counterpart: there is no depth cap. | `ein solve examples/zebra2-minus-15.ein` (§ B.0); `git log -- docs/history` → `cf78590e` |
| 27 | P2.7:107–110 | *"A symbolic-side failure goes to M1d or a followup as a finding with its instance"* | M1d is closed. A wrong answer of the kernel has a standing procedure: a row of `plans/wrong_answers/register.toml` with its witness, in a commit of its own, held by the gate step `utils/wrong_answers.py`; the fix moves it to `corpus/regressions.toml`. A finding outside a phase's scope goes to that phase's own findings page. | `AGENTS.md:404–413`, `:731–735`, `:400–403`; `plans/wrong_answers/README.md:14–40` (9 open rows today) |
| 28 | P2.7:119–125 | *"M1d P1d.10 — the solver's curve on under-determined instances, already measured once. M1e's `review/` — the repo's precedent for a review register kept whole"* | Neither exists in the tree. The review artefact that exists is `utils/review_bundle.py` → `build/review/` (one page per open wrong answer), and the user's rule of 2026-11-18 is that a review bundle holds **only open items**. | `plans/README.md:84–86`; `AGENTS.md` command list; memory `review-bundle-only-open` |

### A.4 P2.8

| # | where | the plan says | what is true today | evidence |
|---|---|---|---|---|
| 29 | P2.8:6–11 | *"M10 — its hand-written encodings for Z3, CVC5, SWI-Prolog, Soufflé, Clingo and Lean are the seed, and its runner, which already installs and drives the systems, is reused rather than rewritten."* | M10 has run no stage. No runner exists: S10.3 places it at `ein.rs/crates/ein-bench` with data under `bench/`, and neither path exists (the workspace has eight crates, none of them `ein-bench`). No encoding exists for Prolog, Soufflé, Clingo or Lean (`git ls-files \| grep -iE '\.(lp\|pl\|dl\|lean\|mzn)$'` → nothing). What exists: `nl2smt/smt/einstain-problem.smt` (194 lines), `einstain-problem-minus-15.smt` (193), `4-queens.smt` (43), SMT-LIB `QF_LIA`, hand-written in 2021, with their own constant names (`nation-england`, `drink-tee`). | `plans/m10_external_benchmarks/s10.3_the_runner.md:13–16`; `plans/README.md:372`; `ls nl2smt/smt`; `nl2smt/README.md:538–543` |
| 30 | P2.8:47–51 | *"the hand-written encodings as the few-shot examples for each target"* | One target of five has any (SMT), covering two problems. M10's own stage calls them *"a warning: they are one person's SMT-LIB, not a published encoding"*. | `s10.1_problem_corpus.md:18–27` |
| 31 | P2.8:14; P2.9:11–12; P2.10:113–114 | *"M5 § 9.4"*, *"M5 § 4 Ein and § 5 Autoformalization"*, *"its § 9.3"* | `plans/m5_presentation/README.md` has no numbered sections: it has Tracks A to D. The numbers are those of `EinAf.md` § Stage P's outline, which M5's header adopts by reference and its body does not follow. | `plans/m5_presentation/README.md:7–14`, `:42–119` |
| 32 | P2.8:72–74 | *"direct evidence for M1d's premise from a second direction"* | M1d shipped; the premise is now the shipped obligations (Q-M1d.2). Wording only. | `docs/decisions/hypotheses/Q-M1d.2-…md` |

### A.5 P2.9

| # | where | the plan says | what is true today | evidence |
|---|---|---|---|---|
| 33 | P2.9:20–22; README:135 | The phase ends with *"the conceptual model written independently of the Rust"*, as a thing still to write | `docs/ein/reasoning/` is that, by its own first paragraph: *"These pages state Ein's reasoning as a theory … stated so that a second implementation would be held to it. Nothing here is true only of the implementation that ships"*. 15 pages, 7 931 lines, one notation table. See § B.3. | `docs/ein/reasoning/README.md:3–11`, `:45–100`; `wc -l docs/ein/reasoning/*/*.md` |
| 34 | P2.9:26 | *"the typed hypergraph: `Value` / `FactId`, relations with signatures"* | *"Ein has no types. No object has one, and no sort of node is one."* A relation is an atom; a declaration is an ordinary fact with a label for each argument, and holds no form to its count. (`Value`, `FactId` exist as named.) | `01-knowledge-base.md:274`; `docs/decisions/language/Q23-…md`; `AGENTS.md:546–557` (Q-M1f.447, Q-M1f.372); `ein-core/src/value.rs:45`, `facts.rs:33` |
| 35 | P2.9:27 | link text `absent_semantics.md` | The page is `03-absent-and-naf.md` (the href is right, the label is the deleted page's name). | `git log` `a36ded94 2026-11-17` |
| 36 | P2.9:28 | *"each saturated, killed by `(false)` or a no-good; a model is a complete, consistent leaf, identified by its sorted fact list"* | A state is dead where a rule that states the engine's word `:ein inconsistent` applied; `(false)` kills only by `std.core`'s rule, and a fact beside its stored negation only where the program imports `std.contradiction`. A solution is **any** node of R, not a leaf: a discharged superset of a discharged state is a node of R in its own right. A model is `st(C)⁺ ∖ K₀⁺`, a projection that *decides nothing*. | `06-solution-semantics.md:262–280`, `:464`, `:532–536`, `:741–761`; probe `clash.ein` (fact beside its negation, no import) → `Solution`; `clash2.ein` (with `(import std.contradiction)`) → `Contradiction`; `Q-M1f.557`, `Q-M1e2.1`, `Q-M1e.6`, `Q-M1e.7` |
| 37 | P2.9:29 | *"the verdict, read off `k` — with `exhausted` … the engine's fourth state (`Aborted`, and the `-m`-cap case Q-M1d.6 names)"*, pinned by `verdict.rs` | `ein-infer/src/verdict.rs` is the query-goal projection; its head says *"`--single`'s four words are not here: they are `crate::explore::Verdict`, read off R"*. The fourth word is `Open`. `Aborted` survives only as `ein-einb/src/solutions.rs:68` (`VerdictKind::Aborted`, the container's record). `-m` is retired. | `verdict.rs:1–6`; `explore.rs:456–468`; `solutions.rs:60–73` |
| 38 | P2.9:41–46 | *"The search is a lattice, not a tree. Commitment sets ordered by cardinality, Apriori-generated, pruned by no-goods and downward closure — F9's ledger is the evidence … An account that draws a DPLL tree describes a different engine."* | F9 is closed and deleted (*"the ledger of the walk's search-layer optimisations"*), and the walk is gone. The **answer** is defined on the lattice; the **search** is a set of routes licensed by lemmas L0 to L8, and the two default routes are trees over owed instances: `--single` prints *"the obligation tree, over states, branching on the owed instance with the fewest alternatives left"*, describe *"the tree and the classes: R's states by the obligation tree"*. The reasoning page *"names no procedure"*. | followups README, F9 row; `ein solve examples/zebra2.ein [--single]` `route` rows (§ B.0); `07-finding-r.md:11–18` |
| 39 | P2.9:47–52 | *"Lower bounds are refutations. `total` / `surjective` are implemented as every candidate excluded ⇒ dead, never as one candidate owed — M1d § what the note says."* | Both exist since M1d. `std.algebra`'s `total` and `surjective` conclude `(false)` when every partner is excluded (`algebra.ein:287–299`); `total-owed` and `surjective-owed` conclude `(open ?R)` while no witness is present (`:330–342`), and `std.slots` has `slot-owed-room` / `slot-owed-fill`. *Owes* and *discharged* are half of R's definition. | `stdlib/algebra.ein:273–342`; `stdlib/slots.ein:326–339`; `10-satisfiability.md:29–42` |
| 40 | P2.9:50–52, :66–68 | *"which is F1b's question answered for the paper"* | F1b's question is M1g's since 2026-11-11 (P1g.4: the fragment as a BNF, the boundary, the decidability table). Two plans now claim the answer; see § E. | `plans/m1g_logic/README.md:413–417` |
| 41 | P2.9:56–60, :76–78 | *"each definition followed by the sentence that says which page of `docs/ein/` is its specification and which test holds it"* | That mapping exists as four pages, `docs/ein/ein.rs/08-reasoning-in-code/` (*where it lives*, *what holds it*, *commands that show it*, for each reasoning page). A page of `reasoning/` itself may name no test (Q-M1f.370). | `08-reasoning-in-code/README.md:1–40`; `docs/decisions/docs/Q-M1f.370-…md` |
| 42 | P2.9:36–40 | *"the engine's `DerivedNafWarning` is the runtime trace of the case the stratification does not cover"* | The name exists (`naf_deps.rs:225`), beside `RefutationUnderAbsentWarning`. What the plan does not have: the static monotonicity check, its `WARNING`, and the rule that nothing is claimed of a flagged program's answer (Q-M1f.564). | `ein-infer/src/naf_deps.rs:213–226`; `08-monotonicity.md` § 3, § 11.3 |

### A.6 P2.10

| # | where | the plan says | what is true today | evidence |
|---|---|---|---|---|
| 43 | P2.10:79–84 | The demo prints *"the verdict with `k` and `exhausted`, and the answer rendered through the puzzle's own `:why` templates"* | A verdict word is `--single`'s, whose rows are `verdict` and `decided`; under it the summary has `k_s: null`, `k_c: null`. `k_s`, `k_c` and `exhausted` are describe's rows, and describe has no verdict word. The headline sentence comes from `(query … :goal-text "…")`. | `zebra2_single.json` region keys; `08-read-out.md:173`; `README.md:81` |
| 44 | P2.10:115–118 | *"`docs/history/m1a_rust/` — the precedent for what a closed milestone leaves behind … this milestone's records are the same kind of artefact and go to `docs/history/` the same way when it closes."* | `docs/history/` was deleted on 2026-11-17 on the user's rulings of 2026-11-15, and M1e2 and M1f closed with no record. A finished plan is deleted; the project documents only its current state. | `ls docs/history` → no such directory; `cf78590e`; `AGENTS.md:386–392`; Q-M1f.341 |
| 45 | P2.10:104–106 | *"M2 done — the milestone README's status and `plans/README.md`'s table updated with the date and the claim's sentence"* | The standing rule is deletion of the folder once what reads it is re-aimed and what it owns has moved (§ C.6). `plans/README.md:391–401` still states the older rule (*"Don't delete; the trail is the project's memory"*), which `AGENTS.md:386` overrides. | as cited |
| 46 | P2.10:62 | *"the kernel has a release matrix awaiting its first tag"* | True in substance, with a detail: `release.yml` triggers on tags `v*`; the repository has eight tags (`0.0.1`, `M1`, `M1.1`, `M1e2`, `M1f`, `p1f.10`, `p1f.23`, `p1f.6`) and none matches. | `.github/workflows/release.yml:28–30`; `git tag` |

### A.7 Claims checked and found true

**57**, listed by file so the count can be re-taken.

- README (22): 10 phases and 22 stage files (`find plans/m2_nl_to_ir`); 21 stage files in P2.1–P2.5, one in P2.6,
  none in P2.7–P2.10; the estimate arithmetic (the `est.` column sums to 24 wk, 7.5 at P2.3, 19 at P2.7); M1a
  shipped; M5 is Stage P by M5's own header; `EinAf.md` at the milestone root and `ideas/11` existing; seventeen
  stages A–Q; `docs/ein/README.md`, `07-diagnostics.md`, `01-grammar.md`, `04-api.md`, `01-install.md`,
  `05-events.md`, `14-natural-language.md` exist; `ein-events/1`; `stdlib/MANIFEST.sha256`; `:why` required on every
  relation and rule; a dead node has a core and the answer is R; F7's *Connection to M2* says sub-track B is on M2's
  critical path and *taken up 2026-08-23*; F12, F13, F16, F17 exist and say what is quoted (F13 holds
  `Cl(T_{i+1},P_{i+1}) = Cl(T_i,P_i)` at line 608); F2 says M2 ships static grammars; Q-M1d.1 and Q-M1d.6 exist;
  `:expect` exists (Q-M1c.1); M20 says M2 can proceed in parallel; the eight families of C1.
- Ledger (14): root index rows Q7–Q11 point at M2; M10 uses `Q-M10.n`; idea 08 and idea 04 anchors resolve;
  `TraceStep` exists (`ein-render/src/trace/`); no `hypothesis-parse` form anywhere; `std.bijection`, `std.slots`,
  `std.algebra` exist and `(bijective R)` is the activator `zebra2.ein` uses; Q2 is closed *never* with a decision
  file; the link-grammar submodule is deinitialised (no `.gitmodules`; `nl2smt/README.md:645–656`); Q-M1a.23 exists
  and says what Q25 quotes; no socket, no server subcommand; `ParseError` is a struct with fields
  (`ein-ir/src/grammar.rs:170`) and `KbLoadError`, `LoadError`, `MacroError`, `CompileError` are `String` newtypes
  (`from_ir.rs:31`, `imports.rs:81`, `macros.rs:26`, `compile.rs:50`); `SolverConfig` exists
  (`ein_core::config`); `ein_ir::dump_canonical` exists; the anchor `#15-commitment-node-state` resolves.
- P2.7 (5): the taxonomy's sixteen categories and five sides, Stage J's eight knobs and K1–K5 match `EinAf.md`;
  `zebra2-minus-15` has 32 states; P2.3 does say object tracking and temporal have no stdlib theory, and the stdlib
  has none today.
- P2.8 (6): M10 names the six systems; `s10.2_systems_and_install.md` exists; the M10 § Risks sentences quoted
  (comparison invites integration, Datalog may not state a choice, Lean is not a solver) are there; Q-M1a.23's
  path; `docs/lib/02`, `/03` exist; F17 places the K framework near Ein (lines 311, 690).
- P2.9 (6): Stage M's formulas as quoted; `01-firing.md#the-core-operations` has O2 and O3 as described;
  `(absent P)` is judged at the closure's boundary (`02-saturation.md:112–168`); `03-design/07-search.md` and
  `13-glossary.md` exist; `DerivedNafWarning` exists; F1, F12, F15 exist.
- P2.10 (4): Stage O's three sentences as quoted; Stage Q's seven layers and the demo sequence; no `v*` tag;
  `01-install.md` has the two channels and *there is no Python package*.

Not verifiable in the tree (in git only, or external): *"as M1a's did"*, *"follows M1d's precedent"*, *"M1a
S1a.9.4 T1a.9.4.6"*, *"acva's client is C++"*, the model names of Q23, llama.cpp's `grammar` field against
ollama's `format`, BBH's 3 / 5 / 7 ladder.

---

## B. What each late phase needs, and its state today

### B.0 The runs the rest of this section cites

```text
$ ein solve examples/zebra2.ein                      # describe
  states (k_s)      1
  commitments (k_c) 261 982
  exhausted         yes — every commitment is classified: this is all of R
  ∅                 not in R — root owes 36 instances
  minimal line      1 state, from 63 commitments — the least guess is 1 candidate
  lattice           monotone (the static check) — R is convex: no hole
  universe          56 candidates, 21 dead alone
  route             the tree and the classes: R's states by the obligation tree, …
  work              120 saturations · 5 411 facts beyond root
  summary: schema ein-summary/2, verdict null, region.{k_s,k_c,exhausted,stopped,states,…}

$ ein solve examples/zebra2.ein --single
  verdict           Solution — R is one state
  decided           yes — R holds no other state
  route             the obligation tree, over states, branching on the owed instance with the fewest alternatives left, …
  work              7 saturations · 278 facts beyond root
  summary: verdict {"word": "Solution", "which": "all"}; region.k_s null, region.k_c null, region.exhausted true

$ ein solve examples/zebra2-minus-15.ein
  states (k_s)      32   (every state of R: the run stopped before it counted their commitments)
  commitments (k_c) 138 621 243   (a lower bound: the commitments of 18 of R's 32 states, one for each of the others)
  exhausted         no — the tree and the classes stopped in the classes, …: the next step projects 93 facts on top of the 1 048 507 spent, past --region-budget 1048576
  summary: region.stopped {"why": "budget", "spent": 1048507, "projected": 93, …}; work.saturations 11222

$ ein solve examples/lattice/02_genuine_3set_death.ein --single            → Ambiguity (all of R)
$ ein solve examples/lattice/02_genuine_3set_death.ein --maximal --single  → Ambiguity (the maximal line)
$ ein solve examples/lattice/02_genuine_3set_death.ein --minimal --single  → Solution  (the minimal line)

$ ein solve examples/word_problems/contra.ein --single   → Contradiction — R is empty: every node is dead
$ ein solve tests/region/04_owing.ein --single           → Open — R is empty: every alive node owes

$ ein solve examples/lattice/17_flagged_region_claim.ein --single
  WARNING           THIS PROGRAM IS NOT MONOTONE: NOTHING IS CLAIMED OF THIS ANSWER.
  summary: monotonicity.monotone false, monotonicity.warning "THIS PROGRAM IS NOT MONOTONE: …", verdict {"word": "Ambiguity", …}

probes (scratch):
  (relation p A :why "…") (p a) (not (p a))                        → Solution   (alive: no rule refutes the pair)
  the same with (import std.contradiction)                         → Contradiction
  (relation p A) (p a) (p a b)                                     → kb load error: relation 'p' has no :why "…"
  the same under --no-require-why                                  → Ambiguity  (the blind enumerator guesses; a fact at another count loads)
```

### B.1 P2.7 — failure analysis, scaling, generalization

| it needs | state today |
|---|---|
| the records of P2.5 and P2.6, the generators and knobs of P2.3 | M2-internal; none exists. |
| a solver-only run of a canonical program, with effort, clock and a completion flag (*"`ein solve -e`; enterings, wall, `exhausted`"*) | **Exists in another form.** A bare `ein solve FILE` or `--count`; `work.saturations`, `work.facts`; `-t/--timing` (load / route / read-out / print), `-s/--stats`; `region.exhausted` and `region.stopped`. The bound is `--region-budget` in facts derived beyond root (default 2²⁰, the user's ruling), `--region-limit` (2¹⁶ saturations) on a flagged program, `-T` seconds. `-e` and *enterings* are gone (A #25). |
| Stage J's knob **model count** | Two counts, `k_s` and `k_c`, which differ by orders of magnitude on one program (1 against 261 982 on `zebra2`); and a third reading, the maximal line's states. Which one a generator's *model count* means is not in the plan. |
| Stage J's knob **hypothesis depth** | No depth cap exists (*"A depth cap changed answers"*). The nearest reported quantities are `region.minimal.least` (*the least guess is n candidates*), `universe.candidates` and `root_owes`. On a monotone program a branch is no longer than the number of instances root owes (`10-satisfiability.md:349–362`). |
| Stage J's knob **proof depth** | I found no such number in the summary (`work` has `saturations`, `facts`, `layers`; `root.saturator` has the boundary's rounds). A derivation's steps are in `--trace` and `--events`. Unsure whether another surface carries it. |
| the solver curve *"already measured once"* (M1d P1d.10) | **Does not exist** in the tree; and the engine it measured (the walk) is gone, so the curve would not be this engine's. Today's behaviour on the same program is in B.0. |
| a place to send a *symbolic*-side failure | **Exists, as a rule the plan does not know**: the wrong-answer register and a phase's findings page (A #27). Under it a wrong answer is filed with its witness in a commit of its own, and the gate holds the row to today's answer. |
| a place to send a *representation*-side failure | Not stated for M2. M1g's P1g.6 and M1h collect language and engine requirements from their own programs; F28 holds what M1f left. |
| the taxonomy's categories read against the language | Two of the sixteen name things Ein does not have as such: *type construction error* (Ein has no types; a label means what a rule makes it mean) and *cardinality error* in the arity sense (a form at another count of a declared relation loads in silence, Q-M1f.447, so it is not a load error the loop is told about). Stage A3's own example diagnostic, `arity_mismatch`, has no counterpart. |
| K3's premise, *"the families the stdlib has no theory for"* | True today for temporal and object tracking. M1h's P1h.1 plans *a timeline and a causal tree, each a stdlib module of its own* (not started): if it ran first, the set of families without a theory would change. Since the reshape the stdlib also gained `std.math.*`, `std.bool.*`, `std.negation`, `std.contradiction`, `std.core`, `std.debts`. |
| a second domain beside the puzzles (README:258–261 names P2.7) | **Exists**: `examples/word_problems/` (A #8). M1g's P1g.10 / P1g.11 and M1h's P1h.10 each plan to hand M2 more pairs of sentence and facts (§ E); none has run. |
| a measuring discipline | Exists as a process rule the plan does not cite: every measurement through `utils/bench_env.sh` (`AGENTS.md:343`), power and cores checked, and no test past the default `--region-budget` (root ledger Q-M1f.304). |
| a flagged program in a scaling or failure run | Not in the plan. A generated theory in which an `absent` reads what a commitment can change is answered with the `WARNING` and nothing is claimed of the answer; for P2.7 that is a class of run that is neither a solver success nor a solver failure. |

### B.2 P2.8 — representations compared

| it needs | state today |
|---|---|
| M10's S10.2 (the systems installed) and S10.3 (the runner) | **Do not exist.** M10 has run no stage. On this machine `z3`, `cvc5`, `cvc4`, `souffle` are installed; `swipl`, `clingo`, `lean`, `minizinc` are not. |
| hand-written encodings per target, as few-shot examples | **One target of five**, two problems (A #29, #30). `AGENTS.md:365–367`: those three files' content *does not change*, because M10 counts them as part of its corpus. |
| an SMT target with a pipeline around it | **Exists in another form**, which the plan does not know: `nl2smt/` produces SMT from the puzzle's sentences by parser and converter (no model), solves it, and has tests over seven sentence files. It is a second NL → SMT route beside the one P2.8 would have an LLM write. |
| *"the loop with the kernel swapped for the target"* | Needs P2.4's loop; nothing exists. Q2 (*Ein hands no problem to an external solver*) is a decision file now, `docs/decisions/engine/Q2-…md`; P2.8 says it does not reopen it. |
| the `ambiguous` class, *"a system that enumerates, or says `unknown`"* | For Ein the plan's three classes are four words plus *not decided* plus *flagged* (A #4, #7). Ein also reports what its answer does not cover: the `outside` row names the relations H holds no candidate of (*"R holds of them only what the program states and saturation derives"*), which is the kernel's own statement of semantic incompleteness (`10-satisfiability.md:364–387`). |
| a reading list for the proof-assistant column | F17 and `docs/lib/02`, `/03` exist as cited. Since then M1g planned proof systems in Ein (P1g.3) and names M10 as the place that runs TPTP and SAT classes; `plans/m10_external_benchmarks/lean.md` (481 lines) exists and P2.8 does not cite it. |
| F26 | Its head names P2.8 for *semantic incompleteness*; its item 6, *the common-sense table as benchmark classes for M2* (contradictory, under-determined by a quantity, under-determined by a reading, complete), *"stays here, for M2"*. P2.8 and the README do not mention F26. |

### B.3 P2.9 — the formal account, against `docs/ein/reasoning/`

**What exists.** `docs/ein/reasoning/` is a statement of Ein's reasoning as mathematics, written to be true of any
implementation, in three parts (`01-static-ir/` 2 pages, `02-kb-evolution/` 10, `03-rule-evolution/` 3), with one
notation table (`reasoning/README.md:45–100`) and a second table of each page's own letters (`:104–120`). Each
page states definitions, propositions and arguments; no page names code or a test. The other half,
`docs/ein/ein.rs/08-reasoning-in-code/`, says for each reasoning page where the code implements it, which test
holds it and which commands show it.

**Stage M's sketch, symbol by symbol.**

| Stage M | what the reasoning tree states today | where | how it differs |
|---|---|---|---|
| `K = (O, R, F, Γ, P)`, a knowledge state as a five-tuple | No tuple. $O$, the objects (atoms, each a name in $\Sigma^{+}$, and facts over objects); $F = O \setminus \Sigma^{+}$; a relation is one atom, with $\operatorname{rel}(f)$, $\operatorname{ext}(r)$ and the held extent $\operatorname{ext}_K(r)$; $K$ is *a finite set of facts*; the rules are the program $P$, a rule $\rho = (\bar p, \varphi, \psi)$ with activators; provenance is $J_K(f)$, the derivations $\delta$ recorded for a fact | notation table; `01-knowledge-base.md`; `02-rewriting-rules.md`; `09-cores.md` § A derivation record | **Three of the five letters mean something else here**: $R$ is the region, $P$ the program, and $\Gamma$ a claim (`10-satisfiability.md` § 3). $O$ and $F$ agree. The rules are not a component of the state. |
| Γ as *rules / constraints* | Four kinds of rule by conclusion: one that derives facts (stored negations among them); a **restriction**, which concludes $\bot$; an **obligation**, whose whole conclusion is $\operatorname{open}(r)$; a **hypothesis rule**, which concludes $\operatorname{open}(f)$. And one conclusion that is no fact: the engine's word $\mathsf{inconsistent}$, stated by the rules $P_{\mathsf{inc}}$ | `02-rewriting-rules.md` § By its conclusion; `06-solution-semantics.md` § 4 | A constraint is not a separate object, and a contradiction is a rule's doing: $\bot$ ends a state by a library rule every program gets, and $f$ beside $\lnot f$ only by a theory a program imports. |
| `T_Γ` monotone; `K* = lfp(T_Γ)` | $T_P(K) = K \cup \bigcup \psi_\rho[a]\theta$, inflationary; **monotone only where no rule reads an absence**; $\operatorname{sat}(K) = \bigcup_n T_P^{\,n}(K) = \min\{K' \supseteq K : T_P(K') = K'\}$, a closure operator. With absences: two phases, the positive closure $\operatorname{pos}$ and a boundary that admits at most one parked candidate a round, in the agenda's order $\lhd$; then a **pass** writes the facts of $\mathsf{open}$. Termination is assumed, not given | `02-saturation.md:12–168`, `:194–`; `03-absent-and-naf.md` | Stage M's one line is the first third of the page. `K*` is $\mathrm{root} = \operatorname{sat}(K_0)$. The seam P2.9 says the account *must* add is already written, with the stratification (`03-absent-and-naf.md:392–`). |
| hypotheses / commitments | $H = \operatorname{gen}(P, \mathrm{root})$, the union of its writers, read once at root; $C \subseteq H$; the lattice $(2^{H}, \subseteq)$; $\operatorname{st}(C) = \operatorname{sat}(\mathrm{root} \cup C)$, **the fork's** (building from scratch may differ); $\operatorname{cl}(C)$, the state class $[C]$ | `04-hypothesis-generation.md`; `05-hypothesis-space.md`; `06-solution-semantics.md` § 1.4–1.5 | Defined, and more than the sketch asks: what writes H is part of the semantics, since it decides what the answer ranges over. |
| the model space, `K* ⊆ M`, `M ⊨ Γ` | $R = \{C \subseteq H : \operatorname{alive}(\operatorname{st}(C)) \land \operatorname{discharged}(\operatorname{st}(C))\}$; its states $\mathcal{S}(R)$; $\operatorname{model}(C) = \operatorname{st}(C)^{+} \setminus K_0^{+}$, *a projection that decides nothing*; *"a state of R is a model of the program **as H scopes it**"*, and of the program itself only where every other relation is closed to derivation or decided | `06-solution-semantics.md` § 2, § 3; `10-satisfiability.md` § 5, § 6 | **The answer is a set of commitments, not a set of models.** It is over H only, and the answer names what H leaves out. $\emptyset \in R$ whenever root is alive and discharged, whatever is proposed. A discharged superset of a member is a member. On a non-monotone program *nothing is claimed* of R. |
| `\|𝓜\| = 0 / 1 / > 1` ⇒ contradiction / unique / ambiguity | $\operatorname{verdict}(R)$: *one solution* ($k_s = 1$), *several* ($k_s \ge 2$), *contradiction* ($k_s = 0$ and every node dead), *open* ($k_s = 0$ and some node alive). Asked of $R$, or of $\min R$ or $\max R$. Two counts, $k_c = \lvert R \rvert$ and $k_s = \lvert R/{\sim} \rvert$. *"One solution has two readings, and an expectation of exactly one has to say which"* | `06-solution-semantics.md:515–544`, `:808–838` | Four values, not three; the empty case splits in two; the count has two units; the verdict is a function of which part of R is asked (B.0: one program, `Ambiguity` on two parts and `Solution` on the third). A count is only *the* count where everything asked was classified (`10-satisfiability.md` § 6). |
| `A_θ(x) → T`, `E(T) → (v, d)`, `T_{i+1} = A_θ(x, T_i, E(T_i))`, `P(faithful ∧ correct)` | **Not stated anywhere in `docs/ein/`**: the loop is not Ein. The nearest kernel notions to $d$: $\operatorname{core}(K)$, $\operatorname{Owed}(S)$, $N$ (the minimal dead sets), the static check's sites; to $v$: $\operatorname{verdict}(R)$, and a claim's $\operatorname{verdict}(\Gamma) \in \{\text{held}, \text{failed}, \text{not checked}\}$ | `09-cores.md`; `10-satisfiability.md` § 3; `08-monotonicity.md` | This is the part of Stage M that is new. **Its letters collide with the table**: $E$ is the minimal discharging sets, $\theta$ a substitution, $T$ is $T_P$ (and closed sets on one page), $x$ a variable, $v$ an object, $\Gamma$ a claim. AGENTS.md's rule is *one notation, the table of `reasoning/README.md`*. |

**P2.9's own three divergences, against today.**

1. *Negation as failure is not in `T_Γ`*: already stated by the reasoning tree, as above. The plan's pointer for
   the uncovered case is one warning name; the tree has a whole page (`08-monotonicity.md`, 718 lines) and a
   decision that nothing is claimed of a flagged program.
2. *The search is a lattice, not a tree*: stale as written (A #38). The tree separates the two: the lattice is
   where the answer is **defined**, and `07-finding-r.md` gives the oracle (the cube) as reference and the routes
   as licensed derivations, *"none says in which order anything is evaluated"*.
3. *Lower bounds are refutations … never as one candidate owed*: false today (A #39). Obligations are first-class,
   and `10-satisfiability.md` § 4 proves that an owed instance's alternatives are jointly exhaustive.

**What P2.9's acceptance asks that the tree's rules shape.** *"Every symbol in the plan's § M has a definition
that names the kernel page that specifies it and the test that holds it"*: a page of `reasoning/` may name no
test, file or command; that pairing is `ein.rs/08-reasoning-in-code/`'s. P2.9 does not say where its account is
written (a page of `docs/ein/`, a paper section under `docs/demo/`, or a plan file), and each has different rules
(§ C.3).

**What P2.9 leans on outside the tree.** F1 (not started; trigger *M1 stabilises*), F1b (now M1g's), F12 and F15
(raw notes; F15's export of rules as formulas is taken up by M1g's S1g.4.2 and S1g.6.3). `03-rule-evolution/`
(rules as data, the dependency graph and strata, transformations that leave R unchanged, with
$\operatorname{Ans}(P)$) is a part of the formal treatment the plan's sketch has no line for; the *semantic* fixed
point of Q-M2.4 is a statement about $\operatorname{Ans}$ or about $\mathrm{root}$, and the plan's `K*ᵢ` (P2.9:63)
does not say which.

### B.4 P2.10 — the result, the artifact, the demo

| layer / need | state today |
|---|---|
| 1 executable system: *run Ein* | Exists: the binary, two install channels, `release.yml` with a four-target matrix on `v*` tags, no such tag yet. *Run the loop*: nothing exists; the root README says the loop is not in this repository. |
| 2 benchmark | M2-internal; nothing exists. If its canonical programs are `.ein` files under `examples/` or `tests/`, each needs a `corpus/corpus.toml` entry or the completeness check fails, and a flagged one may declare no run that checks a claim (`AGENTS.md:291`, `:626–633`). |
| 3 harness, 4 raw results | M2-internal; nothing exists. Other stage files place code under `einaf/`, pages under `docs/einaf/`, records under `experiments/` (e.g. `s2.2.5_from_text.md:57`); none of the three paths exists, and none is placed by the project / process rule (§ C.1). |
| 5 technical documentation: *"the formal semantics and the architecture … the kernel's exists"* | Exists, rebuilt since the plan: `docs/ein/` in three perspectives and `docs/decisions/`. *"The loop's architecture page is this phase's to write"*: no location is named. |
| 6 paper | M5's. `docs/demo/README.md` exists as the home of *the article and the talk "From Zebra puzzle to Autoformalization"* (*"Nothing of it is written yet"*), five beats, the fifth *theory selection, construction and neural-guided synthesis*. Neither M5's README nor P2.10 names `docs/demo/`. |
| 7 the demo, *"one command"* | Nothing exists. What it would print from the kernel is available in today's vocabulary (A #43). Two things called *demo* now exist in plans and pages: P2.10's one command, and `docs/demo/` (the article and talk), with M5's *"worked demo (Zebra + an NL-fed puzzle)"*. |
| *"the pinned Ein commit"* | `ein --version` prints five lines (version `0.1.0`, protocol, container, features, the stdlib's sha256 and where it was resolved) and no commit hash; a commit is the checkout's, read from git. |
| *"the `pip install` trip-wire of Q-M1a.23 re-read"* | The decision stands and names when a binding is added: start-up dominating a loop of many small items, state persisting across many small queries, or *a Python program outside this project that expects a module*. `AGENTS.md:135`: *`pip install` is not a channel*. |
| what a closed milestone leaves | Not `docs/history/` (A #44). Under today's rules: decisions about Ein as files under `docs/decisions/`, process rulings in the root ledger, open findings in followups, the folder deleted. Where **research records** (the tables, the classified failure sample, the raw generations) live after the folder is deleted is not stated by any rule I found. |

---

## C. Conflicts with standing rules and decisions

**C.1 The project and the process are two things** (`AGENTS.md:722–730`;
[Q-M1f.345](../../../docs/decisions/docs/Q-M1f.345-nothing-in-the-project-reads-links-or-names.md)). The project is
`README.md`, `ein.rs/`, `stdlib/`, `examples/`, `tests/`, `corpus/` and `docs/` but `docs/lib/`; the process is
`plans/`, `utils/`, the gate and CI; `c/`, `nl2smt/` and `docs/lib/` are external to both. **M2's deliverables are
on no side of that partition**: a harness (`einaf/`), its pages (`docs/einaf/`), a benchmark, experiment records
(`experiments/`; Stage N's own example is `docs/experiments/experiment-0042/`), a formal account, a demo with a
checked-in transcript. Anything placed under `docs/` other than `docs/lib/` is the project and takes its rules:

- *current state only*, no step of the work, no date, no page kept as a record
  ([Q-M1f.341](../../../docs/decisions/docs/Q-M1f.341-the-project-documents-only-its-current-state.md));
- *no measurement*: *"no time, no memory, no cost taken on some machine on some day"*; a number carries the
  command that re-takes it or the test that pins it
  ([Q-M1f.349](../../../docs/decisions/docs/Q-M1f.349-a-page-of-the-project-states-no-measurement.md),
  [Q-M1e.4](../../../docs/decisions/docs/Q-M1e.4-an-exact-number-in-a-page-carries-the-command.md));
- *release voice*: no milestone, phase, stage or task id, no plan path, no date, in any project `.md`, crate
  comment, `.ein` comment or corpus note (`AGENTS.md:715–721`, `utils/check_release_voice.py`);
- *names no plan* and *cites no open question*
  ([Q-M1f.347](../../../docs/decisions/docs/Q-M1f.347-no-open-question-is-cited-by-the-project.md)).

An experiment record is a dated measurement by nature, and a results table is measurements. `README.md:9–12`
(*"That loop is not in this repository"*) is a current-state sentence that a loop built in this repository would
make false. No decision or ruling I found says which side EinAf is on.

**C.2 `docs/demo/` exists** and is the project's (`README.md:222`, `docs/ein/README.md` § Beside them). It is
M5's home by its own first line, not P2.10's. A page there is under C.1's rules.

**C.3 Each perspective has its languages**
([Q-M1f.370](../../../docs/decisions/docs/Q-M1f.370-each-perspective-of-the-documentation-is-written.md); `AGENTS.md:86–95`).
`reasoning/` holds no Ein source, command, Rust, file path, test name or algorithm, links only pages of
`docs/ein/` and decisions, and uses one notation. P2.9's acceptance (*names the kernel page and the test that
holds it*) is not writable as a page of `reasoning/`; and Stage M's letters collide with the notation table in
six places (§ B.3). P2.1's README already records the same collision for its own page (*"To settle at this
phase's re-plan"*, P2.1:96–108, M1f's finding L-48); P2.9 carries no such note.

**C.4 A decision is a file under `docs/decisions/`, under the id of its question**
(`docs/decisions/README.md`; [Q-M1f.342](../../../docs/decisions/docs/Q-M1f.342-a-decision-is-a-question-with-its-final-answer.md),
Q-M1f.343, Q-M1f.344, [Q-M1f.346](../../../docs/decisions/docs/Q-M1f.346-the-decisions-are-about-ein-not-about-how.md)).
The rules that would apply to an answered M2 question:

- **Where.** A decision *about Ein* (its language, what it answers, its commands, its engine, its library, its
  corpus, its documentation) is one file, `<subject>/<id>-<slug>.md`, the final answer only, in the form
  `# <id> — <the answer as a sentence>` / what holds today / `**Why.**` / `**Held by.**` naming the project's
  tests, code and pages; its folder's `README.md` lists it in id order. A decision *about how the work is done*
  stays in `plans/`. The 19 subject folders have none for the loop, and whether an answer about EinAf (the model,
  the grammars, the harness's language, the unit of faithfulness) is *about Ein* is not ruled. On a plain reading
  only parts of three questions touch Ein itself: Q8 if it adds or declines a language form, Q25 as far as it adds
  a kernel surface (`ein-feedback/1`), Q-M2.4 if the engine is to give a canonical form or digest.
- **The id.** *"An id is kept and never reused"* and names exactly one file, which the gate's link check holds
  (`utils/doc_audit.py --links --check --tracked`, *every decision id the project cites resolves to exactly one
  decision file*). **`Q23` and `Q25` already name decision files** (A #15), so M2's Q23 and Q25 cannot be filed
  under their ids; `Q24` is taken in the root ledger though it has no file. The standing rule for a ruling with no
  usable id is *the next free id of the milestone that ruled it* (`AGENTS.md:118–121`); M2's own series is
  `Q-M2.1–4`.
- **What the ledger says instead.** Ledger:11–13 and README:165 say a verdict lands beside its question and
  `docs/decisions/` *is not coming*. Both are false as descriptions of the tree (A #5, #14).
- **Citing.** A project page cites a decision by link, a comment by bare id; an open question is cited by nothing
  in the project.

**C.5 No estimates** (the user, 2026-11-08; `plans/README.md:364–365` writes *"— (no estimates, the user's ruling
of 2026-11-08)"* for M1g and M1h). Every file audited opens with an `**Estimate:**` line, the README has an `est.`
column and a paragraph defending the numbers (README:153–155), and the status row says *~6 months*. The recorded
rule is to leave existing estimate lines alone and neither maintain nor report them; the list is in § F.

**C.6 A finished plan is deleted** (`AGENTS.md:386–392`). P2.10:104–106 closes M2 by updating statuses with a
date and a sentence, and P2.10:115–118 sends records to `docs/history/`. Both follow the rule
`plans/README.md:391–401` still prints and `AGENTS.md` has replaced.

**C.7 How a run keeps findings, questions and wrong answers** (`AGENTS.md:393–413`). A phase keeps what it finds
outside its scope in a findings page of its own beside its README and shows it; at the phase's close what is open
moves to the milestone's page or its owner, at the milestone's close to the followups. Questions that span
milestones are the root ledger's, ids sticky. A wrong answer is a register row. M2's plan has none of the three:
it routes kernel findings *"to M1d or a followup"* (P2.7:107–110, P2.7:42–48) and has no findings page
(`ls plans/m2_nl_to_ir/*/ | grep -i find` → nothing). Also binding by the user's recorded rulings: a stage per
subagent, run one after another, stopping only for a question that is the user's; a default taken is recorded and
stands until revised; a new surface or syntax is shown on real corpus programs with their output before it is
built; *create is not start*.

**C.8 The answer's rules, where the plan's vocabulary cuts across them.**

- *A non-monotone program loads, and nothing is claimed of its answer*
  ([Q-M1f.564](../../../docs/decisions/answer/Q-M1f.564-a-non-monotone-program-loads-with-a-warning.md); `AGENTS.md:614–633`).
  The plan's central quantities, *`P(correct | Ein certifies unique)`* and *`P(faithful | Ein certifies unique)`*
  (README:72–73, P2.8's fourth question, P2.10:46–53), assume a verdict is a certificate. On a flagged program the
  kernel prints a word and disclaims it; no page, comment or test may say that answer is right, sound or complete.
- *Every answer exits 0; only a refuted claim exits 1* (`AGENTS.md:635–638`): the exit code distinguishes no
  verdict.
- *No config option, lever or flag changes R* (`AGENTS.md:639–642`): Q-M2.1's *resolved `SolverConfig`* as a
  record field records nothing that can move an answer.
- *`.einb` is a private cache format, never an interchange one* (`AGENTS.md:331–336`), against Q-M2.4's *the
  `.einb` of a saturated KB* as the semantic digest a loop compares.
- *The kernel reads no theory's name* and *the contradiction is a theory a program imports* (Q-M1f.436,
  Q-M1f.557): a generated theory that concludes a negation and does not import `std.contradiction` has no dead
  state from it (probe in B.0). The plan's *"killed by `(false)`"* (P2.9:28) and the taxonomy's *missing
  constraint* do not have this case.
- *`:why` is required* on every relation, rule and macro: a generated program without one is a load refusal
  unless `--no-require-why`.

**C.9 Review artefacts.** P2.7's failure sample is described as *"a review register kept whole"* (P2.7:123–125);
the user's rule for a review bundle is that it holds only open items, and that anything built for review carries
every engine artefact, raw and rendered. Whether a research sample is a review bundle in that sense is not ruled.

---

## D. Open questions

### D.1 M2's twelve

*Who decides* is what the plan says. No entry says *the user decides*: each names a stage. Under the standing
practice a stage's answer is a default, recorded and shown, until the user revises it.

| Q | the question | decided in (per the plan) | do its premises hold? | answered or changed since? |
|---|---|---|---|---|
| **Q7** | may the NL surface generator be an LLM | nobody: *outside the research question*, parked for M5 or M20 | **No.** *"`:why` templates already render every fact"* is false since Q-M1f.58 (A #16). | **Changed.** M1p (2026-11-20) owns Ein to natural language and says it meets M2 in a round trip; the root ledger's Q20 still defers *"full human-template fitting to M2 (where an LLM surface generator is available)"*; `plans/README.md:45–49` and `AGENTS.md:159` still give M2 the NL explanation. Four texts, three owners. |
| **Q8** | ambiguous parses: branched on the IR or rejected | S2.4.5, with P2.6 comparing the two strategies | **No.** `hrule` is gone; there are no hypothesis levels (A #17, #18). | **Changed.** The form that makes a choice point is a rule concluding `(open F)`, or `(open F)` facts written by the program. `examples/word_problems/boxes12.ein` and `boxes13.ein` are worked instances of a sentence with two readings, each a candidate, root owing one (*"two states and two answers"*). M1h's P1h.3 (gaps, assumptions and readings) plans the same subject on hand-written programs. |
| **Q9** | ontology declared or inferred | S2.2.4, reframed as theory selection | Mostly. The three modules exist and are selected by an activator fact. Not in the premise: Ein has no types, so *"types and instances"* are facts a rule gives meaning to; the catalogue is larger (calculator, Boolean, contradiction, debts); a selection that needs deaths must import `std.contradiction`. | Not answered. *"User review before solving does not survive"* is the plan's own ruling, taken by nobody. |
| **Q10** | direct LLM → constraint | P2.8 | Partly. *"No in-repo solver path"* is no longer exact (`nl2smt/`); Q2 and M3's drop stand, Q2 now as a decision file. *"M10's hand-written encodings are the seed"*: three files, one target. | Not answered. |
| **Q11** | does link-grammar output help | S2.6.4, by experiment | Yes for the submodule note. New since: `nl2smt/` drives the system `link-parser` 5.13.0 with no submodule and records its quirks; M1k's P1k.20 plans link grammar's output read as Ein facts. | Not answered; `nl2smt/README.md` says it *"decides nothing for M2"*. |
| **Q23** | which local model | S2.2.2, by a benchmark on the seed set | Unverified: the candidates are a list of May 2026. **The id collides** with a decision file (A #15). | Not answered. |
| **Q24** | one GBNF or many | S2.2.3 | Not checked here (P2.2's). **The id is taken** in the root ledger. | Not answered. |
| **Q25** | the frontend's language | S2.1.3, after S2.1.2 | The error-type census holds today; `§1/§4` is stale; `--json-summary` is `ein-summary/2`; a third machine surface exists, `--dump-answer DIR` (every state as loadable Ein text, `answer.json`, `index.json`) and `ein test --json-report` (`ein-test-report/6`). **The id collides** with a decision file. | Not answered, but leaned on: stage files under P2.2 already write Python paths (`einaf/passes/*.py`, `s2.2.1_contract.md:48` *"a Python harness under `einaf/` driving the `ein` binary"*) while the ledger says the decision is S2.1.3's. M15 (an MCP server, planned 2026-11-12) lists M2's formalizer as a possible client and says the dropped server and binding are *read before either is revived*. |
| **Q-M2.1** | when is the kernel frozen | S2.5.1 | **No** as to what is queued (A #21); **no** as to the interim `unknown` computation (A #22). The shape of the answer (frozen per experiment by commit) rests on nothing stale; `ein --version` gives no commit. | The M1d dependency it names is discharged. The tension it describes is unchanged in kind: seven kernel-side milestones are planned, each on the user's word, and *no backward compatibility* is the user's rule of 2026-11-19 (one user; no old spellings or saved-file compatibility until asked). |
| **Q-M2.2** | must the model be local | S2.2.2 | External premises (hosted APIs, token accounting), not checked. | Not answered. |
| **Q-M2.3** | the unit of faithfulness with no gold theory | S2.4.4; the human sample *sized in P2.7* | (a) `:source` exists on a stated fact and is optional; a nested fact takes no keyword. Written provenance has an open followup: **F24** (a written `:rule` / `:using` is stored unchecked and a fact carrying one leaves every core, `unsat core ∅`), whose trigger is *"a consumer [that] reads cores from programs it did not write"*. M2 does not cite F24. | Not answered. The user's line in M1h, *"the solution could be correct, but reasoning not - this case is M2 and Ein theory of itself or theory of solutions"*, hands M2 this question by name (`plans/m1h_beyond_word_problems/README.md:66–69`). |
| **Q-M2.4** | is the fixed point syntactic or semantic | S2.4.1 | **Partly no** (A #23): the page now says a digest never decides identity; no `solutions` key; `.einb` is not for interchange. The canonical dump exists. | Not answered. New material: `--dump-answer` state files that load as themselves (Q-M1f.392); `03-transformations.md`, the rewrites of a rule set that leave R unchanged, with $\operatorname{Ans}(P)$, which is the kernel's own statement of *changed its text and not its meaning*; `ein-render/tests/renaming_invariance.rs`. |

### D.2 What the late phases defer

- **P2.7**: the failure sample's size (*"stated before it is drawn"*, *"set by what a week can read"*); the
  taxonomy's growth; where a *representation* finding goes; the sentence the claim will be, written at the Level C
  gate.
- **P2.8**: everything below the README (*"written to stage depth after the Level C gate"*); each table's
  pre-registered reading; which proof assistant; what *certifies* means per target.
- **P2.9**: its depth (*"a definition and a proposition, or a section with proofs"*), which waits on **M5 fixing
  the venue**, an open question of M5's with no owner or date; what of F1, F1b, F12 the account adopts; where the
  account lives; which of two things `K*ᵢ` is.
- **P2.10**: whether the raw results ship whole or as an index with digests; the loop's own release channel and
  the harness's command names (*"in Q25's language"*, *"or Q25's names"*); the demo's instance; where the
  transcript and the loop's architecture page are checked in.
- **M5's own, which P2.9 and P2.10 wait on**: venue, author list, the open-source story, the reproducibility
  artefact (the last is what S2.5.1 and P2.10 plan to deliver, while M5 lists it as open).
- **Not asked by any file, and raised by the comparison**: which side of the project / process partition EinAf's
  code, pages, benchmark and records are on (C.1); what the loop does with a flagged program's verdict (C.8); which
  part of R a *verdict* is asked of, and which count *model count* is (B.1, B.3); what F26's four classes are to
  the plan's three (B.2).

---

## E. The neighbours

### E.1 The table

| neighbour | what it hands M2, or expects of it | what M2's plan says | contradiction or gap |
|---|---|---|---|
| `plans/README.md` roadmap (:20–49) | M2 is *"the whole of the research programme"*; its theory pass selects activators from the stdlib; *"M2's ultimate ambition is the full row — NL problem → facts → ontology+rules → solution → **NL explanation of solution steps** — i.e. NL parses into the same `(facts …)` / `(ontology …)` blocks the engine consumes"* | The NL explanation *"leaves the research question"* (README:266–268, ledger Q7) | **Contradiction** on the NL explanation. And `(facts …)` / `(ontology …)` are not block forms: a `.ein` file is a flat sequence of forms, and `examples/syntax/wrappers.ein` shows those heads are ordinary facts with nested facts. |
| `plans/README.md` tree (:295–308) and status (:370) | The folder's shape, phase by phase; *"after M1e2 … ~6 months — Level B at ~8 weeks"* | README:3–9 | The tree matches the folder. The order *after M1e2* is two closed milestones old. |
| `plans/README.md` § How to use (:391–401) | *"When a stage is done: append Status: done … Don't delete"* | P2.10:104–106 follows it | Both contradict `AGENTS.md:386` (§ C.6). |
| root ledger, index and bodies | Q7–Q11 live in M2. Q3: *"a future GBNF grammar (M2 S2.2.3) is trivial"*. Q6: *"M2's NL → IR pipeline inherits the decision: the NL parser maps positional phrases … to named House instances by adopting a convention"*. Q20: *"Defer full human-template fitting to M2 (where an LLM surface generator is available)"*. § How to add a question: *"the next free `Q<n>` … don't reset per milestone"* | Ledger: Q7 is not M2's; ids `Q-M2.n` | **Contradiction** on Q20 against Q7. **Id collision** on Q23–Q25 (A #15). The Q6 convention is cited nowhere in M2's plan. The root's id rule and the `Q-M2.n` form disagree (M10's ledger explains the same choice for itself). |
| F1 (categorical) | Nothing by name; trigger *M1 stabilises*; not started | P2.9's S2.9.3 reads the account against it | No conflict; F1 is a note, not a result to read against. |
| F1b → **M1g** | F1b promoted 2026-11-11; M1g's P1g.4 pins the fragment and the boundary. M1g → M2: *"a formula language with checked semantics is a formal target a formalizer can be measured against, with ground truth no puzzle has"*; P1g.10 and P1g.11 hand *the problems as pairs, the sentence and the facts* | P2.9 claims *"F1b's question answered for the paper"*; P2.7 sends representation findings to *"F1b"*; M1g is named nowhere | **Both claim the fragment.** No order between M1g and M2 is ruled (`AGENTS.md`: each on the user's word; M1g *runs after M1f*). M1g's offered benchmark material is unknown to M2's P2.3. |
| F2 (self-modifying grammar) | *Trigger: M2 (LLM infra + GBNF for IR) is complete and stable*; M2 ships static grammars, *"one per task"* | README:292–297 agrees; Q24 says *one per pass* | Consistent. F2 is also pulled into M12. |
| F4 (cross-cutting) | The question-list *"rosetta stone"* stays F4's; the implementation *"lands in M2 P2.2"*, S2.2.4 | M2 cites F4 once | Consistent. |
| F7 (rule induction) | Sub-track B is M2's theory pass; B′, C, D and the statistical half stay parked; *"two of the benchmark's eight families have no stdlib theory on purpose"* | README:59–62 agrees | Consistent. See M1h for what could change the *two families*. |
| F12, F13, F16, F17 | Raw notes of the user's (in Russian); F13 is *"the other benchmark direction, NL-shaped and M2-gated"* | Cited as premises | Consistent. |
| F22 (exact core) | A trigger: *"an explanation consumer that needs the minimal core … M2's NL explanation of a refutation"*. Standing rule: a core is an explanation and may vary with rule order until F22 | The feedback ladder has an unsat-core level (F5) and an ablation (G3); M2 does not cite F22 | **Gap**: the plan treats the core as a fixed diagnostic. (F5 and G3 are other auditors' stages.) |
| F23 (diagnostics in Ein's words) | Triggers: a release, M5's presentation, *a stage that touches the loader's or the compiler's error paths* | P2.1 builds a diagnostic vocabulary; M2 does not cite F23 | **Gap**: overlapping subject, no cross-reference. |
| F24 (written provenance) | Trigger: *a consumer [that] reads cores from programs it did not write* | Not cited | **Gap** (D.1, Q-M2.3). |
| F26 (consistency and completeness) | Item 6 *"stays here, for M2"*: four benchmark classes (contradictory, under-determined by a quantity, under-determined by a reading, complete); its head cites P2.8; trigger *"when M2 or M5 has to say in a paper what Ein's verdicts certify"* | Not cited; the benchmark has three verdict classes | **Gap**, and a count mismatch (four against three). M1g's S1g.4.5 takes F26's glossary, decidability table and chase reading. |
| F27 (cryptarithm theory) | Trigger: *a benchmark of puzzle families beside the zebra starts* | Not cited | M2's P2.3 would fire it. |
| **M5** | Header: M5 *is* Stage P; P2.10 hands it the result and six layers, P2.9 the account. Body (Tracks A–D, older): *"Write up the Ein system for an external audience"*; a paper, a talk with *"a worked demo (Zebra + an NL-fed puzzle)"*, a landing page; Track B wants *"at least one external benchmark"* and *"both `solve` and `prove` results"*; *"Implementation work driven by the paper"* is out of scope; open: venue, authors, the reproducibility artefact | Stage P: *"The final paper should revolve around the research question rather than the software"*; P2.9 waits on M5's venue; P2.10 owns layer 7, the demo | **M5's header and body disagree** (the research question against *the Ein system*). **Circular wait**: M5 is *after M2*, and P2.9's depth waits for M5 to fix the venue. **Two demos** (P2.10's command; M5's talk demo and `docs/demo/`). The reproducibility artefact is open in M5 and planned in M2. `prove`, the task-class modes and the *typed-hypergraph* framing in M5 are not today's vocabulary. |
| **M10** | *"needs M2's NL frontend to be interesting"* is said of F13's direction, not of M10; M10's consumer is M5 Track A; answers are written back as `:expect`; the runner is `ein-bench`; not started | P2.8 depends on S10.2 and reuses *"its runner, which already installs and drives the systems"* | **False premise** (A #29). M10's own text is stale too (`ein test` *"runs `-m 5`"*, `--max-set-size`, *M1d's premise*): both flags are retired. `zebra2-minus-15.ein` states no `:expect` today (`ein test` → *0 expectations*). |
| **M12** (rules as data) | Open question 2: *"Where it sits … against M2, whose S2.6.4 counts rule schemas a model writes"*; its census counts rule shapes over the hand-written corpus, *"the same count"* S2.6.4 would take by machine | Not cited | **Gap**; the census ran (2026-11-12) and its results page exists. |
| **M15** (MCP server) | Lists *the formalizer of M2* as a possible client; *"its first phase is the kernel as instrumentation … which a tool-calling server would also deliver"*; *"a new output format is a contract with a reference page and a test"* | Not cited; P2.1 plans `ein-feedback/1` | **Overlap**, no order: two plans would each define how the kernel's answer reaches a model. (M15 says the binary has five subcommands; it has six since `ein show`.) |
| **M20** (GUI) | *"M2 (NL → IR) can proceed in parallel"*; nothing about a loop | *"displays whatever the loop records; not a dependency either way"* | One-sided: M20's views (code, graph, branches, the user's list of 2026-11-18) have no view of loop records. |
| **M1h** (beyond word problems) | *"Producing those facts from the text is M2's"*; every pair of sentence and facts *"is an instance of what M2 would have to produce"*; P1h.10 hands *"the fifty questions with expected answers, as a small benchmark with graded difficulty"*; the user's line gives M2 *"the solution could be correct, but reasoning not"* | Not cited | **Gap.** And P1h.1 plans temporal and causal theories as stdlib modules, the family M2's K3 counts on having no theory. |
| **M1p** (Ein to NL) | *"the two meet in a round trip, whose target is the annotated walkthrough of the guide"*; P1p.1 mentions *"M2's side of the round trip"* | Not cited; Q7 parks the direction with M5 and M20 | **Contradiction of ownership** (D.1, Q7). |
| **M1k** (notations as facts) | P1k.20: S-expressions (Lisp, SMT-LIB, link grammar's output) read as Ein facts | Not cited | Touches S2.6.4's input and P2.8's SMT target; no cross-reference either way. |
| **M1n** (non-monotone programs) | Nothing by name. It owns what a flagged program's answer is | Not cited | **Gap** (C.8): the loop will generate programs, and some will be flagged. |
| ideas 01, 04, 05, 06, 08, 11; `plans/ideas.md` | Idea 04 carries an implementation hint for S2.2.4; idea 10 says M2 builds the GBNF infrastructure F2 mutates; `ideas.md` has no live entry and no mention of M2 | Cited as origins | Consistent. The ideas index path in `plans/README.md:6–8` (*`docs/ideas/`*) is a stale label. |
| `AGENTS.md` | Two mentions: `:157–159`, the Zebra walkthrough is *"the M2 target for the NL ⇄ IR round-trip"*; `:367–370`, `nl2smt/prototype/` holds the two scripts S2.6.4 starts from. *After M1f* omits M2. *Working priorities* names idea 01 (the LLM ↔ harness loop on GBNF) as the biggest unrealised idea | The round trip's NL half is not M2's (Q7) | Same contradiction as M1p's row. AGENTS.md's pointer (*No milestone is current*) is the thing the file says to edit when the pointer changes. |
| `nl2smt/README.md` (external) | *"It decides nothing for M2"*; keeps `prototype/` because S2.6.4 names it; notes S2.6.4's possible outcome *"deprecate the submodule"* | S2.6.4 | Consistent. |
| `docs/demo/README.md` (project) | The article's fifth beat is M2's subject; names no plan | Not cited | See M5's row. |
| root `README.md` (project) | *"That loop is not in this repository"* | README:27–30, :281–283 describe a section that is gone | A #2, #3. |

### E.2 The contradictions, in one list

1. **Who owns Ein → natural language**: `plans/README.md`, `AGENTS.md` and the root ledger's Q20 say M2; M2's
   ledger says M5 or M20; M1p says M1p, meeting M2 in a round trip.
2. **Who answers F1b's question** (the fragment Ein covers): P2.9 and M1g's P1g.4.
3. **P2.8's dependency on M10** assumes work M10 has not done; M10 does not know P2.8 exists as a consumer of its
   runner (its consumer is M5 Track A).
4. **M5 and M2 wait on each other**: M5 is after M2; P2.9's depth waits for M5's venue.
5. **M5 against itself**: Stage P by its header, *the Ein system* written up by its body.
6. **Ids**: M2's Q23, Q24, Q25 against the root ledger's, two of them decision files.
7. **Where a decision lands**: M2's ledger (beside its question) against `docs/decisions/` and `AGENTS.md`.
8. **What a closed plan leaves**: P2.10 (`docs/history/`, a dated status) against *a finished plan is deleted*.
9. **Order**: `plans/README.md` says *after M1e2*; `AGENTS.md` says nothing after M1f starts without the user's
   word and does not list M2; M1g *runs after M1f is complete*; no text orders M2 against M1g, M1h, M1p, M10,
   M12 or M15, several of which offer M2 inputs or overlap its outputs.
10. **Benchmark classes**: three in the plan, four in F26's hand-over, four words plus two non-answers in the
    engine.
11. **Kernel findings**: P2.7 sends them to M1d; the standing route is the register and a findings page.

---

## F. Estimates, dates used as status, counts

**Estimates** (kept nowhere else since 2026-11-08): README:3–5 (*~6 months*, *~24 weeks*, *Level B at ~8 weeks*,
*Level C at ~19*); README:125–136, the `est.` column (1.5, 3, 3, 3, 2.5, 3, 3, 2, 1.5, 1.5 wk); README:153–155
(the paragraph on what the estimates are); README:159 (*six phases, two months*); P2.7:3 (*3 weeks*), P2.7:104–106
(*what a week can read*); P2.8:3 (*2 weeks*); P2.9:3 (*1.5 weeks*); P2.10:3 (*1.5 weeks*); ledger:245 (*six
months*); `plans/README.md:370` (*~6 months — Level B at ~8 weeks*), `:371` and `:375` (*TBD*), `:372` (*~2.5
weeks*); M5 and M20 `**Estimate:** TBD`; M10:3 and its stage table.

**Dates used as status**: README:6 (*reshaped 2026-08-23 … nothing started*), :20 and :159 (*the 2026-05 plan*),
:140 (*written 2026-08-23*), :228–230 (the idea of 2026-11-11); ledger:8, :49, :127, :185, :209, :237, :263, :289,
:311 (each *2026-08-23*), :122 and :196 (*2026-08-18*), :192 (*2026-08-21*). The four late READMEs carry no date.

**Counts in the text**: *10 phases, 22 stage files*; *21 stage files*; *seventeen stages A–Q*; *thirteen
components* (of a table that is gone); *the six verdicts*, *six phases*, *five puzzles*, *three metrics* of the
old plan; *eight families*; *nine renderers*, F0–F8; B0–B5; G1–G9; K1–K5; *four metric layers*; *three verdict
classes*; *six outcomes*; *seven layers*; *three admissible shapes*; *five narrow questions*, *five targets*,
*six systems*; sixteen taxonomy categories, *five sides*; *32 models by depth 3*; *3 / 5 / 7*; *3–5* and *8–15*
entities; *three or more surface templates*; *≤ 3 attempts*; *Qwen3-30B*, *Phi-4 14B*; *four new questions*
Q-M2.1–4. Only the first two and *seventeen* are counts of the tree, and they hold.

---

## Where I am unsure

- I did not read the stage files of P2.1 to P2.6. Statements here about `einaf/`, `docs/einaf/`, `experiments/`
  and the Python paths come from a `grep` over them, not from reading them whole.
- *Proof depth*: I looked in the summary and the help text and found no such number; another surface may carry it.
- Whether an answer about EinAf is *about Ein* for `docs/decisions/` is a reading, not a rule I found; I report
  the rule and that it is not ruled.
- The coordinator's statement that the user has started M2 is taken as given; nothing in the tree says so yet
  (`AGENTS.md:379`, memory `m1f-closed-work-stopped`).
- External claims (models, llama.cpp and ollama APIs, BBH, acva) were not checked.
- I did not re-take M10's, M15's or M5's own stale claims beyond the ones that touch M2.
