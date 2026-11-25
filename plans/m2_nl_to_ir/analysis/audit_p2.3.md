# M2 analysis — P2.3, the benchmark (Stage C), against the repository at `458bf90b`

Audited: `plans/m2_nl_to_ir/p2.3_benchmark/README.md` and its four stage files
(`s2.3.1` … `s2.3.4`), read whole; for context `plans/m2_nl_to_ir/README.md` and
`EinAf.md` § Stage C. Read-only: nothing in the repository was edited, no `cargo`,
`./build.sh` or `./run_tests.sh` was run. The binary `ein.rs/target/release/ein`
was run on corpus programs only; its output files are under
`(scratch, not kept: re-run the commands) p2.3_runs/`
(`11.summary.json`, `zebra2*.json`, `wp.report.json`, `features.report.json`,
`oracle_11/`). No program outside the corpus was run, so where a point would need
a probe program I say *not probed*.

Short names below: `README` = the phase README, `s1`…`s4` = the stage files.
Wall times quoted are from this session, not taken under `utils/bench_env.sh`;
read them as orders of magnitude only.

---

## A. Stale claims

Each row: plan file:line, the quote, what is true today, the evidence. Repeats of
one claim are one row.

### A.1 The command line and the machine outputs

| # | where | the plan says | today | evidence |
|---|---|---|---|---|
| 1 | README:6, :76, :107, :113; s1:52, :111, :120, :138, :163; s2:7, :35, :40, :117, :160; s4:52, :84 | certification is "`ein solve -e`" / "`ein solve -e --json-summary`" | `-e` is retired and refused, exit 2. A bare `ein solve FILE` describes R; the questions are `--single`, `--maximal`, `--minimal`, `-n N`, `--count` | `ein solve -e examples/features/10_expect.ein` → `error: -e/--exhaustive is retired: \`ein solve FILE\` describes all of R, which is exhaustive, and \`--maximal\` gives the maximal line…`, exit 2; `ein.rs/crates/ein-cli/src/solve.rs:21-27`; `docs/ein/user/04-reference/05-cli.md:53-99` |
| 2 | README:78-80, :110-112; s1:6-8, :51, :165 | "`ein test` — M1c's, not shipped yet"; "the day `ein test` ships"; "once M1c ships" | shipped. `ein test PATH…` is one of the six subcommands; takes files or directories; exit 0 all held, 1 a claim false or not checked, 2 load error or nothing to check | `ein test --help`; `05-cli.md:122-217`; `ein test examples/word_problems/` → `20 files, 31 expectations: 31 held…`, exit 0 |
| 3 | s1:52-53, :112; s2:36 | the harness "compares `verdict.type`, `k`, `exhausted` and the sorted `solutions`" of the summary | the summary is `ein-summary/2`. No `verdict.type`, no `k`, no `solutions`. `verdict` is `null` under describe and `{"word": …, "which": …}` under `--single`; the counts are `region.k_s` and `region.k_c`; `region.exhausted` with `region.stopped`; the states are `region.states[]`; a query's answer is `queries[].classes[].rows[]` | `p2.3_runs/11.summary.json`, `zebra2.single.json` (`{'word': 'Solution', 'which': 'all'}`); `docs/ein/user/04-reference/08-read-out.md:543-634` (§ 11) |
| 4 | README:32; s1:74 (`{"k": n}`); s2:35 ("`k = 1 ∧ exhausted`"), :36 | one count `k`, the number of models | two counts: `k_s` states and `k_c` commitments (`zebra2.ein`: k_s 1, k_c 261 982). Where a program states restrictions only, ∅ and partial commitments are in R: `features/11_expect_ambiguity.ein` has k_s = 7 and a maximal line of 2, and its claim is `:over maximal`. "How many answers" is therefore the maximal line's size or a query's classes, not one `k` | `ein solve examples/zebra2.ein`; `ein solve examples/features/11_expect_ambiguity.ein`; `12-query.md:205-223` |
| 5 | README:34-36, :72-75; s2:33-37; s3:47-49 | three verdicts: *unique*, *ambiguous*, *unsat*; "`Contradiction` means True" | `--single` answers one of four words, `Solution`, `Ambiguity`, `Contradiction`, `Open`, or says the run stopped before it decided. An empty R is of two kinds: every node dead (`Contradiction`) or every alive node owes (`Open`). Corpus programs whose statement is "no solution" answer `Open`: `binary_logic/unsatisfiable.ein`, `beyond_arith/equation_no_solution.ein` | `05-cli.md:74-78`; `ein_infer::explore::Verdict`, `ein-infer/src/explore.rs:458-466`; `ein solve … --single` on the two files → `Open — R is empty: every alive node owes` |
| 6 | s2:40-43; s1:120-122 | a capped run shows "`Contradiction` with an empty core and `exhausted = false`, the Q-M1d.6 signature" | Q-M1d.6 is decided and built: `Contradiction` is said only where no node is alive; a stopped run says *not decided — the run stopped first* | `docs/decisions/answer/Q-M1d.6-contradiction-is-said-only-where-no-node-is.md` |
| 7 | s2:43 | "an `Aborted` under `--max-time`" | `-T/--max-time` exists; an abort prints `** aborted: --max-time S s reached before the question was decided **`, exit 2, nothing on stdout and **no summary**. No `Aborted` word in a summary. (An enum `VerdictKind::Aborted` exists in `ein-einb/src/solutions.rs:68`; I did not trace whether any command writes it.) | `08-read-out.md:539` |
| 8 | s2:46-47 | "Nothing about the benchmark depends on M1d landing; something about its upper size does" | M1d landed. The caps today are `--region-budget` (default 2²⁰ facts beyond root), `--region-limit` (16 candidates, the oracle's and a flagged program's cube) and `-T` | `ein solve --help`; `docs/decisions/cli/Q-M1e2.35-the-default-region-budget-is-2-20-facts.md` |
| 9 | s2:84-87 | the unit of precision / recall is "the `ein-ir` dump, whitespace-normalised" | `ein_ir::dump::dump_canonical` is a library function; no `ein` subcommand prints it (`ein show` has `hyps` only) | `ein-ir/src/dump.rs:170`; `ein show --help` |

### A.2 The language: how a program states its answer

