# S1f.5.28 — The findings taken from the stage records: confirmed, triaged, fixed

**Phase:** [P1f.5](README.md) (The no-owner items)
**Status:** specified 2026-11-10 on the user's ruling on the close review's finding 9, and given finding 7's three
stale counts by default ([close_review.md § Findings with no owner](close_review.md#findings-with-no-owner)). **Done
2026-11-10**, in the working tree on `089511fe` for the coordinator's commit: T1f.5.28.1, every item re-checked on the
release binary, all eleven confirmed, items 6 and 8 wider than written ([§ T1 record](#t1-record-2026-11-10));
T1f.5.28.2, the triage: no wrong answer, ten defects that move no answer, fixed, and four choices taken by default,
Q-M1f.108 to Q-M1f.111, with finding 7's placement recorded as Q-M1f.107 ([§ T2 record](#t2-record-2026-11-10));
T1f.5.28.3, the fixes, by kind: two pages pinned where they had no pin (`region_dump.md`'s `monotonicity` block,
`configuration.md`'s `leftover` block), the read-out's two ungrouped counts, `read_out.md`'s route-1 sentence pinned,
the trace's text line breaks, the stale comments and notes, a test fixture, `utils/guard_split.py`, which split no
program since M1e2 S1e2.1c.2, a re-saved container, the fuzz test parsing its stream once, 4.3× less CPU, and the slow
prose ([§ T3 record](#t3-record-2026-11-10)). One golden moved, `corpus_shapes.md5`'s two region-trace digests, named
before it was blessed; no answer and no register row moved: 249 fixed, 13 open and 1 not wrong (`grep '^status'
corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-10). No wrong answer was found outside the items. The trace writer
and `ein-einb`'s writer changed, neither a DOT writer, the review bundle or a CI install
([§ Acceptance, re-taken](#acceptance-re-taken-2026-11-10)).
**Depends on:** nothing. It runs last of the phase's stages, after S1f.5.27, so its re-checks are taken on the tree
the other seven leave.

## The ruling

The close review's finding 9, as written on 2026-11-10, is a list *"taken from the stage records and not re-checked
today"* (below). The user, 2026-11-10: *"new stage for investigation, confirmation, triage and fixes"*.

Finding 7, the stale slow counts, drew a question rather than a ruling: *"so what? what does mean slow here? slow
for what? compared to what?"*. The close review answers it under the finding. The fix is three prose edits and a
citation, so the coordinator placed it here by default, recorded as a Q-M1f row. The user may move it.

## The items

| # | item | from |
|---|---|---|
| 1 | `region_dump.md` never names seven fields: `reads.dynamic_*`, `absent_reads.dynamic_*`, `cycles[].negative`, `paths.dead_parents`, `paths.saturations`, `paths.unprinted.saturations`, `state.stale_nodes` | S1f.5.16 |
| 2 | `configuration.md` never names `leftover.states_by_relation` | S1f.5.16 |
| 3 | `-t`'s `route` row and the cube's counts print without digit groups | S1f.5.16 |
| 4 | Route 1's *stopped before layer L* is reached by no CLI run | S1f.5.16 |
| 5 | `--trace` prints a `:why` or `:source` as markdown paragraphs, where a single line break is a soft break | S1f.5.16 |
| 6 | Stale comments: `ein-core/src/terms.rs` (the SYMBOL exclusion and `LEXER_KEYWORDS`), `FIFO_GAP_PRIORITY` in `ein-infer/tests/detector.rs`, the headers of `form-in-guard.ein` and `guard-before-binder.ein`, the `corpus/fuzz_findings` notes | several |
| 7 | `player` is declared `T T` but used with one argument in two Rust tests | S1f.5.14 |
| 8 | `utils/guard_split.py` reports `synth/trigger_hole` as *does not load* | S1f.5.14 |
| 9 | `ein kb save` of a container writes 1 288 B where the `.ein` gives 1 248 B for the same KB | S1f.5.19 |
| 10 | The fuzz property costs about 27 s per 20 000 inputs | S1f.5.10 |
| 11 | Finding 7: the `corpus/corpus.toml` header says *"Two entries are"* slow and `corpus/README.md` § `slow` says *"Three"*, naming `features/04_open`, which is not slow since M1e2 S1e2.1a.5. That README and `ein-cli/tests/corpus_cli.rs` line 112 cite `the_slow_set_is_exactly_these_three`, a test renamed `…_these_fourteen` (`ein-corpus/src/manifest.rs`). 14 entries are slow (`grep -c '^slow *= *true' corpus/corpus.toml`, 2026-11-10) | the close review |

Items 12 to 16, the wrong answers S1f.5.21's and S1f.5.24's records found outside their scope, were given here by
the coordinator on 2026-11-10. **They moved the same day to a phase of their own,
[P1f.7](../p1f.7_findings_outside_scope/README.md), a stage each (S1f.7.1 to S1f.7.5)**, on the user's instruction:
*"new batch of findings … creates new phase p1f.7 with stage per finding to allow to close current phase"*. A wrong
answer this stage finds outside items 1 to 11 goes there too, as a new stage.

Item 10 is S1f.5.27's to measure (its T1 attributes the fuzz job's cost). This stage takes it only for what that
measurement leaves to fix.

## The method

**T1f.5.28.1, confirmation.** Each item is re-checked on the release binary at the tree S1f.5.27 leaves, with the
command or the read that shows it, and marked *confirmed*, *not reproduced* (with what changed) or *already fixed*
(with the commit).

**T1f.5.28.2, triage.** Each confirmed item gets one of three dispositions:
- **A wrong answer**: a register row, filed `open` before any fix. For example, item 9 if the two containers do
  not answer alike, or item 8 if `trigger_hole` loads and the script is wrong about a program.
- **A defect that moves no answer** (a page, a comment, a label, a script's report): fixed here.
- **A question that is the user's**: one that would reverse a ruling, or that has no stated criterion. Recorded as a
  Q-M1f row and put to the user. Anything else takes the stage's default and is recorded as defaulted.

**T1f.5.28.3 onward, fixes**, one commit per kind (pages, comments, tests, the script, the printer), each gated.
A test pins a page fix where the page has a test (`read_out_reference.rs`, `config_reference.rs`, the region dump's
reference). Prose that states a count either carries its command or test, or drops the count (Q-M1e.4).

## Acceptance

- Every item is confirmed, not reproduced or already fixed, with its evidence, in the stage's record.
- Every confirmed item is fixed, a register row, or a Q-M1f row the user has seen.
- `corpus/README.md`, the `corpus.toml` header and `corpus_cli.rs` say what `the_slow_set_is_exactly_these_fourteen`
  pins, by citing the test rather than restating its number.
- `./run_tests.sh` green.

## T1 record, 2026-11-10

T1f.5.28.1, on `089511fe` with the release binary built from it (`cargo build --release` in `ein.rs`, up to date).
Evidence under `build/s1f.5.28/` (gitignored). Each item is **confirmed**; none was already fixed or failed to
reproduce.

| # | the check | what it showed |
|---|---|---|
| 1 | `grep -c` each field, backticked, on `region_dump.md` | 0 for each of the seven; and the page named no field of `hypothesised`, `dynamic`, `plans`, `reads`, `absent_reads`, `sites[]`, `status`'s, `state`'s or `paths`' either, nor `region.json`'s `absents[]`: it names the block's keys in prose and no test held any field |
| 2 | `configuration.md`'s `EIN_LEFTOVER` row; `EIN_LEFTOVER=1 ein solve --maximal examples/branching/04_two_levels.ein --json-summary J` | the block's keys are `taken`, `states` and `states_by_relation`, `[{"co-located": 12, "is-a": 30}, …]`; the row named none of the three (S1f.5.16's record has it naming the first two) |
| 3 | `ein-cli/src/modes.rs`' `timing` and `oracle.rs`' `the cube` line; a probe whose counts pass a thousand, eleven slots an hrule guesses and an obligation `(h A)` triggers that nothing pays | both printed the integer bare: *2048 saturations* (`ein solve F -t`), *1024 discharged nodes* (`ein oracle F`), where every other row prints `2 048`. No corpus `oracle` or `-t` cell prints a count of a thousand or more on either line (each such declared run, 0 hits) |
| 4 | `explore.rs`' route choice, `downset::compute_prepared`'s budget check, `--region-budget 0`; a flagged program whose root is dead at `--region-budget 1` | describe takes route 1 only on a flagged program whose root is dead; layer 0, root, projects `projected(1, None)` = 1 fact, which the smallest budget the command takes admits, and *invalid region budget: '0' (expected 1 or more facts)* refuses the one below it; the probe reads root, exhausts (*route 1, the alive down-set — L0*), and layer 1 is empty. `explore::describe_by` reaches the arm, from `route_invariance.rs` |
| 5 | `ein solve tests/text/01_unicode.ein --trace T.md` | `> Ann 有房子, … 🦓` then `вторая	строка` on the next line, unprefixed: a lazy continuation of the quote, which a renderer joins with a space; the same for each premise's `:source` (`\n` and `\r` both) |
| 6 | each comment against today's engine | `terms.rs`' `RESERVED` doc: *the grammar SYMBOL-excludes* eleven words, *`ein-ir`'s `LEXER_KEYWORDS`*, *overlap in four names*, and *`open` … must never be there*, where since S1f.5.23 the thirteen `grammar::RESERVED_WORDS` (the printer's copy `ein_core::text::LEXER_KEYWORDS`) hold `open` and `solve`, and the two sets share five. `detector.rs`' `FIFO_GAP_PRIORITY`: *`r2` at priority 100*, where the program has carried no `:priority` since S1f.5.15's T5 and is `lattice/10`'s. `form-in-guard.ein`'s and `guard-before-binder.ein`'s headers and corpus notes: *refused at load, every command exits 1*, where both load since S1f.5.7's `6841faff` (`ein saturate --dump`: `(hit-eq (f a) :rule e)`, `(hit-eq a :rule e)`, exit 0). `corpus/fuzz_findings/`: the README's `ein solve … d3-unsat-core.ein --max-set-size 2` (*-m/--max-set-size is retired*) and its `ein.rs/target-fd` build (gone with the walk), its kwpair row naming a witness moved to `examples/broken/`; `hrule-reads-not.md`'s `--max-set-size 2` repro, and its verbatim panic site `hrule.rs:113:13`, which is `130:13` on today's debug binary, so `utils/fuzz_engine.py`'s dedup key read from the note no longer matched the panic. **Beyond the list**: eighteen corpus notes say *Open, owned by S1f.5.7* or *S1f.5.10* of a row `fixed` since (W30 ×3, W50, W51 ×3, W52 ×2, W53, W54, W55 ×2, W56, W57 ×4) |
| 7 | `kb_semantics.rs`' two `(relation player T T …)` | used as `(player ?p)` and `(player ?q)`; the tests compare expansions and load no fact of `player`. Matching is arity-coupled (`01_grammar.md` § The membership fact), so a premise of another arity matches nothing, defined, and no answer is read: a fixture defect |
| 8 | `python3 utils/guard_split.py examples/lattice/synth/trigger_hole.ein`; `ein saturate`, `ein solve`, `ein oracle` on it; `--corpus` | *not split — does not load*; the program loads (`ein saturate` exit 0) and `ein solve` and `ein oracle` refuse it with *forbid*'s line, exit 1, as W3's witness (its corpus note), so the reason the script gave was false; not an answer of the engine, whose refusal is W3's right. **Wider**: `--corpus` read 32 programs, 0 agree, 32 not split (3.7 s), 25 of them *cell 0: route states, exhausted True*: the script took a cell's route as `"1"` alone, and since M1e2 S1e2.1c.2 describe on a monotone program takes the state route, so it has split no program since 2026-11-03, while `monotonicity.md` § 6.4 says the cells are solved by the state route |
| 9 | `ein kb save` the witness, then a save of each save; the same for `zebra2`, loaded and `--saturate`; the `PROV` section decoded | 1 248 → 1 288 → 1 328 bytes; `zebra2` loaded 21 392 → 24 096, saturated 62 752 → 65 464 → 68 168, the second past design/10 §6's 64 KB. **The containers answer alike**: `ein solve` stdout (the header aside), `--json-summary` (`source` aside) and `ein saturate --dump` (its size line aside) identical on the `.ein` and every container; on `zebra2` every `ein solve` identical. Only `PROV` differs: 1 → 2 → 3 records, 66 → 132 on `zebra2`, the added ones referenced by nothing and after the file's own: the loader's records for the given facts, pushed when a container is opened (`open_bytes`), which the writer wrote with the whole arena |
| 10 | S1f.5.27's T1 record; `fuzz_properties.rs` | five properties (1, 2, 5, 6, 7) each generated and parsed the whole stream; S1f.5.27 put 99 % of an input's cost on the parse and 1 % on S1f.5.10's property. Before, release, 20 000 inputs (§ T2 record): one P-core, seed 1, 197.3 s |
| 11 | the three places, and `git grep these_three` | the `corpus.toml` header's *Two entries are*, `corpus/README.md`'s *Three* with `features/04_open` and *17.1 s*, `corpus_cli.rs`' *Three* and `…_these_three` in both, against 14 (`grep -c '^slow *= *true' corpus/corpus.toml`, 2026-11-10) and `ein-corpus`'s `the_slow_set_is_exactly_these_fourteen`. Beside them: `corpus_cli.rs`' module doc calls `features/04_open :: render lattice` *the slowest cell today* and *why that entry is `slow`*, the README's *One second* bullet states the 2026-08-22 distribution undated, and AGENTS.md's command block says *the 14 slow entries* |

**Found, not a wrong answer and not new:** the debug build still panics on `corpus/fuzz_findings/hrule-reads-not.ein`
(`ein.rs/target/debug/ein solve …`, *an hrule reads `not`, which hypgen's kill cache writes mid-enumeration*, exit 101),
where release answers; its note has recorded it as an open question since 2026-08-21. Left with its note, whose site
this stage corrected.

## T2 record, 2026-11-10

T1f.5.28.2, the triage. **No item is a wrong answer**: item 9's containers answer alike, item 8's script reports no
answer of the engine, item 7's tests read no answer. Each item's disposition:

| # | disposition | how |
|---|---|---|
| 1 | a page defect, fixed | the block's fields on the page and a test that holds them both ways |
| 2 | a page defect, fixed | the row names the three keys; a test holds the row to the block |
| 3 | a printer defect, fixed | both counts through `ein_render::read_out::group`; the page says so; a test |
| 4 | a choice, defaulted | [Q-M1f.108](../open_questions.md#q-m1f108--route-1s-stopped-before-layer-l-which-no-ein-run-prints) (a): the arm stays, the page says why no `ein` run prints it, a test |
| 5 | a printer defect and a choice, defaulted | [Q-M1f.109](../open_questions.md#q-m1f109--a-texts-line-break-in-the-traces-markdown) (a): a hard break; shown to the user, since it moves the bytes Q-M1f.12 holds |
| 6 | comments and notes, fixed | the list and the eighteen notes beyond it |
| 7 | a test fixture, fixed | `player` declared unary |
| 8 | a script's report, fixed | the refusal's own line; the state route read |
| 9 | a writer defect and a choice, defaulted | [Q-M1f.110](../open_questions.md#q-m1f110--a-container-saved-again) (a); shown to the user, since S1f.5.19 recorded it and left it |
| 10 | the test's shape, a choice, defaulted | [Q-M1f.111](../open_questions.md#q-m1f111--the-fuzz-tests-shape) (a); the recovery search's cost stays Q-M1f.8's |
| 11 | prose, fixed | the coordinator's placement recorded as [Q-M1f.107](../open_questions.md#q-m1f107--where-the-close-reviews-finding-7-is-fixed) |

**Item 10, measured.** `ein-ir/tests/fuzz_properties.rs`, release, `EIN_FUZZ_ITERS=20000`, each under
`utils/bench_env.sh` after the coordinator's container nightly had finished (its message), the before binary built from
`089511fe` and kept as `build/s1f.5.28/t10/fuzz_before`:

| run | before | after | every property's counters |
|---|---:|---:|---|
| `--core 4`, one P-core, `--test-threads 1`, seed 1 | 197.3 s | 46.1 s | the same: 4 298 accepted, 15 702 refused; 3 963 of 5 000 refusals read twice; 6 257 keyword pairs; 3 873 loaded, 15 refused for an unbound conclusion |
| `--cores PT:4`, the runner's count, seed 1 | 92.3 s | 23.6 s | the same |
| `--cores PT:4`, seed 987654321 | 87.4 s | 21.0 s | the same: 4 376 accepted, 15 624 refused; 3 947 of 5 000; 6 379; 4 026 loaded, 16 refused |

The parse of a refused input is still the cost; it is paid once where it was paid five times, and spread over the
threads the process may use. **Each property still bites** (scratch mutations of the test, each restored after, `cmp`
identical): property 5's `relation` list emptied fails at *iteration 52 … carries :why, which it does not list*;
property 2's re-dump altered fails at *iteration 8 … does not round-trip* and writes its minimised finding (removed);
property 4's kept report altered fails at *iteration 4 … reported two ways*; property 7's refusal forced fails at
*iteration 105 … the compiler refuses a program the loader accepts … minimised*.

## T3 record, 2026-11-10

T1f.5.28.3 onward, the fixes, by kind; the files are listed in the coordinator's report for a commit per kind.

- **Pages, each with its pin.** [`region_dump.md`](../../../docs/kernel/inference/region_dump.md) § The `monotonicity`
  block, field by field: a row per field, the summary's and `region.json`'s, held both ways by `region_dump.rs`'
  `every_monotonicity_field_is_defined_on_the_page` over nine runs that reach the five statuses, `paths.unprinted`,
  `-y` on a monotone program and `--monotone`, and an oracle dump (a dropped row and a stray row each fail it).
  [`configuration.md`](../../../docs/kernel/configuration.md)'s `EIN_LEFTOVER` row names `taken`, `states` and
  `states_by_relation`, held by `config_reference.rs`' `the_leftover_row_names_every_key_of_its_block` (it fails on
  `089511fe`'s page). [`read_out.md`](../../../docs/kernel/inference/read_out.md) § 9 says every count prints in digit
  groups, and § 4's partial-description paragraph says why no `ein` run prints route 1's stop, pinned by
  `read_out_reference.rs`' `the_clock_and_the_cube_group_their_counts` (it fails on `089511fe`'s printer: *2048
  saturations*) and `route_1_never_stops_on_an_ein_run`. [`09_natural_language.md`](../../../docs/kernel/ir/03-ein-lang/09_natural_language.md)
  says how `--trace` writes a text's line break. [`monotonicity.md`](../../../docs/kernel/inference/monotonicity.md)
  § 6.4 has the guard split's re-take.
- **The printers.** `ein-cli`'s `modes.rs` and `oracle.rs` group their counts; `ein-render`'s trace writes each text,
  a `:why`, a `:source`, a statement and an owed instance, through `trace::render::md_text`. The coordinator's
  review added one case, a `\r\n` pair, which the stage's first version wrote as two breaks with an empty line between
  them, a new markdown paragraph: it is one break, held by the unit test `each_break_is_one_hard_break`. No corpus text
  holds the pair, so no golden moved again (`grep -rn --include='*.ein' '\\r' examples tests stdlib`, 2026-11-11:
  three files, two writing `\r` in a comment and `tests/text/01_unicode.ein` a lone `\r` in three strings).
- **The writer.** `ein-einb`'s `sections::Kept`: `PROV`, `UNIVERSAL` and `CLOSURE` stop at the last record the KB
  references, so no id moves; `einb_cli.rs`' `a_container_saved_again_is_the_container` holds the witness and a
  saturated `zebra2` byte-identical over two saves of a save, each answering as its program. After: 1 248 bytes three
  times, 62 752 three times; a first save's sections are what `089511fe` wrote (`META` aside, the engine's stamp), but
  where the arena ends on a record nothing references (`examples/syntax/unary-and-nary.ein`, a fact stated twice),
  which goes. The first design, every unreferenced record dropped and ids remapped, failed `ein-einb`'s round trip on
  that program (*primary provenance differs*), and is Q-M1f.110's (b).
- **The test's shape.** `fuzz_properties.rs`: `stream()`, one parse, and `inputs`/`accepted_inputs`/`reparsed`, with
  the module doc and property 4's doc saying so.
- **Tests.** `kb_semantics.rs`' `player` declared `T`; `detector.rs`' constant `FIFO_GAP_WRITTEN`, its doc restated;
  `natural_language_text.rs`' trace expectation and its table row.
- **Comments and notes.** `terms.rs`' `RESERVED` doc; the two W24 headers, rewritten line for line (31 and 32 lines,
  so no position moves) with their corpus notes; the eighteen notes, each closed with *Fixed by Sx's Tx (`hash`,
  date)* from its row's `[case.fixed]`; `corpus/fuzz_findings/README.md` and `hrule-reads-not.md`; `fuzz_engine.py`'s
  site comment.
- **The script.** `utils/guard_split.py` reads route 1 or the state route, and names `ein oracle`'s exit and line where
  there is no `region.json`: `trigger_hole.ein` reads *`ein oracle` exits 1: rule 'dead-1': concludes (false) from
  (absent …) over h, …*. `--corpus` after: 32 programs, 25 agree with the oracle node for node, 0 disagree, 7 past the
  limit (`python3 utils/guard_split.py --corpus`, 5.1 s, 2026-11-10). [`utils/README.md`](../../../utils/README.md)'s
  row says so.
- **The slow prose.** The `corpus.toml` header, `corpus/README.md` § `slow` and `corpus_cli.rs` cite
  `the_slow_set_is_exactly_these_fourteen` and state no count of their own; the README's distribution is dated to
  M1a T1a.7.2.0; `corpus_cli.rs`' slowest-cell sentence dated to 2026-08-22; AGENTS.md's command comment cites the test.

**Goldens**, named before they were blessed (`EIN_BLESS=1 cargo test -p ein-render --test corpus_shapes`): two lines of
`ein-render/tests/golden/corpus_shapes.md5`, the region traces of `examples/features/15_unicode_escapes.ein` (86 lines)
and `tests/text/01_unicode.ein` (71 → 74 lines), each a text's line break. Item 3 moved none: no corpus run prints a
count of a thousand or more on either line.

## Acceptance, re-taken 2026-11-10

On the working tree at `089511fe`, item by item:

- **Every item is confirmed, not reproduced or already fixed, with its evidence, in the stage's record.** All eleven
  confirmed, each with the command or read in § T1 record. **Met.**
- **Every confirmed item is fixed, a register row, or a Q-M1f row the user has seen.** None is a wrong answer, so none
  is a register row. Ten are fixed, each with the test that holds it where a page or a printer moved (§ T3 record);
  items 4, 5, 9 and 10 also took a choice by default, Q-M1f.108 to Q-M1f.111, and item 11's placement is Q-M1f.107.
  Q-M1f.107, .109 and .110 are shown to the user at the close: .107 is the coordinator's placement, .109 moves the bytes
  Q-M1f.12 holds the trace to, and .110 settles what S1f.5.19 recorded and left. **Met**, the user's review of the five
  rows open at the close.
- **`corpus/README.md`, the `corpus.toml` header and `corpus_cli.rs` say what `the_slow_set_is_exactly_these_fourteen`
  pins, by citing the test rather than restating its number.** Each cites it and states no count; `git grep
  these_three`, outside `plans/` and `docs/history/`, finds nothing. **Met.**
- **`./run_tests.sh` green.** Green, 1 042.4 s (17 min 22 s) (`./run_tests.sh`, `build/gate-s1f.5.28.log`, finished
  2026-11-11): 1 031 tests passed and 0 failed, the five new pins among them; the smoke read 263 register rows, 0
  failed, built 4 cases with 0 DOTs refused and 46 SVGs, its link check exit 0. The edits after it started are this
  file's, the phase README's row and `open_questions.md`'s prose, re-checked by `doc_audit.py --links --check` on the
  final tree. The coordinator's review then changed `trace::render::md_text` (§ T3 record, *The printers*) and ran
  the gate again on the reviewed tree: green, 1 032 tests passed and 0 failed, the new unit test among them, the smoke
  4 cases, 0 failed, 0 DOTs refused, 46 SVGs, its link check exit 0 (`./run_tests.sh`,
  `build/gate-s1f.5.28-review.log`, 2026-11-11). **Met.**
