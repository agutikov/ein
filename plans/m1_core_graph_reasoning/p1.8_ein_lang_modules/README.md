# P1.8 — Ein-language modules + standard library

**Estimate:** TBD.
**Status:** **placeholder** — the ein-language + library work parked for
post-P1.7 attention. Originally the home of the modules/imports deferral
from S1.3.0 R8 (2026-05-20); broadened 2026-05-22 — Theme A now also owns
the **standard library** (closure auto-inference deferred whole from
S1.5.5, plus `converse`, the `imply` family, general totality, reflective
rule-implication, type/domain matching, and the relation-algebra library
rules pulled from [F1b](../../followups/f1b_logical_formulation.md)).
**The performance themes split out to [P1.8a](../p1.8a_performance/) on
2026-06-02** (COW fork, version-COW, atom compression, fingerprinting,
participation indexes, negative-fact volume) — this phase is now
language + library only.
**Depends on:** varies per theme; see each §.
**Blocks:** nothing within M1 acceptance — P1.7 still gates "M1
done" regardless. P1.8 ships when one of its themes acquires
enough motivation (puzzle authoring grows past one file, a future
puzzle needs an imported stdlib rule).

Directory name `p1.8_ein_lang_modules` is historical from when the
phase was modules-only; the file inside this directory is the
authoritative scope statement (rename deferred to avoid a churn
of cross-link breakage across S1.3.0, P1.3 README, etc.).

## Stages

Theme A (modules + imports + standard library):

| ID         | Title                                                  | File                                                                  |
|------------|--------------------------------------------------------|-----------------------------------------------------------------------|
| S1.8.A1    | Module-system design                                   | [s1.8.a1_module_system_design.md](s1.8.a1_module_system_design.md)    |
| S1.8.A2    | Grammar / parser extensions for imports                | [s1.8.a2_grammar_import_use.md](s1.8.a2_grammar_import_use.md)        |
| S1.8.A3    | Loader: resolve imports                                | [s1.8.a3_loader_resolve_imports.md](s1.8.a3_loader_resolve_imports.md) |
| S1.8.A4    | Rule library location                                  | [s1.8.a4_rule_library_location.md](s1.8.a4_rule_library_location.md)  |
| S1.8.A5    | Migration to imports                                   | [s1.8.a5_migration_imports.md](s1.8.a5_migration_imports.md)          |
| S1.8.A6    | Closure auto-inference                                 | [s1.8.a6_closure_auto_inference.md](s1.8.a6_closure_auto_inference.md) |
| S1.8.A7    | `imply` family + `converse` + algebra lemmas           | [s1.8.a7_imply_converse_algebra.md](s1.8.a7_imply_converse_algebra.md) |
| S1.8.A8    | General totality + domain-elimination library form     | [s1.8.a8_general_totality.md](s1.8.a8_general_totality.md)            |
| S1.8.A9    | Reflective rule-implication                            | [s1.8.a9_reflective_rule_implication.md](s1.8.a9_reflective_rule_implication.md) |
| S1.8.A10   | Type / domain matching                                 | [s1.8.a10_type_domain_matching.md](s1.8.a10_type_domain_matching.md)  |
| S1.8.A11   | Multi-fact assertion from a single rule                | [s1.8.a11_multi_fact_assert.md](s1.8.a11_multi_fact_assert.md)        |
| S1.8.A12   | Relation-algebra library rules (from F1b §PFL.3)       | *(folded into the stdlib § below; stage file TBC)*                   |
| S1.5.9     | Ein-lang pattern macros (sticky id, relocated 2026-05-24) | [s1.5.9_ein_lang_macros.md](s1.5.9_ein_lang_macros.md)              |

The **performance** themes (COW fork, version-COW, atom compression,
unsat-core fingerprint, participation indexes) and the **negative-fact
volume reduction** theme moved to [P1.8a](../p1.8a_performance/) in the
2026-06-02 split. Theme A activates when authoring scale demands a shared
stdlib.

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

#### Adding a top-level declarator — the P1.7c checklist

Two Theme-A stages introduce new top-level forms — `macro`
([S1.5.9](s1.5.9_ein_lang_macros.md)) and `import` / `use`
([S1.8.A2](s1.8.a2_grammar_import_use.md)). Since
[P1.7c](../p1.7c_block_head_removal/README.md) the surface is a **flat
sequence of forms classified by head against a closed declarator set**, so
each new declarator is a **4-point registration** (not a new wrapper block):

1. **`grammar.lark`** — add the production to the `?form` alternation.
2. **`grammar.lark`** `SYMBOL` negative-lookahead — add the head so a
   malformed form is a *parse* error, not a silent fall-through to a fact.
   (`relation` is the one documented exception — kept a plain `SYMBOL` so
   rules can pattern-match `(relation ?R ?A ?B)`.)