| # | where | the plan says | today | evidence |
|---|---|---|---|---|
| 10 | s1:45-50 | "`(or S1 S2 …)` for an ambiguous instance, the empty expectation (or `none`, Q-M1c.2) for an unsatisfiable one" | a query's claim is `(model …)`, `(or (model …) (model …))` or `(false)`. There is no `none`. The empty `(model)` is *the class where the goal has no row*, not "unsatisfiable" | `ein-ir/src/ein.y:350-355`; `12-query.md:283-299`; `docs/decisions/language/Q-M1c.2-…md` |
| 11 | s1:45-57; README:154 | one claim site, `(query … :expect …)`, "M1c's form" | two sites. `(solve … :expect (region <model>* [(merged …)] [(owing …)]))` or `(false)` claims R by its states; `(query … :expect …)` claims a goal's projection classes over a part of R chosen by `:over all|minimal|maximal`. `(region (owing (model …)))` is how "R empty, alive nodes owe" is stated; `(false)` on a region is the dead kind only, on a query either kind | `12-query.md:239-299`; `ein.y:385-402`; `tests/region/04_owing.ein`, `05_empty.ein`, `06_false.ein` |
| 12 | s1:54-57 | "If M1c decides a different form (Q-M1c.1), the benchmark follows it" | decided; the decision file exists | `docs/decisions/language/Q-M1c.1-a-query-states-the-answer-it-expects-with-expect.md` |
| 13 | s1:48-50 | "the verdict is *implied by the expectation* … ([S1c.1.2 § (c)])" | the link lands on `12-query.md#claims`, which has no § (c): the label names a section of a stage record that is deleted. And one listed model implies "unique" only for the part `:over` names | `12-query.md:239` ff. |
| 14 | s1:32-33, :132-133 | "`gold.ein` is a plain kernel input — no metadata inside it beyond `:source` and `:expect`, both of which the kernel already accepts or is about to" | both are accepted. A program also carries `:why` on every relation, rule and macro, or it is refused unless `--no-require-why` | `ein solve --help` (`--no-require-why`); `docs/decisions/corpus/Q-M1f.57-…md`; `02-syntax.md:79-84, :99` |
| 15 | s1:88 | ordering anchor: "`(includes before immediately-before)`" | `(includes ?p ?q)` copies `?p` into `?q` ("`(includes is-a is-a*)` reads *is-a\* includes is-a*"). As written the activator would copy `before` into `immediately-before` | `stdlib/algebra.ein:427-433`; `examples/zebra2.ein:67-68` (`(includes right-of next-to)`) |
| 16 | README:58; s1:92; s2:133 | set / category theory is "`std.typing`, `is-a`" ("`is-a`, disjointness") | `std.typing` holds a converse-typecheck driver and the `reflexive` closure, nothing else. Subsumption is `std.algebra`'s `transitive` / `includes` over a relation the program names. No stdlib rule states that two classes are disjoint; `type-exclusivity` is a program's own rule. `is-a` is no library or kernel word | `stdlib/typing.ein`; `grep -n -i disjoint stdlib/*.ein` (none); `examples/saturation/type-exclusivity/`; Q-M1f.436 |
| 17 | s2:69-71 | "Six families instantiate a `std.*` module by activators and the program's theory part is a list of imports and property facts" | not true of the attribute-matching reference program: `zebra2.ein` defines 15 rules of its own beside its two imports; `zebra.ein` defines one, the hypothesis rule `guess`. A program that searches also has to say what is hypothesised (a rule concluding `(open F)`, an obligation, or the `(solve …)` lists) | `grep -c '^(rule ' examples/zebra2.ein` → 15, `zebra.ein` → 1; `examples/README.md:39` |
| 18 | s3:46-50 | "run `T + (not C)` — `Contradiction` means **True**" | a fact beside its stored negation kills a state only where a rule says so: the program must import `std.contradiction` (or `std.algebra` / `std.bijection` / `std.slots`, which do) and declare the relation; otherwise the state is alive | `docs/decisions/stdlib/Q-M1f.557-…md:3-18`; `examples/beliefs/05_liar_denial_beside_the_fact.ein` |
| 19 | s3:44-45 | "Ein's verdict is the model count of the *whole* theory" | the answer is a region R; a count is `--count`'s, a word `--single`'s | `docs/decisions/answer/Q-M1e2.7-…md`; `05-cli.md:63-85` |
| 20 | s3:50-52 | "`(absent …)` … ([`absent_semantics.md`])" | the page is `docs/ein/reasoning/02-kb-evolution/03-absent-and-naf.md`; `absent_semantics.md` is deleted. The link was re-aimed, the label not | `git log` `a36ded94` |

### A.3 The corpus, the generators, the other plans

