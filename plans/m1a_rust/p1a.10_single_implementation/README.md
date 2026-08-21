# P1a.10 — One implementation

**Milestone:** [M1a — Rust port](../README.md)
**Estimate:** 3 weeks (16 days of stages)
**Depends on:** nothing outstanding. It was
[P1a.9](../p1a.9_bindings_release/README.md), **and that is reversed —
2026-08-21**: P1a.9 depends on *this* phase and releases ein.rs alone.

> The argument for the old order was that `docs/api/` documents the Python
> embedding surface and the PyO3 module is its successor, so removing the
> engine first would delete the only implementation of a documented contract.
> True, and priced: `docs/api/` documents an unimplemented surface for the
> length of one stage.
>
> The argument for the new order is that **there is nothing to release two
> of**. A binding phase that ships while a second Python engine is in the
> tree has to answer "which one does `import ein` get", keep two exception
> hierarchies in step, publish two packages, and parameterise its test suite
> over both — all of it work that exists only because the other engine does.
> [S1a.10.6](s1a.10.6_docs.md) states the gap;
> [S1a.9.4](../p1a.9_bindings_release/s1a.9.4_documentation.md) closes it,
> and is the milestone's last documentation stage. The P1a.9 stages are
> amended to match.
**Decides:** [Q-M1a.2](../open_questions.md#q-m1a2--does-einpy-have-a-sunset)
— reversing its recommendation.
**Status:** **in progress** — [S1a.10.1](s1a.10.1_bank_the_oracle.md) and
[S1a.10.2](s1a.10.2_port_the_suite.md) shipped 2026-08-20; the ledger is
[`oracle_ledger.md`](oracle_ledger.md) and the suite's file-by-file record is
[`suite_dispositions.md`](suite_dispositions.md). They ran before
[P1a.9](../p1a.9_bindings_release/README.md) on purpose: the dependency is the
*deletion*'s, and an inventory that deletes nothing — followed by a port that
banks its answers while the oracle can still be asked — is exactly what should
happen while both engines still run.

**After S1a.10.2 the gate is already single-engine.** `cargo test --workspace`
is **566 tests in 1 m 07 s**, none of which starts a Python process, where
before it was 312 in 9 m 13 s of which 42 shelled out. What is left in the
phase is the *tree*: the harness, the runner, `utils/`, and `ein.py/` itself.

**[S1a.10.4](s1a.10.4_utils.md) shipped 2026-08-21.** `utils/` is **17
scripts**, all driving `ein.rs`; the eleven that compared two engines or
measured the Python one are gone, each either banked by a checked-in test or
superseded by a named instrument. `baseline.md`, `design/README.md`,
`design/12` and `features.md` now say which of their numbers are **live** and
which are **frozen constants** — every CPython and PyPy figure is the latter.
The `generated` corpus group went with the fuzzer's throwaway manifest, so
`GROUPS` is six and no group is empty.

**The rewritten fuzzer found three things in its first twenty minutes**, which
is the first work this phase has produced that is not clean-up: an `(hrule …)`
reading `not` aborts a debug build on a `debug_assert!`; the **unsat core**'s
contents move under a permuted id space; and the goal-binding row the solve
table prints does too — re-deriving, from a different seed, the exact seven
forms of a finding filed in August against [D3](../divergences.md), and
showing that D3 is one perturbation of that row rather than why it is
perturbable. All three are fixtures with notes in
[`corpus/fuzz_findings/`](../../../corpus/fuzz_findings/README.md) and none is
fixed: each is a semantics decision. They are also a small piece of evidence
about [L1](oracle_ledger.md#6-accepted-loss) — the loss is real, and the
instrument that replaced it re-judged a finding the oracle had mis-attributed.

**[S1a.10.3](s1a.10.3_corpus_without_an_oracle.md) shipped 2026-08-21.** The
harness is gone — `ein-conformance`, `ein-oracle`, the four tiers, 2 164 lines
and 29 unit tests of comparison machinery — and the corpus it read is
[`corpus/`](../../../corpus/README.md), swept by
`ein-cli/tests/corpus_cli.rs`: 542 cells as processes in 2.5 s, exit codes
against a banked 660-line table, four rules that cannot rot, and a per-cell
timeout so a program that stops terminating fails the gate instead of hanging
it. `ein-parity` survived, reduced, and finally has a live consumer for its
event cut. `cargo test --workspace` is **542 tests**.

## Goal

**ein.rs is the only implementation.** `ein.py/`, the differential harness,
the PyPy and venv tooling, and the `nlp/` and `smt/` submodules leave the
tree. What is left is `docs/`, `plans/`, `examples/`, `stdlib/`, `ein.rs/`
and a `utils/` that drives one engine.

## This reverses two standing decisions, on purpose

The milestone was written around a permanent oracle, and said so twice:

> **I1 — Outside, nothing changes.** … `ein.py/` stays in the repo permanently
> as the **oracle**.

> **Non-goal — Deleting ein.py.** It is the oracle and the reference for M2
> experiments. It stays, and stays green.

Both are amended, dated, in [the milestone README](../README.md). The
argument for the oracle was never that a second implementation is valuable in
itself — it was that **a rewrite with a byte-exact oracle is a measurable
rewrite**. That argument has an expiry date, and it is the end of P1a.5:
after the byte gate closed, the oracle stopped being what *drives* the port
and became what *guards* it. P1a.6 already started living without it —
[D3](../divergences.md) is a deliberate narration divergence, and
[S1a.6.11](../p1a.6_performance/s1a.6.11_fixture_goldens.md) replaced the
elided bytes with twelve ein.rs goldens because "a gate that runs one
implementation is no longer the gate" (`run_tests.sh` phase 3's own comment).

So the question this phase answers is not "was the oracle worth it" — it was —
but **"what does it still prove that nothing else does, and can that be
banked?"** [S1a.10.1](s1a.10.1_bank_the_oracle.md) is that inventory, and it
is a **gate**: nothing is deleted until every claim the harness carries has a
checked-in owner in ein.rs. Deleting first and discovering the gap afterwards
is the one way this phase can go wrong that cannot be undone by a revert.

## Stages

| stage | title | est. | |
|---|---|---|---|
| [S1a.10.1](s1a.10.1_bank_the_oracle.md) | Bank what only the oracle proves | 4 d | ✅ **shipped 2026-08-20** |
| [S1a.10.2](s1a.10.2_port_the_suite.md) | Port the Python test suite | 5 d | ✅ **shipped 2026-08-20** — scope grew, see below |
| [S1a.10.3](s1a.10.3_corpus_without_an_oracle.md) | The corpus without a second engine | 2 d | ✅ **shipped 2026-08-21** — `corpus/`, and a sweep |
| [S1a.10.4](s1a.10.4_utils.md) | `utils/`, re-aimed at one engine | 2 d | ✅ **shipped 2026-08-21** — 28 scripts → 17, and three fuzz findings |
| [S1a.10.5](s1a.10.5_removal.md) | The removal | 1 d | has a defect list |
| [S1a.10.6](s1a.10.6_docs.md) | The docs after the oracle | 2 d | |

### What S1a.10.1 moved

The stage was written as an inventory of `conformance/`. The inventory found
that **the largest differential surface in the repo is not the harness — it is
`cargo test --workspace` itself**: 42 of its 91 integration tests start a
Python process, and when one cannot start they *skip*, on a stderr `cargo test`
captures, so the suite reports 311 passed. That is the phase's own acceptance
criterion quietly not being one.

Three consequences, all recorded in the
[ledger](oracle_ledger.md):

- **S1a.10.2's subject doubles.** It is no longer only "port 1 517 pytest
  tests"; it is also "un-differential 42 ein.rs tests", and the ledger already
  says what each of them still owes. *(Done: all 42, plus the 275 behaviours
  the Python suite's 1 538 tests reduce to.)*
- **S1a.10.5 gets a defect list.** Five ein.rs tests read *files* under
  `ein.py/` rather than running it, so no amount of removing Python finds
  them — they are green until the commit that deletes the tree.
  [§4](oracle_ledger.md#4-what-the-removal-must-relocate) names them and says
  to `git mv` the 19 goldens rather than re-bless them, because ein.py's own
  bytes are the last independent provenance the repo has. *(S1a.10.2 did the
  move a stage early — carrying a defect into the stage it warns about is the
  one thing the row exists to prevent. They are
  `tests/golden/from_ein_py/` now. What remains for S1a.10.5 is
  `corpus.rs::tracked`'s fallback stdlib scan, which is a code path.)*
- **[P1a.8](../p1a.8_binary_container/README.md) gets a question.** The
  determinism successor prices a permuted id space at *0 answers and 66
  renderings*, and `.einb`'s remap is a permutation
  ([Q-M1a.22](../open_questions.md#q-m1a22--is-einbs-id-remap-order-preserving-enough-for-its-own-gate)).

## Acceptance for the phase

- **`cargo test --workspace` is the whole gate.** No `run_tests.sh` phases, no
  pytest, no PyPy, no venv, no `PYTHONPATH`.
- **Coverage does not drop.** Every behaviour the Python suite asserted is
  either (a) asserted by a Rust test, (b) demonstrably already covered — with
  the covering test named — or (c) deleted *with* the thing it tested, and
  named in the ledger with the reason. No behaviour moves to (c) because
  porting it was inconvenient.
- **Every parity claim in `plans/` and `docs/` that cites the harness cites
  something that still runs.** A number whose instrument is gone is a story;
  the phase's own writing is the first place that has to be true.
- `git grep -i 'ein\.py\|pypy\|\.venv\|conformance'` returns only history,
  the divergence ledger and this phase's own record.
- The tree is `docs/ plans/ examples/ stdlib/ ein.rs/ utils/ corpus/` plus
  the top-level files, **and `nlp/` + `smt/`** — 56 KB of scratch that
  S1a.10.5 was written to delete and that
  [it keeps, amended](s1a.10.5_removal.md#acceptance): every file in them has
  a named dependent in M1c P1c.2 or M2 P2.5. What goes is the two
  *submodules*, which had none and made every recursive clone fetch two large
  upstream repositories for work that has not started.
- **Neither [P1a.8](../p1a.8_binary_container/README.md)'s `.einb` container
  nor [P1a.9](../p1a.9_bindings_release/README.md)'s PyO3 module has started**,
  so this criterion — "they still pass their own gates" — has nothing to
  check and is **retired** rather than quietly met. What it was guarding is
  real and moves to those phases: both are surfaces that would have been
  checked against the Python engine and now cannot be, so each has to state
  what it is checked against instead. P1a.9 answers *the CLI, and the
  contract in `docs/api/`*
  ([S1a.9.2](../p1a.9_bindings_release/s1a.9.2_api_parity_tests.md));
  P1a.8's answer is
  [Q-M1a.22](../open_questions.md#q-m1a22--is-einbs-id-remap-order-preserving-enough-for-its-own-gate)'s.

## Risks

- **Falsifiability, permanently.** After this there is no independent
  implementation to disagree with. A semantic regression is caught by what
  S1a.10.1 banked and by nothing else, and the corpus's expected outputs
  become *self*-goldens: they say "ein.rs still does what ein.rs did", not
  "ein.rs does what the semantics say". [P1c.1](../../m1c_external_validation/p1c.1_stdlib_conformance/README.md)
  exists partly because of this — a stdlib rule with a stated expectation is
  an external check that survives the oracle.
- **The corpus keeps its value; the runner does not.** `corpus/corpus.toml`
  is the *manifest* several ein.rs tests already read, and the completeness
  check it powers is worth keeping. What becomes moot is the two-engine
  runner, not the list. S1a.10.3 separated them — and found that the runner
  was carrying one claim the library tests structurally cannot make, which is
  that the manifest's `runs` column still names invocations the CLI accepts.
  The sweep keeps it.
- **The fuzzer loses its differential mode.**
  [S1a.6.6](../p1a.6_performance/s1a.6.6_differential_fuzzer.md)'s fuzzer found
  four real bugs by comparing engines. Without a second engine it can still
  check *self*-consistency (no panic, round-trip, determinism across hash
  seeds) — strictly weaker, and the acceptance has to say so rather than keep
  the old headline.
- **`docs/api/` has no subject for one stage.** It documents the Python
  embedding surface; after this it documents the PyO3 one, and the PyO3 one
  does not exist yet. This was the reason the dependency on P1a.9 was called
  hard; **the order is reversed instead** (see the amendment above), so the
  risk is accepted rather than avoided: for the interval between
  [S1a.10.6](s1a.10.6_docs.md) and
  [S1a.9.4](../p1a.9_bindings_release/s1a.9.4_documentation.md), five pages
  describe a contract nothing implements. S1a.10.6 must say so **on the
  pages themselves** — a documented API that quietly names a dead module is
  the failure mode; one that says "the implementation lands in S1a.9.1" is a
  plan.
- **M1 semantics have one home.** Today a semantic change lands twice and the
  harness checks the two agree. After this, "what the engine does" is
  whatever ein.rs does, and `docs/kernel/` is the only statement of intent
  that is not also the implementation. It gets more load-bearing, not less.

## Non-goals

- **Deleting the divergence ledger.** [D1–D3](../divergences.md) record where
  the two engines differed and *why*; that is the port's history and it stays,
  with a note that the second engine is gone.
- **Deleting the Python *bindings*.** P1a.9's PyO3 module is ein.rs exposed to
  Python, which is the opposite of what this phase removes. Python stays a
  supported *consumer*.
- **Re-litigating M1's semantics.** Nothing here changes what the engine
  proves. The suite moves language; it does not move behaviour.

## Cross-links

- [design/01 — Parity contract](../design/01_parity_contract.md) — the tiers
  this phase retires, and the ones it converts to goldens
- [`corpus/README.md`](../../../corpus/README.md) — the harness and
  its tiers
- [Q-M1a.2](../open_questions.md#q-m1a2--does-einpy-have-a-sunset) — the
  sunset question, resolved here
- [P1c.1](../../m1c_external_validation/p1c.1_stdlib_conformance/README.md) — the check that does not
  need a second engine
