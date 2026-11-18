# Q-M1f.427 — A rule's pattern is drawn as a fact, and no macro by its name

`ein render rule` and `rules`, and the program view's rules and query, draw a pattern as a fact is drawn: an octagon
with no name in it, its arrow `1` at its relation, a box per panel, or the variable's diamond where the relation is
a variable, and its next arrows at its arguments; folded under `--fold`
([Q-M1f.425](Q-M1f.425-fold-folds-a-fact-of-three-arrows-into-an-arrow.md)). A guard, `(neq …)` or `(eq …)`, is a
dotted undirected link with its sign beside it; an `(absent …)` is a dashed cluster, one under another a cluster
inside it; a `(not P)` is a red pattern of `not` whose arrow `2` is at `P`'s node. A macro's call is a pattern as
called, with its arguments as written: `std.macro`'s `forall` and `unknown` are no exception, so a `forall`'s guard
written as an `(and …)` with a `neq` under its arrows is drawn as the patterns it writes, each its node in the frame
of what points at it.

**Why.** A pattern is the shape of the facts it matches, so it is drawn as they are, and a reader of a rule reads
the same graph a reader of a knowledge base does. A drawing that read a macro's name would give one theory's
reading of it to every program ([Q-M1f.436](../engine/Q-M1f.436-the-kernel-reads-no-theorys-name.md)); a guard and
an absence are the kernel's own, and keep their marks.

**Held by.** `ein_render::rules`; `ein-render/tests/every_object_once.rs`: `the_rule_view_draws_a_pattern_as_a_fact`;
`ein-render/tests/presentation_semantics.rs`; `ein-render/tests/renaming_invariance.rs`, every view the same with
every name renamed. The page is [rendering](../../ein/user/04-reference/12-rendering.md#rules--ein-render-rules-and-ein-render-rule).
