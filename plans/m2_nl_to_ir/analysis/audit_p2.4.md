# P2.4 "the loop" (Stages E, F) against the repository at `458bf90b`

Audited: `plans/m2_nl_to_ir/p2.4_loop/README.md`, `s2.4.1_state_machine.md`, `s2.4.2_repair.md`,
`s2.4.3_feedback_ladder.md`, `s2.4.4_faithfulness.md`, `s2.4.5_alternatives_as_hypotheses.md`. Context read:
`plans/m2_nl_to_ir/README.md`, `open_questions.md` (Q8, Q-M2.3, Q-M2.4), `EinAf.md` Stages E and F,
`p2.1_kernel_as_instrumentation/s2.1.2_feedback_object.md`.

Binary: `ein.rs/target/release/ein` (`ein 0.1.0`, `protocol ein-events/1`, `container einb/2.2`). Every run's files
are under `(scratch, not kept: re-run the commands) p2.4_runs/`
(one folder a program; `probes/` holds six small programs written for this audit, outside the repository).
Nothing in the repository was edited. No cargo, build or gate was run.

**One fact frames everything below.** Every markdown link in the six files resolves, file and anchor (checked by
script: 0 missing files, 0 missing anchors). The links were re-aimed mechanically when the documentation tree was
renumbered (`aa861dfa`, `322f7a71`, 2026-11-17); the sentences around them were not rewritten. So a link that
resolves often points at a page that now says something else (Q-M1d.6 is the clearest: the plan cites it as a defect
of the engine, and the file it reaches is the decision that removed the defect).

Plan lines are cited `file:line`. `README` alone is `p2.4_loop/README.md`.

---

## A. Stale claims

37 items, 27 claims checked and found true (list at the end of this section). Of the 37, item 6 is a measurement
with no command, item 28 is true under a condition the plan does not state, and items 36 and 37 are the plan against
itself; the other 33 are statements about the project that do not hold today.

### A.1 Commands, flags, exit codes

