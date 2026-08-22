# S1a.10.5 — The removal

**Phase:** P1a.10 (One implementation)
**Estimate:** 1 day
**Depends on:** [S1a.10.2](s1a.10.2_port_the_suite.md),
[S1a.10.4](s1a.10.4_utils.md), and **T1a.10.5.0 below** — which is a
precondition, not a step of the removal.

> **The phase's dependency on [P1a.9](../p1a.9_release/README.md) is
> reversed, 2026-08-21.** It was hard — "if P1a.9 has not landed, this phase
> deletes the only implementation of a documented contract" — and the answer
> is that P1a.9 now runs *after* this phase and releases ein.rs alone. The
> cost is one interval in which `docs/api/` documents an unimplemented
> surface; [S1a.10.6](s1a.10.6_docs.md) states it on the pages and
> [S1a.9.4](../p1a.9_release/s1a.9.4_documentation.md) closes it.
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
- **The two submodules** — `nlp/link-grammar` and `smt/CVC4` — deinitialised:
  `git submodule deinit -f`, the `.gitmodules` entries removed, `.git/modules/`
  cleaned. **The directories stay**, and the acceptance said otherwise.

  > **Amended 2026-08-21, at the user's direction, on evidence.** This read
  > "`nlp/` and `smt/` gone", on the grounds that they are "24 KB of scratch"
  > and "nothing in the active tree imports either". The second clause is true
  > and is the **wrong criterion**: both directories have named dependents in
  > the *planned* tree.
  >
  > | | |
  > |---|---|
  > | `smt/{4-queens,einstain-problem,einstain-problem-minus-15}.smt` | [P1c.2 S1c.2.1](../../m1c_external_validation/p1c.2_external_benchmarks/s1c.2.1_problem_corpus.md) links all three as encodings its benchmark corpus already has |
  > | `nlp/{xxx.py,xxx-link.py}` + the reading list | [M2 S2.5.1](../../m2_nl_to_ir/p2.5_link_grammar_experiment/s2.5.1_runner.md) names them as its starting point |
  > | `nlp/link-grammar` | [M2 P2.5](../../m2_nl_to_ir/p2.5_link_grammar_experiment/README.md) **depends on** it, and exists to decide whether to deprecate it |
  > | `smt/CVC4` | P1c.2 said "the benchmark uses CVC5 and the submodule stays where it is" |
  >
  > What the submodules actually cost is **a `git clone --recurse-submodules`
  > fetching `opencog/link-grammar` and `CVC4/CVC4`** for work that has not
  > started — neither was ever checked out here (`git submodule status` showed
  > both unregistered). Deinitialising removes that cost and keeps every file
  > with a dependent; each README carries the one `git submodule add` line
  > that restores it, and P2.5's first act is to run its own. Deleting
  > `nlp/link-grammar` outright would have pre-empted the stage whose whole
  > purpose is that decision.
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

It is [`docs/kernel/ir/03-ein-lang/00_ebnf.md`](../../../docs/kernel/ir/03-ein-lang/00_ebnf.md)
now — a file of its own, numbered `00` because it is a reference to reach for
rather than a chapter to read first: W3C EBNF, lexical and phrase layers, plus
three things the Lark carried
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

---

## What shipped — 2026-08-21

Five commits, and the tag the Notes asked for.

| | commit | |
|---|---|---|
| — | `two-implementations` | a tag, not a commit: the last revision where the two engines can be diffed. Its message lists what survives of ein.py's provenance without it |
| T1a.10.5.0 | `7f10199` | the grammar → EBNF, **before** anything was deleted |
| T1a.10.5.2/3/4 | `4c1a5b3` | `ein.py/`, `ein_pypy.sh`, `venv_install.sh`, `run_tests.sh`, CI |
| T1a.10.5.1 | `e858bd0` | the two submodules, deinitialised — and the acceptance that was wrong |
| T1a.10.5.4 | `681aa4d` | `README.md`, which the delete made false |
| — | this one | the record |

**183 files.** 94 tests, 79 source, 4 acceptance, the build config, and the
two venv scripts. Nothing tracked under `ein.py/` was anything but Python by
the time the delete ran: the nineteen goldens moved at S1a.10.2, the
diagnostics are `examples/broken/**/*.expected`, and the grammar left an hour
earlier.

### The one thing that had to happen first

