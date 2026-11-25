# M2 audit — P2.5 "baselines, metrics, records" and P2.6 "ablations", against the tree at `458bf90b`

Read-only audit, taken 2026-11-25 on HEAD `458bf90b` with `ein.rs/target/release/ein` (current with HEAD).
Nothing in the repository was edited. Every run's output is under
`(scratch, not kept: re-run the commands) p2.5_runs/`.

Files audited, each read whole:

- `plans/m2_nl_to_ir/p2.5_harness/README.md`, `s2.5.1_experiment_record.md`, `s2.5.2_baselines.md`,
  `s2.5.3_metrics.md`, `s2.5.4_first_table.md`
- `plans/m2_nl_to_ir/p2.6_ablations/README.md`, `s2.6.4_representation_ablations.md`
- context: `plans/m2_nl_to_ir/README.md`; `open_questions.md` Q11, Q-M2.1, Q-M2.2; `EinAf.md` Stages D, G, H, N.

Plan-file paths below are relative to `plans/m2_nl_to_ir/`. "P2.5 README" is `p2.5_harness/README.md`,
"P2.6 README" is `p2.6_ablations/README.md`.

Machine state of the timings quoted here (`utils/bench_env.sh --report`): i9-14900HX, AC (`ACAD online=1`),
governor `powersave`, pinned cpu4 (P-core), git `458bf90b`. They are single samples or best-of-7, taken to show
order of magnitude and which question was asked, not as measurements to keep.

---

## A. Stale claims

"Stale" here means a statement about the project that the tree contradicts today. Claims that are plans for
files that do not exist yet (`experiments/`, `einaf/`) are not counted as stale; they are in § B and § C.

### A.1 The machine outputs and their names

| # | plan file:line | quote | what is true today | evidence |
|---|---|---|---|---|
| 1 | P2.5 README:89 | "the Ein numbers are the summary's `stats`" | The summary has no `stats` block. Its top-level keys are `schema, source, asked, region, verdict, queries, claim, work, monotonicity, leftover, root, config`. The run's cost is `work` = `{saturations, facts, layers}`. | `ein solve examples/zebra2.ein --json-summary …/zebra2.summary.json`; `docs/ein/user/04-reference/10-answer-dump.md` § The run summary (line 235 ff.); `08-read-out.md:562-579` |
| 2 | S2.5.3:52 | "the summary's `stats`: `enterings_total`, `saturate_count`, `layers_explored`, `nogoods_emitted`" | None of the four is a key of any output of any command. The names survive only as fields of `ein_einb::solutions::BaseStats`, "the walk's per-candidate counters … No route keeps them … no command writes the section". | `ein.rs/crates/ein-einb/src/solutions.rs:29-45`; `grep -rn enterings ein.rs/crates` finds only that file and one comment in `ein-corpus/src/manifest.rs:275` |
| 3 | S2.5.3:112-113 | "the summary's `stats` block" | as 1 | as 1 |
| 4 | S2.5.3:135-136 | "`enterings_alive` / `enterings_dead_*`, `facts_merged`, `forced_positives` and `nogoods_subsumed` stay in the record" | No command emits them, so no record can hold them. | as 2 |
| 5 | P2.5 README:89, S2.5.3:52 | "enterings" as an H4 metric | The walk is gone "with its flags and words". The unit a route counts is a **saturation** (a node fixpoint computed); a set a licence decides costs none. | `AGENTS.md:854-856`; `08-read-out.md:135` (`work` row) |
| 6 | S2.5.1:55-56 | "the path of the `ein-summary/1` it was produced from" | The schema is `ein-summary/2`. | `"schema": "ein-summary/2"` in every summary written here; `08-read-out.md:546, 590`; `ein solve --help` (`--json-summary`: "write the read-out as ein-summary/2") |
| 7 | S2.5.1:72 | "`solver_config`: the 17 fields, from the summary's config block" | The summary's `config` block has **2** keys: `lattice-sanity-check`, `record-alternative-justifications`. The type `ein_core::SolverConfig` still has 17 fields, of which 15 are retired, "read by nothing", kept only for the `.einb` layout; a program that sets one is refused at the parse. | summary `config` block; `ein solve -c` prints two lines; `ein.rs/crates/ein-core/src/config.rs:39-42` (`FLAGS`), `:54-126` (`RETIRED`), `:323-324` (`assert_eq!(FIELDS.len(), 17)`, `(FLAGS.len(), RETIRED.len()) == (2, 15)`); `ein-cli/tests/summary_properties.rs:41` (`CONFIG_KEYS: [&str; 2]`); `06-config.md` § 2, § 2.4 |
| 8 | S2.5.1:67 | `"ein": {"max_time": 30, "max_enterings": null}` | `-E/--max-enterings` is retired and refused: `ein solve … -E 100` exits 2, *"-E/--max-enterings is retired: `--region-budget FACTS` bounds the work"*. The bounds today are `--region-budget FACTS` (default 2^20, facts beyond root summed over every saturation), `--region-limit N` (default 16, flagged programs only) and `-T/--max-time`. | run (`…/retired.err`); `06-config.md:203-217`; `ein solve --help` |
| 9 | S2.5.1:77-81 | "`true` while the loop derives `unknown` from `(type, k, exhausted, core)` itself, `false` the day M1d gives the engine a word for it" | M1d shipped (and M1e, M1e2, M1f after it). The verdict key is `verdict.word`, not `type`; it exists only under `--single`; its values are `Solution`, `Ambiguity`, `Contradiction`, `Open`, and `null` where the run stopped first (stdout: *"not decided — the run stopped first"*). `k` is two counts, `region.k_s` and `region.k_c`, both `null` under a mode. A stop is `region.stopped = {why: asked|budget|limit, spent, projected, …}`. | `docs/decisions/answer/Q-M1d.6-…md`, `Q-M1d.1-…md`; runs `…/zebra2.single.json`, `…/zebra2.budget.json`; `08-read-out.md:181, 185, 594` |
| 10 | P2.5 README:94-95 | "Ein certifies k = 1 ∧ exhausted" | As 9. Under `--single` on `zebra2`: `verdict.word = "Solution"`, `region.exhausted = true`, `region.k_s = null`. Under a bare `ein solve`: `k_s = 1`, `k_c = 261 982`, `exhausted = true`. `k = 1` therefore has to say which count, and `exhausted` is not the same field on the two questions (§ B.4). | `…/zebra2.single.json`, `…/zebra2.summary.json` |
| 11 | S2.5.3:24-26 | "`summary_properties.rs`, which holds the engine's counters to thirteen arithmetic identities" | The file exists at the linked path. It now holds `ein-summary/2`'s **region** identities ("k_c ≥ k_s, each line no larger than R, the components summing to k_c, an empty R exactly when k_s = 0 …, a query's classes partitioning the states"), one shape test over every arm, and one structural zero (`root.saturator.naf_dropped`). At the plan's date the file was "the T1 counters, banked as properties … the identities that make a counter set coherent", over the walk's `stats.*` (`git show 4836b743:…/summary_properties.rs`, 2026-08-21), so the sentence described that file; it does not describe this one. The count "thirteen" is in neither version (`git log -S'thirteen' -- …/summary_properties.rs` → nothing). | `ein.rs/crates/ein-cli/tests/summary_properties.rs:1-41` |

### A.2 Measurement instruments and history