| # | plan | quote | today | evidence |
|---|---|---|---|---|
| 1 | README:30, :35; s2.4.1:26, :35, :135 | "`ein solve -e` saturates, searches and certifies in one run" | `-e` is retired, exit 2: *"-e/--exhaustive is retired: `ein solve FILE` describes all of R, which is exhaustive, and `--maximal` gives the maximal line, which is what the walk's models were"* | `ein solve examples/features/10_expect.ein -e`; `ein.rs/crates/ein-cli/src/solve.rs:21-27` |
| 2 | s2.4.1:36, :155, :173; s2.4.3:58 | "the `-m` cap", "`--max-set-size`", "at `-m 5`", "the `-m`-cap case" | retired, exit 2: *"a depth cap changed answers. `--minimal` gives the least commitments… `--region-budget FACTS` bounds the work"* | same command with `-m 5`; `solve.rs:28-33` |
| 3 | README:97; s2.4.1:37, :97, :155 | "`--max-time` / `--max-enterings`" | `--max-enterings` retired (*"`--region-budget FACTS` bounds the work, and a run that reaches it prints what it established"*). The budgets today: `--region-budget FACTS` (default 2^20; exit 0; a partial answer with `region.stopped`), `--region-limit N` (default 16; a flagged program's cube), `-T/--max-time` (exit 2) | `ein solve --help`; `solve.rs:34-37` |
| 4 | README:98; s2.4.1:37, :88-89, :97 | "read from the feedback object, never from exit code 2"; "`Aborted`"; "the feedback object says which fired" | a `-T` abort **writes no summary, nothing on stdout, no file in the answer directory**: one stderr line and exit 2. There is no `Aborted` verdict in any JSON. The only signals of a `-T` abort are the exit code and `** aborted: --max-time S s reached before the question was decided **`. A `--region-budget` stop is the opposite: exit 0, the summary written, `region.exhausted: false`, `region.stopped: {why: "budget", spent, projected, …}` | `ein solve examples/zebra2.ein -T 0` → exit 2; `docs/ein/user/04-reference/08-read-out.md:198-199`, `:539`; `p2.4_runs/zebra2/single_budget.json` |
| 5 | s2.4.1:10-11 | "the additive `--feedback FILE.json`" | no such option (`error: unexpected argument '--feedback'`). Planned by S2.1.2, not shipped; no `einaf/` directory exists | `ein solve … --feedback x.json`; `ls einaf` |
| 6 | README:31; s2.4.2:37 | "29 ms on Zebra", "At 29 ms per solve" | no command beside the number. Indicative wall clock here (AC, process start included, not through `bench_env.sh`): `ein solve examples/zebra2.ein --maximal` 34 ms, `--single` 42 ms, bare describe 97 ms; `examples/zebra.ein` `--single` 40 ms, bare 275 ms. A bare describe of `zebra2-minus-15.ein` takes 10.7 s and stops at the default budget; `--maximal` on it takes 0.11 s | timings in § B.0 |

### A.2 JSON keys, schema names, verdict words

| # | plan | quote | today | evidence |
|---|---|---|---|---|
| 7 | s2.4.1:74 | "`ein-summary/1`" | `ein-summary/2`. Other schema names today: `ein-answer/3` (`index.json`), `ein-lattice/2`, `ein-h-space/1`, `ein-test-report/6`, `ein-saved/1`, `ein-events/1` | `08-read-out.md:546`; `p2.4_runs/*/summary.json` |
| 8 | README:66 | F2 source: "`verdict.type` × `exhausted` × `unsat_core`" | no `verdict.type`. `verdict` is `null` on every run but `--single`, where it is `{word, which}`; `word` ∈ `Solution`, `Ambiguity`, `Contradiction`, `Open`, or `null` (*not decided*). `exhausted` is `region.exhausted`; the core is `region.core` | `p2.4_runs/11_expect_ambiguity/summary.json` (`"verdict": null`), `single.json`; `08-read-out.md:181`, `:618` |
| 9 | README:67; s2.4.3:29 | F3 source: "`verdict.k`, `solutions[].goal_bindings`" | no `k`, no `solutions`. `region.k_s` (states), `region.k_c` (commitments), `region.minimal.states`, `region.maximal.states`; bindings are `queries[].classes[].rows[]` (a map variable → value), grouped into **projection classes** with `classes[].states` | summary of `11_expect_ambiguity`; `08-read-out.md:117-124`, `:619-622` |
| 10 | README:68; s2.4.1:82 | "`verdict.solutions[]` — sorted fact lists"; "`verdict.solutions[].facts`, sorted by model already" | `region.states[].facts`: *a state's facts over the candidates' relations, root's included* — not the state's whole fact set. Beside it `closure`, `from`, `minimal_generators`, `commitments`, `lines`, `listed` | `08-read-out.md:392-397`, `:609-615` |
| 11 | README:69 | "`verdict.unsat_core` — given facts, which carry `:source`" | `region.core` is a list of fact strings; **no JSON carries a core fact's `:source`**: it is the read-out's `source` rows (stdout, `answer.txt`), whose summary column is "—". `region.core` and `region.refuted_by` are `null` under `--single`, `--count` and every mode, on a dead root too | `p2.4_runs/zebra2-bad/summary.json`, `stdout.txt`; `12_expect_false/single.json` (`core: null`, verdict `Contradiction`); `08-read-out.md:404-407`, `:554-556` |
| 12 | s2.4.1:73-75 | "`root.facts`, which `ein-summary/1` already emits as *the KB's facts as sorted s-expressions*" | `root.facts` is an **integer**, the size of root's fixpoint; `root.facts_by_relation` is a map of counts. No JSON surface of `ein` carries a state's fact list | `"root": {"facts": 18, "facts_by_relation": {…}}` in `11_expect_ambiguity/summary.json`; `docs/ein/user/04-reference/10-answer-dump.md:263-264` |
| 13 | s2.4.1:75-77; README:99-101 | "`06-solution-semantics.md` § 1.5 … pins as canonical state identity, *the sorted fact list itself, never a hash*"; "the saturated state's digest, which the engine defines as the sorted fact list" | § 1.5 says: *"The identity of a state is its set of facts, compared as a set… A digest d of a state may label it for a reader and never decides that two states are one."* The quoted words are `03-design/07-search.md` § 5's (*"identity is the canonical form itself, never a hash of it… `canon::state_digest` exists for display alone"*). In code `canon::state_key` is a sorted list of interned `FactId`s, not text, and `canon::state_digest` is a `u64` used by one example and no command | `docs/ein/reasoning/02-kb-evolution/06-solution-semantics.md:364-373`; `docs/ein/ein.rs/03-design/07-search.md:206-212`; `ein.rs/crates/ein-infer/src/canon.rs:40`, `:88-92` |
| 14 | README:49-50, :150-152; s2.4.1:46, :178-180 | "`unknown` is an edge the plan insists on and the engine cannot name yet"; "The kernel cannot say `unknown` until M1d"; "The day M1d gives the engine a word" | M1d shipped. `--single` says `Contradiction` only where no node is alive, `Open` where alive nodes all owe, and *"not decided — the run stopped first"* (`verdict.word: null`) where a budget stopped it; describe says `exhausted no` and why | `docs/decisions/answer/Q-M1d.6-contradiction-is-said-only-where-no-node-is.md`; `ein solve examples/zebra2.ein --single --region-budget 100` → `verdict {'word': None}`, `stopped {'why': 'budget', 'spent': 64, 'projected': 42}`, exit 0 |
| 15 | s2.4.3:57-60 | "A `Contradiction` with an **empty** core is not rendered at F5 at all: it is the `-m`-cap case (Q-M1d.6)" | there is no cap. An empty core `[]` is a real core, *the empty set suffices* (a rule with `:match ()` concluding `(false)`); `null` is *no core*. A written `:rule` on a fact also empties a core today (F24, three open register rows) | `08-read-out.md:548-560`; `docs/decisions/read-out/Q-M1e2.27-…md`; `plans/followups/f24_written_provenance.md:12-19` |
| 16 | s2.4.1:36 | "`unique` but `query.status = absent` (the goal names nothing the model contains)" | two different things today: a goal over a relation the program does not name is **refused at load**, exit 1; a goal with no row in a state is a class with `rows: []`, printed *"the goal has no row: undetermined here"* | `probes/f1_goal.ein`; `p2.4_runs/wp_gap/summary.json` `queries[0]` |
| 17 | open_questions.md:318-322 (Q-M2.4) | "the summary's sorted `solutions` the model-set one"; "or the `.einb` of a saturated KB" | no `solutions`; and `.einb` is a private cache, never an interchange format (AGENTS.md § Where things live) | as 10 |

### A.3 Language forms

| # | plan | quote | today | evidence |
|---|---|---|---|---|
| 18 | README:121; s2.4.3:65; s2.4.5:27-54, :67, :78, :114; open_questions.md:81-83 | "`hrule` is already the kernel's choice-point form"; the `(hrule guess …)` and `(hrule read-7 …)` listings | **there is no `hrule` form and no `:hrules`**; a program that writes one is refused at the parse. A hypothesis rule is a `rule` whose `:assert` concludes `(open F)`. `zebra2.ein`'s is `(rule guess (?R ?T1 ?T2) :match (and (is-a ?a ?T1) (is-a ?b ?T2)) :assert (open (?R ?a ?b)) :why …)` | `docs/decisions/hypotheses/Q-M1f.567-…md`; `07-diagnostics.md:1114-1138`; `examples/zebra2.ein:357-360` |
| 19 | s2.4.5:31-33 | "*hrule presence is the switch* between the hrule-driven and the blind generator" (design/07 § 1) | the page has no such sentence. H is the union of its writers (rules concluding `(open F)`, written `(open F)` facts, obligations activated at root); the blind enumerator writes only where root holds no `open` fact and no obligation is activated | `docs/ein/ein.rs/03-design/07-search.md:13-31`; Q-M1f.570, Q-M1f.576 |
| 20 | s2.4.5:56-65 | "What the kernel cannot say is 'at least one of them' … never as an obligation … M1d's P1d.2 is where it arrives" | obligations ship: a rule whose whole `:assert` is `(open R)` (`std.algebra`'s `total-owed`, `surjective-owed`; `std.slots`'; a program's own). The corpus has the exact encoding: `examples/word_problems/readings.ein` (`reading-owed`, `read-need`, `one-reading`) | `docs/decisions/hypotheses/Q-M1d.2-…md`; `examples/word_problems/readings.ein:33-47` |
| 21 | s2.4.5:59-62 | "the `(not …)` facts come from the singleton `(not h)` writeback when a candidate dies … the `(false)` kills `{}` once both are excluded" | **no route writes into root** and no code writes a writeback. With one of two readings dead and no obligation, ∅ stays in R: `k_s = 2`, `--single` says `Ambiguity`; only `--maximal --single` says `Solution`. With an obligation added, `k_s = 1` and `--single` says `Solution` | `10-answer-dump.md:263`; `docs/ein/ein.rs/07-reserved-engine-strings.md:70-72`; `probes/alt_no_obligation.ein`, `probes/alt_obligation.ein` |
| 22 | s2.4.5:67-69 | "the trace's *Refuted hypotheses* section … `--events`' `nogood` / `writeback` lines" | the trace's section is `## N — k minimal dead sets` (set, core, refuted by). No `nogood` and no `writeback` event kind exists; a dead node is an `enter` line with `kind: "dead"`, `commitment`, `core`, `core_status` | `p2.4_runs/11_expect_ambiguity/trace.md:65`; `docs/ein/ein.rs/05-events.md:153-159`; `grep '"nogood"\|"writeback"' ein.rs/crates/*/src` → nothing |
| 23 | s2.4.5:46-54 | the two sample rules | neither carries a `:why`, which is required on every rule (refused at load without `--no-require-why`) | `07-diagnostics.md` § 2.10 |
| 24 | s2.4.3:43-48; README:68 | "rendered through the program's own `:why` templates … `(relation … :why "{?1} … {?2}")` … the harness substitutes positional arguments … equals the engine's wording in `ein solve`'s table" | a `:why` reads the form it is written on: in a declaration `{?1}` is the atom `relation`, `{?2}` the relation's name, `{?3}` its first label. **A fact of a relation takes no text from the declaration**; it reads by its `:source` or its deriving rule's `:why`, else as written, *"never as invented prose"*. `ein solve` prints facts as s-expressions | `docs/decisions/read-out/Q-M1f.58-…md`; `docs/ein/user/03-syntax-and-semantics/14-natural-language.md:116-131`; `examples/zebra2.ein:368-373` |

### A.4 What the loader and the kernel check

| # | plan | quote | today | evidence |
|---|---|---|---|---|
| 25 | README:65; s2.4.2:29 | "unknown atom … the loader, at load time … `undefined_symbol`" | no such refusal. A fact over an undeclared relation, an undeclared atom, loads in silence | `probes/f1_undeclared.ein` loads and is answered, exit 0 |
| 26 | README:65; s2.4.2:30; s2.4.1:33; s2.4.3:27 | "type mismatch … the loader / compiler … `type_error`"; "`arity_mismatch`" | Ein has no types, and a relation is overloaded by its count of arguments: `(lives Bob)` and `(lives Carl H9 extra)` beside `(relation lives Person House)` load in silence, and a goal at another count answers *undetermined here*. The decision's reason: *"a check of a form's count against a declaration is the loader reading a theory's form"* | `docs/decisions/language/Q-M1f.372-…md`, `Q-M1f.447-…md`; `probes/f1_undeclared.ein`, `probes/f1_goal2.ein` |
| 27 | s2.4.2:33 | "The residue is nothing" | of the old validator's three checks the loader makes none; the third (below) holds only under an imported theory | 25, 26, 28 |
| 28 | s2.4.2:31 | "structural contradiction … the kernel, by `std.algebra`'s `functional` rule" | true **where the program imports `std.algebra` and writes `(functional R)`**: dead root, core the two facts, each with its `:source` on stdout. The kernel itself has no check: a fact beside its stored negation is alive unless the program imports `std.contradiction` | `probes/functional.ein` (core `['(lives Ann H1)', '(lives Ann H2)']`, refuted by `functional`); `probes/negation_no_import.ein` (root in R); Q-M1f.557 |
| 29 | README:70; s2.4.3:32, :62-81 | "**nothing in the engine** — no crate computes a relation dependency graph" | `ein-infer/src/strata.rs` is *"The rule dependency graph and its strata"*: an edge from each relation a compiled plan reads to each it writes, negative at an odd `absent` depth, and one from a rule's activator relation; *"over the compile cache, not the source: a relation-polymorphic rule … has no relation in its head until an activator binds `?R`, so its edges exist only per plan"*. It is a library function; no command prints its edges (only each rule's stratum in `ein saturate --dump`, and negative cycles in the read-out's `strata` rows) | `ein.rs/crates/ein-infer/src/strata.rs:1-24`, `:78`; `docs/ein/ein.rs/08-reasoning-in-code/04-rule-evolution.md:118-129` |
| 30 | s2.4.3:74-75 | stdlib "resolved by `$EIN_STDLIB` → the checkout → the manifest the binary names" | `$EIN_STDLIB` → the checkout → the copy embedded in the binary. Also since: every import brings its module whole under full names (`std.math.decimal-arith.add`), `:symbols` / `:as` only shorten names, a module path is from the program's directory, and `std.core` is loaded into every program with no import line | AGENTS.md § Where things live, § Standing rules; `ein --version` |
| 31 | s2.4.3:76-80 | "the set of rules the graph instantiated equals the set of distinct `rule` names that appear in the entry's `fire` events plus the rules that never fired" | two kinds of rule appear in no `fire` line, no `compile` line and not in `load`'s `rule_names`: an obligation rule (its only evidence is `owe`, written while it is owed) and a hypothesis rule (its candidates are `hyp` lines; `load` counts it in `hypothesis_rules`). For a saturation rule the instantiation is narrated, one `compile` line per (rule, activator) plan. `ein saturate --dump` lists all three kinds: *Rules — the saturation agenda's* with each stratum, *Obligation rules* and *Hypothesis rules* with parameters and activator counts | `05-events.md:102`, `:123-130`; `p2.4_runs/11_expect_ambiguity/events.jsonl` (`"rule_names": ["one-seat-each", "one-person-each", "std.core.inconsistent"]`, `guess` in none); `p2.4_runs/04_owing/events.jsonl` (`total-owed` in `owe` lines only); `ein saturate --dump tests/region/04_owing.ein` |
| 32 | s2.4.3:97-98 | "Facts derived under a commitment are found under the `enter` that introduced it" | a node's saturation events come **before** its `enter` line; a dead node on a monotone program is narrated only to its first contradiction | `05-events.md:257-259` |

### A.5 Counts, fixtures, plan-internal

| # | plan | quote | today | evidence |
|---|---|---|---|---|
| 33 | README:134-135; s2.4.3:108-110 | "the difference between two of the 32 models names the deleted clue's relation"; "…and nothing else" | 32 states on the maximal line: true. Clue 15 is `(adjacent-via next-to nation-loc Norwegian color-loc Blue)`. Of the 496 pairs, the five smallest symmetric differences (4 facts) are over **`pet-loc` alone**; the next (8 facts) over `pet-loc`, `smoke-loc`. `nation-loc` and `color-loc` first appear at 12 facts | `ein solve examples/zebra2-minus-15.ein --maximal --json-summary`; computed over `region.states[].facts` (`p2.4_runs/zebra2-minus-15/maximal.json`); `examples/zebra2.ein:496` |
| 34 | s2.4.4:124 | "the same with one `(functional drink-loc)` duplicated" | `zebra2.ein` writes `(bijective drink-loc)`; it has no `(functional drink-loc)` to duplicate | `examples/zebra2.ein:401-405` |
| 35 | s2.4.5:47 | "sentence 7: 'The man who smokes Chesterfields…'" | condition (11) in `zebra2.ein` | `examples/zebra2.ein:483` |
| 36 | s2.4.1:145-146 | "**this stage asks S2.1.2 for one field**, `digests {program, state, models}`" | S2.1.2's sketch already has it | `p2.1…/s2.1.2_feedback_object.md:44`, `:66-71` |
| 37 | README:68; s2.4.1:82; s2.4.3:29 | `verdict.solutions[]`, `models[].goal_bindings` | not fields of S2.1.2's own sketch, which has top-level `models` (bare fact lists), `query.bindings` and top-level `unsat_core`: P2.4's three files name the object's fields three ways | `s2.1.2_feedback_object.md:25-47` |

### Checked and true (27)

`ein-events/1` is the event schema (README:71). A `fire` line carries `rule`, `bindings`, `premises`, `derived`
(s2.4.3:33; also `activator`, `redundant`, `ein`). `--trace` and `--relevant` exist (README:167). `05-events.md`
exists and is the stream's contract. The glossary says a core is not a subset-minimal unsatisfiable set (README:69;
s2.4.3:52-55; `13-glossary.md:301-313`). `std.algebra` has `functional`, concluding `(false)` (`stdlib/algebra.ein:249`).
`examples/zebra2-minus-15.ein` exists and its maximal line is 32 states. `examples/ein-bugs/zebra2-bad.ein` exists and
its root is dead with a core one fact of which carries a `:source`. `ein_ir::dump_canonical` exists
(`ein-ir/src/dump.rs:170`). `ein kb save --saturate` exists and the container's header holds a BLAKE3 digest
(`03-design/09-binary-format.md:79`). Q-M1a.22 is a decision file. Exit 2 is both a `-T` abort and a usage error
(s2.4.1:88-89). `--jobs` exists and moves no byte of the read-out. `--max-time` exists. There is no
`hypothesis-parse` form (s2.4.5:25). `zebra2.ein` writes `:source "condition (N)"` labels, not quotes (s2.4.4:131).
`(transitive R)` is a relation-parameterised rule of `std.algebra` (`algebra.ein:422`). The stdlib is ein-lang text
under `stdlib/`, `$EIN_STDLIB` first. § 1.5's anchor exists and states identity as the fact set. F13, F16 and F17's
`ideas.md` exist and hold what is cited (`Cl(Tᵢ₊₁,Pᵢ₊₁) = Cl(Tᵢ,Pᵢ)` at f13:605-608; the diagnostics list at
f16:245-253; Lean Workbook at f16:103). `std.macro` exists. The lattice has cardinality layers and no levels
(s2.4.5:112; `06-solution-semantics.md:212`). Idea 03 exists. `nation-loc`, `drink-loc`, `Norwegian`, `House-3` are
names of `zebra2.ein`. `07-diagnostics.md` § 1 is *Parse and load diagnostics*. `naf_deps` is advisory
(`ein-infer/src/naf_deps.rs:1`). `zebra2.ein`'s guess rule has the parameters and the `:match` the plan quotes (its
head word and `:assert` are item 18).

