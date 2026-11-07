# S1f.5.17 — CI green again: the per-commit job on Ubuntu 26.04's Graphviz, and the nightly's mutant baseline

**Phase:** [P1f.5](README.md) (The no-owner items, then NL and `:priority`)
**Estimate:** 1 day.
- Half a day to reproduce the per-commit failure on the runner's Graphviz, find what in a view sets it off,
  and fix it.
- A quarter of a day for the mutant baseline: which commit killed the two mutants, and the baseline
  re-banked.
- A quarter of a day to run both tiers on the runner's image, and the record.

**Status:** specified 2026-11-08 on the user's instruction: *"add new stage to this phase - fix failing CI
jobs [agutikov/ein] Run failed: per-commit - master (ec004a4)"*. Not started.
**Depends on:** nothing. **It runs next after [S1f.5.0](s1f.5.0_the_known_wrong_answers_filed.md)**,
before any other stage. Each later stage boundary can then be held to a green CI as well as a green
`./run_tests.sh`.

## The instruction

The per-commit workflow is red on `master` at `ec004a48`, and it was red at `916d97ac`. The nightly has been
red since 2026-11-06. Nothing local shows either failure: `./run_tests.sh` was green at both commits.

## The case

Every run since 2026-11-03, read with `gh run view <id>` and `gh run view <id> --log-failed`, 2026-11-08.
The logs are banked under `build/ci/`, which is gitignored and re-fetched by the same commands:

| run | workflow | commit | image | `graphviz` | result |
|---|---|---|---|---|---|
| 41707292256 | per-commit | `3ee7796a` | ubuntu-24.04 | 2.42.2-9ubuntu0.1 | green |
| 42011053976 | nightly | `3ee7796a` | ubuntu-24.04 | 2.42.2-9ubuntu0.1 | green |
| 42193521554 | per-commit | `916d97ac` | **ubuntu-26.04** | **14.1.2-1ubuntu1** | red: `dot_wellformed` |
| 42203265579 | nightly | `916d97ac` | **ubuntu-26.04** | | red: `stdlib_mutants.py --check` |
| 42360943742 | nightly | `916d97ac` | **ubuntu-26.04** | **14.1.2-1ubuntu1** | red: `stdlib_mutants.py --check` |
| 42412840680 | per-commit | `ec004a48` | **ubuntu-26.04** | **14.1.2-1ubuntu1** | red: `dot_wellformed` |

**The runner moved.** Every workflow asks for `runs-on: ubuntu-latest`. The runs carry GitHub's annotation:
*"The ubuntu-latest label will migrate to Ubuntu 26 beginning October 19, 2026"*. The label reached this
repository between the green runs of 2026-11-05 and the red ones of 2026-11-06. Every red run is on 26.04,
and every green one on 24.04. Locally `dot -V` prints `graphviz version 16.0.0`, and the gate is green.
So three Graphviz versions now meet these views, and only 14.1.2 refuses them.

### Per-commit: `dot_wellformed` on Graphviz 14.1.2

`ein-render/tests/dot_wellformed.rs`'s `graphviz_accepts_every_view_of_every_corpus_file` fails on 16 views
of 9 files:
- `ir-overlay` and `rules-overlay` of the zebra2 family: `zebra2`, `zebra2-hints`, `zebra2-minus-15`,
  `zebra2-minus-15-obligations`, `zebra2-obligations`, `ein-bugs/zebra2-bad` and `ein-bugs/zebra2-hints`;
- `ir-levi` of W18's two witnesses, `ein-bugs/wrong-answers/str-in-absent.ein` and `str-in-guard.ein`.

On each one `dot` exits 0 and writes `lib/pathplan/shortest.c:333: triangulation failed` and
`shortest.c:203: cannot find triangle path` on stderr. The test reads stderr as its signal, because
*"Graphviz exits 0 on a warning"* (`dot_wellformed.rs`, `fn graphviz`). The message is a failure in Graphviz's
spline router, not a parse error, so the question is what in these views the router cannot route on 14.1.2.

**Three steps never ran** in either red per-commit run. `cargo test` failed first, so `check_test_scratch.py`,
`review_bundle.py --smoke` and the bench smoke were skipped, and a second failure could be hiding behind the
first.

### Nightly: two mutants killed since the baseline

`utils/stdlib_mutants.py --check` holds `tests/mutation_survivors.txt` in both directions. A new survivor
fails it, and so does a banked survivor that is now killed. Both red nightlies report the same two:

