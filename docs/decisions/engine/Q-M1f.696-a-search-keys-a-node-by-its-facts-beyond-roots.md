# Q-M1f.696 — A search keys a node by its facts beyond root's, and a state compared with one built another way by all of them

A state's identity is its facts, sorted, never a hash of them (`canon::state_key`). A search keys a node by the part
beyond root's: the facts its knowledge base holds above root's mark, read as a resumed node's delta is
([Q-M1f.674](Q-M1f.674-a-resumed-nodes-new-facts-are-read-off-its-own.md)), and sorted as the whole key is. Where a
state is compared with one reached another way, which holds no mark of root's stack, the whole key is read, root's
facts in it: the read-out's check of a printed state's parent paths, each path a fresh saturation over the whole
knowledge base (`region::Nodes::by_parent` and `by_split`), and a saved container's solutions, whose stored key is
the whole key, compared with a state rebuilt from the file, so the `.einb` format does not move with how a search
keys its nodes.

**Why.** Every state of a search holds root's facts, so the part beyond them tells two states apart exactly as the
whole does, and costs a node its own facts and not root's fixpoint. A state built from scratch or rebuilt from a file
has no mark to read above, and a key read one way on one side of a comparison and another way on the other would
tell a state from itself.

**Held by.** `ein_infer::state_key`; `ein_infer::region`'s node function, whose key a build with debug assertions,
the gate's, holds to the whole key less root's facts at every node; `ein_infer::region::Nodes::by_parent` and
`by_split`; `ein_einb::solutions::SolutionNode`'s `state_key`, and `ein-einb/tests/roundtrip.rs`:
`a_stored_solution_reconstitutes_the_model_it_was_taken_from`. The design page is
[`07-search.md` § 5](../../ein/ein.rs/03-design/07-search.md#5-a-states-identity).
