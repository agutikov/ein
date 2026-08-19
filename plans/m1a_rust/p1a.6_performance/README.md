# P1a.6 — Performance

**Milestone:** [M1a — Rust port](../README.md)
**Status:** **in progress** — [S1a.6.1](s1a.6.1_profile_baseline.md) and
[S1a.6.8](s1a.6.8_compile_cache_and_extents.md) shipped 2026-08-18;
[S1a.6.9](s1a.6.9_fork_entry_delta.md), [S1a.6.10](s1a.6.10_parity_contract.md),
[S1a.6.11](s1a.6.11_fixture_goldens.md), [S1a.6.2](s1a.6.2_memory_layout.md),
[S1a.6.3](s1a.6.3_beta_memories.md),
[S1a.6.4](s1a.6.4_hypgen_and_lattice.md) and
[S1a.6.5](s1a.6.5_frontend.md) on 2026-08-19. **All four targets are met with
room** — the tightest by 4.9×. S1a.6.4 found that the phase had been measuring
one shape of workload: the four targets are all `(hrule …)`-driven and never
run the blind enumerator, which is 15 % of the corpus's slowest solves.
S1a.6.5, a one-day confirmation of a path already 8× inside its acceptance,
found a load parsing **3.30× the bytes on disk** and took **25 %** off it.
Next is [S1a.6.12](s1a.6.12_boundary_and_snapshot.md), which the profile has
been naming since S1a.6.3 and which is now **written against its own
measurement** ([§17](baseline.md#17-the-boundary-measured-before-the-stage-that-aims-at-it)):
the **NAF boundary** (37.7 % of `zebra -e` cumulatively, of which a *third* is
not the queries) and the **per-entering snapshot** (10.3 %). That measurement
also closes [Q-M1a.17](../open_questions.md#q-m1a17--win-bs-80--assumed-monotone-guards-dominate)
— design/06 § Win B's headline mechanism reaches **7.3–15.3 %** of guard
evaluations where it projected ≥ 80 %, so the stage spends its days on the
refinement that never landed instead. The measurements are in
**[baseline.md](baseline.md)**: §1–§9 are what the phase is chosen by,
§10–§16 are where it stands, §17 is what it does next.
**Estimate:** 5 weeks (30 days of stages — S1a.6.1 added one worth 2 d and
shortened another by 1 d; [S1a.6.9](s1a.6.9_fork_entry_delta.md) added 3 d,
and its decision added [S1a.6.10](s1a.6.10_parity_contract.md) and
[S1a.6.11](s1a.6.11_fixture_goldens.md) at 2 d each; every re-measurement since
S1a.6.3 named the same next stage and
[S1a.6.12](s1a.6.12_boundary_and_snapshot.md) is it, at 4 d)
**Depends on:** [P1a.5](../p1a.5_presentation/README.md) — the byte gate
must be closed first, so every change here is measured against a green
harness.
**Absorbs:** [F11 — deductive-layer perf](../../followups/f11_deductive_layer_perf.md)
(D1 beta-memories, D2 WCOJ).

## Goal

Turn the parity build into a fast one, with the **answer** identical at every
step — and, until [S1a.6.9](s1a.6.9_fork_entry_delta.md), T3 green at every
step too. That clause is where the phase learned something: see Rule 1.
Method is fixed and non-negotiable: **profile, change one thing, re-diff,
re-measure, record.** A change that cannot be attributed is reverted.

## Targets

Against PyPy on the same machine, at `--jobs 1`. **The PyPy column is
re-measured** ([S1a.6.1](s1a.6.1_profile_baseline.md) T1a.6.1.5) — the
numbers the phase was planned with were up to a year old and two of them
moved:

| workload | PyPy today | target | at S1a.6.1 | at S1a.6.8 | at S1a.6.9 | at S1a.6.2 | at S1a.6.3 | at S1a.6.4 | **at S1a.6.5** |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `solve zebra2.ein -e` end-to-end | 4.53 s | ≤ 0.20 s (≥ 20×) | 198.8 ms ✅ | 138.1 ms ✅ | 99.1 ms ✅ | 75.8 ms ✅ | 44.0 ms ✅ | 41.7 ms ✅ | **40.8 ms ✅ 111×** |
| `solve zebra.ein -e` end-to-end | 8.33 s | ≤ 0.40 s | 585.8 ms ❌ | 539.9 ms ❌ | 397.2 ms ✅ | 349.1 ms ✅ | 78.1 ms ✅ | 77.1 ms ✅ | **76.9 ms ✅ 108×** |
| parse + load `zebra2.ein` | 0.43 s ¶ | ≤ 0.015 s (≥ 50×) | 1.04 ms ✅ | 1.01 ms ✅ | 1.01 ms ✅ | 0.90 ms ✅ | 0.90 ms ✅ | 0.89 ms ✅ | **0.66 ms ✅ 648×** |
| the acceptance gate (3 fixtures) | 36.0 s ‡ | ≤ 5 s | 1.27 s ✅ | 1.02 s ✅ | 0.62 s ✅ | 0.58 s ✅ | 0.28 s ✅ | 0.20 s ✅ | **0.20 s ✅ 184× ⁋** |

**All four targets are met with room**, seven stages into the phase. The one
that needed it was `solve zebra.ein -e`, and what met it was
[S1a.6.9](s1a.6.9_fork_entry_delta.md) — a fork resuming root's saturation
instead of re-deriving it, which is also the first change in the port where
matching ein.py byte for byte and building the better engine pulled apart.
[S1a.6.2](s1a.6.2_memory_layout.md) took another 23.5 % and 12.1 % off the two
`-e` cells; [S1a.6.3](s1a.6.3_beta_memories.md) then took **4.5×** off
`zebra -e` with an index key, and the tightest target now has **80 % of
headroom** where it had 0.7 %. Both `-e` cells are **111×** and **108×** the
PyPy column above — [baseline.md §16](baseline.md#16-s1a65--the-load-path-and-the-modules-it-parsed-twice)
divides by [§1](baseline.md#1-end-to-end-process-against-process)'s PyPy run
instead and reads 121× / 114×, which is the same ein.rs number against a
5 %-slower reading of the same interpreter.

**The parse + load row is the one S1a.6.5 moved**, from 0.89 to 0.66 ms — and
it moves every *process*, not just that cell: 0.23 ms off `saturate zebra2`
(3.9 → 3.6 ms) and off each of the 473 cells the harness runs per tier.

The phase's subject has moved with them. `zebra -e` was 84.8 % matcher at the
end of S1a.6.2 and is **38.6 %** now; what is left is the NAF boundary
(`admit_from_boundary`, **37.7 %** *cumulatively*) and the per-entering
snapshot (`Saturator::resume`, 10.3 %) — the same two on the blind-mode
workloads S1a.6.4 added, at 28.2 % and 12.4 %, and both are
[S1a.6.12](s1a.6.12_boundary_and_snapshot.md).

**And the targets are not the whole corpus.** All four are
`(hrule …)`-driven, so none of them runs the blind enumerator at all;
[S1a.6.4](s1a.6.4_hypgen_and_lattice.md) found that the corpus's slowest
`solve` cells all do — `features/05_stdlib_domain_elim -e` is 46× `solve
zebra -e` — and took **15 %** off the two largest. `utils/e2e_baseline.py
--blind` is that row set, and
[baseline.md §15](baseline.md#15-s1a64--the-per-call-setup-and-the-enumerator-the-targets-never-run)
is where it stands.

The planned PyPy column was 4.07 s / 8.15 s / 0.78 s / ~91 s; two of the four
moved when re-measured, which is why the table carries today's.

Much of this should already be true when the phase *starts* — the
register matcher, integer facts, O(1) forks, compile-once and the
semi-naive boundary all land in P1a.2–3. The phase exists to find what is
left, not to assume it.

‡ The planned ~91 s was the *21-test* gate; 36.0 s is the three fixtures the
ein.rs column covers, and the whole gate is 49.3 s today. Three recorded
values of that number so far, all real —
[baseline.md §6](baseline.md#6-cargo-bench--variance-and-the-acceptance-gate).

⁋ **Measured differently from the columns left of it**, and so is its 0.199 s
predecessor: S1a.6.4 timed the three `ein-infer` acceptance tests from their
own binary, unpinned, best of three, at *both* ends of its own A/B (0.199 →
0.196 s). The delta is what that stage claims; the absolute is what the method
gives, and it is the fourth recorded value of a quantity §6 already says has
three.

¶ The planned 0.78 s is not reproducible from its own components on either
interpreter; see
[baseline.md §1](baseline.md#where-the-milestones-denominators-moved). The
target is met on any reading — a whole `saturate zebra2` *process* is 5.0 ms.

**Three of the four were met on day one; the fourth was met at
S1a.6.9.** `solve zebra.ein -e` went 585.8 → 539.9 → **397.2 ms** against a
≤ 400 ms target, and its profile was then **80.5 % matcher** — the two puzzles
finally agreeing about what to optimise next, and it was the join. Two stages
later it is **78.1 ms and 37.7 % matcher**: S1a.6.2 took the allocator and the
fact-store indirection out, and S1a.6.3 found that the join's real problem was
an index that did not key inside a nested argument.

**And it now has a named cause.** 95.0 % of `zebra -e` is inside
`try_commitment_set`, and **94.6 %** of what a fork does there is
re-deriving the root's fixpoint, once per entering — measured at
[baseline.md §9](baseline.md#9-the-fork-entry-re-derivation), which is
where [S1a.6.9](s1a.6.9_fork_entry_delta.md) comes from. Removing it
outright is *observable*
([Q-M1a.18](../open_questions.md#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint));
removing it invisibly is what [S1a.6.8](s1a.6.8_compile_cache_and_extents.md)
and [S1a.6.3](s1a.6.3_beta_memories.md) do to its two halves.

**Removing it outright has now been built and measured**
([§11](baseline.md#11-the-resumed-fork-saturator-measured), off by default):
`zebra -e` **392.6 ms**, so the missed target is a decision rather than an
engineering problem. What the decision costs is that 90 002 facts would record
a different — equally valid — one of their derivations as the primary, which
is [Q-M1a.18](../open_questions.md#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint)'s
to weigh.

## Stages

Everything after S1a.6.1 is *chosen* by the table S1a.6.1 produces. The
list was the expected shape, not a commitment — and the table changed it: **two
stages were added**, one **shortened**, one **un-gated**, and the run order
is now the profile's rather than the plan's. The reasoning is in
[baseline.md §8](baseline.md#8-what-this-chooses-for-the-rest-of-the-phase);
the numbers each row rests on are in [§7](baseline.md#7-the-top-five-costs)
and [§9](baseline.md#9-the-fork-entry-re-derivation).

| # | stage | title | est. | why now |
|---|---|---|---|---|
| 1 | [S1a.6.1](s1a.6.1_profile_baseline.md) ✅ | Fresh profile and bench baseline | 2 d | **shipped 2026-08-18** |
| 2 | [S1a.6.8](s1a.6.8_compile_cache_and_extents.md) ✅ | The compile cache and the extent counts | 2 d | **shipped 2026-08-18** — −30.5 % / −7.8 %, `plan_compile` 17 430 → 305, T3 unchanged |
| 3 | [S1a.6.9](s1a.6.9_fork_entry_delta.md) ✅ | The fork-entry delta — the resumed saturator | 3 d | **shipped 2026-08-19** — fork firings −74 % / −77 %, fork compiles → 0, **`zebra -e` 394.2 ms: the last target met**. Q-M1a.18 answered: ein.rs resumes, ein.py does not, [D3](../divergences.md) records it. `--trace` gained a *Before any assumption* section |
| 4 | [S1a.6.10](s1a.6.10_parity_contract.md) ✅ | The parity contract relaxes | 2 d | **shipped 2026-08-19** — one rule in `ein-parity` replaces six ad-hoc cuts, `--strict` puts them all back. **T3 472/473 and T2 239/240, D2 the only cell in either** (T2 was 142 the day before). The T2 cut was chosen by running six candidates over the captured logs |
| 5 | [S1a.6.11](s1a.6.11_fixture_goldens.md) ✅ | ein.rs fixture goldens | 2 d | **shipped 2026-08-19** — twelve goldens over real solves (trace, `slice` cone, a fork's own dump, the snapshot, the event stream), idea-08's walkthrough assertion ported to ein.rs and un-gated, and `./run_tests.sh` gained a **Phase 3** so the repo's gate runs both engines: 1 506 + 21 + 302 green |
| 6 | [S1a.6.2](s1a.6.2_memory_layout.md) ✅ | Memory layout | 3 d | **shipped 2026-08-19** — −23.5 % / −12.1 %, on **two** of eight tasks: the `snmalloc` global allocator and a *bigger* row with two arguments inline. **Five were closed by measurement rather than by code**, and one was built and reverted at +7.6 %. [§13](baseline.md#13-s1a62--the-layout-stage-and-the-profile-it-starts-from) |
| 7 | [S1a.6.3](s1a.6.3_beta_memories.md) ✅ | Beta-memories (F11 D1) — **gate closed** | 4 d | **shipped 2026-08-19 without the memory.** The index now keys one level *inside* a nested argument (T1a.6.3.0): candidates 25.16 M → **1.17 M**, `zebra -e` **349 → 78 ms**, T2 239/240. Then a per-layer Bloom filter, −7.3 %. The gate says no to the memory: the intermediate it would materialise is **2.2 tuples wide**, and a per-fork copy of one measured **+7.6 %** at T1a.6.2.5. [F11 D1](../../followups/f11_deductive_layer_perf.md) re-priced, **Q-M1a.10 answered *no***, D2's trigger re-checked — the cyclic body exists, the cost does not |
| 8 | [S1a.6.4](s1a.6.4_hypgen_and_lattice.md) ✅ | Hypgen and lattice hot paths | 3 d | **shipped 2026-08-19, aimed elsewhere by its own measurement.** A call offers **125** raw candidates, not 18 k, and spends **71 %** of a `complete()` on setup — 219 compile-cache keys per call. Two new tasks took that; T1a.6.4.2/3 took **15 %** off the blind-mode cells no target covers. **Three planned tasks closed against numbers**: intern-on-probe (probe *is* intern here), the no-good bitmask (**0.3 %** in the regime design/07 §4 said it would dominate), incremental alive (**6** calls a solve) |
| 9 | [S1a.6.5](s1a.6.5_frontend.md) ✅ | Frontend and load path — **shortened** | 1 d | **shipped 2026-08-19** — the confirmation found a load parsing **3.30× the bytes on disk**, because resolution parses a module once per *edge* of a diamond. `load/zebra2` **−25.5 %**, `parse/zebra2_resolve` −31.6 %; two of its six tasks proposed pre-sizing and **both lost** at this scale. Start-up is 1.02 ms, 0.59 ms of it snmalloc's |
| 10 | [S1a.6.12](s1a.6.12_boundary_and_snapshot.md) | The NAF boundary and the per-entering snapshot | 4 d | **next**, and written against [§17](baseline.md#17-the-boundary-measured-before-the-stage-that-aims-at-it). The only two blocks left above 10 % on **both** workload shapes — `admit_from_boundary` 37.7 % / 28.2 % cumulative, `Saturator::resume` 10.3 % / 12.4 % — and the split inside the first is the finding: **a third of the boundary is not the queries**, it is visiting 248 043 parked candidates to ask 29 865 questions. Q-M1a.17 closes against Win B's headline (7.3–15.3 %, not ≥ 80 %); the snapshot is the change S1a.6.9 declined at **0.6 %** and is now 17× that |
| — | [S1a.6.6](s1a.6.6_differential_fuzzer.md) | The differential fuzzer | 3 d | runs *throughout*, not at a position — it guards every row above |
| 11 | [S1a.6.7](s1a.6.7_relever_matrix.md) | Re-measure the lever matrix | 1 d | last, as planned |

## Rules for this phase

1. **The answer stays identical, and T3 stays green.** ~~A perf change that
   needs a ledger entry is not a perf change~~ — **amended 2026-08-19**, by
   the decision on
   [Q-M1a.18](../open_questions.md#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint).
   The previous phases built ein.rs equal enough to ein.py byte for byte; from
   here the hard requirement is that **the final solutions are identical** —
   the verdict, `k`, the models, the query bindings, the unsat core, and every
   counter in `summary.json`, which is T0 and T1 in full. Byte-identical
   *narration* is no longer the gate:
   [S1a.6.9](s1a.6.9_fork_entry_delta.md) trades it for a quarter of the
   firings and the last unmet target, [D3](../divergences.md) records the
   trade, and [S1a.6.10](s1a.6.10_parity_contract.md) teaches the harness the
   new line. A change that moves an **answer** is still not a perf change.
2. **One change per commit, with its number.** The commit message
   carries the before/after for the benchmark it targeted.
3. **A wash is a revert.** P1.8a's D3 cross-fork carry was built and
   reverted the same day; that is the standard.
4. **No search-layer re-litigation.** [F9](../../followups/f9_e_catalog.md)
   measured that cluster inert against a complete cardinality-BFS. Rust
   does not change the branch count.
5. **Record everything** in
   [design/README.md § Measured](../design/README.md#measured) and, for this
   phase's own tables, [baseline.md](baseline.md). Every instrument is
   re-runnable by one command; the list is in
   [baseline.md § Reproducing all of it](baseline.md#reproducing-all-of-it).
6. **Re-measure before choosing the next stage.** S1a.6.1's own finding is
   that the profile does not look like the one the phase was planned against,
   and there is no reason to expect it to hold still after two stages either.
   Every stage ends by re-running the S1a.6.1 instruments.

## Acceptance for the phase

- Targets met, or a written account of which one was not and why.
- T3 green on the whole corpus at every commit in the phase.
- The fuzzer has run for ≥ 24 h with no unexplained T1 divergence.
- `features.md` regenerated with an ein.rs column.
- F11 closed or updated: D1 landed, or D1 measured and parked with the
  numbers.
- [Q-M1a.18](../open_questions.md#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint)
  answered against a rendered before/after trace, and its consequence
  either landed in both engines or written down as declined.

## Cross-links

- **[baseline.md](baseline.md) — the measurements this phase runs on.**
- [baseline.md §9](baseline.md#9-the-fork-entry-re-derivation) — the fork-entry
  re-derivation, and why three of the top five costs are one cost
- [design/05 §7 — beta-memories](../design/05_matcher.md)
- [design/06 §3–§4 — the two exact wins](../design/06_saturation.md)
- [`architecture_and_algorithms.md` §7](../../../docs/kernel/inference/architecture_and_algorithms.md)
  — the lever list this phase works through
