# S1e.1.3 — What the core promises: Q2

**Phase:** [P1e.1](README.md) (The ten questions)
**Estimate:** 1.5 days
**Depends on:** [S1e.1.1](s1e.1.1_search_soundness_probes.md) T1.
**Answers:** [`review/open-questions.md`](../review/open-questions.md) Q2.
**Touches:** [CO-H3](../p1e.2_high/s1e.2.1_correctness.md)(b) — the tree's
empty core is a different defect, but both are about what the word *core*
promises.

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

### Task T1e.1.3.1 — Establish whether 32 is ever reached

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

### Task T1e.1.3.2 — Construct the adversarial shape, or rule it out

The shape wanted: one contradiction reachable by ≥ 33 distinct derivations,
where the derivation with the **smallest frontier over givens** has *more*
premises than 32 others. Build it synthetically — a fan of two-premise
derived steps whose premises are themselves derived, plus one wide step
straight from givens — rather than trying to find it in a puzzle, which is
how the review's own recommendation phrases it.

If the shape cannot be built because the retention rule genuinely dominates
(each shorter-premise alternative's frontier is a subset), that is the
argument, and writing down *why* it dominates is the deliverable.

### Task T1e.1.3.3 — Match the claim to the answer

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