---

## B. What each stage needs from the project, and its state today

### B.0 The runs

| program | what it is | command (each with `--json-summary`) | what came back |
|---|---|---|---|
| `examples/features/10_expect.ein` | one state | bare; `--single` | `k_s 1`, `∅ in R`; `Solution` |
| `examples/features/11_expect_ambiguity.ein` | two full seatings | bare, `--dump-answer`, `--trace`, `--events`, `--dump-lattice`; `--single`; `--maximal --single`; `--count`; `ein show hyps`; `ein render h-space --dump` | **`k_s 7`, `k_c 7`, maximal line 2 states**, N 4; `Ambiguity` over all of R and over the maximal line |
| `examples/features/12_expect_false.ein` | dead root | bare with all four artefacts; `--single` | `k_s 0`, `empty: dead`, core 5 facts, `refuted by no-room`; `Contradiction` (and `core: null` under `--single`) |
| `tests/region/04_owing.ein` | empty R, root alive and owing | bare with all four; `--single` | `empty: owing`, frontier `∅ owes 1: "likes owes Ann a Food."`; `Open` |
| `examples/zebra2.ein` | the puzzle | bare; `--single`; `--single --region-budget 100`; `-T 0` | `k_s 1`, `k_c 261 982`, root owes 36; `Solution`; *not decided*; exit 2 |
| `examples/zebra2-minus-15.ein` | under-determined | bare (10.7 s); `--maximal` (0.11 s) | bare: `k_s 32`, `exhausted no` (stopped counting commitments at the default budget); `--maximal`: 32 states, exhausted |
| `examples/ein-bugs/zebra2-bad.ein` | dead root | bare with all four | core 3 facts, `source` row, `refuted by (given), co-located-negative`; events 8 159 lines / 1.67 MB, trace 290 KB |
| `examples/lattice/08_absent_witness_hole.ein` | flagged | bare with `--events`; `--single`; `--monotone` | `WARNING` first row, stderr first line, `monotonicity.warning`, one `monotonicity` event; route 2, no licence |
| `examples/word_problems/boxes13.ein`, `contra.ein`, `gap.ein` | two readings; refuted; a gap | bare | 2 states / 2 answers, root owes a reading; core of 6 facts with 4 `source` rows; a query whose class has no row |

