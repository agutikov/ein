# Documentation — Low

## Small internal defects in otherwise-normative pages

**Severity:** Low
**Confidence:** High
**Topic:** Documentation

**Locations**
- `docs/kernel/defined_behaviour.md:304-328` ("Nine more" above a 10-row table later called "all ten")
- `docs/kernel/ir/03-ein-lang/06_reserved_names.md:230-233` (the keyword arithmetic — "the six above plus :goal, :goal-text, :hrules" — does not reconstruct the actual 7-keyword allow-list)
- `docs/kernel/ir/01-ein-graph/01_kb.md:26-33, 110-141` (the table says relation nodes are round-rects; both Levi DOT examples in the same file draw them as hexagons — the shape the table assigns to Rule)
- `docs/kernel/ir/03-ein-lang/03_examples.md:20-21` (garbled sentence: "whose :source derives the a given")

### Finding

Each is a one-line fix; grouped here because they all sit in pages the tree calls normative.

---

## Frozen measurements presented as current

**Severity:** Low
**Confidence:** Medium
**Topic:** Documentation

**Locations**
- `examples/README.md:39-43` (46.9 ms / 31.1 ms zebra timing rows)
- `docs/guide/03_rule_families.md:73-75` (≥23×, 101 → 3336+ commitments)

### Finding

Both cite re-takable measurement docs, but the inline numbers have no re-take mechanism and predate M1d's engine changes (the S1d.2.4 activator facts changed fact counts — docs/api/rust.md documents its own 434→444 move; these tables did not get the same audit).

### Recommendation

Date the numbers in place ("as of M1a close") or drop them in favor of the measurement-doc links.