3. **[`06_reserved_names.md`](../../../docs/kernel/ir/03-ein-lang/06_reserved_names.md)**
   — add the head to the closed Declarators table (the parser/loader's source
   of truth; it already forward-reserves `macro`).
4. **`from_ir.load()`** head-switch — add a routing branch.

Each addition **expands the reserved set**, so a pre-existing fact whose head
collides would silently misclassify — grep the corpus first (none collide for
`macro` / `import` / `use` as of 2026-06-03).

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
- **S1.8.A10** — type / domain matching for relations + rules. The
  *opt-in* typing that complements
  [S1.7.23](../p1.7_bootstrapping_zebra/s1.7.23_retire_kernel_type_system.md)
  (P1.7), which *removes* the kernel's *imposed* `is-a`/`T` type system;
  A4/A5 then extract the inheritance rules + `guess` hrule into the
  importable `stdlib/types.ein`.
- **S1.8.A11** — multi-fact assertion from a single rule
  (`:assert (and …)`); the model is one-fact-per-firing today (see the
  [stage file](s1.8.a11_multi_fact_assert.md)). Surface syntax TBC.
- **[S1.5.9](s1.5.9_ein_lang_macros.md) — ein-lang pattern macros**
  (relocated 2026-05-24 from P1.5 with sticky id). Moves the
  `(forall …)` and `(open …)` parser sugars from compile.py SForm
  desugaring into ein stdlib via a `(macro NAME (params) BODY)`
  declaration form. The natural companion to imports — macros +
  imports together unlock a real shareable stdlib.

#### Relation-algebra library rules (S1.8.A12)

Pull the **relation-algebra operations** from
[F1b §PFL.3](../../followups/f1b_logical_formulation.md) into the stdlib
as importable library rules (TODO direction 2026-06-02). F1b §PFL.3 would
systematise the operations the zebra rules already encode ad-hoc as
activators — `compose`, `converse`, `meet`/`join`, identity/`top`/`bottom`
— plus the equivalence-relation laws (reflexive / symmetric / transitive
as a packaged bundle). The A7 `imply` / `converse` family is the seed;
A12 generalises it to the full relation-algebra signature so a puzzle
`(import stdlib/algebra)` instead of re-deriving `right-of`/`left-of`
converse + `next-to` self-converse + transitivity by hand. Sequencing:
after A7 (the `converse` / `imply` lemmas) and A1–A3 (imports), so the
algebra ships as a real importable module; the FOL-fragment + rules-of-
inference framing in F1b is the design backdrop.

### Theme B + Theme C → moved to P1.8a

The **performance** themes (indexes, copy-on-write fork + version-COW,
atom-vector compression, unsat-core fingerprinting, fact-participation
indexes) and the **negative-fact volume reduction** theme were split out
to **[P1.8a — Performance](../p1.8a_performance/)** on 2026-06-02 (they
are runtime optimisations, not ein-language / library work). See that
phase's README.

## Acceptance (TBD per theme)

Theme A drafts its own when activated. Skeleton: a multi-file `.ein`
project loads end-to-end; conflict policy documented; a puzzle
`(import …)`s the stdlib instead of inlining rules; `infer-closure` /
`converse` / the `imply` family / the relation-algebra rules fire from
the imported library; zebra2.ein continues to work. (The Theme B / C
acceptance moved to [P1.8a](../p1.8a_performance/).)

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
- **Relation-algebra surface (A12).** How far F1b §PFL.3's
  `compose`/`converse`/`meet`/`join` go as library rules vs needing
  new engine support (e.g. composition wants a join the matcher
  already does; a variadic algebra would not).

## Cross-links

- Sibling phase: [P1.8a — Performance](../p1.8a_performance/) (the
  runtime-optimisation half, split off 2026-06-02).
- Origin of Theme A (modules deferral):
  [`p1.3_inference_rules/s1.3.0_review_and_revisions.md` §G — Q30](../p1.3_inference_rules/s1.3.0_review_and_revisions.md#g-consolidated-open-questions-for-p13).
- Stdlib content deferred in from P1.5:
  [S1.5.5 — closure auto-inference](../p1.5_hypothesis_loop/s1.5.5_closure_auto_inference.md)
  (deferred whole, 2026-05-22);
  [S1.5.8 — totality + domain elimination](../p1.5_hypothesis_loop/s1.5.8_totality_domain_elimination.md)
  (zebra slice stays M1-blocking; the general form re-homes here).
- Relation-algebra source:
  [F1b — logical formulation](../../followups/f1b_logical_formulation.md)
  §PFL.3 (A12).
- Related followups:
  [F2 self-modifying language](../../followups/f2_self_modifying_language.md),
  [F5 rules as data](../../followups/f5_rules_as_data.md) — F5 rung 2
  *is* the reflective rule-implication fix in the stdlib §; both
  would consume Theme A's module mechanism if it lands.
