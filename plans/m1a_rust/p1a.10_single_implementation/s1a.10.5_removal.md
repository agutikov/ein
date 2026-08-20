# S1a.10.5 — The removal

**Phase:** P1a.10 (One implementation)
**Estimate:** 1 day
**Depends on:** [S1a.10.2](s1a.10.2_port_the_suite.md),
[S1a.10.4](s1a.10.4_utils.md)

## Context

One day, because by now it is `git rm` and the interesting work is behind it.
It is listed as its own stage precisely so it *cannot* start early: a delete
that happens before S1a.10.1's ledger is complete is the phase's one
unrecoverable mistake.

## Acceptance

- `ein.py/` gone. `.venv`, `.venv-pypy`, `pyproject.toml` fragments,
  `run_tests.sh`, `pytest.ini`/`conftest.py` and every Python packaging file
  outside `utils/` gone.
- `nlp/` and `smt/` gone, **as submodules** — `git submodule deinit -f`, the
  `.gitmodules` entries removed, `.git/modules/` cleaned. Both are 24 KB of
  scratch pointing at `opencog/link-grammar` and `CVC4/CVC4`; nothing in the
  active tree imports either, and `CLAUDE.md` already describes them as "not
  wired into the active `ein.py/` package".
- `.gitignore` loses the entries that named the removed trees, and keeps the
  ones that still describe something.
- CI (whatever runs the gate) runs `cargo test --workspace` and nothing else.
- **One commit**, or one commit per removed tree — but not interleaved with a
  behaviour change. The diff should be reviewable as "these things left".

## Tasks

### Task T1a.10.5.1 — The submodules

Distinct from a directory delete and easy to do half of. `git submodule
deinit -f -- nlp/link-grammar smt/CVC4`, remove the `.gitmodules` stanzas,
`git rm` the paths, and check `.git/config` and `.git/modules/` are clean —
a stale entry there makes a fresh clone fail in a way that looks unrelated.

### Task T1a.10.5.2 — The Python tree

`ein.py/` including its tests, its acceptance directory, its build config and
its stdlib copy.

### Task T1a.10.5.3 — The runner

`run_tests.sh` has three phases; two of them are about to have no subject.
Either delete it and let `cargo test --workspace` be the documented gate, or
keep the name as a one-line wrapper. **Recommendation: keep the name.** It is
in `CLAUDE.md`, in the user's own habits and in a dozen plan documents' "Gate:"
lines, and a script that still works is cheaper than re-teaching the habit.

### Task T1a.10.5.4 — The sweep

`git grep` for the removed names across `docs/`, `plans/`, `README`s and
`CLAUDE.md`. Distinguish **live references** (fix) from **historical record**
(leave, and the divergence ledger is all historical record). A plan that says
"measured under PyPy 3.11" is a fact about the past and stays.

## Notes

- Tag or note the commit before the delete. This is the last revision where
  the two implementations can be diffed against each other, and someone will
  want it.
