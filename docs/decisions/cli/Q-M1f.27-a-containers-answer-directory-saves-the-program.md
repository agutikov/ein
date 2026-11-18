# Q-M1f.27 — A container's answer directory saves the program it stores

`ein solve X.einb --dump-answer DIR` writes the directory `ein solve X.ein --dump-answer DIR` writes: the same files,
each the source's but for the path it names, and its state files. A state file carries the program its load read,
and on a container that is the program the container stores, resolved and import-flattened, every module an import
brings written whole, and no module beside it; each state file's `(commit …)` is the source's.

**Why.** The stored program is the program the run answered, so each state file loads to its state, and every view
of a container reads that same program ([Q-M1f.17](Q-M1f.17-a-view-of-a-container-draws-the-program-it.md)). A
command handed a container answers from the file it was handed, never from a source it would have to find again.

**Held by.** `ein-cli/tests/einb_cli.rs`: `the_answer_directory_of_a_container_is_its_sources`, the files, their
names and their bytes held to the source's, and on a program that imports, each state file's program and its
commitment. The directory's files: [10-answer-dump.md](../../ein/user/04-reference/10-answer-dump.md).
