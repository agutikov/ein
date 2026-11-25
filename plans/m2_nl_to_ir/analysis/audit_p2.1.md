# M2 audit — P2.1, "the kernel as instrumentation", against the tree at HEAD

Audited: `plans/m2_nl_to_ir/p2.1_kernel_as_instrumentation/{README.md, s2.1.1_census.md, s2.1.2_feedback_object.md,
s2.1.3_boundary.md}`, read whole; context: `plans/m2_nl_to_ir/README.md`, Q25 and Q-M2.1 of
`plans/m2_nl_to_ir/open_questions.md`, Stage A of `plans/m2_nl_to_ir/EinAf.md`.

Tree: `458bf90b` (master, clean). Binary: `ein.rs/target/release/ein`, `ein 0.1.0`, `protocol ein-events/1`,
`container einb/2.2`, `features einb, parallel, snmalloc`, stdlib `sha256:292894be…` (checkout). Read-only: nothing in
the repository was written, no cargo / build / gate was run. Every output file of the binary is under
`(scratch, not kept: re-run the commands) p2.1_runs/`
(`NAME.json` the summary, `NAME.out` stdout, `NAME.err` stderr, `NAME.exit`); the four scratch programs written
for cases the corpus has no small program for are there too (`claim_refuted.ein`, `syntax_error.ein`, `arity.ein`,
`multi_load.ein`, `kw_*.ein`).

Plan line numbers are those of the files as they stand. "Run NAME" names a file set in `p2.1_runs/`.

---

## 0. The fact base: what the machine surface says today, per Stage A1 situation

`--json-summary` writes **`ein-summary/2`** (`ein-render/src/dump/region_answer.rs:26`). Top-level keys, on every
run that writes one: `schema, source, asked, region, verdict, queries, claim, work, monotonicity, leftover, root,
config`. `region` keys: `k_s, k_c, exhausted, stopped, complete_up_to, root, root_owes, root_owed, empty, minimal,
maximal, components, subcubes, merged, nogoods, discharging, holes, universe, route, licences, states, frontier,
core, core_status, refuted_by`. Every key is present on every run, `null` where it does not apply
(`ein-cli/src/summary.rs:5-13`; `08-read-out.md:545-546`).

| A1 situation | program · command | exit | stderr | summary, the keys that decide it |
|---|---|---:|---|---|
| **invalid**, parse | `examples/broken/unclosed_paren.ein` · `solve` (run `a1_invalid_parse`) | **1** | 2 lines: `FILE:4:10: unexpected \`a\`: a name` / `  repair: Insert \`(\`, …` | **no file written**; stdout empty |
| invalid, parse, two errors | scratch `syntax_error.ein` · `solve` (run `c_syntax_error`) | 1 | 6 lines: two `FILE:L:C: unexpected …`, one `  note:`, three `  repair:` | no file |
| **invalid**, load | `examples/broken/load/rule_duplicate_name.ein` · `solve` (run `a1_invalid_load`) | **1** | 1 line: `kb load error: duplicate rule name 'r' at None` | no file |
| invalid, load, placed | `examples/broken/load/macro_arity_mismatch.ein` (run `a1_invalid_load2`) | 1 | `kb load error: (rule x): macro m/2 invoked with 1 args at FILE:6:20` | no file |
| invalid, load, four problems | scratch `multi_load.ein` | 1 | **one line**, four findings `; `-joined (three missing `:why`, one unbound conclusion variable) | no file |
| invalid, missing file | `solve examples/does-not-exist.ein` | 1 | `FileNotFoundError: [Errno 2] No such file or directory: '…'` | no file |
| invalid, under `ein test` | `test examples/broken/unclosed_paren.ein --json-report R` | **2** | the same parse lines | `ein-test-report/6`: `tally.errors 1`; row `outcome: "error"`, `queries 0`, `expect/answer/route null` — **the message is not in the report** |
| *(not invalid today)* a declared relation at another count, an undeclared relation | scratch `arity.ein`: `(relation owns Person Thing …)`, `(owns Ann Car Red)`, `(likes Ann Tea)` (run `c_arity`) | **0** | none | loads; `region.k_s 1`, `exhausted true` |
| **inconsistent**, bare solve | `tests/stdlib/algebra/09_irreflexive_violated.ein` (run `a2_inconsistent_describe`) | 0 | none | `verdict: null`; `region.k_s 0, k_c 0, exhausted true, root "dead", empty "dead", core ["(loops A A)"], core_status "within-budget", refuted_by ["irreflexive"]`; `root.dead true` |
| inconsistent, `--single` | same (run `a2_inconsistent_single`) | 0 | a `note:` (its region claim NOT CHECKED) | `verdict {word: "Contradiction", which: "all"}`; `region.empty "dead"`, `exhausted true`; **`region.core null`, `refuted_by null`** (a question that prints no core writes none, Q-M1f.29) |
| inconsistent, `--no-search` | same (run `a2_inconsistent_nosearch`) | 0 | none | `asked.ask "no-search"`, **`region null`, `verdict null`**; `root.dead true`. The core is on stdout (`unsat core (loops A A)`, `refuted by irreflexive`) and **in no key** |
| inconsistent, with `:source` | `examples/ein-bugs/zebra2-bad.ein` (run `a2_zebra2_bad`) | 0 | none | `region.core` 3 strings, `refuted_by ["(given)","co-located-negative"]`. The `:source "injected contradiction"` quote is a stdout row; **0 occurrences in the JSON** |
| **satisfiable, incomplete (owing)**, bare solve | `tests/region/04_owing.ein` (run `a3_owing_describe`) | 0 | none | `region.k_s 0, exhausted true, root "owing", root_owes 1, empty "owing", core null`, `frontier` 1 node; `root.dead false, owes 1, owed ["likes owes Ann a Food."]` |
| owing, `--single` | same (run `a3_owing_single`) | 0 | `note:` | `verdict.word "Open"`, `region.empty "owing"` |
| owing, `--no-search` | same (run `a3_owing_nosearch`) | 0 | `note:` | `region null`; `root.owes 1`; `queries[0].exact false` |
| **several complete models**, `--single` | `examples/zebra2-minus-15.ein` (run `a4_multi_single`) | 0 | none | `verdict.word "Ambiguity"`; `region.exhausted false, stopped {why: "asked"}`, 2 states in `region.states`; `queries[0].classes` 2, `exact false` |
| several, `-n 3` | same (run `a4_multi_n3`) | 0 | none | `asked {ask: "find", n: 3}`, 3 states, `exhausted false, stopped {why: "asked"}`, `verdict null` |
| several, `--maximal` | same (run `a4_multi_maximal`) | 0 | none | `asked.which "maximal"`, `exhausted true`, **32 states**, 108 saturations |
| several, bare solve, default budget | same (run `a4_multi_describe`, ~10 s) | 0 | 1 `note: … partial — …` | `region.k_s 32` (every state), `k_c 138621243` (a lower bound), **`exhausted false`, `stopped {why: "budget", spent: 1048507, projected: 93}`**, `maximal.states 32`, `queries[0].classes 18, exact true` |
| **one complete model**, bare solve | `examples/zebra2.ein` (run `a5_unique_describe`) | 0 | none | `verdict null`; `region.k_s 1, k_c 261982, exhausted true, root "owing", root_owes 36, minimal {states 1, commitments 63, least 1}, maximal {states 1}`; `queries[0].classes 1, exact true` |
| one, `--single` | same (run `a5_unique_single`) | 0 | none | `verdict {word: "Solution", which: "all"}`, `region.exhausted true`, 7 saturations |
| one, `--maximal --single` | same (run `a5_unique_maxsingle`) | 0 | none | `verdict {word: "Solution", which: "maximal"}` |
| one, `--no-search` | same (run `a5_unique_nosearch`) | 0 | none | `region null, verdict null`; `root.dead false, owes 36` — **the summary does not say there is one model** |
| **resource limit**, facts budget, bare solve | `examples/zebra2.ein --region-budget 1` (run `a6_limit_budget_describe`) | **0** | 1 `note: … partial — …` | `region.k_s 0` (a lower bound), `exhausted false`, `stopped {why: "budget", spent: 44, projected: 22}`, `root "owing"` |
| limit, facts budget, `--single` | same `--single` (run `a6_limit_budget_single`) | **0** | `note: … partial — stopped at --region-budget 1` | **`verdict {word: null, which: "all"}`**, `exhausted false`, `stopped.why "budget"`; stdout `verdict  not decided — the run stopped first` |
| limit, wall clock | `examples/zebra2.ein -T 0` (run `a6_limit_time`) | **2** | `** aborted: --max-time 0 s reached before the question was decided **` | **no file written**; stdout empty |
| *(usage error)* | `solve F -n 0`; `-e`; `-m 2`; `-E 5`; `-D DIR`; `--feedback X` | 2 | `error: …` (retired flags: `… is retired: …`) | no file |
| *(flagged, not monotone)* | `examples/lattice/04_obligation_trigger_hole.ein` (run `b_flagged_describe`) | 0 | first line `WARNING: FILE: THIS PROGRAM IS NOT MONOTONE: NOTHING IS CLAIMED OF THIS ANSWER (Q-M1f.564)` | `monotonicity.warning` set, `monotonicity.sites` 4; `region.route "2"`, `licences ["L7 the cube"]`; an answer is printed (`k_s 3`) and nothing is claimed of it |
| *(a refuted claim)* | scratch `claim_refuted.ein` (run `c_claim_refuted`) | **1** | `FILE: query :expect FAILED — the claim lists 1 class, …` | summary **is** written; `queries[0].expect {outcome: "failed", lines: […]}` |

