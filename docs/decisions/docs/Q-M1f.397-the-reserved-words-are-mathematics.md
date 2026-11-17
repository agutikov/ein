# Q-M1f.397 — The reserved words are mathematics in the reasoning and the kernel's surface elsewhere

On the pages of `docs/ein/reasoning/` the reserved words are mathematical objects. The stored negation, falsity,
an absence, equality, `written` and `relation` are defined there by what each is in the graph and what the
definitions read of it, and beyond the one section that lists the reserved atoms those pages do not speak of a
word a program may not spell.

On the pages of `docs/ein/user/` and `docs/ein/ein.rs/` the same names are the surface between the reasoning
kernel and a theory written in Ein: the fixed list of relations whose facts the kernel reads or states, with what
it does with each. Everything else a program reasons with is a rule of Ein, the program's or the standard
library's, and means what those rules make it mean.

**Why.** The reasoning is true of any implementation, and there a negation is a fact of a relation with a fixed
shape, not a keyword. A user and a reader of the engine need the other thing: exactly which names the kernel acts
on, since every other relation is theirs to give a meaning to.

**Held by.** [The knowledge base](../../ein/reasoning/01-static-ir/01-knowledge-base.md), § The reserved atoms and
§ A context gives a fact no truth; [the reserved names](../../ein/user/04-reference/03-reserved-names.md), whose
opening states the surface; [static IR in code](../../ein/ein.rs/08-reasoning-in-code/01-static-ir.md), the row of
the reserved words.
