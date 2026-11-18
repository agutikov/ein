# Q-M1f.398 — A fact in a context only is drawn in a frame named for the context's relation

In a drawing of the graph, the ones the pages write by hand and every view `ein render` writes, a fact that an
argument arrow of a fact points at is in a context ([Q-M1f.394](../language/Q-M1f.394-a-fact-is-in-the-global-context-in-a-facts.md)). It stands
**inside a dashed, rounded frame** exactly when it is in a context only: pointed at, and not held. A held fact
stands in no frame, whatever points at it, and no octagon is filled: *held* is *outside every frame*. Which
contexts a fact is in is read off the arrows that enter it, framed or not.

A frame is labelled with the relations of the facts that point at what is in it and the word *ctx*: *think ctx*.
A fact that facts of two relations point at is in one frame that names both, *says, think ctx*, since every
object is drawn once. Frames stand side by side, one for each label, and never one inside another. An atom is in
no frame. A pattern is drawn alike: a fact pattern under an arrow of another is in a frame, and in none where it
is also a premise, or a literal of the conclusion. A drawing with no fact under an arrow has no frame.

**Why.** The arrows already show which facts are nested and in which contexts; what a drawing cannot show
without a mark is whether a nested fact is also held, in the global context. A frame around what is in a context
only is that mark, and a fact in a context that is also held is drawn as every held fact is, outside every
frame. A filling for *held* would be a second mark of the same property. A frame for each context fact, holding
what that one fact speaks about, cannot be drawn where a fact is in two contexts without drawing it twice; a
frame around the held facts would change every drawing and cannot stand beside another frame that overlaps it.

**Held by.** [The knowledge base](../../ein/reasoning/01-static-ir/01-knowledge-base.md), § The global context, and
the context of a fact, which states the convention and draws the three cases;
[the rules](../../ein/reasoning/01-static-ir/02-rewriting-rules.md), § Packing and unpacking;
[facts](../../ein/user/03-syntax-and-semantics/03-facts.md) and
[rules](../../ein/user/03-syntax-and-semantics/05-rules.md), one drawing each. What `ein render` draws is
[Q-M1f.395](../cli/Q-M1f.395-ein-render-draws-every-object-once.md)'s, which follows this one, every fact as
the graph has it ([Q-M1f.419](../cli/Q-M1f.419-ein-render-draws-a-fact-as-the-graph-has-it.md)).