`docs/kernel/ir/03-ein-lang/01_grammar.md` opened with *"Source of truth for
syntax: `grammar.lark` … the grammar file is canonical for what parses"*. A
`git rm` would have left a recursive-descent parser as the only statement of
the language — an implementation, not a specification. **T1a.10.5.0** is the
user's own floating precondition, promoted to a task and done before anything
else; [the task](#task-t1a10450--the-grammar---done-2026-08-21-7f10199) has
what it delivered, including the coverage gap it found.

### T1a.10.5.3 — the runner keeps its name

The recommendation was "keep the name", and it stands: `run_tests.sh` is in
`AGENTS.md`, in a dozen "Gate:" lines and in the user's habits, and a wrapper
that still works is cheaper than re-teaching the habit. Its three phases are
in the header as *history*, each with its successor named, which is the same
treatment the retired CI jobs got.

Two things it gained rather than lost. `--slow` is the nightly tier in one
flag (`EIN_CORPUS_SLOW=1 EIN_ID_SEEDS=8`), and **a missing cargo or a missing
Graphviz is now an error**. The old script skipped Phase 3 loudly when cargo
was absent; the ledger's whole §2 is about what "loudly" is worth when the
message goes to a stream nobody reads, so the gate refuses to run instead of
running partially.

### T1a.10.5.1 — the acceptance was wrong, and the evidence said so

Written above, in the amendment. The short version: "nothing in the active
tree imports either" is true and is the wrong test. `smt/*.smt` is three of
[P1c.2](../../m1c_external_validation/p1c.2_external_benchmarks/README.md)'s
benchmark encodings; `nlp/xxx*.py` is
[M2 S2.5.1](../../m2_nl_to_ir/p2.5_link_grammar_experiment/s2.5.1_runner.md)'s
starting point; and `nlp/link-grammar` is depended on by the phase whose
*purpose* is deciding whether to deprecate it. **The submodules go, the
directories stay** — what the submodules cost was a recursive clone fetching
two large upstream repositories for work that has not started, and neither was
ever checked out here.

Four plan documents were amended to match rather than left to contradict the
tree, which is the half of a decision that usually goes missing.

### The sweep, and where it stops

The delete left **232 dangling markdown links**.

| | | |
|---|---|---|
| 8 | `examples/`, `stdlib/`, `utils/vscode-ein/`, two `ein-infer` module docs | fixed here — this stage's own breakage |
| ~10 | `README.md` | fixed here, and they were **instructions that fail**: `./venv_install.sh`, `python -c 'from ein.ir import parse'`, a Layout table of eight `ein.py/` rows |
| 224 | `docs/kernel/` (220) + `docs/api/` (4) | **[S1a.10.6](s1a.10.6_docs.md)'s**, and counted into its stage file so it is estimated against rather than discovered |

The 220 are not a find-and-replace, which is why they are not done here: most
are "[`world.py`](…) is the boundary" — *a claim about the specification,
evidenced by a pointer into an implementation* — and that is exactly the shape
S1a.10.6's acceptance says must go. `python_impl.md` is 34 of them and is the
one file whose subject is gone entirely.

### CI

Three jobs went with their subject: per-commit's `oracle` (which ran first and
alone, because a parity failure is meaningless if the oracle moved), nightly's
`full-suite` and `packaging`. Each leaves a comment naming its successor
rather than silence. `release.yml`'s `packaging` loses the ein.py wheel and
keeps the binary matrix — the stdlib copy that job existed to check is
`include_dir!`'s now, verified inside the gate.

**`python3` stays in CI**, for three things that are not an engine:
`utils/stdlib_manifest.py`, `utils/check_hashmap_iteration.py`, and
`examples/gen_zebra2_variants.py --check`, which `cli_semantics.rs` runs.

**Python linting left with the tree.** `ruff check ein.py` was the `oracle`
job's last step and its config lived in `ein.py/pyproject.toml`. Nothing lints
`utils/` today; it comes back with
[P1a.9](../p1a.9_release/README.md)'s binding suite, which is the
next Python in the repo, and which also has to answer where a pytest suite
lives now.

## Acceptance, checked

| criterion | |
|---|---|
| `ein.py/`, the venv scripts, `run_tests.sh`'s phases, every Python packaging file outside `utils/` | 183 files; `run_tests.sh` kept as a wrapper per T1a.10.5.3's recommendation |
| `nlp/` and `smt/` gone as submodules | **amended** — the submodules are deinitialised and `.gitmodules` is gone; the directories stay, on evidence, with four plan documents amended to match |
| `.gitignore` loses what named the removed trees and keeps what still describes something | `ein.py/src/ein/stdlib/` gone; the `.venv*` patterns stay, with the reason rewritten (P1a.9's binding tests want a clean venv) |
| CI runs `cargo test --workspace` and nothing else | not literally, and the difference is deliberate: `shared-assets` still runs the stdlib manifest check and the determinism grep, which are two claims `cargo test` structurally cannot make (§T1a.10.4.5 for the first) |
| one commit per removed tree, not interleaved with a behaviour change | five, each reviewable on its own: the grammar *before* the delete, the tree, the submodules, the README, the record |
| tag the commit before the delete | `two-implementations` |

**Not done here, and named:** `docs/api/` and `docs/kernel/` (224 links, and
the rewrite behind them) are [S1a.10.6](s1a.10.6_docs.md)'s. `.venv/` and
`.venv-pypy/` — 272 MB of untracked local virtualenvs — are left on disk at
the user's direction: machine state, not repo state, and nothing references
them.

**The gate:** `./run_tests.sh` — **542 passed**, 0 failed. `cargo fmt
--check` and `cargo clippy --workspace --all-targets -- -D warnings` clean.
`cargo bench --bench engine -- --test` green.
