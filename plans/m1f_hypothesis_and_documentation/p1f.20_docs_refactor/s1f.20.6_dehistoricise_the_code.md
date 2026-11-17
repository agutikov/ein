# S1f.20.6 — De-historicise the code, and dissolve `docs/history/`

**Phase:** [P1f.20](README.md) (`docs/ein/`: the tree a released system would have)
**Estimate:** 5 days.
**Status:** **done 2026-11-17** ([§ Record: `docs/kernel/` deleted](#record-2026-11-17-docskernel-deleted)): six
lanes, then the deletion of `docs/kernel/`. It was `T1f.5.20.6` until 2026-11-08.
**Depends on:** [S1f.20.3](s1f.20.3_user.md), [S1f.20.4](s1f.20.4_reasoning.md) and
[S1f.20.5](s1f.20.5_ein_rs.md): a code comment points at the `docs/ein/` page that states its behaviour, so the
page must exist first.

## What it does

The largest mechanical piece. On 2026-08-28 it was **963 id-bearing lines in 175 `.rs` files**, plus 303 in
`.ein` headers and 47 in `corpus.toml`. [S1f.20.1](s1f.20.1_the_two_checks.md)'s census gives today's count.

Four substitutions, in decreasing frequency:

1. **`(M1d P1d.2 S1d.2.4)` → a link to the `docs/ein/` page that states the behaviour.** This is the user's
   own example, and the common case.
2. **A link into `docs/history/m1a_rust/design/NN` → the same file's new home under
   `docs/ein/ein.rs/design/`.** It is mechanical, and the link check verifies it. S1f.20.3 to S1f.20.5 have
   done this already for every page they moved ([P1f.20 § Two rules](README.md#two-rules-across-the-stages)).
3. **A sentence whose *subject* is the development event**, where the id carries the paragraph's meaning:
   *"Empty before S1a.6.9 and not because there was nothing to say"*. These do not substitute. They are
   **rewritten as statements about the code** (*"A fork that resumes root's saturation does not re-derive
   root's closure, so the givens are told first"*), and the history is dropped. There are on the order of a
   hundred, and they are the only slow ones.
4. **A reference to a question.** Since the user's rulings of 2026-11-15 a decision may be cited: a question
   with an answer, ruled or a standing default, is a file under `docs/decisions/`
   ([S1f.20.8](s1f.20.8_decisions.md)), and the comment cites its id, linked. A question still open is a work
   item: it stays in `plans/`, and the code comment states the *behaviour* and its uncertainty, without the id.
5. **A date that dates a development event** (*"since 2026-11-03"*): deleted, the sentence restated as what
   is true now (the same rulings).

`AGENTS.md` and `README.md` are the last pair touched. They are exempt from the release-voice check, but not
from the link check, so every link they hold into `docs/history/` is rewritten.

**The dissolve of `docs/history/` is [S1f.20.9](s1f.20.9_the_history_sweep.md)'s** since 2026-11-15, with the
finished plans, once every decision is a file: this stage rewrites the references, that one deletes.

## The method

**Before rewriting a `.ein` comment** ([S1f.20.8's gate](s1f.20.8_decisions.md#the-gate-on-the-stages-batch-2026-11-17)): keep the file's line count, since register rows and goldens record positions after it, and expect `ein-ir/tests/golden/fuzz_seeds.txt`, which quotes every fuzz seed's text, to need a bless checked to be comment-only.


### Task T1f.20.6.1 — the crates

Substitutions 1, 3 and 4 over every shipped crate's doc comments and the tests. The release-voice baseline is
lowered as each file reaches 0.

### Task T1f.20.6.2 — `.ein` headers and `corpus.toml`

The same, over the corpus.

### Task T1f.20.6.3 — `AGENTS.md` and `README.md`

The pair's links rewritten. The deletions and the empty baseline are [S1f.20.9](s1f.20.9_the_history_sweep.md)'s.

## Acceptance

- **0 history-referencing lines outside `plans/`**, by S1f.20.1's census, but for those S1f.20.9's deletions
  resolve; a decision id is not one.
- No reference into `docs/history/` is left for S1f.20.9 to break: not in a `.md`, a Rust doc comment, a `.ein`
  comment, `corpus.toml` or `AGENTS.md`.
- `RUSTDOCFLAGS="-D warnings" cargo doc` is green: 464 doc comments change.
- `./run_tests.sh` is green.

## Record 2026-11-17: `docs/kernel/` deleted

The stage's last task, run after its six lanes (whose reports the coordinator summarises), in the main working copy.

**The deletion.** `git rm -r docs/kernel`: 27 tracked files, 15 pages (`README.md`, `architecture.md`,
`defined_behaviour.md`, `inference/{README, architecture_and_algorithms, solution_semantics, algorithm_layer_n,
lattice_diagrams, parity_baselines, features, domain_elim_vs_hypothesis}.md`, `ir/01-ein-graph/{01_kb, 02_rules}.md`,
`ir/02-data-model/{01_entities, 02_store}.md`) and 12 diagram files (`algorithm_layer_n`, the four `lattice_*`
drawings and `region_02_genuine_3set_death_no_lookahead`, each `.dot` and `.svg`). What of them is true today has a
home in `docs/ein/` by the anchor maps of S1f.20.3 to S1f.20.5; what is not carried is what
[`disposition.md`](disposition.md) does not carry (the walk's pages and `solution_semantics.md` § 6, `defined_behaviour.md`
§§ 2.3, 3.1 and 6, the superseded pages and their drawings).

**The links into it**, 128 outside the tree:

- **`plans/` and `docs/history/`, 120 markdown links**, by one script with a decision per link, kept in the
  scratchpad and not banked. **58 re-aimed** (45 in `plans/`, 13 in `docs/history/`) to the page, and where the
  link named a section to the anchor, that states it now: `solution-semantics.md` (the page, § Worked),
  `values-and-order.md` (old §§ 2.1, 2.4, 2.5, 3.2), `saturation.md` (old § 3.3), `algorithms.md` and `firing.md`
  (the operations O1–O9, old §§ 6–7), `ein.rs/architecture.md` § 6 (*one artefact, one owner*), `docs/ein/README.md`
  (the tree), `syntax-and-semantics/` and `static-ir/` (the two `ir/` folders), and `diagnostics.md` for M2's
  README's one general citation. **62 unlinked** to their label's text (38 in `plans/`, 24 in `docs/history/`):
  every link to a page or section not carried (`features.md`, `domain_elim_vs_hypothesis.md`,
  `algorithm_layer_n.md`, `inference/README.md`, `solution_semantics.md` §§ 2.6 and 6.x, `defined_behaviour.md`
  §§ 3, 3.1 and 6); every other citation of `defined_behaviour.md` as a whole, a page split three ways that no one
  page states now; and every record of an edit to an old page at its old section numbers or lines, which a re-aim
  would pin to the wrong page. An unlinked label that was a bare `§ N` got its page's name. Four links had the prose
  right after them restated with the re-aim (`§7` → `§ 2`, `§O1/§O2/§7` and `§O5 / §7` → `§ 1 and § 2`, `§ 2.4
  pins` → `pins`), so the link check's prose-`§` rule reads the new page's numbers.
- **One `.ein` bare path**, in `plans/m1e2_correctness/probes/cfgtest.ein`, turned into text, its line count kept.
- **43 link labels** in `plans/` and `docs/history/` that named a `docs/kernel/…` path while the link already resolved
  into `docs/ein/` (earlier stages re-aimed the target, not the label) now name the target's path.
- **Process files, 7 links**: `AGENTS.md` (4), `utils/README.md` (2), `utils/censuses/region-census.md` (1); and the
  three links `utils/review/run.py` writes into the review bundle, now `solution-semantics.md` § 1.3 (`#13-root`),
  § 1 (`#1-the-vocabulary`) and § 2 (`#2-the-answer-the-region-r`).

**Process files rewritten.** `AGENTS.md` § Where things live: the `docs/kernel/` bullet is a `docs/ein/` bullet (the
three perspectives, the pages an agent most needs at their paths, the current-state rule with Q-M1f.341, Q-M1e.4
cited by its decision file, the standard of proof, what keeps the pages true) beside a new `docs/decisions/` bullet;
the old tree's three states, banners and moves into `docs/history/` are gone. The guide bullet, `diagnostics.md`
§ 2.30, the `--inbound` example and the generated list (now with `printable.rs`) follow. `utils/doc_audit.py`: the
default tree is `docs/ein`, the **states** question says no page keeps a banner, nothing reads
`docs/kernel/README.md`. `utils/history_census.py`: the rows of `docs/kernel`, `docs/api`, `docs/guide` and
`docs/install.md` are one row, `docs/ein`. `run_tests.sh`: two comments. `utils/README.md`: the `doc_audit.py` row
and the `feature_matrix_results.json` note.

**Left as text, on purpose**: the `guard_split.py` row of `utils/README.md` and `utils/censuses/taint-census.md`
cite `docs/kernel/inference/monotonicity.md` *at `50739fdb`*, a path at a commit, which git keeps (S1f.20.4's default
6); the other text mentions of `docs/kernel/` in `plans/` and `docs/history/` are records of their time.

**The generated file.** `ein.rs/crates/ein-core/src/printable.rs`'s header named `utils/gen_unicode_printable.py`.
The generator's template now writes *Generated: an edit by hand is lost the next time the file is written.*
(Q-M1f.348), and lays out its test block as `rustfmt` leaves it, so that a run reproduces the committed file:
regenerated, the file differs from the last commit in that one line (`diff`; the generator has no `--check`).
Q-M1f.348's *Held by* names the file; `AGENTS.md`'s generated list and `utils/README.md` name the script.

**Verification**: `python3 utils/doc_audit.py --links --check --tracked`, 0 findings and 0 decision-id findings;
`python3 utils/check_release_voice.py`, 0 lines; `cargo fmt --all --check`; `RUSTDOCFLAGS="-D warnings" cargo doc
--workspace --no-deps`; `python3 utils/review_bundle.py --smoke`, exit 0 and its link check 0; `cargo test --profile
gate`: `ein-ir`'s `grammar_pages`, `ein-core` whole, `ein-cli`'s `gate_steps` and `nightly_steps`;
`python3 utils/decision_census.py --check`. No gate, by the brief.

### The gate on the stage's batch (2026-11-17)

**Green on `d55cae51`**: `./run_tests.sh` exit 0, 163 test binaries, none failed — the six lanes merged, the
release-voice baseline at 0 lines, `fuzz_seeds.txt` re-blessed for 21 seeds' comments, `docs/kernel/` deleted, and
the findings filed. The batch before it (the three perspectives) was green step by step on `5fc616d3` and
`4a097cfb`: a fmt drift and the review bundle's anchors were fixed on the way.
