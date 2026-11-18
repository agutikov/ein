# Q-M1f.401 — A saved file carries its program and modules as written, and reads no other file

A saved file is one fixed first line, then `(program …)` around the program file's bytes and `(module NAME …)`
around each module's, as the load read them: every module the load read, each once, under the name the resolver
gives it (`std.x.y`, or a path from the program's directory, a dot a directory), sorted by name, a module's own
imports among them. Each text is read as the file it was, its positions its own, and every import is resolved
against the modules the file carries, **never the disk**: a saved file reads no other file. The bytes are those the
resolution chose, from `$EIN_STDLIB`, the checkout or the binary's embedded copy; a state saved from a `.einb`
carries the program the container stores, resolved and import-flattened, and no module. A carried module of the
standard library that is not the one the binary would read is loaded as carried, and a `note:` names it. A
`(program …)`, `(module …)` or `(commit …)` stands only at the top of a saved file, never in a program.

**Why.** The program unchanged and a file that loads alone pull apart where a program imports: inlining the modules
rewrites the program, and the import lines as written need the modules beside the file. Carrying each text as it
is keeps both. The texts are forms of the language and not strings, since a string is natural-language text.

**Held by.** `ein_ir::saved`, `ein_ir::imports::Resolver::carrying`; `ein-cli/tests/saved_files.rs`:
`a_saved_file_reads_no_other_file`, the five modules carried as written, a load with `$EIN_STDLIB` pointing
nowhere, and a local module and the program gone from beside the file; `what_a_saved_file_is_refused_for`, the
note. The page: [10-answer-dump.md](../../ein/user/04-reference/10-answer-dump.md#a-state-file-is-its-program-and-the-choices-made).
