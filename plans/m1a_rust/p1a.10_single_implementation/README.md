# P1a.10 — One implementation

**Milestone:** [M1a — Rust port](../README.md)
**Estimate:** 3 weeks (16 days of stages)
**Depends on:** [P1a.9](../p1a.9_bindings_release/README.md) — the PyO3
surface is what `docs/api/` describes after the Python engine is gone, and
[S1a.9.2](../p1a.9_bindings_release/s1a.9.2_api_parity_tests.md) is the last
stage that compares the two modules while both exist.
**Decides:** [Q-M1a.2](../open_questions.md#q-m1a2--does-einpy-have-a-sunset)
— reversing its recommendation.

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

| stage | title | est. |
|---|---|---|
| [S1a.10.1](s1a.10.1_bank_the_oracle.md) | Bank what only the oracle proves | 4 d |
| [S1a.10.2](s1a.10.2_port_the_suite.md) | Port the Python test suite | 5 d |
| [S1a.10.3](s1a.10.3_corpus_without_an_oracle.md) | The corpus without a second engine | 2 d |
| [S1a.10.4](s1a.10.4_utils.md) | `utils/`, re-aimed at one engine | 2 d |
| [S1a.10.5](s1a.10.5_removal.md) | The removal | 1 d |
| [S1a.10.6](s1a.10.6_docs.md) | The docs after the oracle | 2 d |

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
- The tree is `docs/ plans/ examples/ stdlib/ ein.rs/ utils/` plus the
  top-level files — nothing else.
- **The `.einb` container ([P1a.8](../p1a.8_binary_container/README.md)) and
  the PyO3 module ([P1a.9](../p1a.9_bindings_release/README.md)) still pass
  their own gates**, which are the two surfaces that were checked against the
  Python engine and now cannot be.

## Risks

- **Falsifiability, permanently.** After this there is no independent
  implementation to disagree with. A semantic regression is caught by what
  S1a.10.1 banked and by nothing else, and the corpus's expected outputs
  become *self*-goldens: they say "ein.rs still does what ein.rs did", not
  "ein.rs does what the semantics say". [P1a.11](../p1a.11_stdlib_conformance/README.md)
  exists partly because of this — a stdlib rule with a stated expectation is
  an external check that survives the oracle.
- **The corpus keeps its value; the runner does not.** `conformance/corpus.toml`
  is the *manifest* several ein.rs tests already read, and the completeness
  check it powers is worth keeping. What becomes moot is the two-engine
  runner, not the list. S1a.10.3 separates them.
- **The fuzzer loses its differential mode.**
  [S1a.6.6](../p1a.6_performance/s1a.6.6_differential_fuzzer.md)'s fuzzer found
  four real bugs by comparing engines. Without a second engine it can still
  check *self*-consistency (no panic, round-trip, determinism across hash
  seeds) — strictly weaker, and the acceptance has to say so rather than keep
  the old headline.
- **`docs/api/` changes subject.** It documents the Python embedding surface;
  after this it documents the PyO3 one. If P1a.9 has not landed, this phase
  deletes the only implementation of a documented contract.
  **That is why the dependency is hard, not advisory.**
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
- [`conformance/README.md`](../../../conformance/README.md) — the harness and
  its tiers
- [Q-M1a.2](../open_questions.md#q-m1a2--does-einpy-have-a-sunset) — the
  sunset question, resolved here
- [P1a.11](../p1a.11_stdlib_conformance/README.md) — the check that does not
  need a second engine
