# S1a.10.3 — The corpus without a second engine

**Phase:** P1a.10 (One implementation)
**Estimate:** 2 days
**Depends on:** [S1a.10.1](s1a.10.1_bank_the_oracle.md)

## Context

`conformance/` is two things wearing one name:

1. **the corpus manifest** — `corpus.toml`, one entry per `.ein` file with the
   runs it is exercised under, plus the completeness check that fails when a
   file under `examples/` or `stdlib/` has no entry. Several ein.rs tests read
   it today. This is *inventory*, and it survives the oracle intact.
2. **the differential runner** — `ein-conformance`, which shells out to two
   implementations and diffs them at four tiers, plus `EVENTS.md`'s protocol
   and `ein-parity`'s normalisation list. This is what has no second operand
   any more.

The instinct "remove conformance" would take the first with the second. The
stage exists to separate them.

## Acceptance

- The manifest lives somewhere a single-implementation repo can defend, and
  the completeness check still fails on an unlisted `.ein` file.
- `ein-conformance` and `ein-oracle` are gone; `ein-parity`'s
  normalisation list is either gone or reduced to whatever ein.rs still
  normalises against its own goldens, with the difference recorded.
- The `runs` column keeps its meaning: it is now "the invocations this entry is
  *exercised* under" rather than "…*compared* under", and whatever runs them
  says which.
- `--tier` disappears from the vocabulary, or is re-defined against goldens.
  Leaving T0–T3 in the documentation with no runner is the failure mode this
  acceptance is written against.

## Tasks

### Task T1a.10.3.1 — Decide where the manifest lives

Options: keep `conformance/corpus.toml` and let the directory mean "the
corpus"; or move it under `examples/` (which it describes) or `ein.rs/`
(which reads it). **Recommendation: keep the path**, rename the concept in
`conformance/README.md`, and take the churn in prose rather than in every test
that reads it — the path is referenced from `CLAUDE.md`, both suites and a
dozen plan documents.

### Task T1a.10.3.2 — A runner over one engine

What replaces the harness is not a diff, it is a sweep: run every entry under
every declared run, assert it does not crash, and compare against a golden
where one was banked. That is closer to `utils/render_examples.sh` than to
`ein-conformance`, and it belongs in `cargo test`.

### Task T1a.10.3.3 — Retire the crates

`ein-oracle` (ein.py behind a JSON-Lines protocol) is dev-only and dies with
its subject. `ein-conformance` carries the corpus *bench* set
(`crates/ein-conformance/benches/engine.rs`) — **the eight-bench set moves,
it does not die**; P1a.6's whole record is denominated in it.

### Task T1a.10.3.4 — The events protocol

`conformance/EVENTS.md` specifies `--events`, which is a *product* surface
(T2's operand, but also a debugging tool and M20's likely feed). The protocol
document survives; its framing as "the oracle event protocol" does not.

## Notes

- The bench set is the quiet dependency here, and the one most likely to be
  noticed only after it is gone. `plans/m1a_rust/p1a.6_performance/baseline.md`
  is unreadable without it.

---

## What shipped — 2026-08-21

Two commits: the retirement (`869e9f8`) and the D3 cut's negative control
(`d2d6086`).

### T1a.10.3.1 — the manifest lives in `corpus/`

**The recommendation above is reversed, on purpose.** Keeping the path was the
cheaper edit and the wrong name: `conformance/` meant *two implementations
agreeing*, the word is now claimed by
[P1c.1](../../m1c_external_validation/p1c.1_stdlib_conformance/README.md) with
a different meaning, and the phase's own acceptance asks that
`git grep -i conformance` return only history. Its argument — "the path is
referenced from `CLAUDE.md`, both suites and a dozen plan documents" — was
priced rather than assumed: **21 plan/doc files, 5 scripts, 2 CI files and one
pytest module**, all of them link rewrites, and half of them files S1a.10.4 and
S1a.10.5 open anyway. `corpus.toml` keeps its filename, so every document that
names *the manifest* still names it correctly.

`EVENTS.md` did not move with it. An inventory of `.ein` files is not where a
CLI protocol spec belongs, and the doc is now
[`docs/kernel/inference/events.md`](../../../docs/kernel/inference/events.md).

**Schema 2, and the group vocabulary.** `crash-parity`'s membership rule was
"ein.py raises an unhandled exception here" — neither a directory nor a fact
about the language, and false for two of its ten members, which
[D2](../divergences.md) says ein.rs answers. It splits:

