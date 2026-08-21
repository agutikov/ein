# S1a.10.5 — The removal

**Phase:** P1a.10 (One implementation)
**Estimate:** 1 day
**Depends on:** [S1a.10.2](s1a.10.2_port_the_suite.md),
[S1a.10.4](s1a.10.4_utils.md), and **T1a.10.5.0 below** — which is a
precondition, not a step of the removal.

> **The phase's dependency on [P1a.9](../p1a.9_bindings_release/README.md) is
> reversed, 2026-08-21.** It was hard — "if P1a.9 has not landed, this phase
> deletes the only implementation of a documented contract" — and the answer
> is that P1a.9 now runs *after* this phase and releases ein.rs alone. The
> cost is one interval in which `docs/api/` documents an unimplemented
> surface; [S1a.10.6](s1a.10.6_docs.md) states it on the pages and
> [S1a.9.4](../p1a.9_bindings_release/s1a.9.4_documentation.md) closes it.
> The full argument is in [the phase README](README.md).

## Context

> **Two of the defect list's rows are already closed.**
> [S1a.10.2](s1a.10.2_port_the_suite.md) `git mv`-ed the nineteen goldens a
> stage early, and [S1a.10.3](s1a.10.3_corpus_without_an_oracle.md) closed the
> last one — `corpus.rs::tracked`'s fallback scan of `ein.py/src/ein/stdlib`,
> which would have turned "the stdlib directory is gone" into "seven fewer
> files to check". The completeness check names `stdlib/` and nothing else.
>
> Also already done: **CI no longer runs the harness** (per-commit's
> `conformance-fast`, nightly's `hash-seed-sweep` and `conformance-full` are
> gone), so what is left for T1a.10.5.3 is the `oracle`, `full-suite` and
> `packaging` jobs, which are ein.py's own. `ein.py/tests/test_corpus_manifest.py`
> was re-pointed rather than deleted; its nine claims are all owned by
> `ein_corpus::manifest`, so it goes with the tree and banks nothing.

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

### Task T1a.10.5.0 — The grammar — ✅ **done 2026-08-21**, `7f10199`

> *before deleteion convert lark grammar into EBNF and preserve in docs for
> future use* — the user's own precondition, promoted from a floating line to
> the task it always was, and done first because it is the one loss in this
> stage a revert would not casually undo.

`ein.py/src/ein/ir/grammar.lark` was 244 lines of Lark and
`docs/kernel/ir/03-ein-lang/01_grammar.md` opened by calling it **the source
of truth for syntax**. What would be left after the delete is a
recursive-descent parser — an implementation, not a specification.

It is [§3 of that document](../../../docs/kernel/ir/03-ein-lang/01_grammar.md)
now: W3C EBNF, lexical and phrase layers, plus three things the Lark carried
in header comments and would have lost with the file — why the two layers are
*not* separated by a scanner pass, what the grammar deliberately leaves to the
loader, and what pins the spec. Verified rather than transcribed: every
terminal read off `ein-ir/src/lex.rs`, all 41 Lark productions and its three
`%ignore` directives mechanically confirmed present, and sixteen sharp-edge
probes run against the binary — every one of which turns out to be in
`grammar_decisions.rs`'s 78-case table already, blessed while both parsers
ran. Twelve dangling links re-pointed.

It also reports a coverage gap rather than leaving it implicit: `BranchOpen`,
`BranchClose`, `BranchRef`, `ContradictionDecl` and `SymmetryDecl` are in the
grammar, in no `.ein` file, and emitted by nothing.

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
