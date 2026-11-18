# Q-M1f.527 — A slice is drawn with what its facts point at

A slice of a knowledge base, the facts about relations
([Q-M1f.523](Q-M1f.523-the-facts-about-relations-are-every-fact.md)) or the facts that answer a goal
([Q-M1f.529](Q-M1f.529-ein-render-kb-draws-the-slice-of-a-goal.md)), is drawn by the KB view's code: each fact it
keeps an octagon in the canonical form, or folded under `--fold`
([Q-M1f.425](Q-M1f.425-fold-folds-a-fact-of-three-arrows-into-an-arrow.md)), and every fact one of them points at,
at any depth, its node, held in no frame, and held nowhere in the frame named for the relations of the facts that
point at it ([Q-M1f.395](Q-M1f.395-ein-render-draws-every-object-once.md)). A fact under an arrow that the knowledge
base does not hold is drawn only through a kept fact that points at it: a slice keeps held facts alone. A slice that
keeps no fact is a digraph with no node, which says so.

**Why.** A fact a kept fact points at is one of its objects, and a drawing that left it out would draw an arrow to
nothing; a fact held nowhere is no answer to anything asked of the knowledge base, and is drawn where it is met, in
the context of what points at it.

**Held by.** `ein_render::kb_dot::KbDotOpts::only`; `ein-render/tests/golden_dot.rs`, the `render_constraints`
golden, a fact held nowhere in its frame; `ein-cli/tests/render_doors.rs`: `render_kb_draws_a_goals_slice`. The page is
[rendering](../../ein/user/04-reference/12-rendering.md#the-kb--ein-render-kb).
