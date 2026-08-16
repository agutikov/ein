# P1.21 — Review response: REVIEW_M1-01 (claims vs guarantees)

**Estimate:** ~1–2 weeks (actual: 2 days).
**Status:** ✅ **core executed 2026-08-16/17** — all six review points
investigated (6 confirmed reports), improved, adversarially verified, and
the acceptance gate below passed 2026-08-17 (suite 1302 passed + 1
sanctioned strict-xfail (D5); acceptance 17/17, verdicts unchanged; ruff
clean; closeout greps clean). Both follow-up stages were then **activated
and executed 2026-08-17**:
[S1.21.7](s1.21.7_multi_justification_provenance.md) (multi-justification
provenance + the companion contradiction-answer wiring) and
[S1.21.8](s1.21.8_boundary_naf.md) (purely-positive closure + boundary NAF),
which also closed divergences **D3** and **D5**. Gate after both: 1342 unit
tests, **zero** xfails, acceptance 17/17 verdicts unchanged, `ruff` clean —
and faster than before (acceptance 130 s → 91 s). Open remainder: the
[§Divergences](#divergences-surfaced-by-investigation-2026-08-16)
one-liner D-R5-1.
**Source:** [`../REVIEW_M1-01.md`](../REVIEW_M1-01.md) — an external
architecture review of `master` *as a reasoning engine*. Its core thesis:
the architecture is right (monotone deductive layer + non-monotone search
layer; one `solve()`; ~8/10), but several **formal claims are stronger than
the implementation's guarantees** (*minimal*, *canonical hash*, *CDCL*,
*unconditional*; ~6/10 as a sound solver framework).
**Depends on:** [P1.7a](../p1.7a_solution_search_refactor/README.md) (the
sound `solve()` baseline), [P1.5b](../p1.5b_lattice_search/README.md) (the
lattice engine under review).
**Blocks:** recommended before the [M1a Rust port](../../m1a_rust/README.md)
(don't transcribe an unsound identity or a dormant-unsound API into `ein.rs`)
and informs [M3](../../m3_smt_integration/README.md) (the review's §6 seam is
explicitly motivated by SMT integration).

## Why this phase exists

REVIEW_M1-01 found no need to change the core architecture. What it found is
a **truth-in-labelling debt** with one genuine soundness bug at the top:

1. **§1 (P0)** — `state_hash()` (a Python `int`) is used as *model identity*
   for solution-node dedup. A hash collision silently merges two distinct
   models; since the verdict is defined by the number of distinct models `k`,
   a collision flips `Ambiguity` (k=2) into `Solution` (k=1). Soundness bug,
   not a perf bug.
2. **§2 (P0)** — `commitment.py` still computes + documents
   `unconditional_facts` ("provably true at root"), and
   [`docs/kernel/inference/README.md`](../../../docs/kernel/inference/README.md)
   still describes merging them into root — while the live solver
   ([`solver.py:376`](../../../ein.py/src/ein/inference/monotonic/solver.py))
   correctly refuses to merge because the extraction is **unsound under NAF**
   (`absent`-derived facts carry no positive provenance edge to the
   hypothesis they depend on). Code contradicts docs; the dormant API is
   conceptually dangerous.
3. **§3 (P1)** — `minimal_unsat_core` returns the *smallest
   single-contradiction source frontier*, not a subset-minimal MUS; and since
   the KB stores one `Fact` per `(relation, args)` with **first-derivation
   provenance only**, even "smallest over all derivations" isn't guaranteed.
   The top-level `README.md` promises "minimal unsat core".
4. **§4 (P1)** — `absent` (NAF) semantics is the load-bearing boundary of the
   whole system (it forced no-backprop, fire-time `AbsentGuard` re-eval,
   NAF-safe MUS, semi-naive trigger care) but has **no formal definition
   document**. The review's candidate reading: branch-relative epistemic —
   `absent(P)` ⇔ `KB_C ⊭ P`, a query over the current saturated
   world/environment, not a ground atom.
5. **§5 (P2)** — the lattice search is positioned "≈ CDCL" in docs; the
   mechanism is actually **ATMS-style environment search + Apriori candidate
   generation + nogood (superset) pruning**. CDCL should be an analog /
   optimization direction, not a description.
6. **§6 (P2)** — the review proposes an explicit architectural seam
   (closure ∥ worlds-lattice → complete model → canonical key →
   models/refutations) with **NAF on the closure/world boundary** — worth
   recording as the target picture for P1.9 optimization and M3.

## Structure — every review point = two tasks

Per the phase charter, each review point is processed as **two tasks**:

- **`T1.21.N.1` — investigation + report.** Read-only. Verify the review's
  claim against the code (confirm / partial / refute, with `file:line`
  evidence), enumerate every affected site (code, docs, tests, dumps),
  weigh the fix options, and write
  [`reports/rN_*.md`](reports/) recommending one.
- **`T1.21.N.2` — improvement.** Execute the recommendation from the report.
  Gated on its own report only — never on other points' tasks.

| ID | review § | prio | stage | tasks |
|---|---|---|---|---|
| S1.21.1 | §1 | **P0** | [`state_hash` → collision-safe canonical `StateKey`](s1.21.1_state_identity.md) | T1.21.1.1 / T1.21.1.2 |
| S1.21.2 | §2 | **P0** | [Retire `unconditional_facts`; sync docs to the NAF-safe model](s1.21.2_unconditional_facts.md) | T1.21.2.1 / T1.21.2.2 |
| S1.21.3 | §3 | P1 | [`minimal_unsat_core` — rename or make minimality real](s1.21.3_min_core_naming.md) | T1.21.3.1 / T1.21.3.2 |
| S1.21.4 | §4 | P1 | [Formal semantics of `absent` — dedicated doc](s1.21.4_absent_semantics.md) | T1.21.4.1 / T1.21.4.2 |
| S1.21.5 | §5 | P2 | [Reposition lattice search: ATMS-style, CDCL as analog](s1.21.5_lattice_positioning.md) | T1.21.5.1 / T1.21.5.2 |
| S1.21.6 | §6 | P2 | [Record the closure/worlds seam; NAF at the boundary](s1.21.6_architecture_seam.md) | T1.21.6.1 / T1.21.6.2 |
| S1.21.7 | §3 (b) | follow-up | [Multi-justification (OR/AND) provenance + true minimal explanation](s1.21.7_multi_justification_provenance.md) | ✅ **executed 2026-08-17** (moved in from P1.9 E25; companion wiring shipped with it — `zebra2-bad` core 38 → 1) |
| S1.21.8 | §6 | follow-up | [Purely-positive closure + boundary NAF re-eval](s1.21.8_boundary_naf.md) | ✅ **executed 2026-08-17** (moved in from P1.9 E26; both activation gates met — D3 + D5 were the measured signal. Fixed both; engine got faster) |

## Scheduling

- **All six investigations (`T1.21.*.1`) are independent** — read-only, each
  writes its own `reports/rN_*.md` — and run **in parallel**.
- Each improvement (`T1.21.N.2`) starts once its report exists. Improvements
  are batched into **waves of file-disjoint changes** (the reports declare
  the files each will touch); overlapping ones serialise in priority order
  P0 → P1 → P2. Known overlap: S1.21.2/.3/.5/.6 all touch
  `docs/kernel/inference/README.md`.
- **Gate after every wave and at close:** `./run_tests.sh` (the full gate —
  unit suite + acceptance fixtures; never bare `pytest -q`) + `ruff check .`
  green, and verdict/bindings identical on the acceptance fixtures.

## Acceptance

1. **Six reports** exist under [`reports/`](reports/), each with a
   confirm/partial/refute verdict and `file:line`-cited evidence.
2. **P0s closed:** solution-node identity is a canonical `StateKey`
   (equality-checked; any `int` digest demoted to accelerator with equality
   verification); model identity (`(R, args)` vs layer) is *explicitly
   defined* in code + docs. `CommitmentSetResult.unconditional_facts` /
   `_is_unconditional` are removed (or re-scoped under an explicitly
   documented soundness precondition, if a live sound consumer was found);
   no doc claims the root-merge anymore.
3. **P1s closed:** the API name and every doc promise state exactly what
   `minimal_unsat_core` guarantees (rename, or provenance grew
   multi-justification and minimality is real — per report); a dedicated
   `absent`-semantics doc exists, is cross-linked from the kernel docs, and
   its stated semantics is pinned by tests.
4. **P2s closed:** no doc describes the mechanism *as* CDCL / the docs use
   the ATMS+Apriori+nogood positioning with CDCL as analog; the §6 seam is
   recorded in the kernel docs with a module mapping and its M3 implication.
5. **Suite green:** `./run_tests.sh` + `ruff check .`; behaviour unchanged
   on all fixtures (the only sanctioned semantic change is the P0 identity
   fix, which by construction only matters on hash collision).

## Out of scope (deferred)

- Lattice-search *optimization* (propagation, conflict minimization,
  variable/value selection, backjumping) — the review's item 5 explicitly
  sequences it **after** this phase; it stays in
  [P1.9](../p1.9_hypothesis_loop_followups/README.md) (E-catalog).
- ~~Multi-justification proof DAG *implementation*~~ — was parked **in-phase**
  as [S1.21.7](s1.21.7_multi_justification_provenance.md) (moved from P1.9
  E25 on 2026-08-17) and **executed the same day**, so it is no longer out of
  scope; likewise the boundary-NAF track,
  [S1.21.8](s1.21.8_boundary_naf.md) (ex-E26) — new work is never scheduled
  into previous phases.
- Any `absent` stratification checker — S1.21.4 documents semantics; a
  static stratification/dependency analysis is future work.

## Divergences surfaced by investigation (2026-08-16)

New engine-vs-semantics divergences found by the R4 investigation
([`reports/r4_absent_semantics.md`](reports/r4_absent_semantics.md) §2,
probes P6/P8) and — per the improvement gate ("behaviour unchanged; any
divergence is reported, not silently fixed") — **recorded here, not
fixed** by T1.21.4.2. Both are documented in
[`docs/kernel/inference/absent_semantics.md` §Known divergences](../../../docs/kernel/inference/absent_semantics.md#known-divergences-open-questions-p121)
and pinned in
[`tests/inference/test_absent_semantics.py`](../../../ein.py/tests/inference/test_absent_semantics.py).
D-R5-1 was surfaced by the R5 investigation and is likewise recorded
here, not fixed by the docs-only T1.21.5.2.

- **D3 — lookahead evaluated AbsentGuards against a world without the
  candidate.** ✅ **FIXED 2026-08-17 by [S1.21.8](s1.21.8_boundary_naf.md).**
  [`lookahead.py`](../../../ein.py/src/ein/inference/lookahead.py) posited
  the candidate `h` into one positive premise but ran the rule's *other*
  premises — including `AbsentGuard`s — against `kb` **without** `h`, so the
  NAF queried a different world than the probe hypothesised. A rule whose
  guard watched the candidate's own relation (probe P6) therefore killed a
  live hypothesis, violating the lookahead's own "never reports a live
  hypothesis as dead" docstring; with the default-on kill cache writing
  `(not h)` back to the parent, that was verdict-affecting in principle.
  The guard is now evaluated in the world **with** `h` (no match in `kb`
  *and* `h` creates none); a guard containing a nested absent is
  non-monotone and cannot be decided that cheaply, so the lookahead skips
  that disjunct rather than guessing — which only loses a kill, the safe
  direction. `test_lookahead_naf_world_excludes_candidate` flipped to
  `test_lookahead_naf_world_includes_candidate`.
- **D5 — or-disjunct `AbsentGuard`s skipped fire-time re-eval.**
  ✅ **FIXED 2026-08-17 by [S1.21.8](s1.21.8_boundary_naf.md)** — and it was
  one of that stage's activation signals, not a separate point fix.
  [`match.absents_still_pass`](../../../ein.py/src/ein/inference/match.py)
  walked only `plan.steps` while `match.run` also yielded from the S1.8.A13
  `plan.extra_match_plans` or-disjuncts, so a firing admitted via a
  disjunct whose `(absent …)` had since flipped committed on a stale
  verdict (probe P8). The candidate 3-line fix ("walk
  `extra_match_plans` too") was **not** taken: it would have left the same
  shape one tuple away from breaking again. Instead guards are lifted **per
  disjunct** into `plan.naf_guards`, and `match.run_guarded` yields every
  match paired with its own disjunct's guards — there is no longer a tuple
  a caller can forget. `absents_still_pass` is deleted. The
  `xfail(strict=True)` pin now passes, and the suite has zero xfails.
- **D-R5-1 — `_handle_dead` NameError with `enable_path_nogoods=False`
  and a dumper attached** (from the R5 investigation,
  [`reports/r5_positioning.md`](reports/r5_positioning.md) §2).
  [`_helpers.py:434`](../../../ein.py/src/ein/inference/monotonic/_helpers.py)
  assigns `landed` only inside the `if ctx.cfg.enable_path_nogoods:`
  gate, but the dumper call reads it unconditionally at `:456–457`
  (`nogood_emitted=landed, nogood_subsumed=not landed`) — any dead
  entering under that config raises `NameError`. Candidate fix (one
  line): initialise `landed = False` before the gate. Behaviour change
  on an off-default diagnostic path, so out of the docs-only T1.21.5.2
  — left for the acceptance gate / follow-up.
