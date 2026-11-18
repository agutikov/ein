# Q-M1f.413 — A node's container opens as the node

`ein kb save F X.einb` of a saved file stores the program `F` carries, its imports resolved against the modules `F`
carries and never the disk, and, where `F` has a `(commit …)`, the commitment in a section of the container of its
own, `COMMIT`: each candidate of H as the file writes it, in its order. An open sets the stored program's commitment
from it, as a saved file's load sets it, so every command reads the container as it reads `F`: `ein solve X.einb`
prints what `ein solve F` prints, byte for byte but the path it echoes, with the same stderr and exit code; `ein
saturate` and `ein render kb` read the node's state; `ein test` asks it nothing; `ein show hyps` and `ein render
h-space` read the program's H. Saved again, the container is the same bytes. A root has no commitment, and its
container holds no `COMMIT`, the container of a program's. The section is a kind the format adds, so the format is
2.1, `einb/2.1`; what the kernel interns before a program's names is what `SYMBOLS` holds and not its layout, so a
change there moves no version.

**Why.** A saved file is a node of its program's lattice
([Q-M1f.402](../answer/Q-M1f.402-a-saved-file-is-a-node-of-its-programs-lattice.md)), and a container answers as
the file it was saved from ([Q-M1a.22](../cli/Q-M1a.22-a-containers-solve-is-its-sources-byte-for-byte.md)): the
container of a node that held only the program would answer another question than its file. The commitment is text,
since a choice is a candidate of H by its text
([Q-M1f.400](../read-out/Q-M1f.400-a-saved-file-states-the-choices-made-in-one.md)), read against H after root's
fixpoint. A section a reader of the same major does not know is skipped, which is what a minor adds.

**Held by.** `ein_einb::sections::write_commit` and `read_commit`, `ein_einb::header::Kind::Commit` and
`FORMAT_MINOR`; `ein-cli/tests/einb_cli.rs`: `a_nodes_container_opens_as_the_node`; `ein-cli/tests/answer_round_trip.rs`,
each corpus program's last saved node stored and solved against its file; `ein-einb/tests/roundtrip.rs`:
`a_nodes_commitment_is_stored_and_a_roots_is_not`. The pages:
[09-binary-format.md](../../ein/ein.rs/03-design/09-binary-format.md#23-the-sections) and
[10-answer-dump.md](../../ein/user/04-reference/10-answer-dump.md#a-saved-file-is-a-node-of-its-programs-lattice).
