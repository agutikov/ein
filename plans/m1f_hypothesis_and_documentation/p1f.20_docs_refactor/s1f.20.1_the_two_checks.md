# S1f.20.1 — The two checks: every link resolves, and the release voice holds

**Phase:** [P1f.20](README.md) (`docs/ein/`: the tree a released system would have)
**Estimate:** 3 days, probably less: the markdown half of the link check exists.
**Status:** not started. It was `T1f.5.20.1`, *the link checker, first and out of order*, until 2026-11-08.
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
- a milestone, phase, stage, task or question id: `M1x`, `P1x.y`, `S1x.y.z`, `T…`, `Q-M1x.n`;
- a path into `plans/`.

By default it also reads `.ein` header comments and `corpus.toml`, which
[S1f.20.6](s1f.20.6_dehistoricise_the_code.md) brings to 0 as well. It is the user's rule (2) turned into a
gate; without it the voice comes back one commit at a time.

- **Its escape.** A line marked `release-voice-ok:` passes, as `check_hashmap_iteration.py` lets
  `determinism-ok:` pass. Expect its first finding to be the check itself: that is what happened to
  `check_hashmap_iteration.py`.
- **Its exemptions.** `AGENTS.md` (and so `CLAUDE.md`) and `README.md` are addressed to someone working *on*
  the repo, not to someone using the released system, and the user's rule is about code, tests and docs.
- **It is a ratchet** ([P1f.20 § Two rules](README.md#two-rules-across-the-stages)). It reads a banked
  baseline, each file's count (default `utils/release_voice_baseline.txt`). It fails where a count rises or a
  new file appears. Each later stage lowers the baseline in the commit that lowers a count, and S1f.20.6
  empties it and deletes it.

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