**What changed under the plan's words.** *Model*, *k* and *verdict* each had one meaning in the August plan (the
walk's models under `-e`). Today the answer is the region R, and three facts follow that the plan's vocabulary does
not have:

1. **R holds partial commitments where the program states no obligation.** `11_expect_ambiguity.ein` has two full
   seatings and `k_s = 7` (∅, four singletons, two pairs). The count the plan calls *k* is the **maximal line** there
   (2), and it is `k_s` only where obligations exclude the partial states (`zebra2`: 1; `zebra2-minus-15`: 32 = its
   maximal line).
2. **A verdict is a word only under `--single`, and about a named part of R** (`all`, `minimal`, `maximal`). The
   same program is `Ambiguity` over all of R and `Solution` over its maximal line (`probes/alt_no_obligation.ein`).
3. **`exhausted: false` is not *unknown*.** On `zebra2-minus-15` a bare describe stops at the default budget while
   counting commitments, having found *"R's 32 states, every one, its maximal line"* (stderr `note:`); the question
   `--maximal` is exhausted in 0.11 s.

### B.1 The F1–F8 table

| level (EinAf.md § Stage F) | what the engine supplies today | the surface | what is not supplied |
|---|---|---|---|
| **F1** syntactic: *undefined relation, type mismatch, arity mismatch* | **Parse**: every error recovery reaches, `<file>:<line>:<col>: unexpected X` with the token's kind, notes, at most three ranked `repair:` lines; a non-ASCII character; nesting past 1 000. **Load**: `kb load error: ` and its findings `; `-joined, each *the fault — what to write instead*, with `<file>:<line>:<col>:` where a place is known (31 sections, § 2.1–2.31: unbound conclusion variable, missing `:why`, `open` misplaced, a goal or a `(solve …)` list or a claim naming a relation the program lacks, conflicting hypothesis mechanisms, …). Load `note:` lines. | **stderr only**, exit 1 (`ein test`: 2). On a refused program **no summary and no event file is written** (`probes/f1_parse.ein`, `f1_load.ein`: neither file exists) | Any structured form (kind, location as fields). *Undefined relation* for a fact or a rule, *type mismatch*, *arity mismatch*: decided otherwise (A.25, A.26). A top-level form's place (`at None`: `07-diagnostics.md` § 1.5, Q-M1a.6, F23 D-2). Load notes in JSON (F23 D-18, L-125). Exit 1 is also a refuted `:expect` (`07-diagnostics.md:250-251`). Silent no-ops: an activator of another count (F23 D-15), `(open ?x)` of a name (L-134) |
| **F2** verdict: *SAT / UNSAT / AMBIGUOUS* | `--single`: `verdict.word` ∈ `Solution`, `Ambiguity`, `Contradiction`, `Open`, `null`; `verdict.which`. Describe: `region.k_s`, `region.exhausted`, `region.stopped.{why,spent,projected}`, `region.root` ∈ `in-r`, `owing`, `dead`; `region.empty` ∈ `dead`, `owing`, `null`. Root alone: `--no-search` (`root.dead`, `root.owes`, `root.owed`). Flagged: `monotonicity.monotone`, `.warning`, `.sites[]`. Claims: `claim.outcome`, `queries[].expect.outcome` ∈ `held`, `failed`, `not-checked` | `--json-summary` (`ein-summary/2`); read-out rows `verdict`, `decided`, `exhausted`, `empty`, `∅`, `WARNING`; `--events`' last line, `region` | One word under describe (`verdict` is `null`). A verdict that does not depend on which part of R is asked. A summary for a `-T` abort |
| **F3** cardinality: *4 models remain, query has 3 bindings* | `region.k_s`, `region.k_c` (exact, reported up to 2^63−1), `region.minimal.states`, `region.maximal.states`, `region.universe.{candidates,dead_alone}`, `region.nogoods`; `--count` (the two counts, no state read). Per query: the number of **projection classes** (`queries[].classes`), each class's `states` and `rows`, `queries[].exact`, `queries[].over` | summary; rows `states (k_s)`, `commitments (k_c)`, `maximal line`, `over … n states → m projection classes` | A count named *models*: which of `k_s` and the maximal line it is depends on the program (B.0 item 1). Distinct values per variable (the rows hold them; nothing counts them) |
| **F4** model difference | The states: `region.states[].facts` (sorted, over the candidates' relations), `closure`, `from`, `minimal_generators`, `lines`; `-p`/`-P`/`-f` and `forms/print-final-*.txt` (a state's facts beyond the program; its model; its candidate-relation facts); `models/NN.ein` (a saved file: the program and `(commit …)`, **no fact of the state**). The query's own grouping of states by what the goal sees: `queries[].classes[].{facts,rows,states}`. A difference between a **claimed** model and a found state: `ein test FILE --diff full` (`-` facts the found one lacks, `+` facts beyond it; `match`, `near`, `missing`, `extra`; in `ein-test-report/6` as `diff.items[]`) | summary; `--dump-answer DIR`; `ein test --diff`, `--json-report` | **Nothing computes a difference between two states of R**, picks a pair, or restricts one to a query. A fact as an English sentence (Q-M1f.58; M1p, not started) |
| **F5** unsat core | Dead root: `region.core[]`, `region.core_status` (`within-budget`, `cut-…`), `region.refuted_by[]`; rows `unsat core`, `core search`, `source` (the fact and its `:source`, quoted), `refuted by`. Dead nodes above root: `nogoods.json` (`{set, core, core_status, refuted_by}` per minimal dead set), the trace's `## N` section, `enter` events (`kind: dead`, `core`), `nodes.json` of `--dump-lattice`, `region.holes.minimal[].core` | summary under **describe only**; stdout / `answer.txt` for `source`; `--dump-answer`, `--trace`, `--events`, `--dump-lattice` | The `:source` text in any JSON. A core in the summary under `--single`, `--count`, a mode or `--no-search` (stdout prints a dead root's under `--no-search`; `12_expect_false/no_search.json` has none). The derivation a core was taken over. N on describe's default route where root owes (*"not computed by the tree and the classes"*; `--describe-route states` computes it). A canonical core (Q-M1f.6; F22 open) |
| **F6** dependency: *depends on nationality, on house, no path to pet* | (a) **the relation dependency graph**, `ein_infer::strata::edges`, per compiled plan — library only; each rule's stratum and parameters in `ein saturate --dump`. (b) `monotonicity.hypothesised` (the relations H is over) and `.dynamic` (those a commitment can change). (c) **what a state owes**: `region.root_owed[]`, `region.frontier[].owed[]`, `owe` events (`rule`, `activator`, `relation`, `bindings`, `why`), the `(open R)` facts; `rung` events (`stuck`, `uncovered`). (d) `root.hypgen.outside[]` (`{relation, arity, why}`: a relation the program can conclude or ask about that no writer proposes), `.filtered`, `.pre_candidate`, `.later_debts`. (e) H: `ein show hyps`; its structure, `ein render h-space` / `h-space.json` (`ein-h-space/1`: `pairs`, `theory`, `cover`, `coupling`, per-candidate rows; `--supports`, the stated facts each excluded pair rests on). (f) a goal with no row: `classes[].rows: []`. (g) the goal-relevant provenance slice of a state: `--trace --relevant`, `ein render kb --goal`. (h) which rules are instantiated: a `compile` event per saturation plan, and `ein saturate --dump`'s three rule lists (saturation, obligation, hypothesis) with activator counts. (i) as Ein rules: `std.debts` (`debt-due`, `debt-paid`, `debts-discharged`), the `unknown` macro, and `examples/word_problems/gaps.ein`, which concludes `(unanswered ?q ?g)` — *"the problem does not state […]"* | summary (`monotonicity`, `region`, `root`); `--events`; `ein saturate --dump`; `ein show`; `ein render h-space`; the trace | Reachability from a goal's relations. `missing_dependency`, `unreachable_query`, `dead_relation`, `unused_fact` by any name. The graph's edges on any command's output |
| **F7** provenance: *fact X ← rule R ← facts A, B* | `fire` (`rule`, `activator`, `bindings`, `premises`, `derived`, `redundant`, `ein`), `alt` (a further justification of a fact), `owe`, a segment per node closed by its `enter`. The trace: root's derivation, each listed state's from its first minimal commitment, each step with its rule, the rule's `:why` filled, premises each followed by *"— from ⟨its `:source`⟩"*, and what it derives. `ein saturate --dump`: each fact with its first origin (`:source`, `:rule`, `:hypothesis`). Library: `ein_render::trace::relevant_firings`, `ein_infer::explain` | `--events FILE.jsonl`; `--trace FILE.md` (`-l`, `-R`, `-G`, `-F`); `ein saturate --dump`; `ein render kb` | A chain per fact (it is reassembled). The absences and the activator a firing read (not in `premises`). The `:why` and `:source` texts in the stream (the trace has them). The derivation the core rests on: on `zebra2-bad` the chain rebuilt from each fact's first `fire` ends at `(bijective color-loc)`, `(is-a Ivory Color)`, `(is-a House-5 House)`, while the core is `(color-loc Green House-1)`, `(is-a House-1 House)`, `(relation color-loc Color House)` |
| **F8** combined | the union of the above; and three things the ladder has no rung for: what is owed, the monotonicity warning, the structure of H | — | — |

