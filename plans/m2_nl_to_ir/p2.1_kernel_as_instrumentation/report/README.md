# P2.1 — the phase report, for the user's review

**What this is.** P2.1 ran its three stages on the user's word of 2026-11-25 (*"start implementation of M2,
everything in p2.1, then stop and let me review; each stage in subagents, subagents commit their work"*), and
stops here. Nothing after it is started. This page shows what the phase built, with commands to run and their
real outputs, the renderings, the defaults taken for the user to correct, the findings filed, and the questions
left for the user.

Everything under [`programs/`](programs/) is written by [`make_report.sh`](make_report.sh) from the binaries, on
`ddb1006e` (Ein as of `e74139cf`): re-run it to see the same on any later checkout.

```sh
./build.sh --engine --einaf                                            # ein.rs/target/release/ein, einaf/target/release/einaf
plans/m2_nl_to_ir/p2.1_kernel_as_instrumentation/report/make_report.sh  # about 40 s
```

**Gate:** `./run_tests.sh` green on `ddb1006e` (every static check, 1 434 tests passed and none failed, Ein's and
EinAf's, the process's checks, the bench smoke). The nightly ran after it because the saturator's loop changed: see
[§ 9](#9-verification).

---

## 1. What the phase built

| stage | what it ended with | commits |
|---|---|---|
| [S2.1.1](../s2.1.1_tree_and_outcomes.md) EinAf's tree and the outcomes | `einaf/`, one Cargo package outside the engine's workspace that links `ein-core`, `ein-ir`, `ein-infer`, `ein-render`; a gate step and a CI step; `docs/einaf/`; **the seven outcomes** as a function over the library's values, under every question and reading, held over the 917 corpus programs | `0effa7b0`, record `556c368e` |
| [S2.1.2](../s2.1.2_diagnostics_surface.md) the kernel's diagnostics as values, and a bound inside a saturation | **in Ein**: every saturation a question computes is bounded, by `-T` (now read inside a saturation) and by a new cap, `--max-steps` (decision [Q-M2.16](../../../../docs/decisions/answer/Q-M2.16-every-saturation-a-question-about-r-computes.md)); every refusal a value with one of 38 kinds, a place and its unchanged line; every line a run prints a value of the JSON summary (`diagnostics`), a summary written for a refused or aborted run too; `ein test --json-report`'s error rows carry why; a core's `:source` texts beside it (`region.core_sources`) (decision [Q-M2.9](../../../../docs/decisions/diagnostics/Q-M2.9-a-refusal-is-a-value-with-a-kind-a-place-and-its.md)). **In EinAf**: the outcomes read off those values | bound `83b82833` + `fd2fcd48`, `11d927f2`; EinAf's arm `d61176a3`; clippy for rustc 1.100 `acd04b13`; values `62a8ee45` + `95f5dc49`; surfaces `e74139cf`; EinAf `d13dc4d9`; record `913f8f8c` |
| [S2.1.3](../s2.1.3_feedback_object.md) the feedback object | `einaf::kernel::call`, **the one door** to the engine, every bound set on every call, a panic caught; **`einaf-feedback/1`**, one JSON object for one program under one question; `einaf feedback`; agreement with the binary's summary on every compared corpus run | `0ab1aef0`, record `ddb1006e` |

**What a user of `ein` sees change** (nothing of the language, no answer, no exit code, no message):

- `ein solve` and `ein test` take `--max-steps N` (firings one saturation may take, default 2²⁰); past it the
  run is *not decided*, exit 0, `region.stopped = {why: steps, steps, saturation: root|node}`.
- `-T` now stops a run inside any saturation, root's included (exit 2, as before).
- `--json-summary` ends with `diagnostics`, every line the run printed on stderr as `{kind, severity, message,
  place}`, and is written for a refused program and an aborted run too, its answer keys `null`.
- `ein test --json-report`: every row has `diagnostics`, filled on `error` rows; a claim of a file whose run failed
  is `error` (it said `no-expect`).
- `region.core_sources` beside `region.core`.
- [`07-diagnostics.md`](../../../../docs/ein/user/04-reference/07-diagnostics.md) names a kind under every section
  (`**Kind:**`), with four sections added (§ 2.32 to § 2.35) for twelve refusals no section stated;
  [`08-read-out.md`](../../../../docs/ein/user/04-reference/08-read-out.md) § 10 a kind for each line a run writes.

**EinAf's pages**: [README](../../../../docs/einaf/README.md), [01 the outcomes](../../../../docs/einaf/01-outcomes.md),
[02 the kernel's invariants](../../../../docs/einaf/02-kernel-invariants.md),
[03 the feedback object](../../../../docs/einaf/03-feedback.md), key by key.

## 2. Commands to try

All from the repository's root, after `./build.sh --engine --einaf`. `E=ein.rs/target/release/ein`,
`A=einaf/target/release/einaf` (in zsh write the paths out or use `alias`).

```sh
# The seven outcomes, one program each (§ 3 has the outputs)
$A outcome examples/zebra2.ein                              # one, under each reading
$A outcome examples/lattice/03_state_hash_collision.ein     # several over R, one over each line
$A outcome examples/lattice/03_state_hash_collision.ein --maximal --single
$A outcome tests/region/04_owing.ein                        # incomplete
$A outcome examples/ein-bugs/zebra2-bad.ein                 # inconsistent
$A outcome examples/lattice/04_obligation_trigger_hole.ein  # flagged
$A outcome examples/broken/load/macro_arity_mismatch.ein    # invalid (macro-call)
$A outcome ein.rs/crates/ein-cli/tests/bounded/grow.ein --max-steps 10000   # not decided
$A outcome examples/zebra2.ein --no-search                  # not decided: R was not searched
$A outcome examples/zebra2.ein --region-budget 1            # not decided: the budget

# The feedback object
$A feedback examples/zebra2.ein --single | python3 -m json.tool | less
$A feedback examples/ein-bugs/zebra2-bad.ein | python3 -c 'import json,sys; print(json.load(sys.stdin)["core"])'
$A feedback examples/broken/unclosed_paren.ein --single

# Diagnostics as values, through the binary
$E solve examples/broken/load/derivation_cycle.ein --json-summary /dev/stdout
$E test examples/broken/load/macro_arity_mismatch.ein --json-report /dev/stdout
$E solve examples/ein-bugs/zebra2-bad.ein --json-summary s.json && python3 -c 'import json; r=json.load(open("s.json"))["region"]; print(r["core"], r["core_sources"])'

# The bounds, on a program whose saturation has no fixpoint
cat ein.rs/crates/ein-cli/tests/bounded/grow.ein
$E solve ein.rs/crates/ein-cli/tests/bounded/grow.ein                       # stops at the default cap, ~1 s, ~1 GB
$E solve ein.rs/crates/ein-cli/tests/bounded/grow.ein --max-steps 10000     # not decided, exit 0
$E solve ein.rs/crates/ein-cli/tests/bounded/grow.ein -T 1 --max-steps 1000000000 --json-summary /dev/stdout   # the clock, inside root's saturation: exit 2
$E solve examples/zebra2.ein --max-steps 100                                 # a corpus program: root's saturation needs more than 100

# The probes of S2.1.3 (programs written to exhaust the kernel)
$A outcome plans/m2_nl_to_ir/analysis/probes/p2.1/wide_join.ein --single -T 1
python3 plans/m2_nl_to_ir/analysis/probes/p2.1/deep_term.py 5000 > /tmp/deep.ein && $A outcome /tmp/deep.ein --single

# EinAf's tests, with the corpus's count of each outcome
cargo test --manifest-path einaf/Cargo.toml --profile gate -- --nocapture
```

## 3. The seven outcomes, one program each

`einaf outcome` under each of `ein solve`'s eight questions; a question that reads several parts of R gives one
outcome a part (*all*, *min* the minimal line, *max* the maximal line). Every cell is a file
`programs/<name>/einaf-outcome-<question>.out`.

| program | describe (all / min / max) | `--single` | `--maximal --single` | `--minimal --single` | `--maximal` | `--minimal` | `--count` | `--no-search` |
|---|---|---|---|---|---|---|---|---|
| [`zebra2.ein`](programs/one/) | one / one / one | one | one | one | one | one | one | not decided |
| [`03_state_hash_collision.ein`](programs/several/) | **several** / one / one | several | one | one | one | one | several | not decided |
| [`04_owing.ein`](programs/incomplete/) | incomplete ×3 | incomplete | incomplete | incomplete | incomplete | incomplete | incomplete | not decided |
| [`zebra2-bad.ein`](programs/inconsistent/) | inconsistent ×3 | inconsistent | inconsistent | inconsistent | inconsistent | inconsistent | inconsistent | **inconsistent** (root dead) |
| [`04_obligation_trigger_hole.ein`](programs/flagged/) | flagged ×3 | flagged | flagged | flagged | flagged | flagged | flagged | flagged |
| [`unclosed_paren.ein`](programs/invalid-parse/) and four more refused | invalid ×3 | invalid | invalid | invalid | invalid | invalid | invalid | invalid |
| [`grow.ein`](programs/not-decided-grow/) at `--max-steps 10000` | not decided ×3 | not decided | not decided | not decided | not decided | not decided | not decided | not decided |

<!-- output -->
```text
$ einaf outcome examples/lattice/03_state_hash_collision.ein
einaf outcome · examples/lattice/03_state_hash_collision.ein
  several       over all of R           5 states
  one           over the minimal line   one state
  one           over the maximal line   one state

$ einaf outcome ein.rs/crates/ein-cli/tests/bounded/grow.ein --max-steps 10000
einaf outcome · ein.rs/crates/ein-cli/tests/bounded/grow.ein
  not decided   over all of R           stopped at --max-steps 10000: root's saturation had no fixpoint within it
  not decided   over the minimal line   stopped at --max-steps 10000: root's saturation had no fixpoint within it
  not decided   over the maximal line   stopped at --max-steps 10000: root's saturation had no fixpoint within it

$ einaf outcome examples/lattice/04_obligation_trigger_hole.ein --single
  flagged       over all of R           not monotone: 4 sites of the static check
```

**Over the corpus** (S2.1.3's sweep, 917 programs, describe over all of R): invalid 187, flagged 57, not decided
0, inconsistent 157, incomplete 24, several 153, one 339; 949 runs compared with the binary's `--json-summary`,
all agreeing (outcome, readings, states, classes, claims, debts, frontier, core, work, diagnostics one for one).

**Renderings** (`ein render`, drawn by Graphviz 16.1.1 as the script runs):

| program | the program as written | root's KB (`--fold`) | the lattice the describe visits | H's structure (`render h-space --dump`) | the oracle's region against the answer |
|---|---|---|---|---|---|
| one, `zebra2` | [KB part](programs/one/program.dot.svg), [a rule](programs/one/program.dot.2.svg) (17 graphs) | [kb](programs/one/kb.dot.svg) | [lattice](programs/one/lattice.dot.svg) | [exclusion](programs/one/h-space/exclusion.dot.svg), [cover](programs/one/h-space/cover.dot.svg), [coupling](programs/one/h-space/coupling.dot.svg) | (cube too large) |
| several | [program](programs/several/program.dot.svg) | [kb](programs/several/kb.dot.svg) | [lattice](programs/several/lattice.dot.svg) | [coupling](programs/several/h-space/coupling.dot.svg) | [region](programs/several/region.dot.svg) |
| incomplete | [program](programs/incomplete/program.dot.svg) | [kb](programs/incomplete/kb.dot.svg) | [lattice](programs/incomplete/lattice.dot.svg) | [cover](programs/incomplete/h-space/cover.dot.svg) | [region](programs/incomplete/region.dot.svg) |
| inconsistent, `zebra2-bad` | [program](programs/inconsistent/program.dot.svg) | [kb](programs/inconsistent/kb.dot.svg) | [lattice](programs/inconsistent/lattice.dot.svg) | [exclusion](programs/inconsistent/h-space/exclusion.dot.svg) | — |
| flagged | [program](programs/flagged/program.dot.svg) | [kb](programs/flagged/kb.dot.svg) | [lattice](programs/flagged/lattice.dot.svg) | [coupling](programs/flagged/h-space/coupling.dot.svg) | — |

Each program's folder also holds the read-outs (`solve.out`, `single.out`, `no-search.out`, `count.out`), the
summaries (`*.summary.json`), the answer dump (`answer/`), H as Ein facts (`show-hyps.out`), the h-space read-out
and `h-space.json`, every `.dot`, `ein test`'s report where the program states a claim or is refused, and the
feedback objects (`einaf-feedback-single.out`, `einaf-feedback-describe.out`). Every command's stderr is `*.err`
and its exit code `*.exit`.

## 4. Diagnostics as values

A refusal printed what it prints, exits as it exits, and is now also a value. The summary of a refused run:

<!-- output -->
```text
$ ein solve examples/broken/load/macro_arity_mismatch.ein --json-summary s.json     # exit 1
kb load error: (rule x): macro m/2 invoked with 1 args at examples/broken/load/macro_arity_mismatch.ein:6:20
$ cat s.json
{"schema": "ein-summary/2", "source": "examples/broken/load/macro_arity_mismatch.ein",
 "asked": {"which": "all", "ask": "describe", "n": null},
 "region": null, "verdict": null, "queries": null, "claim": null, "work": null,
 "monotonicity": null, "leftover": null, "root": null, "config": null,
 "diagnostics": [{"kind": "macro-call", "severity": "error",
                  "message": "(rule x): macro m/2 invoked with 1 args at examples/broken/load/macro_arity_mismatch.ein:6:20",
                  "place": {"file": "examples/broken/load/macro_arity_mismatch.ein", "line": 6, "col": 20}}]}
```

A parse error's finding carries the parser's whole block, its repairs among it
([`programs/invalid-parse/solve.summary.json`](programs/invalid-parse/solve.summary.json)); a mechanisms' conflict
is kind `mechanisms` ([`programs/invalid-mechanisms/`](programs/invalid-mechanisms/)); a duplicate rule name
`declaration`, with `place: null`, since a top-level form has no position (Q-M1a.6; E-73). `ein test
--json-report` of a refused file: [`programs/invalid-macro-call/test.report.json`](programs/invalid-macro-call/test.report.json).

**The 38 kinds.** 32 are refusals, one a section of `07-diagnostics.md`: `parse`, `not-ascii`, `nesting-limit`,
`unreadable`, `module-unreadable`, `claim`, `obligation`, `open-shape`, `malformed-primitive`, `artefact`,
`store-limit`, `std-core`, `rule-shape`, `missing-why`, `template`, `unbound-conclusion`, `eq-arity`, `object`,
`no-plan`, `solve-list`, `goal`, `negated-name`, `macro-fact`, `import-list`, `mechanisms`, `full-name`,
`saved-file`, `engine-word`, `declaration`, `macro-call`, `import`, `derivation-cycle`. Six are lines a run
writes beside an answer, named on `08-read-out.md` § 10: `not-monotone` (warning), `partial`,
`claim-not-checked`, `claim-failed`, `deadline`, `stale-container`. `ein-core/tests/diagnostic_kinds.rs` holds
the pages and `ein_core::Kind` to one list.

A core's sentences (`zebra2-bad`, an injected contradiction):

<!-- output -->
```text
region.core          ["(color-loc Green House-1)", "(is-a House-1 House)", "(relation color-loc Color House)"]
region.core_sources  ["injected contradiction", null, null]
```

## 5. The bounds

`grow.ein` is one rule that nests its conclusion one deeper than its premise: its saturation has no fixpoint.
Before the phase, `ein solve` on it ran until killed, under `-T`, under `--region-budget`, at gigabytes.

| command | exit | what it says | the summary |
|---|---|---|---|
| `ein solve grow.ein` | 0 | `note: … partial — root's saturation needed more than --max-steps 1048576 firings, and was stopped before its fixpoint. Established: nothing of R, neither root's standing nor H` | `region.stopped {why: steps, steps: 1048576, saturation: root}`; `diagnostics [{kind: partial, severity: note, …}]` |
| `ein solve grow.ein --max-steps 10000` | 0 | the same at 10 000 | the same at 10 000 |
| `ein solve grow.ein -T 1 --max-steps 1000000000` | 2 | `** aborted: --max-time 1 s reached before the question was decided **` | written now: every answer key `null`, `diagnostics [{kind: deadline, severity: error, …}]` |
| `ein solve grow.ein -T 0` | 2 | the same at 0 s | the same |
| `einaf outcome grow.ein --max-steps 10000` | 0 | *not decided*, root's saturation, under every question | — |

The default cap stops `grow.ein` before one second, at about 1 GB (E-81): `-T 1` alone does not reach the clock.

**S2.1.3's probes**, programs written to exhaust the kernel, under `einaf outcome --single` (release, AC; the
programs and the generator are under [`analysis/probes/p2.1/`](../../analysis/probes/p2.1/)):

| probe | default bounds | `--max-steps 1000` | `-T 1` |
|---|---|---|---|
| `one_rule.ein` (root has no fixpoint) | not decided at the cap, 1.26 s, 727 MB | 0.01 s | stopped at the cap |
| `wide_join.ein` (a five-cycle join over K(40,40) that never fires) | one, 20.1 s | 4.5 s | not decided after 4.7 s: the clock is read only where something fires (E-98) |
| `wide_h.ein` (the blind enumerator, 159 600 candidates) | several, 10.6 s | the cap not reached | not decided after 3.5 s |
| `deep_term.py 5000` (a term 5 000 deep) | one, 3.8 s | 0.08 s | still *one* after 3.8 s: the read-out is outside the clock (E-99) |
| `deep_term.py 20000` | one, after 256 s, 77 MB, no stack overflow | | |

Every probe returns, so the worker process the work order kept in reserve is not built; memory is bounded only
through the cap (E-100).

## 6. The feedback object

One object for one program under one question; every key present, `null` where it does not apply. Keys, in
order ([`03-feedback.md`](../../../../docs/einaf/03-feedback.md) states each): `schema`, `program` (name, SHA-256
of the canonical dump), `kernel` (version, protocol, stdlib digest, commit), `asked` (the question and every
bound), `outcome` (one per reading), `readings` (the verdict word, `k_s`, `k_c`, the lines' counts, exhausted,
stopped), `diagnostics`, `flagged` (the static check's sites), `queries` (classes, rows, each row's
`:goal-text` line, agreement, the `:expect`'s outcome), `claim`, `states` (facts over the candidates' relations,
a digest), `owed` (root's and the frontier's, in the kernel's sentences), `core` (facts, `:source`s, status,
the rules that refuted), `work` (saturations, facts, layers, microseconds), `dependencies` and `provenance`
(left open, the ladder's), `fault` (a panic or an engine error, never an outcome).

<!-- output -->
```text
$ einaf feedback examples/zebra2.ein --single          (trimmed: programs/one/einaf-feedback-single.out)
schema       "einaf-feedback/1"
program      {"name": "examples/zebra2.ein", "digest": "sha256:4e1c13bf…", "of": "canonical-dump"}
kernel       {"version": "0.1.0", "protocol": "ein-events/1", "stdlib": "sha256:292894be…", "commit": null}
asked        {"question": "--single", "which": "all", "ask": "single", "budget": 1048576, "limit": 16, "max_steps": 1048576, "clock_us": null}
outcome      [{"part": "all", "outcome": "one", "why": "Solution: one state"}]
readings     {"word": {"part": "all", "word": "Solution"}, "k_s": null, "k_c": null, "minimal": null, "maximal": null, "exhausted": true, "stopped": null}
diagnostics  []
queries      [{"goal": "(and (drink-loc Water ?h_water) (nation-loc ?who_water ?h_water) …)",
               "goal_text": "The {?who_water} drinks water in {?h_water}; the {?who_zebra} owns zebra in {?h_zebra}",
               "agree": true, "classes": [{… "lines": ["The Norwegian drinks water in House-1; the Japanese owns zebra in House-5"] …}]}]
states       [{"facts": ["(color-loc Blue House-2)", … 25 facts], "digest": "sha256:…", "of": "model", …}]
work         {"saturations": 7, "facts": 278, "layers": null, "run_us": 15263, "load_us": 27637, "ask_us": 22632}
flagged, claim, owed, core, dependencies, provenance, fault: null

$ einaf feedback examples/ein-bugs/zebra2-bad.ein      (describe)
outcome      inconsistent over all / minimal / maximal: "R is empty and every node is dead"
core         {"facts": ["(color-loc Green House-1)", "(is-a House-1 House)", "(relation color-loc Color House)"],
              "sources": ["injected contradiction", null, null], "status": "within-budget",
              "refuted_by": ["(given)", "co-located-negative"]}

$ einaf feedback tests/region/04_owing.ein             (describe)
owed         {"root": ["likes owes Ann a Food."], "frontier": [{"facts": [], "owed": ["likes owes Ann a Food."]}]}
```

**E-67, a flagged program answered in full before it is filed as flagged**, closed for EinAf: a question that
needs the cube is asked at limit 0, which only a flagged program reads. `terminus.ein`: describe 8.43 s → 0.008 s,
`--maximal` 8.14 s → 0.008 s, `--count` 0.58 s → 0.007 s; all *flagged*.

**To notice**: under `--single` a flagged program's object still holds the verdict word the cube gave
(`programs/flagged/einaf-feedback-single.out`: `readings.word` `Ambiguity`), beside `outcome` *flagged*. The
ladder (S2.4.3) decides whether a level shows it; Ein's rule is that nothing is claimed of it.

## 7. The defaults taken, for the user to correct

Each stage recorded its defaults, numbered, in its record; 72 in all. The ones that are the user's to choose
come first.

**The user's to choose:**

1. **EinAf's `cargo fmt` and `cargo clippy -D warnings` as gate steps** (S2.1.1 default 3): proposed, not done;
   both run clean by hand. Two more static checks move the gate's count of static checks.
   [Q-M2.18](../open_questions/einaf/Q-M2.18-are-einafs-fmt-and-clippy-steps-of-the-gate.md).
2. **What EinAf's pages hold themselves to** (S2.1.1 default 5;
   [Q-M2.8](../../open_questions/einaf/Q-M2.8-where-does-einaf-live.md)'s entry): today the link check reads them, the
   release-voice check and `check_doc_blocks.py` do not.
3. **The schema numbers** (S2.1.2, E-92): `ein-summary/2` and `ein-test-report/6` were kept although each gained
   a key, by § 11's reading that the number names the shape's rule (every key on every run); bumping either moves
   `help_shape.txt`, whose option help names the number.
   [Q-M2.19](../open_questions/kernel/Q-M2.19-do-ein-summary-2-and-ein-test-report-6-keep-their-numbers.md).
4. ~~**The 38 kinds' names**~~ (S2.1.2 default 1). **Ruled 2026-11-25, accepted**: the user, *"Ok, I agree, it's
   ein.rs surface names, not ein language"*. A kind is a name of ein.rs's machine surface, one a section of the
   reference, and no word of the language; renaming one is a change to ein.rs (Q-M2.16), not to Ein.
5. **`--max-steps`, its name and its default 2²⁰** (S2.1.2 bound defaults 1, 2): the unit is a firing, an engine
   count (E-85); the corpus's largest saturation is 17 587 firings.
   [Q-M2.20](../open_questions/kernel/Q-M2.20-what-are-max-steps-name-unit-and-default.md).
6. **Which reading certifies *unique*** ([Q-M2.10](../../open_questions/answer/Q-M2.10-what-does-ein-certifies-a-unique-model-mean-on-r.md)),
   open until the experiments: the outcome function gives every reading and certifies none.
7. **The missing file's one line** ([S2.1.4](../s2.1.4_bugs_the_phase_found.md)'s design point 2, F23's D-1),
   added when S2.1.4 was planned: the reference page's `FileNotFoundError: [Errno 2] …`, kept by default, or a line
   in Ein's own words, which moves the page's row and every fixture that holds it.
   [Q-M2.21](../open_questions/kernel/Q-M2.21-a-missing-files-line-the-pages-python-spelling-or-eins-own.md).

Items 1, 3, 5 and 7 are the phase's questions, Q-M2.18 to Q-M2.21, in [its open questions](../open_questions/README.md);
items 2 and 6 are the milestone's; item 4 is ruled.

**Every default, by stage** (the record has each with its reason):

- **S2.1.1** (14): EinAf's own `target/`; the corpus read through `ein-corpus`; one gate step for EinAf; no
  toolchain pin; what each check reads of the new trees; three pages; the eight questions (`-n N` not one);
  `--no-search` is *inconsistent* on a dead root and *not decided* otherwise; a refusal at the run is *invalid*;
  the clock's error is *not decided*, even of a flagged program; a description its budget stopped is *not
  decided*; the mechanisms' judgement and the parse value re-done in EinAf (both since moved into the library);
  `.einb` refused; one commit.
- **S2.1.2, the bound** (10): the option and its unit; 2²⁰; nothing of R established at either stop; `why: steps`
  with `steps` and `saturation`; `monotonicity` and `root` `null` on a cut run; `ein oracle` the clock only and
  `saturate`, `show hyps`, `render h-space`, `render kb`, `solve --monotone` unbounded (E-84); the clock read
  every 2¹⁰ firings and at each quiescence; a coupling's whole node stopped by the cap is not read; the fixtures
  beside the test, not in the corpus (E-83); the examples page's X22 on `zebra2 --max-steps 100`.
- **S2.1.2, values in the library** (15): the kinds' names; four sections added; a line two sections list takes
  the kind of the section whose check makes it; a parse error is `parse` whatever its note; the message is the
  printed line with its place in it; severity follows the exit code; the run's kinds on `08-read-out.md` § 10; the
  kinds in `ein-core`, `#[non_exhaustive]`; the mechanisms' judgement moved into `ein_infer::mechanisms::judge_loaded`
  (E-64); `TextError::Parse` carries the value (E-65); `LoadError` and `MacroError` one finding; `CompileError`
  no kind; `EinbError::Program` a value; `04-api.md`'s example reads a kind and § 1 gains the `lrpar` patch line
  (E-62); the page's `**Kind:**` lines.
- **S2.1.2, the surfaces** (14): `diagnostics` last; `{kind, severity, message, place}`, the message without its
  severity word or `kb load error: `; stderr's order, the load's notes last; a summary with no answer has its eight
  answer keys `null`; no summary for an engine error or `ein oracle`'s refusal past its limit (E-89);
  `--dump-answer` writes nothing for a run with no answer; both schema numbers kept (E-92); an unreadable
  container is `unreadable`; an artefact's failure is a diagnostic; `ein test`'s report rows; `ein test`'s abort
  listed after stderr's lines though printed on stdout (E-90); `core_sources` for root's core alone (E-91); the
  corpus sweep under `--no-search` too; EinAf's container refusal `unreadable`.
- **S2.1.3** (19): one door, one load, one object a question; `einaf feedback`; every key, `null` where it does
  not apply; the probes under `analysis/probes/p2.1/`; the binary `$EIN_BIN`, else the gate's; one shared sweep
  (5.2 s); no change to Ein; E-67 by limit 0, not by the static check first; a flagged program's cube question
  records the flag and no answer; `outcome` a list; kebab-case names; parts beyond the table (`claim`,
  `readings.stopped`, `fault`); `kernel.version` from the engine's manifest at build; microseconds; the library's
  `Answer::wall` kept as `run_us`; `einaf outcome --read-out` removed; `-n N` runs not compared; the repository's
  root stripped from paths in the comparison (E-101); of a flagged program the comparison reads the flag and the
  sites only.

## 8. What the phase found of Ein

[The evaluation](../../evaluation/README.md) gained E-62 to E-101 in this phase, which live in
[the phase's `evaluation/`](../evaluation/README.md) until it closes (E-62 to E-67 S2.1.1's, E-68 to E-93
S2.1.2's, E-94 to E-101 S2.1.3's), and closed E-20, E-21, E-45, E-62, E-64, E-65, E-67 (for EinAf), E-76, E-77,
E-78, E-88. Open, by section:

| section | entries |
|---|---|
| strengths | E-66 the seven outcomes read off values, eight questions agree on every corpus program; E-68 every line a run prints is a value of its summary; E-94 the library and the binary agree on every compared run |
| weaknesses | E-69 one line, two sections; E-70 one walk makes three sections' lines; E-71 a line's place not always in front; E-72 a parse error inside a refusal may be a module's; E-85 the cap counts firings; E-86 `ein test` asks a cut file twice; E-87 `region.stopped.why` has two values no summary reaches; E-90 `ein test` prints the abort on stdout; E-92 a schema's number in an option's help; E-93 a fixture banks one finding of two; E-101 a module's path in a message is absolute |
| gaps | E-73 a refusal about a top-level form has no place; E-75 two refusals only a library caller meets; E-82 a run the cap stops hands back nothing it established; E-89 a line with no kind stays off every summary; E-91 a core's sentences beside root's core alone; E-95 no library value says which kernel it is; E-96 the abort's line is the binary's; E-97 a run's diagnostics come from the rendered read-out; E-100 nothing bounds a call's memory |
| bugs | E-74 the library spells a missing file a second way; **E-79 `ein saturate --max-steps N` crashes (stack overflow) on a term nested past ~30 000** |
| limits | E-80 `--events` grows quadratically on a deepening term; E-81 the default cap costs ~1 GB on a runaway; E-83 a program with no fixpoint cannot be a corpus program; E-84 four readers of root take no bound; E-98 the clock is not read where nothing fires; E-99 a deep term's read-out is outside the clock |

What the phase found that is neither Ein's nor a question for the user, L-M2.1 to L-M2.4 (the host's toolchain,
EinAf's missing pin, EinAf's load judged with no bound, Ein's API change leaving EinAf unbuilt until its own commit),
is in [the phase's findings](../findings/README.md).

## 9. Verification

- `./run_tests.sh` on `ddb1006e`: exit 0. Ten static checks, `cargo test --profile gate --workspace` and
  EinAf's tests (18, the corpus sweep among them): 1 434 tests passed, 0 failed, the process's checks, the review bundle's
  smoke, the bench smoke.
- `./run_tests.sh --nightly` on `ddb1006e`: exit 0. Every slow corpus entry under the deep-corpus sweeps (lever,
  route, import, id-order, job-count and renaming invariance, the region's and the obligation rung's slow halves),
  the four shards (answer round trips, cores, the agenda, the order), the mutants, the fuzzer on both seeds and the
  bench smoke: no answer moved under the bound.
- One golden moved in the phase: `ein-render/tests/golden/corpus_shapes.md5`, its 3 445 `cores[…]` lines, because
  the region block gained `core_sources`; no `.expected` fixture, no `help_shape.txt` line moved for the
  diagnostics. `help_shape.txt` moved once, for `--max-steps`.

## 10. What is left, and where

- **Q-M2.8**'s remaining part, EinAf's gate steps, the schema numbers, the kinds, the cap: § 7, the user's.
- **[S2.1.4](../s2.1.4_bugs_the_phase_found.md)**, planned on the user's word after this report and run to its end
  the same day ([its record](../s2.1.4_bugs_the_phase_found.md#record)): E-79, E-74, E-101 and E-90 fixed, E-99 closed
  as far as it went, E-102 and E-103 filed. The other open entries of § 8 stay the evaluation's, for the iteration of Ein that
  follows M2 or the stage that needs one; none blocks P2.2.
- **P2.2** is planned (S2.2.0a, S2.2.0f added and the user's comments committed, `e63a21b0`) and not started.
