# Generic self-modification — three rungs

The user's framing of the self-modifying-system goal as **three
rungs of a ladder**, each riding on the substrate the previous one
established. None is on a near-term schedule; together they describe
the asymptotic direction.

## The three rungs

### Rung 1 — language (grammar) self-modification

The system mutates the **grammar** that constrains its own output.
GBNF + LLM in a feedback loop; the LLM proposes grammar updates, the
harness applies them, the LLM's next call is constrained by the new
grammar.

- Followup: [F2 — Self-modifying constraint language](../../plans/followups/f2_self_modifying_language.md).
- Lineage: [`docs/ideas/01-self-modifying-constraint-language.md`](01-self-modifying-constraint-language.md)
  — the original "most ambitious idea" framing.

This is the **syntactic** rung. It changes what the system can *say*.

### Rung 2 — rule (semantic) self-modification

The system mutates **its own rule library** — rules can rewrite
rules, induce new rules from fact patterns, simplify proofs by
rewriting trace nodes. The substrate is the same graph engine; the
modification target is the rule entities in the KB.

- Followup: [F5 — Operate IR rules as data](../../plans/followups/f5_rules_as_data.md).
- Lineage: F4 Q34 (rule property cartesian product), F4 Q37
  (induction from facts), F4 Q36 (rule polymorphism).

This is the **semantic / declarative** rung. It changes what the
system can *prove*.

### Rung 3 — harness (procedural) self-modification

The system mutates **its own Python source code** under
`src/ein_bot/`. The most distant rung; only meaningful with F2 + F5
already in place to confine the feedback loop to a known formal
substrate.

- Followup: [F6 — Engine reads + modifies its own harness code](../../plans/followups/f6_modify_own_harness.md).

This is the **procedural** rung. It changes what the system can *do*.

---

## Why three separate followups and not one

Each rung has its own:

- Substrate (GBNF / IR-rules / Python-AST).
- Audit-trail mechanism (grammar versions / rule provenance / git
  commits).
- Failure mode (private-dialect drift / non-termination / bricked
  process).
- Research question (semantic firewall / stratification /
  improvement metric).

Splitting them lets each get its own design-and-risk treatment
without conflating mechanisms. A unifying `m_followups_self_modifying/`
milestone would house phases from all three, but each phase is
distinct.

## Why this idea matters

The three rungs trace a *recursive deepening* of agency over a
formal substrate:

```text
   "What can I say?"    ← F2 (grammar) mutates this
   "What can I prove?"  ← F5 (rules) mutates this
   "What can I do?"     ← F6 (harness) mutates this
```

Each rung's safety hinges on the **next-lower** rung's reliability:
F2 needs the grammar to remain *interpretable* by the F5 engine; F5
needs the engine to remain *executable* by the F6 harness; F6 needs
the harness to remain *under human control*.

That ordering — grammar before rules before harness — is also the
order in which the substrate matures: M2 builds the GBNF infra (F2
substrate), P1.3 builds the rule engine (F5 substrate), and the
existing Python package (already shipped) is the F6 substrate.

## What ein-bot M1 contributes

M1 doesn't implement *any* self-modification. It builds the
*non-mutable* substrate the followups need:

- M1 ships the IR + KB + (planned) inference engine, providing the
  pieces F5 will operate over.
- M2 ships GBNF infra + NL→IR, providing the grammar F2 will mutate.
- Both build on a Python package F6 might one day modify.

The followups are *not* implementation milestones for M1's scope;
they're the *answer to "why is M1 worth building this way?"* — the
shape of the kernel is chosen so the followups *could* be added
without rework.

## Open questions

1. **What's the smallest interesting demo of any rung?**
   - F2: a Zebra grammar that the LLM optimises for a specific
     puzzle dialect (idea 01's bounded problem domain).
   - F5: induction of `(transitive R)` from observed fact patterns.
   - F6: a refactoring suggester that emits patches to its own
     source code (no application — just suggestion).
2. **How do the three rungs interact?** Can F5 rules emit GBNF
   grammar diffs (Rung 2 → Rung 1)? Can F2 grammar updates expose
   new patch DSL primitives for F6 (Rung 1 → Rung 3)? Probably yes,
   but the safety story compounds.
3. **Where's the audit-trail unification point?** Each rung has its
   own audit mechanism. A shared "self-modification log" that
   threads grammar / rule / patch mutations into a single timeline
   would help; design TBD.

## Connections

- [F2](../../plans/followups/f2_self_modifying_language.md) — rung 1.
- [F5](../../plans/followups/f5_rules_as_data.md) — rung 2.
- [F6](../../plans/followups/f6_modify_own_harness.md) — rung 3.
- [Idea 01](01-self-modifying-constraint-language.md) — the original
  framing (predates the three-rung decomposition).
- [Idea 02](02-graph-as-formal-substrate.md) — the graph-as-formal-
  computation substrate that makes F5's "rules-as-data" coherent.
- [F4 Q34 / Q36 / Q37](../../plans/followups/f4_cross_cutting.md) —
  the rule-polymorphism / induction threads that feed F5.
