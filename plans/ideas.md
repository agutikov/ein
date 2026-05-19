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

### P1.2b audit — does the ein-model unification justify a new phase?

Added 2026-05-19, from the user's `# ein model` thoughts in TODO.md.

The reflexive ein-model unification — instance-of-instance, types-as-
relation-holders, the "no copies" + "everything is a node" framing
— landed as new kernel documentation in
[`docs/kernel/ir/01-ein-graph/03_ein_model.md`](../docs/kernel/ir/01-ein-graph/03_ein_model.md)
+ [`04_jack_drinks_coffee.md`](../docs/kernel/ir/01-ein-graph/04_jack_drinks_coffee.md).
The implementation already supports the model (all S1.2.x stages
shipped), but two open design questions surfaced:

- [Q27 — Relation declaration body form](m1_core_graph_reasoning/open_questions.md#q27--relation-body-form)
- [Q28 — Empty parens `()` semantics](m1_core_graph_reasoning/open_questions.md#q28--empty-parens-node-semantics)

User question: *"maybe introduce P1.2b phase that contains IR model,
language, implementation and docs update"*.

**Audit checklist** — does the unification trigger a P1.2b phase, or
does the docs-only embedding suffice?

- [ ] Does any of S1.2.1-S1.2.4's *acceptance* fail under the
      reflexive framing? — *Answer: no; the existing tests cover the
      reflexive model implicitly (e.g. `Relation.rule` returns the
      Rule node when a relation name matches a rule).*
- [ ] Are there new entity / index shapes needed? — *Answer: not
      for M1. F5 (rules-as-data) would need them; M1 doesn't.*
- [ ] Are there new grammar primitives needed? — *Answer: no; Q27
      defers form (a) as future sugar, Q28 punts `()` semantics.*
- [ ] Are there docs gaps the new files don't cover? — *Answer: TBD
      — a second-pass review of `01_kb.md` to harmonise with the new
      03_ein_model.md may be useful but not blocking.*

**Working answer:** *no P1.2b needed today*. The docs reorganisation
captures the model; the implementation already supports it; the two
open Qs (Q27/Q28) are parked, not blocking. Revisit if M1 acceptance
(P1.7) reveals a gap.

If promoted: would be a small (3-5 day) phase covering a kb-internal
audit + a docs-harmony pass + Q27/Q28 decisions.

---

## Promoted / pruned

> *(append entries here when removing them from "Live")*
