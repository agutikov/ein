# Q-M1f.344 — Each decision is one file, in a folder by subject

Each decision is a file of its own, `docs/decisions/<subject>/<id>-<slug>.md`. The slug is its title in lower case,
an apostrophe dropped, every other run of characters outside `[a-z0-9]` one `-`, cut at the last whole word within
48 characters, a trailing connective dropped; the id alone keeps a name unique. The subjects are the folders
[`docs/decisions/README.md`](../README.md) lists: the language, imports, diagnostics, the answer, the hypotheses,
the read-out, the commands, the engine, the stdlib by module, the corpus, and the documentation. Each folder has a
`README.md` with one line a decision, in id order: the id, linked to its file, then the title.

A file holds a title that states the answer as a sentence; what holds today; **Why.**, the reason; and **Held by.**,
the tests, code and pages of the project that hold it.

**Why.** One file a decision keeps every id's link stable when a folder grows, and lets an id resolve by its file
name alone. A folder by subject puts every decision about one part of Ein where a reader of that part looks.

**Held by.** [`docs/decisions/README.md`](../README.md) and each folder's index.
