# P1.7 — Bootstrapping: Zebra puzzle end-to-end

**Estimate:** 1-2 weeks.
**Depends on:** P1.1, P1.2, P1.3, P1.4, P1.5, P1.6.
**Blocks:** M1 done.

## Goal

Make the Zebra puzzle the engine's *first real test* — and the
M1 acceptance gate. Encode it in IR; solve it; produce a trace
that matches the human walkthrough from
[`docs/ideas/08`](../../../docs/ideas/08-human-style-deductive-trace.md);
audit which knowledge is hardcoded in code vs declared in IR;
document the decisions for future puzzles.

This is also where we discover all the gaps that the earlier
phases optimistically deferred. Reserve a third of the phase for
that.

## Updates

IMPORTANT GOALS:
1) merge zebra and zebra2 synatx, remove redunduncy, leave only canonical
2) implement facts closer to NL
   ```lisp
      (relation drink Human Drink)      ;; human drink drinks
      (relation co-located Human Drink) ;; human and drinks can be co-locted
      (imply drink co-located)          ;; when human drinks a drink means they are co-located
      (is-instance Norvegian Human)
      (is-instance Milk Drink)
      (drink Norvegian Milk)            ;; Norvegian drinks milk (some problem fact)
   ```
2.1) What is an appropriate detalization level? I want 1-to-1 with NL facts, what is minimal ontology for this?
2.2) Does detalization affects compexity?
2.3) How to reduce complexity without reducing expressiveness?
   e.g. solve only `co-located` and then directly infer `drink`, `live` etc.

### Kernel de-hardcoding — full purity pass (decided 2026-05-30)

**Supersedes the earlier "drop classic zebra.ein" / proto-library
direction.** Decision: the kernel keeps only the declarators
`relation` / `rule` / `hrule` and the logical primitives `not`
`false` `and` `or` `neq` `forall` `absent`. **Everything else
leaves the kernel** — `type`, `instance`, `a-priori`, `symmetric`,
`is-a`, `closed`/`open` — re-expressed as ein rules (or, for the
dead `a-priori` alias, deleted), with `zebra2.ein` re-validated
end-to-end.

- The audit (full list of kernel-special names) + the decision
  live in [S1.7.2](s1.7.2_dynamic_vs_hardcoded.md).
- The **tractable** execution — drop `a-priori`; `type`/`instance`
  → generic facts + rules in `zebra.ein`; `symmetric` dedup; `or`
  lowering — is [S1.7.6](s1.7.6_kernel_minimization.md).
- The **load-bearing** removals (`is-a`, `closed`/`open`) are
  **parked as an end-of-phase analysis stage**,
  [S1.7.7](s1.7.7_kernel_purity_analysis.md) — they may surface an
  irreducible kernel.
- `type`/`instance` stay **writable** as generic facts on
  user-declared relations (`(type UserType T)` is the basic form);
  their semantics move into rules in `zebra.ein`. **`zebra.ein` is
  a demonstrator, not required to solve or trace-match.**
  `zebra2.ein` stays the canonical solving target.
- **Not an M1 gate:** zebra2 already solves *with* the primitives,
  so the purity pass is a kernel-quality goal; M1-done does not
  wait on it.

## Stages

S1.7.1–S1.7.3 are done / relocated. The **left stages** are listed in
**execution order S1.7.6 → S1.7.4 → S1.7.5 → S1.7.7** (not numeric ID
order — reordered 2026-05-31 to front-load the kernel-minimization
execution). The only hard edge among them, **S1.7.7 after S1.7.6**, is
preserved; S1.7.4/.5 are independent of S1.7.6, and S1.7.4 sits ahead of
S1.7.7 so its derived-NAF map is in place when closure is de-hardcoded.
**S1.7.8–.22 are children of S1.7.7** (one per hardcoded engine name);
they share its parked status and feed its written conclusion (T1.7.7.5).

