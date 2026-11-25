# Q-M1f.505 — `ein kb save --saturate` of a node banks root's fixpoint, and the open commits the node

`ein kb save` of a saved file banks a state of root, never the node's: by default the loaded KB, and under
`--saturate` root's least fixpoint, with the node's commitment beside it in the container's `COMMIT` section
([Q-M1f.413](Q-M1f.413-a-nodes-container-opens-as-the-node.md)). The open commits the node after root's fixpoint, as
the saved file's load does, so every command reads the node's state from either container. The save builds no H and
commits nothing, so a choice that is no candidate of H is stored as written, and the container is refused by the
commands that read the node, as its file is
([07-diagnostics.md § 2.29](../../ein/user/04-reference/07-diagnostics.md#229-a-saved-file-is-its-program-its-modules-and-one-commitment-of-h)).

**Why.** A commitment is committed after root's fixpoint
([Q-M1f.400](../read-out/Q-M1f.400-a-saved-file-states-the-choices-made-in-one.md)), so root's state is the one a
node's file and its container both start from. A container that banked the node's state would start from another
place than its file, and would have the commitment read against H at the save.

**Held by.** `ein-cli/src/kb.rs` (`cmd_save`); `ein_core::Program::commit`, which an open sets;
`ein-cli/tests/einb_cli.rs`: `a_nodes_container_opens_as_the_node`, the container's state the node's. The page:
[09-binary-format.md § 1](../../ein/ein.rs/03-design/09-binary-format.md#1-what-it-is-for).
