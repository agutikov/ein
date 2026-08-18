# P1a.6 — Performance

**Milestone:** [M1a — Rust port](../README.md)
**Status:** **in progress** — [S1a.6.1](s1a.6.1_profile_baseline.md) and
[S1a.6.8](s1a.6.8_compile_cache_and_extents.md) shipped 2026-08-18. The
measurements are in **[baseline.md](baseline.md)**: §1–§9 are what the phase
is chosen by, §10 is where it stands.
**Estimate:** 4 weeks (22 days of stages — S1a.6.1 added one worth 2 d and
shortened another by 1 d; [S1a.6.9](s1a.6.9_fork_entry_delta.md) added 3 d)
**Depends on:** [P1a.5](../p1a.5_presentation/README.md) — the byte gate
must be closed first, so every change here is measured against a green
harness.
**Absorbs:** [F11 — deductive-layer perf](../../followups/f11_deductive_layer_perf.md)
(D1 beta-memories, D2 WCOJ).

## Goal

Turn the parity build into a fast one, with T3 green at every step.
Method is fixed and non-negotiable: **profile, change one thing, re-diff,
re-measure, record.** A change that cannot be attributed is reverted.

## Targets

Against PyPy on the same machine, at `--jobs 1`. **The PyPy column is
re-measured** ([S1a.6.1](s1a.6.1_profile_baseline.md) T1a.6.1.5) — the
numbers the phase was planned with were up to a year old and two of them
moved:

| workload | PyPy today | target | at S1a.6.1 | **at S1a.6.8** |
|---|---:|---:|---:|---:|
| `solve zebra2.ein -e` end-to-end | 4.94 s | ≤ 0.20 s (≥ 20×) | 198.8 ms ✅ | **138.1 ms ✅ 37.0×** |
| `solve zebra.ein -e` end-to-end | 8.79 s | ≤ 0.40 s | 585.8 ms ❌ | **539.9 ms ❌ 16.2×** |
| parse + load `zebra2.ein` | 0.43 s ¶ | ≤ 0.015 s (≥ 50×) | 1.04 ms ✅ | **1.01 ms ✅ 185×** |
| the acceptance gate (3 fixtures) | 36.0 s ‡ | ≤ 5 s | 1.27 s ✅ | **1.02 s ✅ 35×** |

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

¶ The planned 0.78 s is not reproducible from its own components on either
interpreter; see
[baseline.md §1](baseline.md#where-the-milestones-denominators-moved). The
target is met on any reading — a whole `saturate zebra2` *process* is 5.0 ms.

**Three of the four were already met on day one; the fourth needs the
phase.** `solve zebra.ein -e` is **1.35× short** after S1a.6.8, where it was
1.46×, and its profile is now **72.6 % matcher** against `zebra2 -e`'s 42.2 %
— so the two puzzles still do not agree about what to optimise, and the
missed target is the one that decides.

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
| 3 | [S1a.6.9](s1a.6.9_fork_entry_delta.md) ◐ | The fork-entry delta — **measure + decide** | 3 d | **T1a.6.9.1–3 shipped 2026-08-18**; T1a.6.9.4/5/6 wait on Q-M1a.18. Built behind `--features fork-delta`: fork firings −74 % / −77 %, `zebra -e` **392.6 ms — the target crossed** — the fixpoint, verdict, `k`, models and cores verified unchanged over 1.08 M enterings, and **90 002 facts' primary justification moved** |
| 4 | [S1a.6.2](s1a.6.2_memory_layout.md) | Memory layout | 3 d | 21 % of self time is `malloc` / `cfree` / libc, at ~53 bytes per allocation — plus a system allocator (T1a.6.2.7) and a per-entering region (T1a.6.2.8), since ~0.15 % of what a fork allocates outlives it |
| 5 | [S1a.6.3](s1a.6.3_beta_memories.md) | Beta-memories (F11 D1) — **gate opens** | 4 d | 66.9 % of `zebra -e` is the join, and a fork's delta is 3.6 KB — the fact F11 D1 was parked on |
| 6 | [S1a.6.4](s1a.6.4_hypgen_and_lattice.md) | Hypgen and lattice hot paths | 3 d | 7.3 % / 5.3 % self — real, smaller than written; T1a.6.4.1's argument re-aims at saturation |
| 7 | [S1a.6.5](s1a.6.5_frontend.md) | Frontend and load path — **shortened** | 1 d | its acceptance is already met by 8×; reduced to a confirmation plus the allocation report |
| — | [S1a.6.6](s1a.6.6_differential_fuzzer.md) | The differential fuzzer | 3 d | runs *throughout*, not at a position — it guards every row above |
| 8 | [S1a.6.7](s1a.6.7_relever_matrix.md) | Re-measure the lever matrix | 1 d | last, as planned |

## Rules for this phase

1. **T3 stays green.** A perf change that needs a ledger entry is not a
   perf change, it is a semantics change, and it goes back to the
   relevant phase. [S1a.6.9](s1a.6.9_fork_entry_delta.md) is the one
   stage that *starts* on the wrong side of this rule, which is why its
   shipping half is gated on
   [Q-M1a.18](../open_questions.md#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint)
   and lands in both engines or in neither.
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
