# F5 — Operate IR rules as data (graph self-rewriting)

**Theme owner:** the user.
**Trigger:** P1.3 ships (rule firing engine is real) + M3 partial
(some self-reflection use-case in mind).

## What this is

In ein-bot's design, **rules are first-class graph objects** — Rule
entities, Pattern dataclasses, property-application facts. F5 takes
the next step: have the *engine itself* manipulate rules **as graph
data** during reasoning, not just as static configuration loaded
once at startup.

Concrete shapes this can take:

- **Rules that rewrite rules.** A meta-rule whose LHS matches *other
  rule patterns* and whose RHS modifies them — pattern fusion (two
  rules with overlapping LHS combined into one), pattern
  specialisation (a generic rule narrowed to a specific relation),
  pattern abstraction (a concrete rule generalised by replacing a
  named relation with a variable).
- **Inducing rules from facts** ([F4 Q37](f4_cross_cutting.md)) — a
  meta-rule that *discovers* `(transitive R)` by observing that
  `(R a b) ∧ (R b c) ∧ (R a c)` repeatedly co-occurs.
- **Rules over the inference trace.** A rule whose LHS matches
  `(step …)` forms in the `(trace …)` block, and whose RHS proposes
  a *shorter* derivation path — proof simplification.
- **Conditional rule activation.** A rule that fires only after
  another rule has fired *N* times — useful for hypothesis-loop
  heuristics ("after 5 propagation steps with no progress, enable
  global-cardinality search").

The unifying observation: **the engine's working memory and the
engine's rule library are the same graph**. F5 makes this concrete.

## Why deferred

The data-model substrate already supports F5 — rules are entities,
patterns are structural objects, the matcher in P1.3 will operate on
them. What's *not* yet in M1:

- A way for a rule's RHS to *target a rule node* rather than a fact
  node. The current `:assert <pattern>` produces facts; F5 needs
  `:assert <rule definition>` semantics.
- An audit trail for rule mutations (which rule produced which new
  rule, when, with what justification). Provenance already exists
  for facts; F5 extends it to rules.
- A loop-termination story. Meta-rules that produce meta-rules that
  produce meta-rules can run forever; F5 needs a meta-saturation
  bound or a stratification scheme.

## What promotion would look like

A new milestone `m_followups_self_modifying/` (shared with F2 and
F6 — see [`../../docs/ideas/10-generic-self-modification.md`](../../docs/ideas/10-generic-self-modification.md))
with phases:

- **PF5.1** — extend `:assert` to allow rule-shaped RHS; loader and
  matcher accept the new form.
- **PF5.2** — rule-on-rule pattern matching: the matcher iterates
  over `kb.rules.values()` the same way it iterates over
  `kb.facts`.
- **PF5.3** — rule mutation provenance: each rule entity carries a
  `derived-from` field analogous to a fact's `provenance.premises`.
- **PF5.4** — stratification / termination: pick a scheme (negation
  stratification, magic-set transformation, or a hard meta-depth
  bound) and audit a worked example.
- **PF5.5** — worked example: a "rule learning" puzzle where the
  engine starts with `(R a b) (R b c) (R a c)` facts and induces
  `(transitive R)` as a new ontology fact.

## Risks

- **Non-termination.** Meta-rules can build meta-rules without
  bound. Stratification or a hard depth cap is necessary; the design
  is non-trivial.
- **Soundness drift.** A rule that mutates another rule can break
  invariants the original rule depended on. The provenance audit
  trail (PF5.3) is the diagnostic, not a prevention.
- **Loss of trace fidelity.** Mutated rules need their own `:why`
  templates; if the meta-rule doesn't generate readable
  explanations, the trace becomes opaque. M1's acceptance criterion
  (idea 08) carries over to F5: every firing — including a
  meta-firing — must name its cause.

## Kernel minimisation — which inference features belong in ein-lang vs kernel code?

User direction 2026-05-27: the kernel today carries quite a
bit about *how* it derives facts and *where* it propagates
them — saturation, branching, hypothesis generation,
hypothesis filtering, back-prop, the consume loop, NAF
re-eval, the lookahead, the path-condition nogoods, the
mid-sweep re-saturation. The current target is **reasonably
minimal**, not **theoretically minimal** or **esoterically
minimal** — but reasonably-minimal is itself an audit
question:

> *Which inference features make sense to express in ein-lang
> instead of kernel code?*

For each kernel mechanism, the audit asks:

1. Can the mechanism be expressed as ein-lang rules / activators
   without engine support? (E.g., S1.5a.19's `functional-negative`
   et al. are ein rules — they used to be implicit in the engine
   via refutation chains, now they're declared.)
2. If yes, is the engine-side version *strictly more efficient*,
   or just a historical accident?
3. If "just historical", the move is engine → ein-lang: rewrite
   the feature as a stdlib rule pack, drop the engine code.
4. If "engine is strictly more efficient", the feature stays in
   the kernel — but its existence is documented as a
   load-bearing engine primitive (composes with P1.20 Theme I
   features-table).

Sub-track of F5 because the eventual format of the audit
is "rules-that-replace-engine-code" — a literal
rules-as-data exercise. Candidate kernel features to audit:

- **Saturation** — the firing loop itself. Is it expressible
  as a meta-rule that iterates until quiescence? (Probably
  yes, but with stratification headaches.)
- **Branching / hypgen** — the hypothesis enumeration. Is the
  candidate-generation logic expressible as ein-lang queries
  + activators? (S1.5.4's hypgen-filter logic is engine-side
  today; some of it might lift.)
- **Hypfilter (lookahead-kill)** — already an opt-in feature
  (`enable_pre_branch_lookahead`); audit whether it's a
  natural rule rather than engine code.
- **Back-prop** — `(not h)` writes on unconditional death.
  Could this be a meta-rule that reacts to a
  `(branch-died ?branch_id)` activator?
- **NAF re-eval** — fire-time check of `AbsentGuard`. This
  is probably load-bearing engine; document as such.

Promotion trigger: an M1-tail or P1.20 cycle that's
audit-friendly (no active design pressure on the kernel).
The output is (a) a smaller kernel, (b) more rules in the
stdlib, (c) Theme I features-table cross-references the
audit's conclusions.

## Prior art / connections

- [F4 Q34](f4_cross_cutting.md) — the rule-property cartesian product;
  the catalogue of "meaningful" relation profiles that meta-rules
  would label / classify.
- [F4 Q37](f4_cross_cutting.md) — induction from facts; F5 is the
  general case.
- AtomSpace's *Hebbian learning* over atom truth-values (see
  [`docs/index/09-cognitive-architectures-neurosymbolic.md`](../../docs/index/09-cognitive-architectures-neurosymbolic.md))
  — different mechanism (statistical, not deductive) but same
  ambition.
- ACL2's *macro hygiene* and `defaxiom` machinery — closest formal
  analogue. Their lesson: track which axioms were user-added vs
  built-in, and refuse to discharge proofs that depend on questionable
  user-added axioms unless explicitly invoked.
- E-graphs (F4 Q30) — rules-as-equivalence-classes-of-rewrite-rules
  is one natural F5 substrate.
- F2 (self-modifying language) is the *syntactic* analogue at the
  grammar level; F5 is the *semantic* analogue at the rule level.
  See [docs/ideas/10-generic-self-modification.md](../../docs/ideas/10-generic-self-modification.md)
  for the umbrella view.
