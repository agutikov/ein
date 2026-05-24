# P1.11 — Package + CLI restructure

**Estimate:** TBD (~1-2 days mechanical work + cross-ref update).
**Status:** **placeholder** — created 2026-05-24 from the
TODO.md scratchpad. Pure housekeeping; does not gate M1
acceptance, but touches every import in the package so it's best
scheduled at a quiescent point (between M1's P1.7 ship and the
start of M2).
**Depends on:** [P1.7](../p1.7_bootstrapping_zebra/) ships — the
zebra E2E test gives the regression baseline that the rename
must preserve.
**Blocks:** nothing on M1's critical path; informs M2 (NL → IR)
by giving it a cleaner package surface to extend.

## Goal

Three pieces of housekeeping that the M1 implementation
accumulated:

1. **Package rename:** `ein-bot` / `ein_bot` → `ein`. The
   project's user-facing name is "ein"; "ein-bot" was the repo
   placeholder. Bring the Python package + CLI binary into line.
2. **Demo merge:** the `ein.py/demo/` scratchpad
   (`bench_saturate.py`, `bench_solve.py`) becomes part of the
   `ein` package proper — either under
   `src/ein/bench/` or `src/ein/cli/bench.py`. Today they sit
   outside the package's source root and are runnable only via
   `python ein.py/demo/bench_solve.py`.
3. **CLI split:** the single `src/ein_bot/cli.py` becomes a
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

## Likely stages

- **S1.11.1** — directory rename: `git mv src/ein_bot src/ein`,
  `git mv ein.py ein` (or rename the outer working dir entirely),
  update `pyproject.toml` (`name`, `[tool.*]` entries,
  `[project.scripts]`).
- **S1.11.2** — codebase rename: search-and-replace
  `ein_bot` → `ein` across all imports, type hints, docstrings,
  CLI examples.
- **S1.11.3** — demo merge: move `demo/bench_*.py` into
  `src/ein/cli/bench_*.py` (or `src/ein/bench/`); expose as CLI
  subcommands `ein bench solve …`, `ein bench saturate …`.
- **S1.11.4** — CLI split: refactor `cli.py` into
  `src/ein/cli/__init__.py` (top-level dispatcher) + one module
  per subcommand. Reuse the click / argparse setup the existing
  CLI uses.
- **S1.11.5** — docs/plans/examples sweep: update every
  `ein_bot` / `ein.py/demo/` reference under `docs/`,
  `examples/`, `plans/`, `README.md`, `CLAUDE.md`.
- **S1.11.6** — acceptance: full test suite + ruff green; M1
  acceptance reruns (zebra solves, traces match, demos run).

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
- [P1.10 Theme G](../p1.10_kernel_docs/README.md) — sibling
  housekeeping (rename `docs/index/` → `docs/lib/`); same
  scheduling concerns (do at a quiescent point).
- [`pyproject.toml`](../../../ein.py/pyproject.toml) — needs
  the `name`, scripts, and tool configs updated.
- The user's TODO.md historically called this "P1.6" — that label
  collided with [P1.6 rendering + trace](../p1.6_rendering_and_trace/README.md),
  so the phase was numbered P1.11 on intake (the next free M1 slot
  after P1.10 kernel-docs).
