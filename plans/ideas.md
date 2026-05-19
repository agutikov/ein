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

### P1.4 constraints — collapse into "rules with `(not …)` in `:assert`"?

Added 2026-05-20, from a side observation during S1.3.0's Q29
resolution.

**Observation:** constraint-style reasoning may reduce entirely to
the existing rule mechanism — no separate constraint abstraction
needed.

The chain of reasoning:

- A *constraint* in CSP terms = "this proposition must be true".
- A *rule* with `:assert (not X)` produces a negative fact when its
  `:match` clause fires.
- M1's KB is **append-only** (P1.2): facts are never deleted; new
  facts arrive on top.
- If both `X` and `(not X)` appear in the same KB state, the engine
  has a **contradiction signal**. The branch fails; P1.5's
  hypothesis loop retracts it.

So a "constraint" in P1.4's sense = "a rule whose conclusion is a
negative fact". Constraint violation = `X ∧ (not X)` co-existing.

**Worked example — zebra.ein already ships this pattern:**

```lisp
(rule type-exclusivity ()
  :match  (and (instance ?a ?T)
               (instance ?b ?T)
               (neq ?a ?b))
  :assert (not (co-located ?a ?b))
  :why    "{?a} and {?b} share type {?T}, can't co-locate.")
```

Reads as "two distinct instances of the same type cannot
co-locate" — a constraint, expressed as a rule. The engine asserts
`(not (co-located ?a ?b))`; on branches where `(co-located ?a ?b)`
later gets derived, the contradiction surfaces.

**Implications for P1.4:**

- P1.4 may not need new core machinery. Its content shrinks to:
  - A **contradiction detector**: scan for `(X, (not X))` pairs in
    the same layer/branch (one pass; cheap).
  - The **negative-fact representation**: how `(not X)` is stored
    as a `Fact` (likely `(rel="not", args=(positive-fact-id,))`).
  - **Interaction with P1.5**: the hypothesis loop reads
    contradictions from the detector to decide branch retraction.
- Q5 ("explanation completeness on Zebra") is satisfied by the 6
  zebra.ein rules — `type-exclusivity` is the only
  constraint-style rule, and it's already covered.
- **P1.4 might merge into P1.3** — the detector is a small
  addition to the saturator. Worth revisiting once P1.3 ships.

**What this does NOT cover:**

- Arithmetic / linear / interval constraints (not in M1 scope —
  Q33 deferred numeric reasoning to followups).
- Global constraints (`allDifferent`, cardinality) — also followup
  territory.
- Constraints over compound node kinds (Q26-deferred).

For graph-only Zebra-class constraints, the unification holds.

**Cross-links:**
- [S1.3.0 §F (scope reconsideration)](m1_core_graph_reasoning/p1.3_inference_rules/s1.3.0_review_and_revisions.md#f-scope-reconsideration)
- [Q40 — hypothesis-rule premises](m1_core_graph_reasoning/open_questions.md) (P1.5)
- [Idea 03 — three task classes](../docs/ideas/03-three-task-classes.md) — "contradictions" task class consumes exactly this signal
- [Q33 resolution](m1_core_graph_reasoning/p1.3_inference_rules/s1.3.0_review_and_revisions.md#g-consolidated-open-questions-for-p13) — `not` is a structural wrapper handled by matcher/asserter

**Action:** revisit P1.4 README + S1.4.x stages with this lens.
Don't immediately collapse the phase — confirm there's no
non-graph constraint type in M1 scope (there shouldn't be after
Q33). If the unification holds end-to-end, propose either
"merge P1.4 into P1.3" or "shrink P1.4 to detector + negative-fact
representation only".

---

## Promoted / pruned

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