| ID      | Title                                  | Duration |
|---------|----------------------------------------|----------|
| S1.7.1  | Canonical encoding + GAPS/CONTRADICTIONS fixtures (**done** 2026-05-30) | ~1 day |
| S1.7.2  | Kernel audit + minimization decision (full purity) | decision done 2026-05-30 |
| S1.7.3 / S1.7.3a | Trace-matches-human acceptance + the open-hypothesis soundness fix → **relocated 2026-05-31 to [P1.7a](../p1.7a_solution_search_refactor/README.md)** (it grew into a structural search/result/stop refactor — the corrected M1 gate) | see P1.7a |
| S1.7.6  | Kernel minimization — execution (a-priori/type/instance/or **done** 2026-06-01; symmetric tested → moved to S1.7.7) — **not an M1 gate** | 2-3 days |
| S1.7.4  | Static NAF dependency map (observability) — **done** 2026-06-01 (`Engine.naf_dependency_map()` + default-off `DerivedNafWarning`; Scope B / post-saturation) | ~½ day |
| S1.7.5  | Query semantics: who vs where — **done** 2026-06-01 (option 2: `:goal` joins through house to `nation-loc`; `answer.py` de-hardcoded) | 1-2 days |
| S1.7.7  | Kernel purity analysis — **comprehensive inventory** of every hardcoded engine name + de-risk spike (**parked**, phase end) — **not an M1 gate** | research |
| S1.7.8–.22 | **Per-name decomposition** (one stage file per hardcoded name): `is-a` (.8), `symmetric` (.9), `closed` (.10), `open` (.11), `not`/`false`/`and`/`or`/`forall`/`absent` (.12–.17), `eq`/`neq` (.18), bookkeeping heads (.19), `hypothesis-relations` (.20), task-class entries (.21), engine-internal strings (.22) — each: functionality, removal consequences, ein-lang-rule route, name-agnostic route | analysis |
| S1.7.23 | **Retire the kernel-imposed type system** (`is-a`/`T`) — execution of the [S1.7.8](s1.7.8_isa.md) finding (spike-verified no-op on zebra2); kernel keys on no inheritance name; inheritance rules stay inline (stdlib extraction → P1.8) — **not an M1 gate** | ~1-2 days |
| S1.7.24 | **De-hardcode `symmetric`** — execution of the [S1.7.9](s1.7.9_symmetric.md) decision: kernel keys on `is_symmetric` nowhere (generation → user hrule; `k` → generic state-dedup; mirror dropped); sound for user-defined `symmetric` + induction. Converges with S1.7.23 on hrule-driven generation — **not an M1 gate** | ~2-3 days |
| S1.7.25 | **Reserved names: encapsulate + document** — the *kept* half of the kernel: isolate the rule-body/⊥ primitives (`false`/`not`/`and`/`or`/`absent`, `eq`/`neq` model) + `closed` (predicates.py pattern); mark bookkeeping heads / `hypothesis-relations` / task classes reserved M1 strings; write the reserved-names docs in `docs/kernel/ir/03-ein-lang` + `docs/kernel/inference`. `open`/`forall` → P1.8 macros | ~2-3 days |

## Acceptance

> **Relocated 2026-05-31.** The M1 acceptance *gate* (solve / trace /
> three-task-class / answer-in-words) moved to
> [**P1.7a**](../p1.7a_solution_search_refactor/README.md) when S1.7.3's
> soundness bug turned the fix into a structural refactor. Criteria 1–4
> below are now **owned by P1.7a S1.7a.7**; they remain here as the
> historical statement of the gate. P1.7 retains the *encoding* and
> *kernel-purity* work (S1.7.1/.2/.4/.5/.6/.7), none of which is the gate.

This phase *was* the M1 acceptance. **Target file:
`examples/zebra2.ein`** (canonical; `zebra.ein` is a non-solving
demonstrator). The full set:

1. A CLI **answer path** on `zebra2.ein` exits 0 and emits the
   canonical answer in words:

   > The **Japanese** keeps the **zebra**. The **Norwegian**
   > drinks **water**.

   (The engine already solves — `test_monotonic_skeleton`; the
   work is wiring `monotonic_solve` into the CLI + rendering the
   answer line. See S1.7.3 T1.7.3.3.)

2. The markdown trace satisfies the *named-rule-firing checklist*
   — **shipped as** [S1.6.5](../p1.6_rendering_and_trace/s1.6.5_idea08_trace_acceptance.md)
   (`s1.6.5_idea08_checklist.md` + `test_idea08_acceptance.py`);
   the per-move YAML harness is **not** built (user decision
   2026-05-30).

3. `ein-bot solve examples/zebra2-minus-15.ein --mode=gaps`
   returns at least one diverging goal node.

4. `ein-bot solve examples/ein-bugs/zebra2-bad.ein --mode=contradictions`
   returns a 2–3 edge unsat core including the injected fact and
   the colliding condition (6).

5. Pytest default suite green and < 30 s (the `EIN_RUN_SLOW`
   zebra gates excluded); `ruff check .` green. (Suite is already
   ~1038 tests, well over the original ≥ 100.)

**Kernel purity ([S1.7.6](s1.7.6_kernel_minimization.md) +
parked [S1.7.7](s1.7.7_kernel_purity_analysis.md)) is *not* part
of this gate** — zebra2 solves with `is-a` / `symmetric` /
`closed` as primitives.

## Connections

- The whole P1.7 is the operational check that
  [idea 05](../../../docs/ideas/05-zebra-puzzle-graph-reasoner.md) and
  [idea 06](../../../docs/ideas/06-inference-rules-completeness.md)
  produced an implementable design.
- [Idea 08 §The target trace](../../../docs/ideas/08-human-style-deductive-trace.md#the-target-trace-paraphrased)
  is the canonical regression target.
- [Idea 03](../../../docs/ideas/03-three-task-classes.md) — all three
  task classes get exercised here.
