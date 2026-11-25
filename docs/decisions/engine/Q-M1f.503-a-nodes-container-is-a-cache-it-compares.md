# Q-M1f.503 — A node's container is a cache: it compares the standard library, where its saved file compares nothing

A node's container records, as every container does, the digest of the standard library its save resolved, and an
open under another library prints `warning: X.einb was written under different inputs (StaleStdlib)` on stderr,
where the saved file it was made from, which carries its modules and compares nothing
([Q-M1f.401](../read-out/Q-M1f.401-a-saved-file-carries-its-program-and-modules.md)), says nothing. The answer is
the same: the container stores the program the saved file carries, its imports resolved against the modules it
carries, and a container not believed drops only a derived state, which the run derives again. A node's container
prints its saved file's stderr ([Q-M1f.413](Q-M1f.413-a-nodes-container-opens-as-the-node.md)) under the library it
was saved under.

**Why.** A container is a cache, and its freshness is the cache's own: a node's container is one as a program's is,
and a root's container stays a program's container. A saved file is a text that loads alone, and comparing its
modules with a library is not its loader's business.

**Held by.** `ein_einb::meta` (`Meta::freshness`, `stdlib_digest`); `ein-cli/tests/einb_cli.rs`:
`an_edited_stdlib_is_a_cache_miss_rather_than_a_stale_hit`, a container opened under an edited library;
`ein-cli/tests/saved_files.rs`: `what_a_saved_file_is_refused_for`, a saved file under a library that is not there,
nothing said. The page:
[09-binary-format.md § 4.3](../../ein/ein.rs/03-design/09-binary-format.md#43-a-file-not-believed).
