# Q-M1f.400 — A saved file states the choices made in one `(commit …)`

A saved file states the choices made as one form, `(commit F …)`, after the program and its modules: each `F` a
candidate of the program's H, written as the read-out prints it, in H's order. A load commits them after root's
fixpoint, as a node of the search commits its commitment, each fact's origin a hypothesis; a file of root has no
`(commit …)`. The commitment written is the one the state was read from. An owing node's file and a dead node's take
the same form, and nothing in the file says which the node is: the load computes the state, and the read-out says
what it is. A choice that is no candidate of H, or one named twice, is refused on every command that reads the node,
since the file is then no node of its program.

**Why.** A choice is not text of the program: written as a fact the program states, it reads as given, and saturated
together with the program it can move a fact that root derives under an `absent` before the choice. Committed after
root's fixpoint it is what the search did. Naming a candidate by its text, and not by its place in H, keeps the file
readable and keeps it a node when another candidate is added to H.

**Held by.** `ein_ir::saved`; `ein_infer::region::RootStats::commitment`;
`ein-cli/tests/saved_files.rs`: `a_choice_is_saved_as_a_commitment_and_loaded_as_the_node`,
`what_a_saved_file_is_refused_for`; `ein-ir/tests/grammar_pages.rs`' table. The page:
[10-answer-dump.md](../../ein/user/04-reference/10-answer-dump.md#a-state-file-is-its-program-and-the-choices-made);
the refusals: [07-diagnostics.md § 2.30](../../ein/user/04-reference/07-diagnostics.md#230-a-saved-file-is-its-program-its-modules-and-one-commitment-of-h).
