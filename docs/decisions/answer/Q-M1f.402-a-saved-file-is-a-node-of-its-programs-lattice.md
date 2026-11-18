# Q-M1f.402 — A saved file is a node of its program's lattice

A saved file is its program with one commitment of the program's H. Every command reads it as that node: `ein solve
F` describes **the part of the program's R above the node**, its universe H less the commitment, the read-out's
`commit` row naming the node; every mode and `ein oracle F` answer that part, node for node the program's above the
node; `ein saturate --dump F` and `ein render kb F` read the node's state. H is the program's, and `ein show hyps F`
and `ein render h-space F` read the program's H, the node aside. A state found above a saved file is saved as the
program with that state's commitment, the union, in one `(commit …)`, so every saved file is the program and one
commitment. There is no command of its own: *save* is `ein solve FILE --dump-answer DIR`, and `ein solve F
--no-search --dump-answer D` saves a loaded file's root, the node, which is `F` again byte for byte; *load* is any
command given a saved file. The dump two loads are compared by is `ein saturate --dump`'s KB section, byte for byte.
A saved file opens with one fixed line, and what a run says of the state it saved, its number, its lines, how many
commitments reach it and its minimal generators, is the answer directory's `index.json`, never the file's.

**Why.** A file read as a program whose root is its state would make every saved file a new program, with its own
H, and a save of a save would stack commitments; read as a node of its program, a saved file asks the program's
question from a place in the program's lattice, and every file of one program is one shape. A line naming a run, a
path or a state's number is not reproduced by a second save, so the file carries none.

**Held by.** `ein_infer::region` (`Floor`, `saved_node`); `ein-infer/tests/explore.rs`:
`every_mode_is_the_oracle_above_a_saved_node`, the oracle's cube of a saved file the program's above its node, node
for node, and every cell of `ein solve`'s two axes that cube, at states of R, dead nodes and owing ones over the
corpus; `ein-cli/tests/answer_round_trip.rs`; `ein-cli/tests/saved_files.rs`. The page:
[10-answer-dump.md](../../ein/user/04-reference/10-answer-dump.md#a-saved-file-is-a-node-of-its-programs-lattice).
