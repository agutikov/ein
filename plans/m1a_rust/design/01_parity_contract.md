# 01 — The parity contract

**Settles:** what "100 % surface match" means operationally, how it is
measured, and what happens when it cannot be met.
**Phase:** [P1a.0](../p1a.0_conformance_harness/README.md) builds it;
every later phase is gated by it.

---

## 1. Why this document is first

"Same behaviour as ein.py" is not a testable statement until someone
says *which* behaviour, observed *where*, at *what granularity*. Without
that, a port drifts silently: the verdict still comes out right, the
trace has one extra step, nobody notices for three months.

The contract below turns the milestone's invariant I1 into four
mechanical tiers. Each tier is a diff between two runs — one of `ein`
(Python), one of `ein.rs` — over the same corpus. A phase is done when
its tier is green on the whole corpus.

The repo has done this once before, at a much looser tier:
[`parity_baselines.md`](../../../docs/kernel/inference/parity_baselines.md)
compared the retired tree solver against the monotonic one, per fixture,
with an explicit *known divergences* section. That shape — a table plus a
named-divergence ledger — is right; the tiers here are what make it
strict enough for a rewrite.

---

## 2. The four tiers

Ordered by strength. A tier subsumes the ones above it.

### T0 — Verdict parity

The answer is the same.

| observable | source |
|---|---|
| verdict type | `Solution` / `Ambiguity` / `Contradiction` / `Aborted` |
| `k` (`stats.solution_nodes`) and `stats.exhausted` | `MonotonicStats` |
| the model, as a *set* of `(relation, args)` facts, per solution | `verdict.kb.facts` |
| `goal_bindings` rows, as a set | `verdict.kb` |
| `unsat_core`, as a set | `Contradiction.unsat_core` |
| process exit code | CLI |

T0 is the weakest useful gate and the one an early P1a.3 build can
reach. It catches "the engine proves the wrong thing"; it catches
nothing about *how*.

### T1 — Counter parity

T0, plus every number the engine reports about its own work:

`enterings_total` · `enterings_alive` · `enterings_dead_pre` ·
`enterings_dead_post` · `layers_explored` · `saturate_count` ·
`nogoods_emitted` · `nogoods_subsumed` · `facts_merged` ·
`forced_positives` · `Saturator.{naf_rounds, naf_admitted, naf_retired,
naf_dropped}` · `HypGenStats.{raw, emitted, filtered.*, pre_candidate.*}`
· `len(engine.cache)` · fact counts per relation after root saturation.

T1 is the real semantic gate. Two engines can agree on the verdict while
disagreeing about which candidates were pruned, which guards were
retired, or how many alternatives were recorded — and every one of those
disagreements is a latent behaviour change that will surface on the next
puzzle. Note `naf_dropped` is structurally 0 since S1.21.8; ein.rs
reporting anything else means the boundary was rebuilt wrong.

### T2 — Event-trace parity

T1, plus the **ordered event log** (§3): every firing, park, admission,
retirement, entering, no-good emission and hypgen decision, in order,
with its payload. This is the tier that pins *the algorithm*, and it is
what makes the optimisations in [05](05_matcher.md)/[06](06_saturation.md)
auditable — a beta-memory that changes which of two equally-valid
matches is found first shows up here as a diff, and gets either a
justification in the divergence ledger or a fix.

### T3 — Byte parity

T2, plus **byte-for-byte identical output artefacts**:

- `ein solve` stdout (the solution table, `--stats`, `--timing` *field
  layout* — see the timing caveat below), stderr;
- `ein solve --trace out.md` — the whole markdown file, DOT blocks
  included;
- `ein render {rules,rule,constraints,lattice}` DOT on stdout;
- `ein saturate` stdout including `--dump`;
- `--dump-states DIR` — the whole dump tree (`summary.json`,
  `00_timeline.jsonl`, per-layer snapshots), modulo the timestamp fields
  listed in §5;
- every error message on every file in `examples/broken/` and every
  `KBLoadError` in the negative corpus;
- `--help` for every subcommand.

T3 is what "drop-in replacement" means. It is the [P1a.5](../p1a.5_presentation/README.md)
gate.

### Tier → phase map

