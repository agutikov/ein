# Q-M1f.18 — The read-out's `(model …)` is the claim form, and `-P` prints the model

The word *model* names two objects, each printed under its own form. A listing's `(model …)`, and the summary's
`region.states[].facts`, is a state's facts over the candidates' relations, root's included: what tells R's states
apart, and the region language's claim form, pasted into an `:expect` as printed. A state's **model**, its positive
facts minus the positive initial KB, is what `-P` prints. `-t`'s `read-out` row says *model* of the first.

**Why.** `(model …)` is a form of the language, and a claim is written in it; printing the other object under that
name on the screen that holds the claim blocks would give one word two sets in one place.

**Held by.** [08-read-out.md](../../ein/user/04-reference/08-read-out.md#6-the-listing) § 6 and § 8, held to the printer by
`ein-cli/tests/read_out_reference.rs`; the model's definition:
[Solution semantics](../../ein/reasoning/02-kb-evolution/06-solution-semantics.md) § 3.