```
bijection.ein::bijective-setup::swap-assert::(surjective ?R ?isa)   <- killed since the baseline
bijection.ein::bijective-setup::swap-assert::(total ?R ?isa)        <- killed since the baseline
```

Every other step of the `deep-corpus` job is green, and the mutant step is the job's last, so nothing is
hidden behind it. A killed survivor means the suite got stronger. The open question is where: a commit in
`3ee7796a..916d97ac` (M1e2's S1e2.2.21 to S1e2.3.10), or the platform.

## What it changes

- **The views or the test, not the runner by default.** The fix makes `dot_wellformed` hold on the runner's
  Graphviz 14.1.2, on 2.42.2, and on the local 16.0.0. Two fixes are in reach, and the stage prefers them in
  this order:
  1. **A view-side fix.** Some attribute or structure of the overlay and Levi views makes 14.1.2's router
     fail, for example an edge between two nodes of a cluster, a `splines` setting, or a self-loop with a
     port. If such a fix exists, the view stops writing that thing. A change to a view's output moves its
     goldens, and they are named before they move.
  2. **The test's reading of stderr.** If no view-side fix exists, the test tells a parse error (malformed
     DOT) from a layout warning that Graphviz's own router raises. This changes what the test asserts, so it
     goes to the user first (below).

  `runs-on: ubuntu-24.04` would be a stopgap that hides the problem until 24.04 is retired. It is not taken
  without the user.
- **The mutant baseline re-banked** with `utils/stdlib_mutants.py --bless`, once the stage has found which
  commit killed the two mutants and why. The two lines leave `tests/mutation_survivors.txt`, and
  `tests/README.md`'s mutation count moves with them, re-taken with its command and date.
- **Whatever the skipped steps show**, under the same rule: a failure there is fixed in this stage, or it
  becomes a question if the fix is a choice.

## Questions it may put to the user

- If the per-commit fix has to change what `dot_wellformed` asserts (option 2 above): which warnings, if any,
  may a view raise on a Graphviz the gate meets? The stage brings the 16 views, the warning each raises, the
  version that raises it, and the rendered SVG from each version.
- If either failure depends on the platform rather than the code: does the CI pin an image, or does the gate
  hold on every image `ubuntu-latest` names?

## The method

### Task T1f.5.17.1 — reproduced on the runner's image, and the cause

Run both tiers in an `ubuntu:26.04` container (`docker` is on this machine), with the workflow's own steps:
`apt-get install graphviz`, the pinned toolchain, and `./run_tests.sh` and `./run_tests.sh --nightly`, or
their steps one by one. Use a target directory under `build/`, not `ein.rs/target`. Record the versions.
Then:
- `dot_wellformed`: reproduce the 16, minimise one view to the smallest DOT that 14.1.2 refuses and 16.0.0
  accepts, and name the attribute or structure that triggers it. Run the same DOT on 2.42.2 too (a
  `ubuntu:24.04` container).
- The mutants: run `stdlib_mutants.py --check` on the container and on the host, at HEAD and at `3ee7796a`.
  Bisect `3ee7796a..916d97ac` for the commit that kills the two, or show that only the platform does.
- The steps the red runs skipped: run each one on 26.04.

### Task T1f.5.17.2 — the fixes

The view-side fix with its goldens named before they move, or the question to the user. The baseline
re-banked, and the commit that killed the two named in the commit message and in `tests/README.md`.
`./run_tests.sh` green on the host, and both tiers green in the 26.04 container.

### Task T1f.5.17.3 — the record

The stage's Status and the phase README's row. If the cause is the image, the gate's section of `AGENTS.md`
gets a sentence: the CI runs on whatever `ubuntu-latest` names, and Graphviz's version is part of what
`dot_wellformed` checks. If the stage's container commands are worth keeping, they go into `utils/` as a
script with a line in `utils/README.md`, so a later image move can be checked before a push.

## Acceptance

- `dot_wellformed` passes on Graphviz 14.1.2, 2.42.2 and 16.0.0, and no view's rendering on 16.0.0 got worse.
  Or the user has ruled on what the test asserts, and it holds what they ruled.
- `stdlib_mutants.py --check` passes on the host and on 26.04, and the commit that killed the two mutants is
  named.
- Every step of `per-commit.yml` and `nightly.yml` is green in an `ubuntu:26.04` container, including the
  three the red runs skipped.
- `./run_tests.sh` is green.
- The CI itself is read when the user pushes. The stage does not push.