| # | plan file:line | quote | what is true today | evidence |
|---|---|---|---|---|
| 12 | P2.5 README:144; S2.5.1:18 | "`baseline.md`" (as the document whose numbers name instrument and machine state) | Not in the tree. *"The walk's own measurements, `baseline.md`, `scaling.md`, `feature_cost.md` and the M1d censuses, went with the walk and its seven instruments, and live in git."* | `utils/README.md:139-141`; commit `d568d9ee` (2026-11-17) |
| 13 | S2.5.1:34-36 | "when the milestone closes the measurement *documents* go to `docs/history/` the way M1a's did" | `docs/history/` does not exist; it was deleted on the user's ruling ("remove complete milestones and docs/history, the development history is in git"). The standing rule is that the project documents only its current state. | `ls docs/` → `decisions demo ein lib`; commit `cf78590e` (2026-11-17); `docs/decisions/docs/Q-M1f.341-…md`; `AGENTS.md:60-63` |
| 14 | S2.5.3:113-114 | "peak RSS measured as M1a's end-to-end baseline measured it, the process cold" | The instrument (`utils/e2e_baseline.py`) is deleted (the plan's line 53 says so itself); commit `1626c688` deleted it with `fork_split.py`, `feature_matrix.py` and four more. Peak RSS is taken today by `utils/arith_cost.py` (`os.wait4` → `ru_maxrss`, lines 92-96) and, inside the test pool, from `/proc/self/status` `VmHWM` with one worker (`ein-corpus/src/pool.rs:44-45, 305`, the `EIN_TEST_UNITS` line). The `ein` binary reports no memory figure on any surface. | `utils/README.md:133, 139-141`; `git log --diff-filter=D -- utils/e2e_baseline.py` |
| 15 | S2.5.1:147-148 | "the corpus is 128 entries" | 933 entries. | `grep -c '^\[\[entry\]\]' corpus/corpus.toml` → 933 |
| 16 | S2.5.1:148 | "`zebra2`'s solve is 29 ms" | A time with no command, core or power state. Today the figure depends on the question: process wall, cpu4, AC, best of 7: `ein solve zebra2.ein --single` 28.4 ms (7 saturations); bare `ein solve zebra2.ein` (describe) 92.4 ms (120 saturations). `corpus.toml`'s own `cost_ms` for the entry is 2239 over its twelve declared runs. | runs here; `corpus/corpus.toml:997-1002` |
| 17 | S2.5.1:148-149 | "so `solver.jsonl` stays in kilobytes per task" | True of a `--single` summary (`zebra2`: 8.9 KB; `zebra2-minus-15`: 12.7 KB; `zebra2-bad`: 4.0 KB). Not true of a description of an ambiguous instance: `ein solve zebra2-minus-15.ein --json-summary` writes **1.30 MB** (32 states listed with their closures and minimal generators), past the plan's own 1 MiB threshold; `--maximal` on the same file writes 146 KB. | `ls -la` of `p2.5_runs/` |

### A.3 The standard library (G9)

| # | plan file:line | quote | what is true today | evidence |
|---|---|---|---|---|
| 18 | S2.6.4:75 | "**full** (all seven modules …)" | 28 module files, 28 lines in the manifest: the seven of 2026-08 (`algebra, bijection, closure, elim, macro, slots, typing`) plus `core, contradiction, negation, debts`, four under `bool/`, thirteen under `math/`. | `find stdlib -name '*.ein' \| wc -l` → 28; `wc -l stdlib/MANIFEST.sha256` → 28 |
| 19 | S2.6.4:73 | "**none** (no `import` permitted; the model writes every rule)" | A program with no import line still gets `std.core` (its rule `std.core.inconsistent`), loaded into every program. `ein saturate examples/lattice/10_fifo_gap.ein --dump` (a file with no import) lists `std.core.inconsistent`. A library named by `$EIN_STDLIB` with no `core.ein`, or with no `MANIFEST.sha256`, refuses every program. So "no stdlib" cannot mean an empty library; see § B.6. | `AGENTS.md:247-255`; `docs/decisions/stdlib/Q-M1f.551-…md`; `docs/ein/user/01-install.md:111-133`; runs here (the empty-directory refusal is quoted in § B.6) |
| 20 | S2.6.4:74 | "**minimal** (`std.algebra` only — the signature and the property closures)" | `std.algebra` imports `std.macro` and `std.contradiction` (`stdlib/algebra.ein:49-50`), and an import brings a module whole with its own imports. "`std.algebra` only" is therefore `std.algebra` + `std.macro` + `std.contradiction` + `std.core`: 40 + 2 + 14 + 1 top-level `rule`/`macro`/`relation` forms. | `stdlib/algebra.ein:40-50`; `AGENTS.md:576-586`; `grep -c '^(rule\|^(macro\|^(relation' stdlib/*.ein` |
| 21 | S2.6.4:106-107 | "on the six stdlib families" | Unsure. The count rests on S2.3.1's per-family table (another auditor's), written when the library was seven modules. The library now also holds a calculator, Boolean circuits, negation/contradiction and debts; whether the split is still six with a theory and two without is not something my files or the tree settle. By module names alone there is still no state-transition and no temporal (Allen) module (S2.5.4:97-98's "two families with no stdlib theory" is consistent with `find stdlib`). | `find stdlib -name '*.ein'` |

### A.4 Other plans and followups the files lean on

| # | plan file:line | quote | what is true today | evidence |
|---|---|---|---|---|
| 22 | P2.6 README:113-115 | "F9 — the repo's precedent for a ledger of measured-and-declined mechanisms" | F9 is closed and its file deleted; the precedent is in git only. | `plans/followups/README.md:25` |
| 23 | `open_questions.md`:239-246 (Q-M2.1) | "two kernel milestones are queued: M1d changes what the engine says at a depth cap … and may change the traversal (Q-M1d.4), and M1c may find a stdlib rule wrong" | Both shipped, and three more after them (M1e, M1e2, M1f). There is no depth cap (`-m` retired: *"a depth cap changed answers"*) and no traversal lever. "No milestone is current … The work stops after M1f: nothing below starts without the user's word." | `AGENTS.md` § Where the work is; `plans/README.md:359`; retired-flag run |
| 24 | `open_questions.md`:255-259 (Q-M2.1) | "until it lands, the loop computes `unknown` itself from `exhausted = false` and `verdict.type`" | As A.1 #9. | as #9 |
| 25 | `open_questions.md`:250-251 (Q-M2.1); P2.5 README:47-48 | "the resolved `SolverConfig`" | The type name is current. What resolves is two flags (A.1 #7); neither changes R, one may change a dead node's core. | `06-config.md` § 2.2, § 2.5 |

### A.5 Context only (not in my seven files, seen on the way)

- `README.md` (M2):27-30 and :281-283 say the root `README.md` "keeps two things apart: Ein … and EinAf" and
  has a "§ EinAf — the thirteen components". `grep -n -i 'einaf\|autoformal\|thirteen' README.md` finds nothing;
  the root README's sections are *What it does … Layout, License*.
- `p2.3_benchmark/s2.3.4_splits.md:51-52`: "each certified by `ein solve -e` before admission". `-e` is retired
  (exit 2). S2.6.4's *IR + known theory* condition and S2.5.4's `val` split both rest on that certification.
- `p2.1_kernel_as_instrumentation/s2.1.2_feedback_object.md:110-116` maps `summary.type = Aborted`, `k ≥ 2`,
  `k = 1 ∧ exhausted` onto the outcomes `unknown | multiple_models | unique` that S2.5.2:109 and S2.5.3:40-42, :60
  read. The mapping is written against the walk's summary.

### A.6 Claims checked and found true: 31

1. `utils/bench_env.sh` exists and prints the machine state (P2.5 README:26, :144; S2.5.1:109; S2.5.4:87).
2. `cost_ms` is recorded in `corpus/corpus.toml`, and `slow` is a measured claim checked both ways (P2.5 README:27; S2.5.1:19-20).
3. `utils/censuses/corpus-cost.md` exists (P2.5 README:145; S2.5.1:21).
4. F13's `ideas.md` holds the four oracle ablations and "где именно теряются остальные 18%" (lines 519-526) (P2.5 README:148; P2.6 README:71; S2.6.4:55, :67).
5. M5's README exists; "§ 6 Experimental method" and "§ 9.3" are sections of the paper outline in `EinAf.md` Stage P (lines 1268, 1286), which M5's README:13 adopts.
6. Every link and anchor of the seven files resolves: `python3 utils/doc_audit.py --links --check plans/m2_nl_to_ir/p2.5_harness plans/m2_nl_to_ir/p2.6_ablations` → "7 files (7 md), 0 finding(s)".
7. `EinAf.md` Stage N proposes `docs/experiments/experiment-0042/` (lines 1205-1215) (S2.5.1:31-32).
8. `ein --version` begins `ein 0.1.0` / `protocol   ein-events/1` (S2.5.1:72).
9. `ein-events/1` is the events schema (S2.5.1:58).
10. The versioning rule quoted from `05-events.md` is the page's (§ Versioning, lines 308-312) (S2.5.1:106-108).
11. `stdlib/MANIFEST.sha256` is SHA-256 lines (S2.5.1:155).
12. Stage Q's layer 4 is "Raw results — not just charts" (`EinAf.md`:1330) (S2.5.1:156-157).
13. Stage N is the plan's last-but-three stage (S2.5.1:16).
14. `:goal-text` is a `(query …)` keyword (`examples/zebra2.ein:524`; `02-syntax.md:81`) (S2.5.2:108) — but see § B.3 on where its rendering is printed.
15. `ein-cli/tests/summary_properties.rs` exists at the linked path (S2.5.3:24) — content claim stale, A.1 #11.
16. `--timing` (`-t`) exists and is a phase breakdown: load, route, read-out, print, total (S2.5.3:52).
17. `e2e_baseline.py` was "deleted with the walk; in git" (S2.5.3:53).
18. `plans/README.md` has the milestone table S2.5.4:105 hands its paragraph to (line 370).
19. No link-grammar submodule is registered: no `.gitmodules`, `git submodule status` empty (S2.6.4:10-13, :38-39; Q11:157-158).
20. `nl2smt/prototype/` holds `xxx-link.py` and `xxx.py`; `xxx-link.py` imports `linkgrammar` and holds the English Zebra text (S2.6.4:40-42).
21. M1j's pipeline drives the system `link-parser` 5.13.0 by its command line and needs neither submodule nor bindings (`nl2smt/README.md:114, 132, 645-652`; `link-parser --version` → 5.13.0; `import linkgrammar` → ModuleNotFoundError) (S2.6.4:43-46).
22. `ein --version` prints the SHA-256 of the stdlib manifest it resolves (S2.6.4:71-72).
23. F8's `ideas.md` defines C(n) (line 324) (P2.6 README:77; S2.6.4:81-84).
24. Q11's entry says what S2.6.4 and P2.6 say it says (`open_questions.md`:141-158).
25. No stdlib module is named for state transitions or temporal intervals (S2.5.4:97-98).
26. `plans/m12_rules_as_data/README.md:68, :223-224` say S2.6.4 "plans to take the first point of that curve by machine" / "counts rule schemas a model writes".
27. `AGENTS.md`'s `nl2smt/` bullet says `nl2smt/prototype/` "holds the two scripts M2 S2.6.4 starts from".
28. `std.algebra` exists and is the relation-algebra signature plus property checks (`stdlib/algebra.ein:1-10`) (S2.6.4:74).
29. P2.8's README exists and leans on M10 for a second representation (P2.5 README:76-77; S2.5.2:43-45); M10's folder exists, unstarted.
30. The eleven sibling stage files named as dependencies exist and carry the notion named: S2.1.2 (`ein-feedback/1`), S2.2.1 (catalogue), S2.2.2 (invocation envelope, T2.2.2.4), S2.2.3 (`grammars/theory.gbnf`, import of `std.*`), S2.2.4 (four passes), S2.2.5 (record), S2.3.2 (the three classes; § The unit of constraint precision / recall, line 82), S2.3.4 (`dev`/`val`/`test`), S2.4.1 (`transitions.jsonl`, the cycle predicate), S2.4.3 (F0–F8), S2.4.4 (§ Faithfulness events).
31. The phase's arithmetic: four stages × 3 d = 12 days (P2.5 README:3, :106-109).

---

## B. What each stage needs from the project, and its state today

### B.1 The record's fields (S2.5.1 `config.json` and the directory), against where each can be read

| field (S2.5.1:64-75, :40-60) | where it can be read today | state |
|---|---|---|
| `ein.commit` | **Not from the binary.** `ein --version` prints five lines (below) and none is a commit; `ein-cli` has no `build.rs` and no git hash is compiled in. The checkout's HEAD is `git rev-parse`; `utils/bench_env.sh` prints `git <short HEAD>` (line 257), with no dirty-tree flag. A binary's identity with a commit is therefore a fact about how it was built, which nothing in the binary states. | exists in another form |
| `ein.version_output` | `ein --version`, five lines: `ein 0.1.0` · `protocol   ein-events/1` · `container  einb/2.2` · `features   einb, parallel, snmalloc` · `stdlib     sha256:292894be…d681  checkout /home/user/work/ein/stdlib`. The last line holds an absolute path, and says which of the three places the library was found in (`$EIN_STDLIB <path>` / `checkout <path>` / `embedded`). | exists as assumed (two lines more than the plan quotes) |
| `stdlib_manifest_sha256` | `ein --version`'s `stdlib` line = `sha256sum stdlib/MANIFEST.sha256` (both `292894be…d681` here). **The load tests the manifest's presence, not its content** (`ein-ir/src/stdlib.rs:26-31`: "*Presence* is what the walk tests"; `01-install.md:135-139`: "No warning is given when the resolved manifest differs"). The manifest is held to the files at a commit by the gate step `python3 utils/stdlib_manifest.py` (`run_tests.sh:772-773`, `per-commit.yml:42`), not at run time. | exists; identifies the files only where the manifest is current |
| `ein.solver_config` | `ein solve -c` (two lines, before the read-out); the summary's `config` block (two keys); the `run` event's `config` (the same two keys, and `argv`, `impl`, `file`). | exists, 2 fields not 17 (A #7) |
| `budgets.ein.max_time` | `-T/--max-time SECONDS`. **An abort is not an answer**: `ein solve examples/zebra2-minus-15.ein -T 0 --json-summary t0.json` → exit 2, stdout empty, stderr `** aborted: --max-time 0 s reached before the question was decided **`, **no summary file written**. The help: "checked before each step of work". | exists; an aborted solve leaves no summary for `solver.jsonl` |
| `budgets.ein.max_enterings` | retired (A #8). The engine's bound that stops a run *with* an answer is `--region-budget FACTS` (default 1 048 576): exit 0, `verdict.word = null`, `region.exhausted = false`, `region.stopped = {why: "budget", spent, projected}`, stderr `note: FILE: partial — …`. `--region-limit N` (default 16) is the same for a flagged program's cube. | does not exist; another bound does |
| `spec_commit` | (S2.2.1:154: "the Ein commit the excerpts were cut at") — `git`. | harness's |
| `model.*`, `generation.*`, `prompts.*`, `benchmark.*`, `condition`, `level`, `strategy`, `budgets.iterations/output_tokens` | nothing in the tree; these are S2.2.2's, S2.3.4's, S2.4.x's. | does not exist (planned) |
| `unknown_derived` | the premise is gone (A #9): the engine says when a run stopped before it decided. | moot as written |
| `machine.txt` = "`utils/bench_env.sh` output, verbatim" | `utils/bench_env.sh --report` prints, **to stderr**: date, host, cpu, pinned core, governor, turbo, cur/max MHz, **power** (`AC (ACAD online=1); BAT1 Full 100 %`), loadavg, perf_paranoid, git short hash. It has no GPU, no RAM and no model-server line (`grep -ci 'gpu\|nvidia' utils/bench_env.sh` → 0). Run with a command it also `unset EIN_STDLIB`, sets `LC_ALL=C` and `exec taskset -c` onto **one** P-core thread (cpu4) unless `--cores` is given (lines 274-276); `--require-ac` exits 3 on battery. | exists; its fingerprint is of the CPU side only, and it pins what it runs |
| `theories/<task>/<iter>.ein` | plain `.ein`; every command takes one. | — |
| `solver.jsonl`: "every `ein-feedback/1` object, with the path of the `ein-summary/1`" | `ein-feedback` exists nowhere in `ein.rs/` (P2.1 has not run). `ein-summary/2` exists. **No summary is written on a parse or load error** (`ein solve examples/broken/absent_arity_high.ein --json-summary broken.json` → exit 1, stdout empty, stderr `file:15:38: unexpected …`, no file) **nor on a `-T` abort**. For those outcomes the only sources are stderr text and the exit code. | exists in another form; absent for two of the outcomes |
| `events/<task>/<iter>.jsonl.gz` (`ein-events/1`, "only when the level needs it") | `--events FILE [--events-level normal\|verbose]`. Sizes measured on `zebra2`: `--single`, normal: 10 753 lines, 1.8 MB; describe, normal: 186 486 lines, 30.4 MB; describe, verbose: 203 237 lines, 37.5 MB. `05-events.md` ground rule 4: "Cost when enabled is irrelevant. This is a debugging and comparison mode, never a benchmark mode; no line carries a time." Measured: `--single` 28.4 → 83.6 ms with `--events`; describe 92.4 → 567.8 ms. | exists as assumed; a solve that records events is not a solve whose wall is an H4 number |
| `metrics.json`, `transitions.jsonl`, `generations.jsonl`, `tasks.jsonl` | harness's. | does not exist (planned) |

`ein --version`, verbatim (`…/p2.5_runs/` has no file for it; the command is one line):

```text
ein 0.1.0
protocol   ein-events/1
container  einb/2.2
features   einb, parallel, snmalloc
stdlib     sha256:292894befd300e536cdd1181db772f44fe3f3f7ad6ff52b1467f1735ff83d681  checkout /home/user/work/ein/stdlib
```

### B.2 The H4 system metrics (`EinAf.md`:866-877; P2.5 README:89; S2.5.3:50-54), against today's counters

| H4 metric | the plan's source | what exists today, and where | state |
|---|---|---|---|
| LLM calls, tokens in/out | the envelopes | nothing in the tree | planned (S2.2.2) |
| wall-clock latency | "the harness's clock" | the harness's. `ein solve -s` prints a `wall` row and `-t` a `timing` table, **on stdout only**; the summary, the events and the answer dump carry no time (the summary is deterministic, the events say "no line carries a time"). The one JSON surface with a time is `ein test --json-report` (`ein-test-report/6`), whose rows carry `ms` (`ein-cli/src/test.rs:428`). | exists in another form |
| Ein runtime | "`--timing` the phase breakdown when asked" | `-t`: `load` (parse, imports, the program), `route` (root, H and every node visited, with the saturation count), `read-out`, `print`, `total`. On `zebra2` describe: 21.66 / 71.60 / 3.18 / 0.11 / 96.54 ms. Text, not JSON. | exists as assumed, as text |
| search nodes (`EinAf`), "enterings" (plan), `enterings_total` | summary `stats` | `work.saturations`: "node fixpoints computed, root's included". **It is a function of the question asked**: `zebra2` describe 120 saturations / 5 411 facts; `--single` 7 / 278; `zebra2-minus-15` `--single` 39 / 2 185, `--maximal` 108 / 8 601, describe 11 222 / 1 048 507 (budget reached). Per node: `--events`' `enter` lines (`layer`, `commitment`, `kind` = `dead`/`owing`/`in-r`, `owes`, `core`); `--dump-lattice DIR` (`ein-lattice/2`). | exists under another name and definition |
| saturation rounds, `saturate_count` | summary `stats` | No per-run "rounds" counter. Nearest: `work.saturations` (count of saturations); `root.saturator.naf_rounds / naf_admitted / naf_retired / naf_dropped` (root's saturation only); `--events`' `quiesce` lines (`round`, `n_facts`, `n_queue`, `n_parked`) and `fire` lines per node; `ein saturate FILE` prints `saturate: … ms (355 firings)` for root alone. | exists in another form, root-only in the summary |
| layers, `layers_explored` | summary `stats` | `work.layers`: route 1's or the state route's layers; `null` under route 2 and on every run recorded here but the dead-root describe (`zebra2-bad`: 1). `--events`' `layer` line. | exists, often `null` |
| no-goods, `nogoods_emitted` | summary `stats` | `region.nogoods` = N, the minimal dead sets. **`null` by default on a describe that branches** ("not computed by the tree and the classes … `--describe-route states` computes N"), and `null` under every mode. | exists in another form, not on the default route |
| facts | not named | `work.facts`: facts derived beyond root, summed over nodes, "what `--region-budget` is spent against"; `root.facts`, `root.facts_by_relation`. | exists (new since the plan) |
| memory | "peak RSS … the way `e2e_baseline.py` took it" | the engine reports none. Sampled here by `os.wait4`: 22.0 MB (`zebra2 --single`), 22.3 MB (describe), 21.9 MB (`zebra2-minus-15 --maximal`). Instruments in the tree: `utils/arith_cost.py:92-96`; `ein-corpus`'s pool (`VmHWM`). | does not exist in the engine; two instruments on the process side |
| matcher work (not in H4; the plan's notes keep "the rest in the record") | — | `ein_core::counters` (`unify_slot`, `candidates`, `scan_*` …) behind the `counters` cargo feature, **compiled out of the shipped binary**; its one test, `arith_batch`, is run by hand, in no tier (`plans/open_questions.md` Q-M1f.150, overruled to (b)). | exists, not in the binary an experiment runs |
| cost per correctly solved task | harness | harness | planned |

### B.3 What the baselines read (S2.5.2)

- **"the kernel's goal bindings are the answer, rendered through the program's own `:goal-text` template"**
  (S2.5.2:107-109). The bindings are in the summary: `queries[].classes[].rows[]` (a map from each goal
  variable to its value) and `queries[].classes[].facts`. The **rendered sentence is not**: on `zebra2` the
  line *“The Norwegian drinks water in House-1; the Japanese owns zebra in House-5”* is in stdout and in
  `--dump-answer DIR/answer.txt`, and in neither `ein-summary/2` nor the event stream
  (`grep -c 'drinks water in'` → summary 0, stdout 1, events 0). Exists in another form.
- **"a `multiple_models` or `inconsistent` verdict *is* the answer"** (S2.5.2:109-110; S2.5.3:40-41). These are
  S2.1.2's planned words. The engine's are `Solution`, `Ambiguity`, `Contradiction`, `Open` under `--single`,
  and it has **two kinds of empty R** (Q-M1d.6): `Contradiction` where no node is alive, `Open` where nodes
  are alive and every one owes. The benchmark's three classes (unique / ambiguous / unsat) meet four words.
- **The model count** ("`k`", G5's *"4 models remain"*, S2.3.2's `model_count`). `--single` decides
  `Ambiguity` at the second state and stops (`zebra2-minus-15`: `stopped = {why: "asked"}`, 2 states listed,
  23.8 ms). The 32 are: `--maximal`, "32 states — all of them", `exhausted yes`, 69 ms; or a bare describe /
  `--count`, which report `states (k_s) 32 (every state of R …)` **and `exhausted no`** after 10.6 s, because
  the default budget stops the run while it counts commitments (`k_c` a lower bound of 138 621 243). So
  `exhausted = false` on this instance does not mean the model count is unknown.
- **The unsat core** (G3; S2.5.2's B5 at F8). `region.core` and `region.refuted_by` are filled on a
  **description** of a dead root (`zebra2-bad`: core = three facts, `refuted_by = ["(given)", "co-located-negative"]`,
  the `source` row quoting `:source "injected contradiction"`), and are `null` under `--single`, `--count` and
  every mode ("a question that prints no core writes a dead root's as null", Q-M1f.29). A core "is an
  explanation": it "may vary with the rules' written order and with the agenda's" (Q-M1f.6), and
  `:record-alternative-justifications false` may change it (Q-M1e2.11). Making it canonical is followup F22, open.
- **The gold verdict as `:expect`** (M2 README:301-302). `ein test FILE --json-report` (`ein-test-report/6`)
  gives per claim `outcome`, `expect`, `answer`, `route`, `fell_back_from`, `diff`; `ein test --diff full`
  classes each model *match / near / missing / extra*. Not named by P2.5; it is the existing machine surface
  for "the answer equals the gold answer" on B2–B5.

### B.4 "Certified" (P2.5 README:93-96; S2.5.3:59-64), field by field

| question asked | instance | `verdict.word` | `region.k_s` | `region.exhausted` | `region.stopped` |
|---|---|---|---|---|---|
| `--single` | `zebra2` | `Solution` | `null` | `true` | `null` |
| `--single` | `zebra2-minus-15` | `Ambiguity` | `null` | `false` | `{why: asked}` |
| `--single` | `ein-bugs/zebra2-bad` | `Contradiction` | `null` | `true` | `null` |
| `--single --region-budget 100` | `zebra2` | `null` | `null` | `false` | `{why: budget, spent: 64, projected: 42}` |
| describe | `zebra2` | — (`verdict: null`) | 1 | `true` | `null` |
| describe | `zebra2-minus-15` | — | 32 | `false` | `{why: budget, …}` |
| `--maximal` | `zebra2-minus-15` | — | `null` (32 listed) | `true` | `null` |

Two more things the predicate meets today that the plan's text does not name:

- **A flagged program.** A program the static check calls non-monotone is answered under
  `WARNING … THIS PROGRAM IS NOT MONOTONE: NOTHING IS CLAIMED OF THIS ANSWER` (stdout's first row, stderr's
  first line, `monotonicity.warning`, `monotonicity.monotone = false`). `ein solve examples/lattice/04_obligation_trigger_hole.ein --single`
  still prints a word (`Ambiguity`), exit 0. A model-written program can be flagged (any rule whose `absent`
  reads a relation a commitment can fill). See § C.4.
- **Termination.** "Termination is assumed, not given … Everything stated about an answer is about programs
  whose saturation reaches a fixpoint" (`docs/ein/reasoning/02-kb-evolution/02-saturation.md:83-85`).
  `--region-budget` counts facts *beyond root*. Unsure, not tested (no corpus program diverges, and I ran only
  corpus programs): whether `-T` interrupts a single saturation that never quiesces — the help says the
  deadline is "checked before each step of work", and `ein-infer/src/explore.rs:1245-1248` checks it where a
  step of new fixpoints is admitted. F26's item 3 ("the engine's word for *did not stop*") is open.

### B.5 Stage by stage

| stage / task | needs from the project | state today |
|---|---|---|
| **S2.5.1** T2.5.1.1 layout, `einaf-record/1` | the kernel-identifying fields; a home for `experiments/` | fields: § B.1 (commit not in the binary; config is 2 flags; `max_enterings` gone). Home: `experiments/`, `docs/experiments/`, `docs/history/` do not exist; § C.1 |
| T2.5.1.1 `machine.txt` | `bench_env.sh` | exists; CPU-side fingerprint with power state; to stderr; pins to one core |
| T2.5.1.2 `einaf run` | index line "id · date · …" | planned; the date and the level labels meet the release-voice check if the file is tracked outside `plans/`, `utils/` (§ C.1) |
| T2.5.1.3 `evaluate`, `report` | nothing of the kernel | planned |
| T2.5.1.4 Q-M2.1 | `ein.commit`, manifest SHA, config | § D.1 |
| T2.5.1.5 size and git | sizes of summaries and event streams; `.gitignore` | summaries 4 KB–1.3 MB; events 1.8–37 MB per solve on `zebra2`; `.gitignore` has no `experiments/_blobs/` rule (it ignores `build/`, `dist/`, `ein.rs/target*`, `corpus/out/`, review outputs) |
| **S2.5.2** B2–B5 answer without extraction | goal bindings; the verdict | bindings in the summary; rendered `:goal-text` in text only; four words, not three (§ B.3) |
| B3–B5 "same budgets" | a budget the kernel honours with an answer | `--region-budget` (answer, partial, exit 0) and `-T` (no answer, exit 2) are different kinds of stop |
| B0/B1, extractors, budget match | nothing of the kernel | planned |
| "another symbolic backend" → P2.8 → M10 | M10's encodings | M10 unstarted; `nl2smt/smt/` holds three hand-written `.smt` files; `z3` and `cvc5` are installed here |
| **S2.5.3** H1 | verdict and answer per instance | § B.3, § B.4 |
| H2 parse / load / acceptance | a machine-readable stage of failure | parse and load errors are stderr lines and exit 1 (2 under `ein test`), no summary (§ B.1). `EinAf`'s "type correctness" has no counterpart: "Ein has no types" (Q-M1f.372), and a form of a declared relation at another count "loads, in silence" (Q-M1f.447). A relation, rule or macro without `:why` is refused at load unless `--no-require-why` |
| H2 constraint P/R "over forms in canonical text" | a canonical text of a form | `ein_ir::dump_canonical` / `dump_compact` are public functions of the `ein-ir` crate (`ein-ir/src/dump.rs:170`, re-exported `lib.rs:34`); **no CLI command prints a program in canonical text** (`ein show` has one subcommand, `hyps`; `ein saturate --dump` prints the saturated KB by origin). Reachable from Rust, not from the binary |
| H3 | the transcript | harness's (S2.4.1) |
| H4 | counters, time, memory | § B.2 |
| T2.5.3.4 `certified` | `outcome = unique ∧ search_complete` | § B.4 |
| **S2.5.4** | all of the above; `val`; the loop | nothing of it exists; `val` is certified by a retired flag in S2.3.4 (§ A.5) |
| T2.5.4.3 "machine state captured per record by `bench_env.sh`" | as T2.5.1.1 | exists |
| acceptance "the test that checks this runs in the gate" | a gate step | § C.5 |
| **S2.6.1–S2.6.3** (paragraphs) | F0…F8 renderers, the transcript | harness's; G3 reads the core (§ B.3: only on a description; an explanation, not canonical); G5 reads the model count and two models (§ B.3); G6's "complete derivation trace" exists as `--trace FILE.md` and as `--events` |
| **S2.6.4** T2.6.4.1 register and build link-grammar | a parser | **exists in another form**: `/usr/bin/link-parser` 5.13.0 is installed; `nl2smt/nl2smt.py:38, 151-155` (`linkages(lines)`) runs `link-parser en -postscript=1 -graphics=0 -echo=1` and returns the linkages as printed, the next linkage "for every empty line after it". The `linkgrammar` Python binding the task builds is not installed |
| T2.6.4.2 runner + serialiser; tests "a Zebra sentence parses … several linkages" | — | runner: partly shipped as above, with 19 tests (`nl2smt/test_pipeline.py`; `nl2smt/README.md:519-523` records all 19 passing on 2026-11-19; I did not run them, since Racket would write into the tree). The serialiser to a `(link-grammar …)` fragment and the augmented prompt do not exist. `nl2smt/` is "wired into nothing … external to both" the project and the process |
| acceptance "the serialiser's `(link-grammar …)` fragment round-trips through the parser" | which parser | unsure which is meant. If Ein's: the fragment as written (`:sentence "…"`, `(det "the" Norwegian)`) is refused — "a string is natural-language text, written only after `:why`, `:source` or `:goal-text`" (`07-diagnostics.md:37, :54-55`; `AGENTS.md:485-486`) |
| T2.6.4.3 decomposition | S2.2.4's passes | planned |
| T2.6.4.4 oracle conditions | `gold.ein` split into theory / ontology / instance / query | planned (S2.3.x). Today a program's "theory part" is not only its `import` lines and activators: a hypothesis rule is an ordinary rule that concludes `(open F)` (there is no `hrule` form), `(solve …)` lists configure the blind enumerator, and the contradiction is an import (`std.contradiction`) |
| T2.6.4.5 library conditions | how a program gets the library | § B.6 |
| T2.6.4.5 "distinct rule schemas after canonicalisation" | a canonicaliser of a rule's shape | **exists in another form**: `utils/rule_census.py` computes a rule's shape at ten levels (`alpha`, `reli`, `coni`, `alphau`, `relren`, `conren`, `i`, `ii`, `iii`, `iiib`) over tracked `.ein` under `stdlib/`, `examples/`, `tests/`, with its own reader. M12's open question 3: "Which reading of a shape is the one to keep. The census … chooses none." So "canonicalisation" has ten candidate definitions and no ruling |
| T2.6.4.6 Q11 | — | § D |

### B.6 What "no stdlib / minimal / full" can mean today (G9)

How a program gets the library now:

1. **`std.core` is loaded into every program**, with no import line (Q-M1f.551). Its one rule matches a held
   `(false)` and states `:ein inconsistent`. Without it a held `(false)` kills nothing: "the kernel reads
   `(false)` and `not` by name nowhere to judge a state".
2. **Every import brings its module whole, with its own imports** (`docs/decisions/imports/Q-M1f.247-every-import-brings-its-module-whole.md`, cited at `stdlib/algebra.ein:20`;
   `AGENTS.md:576-586`). `:symbols` only lets the file write the listed names short. So an allow-list of
   `import` targets is an allow-list of import *closures*: `std.algebra` → + `std.macro`, `std.contradiction`;
   `std.bijection` → + `std.macro`, `std.contradiction`, `std.algebra`; `std.slots` → + `std.macro`,
   `std.contradiction`.
3. **The contradiction is a theory a program imports** (Q-M1f.557). "The kernel has no check of its own: without
   such a rule a state that holds a fact beside its negation is alive" (`AGENTS.md:226-229`). Under "no
   import permitted" a program that derives `(p A)` and `(not (p A))` is alive unless the model writes the
   clash rules itself. This bears directly on H1's *contradiction detection* under the **none** condition.
4. **What a state owes** comes from four stdlib obligation rules (`total-owed`, `surjective-owed` in
   `std.algebra`; `slot-owed-room`, `slot-owed-fill` in `std.slots`). With none imported, H is what the
   program's own hypothesis rules and written `(open F)` facts give, else the blind combinatorial enumerator.
5. **`$EIN_STDLIB` replaces the library.** The directory must hold `MANIFEST.sha256` and `core.ein`, or every
   program is refused. Run here with an empty directory:
   `kb load error: std.core, the module every program gets, cannot be read: (import std.core) — $EIN_STDLIB names …/empty_stdlib, which has no MANIFEST.sha256 …`.
   `ein --version` then prints `stdlib     unreadable  $EIN_STDLIB …`. A subset library under `$EIN_STDLIB`
   would carry its own manifest and so its own SHA on the `stdlib` line; `utils/stdlib_manifest.py --write`
   is the tool that writes a manifest. `utils/bench_env.sh` unsets `EIN_STDLIB` before the command it runs.

So the smallest library a program can be given today is `std.core` alone; "no `import` permitted" is that,
with the manifest SHA unchanged (the plan's "named by manifest … plus the subset list", S2.6.4:71-73, then
rests on the subset list alone, since the SHA is the same in all three conditions unless a subset directory is
built). The plan's mechanism — the grammar lists permitted import targets — is unaffected by any of this; what
changed is what each condition contains.

---

## C. Conflicts with standing rules and decisions

### C.1 Where the records live — the project / process separation, the current-state rule, the release voice

The plan: `experiments/` at the repository root, committed (S2.5.1:31-36; P2.5 README:51-53, :121-123), with an
index line "id · date · what it is evidence for · … · Ein commit · status" (S2.5.1:42), pre-registration
READMEs, generated tables under `experiments/_reports/`, and the harness under `einaf/` (S2.5.2:49;
S2.5.3:74, :90, :125; S2.6.4:139).

The rules since:

- **The project and the process are two things** (`AGENTS.md:722-730`; Q-M1f.345). The project is "`README.md`,
  `ein.rs/` with its tests, `stdlib/`, `examples/`, `tests/`, `corpus/` and `docs/` but `docs/lib/`"; the
  process is "`plans/`, `utils/`, the gate and CI … and this file"; "`c/`, `nl2smt/` and `docs/lib/` are
  external to both". A new root directory is in none of the three lists.
- **`utils/check_release_voice.py` decides it mechanically**: a tracked `.md` or `.ein` is in scope unless its
  path starts with one of `plans/`, `utils/`, `.github/`, `run_tests.sh`, `AGENTS.md`, `CLAUDE.md`, `c/`,
  `nl2smt/`, `docs/lib/`, `ein.rs/vendor/` (lines 71-74, 111-118). Checked by calling the script's own
  functions (read-only):

  | path | `in_scope` |
  |---|---|
  | `experiments/README.md`, `experiments/exp-0042/README.md` | True |
  | `experiments/exp-0042/theories/t1/0.ein` | True |
  | `einaf/README.md`, `einaf/tests/fixtures/exp-fixture/VALUES.md` | True |
  | `docs/experiments/README.md` | True |
  | `experiments/exp-0042/config.json`, `einaf/conditions/B4.json` | False (not `.md`/`.ein`) |
  | `plans/m2_nl_to_ir/experiments/README.md`, `utils/einaf/README.md` | False |

  and what fails a line there, on sample lines:

  | line | findings |
  |---|---|
  | `exp-0042 · 2026-12-01 · B5 at F8 on val · ein 458bf90b · done` | `followup: F8`, `date: 2026-12-01` |
  | `the loop at F2, strategy repair (B4)` | `followup: F2` |
  | `feedback level F0` | `followup: F0` |
  | `handed to S2.6.1, read against P2.7` | `phase: P2.7`, `stage: S2.6.1` |
  | `G1: one-shot vs generic retry`, `H4 system metrics` | none |

  So, as the tree stands, a committed `experiments/README.md` index with dates fails the gate's release-voice
  step, **and the research plan's feedback-level labels F0–F8 collide with the followup-id pattern
  `F\d{1,2}`** in every tracked `.md` or `.ein` comment outside the excluded prefixes. `B0–B5`, `G1–G9`,
  `H1–H4` do not collide. The check has no baseline; the only escape is `release-voice-ok: <reason>` on the line.
- **The project documents only its current state** (Q-M1f.341): "no page records a step of the work, the date
  of an event, or a measurement"; **a page of the project states no measurement** (Q-M1f.349): "A measurement
  is taken by the work on Ein and kept beside what takes it". Both bind `docs/experiments/` (EinAf's proposal)
  and any project page that would quote a table.
- **A measurement lives beside the script that takes it** (`AGENTS.md:730`; `utils/README.md` § Censuses: "kept
  beside the tool that re-takes it, with the date and the commit it was taken at").
- **Nothing in the project names the process** (Q-M1f.345). S2.5.1:26-28: "a sentence in M5 cites a record id".
  M5's article is to be under `docs/demo/` (`docs/demo/README.md`), which is the project. If the records are
  the process's, a project page may not link or name them; if they are the project's, the three rules above
  apply to them.
- **A finished plan is deleted** (`AGENTS.md` § Where the work is). Records kept under `plans/m2_nl_to_ir/`
  would go with the folder at the milestone's close; the plan wants them to outlive it ("the records stay
  where they are and are cited by id", S2.5.1:36).

What the present rules leave, each with its consequence (stated, not recommended): (i) a project location →
release voice, no-measurement and no-date rules apply; (ii) a process location (`plans/`, `utils/`) → none of
those apply, the project cannot cite the records, and under `plans/` they are deleted with the folder;
(iii) a third "external to both" location like `c/` and `nl2smt/` → needs the directory added to
`NOT_PROJECT` in `check_release_voice.py` and to `AGENTS.md`'s list, and "nothing of the project depends on
them"; (iv) untracked under `build/` (gitignored) → contradicts the plan's "committed with its records". The
plan's one-sentence decision for the root (S2.5.1:31-36) predates all four rules and argues from
`docs/history/`, which no longer exists.

### C.2 `docs/history/` as the destination of the measurement documents

S2.5.1:34-36. Forbidden as written: the directory was deleted on the user's ruling and Q-M1f.341 says "A page
that stops describing Ein is rewritten or deleted … The history of the work is the version control's".

### C.3 "No config option, lever or flag changes R" (Q-M1e2.30; `AGENTS.md:639-642`)

- No stage proposes an `ein` flag that changes R. The harness flags (`--link-grammar`, `--decompose`,
  `--mixed-commits`) are `einaf`'s; G9 varies the *program*, not a configuration.
- The record's `budgets.ein.max_enterings` (S2.5.1:67) names a retired cap, and `-m`'s retirement note is "a
  depth cap changed answers".
- "The resolved `SolverConfig`" as a field that *freezes the instrument* (Q-M2.1; T2.5.1.4) now freezes two
  flags, neither of which can change R. One, `record-alternative-justifications`, can change which **core** a
  dead node reports — which is the content of the F5 feedback level and of ablation G3.
- A budget-stopped run "prints what it established and bounds, never a different answer" and exits 0
  (Q-M1d.1, Q-M1e2.33). The plan's `certified` reads a completeness flag, which is consistent with this; the
  field names differ (§ B.4).

### C.4 Non-monotone (flagged) programs (Q-M1f.564; `AGENTS.md:613-633`)

"A program the static check flags … is answered by the cube, with no licence, and every command that answers
about a commitment says in capitals that the program is not monotone and that nothing is claimed of the
answer"; "no page, comment or test says what it answers is right, sound or complete"; "No suite validates a
flagged program's answer".

- P2.5 README:93-96 and S2.5.3:59-64 define `certified` and `P(correct | certified)` with no clause for a
  flagged program. On such a program the engine prints a `--single` word and disclaims it in the same output.
  A model-written program can be flagged; a canonical (`gold.ein`) program that is flagged would be one of
  which "nothing is claimed" as ground truth.
- Past `--region-limit` (default 16 candidates → 2^16 saturations) a flagged program's question that needs the
  cube "is answered in part".
- The summary carries what a harness would need to tell: `monotonicity.monotone`, `monotonicity.warning`,
  `monotonicity.sites`.

### C.5 "The test that checks this runs in the gate" (S2.5.4:50-51) — CI tiers

- "**The tests read only the project**: what ties a process file to the project is a `utils/` check and a gate
  step, never a crate test" (`AGENTS.md:866-872`). A byte-identity test of `einaf report` over committed
  records is not a crate test of the project unless `einaf` and the records are the project's (§ C.1).
- A gate step is a line in `run_tests.sh` **and** in `.github/workflows/per-commit.yml`, which
  `utils/check_gate_steps.py` diffs; anything the runner must install first is a provisioning step named in
  `NOT_GATE_STEPS` with why. A nightly step fits in 5 min on the runner or runs in shards
  (`plans/open_questions.md` Q-M1f.293, Q-M1f.102); a step outside both is the dev host's and is named in
  `DEV_HOST_ONLY`.
- Precedent for code beside the engine: `nl2smt/` and `c/` are "wired into nothing: no crate depends on it and
  the gate does not run it" (`AGENTS.md`); `nl2smt/README.md:569-570`: "It is not in the gate. Its tests need
  a parser, Racket and a solver where they run."
- S2.3.2:110 (another auditor's) also puts `einaf bench check` "green in CI".

### C.6 "An exact number in a page carries the command that re-takes it" (Q-M1e.4) and Q-M1f.349

Both bind "a page, a comment or a manifest note" **of the project**. The plan files are the process's and are
not bound by them. They would bind: generated tables and readings if those are project pages; M5's article
under `docs/demo/`; any `docs/ein/` page that quoted an experiment. The plan's own principle ("a number that
cannot be traced to a record directory is not in a table") is the same discipline with a record id in place
of a command; Q-M1f.349 goes further for the project and allows no measured number at all, only "a relation a
test holds".

### C.7 Time estimates, and dates as status

- Every one of the seven files opens with an **Estimate** line, and the two READMEs carry `est.` columns
  (§ F). The user's ruling that estimates are not kept (2026-11-08, in the session memory "No estimates") is
  not written anywhere in the tree that I found: `plans/README.md:337` still shows `**Estimate:** N days` in
  its stage-file template and line 355 a "rough estimate" column.
- S2.5.4:104-106 writes "the date, the record ids, the shape selected" for the milestone README's **Status**
  line and `plans/README.md`'s table. That is the process's own voice and no rule forbids it there;
  `AGENTS.md` itself is "not a log" and takes no dated paragraph.

### C.8 Measurement discipline

- Timings here are read only with the power state and the cores named (`utils/bench_env.sh`: "ON BATTERY and a
  warning"; `--require-ac`; `--cores P:8 | PT:8 | E:8`). The plan's `machine.txt` inherits this. It does not
  say that `bench_env.sh` pins the command it runs to one P-core thread by default, nor that the fingerprint
  has nothing about a GPU or the model server.
- "Tests within the default budget" (`plans/open_questions.md` Q-M1f.304, ruled (d): "no test runs past the
  ruled `--region-budget` 2^20"). A describe of an ambiguous instance of Zebra size reaches that budget
  (§ B.3): 11 222 saturations, 10.6 s, `exhausted no`.

### C.9 Small ones

- `scripts/build-link-grammar.sh` (S2.6.4:113, :133): there is no `scripts/` directory; the process's scripts
  are `utils/` ("`utils/README.md` is the catalogue — one line per script"), and `nl2smt/` is external.
- "Nothing upstream" and "no ein.py" are not touched by these files.
- "No numbers in Ein" (`AGENTS.md:440-442`): not touched by P2.5/P2.6 directly; the benchmark's arithmetic
  instances, if any, are the calculator theory's.

---

## D. Open questions and decisions the phases defer

| # | question / decision | who the plan says decides | anything since |
|---|---|---|---|
| 1 | **Q-M2.1 — when is the kernel frozen?** "frozen per experiment, by commit" | S2.5.1 T2.5.1.4, written beside the question | Its two stated reasons are gone (A #23, #24): M1c and M1d shipped, and the engine has a word for a stopped run. `AGENTS.md`: "The work stops after M1f … nothing below starts without the user's word", so no kernel milestone is running; M1g, M1h, M12, M15, M1k, M1n, M1p are each queued on the user's word. The three freezing fields changed shape: commit (not in the binary), manifest SHA (presence-checked), config (2 flags). Not answered by anything in the tree. |
| 2 | **Q-M2.2 — must the model be local?** | S2.2.2 (recommendation: local primary, hosted a replication arm); S2.5.2:127-129 leans on it | Nothing in the tree answers or changes it. `docs/lib/01-llm-constrained-generation.md` is the catalogue it would read. |
| 3 | **Q11 — link-grammar value** | S2.6.4, by experiment and the pre-registered rule (`> 0.03` / `< 0.0` / between) | M1j (done 2026-11-13) built a working `link-parser` pipeline in `nl2smt/` and says "It decides nothing for M2". Q11's entry is unchanged. The submodule question now has two written positions: S2.6.4:10-13 "registering it is this stage's first act"; `nl2smt/README.md:650-653` "S2.6.4 re-adds it in one command **if it wants** the source or the bindings — … registering it now would pre-empt the decision". |
| 4 | **Q25 — the frontend's language** | S2.1.3 | Unchanged, open. P2.5/P2.6 say "(or the name Q25 gives them)" (P2.5 README:106) and yet name Python files: `einaf/metrics/h1.py … h4.py` (S2.5.3:90), `einaf/nlp/link_grammar.py` (S2.6.4:139). Facts that bear on it today: canonical text is a crate function with no CLI (§ B.5); the rendered `:goal-text` is text-only (§ B.3); `docs/ein/user/01-install.md:144-153`: "There is no Python package … run the binary and read its output … To embed Ein in a Rust program, link the crates". |
| 5 | **Where the records live** | decided in the plan's text (S2.5.1:31-36), declining `EinAf.md`'s `docs/experiments/`; not put to the user | § C.1: four rules since bear on it; the argument's premise (`docs/history/`) is gone. |
| 6 | **Q-M2.3 — the unit of faithfulness without a gold theory** | S2.4.4; S2.5.3 marks numbers with "the judge's flag" | Not mine; unchanged as far as P2.5 reads it. |
| 7 | The budget rule's unit and tolerance (output tokens, ±10 %) | S2.5.2 T2.5.2.3, "written once" | nothing since |
| 8 | δ and the selecting inequalities of the three shapes | S2.5.4 T2.5.4.1, at pre-registration | nothing since |
| 9 | Multiple-comparison correction over nine ablations | "the stage that writes the readings" (P2.6 README:95-98) | nothing since |
| 10 | S2.6.1–S2.6.3 written to stage depth | "when S2.5.4's table exists and says which of them matter" (P2.6 README:5-7) | nothing since |
| 11 | What "canonicalisation" of a rule schema is, for C(n) | S2.6.4 T2.6.4.5 (implicitly) | M12 exists since 2026-11-12 with ten shape levels and its open question 3 ("which reading of a shape is the one to keep"); M12's open question 2 asks where M12 sits "against M2, whose S2.6.4 counts rule schemas a model writes". Both are M12's, on the user's word. |
| 12 | Which question the harness asks the kernel per iteration (`--single`, describe, `--maximal`, `--count`) | not raised in my files; S2.1.2's | The answer has four questions with different costs, different filled fields and different `exhausted` (§ B.3, § B.4). H1's detection metrics, G3 (core), G5 (count and two models) and H4 each need a different one. |
| 13 | Whether a flagged program can be `certified` | not raised | § C.4 (Q-M1f.564). |
| 14 | Which of `Contradiction` and `Open` the `unsat` class's gold verdict is | not raised in my files; S2.3.2's | Q-M1d.6. F26 item 6 ("the common-sense table as benchmark classes for M2: contradictory, under-determined by a quantity, under-determined by a reading, complete") was filed 2026-11-11 "for M2" and is open. |
| 15 | Hosted replication as "a separate table, never a seventh column" | S2.5.2 note, hanging on Q-M2.2 | nothing since |

---

## E. Dependencies

### E.1 On other M2 stages

| my file | depends on | target exists | still says what the plan says it says |
|---|---|---|---|
| S2.5.1 | S2.1.2 `ein-feedback/1` | yes | yes in name; its outcome mapping is written against the walk's summary (`summary.type = Aborted`, `k`, `-e`; lines 82-84, 110-116) |
| S2.5.1 | S2.2.2 invocation envelope | yes (T2.2.2.4) | yes |
| S2.5.1, S2.5.3 | S2.2.5 seed-set run = `exp-0001` | yes | yes ("every run writes a record directory", line 49) |
| S2.5.1 | S2.2.4 `einaf/prompts/` | yes | not checked beyond existence |
| S2.5.2 | S2.4.1, S2.4.3 (loop at F0/F2/F8) | yes | yes (F0 "try again", F8 "the union, under a size cap") |
| S2.5.2, S2.5.4 | S2.3.4 (`dev`, `val`, `test`) | yes | yes; **its certification is `ein solve -e`, a retired flag** (line 52) |
| S2.5.3 | S2.3.2 "the constraint unit" | yes | **circular**: S2.5.3:104-105 "the unit's definition imported from S2.3.2 rather than restated"; S2.3.2:77 "(S2.5.3 defines the unit)". The definition text is S2.3.2:82-90, and it reads "the `ein-ir` dump, whitespace-normalised" (§ B.5: a crate function) |
| S2.5.3 | S2.4.4 faithfulness events; Q-M2.3 | yes | yes |
| S2.5.4 | P2.4 closed; P2.10 "S2.10.1" | P2.10 is a README only; S2.10.1 is one bullet (line 88) | yes |
| S2.6.4 | S2.2.1 catalogue, S2.2.3 theory grammar, S2.2.4 passes, S2.3.2 families | yes | S2.2.1:96 still writes "a `rule` / `hrule` form" (no `hrule` form exists); S2.2.3:103 lists "`import` of `std.*`" |
| S2.6.4 | `einaf bench check` (S2.3.1 T2.3.1.5) | yes | yes |
| P2.6 | P2.5's table; P2.7 draws its failure sample from P2.6's runs | yes | yes |

### E.2 On other plans and followups

| leaned on | from | exists | says what the plan says |
|---|---|---|---|
| F13 `ideas.md` (four oracle ablations, "18 %") | P2.5 README:148; P2.6 README:70-72; S2.6.4:55, :67 | yes | yes (lines 519-526) |
| F8 `ideas.md` (C(n)) | P2.6 README:77; S2.6.4:81 | yes | yes (line 324) |
| F9 | P2.6 README:113 | **no — closed, deleted** | in git only |
| M5 README (§ 6, § 9.3) | P2.5 README:152; S2.5.1:27; S2.5.3:121 | yes, "placeholder README only" | the section numbers are `EinAf.md` Stage P's; M5 README:13 adopts the twelve-section outline |
| M10 (via P2.8) | P2.5 README:76; S2.5.2:43 | yes, unstarted, 5 stage files | yes; `nl2smt/smt/` holds the three encodings |
| M12 | not cited by my files; M12 cites S2.6.4 twice | yes | § D #11 |
| M1j | S2.6.4:41-46 | plans removed; `nl2smt/` is its result | yes |
| M1a S1a.10.5 (submodule deinitialised) | S2.6.4:11-12 | plans removed; `nl2smt/README.md:645-646` repeats it | yes |
| M1d (the `unknown` word), M1c | S2.5.1:79-81; Q-M2.1 | shipped; decisions `Q-M1d.6`, `Q-M1d.1` | the premise "until it lands" is over |
| F22 (canonical core), F26 (items 3, 6) | not cited | yes, open | bear on G3 and on H1's classes (§ B.3, § D #14) |
| `utils/bench_env.sh`, `utils/censuses/corpus-cost.md` | P2.5 README, S2.5.1, S2.5.4 | yes | yes; `bench_env.sh` gained the power line since |
| `baseline.md`, `docs/history/`, `e2e_baseline.py` | S2.5.1, S2.5.3 | **no** | — |
| `ein-cli/tests/summary_properties.rs` | S2.5.3:24 | yes | content changed (A #11) |
| `docs/ein/ein.rs/05-events.md` | S2.5.1:106 | yes | yes |
| `nl2smt/README.md`, `nl2smt/prototype/` | S2.6.4:41 | yes | yes; and `nl2smt/nl2smt.py` is new beside them |
| root `README.md` § EinAf (M2 README) | context | **no such section** | — |
| `docs/lib/` catalogue of baselines | not cited by my files | `docs/lib/12-llm-and-reasoning-benchmarks.md` lists BBH, ProofWriter, FOLIO, LogiQA and, in § 8, generic architectures (Toolformer, ReAct, DSPy, PoT, PAL, GoT, ToT, "SAT-augmented LLM"). **No entry for Logic-LM, LINC, SatLM, self-refinement / Reflexion, or link grammar** (`grep -rniE 'logic.?lm\|\blinc\b\|satlm\|self.?refin\|reflexion\|link.?gram' docs/lib/*.md` → nothing). `EinAf.md` Stage D's "where possible add another symbolic backend or structured reasoning approach" has no catalogue behind it | — |

---

## F. Estimates, dates used as status, and counts in the text

**Estimates.** P2.5 README:3 "2.5 weeks — 4 stages, 12 days"; :104-109 `est.` column, 3 d × 4. S2.5.1:4,
S2.5.2:4, S2.5.3:4 "3 days". S2.5.4:4 "3 days — one of them the run, two of them the reading"; :114-116 "the
first run in the milestone whose wall is days rather than minutes". P2.6 README:3 "3 weeks — 4 stages".
S2.6.4:4 "5 days". (M2 README:3-5, context: "~6 months … Level B at ~8 weeks, Level C at ~19".)

**Dates used as status or provenance.** S2.6.4:43 "A note from M1j, 2026-11-13". S2.5.4:104-106 asks for "the
date" in the Status paragraph. S2.5.1:42 puts a date in every index line of `experiments/README.md`. Context:
`open_questions.md` Q-M2.1/Q-M2.2 "Arrived 2026-08-23"; M2 README "reshaped 2026-08-23", "written 2026-08-23".
No other date in the seven files.

**Counts and thresholds stated as facts about the project (each checked in § A):** "128 entries" (933);
"29 ms" (question-dependent); "the 17 fields" (2 flags of 17); "thirteen arithmetic identities" (no such
count); "all seven modules" (28); "the six stdlib families" / "the two families with no stdlib theory"
(unsure / consistent by module names).

**Counts and thresholds that are the plan's own design numbers:** six conditions B0–B5; nine levels F0–F8;
nine ablations G1–G9; four layers H1–H4; eight families, "≥ 5 of 8"; three seeds; three verdict classes; a
hand-read sample of 100; 30 hand-written answers per format; a twelve-task fixture over three families and
three classes; budget tolerance ±10 %; `iterations: 6`, `output_tokens: 4000`, `max_time: 30`,
`max_tokens: 2048`; 1 MiB gzip / 10 MiB not-committed thresholds; link-grammar rule `> 0.03` / `< 0.0`;
linkages `k = 2..3`; "the 18 %" (the user's phrase, from "82 % BBH"); "five puzzles"; "thousands of instances".
