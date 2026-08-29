# S1e.1.3 — What the core promises: Q2

**Phase:** [P1e.1](README.md) (The ten questions)
**Estimate:** 1.5 days
**Depends on:** [S1e.1.1](s1e.1.1_search_soundness_probes/README.md) T1.
**Answers:** [`review/open-questions.md`](../review/open-questions.md) Q2.
**Touches:** [CO-H3](../p1e.2_high/s1e.2.1_correctness.md)(b) — the tree's
empty core is a different defect, but both are about what the word *core*
promises.

---

> **Done 2026-08-29 — acceptance (1), the fixture. The answer is yes.**
>
> Retention is by **premise count**, which is local; the frontier search
> minimises **frontier size**, which is transitive. They disagree exactly where
> the stage said they might, and the shape builds:
> [`examples/ein-bugs/alt-cap-core.ein`](../../../examples/ein-bugs/alt-cap-core.ein)
> reports a **3-fact** core where a **2-fact** one exists, and its
> [`-reordered` twin](../../../examples/ein-bugs/alt-cap-core-reordered.ein) —
> one `:priority` apart, same facts, same rules, same verdict — reports the
> 2-fact one. Verified against the cause: at `MAX_ALT_JUSTIFICATIONS = 1_000_000`
> the first file reports 2 as well.
>
> **The sharper half is the order, not the size.** `explain.rs` exists because
> walking one justification per fact made the core depend on `:priority`; the
> cap hands that back one level up. So the wording that moved is not only
> *recorded* → *retained* but `glossary.md`'s **"Independent of the order in
> which the rules fired"**, which was true of the search and not of the
> pipeline.
>
> **And it is not urgent, which T1 established before anything was built.** All
> 202 corpus entries: **one** reaches the cap —
> `examples/ein-bugs/zebra2-bad.ein`, the entry the README names as the
> unsat-core fixture — where 1 017 of 2 425 arrivals are refused and the
> longest list would be **1 049** without the cap (next-longest entry anywhere:
> **8**), and **zero** evictions corpus-wide. The cap can only change an answer
> on an entry that reaches it: that entry's core is one fact at 32 and at 10⁶
> alike, and every corpus file's *root* explanation is byte-identical between
> the two caps. So the cap is live on exactly one program and costs it
> nothing.
>
> The fix is engine work with three candidates and no obvious winner —
> [Q-M1e.15](../open_questions.md#q-m1e15--the-alternatives-cap-decides-which-unsat-core-is-reported),
> owner unassigned, fixture attached. Not taken here, per the acceptance.

## Context

The unsat core is a user-facing explanation artefact and, from
[M2](../../m2_nl_to_ir/README.md) on, a feedback signal a model reads. The
README states what it is with care: *the smallest set of given facts …* and
explicitly **not** a subset-minimal MUS. The operative qualifier is that the
smallest frontier is searched **across every recorded derivation**.

`recorded` is where the question lives. `alts` is capped at
`MAX_ALT_JUSTIFICATIONS = 32`
([`kb.rs:42-49, 1456-1535`](../../../ein.rs/crates/ein-core/src/kb.rs)),
sorted shortest-premises-first, and the shortest evicts the longest. The core
search then walks what survived
([`explain.rs`](../../../ein.rs/crates/ein-infer/src/explain.rs),
`smallest_contradiction_frontier`). If eviction can remove the derivation
whose *frontier* is smallest, the reported core is larger than the
recorded-derivation minimum the docs promise — and the promise, not the size,
is the defect.

The plausible reason it never bites is that retention is by **premise
count**, and a derivation with fewer premises tends to have a smaller
frontier. *Tends to* is not an argument: premise count is local and frontier
size is transitive, so a two-premise step whose premises each unfold into
long chains can have a larger frontier than a five-premise step over givens.
Whether that shape survives 32 alternatives is the question.

## Acceptance

- One of two, and the stage says which:
  1. **A fixture** where more than 32 alternative justifications exist and
     the shortest-frontier derivation is evicted — with the reported core and
     the true recorded minimum both printed, and the README's sentence
     corrected to what the engine actually promises.
  2. **The argument** that shortest-premises-first retention is harmless to
     the frontier search, written beside `MAX_ALT_JUSTIFICATIONS` where the
     cap is declared, with the counterexample shape it rules out named.
- Either way `MAX_ALT_JUSTIFICATIONS`'s doc comment states what the cap costs
  — today it states only what it bounds.
- If the answer is (1) and the fix is engine work (retain by frontier
  estimate, or raise the cap, or record the minimum separately), the fix is
  **not** taken here: it is filed with the fixture attached. This stage
  establishes the promise, not the implementation.

## Tasks

### Task T1e.1.3.1 — Establish whether 32 is ever reached ✅

**Done 2026-08-29 — outcome 3 of the three, "reached", with a twist.** A
temporary counter in `Kb::record_justification` and `Kb::accepts_justification`
(reverted; the numbers are banked in `MAX_ALT_JUSTIFICATIONS`'s doc comment),
over all 202 corpus entries under a bounded exhaustive solve at each file's own
config:

| | |
|---|---:|
| entries recording any alternative | 50 of 202 |
| alternatives recorded | 12 337 500 |
| entries that **reach** the cap | **1** — `examples/ein-bugs/zebra2-bad.ein` |
| arrivals refused there | 1 017 of 2 425 |
| longest list at `MAX_ALT_JUSTIFICATIONS = 1_000_000` | **1 049** |
| next-longest list anywhere | **8** (the `square-fwd` / `square-bwd` family) |
| **evictions** — an arrival displacing a longer one — corpus-wide | **0** |

Two things in that table were not among the three outcomes the task listed.

**The cap is reached on exactly the entry the README names as the unsat-core
fixture**, which is not a coincidence: `zebra2-bad` is the corpus's one
deliberately over-constrained puzzle, so it is where a fact gets re-derived a
thousand ways.

**And the retention rule has never fired.** Zero evictions means every
over-cap arrival was *refused* (`n >= longest kept`), never inserted in place of
a longer one — so "sorted by premise count, shortest retained" has, on this
corpus, only ever meant "the first 32 wins". The sort is doing nothing, and the
refusal is doing everything.

**The control.** At `MAX_ALT_JUSTIFICATIONS = 1_000_000`, all 202 entries'
`explain` shapes — alternatives on *and* off, each over an unbudgeted root
saturation — are byte-identical to the capped run, as are their `solve` shapes;
and `zebra2-bad`'s CLI core is the same single `(color-loc Green House-1)`.
That is the whole exposure, because **the cap can only change an answer on an
entry that reaches it**. So the cap is reached, is not evicting, and costs the
corpus nothing — which is what made T1e.1.3.2 a construction rather than a
search.

The task as written:

Before constructing anything, measure. Add a temporary counter — or read it
off `--events` if a suitable event exists — for the maximum `alts` length
reached per fact across the corpus, and run the whole manifest. Three
outcomes:

- **Never near 32.** The cap is unreachable on real programs and the finding
  is `accepted` with a measured number in the comment (*the corpus maximum is
  N*), which is a far better comment than the cap alone.
- **Reached on some entries.** Name them; they are where the fixture comes
  from, and a natural one is the family with the most redundant derivations
  — the saturation type-exclusivity set or `zebra2-bad`.
- **Reached and evicting.** Then eviction is live on shipped programs and
  T1e.1.3.2 is not hypothetical.

This measurement is worth having whichever way it lands, and it is the
cheapest half-day in the phase.

### Task T1e.1.3.2 — Construct the adversarial shape, or rule it out ✅

**Constructed 2026-08-29**, and smaller than the task's sketch: 33 one-premise
derivations three givens deep, plus **one two-premise** derivation over givens
— not "a wide step straight from givens" with more than 32 premises, because
the refusal only needs the arrival to be *no shorter* than the longest kept,
and the longest kept is 1.

```
(false) ← (q Yi)              × 33   1 premise,  frontier 3   (q Yi) ← 3 givens
(false) ← (w1 X) (w2 X)       × 1    2 premises, frontier 2
```

Fire the 33 first and the list is a primary plus 32 one-premise alternatives;
`wide` then arrives with `2 >= 1` and is refused outright. The store never
holds the 2-fact explanation, so the search cannot choose it and reports 3.

The **twin** is the same file with `wide` at `:priority 50` instead of `300`.
`wide` fires first, its derivation becomes the *primary* — which is never
evicted — and the reported core is 2. Same facts, same rules, same verdict,
same `k`; one integer apart.

Both are corpus entries with `:expect (false)`, and the cores are asserted in
`ein-infer/tests/explain_semantics.rs::the_alternatives_cap_can_enlarge_the_reported_core`
— `:expect` states a verdict and cannot state a core, the same reason M1d's
obligation fixtures assert their owe counts in Rust.

The retention rule does **not** dominate, so option (2) — the argument — was
not available. Why it does not: each shorter-premise alternative's frontier is
*not* a subset of the longer one's, and nothing makes it so. Premise count is
one edge of the AND/OR graph; frontier size is the whole cone under it.

The task as written:

The shape wanted: one contradiction reachable by ≥ 33 distinct derivations,
where the derivation with the **smallest frontier over givens** has *more*
premises than 32 others. Build it synthetically — a fan of two-premise
derived steps whose premises are themselves derived, plus one wide step
straight from givens — rather than trying to find it in a puzzle, which is
how the review's own recommendation phrases it.

If the shape cannot be built because the retention rule genuinely dominates
(each shorter-premise alternative's frontier is a subset), that is the
argument, and writing down *why* it dominates is the deliverable.

### Task T1e.1.3.3 — Match the claim to the answer ✅

**Done 2026-08-29 — and it was six places, not three.** The task named the
README's sentence, the glossary entry and `explain.rs`'s doc comment; a grep
for the promise found three more, and one of them was flatly false rather than
merely imprecise:

| where | what moved |
|---|---|
| [`README.md`](../../../README.md) | *searched across every recorded derivation* → *every derivation the store **retained** — at most 32 per fact, shortest-premises-first*; and the capability table's Known-gaps cell now names Q-M1e.15 |
| [`docs/kernel/glossary.md`](../../../docs/kernel/glossary.md) | § Smallest contradiction frontier — *retained*, the cap's **32 and its retention rule** spelled out, and **"Independent of the order in which the rules fired"** qualified: the *search* is, the store in front of it is not |
| [`explain.rs`](../../../ein.rs/crates/ein-infer/src/explain.rs) | a module section on what the cap costs, and `Explanation::exhausted`'s doc — it reports the `ExplanationBudget` and **not** the alternatives cap, so `exhausted = true` is not a claim that no smaller explanation exists |
| [`inference/reserved_engine_strings.md`](../../../docs/kernel/inference/reserved_engine_strings.md) | *the answer does not depend on which derivation fired first* → *the **search** does not* |
| [`inference/architecture_and_algorithms.md`](../../../docs/kernel/inference/architecture_and_algorithms.md) §O6 | *independent of rule-firing order* → *over the derivations it is given*; the Gap's caveat (2) renamed **Retained, not all** and given the measurement |
| [`ir/02-data-model/01_entities.md`](../../../docs/kernel/ir/02-data-model/01_entities.md) §3.1 | **the false one** — *the cap retains the shortest derivations — the ones a minimum-cardinality explanation can use*. It retains the fewest **premises**, which is a different quantity, and that difference is the whole finding |

`docs/api/*` is **not** in the list on purpose: it is history under a 🏛 banner
and states ein.py's contract, which is what it said.

The task as written:

Whichever holds, three places state this promise and they move together: the
README's core sentence, the
[glossary](../../../docs/kernel/glossary.md)'s entry, and
`explain.rs`'s doc comment. The wording to aim for names the qualifier
explicitly — *smallest over the derivations the store retained, and the store
retains at most 32 per fact, shortest-premises-first* — because a promise
with its own limit stated is a promise a consumer can code against, and M2 is
the consumer.

## Notes

`Contradiction` with an **empty** core is a different problem with two
sources: the `-m` cap ([Q-M1d.6](../../../docs/history/m1d_satisfiability/open_questions.md#q-m1d6--may-contradiction-be-said-with-exhausted--false),
not this milestone's) and the tree's dead arm
([CO-H3](../p1e.2_high/s1e.2.1_correctness.md)(b), which is). This stage is
about a core that is *non-empty and larger than promised*; it should not
absorb either of those, and its wording fix should not accidentally state
something about them.

## What landed

| | |
|---|---|
| the fixture | [`examples/ein-bugs/alt-cap-core.ein`](../../../examples/ein-bugs/alt-cap-core.ein) + its `-reordered` twin, two corpus entries, `:expect (false)` on both |
| the test | `ein-infer/tests/explain_semantics.rs::the_alternatives_cap_can_enlarge_the_reported_core` — the two cores, the two primaries, and the fact sets asserted equal so the pair cannot silently become two puzzles |
| the measurement | `MAX_ALT_JUSTIFICATIONS`'s doc comment § What it costs — seven numbers over 202 entries |
| the wording | six sites, above |
| filed | [Q-M1e.15](../open_questions.md#q-m1e15--the-alternatives-cap-decides-which-unsat-core-is-reported) — three candidate fixes priced, owner unassigned |
| not changed | the engine, and no corpus answer |

**The two goldens this stage moves, named before they moved**, and both moves
are **additive** — new rows for two new files, no existing row edited, which is
what adding a fixture always costs and what
[S1e.1.1](s1e.1.1_search_soundness_probes/README.md) paid three times:

| golden | what it gains |
|---|---|
| `ein-cli/tests/golden/corpus_exits.txt` | 6 rows — two files × `solve`, `solve -e`, `test`, all exit **0** |
| `ein-render/tests/golden/corpus_shapes.md5` | 90 rows — 45 observable surfaces per file |

**96 insertions, 0 deletions**, which is the check that *additive* is what it was.
