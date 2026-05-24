# P1.8 — Improvements

**Estimate:** TBD.
**Status:** **placeholder** — themes parked here for post-P1.7
attention. Originally the home of the modules/imports deferral
from S1.3.0 R8 (2026-05-20); broadened 2026-05-21 to also collect
runtime optimisations the M1 implementation surfaced; broadened
again 2026-05-22 — Theme A now also owns the **standard library**
(closure auto-inference deferred whole from S1.5.5, plus
`converse`, the `imply` family, general totality, reflective
rule-implication, and type/domain matching).
**Depends on:** varies per theme; see each §.
**Blocks:** nothing within M1 acceptance — P1.7 still gates "M1
done" regardless. P1.8 ships when one of its themes acquires
enough motivation (puzzle authoring grows past one file, perf
hits ergonomic thresholds, or a future puzzle's branching cost
overruns the laptop).

Directory name `p1.8_ein_lang_modules` is historical from when the
phase was modules-only; the file inside this directory is the
authoritative scope statement (rename deferred to avoid a churn
of cross-link breakage across S1.3.0, P1.3 README, etc.).

## Themes

### Theme A — Ein-lang modules, imports + standard library

The original P1.8 deferral from S1.3.0 R8.

P1.8 designs the **module / import / include mechanism** for
ein-lang and decides where rule libraries live across files. The
phase is the canonical home for:

- [Q30 — Universal rule library + import mechanism](../open_questions.md#q30--universal-rule-library--import-mechanism)
  — the question P1.3 deferred here.
- Any related decisions about cross-file references in `.ein`
  files (relation / type / instance imports, namespace scoping,
  re-exports, conflict resolution).

The P1.3 review surfaced three coupled questions:

1. Where do rule definitions live — inline per puzzle, in a
   universal library, or hybrid?
2. How does one `.ein` file reference rules in another?
3. How are conflicts resolved when a puzzle redefines a library
   rule?

These collectively form a *module-system design* too large for
P1.3 (rule engine) to absorb. P1.3 proceeds under the working
assumption that **rules are inline per puzzle** (current zebra.ein
convention); P1.8 revisits and decides.

Likely stages once activated:

- **S1.8.A1** — module-system design (file-level vs symbol-level
  imports; namespace scoping; conflict policy).
- **S1.8.A2** — grammar / parser extensions for `(import …)` /
  `(use …)` (or analogous) forms.
- **S1.8.A3** — loader changes: resolve imports, merge modules
  into a single `KnowledgeBase`, detect conflicts.
- **S1.8.A4** — rule library location (Q30 options a / b / c →
  final call). If (a) or (c): ship `examples/rules.ein` (or
  equivalent) with the kernel rule core.
- **S1.8.A5** — migration: zebra.ein adopts imports for any rules
  promoted to the universal library. If Q30 lands on (b), this
  stage is empty.

#### Standard library

Imports (above) make the rule library a *shared module* a puzzle
`(import …)`s instead of inline-copying. Beyond the existing
kernel rules (`symmetric`, `transitive`, `implies`, `square-*`,
`type-exclusivity`, …), the stdlib is the home for the families
and engine support discussed **2026-05-22** and deferred out of
M1's P1.5.

**Closure auto-inference** — the whole of the former
[S1.5.5](../p1.5_hypothesis_loop/s1.5.5_closure_auto_inference.md),
deferred here 2026-05-22. An `infer-closure` rule asserts
`(closed R)` so an author needn't declare it by hand; gated by
`:enable-auto-closure`. Two corrections from the review:

- **Parameter-less.** `(rule infer-closure () :match … :assert
  (closed ?R))`. S1.5.5's `(?R)` sketch reads as a *param list*;
  the engine then wants `(infer-closure …)` activator facts
  (`engine.py` `_activators_for` — non-empty `params` ⇒
  `rule.applications`), finds none, compiles zero plans, never
  fires. `?R` is a free match var, like `hypothesis-contradiction
  ()`.
- **`functional ⇒ closed` is too weak.** `functional` gives
  uniqueness (≤ 1 image), not completeness. Principled witness:
  **`functional ∧ total ⇒ closed`**. Two senses of "closed":
  *strong* (the extension cannot grow — a cardinality fixpoint,
  needs a count predicate the eq/neq-only matcher lacks) and
  *operational* (hypgen needn't branch on `R` — every `R`-fact
  arrives by saturation). S1.5.4/5's `(closed R)` is operational;
  `functional ∧ total` is a sound operational witness once
  domain-elimination ([S1.5.8](../p1.5_hypothesis_loop/s1.5.8_totality_domain_elimination.md))
  exists. Stronger witnesses (cardinality saturation; every cell
  decided `R`-or-`¬R`) need the count predicate — later.

**`converse` rule** — `(converse R1 R2)` ⟺ `R2 = R1°`, i.e.
`(R1 ?a ?b) ⇒ (R2 ?b ?a)` — the user's `imply2-reverse`. Algebra
lemmas to ship with it: `symmetric R ⟺ converse R R`; `converse`
is symmetric on the pair. `right-of`/`left-of` are converse,
`next-to` is self-converse.

**`imply` family** — one rule per arity (the matcher compiles
fixed-arity `Scan`/`Join`, no variadic slot): `imply1`
(`(R1 ?a) ⇒ (R2 ?a)`), `imply2-fwd` (the shipped `implies` —
`examples/zebra2.ein` `(implies right-of next-to)`),
`imply2-reverse` (= `converse`). `imply1` doubles as
**property→property implication**: a property fact
`(functional is-a)` is a 1-arg fact, so `(imply functional
closed)` *is* `imply1` — no new rule shape.

**General totality.** S1.5.8 ships a **zebra-scoped** `(total …)`
+ `domain-elimination` + `StructuralScan`, kept M1-blocking
(2026-05-22 decision). The stdlib re-homes the *general* form —
totality as a reusable library property, `domain-elimination` /
`elimination-by-exhaustion` as a library rule, the
auto-infer-totality idea (Q-S1.5.8.A). Concept overlap with
S1.5.8 is **accepted**: S1.5.8 is the zebra-acceptance slice, the
stdlib the reusable generalisation.

**Engine support the stdlib needs** — two capabilities M1 lacks;
the stdlib is their first real consumer.

- **Reflective rule-implication.** For `imply1` to implement
  implication *between rules* (derive `(symmetric foo)` → the
  `symmetric` rule then fires on `foo`), a *derived* fact must be
  able to activate a rule. It cannot: `Engine.compile_all()`
  snapshots `rule.applications` **once**; `Saturator._enqueue_pass`
  then iterates the frozen `engine.cache`, so a derived activator
  gets no plan. The store side is fine (`_index_fact` live-updates
  `_rule_apps_by_rule`); only the engine cache is static. Fix
  (~6 lines): after a productive firing whose conclusion head is
  a rule name, call `engine.compile_for(rule, derived_fact)` (it
  exists, idempotent) + set `_needs_enqueue`. This is **F5 (rules
  as data)** rung 2. Note `(imply functional closed)` itself does
  *not* need the fix — `closed` feeds hypgen (a live-index
  reader), not a rule.
- **Type / domain matching.** `match._bind_args` never consults
  `Relation.signature`; nothing enforces declared signatures — no
  relation-arg check *and* no rule-domain check. A generic stdlib
  (rules parametrised over relations) wants both, to catch an
  `imply`/`converse` across mismatched domains. M1 is deliberately
  untyped here (open-world); the stdlib is where typing earns its
  keep.

Likely stages once activated (continuing the A-numbering):

- **S1.8.A6** — closure auto-inference (`functional ∧ total ⇒
  closed`, parameter-less) + `:enable-auto-closure`.
- **S1.8.A7** — the `imply` family + `converse` + algebra lemmas.
- **S1.8.A8** — general totality / `domain-elimination` library
  form; reconcile with S1.5.8's zebra slice.
- **S1.8.A9** — reflective rule-implication (the compile-cache
  fix).
- **S1.8.A10** — type / domain matching for relations + rules.
- **[S1.5.9](s1.5.9_ein_lang_macros.md) — ein-lang pattern macros**
  (relocated 2026-05-24 from P1.5 with sticky id). Moves the
  `(forall …)` and `(open …)` parser sugars from compile.py SForm
  desugaring into ein stdlib via a `(macro NAME (params) BODY)`
  declaration form. The natural companion to imports — macros +
  imports together unlock a real shareable stdlib.

### Theme B - PERFORMACE

saturation that produce 150 facts and runs 1.5 seconds IS INSANE!
I estimate Python performance as 1M-10M+ ops/s
So it's already over 1M theoretical ops performed, or maybe even 10M or more.
Before saturation we have 64 facts and 8 rules.
If number of theoretical ops consumed is between (64*8)^2 and (64*8)^3
Ok, 64^8 is much bigger, but it doesn't make sense! The (64*8)^2 either!
WHAT IS THE SATURATION DOING SO LONG???

### Theme B1 - indexes

speedup every step with indexes
- index of all names
- indexes of names for every type (object, relation, rule, ....)
- index of facts by relation name, by number of args, by object



### Theme B2 — Copy-on-write hypothesis branching

P1.2's `KnowledgeBase.fork()` shallow-copies the `facts` list +
the four reverse-index dicts (O(|facts|) cost). For P1.5's
hypothesis loop, where each branch potentially forks many sub-
branches, that copy cost compounds.

**Insight (user direction 2026-05-21):** the M1 KB is **append-
only** — no deletes, no in-place writes, only appends. That makes
**true copy-on-write** trivially correct: a fork inherits the
parent's facts + indexes by reference; the fork's own appends
write to per-fork overlays; lookups consult the fork's overlays
first, then fall through to the parent.

Likely stages once activated:

- **S1.8.B1** — `ForkedKnowledgeBase` overlay class (or rework
  `KnowledgeBase` itself with a layered backing store).
- **S1.8.B2** — adjust the engine's `_facts_by_relation` lookups
  to traverse the parent chain transparently.
- **S1.8.B3** — saturator + contradiction-detector behaviour
  preserved (same `Firing` outputs, same `Contradiction` records).
- **S1.8.B4** — perf benchmark vs the shallow-copy fork (target:
  O(1) fork; lookup overhead ≤ depth × constant).

Out of scope: garbage-collecting collapsed branches (M1's append-
only model says we don't free anything). Out of M1 entirely.

### Theme C — Negative-fact volume reduction

P1.3 saturation produces **lots** of negative facts. Zebra.ein's
`(type-exclusivity co-located)` alone derives 120 `(not (co-located
A B))` facts; over the full saturation the REASONING layer holds
~120 not-facts vs ~25 positive derivations. The volume is
correct (each negative is a real consequence of the rule), but
much of it is *load-bearing only via the contradiction detector* —
the negatives are scanned for matching positives, never queried
directly.

**Possible optimisations** (each is a research direction, not a
quick win):

- **Lazy materialisation**: derive `(not X)` *on demand* when the
  contradiction detector asks rather than during saturation. Trade
  fact-volume for detector-time; the detector becomes a pull-style
  matcher.
- **Layer-aware filtering**: a `(not X)` derived from
  type-exclusivity is mechanically true given the puzzle's type
  declarations — it doesn't need to live in REASONING at all if no
  consumer reads it. Move to a virtual layer.
- **Goal-driven pruning**: combined with F7 §C (rule-set
  sufficiency), suppress type-exclusivity firings whose
  conclusions are demonstrably unconsumed by any subsequent rule
  *and* unconsumed by the query.
- **Compressed representation**: a *single* `(not (co-located ?
  ?_T))` fact with `?_T` denoting "any pair distinct under T",
  expanded lazily. Needs compound-node-kind support (Q26).

Likely stages once activated:

- **S1.8.C1** — measure: how much of the (not …) volume is
  actually consumed? Empirical study on Zebra + the demos.
- **S1.8.C2** — pick a representation (lazy / virtual layer /
  compressed) based on the measurement.
- **S1.8.C3** — refactor the saturator + detector + trace
  renderer to honour it.

Cross-cuts F7 §C (rule-set sufficiency) — the deepest win likely
combines volume reduction with activator-selection pruning.

## Acceptance (TBD per theme)

Each theme drafts its own when activated. Skeleton:

- **A:** a multi-file `.ein` project loads end-to-end; conflict
  policy documented; a puzzle `(import …)`s the stdlib instead of
  inlining rules; `infer-closure` / `converse` / the `imply`
  family fire from the imported library; zebra.ein continues to
  work.
- **B:** `fork()` is O(1); zebra.ein saturation through a fork
  matches the non-fork baseline; perf benchmark green.
- **C:** Zebra REASONING-layer fact count drops by ≥ 50% with no
  loss of contradiction-detection power; saturation time within
  20% of pre-optimisation.

## Open questions

- [Q30 — Universal rule library + import mechanism](../open_questions.md#q30--universal-rule-library--import-mechanism)
  (Theme A).
- **Stdlib — closure sense.** Which sense of "closed" the stdlib
  `infer-closure` rule targets (strong/semantic vs operational),
  and whether `:enable-auto-closure` defaults on once
  `functional ∧ total` proves sound (the old Q-S1.5.5.A,
  inherited with the deferral).
- **Stdlib — reflective rule-implication default.** Whether the
  compile-cache fix is always-on or gated; bears on F5.
- *(Theme B and Theme C have not yet surfaced specific Q-numbered
  questions; activation will probably introduce some.)*

## Cross-links

- Origin of Theme A (modules deferral):
  [`p1.3_inference_rules/s1.3.0_review_and_revisions.md` §G — Q30](../p1.3_inference_rules/s1.3.0_review_and_revisions.md#g-consolidated-open-questions-for-p13).
- Stdlib content deferred in from P1.5:
  [S1.5.5 — closure auto-inference](../p1.5_hypothesis_loop/s1.5.5_closure_auto_inference.md)
  (deferred whole, 2026-05-22);
  [S1.5.8 — totality + domain elimination](../p1.5_hypothesis_loop/s1.5.8_totality_domain_elimination.md)
  (zebra slice stays M1-blocking; the general form re-homes here).
- Related followups:
  [F2 self-modifying language](../../followups/f2_self_modifying_language.md),
  [F5 rules as data](../../followups/f5_rules_as_data.md) — F5 rung 2
  *is* the reflective rule-implication fix in the stdlib §; both
  would consume Theme A's module mechanism if it lands.
- Theme B context: P1.2's `KnowledgeBase.fork()` shallow-copy,
  P1.5 hypothesis branching's per-fork cost.
- Theme C context:
  [F7 — Rule taxonomy + rule induction](../../followups/f7_rule_induction.md) §C
  (rule-set sufficiency) is the upstream design partner.