### B.2 Stage by stage

**S2.4.1 — the state machine and the transition log**

| needs | state today |
|---|---|
| `ein-feedback/1` and `--feedback` | does not exist (S2.1.2's to build; its derivation table reads `ein-summary/1` keys that are gone) |
| GENERATE, `einaf from-text` | does not exist |
| one SOLVE call, `ein solve -e` | exists in another form: a bare `ein solve` (describe), `--single`, `--maximal`, `--minimal`, `-n N`, `--count`, `--no-search`, each *the least work its question needs*. The plan's claim that ANALYZE and SOLVE are one process no longer holds as the only option: `--no-search` (root alone: `root.dead`, `root.owes`, `root.owed` in the summary, whose `region` is `null`; a dead root's core on stdout only), `--monotone` and `ein show hyps` answer without searching |
| the outcomes | exists in another form: four words, *not decided*, and *flagged* (B.1 F2). `Open` and the flagged case have no row in the stage's table |
| the solver budget as a termination condition | two budgets with opposite behaviour: `--region-budget` (exit 0, an answer with bounds) and `-T` (exit 2, no artefact) |
| `program_sha` from `ein_ir::dump_canonical` | exists as the stage assumes, **as a library function**. No command prints the canonical dump. One indirect text route exists: `ein kb save F X.einb`, then `ein solve X.einb --no-search --dump-answer D` writes `D/root.ein`, whose `(program …)` is the import-flattened program under full names, macro calls unexpanded (`probes/einb_root/root.ein`, 449 lines for `10_expect.ein`) |
| `state_digest` from `root.facts` | **does not exist as assumed** (A.12). In other forms: `ein saturate --dump` prints root's whole fixpoint as text, grouped by origin, each derived fact with its rule; `-p` prints each listed state's facts beyond the program as loaded; the library has `canon::state_key`. What root's fixpoint holds bears on the acceptance line *"`state_digest` over two textually different programs with the same closure is equal"*: it holds declarations (`(relation R …)` and `(relation R)`), every activator as a fact of its rule's name (`(guess Ann S1)`, `(one-seat-each seat)`), root's `(open …)` facts, and facts under module full names (`std.algebra.converse`, `std.contradiction.clash-2`) — `root.facts_by_relation` of `10_expect` and `11_expect_ambiguity` |
| `models_digest` | exists in another form: `region.states[].facts`, sorted, in one fixed order whatever the route or `-j` (`08-read-out.md:377-379`) |
| replay without the model | nothing of the project is needed |