What this means for the plan's "six outcomes", stated as facts:

1. **A bare `ein solve` has no verdict word.** `verdict` is `null` unless `--single` is asked; the four words
   (`Solution`, `Ambiguity`, `Contradiction`, `Open`) are `verdict.word`, and `null` there is *not decided*
   (`ein-infer/src/explore.rs:456-467`; `08-read-out.md:181`; `04-api.md:129-134`).
2. **`unknown` is the engine's own, in two shapes.** A facts budget or `--region-limit` stop exits 0 with a summary
   that says `region.exhausted: false` and `region.stopped.why: "budget" | "limit"` (and `verdict.word: null` under
   `--single`); a `-T` abort exits 2 with one stderr line and **no summary**. No stopped run says `Contradiction`
   (Q-M1d.6, Q-M1d.1; `05-cli.md` contract C3, line 36).
3. **An empty R is of two kinds, told apart on every surface**: `region.empty: "dead"` (`Contradiction`) or
   `"owing"` (`Open`). *Satisfiable, incomplete* exists on the model axis: `region.root: "owing"`, `root.owes`,
   `root.owed`, `region.frontier`.
4. **"One model" has two readings**, and the engine has a question for each: `--single` (k_s = 1) and
   `--maximal --single` (one maximal state); a program with restrictions only answers `Ambiguity` to plain
   `--single` whenever any candidate is alive, since ∅ is in R (`05-cli.md:361-362`;
   `06-solution-semantics.md:540-544`; run `b_restrictions_single`).
5. **Exit 1 is not "invalid"**: it is a load error, *or* a claim the run refutes (with a summary), *or* an
   `oracle` cube that contradicts a monotone verdict, *or* a dump directory that cannot be made
   (`05-cli.md:288-292`; `07-diagnostics.md:337`). Under `ein test` a load error is 2 and a false claim 1.
6. **A seventh situation Stage A's list does not have**: a program the static check flags is answered, with a
   warning on three surfaces, and nothing is claimed of the answer (Q-M1f.564).
7. **No JSON surface carries a diagnostic.** A parse or load refusal is text on stderr; `ein test --json-report`
   carries `outcome: "error"` and no message. A load's `note:` and a `-T` abort line are stderr only. (The same gap
   is on file in F23: D-18, *live*, and L-125, *left as is*.)
8. **The count is two counts**: `k_s` (states) and `k_c` (commitments), both `null` under a mode.
9. **The summary's "model" is a state's facts over the candidates' relations**, root's included
   (`region.states[].facts`, `08-read-out.md:392-397, 609`); the whole state is in no JSON key
   (`--dump-answer` writes each state as a loadable `.ein`, `ein-answer/3`).

---

## A. Stale claims

Claims checked and found **true today: about 35** (listed at the end of this section). All **111 links and
anchors** of the four files resolve (checked with a link reader of my own, not `doc_audit.py`); the staleness is in
what is said, not in where it points.

### A.1 `README.md`