| phase | gate |
|---|---|
| [P1a.1](../p1a.1_ir_frontend/README.md) | T3 on parse/dump/render surfaces only |
| [P1a.2](../p1a.2_kb_core/README.md) | T3 on load errors; T1-shaped KB-structure diff |
| [P1a.3](../p1a.3_deductive_core/README.md) | T2 on saturation-only fixtures (`examples/saturation/**`, `features/**`) |
| [P1a.4](../p1a.4_search_layer/README.md) | T1 corpus-wide, T2 on `branching/**` + `lattice/**` + `domain_elim/**` |
| [P1a.5](../p1a.5_presentation/README.md) | **T3 corpus-wide** |
| [P1a.6](../p1a.6_performance/README.md)–[P1a.9](../p1a.9_bindings_release/README.md) | T3 stays green; regressions are release blockers |

---

## 3. The oracle event protocol

T2 needs both implementations to say what they did in a comparable
format. ein.py already emits *something* like this — `MonotonicDumper`
and `LatticeDumper` write a `00_timeline.jsonl` via
`_TimelineMixin._emit_timeline` under `--dump-states` — but it covers
only the search layer's five lifecycle hooks. T2 needs the deductive
layer too.

**Decision: a new opt-in flag, `--events FILE`, on both implementations,
emitting one JSON object per line.** Additive and off by default, so it
cannot perturb any existing behaviour; when off, not a single branch is
taken on the hot path (the emitter is behind an `Option<Writer>` in Rust
and a `None` check in Python).

> This edits `ein.py` code. That is deliberate and it is **M1a work, not
> retro-fitted M1 work**: the flag exists to serve the port, it is
> planned as [S1a.0.2](../p1a.0_conformance_harness/s1a.0.2_oracle_event_protocol.md),
> and it lands under this milestone's number.

### Event schema (v1)

Every line: `{"e": "<kind>", "n": <seq>, ...}` where `n` is a per-run
monotonic counter. Numbers are integers; facts are rendered as the
canonical s-expression string `fact_sexpr` already produces, so the
protocol has no dependency on either side's internal ids.

| `e` | emitted at | payload |
|---|---|---|
| `run` | start | `impl`, `version`, `file`, `argv`, resolved `config` (all fields) |
| `load` | after `kb.from_ir` | counts: relations, rules, hrules, macros, facts; ordered relation names; ordered rule names |
| `compile` | each `Engine.compile_for` **miss** | `rule`, `activator`, `n_steps`, `n_disjuncts`, `n_guards`, `asserts` |
| `enqueue` | `_enqueue_binding` | `rule`, `activator`, `bindings` (in binding order), `priority`, `tiebreaker`, `parked` |
| `fire` | each `Firing` yielded | `rule`, `activator`, `bindings`, `premises`, `derived`, `redundant` |
| `mirror` | native `__symmetric__` write | `relation`, `src`, `derived` |
| `park` / `admit` / `retire` | boundary decisions | `tiebreaker`, `round`, failing guard's `watched` set |
| `quiesce` | closure quiescence | `round`, `n_facts`, `n_queue`, `n_parked` |
| `alt` | `record_justification` returns True | `fact`, `rule`, `premises` |
| `hyp` | each hypgen verdict | `fact`, `verdict` ∈ {emitted, filter name, pre-skip name} |
| `enter` | `try_commitment_set` returns | `layer`, `commitment`, `kind`, `n_firings`, `core` |
| `nogood` | `emit_nogood` | `clause`, `emitted`, `subsumed` |
| `writeback` | singleton `(not h)` / forced positive | `fact`, `reason` |
| `verdict` | end | type, `k`, `exhausted`, all counters, model facts |

Cost when enabled is irrelevant (it is a debugging/parity mode, not a
benchmark mode); the harness never times an `--events` run.

### Comparison

`ein-conformance diff a.jsonl b.jsonl` reports the first differing line
with a structural diff, plus a summary of divergence classes. It knows
about the *normalisations* in §5 and applies them before comparing.

---

## 4. The corpus

Derived from the repo, not invented:

| group | source | count | what it exercises |
|---|---|---|---|
| positive | `examples/**/*.ein` minus `broken/` | 53 | the whole feature surface; catalogued in [`examples/README.md`](../../../examples/README.md) |
| negative | `examples/broken/*.ein` | 4 | parse errors |
| load-negative | fixtures extracted from `ein.py/tests/**` `KBLoadError` cases | ~40 | loader validation messages |
| stdlib | `stdlib/*.ein` loaded standalone | 7 | import/macro machinery |
| golden | `ein.py/tests/golden/**` | 19 | DOT + trace + dump goldens already pinned |
| generated | `examples/gen_zebra2_variants.py` output | ~15 | clue-subset variants → Ambiguity / Contradiction verdicts |

