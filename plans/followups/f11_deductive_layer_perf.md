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

**Both measurements above are ein.py's, and
[S1a.6.1](../m1a_rust/p1a.6_performance/s1a.6.1_profile_baseline.md) (2026-08-18)
re-took them on ein.rs, where D1 will actually land.** What changed:

- **The 95 % split does not carry over.** On the parity build an exhaustive
  `zebra2` is 59.7 % saturation-side and 29.0 % matcher; an exhaustive `zebra`
  is 66.9 % **matcher**. So D1's lever is real but puzzle-dependent, and the
  workload that needs it is `zebra`, which is also the one still missing its
  P1a.6 target.
- **D1's catch dissolved, as this file predicted it would.** "A memory that
  must be copied per fork can lose more than it saves" was the parking reason;
  a fork's whole delta in ein.rs is **3.6 KB mean, 9 KB worst case** over 101
  enterings ([baseline.md §5](../m1a_rust/p1a.6_performance/baseline.md#5-memory)),
  so per-fork memories are affordable outright and the read-only-share
  design is an optimisation rather than a precondition.
- The gate on [S1a.6.3](../m1a_rust/p1a.6_performance/s1a.6.3_beta_memories.md)
  is therefore **open**. It runs fourth in the phase, behind two cheaper wins
  the profile found first.

Re-run the baseline before starting either entry — this has moved twice, and
four times counting the two above:

```sh
PYTHONPATH=ein.py/src python3 utils/profile_solve.py examples/zebra2.ein --exhaustive   # attribution (CPython)
.venv-pypy/bin/python utils/profile_solve.py examples/zebra2.ein --exhaustive --no-profile  # wall-clock (PyPy)
PYTHONPATH=ein.py/src .venv-pypy/bin/python utils/feature_matrix.py                    # the lever matrix
```

For ein.rs the equivalents are `utils/profile_ein_rs.py` (attribution),
`utils/e2e_baseline.py` (wall-clock) and `cargo run -p ein-infer --example
lever_matrix`; the whole set is listed in
[baseline.md § Reproducing all of it](../m1a_rust/p1a.6_performance/baseline.md#reproducing-all-of-it).

## D1 — RETE beta-memories

**Effort:** M. **Value:** H (perf) — the named next rung.

> **Measured and declined, 2026-08-19
> ([S1a.6.3](../m1a_rust/p1a.6_performance/s1a.6.3_beta_memories.md)).** The
> gated stage ran and the gate said no — not because the memory would not work,
> but because two cheaper changes removed what it was for.
>
> - **The intermediate is 2.2 tuples wide.** An exhaustive `zebra` offers
>   1 171 385 candidates over 530 405 steps entered, where before
>   [T1a.6.3.0](../m1a_rust/p1a.6_performance/baseline.md#14-s1a63--the-alpha-memory-and-the-gate-the-beta-memory-did-not-pass)
>   it offered 25 160 149 — **47.4 per step down to 2.21**. What T1a.6.3.0 did
>   was key the participation index *one level inside a nested argument*, so a
>   `(not (R ?b ?i))` premise probes a bucket instead of walking a 368-fact
>   extent. A table lookup that replaces 47 candidates is a lever; one that
>   replaces 2.2 is a constant factor with a per-fork table attached.
> - **The catch came back, with a measurement this time.** This file recorded
>   in August that the catch had "dissolved" because a fork's delta is only
>   3.6 KB. [T1a.6.2.5](../m1a_rust/p1a.6_performance/s1a.6.2_memory_layout.md)
>   then built the exact shape design/05 §7 calls the *root memory* — a flat
>   per-relation table — and **cloning it per fork cost 7.6 %** while making the
>   fork-free bench 8 % faster. Affordable in bytes is not the same as free in
>   cache: a fork *shares* the layered index by `Arc`, and a materialised table
>   gives every live fork its own copy.
> - **The re-derivation it would have replayed is gone.**
>   [S1a.6.9](../m1a_rust/p1a.6_performance/s1a.6.9_fork_entry_delta.md) made a
>   fork resume root's saturation rather than re-derive it, which was the
>   "root memories" task's whole purpose.
>
> D1 stays **open but re-priced**: it is no longer the largest remaining lever
> (Q-M1a.10, answered *no*), and a future promotion needs a workload where the
> per-step candidate count is large again — a wider rule body, a bigger extent,
> or a puzzle whose premises the index cannot key.

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
patterns. This is **conditional work**: open it only when a rule set with a
genuine cycle shows up *and* profiles badly.

> **Half the trigger is already met, 2026-08-19.** "Ein's rule bodies are
> currently acyclic (chains and stars)" is **false**, and S1a.6.3's re-check is
> what caught it: `stdlib/slots.ein`'s `slot-adjacent-fwd` binds
> `(?S ?a ?b) (?R ?b ?p1) (?isa ?p1 ?PT) (?S ?p2 ?p1) (?isa ?p2 ?PT)`, whose
> variable graph contains the triangle `p1 — PT — p2 — p1`. The *cost* half is
> not met: those relations hold 30 and 16 facts on `zebra`, matching is 37.7 %
> of a 78 ms run, and a binary plan is within a small constant of the AGM
> bound at that size. So D2 stays closed — but the next re-check should ask
> about the cost, not re-derive the shape.

**Trigger.** A compiled `JoinPlan` whose step graph is cyclic *and* whose
match cost dominates a solve.

**Prior art.** Atserias–Grohe–Marx bound; Ngo–Porat–Ré–Rudra (2012);
Veldhuizen, *Leapfrog Triejoin* (2014);
[docs/lib/02 — solvers / CSP / SAT / SMT](../../docs/lib/02-solvers-csp-sat-smt.md).

## Promotion criteria

Promote into a perf phase (not into this file) when **any** holds:

- a user-facing workload — M2's NL output, M1b's GUI turnaround, a bigger
  puzzle corpus — exceeds its ergonomic time envelope *and* a fresh profile
  puts the matcher back on top;
- the [M1a Rust port](../m1a_rust/) reaches the matcher and wants the
  final algorithm rather than a transcription of the Python one (the port
  is the most likely trigger, and the reason this file exists rather than
  being dropped);
- a cyclic rule body appears **and its match cost dominates** (D2's specific
  trigger — the body exists already, see D2).

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
