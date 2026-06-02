# P1.7c — Retire the `(rules …)` / `(ontology …)` / `(facts …)` block heads

**Estimate:** TBD (grammar + loader; small but touches every `.ein`).
**Status:** **placeholder** — created 2026-06-02 from `TODO.md`.
**Origin (user, 2026-06-02):**

> remove ein heads: `rules`, `ontology`, `facts` — make plain code. We
> already have special names for: `rule`, `hrule`, `relation`, `query`,
> `config`, and will have one more — `macro`; everything else is a fact.
> So the parser can easily detect facts by *not* being in
> `{rule, hrule, relation, query, config, macro}`.

**Depends on:** the P1.7 kernel-purity arc (S1.7.23/.24/.25) — this is the
*surface-syntax* continuation of the same "fewer special forms" thrust.
The reserved-name set it keys on is exactly
[`docs/kernel/ir/03-ein-lang/06_reserved_names.md`](../../../docs/kernel/ir/03-ein-lang/06_reserved_names.md)
(declarators) — once `macro` lands in [P1.8 S1.5.9](../p1.8_ein_lang_modules/s1.5.9_ein_lang_macros.md).
**Blocks:** nothing — M1 is done; this is a post-M1 ergonomics/purity
cleanup.

## The idea

Today a `.ein` file groups forms under wrapper block heads — `(ontology
…)`, `(facts …)`, `(reasoning …)`, `(rules …)`. The proposal: drop the
wrappers and write a **flat list of forms**; the parser classifies each
top-level form by its head:

- head ∈ `{rule, hrule}` → a rule / hypothesis-rule declaration;
- head = `relation` → a relation signature declaration;
- head = `query` → the query block;
- head = `config` → solver config;
- head = `macro` → a macro definition (once P1.8 S1.5.9 lands);
- **anything else → a fact.**

This removes three reserved *block* heads (`rules` / `ontology` / `facts`,
+ `reasoning`) and makes the grammar uniform: a program is just forms.

## Open design questions (the reason it's not trivial)

1. **Layer attribution.** The block heads currently carry the
   :class:`Layer` of their children — `(ontology …)` → ONTOLOGY,
   `(facts …)` → FACT, `(reasoning …)` → REASONING. Flat facts need
   another layer signal. Candidates: derive it (schema-shaped facts —
   `relation`/`type`/`instance`/property tags → ONTOLOGY; a `:source
   "(N)"` annotation → FACT; everything user-asserted → FACT); or a
   per-form `:layer` keyword; or drop the distinction for authored input
   (REASONING is engine-only anyway, so authored input is just
   ONTOLOGY-vs-FACT). **This is the crux** — the layer split is
   load-bearing for the contradiction detector's cross-layer rule and
   for the renderer's styling.
2. **Migration.** Every `examples/*.ein` + the test inline fixtures use
   the block heads; a flat-form rewrite is a wide (mechanical) churn, or
   the loader stays **back-compatible** (accept both the wrapped and flat
   forms) for a deprecation window.
3. **Reserved-head collision.** A fact whose head happens to be a future
   declarator name would be misclassified — pins the declarator set as
   *closed* (the [reserved-names](../../../docs/kernel/ir/03-ein-lang/06_reserved_names.md)
   doc becomes the parser's source of truth).
4. **Grammar shape.** Whether the top level is a bare sequence of forms
   or still a single `(program …)`-style root.

## Likely stages

- **S1.7c.1** — decide layer attribution (Q1) — the gating design call.
- **S1.7c.2** — grammar: top level = flat form sequence; classify by head
  against the reserved declarator set.
- **S1.7c.3** — loader: route each form by head; back-compat shim for the
  wrapped form (deprecation window).
- **S1.7c.4** — migrate `examples/` + inline fixtures; drop the shim.
- **S1.7c.5** — docs: grammar (`01_grammar.md`) + reserved-names update.

## Connections

- [P1.7 kernel-purity arc](../p1.7_bootstrapping_zebra/) — same "fewer
  special forms" thrust, one layer up (surface syntax vs engine).
- [P1.8 S1.5.9 macros](../p1.8_ein_lang_modules/s1.5.9_ein_lang_macros.md)
  — introduces the `macro` declarator this keys on.
- [`docs/kernel/ir/03-ein-lang/06_reserved_names.md`](../../../docs/kernel/ir/03-ein-lang/06_reserved_names.md)
  — the declarator set the flat-parser dispatches on.
