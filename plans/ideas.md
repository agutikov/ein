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

> *(none — promote new entries from raw notes / TODO.md here.)*

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
