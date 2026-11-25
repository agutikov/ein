# Q-M1f.674 — A resumed node's new facts are read off its own layers, above root's mark

A node of a search resumes root's saturation, seeded with the facts its knowledge base holds and root's snapshot did
not ([Q-M1a.18](Q-M1a.18-a-node-resumes-roots-saturation.md)). They are read off the node's own layers. The snapshot
keeps a mark of root's stack, each layer that holds a fact by its stamp and its length; a knowledge base whose stack
begins with those layers, each as long and the last at least as long, grew from root's and holds root's facts first,
so what it holds after them is all that can be new. Of those, a fact the snapshot held is left out, for the snapshot
keeps its set of root's facts: an `open` fact the empty commitment's fork holds as root did is no part of the delta.
The mark is compared at every read. A knowledge base that did not grow from root's, a copy whose top is its own or
one flattened or rebuilt, is walked whole, every fact it holds asked of the snapshot's set.

A layer's stamp is its identity and nothing else: a number no other layer is given, drawn from one process-wide
counter where a layer is made or copied, and kept where one is moved, as sealing moves the top. It is compared and
never ordered or printed, and a layer's debug form does not show it, so no output depends on how many layers any
thread made before.

**Why.** The walk read root's whole fixpoint at every node to find the few facts a commitment wrote; the read above
the mark costs the delta's own size. Recognising the stack by its layers' stamps and lengths, and not by a count of
facts, keeps a caller that hands a knowledge base of another history from reading a wrong delta in silence, and
walking that one, rather than refusing it, keeps every caller answering as before.

**Held by.** `ein_core::Kb::mark` and `ein_core::Kb::facts_above`, over the layers' stamps of `ein-core/src/kb.rs`;
`ein_infer::saturator::Snapshot::new_facts_of`, which a build with debug assertions, the gate's, holds to the walk at
every read; `kb.rs`' `the_facts_above_a_mark_are_what_a_kb_grown_from_it_holds_after_them`; `saturator.rs`'
`a_delta_holds_nothing_the_snapshot_held_on_either_read` and `a_resumed_saturation_reaches_the_same_fixpoint`. The
design pages: [`03-data-model.md` § 5](../../ein/ein.rs/03-design/03-data-model.md#5-the-layered-kb) and
[`06-saturation.md` § 4](../../ein/ein.rs/03-design/06-saturation.md#4-a-node-resumes-roots-saturation).