**S2.4.2 — repair**

| needs | state today |
|---|---|
| the old validator's checks made by the kernel | two of three are not made at all (A.25–A.27); the third only under an imported theory (A.28). So the *cheap pre-check* the stage expects to drop is not the only residue |
| a per-form canonical text for the diff | `dump_canonical` (library); `--dump-answer`'s saved file holds the program's bytes as read |
| the fixtures | their **texts exist** under `nl2smt/puzzles/` (added 2026-11-13, a tree AGENTS.md calls external to project and process): `zebra.txt`, `zebra-swapped.txt` (two nationalities swapped across three sentences), `zebra-minus-15.txt`, `zebra-contradiction.txt` (one sentence added). `examples/zebra.txt`, which the milestone README's Level B acceptance names, does not exist |
| F5 on the one-wrong-fact fixture | exists (B.1 F5), under describe |
| the *unchanged* reply ending a run by fixed point | nothing of the project is needed |

**S2.4.3 — the ladder**: B.1. Two of its tasks are smaller or other than planned. **T2.4.3.4 (F6)**: the graph the
stage plans to rebuild in Python *"re-implementing the binding step"* is `strata::edges`, over the engine's own
compiled plans; what does not exist is reachability from a goal and a command that prints the edges. The stage's
fallback, *"asking `ein` to dump the expanded program"*, half exists (the container route above; `compile` events
list every saturation plan, and `ein saturate --dump` every rule of the three kinds). **T2.4.3.2 (F4)**: the difference is a set difference over `region.states[].facts` as planned; the
`:why` rendering has no mechanism to lean on (A.24), and the acceptance sentence does not hold on today's 32 states
(A.33).

**S2.4.4 — faithfulness**

| needs | state today |
|---|---|
| a `:source` on every extracted fact | exists: `:source` is a clause of a fact, negated or not, activators included (`02-syntax.md:87`, `:104`); it is optional; a rule and a declaration take `:why` and no `:source` |
| the closure test for a redundant rule (`state_digest` with and without) | as S2.4.1's row: an activator is itself a fact of the closure, so adding `(functional drink-loc)` changes root's fact set whether or not it derives anything |
| *"the kernel certifies `k = 1 ∧ exhausted`"* | `verdict.word = "Solution"` with `decided yes`, on a monotone program; on a flagged one nothing is claimed (§ C.2) |
| B2's sidecar, the canonical theory | S2.2.1's and S2.3.2's (other auditors) |

**S2.4.5 — alternatives as hypotheses**

| needs | state today |
|---|---|
| a choice-point form | exists in another form: a `rule` concluding `(open F)`, or `(open F)` written as a fact (`examples/features/24_written_hypotheses.ein`) |
| mutual exclusion | exists: a rule concluding `(false)` |
| *at least one* | **already shipped**: an obligation, a rule whose `:assert` is `(open R)` |
| the pattern fixed on fixtures (T2.4.5.1) | **a corpus program is the pattern**: `examples/word_problems/readings.ein` holds a word's two readings as hypotheses with a reading owed and a one-reading restriction; `boxes12.ein` (two states, one answer) and `boxes13.ein` (two states, two answers) state the claims; the decision is Q-M1f.332, *a word with two readings always has one chosen* |
| the surviving reading named from the trace | `region.states[].facts` / `from`; the dead reading in `nogoods.json` and `region.universe.dead_alone`; which rule proposed a candidate in `ein saturate --dump` (`(open (seat Ann S1) :rule guess)`) and `ein show hyps`' stderr note |
| the verdict `unique` on a two-reading fixture where one dies | `Solution` under `--single` only with the obligation, or over the maximal line (A.21) |

---

## C. Conflicts with standing rules and decisions

1. **A core is an explanation** ([Q-M1f.6](../../../docs/decisions/answer/Q-M1f.6-a-core-is-an-explanation.md)): *"It may vary
   with the rules' written order and with the agenda's… Which core is reported is not canonical."* So is a fact's
   reason (Q-M1f.58: *"its reason is an explanation, as a core is"*).
   - s2.4.2:104-106 counts *"a new core member on the next iteration"* as a regression introduced by a repair. Two
     iterations are two programs; a repair that only reorders rules may move a core.
   - s2.4.3:52-54 describes the core as *"the smallest set of given facts"*. The pages put three bounds on *smallest*:
     not subset-minimal, over the derivations recorded (capped per fact), and budgeted, with `core_status` saying when
     the search was cut (`09-cores.md:346-364`). The plan's renderer caveat does not know `core_status`.
   - Of the nine open rows of `plans/wrong_answers/register.toml`, five are F22's (a core that is not the smallest)
     and three F24's (a written `:rule` / `:using` empties a core). F22's trigger list names *"M2's NL explanation of
     a refutation"*; F24's names *"a consumer that reads cores or `refuted by` from programs it did not write"*, which
     is what the loop is.
