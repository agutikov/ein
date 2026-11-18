# Q-M1f.426 — The constraint view draws each fact as a fact, reading no name

`ein render constraints` draws the facts a program states with no `:source`, `:rule` or `:using`, of a relation it
does not declare: each one as a fact, by the KB view's code, an octagon whose arrow `1` is at its relation and whose
next arrows are at its arguments, or folded under `--fold`
([Q-M1f.425](Q-M1f.425-fold-folds-a-fact-of-three-arrows-into-an-arrow.md)). `(symmetric next-to)` is an octagon
whose arrow `2` is at `next-to`, never a word in `next-to`'s node. No name is read for what it means: a declaration,
the fact the loader stores, of `relation`, which no program declares, is drawn among them. A fact one of them has
under an arrow is its node, in its frame where the program does not state it. The view is one slice of the knowledge
base, the facts it keeps.

**Why.** A property of a relation is a fact about the relation, and a drawing that wrote it into the relation's node
would draw a fact as text. Which facts the view keeps is read off their shape, a fact of a relation the program does
not declare, so that it draws whatever a program states and gives no theory's names a reading
([Q-M1f.436](../engine/Q-M1f.436-the-kernel-reads-no-theorys-name.md)).

**Held by.** `ein_render::constraints`; `ein-render/tests/every_object_once.rs`:
`the_constraint_view_draws_a_property_as_a_fact`, and the sweep of every view of every corpus program. The page is
[rendering](../../ein/user/04-reference/12-rendering.md#constraints--ein-render-constraints).