| was | is | why |
|---|---|---|
| `crash-parity` ∩ `examples/broken/compile/` | **`compile-negative`** | parses, loads, then the compiler refuses — the third pass, beside `parse-negative` and `load-negative`, with `.expected` beside each fixture |
| `crash-parity` ∩ `examples/ein-bugs/`, and the five `positive` entries in the same directory | **`regression`** | inputs that once broke an implementation. Seven answer under every run, one answers under `saturate` and is refused under `solve`, two are refused; the exit table is the only statement of which |
| `golden` (empty) | *gone* | the group was "a question with a home" and [Q-M1a.9](../open_questions.md#q-m1a9--where-do-goldens-live) is answered — goldens live in `ein.rs/crates/<crate>/tests/golden/`, which S1a.10.2 made true when it moved the nineteen from `ein.py/` |
| `generated` (empty) | *kept, empty* | the question is still open and is [S1a.10.4](s1a.10.4_utils.md)'s: does the fuzzer still file its cases as corpus entries? |

A group is a **directory** now, which is what makes
`negatives_are_grouped_by_where_they_fail` a mechanical check rather than a
list of tolerances. `examples/broken/compile/activator_arity.ein` is the one
exception it always was, and it is `positive` because its error is unreachable
through the engine by design.

The version bump is not decoration: a schema-1 manifest names groups that no
longer exist, and without it the reader would report a *typo* on a group that
was renamed.

### T1a.10.3.2 — the runner

[`ein-cli/tests/corpus_cli.rs`](../../../ein.rs/crates/ein-cli/tests/corpus_cli.rs):
every entry, every declared run, as a **process**. 542 cells in **2.5 s**;
660 and ~4 minutes under `EIN_CORPUS_SLOW=1`, which is nightly's.

The stage guessed "assert it does not crash, and compare against a golden where
one was banked", and the measurement changed the shape of both halves.

**Why the exit codes are a golden and not a rule.** The obvious design — an
expected exit code per group — does not survive contact with the corpus:

| what the group rule would say | what the sweep measured |
|---|---|
| `load-negative` fails | 10 of its 30 entries exit **0** under `render rules`, which never loads the KB |
| `positive` succeeds | 17 entries exit **1** under `render rules` — they have no rule forms |
| `crash-parity` fails | 3 of 13 cells exit **0** |
| `stdlib` succeeds | `stdlib/macro.ein :: render rules` exits **1**, same reason as the 17 |

Four carve-outs is not a rule, it is a table written badly. So the table is the
table: `tests/golden/corpus_exits.txt`, 660 lines, `code  path :: run`, sorted
by `(path, run)` so re-ordering the manifest does not churn it. The
`render rules` finding is worth its own sentence — `corpus/README.md` used to
claim the negative groups get "three presentations of one error", and a third
of one group gets two presentations and a DOT file.

**Everything else is a rule**, because a rule does not rot: nothing crashes
(exit ∉ {0,1} — and a **usage error** there means the manifest names an argv
the CLI no longer accepts, which nothing else in the workspace can see because
every other test writes its own argv); every `positive`/`stdlib` entry answers
under at least one run — the liveness check's successor, and sharper than it,
since one engine that never runs is an entry that never exits 0; every refusal
carries a diagnostic; every artefact flag leaves its artefact, with the
`--json-summary` *parsed* rather than stat-ed.

**The timeout was not in the plan and is the most load-bearing line in the
file.** Each cell runs under `EIN_CORPUS_TIMEOUT` (default 300 s, the harness's
own default) and a cell that outlives it is killed and recorded as `-2`.
Without it the failure mode of a change that makes some corpus program stop
terminating is not a red gate but a **hung** one, with no output and no name.
This is the only test in the workspace that runs unbounded search on arguments
it did not write. stdout and stderr go to files rather than pipes, because a
bounded run has to poll and a child filling a 64 KB pipe nobody drains would
deadlock instead of finishing.

All three instruments were checked against a deliberate break before being
trusted — a manifest run the CLI refuses (`no_cell_crashes` reports the usage
error and names the cell), an exit code edited in the golden (reported as
`was 1, now 0` against the cell's name), and `EIN_CORPUS_TIMEOUT=0` (every cell
killed and named). A golden nobody has ever seen fail is a file, not a test.

### T1a.10.3.3 — the crates

`ein-oracle` and `ein-conformance` are gone; **`ein-corpus`** is what their
surviving halves became, dev-only, `publish = false`:

| moved in | from |
|---|---|
| `manifest.rs` — the reader, the group vocabulary, the nine invariants | `ein-conformance/src/corpus.rs` |
| `plan.rs` — a run name → an `ein` argv | `ein-conformance/src/plan.rs` |
| `benches/engine.rs` — the eight-bench M1a set, and its `snmalloc` feature | `ein-conformance/benches/engine.rs` |
| `repo_root`, `corpus_files`, `golden`, `golden_path` | `ein-oracle/src/lib.rs` |

The bench's `default = ["snmalloc"]` is why the crate exists rather than the
benches moving into `ein-infer`: a bench's allocator choice does not belong in
the dependency graph of everything that links the engine.
`cargo bench -p ein-corpus --no-default-features` is still the system-allocator
arm, and `cargo bench --bench engine` is unchanged.

**2 164 lines and 29 unit tests died with the harness** — `tier.rs` (10),
`normalise.rs` (11), `events.rs` (4), `main.rs` (4) — plus `ein-oracle`'s
`Oracle`/`Answer`/`IR_ORACLE`/`PY_ORACLE`, which S1a.10.2 had already left
without callers. `cargo test --workspace` went 566 → 535 → **542** with this
stage's seven new tests. Every one of the 31 removed is a claim about a
comparison that no longer has two operands; none is a claim about the language.

**`ein-parity` survives**, minus `narrated_artefacts` (the harness's alone).
[The ledger §8](oracle_ledger.md#8-the-divergence-ledger-re-read) is why, and
it was right: the same three observables move *inside one engine* under a
permuted id space, so the rule stopped being a statement about two
implementations. Its module doc now names the four call sites that apply it.
`events` survives too, and got the live consumer it had been missing —

### T1a.10.3.4 / the events protocol, and the cut's control

The doc moved to `docs/kernel/inference/events.md` and lost its framing: it is
`--events`, a product surface, not "the oracle event protocol". Its
§ Comparison used to open with `ein-conformance diff a.jsonl b.jsonl`; the
differ is a library function now, and what calls it is the second commit.

[`ein-infer/tests/event_cut_control.rs`](../../../ein.rs/crates/ein-infer/tests/event_cut_control.rs)
is `utils/mutant_ein.py` with the two processes taken out — the ledger's
[§3.7](oracle_ledger.md#3-the-instruments-that-are-not-tiers), which claimed
the mutation control as **covered** and would have become false otherwise. The
script's three mutations, unchanged: delete the first productive firing (the
cut must report it), the first redundant one and the first `enqueue` (it must
not). The mutation was always applied to the *artefact*, so the harness and the
second engine bought nothing the test does not have.

Two claims the script could not make: a stream compares equal to itself first,
and deleting an elided event still moves `elided_total` — so "the cut stayed
silent" is distinguishable from "the parser dropped the line".

### Two things the removal forced

- **`dot_wellformed.rs` fails rather than skips** when Graphviz is missing. Its
  skip went to a stderr line `cargo test` captures for a *passing* test — the
  ledger [§2](oracle_ledger.md#2-the-finding--46--of-einrss-own-integration-tests-are-differential)'s
  shape, and the last instance of it in the workspace — so CI had been
  reporting a pass for 5 209 renderings nothing checked. The rust job installs
  graphviz now. The module's own doc comment said "skipped, **loudly**", which
  is the same adverb the ledger caught in the CI file.
- **`--events` and `--json-summary`'s `--help` text** advertised "the M1a
  conformance harness's T2 parity tier". That is shipped user-facing text
  describing a thing that does not exist; it names the schema instead.

### CI

`conformance-fast` is gone from per-commit — the corpus is swept every commit
by `cargo test --workspace`, in three seconds rather than eight minutes of two
engines. Nightly's `hash-seed-sweep` and `conformance-full` become one
**`deep-corpus`** job: the 118 slow cells, and eight id-space permutations
(`EIN_ID_SEEDS=8`), which is the PYTHONHASHSEED sweep's successor asking the
same question of the engine that is left. `frontend-fuzz` was invoking
`cargo test -p ein-ir --test fuzz_parity`, a test S1a.10.2 renamed, and had
been failing to run since.

### `utils/`, touched only where it dangled

S1a.10.4's, except that this stage moved the ground under three scripts.
`fuzz_ein.py` cannot run at all without the harness and now says so at the top
and on startup, naming T1a.10.4.2 and the ledger's L1 — the alternative was a
script that looks runnable and exits on a missing binary nobody can build.
`mutant_ein.py` names its successor. `fork_delta_verify.py` and `spec_audit.py`
select `positive`/`stdlib` from the manifest, so the regrouping would have
silently cost them five files; both now take `regression` too, which is the
same file set they had.

## Acceptance, checked

| criterion | |
|---|---|
| the manifest lives somewhere a single-implementation repo can defend, and the completeness check still fails on an unlisted `.ein` | `corpus/corpus.toml`; `ein_corpus::manifest`'s nine tests, and the `stdlib/` fallback scan that S1a.10.5 was owed is closed here (it would have turned "the stdlib is gone" into "seven fewer files to check") |
| `ein-conformance` and `ein-oracle` gone; `ein-parity` reduced, with the difference recorded | both crates deleted; `narrated_artefacts` gone; the four surviving call sites are tabulated in the crate's module doc, and `events` has the control it lacked |
| the `runs` column keeps its meaning, and whatever runs them says which | `corpus/README.md` § Manifest format states the change from *compared under* to *exercised under*; the reader table names all five |
| `--tier` disappears from the vocabulary | no live document defines a tier. `design/01` keeps its four as history, with a marker; `corpus/README.md` says the vocabulary went with the runner |

**Not done here, and named:** `utils/fuzz_ein.py`'s rewrite (S1a.10.4
T1a.10.4.2), deleting `mutant_ein.py` / `ir_oracle.py` / `py_oracle.py`
(T1a.10.4.1), and the `generated` group's fate, which is the same decision.

**The gate:** `cargo test --workspace` — **542 passed**, 0 failed.
`EIN_CORPUS_SLOW=1` — 660 cells, 236.9 s, green. `cargo bench --bench engine
-- --test` green on both allocator arms. `cargo fmt --check`, `cargo clippy
--workspace --all-targets -- -D warnings` clean. `pytest
tests/test_corpus_manifest.py` — 9 passed.
