# Q-M1f.433 — An object is a labelled box, a fact an unlabelled octagon, a variable a labelled diamond

In every drawing of facts an atom is a box labelled with its name, whatever relation points at it; a fact is an
octagon with no name in it; a pattern's variable, in a rule's drawing, is a diamond labelled with its name, `?x`.
No other shape stands for an object.

**Why.** An object is an atom or a fact, and nothing tells one atom from another but its name: Ein has no types
([Q-M1f.372](../language/Q-M1f.372-ein-has-no-types-a-declarations-slot-names-are.md)), and no relation is drawn
otherwise than another ([Q-M1f.421](Q-M1f.421-no-view-draws-a-relation-otherwise-than-another.md)). A fact's
identity is its tuple, which its arrows draw, so its node carries no name.

**Held by.** `ein_render::dot_util` (`GROUND_SHAPE`, `HYPER_SHAPE`, `VAR_SHAPE`) and `ein_render::kb_dot`'s node
default; `ein-render/tests/every_object_once.rs`, which fails on an octagon with a name in it or an oval, in every
view of every corpus program. The page is
[rendering](../../ein/user/04-reference/12-rendering.md#node-shape-legend).