| # | where | the plan says | today | evidence |
|---|---|---|---|---|
| 21 | README:157 | "`examples/gen_zebra2_variants.py` — the one generator the repo already has" | the path is `utils/gen_zebra2_variants.py` (moved in `c965d070`); it is not the one generator: `utils/` holds `gen_arith_examples.py`, `gen_arith_tables.py`, `gen_gates.py`, `gen_num9.py`, `gen_random_programs.py`, `gen_send_written.py`, `gen_stdlib_rule_pages.py`, `region_synth.py`, `fuzz_engine.py` | `ls utils/`; `utils/README.md:82-101` |
| 22 | s2:23-29; s1:99-104 | it "derives `zebra2-minus-15.ein` … and `zebra2-bad.ein` … by removing one `:source`d fact and adding one — *exactly one `:source`d fact, an identical schema + rule set*, the invariant its docstring pins"; "the three programs the repo already generates from one another"; "a generator of exactly S2.3.2's shape" | it writes **four** variants; two of them delete the hypothesis rule and its five activators, no `:source`d fact. The docstring says "exactly one block added or removed, and an *identical* schema + rule set". It replaces verbatim text blocks of `zebra2.ein`; it draws no model, runs no engine, certifies nothing and realises no English | `utils/gen_zebra2_variants.py:1-32, :150-170`; `utils/README.md:90` |
| 23 | s1:101-102 | the triple is "already certified" | none of `zebra2.ein`, `zebra2-minus-15.ein`, `ein-bugs/zebra2-bad.ein` states an `:expect` (`ein test examples/zebra2.ein` → `(no expect)`, exit 2). Their answers are held by Rust tests (`ein-infer/tests/acceptance.rs`) and a corpus note. A bare `ein solve examples/zebra2-minus-15.ein` at the default budget ends **`exhausted: no`** (k_s = 32 established, k_c a lower bound, about 10 s); `--maximal` ends exhausted with 32 states in about 0.1 s | `grep -c ':expect'` → 0 on each; `p2.3_runs/zebra2-minus-15.summary.json` (`exhausted False`, `stopped.why = budget`); `corpus/corpus.toml` (the entry's note, `slow = true`, `cost_ms = 10798`) |
| 24 | s1:97, :147 | Zebra's text "is S2.2.5's `examples/zebra.txt`" | no such file (S2.2.5 plans to add it). A text of the puzzle, a line a sentence, is `nl2smt/puzzles/zebra.txt`, with `zebra-minus-15.txt` and `zebra-contradiction.txt` beside it | `ls examples/*.txt` (none); `nl2smt/puzzles/` |
| 25 | s1:117 | "`benchmark/README.md` is a catalog in `corpus/README.md`'s style — one line per entry" | `corpus/README.md` documents the manifest; the one-line-per-file catalogue is `examples/README.md` (which M10's S10.1 names for the same purpose) | `corpus/README.md`; `examples/README.md`; `plans/m10_external_benchmarks/s10.1_problem_corpus.md:85-87` |
| 26 | s1:37-39; README:117-118; s4:146-148 | "`bench/` is M10's — the six-system corpus and `ein-bench`'s results (S10.1)" | neither `bench/` nor a crate `ein-bench` exists (eight crates, none of that name). Where M10's benchmark lives is M10's **open** question Q-M10.3, with a recommendation; `ein-bench` is S10.3's and Q-M10.3's word, not S10.1's | `ls bench` (absent); `ls ein.rs/crates`; `plans/m10_external_benchmarks/open_questions.md:73-93` |
| 27 | s3:23-27 | the sources are "catalogued in `docs/lib/12` § 1 and § 3" | BBH is § 1; ProofWriter, FOLIO, LogiQA, ReClor are § 3. **CLUTRR is not in `docs/lib/` and never was** | `grep CLUTRR docs/` (none); `git log -S CLUTRR -- docs/lib` (empty) |
| 28 | s3:127-129 | LogiQA and ReClor are left out because "the user's ladder does not name them" | F13's note names both, in its second ladder: "CLUTRR → FOLIO → LogiQA → ReClor". (F13 is a raw note; which lines are the user's ladder is the plan's reading. Unsure.) | `plans/followups/f13_puzzles_beyond_zebra/ideas.md:56-57, :71-72` |
| 29 | s4:84 | `runs = ["solve -e --json-summary {out}/summary.json"]`, "the corpus's" column | `-e` as row 1; and in the corpus's convention every `solve` run already gains `--json-summary {out}/summary.json` | `corpus/README.md:84-90` |
| 30 | s4:89 | "(`corpus/README.md` § runs)" | no section of that name; the text is under § Manifest format | `grep -n '^## ' corpus/README.md` |
| 31 | README:154; s1:7, :45, :115 | "M1c S1c.1.2", "S1c.1.2's form" | the stage record is deleted with M1c's plans; the claim forms are stated in `12-query.md` and Q-M1c.1 / .2 / .6 / .7, Q-M1f.219, .221, .222 | `plans/README.md:359` |
| 32 | README:56 | object tracking is "the first family Ein's saturator does not obviously fit" | the corpus since holds state change over named moments, computed by saturation alone: the word problems' `(gave WHO WHOM KIND N BEFORE AFTER)`; and long chains (`arith/factorial.ein`, `beyond_arith/collatz.ein`). No swap puzzle exists | `examples/word_problems/README.md` § The model's vocabulary |

Not of P2.3's five files but read for context, one line: the M2 README's
"S1f.6.20 … its results, when it has run" (`plans/m2_nl_to_ir/README.md:258`) —
it has run; the programs are `examples/word_problems/` (§ B.4).

**Checked and found true: 36 claims.** The corpus manifest's schema is
`ein-corpus/2` (`corpus/corpus.toml:29`); one entry a file with its runs, the
argv with the file elided and `{out}` substituted; `cost_ms` / `slow` measured
and held both ways; the completeness check; `std.algebra` has `transitive`
(:422), `includes` (:430), `converse` (:55), `symmetric` (:392); `std.bijection`
and `zebra2.ein` imports it (:113); `std.slots` and `zebra.ein` imports it
(:145); `std.typing` exists; no stdlib module for state or for intervals;
`zebra2-minus-15` has 32 states; `zebra2-bad` is a contradiction;
`gen_zebra2_variants.py` exists with `--check`; the repo keeps both Zebra
encodings and says why; Q17 is decided as the plan reads it; F13 carries the
ladder, the BBH record format, 3 / 5 / 7, tracking as the test of genericity
(:552), the four oracle conditions (:519-524), ARC excluded as not text (:51);
F8 names Allen (:351, :418; the "second rung of `C(n)`" wording not checked);
`docs/lib/12` § 1, § 3 and § 7 say what is quoted of them; idea 09, F7, M5, M10
exist; M10 is "formal-language shaped, not M2-gated" in its own non-goals;
Q-M2.1 and Q-M2.3 are open with the deciding stages named; Q-M1c.1, Q-M1c.2,
Q-M1d.6 files exist; `utils/bench_env.sh` exists; `:source` is accepted on a
fact and printed beside an unsat core; `--json-summary` and `region.exhausted`
exist; `-T` exists; every EinAf anchor cited exists; **all 101 relative links
of the five files resolve, anchors included** (script, this session); S2.2.5
names `examples/zebra.txt` as the plan says.

---

## B. What each stage needs from the project, and its state today

### B.1 EinAf § C1's eight families against the corpus

"NL" = where an English statement of the problem sits. Outside attribute
matching, the only instances below that state a claim are `beliefs/01` and the
rule tests under `tests/stdlib/`; the single-rule demonstrations write their
rule in the file and import nothing.

| family | corpus programs that are instances | NL statement | claim | theory |
|---|---|---|---|---|
| logical deduction (*A older than B …; who is youngest?*) | `examples/saturation/transitive/{colocation-chain,taxonomy,mealtimes}.ein`: one application of transitivity each, two facts in, one out. **No program asks for an extremum.** | prose in the header comment; `:source` is a label (`"demo"`, `"(1)"`) | none (`(query :goal …)` only) | the rule `transitive` written in the file |
| ordering | `transitive/mealtimes.ein` (three meals); the Zebra row (`right-of` ×4 with `(includes right-of next-to)`); `implies/right-then-next.ein`. **No arrival-order puzzle, no positional question.** | header comment | none | own rule; Zebra: `std.algebra` |
| spatial | one order only: the Zebra family's `right-of` / `next-to`; `saturation/square-fwd|bwd/{houses,floors}.ein`; `square-unique/{corner-house,cul-de-sac,terminus}.ein` (all three flagged); `hypothesis-contradiction/{right-of,next-to}-disproved.ein`; `features/09_adjacent_via_same_house.ein`; `tests/stdlib/slots/07, 08, 13`. **No two-order (left-of and above) program**; `converse` appears only in `tests/stdlib/algebra/01, 06, 16, 27` and `tests/stdlib/typing/02, 03` | header comment | none in `examples/`; the `tests/` ones carry rule-level claims | own rules or `std.slots` |
| object tracking | **none.** Nearest: transfers over named moments in `word_problems/{bob1,bystander,unused,before,pencils}.ein`; chains in `arith/factorial.ein`, `beyond_arith/collatz.ein` | — | — | — |
| attribute matching | `examples/zebra.ein`, `zebra2.ein`; generated `zebra2-minus-15.ein`, `zebra2-obligations.ein`, `zebra2-minus-15-obligations.ein`, `ein-bugs/zebra2-bad.ein`; `zebra2-hints.ein`, `ein-bugs/zebra2-hints.ein`. Small ones: `branching/05_mini_zebra.ein` (2 × 3), `domain_elim/{ab,b_only,b_branch}.ein` (3 × 3), `features/14_unicode_text.ein` (3 people, 3 houses), `features/11_expect_ambiguity.ein` (two seatings), `features/12_expect_false.ein` (pigeonhole), `lattice/06_friends_tables_everyone_sits.ein`, `lattice/07_two_pairs_two_tables.ein` (8 seatings) | Zebra: each clue as a `;;` comment above its fact, `:source "condition (N)"`; the clue-to-fact table is `docs/ein/user/02-guide/06-zebra-walkthrough.md:59-77`; the text as a file is `nl2smt/puzzles/zebra.txt`. `features/14`: Russian sentences as `:source`. Others: header comments | Zebra family: **none**. `features/10`-`14`: yes | `std.bijection` + 15 own rules (`zebra2`); `std.slots` (`zebra`) |
| set / category (*every A is B; no B is C; can x be C?*) | `transitive/taxonomy.ein` (Cat, Mammal, Animal); `saturation/type-exclusivity/{colors,nationalities,pets}.ein`; `beliefs/01_common_belief.ein` (a `forall` over a closed class). **No program asks a *can* question.** | header comment | `beliefs/01` only | own rules |
| graph | `saturation/implies/{parent-to-ancestor,org-chart}.ein`; `saturation/symmetric/{couple,neighbours,friends}.ein`. **No edge list with a reachability or ancestor question; no kinship chain.** | header comment | none | own rules |
| temporal | `transitive/mealtimes.ein`; `square-fwd/meetings.ein`, `square-bwd/meetings.ein` (`before-of` over events); the word problems' moments `t0`, `t1`. **No interval, overlap or containment.** | header comment | none | own rules |

So today's corpus holds a full-size anchor for one family (attribute matching),
single-rule demonstrations for five, and nothing for object tracking. The
single-rule demonstrations are deliberately "the smallest program that
exercises one saturation rule" (`examples/saturation/README.md:3`).

**Families the corpus holds that § C1 does not list** (each with claims, all
monotone by `ein solve FILE --monotone`): arithmetic word problems (16, § B.4);
equations and cryptarithms by search (`beyond_arith/equation_by_search.ein`,
`equation_two_unknowns.ein`, `equation_no_solution.ein`, `cryptarithm.ein`,
`send_more_money.ein` and its generated twin); Boolean circuits and truth-teller
puzzles (`binary_logic/`, 12 programs: `knights.ein` is Knights and Knaves with
its statement in the header, `satisfiable` / `unsatisfiable` / `equivalent` /
`not_equivalent`); number theory by saturation; beliefs and contexts
(`beliefs/`, 11). F13's BBH list has counterparts for some (`boolean_expressions`,
`multistep_arithmetic`); the plan's table has no row for them.

**Sentence-level `:source` texts.** Counting programs whose `:source` strings are
sentences (three words or more, ending in `.`/`?`/`!`; script, this session):
17 in the whole corpus — the 16 word problems and `features/14_unicode_text.ein`.
Everywhere else `:source` is a label or absent.

### B.2 The "five puzzles" seed set

| old puzzle | exists? |
|---|---|
| Zebra | yes: two encodings, six variants and fixtures. No `:expect`; `:source` strings are `"condition (N)"`, so s1:115-116 ("every `:source` string … is a substring of `text.md`") does not hold of `zebra2.ein` as it stands, the program s1:147 names |
| the Einstein variant | no `.ein`. (`nl2smt/smt/einstain-problem.smt` is the same Zebra puzzle in SMT-LIB, not a variant) |
| logic grid A (4 × 4, another ontology) | no |
| logic grid B (5 × 5 with spatial constraints) | no (Zebra itself is the only 5 × 5) |
| the ambiguity stress (four sentences, one ambiguous) | no. Nearest in kind: `word_problems/boxes13.ein`, a word with two readings, two states and two answers, with its sentences as `:source`; `features/11_expect_ambiguity.ein`, no text |

The five were planned by the old P2.6 and never written; `git log -- examples`
shows no such files added or deleted.

**The Zebra unique / ambiguous / unsat triple exists three times over**, in forms
that do not line up:

- as programs: `zebra2.ein` (`--single` → `Solution`), `zebra2-minus-15.ein`
  (`Ambiguity`; 32 states), `ein-bugs/zebra2-bad.ein` (`Contradiction`; core
  `(color-loc Green House-1)`, `(is-a House-1 House)`, `(relation color-loc Color
  House)`, `refuted by (given), co-located-negative`);
- as texts: `nl2smt/puzzles/zebra.txt`, `zebra-minus-15.txt`,
  `zebra-contradiction.txt`. The contradiction text adds *The Norwegian drinks
  milk.*, while `zebra2-bad.ein` injects Green in House-1: **the unsat text and
  the unsat program are different contradictions**;
- as an independent count: `nl2smt`'s pipeline runs the three texts through an
  SMT solver, and its README reports one solution, 32 solutions "and no other"
  (`--solutions 40`), and none (`nl2smt/README.md:496-531`,
  `nl2smt/test_pipeline.py`). `nl2smt/` is external to project and process and
  the gate does not run it.

### B.3 How a program states its answer today, and what certifies it

What the plan calls the verdict classes, as they can be written and checked now:

| class | how the corpus states it | example | what `ein test` asks |
|---|---|---|---|
| unique | `(query … :expect (model …))`; or `(solve :expect (region (model …)))` | `word_problems/bob1.ein`; `features/10_expect.ein` | `--single` over the query's part |
| several answers | `(query … [:over maximal] :expect (or (model …) (model …)))`; or a region listing each state | `features/11_expect_ambiguity.ein`; `word_problems/boxes13.ein`; `binary_logic/knights.ein` | `-n k+1` over that part |
| no solution, contradiction | `(solve :expect (false))` | `features/12_expect_false.ein`; `word_problems/contra.ein` | describe |
| no solution, every alive node owes | `(solve :expect (region (owing (model …) …)))`, a query's `(false)` beside it | `binary_logic/unsatisfiable.ein`; `beyond_arith/equation_no_solution.ein`; `tests/region/04_owing.ein` | describe |
| discharged and owing states together | a region listing the states of R | `features/13_mixed_solution_and_open.ein` | describe |
| not enough information, the gap named | a query whose class is `(model)` beside a query claiming a derived `(unanswered Q G)` | `word_problems/gap.ein`, `kinds.ein`, `before.ein` | `--single` |

Two things in that table the plan's three classes do not have a place for:

- **`Open`**, the second kind of empty R. A generator that "adds one clue false
  of the model" gets `Contradiction` or `Open` depending on whether the theory
  refutes or only owes; the two are different words, different claims, and only
  the dead kind has a core.
- **The kernel's word is not the problem's class.** `--single` on `gap.ein`,
  `kinds.ein` and `before.ein` says `Solution` (k_s = 1) while the problem's
  right answer is *not enough information*, carried by a derived fact; on
  `boxes12.ein` it says `Ambiguity` (two states) while the problem has one
  answer (one projection class over both states). The class is read off a
  query's classes and facts, not off R's count.

**What "exact ground truth" can be taken from today:**

| source | what it gives | reach |
|---|---|---|
| `ein oracle FILE [--dump DIR]` | R by brute force, every subset of H saturated, no licence; `region.json` / `region.md`; the reference every mode is held to (`ein-infer/tests/explore.rs`) | default `--region-limit 16` candidates, refused above (exit 2): `zebra2.ein` has 56, `send_more_money.ein` 80. `features/11`: 4 candidates, 16 saturations |
| `ein solve FILE` (describe) with `exhausted: yes` | all of R over the universe H, on a monotone program | bounded by `--region-budget`; the `outside` row / `root.hypgen.outside` names relations no candidate is of, so the certificate's scope is H (Q-M1e2.3); on a flagged program nothing is claimed |
| `ein solve FILE --single` | one of four words | `zebra2`, `zebra`, `zebra2-minus-15`: well under a second here; `send_more_money` about 0.4 s |
| `ein solve FILE --maximal` | the maximal line, exhaustively | `zebra2-minus-15`: 32 states, exhausted, about 0.1 s |
| `ein solve FILE --count` | k_s, k_c | `zebra2-minus-15`: stops at the default budget with k_s established |
| `ein test PATH… [--json-report F] [--diff full]` | each claim held / FAILED / NOT CHECKED, the question it was read off, `ein-test-report/6` with one row a claim | any selection; a claim the budget leaves undecided is NOT CHECKED, exit 1 |
| `ein solve … --dump-answer DIR` | every state as a loadable file, `answer.json` | — |
| second routes outside the engine's code | `utils/region.py` (one `ein saturate` a subset), held to the oracle by `utils/check_region_routes.py`, a gate step | small programs |
| by construction | `region_synth.py` writes a program for a chosen region with the target in its header; `gen_arith_examples.py` computes claims with Python's integers | abstract candidates; arithmetic |
| other systems | `nl2smt` on the three Zebra texts (above). M10 has not run | Zebra only |

So a certification step exists in a different and wider form than the plan's
single `-e` run with `exhausted = true`. A certificate today also has to read
`monotonicity.monotone` (a flagged program's answer is claimed by nobody) and
`root.hypgen.outside`, and "exhausted" depends on the question: the same
program is `exhausted: no` under describe and `yes` under `--maximal` (row 23).

### B.4 The word problems M1f wrote

`examples/word_problems/`, 20 files: four theory modules (`words.ein`,
`gaps.ein`, `readings.ein`, `listed_is_all.ein`) and **16 problems**. Every
problem: the full English statement as its header comment; each fact with the
sentence it encodes as its `:source` (16 of 16, 3 to 7 per file); the question
as an `(asked …)` fact with its sentence; one to three `:expect` claims (31 in
all, `ein test examples/word_problems/` → 31 held); `(query … :goal-text …)`
that reads the answer as a sentence; all monotone; all corpus entries with a
`test` run (Q-M1f.197). The table of problems with answers is
`examples/word_problems/README.md`; the guide is
`docs/ein/user/02-guide/08-word-problems.md`; the decisions are Q-M1f.194 to
.198 and .329 to .335 under `docs/decisions/corpus/`.

| class, as F26 item 6 names them | problems | `--single` |
|---|---|---|
| complete, one answer | `bob1`, `gap_fact`, `kinds_full`, `more`, `pencils`, `unused`, `bystander` | `Solution` |
| one answer resting on a marked assumption | `gap_assume`, `kinds_assume`, `kinds_listed` | `Solution` |
| under-determined by a quantity, the gap named | `gap`, `kinds`, `before` | `Solution` (the gap is a fact) |
| under-determined by a reading | `boxes13` (two answers), `boxes12` (two states, one answer) | `Ambiguity` |
| contradictory | `contra` (core: the two sentences, the question and the model's assumption) | `Contradiction` |

Numbers in them are the calculator's numerals, `(n z 5)`, `(n (n z 1) 2)` for
12; a quantity is a nested fact, `(amount Bob apples t1)`. The theory modules
are imported by file name from the problems' own directory.

P2.3's five files do not mention the word problems; only the M2 README does
(§ The reasoning as a program), which says "no phase or stage is changed by it
yet".

### B.5 Stage by stage

**S2.3.1 — families and the seed set**

| needs | state today |
|---|---|
| the `:expect` form | exists, wider (rows 10-11) |
| `ein test` to re-check | **already shipped**: the "until it is" branch of T2.3.1.5 (compare a summary to `meta.json`) is moot for the kernel half; `ein test DIR --json-report` gives per-claim rows. What does not exist: comparing `:source` to `text.md`, and `meta.json` itself |
| a summary with `verdict.type`, `k`, `solutions` | exists in another form, `ein-summary/2` (row 3) |
| stdlib theories for six families | exist for five (`std.algebra`, `std.bijection`, `std.slots`); for set / category in part (row 16) |
| hand anchors for eight families | one family has a real anchor; § B.1 |
| the old five puzzles | one of five exists; § B.2 |
| Zebra's text | not where the plan puts it; exists in `nl2smt/puzzles/` |
| the Zebra triple "already certified" | exists; states no claim; § B.2 |
| a `gold.ein` with no metadata beyond `:source` and `:expect` | `:why` is required too (row 14) |
| `benchmark/` beside M10's `bench/` | neither exists; § C.1 |
| a catalogue style to copy | `examples/README.md` |

**S2.3.2 — the generators**

| needs | state today |
|---|---|
| a certification command | exists as several questions; § B.3 |
| "exhausted" as the admission test | `region.exhausted` with `region.stopped`; depends on the question; plus `monotonicity` and `outside` |
| the kernel's unsat core with `:source` spans as recorded truth | `region.core`, `region.refuted_by` and the read-out's `source` rows exist under describe; under `--single`, a line mode and `--count` they are `null` (Q-M1f.29); a core is not canonical (§ C.4) |
| `gen_zebra2_variants.py` as the frame's seed | exists in `utils/`, a text splicer over one file (row 22) |
| canonical text of a form | a library function only (row 9) |
| "the default caps" | `--region-budget` 2²⁰, `--region-limit` 16 |
| theories for object tracking and temporal | still none in the stdlib. **M1h's P1h.1** plans a temporal and a causal theory as stdlib modules, on the user's reply of 2026-11-12; not started (`plans/m1h_beyond_word_problems/p1h.1_temporal_and_causal_theories/`) |

The existing generators, against the plan's:

| script | what it generates | as the plan's generator |
|---|---|---|
| `utils/gen_zebra2_variants.py` | four files from `zebra2.ein` by replacing verbatim blocks; `--check` | the delete-a-clue / add-a-clue idea only; hard-wired to one file's text. T2.3.2.2 "port into the frame" would give two of its four outputs a second writer (§ C.2) |
| `utils/region_synth.py` | a program for a chosen region: a status (in R, owing, dead) per node of the cube over candidates `(h A)`, `(h B)` …, as `(false)` rules and `(open h)` obligations; target in the header; `--check` against the oracle; `--random N --seed S` | exact R by construction, including the owing kind and holes, deterministic by seed; but abstract: no family, no ontology, no sentence. It could supply verdict-class skeletons, not instances |
| `utils/gen_arith_examples.py` | three shipped arithmetic programs, and on demand one of a stated size (`--op … --digits N --count N --seed S -o FILE`), claims from Python's integers; same arguments, same bytes; written under `build/`, never checked in, run by no tier | the nearest precedent for the plan's shape: program first, an oracle outside the engine, sizes as arguments, determinism |
| `utils/gen_random_programs.py` | `ein-infer/tests/golden/random_programs.txt`, 200 `region_synth` programs as one test fixture | precedent for "a banked generated sample the gate checks" |
| `utils/gen_send_written.py` | `send_more_money.ein` with its letters declared in another order | a paraphrase at the level of the program: one answer, two written orders |
| `utils/gen_gates.py` | stdlib rows and the programs under `tests/stdlib/bool/`, claims computed from the columns | claims computed outside the engine |
| `utils/fuzz_engine.py` | generated or mutated programs for six run properties (no crash, determinism, id order, jobs …) | no ground truth; not a benchmark generator. Its `EIN_ID_FILES` seam points a corpus sweep at a directory outside the corpus |

None of them realises English, and none conflicts with the plan's generators
except the Zebra port.

**S2.3.3 — external benchmarks**

| needs | state today |
|---|---|
| the datasets | none in the tree (no BBH, CLUTRR, ProofWriter, FOLIO, LogiQA, ReClor, bAbI file; `git ls-files` has no dataset file). bAbI is named nowhere |
| the catalogue | `docs/lib/12-llm-and-reasoning-benchmarks.md`: BBH § 1, ProofWriter / FOLIO / LogiQA / ReClor § 3, no CLUTRR (row 27). `docs/lib/` is external to project and process |
| the ladder | `plans/followups/f13_puzzles_beyond_zebra/ideas.md`, last changed 2026-08-20 |
| the two-run True / False / Unknown contract | needs `std.contradiction` imported for `(not C)` to refute (row 18); the answers are four words, not three; a closed-world *False* by `(absent …)` meets § C.3 |
| licences and versions | nothing recorded anywhere in the tree |
| S2.2.2's download pattern | planned, not built |

**S2.3.4 — splits and the manifest**

| needs | state today |
|---|---|
| the corpus's manifest form | exists as the plan says: `ein-corpus/2`, `runs`, `cost_ms`, `slow` (threshold 1 s, the sum of the runs), also `heavy_dump`, `levers`, groups by directory; the completeness check walks `examples/`, `stdlib/`, `tests/` only (`ein-corpus/src/lib.rs:303`, `manifest.rs:208-210`) |
| `bench_env.sh`'s banner | exists; it also reports the power supply and has `--require-ac` |
| the root README's layout table | exists (`README.md:213-227`); it lists the project's paths and, apart, "outside the project, as references" `c/`, `nl2smt/`, `docs/lib/` |
| P2.2's one-shot formalizer for the Level B run | not built; no `einaf` code exists anywhere outside `plans/` |
| a CI step | the gate's step lists are held equal by `utils/check_gate_steps.py` |

---

## C. Conflicts with standing rules and decisions

### C.1 Where the benchmark, its generators and external data live

AGENTS.md § Edits that go together and
[Q-M1f.345](../../../docs/decisions/docs/Q-M1f.345-nothing-in-the-project-reads-links-or-names.md)
divide the tree three ways: the project (`README.md`, `ein.rs/`, `stdlib/`,
`examples/`, `tests/`, `corpus/`, `docs/` but `docs/lib/`), the process (`plans/`,
`utils/`, the gate, CI, AGENTS.md), and what is external to both (`c/`, `nl2smt/`,
`docs/lib/`). The plan's `benchmark/` (README:117; s1:30) and `einaf/` (s2:95;
s3:93) are new top-level directories in none of the three. What follows from each
choice, as the checks stand:

- **Left unclassified**, a tracked `benchmark/**` is the project's to
  `utils/check_release_voice.py`: its `NOT_PROJECT` is a deny-list (:63-66) and
  every other tracked `.md` and `.ein` is in scope (:110-117). The sentence s1:117-119
  requires of `benchmark/README.md`, "`bench/` is M10's", fails it (a milestone
  id), as would any `P2.7`, `S2.3.4`, `F13` or ISO date in a benchmark page or a
  `gold.ein` comment. `utils/doc_audit.py --links --check --tracked` reads its
  links too.
- **Under a corpus root** (`examples/`, `tests/`, `stdlib/`): every `.ein` needs a
  `corpus/corpus.toml` entry or the completeness check fails; its group is fixed
  by its directory (`manifest.rs:464-487`); every corpus sweep reads it (the
  shape golden `corpus_shapes.md5`, `dot_wellformed`, `answer_round_trip`,
  `renaming_invariance`, `import_invariance`, …); the slow set is pinned by name
  (`the_slow_set_is_exactly_these_sixteen`); and corpus programs are sized so
  that a bare `ein solve` describes R within the default budget
  ([Q-M1f.166](../../../docs/decisions/corpus/Q-M1f.166-the-generated-arithmetic-examples-are-sized-for.md),
  [Q-M1f.187](../../../docs/decisions/corpus/Q-M1f.187-each-program-beyond-arithmetic-is-sized-for.md)).
  The plan's 4 080 instances stand against 933 entries today.
- **External, like `nl2smt/`**: wired into nothing, the gate does not run it —
  which s2:109-110 ("re-certified by the gate … green in CI") and README:78-80
  ("on every gate run") contradict.

Related, by the same rule: T2.3.4.4's sentence in the root README's layout
table (s4:146-148) puts a path in a project page; s1:97-99's test holding
`benchmark/…/zebra` equal to `examples/zebra.txt` ties the two trees, and what
ties a process file to the project is a `utils/` check and a gate step, never a
crate test (AGENTS.md § Running the gate).

M10's own ledger asks the same question of its `bench/` and has not answered it
(Q-M10.3, and its sub-question whether such `.ein` files count for the
completeness check).

### C.2 Generated files

[Q-M1f.348](../../../docs/decisions/docs/Q-M1f.348-a-generated-file-says-it-is-generated-and-names.md)
and AGENTS.md § Edits that go together: a generated project file says it is
generated and names no tool; every generator has a `--check` run by
`utils/check_generated.py`; no crate test runs a generator; AGENTS.md's list and
`utils/README.md` are where the script is written down. Against that:

- s1:158-159, "`benchmark/README.md` … Generated by `einaf bench catalog`": if the
  page is the project's, it may not name the command.
- s2:106-108 and T2.3.2.2 (s2:124-128): the attribute-matching generator
  "reproduces `zebra2-minus-15.ein` and `zebra2-bad.ein`" and "the `--check`
  invariant becomes a test". Those two files are among the four AGENTS.md lists
  as written by `utils/gen_zebra2_variants.py`; a second writer in `einaf/`
  would make one generated file two generators', and its check a test rather
  than a `check_generated.py` step.

### C.3 A flagged program states no validated answer

[Q-M1f.564](../../../docs/decisions/answer/Q-M1f.564-a-non-monotone-program-loads-with-a-warning.md):
a program the static check flags loads, every command says in capitals that
nothing is claimed of its answer, no suite validates it, and a flagged corpus
entry declares no `test` run. The check flags a saturation rule whose `absent`
reads a relation a commitment can change
(`docs/ein/reasoning/02-kb-evolution/08-monotonicity.md:91-135`).

The plan's phrase "the benchmark's ground truth is **certified by the kernel,
exhaustively**" (README:108) cannot be said of a flagged `gold.ein`. Three places
propose a closed-world `(absent …)` in a canonical theory:

- s1:87, logical deduction: "the minimum by closed-world `(absent …)`";
- s1:92, set / category: the *can* question;
- s3:50-52, ProofWriter's CWA reading, "what `(absent …)` says on this kernel".

Whether each is flagged depends on its encoding: an `absent` over a relation no
candidate is of is static and fine (the word problems' `gaps.ein` reads `absent`
and all 16 problems are monotone); the same rule beside hypotheses over that
relation, which the family's `ambiguous` sibling needs, is a site. Corpus
examples of each side: `examples/features/01_not_and_absent.ein` is flagged
("absent reads likes at absent depth 1 … at stake: the state");
`examples/word_problems/gap.ein` is not. *Not probed on the plan's anchors*,
which do not exist.

### C.4 A core is an explanation, not an answer

[Q-M1f.6](../../../docs/decisions/answer/Q-M1f.6-a-core-is-an-explanation.md): which core
is reported "is not canonical" and may vary with the rules' order;
[Q-M1e2.11](../../../docs/decisions/answer/Q-M1e2.11-a-core-is-the-smallest-over-every-derivation.md):
`(config :record-alternative-justifications false)` may change it; F22 (open)
is what would make it exact. The plan makes the core the *gold answer* of the
`unsat` class: `answer` = `{"core": [...]}` "what the H1 metrics compare against"
(s1:73-75), "the ground truth: the core" (s2:37). A formalizer's program with
another rule order may report another valid core.

### C.5 `:why` is required

Standing rule (AGENTS.md § The language) and
[Q-M1f.57](../../../docs/decisions/corpus/Q-M1f.57-every-fixture-the-loader-reads-carries-a-why.md):
every relation, rule and macro carries a `:why`, or the program is refused
unless `--no-require-why`. s1:132-133 allows a `gold.ein` "no metadata … beyond
`:source` and `:expect`"; s2:84-87 strips `:source` only from the unit of
precision and recall, so two programs that differ in a `:why` text would differ
as forms. The M2 README (:243-246) already reads `:why` as part of the output.

### C.6 Ein has no numbers

[Q-M1e2.24](../../../docs/decisions/language/Q-M1e2.24-ein-has-no-numbers-and-no-ranges.md),
[Q17](../../../docs/decisions/language/Q17-spatial-structure-is-facts-and-rules-with-no.md).
The plan's family table is consistent with it (README:55, "no arithmetic lattice,
adjacency is extensional"). Where numbers still enter: the ordering anchor's
"who arrived third" (s1:88), the spatial "3 × 3 … centre" (s1:89), the temporal
"at 3 PM" (s1:94) and BBH `temporal_sequences`' clock times (F13:272-300) — each
a named position with stated order facts, or a numeral of `std.math`
(`(n z 3)`), by whatever the instance's theory says; `1` and `01` are two names.
The knobs in `meta.json` are JSON, not Ein. No stage proposes engine arithmetic.

### C.7 The region budget, the slow set, the tiers

- [Q-M1e2.35](../../../docs/decisions/cli/Q-M1e2.35-the-default-region-budget-is-2-20-facts.md)
  and the ruling on Q-M1f.304 (`plans/open_questions.md:75`): no test runs past
  the default `--region-budget`. The plan's admission rule, "within the default
  caps" (s2:153-155), agrees. What it does not have: under `ein test` a claim
  the budget leaves undecided is NOT CHECKED and exits 1, and `exhausted`
  depends on the question (row 23), so an ambiguous instance of Zebra's size is
  admitted or rejected by which question certifies it.
- s2:109-110 and s4:141-142 add gate / CI steps. A step is in `run_tests.sh` and
  `per-commit.yml` alike or `utils/check_gate_steps.py` fails; a nightly step
  stays within five minutes or runs in shards (Q-M1f.293, Q-M1f.102, root
  ledger).
- s4:114-116's "`slow` threshold stated and measured": the corpus's is one
  second over an entry's runs together, measured on AC through `bench_env.sh`.

### C.8 Smaller ones

- **Every answer exits 0** ([Q-M1e2.33](../../../docs/decisions/cli/Q-M1e2.33-every-answer-exits-0.md)):
  `Solution`, `Ambiguity`, `Contradiction`, `Open` all exit 0; only a refuted
  claim exits 1. A certification by exit code is `ein test`'s, not `ein solve`'s.
- **Imports resolve from the program file's directory; `..` is refused**
  ([Q-M1f.252](../../../docs/decisions/imports/Q-M1f.252-a-modules-name-has-no-empty-segment-and-no-slash.md)).
  With one directory an instance (s1:30), a `gold.ein` cannot import a module
  kept once per family in the parent. The plan copies the two synthesis
  families' rules into every `gold.ein` (s2:139-142), which agrees; a
  word-problem instance, whose theory is four modules imported by file name,
  would need them copied beside each instance.
- **The kernel reads no theory's name** (Q-M1f.436): "the unified is-a model"
  (s1:149-150) is `zebra2.ein`'s convention, not the language's.
- **A stdlib rule needs a program under `tests/stdlib/` and a region on its
  rules page**, or the gate fails: what README:129-134's "recorded kernel change"
  would cost if `std.state` or `std.allen` were ever added.
- **No estimates** (the user's ruling of 2026-11-08, `plans/README.md:364-365`):
  § F.

---

## D. Open questions and decisions the phase defers

| question | the plan says who decides | since |
|---|---|---|
| the form of `:expect` (Q-M1c.1) | M1c | **answered**: decided and shipped, then widened by M1e2 and M1f (region claim, `(owing …)`, `:over`, `ein test`'s least question) |
| what a capped run says (Q-M1d.6) | M1d | **answered**: decided and shipped |
| spatial formalisation (Q17) | decided | still decided; now a decision file |
| when the kernel is frozen; stdlib growth as a "recorded kernel change" (Q-M2.1; README:129-134) | S2.5.1 | open. Since: five kernel milestones; work stops after M1f and seven milestones wait on the user's word, each of which would change the kernel or the stdlib (M1g, M1h, M1k, M1n, M1p, M12, M15) |
| a stdlib theory for object tracking and temporal ("never a silent improvement") | the milestone, as a G9 condition | **changed**: M1h P1h.1 makes temporal and causal theories stdlib modules on the user's word; not started. If it runs first, the table's two "none" cells change |
| the unit of faithfulness without a gold theory (Q-M2.3) | S2.4.4 | open, unchanged |
| the harness's language (Q25) | S2.1.3 | open. P2.3 already writes `einaf/benchmark/gen/<family>.py` (s2:95) and `einaf bench …` commands. Since: `ein-summary/2`, `ein-test-report/6`, `--dump-answer`, `ein oracle --dump` as machine outputs; a canonical form dump only as a library function |
| where M10's `bench/` lives, and whether its `.ein` count as corpus (Q-M10.3) | M10 | open; P2.3 writes it as settled, and does not ask the same of `benchmark/` |
| **project, process or external** for `benchmark/` and `einaf/` | not asked by the plan | new since the separation ([Q-M1f.345](../../../docs/decisions/docs/Q-M1f.345-nothing-in-the-project-reads-links-or-names.md)); § C.1 |
| three verdict classes (C3) | EinAf, the user's | **changed around it**: the kernel has two kinds of empty R and four `--single` words; F26 item 6 (2026-11-11) proposes four common-sense classes "for M2" — contradictory, under-determined by a quantity, under-determined by a reading, complete — and "6 stays here, for M2" (`f26…md:30`, `m1g_logic/README.md:421`). No one has mapped the one onto the other |
| which question defines *unique* (all of R, the maximal line, a query's classes) | not asked | new: § B.3 |
| whether M1f's word problems become instances, a family, or a second domain | M2 README: "this milestone's to take up … no phase or stage is changed by it yet" | S1f.6.20 has run; 16 programs exist; undecided |
| whether the unsat instance's recorded truth is a core | s2:37 takes it as given | § C.4 |
| licences of the external sources | T2.3.3.1, "read from the source and recorded, not assumed" | nothing recorded |
| LogiQA and ReClor left out | s3:127-130, "listed here so that the omission is a decision" | row 28 |
| split sizes, "met or the shortfall per cell stated"; size bins "a first cut" | S2.3.4; P2.7 for finer bins | unchanged |
| the `cert_ms` slow threshold | S2.3.4 | the corpus's is 1 s |
| RU realisations ("same id + `-ru`", sharing `gold.ein`) | "a Level C addition" (s1:176-178) | names stay ASCII, text is Unicode (`features/14`, `tests/text/`). The plan's own tension: one shared `gold.ein` cannot have every `:source` a substring of both an English and a Russian `text.md` (s1:115-116) |
| the `minimal` flag (no redundant clue) | optional, recorded (s2:160-162) | unchanged |
| whether a model's branching is written as hypotheses | M2 README, "open" | not P2.3's |

---

## E. Dependencies

**On other M2 stages** (every target exists and says what P2.3 says of it):
S2.2.1 (the contract checks prompts against the split file, :134-146), S2.2.2
(the download pattern, :92), S2.2.5 (`examples/zebra.txt`, :107), S2.4.4 (the
judged sample), S2.5.1 (`split_digest` a field of the record, :73), S2.5.2 (the
extractor's own error rate, :55, :99), S2.5.3 (the unit, :44), S2.6.4 (the four
oracle conditions, :62-65), P2.7 (S2.7.2, :53), P2.10 (S2.10.1, :88). P2.2 is
needed only to run the Level B gate (T2.3.4.5). P2.1's `ein-feedback/1` is not
needed by P2.3. Those stages share P2.3's stale words (`-e`, the summary keys);
that is their auditors'.

**M10** (`plans/m10_external_benchmarks/`): exists, five stage files, **not
started**; "queued behind M1a" in `plans/README.md:372`. It still says what P2.3
says: formal-language shaped, hand-encoded for six systems, not M2-gated, the
NL-shaped sweep left to F13 and M2 (its Non-goals). Its problems — `zebra`,
`zebra2`, `zebra2-minus-15`, n-queens at 4, 6, 8 — are three programs P2.3
already has and one that is not written. M10's own text is stale in the same
way (`ein test` at `-m 5`, "`ein test <file> -m 38`"; `-m/--max-set-size` is
retired, `ein-cli/src/solve.rs:29-30`). Its promise to write confirmed answers
back as `:expect` is unkept: no Zebra file states one. `nl2smt/smt/` holds the
three `.smt` files M10 counts as its corpus (`4-queens.smt`,
`einstain-problem.smt`, `einstain-problem-minus-15.smt`), and since M1j the
pipeline beside them has done for Zebra what M10's S10.4 plans (an independent
count of 1, 32 and 0).

**Followups**

- F13 (`f13_puzzles_beyond_zebra/ideas.md`): exists, last changed 2026-08-20
  (`git log`), says what is quoted. P2.3 leans on it for the ladder and the BBH format.
- F8, F7: exist; cited once each, as quoted.
- **F26** (`f26_consistency_and_completeness.md`), not cited by P2.3: its item 6,
  *the common-sense table as benchmark classes for M2*, is addressed to this
  phase and nothing in P2.3 takes it up. It also gives Ein's own vocabulary for
  the verdicts (alive, dead, owes, discharged, exhausted).
- **F27** (`f27_cryptarithm_theory.md`), not cited: a decided, unbuilt
  `std.math.crypt`; one of its triggers is "a benchmark of puzzle families beside
  the zebra (F13, M10)". The corpus has three cryptarithm programs.
- **F25** (`f25_roughness_found_outside_zebra.md`), not cited: its trigger is "a
  corpus of real Ein programs exists … and one of them meets a row". A benchmark
  of hand-written and generated programs is such a corpus. Two of its rows (R-2,
  R-3) speak of `:hrules`, which is gone from the language.

**M1h** (`plans/m1h_beyond_word_problems/README.md`), not cited by P2.3: says of
M2 that "every pair of a sentence and its facts this milestone writes is an
instance of what M2 would have to produce" (:64-66) and that "P1h.10's fifty
questions with expected answers are a small benchmark with graded difficulty"
(:594-595). Not started. Its P1h.1 is the temporal and causal theories (§ D).

**The kernel**: "Depends on: the kernel only" (README:4) holds; the kernel it
depends on is not the one the plan describes (§ A.1, A.2).

**External tools**: none assumed by P2.3 beyond the model server of P2.2 and
network access for S2.3.3's downloads.

---

## F. Estimates, dates used as status, counts

**Estimates** (against the user's ruling of 2026-11-08 that plans carry none,
`plans/README.md:364-365`): README:3 "3 weeks — 4 stages, 15 days"; README:94-99
the table's `est.` column (3 d, 6 d, 3 d, 3 d; they sum to 15); s1:4 "3 days";
s2:4 "6 days"; s3:4 "3 days"; s4:4 "3 days"; s4:52-53 "at the anchors' cost is
minutes"; s2:160 "costs one `-e` run per clue".

**Dates and status words**: no status line in the five files. "not shipped yet"
(README:79), "is not shipped" (s1:51), "once M1c ships" (s1:165), "the day
`ein test` ships" (README:112; s1:8), "or is about to" (s1:133) — all of a state
that ended 2026-08-24. s4:32 `"frozen": "2026-…"` and s3:96-97 "date fetched" are
fields of planned data files. Since the reshape of 2026-08-23 four of the five
files were touched only by commits whose subject is a move or a re-aiming of
references (2026-08-24, 2026-08-27, 2026-11-17; `git log`, the diffs not read);
s4 has one commit, 2026-08-23.

**Counts in the text**: eight families; three verdict classes; five old puzzles
(s1:109 puts all five in attribute matching, s1:20 and :88-91 put four there and
one in ordering); 32 models (true); "the three programs the repo already
generates" (four); six stdlib families and two synthesis families; ≥ 3 surface
templates per statement type; 11 knobs in s2's table against 10 in EinAf § C4
(`steps` added); six external sources; BBH "250 per task (a few fewer)"; FOLIO
"~1 400"; twenty hand-read items per source; splits 20 / 50 / 100 and "8 × 3 ×
170 = 4 080 instances"; BBH 10 / 30 / 60 %; 200 items each of CLUTRR, ProofWriter,
FOLIO; size bins [3, 5], [6, 8], [9, 15]; three seeds for the Level B run;
`cert_ms = 41` in the manifest example; "six-system corpus" (M10's). The schema
name `einaf-bench/1` is given to two different documents, `meta.json` (s1:62)
and `manifest.toml` (s4:72, :75). None of the dataset counts can be checked in
the tree.