| # | plan line · quote | true today | evidence |
|---|---|---|---|
| R1 | L4-8 · *"Not on M1d: the one word M2 needs from it (`unknown`) is computed here from fields the engine already emits, and replaced by the engine's own the day M1d gives it one"* | M1d shipped it. The fields named (`type, k, exhausted, unsat_core`) no longer exist; the engine says *not decided* itself | `docs/decisions/answer/Q-M1d.6-…md`; `explore.rs:456-467`; run `a6_limit_budget_single` (`verdict.word null`) |
| R2 | L31 · *"Taken 2026-08-23 against `ein.rs` at the M1a close"* | five kernel milestones later; every "exists" and "missing" cell below is of another engine | `git log`; this section |
| R3 | L37 (A1) · *"`absent_semantics.md`"* (link label) | the page is `03-absent-and-naf.md`; the link resolves, the label is the old name | `docs/ein/reasoning/02-kb-evolution/` |
| R4 | L37 · *"*invalid* = exit 1 with a diagnostic"* | exit 1 is also a refuted claim, an oracle contradiction, a dump directory that cannot be made; `ein test` exits 2 on a load error | `05-cli.md:288-292`; run `c_claim_refuted` |
| R5 | L37 · *"*inconsistent* = `Contradiction` with a core or exhausted; *multiple models* = `k ≥ 2` …; *unique* = `k = 1 ∧ exhausted`"* | none of `Contradiction`-as-`type`, `k`, `unsat_core`, `solutions` is a summary field. Today: `region.empty`, `region.k_s`/`k_c`, `region.exhausted`, `region.core`, `verdict.word` (only under `--single`) | § 0; `08-read-out.md:587-627` |
| R6 | L37 · *"*satisfiable incomplete* is the **query** axis … until M1d gives the engine an obligation-level *incomplete*"* | given: a state **owes** (`(open R)` facts; `region.root "owing"`, `empty "owing"`, `Open`) | `06-solution-semantics.md:826-838`; run `a3_owing_*` |
| R7 | L37 · *"the sixth, **`unknown`**, does not [exist]"* | exists, as § 0 item 2 | runs `a6_*` |
| R8 | L38 (A2) · *"the gate — 619 tests"* | not re-taken (no cargo run). `grep -rn '#\[test\]' ein.rs/crates` counts 1 391 attributes | the grep |
| R9 | L38 · *"the thirteen counter identities `summary_properties.rs` holds the summary to"* | the file holds the **region's** identities (k_c ≥ k_s, each line no larger than R, components summing to k_c, the kind of empty R, a query's classes partitioning its states) and one shape on every arm; no "thirteen" | `ein-cli/tests/summary_properties.rs:1-25` |
| R10 | L38 · *"the six-property fuzzer"* | seven `#[test]` properties | `ein-ir/tests/fuzz_properties.rs:504-939` |
| R11 | L38 · *"`-e` as the certification of uniqueness"* | `-e/--exhaustive` is retired and refused, exit 2. Uniqueness is `--single` with `decided yes` | `cmdline.rs:891`; `06-config.md:205`; run output in § 0 |
| R12 | L38, L76, L100, L123, L130 · *"`defined_behaviour.md`, where the thirteen behaviours already live"*; *"`docs/kernel/defined_behaviour.md`"* | the page and `docs/kernel/` are deleted (the README's own L96-108 says so). Its parts are `07-diagnostics.md`, `05-cli.md`, `01-knowledge-base.md` | `ls docs/kernel` fails; commit `3d0e8e6c` |
| R13 | L39 (A3) · *"`--json-summary` is **`ein-summary/1`**: `verdict {type, k, exhausted, unsat_core, solutions[{facts, goal_bindings}]}` … plus `stats`, `root`, `config`"* | `ein-summary/2`; keys as § 0. No `stats` (it is `work {saturations, facts, layers}`), no `solutions`, no `goal_bindings` (it is `queries[].classes[].rows`) | `region_answer.rs:26`; `08-read-out.md:543-627` |
| R14 | L39 · *"the fifteen kinds `05-events.md` tables plus `admit` and `warn` at the call sites"* | 18 kinds tabled (`run, load, region, monotonicity; compile, enqueue, fire, park, retire, admit, quiesce, alt, owe; rung, hyp, hypskip, enter, layer`); `admit` is tabled; **`warn` is retired**, no emitter writes it | `05-events.md:63-68, 100-109, 153-159`; `grep -o 'emit("…"'` over the crates; commit `75fe1b78` (*"drops … the retired warn"*) |
| R15 | L39 · *"the exit code for a budget abort is **2, the usage-error code, by design**"* | true of `-T` alone. A `--region-budget` or `--region-limit` stop exits 0 with a summary | runs `a6_limit_budget_*` (0), `a6_limit_time` (2); Q-M1e2.33 |
| R16 | L40 (A4) · *"`ParseError` is a struct — `file, line, col, context` — and the only typed one"* | `ParseError {file, errors: Vec<SyntaxError>, cut, recovery_steps}`; `SyntaxError {line, col, what, notes, repairs, more_repairs}`; no context (`07-diagnostics.md` § 1.2 *"There is no context block"*). Also typed: `ReadError {line, kind}`, `TextError::{Parse, Load}`, `EinbError` | `ein-ir/src/grammar.rs:146-179`; `read.rs:44`; `saved.rs:112`; `ein-einb/src/lib.rs:123` |
| R17 | L40 · *"`07-diagnostics.md § 1` pins the parse and load *messages*"*; L100, L123 *"§ 1 and § 4"* | § 1 is the parse, **§ 2** everything else (31 subsections, 2.1–2.31); there is no § 4 | `grep '^#' 07-diagnostics.md` |
| R18 | L40 · *"the unsat core is a value (`verdict.unsat_core`); `warn` events carry a `category`"* | `region.core` (+ `core_status`, `refuted_by`), `null` under a mode, `--count` and `--no-search`; no `warn` | § 0; `08-read-out.md:548-560` |
| R19 | L40 · *"no `dead_relation` / `unreachable_query` / `unused_fact` analysis anywhere"* | partly stale. A goal or a claim over a relation the program does not name is **refused at load** (`07-diagnostics.md` § 2.20-2.21; `examples/broken/load/expect_unknown_relation.ein`); `root.hypgen.outside` and the read-out's `outside` row name the relations no writer proposes a fact of (`ein-infer/src/scope.rs`); `ein-infer/src/strata.rs` builds the rules' relation dependency graph, which `ein saturate` prints strata from. Still absent: an *unused fact* or *dead relation* analysis as such | the files named |
| R20 | L41 (A5) · *"the corpus: 128 entries in six groups (… `compile-negative`, `regression`)"* | 933 entries; five groups have entries (`positive` 216, `stdlib` 316, `parse-negative` 27, `load-negative` 36, `regression` 338); `compile-negative` **has no entry** | `grep -c '^\[\[entry\]\]' corpus/corpus.toml`; `grep '^group' … | sort | uniq -c`; `corpus.toml:2920-2927`; `corpus/README.md:144` |
| R21 | L41 · *"the negative fixtures under `examples/broken/` … (37 today, in three groups)"* | 64 `.ein`/`.expected` pairs: 27 + 36 under `load/` + 1 under `compile/`, and that one (`activator_arity.ein`) is a `positive` entry that **solves, exit 0** | `find examples/broken -name '*.ein' | wc -l`; `corpus.toml:3066-3069`; `corpus_exits.txt` |
| R22 | L41 · *"design/02 for determinism"* (label) | the page is `03-design/01-determinism-and-order.md` | the link's own target |
| R23 | L41 · *"**unsat-core correctness and model-count correctness have no external check** — every test compares the engine to its own past"* | no longer every test: programs state their answer as `:expect` and `ein test` checks it (410 corpus `test` runs); `ein oracle` is a brute-force reference every mode is held to (`ein-infer/tests/explore.rs`); `corpus/regressions.toml` holds once-wrong answers to the right one. Still true: nothing in the gate compares with a system that is not Ein (`c/` and `nl2smt/` are wired into nothing) | AGENTS.md § The answer; `corpus.toml` runs census |
| R24 | L43-64 · the whole section *"The sixth outcome, today"*: `--max-enterings`; *"with the summary, `verdict.type = "Aborted"`, `reason`"*; `--max-set-size (default 5)` → *"`Contradiction`, `k = 0`, `exhausted = false`, empty unsat core"*, exit 0; *"ten corpus entries end this way"*; *"(not certified — pass `--exhaustive`)"*; *"it is M1d's, and the choice … is theirs"* | `-E`, `-m`, `-e` are retired (exit 2, each with what replaced it). An abort writes no summary. The depth cap is gone (*"a depth cap changed answers"*). The choice was taken: `Open`, and *not decided* for a stopped run | run output for `-e`, `-m 2`, `-E 5`; `06-config.md:201-211`; Q-M1d.6 |
| R25 | L70-71 · stages table: *"the mapping from `ein-summary/1` + `ein-events/1` + the typed errors"*, *"the corpus's 128 entries each producing one"* | as R13, R16, R20 | — |
| R26 | L80-82 · *"`cargo test --workspace` diffs them the way it diffs the summary goldens"* | **no golden of a summary exists.** `ein-cli/tests/golden/` holds `corpus_exits.txt` and `help_shape.txt`; `summary_properties.rs` says why it banks properties and not values | `ls ein.rs/crates/*/tests/golden`; `summary_properties.rs:3-11` |
| R27 | L87-90 · *"the three trip-wires of Q-M1a.23 re-checked against the decision"* | the file is a decision: three conditions inside its *Why*, no list of trip-wires, no dated check, and no measurement | `docs/decisions/engine/Q-M1a.23-…md` |
| R28 | L133-134 · *"`ein-cli/src/summary.rs` — `ein-summary/1`"* | `summary.rs` assembles `ein-summary/2`; the schema constant and the answer blocks are `ein-render/src/dump/region_answer.rs` | `summary.rs:256-299`; `region_answer.rs:26, 438` |
| R29 | L136-139 · *"Q-M1d.6, Q-M1d.1 — the verdict vocabulary's owner; M1c / M10 — A5's external check"* | both are decided and shipped; M1c shipped (`:expect`, `ein test`); M10 is unstarted | the decision files; `plans/m10_external_benchmarks/README.md:3-10` |

### A.2 `s2.1.1_census.md`

| # | plan line · quote | true today | evidence |
|---|---|---|---|
| C1 | L27-33 · *"`ein-summary/1` (`summary.rs:26`): `schema`, `source`, `verdict {type, k, exhausted, unsat_core, solutions}`, `stats`, `root`, `config` … `type` one of `Solution | Ambiguity | Contradiction | Aborted`"* | as R13. `summary.rs:26` is inside `fn sparse`. The words are `Solution | Ambiguity | Contradiction | Open`, under `--single` only; no `Aborted` | `region_answer.rs:26`; `explore.rs:458-467` |
| C2 | L34-37 · *"`ein-events/1` (`events.rs:32`) … `verdict` carrying `core` and `models`"* | the constant is `events.rs:27`. There is no `verdict` event: the last line is `region` (`asked`, `region` without `states` and `frontier`, `verdict`, `work`); a dead node's core is on its `enter` line | `05-events.md:67, 158`; run `ev_mini.jsonl` (kinds: `enqueue, fire, compile, alt, enter, hyp, quiesce, layer, run, load, region`) |
| C3 | L38-47 · *"`verdict.rs:31-38` has three variants; `Answer::Aborted { reason }` … `verdict.rs:52-57`"*; *"`--max-time` / `--max-enterings` raise `SolveError::Budget`"*; *"`--max-set-size` … (`ein-infer/src/solve.rs:1065-1069`)"*; *"Q-M1d.6's ten entries"* | `verdict.rs` is the query goal's projection now (its header says the four words are `explore::Verdict`); four variants; the abort is `SolveError::Deadline`; a budget stop is `Stopped::Budget` inside an answer, not an error; `ein-infer/src/solve.rs` does not exist | `verdict.rs:1-6`; `error.rs:12-32`; `region_answer.rs:68-96`; `ls` |
| C4 | L44 · *"**exit 2** — the usage-error code, deliberately (`ein-cli/src/lib.rs:9-11`)"* | true of `-T`; the comment is at `lib.rs:8-10` and still says *"2 again for a budget abort"* | `ein-cli/src/lib.rs:8-10` |
| C5 | L48-55 · *"`ParseError` … (`ein-ir/src/parse.rs:33-40`). `KbLoadError(pub String)` (`from_ir.rs:35`), `LoadError` (`imports.rs:56`), `MacroError` (`macros.rs:26`) and `CompileError(pub String)` (`compile.rs:42`)"*; *"§ 1 and § 4"* | `ein-ir/src/parse.rs` does not exist (the parser is generated from `ein.l` / `ein.y`); `ParseError` is `grammar.rs:170`; `KbLoadError` `from_ir.rs:31`; `LoadError` `imports.rs:81`; `MacroError` `macros.rs:26` (unchanged); `CompileError` `compile.rs:50`, with *"**No program reaches one**"* above it | the files |
| C6 | L56-61 · *"Nothing computes a dependency graph, nothing diffs two models, and the provenance DAG is a Rust value rendered only to DOT"* | a rule dependency graph: `ein-infer/src/strata.rs` (`edges`, `graph`, `rule_strata`); a diff of models: `ein test --diff [stats|full]`, a claim's models against the states or classes found, in `ein-test-report/6`'s `diff` (`ein-render/src/claim_diff.rs`) — not one run's models against each other; derivations also as markdown, `--trace FILE.md`, and as `fire` lines | `strata.rs:1-14`; `05-cli.md:483`; `ein solve --help` |
| C7 | L59-61 · *"Of the DOT views in `ein-render/src/lib.rs:31-44`, four reach the CLI (`render rules | rule | constraints | lattice`)"* | eight: `rules, rule, constraints, program, kb, region, lattice, h-space`; every renderer has a command (Q-M1e.20) | `ein render --help` |
| C8 | L62-64 · *"(`version.rs:41-58`, `109-118`)"* | content true; the lines are 39-57 and 108-120. `--version` prints no commit | `ein-cli/src/version.rs` |
| C9 | L79-80 · *"The page's and `AGENTS.md`'s "thirteen" is updated"* | `AGENTS.md` holds no "thirteen" | `grep -n thirteen AGENTS.md` (empty) |
| C10 | L89-91 · *"every claim a `file:line` in `ein.rs/crates/` or a page under `docs/kernel/` … confirm them at the M1a close commit"* | no `docs/kernel/`; the engine to cite is HEAD's | — |
| C11 | L96-97 · *"the `--dump-states` `summary.json` (`ein-render/src/dump/state.rs:5-16`), which is a *different* file"* | `-D/--dump-states` is retired; no `dump/state.rs`. `--dump-answer DIR` writes `answer.json` (= the summary, byte for byte), `index.json`, one saved file a state (`ein-answer/3`); `--dump-lattice DIR` writes `nodes.json` (`ein-lattice/2`) | run output for `-D`; `10-answer-dump.md:17-26`; `06-lattice-dump.md` |
| C12 | L103-104 · *"`compile` events (`ein-infer/src/engine.rs:196`)"* | `engine.rs:222`. `compile` is emitted on a plan-memo miss only | `05-events.md:102` |
| C13 | L104-110 · *"the difference is the dead set … the rule appears in `compile` and in no `fire`"* | an **obligation rule never fires**: it emits `owe`, never `fire` (*"A rule never emits both"*). A reading by `fire` alone calls every obligation rule dead. `ein-infer/tests/stdlib_coverage.rs` already takes this census over the stdlib, by `fire` and `owe` | `05-events.md:123-130`; AGENTS.md § tests |
| C14 | L111-120 · *"The loader layers a fact by its keywords — `:source` → fact, `:rule` / `:using` → reasoning, else ontology … Whether `(left-of A B :origin generated)` parses, loads, and survives the canonical dump"*; *"`ein-ir/src/parse.rs`"* | **answered**: each form takes a closed list of keywords; a fact takes `:source`, `:rule`, `:using`, and any other is a parse error at its position | Q-M1e2.28; run `kw_origin` (exit 1, *"unexpected `:origin`: a keyword this form does not take"*); `kw_source` loads |
| C15 | L128-135 · the outcomes table, every "operational test" cell and four "held by" cells: *"`compile-negative` corpus group"*; *"the `solve -e` cells"*; *"`solve -n 3` and `solve -e` cells"*; *"**no fixture today**"* (satisfiable, incomplete); *"`summary_properties.rs`, where `k = solution_nodes` is held or joins the thirteen"*; the `unknown` row | tests are on fields that are gone; no `-e` cell exists (corpus runs: 879 `solve`, 15 `solve --single`, 14 `--minimal`, 10 `--count`, 9 `-n 1`, 8 `-n 3`, 4 `--maximal`, …); owing fixtures exist (`tests/region/04_owing.ein`, `examples/features/13_mixed_solution_and_open.ein`; `embedding.rs`'s `the_other_three_verdicts_are_reachable`); no `k`, no `solution_nodes` in a summary | corpus runs census; `04-api.md:268-273` |
| C16 | L133 · *"`zebra2-minus-15` (32)"* | 32 states, yes — and a bare solve of it at the default budget is **not exhausted** (it stops in the classes); `--maximal` lists the 32 exhausted | runs `a4_multi_describe`, `a4_multi_maximal`; `corpus.toml:977` |
| C17 | L137-140 · *"every `derived` in a `fire` has premises that are earlier `load` or `fire` facts, over the corpus's event streams"* | the `load` event is one line of **counts and names** (`relations, rules, hypothesis_rules, macros, facts, relation_names, rule_names`); the loaded facts are not in the stream | `05-events.md:66`; run `ev_mini.jsonl` |
| C18 | L140-144 · *"`Contradiction ∧ exhausted ⇒ unsat_core ≠ ∅` — the core is the smallest frontier over recorded derivations"* | false by decision: `[]` is the empty core of a dead node (a rule whose `:match` is `()`), and a core is `null` under a mode, `--count` and `--no-search`. A core is *an explanation*, budgeted, not subset-minimal, and may be cut (`core_status: "cut-…"`) | `08-read-out.md:548-560`; Q-M1e2.27, Q-M1f.48, Q-M1f.29, Q-M1f.6; `13-glossary.md:312-317`; `corpus_cores.rs:1043` |
| C19 | L144-147 · *"no `(false)` and no `(X, ¬X)` pair in `solutions[].facts`; `-y` / `--lattice-sanity-check`"* | the kernel has no check of a fact beside its stored negation: such a state is dead only where the program imports `std.contradiction`, and alive without it. `region.states[].facts` holds only the facts over the candidates' relations. `-y` exists and is the parent-path spot check | Q-M1f.557; AGENTS.md § stdlib; `08-read-out.md:609`; `ein solve --help` |
| C20 | L156-162 · owners table: *"`unknown` at the `-m` cap"*; *"§ 4 pins the codes"*; *"`warn` absent from `05-events.md`'s payload tables … documents it when the producer reads it"* | no `-m`; the codes are `05-cli.md` § 3 and Q-M1e2.33; no `warn` to document | as above |
| C21 | L170-171 · *"Those are F1b's and M1d's questions"* | F1b was promoted into M1g (unstarted); M1d is closed | `plans/followups/f1b_logical_formulation.md:3-7` |

### A.3 `s2.1.2_feedback_object.md`

| # | plan line · quote | true today | evidence |
|---|---|---|---|
| F1 | L16-17 · *"spread over an exit code, a last line of stderr, `ein-summary/1` and `ein-events/1`"* | the diagnostic is not *a last line*: a parse refusal is several lines (each error, its notes, its repairs); on a flagged program stderr's **first** line is the warning; a load's `note:` lines come when the run ends | runs `c_syntax_error`, `b_flagged_describe`; `08-read-out.md:526-539` |
| F2 | L33, L45 · sketch: *"`"engine": { "type": "Ambiguity", "exhausted": true, "reason": null }`"*, *"`"stats": { "enterings_total": 101, "layers_explored": 2 …}`"* | no `type`, no `reason`; the counters are `work.saturations`, `work.facts`, `work.layers`. (`enterings_total`, `layers_explored`, `solution_nodes` survive only in `ein-einb/src/solutions.rs`) | § 0; `10-answer-dump.md:247-251` |
| F3 | L55-59 · `query.status` as a thing to derive from `goal_bindings` | the engine ships the second axis: `queries[]` with `over`, `exact`, `classes[]` (projection classes, each with `states`, `facts`, `rows`), and `expect` | `08-read-out.md:421-434, 619-624` |
| F4 | L65-66, L94-95, L158 · *"the string `defined_behaviour.md` § 1 / § 4 pins"*; *"`docs/kernel/inference/feedback.md`"* | no such page, no such tree; the events page is `docs/ein/ein.rs/05-events.md` | R12, R17 |
| F5 | L66-71 · *"`digests` … the canonical dump's SHA-256 (the program), the sorted fact list's (the saturated state — … § 1.5, the state class's identity, free with every solve) … none is a new kernel computation"* | `ein_ir::dump_canonical` is a library function no command prints (`ein-einb` stores its text). The sorted fact list is `ein_infer::canon::state_key`, internal; no JSON key holds a whole state. No digest of either is written anywhere but the stdlib manifest's | `grep dump_canonical ein.rs/crates/*/src`; `canon.rs:40`; § 0 item 9 |
| F6 | L81-85 · *"The ten Q-M1d.6 corpus entries yield `outcome = unknown` … and a `search_limit` diagnostic; `zebra2-minus-15` yields `multiple_models` with `k = 32` under `-e`; `ein-bugs/zebra2-bad.ein` yields `inconsistent` with a non-empty `unsat_core` whose entries carry their `:source` quotes"* | no such ten entries; no `-e`; the summary's core is strings with no `:source` (the quote is a stdout row) | R24; run `a2_zebra2_bad` |
| F7 | L109-119 · the derivation, every line: *"`exit 1 → invalid (diagnostics from the last line)`"*; *"`summary.type = Aborted → unknown`"*; `Contradiction ∧ …`; `k ≥ 2`; `k = 1 ∧ exhausted`; `k = 1 ∧ ¬exhausted`; *"no model binds the goal"* | exit 1 is four things (R4); an abort has no summary; the rest reads fields that are gone | § 0 |
| F8 | L121-128 · *"`query.status` and `bindings` come from `solutions[].goal_bindings`; `models` from `solutions[].facts` …; `stats` is the summary's block verbatim; … `warn` lines become diagnostics (`kind = other`, `data.category`) — and `05-events.md`'s payload tables gain the `warn` row they lack"* | `queries[].classes[].rows`; `region.states[].facts`; `work`; no `warn` event exists | R13, R14 |
| F9 | L129-131 · *"(a) inside `ein-cli` as `--feedback FILE.json`, additive like `--json-summary`, with the typed errors and `Answer` in hand"* | `--json-summary` is written only by a run that answered; no artefact option writes anything on a load refusal or a `-T` abort today | runs `a1_*`, `a6_limit_time`; `08-read-out.md:198-199` |
| F10 | L141-146 · *"matched against the finite set of shapes the corpus's `load-negative` and `compile-negative` groups exhibit (`undefined_symbol`, `arity_mismatch`, `type_error`, …)"* | `compile-negative` has no entry; Ein has no types (Q-M1f.372) and a relation is overloaded by its count of arguments (Q-M1f.447), so a fact of a declared relation at another count, and a fact of an undeclared relation, **load in silence**. What the loader refuses is `07-diagnostics.md` § 2.1–2.31 (a goal over a relation the program does not name; a conclusion's unbound variable; a missing `:why`; a macro's call at another count; import and name refusals; the hypothesis mechanisms' conflicts; …) | run `c_arity` (exit 0); `docs/decisions/language/Q-M1f.447-…md` |
| F11 | L162-165 · *"the 97 `solve`, 51 `solve -e`, 7 `solve -n 3` and 7 `solve -m 2` cells"* | 879 `solve`, 8 `solve -n 3`, no `-e`, no `-m` | corpus runs census (`grep -o '^runs…'`) |
| F12 | L173-175 · *"Every `.ein` under `examples/broken/` — the three groups … yields `invalid`"* | `examples/broken/compile/activator_arity.ein` solves, exit 0 | `corpus_exits.txt`: `0  examples/broken/compile/activator_arity.ein :: solve` |

### A.4 `s2.1.3_boundary.md`

| # | plan line · quote | true today | evidence |
|---|---|---|---|
| B1 | L16-18 · *"`07-diagnostics.md` § 1 / § 4"* | § 1 / § 2 | R17 |
| B2 | L25-29 · *"the census found the "values" advantage of the Rust column to be one struct"* | a Rust caller now gets `ParseError` with every error, its notes and ranked repairs, and the whole answer as values: `ReadOut {answer, states, queries}`, `answer.verdict: Option<Verdict>`, `answer.described`, `answer.stopped`, root's debts and core. The four load-side errors are still strings | `04-api.md:79-82, 127-141`; A.1 R16 |
| B3 | L42-44, L121-133 · *"the result recorded under them, dated — one status line, not a rewrite of a history page"*; *"One dated line under the entry's trip-wires"* | Q-M1a.23 is a decision file, which holds the final answer alone and no date (see C.4) | `docs/decisions/README.md:3-7` |
| B4 | L69-71 · *"Q-M1a.23 took it on 2026-08-21: `ein --help` **1.68 ms** … and `ein solve examples/zebra2.ein` 11 ms, cold"* | the decision file carries no number (a page states no measurement, Q-M1f.349, cited by Q-M1e.4). Re-taken here, **not** under `bench_env.sh` (AC on, cores not pinned, min of repeated runs): `--help` 2.1 ms; `solve zebra2 --no-search` 22 ms; `--single` 28 ms; bare `solve` (a description of all R) 93 ms. The argument (start-up ≪ one LLM call) is not changed by these | my timing run; `docs/decisions/docs/Q-M1e.4-…md` |
| B5 | L91-93 · *"a Python loop would re-parse with a second parser, which is drift, or need a dump the CLI does not have"* | still true that no command prints the parsed program as data. What the CLI does write of a program today: `ein render program` (DOT), `ein saturate --dump` (text), `ein show hyps` (H as Ein facts), `ein render h-space --dump DIR` (`h-space.json`), `--dump-answer` (states as `.ein`) | `ein render --help`; `05-cli.md` § 2.5 |

### A.5 Found true (count: about 35)

`ein-events/1` is the events schema and rides in the `run` event · `fire` carries `rule`, `bindings`, `premises`,
`derived` · `ein --version` prints engine, protocol, container, features and the resolved stdlib's manifest SHA ·
the summary is additive, order-free, self-describing, and `summary_properties.rs` reads it · it has no diagnostics
block, and a parse or load failure writes none · `-T` exits 2, the usage-error code · `KbLoadError`, `LoadError`,
`MacroError`, `CompileError` are `String` newtypes, the problems `; `-joined, with no kind and no id ·
`SaturateError`, `FireError`, `SolveError` are enums of runtime failures · `07-diagnostics.md` pins the lines as
normative · loader lines about a top-level form end `at None` (§ 1.5, Q-M1a.6) · `naf_deps.rs` is advisory ·
`trace/relevance.rs` is a post-hoc provenance backtrack · `KbDotOpts::since` exists · the summary's `root` block has
`facts_by_relation`, `plans` and the `hypgen` counters · `compile` events name rule × activator · `ein saturate
--dump` exists · every corpus `solve` run gains `--json-summary {out}/summary.json` (`ein-corpus/src/plan.rs:15,
48`) · a golden lives beside its crate (Q-M1a.9) and `EIN_BLESS=1` re-banks · every `examples/broken/` fixture has
its `.expected` · `ein-bugs/zebra2-bad.ein` is dead at root with a non-empty core · `zebra2-minus-15` has 32
states · `-y` exists · Ein has no binding (Q-M1a.23), no PyO3, no `pip` channel · `utils/` scripts drive the binary
by `$EIN_BIN`, default `ein.rs/target/release/ein` · `ein-ir` holds the `Ast` as values and `04-api.md` is the
crate surface · `.einb` exists · a fact takes `:source "…"` · the id-order and job-count invariance sweeps exist ·
`01-firing.md` has O1–O9 and O6 is provenance · the glossary exists · `utils/censuses/corpus-cost.md` exists · F16's
note has the list of diagnostics the plan quotes from (`ideas.md:238-255`) · M10, M5, F1b, S2.2.x, S2.3.2, S2.4.1,
S2.4.3, S2.4.4, S2.5.1 exist where linked · process start-up is small beside an LLM call.

---

## B. What each stage needs from the project, and its state today

### S2.1.1 — the census

| deliverable / premise | state today |
|---|---|
| T2.1.1.1, the A1–A5 table re-taken with `file:line` | **Needed, and larger than planned**: the stage says *"the rows above are the expected result; the task is to confirm them"*, and almost none confirms (A.1, A.2). Section A of this report is most of the re-take. *"Nothing in `ein.rs/` changes"* still holds for it |
| T2.1.1.1's two things to look for: a `root` field that answers *which relations have no facts*; whether `--dump-states`' `summary.json` carries more | first: `root.facts_by_relation` lists only relations that have a fact; `root.hypgen.outside` names relations no writer proposes; second: **moot**, `--dump-states` is retired; `answer.json` is the summary byte for byte |
| T2.1.1.2 (1), rules that never fire, from one `--events` run | **exists as assumed, with a trap**: `compile`, `fire` are there; an obligation rule is activated by `owe`, never `fire` (A.2 C13). The same census is already a gate test over the stdlib (`stdlib_coverage.rs`). `ein saturate --dump` lists each rule with its plans (`no plan compiled` for an activator that binds none, F23 D-15) |
| T2.1.1.2 (2), an unknown keyword on a fact | **already answered; the task is moot**: closed keyword lists (Q-M1e2.28); `(left-of A B :origin generated)` is a parse error (run `kw_origin`). A fact takes `:source`, `:rule`, `:using` |
| T2.1.1.3, the six outcomes as defined behaviour, with operational tests | **the page it writes into is gone; most of the substance shipped in another form.** The answer is defined in `06-solution-semantics.md` (R; § 2.2 ∅ ∈ R; § 2.3 the two counts and the two readings of *one solution*; § 5 `verdict(R)`, four cases; the two kinds of empty R); the operational side in `05-cli.md` (§ 1 the contract C1–C4; § 3 exit codes; § 4.2 *Is it unique?*) and `08-read-out.md` (every row with its summary field; § 11 the summary). **Not shipped**: one list keyed by Stage A's six names with the test of each on `ein-summary/2`'s keys. § 0 above is the fact base such a list would be written from. The four words and *not decided* are five of the six; *invalid* is not a summary state at all |
| T2.1.1.3, the six A2 invariants, each with the test that holds it | **shipped in another form, per invariant**: *unique = one ∧ exhausted* and *unsat = zero by exhaustion* are Q-M1d.1 and Q-M1d.6, held by `ein-infer/tests/explore.rs`'s `every_mode_is_the_oracle`; *ambiguous = two distinct* is `Ambiguity`, which prints two states; *every solution satisfies the constraints* is contract C1 (every state printed is a member of the oracle's R) and `answer_round_trip.rs` (every dumped state reloaded); *every contradiction has a recorded derivation* is `Kb::ended`'s record, over which a core is taken, with `[]` a legal core (A.2 C18); *every fact justified* is provenance (O6), and I found no test of the exact shape the stage sketches (searched test names only — unsure). Where a premise of the answer is listed with what enforces it today: `docs/ein/ein.rs/03-design/README.md` § What the answer rests on (four premises), `docs/ein/ein.rs/08-reasoning-in-code/` (per reasoning page: the code, the tests, the commands), and each decision's *Held by* |
| T2.1.1.4, every *missing* owned | owners moved: `unknown` at a cap — **shipped** (M1d); exit 2 — fixed by Q-M1e2.33, `-T` alone; kinds and locations on load diagnostics — **F23** owns `at None` (D-2, D-22) and *"a load diagnostic reaches stderr and nothing else"* (D-18, L-125); a relation dependency graph — exists in the engine (`strata.rs`), printed only as strata by `ein saturate`; a model difference — `ein test --diff` (claim against found); the provenance DAG as data — `fire` events (rule, bindings, premises, derived) and `--trace FILE.md`; `--dump-lattice`'s `nodes.json` holds each node's status and core, no firing; `warn` — **premise removed**; unsat-core / model-count correctness against something that is not Ein — still no gate check; the oracle and `:expect` are Ein's own; five wrong-answer rows about cores are open under F22 |

### S2.1.2 — the feedback object

| deliverable / premise | state today |
|---|---|
| `ein-feedback/1`, a schema and a producer | **does not exist** (`grep -r ein-feedback ein.rs/crates docs` empty; `--feedback` is refused by the command line, exit 2) |
| its input `ein-summary/1` | **exists in another form**, `ein-summary/2`, of another shape: the derivation of T2.1.2.2 has no line that still reads a real key |
| `verdict.outcome = unknown`, derived by M2 | **already shipped by the engine** for every run that writes a summary: `region.exhausted`, `region.stopped {why, spent, projected, …}`, `verdict.word: null`. What is left to a consumer: a `-T` abort (exit 2, one stderr line, no summary) and telling it from a usage error (both exit 2; the abort line starts `** aborted:`, a usage error `error:`) |
| `search_complete` | **shipped**: `region.exhausted` (and the read-out's `decided` under `--single`) |
| `k` | **shipped as two**: `region.k_s`, `region.k_c`; `null` under a mode; under a mode the count is the length of `region.states` with `listed` |
| `satisfiable_incomplete` | **shipped on the model axis** (`root`/`empty` = `owing`, `Open`, `root.owed`, `frontier[].owed`); the plan's query-axis reading (*no model binds the goal*) is a query's class with no row (*undetermined here*), also shipped |
| `query.status` (`answered / underdetermined / absent / not_applicable`) | **shipped in another form**: `queries[].classes` (one class, several, a class with no row), with `over` and `exact` |
| `models` | **shipped in another form**: `region.states[]` (`facts` over the candidates' relations, `closure`, `from`, `minimal_generators`, `commitments`, `lines`, `listed`); stdout cuts at 100 lines, the JSON does not |
| `unsat_core` entries with their `:source` | **half**: `region.core` (strings), `core_status`, `refuted_by`, under describe only; `:source` is on stdout and in `answer.txt`, in no JSON key |
| `diagnostics[]` with `kind`, `location`, `data` | **does not exist.** In the library a parse error is typed and placed (`SyntaxError`); every load refusal is a string, some with `FILE:L:C:` in front, some ending `at FILE:L:C`, some `at None`, some with no place. The premise *"the corpus is the test: every negative entry classified"* stands on 27 + 36 fixtures; beyond them, 127 more bare-`solve` cells of the corpus exit 1 (190 of 879 in `ein-cli/tests/golden/corpus_exits.txt`, 63 of them under `examples/broken/`), refusals and refuted claims not split here |
| `warn` lines as diagnostics | **premise removed**: no `warn` event. What warns today: stderr's `WARNING` line and `monotonicity.warning` for a flagged program; `note:` lines on stderr for a partial run, an unchecked claim, a redundant hypothesis declaration (in no JSON key, F23 L-125); the `monotonicity` event |
| `stats` verbatim | **in another form**: `work` |
| `versions` from `ein --version` | exists; **no commit** in it (Q-M2.1 wants the commit as a field: it has to come from outside the binary) |
| `digests` | **does not exist** on any surface (A.3 F5) |
| goldens over the corpus's `solve` cells | the cells are 879 today (the 128-entry premise is gone); sixteen entries are `slow`; flagged entries may bank no answer (C.3) |
| T2.1.2.3 step three, an optional `kind` on `KbLoadError` / `CompileError` | `CompileError` is unreachable by any program (`compile.rs:41-48`), so only the three load-side newtypes can matter |
| T2.1.2.5, every `examples/broken/` file → `invalid` | 63 of 64; `compile/activator_arity.ein` solves |
| the outcome `unknown` for *"`k = 1 ∧ ¬exhausted`, the uncertified solution"* | **premise removed**: `--single` says `Solution` only when decided; `-n 1` says *1 state, as asked — R may hold more* with `exhausted false, stopped.why "asked"` |

### S2.1.3 — the boundary

| deliverable / premise | state today |
|---|---|
| Q25, still to decide | **open**; nothing since has decided the loop's language. Its id collides (C.5) |
| *"the protocol exists"* as the decision's ground | it does not (S2.1.2); what exists for a process-driven consumer is larger than the plan's Python column: `ein-summary/2` (the whole read-out), `ein-answer/3` (states as loadable files), `ein-lattice/2`, `ein-region/1` (`ein oracle --dump`), `ein-test-report/6`, `ein-events/1`, `--trace FILE.md`, `ein show hyps`, `h-space.json` |
| the Rust column | larger too (A.4 B2); `04-api.md`'s worked example is a test (`embedding.rs`), with all four words and `None` |
| the measurement of T2.1.3.2 | the old numbers are no longer on file; a re-take is A.4 B4 (uncontrolled) |
| `utils/` as the precedent for Python driving the binary | exists as assumed; six `utils/` scripts read `--json-summary` today (`region.py`, `region_census.py`, `corpus_cost.py`, `guard_split.py`, `review/run.py`, `review/command.py`) |
| `einaf/`, `grammars/`, `packaging/compose/llm.yml`, `experiments/` | none exists; none is classified by the standing rules (C.1) |
| the trip-wires of Q-M1a.23, re-checked and recorded | the conditions are in the decision's *Why*; where a dated check may be recorded is C.4 |
| *"every path in the P2.2 – P2.5 stage files that said *in Q25's language* is corrected"* | those files exist; their content is the other auditors' |

**Every deliverable of the phase the engine has since shipped in some form**: `unknown` (as *not decided*, `exhausted`,
`stopped`); `search_complete`; the obligation-level *incomplete* (`owing`, `Open`); the query axis (projection
classes); the models as sorted lists; the unsat core with its status and refuting rules; the two-kind empty R; the
defined-behaviour statement of the verdicts; the invariants' enforcement; the answer to the unknown-keyword
measurement; a rule dependency graph; a model diff (for claims); a typed, multi-error, placed parse error (library
only).

**Every premise the engine has since removed**: `ein-summary/1` and all its field names; `verdict.type` on every
run; `Aborted` as a summary value; `-e`, `-m`, `-E`, `-D`; the depth cap and the ten truncated `Contradiction`
entries; the `warn` event; `docs/kernel/` and `defined_behaviour.md` with its § 4; `compile-negative` as a
populated group; the 128-entry corpus; `ein-ir/src/parse.rs`, `ein-infer/src/solve.rs`, `dump/state.rs`; types and
fixed arity as things a loader could refuse (`type_error`, `arity_mismatch`); a kernel check of a fact beside its
negation; the non-empty-core invariant; *smallest* as a property of a core; the per-fact reading of `load` events.

---

## C. Conflicts with standing rules and decisions

**C.1 The project and the process are two things (Q-M1f.345; AGENTS.md § Edits that go together).**
- `einaf/`, `grammars/`, `experiments/`, `packaging/` (`s2.1.3` L109-114) are in none of the three lists (project;
  process; external to both: `c/`, `nl2smt/`, `docs/lib/`). The checks read them by a deny-list:
  `utils/check_release_voice.py`'s `NOT_PROJECT` is `plans/, utils/, .github/, run_tests.sh, AGENTS.md, CLAUDE.md,
  c/, nl2smt/, docs/lib/, ein.rs/vendor/`, so **a new top-level directory is the project's by default** and its
  every `.md` and `.ein` comment is held to the release voice. Which list each tree joins is not decided anywhere.
- The recommended producer is `ein-cli`'s `--feedback` (`s2.1.3` L101-108; `README` L91-94), which is the
  project. Then its page, its code comments and its tests may not name the loop, the harness, a stage or the
  records: *"nothing in the project reads, links or names the process"*. `s2.1.2` L99-101 has the page say *"the
  renderers consume it, the records (S2.5.1) store it"*.
- **The research plan's level names collide with the release-voice patterns**: `F0`–`F8` match the followup
  pattern `F\d{1,2}`, and `M2`, `P2.1`, `S2.1.2`, `T2.1.2.3` the milestone, phase, stage and task patterns
  (`check_release_voice.py:76-88`). `s2.1.3` L103-104 puts *"F6's dependency block"* inside `ein-cli`.
- `s2.1.1` L78 and `README` L78-79 have the page mark an outcome *"as M1d's word with M2's derivation beside it"*:
  a milestone id in a project page.
- `s2.1.2` L162-169 banks the objects *"under the producing crate's `tests/golden/`"* and has *"the manifest say
  so"*; if the producer is not a crate (placement (c), Python), a golden diffed by `cargo test --workspace` cannot
  be its, and *"the tests read only the project"* (AGENTS.md § Running the gate): what ties a process file to the
  project is a `utils/` check and a gate step, mirrored in CI (`check_gate_steps.py`).

**C.2 The documentation states only the current state (Q-M1f.341), in its perspectives (Q-M1f.370), numbered
(Q-M1f.367).**
- `s2.1.1` L148-150: *"Where a test does not exist, the section names the one-line consumer-side test to add"* — a
  page naming work to do.
- The outcomes *"each with its operational test and the gate test that holds it"* (`README` L76-77) cannot be one
  page of `reasoning/`, which names no command, file or test; `README` L96-108 already says so and leaves it to
  the re-plan.
- A page added beside `05-events.md` (`s2.1.2` L94) renumbers `docs/ein/ein.rs/` and moves every reference in the
  same commit.
- A count in a page carries its command or its test (Q-M1e.4): *"the six outcomes"*, *"the six invariants"*, a
  list of kinds.

**C.3 A non-monotone program: nothing is claimed of its answer (Q-M1f.564).**
- `s2.1.2` L75-77 banks a golden of the object for **every** corpus entry's `solve` cell. For a flagged entry
  that is a golden of its answer: *"no line of the shape golden for an answer and no assertion holds a flagged
  program to what it answers"*; a flagged entry declares no run that checks a claim
  (`ein-corpus`'s `a_flagged_program_declares_no_run_that_checks_a_claim`).
- The object's `outcome` enum (`unique`, `multiple_models`, …) has no value for *answered, nothing claimed*, and
  the sketch has no field for the warning; `ein-summary/2` has `monotonicity.warning`, and every command that
  answers about a commitment must say it in capitals.

**C.4 A decision is the final answer alone, and the project's prose carries no date (Q-M1f.342, release voice).**
`s2.1.3` L42-44 and L131-133 record the trip-wire check *"under [Q-M1a.23's] trip-wires, dated"*. Q-M1a.23 is
`docs/decisions/engine/Q-M1a.23-…md`; a date there fails `check_release_voice.py`, and the file's form is *what
holds, why, held by*. (`plans/m2_nl_to_ir/open_questions.md` is the process's and may carry a dated verdict, which
is where `s2.1.3` L37-38 puts Q25's.) The measurement of T2.1.3.2 may not go into a project page either (no page
states a measurement, Q-M1f.349).

**C.5 An id is kept and never reused (AGENTS.md § docs/decisions; `plans/open_questions.md:9-10`).**
M2's ledger says *"Q7–Q11 and Q23–Q25 are sticky ids from the cross-milestone pool"* (`open_questions.md:6-8`). In
the root ledger **Q23, Q24 and Q25 are M1's**, each another question (`plans/open_questions.md:31-33`: an explicit
type slot; `:where`; cardinality and ordinality), and two have decision files:
`docs/decisions/language/Q23-a-premise-says-what-a-variable-ranges-over.md` and
`docs/decisions/stdlib/Q25-ein-has-no-ordinal-form-an-order-is-a-relation.md`. The root index lists Q7–Q11 as
M2's and no Q23–Q25 of M2. So *"Q25 decided"* (the title of S2.1.3) names an id that already resolves to a
decision about ordinals; a bare `Q25` in a comment resolves to that file.

**C.6 The CLI's behaviour is fixed, and every option is in the census (Q-M1a.13; `06-config.md`).**
A new `ein solve --feedback` is a row of `ein-cli/tests/golden/help_shape.txt`, moves `06-config.md`'s counts
(*"99 options across eighteen parsers"*, `ein solve` 36, held by `config_reference.rs`), needs its class in
§ 3.3 with the *does it change the answer* column, a row in `07-diagnostics.md` § 2.4 (*"Seven options name a file
…"*), and `05-cli.md`. The stage's acceptance, *"no golden under `ein-cli/tests/golden/` that existed before the
stage is re-blessed"* (`s2.1.2` L86-88), cannot hold for `help_shape.txt` under placement (a).
Also: an artefact that reports a **load** refusal would be the first artefact a refused program writes; today a
refusal writes none, and *"an artefact option … changes neither stream nor the exit code"* (`05-cli.md:330-332`).

**C.7 Every answer exits 0; an abort is no answer (Q-M1e2.33).** The plan proposes no change of exit code
(`s2.1.1` L157: *"nobody's to change"*), so there is no conflict; what conflicts is the derivation's first line,
*exit 1 → invalid*, with the decision's *"a claim the program states and the run refutes, 1"*. M2's README plans
gold verdicts as `:expect` (L301-302), which makes exit 1 on a valid program a normal event of the loop.

**C.8 No option changes R (Q-M1e2.30).** Nothing in the phase proposes one. Q-M2.1's *"resolved `SolverConfig`"*
as a record field is two flags today (`lattice-sanity-check`, `record-alternative-justifications`), the second of
which may move a dead node's core and nothing else.

**C.9 `.einb` is a private cache format, never an interchange one (AGENTS.md; `04-api.md:286-288`).**
`s2.1.3` L127-128 names `.einb` for *"state that must persist across items"*. Read by the same binary it is a
cache; stored in an experiment record or a released artifact (P2.5, P2.10) it would cross a tool boundary, where
only `.ein` text and the event protocol's JSON may.

**C.10 Ein has no types, and a relation is overloaded by its count (Q-M1f.372, Q-M1f.447).** The vocabulary of
`s2.1.2` L60-63 has `type_error` and `arity_mismatch` as kinds the producer assigns to load refusals; the loader
has no such refusal for a relation (a macro's call at another count is refused; `eq`/`neq`/`absent`/`open` at
another count are parse errors). The engine's own lines still say *type* and *value* in places (F23 D-20, the
user's call).

**C.11 The ruling on estimates.** The four files carry estimates (F below); later plans state *"It carries no
estimate (the user's ruling of 2026-11-08)"* (`plans/m1g_logic/README.md:4`, `plans/m15_mcp_server/README.md:4`).

**No conflict found** with: the stdlib rules (the phase touches none); `ein.rs/vendor/lrpar/`; the generated
files (unless a location is given to top-level forms, which moves `.expected` fixtures and
`ein-ir/tests/golden/load_error_order.txt`, not generated ones).

---

## D. Open questions and decisions the phase defers

| # | question | who the plan says decides | since |
|---|---|---|---|
| D1 | Q25, the loop's language | S2.1.3, on its own recommendation (Python drives the binary; the producer in `ein-cli`) | not answered. Both columns of the trade grew (B, S2.1.3). The id collides (C.5) |
| D2 | where the producer lives: (a) `ein-cli --feedback`, (b) a Rust `einaf` crate, (c) a Python module (`s2.1.2` L128-133) | S2.1.3 | (a) now meets Q-M1a.13, the option census and the project/process rule (C.1, C.6); (c) meets the string-only load refusals and the no-summary cases |
| D3 | which page the six outcomes and the invariants join (`README` L96-108) | *"decided when this phase is re-planned"* | not decided; the rules it must fit are Q-M1f.367, .370, .341, Q-M1e.4 |
| D4 | the engine's word at a cap: *say `Aborted`*, *qualify the rendering*, or *a fourth word* (`README` L56-58) | M1d | **answered**: `Open` for an owing empty R, *not decided* for a stopped run, `Contradiction` only where no node is alive (Q-M1d.6, Q-M1d.1) |
| D5 | Q-M1a.6, a position on loader messages about a top-level form (`s2.1.2` L150-158) | the stage *recommends* taking it up; *"the fallback, if that cost is declined"* — by whom is not said | still open; the decision says it *"waits for the change that wants positions on those messages"*; F23 holds it (D-2, D-22). The cost named (re-bless the corpus goldens) is today: 11 of the 36 `examples/broken/load/*.expected` hold `at None` and 2 `(None)` (`grep -l`), beside `ein-ir/tests/golden/load_error_order.txt` and whatever digest banks a refused program's stderr (not traced here) |
| D6 | an optional `kind` on the load errors in the crates (`s2.1.2` L147-150) | the stage, only *"if a message shape is genuinely ambiguous"* | open |
| D7 | the vocabulary of kinds (`README` L112-116: *"A4's list is the plan's guess; P2.6 is the experiment"*) | left open on purpose (an open enum, `other`) | Ein lost the notions two kinds name (C.10) |
| D8 | Q-M2.1, when the kernel is frozen | S2.5.1 | its premises moved: M1d and M1c landed; *"two kernel milestones are queued"* is now seven milestones planned after M1f and none started (M1g, M1h, M12, M15, M1k, M1n, M1p; not all touch the kernel), each *"started only on the user's word"*; the binary names no commit |
| D9 | the two measurements of T2.1.1.2 | S2.1.1 | (2) answered by Q-M1e2.28; (1) open, with the `owe` caveat |
| D10 | the command names `python -m einaf run | evaluate | report` | S2.5.1 *"may still adjust"* | open |
| D11 | the one condition that would reverse the verdict (a renderer or judge that needs the KB as values each iteration) | recorded as a trip-wire by S2.1.3 | open |
| D12 | *(arisen since, not in the plan)* which reading of *unique* an instance means: k_s = 1 or one maximal state | — | the engine asks the program to say (`06-solution-semantics.md:540-544`); the plan's `unique` has one reading |
| D13 | *(arisen since)* what the object says of a flagged program | — | Q-M1f.564 |
| D14 | *(arisen since)* what the object is built from where no summary is written (a refusal; a `-T` abort) | — | § 0 |
| D15 | *(arisen since)* which question the loop asks: a bare solve has no word and may stop in the classes on an under-determined program (run `a4_multi_describe`, ~10 s, not exhausted) where `--single` decides in 39 saturations and `--maximal` lists all 32 | — | `05-cli.md` § 4; Q-M1e2.31 |

---

## E. Dependencies

| the phase leans on | exists? | still says what the plan says it says? |
|---|---|---|
| M1a, shipped | yes | — |
| M1d (*"not depended on"*) | shipped; its plans deleted | its answers are decisions; the phase's interim derivation is moot |
| M1c (`:expect`; A5's thesis) | shipped; no folder | `:expect` and `ein test` exist; a region claim is `(solve … :expect (region …))`, a query's claim its projection classes |
| `plans/m10_external_benchmarks/` | yes, unstarted | yes: the external check, *"named, not scheduled"* |
| Q-M1d.6, Q-M1d.1 | decision files | **changed**: answers, not open questions |
| Q-M1a.23 | decision file | no trip-wire list, no measurement, no dates |
| Q-M1a.6, Q-M1a.9, Q-M1f.370 | decision files | yes |
| `plans/followups/f16_autoformalization/ideas.md` | yes | yes (the list at L238-255) |
| `plans/followups/f1b_logical_formulation.md` | yes | promoted into M1g (unstarted) |
| S2.1.1 → S2.1.2 → S2.1.3 | the files | the chain holds; S2.1.2's *"outcome table [S2.1.1] writes into `defined_behaviour.md`"* has no page |
| blocks P2.2 (S2.2.1 names the protocol), P2.4 (S2.4.3, nine renderers; S2.4.1 stops on `digests`), S2.5.1 (the record) | the files exist; `ein-feedback` is named in 13 files under `plans/m2_nl_to_ir/` and in no plan outside it | not read here (other auditors); S2.4.1 and S2.5.1 each still name `ein-summary/1` once |
| `docs/ein/ein.rs/05-events.md`, `04-api.md`, `07-diagnostics.md`, `01-firing.md`, `13-glossary.md`, `06-solution-semantics.md` § 1.5 | yes | each rewritten since; A lists where the plan's reading no longer fits |
| `utils/censuses/corpus-cost.md`, `utils/README.md` | yes | yes |
| `plans/m5_presentation/`, `p2.10_result_artifact_demo/` | yes | not read |

**Not named by the phase, and now on its ground:**
- **F23** (`plans/followups/f23_diagnostics_in_eins_words.md`): D-2 and D-22 (`at None`, `(None)`), **D-18** (*a load
  diagnostic reaches stderr and nothing else … has no key in `--json-summary` and none in `ein test
  --json-report`*), **L-125** (*the JSON summary carries no line the load or the run prints on stderr*), D-1
  (`FileNotFoundError`), D-3 (`ein test` on a refuted region claim: exit 1, empty stderr), D-20 (the engine's
  words *type*, *value*). Its trigger list includes *"a stage that touches the loader's or the compiler's error
  paths"*.
- **F22** (`f22_exact_core_past_the_cap.md`): a core is not the smallest in the cases its open register rows hold;
  bears on any invariant or renderer that reads a core as minimal.
- **F26** (`f26_consistency_and_completeness.md`): Ein's words (alive / dead, owes / discharged, exhausted) beside
  logic's; its trigger names *"when M2 or M5 must say what a verdict certifies"*, and its common-sense table is
  kept *"for M2"* (`plans/followups/README.md:35`).
- **M15** (`plans/m15_mcp_server/README.md`): an MCP server, a placeholder; it names M2's formalizer as a possible
  client and leaves *what a tool call has to return* undecided.
- **M1n** (non-monotone programs) and **M1p** (Ein to natural language: every fact a sentence, reasoning in
  words): unstarted; each is next to what the object would carry.
- `plans/open_questions.md` (the root ledger): the M1 owners of ids Q23–Q25 (C.5).

---

## F. Estimates, dates used as status, counts (a list)

- **Estimates**: `README` L3 (*1.5 weeks — 3 stages, 8 days*), L68-72 (*2 d, 4 d, 2 d*); `s2.1.1` L4; `s2.1.2` L4;
  `s2.1.3` L4. Speed and volume guesses: `s2.1.3` L72-76 (*a local 30B model at tens of tokens per second*, *a few
  thousand instances at five iterations*, *under a minute of startup*).
- **Dates as status**: `README` L31 (*Taken 2026-08-23 … at the M1a close*), L98 (*2026-11-17*, with a finding
  id); `s2.1.1` L16; `s2.1.3` L13 (*since 2026-08-23*), L70 (*2026-08-21*); *dated* as an acceptance word at
  `s2.1.3` L37-38, L43-44, L131.
- **Counts**: *five things, four shipped* (`README` L19); *619 tests*, *thirteen identities*, *six-property
  fuzzer*, *six outcomes*, *six invariants*, *thirteen behaviours* (L37-38, L76, L131); *fifteen kinds* (L39);
  *128 entries in six groups*, *37 fixtures in three groups* (L41, L71); *default 5*, *ten corpus entries* (L52;
  `s2.1.1` L47; `s2.1.2` L81); *nine renderers* (L11; `s2.1.3` L32); *four reach the CLI* (`s2.1.1` L61);
  *zebra2-minus-15 (32)*, *k = 32* (`s2.1.1` L133; `s2.1.2` L84); *97 / 51 / 7 / 7 cells* (`s2.1.2` L164-165);
  *1.68 ms*, *11 ms* (`s2.1.3` L70-71); *three trip-wires* (`README` L88; `s2.1.3` L42, L125); sketch values
  `enterings_total 101`, `layers_explored 2`, `k 4` (`s2.1.2` L32, L45).

---

## G. Seen on the way, outside this part

- `plans/m2_nl_to_ir/README.md:27` and `:281` cite the root `README.md`'s *§ EinAf* and its *thirteen components*;
  the root README has no such section and the word `EinAf` is nowhere in the project
  (`grep -ci einaf README.md` → 0; headings: What it does, The running example, How it reasons, The standard
  library, Build and run, ein-lang at a glance, Documentation, Layout, License).
- `plans/m15_mcp_server/README.md` says the binary has five subcommands; it has six and `ein kb`.
- `ein.rs/crates/ein-corpus/src/regressions.rs:716` says *"as `--json-summary`'s `verdict.type` spells them"*; the
  key is `verdict.word`.
- Under `--maximal` (the whole line, `exhausted true`) the summary writes `queries[0].over "listed"`, `exact
  false` (run `a4_multi_maximal`); I did not check this against `08-read-out.md` § 7.
