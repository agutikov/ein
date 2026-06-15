# P1.11 — Package + CLI restructure

**Estimate:** TBD (~1-2 days mechanical work + cross-ref update).
**Status:** **DONE 2026-06-16** — rename shipped 2026-06-15 (commit
`e49a325`: S1.11.1 package dir + S1.11.2 codebase + S1.11.5
docs/plans/examples sweep; `ein_bot` → `ein`, `ein-bot` → `Ein`/`ein`
by context). The CLI-split (S1.11.4) + demo-cleanup (S1.11.3) + final
acceptance (S1.11.6) landed 2026-06-16: `cli.py` → a `cli/` package; the
former `demo/` (13 scripts, not the 2 the plan assumed) split into 5
promoted `ein` subcommands (`saturate` / `search` / `lattice` / `profile`
/ `symmetric`) and 8 one-off probe/measure scripts relocated to repo-root
`utils/`. **Still deferred:** the outer `ein.py/` dir rename
(Q-S1.11.1.A — see [open questions](#open-questions)). Created 2026-05-24
from the TODO.md scratchpad. Pure housekeeping; does not gate M1
acceptance, but touched every import in the package so it was scheduled at
a quiescent point (between M1's P1.7 ship and the start of M2).
**Depends on:** [P1.7](../p1.7_bootstrapping_zebra/) ships — the
zebra E2E test gives the regression baseline that the rename
must preserve.
**Blocks:** nothing on M1's critical path; informs M2 (NL → IR)
by giving it a cleaner package surface to extend.

## Goal

Three pieces of housekeeping that the M1 implementation
accumulated:

1. **Package rename:** `ein-bot` / `ein_bot` → `ein`. The
   project's user-facing name is "ein" / "Ein"; "ein-bot" was the
   repo placeholder. Bring the Python package + CLI binary into
   line. **DONE 2026-06-15** (commit `e49a325`): package dir
   `src/ein_bot/` → `src/ein/`, every import, `pyproject` `name` +
   `[project.scripts]` (`ein-bot` → `ein`), repo URL, and prose
   (`Ein` proper-noun / `ein` command). The *outer* `ein.py/`
   directory was left as-is (Q-S1.11.1.A).
2. **Demo merge:** the `ein.py/demo/` scratchpad
   (`bench_saturate.py`, `bench_solve.py`) becomes part of the
   `ein` package proper — either under
   `src/ein/bench/` or `src/ein/cli/bench.py`. Today they sit
   outside the package's source root and are runnable only via
   `python ein.py/demo/bench_solve.py`.
3. **CLI split:** the single `src/ein/cli.py` becomes a
   folder `src/ein/cli/` with per-subcommand modules (`solve.py`,
   `saturate.py`, `query.py`, `dump.py`, etc.). Today everything
   piles onto one CLI module; subcommands proliferate as M1's
   features ship and the single-file approach won't scale.

## Why deferred

The work is mechanical (no design questions) but high-churn
(every `from ein_bot.…` import becomes `from ein.…`, every
`ein.py/demo/*` reference shifts, every doc cross-link that
hard-codes `ein_bot` or the demo path updates). Best done at a
quiescent point — between milestones, with a green test suite
on both sides of the rename.

## Stages

| ID       | Title                                                 | Status                          | File                                                                   |
|----------|-------------------------------------------------------|---------------------------------|------------------------------------------------------------------------|
| S1.11.1  | Directory rename                                      | ✅ pkg dir done; outer `ein.py/` deferred | [s1.11.1_directory_rename.md](s1.11.1_directory_rename.md)            |
| S1.11.2  | Codebase rename                                       | ✅ DONE 2026-06-15              | [s1.11.2_codebase_rename.md](s1.11.2_codebase_rename.md)              |
| S1.11.3  | Demo merge                                            | ✅ DONE 2026-06-16 (5→cli, 8→utils) | [s1.11.3_demo_merge.md](s1.11.3_demo_merge.md)                  |
| S1.11.4  | CLI split                                             | ✅ DONE 2026-06-16              | [s1.11.4_cli_split.md](s1.11.4_cli_split.md)                          |
| S1.11.5  | Docs / plans / examples sweep                         | ✅ DONE 2026-06-15              | [s1.11.5_docs_sweep.md](s1.11.5_docs_sweep.md)                        |
| S1.11.6  | Acceptance                                            | ✅ DONE 2026-06-16 (1287+8 green, ruff clean) | [s1.11.6_acceptance.md](s1.11.6_acceptance.md)      |

S1.11.1 → S1.11.2 → (S1.11.3 ∥ S1.11.4) → S1.11.5 → S1.11.6.
The rename spine (S1.11.1 package dir → S1.11.2 → S1.11.5) landed
in one commit; S1.11.3/S1.11.4 + the outer-dir rename remain.

## Out of scope

- Public package release / PyPI publication — the rename
  prepares for this but doesn't commit to it.
- Multi-package split (e.g. `ein-core` + `ein-cli` + `ein-bench`)
  — pre-mature; revisit if the package grows past the one-package
  shape.
- API stabilisation — that's a separate concern; this phase only
  renames, it doesn't redesign.

## Acceptance

- `from ein.ir import parser` (or whatever the new import path
  is) works; `from ein_bot.ir import parser` does not.
- `ein solve examples/zebra2.ein` works (renamed CLI entry).
- `ein bench solve examples/zebra2.ein` works (merged-demo CLI
  subcommand).
- `python ein.py/demo/bench_solve.py` is gone; readers who
  hit a stale doc reference see the new command in the
  README / CLI help.
- Pytest suite + ruff green; zebra E2E (M1 acceptance #2/#3)
  passes against the renamed package.

## Open questions

- **Outer-dir rename — `ein.py/` → `ein/`?** The `ein.py`
  directory name is also a holdover from when the repo's only
  Python artefact was a single file. Renaming the directory
  costs another wave of doc cross-link churn; the alternative
  is to leave the dir name and rename only the package inside.
  Pick at S1.11.1 time.
- **CLI subcommand surface.** What subcommands does `ein` expose?
  `solve`, `saturate`, `dump`, `query`, `bench solve`, `bench
  saturate` are obvious; `validate`, `repl`, `trace` are possible.
  Worth a short design pass at S1.11.4 time.

## Cross-links

- `ein.py/src/ein_bot/` — the package source affected by
  the rename.
- `ein.py/demo/` — the scratchpad that gets merged into
  the package.
- [`CLAUDE.md`](../../../CLAUDE.md) — needs an update once the
  package name changes (the agent guide references `ein.py/` and
  `ein_bot/` paths).
- [P1.20 Theme G](../p1.20_kernel_docs/README.md) — sibling
  housekeeping (rename `docs/index/` → `docs/lib/`); same
  scheduling concerns (do at a quiescent point).
- [`pyproject.toml`](../../../ein.py/pyproject.toml) — needs
  the `name`, scripts, and tool configs updated.
- The user's TODO.md historically called this "P1.6" — that label
  collided with [P1.6 rendering + trace](../p1.6_rendering_and_trace/README.md),
  so the phase was numbered P1.11 on intake (the next free M1 slot
  after the then-P1.10 kernel-docs, since renumbered to P1.20 so docs
  sort last).