2. **A non-monotone program is flagged and nothing is claimed of its answer**
   ([Q-M1f.564](../../../docs/decisions/answer/Q-M1f.564-a-non-monotone-program-loads-with-a-warning.md)). A program is
   flagged where the static check finds a site (`05-events.md:76-81`): an `(absent …)` at odd depth over a relation a
   commitment can change, a rule that reads `open`, a rule whose activator a commitment can derive, an obligation's
   trigger or guard over such a relation, a hypothesis rule that reads one. Nothing keeps a model-written program
   clear of these. The engine then loads it, answers by the cube with no licence, and says so three times:
   the read-out's first row `WARNING … NOTHING IS CLAIMED OF THIS ANSWER. IT IS NOT CLAIMED TO BE RIGHT, SOUND OR
   COMPLETE`, stderr's first line, and `monotonicity.warning` (run: `examples/lattice/08_absent_witness_hole.ein`;
   `monotonicity.sites[]` names the rule, the relation, the depth and what is at stake). Past `--region-limit`
   (16 candidates) the answer is partial.
   - The stage's *verified success* (s2.4.1:93), *"the kernel certifies"* (s2.4.4:17) and the metric
     `P(faithful | Ein certifies unique)` (s2.4.4:67) have no case for it; neither has `ein-feedback/1`'s outcome
     enum.
   - The same rule says no golden banks a flagged program's answer (AGENTS.md § The answer: *"the shape golden banks
     none of its answers… no regression row, register row or assertion holds one to what it answers"*). s2.4.3:105-107
     banks every level's rendering *"over the benchmark anchors"* as goldens.
3. **A hypothesis rule replaces `hrule`**
   ([Q-M1f.567](../../../docs/decisions/hypotheses/Q-M1f.567-a-hypothesis-rule-is-a-rule-that-concludes-open.md)): S2.4.5's
   encoding, its acceptance line *"The encoding uses `hrule` and a `(false)` rule only"* (s2.4.5:78) and README:121.
4. **Ein has no types, and a declaration holds no form to its count**
   ([Q-M1f.372](../../../docs/decisions/language/Q-M1f.372-ein-has-no-types-a-declarations-slot-names-are.md),
   [Q-M1f.447](../../../docs/decisions/language/Q-M1f.447-a-relation-is-overloaded-by-its-count-of.md)). F1's
   `type_error` and `arity_mismatch` (s2.4.1:33; s2.4.3:27), and s2.4.2's table, assume a loader check the user
   ruled against. EinAf.md's own F1 list is *undefined relation, type mismatch, arity mismatch*; none of the three is
   a load diagnostic for a fact today. A typing check exists as theories a program imports (`std.bijection`'s
   `typecheck-setup`, `std.algebra`'s `converse-illtyped-*`, `std.typing`), whose verdict is `(false)` with a core.
5. **The kernel reads no theory's name, and has no contradiction check of its own**
   ([Q-M1f.557](../../../docs/decisions/stdlib/Q-M1f.557-the-contradiction-is-a-theory-a-program-imports.md)). s2.4.2:31's
   *"the kernel"* reports a structural contradiction only where the generated program imports the theory and writes
   its activator.
6. **`[]` is the empty core and `null` is no core**
   ([Q-M1e2.27](../../../docs/decisions/read-out/Q-M1e2.27-in-eins-json-is-the-empty-set-and-null-what-does.md)).
   s2.4.3:57-60 and its acceptance *"F5 never renders an empty core"* read `[]` as a truncated search.
7. **A fact takes no text from its relation's declaration**
   ([Q-M1f.58](../../../docs/decisions/read-out/Q-M1f.58-a-fact-reads-in-english-in-the-trace.md)). s2.4.3:43-48 renders F4's
   facts through declaration templates and tests the wording against the engine's; the engine has no such wording.
   The replacement the user asked for is M1p (created 2026-11-20, not started).
8. **`.einb` is never an interchange format.** S2.4.1 rejects the container's digest (s2.4.1:77-81), which agrees
   with the rule. Q-M2.4's text still offers *"the `.einb` of a saturated KB"* (open_questions.md:321).
9. **A digest never decides that two states are one** (`06-solution-semantics.md:371-373`). s2.4.1:99 stops the run
   on `state_digest(i) ∈ {state_digest(j)}`; the stage's own note that the loop *"stores the list too"* is the part
   that agrees with the page.
10. **The project and the process are two things, and decisions about Ein live under `docs/decisions/`.**
    - The six files put the harness at `einaf/` and `grammars/` and its goldens at `einaf/tests/golden/`; they do not
      say whether that tree is project (then nothing in it names a plan, a stage or a date, and `cargo test
      --workspace` is what holds its claims), process, or external like `nl2smt/` (*"wired into nothing"*). s2.4.3's
      drift test *"for every corpus entry"* is a check tying such a tree to the project; AGENTS.md has those as a
      `utils/` check and a gate step, never a crate test.
    - s2.4.1:101, s2.4.4:86 and s2.4.5:81 have a stage decide Q-M2.4, Q-M2.3 and Q8 and mark each *"decided beside
      its entry"*, after open_questions.md:11-13 (*"`docs/decisions/M2-*.md` never existed and is not coming"*).
      `docs/decisions/` exists now as *"every question about Ein that has an answer"*, and a decision about the
      process stays in `plans/`. Which of the two these three are is not said anywhere.
11. **Every answer exits 0; only a refuted claim exits 1** (AGENTS.md § The answer). Exit 1 is therefore a load
    refusal or a refuted `:expect`; S2.1.2's *"exit 1 → invalid"*, which S2.4.1's CHECK state reads, cannot tell
    them apart on a generated program that states a claim.

---

## D. Open questions and decisions the phase defers

| question | who the plan says decides | since |
|---|---|---|
| **Q8**, ambiguous parses | S2.4.5, *"as a strategy, not a default"*; P2.6 compares | its two premises moved: `hrule` is gone, and *"S2.4.5 decides whether a new form is needed or `hrule` is it"* has neither answer left (the form is a rule concluding `(open F)`, with an obligation for *at least one*). Q-M1f.332 decided the same shape for the word theory's readings. Nothing has decided Q8 itself |
| **Q-M2.3**, the unit of faithfulness | S2.4.4, as its recommendation | nothing in the kernel answers it. F24 (what a written `:rule` / `:using` may claim) is an open question of the user's that touches generated programs. The milestone README's § *The reasoning as a program* (2026-11-11) says it *"bears on faithfulness"* and changes no stage yet |
| **Q-M2.4**, syntactic or semantic fixed point | S2.4.1, as its recommendation | its sentence *"The engine can do both, today"* is now half true: the canonical dump is a library function, and the summary carries no fact list (A.12, A.13, A.17) |
| **Q-M2.1**, when the kernel is frozen (README:152; s2.4.1:180) | S2.5.1 | its M1d dependency is resolved (A.14). Its premise *"two kernel milestones are queued"*: five shipped; seven more are listed in AGENTS.md, each *started only on the user's word* |
| which kind of `unknown` a run was (README:51-53) | the failure taxonomy, P2.7 | the engine distinguishes them itself: `region.stopped.why` is `budget`, `limit` or `asked`; a `-T` abort is exit 2 |
| the iteration budget, default 3 | a parameter; G7 | unchanged |
| the `:source` pre-check kept or dropped (T2.4.2.4) | a measurement in the stage | unchanged |
| F7's depth cap (default 3), F8's three size caps | config; G6 | unchanged. Sizes observed: `zebra2-bad` 8 159 event lines, a 290 KB trace |
| the κ threshold 0.85 | stated before measurement, in S2.4.4 | unchanged |
| a dump flag on `ein` *"the day the drift test fails"* (s2.4.3:80-81, :157-160) | deferred | half exists (B.2) |
| anchors that carry a label, not a quote, in `:source` (s2.4.4:131-139) | S2.3.1 | unchanged; `zebra2.ein` still writes labels |
| Q-M1a.6, locations on loader messages (through S2.1.2) | S2.1.2 recommends taking it up | still open: `07-diagnostics.md` § 1.5 calls it *"the clearest candidate on this page for being fixed"*; F23 D-2 |

**What today's kernel asks that the six files do not** (stated as facts, each with its evidence above): which part of
R a SOLVE call asks about; what the machine does with a flagged program; whether a generated program states
obligations (which decides whether `k_s` counts partial states); what happens when a generated program has no
hypothesis writer at all. On the last: `probes/f1_undeclared.ein`, ten lines with no rule, no obligation and no
`(solve :no-search)`, gets the blind enumerator: 83 candidates, R *the whole cube*, `k_c` reported as
*9 223 372 036 854 775 807 or more*, a describe that stops at the default budget with 95 369 states, exit 0, and a
**69.6 MB** summary.

---

## E. Dependencies

| on | exists | still says what the plan says |
|---|---|---|
| S2.1.2, `ein-feedback/1` | as a plan | no: its derivation reads `summary.type`, `Aborted`, `exhausted`, `core`, `solutions[].goal_bindings`, `-e`, `-m` (A.1–A.4, A.7–A.11). Every P2.4 stage reads this object and nothing else, so every field name in P2.4 waits on it. P2.4's three spellings of its fields disagree with its sketch (A.37) |
| S2.1.3, the boundary (Q25) | as a plan | P2.4 assumes its recommendation, a Python harness at `einaf/` |
| S2.2.1 (the B2 sidecar), S2.2.3 (grammars), S2.2.4 (passes), S2.2.5 (`einaf from-text`) | files exist | other auditors' |
| S2.3.1 (anchors), S2.3.2 (generators, the unit of a constraint) | files exist | other auditors' |
| S2.5.1, S2.5.2, S2.5.3; P2.6 (G1, G5, G6, G7), S2.6.4; P2.7; P2.10 | files exist | other auditors' |
| M1d's *P1d.2* (s2.4.5:65) | the plan folder is deleted (`9aa598a3`, 2026-08-27) | obligations shipped; Q-M1d.2 is the decision |
| Q-M1d.6 | decision file | the opposite of the plan's use (A.14, A.15) |
| F13, F16, F17 `ideas.md`; idea 03 | yes | yes |
| `05-events.md`, `13-glossary.md`, `06-solution-semantics.md` § 1.5, `02-syntax.md`, `03-design/07-search.md`, `07-diagnostics.md` § 1 | yes, every anchor | partly: A.13, A.18, A.19, A.22, A.25, A.32 |
| the root `README.md`'s EinAf table (milestone README:281-283, *"the two scheduled cells … now point at P2.4"*) | the root README has no line containing *EinAf*, *LLM* or *autoformal* (`grep -ci einaf README.md` → 0) | no |
| `examples/zebra.txt` (the smoke test's text, milestone README:192) | no; `nl2smt/puzzles/zebra.txt` does | — |

**Not named by the phase, and bearing on it today**: F22 (the exact core; its trigger names M2), F24 (written
provenance; its trigger is a consumer of cores from programs it did not write), F23 (D-2, D-15, D-18, L-125, L-134:
what a load says, where, and what it leaves silent), M1p (a fact as a sentence; not started), M1n (non-monotone
programs; not started), M15 (an MCP server; a placeholder, no design), `nl2smt/puzzles/` (the Zebra texts and four
variants).

---

## F. Estimates, dates used as status, counts

- **Estimates**: README:3 (*3 weeks — 5 stages, 15 days*), README:115-121 (3 d, 3 d, 5 d, 3 d, 1 d); the
  **Estimate:** line of each stage file (s2.4.1:4, s2.4.2:4, s2.4.3:4, s2.4.4:4, s2.4.5:4).
- **Dates as status**: none in the six files. In the context files: *"reshaped 2026-08-23 … nothing started"*
  (milestone README:6), *"written 2026-08-23"* (:140), *"Arrived 2026-08-23 with the reshape"* (open_questions.md,
  each Q-M2.n).
- **Measurements with no command**: *29 ms* (README:31; s2.4.2:37).
- **Counts**: *nine levels / nine renderers* (README:57, :119; s2.4.3:15); *seven termination conditions*
  (README:117, :128; s2.4.1:87, :120, :165); *five states and three repair edges* (README:29; the stage's table has
  five states); *three checks* (README:65; s2.4.2:22); *three digests* (s2.4.1:67); *four diagnostics*
  (s2.4.3:83); *three judges* (s2.4.4:27); *eight faithfulness kinds* (s2.4.4:45-48); *32 models* (README:135;
  s2.4.3:109); *default 3* iterations (s2.4.1:95), *default 2* rejects (s2.4.1:94), *depth cap 3* (s2.4.3:96),
  *three caps* (s2.4.3:118), *three seeds* (s2.4.2:67), *0.85 κ* (s2.4.4:33), *sentence 7* (s2.4.5:47), *F1 of
  nine*, *≤ 3 attempts* (README:83).

---

## Where I am unsure

- Whether the plan's F4 acceptance fails under its own pair rule depends on F6's reachable set for `zebra2`'s goal.
  The goal names `drink-loc`, `nation-loc` and `pet-loc`, so `pet-loc` is in any such set and the 4-fact `pet-loc`
  differences stay the smallest; I did not build the reachable set.
- I did not run `ein test --diff full` or read a `--json-report`; the F4 row's account of them is from
  `05-cli.md:162-198`, `:483` and `ein test --help`.
- `strata::edges` was read, not called: that it covers obligation and hypothesis rules' plans is not established
  (its doc says *"the saturation rules' dependency graph"*, and it filters to `program.rules`).
- The timings are single runs with process start included; they show order of magnitude only.
- The S2.1.2, S2.2.x, S2.3.x and S2.5.x files were read only as far as P2.4 cites them.
