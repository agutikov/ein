# F11 — Deductive-layer perf (the matcher, not the search)

**What.** The remaining engine-performance work, all of it in the
*deductive* layer: making a rule firing cheaper. Two entries, both named
by the Datalog/RETE literature rather than invented here — D1 (RETE
beta-memories) and D2 (worst-case-optimal joins).

**Why it's a followup and not a phase.** No workload is currently blocked
on it. `zebra2` solves in ~1.3 s fast / ~3.9 s exhaustive under PyPy, which
is inside every ergonomic envelope the project has. This is the file to
promote when one isn't.

**Provenance.** Recorded 2026-06-15 inside the F9 E-catalog as 📌 rows —
"for visibility, not a P1.9 entry" — because F9 was a *hypothesis-loop*
backlog and these are saturation items. Moved here 2026-08-17 when F9
closed: with the search-layer catalog exhausted, these are the live perf
work and deserve their own theme rather than a footnote in a ledger.

## Where the time goes (measure before touching either entry)

Two measurements, both current as of 2026-08-17:

- **P1.8a's split still holds: ~95 % of a solve is the matcher inside
  saturation** (O1+O2), not the search
  ([`architecture_and_algorithms.md`](../../docs/kernel/inference/architecture_and_algorithms.md)
  §7). Every *search*-layer optimisation was tried and rejected against
  that split ([F9 ledger](f9_e_catalog.md)).
- **S1.9.E23 removed the largest chunk of pointless matching**: over half
  the exhaustive wall-clock was saturating forks already known to be dead.
  What's left is saturation of forks that genuinely live — i.e. the matcher
  doing work that has to happen. That makes D1 the largest remaining lever
  and, unlike before, the *only* one with a clear ceiling story.

Re-run the baseline before starting either entry — this has moved twice:

```sh
PYTHONPATH=ein.py/src python3 utils/profile_solve.py examples/zebra2.ein --exhaustive   # attribution (CPython)
.venv-pypy/bin/python utils/profile_solve.py examples/zebra2.ein --exhaustive --no-profile  # wall-clock (PyPy)
PYTHONPATH=ein.py/src .venv-pypy/bin/python utils/feature_matrix.py                    # the lever matrix
```

## D1 — RETE beta-memories

**Effort:** M. **Value:** H (perf) — the named next rung.

Persist **partial joins** across firings. The optimisation arc so far has
walked up the Datalog ladder — naive → semi-naive (participation index =
alpha-memory; delta-driven; seeded delta join, P1.8a, ~3.6×) — and the one
thing the seeded delta join still recomputes is the intermediate join
result. Beta-memories are exactly that missing rung: materialise
`(plan, prefix-of-steps) → bindings` and update it incrementally per
derived fact.

**Where.** `inference/saturator.py` (the enqueue pass), `match.py`
(`_run_steps`), `engine.py` (plan cache — the natural owner of a per-plan
memory).

**The catch to design against.** A beta-memory is per-KB state, and this
engine forks KBs constantly (one per commitment). A memory that must be
copied per fork can lose more than it saves — P1.8a's D3 (cross-fork
carry) was built and reverted the same day as a measured wash for
adjacent reasons. Sketch the fork story *first*: share the root's memories
read-only and keep only a fork-local delta, or accept per-fork rebuild and
measure whether the within-fork reuse pays for it.

**Prior art.** Forgy, *Rete: A Fast Algorithm for the Many
Pattern/Many Object Pattern Match Problem* (1982);
[docs/lib/06 — graphs & rewrite systems](../../docs/lib/06-graphs-rewrite-systems.md).

## D2 — Worst-case-optimal join

**Effort:** L. **Value:** L until the shape appears.

Leapfrog-Triejoin / Generic-Join: a join order-free algorithm whose runtime
matches the AGM bound, beating any binary-join plan on **cyclic** join
patterns. Ein's rule bodies are currently acyclic (chains and stars), where
a good binary plan is already optimal — so this is **conditional work**:
open it only when a rule set with a genuine cycle (e.g. a triangle over
three relations) shows up and profiles badly.

**Trigger.** A compiled `JoinPlan` whose step graph is cyclic *and* whose
match cost dominates a solve.

**Prior art.** Atserias–Grohe–Marx bound; Ngo–Porat–Ré–Rudra (2012);
Veldhuizen, *Leapfrog Triejoin* (2014);
[docs/lib/02 — solvers / CSP / SAT / SMT](../../docs/lib/02-solvers-csp-sat-smt.md).

## Promotion criteria

Promote into a perf phase (not into this file) when **any** holds:

- a user-facing workload — M2's NL output, an M3 SMT slice, a bigger puzzle
  corpus — exceeds its ergonomic time envelope *and* a fresh profile puts
  the matcher back on top;
- the [M1a Rust port](../m1a_rust/) reaches the matcher and wants the
  final algorithm rather than a transcription of the Python one (the port
  is the most likely trigger, and the reason this file exists rather than
  being dropped);
- a cyclic rule body appears (D2's specific trigger).

## Cross-links

- [F9 ledger](f9_e_catalog.md) — the closed *search*-layer catalog; read
  its "what the catalog taught" section before opening a perf entry here.
- [F10 — M1 refactor-debt tail](f10_m1_refactor_tail/README.md) — structural
  debt to clear **before** the Rust port; overlaps this file's second
  promotion trigger.
- [`architecture_and_algorithms.md`](../../docs/kernel/inference/architecture_and_algorithms.md)
  §O1/§O2/§7 — the operation-by-operation SOTA comparison these two entries
  are the open rows of.
- [`features.md`](../../docs/kernel/inference/features.md) — the measured
  lever matrix; regenerate it alongside any perf change.
