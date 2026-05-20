# Ideas — rolling scratchpad

A working surface for half-formed thoughts that are *not yet* ready
to land in a stage file. Cheaper than spawning a new
`docs/ideas/<n>-…md`, faster than figuring out which milestone owns
it. The intent is: ideas live here briefly, then either get
promoted into a stage (`plans/`), a research note (`docs/ideas/`),
or pruned.

When promoting, leave a one-line stub here with a forward-pointer
and a date; that's the breadcrumb trail for "why did we think X
mattered?".

---

## Live entries

*(none currently — all parked entries below were promoted or pruned.)*

---

## Promoted / pruned

### P1.4 constraints — collapse — closed 2026-05-21, verdict: shrink, don't merge

Added 2026-05-20 during S1.3.0's Q29 resolution. The observation
that *"constraint = rule with `:assert (not X)`"* led to a question
about whether P1.4 should merge into P1.3 or just shrink.

**Verdict** (2026-05-21, after P1.3 shipped): *shrink, don't merge.*
P1.4 keeps a clean phase boundary but drops ~85% of its original
scope. What's left is a focused contradiction detector
(1-2 days of work) that P1.5 will call at branch boundaries.

- Structural constraints (single-type / all-different) → already
  handled by `type-exclusivity` (T2 on `co-located`) shipped in P1.3.
- Spatial constraints (1-D position lattice + arithmetic
  propagation) → rejected by [Q17](m1_core_graph_reasoning/open_questions.md#q17--spatial-relation-formalisation),
  resolved declaratively by `right-of` / `next-to` relations +
  `square-fwd` / `square-bwd` / `square-unique` rules. The
  non-adjacent "Ivory left of Green" disjunctive case migrates to
  P1.5 hypothesis branching.
- `Constraint` entity / `verify_incremental` API → dropped; the
  saturator's existing `_index_fact` provides incremental maintenance.

**Survives:** a small `ContradictionDetector` scanning the KB for
`(X, (not X))` pairs in the same layer. Output feeds P1.5 directly.

**Recorded in:**
- [`p1.4_constraints/s1.4.0_review.md`](m1_core_graph_reasoning/p1.4_constraints/s1.4.0_review.md)
  — the full audit + revisions list.
- [`p1.4_constraints/README.md`](m1_core_graph_reasoning/p1.4_constraints/README.md)
  — rewritten phase overview (post-shrink).
- [`p1.4_constraints/s1.4.1_contradiction_detector.md`](m1_core_graph_reasoning/p1.4_constraints/s1.4.1_contradiction_detector.md)
  — the surviving stage.
- `p1.4_constraints/s1.4.2_spatial.md` — deleted.

**Cross-links retained:**
- [S1.3.0 §F (scope reconsideration)](m1_core_graph_reasoning/p1.3_inference_rules/s1.3.0_review_and_revisions.md#f-scope-reconsideration)
- [Q40 — hypothesis-rule premises](m1_core_graph_reasoning/open_questions.md) (P1.5)
- [Idea 03 — three task classes](../docs/ideas/03-three-task-classes.md) — "contradictions" task class consumes the detector's output.
- [Q33 resolution](m1_core_graph_reasoning/p1.3_inference_rules/s1.3.0_review_and_revisions.md#g-consolidated-open-questions-for-p13) — `not` is a structural wrapper handled by matcher/asserter; the detector reads its output, not the wrapper machinery.

### P1.2b audit — closed 2026-05-19, verdict: no phase needed

Audited the ein-model unification
([`03_ein_model.md`](../docs/kernel/ir/01-ein-graph/03_ein_model.md)
+ [`04_jack_drinks_coffee.md`](../docs/kernel/ir/01-ein-graph/04_jack_drinks_coffee.md))
against the existing P1.2 stages.

- [x] **S1.2.1-S1.2.4 acceptance under reflexive framing** — *all 4
      stages pass.* Numeric counts (types=7, instances=30, declared
      rels=3, total rels=9 incl. open-world `instance`, rules=6,
      facts=54) and cross-refs hold; 144 kb tests green. The
      kernel meta-primitive `instance` was already called out as an
      auto-vivified relation in S1.2.1's acceptance — the reflexive
      framing was anticipated.
- [x] **New entity / index shapes** — *none needed for M1.* The
      reflexive claims are all expressible at the graph level today
      (`Fact` with head=`instance` and `Relation` auto-vivification
      cover it). The partial entity-level homoiconicity — e.g.
      `(instance instance instance)` doesn't get a special structural
      marker — is fine for M1; F5 (rules-as-data) is where it would
      matter.
- [x] **New grammar primitives** — *none needed.* Q27 (body-form
      sugar) and Q28 (`()` semantics) are parked as future seams; the
      current grammar handles both encodings (zebra.ein + zebra2.ein).
- [x] **Docs gaps** — *three docs-hygiene fixes applied, no
      architectural gaps:* (1) added forward-pointer
      `01_kb.md` → `03_ein_model.md` in the See-also; (2) updated
      S1.2.4 acceptance to point at the moved
      `04_dot_rendering.md`; (3) closed the "flagged for the user's
      review" prompt in `03_ein_model.md` §8 with a link back here.

**Verdict:** *no P1.2b needed.* The kernel-docs reorg + existing
P1.2 implementation jointly cover the unified model. Q27/Q28 remain
parked; revisit if P1.7 acceptance reveals a gap.