Each corpus entry has a **run matrix** — the flag combinations to run it
under: default solve, `-e`, `-n 3`, `-m {1,2,3,5}`, `--trace`,
`--print-final-*`, `--dump-states`, plus each `SolverConfig` lever
flipped off (the same matrix `utils/feature_matrix.py` already drives).
Full cross-product is ~5 000 runs; the CI tier runs a pinned subset and
the nightly tier runs all of it.

**Corpus growth rule.** Any divergence found outside the corpus becomes a
corpus entry in the same commit that fixes it. The corpus only grows.

---

## 5. Legitimate divergences (the normalisation list)

Some outputs cannot be identical and must not be pretended to be. Each
one is normalised by the harness, and *the list is closed* — adding to
it requires an entry in [`open_questions.md`](../open_questions.md).

| what | why | normalisation |
|---|---|---|
| wall-clock numbers (`--timing`, `--stats` `wall`, `elapsed_seconds`) | that is the point of the port | matched by *field presence and format*, values elided |
| `--verbose` progress cadence lines | tied to timing | elided |
| absolute paths in messages | machine-specific | rewritten relative to repo root |
| `Loc` line/col on **synthesised** nodes | Python has `None`, Rust may too — but a real position is *better* | must be identical; a Rust improvement here is a divergence, deferred to a post-parity stage |
| DOT node ids | `hashed_id` is `md5(seed)[:10]` — portable and stable (verified 2026-08-17) | **none** — ein.rs must reproduce them exactly, including the `seed` builders |
| `state_hash.txt` / `state_digest` in messages | `hash(tuple)` is `PYTHONHASHSEED`-salted, so ein.py is not even self-stable here | compared for shape, not value ([02](02_determinism_and_order.md) §8) |
| `--shuffle` runs | RNG is `random.Random(seed)` | ein.rs must reimplement CPython's Mersenne-Twister `shuffle` **exactly** (see Q-M1a.5) or the harness skips shuffled runs at T2/T3 |
| dict iteration order | *not* a divergence — see [02](02_determinism_and_order.md) | ein.rs reproduces insertion order structurally |

The `--shuffle` row is the only one with real risk. Options in
Q-M1a.5: (a) port MT19937 + CPython's `random.shuffle` loop (≈ 60 lines,
fully deterministic, gives T3 everywhere); (b) declare shuffled runs
T0-only, since the whole point of `--shuffle` is that the verdict is
*shuffle-invariant*. (a) is cheap and is the recommendation.

---

## 6. The divergence ledger

`plans/m1a_rust/divergences.md` (created in P1a.0) records every
difference that is *accepted* rather than fixed, in the shape
`parity_baselines.md` used:

```
### D<n> — <one-line title>
**Found:** <date, phase>   **Tier:** T<k>   **Status:** accepted | fixed in <stage>
**What:** …   **Why it is acceptable:** …   **What would make it unacceptable:** …
```

An empty ledger at the P1a.5 gate is the goal. A non-empty one is
allowed only with a written reason per entry — this is the mechanism
that stops "100 %" from quietly becoming "99 % and a shrug".

---

## 7. Beyond diffing: property tests

Diffing catches what the corpus covers. Three properties cover what it
does not, and they run against **both** implementations:

1. **Round-trip.** `parse(dump(parse(x))) == parse(x)` for every corpus
   file and every generated AST (already a Python property; ported to
   `proptest` with a shared AST generator).
2. **Shuffle invariance.** `solve(-e)` verdict + model set is invariant
   under `--shuffle` seeds (already pinned by
   `tests/inference/lattice/test_shuffle_invariance.py`).
3. **Commutativity.** For an alive size-*k* commitment, every
   (*k*−1)-subset parent path saturates to the same KB
   (`lattice_sanity_check`; `monotonic/sanity.py`).

Plus a **differential fuzzer** (P1a.6): generate random small `.ein`
programs from the grammar, run both engines under a budget, diff at T1.
This is the only mechanism that finds parity bugs in the parts of the
input space no human wrote a fixture for, and it is cheap once the event
protocol exists.

---

## 8. Cross-links

- [02 — Determinism & order](02_determinism_and_order.md) — the audit
  that makes T2/T3 achievable at all.
- [11 — Shared assets](11_shared_assets.md) — why the corpus and stdlib
  must be single-sourced.
- [`docs/kernel/inference/parity_baselines.md`](../../../docs/kernel/inference/parity_baselines.md)
  — the precedent (historical, looser tier).
- [`utils/feature_matrix.py`](../../../utils/feature_matrix.py) — the
  existing lever matrix; its run matrix is reused as the corpus's.
- [P1a.0 stages](../p1a.0_conformance_harness/README.md) — where this is
  built.
