# S1f.20.1 — The two checks: every link resolves, and the release voice holds

**Phase:** [P1f.20](README.md) (`docs/ein/`: the tree a released system would have)
**Estimate:** 3 days, probably less: the markdown half of the link check exists.
**Status:** **done 2026-11-16** ([§ Record](#record-2026-11-16)). It was `T1f.5.20.1`, *the link checker, first
and out of order*, until 2026-11-08.
**Depends on:** nothing. It runs first, because a 55-page re-tree with 464 inbound code links cannot be done
by hand without a checker.

## What it builds

### The link check, over the whole tree

Three classes of link:
- markdown links, with and without an anchor, in every tracked `.md`;
- repo-relative paths inside Rust doc comments, the `](../../../docs/…)` form the crates used 464 times on
  2026-08-28;
- bare paths in `.ein` header comments and in `corpus.toml`.

It exits 1 on a target that does not exist, or an anchor no heading of the target has, and reports the
referrer.

**The first class exists.** `utils/doc_audit.py --links --check` has been a gate step since M1e S1e.3.8
(`DO-M2`), over `docs`, `plans` and the tree's READMEs. The stage's note of 2026-10-14 put it: *"check_links.py
should be that script's scope, not a second link checker"*. So this stage widens `doc_audit.py` to the other
two classes, and writes no second checker.

**Its baseline is written down before anything moves.** The tree is not known to be clean in the two new
classes. A checker that starts red is fine; a checker whose starting number nobody wrote down is not. The
stage fixes what it finds, as S1e.3.8 fixed its 264, and records the count. The gate then runs the widened
check.

### The release-voice check

`utils/check_release_voice.py` (default name). No file under `docs/ein/`, and no doc comment in a shipped
crate, may contain:
- a milestone, phase, stage or task id: `M1x`, `P1x.y`, `S1x.y.z`, `T…`;
- a path into `plans/`;
- a date that dates a development event (the user's rulings of 2026-11-15).

**A decision id passes** (`Q-M1x.n`): the user's rulings of 2026-11-15 let docs and comments cite a decision,
which [S1f.20.8](s1f.20.8_decisions.md) makes a file under `docs/decisions/`.

By default it also reads `.ein` header comments and `corpus.toml`, which
[S1f.20.6](s1f.20.6_dehistoricise_the_code.md) brings to 0 as well. It is the user's rule (2) turned into a
gate; without it the voice comes back one commit at a time.

- **Its escape.** A line marked `release-voice-ok:` passes, as `check_hashmap_iteration.py` lets
  `determinism-ok:` pass. Expect its first finding to be the check itself: that is what happened to
  `check_hashmap_iteration.py`.
- **Its exemption**, narrowed by the user's rulings of 2026-11-15 (*"code+comment+docs … must be closed and not
  connected to plans"*): `AGENTS.md` (and so `CLAUDE.md`) and `README.md` are held to it too, but for
  `AGENTS.md`'s § Where the work is, which points at the current milestone's plan.
- **It is a ratchet** ([P1f.20 § Two rules](README.md#two-rules-across-the-stages)). It reads a banked
  baseline, each file's count (default `utils/release_voice_baseline.txt`). It fails where a count rises or a
  new file appears. Each later stage lowers the baseline in the commit that lowers a count, and
  [S1f.20.9](s1f.20.9_the_history_sweep.md) empties it and deletes it.

### The census

The phase's [§ Context](README.md#context--the-size-of-it) table is of `9ba2349`, 2026-08-28, and its counting
rule was not written down. This stage re-takes it with a script, so each number carries its command. The
per-file count of history-referencing lines is the release-voice check's first baseline.

## The method

### Task T1f.20.1.1 — the census

The table re-taken, the script banked under `utils/`, and the commands written into the phase README beside
the old numbers.

### Task T1f.20.1.2 — the link check widened

`doc_audit.py` reads Rust doc comments, `.ein` header comments and `corpus.toml`. Its baseline is recorded,
what it finds is fixed, and the gate step's scope widens in `run_tests.sh` and `per-commit.yml`, which
`ein-cli/tests/gate_steps.rs` holds together.

### Task T1f.20.1.3 — the release-voice check

The script, its escape, its exemptions and its baseline. A step in `run_tests.sh` and `per-commit.yml`, and
a line in [`utils/README.md`](../../../utils/README.md).

## Acceptance

- The link check reads every tracked `.md`, every repo-relative path in a Rust doc comment, and the bare paths
  in `.ein` headers and `corpus.toml`. It is in the gate, and green.
- The release-voice check is in the gate, green against its banked baseline, and fails on a count that rises.
- The census is re-taken by a script, with its commands written down.
- `./run_tests.sh` is green.

## Record 2026-11-16

Run in the main working copy on `master`, from `8b237c39`. Commits: `df24feac` (T1f.20.1.2, the link check
widened), `0667ecda` (T1f.20.1.2 continued, a reference-style definition read as a link), `f5e5a644`
(T1f.20.1.3, the release-voice check), `40644745` (T1f.20.1.1, the census), and this record's.

### T1f.20.1.2 — the link check widened

`utils/doc_audit.py --links --check --tracked` reads every file `git ls-files` lists of four kinds, and no second
checker exists:

- **`.md`**: every markdown link, and a reference-style definition (`[label]: target`), as before;
- **`.rs`**: the markdown links of every full-line comment (`//`, `///`, `//!`) and their reference-style
  definitions, relative to the source file, a fenced block blanked; a `#`-only target is rustdoc's and is not
  read; `ein.rs/vendor/` is not read (its links are rustdoc's, relative to the rendered HTML);
- **`.ein`**: the bare paths of every comment, the text after a `;` outside a string;
- **`corpus/corpus.toml`**: the bare paths of the whole file.

A **bare path** starts at a top-level directory (`docs/…`, `examples/…`) or at a crate's directory, which the repo
writes short for `ein.rs/crates/<crate>/` (`ein-cli/tests/gate_steps.rs`), and names a file with an extension or a
directory with a trailing `/`; a token with `*`, `<`, `>`, `{` or `}` is a pattern. A missing one git would ignore
(`ein.rs/target/…`, `build/…`) names an output, not a file of the tree, so it passes here and in CI alike. A
directory argument still walks `.md` pages only, so the review bundle's smoke and the dev host's bundle step read
what they read before.

**`--inbound --tracked [--target PREFIX …]`** prints, per target file (a directory with a trailing `/`), the
references that resolve to it: `all`, `md` (with `plans` and `hist` counting again those from `plans/` and
`docs/history/`), `rs`, `ein`, `toml`; a page's references to itself are not counted. That is the count
[S1f.20.2](s1f.20.2_the_disposition_table.md) takes for each source page:

```sh
python3 utils/doc_audit.py --inbound --tracked --target docs/kernel/ --target docs/history/
```

On this tree, for instance, `docs/kernel/defined_behaviour.md` has 219 (210 `.md`, 87 of them from `plans/` and
38 from `docs/history/`; 9 `.rs`), and the crates' comments hold 273 links into `docs/history/`.

**Baseline, before fixing** (`python3 utils/doc_audit.py --links --check --tracked` on the tree of `8b237c39`
with the widened script): **1 959 files read (611 `.md`, 350 `.rs`, 997 `.ein`, 1 `.toml`), 15 findings**, none in
a `.md` (the old scope, and the `.md` files it had not read, were clean):

| referrer | finding | fix |
|---|---|---|
| `ein-cli/tests/leftover_probe.rs:4` | `../../ein-infer/tests/obligation_rung_control.rs`, deleted with the walk (`f8c0cf53`) | made plain: the sentence states the reason without the file |
| `ein-infer/tests/naf_semantics.rs:1047` | `#s1a34--the-saturator`, a heading M1a's record has as S1a.3.3; the label says S1a.3.4 | the anchor of the label's heading, `#s1a34--the-naf-boundary` |
| `ein-render/src/dump/region_lattice.rs:2` | a `plans/` link one `../` short | the depth |
| `corpus.toml:943`, `examples/ein-bugs/closed-and-owing.ein:8`, `…/wrong-answers/owed-two-step.ein:14`, `…/wrong-answers/tree-dead.ein:6` | `tests/stdlib/closure/03_closed_and_owing.ein`, which became the refusal fixture `closed-and-owing.ein` and no longer banks what they cite it for | plain: *the stdlib test `03_closed_and_owing.ein`* |
| `corpus.toml:3511`, `:3552` | `examples/binary_arithmetic/conv.ein`, `examples/binary_logic/props_rules.ein`, moved into the stdlib | plain: *the binary-arithmetic examples' `conv.ein`*, and the logic one's |
| `stdlib/math/decimal-arith.ein:187` | `examples/binary_arithmetic/conv.ein`, cited for the walk that doubles and adds a bit | `std.math.convert`, which does it today (`(add-walk (doubling …) ?acc ?acc ?bit z)`); manifest re-written |
| `examples/ein-bugs/fact-goal-binding.ein:12`, `int-goal-binding.ein:17` | `ein-render/src/dump/answer.rs`, deleted with the walk | plain: *the JSON summary*, whose behaviour the corpus entry holds |
| `examples/ein-bugs/minimal-line-stated-obligation{,-control}.ein:3` | `plans/m1e2_correctness/inventory_2026-11-06/findings/`, gone | plain: *that inventory's findings* |
| `examples/syntax/config.ein:6` | `examples/broken/load/config_unknown_flag.ein`, moved (`8c3fe86e`) | `examples/broken/config_unknown_flag.ein` |

**After: 0.** Reading reference-style definitions added none: 24 are links by the rule, all resolving. The gate
step is `python3 utils/doc_audit.py --links --check --tracked` in `run_tests.sh` and `per-commit.yml` alike
(`gate_steps` green). `docs/kernel/README.md` § Keeping this true, `utils/README.md` and AGENTS.md (a rule in
§ Edits that go together, a command) say so. Probed with scratch files: a missing `.ein` comment path, a missing
crate-relative path and a Rust link out of the repo are findings; an ignored output, a prefix, a path in a string,
a fenced link, a link in code and an intra-doc link are not.

### T1f.20.1.3 — the release-voice check

`utils/check_release_voice.py`, its escape `release-voice-ok: <reason>` (same line, a reason required), and its
baseline `utils/release_voice_baseline.txt` (`count  path`, sorted, written by `--bless`). Scope and patterns as
the stage file states them, with the readings [Q-M1f.314](../open_questions.md#q-m1f314--what-the-release-voice-check-reads-and-what-fails-a-line)
records: every `///`/`//!` line of the six shipped crates (tests, examples, benches included), every comment of a
tracked `.ein` outside `plans/` and `docs/history/`, `corpus.toml` whole, `AGENTS.md` without § Where the work is,
`README.md`, `docs/ein/**/*.md` (none yet), and the script itself. A line fails on a milestone, phase, stage or
task id (letter forms too, a `Q-M…` decision id passing by its `-`), a path into `plans/` (the folder named alone
passes), or an ISO date; a line counts once.

**Its first findings were itself**: its docstring's examples of ids and of a `plans/` path, which carry the escape.

**The first baseline** (`python3 utils/check_release_voice.py --bless`, on the tree of `f5e5a644`): **6 743 lines in
1 041 files** of the 1 227 read — shipped crates' doc comments 3 271, `.ein` comments 2 501, `corpus.toml` 765,
`README.md` 152, `AGENTS.md` 54. By kind of match over all lines (`--list`): stage ids 5 407, milestone ids 3 605,
dates 1 748, task ids 1 123, phase ids 393, paths into `plans/` 295 (a line may hold several). The one false
positive seen in a sample of the rarer ids is `stdlib/math/decimal-arith.ein:621`'s `(multiples M1 … M9)`, a
variable list, which S1f.20.6 escapes or rewords.

**The ratchet**: exit 1 where a file's count rises or a file not in the baseline has one, naming each with its
`--list` command; a fall is reported and exits 0 (Q-M1f.314, 6). Probed: a stage id appended to a doc comment of
`ein-core/src/lib.rs` fails (4 against 3), the same line with the escape passes, a new `.ein` with one fails. A step
`utils/check_release_voice.py` after the link check in `run_tests.sh` and `per-commit.yml` (its `shared-assets`
job); `gate_steps.rs`'s guarded set and the static-check count (six to seven) in `run_tests.sh`'s header,
AGENTS.md and README.md; a row in `utils/README.md`; AGENTS.md's rule and command.

### T1f.20.1.1 — the census

`utils/history_census.py`, its rules in its docstring and in [the phase README's re-taken table](README.md#re-taken-2026-11-16-by-a-script),
which carries the table beside the 2026-08-28 one: `python3 utils/history_census.py` on the tree of `40644745`.
**9 397 lines across 1 101 files carry a milestone, phase, stage or task id** outside `docs/history` (2 034 across
330 then, by a rule not written down); the crates' comments hold **273 links into `docs/history/`** (197 into
`m1a_rust`, 84 of them into `design/`; 37 `m1e_review_processing`, 22 `m1d_satisfiability`, 17
`m1c_external_validation`) and **142 into `plans/`**; **no tracked name** outside `plans/` and `docs/history/`
carries an id, so S1f.20.9's T1f.20.9.3 has nothing to rename by that rule.

### Defaults taken

- **The link check's classes**: every Rust comment line's links, not only doc comments' (six links were in plain
  `//` comments); every `.ein` comment, not only the header; the crate-relative short form read as a path; a bare
  path only with an extension or a trailing `/` (so `tests/stdlib/typing/03`, a prefix, and `ein-corpus/2`, a
  schema, are not read); a git-ignored target skipped; `ein.rs/vendor/` not read; a directory argument walks `.md`
  only. None is the user's: each is how the stage's three classes are read mechanically.
- **The release-voice check's readings**: Q-M1f.314, *defaulted, to be shown at S1f.20.2's review*.
- **The census' rules**: the docstring's; files are the row's kind (`.md` for a docs tree), and the id column counts
  whole lines of a `.rs` file, code and strings too, so it is wider than the check's scope.

### Findings outside the stage

- **Bare paths outside link syntax are read only in `.ein` comments and `corpus.toml`.** In a `.md` (backticked or
  plain) and in a Rust comment they are not, and a page that moves leaves them behind. Outside `plans/` and
  `docs/history/`, five `.md` ones name nothing today: `docs/kernel/README.md:44` `docs/ir.md`;
  `examples/README.md:43`, `:47`, `:53` (`plans/m1_core_graph_reasoning/…`, `docs/history/m1_core/`);
  `utils/README.md:85` `plans/m1e2_correctness/probes/region.py`. The crates' comments hold 53 bare `docs/…` or
  `plans/…` paths, one of them gone (`ein-infer/tests/refutation_under_absent.rs:47`, a deliberate *the original
  was*). `plans/` holds 98 such, `docs/history/` 11. S1f.20.3 to .5 rewrite these by grep when they move a page.
- **`.ein` strings carry the development too**, which neither check reads: `stdlib/elim.ein`'s `:why "M1-blocking
  stdlib"` and the dates in `examples/lattice/synth/*.ein`'s texts (Q-M1f.314, 5).
- **`utils/README.md`'s count of scripts that do not drive `ein.rs`** names eight; `check_hashmap_iteration.py`,
  `stdlib_manifest.py` and `check_test_scratch.py` do not drive it either.

### Verification

`python3 utils/doc_audit.py --links --check --tracked` (0 findings), `python3 utils/check_release_voice.py` (green
against its baseline), `cargo test --profile gate -p ein-cli --test gate_steps` (3 passed) at each of the three
task commits, and `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` after the doc-comment fixes. The
gate itself is the coordinator's, after the batch.

