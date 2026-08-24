# P2.3 — The benchmark (Stage C)

**Estimate:** 3 weeks — 4 stages, 15 days.
**Depends on:** the kernel only, for the generators: a synthetic instance is
an Ein program *first* and a text second, so the canonical theory is checked
by `ein solve -e` before a sentence of English exists. The seed set's hand
encodings depend on nothing. [P2.2](../p2.2_formalizer/README.md) is needed
only to *run* the benchmark, not to build it, and the two phases can overlap.
**Blocks:** the **Level B gate** (with P2.2); [P2.5](../p2.5_harness/README.md)
— every table is over this benchmark's splits;
[P2.7](../p2.7_failure_scaling_generalization/README.md) — the difficulty
knobs and the paraphrase sets are this phase's generators with different
arguments.
**Research plan:** [`EinAf.md` § Stage C](../EinAf.md#stage-c--construct-a-heterogeneous-benchmark),
C1–C5.

---

## Why the old five puzzles are not a benchmark

The old P2.6 chose five puzzles — Zebra, an Einstein variant, two logic
grids, one hand-written ambiguity stress — with gold IR for each, and it was
right that *five well-documented puzzles beat fifty noisy ones* for a
**seed set**. It is not a benchmark for the research question, for three
reasons the plan's Stage C makes precise:

1. **It tests whether Ein can solve Zebra-like puzzles**, not whether the
   formalizer can construct a theory. Every instance is attribute matching;
   one family cannot tell reinterpretation from theory selection from theory
   synthesis, because in one family the theory is always the same one.
2. **Every instance has a unique answer.** A loop that drives every theory
   to `k = 1` scores perfectly on it and is hallucinating on half of the real
   inputs — the E3 failure mode. A benchmark that cannot produce the verdicts
   *ambiguous* and *unsatisfiable* as **correct answers** cannot measure
   whether the system distinguishes *I don't know*, *several answers*,
   *inconsistent assumptions* and *the formalization failed*.
3. **Five points are not a curve.** Scaling ([P2.7](../p2.7_failure_scaling_generalization/README.md))
   needs the same family at several sizes — BBH ships `logical_deduction` at
   three, five and seven objects for exactly this reason — and needs the
   difficulty knobs of C4 to be *set*, not found.

So the five become the seed set of [S2.3.1](s2.3.1_families_and_seed_set.md),
kept with their gold programs as the hand-written anchor of each family they
belong to, and the benchmark is built around them.

## The shape

**Eight families** (C1), each with a hand-written anchor, a generator, and
where one exists an external source:

| family | the plan's example | what it stresses that Zebra does not | stdlib theory | external source |
|---|---|---|---|---|
| logical deduction | *A is older than B, B than C; who is youngest?* | transitivity alone; a total order from partial statements | `std.algebra` `transitive` | BBH `logical_deduction_{three,five,seven}_objects` |
| ordering | *Alice arrived before Bob, Carol after Bob* | the same, with `immediately-before` ⊂ `before` — two relations, one theory | `std.algebra` `includes`, `transitive` | BBH `logical_deduction` (positional variant) |
| spatial | *A left of B, C above A* | two orders at once; `converse` (`right-of` = `left-of`ᵀ) | `std.algebra` `converse`; [Q17](../../open_questions.md#q17--spatial-relation-formalisation) — no arithmetic lattice, adjacency is extensional | — |
| object tracking | *the key starts in A; A and B are swapped; then B and C* | **state**: a sequence of transitions, not a static model — the first family Ein's saturator does not obviously fit ([F13](../../followups/f13_puzzles_beyond_zebra/ideas.md) names it as the test of genericity) | none yet — the family whose theory the formalizer must *synthesise* or the stdlib must grow | BBH `tracking_shuffled_objects_{three,five,seven}_objects` |
| attribute matching | Zebra | bijections between several attribute sets and one spine | `std.bijection` / `std.slots` | the seed set; logic-grid collections |
| set / category | *every A is B; no B is C; x is A; can x be C?* | subsumption and disjointness; a `can` question, which is a satisfiability question, not a deduction | `std.typing`, `is-a` | ProofWriter, FOLIO (a subset) |
| graph | reachable, adjacent, connected, ancestor, dependency | transitive closure over an explicit edge set; cycles | `std.algebra` `transitive`, `symmetric` | CLUTRR (kinship chains) |
| temporal | before, after, overlap, interval containment | Allen-style interval relations; a composition table, not a single property | none — a *theory* the stdlib does not have; [F8](../../followups/f8_FCA_RCA_odis_tptp/ideas.md) names Allen as the second rung of its `C(n)` curve | BBH `temporal_sequences` |

The fourth column is the point of the table: the benchmark is heterogeneous
**in which theory applies**, and two families have none in the stdlib. That
is not a gap to fill before the benchmark runs — it is the condition under
which the formalizer's *theory synthesis* action is exercised at all, and a
family where it fails is a result about synthesis, reported as such.

**Both sources** (C2). Synthetic instances give arbitrary size, exact ground
truth, the canonical theory, and the knobs; external instances give ecological
validity and published numbers to stand beside. Neither alone.

**Three verdict classes per family** (C3). The generator emits, for every
family, instances whose gold verdict is `unique`, `ambiguous` (a clue
deleted — the old acceptance criterion 3, made an instance rather than a
manual test) and `unsat` (a contradicting clue added). The gold verdict is
checked by `ein solve -e` on the canonical program before the instance is
admitted, and it is written into the program as an `:expect` in
[M1c](../../../docs/history/m1c_external_validation/README.md)'s form, so that `ein test` —
M1c's, not shipped yet — re-checks the benchmark's own ground truth on every
gate run from the day it exists.

**Knobs** (C4): entities, relations, chain depth, branching factor, constraint
density, irrelevant statements, paraphrase, implicit vs explicit relations,
model count, the distance between the statements a deduction needs. Each is a
generator argument recorded in the instance's metadata, so every table in
[P2.7](../p2.7_failure_scaling_generalization/README.md) is a slice of this
benchmark, not a new one.

**Frozen splits** (C5): `dev` / `val` / `test`, by instance id, frozen with a
version and a digest; the test split never drives a prompt.

## Stages

| ID | title | est. | ends with |
|---|---|---:|---|
| [S2.3.1](s2.3.1_families_and_seed_set.md) | The eight families and the seed set | 3 d | a hand-written anchor per family — text, program, `:expect` — the old five puzzles among them; the per-family note of which theory applies and whether the stdlib has it; the instance format |
| [S2.3.2](s2.3.2_generators.md) | The generators — exact ground truth, three verdicts, the knobs | 6 d | one generator per family, each an Ein program generator plus an NL realiser; unique / ambiguous / unsat by construction; every instance checked by `ein solve -e` before admission; the knobs as arguments in the metadata |
| [S2.3.3](s2.3.3_external_benchmarks.md) | External benchmarks — adapters | 3 d | BBH first (`logical_deduction` 3/5/7, `tracking_shuffled_objects`, `temporal_sequences`), then CLUTRR, ProofWriter, FOLIO; the answer-extraction contract per source (multiple choice, True/False/Unknown, an entity); licences and versions recorded |
| [S2.3.4](s2.3.4_splits.md) | Frozen splits, the benchmark's own record | 3 d | `dev` / `val` / `test` by id, versioned and digested; the rule that the test split is untouched until [P2.10](../p2.10_result_artifact_demo/README.md); the benchmark's manifest, in the corpus's form |

## Acceptance

- Eight families, each with an anchor, a generator and — where the table
  names one — an adapter; every synthetic instance carries its canonical
  program, its gold verdict and answer, and the knob values that produced it.
- For every family, instances of all three verdict classes exist and `ein
  solve -e` on the canonical program agrees with the gold verdict — the
  benchmark's ground truth is **certified by the kernel, exhaustively**,
  before a model sees it.
- The `:expect` forms on the canonical programs are written in
  [M1c](../../../docs/history/m1c_external_validation/README.md)'s form now, so the ground
  truth is re-checked by the gate the day `ein test` ships; until then
  `einaf bench check` compares `ein solve -e`'s summary to `meta.json`
  ([S2.3.1](s2.3.1_families_and_seed_set.md)).
- The splits are frozen with a version; the digest of the test split is in
  the benchmark manifest and in every experiment record that used it.
- The benchmark is a directory — `benchmark/` at the root; `bench/` is
  [M10](../../m10_external_benchmarks/README.md)'s — another researcher can
  read without the system: the text, the answer, the program, the theory, the verdict — Stage
  Q's second layer, delivered early.

## Risks

- **The realiser writes a dialect.** A generator's English is one template
  family's English, and a formalizer that learns it is not learning English.
  K1's paraphrase sets ([P2.7](../p2.7_failure_scaling_generalization/README.md))
  are the check; S2.3.2 writes at least three surface templates per
  statement type from the start so the check has something to vary.
- **Object tracking and temporal have no stdlib theory.** Said above: a
  feature, not a blocker. The risk is the opposite one — that the milestone
  quietly adds `std.state` and `std.allen` to make the numbers better. If the
  stdlib grows during Level C it is a G9 condition (*library size*) and a
  recorded kernel change ([Q-M2.1](../open_questions.md#q-m21--when-is-the-kernel-frozen)),
  never a silent improvement.
- **External benchmarks have answers, not theories.** Constraint precision /
  recall is undefined on them; faithfulness falls back to
  [Q-M2.3](../open_questions.md#q-m23--what-is-the-unit-of-faithfulness-without-a-gold-theory)'s
  candidates. The adapters record which metrics an instance supports.
- **Contamination.** BBH is in every training set. The synthetic families are
  the uncontaminated half, and K4's novel vocabulary (`zorps` for `left-of`)
  is the test of whether a published number is memory.

## Connections

- [`EinAf.md` § Stage C](../EinAf.md#stage-c--construct-a-heterogeneous-benchmark).
- [F13](../../followups/f13_puzzles_beyond_zebra/ideas.md) — the ladder
  (BBH `logical_deduction` → `tracking_shuffled_objects` → CLUTRR, FOLIO,
  ProofWriter → logic grids and Knights & Knaves → ARC), the BBH record
  format, the observation that `logical_deduction_3/5/7` is a scaling
  experiment already; [idea 09](../../ideas/09-puzzles-beyond-zebra.md) — the
  human-puzzle menu; [`docs/lib/12`](../../../docs/lib/12-llm-and-reasoning-benchmarks.md) —
  the catalogue the external sources are drawn from, § 7 *Zebra-style custom
  sets — the class of relevance to Ein*.
- [M1c](../../../docs/history/m1c_external_validation/README.md) — the `:expect` form;
  [`corpus/`](../../../corpus/README.md) — the manifest convention the
  benchmark's own manifest follows (`ein-corpus/2`, one entry per file with
  its runs); `examples/gen_zebra2_variants.py` — the one generator the repo
  already has, which S2.3.2 starts from.
- [M10](../../m10_external_benchmarks/README.md) — the *other* benchmark:
  formal-language shaped, hand-encoded, not M2-gated; its problems are
  candidates for this benchmark's anchors and nothing else is shared.
- [Q17](../../open_questions.md#q17--spatial-relation-formalisation) — why
  spatial adjacency is extensional.
